use crate::bounds::Bounds;
use crate::byte_set::ByteSet;
use crate::{Arith, Expr, Format, IntRel, IntoLabel, Label, Pattern, ValueType};

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
pub fn packed_bits_u8<const N: usize>(
    field_bit_lengths: [u8; N],
    field_names: [&'static str; N],
) -> Format {
    const BINDING_NAME: &'static str = "packedbits";
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

/// Selects `nbits` bits starting from the highest unused bit in an 8-bit packed-field value, returning a U8-typed Expr
fn mask_bits(x: Expr, high_bits_used: u8, nbits: u8) -> Expr {
    let shift = 8 - high_bits_used - nbits;
    let mask = (1 << nbits) - 1;
    bit_and(shr(x, Expr::U8(shift)), Expr::U8(mask))
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
    Expr::Match(Box::new(head), Vec::from_iter(branches.into_iter()))
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
        head,
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
    Format::Union(Vec::from_iter(branches.into_iter()))
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
    Format::RepeatCount(len, Box::new(format))
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
    Format::RepeatBetween(min, max, Box::new(format))
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
    Format::RepeatUntilLast(predicate, Box::new(format))
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
    Format::RepeatUntilSeq(predicate, Box::new(format))
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
        cond,
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
    Format::Maybe(cond, Box::new(format))
}

/// Helper function for [`Format::Map`].
pub fn map(f: Format, expr: Expr) -> Format {
    Format::Map(Box::new(f), expr)
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
    Format::RepeatCount(Expr::U32(count), Box::new(is_byte(b)))
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
        return head;
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

pub fn rem(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::Rem, Box::new(x), Box::new(y))
}

pub fn or(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::BoolOr, Box::new(x), Box::new(y))
}

pub fn and(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::BoolAnd, Box::new(x), Box::new(y))
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
    Expr::FlatMapAccum(Box::new(f), Box::new(accum), accum_type, Box::new(seq))
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
    Expr::FlatMapList(Box::new(f), ret_type, Box::new(seq))
}

/// Helper-function for [`Expr::Dup`].
pub fn dup(count: Expr, expr: Expr) -> Expr {
    Expr::Dup(Box::new(count), Box::new(expr))
}

/// Composed `Format::Where` and `Expr::Lambda` taking a raw format, an arbitrary name for the lambda expression head, and the lambda body as an Expr.
pub fn where_lambda(raw: Format, name: impl IntoLabel, body: Expr) -> Format {
    Format::Where(Box::new(raw), lambda(name, body))
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

/// Returns an Expr that evaluates to `true` if the given U8-typed expression is non-zero
pub fn is_nonzero_u8(expr: Expr) -> Expr {
    expr_ne(expr, Expr::U8(0))
}

/// Returns an Expr that evaluates to `true` if the given U16-typed expression is non-zero
pub fn is_nonzero_u16(expr: Expr) -> Expr {
    expr_ne(expr, Expr::U16(0))
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
pub const fn format_none() -> Format {
    Format::Compute(expr_none())
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
/// Helper for constructing `Format::ForEach`
pub fn for_each(seq: Expr, name: impl IntoLabel, inner: Format) -> Format {
    Format::ForEach(seq, name.into(), Box::new(inner))
}

/// Helper for specifying a byte-aligned Format with a given byte-multiple `align`
pub fn aligned(f: Format, align: usize) -> Format {
    chain(Format::Align(align), "_", f)
}

/// Helper for parsing `(f, suffix)` where we only want to see the `f` component
#[inline]
pub fn discard_suffix(f: Format, suffix: Format) -> Format {
    map(tuple([f, suffix]), lambda("x", tuple_proj(var("x"), 0)))
}

/// Shortcut for computing a standalone `Format::Pos` that we immediately consume without ever needing to reuse,
/// which discards the `Format::Pos` token via `map`
///
/// The `pos_varname` parameter is the verbatim name of the variable that `f` internally uses to refer to the parsed `Format::Pos`.
#[inline]
pub fn with_pos(pos_varname: &'static str, f: Format) -> Format {
    chain(Format::Pos, pos_varname, f)
}

/// Shortcut for two-layer decoding of a Format that is embedded in the result of parsing another.
///
/// The first argument is the outer layer, whose immediate result is discarded after processing the
/// second layer, which is treated as the final result.
pub fn two_pass(primary: Format, secondary: Format) -> Format {
    chain(
        primary,
        "raw",
        Format::DecodeBytes(var("raw"), Box::new(secondary)),
    )
}

/// Helper method for [`Format::LetFormat`]
#[inline]
pub fn chain(f0: Format, name: impl IntoLabel, f: Format) -> Format {
    Format::LetFormat(Box::new(f0), name.into(), Box::new(f))
}

/// Shortcut for discarding a Format's return value but perserving its effect on the overall parse
pub fn void(f: Format) -> Format {
    chain(f, "_", Format::EMPTY)
}

/// Shortcut for matching an explicit pattern of bytes wrapped in a sequence.
pub fn pattern_bytestring(bytes: &[u8]) -> Pattern {
    Pattern::Seq(bytes.into_iter().map(|b| Pattern::U8(*b)).collect())
}

/// Constructs a format that will fallback to parsing an abstracted byte-sequence if the given Format `f`
/// fails to parse.
pub fn binary_fallback(f: Format) -> Format {
    union_nondet([
        ("valid", f),
        ("invalid", repeat(Format::Byte(ByteSet::full()))),
    ])
}

/// Helper for the identity function as an Expr::Lambda
pub fn f_id() -> Expr {
    lambda("x", var("x"))
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

/// Performs a table lookup operation over a sequence of entries `seq` of type `elem_type`.
///
/// Uses `f_getkey` to map from each entry to its key, and `query_key` as the query key.
///
/// Even if mulitple matching entries exist, only the first one is returned, as an Option.
pub fn table_find(
    seq: Expr,
    elem_type: ValueType,
    f_getkey: impl Fn(Expr) -> Expr,
    query_key: Expr,
) -> Expr {
    let match0_or1 = flat_map_list(
        lambda(
            "list-entry",
            expr_match(
                seq_length(tuple_proj(var("list-entry"), 0)),
                [
                    (Pattern::Int(Bounds::at_least(1)), Expr::Seq(Vec::new())),
                    (
                        Pattern::Wildcard,
                        expr_match(
                            expr_eq(f_getkey(tuple_proj(var("list-entry"), 1)), query_key),
                            [
                                (Pattern::Bool(true), Expr::Seq(vec![var("entry")])),
                                (Pattern::Bool(false), Expr::Seq(Vec::new())),
                            ],
                        ),
                    ),
                ],
            ),
        ),
        elem_type,
        seq,
    );
    seq_to_opt(match0_or1)
}

/// Performs the natural transformation from a singleton-or-empty `Expr::Seq` into `Option`.
///
/// # Notes
///
/// Not well-formed (i.e. will result in a runtime error) if the provided Expr can ever be a sequence of length 2 or higher.
pub fn seq_to_opt(empty_or_singleton: Expr) -> Expr {
    expr_match(
        empty_or_singleton,
        [
            (Pattern::Seq(vec![bind("x")]), expr_some(var("x"))),
            (Pattern::Seq(Vec::new()), expr_none()),
        ],
    )
}
