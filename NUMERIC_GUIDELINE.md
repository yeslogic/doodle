# `doodle::numeric` Usage Constraints: Key Points

Terminology follows `doc/NUMERIC.md`: "Numeric" = `TypedConst` / `numeric::core::Expr`; "native" = core-grammar integer nodes.

## Where Numerics can appear at all
- **`Expr`**: only through `Expr::Numeric(Box<NumExpr>)`. Inside that subtree, `NumVar` is the only way to reach native scope. A bound variable is converted into a Numeric by `TryFrom<&Value> for StrictValue` (`src/numeric/core.rs:1148`):
  - `U8`–`U64` become concrete reps.
  - `Usize` becomes `Auto`, with a warning.
  - `Mapped`, `Branch` and `Permit(Ok|Err(Some))` are unwrapped transparently.
  - Everything else (Bool, Char, compound values, `Permit(Err(None))`) fails with `EvalError::BadVariable`.
- **`Format`**: `decoder.rs` has no signed or Numeric read primitive. At the interpreter level, a Numeric value only enters a Format through `Compute`/`Map` over `Expr::Numeric`. The exceptions are `ReadArray`'s `FixedReadKind`, which has signed kinds (`BaseNumType::Signed`) in both type checkers, and `Format::Pos`, typed `NumericHole` (it may elaborate to a signed type in codegen, `typed_decoder.rs:673`).
- **`Pattern`**: there's no Numeric pattern node. The numeric-model patterns are `ZConst(BigInt)` and `ZRange(NumBounds)`, which ignore representation. `U8`–`U64` and `Int(Bounds)` can also match against `Value::Numeric`.

## Type inference: legacy `infer_type` vs `TypeChecker`
- **NumExpr typing** (`core.rs:1042`):
  - `Const` takes the type of its rep (`Auto` → `NumericHole`).
  - An untagged `BinOp` unifies its operands, so `I8` with `U8` is an error, and `Auto`/`NumericHole` unifies with anything numeric.
  - A tagged `BinOp`/`UnaryOp` or a `Cast` takes the type of its `out_rep`.
  - `NumVar` takes the type of the variable in scope.
- **Legacy `ValueType::is_numeric()`** counts `Signed` and `NumericHole` as numeric. They're accepted by: `IntRel`, `Arith`, `IntSucc`/`IntPred`, `As{U8..U64,Char}`, `EnumFromTo`, `ViewExpr::Offset`, `CaptureBytes`, `ReadArray`.
- **Legacy sites that require `ValueType::Base(b) if b.is_numeric()`** reject both `Signed` and `NumericHole`: `RepeatCount` and `RepeatBetween`. `SeqIx`, `SubSeq` and `Dup` require exactly `U32`.
- **The `FindByKey` key (legacy)** rejects `Signed` but accepts `NumericHole` (survey: `5auto` and `-1auto` keys both pass registration). The survey doesn't show whether `Signed` is rejected by a `Base` guard or by failing to unify with the table's key type.
- **`TypeChecker`** (codegen path) is stricter. It constrains to *unsigned* sets:
  - `BaseSet::UAny`: `Arith`, `IntRel`, `IntSucc`/`IntPred`, `AsChar`, `RepeatCount`, `RepeatBetween`.
  - `UAny32`: `SeqIx`, `EnumFromTo`, `SeqLength`. `SubSeq`'s start and `Dup`'s count behave the same way in the survey (`u8`/`u16`/`Auto` accepted, `i8` rejected), so the TypeChecker is looser than legacy's exact-`U32` rule at all three sites.
  - `UintSet::ANY`: `CaptureBytes`/`ReadArray` length and `ViewExpr::Offset`.
  - Only `AsU8`–`AsU64` accept `IntSet::ZAny`, i.e. signed operands.

  **So a signed Numeric feeding any native operation except `AsUN` passes legacy `infer_type` but fails `TypeChecker`.** `Auto` trees can still be resolved to an unsigned type, but only when something in the context pins the type.
- **Unresolved `Auto` in the `TypeChecker`**:
  - With nothing to pin it, an `Auto` const is ambiguous, and `generate_code` panics with "no unique solution". This happens under `UAny`/`UintSet` (`5auto + 5auto`, `CaptureBytes(5auto)`, `RepeatCount(5auto)`), under `ZAny` (`AsU8(5auto)`), as a `ZConst` scrutinee, and in `[5auto, 5auto]` / `if … 5auto else 5auto`.
  - A negative `Auto` const is restricted to `{I8..I64}`, so at any unsigned-only site it panics with "no valid solutions".
  - Neither panic carries `generate_code`'s "Failed to infer module-wide type annotations:" prefix, so they bypass its normal rejection path.
- **Pattern typing (legacy)**: `ZConst`/`ZRange` need a scrutinee that is numeric in the broad sense (signed OK). `Pattern::Int` and `Pattern::U8`–`U64` need a native unsigned `Base` scrutinee of the matching type. **Any mismatch panics in `build_scope` instead of returning an error**, including a `Signed` or `NumericHole` scrutinee and a mismatched native width (`5u16 ~ U8(5)`). There's a `REVIEW` note on this at `pattern.rs:72`. So an `Auto` scrutinee can't be matched with `Int` or `U8`–`U64` at all, and the TypeChecker hits a no-unique-solution panic on `5auto ~ Int(..)` as well; `5auto ~ U8(5)` passes codegen only.
- **Pattern typing (`TypeChecker`)**: `ZConst`/`ZRange` restrict the scrutinee to primitive ints whose range contains the constant or bounds, so `ZConst(-1)` excludes every unsigned type. `Pattern::Int` restricts to `UintSet`, i.e. unsigned only.

