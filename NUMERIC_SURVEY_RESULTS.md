# Numeric survey results

Output of the `numeric_survey` diagnostic test in `doodle-formats/tests/signed_intops.rs`. Regenerate with:

```sh
cargo test -p doodle-formats --test signed_intops -- --ignored --nocapture numeric_survey
```

Each case feeds the same `Format` to three layers:

- **registration**: legacy `Expr::infer_type`, via `Compiler::compile_program`. A failure here can also come from the decoder-compilation step that runs after inference.
- **codegen**: the `TypeChecker`, via `generate_code`. The emitted code is not compiled or run.
- **interpreter**: `Program::run`.

`✓` means accepted, `✗ #n` means rejected with an error, and `! #n` means it panicked, where `#n` is an entry in the Messages list below. `–` means the interpreter was never run because registration failed. `generate_code` reports every type-checker rejection by panicking; panics that carry its "Failed to infer module-wide type annotations:" prefix are counted as `✗`.

Only cases where the layers behave inconsistently appear in the table. Cases every layer accepts, or every reached layer rejects with an error (no panics), are listed by category after the table.

## Inconsistent cases

| case | registration | codegen | interpreter | note |
|---|:-:|:-:|:-:|---|
| `IntSucc(5auto)` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `IntSucc(-1auto)` | ✓ | ✗ #2 | ✓ | codegen stricter |
| `IntPred(5auto)` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `IntPred(-1auto)` | ✓ | ✗ #2 | ✓ | codegen stricter |
| `AsU8(-1i8)` | ✓ | ✓ | ✗ #3 |  |
| `AsU8(-1auto)` | ✓ | ✗ #4 | ✗ #5 | codegen stricter |
| `AsU8(i8()=-1)` | ✓ | ✓ | ✗ #6 |  |
| `AsU32(-1i8)` | ✓ | ✓ | ✗ #7 |  |
| `AsU32(-1auto)` | ✓ | ✗ #4 | ✗ #8 | codegen stricter |
| `AsU32(i8()=-1)` | ✓ | ✓ | ✗ #9 |  |
| `AsChar(5auto)` | ✓ | ✗ #10 | ✓ | codegen stricter |
| `AsChar(-1auto)` | ✓ | ✗ #11 | ✗ #12 | codegen stricter |
| `SeqIx(seq, -1auto)` | ✓ | ✗ #13 | ✗ #14 | codegen stricter |
| `SubSeq(seq, -1auto, 1)` | ✓ | ✗ #13 | ✗ #15 | codegen stricter |
| `Dup(-1auto, U8(0))` | ✓ | ✗ #11 | ✗ #16 | codegen stricter |
| `FindByKey(id, -1auto, [U8(5)])` | ✓ | ✗ #17 | ✗ #18 | codegen stricter |
| `RepeatCount(5auto, u8)` | ✓ | ✗ #19 | ✓ | codegen stricter |
| `RepeatCount(-1auto, u8)` | ✓ | ✗ #20 | ✗ #21 | codegen stricter |
| `CaptureBytes(5auto)` | ✓ | ✗ #22 | ✓ | codegen stricter |
| `CaptureBytes(-1auto)` | ✓ | ✗ #23 | ✗ #24 | codegen stricter |
| `ReadArray(5auto, U8)` | ✓ | ✗ #22 | ✓ | codegen stricter |
| `ReadArray(-1auto, U8)` | ✓ | ✗ #23 | ✗ #25 | codegen stricter |
| `Offset(v, 5auto)` | ✓ | ✗ #10 | ✓ | codegen stricter |
| `Offset(v, -1auto)` | ✓ | ✗ #11 | ✗ #26 | codegen stricter |
| `-1auto ~ U8(5)` | ✓ | ✗ #27 | ✓ | codegen stricter |
| `5auto ~ Int(0..=10)` | ✓ | ✗ #10 | ✓ | codegen stricter |
| `-1auto ~ Int(0..=10)` | ✓ | ✗ #11 | ✓ | codegen stricter |
| `5auto ~ ZConst(5)` | ✓ | ✗ #28 | ✓ | codegen stricter |
| `-1auto ~ ZConst(5)` | ✓ | ✗ #4 | ✓ | codegen stricter |
| `U8(5) ~ ZConst(-1)` | ✓ | ✗ #29 | ✓ | codegen stricter |
| `5u8 ~ ZConst(-1)` | ✓ | ✗ #29 | ✓ | codegen stricter |
| `5u16 ~ ZConst(-1)` | ✓ | ✗ #30 | ✓ | codegen stricter |
| `5auto ~ ZConst(-1)` | ✓ | ✗ #4 | ✓ | codegen stricter |
| `-1auto ~ ZConst(-1)` | ✓ | ✗ #4 | ✓ | codegen stricter |
| `u8()=5 ~ ZConst(-1)` | ✓ | ✗ #29 | ✓ | codegen stricter |
| `5auto == 5auto` | ✓ | ✗ #28 | ✓ | codegen stricter |
| `5auto < 5auto` | ✓ | ✗ #28 | ✓ | codegen stricter |
| `5auto + 5auto` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `[5auto, 5auto]` | ✓ | ✗ #28 | ✓ | codegen stricter |
| `if _ then 5auto else 5auto` | ✓ | ✗ #31 | ✓ | codegen stricter |
| `RepeatBetween(5auto, 5auto, u8)` | ✓ | ✗ #19 | ✓ | codegen stricter |

