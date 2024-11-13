use core::panic;
use std::collections::BTreeSet;

use crate::byte_set::ByteSet;
use crate::{
    Arith, BaseType, Expr, Format, IntRel, IntoLabel, Label, Pattern, TypeHint, UnaryOp, ValueType,
};

/// Constructs a Format that expands a single parsed byte into a multi-field record whose elements
/// are `u8`-valued sub-masks of the original byte.
///
/// Currently supports only static-string names for the sub-fields.
///
/// The order in which the fields are listed, both in the `field_bit_lengths` and `field_names` parameters,
/// is to be understood as a MSB-to-LSB order partition.
///
/// Zero-bit field-lengths are not explicitly supported, but 'just work' as implemented.
///
/// # Notes
///
/// Requires that the total length of all fields is 8 bits, and panics otherwise.
///
/// Due to implementation details, will break if there is a single 8-bit field.
pub fn packed_bits_u8<const N: usize>(
    field_bit_lengths: [u8; N],
    field_names: [&'static str; N],
) -> Format {
    const BINDING_NAME: &str = "packedbits";
    let _totlen: u8 = field_bit_lengths.iter().sum();
    assert_eq!(
        _totlen, 8,
        "bad packed-bits field-lengths: total length {_totlen} of {field_bit_lengths:?} != 8"
    );
    let mut fields = Vec::new();
    let mut high_bits_used = 0;
    for (nbits, name) in Iterator::zip(field_bit_lengths.into_iter(), field_names.into_iter()) {
        fields.push((
            Label::Borrowed(name),
            mask_bits(var(BINDING_NAME), high_bits_used, nbits),
        ));
        high_bits_used += nbits;
    }
    map(
        Format::Byte(ByteSet::full()),
        lambda(BINDING_NAME, Expr::Record(fields)),
    )
}

/// Like [`packed_bits_u8`], except all fields are 1-bit and implied to be flags,
/// with fields of type `Bool` rather than `U8`.
///
/// The `field_names` parameter specifies, in MSB-to-LSB order, the name of each flag to be
/// extracted from the appropriate bit-position. None indicates that the bit is unused (at least
/// one name should be `Some`, or else the operation is perfunctory).
pub fn flags_bits8(field_names: [Option<&'static str>; 8]) -> Format {
    const BINDING_NAME: &str = "flagbits";

    let mut flags = Vec::new();

    for (ix, field_name) in field_names.into_iter().enumerate() {
        if let Some(name) = field_name {
            flags.push((
                Label::Borrowed(name),
                is_nonzero_u8(mask_bits(var(BINDING_NAME), ix as u8, 1)),
            ));
        }
    }

    map(
        Format::Byte(ByteSet::full()),
        lambda(BINDING_NAME, Expr::Record(flags)),
    )
}

/// Selects `nbits` bits starting from the highest unused bit in an 8-bit packed-field value, returning a U8-typed Expr
fn mask_bits(x: Expr, high_bits_used: u8, nbits: u8) -> Expr {
    let shift = 8 - high_bits_used - nbits;
    let mask = (1 << nbits) - 1;
    bit_and(shr(x, Expr::U8(shift)), Expr::U8(mask))
}

fn mask_bits16(x: Expr, high_bits_used: u8, nbits: u8) -> Expr {
    let shift = 16 - (high_bits_used + nbits) as u16;
    let mask = (1u16 << nbits) - 1;
    bit_and(shr(x, Expr::U16(shift)), Expr::U16(mask))
}

/// Constructs a Format that expands a parsed 2-byte value into a multi-field record whose elements
/// are `u16`-valued sub-masks of the original u16 (big-endian) order.
///
/// Currently supports only static-string names for the sub-fields.
///
/// The order in which the fields are listed, both in the `field_bit_lengths` and `field_names` parameters,
/// is to be understood as a MSB-to-LSB order partition of the 16-bit big-endian value.
///
/// Zero-bit field-lengths are not explicitly supported, but 'just work' as implemented.
///
/// # Notes
///
/// Requires that the total length of all fields is 16 (bits), and panics otherwise.
pub fn packed_bits_u16<const N: usize>(
    field_bit_lengths: [u8; N],
    field_names: [&'static str; N],
) -> Format {
    const BINDING_NAME: &str = "packedbits";
    let _totlen: u8 = field_bit_lengths.iter().sum();
    assert_eq!(
        _totlen, 16,
        "bad packed-bits field-lengths: total length {_totlen} of {field_bit_lengths:?} != 16"
    );
    let mut fields = Vec::new();
    let mut high_bits_used = 0;
    for (nbits, name) in Iterator::zip(field_bit_lengths.into_iter(), field_names.into_iter()) {
        fields.push((
            Label::Borrowed(name),
            mask_bits16(var(BINDING_NAME), high_bits_used, nbits),
        ));
        high_bits_used += nbits;
    }
    map(
        map(
            tuple_repeat(2, Format::Byte(ByteSet::full())),
            lambda("x", Expr::U16Be(Box::new(var("x")))),
        ),
        lambda(BINDING_NAME, Expr::Record(fields)),
    )
}

/// Like [`packed_bits_u16`], except all fields are 1-bit and implied to be flags,
/// with fields of type `Bool` rather than `U16`.
///
/// The `field_names` parameter specifies, in MSB-to-LSB order, the name of each flag to be
/// extracted from the appropriate bit-position. None indicates that the bit is unused (at least
/// one name should be `Some`, or else the operation is perfunctory).
pub fn flags_bits16(field_names: [Option<&'static str>; 16]) -> Format {
    const BINDING_NAME: &str = "flagbits";

    let mut flags = Vec::new();

    for (ix, field_name) in field_names.into_iter().enumerate() {
        if let Some(name) = field_name {
            flags.push((
                Label::Borrowed(name),
                is_nonzero_u16(mask_bits16(var(BINDING_NAME), ix as u8, 1)),
            ));
        }
    }

    map(
        map(
            tuple_repeat(2, Format::Byte(ByteSet::full())),
            lambda("x", Expr::U16Be(Box::new(var("x")))),
        ),
        lambda(BINDING_NAME, Expr::Record(flags)),
    )
}

/// Returns an [`Expr`] that refers to a (hopefully) in-scope variable by name.
///
/// # Notes
///
/// This helper function does not itself require the named variable to be in-scope at the site where it is called, but
/// out-of-scope variable references are not sound in the larger context of the program, and will typically result in
/// Error or panic.
pub fn var<Name: IntoLabel>(name: Name) -> Expr {
    Expr::Var(name.into())
}

/// Helper-function for [`Expr::Lambda`].
pub fn lambda<Name: IntoLabel>(name: Name, body: Expr) -> Expr {
    Expr::Lambda(name.into(), Box::new(body))
}

/// Helper-function for [`Expr::Variant`].
pub fn variant<Name: IntoLabel>(name: Name, value: Expr) -> Expr {
    Expr::Variant(name.into(), Box::new(value))
}

/// Helper-function for [`Format::Variant`].
pub fn fmt_variant<Name: IntoLabel>(name: Name, fmt: Format) -> Format {
    Format::Variant(name.into(), Box::new(fmt))
}

/// Helper-function for [`Pattern::Binding`] that can take `&'static str`, `String`, or `Label` parameters.
pub fn bind<Name: IntoLabel>(name: Name) -> Pattern {
    Pattern::binding(name)
}

/// Helper-function for [`Format::Tuple`] that can take any iterable container of [`Format`]s.
pub fn tuple(formats: impl IntoIterator<Item = Format>) -> Format {
    Format::Tuple(formats.into_iter().collect())
}

/// Helper-function for [`Format::Union`] over branches that are all [`Format::Variant`].
///
/// Accepts any iterable container of tuples `(Name, Format)` for any `Name` that implements [`IntoLabel`].
pub fn alts<Name: IntoLabel>(branches: impl IntoIterator<Item = (Name, Format)>) -> Format {
    Format::Union(
        (branches.into_iter())
            .map(|(label, format)| Format::Variant(label.into(), Box::new(format)))
            .collect(),
    )
}

/// Helper-function for [`Expr::Match`] that accepts any iterable container `branches` of `(Pattern, Expr)` pairs.
pub fn expr_match(head: Expr, branches: impl IntoIterator<Item = (Pattern, Expr)>) -> Expr {
    Expr::Match(Box::new(head), Vec::from_iter(branches))
}

/// Helper-function for [`Format::Match`] where the body of every branch is a
/// [`Format::Variant`].
///
/// Accepts any iterable container of `(Pattern, Name, Format)` tuples, which prescribe
/// a match-case of the shape `Pattern => AnonType::Name(Format)`.
///
/// Used primarily when the native types of the raw branch-bodies would not
/// otherwise agree.
///
/// May also be used to add indicators of provenance to values that might be
/// typed identically but have different semantics and might need to be treated
/// differently based on which branch was taken.
pub fn match_variant<Name: IntoLabel>(
    head: Expr,
    branches: impl IntoIterator<Item = (Pattern, Name, Format)>,
) -> Format {
    Format::Match(
        Box::new(head),
        (branches.into_iter())
            .map(|(pattern, label, format)| {
                (pattern, Format::Variant(label.into(), Box::new(format)))
            })
            .collect(),
    )
}

/// Helper-function for [`Format::Union`].
///
/// Accepts any iterable container of `Format`s.
///
/// If the branches in question are all `Format::Variant`, use [`alts`] instead.
///
/// If the given branches cannot be deterministically distinguished within a fixed finite lookahead, use [`union_nondet`] instead.
pub fn union(branches: impl IntoIterator<Item = Format>) -> Format {
    Format::Union(Vec::from_iter(branches))
}

/// Helper-function for constructing a [`Format::Union`] over branches that cannot be deterministically distinguished within a fixed finite lookahead.
///
/// Accepts any iterable container of tuples `(Name, Format)` for any `Name` that implements [`IntoLabel`], where the `Name` element becomes
/// an identifying Variant-name for the resulting branch of the union.
///
/// # Notes
///
/// To be used sparingly, ideally only for the highest-level format definition that covers the full range of known formats.
///
/// If there is a potential overlap in the inputs that would be accepted as two distinct branches, the preferred (ideally, more specific) branch
/// should always appear earlier in the iteration order.
pub fn union_nondet<Name: IntoLabel>(branches: impl IntoIterator<Item = (Name, Format)>) -> Format {
    Format::UnionNondet(
        (branches.into_iter())
            .map(|(label, format)| Format::Variant(label.into(), Box::new(format)))
            .collect(),
    )
}

/// Helper-function for [`Format::Record`] taking any iterable container of
/// `(Name, Format)` pairs, which define each field's name and contents, in order.
///
/// # Notes
///
/// Care should be taken for any structure whose `IntoIterator` implementation
/// does not preserve the order of insertion, as record-like values within
/// binary formats must decode in the same order they were encoded, which must
/// conform to the specification and will typically be invariant for
/// non-self-describing formats.
pub fn record<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Format)>) -> Format {
    Format::Record(
        (fields.into_iter())
            .map(|(label, format)| (label.into(), format))
            .collect(),
    )
}

/// Helper function that returns a novel Format that is the (distinguished) union of `format` and [`Format::EMPTY`].
///
/// The variant-name assigned to a positive match for the given format will be `"some"`,
/// and the variant-name assigned to a negative match will be `"none"`.
pub fn optional(format: Format) -> Format {
    alts([("some", format), ("none", Format::EMPTY)])
}

/// Helper-function for [`Format::Repeat`].
pub fn repeat(format: Format) -> Format {
    Format::Repeat(Box::new(format))
}

/// Helper-function for [`Format::Repeat1`].
pub fn repeat1(format: Format) -> Format {
    Format::Repeat1(Box::new(format))
}

/// Helper-function for [`Format::RepeatCount`].
pub fn repeat_count(len: Expr, format: Format) -> Format {
    Format::RepeatCount(Box::new(len), Box::new(format))
}

/// Helper-function for [`Format::RepeatBetween`].
///
/// # Notes
///
/// Will result in downstream panic or error if `min` is found to exceed `max` at runtime.
///
/// If `min` is statically guaranteed to be equal to `max`, use [`repeat_count`] instead.
///
/// As currently implemented, the only `Expr`s that are accepted in the `min` and `max` positions
/// are those that can be evaluated independent of Scope (i.e. contain no variable expressions).
/// This is to ensure that `min <= max` can be checked in a context-free manner. This is not an inherent
/// requirement of the primitive, but rather an imposed limitation of the implementation designed
/// to keep the logic simple.
pub fn repeat_between(min: Expr, max: Expr, format: Format) -> Format {
    Format::RepeatBetween(Box::new(min), Box::new(max), Box::new(format))
}

/// Helper-function for [`Format::RepeatUntilLast`].
///
/// Creates a repetition that will consume `format` repeatedly, stopping after (specifically not 'just before')
/// the first element for which `cond` evaluates to `true` when called with said element as its sole argument.
///
/// # Notes
///
/// By virtue of its definition, the repetition will always contain at least one element.
pub fn repeat_until_last(predicate: Expr, format: Format) -> Format {
    Format::RepeatUntilLast(Box::new(predicate), Box::new(format))
}

/// Helper-function for [`Format::RepeatUntilSeq`].
///
/// Creates a repetition that will consume `format` repeatedly, stopping after (specifically not 'just before')
/// `cond` evaluates to `true` when called with the entire sequence thus-far as its sole argument.
///
/// # Notes
///
/// If `cond` evaluates is true when called with the empty sequence, will always yield an empty repetition.
///
/// If the condition being evaluated is a bounds-check that the length of the sequence falls between some `N` and `M`, use [`repeat_between`] instead.
///
/// If the condition being evaluated only ever returns true based on a predicate over the final element of the sequence, use [`repeat_until_last`] instead.
pub fn repeat_until_seq(predicate: Expr, format: Format) -> Format {
    Format::RepeatUntilSeq(Box::new(predicate), Box::new(format))
}

/// Helper-function for alternating between two formats based on a boolean predicate.
///
/// If `cond` evaluates to `true`, will decode as `format_true`, otherwise as `format_false`.
///
/// # Notes
///
/// Implicitly requires that the two formats have the same value-type.
///
/// If the two formats have different value-types, or if knowledge of the chosen branch is needed, use [`if_then_else_variant`] instead.
pub fn if_then_else(cond: Expr, format_true: Format, format_false: Format) -> Format {
    Format::Match(
        Box::new(cond),
        vec![
            (Pattern::Bool(true), format_true),
            (Pattern::Bool(false), format_false),
        ],
    )
}

/// Helper function for branching between two formats based on a boolean predicate, even when the two formats have different value-types.
///
/// If `cond` evaluates to `true`, will decode into the variant-format `yes(format_yes)`, and otherwise `no(format_no)`.
///
/// # Notes
///
/// If `format_no` happens to be `Format::EMPTY`, consider using [`cond_maybe`] instead.
pub fn if_then_else_variant(cond: Expr, format_yes: Format, format_no: Format) -> Format {
    if_then_else(
        cond,
        Format::Variant("yes".into(), Box::new(format_yes)),
        Format::Variant("no".into(), Box::new(format_no)),
    )
}

/// Helper function for accepting a given format if and only if the given expression evaluates to `true`, and otherwise
/// returning a None-value without parsing any bytes.
pub fn cond_maybe(cond: Expr, format: Format) -> Format {
    Format::Maybe(Box::new(cond), Box::new(format))
}

/// Helper function for [`Format::Map`].
pub fn map(f: Format, expr: Expr) -> Format {
    Format::Map(Box::new(f), Box::new(expr))
}

/// Returns a `Format` that matches the byte `b` and fails on any other byte.
pub fn is_byte(b: u8) -> Format {
    Format::Byte(ByteSet::from([b]))
}

/// Returns a `Format` that matches any byte in `v`, and fails on any byte not in `v`.
///
/// `v` can be of any type with an implemented conversion `Into<`[`ByteSet`]`>` (e.g. a u8-typed Range, any slice/array of u8, any iterator over u8).
///
/// If `v` is a singleton value, use [`is_byte`] instead.
pub fn byte_in<I>(v: I) -> Format
where
    I: Into<ByteSet>,
{
    Format::Byte(v.into())
}

/// Returns a format consisting of `count` consecutive bytes all matching `b`.
pub fn repeat_byte(count: u32, b: u8) -> Format {
    Format::RepeatCount(Box::new(Expr::U32(count)), Box::new(is_byte(b)))
}

/// Returns a format that matches any byte *other than* `b`.
pub fn not_byte(b: u8) -> Format {
    Format::Byte(!ByteSet::from([b]))
}

/// Returns a format that matches a given byte-sequence.
pub fn is_bytes(bytes: &[u8]) -> Format {
    tuple(bytes.iter().copied().map(is_byte))
}

/// Helper-function for [`Expr::RecordProj`].
///
/// Provided that `label` is a valid field within the record (whether natural, or mapped) `head`, will evaluate to the value of the corresponding field.
pub fn record_proj(head: Expr, label: impl IntoLabel) -> Expr {
    Expr::RecordProj(Box::new(head), label.into())
}

/// Helper-function for a left-associative fold over a list of field-projections on a nested-record
/// that operates in a Lens-like fashion. The list of labels should begin with the outermost field projection.
///
/// If the list of labels is empty, will simply return `head`.
///
/// Otherwise, will return `(((head->label0)->label1)->...)->labelN`.
pub fn record_projs(head: Expr, labels: &[&'static str]) -> Expr {
    if labels.is_empty() {
        head
    } else {
        record_projs(record_proj(head, labels[0]), &labels[1..])
    }
}

/// Helper-function for [`Expr::TupleProj`].
///
/// Provided that `index` is a valid position within the tuple (whether natural, or mapped) `head`, will evaluate to the value of the corresponding positional argument.
pub fn tuple_proj(head: Expr, index: usize) -> Expr {
    Expr::TupleProj(Box::new(head), index)
}

pub fn expr_eq(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Eq, Box::new(x), Box::new(y))
}

pub fn expr_ne(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Ne, Box::new(x), Box::new(y))
}

pub fn expr_lt(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Lt, Box::new(x), Box::new(y))
}
pub fn expr_lte(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Lte, Box::new(x), Box::new(y))
}

