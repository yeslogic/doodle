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
| `IntSucc(5i8)` | ✓ | ✗ #1 | ! #2 | interp panics after registration accepts |
| `IntSucc(-1i8)` | ✓ | ✗ #1 | ! #3 | interp panics after registration accepts |
| `IntSucc(5auto)` | ✓ | ! #4 | ! #5 | interp panics after registration accepts |
| `IntSucc(-1auto)` | ✓ | ! #6 | ! #7 | interp panics after registration accepts |
| `IntSucc(i8()=5)` | ✓ | ✗ #1 | ! #2 | interp panics after registration accepts |
| `IntSucc(i8()=-1)` | ✓ | ✗ #1 | ! #3 | interp panics after registration accepts |
| `IntPred(5i8)` | ✓ | ✗ #1 | ! #2 | interp panics after registration accepts |
| `IntPred(-1i8)` | ✓ | ✗ #1 | ! #3 | interp panics after registration accepts |
| `IntPred(5auto)` | ✓ | ! #4 | ! #5 | interp panics after registration accepts |
| `IntPred(-1auto)` | ✓ | ! #6 | ! #7 | interp panics after registration accepts |
| `IntPred(i8()=5)` | ✓ | ✗ #1 | ! #2 | interp panics after registration accepts |
| `IntPred(i8()=-1)` | ✓ | ✗ #1 | ! #3 | interp panics after registration accepts |
| `AsU8(-1i8)` | ✓ | ✓ | ✗ #8 |  |
| `AsU8(5auto)` | ✓ | ! #9 | ✓ | codegen panics |
| `AsU8(-1auto)` | ✓ | ! #10 | ✗ #11 | codegen panics |
| `AsU8(i8()=-1)` | ✓ | ✓ | ✗ #12 |  |
| `AsU32(-1i8)` | ✓ | ✓ | ✗ #13 |  |
| `AsU32(5auto)` | ✓ | ! #9 | ✓ | codegen panics |
| `AsU32(-1auto)` | ✓ | ! #10 | ✗ #14 | codegen panics |
| `AsU32(i8()=-1)` | ✓ | ✓ | ✗ #15 |  |
| `AsChar(5i8)` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `AsChar(-1i8)` | ✓ | ✗ #1 | ✗ #16 | codegen stricter |
| `AsChar(5auto)` | ✓ | ! #17 | ✓ | codegen panics |
| `AsChar(-1auto)` | ✓ | ! #18 | ✗ #19 | codegen panics |
| `AsChar(i8()=5)` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `AsChar(i8()=-1)` | ✓ | ✗ #1 | ✗ #20 | codegen stricter |
| `SeqIx(seq, U8(5))` | ✗ #21 | ✓ | – | codegen looser |
| `SeqIx(seq, 5u8)` | ✗ #21 | ✓ | – | codegen looser |
| `SeqIx(seq, 5u16)` | ✗ #22 | ✓ | – | codegen looser |
| `SeqIx(seq, 5auto)` | ✗ #23 | ✓ | – | codegen looser |
| `SeqIx(seq, -1auto)` | ✗ #23 | ! #24 | – | codegen panics |
| `SeqIx(seq, u8()=5)` | ✗ #21 | ✓ | – | codegen looser |
| `SubSeq(seq, U8(5), 1)` | ✗ #25 | ✓ | – | codegen looser |
| `SubSeq(seq, 5u8, 1)` | ✗ #25 | ✓ | – | codegen looser |
| `SubSeq(seq, 5u16, 1)` | ✗ #26 | ✓ | – | codegen looser |
| `SubSeq(seq, 5auto, 1)` | ✗ #27 | ✓ | – | codegen looser |
| `SubSeq(seq, -1auto, 1)` | ✗ #27 | ! #24 | – | codegen panics |
| `SubSeq(seq, u8()=5, 1)` | ✗ #25 | ✓ | – | codegen looser |
| `Dup(U8(5), U8(0))` | ✗ #28 | ✓ | – | codegen looser |
| `Dup(5u8, U8(0))` | ✗ #29 | ✓ | – | codegen looser |
| `Dup(5u16, U8(0))` | ✗ #30 | ✓ | – | codegen looser |
| `Dup(5auto, U8(0))` | ✗ #31 | ✓ | – | codegen looser |
| `Dup(-1auto, U8(0))` | ✗ #32 | ! #18 | – | codegen panics |
| `Dup(u8()=5, U8(0))` | ✗ #33 | ✓ | – | codegen looser |
| `FindByKey(id, 5u8, [U8(5)])` | ✓ | ✓ | ! #34 | interp panics after registration accepts |
| `FindByKey(id, 5auto, [U8(5)])` | ✓ | ✓ | ! #35 | interp panics after registration accepts |
| `FindByKey(id, -1auto, [U8(5)])` | ✓ | ✗ #36 | ! #37 | interp panics after registration accepts |
| `RepeatCount(5auto, u8)` | ✗ #38 | ! #39 | – | codegen panics |
| `RepeatCount(-1auto, u8)` | ✗ #38 | ! #40 | – | codegen panics |
| `CaptureBytes(5i8)` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `CaptureBytes(-1i8)` | ✓ | ✗ #1 | ✗ #41 | codegen stricter |
| `CaptureBytes(5auto)` | ✓ | ! #42 | ✓ | codegen panics |
| `CaptureBytes(-1auto)` | ✓ | ! #43 | ✗ #44 | codegen panics |
| `CaptureBytes(i8()=5)` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `CaptureBytes(i8()=-1)` | ✓ | ✗ #1 | ✗ #45 | codegen stricter |
| `ReadArray(5i8, U8)` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `ReadArray(-1i8, U8)` | ✓ | ✗ #1 | ✗ #46 | codegen stricter |
| `ReadArray(5auto, U8)` | ✓ | ! #42 | ✓ | codegen panics |
| `ReadArray(-1auto, U8)` | ✓ | ! #43 | ✗ #47 | codegen panics |
| `ReadArray(i8()=5, U8)` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `ReadArray(i8()=-1, U8)` | ✓ | ✗ #1 | ✗ #48 | codegen stricter |
| `Offset(v, 5i8)` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `Offset(v, -1i8)` | ✓ | ✗ #1 | ✗ #49 | codegen stricter |
| `Offset(v, 5auto)` | ✓ | ! #17 | ✓ | codegen panics |
| `Offset(v, -1auto)` | ✓ | ! #18 | ✗ #50 | codegen panics |
| `Offset(v, i8()=5)` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `Offset(v, i8()=-1)` | ✓ | ✗ #1 | ✗ #51 | codegen stricter |
| `5u16 ~ U8(5)` | ! #52 | ✗ #53 | – |  |
| `5i8 ~ U8(5)` | ! #54 | ✗ #55 | – |  |
| `-1i8 ~ U8(5)` | ! #54 | ✗ #55 | – |  |
| `5auto ~ U8(5)` | ! #56 | ✓ | – | codegen looser |
| `-1auto ~ U8(5)` | ! #56 | ✗ #36 | – |  |
| `i8()=5 ~ U8(5)` | ! #54 | ✗ #55 | – |  |
| `i8()=-1 ~ U8(5)` | ! #54 | ✗ #55 | – |  |
| `5i8 ~ Int(0..=10)` | ! #57 | ✗ #1 | – |  |
| `-1i8 ~ Int(0..=10)` | ! #57 | ✗ #1 | – |  |
| `5auto ~ Int(0..=10)` | ! #58 | ! #17 | – | codegen panics |
| `-1auto ~ Int(0..=10)` | ! #58 | ! #18 | – | codegen panics |
| `i8()=5 ~ Int(0..=10)` | ! #57 | ✗ #1 | – |  |
| `i8()=-1 ~ Int(0..=10)` | ! #57 | ✗ #1 | – |  |
| `5auto ~ ZConst(5)` | ✓ | ! #9 | ✓ | codegen panics |
| `-1auto ~ ZConst(5)` | ✓ | ! #10 | ✓ | codegen panics |
| `U8(5) ~ ZConst(-1)` | ✓ | ✗ #59 | ✓ | codegen stricter |
| `5u8 ~ ZConst(-1)` | ✓ | ✗ #59 | ✓ | codegen stricter |
| `5u16 ~ ZConst(-1)` | ✓ | ✗ #60 | ✓ | codegen stricter |
| `5auto ~ ZConst(-1)` | ✓ | ! #10 | ✓ | codegen panics |
| `-1auto ~ ZConst(-1)` | ✓ | ! #10 | ✓ | codegen panics |
| `u8()=5 ~ ZConst(-1)` | ✓ | ✗ #59 | ✓ | codegen stricter |
| `5i8 == 5i8` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `5auto == 5i8` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `5auto == 5auto` | ✓ | ! #17 | ✓ | codegen panics |
| `-1i8 == 5auto` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `i8()=5 == 5auto` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `i8()=5 == 5i8` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `5i8 < 5i8` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `5auto < 5i8` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `5auto < 5auto` | ✓ | ! #17 | ✓ | codegen panics |
| `-1i8 < 5auto` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `i8()=5 < 5auto` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `i8()=5 < 5i8` | ✓ | ✗ #1 | ✓ | codegen stricter |
| `5i8 + 5i8` | ✓ | ✗ #1 | ! #61 | interp panics after registration accepts |
| `5auto + 5i8` | ✓ | ✗ #1 | ! #62 | interp panics after registration accepts |
| `5auto + 5auto` | ✓ | ! #4 | ! #63 | interp panics after registration accepts |
| `-1i8 + 5auto` | ✓ | ✗ #1 | ! #64 | interp panics after registration accepts |
| `i8()=5 + 5auto` | ✓ | ✗ #1 | ! #65 | interp panics after registration accepts |
| `i8()=5 + 5i8` | ✓ | ✗ #1 | ! #61 | interp panics after registration accepts |
| `EnumFromTo(5i8, 5i8)` | ✓ | ✗ #66 | ✓ | codegen stricter |
| `EnumFromTo(5auto, 5i8)` | ✓ | ✗ #66 | ✓ | codegen stricter |
| `EnumFromTo(-1i8, 5auto)` | ✓ | ✗ #66 | ✗ #67 | codegen stricter |
| `EnumFromTo(i8()=5, 5auto)` | ✓ | ✗ #66 | ✓ | codegen stricter |
| `EnumFromTo(i8()=5, 5i8)` | ✓ | ✗ #66 | ✓ | codegen stricter |
| `[5auto, 5auto]` | ✓ | ! #9 | ✓ | codegen panics |
| `if _ then 5auto else 5auto` | ✓ | ! #68 | ✓ | codegen panics |
| `RepeatBetween(5u8, U8(5), u8)` | ✓ | ! #69 | ✓ | codegen panics |
| `RepeatBetween(U8(5), 5u8, u8)` | ✓ | ! #69 | ✓ | codegen panics |
| `RepeatBetween(5u8, 5u8, u8)` | ✓ | ! #69 | ✓ | codegen panics |
| `RepeatBetween(5auto, U8(5), u8)` | ✗ #70 | ! #69 | – | codegen panics |
| `RepeatBetween(5auto, 5auto, u8)` | ✗ #70 | ! #39 | – | codegen panics |
| `RepeatBetween(u8()=5, 5u8, u8)` | ! #69 | ! #69 | – | codegen panics |

