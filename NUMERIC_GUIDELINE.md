# `doodle::numeric` Usage Constraints: Key Points

Terminology follows `doc/NUMERIC.md`: "Numeric" = `TypedConst` / `numeric::core::Expr`; "native" = core-grammar integer nodes; "Auto" = a Numeric with `NumRep::Auto`. The rules below are the outcome of `doc/NUMERIC_PLAN.md`; its decision table records why each one was chosen.

## Guiding principle
- If registration (legacy `infer_type`) and the `TypeChecker` **both reject** a construction, nothing downstream supports it.
- If they **both accept** it, the interpreter and codegen must both handle it without panicking: both succeed, or both return an error (not necessarily at the same place).
- `numeric_survey` (`doodle-formats/tests/signed_intops.rs`, run with `--ignored --nocapture`) checks this across all three layers. As of the plan's step 5, no layer panics on any surveyed case.

## Where Numerics can appear at all
- **`Expr`**: only through `Expr::Numeric(Box<NumExpr>)`. Inside that subtree, `NumVar` is the only way to reach native scope. A bound variable is converted into a Numeric by `TryFrom<&Value> for StrictValue` (`src/numeric/core.rs`):
  - `U8`–`U64` become concrete reps.
  - `Mapped`, `Branch` and `Permit(Ok|Err(Some))` are unwrapped transparently.
  - Everything else (Bool, Char, compound values, `Permit(Err(None))`) fails with `EvalError::BadVariable`.
- **`Format`**: `decoder.rs` has no signed or Numeric read primitive. At the interpreter level, a Numeric value only enters a Format through `Compute`/`Map` over `Expr::Numeric`. The exceptions are `ReadArray`'s `FixedReadKind`, which has signed kinds (`BaseNumType::Signed`) in both type checkers, and `Format::Pos`, typed `NumericHole` by registration and evaluated by the interpreter to an Auto Numeric (`Value::from_pos`), so it takes the type of whatever native operand it meets.
- **`Pattern`**: there's no Numeric pattern node. The numeric-model patterns are `ZConst(BigInt)` and `ZRange(NumBounds)`, which ignore representation. `U8`–`U64` and `Int(Bounds)` can also match against `Value::Numeric`.

## Type inference: registration vs `TypeChecker`
- **NumExpr typing** (`core.rs`):
  - `Const` takes the type of its rep (`Auto` → `NumericHole`).
  - An untagged `BinOp` unifies its operands, so `I8` with `U8` is an error, and `Auto`/`NumericHole` unifies with anything numeric.
  - A tagged `BinOp`/`UnaryOp` or a `Cast` takes the type of its `out_rep`.
  - `NumVar` takes the type of the variable in scope.
- **Registration's "unsigned or Auto" rule** (`ValueType::is_unsigned_or_auto`) accepts a native unsigned type or `NumericHole`, and rejects `Signed`. Where two operands are involved, they are unified first, so a signed operand can't hide behind an Auto one. It applies to:
  - native `Arith`, `IntSucc`/`IntPred` and `AsChar`;
  - `EnumFromTo` bounds, `CaptureBytes`/`ReadArray` lengths and `ViewExpr::Offset`;
  - `RepeatCount` and `RepeatBetween` counts;
  - the `Slice` length and the `WithRelativeOffset` base and offset.
- **Registration sites that keep `is_numeric()`** (which also accepts `Signed`): `IntRel` (the operands must still unify, so mixed-sign comparisons are rejected) and `AsU8`–`AsU64`.
- **Other registration sites**: the `FindByKey` key must unify to a native unsigned `Base` type (so `Signed` is rejected and an Auto query key is accepted only if the key lambda pins it). `SeqIx`, `SubSeq`, `SubSeqInflate` and `Dup` require exactly `U32`, which is deferred to the `seqlen-always-u32` project.
- **`RepeatBetween` bounds must be constant** in registration, the `TypeChecker` and the decoder compiler alike. All three use `Expr::exact_repeat_bounds`, which evaluates closed Numerics.
- **`TypeChecker`** (codegen path) constrains:
  - `BaseSet::UAny` (any unsigned type): `Arith`, `IntSucc`/`IntPred`, `AsChar`, `RepeatCount`, `RepeatBetween`, the `FindByKey` key, `CaptureBytes` length and `ViewExpr::Offset`.
  - `UAny32` (any unsigned type, with `U32` as the tiebreak default; it does **not** exclude `U64`): `SeqIx`, `EnumFromTo`, `SeqLength`, `Slice`, `WithRelativeOffset`. `SubSeq`'s start and `Dup`'s count behave the same way, so the `TypeChecker` is looser than registration's exact-`U32` rule at those sites.
  - `UintSet::ANY`: `ReadArray` length.
  - `IntSet::ZAny` (signed allowed): `AsU8`–`AsU64` and `IntRel`.
