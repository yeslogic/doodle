# Plan: where Numerics are accepted and rejected

Status: all decisions below are confirmed by the user. Implemented: T3 (b43ee516); R1, R3, R4, R5, T1, T2, T4 (step 2, 81aea425); I1, I2, I3 (step 3, 8be98724); C1, C2, C3 (step 4, 5aa40bc3); R6; step 5. Q2 (`Pos`) is resolved as item 14. Everything is implemented.

Note: `BaseSet::UAny32` admits every unsigned type including `U64`; the `32` is only the default width used to break ties when several solutions are sound. Registration's counterpart to both `UAny` and `UAny32` is therefore the same "unsigned or Auto" rule.

Inputs: `NUMERIC_GUIDELINE.md`, `NUMERIC_SURVEY_RESULTS.md`, `DIVERGENCE.md`, `doc/NUMERIC.md`.

Terminology:
- **registration**: legacy inference, `Expr::infer_type` and its format and pattern counterparts.
- **TC**: the `TypeChecker`.
- **Auto**: a Numeric with `NumRep::Auto`, typed as `ValueType::NumericHole` by registration.

## Guiding principle

- If registration and TC **both reject** a construction, nothing downstream has to support it.
- If registration and TC **both accept** a construction, the interpreter and codegen must both support it without panicking. For example, `numeric(numexpr!(5u32))` must behave like `Expr::U32(5)`.
- Where registration and TC **disagree**, the user decides each site. The decisions are recorded below.

"Behaves the same downstream" means both layers succeed, or both return an error. The error doesn't have to happen at the same place. For example, an overflow can be reported at the `+` in codegen and at the next comparison in the interpreter.

## Decisions

| # | Site | Decision |
|---|------|----------|
| 1 | Signed operands to native `Arith` / `IntSucc` / `IntPred` | TC is right: unsigned only. Registration must reject `Signed` and still allow Auto. Signed arithmetic belongs in `NumExpr`. |
| 2 | Signed operands to `IntRel` | Registration is right. TC widens `IntRel` operands from `UAny` to `IntSet::ZAny`; the two operands must still unify, so mixed-sign comparisons stay rejected. |
| 3 | `AsChar` | TC is right (`UAny`). Registration must reject `Signed`; the idiom for a signed value is `AsChar(AsU32(x))`. `AsChar` deliberately differs from `AsU8`–`AsU64`, which use `ZAny`. |
| 4 | Signed lengths, offsets and `EnumFromTo` bounds (`CaptureBytes`, `ReadArray` length, `Offset`, `EnumFromTo`) | TC is right. Registration must reject `Signed`. Signed ranges are left as possible future work. |
| 5 | `SeqIx` / `SubSeq` / `SubSeqInflate` / `Dup` | **No change for now.** Deferred to the `seqlen-always-u32` project. TC is only looser than registration here, so nothing both accept fails downstream. **Resolved by that project** (`doc/SEQLEN_PLAN.md`): registration now uses the "unsigned or Auto" rule at these sites, and `SeqLength` is Auto. |
| 6 | `FindByKey` key | Registration is right. TC adds a `UAny` constraint on the key. Also, `Value::eq_key` must return an `EvalError` instead of panicking on an out-of-range Numeric key (`-1auto` against `U8`). |
| 7 | Auto as `RepeatCount` / `RepeatBetween` counts | Registration must accept Auto, matching `CaptureBytes`, `ReadArray` and `Offset`. Registration's rule everywhere becomes "signed no, Auto yes". |
| 8.1 | Pattern type mismatches in registration | `build_scope` returns errors instead of panicking. |
| 8.2 | Auto matched against `U8(n)`…`U64(n)` / `Int(..)` | TC is right. Registration accepts it. The interpreter compares `U8(n)` against an Auto Numeric by value. `pattern_matches_auto_rep`, which currently matches like a wildcard, is removed or redefined. |
| 9 | Auto whose type TC can't pin | No default types; TC is right that it's ambiguous. "no unique solution" and "no valid solutions" become proper `TCResult` errors instead of panics. Registration stays looser; nothing downstream has to support a bare Auto. |
| 10 | Auto that TC pins but the interpreter can't see (`5auto + 5u8`, `(5auto + 5auto) == U8(10)`, `IntSucc(5auto) == U8(6)`) | The interpreter handles Auto by value. In arithmetic, an Auto operand takes the other operand's type; Auto with Auto computes by value and returns Auto; `IntSucc`/`IntPred` on Auto compute ±1 by value and return Auto. The width is checked wherever the value next meets a concrete type. Passing TC's resolved types into the interpreter would be the principled end state, but it's deferred. |
| 11.1 | `RepeatBetween` with constant Numeric bounds panics in codegen | Codegen's bounds analysis must evaluate Numerics with no variables, as registration does. |
| 11.2 | `RepeatBetween` with bounds that aren't constant | Registration and TC both reject it with an error. |
| 11.3 | Match with only `ZConst`/`ZRange` arms on an unsigned value, no fallback arm | First confirm with a test that it hits `unreachable!` in `IntCoverage::add`. Then fix the coverage check: treat the arms like `Int` ranges, or otherwise treat the match as possibly incomplete. |
| 12 | `Slice` length, `WithRelativeOffset` base and offset (was Q1) | TC is right (`UAny32`). Registration adopts the equivalent, which is R1's "unsigned or Auto" rule. Signed and non-numeric types are rejected. |
| 14 | `Format::Pos` (was Q2) | Registration keeps typing `Pos` as `NumericHole`; both interpreters produce an Auto Numeric (`Value::from_pos`) instead of a native `U64`, so `Pos` takes the type of whatever operand it meets. Explored first against regression tests (`doodle-formats/tests/pos.rs`); the only live effect is display (`start_of_header := 12` becomes `12?` in the `test2.jpg` snapshot), which is acceptable. |
| 13 | The `_ext` inference in `src/alt.rs` (was Q3) | Mirror the registration changes (R1, R3, R5) there to keep it current. `alt.rs` is the currently-unused "alternate processing model", so this is low priority and comes after the main registration work. |