pub fn expr_gt(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Gt, Box::new(x), Box::new(y))
}

pub fn expr_gte(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Gte, Box::new(x), Box::new(y))
}

pub fn as_u8(x: Expr) -> Expr {
    Expr::AsU8(Box::new(x))
}

pub fn as_u16(x: Expr) -> Expr {
    Expr::AsU16(Box::new(x))
}

pub fn as_u32(x: Expr) -> Expr {
    Expr::AsU32(Box::new(x))
}

pub fn as_u64(x: Expr) -> Expr {
    Expr::AsU64(Box::new(x))
}

pub fn as_char(x: Expr) -> Expr {
    Expr::AsChar(Box::new(x))
}

pub fn add(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::Add, Box::new(x), Box::new(y))
}

pub fn sub(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::Sub, Box::new(x), Box::new(y))
}

pub fn mul(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::Mul, Box::new(x), Box::new(y))
}

pub fn div(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::Div, Box::new(x), Box::new(y))
}

pub fn rem(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::Rem, Box::new(x), Box::new(y))
}

pub fn or(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::BoolOr, Box::new(x), Box::new(y))
}

pub fn and(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::BoolAnd, Box::new(x), Box::new(y))
}

pub fn expr_not(x: Expr) -> Expr {
    Expr::Unary(UnaryOp::BoolNot, Box::new(x))
}

