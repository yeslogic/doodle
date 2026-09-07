# PLAN: Porting `doodle-rec`'s recursion model into `doodle` (`src/`)

Written 2026-09-07 on branch `recursion-model`, against `experiments/doodle-rec` as of commit
`8666c57`. This document is meant to be **portable**: if `experiments/doodle-rec/*` and this file
are copied onto a different working tree (a different feature branch, with `src/` that has evolved
but is still recognizably the same architecture as described in the root `CLAUDE.md`), a fresh agent
with no memory of the sandbox work should be able to execute it end to end, pausing only at the
`TRIAGE:` checkpoints called out explicitly below.

## Purpose

`experiments/doodle-rec` is a sandbox crate that worked out, in isolation, exactly what real
`doodle` needs in order to support **genuinely self-referential formats that actually get decoded**
(as opposed to `Format::Phantom`, real doodle's one existing self-referential construct, which is
deliberately designed to *never* be decoded). This plan ports that design into `src/` for real.

The sandbox's own conclusion (reached via direct source investigation of real `doodle`, not
assumption) is that the gap is narrow: real doodle already has, independently, three different
instances of the "reserve a placeholder before recursing" pattern that make recursion-safe design
possible (`infer_var_format_level`'s `level_vars`, the `Elaborator`'s `t_formats` cache,
`CodeGen::lift_uvar`'s `in_progress`/`NameGen` reservation). What's missing is four specific, narrow
pieces, each independently confirmed and each mapped to a proven-out analog already built and tested
in this sandbox:

1. A cycle guard in `MatchTreeStep::from_format`'s `ItemVar` arm (currently unconditional recursive
   descent, zero guard — confirmed unchanged since this investigation began).
2. A generalization of `TypeChecker::occurs_in`'s exemption logic, from "skip `PhantomData`
   entirely" (a structural, content-blind exemption, safe only because `PhantomData` is provably
   inert) to "reject only a self-reference reached with no intervening indirection" (a
   representability criterion — the same one Rust's own E0072 uses — that works for content that
   actually gets decoded).
3. An extension of `CodeGen::lift_uvar`'s existing `in_progress` reservation state to also decide
   where a `Box` needs to go, the same way `elaborate.rs`'s `Lifter::lift_ref` does in this sandbox.
4. Verification (not new code, per the sandbox's own finding) that decode-time dispatch
   (`TypedDecoder::Call` / `SimpleLogic::Invoke` / `decoder_map`/`compile_queue`) already handles a
   cycle correctly, since Rust function calls resolve at link time rather than by AST substitution.

Two areas were investigated and confirmed to need **no** work at all: `ViewFormat::ReadArray`/
`fixed.rs`'s fixed-shape analysis (definitionally incompatible with unbounded recursion, already
excluded for that reason) and `Pattern` (structurally incapable of embedding a `Format`/level, so it
can never reintroduce a cycle). Do not re-investigate these; see "Explicitly out of scope" below.

## How to use this document

- Work through the phases **in order** — 1 and 2 are prerequisites for 3, and 3 is a prerequisite
  for the phase-5 capstone test compiling. Phase 4 is a checkpoint, not implementation work.
- Every phase names the doodle-rec file(s)/function(s) to read **first**, as the reference design.
  Read them in full before writing any real-doodle code — do not paraphrase from this plan's
  summaries alone, they are compressed and will omit details that matter.
- Every phase also names a best-effort current location in real `doodle`'s `src/`. These are
  snapshotted from investigation done in September 2026 and **may have drifted** on whatever tree
  this plan is now running against. Phase 0 below is a mandatory reconnaissance pass to confirm or
  relocate every anchor before changing anything.
- `TRIAGE:` callouts mark a genuine design fork the sandbox could not pre-resolve from doodle-rec
  alone, because it depends on specifics of real doodle's current architecture. Resolve each by
  reading the relevant real-doodle code directly; escalate to the user only if the code itself
  doesn't settle it (e.g., a genuine judgment call, not a factual question).