## Changes by layer

### Registration (`Expr::infer_type` in `src/lib.rs`, the format-level inference, `src/pattern.rs`)

- **R1. One shared rule: "unsigned or Auto".** It accepts `Base(b)` where `b` is numeric, or `NumericHole`. At sites that unify two operands first, it applies to the unified type, so `Signed` combined with Auto is rejected. Where it applies:
  - `Arith` and `IntSucc`/`IntPred` (item 1)
  - `AsChar` (item 3)
  - `EnumFromTo` bounds, `CaptureBytes` and `ReadArray` lengths, and `Offset` (item 4)
  - `RepeatCount`/`RepeatBetween`, where it *relaxes* the current check, which accepts only `Base` (item 7)
- **R2. No change:**
  - `IntRel` and `AsU8`–`AsU64` keep `is_numeric()` (items 2 and 3).
  - `SeqIx`, `SubSeq`, `SubSeqInflate` and `Dup` (item 5).
  - `FindByKey` (item 6).
- **R3. Patterns:** `build_scope` returns errors instead of panicking. `NumericHole` is accepted against `U8(n)`…`U64(n)` and `Int(..)` (item 8).
- **R4.** `RepeatBetween` bounds that aren't constant are rejected with an error, replacing the "not implemented" panic when the decoder is compiled (item 11.2).
- **R5.** `Slice` lengths and `WithRelativeOffset` bases and offsets get the "unsigned or Auto" check. Registration previously didn't type-check these expressions at all (item 12).
- **R6.** Mirror R1, R3 and R5 in `src/alt.rs` (`infer_type_ext`, `check_type_ext`, `build_scope_ext`). Low priority (item 13).

### TypeChecker (`src/typecheck.rs`)

