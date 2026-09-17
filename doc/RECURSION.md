# Recursion in doodle

This document is the authoritative reference for how genuinely self-referential formats — ones
that actually get decoded, as opposed to [`Format::Phantom`](#phantom-is-a-different-thing) — are
supported across `doodle`'s three passes (`MatchTree` lookahead construction, the bidirectional
type-checker, and code generation) and the interpreter. It is a living document: update it whenever
the recursion model itself changes, not just when a bug in it is fixed.

This is distinct from `experiments/doodle-rec/PLAN.md`, which is a *migration aid* — a phase-by-phase
execution log for the specific work of porting a design worked out in the `experiments/doodle-rec`
sandbox into `src/`. PLAN.md will eventually be irrelevant once that port is finished and merged;
this document is meant to outlive it, recording the settled design, not the process of arriving at it.

## Mental model

A recursive format is one where some `Format::ItemVar(level, ..)` transitively refers back to its
own `level`. Nothing about `ItemVar` itself is special-cased for this — `level` is an ordinary,
absolute index into the enclosing `FormatModule`'s flat table, so a self- or mutually-referencing
group of formats ("a batch") is just a set of `ItemVar`s that happen to point into each other.

What *is* special-cased, in exactly four places (one per pipeline stage — see below), is *reserving
a placeholder before recursing into a level's own body*, so that construction/inference/codegen
terminates instead of looping forever the first time it revisits a level it's already in the middle
of processing. All four independently follow the same shape: open a level, recurse, close it; if the
same level is reached again while still open, stop and either substitute a deferred/cyclic reference
(safe) or fail loudly (a genuine grammar defect, not a decidable recursive shape).

### `Format::Phantom` is a different thing