- Follow the verification discipline in "Working method" throughout — it is not optional scaffolding,
  it is how every claim in the sandbox's own design was actually validated.

## Non-negotiable safety constraints

- **`experiments/doodle-rec/*` is read-only reference material for this plan.** Never modify it while
  executing this plan — it is the design source, not the deliverable.
- Every change in Phases 1–3 touches code shared by **every existing format** in `doodle-formats/`
  (`MatchTreeStep::from_format`, `occurs_in`, `CodeGen::lift_uvar` all run on every format, recursive
  or not). Every change must be **strictly additive/permissive for the new recursive case and
  behavior-preserving for everything else.** After each phase, run the full test suite (`cargo
  testall`, not just a spot check) and diff `generated/gencode.rs` before/after regenerating it
  (`cargo cg`) — it must come out byte-identical for all currently-supported (non-recursive) formats.
  A diff there is a regression, not a sign the port is "improving" existing output.
- Do this work on a dedicated branch, with one commit per phase (not one giant commit), each with a
  real "why" message. If a phase's fix turns out to be a no-op after investigation (e.g. Phase 3's
  alias-vs-nominal question, see below), commit that finding as a comment/doc update, don't skip
  recording it.
- `RUST_MIN_STACK=8388608` is already set project-wide (`.cargo/config.toml`) — deep `Format` trees
  can blow the default debug-build stack even without recursion; keep this in mind when a stack
  overflow appears during Phase 1 work, it isn't automatically proof of an unguarded cycle.

## Glossary / file map (doodle-rec → real doodle)

Doodle-rec split its analog of real doodle's `src/lib.rs` monolith into several separate files. Real
doodle's actual locations (as last confirmed) don't mirror that split — don't assume file names
transfer.

| Concept | doodle-rec | real doodle (last confirmed location — **re-verify in Phase 0**) |
|---|---|---|
| `Format`/`Expr`/`FormatModule`, `ItemVar` | `src/lib.rs` | `src/lib.rs` |
| MatchTree construction (`MatchTreeStep::from_format`) | `src/matchtree.rs` | `src/lib.rs` (confirmed: `ItemVar` arm was at `src/lib.rs:2121-2123`, `Match` arm at `src/lib.rs:2261-2267`) |
| Eager grammar validity / left-recursion detection (`Traversal`, `solve_determinations`) | `src/matchtree/determinations.rs` | **does not exist in real doodle** — real doodle has never needed to reject left-recursion because it has never permitted recursion at all. Phase 1 may need to introduce an equivalent, or may be able to fold the check directly into the MatchTree-building guard — see Phase 1's `TRIAGE`. |
| Old eager unification model (`FormatType`, `infer_type`) | `src/lib.rs` | **does not exist** — real doodle only has the `typecheck.rs` bidirectional-unification model; there is no older parallel model to worry about disturbing. |
| Bidirectional unification (`UVar`/`UType`, `occurs_in`) | `src/typecheck.rs` (new parallel module, mirrors real doodle) | `src/typecheck.rs` (`UType` enum at `:330-348`, `occurs_in` at `:1330-1364`, `TypeChecker { constraints, aliases, varmaps, level_vars }`, `infer_var_format_level` uses `level_vars`) |
| Lifting solved types to named Rust types (`Lifter`, `lift_ref`, reservation) | `src/elaborate.rs` | `CodeGen::lift_uvar` (`in_progress: StableMap<UVar, Option<(name, ix, path)>>`, `NameGen::reserve_name`/`commit_reservation`) — locate via LSP/grep, believed to live under `src/codegen/` |
| Decode-time recursive dispatch | `src/decoder.rs` (`Compiler`/`Decoder`/`Program`, `Decoder::CallRec`) | `TypedDecoder::Call`, `CaseLogic::Simple(SimpleLogic::Invoke)`, `decoder_map`/`compile_queue` — confirmed at `src/codegen/typed_decoder.rs:284-356` |
| Toy recursive-codegen prototype (types + decode fns) | `src/codegen.rs`, `src/elaborate.rs::generate_combined_source` | N/A — real doodle's actual production codegen (`ToAst`, `src/codegen/rust_ast/`, `src/codegen/model.rs`) is the real target; there is no toy layer to build, Phase 3 extends the real thing directly |
| `Format::Phantom` (the one self-ref real doodle already has, never decoded) | no analog | `src/lib.rs:1031-1037`; `doodle-formats/src/format/opentype/colr.rs:25` is the one real caller, explicitly warns it's "unsafe if the phantom-format is actually parsed" — leave this construct and its callers untouched |