- **T1.** `IntRel` operands change from `UAny` to `IntSet::ZAny`; the operands still unify (item 2).
- **T2.** The `FindByKey` key gets a `UAny` constraint (item 6).
- **T3. (Done.)** "no unique solution" and "no valid solutions" become `TCResult` errors that go through `generate_code`'s normal "Failed to infer module-wide type annotations" path. No defaults are added (item 9).
  - Implemented as `TypeChecker::check_unique_solutions`, run at the end of `infer_module`. It calls `get_unique_solution` on every canonical `Elem`/`NumTree` variable. The `expand_var` arms are now `unreachable!`.
  - Test: `test_unpinned_auto_is_tc_error`. `cargo cg` output is unchanged.
- **T4.** `RepeatBetween` bounds that aren't constant are rejected, ideally by the same check as R4 (item 11.2).

### Interpreter (`src/decoder/value.rs`, `src/decoder/eval.rs`, `loc_decoder` mirror)

- **I1.** An out-of-range key in `eq_key` returns an `EvalError`, using the existing `Cell<Option<EvalError>>` pattern for errors raised inside `FindByKey` (item 6).
- **I2.** `U8(n)`…`U64(n)` patterns compare an Auto Numeric by value. `pattern_matches_auto_rep` is removed or redefined. The `loc_decoder` pattern matching is changed the same way (item 8).
- **I3. Auto in `arith` and `unary`** (item 10):
  - Auto with a concrete operand: the Auto takes the other operand's type and goes through the existing `get_as_unsized` path.
  - Auto with Auto: computed by value, and the result is Auto.
  - `IntSucc`/`IntPred` on Auto: ±1 by value, and the result is Auto.
  - Signed Numerics in `arith`/`unary` stay as panics. Registration now rejects them, so they should be unreachable, the same as the existing type-checker-invariant panics.

### Codegen

- **C1.** `RepeatBetween`'s bounds analysis evaluates Numerics with no variables exactly (item 11.1).
- **C2.** Check that the `IntRel` lowering emits correct comparisons on signed types (item 2).
- **C3.** Add a test for a match with only `ZConst`/`ZRange` arms on an unsigned value; if it hits the `unreachable!`, fix `IntCoverage` (item 11.3).

## Verification

- Re-run `numeric_survey`. The rows where the interpreter panics after registration accepts, for signed operands and unpinned Auto, should become consistent rejections. `SeqIx`/`SubSeq`/`Dup` are expected to stay as they are.
- Add survey cases the current survey doesn't cover:
  - pinned Auto: `5auto + 5u8`, `(5auto + 5auto) == U8(10)`, `IntSucc(5auto) == U8(6)`
  - `5auto ~ U8(5)` running in the interpreter
  - `RepeatBetween` with a variable bound
  - a `ZConst` match with no wildcard arm
  - signed `IntRel` through codegen
- Compile the generated code for the rows that change, at least the signed `IntRel` cases. The survey never compiles what codegen emits.
- Turn `test_repro_intrel_on_signed_fails_to_unify` into a test that expects the program to be accepted.
- Update `doc/NUMERIC.md`, including footnotes 2–4, and `NUMERIC_GUIDELINE.md`.

## Findings from reading the code

These come from reading registration's format inference (`src/lib.rs`), TC's format, view and elaboration code (`src/typecheck.rs`), `src/pattern.rs`, `src/codegen/util.rs`, `src/codegen/mod.rs`, `src/codegen/typed_format.rs`, `src/codegen/typed_decoder.rs`, `src/decoder.rs`, `src/decoder/value.rs` and `src/loc_decoder.rs`. Each one refines the plan above.

- **T3.** TC's inference step doesn't panic. It returns `Ok` even when a variable has no unique solution. The panics happen later, during elaboration, in `TypeChecker::expand_var`'s `Constraint::Elem` and `Constraint::NumTree` arms (`Err(e) => panic!("{e}")`). So T3 means one of:
  - a check pass after inference, before elaboration, that calls `get_unique_solution` on every `Elem`/`NumTree` constraint and returns a `TCError`, or
  - making `expand_var` fallible.