## Interpreter: `Expr::eval`
- **`Expr::Numeric` uses the non-strict `NumExpr::eval`** (`decoder/eval.rs:97`), so representability isn't checked when the value is produced. A `Concrete(U32)` value can hold an out-of-range or negative `BigInt`.
- **NumExpr eval errors**: `DivideByZero`, `RemainderNonPositive` (the divisor must be > 0), `Ambiguous(rep0, rep1)` (untagged `BinOp` over two different concrete reps; `Auto` defers to the other side), `UnknownVar`, `BadVariable`. Arithmetic is unbounded `BigInt` with no overflow checks. `Cast` is arithmetic (value kept, rep retagged) or bitwise (`bitwise_cast` reinterprets).
- **Native operations on a `Value::Numeric` operand** (`decoder/value.rs`):
  - **`int_rel`**: Numeric vs Numeric compares by value, ignoring rep. Numeric vs `U8`–`U64` uses `as_native` (value-only; errors if the value doesn't fit the native width). **Numeric vs `Usize` panics.**
  - **`arith`**: Numeric vs `U8`–`U64` uses `get_as_unsigned`, which is rep-checked: the rep must be exactly the sibling's width or `Auto`, and the value must be representable. **Numeric vs `Usize` panics.**
  - **`arith`, Numeric vs Numeric**: works only when both reps are the same concrete *unsigned* width (`5u8 + 5u8`). **It panics if either rep is signed**, even when both match (`5i8 + 5i8`: "cannot apply native-arith Add to signed-rep"), and **if either is `Auto` or the reps differ** ("auto-or-mismatched"). Such arithmetic belongs in the numeric model.
  - **`unary` (`IntSucc`/`IntPred`)**: dispatches on the Numeric's own rep. Only concrete `U8`–`U64` work. **Signed or `Auto` panics.**
  - **`AsU8`–`AsU64`/`AsChar`**: `as_native`, value-only and rep-agnostic; errors if the value doesn't fit.
  - **`as_usize` sites** (lengths, counts, offsets, `EnumFromTo` bounds): value-only; negative or oversized values error.
  - **`FindByKey` key (`Value::eq_key`)**: Numeric vs `U8`–`U64` compares by value, rep-agnostic (`5u8` and `5auto` keys work). **An out-of-range key panics** (`Value::to_uniform_integer_pair encountered error`) instead of returning an error. Codegen rejects the surveyed case (`-1auto` against a `U8` key), so only registration and the interpreter reach it.
  - **Type-check vs runtime gaps**: legacy `infer_type` accepts `Arith` over two Numerics whose types unify (e.g. two `I8`s, or two `Auto`s), which then panics at runtime. It also accepts `IntSucc`/`IntPred` on signed or `Auto` operands, which panic in `unary`.
- **Pattern matching on `Value::Numeric`** (`value.rs:378`):
  - `U8(n)`–`U64(n)`: the rep must be *exactly* that concrete width and the value equal. `Auto` never matches, unless the `pattern_matches_auto_rep` feature is on, in which case it matches like a wildcard. (Not reachable from a registered program, since legacy `build_scope` panics on a `NumericHole` scrutinee first.)
  - `Int(bounds)`: value-only; negative values never match.
  - `ZConst`/`ZRange`: value-only, rep ignored. These also work on native `U8`–`U64`/`Usize` values.

## Static analysis (`Expr::bounds`)
- `Expr::Numeric` gets exact bounds only when the tree is closed (evaluated with `eval_strict` against the empty `VOID` scope, so any `NumVar` makes it `Any`), strictly valid, and fits in `usize`. Otherwise it's `Bounds::any()`.
- `mk_value_expr` placeholders: `Signed(t)` → `Const(-1 : t)`; `NumericHole` → `Const(0u8)`.
- **`RepeatBetween` needs exact bounds** and panics with `not implemented: RepeatBetween on inexact bounds-expr` otherwise:
  - In legacy decoder compilation, a bound that isn't closed (e.g. a `Var`) panics at registration.
  - In codegen, **any** Numeric bound panics, even a closed one like `RepeatBetween(5u8, U8(5), u8)` that legacy and the interpreter accept. Codegen's bounds analysis apparently doesn't use the closed-Numeric exact path above.

## Codegen
- Embedded trees are elaborated to `TypedNumExpr` and emitted via `numeric::codegen::synthesize`. Operations classified `HomLossy`/`HetLossy` fall back to `eval_fallback` instead of boilerplated backend functions (`numeric/codegen.rs`).
- `ZConst`/`ZRange` are emitted as untyped `SomeInt` literals and ranges, relying on Rust's type inference against the scrutinee.
- **Match exhaustiveness** (`codegen/mod.rs:1782`):
  - A signed scrutinee is always treated as `Refutable`, so a fallback arm is always emitted.
  - **An unsigned scrutinee with `ZConst`/`ZRange` arms and no irrefutable arm hits `unreachable!` in `IntCoverage::add` (`util.rs:133`).**
  - `Pattern::Int` against a signed type reaches `unreachable!` if it ever gets to coverage.

## Not covered
Not yet surveyed: `loc_decoder.rs`, `output/tree.rs`, `read.rs:370`, `helper.rs` (e.g. how an `i8()` format is built) and the rest of `numeric/eval.rs`/`elaborator.rs`. The `loc_decoder` path mirrors the `Value` pattern matching but hasn't been checked line by line. Which `TypeChecker` feature flags (e.g. `__unify_mixed_to_int`) are on by default is also unchecked.
