# Divergences: `NUMERIC_SURVEY_RESULTS.md` vs `NUMERIC_GUIDELINE.md` / `doc/NUMERIC.md`

Below are the places where the survey disagrees with or adds to `NUMERIC_GUIDELINE.md`. The first group also contradicts or is missing from `doc/NUMERIC.md`.

## A. Divergences that affect `doc/NUMERIC.md`

**A1. The `FindByKey` key accepts `NumericHole` in legacy `infer_type`** (NUMERIC.md:87, guideline:22)
- Both docs say the key rejects `Signed` and `NumericHole`. The survey shows `FindByKey(id, 5auto, …)` and `FindByKey(id, -1auto, …)` both pass registration.
- `Signed` is still rejected. From the results alone it's unclear whether that comes from the `Base(b)` guard or from failing to unify with the table's `U8` key.

**A2. `FindByKey` with an out-of-range Numeric key panics in the interpreter** (not in either doc)
- A Numeric key whose value fits the other key's width now compares by value: `FindByKey(id, 5u8, …)` and `FindByKey(id, 5auto, …)` are accepted by all three layers.
- An out-of-range key still panics instead of returning an error: `FindByKey(id, -1auto, [U8(5)])` gives `Value::eq_key: Value::to_uniform_integer_pair encountered error: out of range conversion …` (#35). Compare the `AsU8(-1auto)` family, which returns an error.
- Codegen rejects that case (#34), so the panic is reachable only through registration and the interpreter.
- This belongs in the eval/"Panics" part of NUMERIC.md.

**A3. `arith` between two Numerics** (NUMERIC.md:182, guideline:43)
- NUMERIC.md says it panics when both sides are auto or have different concrete reps. The guideline says Numeric vs Numeric always panics. Neither is quite right:
  - `5u8 + 5u8` succeeds in all three layers, so two matching concrete **unsigned** reps work.
  - A **signed** rep panics even when both reps match: `5i8 + 5i8` gives #59 "cannot apply native-arith Add to signed-rep".
  - Auto+signed, signed+auto and auto+auto also panic (#60–63).
- NUMERIC.md is missing the signed-rep panic. The guideline's "panics by design" is too broad.

**A4. `Pattern::Int` (and `Pattern::U8`) against an Auto scrutinee** (NUMERIC.md:69)
- NUMERIC.md says an Auto Numeric can be matched by `Pattern::Int` if it resolves to an unsigned value. In practice no layer gets that far:
  - Legacy `build_scope` panics on `NumericHole` (#56 for `Int`, #54 for `U8`).
  - The TypeChecker hits a "no unique solution" panic on `5auto ~ Int(0..=10)` (#17).
- `5auto ~ U8(5)` is accepted by codegen but panics at registration.

**A5. `SubSeq` and `Dup` behave like `UAny32` in the TypeChecker** (NUMERIC.md:124 lists only `SeqIx`/`EnumFromTo`/`SeqLength`)
- Codegen accepts `u8`, `u16` and `5auto` (so a default applies) and rejects `i8`, the same as `SeqIx`.
- These are the "codegen looser" rows: legacy still requires exactly `U32`.

**A6. `RepeatBetween` with Numeric bounds cannot go through codegen at all** (NUMERIC.md:112 treats it as ordinary `UAny`)
- Codegen panics with #67 `not implemented: RepeatBetween on inexact bounds-expr` even for closed bounds such as `RepeatBetween(5u8, U8(5), u8)`. Legacy and the interpreter accept that case.
- This suggests codegen's bounds analysis doesn't use the closed-Numeric exact path described in guideline:55.
- Legacy also panics (not an error) at registration for a non-closed bound (`u8()=5`, #67).

**A7. Unresolved Auto constraints panic instead of producing a type error** (the `UAny`/`ZAny` sections imply ambiguity but don't say what happens)
- With no sibling to pin its type, an Auto const fails with "no unique solution" in these cases:
  - under `UAny`/`UintSet`, e.g. `5auto + 5auto`, `CaptureBytes(5auto)`, `RepeatCount(5auto)`;
  - under `ZAny`, e.g. `AsU8(5auto)` (#9);
  - as a `ZConst` scrutinee;
  - in `[5auto, 5auto]` and `if … 5auto else 5auto` (#66).
- A **negative** Auto const is limited to `{I8..I64}` (#10). Every unsigned-only site then fails with "no valid solutions" (#6, #18, #24, #41).
- These panics don't carry the "Failed to infer module-wide type annotations" prefix. They come from somewhere other than `generate_code`'s normal error-reporting path.

## B. Guideline-only divergences and additions

1. **Line 29**, "Auto trees can still be resolved to an unsigned type": this only holds when something in the context pins the type (see A7).
2. **Line 34**, legacy `build_scope` panics: it panics on `NumericHole` as well as `Signed`. It also panics for `Pattern::U8` against `Signed`, `NumericHole`, or even a mismatched native type (`5u16 ~ U8(5)`, #50). In other words, any mismatch there is a panic.
3. **Line 46**, `as_usize` sites: add `EnumFromTo` (#65).
4. **Line 50**, Auto never matching `U8(n)`: the survey can't confirm this, because registration panics before the interpreter runs (A4).
5. **Line 66**, `IntCoverage::add` hitting `unreachable!`: *not tested by the survey.* Every pattern case is built by `match_bool` (`doodle-formats/tests/signed_intops.rs:238`), which always appends a `Pattern::Wildcard` fallback arm. So `U8(5) ~ ZConst(5)` passing `generate_code` neither confirms nor contradicts the claim, which is about matches with *no* irrefutable arm. The guideline line should stay as-is (unverified by survey).
6. **Add** the new `FindByKey` behaviour from A1/A2, and the `RepeatBetween` panics from A6.

## Confirmed as recorded

These match the survey:
- the legacy `is_numeric` acceptance list;
- `RepeatCount`/`RepeatBetween` rejecting `Signed`/`Hole`;
- `SeqIx`/`SubSeq`/`Dup` requiring exactly `U32` in legacy;
- the TypeChecker's unsigned sets rejecting `i8` everywhere except `AsUN`;
- `ZConst(-1)` excluding unsigned scrutinees;
- `Pattern::Int` restricted to `UintSet`;
- `int_rel` comparing by value;
- `unary` panicking on signed or Auto;
- `AsUN`/`AsChar` being value-only;
- `as_usize` erroring on negatives;
- footnotes 1–3 of NUMERIC.md.

Not covered by the survey: `Usize` operands, `Format::Pos`, signed `ReadArray` kinds, `SubSeqInflate`.