`Format::Phantom` is doodle's other self-referential construct, and predates all of the above. It is
deliberately designed to **never be decoded** — its contents are include in the format tree only for
naming/typing purposes (see `doodle-formats/src/format/opentype/colr.rs`'s `paint`, which reads its
own recursive COLR structure out-of-band and explicitly warns that using `Phantom` for something
that's actually parsed is unsafe). Because `PhantomData<T>` is always zero-sized regardless of `T`,
none of the box-placement or cycle-termination work below was ever needed for it. Where `Phantom`'s
existing behavior interacts with the newer, real-recursion mechanisms (see
[`in_phantom_context`](#3-box-placement-in-codegen) below), it is preserved unchanged, not extended.

## Constructing a recursive format

Use `Format::RecVar(usize)` and `FormatModule::define_format_rec_batch`:

```rust
let peano = Format::Union(vec![
    Format::Variant(Label::Borrowed("Z"), Box::new(Format::Byte(ByteSet::from([b'Z'])))),
    Format::Variant(
        Label::Borrowed("S"),
        Box::new(Format::Tuple(vec![
            Format::Byte(ByteSet::from([b'S'])),
            Format::RecVar(0), // "self" - the first (only) member of this batch
        ])),
    ),
]);
let mut module = FormatModule::new();
let refs = module.define_format_rec_batch(vec![(Label::Borrowed("peano"), peano)]);
let peano_ref = refs[0];
```

`RecVar(n)` is batch-relative sugar only: it names the `n`th format passed to
`define_format_rec_batch` in the same call (`0` for a singly-recursive format's own level, `1` for
the second member of a mutually-recursive pair, and so on). `define_format_rec_batch` reserves all
`formats.len()` levels up front (so every `RecVar` has a real level to resolve against, even before
any body is installed), rewrites every `RecVar` occurrence to the corresponding real, absolute
`Format::ItemVar` via `Format::substitute_rec_var`, and only then installs and type-checks each
member. **No other pass over `Format` ever sees a `RecVar`** — every other exhaustive match over
`Format` in the codebase carries a `Format::RecVar(_) => unreachable!(...)` arm, enforced by the
compiler's own exhaustiveness checking whenever a new match is added.

There is no lower-level public constructor for a genuinely self-referential, non-`Phantom` format —
`FormatModule::define_format`/`define_format_args_views` type-check a format *before* the level it
would occupy exists, so an out-of-bounds self-reference is rejected outright by design.
`define_format_rec_batch` is the one sanctioned entry point.

## The four mechanisms

Each of these runs on *every* format, recursive or not, and each is additive/permissive for the new
recursive case while leaving non-recursive formats' behavior unchanged.

### 1. `MatchTree` cycle guard

**Where**: `CycleGuard` (`src/lib.rs`), threaded through `MatchTreeStep::from_format` and friends;
consulted by `MatchTreeLevel::grow`.

Bounded-lookahead disambiguation (`MatchTree::build`, see `doc/DESIGN.md`) eagerly descends into a
format's structure to build a choice-tree. Left unguarded, a `Format::ItemVar` arm that revisits its
own level recurses in plain Rust call-stack fashion with no depth bookkeeping — a self-referential
format with no byte-consuming step before revisiting its own level stack-overflows *inside a single
`from_format` call*, before ever reaching `grow`'s own depth-limited BFS loop.

`CycleGuard` tracks which levels are currently open within one top-level `MatchTreeStep`
construction. Re-entering an already-open level can only happen with **zero bytes consumed** since it
was opened — any `Format::Byte` reached along the way already breaks eager descent into a deferred,
lazily-expanded `Next` (evaluated fresh, with an empty guard, at the next lookahead depth). So:

- A **guarded** self-reference (some byte-consuming step happens before the cycle repeats, e.g.
  peano's `'S' peano`) never re-opens an already-open level within a single step, and `MatchTree`
  construction terminates and disambiguates normally.
- A **genuinely left-recursive** self-reference (the cycle repeats with literally zero progress, e.g.
  `bad := bad | 'Z'`) sets `CycleGuard::detected`, and `MatchTreeLevel::grow` returns `None` outright
  the instant that fires — reusing the exact same `Option`-based "cannot build match tree" failure
  convention every other undisambiguable (non-recursive) `Union` already uses, surfaced as
  `anyhow!("cannot build match tree for {}", ...)` by every caller of `MatchTree::build`. This is not
  a new error type: a zero-progress cycle genuinely is a `Union` that can never be disambiguated, no
  matter how much lookahead is allowed.

### 2. `TypeChecker::occurs_in` generalization

**Where**: `TypeChecker::occurs`/`occurs_in`/`occurs_in_constraints` (`src/typecheck.rs`).

The occurs-check exists to reject an unrepresentable type (`type X = Box<X>;` — Rust's own E0072).
Before genuine recursion was supported, it rejected *every* self-alias unconditionally, with exactly
one exemption: `UType::PhantomData`, safe only because that content is structural and never actually
walked by a decoder.

That blanket rejection can't work for content that's meant to be decoded, so the check now tracks an
`indirected: bool` flag: `false` until the walk crosses a **representability boundary**, then `true`
for the rest of that walk. A self-reference is only rejected when reached with `indirected == false`
— exactly Rust's own criterion (`type X = Box<X>` illegal, `enum X { Y(Box<X>) }` fine).

The boundaries, precisely (there is no `UType::Union` in this codebase — sum-type-ness lives in
`Constraints::Variant`, not `UType`, unlike a more textbook ADT-shaped type representation):

- A `UType::Tuple`/`Record`/`Seq`/`Option` position (`ProjShape::TupleWith`/`RecordWith`/`SeqOf`/
  `OptOf`).
- A `Constraints::Variant`'s labeled union-arm (real doodle's analog of an enum variant boundary).

Not a boundary: a plain `Var`-forwarding dereference, or `Constraint::Equiv` (direct type
equivalence, not an embedding) — both carry `indirected` through unchanged. `visited` is keyed by
`(canonical constraint index, indirected)` and threaded through the *whole* walk without resetting at
boundaries, so a legitimate, already-indirected cycle reached while searching for an unrelated target
can't cause an infinite loop either.

`PhantomData` keeps its separate, unconditional exemption unchanged — it doesn't even bind its inner
content in a way the new indirection tracking would need to reason about, and remains safe for the
same reason it always was (never walked by a decoder).

### 3. Box placement in codegen

**Where**: `CompType::RecBox` (`src/codegen/rust_ast/mod.rs`) and its insertion, in
`CodeGen::lift_uvar`/`CodeGen::box_wrap_if_needed`/`RustExpr::wrap_box` (`src/codegen/mod.rs`).

Closing a recursive type in generated Rust needs a `Box` somewhere on the cycle (Rust can't size an
infinitely-nested type). `CompType::RecBox(T)` is a distinct variant from the heap-allocation
`Box` that `HeapOptimize` can independently choose for a large-but-non-recursive field — "*representation
of `Box<X>` specifically in the context of auto-recursive or mutually-recursive types (separate from
the theoretical boxing done via `HeapOptimize`)*", per its own doc comment — with constant, non-recursive
answers everywhere it's threaded (`MemSize`, `CanOptimize`, `CopyEligible`, `HeapOptimize`,
`Rebindable`, lifetime resolution): pointer-sized, one niche, never `Copy`, no heap-strategy action of
its own.

Where the `Box` goes falls out of the *same* reservation state that already exists to make the
compiler's own graph-walk over solved `UVar`s terminate (`CodeGen::lift_uvar`'s pre-existing
`in_progress: StableMap<UVar, Option<(Label, usize, PathLabel)>>`, populated *before* recursing into
a `Record`/`Union` member's own fields, exactly like the `MatchTree`/occurs-check reservations above).
A reference is boxed, at both the **type** declaration site and the **value-construction** site
(`box_wrap_if_needed`, mirroring the existing `RustExpr::wrap_some`/`GenExpr::WrapSome` machinery for
`Option`), precisely when the `UVar` it names is an ancestor still `in_progress` — i.e. exactly where
a cycle closes, and nowhere else. Box placement is therefore determined by *traversal/reservation
order*, not "always box a named reference": in a `ping`/`pong` mutual-recursion pair, `ping`'s forward
reference to `pong` is bare, while `pong`'s reference back to the still-open `ping` is boxed.

`Format::Phantom`'s own pre-existing self-reference never needs boxing (`PhantomData<T>` never
actually stores a `T`) — `CodeGen::in_phantom_context: bool`, set for the whole subtree under
`Expansion::PhantomData` (not just its immediate child), suppresses `RecBox` insertion throughout so
this doesn't regress.

**A `Format::record`'s own field construction needed a separate fix.** `box_wrap_if_needed` is only
ever consulted from `CodeGen::translate`'s `TypedDecoder::Variant`/`TypedDecoder::Tuple` arms — a
record's closing `Compute(Record(...))` step is a plain `Expr`, translated by the free function
`embed_expr` (`TypedExpr::Record`'s arm), a completely separate code path with no knowledge of
`RecBox` at all until this was fixed. The result, found only once someone actually tried to
`rustc`-compile a self-referential `Format::record`'s generated output for the first time (see
`tests/recursion/`, Phase 5): the *type* declaration for a `RecBox`-eligible field came out correctly
boxed (`next: Box<ping>`), but the *value* constructed for it did not (`pong { tag, next }` instead of
`pong { tag, next: Box::new(next) }`) — a real `E0308` mismatch, not a cosmetic gap. Fixed by having
`embed_expr`'s `TypedExpr::Record` arm consult the record's own declared field types (available
inline off `GenType::Def`'s `RustTypeDecl`, no `defined_types` table lookup needed) and calling
`.wrap_box()` on any field whose declared type is `RecBox`-wrapped, mirroring `box_wrap_if_needed`
exactly. (The `GenType::Inline(LocalDef)` case, which doesn't carry field types inline, doesn't get
this treatment - no observed case has needed it there yet.) `cargo cg` byte-identical - a
`RecBox`-wrapped field type only exists on an already-recursive format, so no existing non-recursive
record is affected.

**Deliberately unsupported**: promoting a self-referential `Tuple`/`Seq`/`Option` to a named nominal
type (the way `doodle-rec`'s own `elaborate.rs` gives every batch member a name) has not been built.
See [Anti-patterns](#anti-patterns--things-that-are-rejected-or-unsupported) below for the precise
boundary of what this means in practice — it does **not** mean "a `Tuple` can't be part of a
recursive cycle," which is a much narrower restriction than it might sound.

### 4. Decode-time dispatch

**Where**: `TypedDecoder::Call`/`CaseLogic::Simple(SimpleLogic::Invoke)`, backed by `decoder_map`/
`compile_queue` (`src/codegen/typed_decoder.rs`); the analogous `Decoder::CallRec`-style dispatch in
the interpreter (`src/decoder.rs`).

This needed no new cycle-termination logic of its own: a queued level's slot in `decoder_map` is
reserved the instant it's discovered, before its body is ever walked, so a self-reference reached
through `TypedDecoder::Call`/an interpreted `ItemVar` never needs its own target slot's content to
already exist — Rust function calls (and the interpreter's own indexed dispatch) resolve at call
time, not by inlining/substitution the way a type alias would, so mutual recursion between compiled
decoder functions needs zero special-casing.

**The actual trap here is upstream of dispatch itself**: `decoder_map`'s cache key is `(level, next)`
— two references to the same level only share one compiled decoder if they were compiled under
*structurally identical* `next` continuations. Several constructs used to wrap `next` in a variant
of its own name unconditionally, even when doing so added no real information (an empty trailing
remainder, or a zero-width trailing step) — which, for a *non*-recursive format, is merely a missed
sharing optimization (harmless, if wasteful), but for a self-referential reference specifically, means
the recursive call is compiled under a `next` that differs from whatever that same level's own
top-level/other-caller entry used, missing the cache and queuing a **doomed duplicate compile** —
either an unbounded, ever-growing `compile_queue` (interpreter path) or a `next` under which
`MatchTree`/lookahead construction fails, silently compiling into a dead, always-failing decoder
(codegen path: `TypedDecoder::Fail`, surfacing as generated Rust like
`fn Decoder3(...) { return Err(ParseError::FailToken(...)); }`).

Two constructs needed this fix, both settled into one pair of smart constructors —
`Next::cat`/`Next::sequence` (`src/lib.rs`, right after the `Next` enum) — skip the wrap when it
would add nothing, use `next` directly otherwise, and are used identically by both
`decoder::Compiler::compile_format` (interpreter) and `GTCompiler::compile_gt_format` (codegen):

- **`Tuple`/`Sequence`'s trailing field-suffix** — every field but the last was wrapped in
  `Next::Sequence(remaining, next)`; the *last* field was too, with `remaining` empty, which is
  semantically a no-op but structurally a new wrapper layer on every visit. `Next::sequence`'s check
  is `remaining.iter().all(is_nonproductive)`, vacuously true for `remaining == []` (the originally
  fixed case falls out for free) — and it generalizes past that: a trailing suffix of *several*
  nonproductive fields, not just one, now also correctly collapses. (An earlier, narrower attempt at
  this generalization special-cased only a single trailing field, `[last]` — genuinely incomplete,
  since two or more nonproductive trailing fields still fell through to the general wrap; `.all(...)`
  subsumes that cleanly instead of needing a length-1 special case.)
- **`LetFormat`/`MonadSeq`'s first component** — wrapped in `Next::Cat(second, next)`
  unconditionally, even when `second` can only ever match zero bytes (the common case: a record's
  closing `Compute(Record(...))` step, from `Format::record`'s desugaring to nested `LetFormat`s).
  `Next::cat`'s check is simply `is_nonproductive(second)`.

Both smart constructors dispatch on `MaybeTyped::Untyped`/`Typed` to reach the right
`is_nonproductive`: `Format::is_nonproductive(module)` (`src/format.rs`, module-based, unconditionally
safe) for the interpreter path, or `TypedFormat::is_nonproductive()` (`src/codegen/typed_format.rs`,
module-free) for the codegen path — see the next subsection for why the latter needed its own fix
before this was safe to call on an arbitrary subtree.

Both fixes are also observable for *non*-recursive formats (more `decoder_map` sharing = fewer,
differently-numbered generated decoder functions) — this is a genuine, if minor, pre-existing codegen
inefficiency the fix incidentally also cleans up, not a sign the fix is too broad; confirmed
byte-identical (fresh `cargo cg` vs. the checked-in `generated/gencode.rs`) after generalizing to the
full-slice check, since no currently-registered format happens to have a multi-field
all-nonproductive trailing suffix for the generalization to actually change anything for. See
`experiments/doodle-rec/PLAN.md`'s disclaimer for how this was reconciled with that document's own
(now-revised) byte-identical-codegen expectation.

#### `TypedFormat`'s own cycle-safe bounds computation

**Where**: `TypedFormat::match_bounds`/`lookahead_bounds`/`is_nonproductive` and
`TypedFormat::recursion_placeholder`/`is_recursion_placeholder` (`src/codegen/typed_format.rs`);
`RecursiveBounds`/`guarded_bounds`/`OpenSet` (`src/recursion.rs`, factored out so `Format`'s own
`match_bounds_recursive`/`lookahead_bounds_recursive` in `src/format.rs` and `TypedFormat`'s share one
implementation).

`Next::cat`/`Next::sequence` need to ask "is this `TypedFormat` guaranteed zero-width?" without a
`&FormatModule` in hand — the whole reason a `TypedFormat`-native `is_nonproductive` exists at all is
that `TypedFormat::FormatCall` stores its callee's already-*resolved* body inline, so in principle no
module lookup should be needed. Two distinct hazards had to be closed before that was actually safe:

- **Live re-entry within one traversal.** A naive `match_bounds`/`lookahead_bounds` walk that
  recurses straight into a `FormatCall`'s `def` has no protection against a self-reference reached
  while that same level is still being walked by *this call* — unlike `Format::ItemVar`, which
  re-resolves via `module.get_format(level)` on every visit and was already guarded by the `open`-set
  in `Format::match_bounds_recursive`. Fixed by porting the identical `open: &mut OpenSet` /
  `RecursiveBounds` pattern onto `TypedFormat` (via the shared `guarded_bounds` helper): re-entering
  an already-open level reports `RecursiveBounds::unresolved()` instead of recursing into `def`.
- **A stale placeholder from a *different*, already-finished traversal — the sharper of the two, and
  not covered by the fix above.** `Elaborator::elaborate_format`'s `Format::ItemVar` arm
  (`src/codegen/mod.rs`) reserves a placeholder `def` *before* recursing into a level's own body (see
  [Mental model](#mental-model) above), so any self-reference reached while that level is still open
  gets that placeholder baked into its `FormatCall::def` *permanently* — `def` is never patched once
  the real body becomes available; only the level's own top-level entry in `Elaborator`'s `t_formats`
  cache is updated. `GTCompiler::compile_gt_format`'s own `FormatCall` arm never trusts this blindly:
  it discards `def` whenever `decoder_map` already has an entry for the level (see above). But a
  *fresh*, standalone `TypedFormat::match_bounds`/`lookahead_bounds` call — exactly what
  `Next::cat`/`Next::sequence` need to make on an arbitrary subtree, possibly one nested several
  `FormatCall`s deep — has no such protection: its `open`-set starts empty, so it happily recurses
  into a placeholder `def` with no way of knowing it's stale, and silently reports a genuinely
  recursive, byte-consuming reference as `TypedFormat::Fail`'s trivial `exact(0)` — a false positive,
  the opposite of what `is_nonproductive`'s "false negatives only" contract promises. Confirmed with a
  real regression test (`is_nonproductive_agrees`, see below) before being fixed.

  Fixed by giving the elaborator's placeholder a recognizable identity: `TypedFormat::FormatCall`'s
  `def` is now reserved from a single, canonical thread-local sentinel
  (`TypedFormat::recursion_placeholder()` — `Rc::new(TypedFormat::Fail)` constructed once) rather than
  a fresh `Rc::new(TypedFormat::Fail)` per level. `match_bounds_recursive`/`lookahead_bounds_recursive`'s
  `FormatCall` arm checks `Self::is_recursion_placeholder(def)` (an `Rc::ptr_eq` comparison) *before*
  trusting `def`'s content, reporting `RecursiveBounds::unresolved()` immediately on a match.
  Deliberately identity-based, not structural (`*def == TypedFormat::Fail`): a level whose *real*,
  fully-resolved body genuinely is just `Format::Fail` would false-positive under a structural check,
  since `Fail`'s own bounds are legitimately `exact(0)` — only the elaborator's own sentinel `Rc`
  should ever be treated as untrustworthy, not anything that merely looks like it.

  With this in place, the open-set port above turns out to be a defensive backstop rather than the
  load-bearing fix: every *genuine* self-reference within a level's own body is, by construction,
  reached while that level is still open at elaboration time, so it always gets the sentinel — a live
  re-entry past the sentinel check would require some other, currently-unknown route to a real `Rc`
  cycle. Kept anyway, since it costs little and guards against future `TypedFormat` constructions this
  reasoning doesn't anticipate.

## Patterns that work

- **Self-recursion through a `Union`** (peano: `'Z' | 'S' peano`). The cycle passes through a named
  `enum`, so type declaration, occurs-check, and box placement all have a natural home.
- **Mutual recursion where at least one member on the cycle is a `Union`/`Record`**, even when other
  members are raw `Tuple`s (ping/pong: `ping := 'Z' | 'A' pong`, `pong := 'B' ping`, where `pong`
  itself is a bare `Tuple`). What matters is that the *cycle as a whole* crosses a `Union`/`Record`
  boundary somewhere, not that *every* member individually is one — see the next section for exactly
  where that stops being true.

## Anti-patterns — things that are rejected or unsupported

- **A bare `Tuple`/`Seq`/`Option` batch member that is *directly* self-referential, with no
  `Record`/`Union` boundary anywhere on its cycle** (e.g. two batch members that are each just
  `Tuple[Byte, ItemVar(other)]`, referencing each other with nothing else in between). This is
  rejected loudly and immediately by codegen with `unreachable!("Expansion::Tuple(UVar) is directly
  self-referential (an anonymous tuple whose own element type is itself, with no Record/Union
  boundary in between) - not yet supported by codegen")`, rather than hanging or producing broken
  output. This is a real, load-bearing distinction from the pattern above: `pong` (a bare `Tuple`) is
  fine specifically *because* its own cycle routes through `ping`'s `Union` before ever coming back to
  `pong`'s own tuple type — the guard only fires when a `Tuple`'s own `UVar` would need to resolve
  directly back to itself while still `in_progress`, not merely "a `Tuple` participates in a
  recursive cycle somewhere." If a real format ever needs this shape, it should be worked out against
  that concrete case (likely: extending `elaborate`-style promotion-to-nominal-struct for a bare
  `Tuple`) rather than assumed to already work.
- **Genuine left recursion** — a self-reference reached with zero possible byte-consuming progress
  (`bad := bad | 'Z'`). Rejected at `MatchTree`-build time (see [mechanism 1](#1-matchtree-cycle-guard)
  above), not a stack overflow or hang. `define_format_rec_batch` itself does **not** check for this
  at registration time (`infer_format_type` has no occurs-check of its own for progress, only for
  representability) — the rejection only happens once something actually tries to build a
  `MatchTree` for the offending `Union`, e.g. via `decoder::Compiler::compile_program` or the codegen
  equivalent.

## Known, deliberately deferred rough edges

- **A `Tuple`'s solved `RustType` is recomputed fresh, uncached, every time it's referenced** (unlike
  `Record`/`Union`, which are always cached) — fine for an ordinary non-recursive `Tuple`, but not
  when a nested element's `RecBox`-eligibility depends on live `in_progress` state: the same
  reference can be computed as boxed in one context (nested inside an in-progress `Union`) and
  unboxed in another (computed later, once that `Union` is no longer `in_progress`), producing two
  disagreeing types for what should be one. Not yet fixed; work around it in a real format by
  preferring a named `Format::record` over a raw multi-field `Tuple` for anything that both
  participates in a recursive cycle *and* is referenced from more than one place, since `Record` is
  always cached and doesn't have this problem.
- **Parametric `ItemVar` references** (an `Expr::Var` + named-parameter mechanism for a recursive
  call) — confirmed to be a general language feature with nothing structurally recursion-specific
  about it (`Expr::Var`/`define_format_args` already exist independent of recursion); not attempted
  as part of this work, nothing to build here specifically.
- **`WithRelativeOffset`'s `MatchTree` opacity** — a legitimate, pre-existing, deliberate design point
  (its own `Self::accept() // FIXME`), inherited as-is. Not a recursion-specific gap.
- **`ViewFormat::ReadArray`/`fixed.rs`'s fixed-shape analysis** and **`Pattern`** — confirmed *not*
  recursion-sensitive. Fixed-shape analysis requires a statically-known byte-width, definitionally
  incompatible with unbounded recursion, so it already excludes recursive formats for the same reason
  it excludes anything data-dependent. `Pattern` cannot embed a `Format`/level anywhere in its shape,
  so it can never reintroduce a cycle through any path independent of the one `MatchTree`'s `ItemVar`
  arm already covers.

## Where the tests live

- `src/lib.rs`'s `mod test` — `define_format_rec_batch_*`: self-recursive peano and mutually-recursive
  ping/pong through the real public API and the plain interpreter, a left-recursion-rejection case,
  and a record-shaped mutual-recursion case (exercising the `LetFormat`/`MonadSeq` fix specifically).
- `src/codegen/mod.rs`'s `mod tests` — `phase3_final_check_peano_and_ping_pong` (Box placement,
  string-level check), `phase3_final_check_pure_tuple_cycle_rejects_cleanly` (the bare-`Tuple`
  anti-pattern, confirmed to panic with the expected message),
  `recursive_format_through_record_field_generates_no_dead_decoder` (the codegen-path counterpart to
  the `LetFormat`/`MonadSeq` fix), `regenerate_recursion_fixture` (an `#[ignore]`d generator for
  the `tests/recursion/` fixture below — run it after changing that fixture's format definitions),
  `is_nonproductive_agrees` (elaborates a JSON-like, 4-member mutually-recursive format and checks
  `TypedFormat::is_nonproductive` agrees with the clone-erase-`Format::is_nonproductive` path at every
  distinct node of the elaborated tree — the test that originally caught the stale-placeholder gap
  described above), and `is_nonproductive_agrees_over_arbitrary_formats` (a `proptest` counterpart over
  small, arbitrary *non-recursive* `Format` trees biased toward the variants the JSON-like fixture
  doesn't reach — `Peek`/`PeekNot`/`Slice`/`RepeatCount`/`RepeatBetween`/`Match`/`UnionNondet`/`Map`/
  `Where`/`WithRelativeOffset` — for arm-by-arm coverage of the `match_bounds_recursive`/
  `lookahead_bounds_recursive` port independent of the recursion/placeholder concern the other test
  targets).
- `tests/recursion/` — a self-contained mini codegen fixture (matching the existing convention of
  `tests/runtime_repeat/`/`tests/permit_state_error/`, see root `CLAUDE.md`): real, frozen production
  codegen output for peano/ping-pong (`pong` built via `Format::record`, per the Finding A workaround
  above), compiled and run as ordinary Rust against real crafted bytes at recursion depth ≥ 2 for both
  shapes, plus a malformed-input rejection case for each. This is the capstone that found and drove
  the [Box placement](#3-box-placement-in-codegen) `Format::record`-construction fix above - the first
  time this project actually compiled a self-referential format's generated output with `rustc`.

## Further reading

`experiments/doodle-rec/PLAN.md` has the full session-by-session narrative of how each mechanism
above was investigated, designed, and verified (including the bug-injection results proving each
fix actually exercises the bug it claims to), if the *why* behind a specific design choice here isn't
enough — but treat it as historical/process record, not as a second source of truth for current
behavior once this document and PLAN.md disagree.