## Consistent cases (omitted from the table)

- **increment/decrement**, all accept: `IntSucc(U8(5))`, `IntSucc(5u8)`, `IntSucc(5u16)`, `IntSucc(u8()=5)`, `IntPred(U8(5))`, `IntPred(5u8)`, `IntPred(5u16)`, `IntPred(u8()=5)`, `IntSucc(5auto) == U8(6)`
- **increment/decrement**, all reject: `IntSucc(5i8)`, `IntSucc(-1i8)`, `IntSucc(i8()=5)`, `IntSucc(i8()=-1)`, `IntPred(5i8)`, `IntPred(-1i8)`, `IntPred(i8()=5)`, `IntPred(i8()=-1)`
- **casts**, all accept: `AsU8(U8(5))`, `AsU8(5u8)`, `AsU8(5u16)`, `AsU8(5i8)`, `AsU8(5auto)`, `AsU8(i8()=5)`, `AsU8(u8()=5)`, `AsU32(U8(5))`, `AsU32(5u8)`, `AsU32(5u16)`, `AsU32(5i8)`, `AsU32(5auto)`, `AsU32(i8()=5)`, `AsU32(u8()=5)`, `AsChar(U8(5))`, `AsChar(5u8)`, `AsChar(5u16)`, `AsChar(u8()=5)`
- **casts**, all reject: `AsChar(5i8)`, `AsChar(-1i8)`, `AsChar(i8()=5)`, `AsChar(i8()=-1)`
- **sequence index/length arguments**, all accept: `SeqIx(seq, U8(5))`, `SeqIx(seq, 5u8)`, `SeqIx(seq, 5u16)`, `SeqIx(seq, 5auto)`, `SeqIx(seq, u8()=5)`, `SubSeq(seq, U8(5), 1)`, `SubSeq(seq, 5u8, 1)`, `SubSeq(seq, 5u16, 1)`, `SubSeq(seq, 5auto, 1)`, `SubSeq(seq, u8()=5, 1)`, `Dup(U8(5), U8(0))`, `Dup(5u8, U8(0))`, `Dup(5u16, U8(0))`, `Dup(5auto, U8(0))`, `Dup(u8()=5, U8(0))`, `SeqIx(seq, IntPred(SeqLength(seq)))`
- **sequence index/length arguments**, all reject: `SeqIx(seq, 5i8)`, `SeqIx(seq, -1i8)`, `SeqIx(seq, i8()=5)`, `SeqIx(seq, i8()=-1)`, `SubSeq(seq, 5i8, 1)`, `SubSeq(seq, -1i8, 1)`, `SubSeq(seq, i8()=5, 1)`, `SubSeq(seq, i8()=-1, 1)`, `Dup(5i8, U8(0))`, `Dup(-1i8, U8(0))`, `Dup(i8()=5, U8(0))`, `Dup(i8()=-1, U8(0))`
- **FindByKey keys**, all accept: `FindByKey(id, U8(5), [U8(5)])`, `FindByKey(id, 5u8, [U8(5)])`, `FindByKey(id, 5auto, [U8(5)])`, `FindByKey(id, u8()=5, [U8(5)])`
- **FindByKey keys**, all reject: `FindByKey(id, 5u16, [U8(5)])`, `FindByKey(id, 5i8, [U8(5)])`, `FindByKey(id, -1i8, [U8(5)])`, `FindByKey(id, i8()=5, [U8(5)])`, `FindByKey(id, i8()=-1, [U8(5)])`
- **repetition counts**, all accept: `RepeatCount(U8(5), u8)`, `RepeatCount(5u8, u8)`, `RepeatCount(5u16, u8)`, `RepeatCount(u8()=5, u8)`, `RepeatBetween(U8(5), U8(5), u8)`, `RepeatBetween(5u8, U8(5), u8)`, `RepeatBetween(U8(5), 5u8, u8)`, `RepeatBetween(5u8, 5u8, u8)`, `RepeatBetween(5auto, U8(5), u8)`, `RepeatBetween(5auto, 5u8, u8)`
- **repetition counts**, all reject: `RepeatCount(5i8, u8)`, `RepeatCount(-1i8, u8)`, `RepeatCount(i8()=5, u8)`, `RepeatCount(i8()=-1, u8)`, `RepeatBetween(U8(5), U16(5), u8)`, `RepeatBetween(5u8, 5u16, u8)`, `RepeatBetween(5i8, 5i8, u8)`, `RepeatBetween(5i8, 5u8, u8)`, `RepeatBetween(5auto, 5i8, u8)`, `RepeatBetween(-1i8, 5auto, u8)`, `RepeatBetween(i8()=5, 5auto, u8)`, `RepeatBetween(i8()=5, 5i8, u8)`, `RepeatBetween(u8()=5, 5u8, u8)`, `RepeatBetween(U8(1), u8()=2, u8)`
- **view lengths/offsets**, all accept: `CaptureBytes(U8(5))`, `CaptureBytes(5u8)`, `CaptureBytes(5u16)`, `CaptureBytes(u8()=5)`, `ReadArray(U8(5), U8)`, `ReadArray(5u8, U8)`, `ReadArray(5u16, U8)`, `ReadArray(u8()=5, U8)`, `Offset(v, U8(5))`, `Offset(v, 5u8)`, `Offset(v, 5u16)`, `Offset(v, u8()=5)`
- **view lengths/offsets**, all reject: `CaptureBytes(5i8)`, `CaptureBytes(-1i8)`, `CaptureBytes(i8()=5)`, `CaptureBytes(i8()=-1)`, `ReadArray(5i8, U8)`, `ReadArray(-1i8, U8)`, `ReadArray(i8()=5, U8)`, `ReadArray(i8()=-1, U8)`, `Offset(v, 5i8)`, `Offset(v, -1i8)`, `Offset(v, i8()=5)`, `Offset(v, i8()=-1)`
- **pattern matching**, all accept: `U8(5) ~ U8(5)`, `5u8 ~ U8(5)`, `5auto ~ U8(5)`, `u8()=5 ~ U8(5)`, `U8(5) ~ Int(0..=10)`, `5u8 ~ Int(0..=10)`, `5u16 ~ Int(0..=10)`, `u8()=5 ~ Int(0..=10)`, `U8(5) ~ ZConst(5)`, `5u8 ~ ZConst(5)`, `5u16 ~ ZConst(5)`, `5i8 ~ ZConst(5)`, `-1i8 ~ ZConst(5)`, `i8()=5 ~ ZConst(5)`, `i8()=-1 ~ ZConst(5)`, `u8()=5 ~ ZConst(5)`, `5i8 ~ ZConst(-1)`, `-1i8 ~ ZConst(-1)`, `i8()=5 ~ ZConst(-1)`, `i8()=-1 ~ ZConst(-1)`, `u8()=5 ~ ZConst(5) | ZRange(0..=255) (no wildcard)`
- **pattern matching**, all reject: `5u16 ~ U8(5)`, `5i8 ~ U8(5)`, `-1i8 ~ U8(5)`, `i8()=5 ~ U8(5)`, `i8()=-1 ~ U8(5)`, `5i8 ~ Int(0..=10)`, `-1i8 ~ Int(0..=10)`, `i8()=5 ~ Int(0..=10)`, `i8()=-1 ~ Int(0..=10)`
- **comparison**, all accept: `U8(5) == U8(5)`, `5u8 == U8(5)`, `U8(5) == 5u8`, `5u8 == 5u8`, `5i8 == 5i8`, `5auto == U8(5)`, `5auto == 5u8`, `5auto == 5i8`, `-1i8 == 5auto`, `i8()=5 == 5auto`, `i8()=5 == 5i8`, `u8()=5 == 5u8`, `U8(5) < U8(5)`, `5u8 < U8(5)`, `U8(5) < 5u8`, `5u8 < 5u8`, `5i8 < 5i8`, `5auto < U8(5)`, `5auto < 5u8`, `5auto < 5i8`, `-1i8 < 5auto`, `i8()=5 < 5auto`, `i8()=5 < 5i8`, `u8()=5 < 5u8`, `(5auto + 5auto) == U8(10)`, `SeqLength(seq) == U8(8)`
- **comparison**, all reject: `U8(5) == U16(5)`, `5u8 == 5u16`, `5i8 == 5u8`, `U8(5) < U16(5)`, `5u8 < 5u16`, `5i8 < 5u8`
- **arithmetic**, all accept: `U8(5) + U8(5)`, `5u8 + U8(5)`, `U8(5) + 5u8`, `5u8 + 5u8`, `5auto + U8(5)`, `5auto + 5u8`, `u8()=5 + 5u8`, `SeqLength(seq) + U64(1)`
- **arithmetic**, all reject: `U8(5) + U16(5)`, `5u8 + 5u16`, `5i8 + 5i8`, `5i8 + 5u8`, `5auto + 5i8`, `-1i8 + 5auto`, `i8()=5 + 5auto`, `i8()=5 + 5i8`
- **EnumFromTo bounds**, all accept: `EnumFromTo(U8(5), U8(5))`, `EnumFromTo(5u8, U8(5))`, `EnumFromTo(U8(5), 5u8)`, `EnumFromTo(5u8, 5u8)`, `EnumFromTo(5auto, U8(5))`, `EnumFromTo(5auto, 5u8)`, `EnumFromTo(5auto, 5auto)`, `EnumFromTo(u8()=5, 5u8)`
- **EnumFromTo bounds**, all reject: `EnumFromTo(U8(5), U16(5))`, `EnumFromTo(5u8, 5u16)`, `EnumFromTo(5i8, 5i8)`, `EnumFromTo(5i8, 5u8)`, `EnumFromTo(5auto, 5i8)`, `EnumFromTo(-1i8, 5auto)`, `EnumFromTo(i8()=5, 5auto)`, `EnumFromTo(i8()=5, 5i8)`
- **sequence literals/if-else branches**, all accept: `[U8(5), U8(5)]`, `[5u8, U8(5)]`, `[U8(5), 5u8]`, `[5u8, 5u8]`, `[5i8, 5i8]`, `[5auto, U8(5)]`, `[5auto, 5u8]`, `[5auto, 5i8]`, `[-1i8, 5auto]`, `[i8()=5, 5auto]`, `[i8()=5, 5i8]`, `[u8()=5, 5u8]`, `if _ then U8(5) else U8(5)`, `if _ then 5u8 else U8(5)`, `if _ then U8(5) else 5u8`, `if _ then 5u8 else 5u8`, `if _ then 5i8 else 5i8`, `if _ then 5auto else U8(5)`, `if _ then 5auto else 5u8`, `if _ then 5auto else 5i8`, `if _ then -1i8 else 5auto`, `if _ then i8()=5 else 5auto`, `if _ then i8()=5 else 5i8`, `if _ then u8()=5 else 5u8`
- **sequence literals/if-else branches**, all reject: `[U8(5), U16(5)]`, `[5u8, 5u16]`, `[5i8, 5u8]`, `if _ then U8(5) else U16(5)`, `if _ then 5u8 else 5u16`, `if _ then 5i8 else 5u8`

