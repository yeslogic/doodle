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

There is no native support for `Format`-level construction of Numerics. All Numerics are generated at the `Expr` layer through `Format::Map` or `Format::Compute` and similar.

Though not directly an embedding of `TypedConst`, `ViewFormat::ReadArray` can carry a signed marker-type, though that does not directly result in Numerics being manifested in memory while parsing.

## `Pattern`

A `NumExpr` with an unsigned `MachineRep` can match against the corresponding `Pattern::U?` variant: `Pattern::U8` will match against `TypedConst(N, U8)`, and similarly for `U16`, `U32`, and `U64`.
All unsigned Numerics can also be matched by `Pattern::Int`.

Auto-rep numerics can only be matched via `Pattern::Int` if they resolve to an unsigned value, and otherwise, can only match against `Pattern::ZConst` or `Pattern::ZRange`.

For signed Numerics, the only patterns that are compatible are `ZConst` and `ZRange`, neither of which care about the rep of the scrutinee.

## `Expr::infer_type`/`ValueType`

A `NumExpr` with `Auto` rep is inferred as `ValueType::NumericHole`.

An untagged `NumExpr::BinOp` requires its arguments to agree via unification, so two distinct non-`Auto` operands result in an error; anything else is resolved to the natural `ValueType` corresponding with the `MachineRep`.

`ValueType::is_numeric()` returns `true` for both `NumericHole` and `ValueType::Signed(..)`. Both of these are then accepted as well-formed argument (`Expr`) types for `IntRel`, `Arith`, `IntSucc/IntPred`, `AsU*`/`AsChar`, `EnumFromTo`, `ViewExpr::Offset`, `ViewFormat::CaptureBytes`, and `ViewFormat::ReadArray` (in the 'length' position).

Certain sites in `Expr::infer_type` mandate `ValueType::Base(b)` guarded by `b.is_numeric()`, not just `ValueType::is_numeric` on the overall type (i.e. they reject `Signed` and `NumericHole`); these include `RepeatCount`, `RepeatBetween`, and the 'key' field of `FindByKey`.[^1]

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

### `BaseSet::UAny`

All `Expr` nodes ascribed uppercase labels in in the following constructions receive a type-constraint marking them as 'unsigned of any width, no tiebreaker'.
If the actual type inferred happens to be signed, a unification error will occur.

- `X := Expr::Arith(Y, Z)`
- `_ := Expr::IntRel(Y, Z)` [^2]
- `X = Expr::IntSucc(Y)`/`X := Expr::IntPred(Y)` [^3]
- `_ = Expr::AsChar(Y)` (Y) [^4]
- `_ := Format::RepeatCount(Y)`/`_ := RepeatBetween(Y, Z)` (the types of Y and Z are also required to unify)
- `_ := ViewFormat::CaptureBytes(X)`/`_ := ViewFormat::ReadArray(X, _)`
- `_ := ViewExpr::Offset(_, X)`

### `BaseSet::UAny32`/`UintSet::any_default(..)`

All `Expr` nodes ascribed uppercase labels in in the following constructions receive a type-constraint marking them as 'unsigned of any width, defaulting to U32 if not excluded'.

If the actual type inferred happens to be signed, a unification error will occur.

If `U32` is excluded but more than one possible unsignedsolution remains, a unification error will also occur (highly unlikely in practice).

- `_ := SeqIx(_, Z)`
- `_ := EnumFromTo(Y, Z)` (the types of Y and Z are also required to unify)
- `X := SeqLength(_)` [^5]

The following cases receive one-off UintSet constraints:

- `X := DynFormat::Huffman(Y, Z)`: the projective array-elem-types of Y, Z are given `UintSet::SHORT8` (U8 or U16, preferring U8 to tiebreak), X is given `UintSet::any_default(Bits16)` (like UAny32, with U16 as the default)
- `X := Format::Pos`: X is given `UintSet::any_default(Bits64)` (prefers U64 but accepts any other unsigned int-type).

### `IntSet::ZAny`

The following `Expr` nodes are constrained with the broadest, Numeric-permissive `IntSet::ZAny`

- `_ := Expr::AsU8(X)`
- `_ := Expr::AsU16(X)`
- `_ := Expr::AsU32(X)`
- `_ := Expr::AsU64(X)`

### Dynamic