## Consistent cases (omitted from the table)

- **increment/decrement**, all accept: `IntSucc(U8(5))`, `IntSucc(5u8)`, `IntSucc(5u16)`, `IntSucc(u8()=5)`, `IntPred(U8(5))`, `IntPred(5u8)`, `IntPred(5u16)`, `IntPred(u8()=5)`
- **casts**, all accept: `AsU8(U8(5))`, `AsU8(5u8)`, `AsU8(5u16)`, `AsU8(5i8)`, `AsU8(i8()=5)`, `AsU8(u8()=5)`, `AsU32(U8(5))`, `AsU32(5u8)`, `AsU32(5u16)`, `AsU32(5i8)`, `AsU32(i8()=5)`, `AsU32(u8()=5)`, `AsChar(U8(5))`, `AsChar(5u8)`, `AsChar(5u16)`, `AsChar(u8()=5)`
- **sequence index/length arguments**, all reject: `SeqIx(seq, 5i8)`, `SeqIx(seq, -1i8)`, `SeqIx(seq, i8()=5)`, `SeqIx(seq, i8()=-1)`, `SubSeq(seq, 5i8, 1)`, `SubSeq(seq, -1i8, 1)`, `SubSeq(seq, i8()=5, 1)`, `SubSeq(seq, i8()=-1, 1)`, `Dup(5i8, U8(0))`, `Dup(-1i8, U8(0))`, `Dup(i8()=5, U8(0))`, `Dup(i8()=-1, U8(0))`
- **FindByKey keys**, all accept: `FindByKey(id, U8(5), [U8(5)])`, `FindByKey(id, u8()=5, [U8(5)])`
- **FindByKey keys**, all reject: `FindByKey(id, 5u16, [U8(5)])`, `FindByKey(id, 5i8, [U8(5)])`, `FindByKey(id, -1i8, [U8(5)])`, `FindByKey(id, i8()=5, [U8(5)])`, `FindByKey(id, i8()=-1, [U8(5)])`
- **repetition counts**, all accept: `RepeatCount(U8(5), u8)`, `RepeatCount(5u8, u8)`, `RepeatCount(5u16, u8)`, `RepeatCount(u8()=5, u8)`, `RepeatBetween(U8(5), U8(5), u8)`
- **repetition counts**, all reject: `RepeatCount(5i8, u8)`, `RepeatCount(-1i8, u8)`, `RepeatCount(i8()=5, u8)`, `RepeatCount(i8()=-1, u8)`, `RepeatBetween(U8(5), U16(5), u8)`, `RepeatBetween(5u8, 5u16, u8)`, `RepeatBetween(5i8, 5i8, u8)`, `RepeatBetween(5i8, 5u8, u8)`, `RepeatBetween(5auto, 5i8, u8)`, `RepeatBetween(-1i8, 5auto, u8)`, `RepeatBetween(i8()=5, 5auto, u8)`, `RepeatBetween(i8()=5, 5i8, u8)`
- **view lengths/offsets**, all accept: `CaptureBytes(U8(5))`, `CaptureBytes(5u8)`, `CaptureBytes(5u16)`, `CaptureBytes(u8()=5)`, `ReadArray(U8(5), U8)`, `ReadArray(5u8, U8)`, `ReadArray(5u16, U8)`, `ReadArray(u8()=5, U8)`, `Offset(v, U8(5))`, `Offset(v, 5u8)`, `Offset(v, 5u16)`, `Offset(v, u8()=5)`
- **pattern matching**, all accept: `U8(5) ~ U8(5)`, `5u8 ~ U8(5)`, `u8()=5 ~ U8(5)`, `U8(5) ~ Int(0..=10)`, `5u8 ~ Int(0..=10)`, `5u16 ~ Int(0..=10)`, `u8()=5 ~ Int(0..=10)`, `U8(5) ~ ZConst(5)`, `5u8 ~ ZConst(5)`, `5u16 ~ ZConst(5)`, `5i8 ~ ZConst(5)`, `-1i8 ~ ZConst(5)`, `i8()=5 ~ ZConst(5)`, `i8()=-1 ~ ZConst(5)`, `u8()=5 ~ ZConst(5)`, `5i8 ~ ZConst(-1)`, `-1i8 ~ ZConst(-1)`, `i8()=5 ~ ZConst(-1)`, `i8()=-1 ~ ZConst(-1)`
- **comparison**, all accept: `U8(5) == U8(5)`, `5u8 == U8(5)`, `U8(5) == 5u8`, `5u8 == 5u8`, `5auto == U8(5)`, `u8()=5 == 5u8`, `U8(5) < U8(5)`, `5u8 < U8(5)`, `U8(5) < 5u8`, `5u8 < 5u8`, `5auto < U8(5)`, `u8()=5 < 5u8`
- **comparison**, all reject: `U8(5) == U16(5)`, `5u8 == 5u16`, `5i8 == 5u8`, `U8(5) < U16(5)`, `5u8 < 5u16`, `5i8 < 5u8`
- **arithmetic**, all accept: `U8(5) + U8(5)`, `5u8 + U8(5)`, `U8(5) + 5u8`, `5u8 + 5u8`, `5auto + U8(5)`, `u8()=5 + 5u8`
- **arithmetic**, all reject: `U8(5) + U16(5)`, `5u8 + 5u16`, `5i8 + 5u8`
- **EnumFromTo bounds**, all accept: `EnumFromTo(U8(5), U8(5))`, `EnumFromTo(5u8, U8(5))`, `EnumFromTo(U8(5), 5u8)`, `EnumFromTo(5u8, 5u8)`, `EnumFromTo(5auto, U8(5))`, `EnumFromTo(5auto, 5auto)`, `EnumFromTo(u8()=5, 5u8)`
- **EnumFromTo bounds**, all reject: `EnumFromTo(U8(5), U16(5))`, `EnumFromTo(5u8, 5u16)`, `EnumFromTo(5i8, 5u8)`
- **sequence literals/if-else branches**, all accept: `[U8(5), U8(5)]`, `[5u8, U8(5)]`, `[U8(5), 5u8]`, `[5u8, 5u8]`, `[5i8, 5i8]`, `[5auto, U8(5)]`, `[5auto, 5i8]`, `[-1i8, 5auto]`, `[i8()=5, 5auto]`, `[i8()=5, 5i8]`, `[u8()=5, 5u8]`, `if _ then U8(5) else U8(5)`, `if _ then 5u8 else U8(5)`, `if _ then U8(5) else 5u8`, `if _ then 5u8 else 5u8`, `if _ then 5i8 else 5i8`, `if _ then 5auto else U8(5)`, `if _ then 5auto else 5i8`, `if _ then -1i8 else 5auto`, `if _ then i8()=5 else 5auto`, `if _ then i8()=5 else 5i8`, `if _ then u8()=5 else 5u8`
- **sequence literals/if-else branches**, all reject: `[U8(5), U16(5)]`, `[5u8, 5u16]`, `[5i8, 5u8]`, `if _ then U8(5) else U16(5)`, `if _ then 5u8 else 5u16`, `if _ then 5i8 else 5u8`