pub fn bit_or(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::BitOr, Box::new(x), Box::new(y))
}

pub fn bit_and(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::BitAnd, Box::new(x), Box::new(y))
}

pub fn shl(value: Expr, places: Expr) -> Expr {
    Expr::Arith(Arith::Shl, Box::new(value), Box::new(places))
}

pub fn shr(value: Expr, places: Expr) -> Expr {
    Expr::Arith(Arith::Shr, Box::new(value), Box::new(places))
}

pub fn seq_length(seq: Expr) -> Expr {
    Expr::SeqLength(Box::new(seq))
}

pub fn sub_seq(seq: Expr, start: Expr, length: Expr) -> Expr {
    Expr::SubSeq(Box::new(seq), Box::new(start), Box::new(length))
}

/// Helper-function for [`Expr::SubSeqInflate`]
///
/// # Notes
///
/// Unlike `sub_seq`, which is a pure slice operation, the `start` and `length` parameters
/// may describe a larger range of the sequence than currently exists, provided `start` is itself in-bounds,
/// following the Inflate/LZ77 decoding algorithm.
pub fn sub_seq_inflate(seq: Expr, start: Expr, length: Expr) -> Expr {
    Expr::SubSeqInflate(Box::new(seq), Box::new(start), Box::new(length))
}

