# Recursive Formats in `doodle`

This is a descriptive overview of the currently-experimental support for defining auto-recursive and
mutually-recursive formats in `doodle`.

As many of the core features of the `doodle` grammar, and the machinery surrounding it (e.g. `MatchTree`),
were designed in a purely non-recursive format context, there will be numerous gaps and considerations
not yet discovered or descibed in this document.

Recursive self-references, whether direct (auto-recursion) or cyclic (mutual recursion), are given
provisional support across `doodle`'s three passes (`MatchTree` lookahead construction, bidirectional TC, and code generation),
as well as in the interpreter; ones
where embedded self-references are actually parsed, as opposed to self-referential types using
[`Format::Phantom`](#phantom-is-different)

Details will frequently change, and understanding will develop as support for recursion is ironed out,
so this doc will be updated regularly, not just when descriptive bugs are fixed.

This is distinct from `experiments/doodle-rec/PLAN.md`, which is a *migration aid* for porting
design choices and implementation details of the `experiments/doodle-rec`
sandbox into `src/` (`doodle` proper). PLAN.md will eventually be irrelevant once that port is finished and merged;
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

### `Format::Phantom` is a different thing {#phantom-is-different}

`Format::Phantom` is doodle's other self-referential construct, and predates all of the above. It is
deliberately designed to **never be decoded** — its contents are include in the format tree only for
naming/typing purposes (see `doodle-formats/src/format/opentype/colr.rs`'s `paint`, which reads its
own recursive COLR structure out-of-band and explicitly warns that using `Phantom` for something
that's actually parsed is unsafe). Because `PhantomData<T>` is always zero-sized regardless of `T`,
none of the box-placement or cycle-termination work below was ever needed for it. Where `Phantom`'s
existing behavior interacts with the newer, real-recursion mechanisms (see
[`in_phantom_context`](#3-box-placement-in-codegen) below), it is preserved unchanged, not extended.

## Constructing a recursive format

### `FormatModule::define_batch_recvar`

Using `Format::RecVar(usize)` and `FormatModule::define_batch_recvar`:

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
let refs = module.define_batch_recvar(vec![(Label::Borrowed("peano"), peano)]);
let peano_ref = refs[0];
```

`RecVar(n)` is batch-relative sugar only: it names the `n`th format passed to
`define_batch_recvar` in the same call (`0` for a singly-recursive format's own level, `1` for
the second member of a mutually-recursive pair, and so on). `FormatModule::define_batch_recvar`
reserves `N` levels in advance (so that every `RecVar` has a real level to resolve against, even before
any body is installed) and replaces each and every `Format::RecVar` within the provided format-definitions
into the corresponding absolute `Format::ItemVar`, via `Format::substitute_rec_var`. `RecVar(i)` is only
well-defined when `i < N`, and a `RecVar` index that is `N` or higher is rejected with a panic during
this substitution. After the substitution is completed, all time-of-registration analysis
(e.g. inferring the `ValueType` of a format) and post-registration passes (e.g. compilation to `Decoder`, bidirectional TC)
**should never** see `Format::RecVar`, and virtually every exhaustive `match` statements scrutinizing a format that
can only be reached *after* `FormatModule::substitute_rec_var` has been called,
will typically have a  `Format::RecVar(_) => unreachable!(...)` arm, enforced by the
compiler's own exhaustiveness checking whenever a new match is added.

### `FormatModule::define_batch_with`

As an alternative, `FormatModule::define_batch_with` does not rely on or even allow `RecVar`:

```rust
let mut module = FormatModule::new();
let [peano_ref] = module.define_batch_with(
    ["peano"],
    [Box::new(|&[peano]: &[FormatRef; 1]| {
        Format::Union(vec![
            Format::Variant(Label::Borrowed("Z"), Box::new(Format::Byte(ByteSet::from([b'Z'])))),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'S'])),
                    peano.call(), // "self" - the first (only) member of this batch
                ])),
            ),
        ])
    })],
);
```

There is no lower-level public constructor for an auto-recursive-by-value (viz. as opposed to auto-recursive-through-`Phantom`)
format; all format-registration methods of `FormatModule` that were not designed for recursive formats,
infer the type of a to-be-registered format before its level is occupied. Even `FormatModule::define_format_phantom_rec_args`
relies on the fact that the `ValueType` of `Format::Phantom(..)` is contents-agnostic, allowing recursive self-references
wrapped in `Phantom` to side-step the issue of how to type a format whose level does not exist yet.

## The mechanisms

Each is additive/permissive for the new recursive case while leaving non-recursive formats' behavior
unchanged. Mechanisms 0, 5 and 6 apply only to the levels of a recursive batch; the others run on
*every* format, recursive or not.

### 0. Reachable-base-case check at batch registration

**Where**: `FormatModule::check_batch_has_base_case` (`src/lib.rs`), called from both
`define_batch_recvar` and `define_batch_with` right after a batch's bodies are installed
(and, for `define_batch_recvar`, after `Format::substitute_rec_var` has rewritten every
`RecVar` to a real `ItemVar`), before `infer_batch_types` runs. Backed by
`Format::has_reachable_base_case` (`src/format.rs`) and `RecursiveBounds::is_min_unresolved`
(`src/recursion.rs`).

None of mechanisms 1-4 reject `F := 'a' F` (a batch member that is *unconditionally*,
mandatorily self-referential, with no non-recursive alternative anywhere on its cycle): it makes
byte progress on every step, so the `MatchTree` cycle guard (mechanism 1 below) treats it as
"guarded" and happily terminates construction; it isn't left-recursive, so nothing about it is
undisambiguable; and `infer_format_type`'s `ItemVar` arm never recurses into a referenced level's
body at all (see `infer_batch_types`'s own doc comment), so there is no occurs-check-shaped
opportunity to notice it either. Left unchecked, such a format reaches `MatchTree`/codegen, and a
decoder for it can never complete on any finite input: it only ever stops because the input ran out
(or, in the interpreter, because it hit the recursion limit of
[mechanism 5](#5-recursion-depth-limit-in-the-interpreter)).

This check is about whether decoding *can* terminate, not about how deep it nests: a format with a
perfectly reachable base case still recurses as deeply as its input nests it, and enough nesting
exhausts the stack in either backend. That is what mechanism 5 bounds, in the interpreter only. (The
`RUST_MIN_STACK=8388608` set project-wide, see root `CLAUDE.md`, exists for deep
*construction-time* `Format` trees in debug builds, and is not a mitigation for either.)

The fix piggybacks on the *existing* `match_bounds`/`lookahead_bounds` machinery
(`RecursiveBounds`/`OpenSet`, `src/recursion.rs`) rather than adding a new traversal:
`Format::has_reachable_base_case` runs `match_bounds_recursive` and asks whether the result is
still `min_unresolved` once the whole traversal finishes. A `Format::ItemVar` re-entering an
already-open level already reports `RecursiveBounds::unresolved()`, and `RecursiveBounds::union`
(used by every `Union`/`Match` arm) already knows how to let one resolved sibling "rescue" an
unresolved one - exactly the existence-of-*some*-terminating-derivation semantics this check needs,
for free. `check_batch_has_base_case` calls this once per batch member (on that member's own,
already-installed body) and panics naming every member with no reachable base case, mirroring the
"Panics if any constructed format fails to type-check" convention both registration functions
already document.

This is a genuinely different, narrower failure than genuine left recursion (mechanism 1's `bad :=
bad | 'Z'`): a format can have a perfectly reachable base case and still be left-recursive (progress
is not reachability - `'Z'` is a real escape, it's simply reachable via the same zero-progress
position `MatchTree` can't disambiguate), and a format can make byte progress on every single step
and still have no base case at all (reachability is not progress - `F := 'a' F` is exactly this).
Neither check subsumes the other, and both remain necessary; this one simply runs much earlier
(eagerly, at registration time, for *both* backends at once, since both compile from the same
`FormatModule`) rather than only once something tries to build a `MatchTree`.

One consequence worth calling out: the bare-`Tuple`/`Seq`/`Option` anti-pattern (mechanism 3's
codegen `unreachable!`, below) is, by construction, *always* also a no-reachable-base-case cycle - a
`Tuple`'s sequencing has no analog of `Union`'s "pick a different branch" escape, so a directly
self-referential bare `Tuple` can never have one. This check now catches that shape earlier, at
registration, before a `FormatRef` to it can ever reach codegen - see
`phase3_final_check_pure_tuple_cycle_rejects_cleanly`'s updated expectations under
[mechanism 3](#3-box-placement-in-codegen).

### 1. `MatchTree` cycle guard

**Where**: `CycleGuard` (`src/lib.rs`), threaded through `MatchTreeStep::from_format`/`from_next`
and friends; consulted by `MatchTreeLevel::try_grow`.

Bounded-lookahead disambiguation (`MatchTree::try_build`, see `doc/DESIGN.md`) eagerly descends into a
format's structure to build a choice-tree. Left unguarded, a `Format::ItemVar` arm that revisits its
own level recurses in plain Rust call-stack fashion with no depth bookkeeping — a self-referential
format with no byte-consuming step before revisiting its own level stack-overflows *inside a single
`from_format` call*, before ever reaching `try_grow`'s own depth-limited BFS loop.

`CycleGuard` tracks which levels are currently open within one top-level `MatchTreeStep`
construction. Re-entering an already-open level can only happen with **zero bytes consumed** since it
was opened — any `Format::Byte` reached along the way already breaks eager descent into a deferred,
lazily-expanded `Next` (evaluated fresh, with an empty guard, at the next lookahead depth). So:

- A **guarded** self-reference (some byte-consuming step happens before the cycle repeats, e.g.
  peano's `'S' peano`) never re-opens an already-open level within a single step, and `MatchTree`
  construction terminates and disambiguates normally.
- A **genuinely left-recursive** self-reference (the cycle repeats with literally zero progress, e.g.
  `bad := bad | 'Z'`) sets `CycleGuard::has_cycle` (recording the re-entered level in
  `cycle_level`), and `MatchTreeLevel::try_grow` returns `MatchTreeError::Cycle { level }` the
  instant that fires. Every compile site builds its trees through `MatchTree::build_for`, which
  reports this as `cannot build match tree for <format>: zero-progress recursive cycle through
  <level>`. It sits alongside the two non-recursive ways a `Union` can fail to be disambiguated
  (`MatchTreeError::Ambiguous`, `MatchTreeError::DepthExhausted`): a zero-progress cycle is a
  `Union` that can never be disambiguated, no matter how much lookahead is allowed.

An open level is closed again once the walk leaves its body. Each open level records the
continuation it was opened with (its `exit`, stored as a `NextPtrAddr`), and `from_next` called on
exactly that continuation closes the level for the duration of the call, then reopens it
(`CycleGuard::leave`/`reenter`). Without this, a nullable level referenced again within its own
continuation (zero-width `B ~ B`, or `B ~ C ~ B` where `C` matches empty only before a `'c'`) would
be taken for a zero-progress re-entry, failing well-formed, non-recursive formats that compile fine
when `B` is inlined. Exits are matched by `Rc` pointer identity: the continuation stays alive for as
long as its level is open, so no other live `Next` can share its address, and a structurally equal
but separately allocated continuation is simply not recognized as an exit (conservative). Real left
recursion re-enters the level before reaching its exit, so it is still caught, including through a
nullable prefix. Cycle detection itself depends on the level alone; `exit` only decides when the
level stops being open.

### 2. `TypeChecker::occurs_in` generalization

**Where**: `TypeChecker::occurs`/`occurs_in`/`occurs_in_constraints` (`src/typecheck.rs`).

The occurs-check exists to reject a type that generated Rust cannot represent. Before genuine
recursion was supported, it rejected *every* self-alias unconditionally, with exactly one exemption:
`UType::PhantomData`, safe only because that content is structural and never actually walked by a
decoder.

That blanket rejection can't work for content that's meant to be decoded, so the check now tracks an
`indirected: bool` flag: `false` until the walk crosses a **nominal boundary**, then `true` for the
rest of that walk. A self-reference is only rejected when reached with `indirected == false`.

The boundaries are exactly the positions where codegen can close a cycle, with a `Box` around a
reference to a named type (there is no `UType::Union` in this codebase — sum-type-ness lives in
`Constraints::Variant`, not `UType`, unlike a more textbook ADT-shaped type representation):

- A `UType::Record` position (`ProjShape::RecordWith`): a generated `struct`.
- A `Constraints::Variant`'s labeled union-arm: a generated `enum` variant.

Not a boundary: a `UType::Tuple`/`Seq`/`Option` position (`ProjShape::TupleWith`/`SeqOf`/`OptOf`).
Closing a cycle there would need a recursive type alias (`type X = (u8, Option<Box<X>>)`, or even
`type X = Vec<X>`), which Rust does not allow. This is a nominal-type rule, not Rust's E0072
criterion: the two disagree in both directions (E0072 rejects `struct X(u8, Option<X>)`, and accepts
`struct X(Vec<X>)`). Also not a boundary: a plain `Var`-forwarding dereference, or
`Constraint::Equiv` (direct type equivalence, not an embedding). All of these carry `indirected`
through unchanged. `visited` is keyed by
`(canonical constraint index, indirected)` and threaded through the *whole* walk without resetting at
boundaries, so a legitimate, already-indirected cycle reached while searching for an unrelated target
can't cause an infinite loop either.

A batch member's self-reference closes its cycle through `unify_var_pair`, which runs no
occurs-check of its own, so `infer_var_format_level` runs `occurs` on the level's placeholder
variable after unifying it with the inferred body. Without that, the check above never fires for a
batch member at all.

Only codegen needs the nominal rule; the interpreter handles anonymous recursive values. It is
controlled by `TypeChecker`'s `nominal_cycles_only` flag, which codegen and `doodle type-check`
leave set. The registration-time batch check ([mechanism 6](#6-type-checking-batches-at-registration))
clears it, so `Tuple`/`Seq`/`Option` count as boundaries again there.

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

The same path also handles a field whose declared type is `Option<RecBox<T>>` (a self-reference
closing its cycle under a `Maybe`). The local it is built from has type `Option<T>`: the
`LiftedOption` node producing it is lifted outside the context where `RecBox` is inserted, so the
box belongs at the construction site, and `embed_expr` emits `.map(Box::new)` for such fields,
alongside `Box::new(..)` for a bare `RecBox`. Other uses of the local keep their unboxed type. Not
covered: `box_wrap_if_needed` (Variant/Tuple payloads built through `CaseLogic`) still recognises
only a bare `RecBox`, and `TypedExpr::Variant` does no boxing at all; no current format or fixture
reaches either with an `Option<RecBox<_>>` payload.

**Deliberately unsupported**: promoting a self-referential `Tuple`/`Seq`/`Option` to a named nominal
type (the way `doodle-rec`'s own `elaborate.rs` gives every batch member a name) has not been built.
See [Anti-patterns](#anti-patterns--things-that-are-rejected-or-unsupported) below for the precise
boundary of what this means in practice — it does **not** mean "a `Tuple` can't be part of a
recursive cycle," which is a much narrower restriction than it might sound.

### 4. Decode-time dispatch

**Where**: `TypedDecoder::Call`/`CaseLogic::Simple(SimpleLogic::Invoke)`, backed by `decoder_map`/
`compile_queue` (`src/codegen/typed_decoder.rs`); `Decoder::Call` in the interpreter
(`src/decoder.rs`).

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

Two constructs used to add such a wrapper, and both now go through one pair of smart constructors,
`Next::cat`/`Next::sequence` (`src/lib.rs`, right after the `Next` enum), used identically by both
`decoder::Compiler::compile_format` (interpreter) and `GTCompiler::compile_gt_format` (codegen):

- **`Tuple`/`Sequence`'s trailing field-suffix**, wrapped in `Next::Sequence(remaining, next)`, the
  last field's included (with `remaining` empty).
- **`LetFormat`/`MonadSeq`'s first component**, wrapped in `Next::Cat(second, next)`, where
  `second` is commonly a record's closing `Compute(Record(...))` step (from `Format::record`'s
  desugaring to nested `LetFormat`s).

Both classify the content they would wrap by its *lookahead effect* (`Format::lookahead_effect`,
`TypedFormat::lookahead_effect`): how a format at the head of `Next::Cat(format, next)` affects the
`MatchTree` walk of that continuation.

- `LookaheadEffect::Transparent` (`Compute`, `Pos`, ..., and wrappers/sequences of these): walking
  it reduces to walking `next`, so no wrapper is added and `next` is used as-is.
- `LookaheadEffect::Terminal` (e.g. `EndOfInput`): accepts without consulting `next`, so the
  continuation becomes `Next::Empty`.
- `LookaheadEffect::Opaque`: everything else, including any `ItemVar`/`FormatCall`, which is not
  descended into. The wrapper is kept.

A sequence's effect is that of its first element that isn't transparent, so an empty or
all-transparent suffix is transparent (the original `remaining == []` case falls out of this). Matching
zero bytes is not enough to be transparent: a zero-width `Peek`/`PeekNot`/`Fail` still constrains the
lookahead of whatever precedes it, so it is opaque. The classification stops at the first node that
isn't transparent and never enters a named level, so it needs neither a `&FormatModule` nor any
protection against cycles. (`is_nonproductive`, which these constructors used to call, is now only
used by tests.)

These changes are also observable for *non*-recursive formats (more `decoder_map` sharing means
fewer, differently-numbered generated decoder functions), which is an incidental cleanup of a
pre-existing inefficiency rather than a sign the change is too broad.

#### Which continuation a level's decoder is keyed on

Dropping no-op wrappers is not enough on its own. In `tree := 'L' | 'N' ~ tree* ~ 'E'`, compiling
`tree` against some continuation `N` reaches the inner call with continuation `Repeat(tree, 'E' ++ N)`,
a new key, which queues another compile one nesting level deeper, without end. Yet `tree`'s decoder
never looks at its continuation: every choice in it is settled by `'L'`/`'N'` against `'E'`. Two things
bound this, both in `src/lib.rs` and used by both compilers:

- **`NextDependence`** works out, per level and cached, whether the level's decoder depends on its
  continuation at all. If not, the call is keyed on `(level, Next::Empty)` instead of
  `(level, next)`. A level depends on its continuation if some `MatchTree` built while compiling it
  can consult that continuation, directly or within a level it calls with a continuation that
  extends its own. To find out, each decision point's tree is built against a `Next::Probe` marker,
  which behaves like `Next::Empty` for lookahead but is recorded in the `CycleGuard` when reached
  (`MatchTree::reaches_probe`; a failed build also counts, since whether it fails can depend on the
  probe). Recursive levels are resolved as a least fixpoint: a level reached again while its own
  answer is being worked out is assumed independent. A `true` answer is always final; a `false`
  answer that relied on that assumption for a level further out is not cached. `Maybe`, whose inner
  format codegen compiles against the outer continuation and the interpreter against `Next::Empty`,
  is analysed as the former, so the answer covers both backends.
- **`Next::truncated`** cuts a continuation, at a call, once its minimum length reaches
  `MATCH_TREE_MAX_DEPTH` (80 bytes). No decision inside the level can look that far, so the compiled
  decoder is unchanged, and a recursive level that *does* depend on its continuation is compiled for
  a bounded set of continuations rather than an unbounded one, provided each nesting level adds at
  least a byte.

That leaves levels that depend on their continuation but are called with continuations that happen
not to differ in anything they consult: these compile to identical decoders. Once compilation
finishes, `merge_identical_decoders` (shared by `decoder::Compiler` and `GTCompiler`) merges decoders
that are equal along with their types into the first of them, renumbers every `Call` accordingly, and
repeats until nothing more merges, since merging two decoders' callees can make their callers
identical too. Index 0, the entry point, keeps its index. Merging is by exact equality, so it also
merges identical decoders of distinct levels, which is equally safe.

#### `TypedFormat`'s own cycle-safe traversals

**Where**: `TypedFormat::match_bounds`/`lookahead_bounds` and `LevelCell`
(`src/codegen/typed_format.rs`); `RecursiveBounds`/`guarded_bounds`/`OpenSet` (`src/recursion.rs`,
factored out so `Format`'s own `match_bounds_recursive`/`lookahead_bounds_recursive` in
`src/format.rs` and `TypedFormat`'s share one implementation).

`TypedFormat::FormatCall` holds its callee's body through that level's `LevelCell`, a newtype over
`Rc<OnceCell<Rc<TypedFormat>>>`. `Elaborator::elaborate_format`'s `Format::ItemVar` arm
(`src/codegen/mod.rs`) creates a level's cell, unset, *before* recursing into the level's own body
(see [Mental model](#mental-model) above). Every `FormatCall` to that level, including the
self-references inside its body, shares that one cell, which is set exactly once, when the body is
finished. An elaborated recursive format is therefore a genuinely cyclic structure, and anything that
follows `FormatCall` bodies has to be cycle-safe:

- **Bounds.** `match_bounds_recursive`/`lookahead_bounds_recursive` use the same
  `open: &mut OpenSet`/`RecursiveBounds` pattern as `Format`'s (via the shared `guarded_bounds`
  helper): re-entering an already-open level reports `RecursiveBounds::unresolved()` instead of
  recursing into its body. A cell that is still unset (only possible mid-elaboration) also reads as
  unresolved.
- **`Debug`.** `LevelCell` prints `LevelCell(<recursive>)` where it re-enters a cell it is already
  printing, and `LevelCell(<open>)` for an unset cell; otherwise it prints the body. A derived
  `Debug` would follow the cycle without end.
- **Equality.** `LevelCell` compares by identity, since comparing contents could loop forever on
  cyclic formats from separate elaborations. Within one elaboration this gives the same answers as
  comparing contents, since each level has one cell and `FormatCall` compares its level first.

### 5. Recursion-depth limit in the interpreter

**Where**: `FormatModule::is_recursive_level` (`src/lib.rs`); `Decoder::Recursive`,
`Program::set_recursion_limit` and `Program::DEFAULT_RECURSION_LIMIT` (`src/decoder.rs`);
`DecodeErrorKind::RecursionLimit`.

[Mechanism 0](#0-reachable-base-case-check-at-batch-registration) ensures a recursive format *can*
terminate, not that it does so within any particular depth. Nested BSON documents, for instance,
recurse as deeply as the input nests them, and a few KB of input is enough to overflow the
interpreter's stack, which aborts the process rather than returning an error.

`FormatModule` records which levels belong to a recursive batch (`is_recursive_level`; not
serialized). Only batch levels can be referenced before their definition, so every cycle of
references passes through one. The interpreter's compiler wraps each such level's decoder in
`Decoder::Recursive(name, inner)`, which counts itself in a thread-local depth while it runs (through
an RAII guard, so every return path restores it), and fails with
`DecodeErrorKind::RecursionLimit { level, limit, offset }` once the `Program`'s limit is reached. The
limit defaults to `Program::DEFAULT_RECURSION_LIMIT` (256, enough for MongoDB's 100-level document
nesting at 2 calls per level) and is set with `Program::set_recursion_limit`.

Constructs that recover from a failed parse would otherwise let the limit change what an input
decodes to (`Permit` substituting its default, `PeekNot` succeeding, `Parallel` trying another
branch), so:

- `Permit` and `PeekNot` always propagate a `RecursionLimit` error.
- `Parallel` propagates it only inside a recursive decoder (depth > 0). At depth 0, e.g. the
  top-level choice between the formats a file is tried as, it rules out that branch like any other
  error.

Both interpreters (`run` and `run_with_loc`) behave the same.

The limit counts nested calls, not stack space, so it prevents an overflow only if the stack has room
for `limit` nested calls. Both `Decoder::parse` and `parse_with_loc` therefore select one
`#[inline(never)]` method per arm and call it through a function pointer, rather than running one
large `match` whose frame every recursive decode would pay for. Nested BSON decodes to depth 127 and
fails cleanly with the limit error from 128 on; at depth 127 this needs about 2.3MB of stack in a
debug build (about 2.5MB with `--trace`), and 0.7-0.8MB in release.