The scrutinee for a pattern-match against the following patterns are constrained according to the following rules:

- `X ~ Pattern::ZConst(N) | Pattern::ZRange(N..=M)`: `X` must have a repr in which `N` (and `M`) are representable values; if either is negative, `X` cannot be an unsigned Numeric; similarly enforces bit-width minima
- `X ~ Pattern:Int(..)`: `X` restricted to `UintSet`, precluding signed Numerics

## `Expr::eval`/`Value`

When called on `Expr::Numeric(n)`, `Expr::eval` calls into `NumExpr::eval`, which does not guard against out-of-bounds values (underflow, overflow) on the claimed numrep. `NumExpr::eval_strict` does perform these checks, not only on the end-result but also on all intermediate terms.

However, `NumExpr::eval` can still produce errors, in the case of division-by-zero, modulo-non-positive (negative or zero), ambiguous untagged binary operations over different concrete reps, out-of-scope or non-numeric variable referenced by `NumVar`.

Furthermore, whenever the `TypedConst` held by a `Value` is coerced to a fixed machine-type by an enclosing `Expr::eval` call, unrepresentable values are eventually caught.

Various numeric-kinded expr nodes treat Numeric values differently:

### `int_rel`

Comparison between `Numeric(X)` and `Numeric(Y)` is well-typed, and the reps of `X` and `Y` need not agree. `Numeric(X)` against `U8`-`U64` coerce `X` via `NumExpr::as_native`, which ignores rep and just requires that the value
of `X` fit in the equivalent width to the other operand.

### `eq_key`

#### Panics

Comparison of `Usize` against `Numeric` panics.

### `arith`

Arithmetic between `Numeric(X)` and `Value::U*(N)` (for U8--U64) call into `NumExpr::get_as_unsigned` which requires the rep of `X` to either be auto, or to match the exact branch for the core-integer type of its co-term.

#### Numeric Support

The following combinations of terms (in either order) are accepted by `arith`:

- `(Unsigned, Native)` (e.g. `add(numexpr!(5u8), Expr::U8(5))`)
- `(Unsigned, Auto)` (e.g. `add(numexpr!(5u8), poly_zero())`)
- `(U0, U0)` (the same Unsigned rep; e.g. `add(numexpr!(5u8), numexpr!(5u8))`)

Panics on ambiguous operations (both auto or different concrete rep) between two Numerics,
and on any operations over one or more signed-rep Numerics.

Also panics when mixing Numeric with Usize[^6]

[^6]: `Value::Usize` only appears when using `EnumFromTo`, and can propagate from there via index operations and through scoped variables. It is currently a candidate for deprecation since
it is rare and complicates the model in certain ways, e.g. by failing to preserve the claimed type of a value derived by indexing into an `EnumFromTo` sequence.

## Noted Gaps

[^2]: While `IntRel` applies properly in the interpreter layer over inner `Expr`s with signed-int `ValueType`s, the `TypeChecker` layer currently rejects any `IntRel` node whose terms do not both resolve
to a unique unsigned `BaseType`. This is a gap in the design and may be patched later on. This is exdercised by unit-test `test_repro_intrel_on_signed_fails_to_unify` in `src/typecheck.rs`.

[^3]: `IntPred` and `IntSucc` directly mirror Unary operators in the Numeric layer, so this is a less-glaring gap than `IntRel`. It may also benefit from an update, but this is lower-priority. Confirmed gap
in both interpeter (`Expr::eval`) and code-generator (`TypeCheck::infer_var_expr`), exercised by [integration test (regression)](/doodle-formats/tests/signed_intops.rs)

[^4]: On the interpreter-side, `AsChar` is allowed to run on any non-negative value without first converting to an unsigned ValueType via `AsU8`/`AsU16`/etc. This gap only exists in the typechecker, and is demonstrated
in the same integation test as [^2].

[^5]: `Expr::SeqLength` and `Format::Pos` are special cases in that they represent a numeric value is generated *ex nihilo*, and which therefore cannot cause constraint failures for local unification. However, if something downstream uses a variable bound to either one in a context
where only a signed-Numeric assignment results in a well-typed tree, these constraints may still
contribute to a unification failure. However, it is much more sensible in such cases to view
the downstream construction as the reason for unification failure, rather than the `BaseSet`/`UintSet` constraint itself.