/// Helper-function for [`Expr::FlatMap`]
///
/// # Notes
///
/// The `seq` parameter must be a sequence type, and `f` must be a lambda that returns a sequence type. Model-wise equivalent to
/// [`Iterator::flat_map`].
pub fn flat_map(f: Expr, seq: Expr) -> Expr {
    Expr::FlatMap(Box::new(f), Box::new(seq))
}

/// Helper-function for [`Expr::FlatMapAccum`]
///
/// # Notes
///
/// The `seq` parameter must be a sequence type, `accum` must have the type
/// `accum_type`, and `f` must be a lambda that takes a pair `(accum, x)` and
/// returns a pair `(accum', ys)`, where `ys` is typed as a sequence.
///
/// The final value of `accum` is discarded, but the immediate return value
/// after any non-final iteration is used as the input value for the next.
pub fn flat_map_accum(f: Expr, accum: Expr, accum_type: ValueType, seq: Expr) -> Expr {
    Expr::FlatMapAccum(
        Box::new(f),
        Box::new(accum),
        accum_type.into(),
        Box::new(seq),
    )
}

/// Helper-function for [`Expr::FlatMapList`]
///
/// # Notes
///
/// The `seq` parameter must evaluate to a sequence, and `f` must be a lambda that takes a `(list, x)` pair and returns a sequence with the same type as `list`.
///
/// The first iteration will pass in an empty list, and each iteration will extend the list by appending the return value of its corresponding call to `f`.
///
/// The parameter `ret_type` corresponds to the element-type of the list being returned, not the overall type of the return-value.
pub fn flat_map_list(f: Expr, ret_type: ValueType, seq: Expr) -> Expr {
    Expr::FlatMapList(Box::new(f), ret_type.into(), Box::new(seq))
}