- **Unresolved Auto in the `TypeChecker`**: with nothing to pin it, an Auto const is ambiguous and `infer_module` returns a `TCError` ("no unique solution"); a negative Auto at an unsigned-only site gives "no valid solutions". Both go through `generate_code`'s "Failed to infer module-wide type annotations:" path. No default types are applied. Registration stays looser here: it accepts a bare Auto, and the interpreter handles it by value.
- **Pattern typing (registration)**: `build_scope` returns an error, never panics, on a pattern/type mismatch. `ZConst`/`ZRange` need a numeric scrutinee (signed OK). `U8`–`U64` and `Int` need the matching native unsigned type or a `NumericHole` scrutinee.
- **Pattern typing (`TypeChecker`)**: `ZConst`/`ZRange` restrict the scrutinee to primitive ints whose range contains the constant or bounds, so `ZConst(-1)` excludes every unsigned type. `Pattern::Int` restricts to `UintSet`, i.e. unsigned only.

## Interpreter: `Expr::eval`
- **`Expr::Numeric` uses the non-strict `NumExpr::eval`**, so representability isn't checked when the value is produced. A `Concrete(U32)` value can hold an out-of-range or negative `BigInt`; it is caught wherever the value next meets a concrete native type.
- **NumExpr eval errors**: `DivideByZero`, `RemainderNonPositive` (the divisor must be > 0), `Ambiguous(rep0, rep1)` (untagged `BinOp` over two different concrete reps; `Auto` defers to the other side), `UnknownVar`, `BadVariable`. Arithmetic is unbounded `BigInt` with no overflow checks. `Cast` is arithmetic (value kept, rep retagged) or bitwise (`bitwise_cast` reinterprets).
- **Native operations on a `Value::Numeric` operand** (`decoder/value.rs`):
  - **`int_rel`**: Numeric vs Numeric compares by value, ignoring rep. Numeric vs `U8`–`U64` uses `as_native` (value-only; errors if the value doesn't fit the native width).
  - **`arith`, Numeric vs native**: `get_as_unsigned`, which is rep-checked: the rep must be exactly the sibling's width or `Auto`, and the value must be representable.
  - **`arith`, Numeric vs Numeric**: the same concrete unsigned rep computes natively. **Auto with Auto computes by value and stays Auto** (division by zero and shifts outside `0..64` are errors). Mismatched concrete reps and signed reps still panic; registration rejects them, so these are type-checker-invariant panics.
  - **`unary` (`IntSucc`/`IntPred`)**: dispatches on the Numeric's own rep. Concrete `U8`–`U64` compute natively; **Auto is ±1 by value and stays Auto**; signed panics (rejected by registration).
  - **`AsU8`–`AsU64`/`AsChar`**: `as_native`, value-only and rep-agnostic; errors if the value doesn't fit.
  - **`as_usize` sites** (lengths, counts, offsets, `EnumFromTo` bounds): value-only; negative or oversized values error.
  - **`FindByKey` key (`AsKey`)**: Numeric vs `U8`–`U64` is coerced with `get_as_unsigned`; **an out-of-range key (e.g. `-1auto` against `U8`) returns `EvalError::NumericConvert`.**
- **Pattern matching on `Value::Numeric`** (`Value::matches_numeric_literal`, shared by the main interpreter and `loc_decoder`):
  - `U8(n)`–`U64(n)`: a concrete rep must be exactly that width; **Auto is compared by value**.
  - `Int(bounds)`: value-only; negative values never match.
  - `ZConst`/`ZRange`: value-only, rep ignored. These also work on native `U8`–`U64` values.

## Static analysis (`Expr::bounds`)
- `Expr::Numeric` gets exact bounds only when the tree is closed (evaluated with `eval_strict` against the empty `VOID` scope, so any `NumVar` makes it `Any`), strictly valid, and fits in `usize`. Otherwise it's `Bounds::any()`.
- Codegen's `TypedExpr::bounds` converts to `Expr` and reuses `Expr::bounds`, so codegen accepts exactly the same constant `RepeatBetween` bounds.
- `mk_value_expr` placeholders: `Signed(t)` → `Const(-1 : t)`; `NumericHole` → `Const(0u8)`.

## Codegen
- Embedded trees are elaborated to `TypedNumExpr` and emitted via `numeric::codegen::synthesize`. Operations classified `HomLossy`/`HetLossy` fall back to `eval_fallback` instead of boilerplated backend functions (`numeric/codegen.rs`).
- `ZConst`/`ZRange` are emitted as untyped `SomeInt` literals and ranges, relying on Rust's type inference against the scrutinee.
- `IntRel` is emitted as a plain infix comparison, which is also correct for signed operands.
- **Match exhaustiveness** (`refutability_check` in `codegen/mod.rs`):
  - A signed scrutinee is always treated as `Refutable`, so a fallback arm is always emitted.
  - For an unsigned scrutinee, `IntCoverage` counts `U8`–`U64`, `Int`, and `ZConst`/`ZRange` arms (the latter clamped to the unsigned domain).

## Known gaps
- **`src/alt.rs`** mirrors registration's "unsigned or Auto" rule and pattern errors, but its `IntRel` and `AsU8`–`AsU64` still require a native unsigned `Base` type, and it types `Pos` as `U64`.
- Not yet surveyed: `output/tree.rs`, `read.rs`, `helper.rs` (e.g. how an `i8()` format is built) and the rest of `numeric/eval.rs`/`elaborator.rs`. The survey doesn't compile the code that codegen emits.