## Working method (apply throughout every phase)

This mirrors how every claim in the sandbox itself was established — don't skip steps because a fix
"looks obviously right."

1. **Investigate before writing code.** Read the real-doodle target function(s) in full (via LSP —
   `find_definition`/`find_references`/`get_hover`, not Grep, if an LSP server is connected on this
   tree) before assuming its current shape matches what's described here.
2. **Verify empirically, not just structurally.** A fix to `occurs_in` or `MatchTreeStep::from_format`
   needs a real regression test that exercises it — for typecheck/MatchTree changes, a unit test
   directly against the changed function; for the capstone (Phase 5), an actual `cargo run --bin
   doodle -- file ...` decode against real crafted bytes, and (once Phase 3 lands) actually compiling
   the generated Rust with `cargo build` and running it — not just "the generator didn't panic."
3. **Bug-injection-verify every claimed fix.** After a fix passes its new test, deliberately revert
   just the fix (comment out the new guard/branch, not the whole change) and confirm the test fails
   with the *specific* predicted error (a panic message, a compile error, a wrong decoded value) —
   not just "fails somehow." Then re-apply the fix. This is the only way to know the test actually
   exercises the bug rather than passing by coincidence — this exact mistake (a test that passes even
   with the bug reintroduced) happened twice during the sandbox work and was only caught this way.
4. **Check the regression surface, not just the new case.** Run `cargo testall` after every phase.
   Diff `generated/gencode.rs` before/after `cargo cg` — any change there for a pre-existing,
   non-recursive format is a bug in the port, full stop.
5. **Don't assume a doodle-rec finding "just because it was true in the sandbox."** The sandbox's
   `RecVar`/`ItemVar` split and `RecurseCtx` machinery exist *because* doodle-rec addresses recursive
   references relative to a declared batch. Real doodle's `ItemVar` is already absolute-addressed —
   there is no relative/absolute distinction to preserve, so do not port `RecurseCtx`-style
   ctx-threading wholesale; only the *cycle-guard* idea (not the relative-addressing bookkeeping
   built to support it) is the actual thing to port. This is called out again in Phase 1.
6. Ask via a direct question (not silent judgment) only at a marked `TRIAGE:` point, and only if
   reading the code doesn't settle it outright.

---

## Phase 0 — Reconnaissance (run this first, and again if resuming after a gap)

Goal: confirm every location in the glossary above still matches, before changing anything.

1. Confirm `MatchTreeStep::from_format`'s `ItemVar` arm still exists, still has zero cycle guard, and
   is still (or is now) wherever the glossary says.
2. Confirm `TypeChecker::occurs_in`'s current shape: still walks looking for a `UVar` aliased to the
   target, still exempts exactly `PhantomData` and nothing else. List the full current `UType` enum
   — the sandbox's snapshot (`Empty/Hole/ViewObj/Var/Base/Tuple/Record/Seq/Option/PhantomData/Int`)
   may be stale.
3. Confirm `CodeGen::lift_uvar` still has an `in_progress`-style reservation map, and locate its
   current file (the sandbox never nailed this down more precisely than "under `src/codegen/`").