/// Helper-function for [`Expr::Dup`].
pub fn dup(count: Expr, expr: Expr) -> Expr {
    Expr::Dup(Box::new(count), Box::new(expr))
}

/// Composed `Format::Where` and `Expr::Lambda` taking a raw format, an arbitrary name for the lambda expression head, and the lambda body as an Expr.
pub fn where_lambda(raw: Format, name: impl IntoLabel, body: Expr) -> Format {
    Format::Where(Box::new(raw), Box::new(lambda(name, body)))
}

/// Numeric validation helper that constrains a given format to yield a value that falls in the inclusive range `lower..=upper`
///
/// Attempts to check for `lower == 0` to avoid vacuous lower bounds on unsigned types.
///
/// # Notes
///
/// Does not check that `lower <= upper` as that cannot be statically determined.
pub fn where_between(format: Format, lower: Expr, upper: Expr) -> Format {
    let cond = if lower.bounds().is_exact().is_some_and(|x| x == 0) {
        expr_lte(var("x"), upper)
    } else {
        and(expr_gte(var("x"), lower), expr_lte(var("x"), upper))
    };
    where_lambda(format, "x", cond)
}

/// Homogenous-format tuple whose elements are all `format`, repeating `count` times
pub fn tuple_repeat(count: usize, format: Format) -> Format {
    let iter = std::iter::repeat(format).take(count);
    Format::Tuple(iter.collect())
}