Generated code has no corresponding limit: its recursion depth is bounded only by the stack.

### 6. Type-checking batches at registration

**Where**: `infer_batch_types` (`src/lib.rs`), `TypeChecker::solve_batch_types` (`src/typecheck.rs`),
`ValueType::Rec` (`src/valuetype.rs`).

Registration's own inference (`infer_format_type`) never recurses into a referenced level's body, so
within a batch it reads a member's self- and forward-references as `ValueType::Any`, which
`ValueType::unify` accepts. On its own, that let an ill-typed use of a recursive field (comparing
peano's `S` payload against a `u8`) pass registration and then panic in the interpreter, and likewise
for a format registered *after* the batch that reaches into a member's recursive field. A second
inference pass would only push the `Any` one level deeper, since structural recursive types never
converge.

`infer_batch_types` therefore also runs the equirecursive `TypeChecker` over the batch's members
(`TypeChecker::solve_batch_types`), and panics on failure like registration's other type errors. It
checks only that a consistent typing exists: it skips `check_unique_solutions`, which exists so that
codegen's elaboration cannot fail, and clears `nominal_cycles_only` (see
[mechanism 2](#2-typecheckeroccurs_in-generalization)).

The solved types are then installed as the members' registered types, which needs a way to write a
recursive `ValueType`. `ValueType::Rec(level)` is a reference to a level's registered type, occurring
only where a batch member's type closes a cycle; a registered type's head is always structural. For
peano, the registered type is `Union { Z: U8, S: (U8, Rec(peano)) }`. The conversion
(`legacy_valuetype`/`legacy_valuetype_var`) follows registration's conventions rather than `reify`'s:

- an unconstrained variable becomes `Any`;
- a numeric candidate set that is not a singleton becomes `NumericHole`, even when it has a default
  (highest-priority) solution;
- every union branch is kept, with no pruning of uninhabitable ones;
- a level outside the batch becomes its registered type, as `ItemVar` inlines it;
- a batch level's variable reached again while it is being expanded becomes `Rec(level)`. Every
  cycle in the solution closes at a level's placeholder variable, so this terminates.

Registration's inference resolves `Rec` wherever it looks inside a type. `TypeScope` carries the
module's registered types (inherited by child scopes) and unfolds a head `Rec` where a component type
is extracted (`Var` lookup, `TupleProj`/`RecordProj`/`SeqIx` results, `Pattern::build_scope`).
`ValueType::unify_with` resolves `Rec` against those types, assuming coinductively that `Rec` pairs
already being unified do unify, so it terminates; plain `ValueType::unify` resolves nothing.
Conversions that only see type annotations (`AugValueType`/`ValueTypeExt` from `ValueType`) panic on
a hand-written `Rec`; `mk_value_expr`, `RustType::try_from` and `UType::from_valuetype` treat it like
the other types they cannot convert.

## Patterns that work

- **Self-recursion through a `Union`** (peano: `'Z' | 'S' peano`). The cycle passes through a named
  `enum`, so type declaration, occurs-check, and box placement all have a natural home.
- **Mutual recursion where at least one member on the cycle is a `Union`/`Record`**, even when other
  members are raw `Tuple`s (ping/pong: `ping := 'Z' | 'A' pong`, `pong := 'B' ping`, where `pong`
  itself is a bare `Tuple`). What matters is that the *cycle as a whole* crosses a `Union`/`Record`
  boundary somewhere, not that *every* member individually is one — see the next section for exactly
  where that stops being true.

## Anti-patterns — things that are rejected or unsupported

- **A recursive cycle with no `Record`/`Union` boundary anywhere on it**, i.e. one passing only
  through `Tuple`/`Seq`/`Option` positions (e.g. `(tag, Maybe(tag == 'C', self))`, or two batch
  members that are each just `Tuple[Byte, ItemVar(other)]`, referencing each other with nothing else
  in between). Codegen cannot represent such a type, so `TypeChecker`'s occurs-check rejects it in
  codegen and `doodle type-check` (see [mechanism 2](#2-typecheckeroccurs_in-generalization)). Batch
  registration and the interpreter accept it as long as it has a reachable base case, as the first
  example does through `Maybe`. The second example has none (a `Tuple`'s sequencing has no analog of
  `Union`'s "pick a different branch" escape), so it is rejected even
  earlier, at `define_batch_recvar`/`define_batch_with` registration time, by
  [mechanism 0](#0-reachable-base-case-check-at-batch-registration)'s
  `no reachable base case for: ...` panic - before a `FormatRef` to it can ever reach codegen. The
  original, codegen-specific guard (`unreachable!("Expansion::Tuple(UVar) is directly
  self-referential (an anonymous tuple whose own element type is itself, with no Record/Union
  boundary in between) - not yet supported by codegen")`) is kept as a defensive backstop - since
  `define_batch_recvar`/`define_batch_with` are documented as the only sanctioned entry
  points for real recursion, nothing in the public API can currently reach it, but it costs little to
  leave in place against a future construction path this reasoning doesn't anticipate. This is a
  real, load-bearing distinction from the pattern above: `pong` (a bare `Tuple`) is fine specifically
  *because* its own cycle routes through `ping`'s `Union` before ever coming back to `pong`'s own
  tuple type — the (now backstop) guard only fires when a `Tuple`'s own `UVar` would need to resolve
  directly back to itself while still `in_progress`, not merely "a `Tuple` participates in a
  recursive cycle somewhere." If a real format ever needs this shape, it should be worked out against
  that concrete case (likely: extending `elaborate`-style promotion-to-nominal-struct for a bare
  `Tuple`) rather than assumed to already work.
- **A recursive batch member with no reachable base case at all** (e.g. `F := 'a' F`, with no
  non-recursive alternative anywhere on its cycle - see [mechanism 0](#0-reachable-base-case-check-at-batch-registration)).
  Unlike left recursion below, this makes byte progress on every step, so nothing about `MatchTree`
  or the occurs-check would otherwise catch it; rejected immediately at
  `define_batch_recvar`/`define_batch_with` registration time instead, naming every
  offending batch member.
- **Genuine left recursion** — a self-reference reached with zero possible byte-consuming progress
  (`bad := bad | 'Z'`). Rejected at `MatchTree`-build time (see [mechanism 1](#1-matchtree-cycle-guard)
  above), not a stack overflow or hang. `define_batch_recvar` itself does **not** check for this
  at registration time (neither registration's own inference nor its batch type-check, see
  [mechanism 6](#6-type-checking-batches-at-registration), looks at progress) — the rejection only
  happens once something actually tries to build a `MatchTree` for the offending `Union`, e.g. via
  `decoder::Compiler::compile_program` or the codegen equivalent, and the resulting error names the
  level the cycle runs through.

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
- **`ViewFormat::ReadArray`/`fixed.rs`'s fixed-shape analysis** and **`Pattern`** — confirmed *not*
  recursion-sensitive. Fixed-shape analysis requires a statically-known byte-width, definitionally
  incompatible with unbounded recursion, so it already excludes recursive formats for the same reason
  it excludes anything data-dependent. `Pattern` cannot embed a `Format`/level anywhere in its shape,
  so it can never reintroduce a cycle through any path independent of the one `MatchTree`'s `ItemVar`
  arm already covers.

## Where the tests live

- `src/lib.rs`'s `mod test` — `define_batch_recvar_*`: self-recursive peano and mutually-recursive
  ping/pong through the real public API and the plain interpreter, a left-recursion-rejection case,
  a record-shaped mutual-recursion case (exercising the `LetFormat`/`MonadSeq` fix specifically), and
  `define_batch_recvar_no_base_case_rejected_at_registration`/
  `define_batch_recvar_mutual_no_base_case_rejected_at_registration` (mechanism 0: `F := 'a' F`
  self- and mutually-recursive shapes, confirmed to panic naming every offending member, at
  registration time rather than reaching `MatchTree`/codegen at all).
- `src/lib.rs`'s `mod test`, continued:
  - mechanism 1 and `MatchTreeError`: `matchtree_try_build_reports_*` (one per error case),
    `matchtree_left_recursion_through_nullable_prefix_is_a_cycle`,
    `matchtree_repeated_zero_width_item_is_not_a_cycle`,
    `matchtree_lookahead_separated_nullable_item_is_not_a_cycle`,
    `matchtree_ambiguous_repeated_nullable_item_not_reported_as_cycle`, and
    `match_tree_compile_errors_report_the_reason`;
  - `RecVar` misuse: `define_batch_recvar_rejects_out_of_range_rec_var`,
    `define_format_rejects_rec_var`, `define_batch_with_rejects_rec_var`;
  - continuation keys (mechanism 4): `next_dependence_is_decided_by_reachable_lookahead`,
    `interpreter_compiles_and_decodes_repeat_of_recursive_item`,
    `continuation_dependent_recursion_compiles_to_bounded_decoders`;
  - mechanism 6: `ill_typed_use_of_recursive_field_is_rejected_or_decodes_without_panic`, its
    `..._from_later_registration_...` counterpart, `rec_batch_member_type_refers_to_itself_through_rec`,
    and `rec_batch_member_type_keeps_registration_conventions`.
- `src/decoder.rs`'s `tests::recursion_limit` (mechanism 5), each case run through both `run` and
  `run_with_loc`: exact counting, the default limit, `Permit`, `PeekNot`, and `Parallel` inside and
  outside recursion.
- Codegen-side tests: `codegen_repeat_of_recursive_item_terminates` (the `tree*` case of mechanism
  4), `typecheck_accepting_tuple_self_reference_implies_codegen_succeeds` (the nominal rule of
  mechanism 2), `debug_terminates_on_recursive_format_call` and
  `level_cell_resolved_and_shared_after_elaboration` (`LevelCell`).
- `src/codegen/tests.rs` — `phase3_final_check_peano_and_ping_pong` (Box placement,
  string-level check), `phase3_final_check_pure_tuple_cycle_rejects_cleanly` (the bare-`Tuple`
  anti-pattern; now confirmed to panic with mechanism 0's earlier, registration-time message rather
  than reaching the codegen-level backstop it originally targeted — see
  [Anti-patterns](#anti-patterns--things-that-are-rejected-or-unsupported)),
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
- `tests/recursive_maybe/` and `tests/recursive_repeat/` — fixtures of the same kind, for a
  self-reference under a conditional `Maybe` (the `Option<RecBox<T>>` record field of mechanism 3)
  and one in `RepeatCount` element position, respectively.

## Further reading

`experiments/doodle-rec/PLAN.md` has the full session-by-session narrative of how each mechanism
above was investigated, designed, and verified (including the bug-injection results proving each
fix actually exercises the bug it claims to), if the *why* behind a specific design choice here isn't
enough — but treat it as historical/process record, not as a second source of truth for current
behavior once this document and PLAN.md disagree.