4. Confirm `TypedDecoder::Call`/`CaseLogic::Simple(SimpleLogic::Invoke)`/`decoder_map`/`compile_queue`
   still exist and still look structurally like "reserve a slot before recursing" (per the Phase 4
   description below).
5. Skim `doc/DESIGN.md` again for anything about `MatchTree` that's changed since this plan's glossary
   was written — it's real doodle's own authoritative doc for the lookahead-disambiguation model, and
   Phase 1's design should agree with it, not contradict it.
6. Record any drift found (new file locations, renamed types, structural changes) at the top of this
   document, above this Phase 0 section, before proceeding — that keeps the plan honest for the next
   time it's copied elsewhere.

---

## Phase 1 — MatchTree cycle guard for `ItemVar`

**Read first**: doodle-rec's `matchtree/determinations.rs` (`Traversal`, `Entry` —
`Novel`/`Guarded`/`LeftRecursive`, `guard()`/`insert()`/`escape()`/`reset()`), and doodle-rec's
History item 1 ("`Next::DelayRef` resolution in `MatchTree`-building, was stubbed to unconditional
accept") plus item 2 ("eager static left-recursion / grammar-error validation... fixing `Traversal`'s
cycle detector to distinguish guarded recursion from genuine left recursion"). These are the two
halves of what real doodle needs: (a) a way for lookahead-tree construction to *terminate* on a cycle
instead of recursing forever, and (b) a way to tell a genuinely-unbounded left-recursive cycle (a
real grammar error — would never make progress no matter how deep you look ahead) apart from a
guarded one (safe — the cycle passes through at least one byte-consuming/progress-making step).

**Do not port `RecurseCtx` wholesale.** That machinery exists in doodle-rec to resolve *which
instantiation* of a batch-relative `RecVar` is meant when reached through nested lookahead depth —
a problem created by doodle-rec's relative-addressing scheme. Real doodle's `ItemVar(level)` already
names an absolute format unambiguously; there is no instantiation-identity problem to solve. What
transfers is only the *cycle-guard/termination* idea, not the ctx-threading bookkeeping built to
support relative addressing.

**Target**: `MatchTreeStep::from_format`'s `Format::ItemVar` arm (see glossary/Phase 0 for current
location).

**Design**:
1. Thread a "currently being expanded" set (level indices) through the lookahead-tree construction
   call chain — analogous to `Traversal::open`, but simpler since there's no relative addressing to
   resolve.