/// Homogenous-format record whose fields all have the same Format `format`, with each of the names of `field_names` in order
pub fn record_repeat<const N: usize>(field_names: [&'static str; N], format: Format) -> Format {
    let iter = field_names
        .iter()
        .map(|name| (Label::Borrowed(name), format.clone()));
    Format::Record(iter.collect())
}

/// Returns an Expr that evaluates to `true` if the given U8-typed expression is non-zero
pub fn is_nonzero_u8(expr: Expr) -> Expr {
    expr_ne(expr, Expr::U8(0))
}

/// Returns an Expr that evaluates to `true` if the given U16-typed expression is non-zero
pub fn is_nonzero_u16(expr: Expr) -> Expr {
    expr_ne(expr, Expr::U16(0))
}

/// Returns an Expr that evaluates to `true` if the given U32-typed expression is non-zero
pub fn is_nonzero_u32(expr: Expr) -> Expr {
    expr_ne(expr, Expr::U32(0))
}
/// Helper for constructing `Option::None` within the Expr model-language.
pub const fn expr_none() -> Expr {
    Expr::LiftOption(None)
}

/// Helper for constructing `Option::Some(expr)` within the Expr model-language.
pub fn expr_some(expr: Expr) -> Expr {
    Expr::LiftOption(Some(Box::new(expr)))
}

/// Helper for constructing `Option::None` within the Pattern model-language.
pub const fn pat_none() -> Pattern {
    Pattern::Option(None)
}

/// Helper for constructing `Option::Some(pat)` within the Pattern model-language.
pub fn pat_some(pat: Pattern) -> Pattern {
    Pattern::Option(Some(Box::new(pat)))
}

/// Helper for constructing `fmt -> Option::Some` within the Format model-language.
pub fn format_some(f: Format) -> Format {
    map(
        f,
        lambda("val", Expr::LiftOption(Some(Box::new(var("val"))))),
    )
}

/// Helper for constructing `Option::None` within the Format model-language.
pub fn format_none() -> Format {
    compute(expr_none())
}

/// Shortcut for `where_lambda` applied over the simple predicate [`is_nonzero_u8`]
pub fn where_nonzero_u8(format: Format) -> Format {
    where_lambda(format, "x", is_nonzero_u8(var("x")))
}

/// Shortcut for `where_lambda` applied over the simple predicate [`is_nonzero_u16`]
pub fn where_nonzero_u16(format: Format) -> Format {
    where_lambda(format, "x", is_nonzero_u16(var("x")))
}

/// Helper for constructing `Format::ForEach`
pub fn for_each(seq: Expr, name: impl IntoLabel, inner: Format) -> Format {
    Format::ForEach(Box::new(seq), name.into(), Box::new(inner))
}

/// Helper for specifying a byte-aligned Format with a given byte-multiple `align`
pub fn aligned(f: Format, align: usize) -> Format {
    chain(Format::Align(align), "_", f)
}

/// Helper method for [`Format::LetFormat`]
#[inline]
pub fn chain(f0: Format, name: impl IntoLabel, f: Format) -> Format {
    Format::LetFormat(Box::new(f0), name.into(), Box::new(f))
}

/// Shortcut for matching an explicit pattern of bytes wrapped in a sequence.
pub fn pattern_bytestring(bytes: &[u8]) -> Pattern {
    Pattern::Seq(bytes.iter().map(|b| Pattern::U8(*b)).collect())
}

/// Helper for the identity function as an Expr::Lambda
pub fn f_id() -> Expr {
    lambda("x", var("x"))
}

/// Shorthand for matching on a boolean-typed Expr `scrutinee` and returning one of two values
/// depending on if it is true (`if_true`) or false (`if_false`)
#[inline]
pub fn expr_if_else(scrutinee: Expr, if_true: Expr, if_false: Expr) -> Expr {
    expr_match(
        scrutinee,
        [
            (Pattern::Bool(true), if_true),
            (Pattern::Bool(false), if_false),
        ],
    )
}

/// Helper function simulating the `bool::then` function that returns either `Some(if_true)` or `None` according
/// to the boolean value of `scrutinee`.
///
/// Note that `if_true` is interpreted as having the parametric type of the resulting `Option<T>`. If the value
/// we want to return is already an Option, use [`expr_if_else`] instead.
#[inline]
pub fn expr_opt_if(scrutinee: Expr, if_true: Expr) -> Expr {
    expr_if_else(scrutinee, expr_some(if_true), expr_none())
}

/// Performs a fallible destructuring of the provided `expr` within the Expr layer,
/// either extracting the contents of a `Some(_)` value, or resulting in an ExcludedBranch
/// error at runtime due to a fallible pattern-match.
pub fn expr_unwrap(expr: Expr) -> Expr {
    Expr::Match(Box::new(expr), vec![(pat_some(bind("x")), var("x"))])
}

/// Performs an index operation on an expression `seq` with an index `index`, without checking for OOB array access.
///
/// This will result in a runtime panic during parse-evaluation if the index is out of bounds.
pub fn index_unchecked(seq: Expr, index: Expr) -> Expr {
    Expr::SeqIx(Box::new(seq), Box::new(index))
}

/// Performs a guarded index operation on an expression `seq` with an index `index`, returning `Some(elt)`
/// if the index is in-bounds, or `None` otherwise.
pub fn index_checked(seq: Expr, index: Expr) -> Expr {
    let len = seq_length(seq.clone());
    let is_sound = expr_lt(index.clone(), len);
    expr_opt_if(is_sound, index_unchecked(seq, index))
}

/// Performs an equivalent operation to `tuple_proj` under the transformation between fixed-length sequences
/// and homogeneously-typed tuples.
///
/// The resulting Expr may panic upon evaluation if the provided index is not statically guaranteed to be in-bounds.
#[inline]
pub fn seq_proj(seq: Expr, index: usize) -> Expr {
    // FIXME - this extra step can be avoided by adding something like `Int(usize)` to our set of Expr const-primitives
    let index: u32 = index
        .try_into()
        .expect("seq_proj index larger than U32::MAX is not supported");
    index_unchecked(seq, Expr::U32(index))
}

/// Boilerplate helper for simulating multi-argument lambda via pattern-match tuple destruction
pub fn lambda_tuple<const N: usize>(names: [&'static str; N], body: Expr) -> Expr {
    const HEAD_VAR: &str = "tuple_var";
    lambda(
        HEAD_VAR,
        expr_match(
            var(HEAD_VAR),
            [(Pattern::Tuple(names.into_iter().map(bind).collect()), body)],
        ),
    )
}

/// Shorthand for Expr::LeftFold
pub fn left_fold(f: Expr, init: Expr, init_vt: ValueType, seq: Expr) -> Expr {
    Expr::LeftFold(Box::new(f), Box::new(init), init_vt.into(), Box::new(seq))
}

/// Helper for constructing an `Expr::Seq` of length == 0
pub const fn seq_empty() -> Expr {
    Expr::Seq(Vec::new())
}

/// Helper for constructing an `Expr::Seq` of length == 1
pub fn singleton(value: Expr) -> Expr {
    Expr::Seq(vec![value])
}

/// Helper for constructing an `Expr::Tuple` of arity == 2
pub fn pair(x: Expr, y: Expr) -> Expr {
    Expr::Tuple(vec![x, y])
}

/// Computes the larger of two given `Expr`s, left-biased if equal
pub fn expr_max(a: Expr, b: Expr) -> Expr {
    expr_if_else(expr_gte(a.clone(), b.clone()), a, b)
}

/// Convenience tool for cloning a subset of a record-typed Expr's field-set in an arbitrary order
///
/// # Notes
/// The list of fields must all appear in the original, and should contain no duplicates
pub fn subset_fields<const N: usize>(original: Expr, field_set: [&'static str; N]) -> Expr {
    let mut accum_fields = Vec::with_capacity(N);
    let mut included_fields = BTreeSet::new();

    for field_name in field_set.into_iter() {
        if !included_fields.insert(field_name) {
            unreachable!("duplicate field in subset_fields: `{field_name}`");
        }
        accum_fields.push((
            Label::Borrowed(field_name),
            record_proj(original.clone(), field_name),
        ));
    }
    Expr::Record(accum_fields)
}

pub fn prepend_field<const N: usize>(
    field: (&'static str, Expr),
    original: (Expr, [&'static str; N]),
) -> Expr {
    let (field_name, field_expr) = field;
    let (original_expr, original_fields) = original;

    let mut accum_fields = Vec::with_capacity(N + 1);
    let mut included_fields = BTreeSet::new();

    accum_fields.push((field_name.into(), field_expr));
    let _ = included_fields.insert(field_name);

    for field_name in original_fields.into_iter() {
        if !included_fields.insert(field_name) {
            unreachable!("duplicated field in prepend_field: `{field_name}`");
        }
        accum_fields.push((
            Label::Borrowed(field_name),
            record_proj(original_expr.clone(), field_name),
        ));
    }
    Expr::Record(accum_fields)
}

/// Given an expression of type `Seq(Seq(T))`, return an expression of type `Seq(T)` corresponding to the concatenation
/// of each sub-list in turn.
#[inline]
pub fn concat(xs: Expr) -> Expr {
    flat_map(f_id(), xs)
}

/// Helper for the lambda-abstracted form of [`concat`].
pub fn f_concat() -> Expr {
    lambda("xs", concat(var("xs")))
}

pub fn seq_any<F>(f: F, seq: Expr) -> Expr
where
    F: FnOnce(Expr) -> Expr,
{
    left_fold(
        lambda_tuple(["any", "x"], or(var("any"), f(var("x")))),
        Expr::Bool(false),
        ValueType::Base(BaseType::Bool),
        seq,
    )
}

/// Analogue of [`std::option::Option::map_or`] expressed within the Expr model.
///
/// Given a default value `dft` of type `Expr@T`, and a callable `f` mapping `Expr@T -> Expr@U`,
/// as well as a value `x` of type `Expr@Option(T)`, computes the value of type `Expr@U`
/// corresponding to `f` applied to the `Some(_)` case, or dft if `x` is `None`.
pub fn expr_option_map_or(dft: Expr, f: impl FnOnce(Expr) -> Expr, x: Expr) -> Expr {
    expr_match(x, [(pat_some(bind("x")), f(var("x"))), (pat_none(), dft)])
}

/// Helper function for [`Format::Compute`].
#[inline]
pub fn compute(expr: Expr) -> Format {
    Format::Compute(Box::new(expr))
}

/// Helper function for [`Format::Slice`]
#[inline]
pub fn slice(len: Expr, inner: Format) -> Format {
    Format::Slice(Box::new(len), Box::new(inner))
}

/// Constructs a balanced (i.e. minimiazed max depth) tree of `bitor`-joined
/// nodes of type Expr (U8 or U16).
///
/// Does not work if there are more than 16 elements in `nodes`
pub fn balanced_bitor_max16(mut nodes: Vec<Expr>) -> Expr {
    let n = nodes.len();

    let (l, r) = match () {
        _ if n > 8 => (
            balanced_bitor_max16(nodes.drain(0..8).collect::<Vec<_>>()),
            balanced_bitor_max16(nodes.drain(..).collect::<Vec<_>>()),
        ),
        _ if n > 4 => (
            balanced_bitor_max16(nodes.drain(0..4).collect::<Vec<_>>()),
            balanced_bitor_max16(nodes.drain(..).collect::<Vec<_>>()),
        ),
        _ if n > 2 => (
            balanced_bitor_max16(nodes.drain(0..2).collect::<Vec<_>>()),
            balanced_bitor_max16(nodes.drain(..).collect::<Vec<_>>()),
        ),
        _ if n == 2 => {
            let mut two_shot = nodes.drain(..);
            let l = two_shot.next().unwrap();
            let r = two_shot.next().unwrap();
            (l, r)
        }
        _ if n == 1 => {
            return nodes.drain(..).next().unwrap();
        }
        _ => {
            panic!("balanced_bitor_max16 called with n == 0")
        }
    };
    bit_or(l, r)
}

/// Helper function for `Format::AccumUntil`
pub fn accum_until(
    f_done: Expr,
    f_update: Expr,
    init: Expr,
    vt: impl Into<TypeHint>,
    format: Format,
) -> Format {
    Format::AccumUntil(
        Box::new(f_done),
        Box::new(f_update),
        Box::new(init),
        vt.into(),
        Box::new(format),
    )
}

/// Computes the final element of a sequence-typed Expr, evaluating to None if it is empty
pub fn seq_opt_last(seq: Expr) -> Expr {
    expr_opt_if(
        expr_gt(seq_length(seq.clone()), Expr::U32(0)),
        index_unchecked(seq.clone(), sub(seq_length(seq.clone()), Expr::U32(1))),
    )
}
