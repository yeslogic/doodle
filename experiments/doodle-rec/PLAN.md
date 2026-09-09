# PLAN: Porting `doodle-rec`'s recursion model into `doodle` (`src/`)

Written 2026-09-07 on branch `recursion-model`, against `experiments/doodle-rec` as of commit
`8666c57`. This document is meant to be **portable**: if `experiments/doodle-rec/*` and this file
are copied onto a different working tree (a different feature branch, with `src/` that has evolved
but is still recognizably the same architecture as described in the root `CLAUDE.md`), a fresh agent
with no memory of the sandbox work should be able to execute it end to end, pausing only at the
`TRIAGE:` checkpoints called out explicitly below.

**Disclaimer added 2026-09-09** (per the user, after a real case where a written requirement below
turned out not to reflect their actual intent): this document was drafted largely by an LLM. Its
stated requirements, constraints, and "non-negotiable" language are LLM-authored scaffolding, not
automatically an accurate reflection of what the user (the actual programmer directing this work)
intends - they were never individually reviewed and endorsed line-by-line. **Only requirements
written in ALLCAPS have been explicitly confirmed by the user as genuinely binding.** Everything
else is a draft default. If work is ever blocked by a non-ALLCAPS requirement or limitation stated
here, stop and ask the user before assuming it's intentionally binding, rather than treating it as
settled - do not silently comply with it, and do not silently override it either. If the user
confirms it should hold, rewrite it in ALLCAPS at that point (updating this disclaimer's own
description of what's confirmed is not required, the convention itself covers it); if they disagree
with it, remove it entirely rather than leaving it as dead text.

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
  or not). EVERY CHANGE MUST BE STRICTLY ADDITIVE/PERMISSIVE FOR THE NEW RECURSIVE CASE AND
  BEHAVIOR-PRESERVING FOR EVERYTHING ELSE. After each phase, RUN THE FULL TEST SUITE (`cargo
  testall`, not just a spot check) AND DIFF `generated/gencode.rs` BEFORE/AFTER REGENERATING IT
  (`cargo cg`).
  **(Revised 2026-09-09, per the user, after Phase 4's Finding B fix produced a real, confirmed-benign
  `gencode.rs` diff — see the disclaimer at the top of this document.)** The original form of this
  requirement — that the diff must come out **byte-identical**, full stop, with any diff automatically
  treated as a regression — was an LLM-authored overstatement the user did not endorse at that
  strength: a change can legitimately improve `codegen`'s existing decoder-sharing (`decoder_map`
  merging two previously-accidentally-distinct decoders for the same job into one, causing pure
  renumbering/rehashing with no behavior change) as a side effect of a real fix, and that is not a
  regression. THE ACTUALLY-BINDING RULE IS: A `cargo cg` DIFF WHOSE ONLY CONTENT IS THE KIND OF CHANGE
  EXPECTED FROM ADDING/CHANGING A FORMAT DEFINITION (NEW DECODERS APPEARING, AND THE CONSEQUENT
  RENUMBERING/REHASHING OF EVERYTHING AFTER THEM) — INCLUDING A CONFIRMED-BENIGN DEDUP/MERGE LIKE
  FINDING B'S — IS ACCEPTABLE TO COMMIT. ANY DIFF THAT ISN'T OBVIOUSLY THAT SHAPE (A DECODER'S ACTUAL
  LOGIC CHANGING FOR AN UNRELATED EXISTING FORMAT, A TYPE CHANGING, CONTROL FLOW CHANGING, ETC. —
  ANYTHING WHERE "IS THIS STILL BEHAVIOR-PRESERVING" ISN'T SELF-EVIDENT FROM THE DIFF ALONE) MUST BE
  REPORTED TO THE USER FOR REVIEW BEFORE BEING COMMITTED, NEVER SILENTLY SUPPRESSED, ACCEPTED, OR
  COMMITTED AS-IS ON THE ASSUMPTION IT'S FINE.
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

## Phase 0 findings (recorded 2026-09-07, against current tree on branch `archaephyrryx/doodle-recursion-graft`)

Anchors confirmed accurate unless noted. Drift/new information found:

1. `Format::ItemVar` arm: confirmed unchanged, exactly at `src/lib.rs:2121-2123`
   (`Self::from_format(module, module.get_format(*level), next)`, zero guard). This is a plain
   Rust-recursive tail call with **no depth bookkeeping of its own** — it is entirely independent of
   `MatchTreeLevel::grow`'s `MAX_DEPTH: usize = 80` BFS-level counter (`src/lib.rs:2448-2456`). A
   self-recursive format with no byte-consuming step before revisiting its own level will stack-overflow
   *inside this single `from_format` call*, never even reaching `grow`'s depth-limited loop. Confirms
   the plan's Phase 1 characterization exactly, and clarifies the guard belongs in `from_format` itself,
   not e.g. as a `grow`-level depth check.
2. `TypeChecker::occurs_in`: confirmed at `src/typecheck.rs:1333-1366` (glossary said 1330-1364, minor
   line drift, same function/logic, still exempts exactly `PhantomData`). `UType` enum confirmed at
   `src/typecheck.rs:330-348`, variant list **exact match** to the glossary's snapshot
   (`Empty/Hole/ViewObj/Var/Base/Tuple/Record/Seq/Option/PhantomData/Int`) — no drift here at all.
3. Found a second, unrelated `UType`-adjacent type: `NUType`/`NVType` in `src/typecheck/inference.rs`.
   This belongs to the numeric/arithmetic-embedding extension (`crate::numeric`), a completely separate
   system from the `Format`/`Expr` type system this plan targets — not drift, but flagged so a future
   reader doesn't confuse the two modules by name similarity.
4. `CodeGen::lift_uvar`: confirmed at `src/codegen/mod.rs:241`, `in_progress` field at `:107-108`
   exactly as described (`StableMap<UVar, Option<(Label, usize, PathLabel)>, FxHash>`). Also found a
   companion field `recursion_lt_needed: HashMap<UVar, bool>` not mentioned in the plan, and — more
   importantly — the existing self-reference-handling code at `lift_uvar`'s `Expansion::Record` arm
   carries an explicit comment asserting the self-reference case "always" occurs "via a
   `Format::Phantom`, the only place this can validly occur." **This is the load-bearing assumption
   Phase 3 needs to generalize** — it's currently hard-assumed to be Phantom-only, and that comment
   text is the concrete thing to revisit/update when extending to genuine (non-Phantom) recursion.
5. Drift: `CaseLogic`/`SimpleLogic` **enum definitions** actually live in `src/codegen/mod.rs`
   (`CaseLogic` at `:3046`, `SimpleLogic` at `:3106`), not `src/codegen/typed_decoder.rs` as the
   glossary implied. The `decoder_map`/`compile_queue`/reservation-before-walk *logic* (`GTCompiler`,
   `queue_compile`, `compile_gt_format`'s `FormatCall` arm) is correctly in
   `src/codegen/typed_decoder.rs`, spanning roughly `:283-410` (glossary's `:284-356` was close but
   slightly stale).
6. **Phase 4's core claim independently confirmed**, ahead of schedule: `queue_compile`
   (`src/codegen/typed_decoder.rs`) pushes a placeholder decoder (`TypedDecoder::Fail`) into
   `self.program.decoders` and the `FormatCall` arm inserts `(level, next) -> n` into `decoder_map`
   *before* that queue item is ever popped/compiled. A self-referential `FormatCall` reaching the same
   `(level, next)` key therefore resolves immediately via the `decoder_map` cache to
   `TypedDecoder::Call(gt, n, args)`, without ever re-entering `compile_gt_format`. Matches the plan's
   prediction exactly — pending Phases 1-3 actually landing so a recursive format can reach this path
   for the first time.
7. **Phase 1's TRIAGE resolved** (no user escalation needed): `MatchTree::build`
   (`src/lib.rs:2448`) already returns `Option<MatchTree>`. All four call sites
   (`src/decoder.rs:731,782,799,837` and `src/codegen/typed_decoder.rs:450,505,526,571`) already share
   one established failure convention: `None` → `Err(anyhow!("cannot build match tree for {}", ...))`.
   `src/format.rs:491` additionally uses `.is_none()` as a boolean "is this union ambiguous" check.
   **Decision: Phase 1's left-recursion case should reuse this exact existing convention** (surface as
   a `None` from `build`/`grow`, propagating to the same `anyhow!` error every other undisambiguable
   union already hits) rather than inventing a new error type or panic path.
8. `TypeChecker::infer_var_format_level`/`level_vars` reservation-before-recursion pattern confirmed
   exactly as described at `src/typecheck.rs:999-1017`, reinforcing this as a third independently-proven
   "reserve a placeholder before recursing" instance already live in the codebase.

No other drift found; `doc/DESIGN.md`'s `MatchTree` description is unchanged and contains nothing about
`ItemVar`/cycles to reconcile against yet (expected — that's what Phase 1 adds).

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

## Phase 1 design note (recorded 2026-09-07, after implementation)

Two things worth recording for whoever reads this next, beyond what the TRIAGE resolution in
"Phase 0 findings" already settled:

1. **`reject()`-on-cycle vs. total-failure-propagation.** Before implementing, direct analysis
   confirmed the guard can *only* ever fire on a genuinely zero-progress self-reference (any
   `Format::Byte` along the way already breaks eager descent into a deferred `Next`, evaluated
   fresh - with an empty guard - at the next lookahead depth), so there is no false-positive risk
   either way. The remaining question was what a fired guard should *do*. A local `reject()` (the
   doodle-rec `Next::DelayRef` precedent) would let `MatchTree::build` quietly succeed with the
   cyclic branch simply un-selectable - and neither `typecheck`'s `occurs_in` (a different,
   structural property - `ItemVar(X)`'s type is just the same `UVar` as `X` itself, no new
   structural wrapping to flag) nor either compile path's `decoder_map`/`compile_queue`
   reservation (both terminate by design regardless of whether the self-reference is guarded or
   genuinely left-recursive) would catch it later - so the failure would only ever surface as an
   infinite loop the first time someone actually decodes a byte with it. `MatchTreeLevel::grow`
   is the *only* pass with the right shape (eager, byte-by-byte descent) to notice "there's no
   byte to look at here, ever," so it was made to `return None` the instant `guard.detected` fires
   - reusing the `Option`/`anyhow!("cannot build match tree for {}", ...)` convention already used
   by every `MatchTree::build` call site (`src/decoder.rs`, `src/codegen/typed_decoder.rs`).
2. **`TypedFormat::FormatCall`'s guard is likely unreachable today.** For symmetry/correctness
   (`guard.open` should reflect "currently expanding" precisely, not just "eventually correct
   after one extra unwind through the untyped body") the same insert-before-recurse/remove-after
   was added to `from_gt_format`'s `FormatCall` arm, mirroring `from_format`'s `ItemVar` arm.
   However: `MatchTree::build`'s only parameter is `branches: &[Format]` (untyped - codegen's own
   `TypedFormat::Union` arm erases branches via `.into()` before calling it), and `find_references`
   confirmed nothing outside this one `impl<'a> MatchTreeStep<'a>` block calls `from_gt_format`/
   `from_mt_format` at all - so the typed path appears to be dead code via any currently-live entry
   point. It was fixed anyway (cheap, and correct-by-construction beats "correct by lucky
   unreachability"), but wasn't given its own dedicated unit test/bug-injection round for that
   reason - flag this if/when a live typed `MatchTree`-building entry point is ever wired up.

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

## Phase 1.5 — Batch construction API + three more unguarded-recursion bugs (added 2026-09-07, mid-port)

Not in the original plan at all. Surfaced from a direct, explicit question from the user partway
through Phase 1's follow-up work: *"what step of the plan would the actual `RecVar` and
batch-definition model land in?"* Investigating the answer found that real doodle has **no way to
construct a genuinely self-referential (non-`Phantom`) format through its public API at all** -
`define_format_args_views` type-checks a format *before* the level it would occupy exists, so any
out-of-bounds `Format::ItemVar` self-reference is rejected outright; the only existing escape hatch,
`define_format_phantom_rec_args_views`, is hard-wired to require `Format::Phantom` wrapping. Without
fixing this, Phase 3 onward literally cannot construct the peano/ping-pong test formats their own
deliverables call for. This gap was never named as a phase because the original plan's glossary
incorrectly asserted real doodle has no eager, pre-typecheck.rs type-inference pass at all
("`FormatType`/`infer_type` does not exist") - it does: `FormatModule::infer_format_type`
(`src/lib.rs`), a completely separate, earlier pipeline stage from `typecheck.rs`'s bidirectional
`UType` model that Phase 2 targets. Confirmed via the plain interpreter's own imports
(`src/decoder.rs` never touches `typecheck::TypeChecker`) that `infer_format_type` is *also* what
gates the interpreter path, independent of codegen.

**Design decision** (see conversation, not re-derived here): rather than avoid porting anything
resembling doodle-rec's `Format::RecVar`/batch-relative addressing (the original plan's stance,
based on "real doodle's `ItemVar` is already absolute, so there's nothing to port"), the user
explicitly overrode that guidance after weighing the tradeoff directly - PLAN.md is LLM-authored and
not automatically authoritative over the user's own judgment; this is the first recorded instance of
that override, per the user's standing instruction to flag (not silently follow) any PLAN.md
requirement that conflicts with their actual preference. Chosen shape ("Option C" in-conversation):
`Format::RecVar(usize)` **is** a real variant (batch-relative index, `0` = self), giving
`FormatModule::define_format_rec_batch(formats: Vec<(Label, Format)>) -> Vec<FormatRef>` doodle-rec's
own flat, up-front-values signature - but it is construction-time sugar only. Every `RecVar`
occurrence is rewritten to a real, absolute `Format::ItemVar` (via the new `Format::substitute_rec_var`)
before the format is ever installed into the module, so no other pass ever sees one at runtme; every
other exhaustive `match` over `Format` in the codebase (16 sites, found and fixed via the compiler's
own exhaustiveness errors after adding the variant - `cargo build` enumerates them completely, so
none can be silently missed) just carries a boilerplate `Format::RecVar(_) => unreachable!(...)` arm.
This keeps the "avoid duplicating every phase's cycle-handling for a second variant" property the
original plan's stance was actually protecting, while matching doodle-rec's ergonomics exactly.
`define_format_rec_batch` itself reserves all batch levels up front (placeholder `Format::EMPTY` /
`ValueType::Any`, mirroring `define_format_phantom_rec_args_views`'s own pattern), then rewrites and
installs each real body, then runs `infer_format_type` per member in `formats`-order - no
occurs-tracking needed here (unlike doodle-rec's own `FormatType::Ref`/`visited` machinery) because
`infer_format_type`'s `ItemVar` arm never recurses into a referenced level's body, it only reads
`format_types[level]` directly; a still-provisional sibling is read as `ValueType::Any`, which
`ValueType::unify` always accepts, giving the same accepted order-dependent imprecision doodle-rec's
own design already lives with.

**Three further, independent unguarded-recursion bugs found** while actually exercising the new API
end-to-end for the first time (each confirmed by direct, isolated repro before fixing, each fixed
with a lightweight `&mut HashSet<usize>` "currently open" guard - no `MatchTree`-style total-failure
propagation needed, since none of these have a "silently paper over a grammar defect" failure mode;
a cycle here just means the conservative/correct answer, not an error):

1. **`Format::depends_on_next`** (`src/format.rs`) - confirmed, in isolation, to stack-overflow on a
   self-recursive format with zero `MatchTree`/compile machinery involved. This sits on the hot path
   of *both* `decoder::Compiler::compile_format` and `codegen::GTCompiler::compile_gt_format`'s
   `ItemVar`/`FormatCall` arms - unconditionally, before either one reaches the `MatchTree`-building
   code Phase 1 fixed - so no self-referential format could be compiled by either pipeline before
   this fix, regardless of Phase 1 already being in place. Fixed by threading `open` through a new
   private `depends_on_next_open`/`union_depends_on_next_open`, returning `true` (the safe
   default - a missed decoder-sharing optimization, not a soundness bug) on re-entry.
2. **`Format::match_bounds`** and **`Format::lookahead_bounds`** (`src/format.rs`) - identical
   unguarded shape, same file. Fixed the same way; on re-entry, `Bounds::any()` is returned directly -
   not a conservative fallback but the *correct* answer, the same value `Format::Repeat`'s own
   unbounded repetition already returns.
3. **`decoder::Compiler::compile_format`'s and `codegen::typed_decoder::GTCompiler::compile_gt_format`'s
   `Tuple`/`Sequence` arms** - a completely different bug, unrelated to the `ItemVar`-guard family
   above: both unconditionally wrap `next` in `Next::Sequence(<remaining fields>, next)` on *every*
   field, including the last one, where `<remaining fields>` is empty. Semantically a no-op (an empty
   `Next::Sequence` unwraps to nothing downstream), but for a self-referential format it means `next`
   gained one more structurally-distinct wrapper layer on every re-entry, defeating `decoder_map`'s
   `(level, next)` memoization outright - not a stack overflow but an unbounded, ever-growing
   `compile_queue` (confirmed via direct `log::trace!` instrumentation showing `next` growing a new
   `Sequence(Untyped([]), ...)` layer on every single `ItemVar` encounter, never once hitting the
   cache). Fixed by using `next` directly when no fields remain, in both `decoder.rs` and
   `codegen/typed_decoder.rs`'s `Tuple`/`Sequence` arms.

**Deliverables**: `Format::RecVar` + `Format::substitute_rec_var` (`src/format.rs`);
`FormatModule::define_format_rec_batch` (`src/lib.rs`); the three bug fixes above; three new
end-to-end regression tests in `src/lib.rs`'s `mod test`, all going through the *real* public API and
the actual interpreter (not hand-poked `FormatModule` fields, unlike Phase 1's own tests) -
self-recursive peano and mutually-recursive ping/pong both decoding real bytes correctly at depth ≥ 2,
plus a left-recursion-still-rejected case reached via `decoder::Compiler::compile_program` rather than
calling `MatchTree::build` directly. `cargo testall` clean; `cargo cg` byte-identical.

**Known, deliberately deferred**: the same unguarded-`ItemVar`-recursion shape also exists in
`Format::is_ascii_char_format`/`is_ascii_string_format` (`src/format.rs`) and several `output/`-module
functions (`flat.rs`'s `check_covered`/`write_flat`; `tree.rs`'s `is_implied_value_format`,
`is_atomic_format`, `try_as_record_with_atomic_fields`, `unwrap_itemvars`,
`compile_decoded_parsedvalue`, `compile_decoded_value`). None of these sit on the core decode/codegen
path - they're naming heuristics and `doodle format --output debug`-style pretty-printing/coverage
tooling - and none block Phases 3-5's own deliverables, so they were left unfixed and are just
flagged here as the same bug class, for whoever eventually exercises a recursive format through one
of those tools.

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

## Phase 3 & 4 findings (recorded 2026-09-08, session paused here — uncommitted)

Phase 3's TRIAGE was resolved directly from code, same as prior phases: real doodle's `lift_uvar`
never emits a bare `type X = Y;` alias for anything — `Record`/`Union` always become nominal
`struct`/`enum`; `Tuple`/`Seq`/`Option`/`PhantomData` become anonymous inline compound types. So the
E0391 bug doodle-rec found is not reachable here in that form; that half of Phase 3 really was
verification-only, as the TRIAGE's "already-nominal" branch predicted. What *was* missing turned out
to be substantially larger than "extend `in_progress` to add `Box`" — four more layers of work were
needed, in order, each surfaced only by actually running a genuinely recursive format through the real
pipeline (not just reading the code):

1. **A `Box` type didn't exist anywhere in real doodle's codegen model.** Added `CompType::RecBox(T)`
   (`src/codegen/rust_ast/mod.rs`), threaded through `ToFragment`, `lt_param`/`alpha_convert_lifetime`,
   `MemSize`/`CanOptimize`/`CopyEligible` (`analysis.rs`), `HeapOptimize` (`analysis/heap_optimize.rs`),
   `Rebindable` (`rebind.rs`), and the `OwnedKind`-resolution `solve_comp_type` (`resolve.rs`) — every
   site found via `cargo build`'s own exhaustiveness errors, same technique as Phase 1.5's `RecVar`.
   Every arm added is a **non-recursive, constant-answer** case (pointer size, 1 niche, never `Copy`,
   `Noop` for heap-strategy) — this is what actually fixes the `CopyEligible::copy_hint` infinite
   recursion confirmed by direct reproduction (a genuinely self-referential `RustTypeDecl` graph, with
   no `Box` marking the boundary, sent `copy_hint`'s existing `Named`/`LocalDef` dereferencing into an
   unbounded loop — `analysis.rs:630`'s own pre-existing comment, *"this can be circular if we are not
   careful, but we don't expect circularity in practice"*, named the exact assumption this port breaks).
2. **`Box` insertion at the value-construction sites was never done** — Steps 1–3 only fixed the type
   *declaration* side; the code that actually builds a decoded value (`CodeGen::translate`,
   `SequentialLogic::AccumTuple` construction) never learned to wrap a sub-result in `Box::new(...)`
   when its target position was `RecBox`-wrapped, so a self-recursive format's generated Rust flatly
   didn't compile (`peano::S(arg0, arg1)` where the field type says `Box<peano>` but `arg1: peano`).
   Fixed by mirroring the existing `RustExpr::wrap_some`/`GenExpr::WrapSome` machinery exactly:
   `RustExpr::wrap_box`/`GenExpr::WrapBox`/`GenBlock::wrap_box_final_value`/`DerivedLogic::WrapBox`, and
   a `CodeGen::box_wrap_if_needed(cl, ty)` helper called at both `TypedDecoder::Variant`'s
   tuple-arity-match construction and `TypedDecoder::Tuple`'s own bare-tuple construction — the same
   spot a pre-existing `// FIXME - ... we also want to selectively box the elements` comment in
   `SequentialLogic::AccumTuple`'s `to_ast` (`src/codegen/mod.rs`, near the `AccumTuple` match arm) had
   already flagged as unaddressed, unrelated to this port. Verified: peano now generates and (via a
   `cargo cg`-style round-trip) compiles correctly; `cargo testall` clean, `cargo cg` byte-identical
   throughout — including catching and fixing a real regression along the way, where the naive
   `RecBox`-insertion check also fired for real doodle's one pre-existing self-reference
   (`Format::Phantom`, `doodle-formats/src/format/opentype/colr.rs`'s `paint`), which never needed
   boxing (`PhantomData<T>` never stores a `T`). Fixed with a `CodeGen::in_phantom_context: bool` flag,
   set for the whole subtree under `Expansion::PhantomData` (not just its immediate child), suppressing
   `RecBox` insertion throughout; bug-injection confirmed the exact same regression reappears with the
   suppression disabled.
3. **`Expansion::Tuple` needed its own cycle-termination guard**, same as `Seq`/`Option` — confirmed by
   direct reproduction that a pure-`Tuple` mutual cycle (no `Union` anywhere, e.g. two batch members
   that are each just `Tuple[Byte, ItemVar(other)]`) stack-overflows `lift_uvar` itself, since only
   `Record`/`Union` had `in_progress`-based termination before this port. **Scope explicitly narrowed by
   the user, overriding this document's own original framing**: rather than build promotion-to-a-real-
   nominal-struct support for a self-referential `Tuple` (the shape doodle-rec's own `elaborate.rs`
   handles by giving every batch member a name), `Expansion::Tuple`/`Seq`/`Option` now just detect a
   genuine self-hit and reject it loudly (`unreachable!("... not yet supported by codegen")`) rather
   than hang or produce broken output — mirroring the existing convention elsewhere in this file for
   out-of-scope shapes (`"unexpected result in structural type"` etc.). Neither peano nor ping/pong
   needs this path (both close their cycle through `Union`), so nothing about the two target capstone
   shapes is blocked by leaving it unimplemented; if a real format ever needs it, its actual
   requirements can be worked out against that concrete case instead of speculatively now.
4. **A deeper, still-unresolved inconsistency**, found while actually trying to build the Phase 4/5
   capstone fixture (below) — **not fixed, left for a future session**:
   - **Finding A — mixed shapes cause silently-inconsistent `Box` decisions.** A `Tuple`-shaped batch
     member that merely *contains* a reference to something recursive (not itself the cycle point, e.g.
     `pong := Tuple[Byte, ItemVar(ping)]` where `ping` is the one that's actually self-referential) gets
     its `RustType` recomputed **fresh, uncached** every time it's referenced (per point 3's scope
     narrowing — `Tuple` deliberately isn't memoized like `Record`/`Union` are). This is fine/idempotent
     for an ordinary non-recursive `Tuple`, but not when a nested element's `RecBox`-eligibility depends
     on live `in_progress` state: the *same* reference gets `Box`-wrapped when computed while nested
     inside `ping`'s own in-progress `Union` processing, but *not* when computed later/separately for
     `pong`'s own top-level decode-function signature (by which point `ping` is no longer
     `in_progress`) — producing two disagreeing types for what should be one. **Worked around** (per the
     user's direction) by reshaping the capstone test format itself rather than fixing the underlying
     inconsistency: rebuilt both `pong` *and* `ping`'s "More" variant payload as named `Format::record`s
     (single-value `Union`-variant payloads — *"the 1-tuple variant convention real doodle productions
     end up with"* — rather than raw multi-field `Tuple`s), since `Record` is always cached and so
     doesn't have this problem. This resolved Finding A for the capstone shape, but the underlying
     `Tuple`-recompute inconsistency is still real and unfixed; flagged here for whoever next needs a
     recursive format whose cycle genuinely must pass through a raw multi-field `Tuple`.
   - **Finding B — a separate, deeper pre-existing bug in `decoder_map`/`compile_queue`.** Even after
     Finding A's type-level fix (both `pong` and `ping`'s variant payload as records), the *decode
     logic* itself compiled `pong`'s reference back to `ping` (its `next` field) into a **dead decoder
     that unconditionally fails** (`fn Decoder3(...) -> Result<ping, ParseError> { return
     Err(ParseError::FailToken(...)); }`), rather than reusing `ping`'s real decoder. Root-caused
     precisely: `Format::record` desugars to nested `Format::LetFormat`/`Format::MonadSeq` (`chain`/
     `monad_seq`), and **both** `TypedFormat::LetFormat` and `TypedFormat::MonadSeq`'s compile arms — in
     *both* `src/decoder.rs`'s `Compiler::compile_format` *and* `src/codegen/typed_decoder.rs`'s
     `GTCompiler::compile_gt_format` — unconditionally wrap `next` with `Next::Cat(Typed(second),
     next.clone())` for the first component, regardless of whether `second` (here, the record's final
     `Format::Compute(Expr::Record(...))` construction step) ever consumes a byte. This produces a
     structurally-different `next` (`Cat(Typed(compute_expr), Empty)`) for what is semantically the
     identical "nothing more to look ahead at" continuation that `ping`'s own top-level entry used
     (bare `Empty`) — `decoder_map`'s `(level, next)` key misses, queues a **second, duplicate**
     compilation of `ping` under the mismatched `next`, and compiling `ping`'s `Union` body against that
     wrong continuation makes `MatchTree`/lookahead construction fail, which becomes
     `TypedDecoder::Fail`. **This is the exact same bug class Phase 1.5 already found and fixed for
     `Tuple`/`Sequence`'s trailing-field handling** (`decoder.rs`/`typed_decoder.rs`, unconditional
     `Next::Sequence` wrapping even for an empty/zero-progress remainder) — it was simply never audited
     for `LetFormat`/`MonadSeq`, because Phase 1.5's own test formats used bare `Tuple`, never
     `Format::record`.

     **Fixed 2026-09-09** (a later session): both arms now skip the `Next::Cat`/wrap when `second`
     can only ever match zero bytes (`second.match_bounds(module).as_exact() == Some(0)`, delegating
     to the existing `Format::match_bounds`/`Bounds::as_exact`, which already treats `Compute` as
     exact-zero and `Hint` as transparent) and use `next` directly instead — mirroring Phase 1.5's
     `Tuple`/`Sequence` special-casing exactly, as originally planned. Bug-injection verified in both
     compile paths independently: disabling the `decoder.rs` half makes `compile_program` hang
     (unbounded `compile_queue` growth from cascading duplicate compiles, not a clean error - a new,
     real observation this plan hadn't predicted) on
     `define_format_rec_batch_mutual_recursion_ping_pong_record_variant_decodes` (new test,
     `src/lib.rs`); disabling the `typed_decoder.rs` half reproduces the *exact* predicted
     `Decoder3(...) { return Err(ParseError::FailToken(...)) }` dead decoder on
     `recursive_format_through_record_field_generates_no_dead_decoder` (new test, `src/codegen/mod.rs`).
     `cargo testall` clean.

     This fix is **not** narrowly scoped to the recursive case the way Phases 1–3's changes were:
     `Format::record`'s last field always has this exact zero-width-trailing-`Compute` shape, so the
     bypass also fires for every *non*-recursive record in `doodle-formats/` whose last field happens
     to be a named sub-format call. Regenerating `generated/gencode.rs` confirmed this in practice —
     a large diff (1758 insertions / 2103 deletions) across ELF, gzip, OpenType, etc., all pure
     `decoder_map` dedup/renumbering (concretely verified: on-disk `Decoder79`, a pure
     `Decoder80(input)` forwarding wrapper, and on-disk `Decoder80`, the real `Vec<char>` decode loop
     it forwarded to, both disappear post-fix, merged into the decoder already doing that job
     elsewhere) — **not a correctness change**. Per the user, this is itself a genuine (if minor,
     pre-existing) codegen inefficiency this fix incidentally also cleans up, not a regression to
     avoid; see the disclaimer at the top of this document and the revised "Non-negotiable safety
     constraints" entry above for the resulting change to this plan's own byte-identical requirement.

**Net effect**: points 1–2 (a real `Box` type, correctly inserted at both type and construction sites,
with the `Phantom` regression caught and fixed) are solid, committed-quality work — `cargo testall`
clean, `cargo cg` byte-identical throughout, bug-injection-verified. Point 3 is a deliberate, narrow
scope cut. Point 4's Finding A is still open (the `Tuple`-recompute inconsistency, worked around for
the capstone shape but not fixed in general); **Finding B is now fixed** (see above) — Phase 4's own
"confirm decode-time dispatch" investigation target turned out **not** to hold as originally stated for
a cycle reached through a `Format::record`, contra this document's original "expected: no code
changes" framing for that phase, but does hold once this fix is applied.

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

## Phase 5 findings (recorded 2026-09-09, after implementation)

Used the recommended shape exactly (peano self-recursive, ping/pong mutually-recursive), built via
`tests/recursion/` (`mod.rs`/`codegen_tests.rs`/`api_helper.rs`, matching the `tests/runtime_repeat`/
`tests/permit_state_error` convention precisely, registered as its own `[[test]]` in `Cargo.toml`).
`tests/recursion/mod.rs` is never hand-edited - it's frozen output of `src/codegen/mod.rs`'s
`#[ignore]`d `regenerate_recursion_fixture` test (per the user's explicit direction, to keep the
fixture's provenance traceable/reproducible rather than a one-off manual paste). Deliverables:

- Typecheck: already covered by Phase 2's own end-to-end test plus every `define_format_rec_batch`
  call's own panic-on-typecheck-failure registration; no new test needed here specifically.
- MatchTree/interpreted-decode: existing Phase 1.5 tests already covered depth ≥ 2 decode for both
  shapes. The one genuinely missing piece per this phase's own deliverable list - a malformed-input
  rejection case through the *interpreter*, as opposed to the build-time left-recursion rejection
  Phase 1.5 already covers - is new:
  `define_format_rec_batch_self_recursive_peano_rejects_malformed_input` (`src/lib.rs`), proving a
  well-formed recursive grammar's `MatchTree`/decoder still correctly rejects bad bytes reached mid-
  recursion, not just at the top level.
- Codegen: `tests/recursion/codegen_tests.rs`, a real `rustc`/`cargo test` round-trip (not a bare
  `rustc` probe - `[[test]]` compiles it as part of the normal workspace build) - decodes real bytes
  at depth ≥ 2 for both peano and ping/pong, plus a malformed-input rejection case for each.

**A second, previously-undiscovered Box-placement gap was found and fixed**, the first time anyone
actually tried to `rustc`-compile a self-referential `Format::record`'s generated output (every prior
check of this shape - Phase 3's `phase3_final_check_peano_and_ping_pong`, Phase 4's
`recursive_format_through_record_field_generates_no_dead_decoder` - was a string-level check only,
never an actual compile): `box_wrap_if_needed` (Phase 3's Box-insertion helper) is only ever consulted
from `CodeGen::translate`'s `TypedDecoder::Variant`/`TypedDecoder::Tuple` arms; a record's closing
`Compute(Record(...))` step is a plain `Expr`, translated by the free function `embed_expr`'s
`TypedExpr::Record` arm - a separate code path with no `RecBox` awareness at all. Concretely: `pong`'s
*type* declaration correctly said `next: Box<ping>`, but `Decoder_pong`'s *construction* built
`pong { tag, next }` (unboxed) - a real `E0308` type mismatch, not cosmetic. Fixed by having
`embed_expr`'s `TypedExpr::Record` arm consult the record's own declared field types (available
inline off `GenType::Def`'s `RustTypeDecl`, no `defined_types` table needed from this free function)
and calling `.wrap_box()` on any field whose declared type is `RecBox`-wrapped, mirroring
`box_wrap_if_needed` exactly. Bug-injection-verified: disabling the check reproduces the exact
predicted `rustc` error (`expected Box<ping>, found ping`, with `rustc`'s own suggested fix matching
what's already there) on `tests/recursion`'s own build. `cargo cg` byte-identical - a `RecBox`-wrapped
field type only exists on an already-recursive format, so no existing non-recursive record is
affected. See `doc/RECURSION.md`'s "Box placement in codegen" section for the permanent writeup.

This also settled which shape to use for `pong`: a raw-`Tuple` `pong` (matching
`phase3_final_check_peano_and_ping_pong`'s existing shape) hits Finding A's still-open recompute
inconsistency instead (`Decoder_pong`'s own return-type signature disagreeing with `ping::More`'s
field type on whether `pong`'s back-reference to `ping` needs boxing) the moment it's actually
compiled - confirmed directly, not assumed. `tests/recursion/`'s `pong` is therefore a named
`Format::record` (`{ tag, next }`), exactly the workaround Finding A's own writeup already
anticipated. Finding A itself remains open and unfixed for the raw-`Tuple` shape.

`cargo testall` clean; `cargo fmt` clean; `cargo cg` byte-identical.

---

## Phase 6 — Full-suite verification and wrap-up

1. `cargo fmt -- --check` (project-wide, not just changed files).
2. `cargo testall` (`cargo test --workspace --exclude smallsorts --exclude analytic-engine --exclude
   analytic-parser`) — must be fully clean, not just "no new failures."
3. `cargo build` (full workspace).
4. `cargo cg` (regenerate `generated/gencode.rs`) and diff against the pre-port version. Per the
   revised "Non-negotiable safety constraints" entry above, a diff limited to the expected
   new-decoders-plus-renumbering/rehashing shape (including a confirmed-benign `decoder_map`
   dedup/merge, as found in Phase 4's Finding B) is acceptable; anything else must be reported to
   the user for review before being treated as done, not silently committed.
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
| 0 — Reconnaissance | Done | (uncommitted) | All anchors confirmed accurate modulo minor line drift; Phase 1 TRIAGE resolved (reuse `MatchTree::build`'s existing `Option`→`anyhow!` convention); see "Phase 0 findings" section above |
| 1 — MatchTree cycle guard | Done | 1d47955 | `CycleGuard` (open-set + `detected` flag) threaded through `MatchTreeStep::from_format`/`from_gt_format`/`from_next`/helpers in `src/lib.rs`; `Format::ItemVar` and `TypedFormat::FormatCall` both insert/remove their own level around the recursive call; `MatchTreeLevel::grow` creates a fresh guard per top-level step and returns `None` outright the instant `detected` fires (reusing the existing "cannot build match tree" convention, not a silent per-branch `reject()`). See "Phase 1 design note" below for why total-failure-propagation was chosen over a local reject. 3 regression tests added directly in `src/lib.rs`'s `mod test` (guarded peano-shaped recursion terminates+disambiguates; genuine zero-progress self-reference sets `guard.detected`; `MatchTree::build` itself returns `None` on that case) - bypassing `define_format`'s type-checked registration on purpose, since it still cannot express a non-`Phantom` self-reference (see `reserve_recursive_level`'s doc comment) and these tests only need to exercise `MatchTreeStep`/`MatchTree` construction, not the full pipeline. Bug-injection verified: with the `ItemVar` guard temporarily removed, the left-recursion test reproduces a real stack overflow (not a hang or unrelated crash), confirming the test actually exercises the fix. `cargo testall` clean; `cargo cg` diff against pre-change `generated/gencode.rs` is byte-identical. `TypedFormat::FormatCall`'s guard mirrors `ItemVar`'s for correctness/symmetry, but is very likely dead code today - `MatchTree::build`'s only entry point takes `&[Format]` (untyped), and nothing outside this `impl` block calls `from_gt_format`/`from_mt_format` (confirmed via `find_references`) - so it wasn't given its own dedicated unit test/bug-injection round; flagged here for whoever eventually wires up a live typed entry point. |
| 1.5 — Batch construction API + 3 more unguarded-recursion bugs | Done | a6f328b | Not in the original plan - see "Phase 1.5" section above. `Format::RecVar` (construction-time sugar, rewritten to `ItemVar` before install) + `FormatModule::define_format_rec_batch`; fixed `depends_on_next`/`match_bounds`/`lookahead_bounds` (unguarded `ItemVar` recursion, confirmed stack-overflowing in isolation) and the `Tuple`/`Sequence` `Next`-wrapping bug that broke `decoder_map` memoization for any self-reference. 3 new end-to-end tests (peano, ping/pong, left-recursion-rejected) via the real public API. `cargo testall` clean; `cargo cg` byte-identical. |
| 2 — `occurs_in` generalization | Done | 35b978b | Both TRIAGE items resolved directly from code: indirection boundaries are `Tuple`/`Record`/`Seq`/`Option` plus `Constraints::Variant`'s labeled `VarMap` entries (real doodle has no `UType::Union` - sum-type-ness lives in `Constraints`, not `UType`) and `Constraint::Proj`'s `TupleWith`/`RecordWith`/`SeqOf`/`OptOf`; `Constraint::Equiv` and plain `Var`-forwarding are not boundaries. `PhantomData` keeps its existing unconditional exemption unchanged (already fully opaque, doesn't even bind its inner content). `occurs_in`/`occurs_in_constraints` now thread `indirected: bool` + `visited: &mut HashSet<(UVar, bool)>` (keyed by canonical constraint-index, never reset at boundaries) mirroring the design directly. 4 low-level tests mirroring doodle-rec's own names (`direct_self_alias_with_no_indirection_is_rejected`, `self_reference_behind_a_tuple_is_accepted`, `self_reference_behind_a_union_variant_is_accepted` - real doodle's Union analog, `an_unrelated_pre_existing_cycle_does_not_hang_occurs_check`) plus one end-to-end test proving a genuinely self-recursive format (built via Phase 1.5's `define_format_rec_batch`) now typechecks through the real `TypeChecker::infer_module` entry point, not just the isolated `occurs` check. Bug-injection verified in two rounds: disabling the base rejection breaks `direct_self_alias_...` as predicted; disabling just the `Tuple` boundary breaks exactly the two tests that rely on it (`self_reference_behind_a_tuple_is_accepted` and the unrelated-cycle test, which also crosses a `Tuple`) and no others. `cargo testall` clean; `cargo cg` byte-identical. |
| 3 — `CodeGen` Box placement | Type+construction-site `Box` done; nominal-promotion for self-referential `Tuple` deliberately out of scope | c50f47e | See "Phase 3 & 4 findings" above. `CompType::RecBox` + full trait wiring; `RustExpr::wrap_box`/`GenExpr::WrapBox`/`DerivedLogic::WrapBox` construction-site insertion; `Expansion::Tuple`/`Seq`/`Option` cycle-guards (reject-loudly, not promote); `Format::Phantom` regression found and fixed (`in_phantom_context`). `cargo testall` clean, `cargo cg` byte-identical, bug-injection-verified. |
| 4 — Decode-time confirmation | Done | d10714f | Contra this phase's own "expected: no code changes": found, root-caused, and fixed Finding B. `LetFormat`/`MonadSeq`'s compile arms in both `decoder::Compiler::compile_format` and `codegen::typed_decoder::GTCompiler::compile_gt_format` now skip the `Next::Cat` wrap when the trailing continuation can only match zero bytes (`second.match_bounds(module).as_exact() == Some(0)`) and use `next` directly — same bug class and fix shape as Phase 1.5's `Tuple`/`Sequence` special-casing, unaudited for this pair until now. 2 new end-to-end regression tests: `define_format_rec_batch_mutual_recursion_ping_pong_record_variant_decodes` (`src/lib.rs`, interpreter path) and `recursive_format_through_record_field_generates_no_dead_decoder` (`src/codegen/mod.rs`, codegen path). Bug-injection verified independently for each compile path: disabling the interpreter half hangs `compile_program` (unbounded `compile_queue` growth); disabling the codegen half reproduces the exact predicted `Decoder3(...) { return Err(ParseError::FailToken(...)) }` dead decoder. `cargo testall` clean. `cargo cg` diff is non-empty but confirmed pure `decoder_map` dedup/renumbering (see Finding B writeup above and the revised byte-identical rule in "Non-negotiable safety constraints") — not a regression. |
| 5 — Capstone integration test | Done | e1a303a | See "Phase 5 findings" above. `tests/recursion/` fixture (peano self-recursive, ping/pong mutually-recursive with `pong` as a `Format::record`) - real production codegen output, frozen by `src/codegen/mod.rs`'s `#[ignore]`d `regenerate_recursion_fixture`, actually compiled and run via `cargo test --test recursion` against real crafted bytes at depth ≥ 2 for both shapes plus malformed-input rejection for each. Found and fixed a second, previously-undiscovered Box-placement gap (`embed_expr`'s `TypedExpr::Record` arm never boxed a self-referential field's constructed value) - the first time this project actually `rustc`-compiled a self-referential `Format::record`'s output. Also added the one missing interpreter-path deliverable, a malformed-input rejection test (`define_format_rec_batch_self_recursive_peano_rejects_malformed_input`, `src/lib.rs`). `cargo testall` clean, `cargo fmt` clean, `cargo cg` byte-identical. |
| 6 — Full-suite verification | Done | (this commit) | `cargo fmt`/`cargo testall`/`cargo build --workspace`/`cargo cg` diff all clean (byte-identical) as of this commit. Updated `doc/DESIGN.md` (new "Cycle guard for self-referential formats" section) and `TYPECHECKER.md` (new "Occurs-check and representability" section) to describe the now-landed model, both pointing to `doc/RECURSION.md` for the full writeup rather than duplicating it. `READARRAY_AUDIT.md` left unchanged - nothing new surfaced beyond Phase 0's already-confirmed non-issue finding. This is the plan's final phase; see `doc/RECURSION.md` for the settled design going forward, this file for the process history. |