## Messages

1. Failed to infer module-wide type annotations: cross-layer numeric error: PrimInt not in BaseSet: `i8` ∉ `{ U8, U16, U32, U64 }` (
2. top-level unary operations should not be performed on raw-numeric with signed or auto representation (Numeric(TypedConst(5, Concrete(MachineRep { is_signed: true, bit_width: Bits8 }))))
3. top-level unary operations should not be performed on raw-numeric with signed or auto representation (Numeric(TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 }))))
4. no unique solution for `?0 ∈ { U8, U16, U32, U64 }`
5. top-level unary operations should not be performed on raw-numeric with signed or auto representation (Numeric(TypedConst(5, Auto)))
6. no valid solutions for `?0`
7. top-level unary operations should not be performed on raw-numeric with signed or auto representation (Numeric(TypedConst(-1, Auto)))
8. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU8(Numeric(-1i8))")] }
9. no unique solution for `?2 ∈ ℤ { U8, U16, U32, U64, I8, I16, I32, I64 }`
10. no unique solution for `?2 ∈ ℤ { I8, I16, I32, I64 }`
11. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Auto) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU8(Numeric(-1?))")] }
12. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU8(Var(\"x\"))")] }
13. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU32(Numeric(-1i8))")] }
14. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Auto) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU32(Numeric(-1?))")] }
15. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsU32(Var(\"x\"))")] }
16. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsChar(Numeric(-1i8))")] }
17. no unique solution for `?2 ∈ { U8, U16, U32, U64 }`
18. no valid solutions for `?2`
19. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Auto) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsChar(Numeric(-1?))")] }
20. DecodeError { err: Eval(NumericConvert(TypedConst::as_native: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to target width: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "AsChar(Var(\"x\"))")] }
21. SeqIx `index` param: expected U32, found Base(U8)
22. SeqIx `index` param: expected U32, found Base(U16)
23. SeqIx `index` param: expected U32, found NumericHole
24. no valid solutions for `?12`
25. SubSeq `start` param: expected U32, found Base(U8)
26. SubSeq `start` param: expected U32, found Base(U16)
27. SubSeq `start` param: expected U32, found NumericHole
28. Dup: count is not U32: U8(5)
29. Dup: count is not U32: Numeric(5u8)
30. Dup: count is not U32: Numeric(5u16)
31. Dup: count is not U32: Numeric(5?)
32. Dup: count is not U32: Numeric(-1?)
33. Dup: count is not U32: Var("x")
34. Value::eq_key: can't compare Numeric(TypedConst(5, Concrete(MachineRep { is_signed: false, bit_width: Bits8 }))) and U8(5) as keys
35. Value::eq_key: can't compare Numeric(TypedConst(5, Auto)) and U8(5) as keys
36. Failed to infer module-wide type annotations: unsatisfiable equivalence  `NumTree(Z(PrimIntSet { i8: At(3), i16: At(3), i32: At(3), i64: At(3) })) = Equiv(Int(Prim(U8)))` (
37. Value::eq_key: can't compare Numeric(TypedConst(-1, Auto)) and U8(5) as keys
38. RepeatCount first argument type should be numeric, found NumericHole instead
39. no unique solution for `?1 ∈ { U8, U16, U32, U64 }`
40. no valid solutions for `?1`
41. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to usize: out of range conversion regarding big integer attempted)), _trace: [("CaptureBytes(len)", "Numeric(-1i8)"), ("LetView(parse)", "here_view", "CaptureBytes(Var(\"here_view\"), Numeric(-1i8))")] }
42. no unique solution for `?3 ∈ { U8, U16, U32, U64 }`
43. no valid solutions for `?3`
44. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto) to usize: out of range conversion regarding big integer attempted)), _trace: [("CaptureBytes(len)", "Numeric(-1?)"), ("LetView(parse)", "here_view", "CaptureBytes(Var(\"here_view\"), Numeric(-1?))")] }
45. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to usize: out of range conversion regarding big integer attempted)), _trace: [("CaptureBytes(len)", "Var(\"x\")"), ("LetView(parse)", "here_view", "CaptureBytes(Var(\"here_view\"), Var(\"x\"))")] }
46. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to usize: out of range conversion regarding big integer attempted)), _trace: [("ReadArray(len)", "Numeric(-1i8)"), ("LetView(parse)", "here_view", "ReadArray(Var(\"here_view\"), Numeric(-1i8), Base(U8))")] }
47. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto) to usize: out of range conversion regarding big integer attempted)), _trace: [("ReadArray(len)", "Numeric(-1?)"), ("LetView(parse)", "here_view", "ReadArray(Var(\"here_view\"), Numeric(-1?), Base(U8))")] }
48. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to usize: out of range conversion regarding big integer attempted)), _trace: [("ReadArray(len)", "Var(\"x\")"), ("LetView(parse)", "here_view", "ReadArray(Var(\"here_view\"), Var(\"x\"), Base(U8))")] }
49. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to usize: out of range conversion regarding big integer attempted)), _trace: [("LetView(parse)", "v", "CaptureBytes(Offset(Var(\"v\"), Numeric(-1i8)), U32(1))")] }
50. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto) to usize: out of range conversion regarding big integer attempted)), _trace: [("LetView(parse)", "v", "CaptureBytes(Offset(Var(\"v\"), Numeric(-1?)), U32(1))")] }
51. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to usize: out of range conversion regarding big integer attempted)), _trace: [("LetView(parse)", "v", "CaptureBytes(Offset(Var(\"v\"), Var(\"x\")), U32(1))")] }
52. pattern build_scope failed: (U8(5), Base(U16))
53. Failed to infer module-wide type annotations: cross-layer numeric error: non-matching PrimInt and BaseType: `u16` ≄ `u8` (
54. pattern build_scope failed: (U8(5), Signed(I8))
55. Failed to infer module-wide type annotations: cross-layer numeric error: non-matching PrimInt and BaseType: `i8` ≄ `u8` (
56. pattern build_scope failed: (U8(5), NumericHole)
57. pattern build_scope failed: (Int(Bounds { min: 0, max: Some(10) }), Signed(I8))
58. pattern build_scope failed: (Int(Bounds { min: 0, max: Some(10) }), NumericHole)
59. Failed to infer module-wide type annotations: unsatisfiable equivalence  `NumTree(Z(PrimIntSet { i8: At(7), i16: At(7), i32: At(7), i64: At(7) })) = Equiv(Int(Prim(U8)))` (
60. Failed to infer module-wide type annotations: unsatisfiable equivalence  `NumTree(Z(PrimIntSet { i8: At(7), i16: At(7), i32: At(7), i64: At(7) })) = Equiv(Int(Prim(U16)))` (
61. cannot apply native-arith Add to signed-rep (`TypedConst(5, Concrete(MachineRep { is_signed: true, bit_width: Bits8 }))`, `TypedConst(5, Concrete(MachineRep { is_signed: true, bit_width: Bits8 }))`)
62. cannot apply native-arith Add to signed-rep (`TypedConst(5, Auto)`, `TypedConst(5, Concrete(MachineRep { is_signed: true, bit_width: Bits8 }))`)
63. cannot apply native-arith Add to auto-or-mismatched (`TypedConst(5, Auto)`, `TypedConst(5, Auto)`)
64. cannot apply native-arith Add to signed-rep (`TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 }))`, `TypedConst(5, Auto)`)
65. cannot apply native-arith Add to signed-rep (`TypedConst(5, Concrete(MachineRep { is_signed: true, bit_width: Bits8 }))`, `TypedConst(5, Auto)`)
66. Failed to infer module-wide type annotations: cross-layer numeric error: PrimInt not in BaseSet: `i8` ∉ `{ U32 > U8, U16, U64 }` (
67. DecodeError { err: Eval(NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Concrete(MachineRep { is_signed: true, bit_width: Bits8 })) to usize: out of range conversion regarding big integer attempted)), _trace: [("Compute(expr)", "EnumFromTo(Numeric(-1i8), Numeric(5?))")] }
68. no unique solution for `?0 ∈ ℤ { U8, U16, U32, U64, I8, I16, I32, I64 }`
69. not implemented: RepeatBetween on inexact bounds-expr
70. RepeatBetween first argument type should be numeric, found NumericHole instead

## Summary

275 cases (119 inconsistent): registration ok 192, codegen ok 131, interpreter ok 152; interp panics after registration accepts 21, codegen stricter 35, codegen looser 16