2. When `from_format` reaches `ItemVar(level)` and `level` is already in that set: stop recursing.
   Produce a back-edge/deferred node (mirroring doodle-rec's `Next::DelayRef`) that, at lookahead
   *evaluation* time, redirects to the already-open node's tree rather than unrolling a fresh copy —
   this is what gives a self-referential format a finite (cyclic) lookahead structure instead of an
   infinite one.
3. Track whether any byte-consuming progress has happened since `level` was opened. If `ItemVar(level)`
   is re-reached with zero such progress, that's genuine left recursion — a real grammar error, not
   something the guard can silently paper over (it would never actually disambiguate, no matter how
   much lookahead depth is allowed).

`TRIAGE:` **How should the left-recursion case surface as an error?** Doodle-rec built a dedicated
`GrammarError`/panic-at-declare-time mechanism (`determinations.rs`) because its old eager
`FormatType` model needed one anyway. Real doodle has no such mechanism today. Before building a new
one, check: does `MatchTree` construction already return an `Option`/`Result` at the point
`from_format` is called (i.e., does "not disambiguable within bounded lookahead" already have an
existing failure path, the same one an ordinary ambiguous — non-recursive — `Union` would hit)? If
so, genuine left recursion may simply be able to reuse that *existing* failure mode (report "not
disambiguable" — which, for a zero-progress cycle, is even factually true) rather than inventing a
new error type. If MatchTree construction currently assumes infallible success, this needs a real
design decision (introduce fallibility here vs. panic vs. some other existing convention in this
codebase for reporting a bad format definition) — resolve by reading how the nearest existing
"format definition is structurally invalid" case is reported today (e.g. what happens today for an
ambiguous `Union` that can't be disambiguated in bounded lookahead), and match that convention.

**Deliverables**: the guard + back-edge logic in `from_format`'s `ItemVar` arm; a left-recursion
detection path with a real error report; regression tests: (a) a small guarded-recursive format
(e.g. `peano := Union[('S', ItemVar(peano)), ('Z', Unit)]`-shaped) builds a `MatchTree` successfully
and disambiguates correctly; (b) a genuinely left-recursive format (recurses with zero consumed bytes
before hitting itself again) is rejected with the chosen error, not a stack overflow or hang; (c) a
regression test proving *non-recursive* formats are completely unaffected — run this against a large
representative sample of `doodle-formats/`, not just a synthetic case.

**Verification**: bug-injection — temporarily remove the guard, confirm test (a) hangs/stack-overflows
(with a timeout wrapper, don't let this actually hang a test run — see the sandbox's own
`level_vars`-disabling verification for the pattern used there), then restore. Temporarily disable the
left-recursion classification (treat everything as "guarded"), confirm test (b) either hangs or
silently produces a wrong (non-terminating-in-practice) tree instead of a clean error, then restore.

---

## Phase 2 — `TypeChecker::occurs_in` generalization

**Read first**: doodle-rec's `src/typecheck.rs` in full, specifically the `occurs_in`/
`occurs_in_shape` functions and their `indirected: bool` tracking, and Phase 2 Step 1's findings in
`experiments/doodle-rec`'s own history (the `visited: HashSet<(UVar, bool)>` design, and why an
earlier draft that reset `visited` at indirection boundaries stack-overflowed on an unrelated
pre-existing cycle — don't repeat that mistake).

**Target**: `TypeChecker::occurs_in` (real doodle, `src/typecheck.rs`, confirmed location as of this
writing `:1330-1364`; re-confirm in Phase 0).

**Design** (already fully designed and tested in the sandbox — this is closer to a direct port than
Phases 1/3):
- Replace the current logic (skip `PhantomData` entirely, walk everything else looking for a `UVar`
  aliased to the target) with an `indirected: bool` flag that starts `false` and flips to `true` the
  instant the walk crosses a representability boundary (a `Tuple`/`Record`/`Seq`/`Option` field, or
  whatever real doodle's analog of a sum-type/enum boundary is at the `UType` level — see `TRIAGE`
  below), but stays `false` across a plain `Var`-forwarding dereference.
- Reject a `target == v` match only when reached with `indirected == false`. This is exactly Rust's
  own E0072 criterion (`type X = Box<X>` illegal, `enum X { Y(Box<X>) }` fine) — representability, not
  "is this construct provably inert," is now the criterion, which is what makes it safe to also cover
  self-references that *are* meant to be decoded (unlike the old `PhantomData`-only exemption, which
  was safe only because that specific content is never walked by a decoder at all).
- Thread the visited-set through the *whole* walk without resetting it at indirection boundaries (the
  sandbox's corrected design, not its first draft).

`TRIAGE:` **Full enumeration of "representability boundary" `UType` variants.** The sandbox's list
(`Tuple`/`Seq`/`Option`/`Union`) doesn't map cleanly onto real doodle's confirmed `UType` variant list
(`Empty/Hole/ViewObj/Var/Base/Tuple/Record/Seq/Option/PhantomData/Int` — no `Union` variant listed).
Before implementing, determine where format-level `Union`/sum-type-ness actually shows up in
`UType` — via `Record`, via the `constraints`/`aliases` machinery producing a shared type across
branches rather than a tagged `UType` variant, or something else — and include whatever construct is
the real analog of an enum/sum boundary in the indirection-boundary list. Missing one here would
under-reject (accept a genuinely unrepresentable cycle) rather than over-reject, so get this right
before moving on; a targeted regression test using each boundary kind is the way to check, not
inspection alone.

`TRIAGE:` **Does `PhantomData` still need special handling?** Check whether the new indirection-based
logic naturally handles `PhantomData` correctly on its own (plausible — walking into `PhantomData`'s
inner content and reaching `target` un-indirected should now correctly still get exempted only if
genuinely indirected, same as everywhere else) or whether `PhantomData`'s "never actually walked by a
decoder" property means it still needs to be skipped unconditionally for a *different* reason
(performance, or because its inner content isn't a real `UType` graph node in the same sense). Resolve
by reading `PhantomData`'s current construction/usage in `typecheck.rs`, not by assumption.

**Deliverables**: the generalized `occurs_in`; regression tests mirroring the sandbox's own test
names/shapes (`direct_self_alias_with_no_indirection_is_rejected`,
`an_unrelated_pre_existing_cycle_does_not_hang_occurs_check`, plus a new
`self_reference_through_tuple_is_accepted` proving the actual unblock); a before/after test proving
every currently-accepted (non-recursive) format's typecheck result is byte-identical.

**Verification**: bug-injection — disable the rejection branch, confirm a direct self-alias test now
passes when it shouldn't; make each indirection-boundary variant pass `indirected` through unchanged
instead of forcing `true`, confirm that specific test (and only tests genuinely relying on that
boundary) fails.

---

## Phase 3 — `CodeGen::lift_uvar` Box-placement + nominal-type check

**Read first**: doodle-rec's `src/elaborate.rs` in full — `Lifter::lift_ref`'s `in_progress` check
(Box decision falls out of the *same* state that prevents runaway recursion, no separate `Ref`-style
check needed), and the "bare type alias cannot close a cycle" finding (`E0391`, verified via a bare
`rustc` probe) plus how `elaborate.rs` avoids it (never emits a plain alias for a batch member, always
a real nominal `struct`/`enum`).

**Target**: `CodeGen::lift_uvar` (real doodle — location per Phase 0/glossary).

**Design**:
- Extend the existing `in_progress: StableMap<UVar, Option<(name, ix, path)>>` reservation (already
  present for name-reservation/termination purposes) to also answer "is the `UVar` currently being
  referenced an ancestor still under construction?" at the point a reference is about to be lifted to
  a concrete Rust type reference. If yes, that's exactly where a cycle closes — wrap in `Box::new(...)`
  (construction site) / `Box<...>` (type site) there; nowhere else needs one, by construction.

`TRIAGE:` **Does real doodle's `CodeGen` already always emit nominal (struct/enum) declarations for
every named type, or can it emit a bare alias for a single-field/tuple-shaped type?** This determines
whether the E0391 bug doodle-rec found is even reachable in real doodle. Check `CodeGen`'s type-decl
emission path directly (grep/LSP for wherever it decides between a `struct` item and a `type ... = ...`
alias item). If real doodle already always emits nominal types (plausible for a mature production
codegen backing real OTF table structs), this half of Phase 3 is **verification only** — write a test
proving a same-shape-as-doodle-rec's-uncompilable-case (a two-member all-tuple recursive cycle with no
enum anywhere on either cycle) already compiles today once Phases 1–2 land, and record that finding;
do not add speculative alias-avoidance logic for a bug that may not exist here. If it *can* emit a
bare alias for some type shapes, port the "always nominal for anything reachable via `Box`-eligible
self-reference" rule narrowly — don't force every alias in the whole codegen to become nominal, only
ones on an actual cycle.

**Deliverables**: extended `in_progress`-driven Box decision in `lift_uvar`; a peano-shaped
(self-recursive) and a ping/pong-shaped (mutual recursion, proving Box placement is determined by
traversal/reservation order, not naive "always box a named reference") test, each actually compiled
(`cargo build`, or the equivalent real-doodle codegen round-trip test convention — see `tests/
runtime_repeat/`, `tests/permit_state_error/` for the existing precedent of a self-contained
codegen-fixture test in this repo) and *run* against real decoded input, not just structurally
asserted.

**Verification**: bug-injection — disable the Box insertion, confirm the generated Rust fails to
compile with `E0072` (recursive type has infinite size) on the mutual-recursion case specifically at
the un-boxed back-edge, matching the sandbox's own confirmed result for the same shape.

---

## Phase 4 — Confirm decode-time dispatch (expected: no code changes)

**Read first**: doodle-rec's Step 3 finding in full ("CaseLogic for auto-recursive references
(RESOLVED, no code needed)") — the conclusion was that `TypedDecoder::Call` compiling to
`CaseLogic::Simple(SimpleLogic::Invoke(ix, ...))`, backed by `decoder_map`/`compile_queue`
(`src/codegen/typed_decoder.rs:284-356`), is *structurally identical* to doodle-rec's own
`Compiler::compile_queue` (which needed a real fix in the sandbox — the `level_slot` batch-reuse bug,
Step 2 of the extension plan) — except real doodle's version, being production code with heavier
existing test coverage, was found to already reserve a slot the instant a level is discovered, before
its body is walked, so a cycle back through `Invoke(ix)` never needs slot `ix`'s content to already
exist. Since Rust function calls resolve at link time, not AST substitution, this needs no
special-casing at all.

**This phase should not require writing new production code.** Its job is to write the test that
actually proves the claim on real doodle's real mechanism, now that Phases 1–3 make a genuinely
recursive format reach this code path for the first time. If the test reveals the claim doesn't
actually hold on the current tree (e.g. `compile_queue`'s reservation-before-walk property has
regressed or never quite matched the sandbox's read of it), that's new, real information — stop and
diagnose it as its own fix, don't force the plan's prediction through.

**Deliverables**: a decode-time regression test using the Phase-1/2/3 recursive test format, run
through the actual decoder (not just codegen) — `cargo run --bin doodle -- file ...` against real
crafted recursive-structure bytes, asserting correct decoded values at real recursion depth ≥ 2 (depth
1 alone can't distinguish "recursion works" from "got lucky with a degenerate case").

---

## Phase 5 — Capstone integration test

Goal: one real, small, genuinely self-referential format, proven through the *entire* pipeline this
plan touches — typecheck, MatchTree, interpreted decode, and full production codegen — the same
end-to-end bar doodle-rec held itself to with peano/ping-pong, but now against real doodle's actual
production infrastructure instead of a toy prototype.

Precedent for where this lives: `tests/runtime_repeat/` and `tests/permit_state_error/` are already
"self-contained mini codegen fixtures... exercising specific codegen edge cases outside the main
`generated/` crate" (per root `CLAUDE.md`) — each with its own `mod.rs`/`codegen_tests.rs`/
`api_helper.rs`. A new `tests/recursion/` (or similarly named) fixture following that exact convention
is the natural home; it keeps a fundamentally new capability's test isolated from the main
`generated/gencode.rs` artifact rather than entangling it with every existing OTF/PNG/etc. format.

Recommended shape: two small formats mirroring the sandbox's own — one self-recursive (peano-style
depth counter) and one two-member mutually-recursive pair (ping/pong-style) — since the sandbox found
these exercise genuinely different code paths (self-recursion alone doesn't prove Box-placement
ordering logic works across *distinct* types the way mutual recursion does).

**Deliverables**:
- The two format definitions (wherever `tests/recursion/`'s own `mod.rs` defines its `FormatModule`
  content, per the existing fixture convention).
- A typecheck test: the module registers without an `occurs_in` rejection.
- A MatchTree/interpreted-decode test: `cargo run`-equivalent decode of real crafted bytes at depth
  ≥ 2, both self- and mutually-recursive cases, plus a rejection case for malformed input (proves the
  guard/back-edge in Phase 1 doesn't just avoid hanging, it still disambiguates correctly).
- A codegen test: generate real Rust source via the actual production codegen path (not a toy), and
  either (a) if this repo already has infrastructure to compile+run a generated fixture in-process
  (check `tests/runtime_repeat/codegen_tests.rs` for the existing convention), reuse it; or (b) a bare
  `rustc`/`cargo build` round-trip, matching the sandbox's own verification method, if no such
  in-repo convention exists yet.

**Verification**: this phase's tests *are* the verification for Phases 1–4 taken together — if they
pass and the bug-injection checks from each individual phase already passed, the port is functionally
complete.

---

## Phase 6 — Full-suite verification and wrap-up

1. `cargo fmt -- --check` (project-wide, not just changed files).
2. `cargo testall` (`cargo test --workspace --exclude smallsorts --exclude analytic-engine --exclude
   analytic-parser`) — must be fully clean, not just "no new failures."
3. `cargo build` (full workspace).
4. `cargo cg` (regenerate `generated/gencode.rs`) and diff against the pre-port version — must be
   byte-identical. If it isn't, something in Phases 1–3 changed behavior for an existing,
   non-recursive format; treat as a regression to fix, not an acceptable side effect, before
   considering any phase done.
5. Update or add to root-level docs where this plan's changes are now load-bearing for future readers:
   `doc/DESIGN.md` (MatchTree's cycle-guard behavior is now part of the model it describes),
   `TYPECHECKER.md` (the `occurs_in` exemption rule changed — TYPECHECKER.md is described in root
   `CLAUDE.md` as "a guide to extending the type-inference engine," this belongs there), and
   `READARRAY_AUDIT.md` if Phase 0/investigation surfaces anything new about the fixed-shape
   interaction beyond what's already confirmed as a non-issue.
6. Fill in the progress table below with final commit hashes for traceability.

---

## Explicitly out of scope — do not attempt as part of this plan

- **Parametric `ItemVar` refs** (an `Expr::Var` + named-parameter mechanism). Confirmed in the
  sandbox to be a general language feature with nothing structurally recursion-specific about it —
  real doodle already has `Expr::Var`/`define_format_args`, so this isn't even a gap on the real side;
  nothing to port.
- **`WithRelativeOffset`'s `MatchTree` opacity.** A legitimate, deliberate, pre-existing design point
  in real doodle (its own `Self::accept() // FIXME`) — inherit it as-is, do not attempt to fix it as
  part of this port.
- **`ViewFormat::ReadArray`/`fixed.rs`'s fixed-shape analysis.** Confirmed definitionally incompatible
  with unbounded recursion (no static byte-width) — already correctly excludes recursive formats for
  that reason. No change needed; if this port somehow causes a fixed-shape analysis to accept a
  recursive format, that is itself a new bug to flag, not a sign more porting work is needed here.
- **`Pattern`.** Confirmed structurally incapable of embedding a `Format`/level — cannot reintroduce a
  cycle through any path independent of the one Phase 1 already covers. No change needed.
- **Full equi-recursive `FormatType::unify`.** N/A on the real side — real doodle has no `FormatType`/
  eager-unification model to begin with; only the bidirectional `typecheck.rs` model exists there.

---

## Progress tracking

| Phase | Status | Commit(s) | Notes |
|---|---|---|---|
| 0 — Reconnaissance | Not started | | |
| 1 — MatchTree cycle guard | Not started | | |
| 2 — `occurs_in` generalization | Not started | | |
| 3 — `CodeGen` Box placement | Not started | | |
| 4 — Decode-time confirmation | Not started | | |
| 5 — Capstone integration test | Not started | | |
| 6 — Full-suite verification | Not started | | |