## Messages

1. Failed to infer module-wide type annotations: no unique solution for `?0 ∈ { U8, U16, U32, U64 }`
2. Failed to infer module-wide type annotations: no valid solutions for `?0`
3. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU8(Numeric(-1i8))")] }
4. Failed to infer module-wide type annotations: no unique solution for `?2 ∈ ℤ { I8, I16, I32, I64 }`
5. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Auto) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU8(Numeric(-1?))")] }
6. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU8(Var(\"x\"))")] }
7. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU32(Numeric(-1i8))")] }
8. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Auto) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU32(Numeric(-1?))")] }
9. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU32(Var(\"x\"))")] }
10. Failed to infer module-wide type annotations: no unique solution for `?2 ∈ { U8, U16, U32, U64 }`
11. Failed to infer module-wide type annotations: no valid solutions for `?2`
12. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Auto) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsChar(Numeric(-1?))")] }
13. Failed to infer module-wide type annotations: no valid solutions for `?12`
14. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto) to usize: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "SeqIx(Seq([U8(0), U8(1), U8(2), U8(3), U8(4), U8(5), U8(6), U8(7)]), Numeric(-1?))")] }
15. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto) to usize: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "SubSeq(Seq([U8(0), U8(1), U8(2), U8(3), U8(4), U8(5), U8(6), U8(7)]), Numeric(-1?), U32(1))")] }
16. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto) to usize: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "Dup(Numeric(-1?), U8(0))")] }
17. Failed to infer module-wide type annotations: unsatisfiable equivalence  `Elem(U(RankedUintSet { ranks: [Excluded, Excluded, Excluded, Excluded] })) = Equiv(Base(U8))` (
18. DecodeError { err: Eval(NumericConvert(out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "FindByKey(false, Lambda(\"elem\", Var(\"elem\")), Numeric(-1?), Seq([U8(5)]))")] }
19. Failed to infer module-wide type annotations: no unique solution for `?1 ∈ { U8, U16, U32, U64 }`
20. Failed to infer module-wide type annotations: no valid solutions for `?1`
21. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto) to usize: out of range conversion regarding big integer attempted)), _trace: [("RepeatCount(expr)", "Numeric(-1?)")] }
22. Failed to infer module-wide type annotations: no unique solution for `?3 ∈ { U8, U16, U32, U64 }`
23. Failed to infer module-wide type annotations: no valid solutions for `?3`
24. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto) to usize: out of range conversion regarding big integer attempted)), _trace: [("CaptureBytes(len)", "Numeric(-1?)"), ("LetView(parse)", "here_view", "CaptureBytes(Var(\"here_view\"), Numeric(-1?))")] }
25. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto) to usize: out of range conversion regarding big integer attempted)), _trace: [("ReadArray(len)", "Numeric(-1?)"), ("LetView(parse)", "here_view", "ReadArray(Var(\"here_view\"), Numeric(-1?), Base(U8))")] }
26. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto) to usize: out of range conversion regarding big integer attempted)), _trace: [("LetView(parse)", "v", "CaptureBytes(Offset(Var(\"v\"), Numeric(-1?)), U32(1))")] }
27. Failed to infer module-wide type annotations: unsatisfiable equivalence  `NumTree(Z(PrimIntSet { i8: At(3), i16: At(3), i32: At(3), i64: At(3) })) = Equiv(Int(Prim(U8)))` (
28. Failed to infer module-wide type annotations: no unique solution for `?2 ∈ ℤ { U8, U16, U32, U64, I8, I16, I32, I64 }`
29. Failed to infer module-wide type annotations: unsatisfiable equivalence  `NumTree(Z(PrimIntSet { i8: At(7), i16: At(7), i32: At(7), i64: At(7) })) = Equiv(Int(Prim(U8)))` (
30. Failed to infer module-wide type annotations: unsatisfiable equivalence  `NumTree(Z(PrimIntSet { i8: At(7), i16: At(7), i32: At(7), i64: At(7) })) = Equiv(Int(Prim(U16)))` (
31. Failed to infer module-wide type annotations: no unique solution for `?0 ∈ ℤ { U8, U16, U32, U64, I8, I16, I32, I64 }`

## Summary

289 cases (41 inconsistent): registration ok 197, codegen ok 160, interpreter ok 182; interp panics after registration accepts 0, codegen stricter 37, codegen looser 0
