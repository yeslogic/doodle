# `doodle::numeric` Patterns, Antipatterns, and Behavior

This document serves as a comprehensive summary of the `doodle::numeric` sub-model and how it fits (and where it doesn't fit)
into the `doodle` core grammar and engine.

As new support is added for existing patterns, or new patterns emerge through novel designs, this document should be updated accordingly.

## Terminology

For the purposes of this document, we use the term 'Numeric' (titlecase) to denote both values (`doodle::numeric::core::TypedConst`) and expressions (`doodle::numeric::core::Expr`)
within the `doodle::numeric` sub-model.

This exists in aposition to the terms 'core grammar integer' or 'native integer', which refer to either the pre-existing integer nodes in `Expr` and the like (e.g. `Expr::U8`),
as well as the `ValueType` and `UType`s they constitute.

The term 'numeric' (lowercase) as an adjective refers to both native integer AST nodes (`Expr`/`Pattern`/`Value`/`ValueType`/etc.), as well as Numerics.

## Background

The `doodle::numeric` layer addresses three major gaps in the existng `doodle` core grammar that predated it:

### Signed-Int Semantic Transparency

Though there is nothing stopping a nominally signed-int data-field in a format from being parsed byte-by-byte, or even byte-cast to an unsigned integer type
of the appropriate width, `doodle` had no way of treating the bytes in question as the endian component-bytes of a two's-complement signed integer.

Because of this, data shown through the `tree` output style might show large `uX` values instead of the negative `iX` their bytes were meant to signify,
and in generated-code, types whose fields were supposed to be signed-integers would lossily represent them as unsigned-integer Rust-types, without any
breadcrumb that would tell the end-user of the generated-code that a particular field needed to be reinterpreted.

The introduction of signed-integers into the model via `doodle::numeric` allows for the correct values to be shown in interpreter output,
and the correct types to be emitted in generated code output.

### Context-Free Operations

Terms like `Expr::Var(..)` carry no associated type-information. Because `Expr::Arith` and `Expr::IntRel` require their operands to have identical `ValueType`, this means
that any non-constant numeric operation (comparison or arithmetic) can only be valid if one knows the type of any opaquely-typed terms within them.

In order to add 1 to a variable `x`, in other words, one needs to know the exact `ValueType` bound to "x" in the exact scope where the expression would be evaluated.
This becomes additionally more complicated with holdover type-ascriptions like `Expr::SeqLen` always being `U32`, which isn't visible anywhere except in the `doodle`
source-code itself.

In such cases, even a simple operation like `x + 2` requires knowledge of what `Expr` constructor to wrap around `2` to yield a well-typed expression.

In constrast, the Numeric model permits explicitly-typed mixed-type operations that are agnostic to the type of an incoming variable and work regardless of what it is,
provided the overall expression's mathematical value when evaluated is representable in the root type.

### Mixed-Sign Operations

The Numeric model allows two integers to be comined through mathematical operations without forcing either into the target type, even when their own types disagree.
This allows operations that might temporarily overflow/overflow in the target representation upon cast, to resolve to a valid result that agrees mathematically with
the ideal answer.

## `Expr`

The only Expr that can *directly* embed a Numeric value is `Expr::Numeric`, which takes a `Box<NumExpr>` (`doodle::numeric::core::Expr`); `Expr::Var` is the only variant that *indirectly* represent a Numeric.

## `Format`

There is no native support for `Format`-level construction of Numerics, with one exception: `Format::Pos` evaluates to an Auto Numeric holding the current buffer offset, matching its `NumericHole` typing in registration. Otherwise, all Numerics are generated at the `Expr` layer through `Format::Map` or `Format::Compute` and similar.

Though not directly an embedding of `TypedConst`, `ViewFormat::ReadArray` can carry a signed marker-type, though that does not directly result in Numerics being manifested in memory while parsing.

## `Pattern`

A `NumExpr` with an unsigned `MachineRep` can match against the corresponding `Pattern::U?` variant: `Pattern::U8` will match against `TypedConst(N, U8)`, and similarly for `U16`, `U32`, and `U64`.
All unsigned Numerics can also be matched by `Pattern::Int`.

Auto-rep Numerics can be matched by `Pattern::U8`–`Pattern::U64`, which compare them by value, and by `Pattern::Int`, `Pattern::ZConst` and `Pattern::ZRange`.

For signed Numerics, the only patterns that are compatible are `ZConst` and `ZRange`, neither of which care about the rep of the scrutinee.

Both interpreters (`decoder` and `loc_decoder`) match numeric-literal patterns through the shared `Value::matches_numeric_literal`.

## `Expr::infer_type`/`ValueType` (registration/legacy typechecker)

Unsigned-rep Numerics are judged to have `ValueType::U8`/`ValueType::U16`/`ValueType::U32`/`ValueType::U64` based on the width of their rep. In this way, from a ValueType context,
they are indistinguishable from core grammar integers of that type.

Signed-rep Numerics are ascribed `ValueType::Signed(SignedIntType::*)` according to their rep.

Anything with `Auto` rep is inferred as `ValueType::NumericHole`. `ValueType::NumericHole` acts like `ValueType::Any`, but the only concrete value-type it can unify against are numeric `ValueType`s (i.e. `ValueType::Base` holding a numeric `BaseType`, or `ValueType::Signed`).

The type-inference logic for `Expr::Numeric` recurses into the rep-solver for `NumExpr`, which can result in unification failure if an untagged `NumExpr::BinOp` holds terms with distinct non-`Auto` reps. Otherwise, the same type will be inferred as with `TypeChecker`, though a locally `Auto`-rep node will be judged as `NumericHole` because `infer_type` is a context-free
type-solver, whereas `TypeChecker` does full bidirectional type-checking that can solve locally-`Auto` nodes if they are later referenced in type-constrained contexts.

`ValueType::is_numeric()` returns `true` for `ValueType::Base(b)` when `b.is_numeric()` holds, as well as for `NumericHole` and `ValueType::Signed(..)`. It is the argument check for `IntRel` (whose operands must also unify) and `AsU8`–`AsU64`, which therefore accept signed operands.

`ValueType::is_unsigned_or_auto()` accepts a native unsigned type or `NumericHole`, rejecting `Signed`. It is the argument check for native `Arith`, `IntSucc`/`IntPred`, `AsChar`, `EnumFromTo`, `ViewExpr::Offset`, `ViewFormat::CaptureBytes` and `ViewFormat::ReadArray` (lengths), `RepeatCount`, `RepeatBetween`, and the `Slice` length and `WithRelativeOffset` base and offset. Where two operands are involved, they are unified first, so a signed operand cannot hide behind an Auto one. Signed arithmetic belongs in `NumExpr`, and the idiom for converting a signed value to a char is `AsChar(AsU32(x))`.

`RepeatBetween` additionally requires both bounds to be constant (`Expr::exact_repeat_bounds`), as do the `TypeChecker` and the decoder compiler.

The 'key' field of `FindByKey` mandates `ValueType::Base(b)` guarded by `b.is_numeric()` (i.e. it rejects `Signed`).[^1]

[^1]: `FindByKey` is a special-case in that it combines (via ValueType unifcation) two
separate sources-of-truth for the key-type, before testing that it is `ValueType::Base`
satisfying the `is_numeric` predicate; namely, the inferred type of the query-key, and
the expression-type of the body of the `Lambda` used to generate the key for an element
in the array being searched. This means, in practice, that a query-key with an `Auto` rep
may be accepted by `infer_type` as long as the lambda returns values of a concrete unsigned
numeric type.

At this point in time `SeqIx` requires its argument to be typed as `ValueType::U32`, as do other `Expr` with implied 'sequence index' or 'sequence length' semantics: `SubSeq`, `SubSeqInflate`, `Dup`.

## [`TypeChecker`](/src/typecheck.rs)

The `TypeChecker` engine ascribes more specific types for any Numeric, and infers set-based constraints on certain `Expr` nodes that `infer_type` currently hard-codes to return specific `ValueType`s for.

A Numeric whose type cannot be pinned to a unique solution (e.g. a bare Auto constant) is a `TCError` ("no unique solution", or "no valid solutions" for e.g. a negative Auto at an unsigned-only site), reported through `generate_code`'s normal "Failed to infer module-wide type annotations" path. No default types are applied.

### `BaseSet::UAny`

All `Expr` nodes ascribed uppercase labels in in the following constructions receive a type-constraint marking them as 'unsigned of any width, no tiebreaker'.
If the actual type inferred happens to be signed, a unification error will occur.

- `X := Expr::Arith(Y, Z)`
- `X = Expr::IntSucc(Y)`/`X := Expr::IntPred(Y)` [^3]
- `_ = Expr::AsChar(Y)` (Y) [^4]
- `_ := Format::RepeatCount(Y)`/`_ := RepeatBetween(Y, Z)` (in the case of `RepeatBetween` specifically, the types of Y and Z are also required to unify, and both must be constant)
- `_ := Expr::FindByKey(_, _, Y, _)` (the query key)
- `_ := ViewFormat::CaptureBytes(X)`/`_ := ViewFormat::ReadArray(X, _)`
- `_ := ViewExpr::Offset(_, X)`

### `BaseSet::UAny32`/`UintSet::any_default(..)`

All `Expr` nodes ascribed uppercase labels in in the following constructions receive a type-constraint marking them as 'unsigned of any width, defaulting to U32 if not excluded'.

If the actual type inferred happens to be signed, a unification error will occur.

If `U32` is excluded but more than one possible unsignedsolution remains, a unification error will also occur (highly unlikely in practice).

- `_ := SeqIx(_, Z)`
- `_ := EnumFromTo(Y, Z)` (the types of Y and Z are also required to unify)
- `X := SeqLength(_)` [^5]
- `_ := Format::Slice(Y, _)`
- `_ := Format::WithRelativeOffset(Y, Z, _)` (the types of Y and Z are also required to unify)

Despite the name, `UAny32` admits every unsigned width, including `U64`; the `32` is only the tiebreak default.

The following cases receive one-off UintSet constraints:

- `X := DynFormat::Huffman(Y, Z)`: the projective array-elem-types of Y, Z are given `UintSet::SHORT8` (U8 or U16, preferring U8 to tiebreak), X is given `UintSet::any_default(Bits16)` (like UAny32, with U16 as the default)
- `X := Format::Pos`: X is given `UintSet::any_default(Bits64)` (prefers U64 but accepts any other unsigned int-type).

### `IntSet::ZAny`

The following `Expr` nodes are constrained with the broadest, Numeric-permissive `IntSet::ZAny`

- `_ := Expr::AsU8(X)`
- `_ := Expr::AsU16(X)`
- `_ := Expr::AsU32(X)`
- `_ := Expr::AsU64(X)`
- `_ := Expr::IntRel(X, Y)` (the types of X and Y are also required to unify, so mixed-sign comparisons are rejected) [^2]

### Dynamic

The scrutinee for a pattern-match against the following patterns are constrained according to the following rules:

- `X ~ Pattern::ZConst(N) | Pattern::ZRange(N..=M)`: `X` must have a repr in which `N` (and `M`) are representable values; if either is negative, `X` cannot be an unsigned Numeric; similarly enforces bit-width minima
- `X ~ Pattern:Int(..)`: `X` restricted to `UintSet`, precluding signed Numerics

## `Expr::eval`/`Value` (interpreter)

When called on `Expr::Numeric(n)`, `Expr::eval` calls into `NumExpr::eval`, which does not guard against out-of-bounds values (underflow, overflow) on the claimed numrep. `NumExpr::eval_strict`,on the other hand, will always perform these checks, on all intermediate terms as well as on the final result.

However, `NumExpr::eval` can still produce errors for certain numeric operations, in the case of division-by-zero, modulo-non-positive (negative or zero), ambiguous untagged bin-ops over different concrete reps, and out-of-scope or non-numeric variable-bindings through `NumVar`.

Furthermore, whenever the `TypedConst` held by a `Value` is coerced to a fixed machine-type by an enclosing `Expr::eval` call, unrepresentable values are eventually caught.

Various numeric-kinded expr nodes treat Numeric values differently:

### `int_rel`

Comparison between `Numeric(X)` and `Numeric(Y)` is well-typed, and the reps of `X` and `Y` need not agree. `Numeric(X)` against `U8`-`U64` coerce `X` via `NumExpr::as_native`, which ignores rep and just requires that the value
of `X` fit in the equivalent width to the other operand.

### `FindByKey` (`AsKey::eq_key`/`compare_as_key`)

A `Numeric(X)` key compared against a `U8`–`U64` key is coerced with `TypedConst::get_as_unsigned`. If that fails (e.g. a `-1` Auto query key against `U8` keys), the comparison returns `EvalError::NumericConvert` rather than panicking.

### `arith`

Arithmetic between `Numeric(X)` and `Value::U*(N)` (for U8--U64) call into `NumExpr::get_as_unsigned` which requires the rep of `X` to either be auto, or to match the exact branch for the core-integer type of its co-term.

#### Numeric Support

The following combinations of terms (in either order) are accepted by `arith`:

- `(Unsigned, Native)` (e.g. `add(numexpr!(5u8), Expr::U8(5))`)
- `(Auto, Native)` (e.g. `add(poly_zero(), Expr::U8(5))`): the Auto operand takes the native operand's type
- `(Unsigned, Auto)` (e.g. `add(numexpr!(5u8), poly_zero())`)
- `(U0, U0)` (the same Unsigned rep; e.g. `add(numexpr!(5u8), numexpr!(5u8))`)
- `(Auto, Auto)` (e.g. `add(poly_zero(), poly_zero())`): computed by value, and the result is Auto. No width is imposed until the value next meets a concrete type, where it is checked. Division by zero and shift amounts outside `0..64` are errors.

Panics on operations between two Numerics with different concrete reps, and on any operations over one or more signed-rep Numerics.
Registration rejects both of these, so the panics are type-checker invariants rather than data-reachable failures.

### `unary`

`IntSucc`/`IntPred` on a Numeric dispatch on its own rep: concrete `U8`–`U64` compute natively, and Auto is incremented or decremented by value, staying Auto. Signed reps panic (registration rejects them).

## Notes

[^2]: `IntRel` accepts signed operands in every layer. The operands must still have the same type, so comparing a signed value against an unsigned one is rejected by both type checkers. Exercised by `test_intrel_on_signed` in `src/typecheck.rs`.
Codegen emits a plain infix comparison, which is also correct for signed types.

[^3]: `IntPred` and `IntSucc` on signed operands are rejected by both registration and the `TypeChecker`; signed increment/decrement belongs in `NumExpr`, which mirrors these as Unary operators.
Exercised by the [integration tests](/doodle-formats/tests/signed_intops.rs) `test_registration_unary` and `test_gen_unary`.

[^4]: `AsChar` on signed operands is rejected by both registration and the `TypeChecker`, unlike `AsU8`–`AsU64`. The idiom for a signed value is `AsChar(AsU32(x))`, exercised by `test_interp_char_via_u32` in the same
integration test. The interpreter itself would accept any non-negative value, but registration prevents signed operands from reaching it.

[^5]: `Expr::SeqLength` and `Format::Pos` are special cases in that they represent a numeric value is generated *ex nihilo*, and which therefore cannot cause constraint failures for local unification. However, if something downstream uses a variable bound to either one in a context
where only a signed-Numeric assignment results in a well-typed tree, these constraints may still
contribute to a unification failure. However, it is much more sensible in such cases to view
the downstream construction as the reason for unification failure, rather than the `BaseSet`/`UintSet` constraint itself.