- **R1 for `RepeatBetween`.** Registration currently compares the two bounds with `ValueType::Base(b1) if b0 == b1`, an exact equality. For Auto to work on either bound it needs a unify followed by the "unsigned or Auto" check.
- **R4 and T4.** The registration-side panic is in `Compiler::compile_format` (`decoder.rs:487`/`490`), which already returns a `Result`, so it can simply return `Err`. For T4, TC's `Format::RepeatBetween` arm can call `Expr::bounds().as_exact()` on both bounds and return a `TCError`.
- **C1 covers more than Numerics.** `TypedExpr::bounds` (`typed_format.rs:914`) handles only `U8`–`U64` literals, `Add` and `Mul`. `Expr::bounds` also handles `AsU8`–`AsU64`, closed Numerics and more. So a bound such as `RepeatBetween(AsU8(..), …)` also passes registration and panics in codegen.
  - Fix: have `TypedExpr::bounds` convert to `Expr` using the existing conversion (`typed_format.rs:1136`) and call `Expr::bounds`, so both layers use one analysis.
  - Once T4 is in place, the codegen `unimplemented!` should be unreachable.
- **C2.** The `IntRel` lowering (`codegen/mod.rs:1395`) emits a plain infix operator and ignores the type, so signed operands need no codegen change. The generated code still needs a compile check.
- **C3 is confirmed in the code.** `IntCoverage::add` has explicit `unreachable!("IntCoverage does not support ZConst/ZRange yet")` arms. Any match on an unsigned value that has at least one `ZConst`/`ZRange` arm and no irrefutable arm reaches them, even when the other arms are `U8(n)`.
  - Fix: `ZConst(n)` inserts `n` when `0 <= n <= max`; `ZRange` inserts its range clamped to `[0, max]`.
- **A new violation of the principle in `loc_decoder`.** `ParsedValue::matches_inner` (`loc_decoder.rs:366`) has no `Value::Numeric` arm for `U8(n)`…`U64(n)` or for `Int(..)`. The main interpreter's `Value::matches_inner` has them, via `TypedConst::matches_u8` and friends and `matches_int_range`.
  - So `5u8 ~ U8(5)` and `5u8 ~ Int(0..=10)` fail to match under the location-tracking interpreter, even though every layer in the survey accepts them.
  - I2 now also covers giving `loc_decoder` the same Numeric arms.
  - The Auto value comparison for I2 goes into the shared `TypedConst::matches_u8`…`matches_u64` helpers.
- **Step 2 implementation notes.**
  - The rule is `ValueType::is_unsigned_or_auto` (`src/valuetype.rs`).
  - R4 and T4 share `Expr::exact_repeat_bounds`, which `Compiler::compile_format` also uses, returning `Err` in place of its `unimplemented!`. The TC error is `TCErrorKind::NonConstantRepeatBounds`.
  - `test_repro_intrel_on_signed_fails_to_unify` is replaced by `test_intrel_on_signed`.
  - The `signed_intops.rs` repro tests now assert that registration and codegen both reject; `test_interp_char` became `test_interp_char_via_u32` (the `AsChar(AsU32(x))` idiom).
- **Step 3 implementation notes.**
  - I1: `AsKey::compare_as_key`/`eq_key` return `Result<_, EvalError>`, and `find_index_by_key_sorted`/`_unsorted` return `Result<Option<usize>, EvalError>`. This replaces the `.expect()`s directly instead of adding a second `Cell`; the existing `Cell` still carries key-lambda errors, which take priority.
  - I2: `TypedConst::pat_matches` compares Auto by value; the `pattern_matches_auto_rep` Cargo feature (which made Auto a wildcard, and was off by default, making Auto never match) is removed. A concrete rep must still equal the pattern's width. Numeric-literal patterns go through one shared `Value::matches_numeric_literal`, used by both `Value::matches_inner` and `loc_decoder`'s `ParsedValue::matches_inner`, which gives `loc_decoder` its missing Numeric arms.
  - I3: Auto with a concrete operand already worked (`get_as_unsigned` accepts Auto). Auto with Auto now goes through `__arith_auto` on `BigInt`, returning Auto. Division by zero and shift amounts outside `0..64` are `ArithError`s. `IntSucc`/`IntPred` on Auto are ±1 by value, returning Auto. Signed reps still panic.
  - Survey after step 3: 0 rows where the interpreter panics after registration accepts. The remaining codegen panics are C1 (`RepeatBetween` with constant Numeric bounds).
- **Step 4 implementation notes.**
  - C1: `TypedExpr::bounds` converts to `Expr` and calls `Expr::bounds`. The `RepeatBetween` arm of `typed_decoder.rs` is now `unreachable!`, since T4 rejects non-constant bounds.
  - C2: confirmed no change is needed. The emitted code for `x < y` and `x == -1i8` over two `i8` reads was compiled and run in a scratch crate against `doodle::prelude`, returning `(true, true)` for input `[0xFF, 0x01]`. This was a one-off check, not a checked-in test.
  - C3: `test_gen_zrange_match` (`signed_intops.rs`) first confirmed the `unreachable!`. `IntCoverage` now inserts `ZConst`/`ZRange` values clamped to `[0, usize::MAX]`.
  - Survey after step 4: 55 of 275 cases inconsistent, with no panics in any layer. All remaining rows are item 9 (TC rejects unpinned Auto; 36 "codegen stricter") or item 5 (`SeqIx`/`SubSeq`/`Dup`; 15 "codegen looser"), plus value-dependent interpreter errors.
- **R3.** `build_scope` returns `()`, so it needs to return `AResult<()>`. Its callers already return `AResult`. The panic when a `Variant` label is missing converts in the same change.
- **Doc correction.** TC's `ViewExpr::Offset` and `CaptureBytes` use `BaseSet::UAny`, not `UintSet::ANY`; only `ReadArray` uses `UintSet::ANY`. The behaviour is the same.

## Open questions from reading the code

All three are resolved, as items 12, 13 and 14.

- **Q2. `Format::Pos`.** Resolved as item 14. Registration types it as `NumericHole`; TC gives it `UintSet::any_default(Bits64)`; the interpreter produced a native `Value::U64`, so `pos + U32(1)` passed registration and then panicked in `arith`. The interpreter now produces an Auto Numeric.

## Suggested order

1. T3, so every later survey run reports failures as errors instead of panics. (Done.)
2. R1, R3, R4 and R5, together with T1, T2 and T4. (Done; R6 (`alt.rs`) done after step 4.)
3. I1, I2 and I3. (Done.)
4. C1, C2 and C3. (Done.)
5. Survey and docs. (Done.)

## Completion notes

- **R6 (`alt.rs`).** `ValueTypeExt::is_unsigned_or_auto` (treating `EngineSpecific` like `is_numeric` does) now guards native `Arith`, `IntSucc`/`IntPred`, `AsChar`, `EnumFromTo` (after unification), `RepeatCount`, `RepeatBetween` (after unification), `CaptureBytes`/`ReadArray` lengths, `ViewExpr::Offset` (`check_type_ext`), `Slice` (previously only a `debug_assert!`) and `WithRelativeOffset` (previously unchecked). `build_scope_ext` mirrors R3. Test: `alt::tests::ext_numeric_acceptance`.
  - Not changed, though they still differ from registration: `IntRel` and `AsU8`–`AsU64` in `_ext` require a native unsigned `Base` type (registration uses `is_numeric()`), `RepeatBetween` bounds are not required to be constant (R4 was not in R6's scope; registration still enforces it after compilation to `Format`), and `Pos` is typed `U64`.
  - The `doodle` binary fails to build with `--features alt` (`opentype.rs:621`, type annotations needed). This predates this work (it also fails at 5aa40bc3); the library builds.
- **Step 5.** The survey gained the operand pair `(Auto, NumU8)` and a set of standalone cases: `(5auto + 5auto) == U8(10)`, `IntSucc(5auto) == U8(6)`, `RepeatBetween` with a variable bound, and a no-wildcard `ZConst`/`ZRange` match. All are consistent across the three layers. 286 cases, 55 inconsistent, none panicking; the remainder are item 5, item 9 and value-dependent interpreter errors. `doc/NUMERIC.md` (including footnotes 2–4) and `NUMERIC_GUIDELINE.md` are updated, and stale `Value::Usize` references in the rewritten sections are removed.

