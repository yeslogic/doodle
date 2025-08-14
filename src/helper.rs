use std::collections::BTreeSet;

use num_traits::{ToPrimitive, Zero};

use crate::byte_set::ByteSet;
pub use crate::marker::BaseKind;
use crate::{
    Arith, BaseType, Expr, Format, IntRel, IntoLabel, Label, Pattern, StyleHint, TypeHint, UnaryOp,
    ValueType, ViewExpr, ViewFormat,
};
use crate::{Endian, bounds::Bounds};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BitFieldKind {
    FlagBit(&'static str),
    BitsField {
        field_name: &'static str,
        bit_width: u8,
    },
    Reserved {
        bit_width: u8,
        check_zero: bool,
    },
}

impl BitFieldKind {
    pub const fn bit_width(&self) -> u8 {
        match self {
            BitFieldKind::FlagBit(..) => 1,
            BitFieldKind::BitsField { bit_width, .. }
            | BitFieldKind::Reserved { bit_width, .. } => *bit_width,
        }
    }

    pub const fn is_flag(&self) -> bool {
        matches!(self, BitFieldKind::FlagBit(..))
    }

    pub const fn field_name(&self) -> Option<&'static str> {
        match self {
            BitFieldKind::FlagBit(lab) => Some(*lab),
            BitFieldKind::BitsField { field_name, .. } => Some(*field_name),
            BitFieldKind::Reserved { .. } => None,
        }
    }

    pub const fn check_zero(&self) -> bool {
        match self {
            BitFieldKind::Reserved { check_zero, .. } => *check_zero,
            _ => false,
        }
    }
}

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
// TODO - phase out
pub fn packed_bits_u8<const N: usize>(
    field_bit_lengths: [u8; N],
    field_names: [&'static str; N],
) -> Format {
    const BINDING_NAME: &str = "packed_bits";
    #[cfg(debug_assertions)]
    {
        let _len: u8 = field_bit_lengths.iter().sum();
        debug_assert_eq!(
            _len, 8,
            "bad packed-bits field-lengths: total length {_len} of {field_bit_lengths:?} != 8"
        );
    }
    let mut fields = Vec::new();
    let mut high_bits_used = 0;
    for (nbits, name) in Iterator::zip(field_bit_lengths.into_iter(), field_names.into_iter()) {
        fields.push((
            Label::Borrowed(name),
            mask_bits_u8(var(BINDING_NAME), high_bits_used, nbits),
        ));
        high_bits_used += nbits;
    }
    map(ANY_BYTE, lambda(BINDING_NAME, Expr::Record(fields)))
}

/// Ergonomic helper for parsing a 8-bit packed value into a multi-field record with more
/// context-awareness for determining the interpretation (and semantics) of the various
/// segments of contiguous bits.
///
/// Can be used to to simulate [`packed_bits_u8`], as well as handle flag-bits and explicitly mark reserved (non-recorded)
/// bits with optional zero-checking.
pub fn bit_fields_u8<const N: usize>(bit_fields: [BitFieldKind; N]) -> Format {
    const BINDING_NAME: &str = "packed_bits";
    #[cfg(debug_assertions)]
    {
        let _len: u8 = bit_fields.iter().map(BitFieldKind::bit_width).sum();
        debug_assert_eq!(
            _len, 8,
            "bad packed-bits field-lengths: total width {_len} of {bit_fields:?} != 8"
        );
    }
    let mut fields = Vec::new();

    // mask value that should yield `0` when `&`-ed with the original u16
    // NOTE - currently, we set this value but don't directly use it
    let mut _zero_mask: u8 = 0;

    let mut high_bits_used = 0;
    for bit_field in bit_fields.into_iter() {
        let nbits = bit_field.bit_width();
        if let Some(name) = bit_field.field_name() {
            let raw = mask_bits_u8(var(BINDING_NAME), high_bits_used, nbits);
            let field_value = if bit_field.is_flag() {
                is_nonzero_u8(raw)
            } else {
                raw
            };
            fields.push((Label::Borrowed(name), field_value));
        } else if bit_field.check_zero() {
            _zero_mask &= ((1u8 << nbits) - 1) << (8 - (high_bits_used + nbits));
        }
        high_bits_used += nbits;
    }

    let packed = if _zero_mask != 0 {
        const PREPACKED: &str = "packed";
        // NOTE - only bother using where-lambda if zero_mask is non-vacuous
        where_lambda(
            ANY_BYTE,
            PREPACKED,
            expr_eq(bit_and(var(PREPACKED), Expr::U8(_zero_mask)), Expr::U8(0)),
        )
    } else {
        ANY_BYTE
    };

    map(packed, lambda(BINDING_NAME, Expr::Record(fields)))
}

/// Selects `nbits` bits starting from the highest unused bit in an 8-bit packed-field value, returning a U8-typed Expr.
///
/// Will panic if `nbits + high_bits_used > 8`.
pub fn mask_bits_u8(x: Expr, high_bits_used: u8, nbits: u8) -> Expr {
    assert!(
        nbits + high_bits_used <= 8,
        "mask_bits_u8 cannot create mask {nbits} bits out of available {}",
        8 - high_bits_used
    );
    let shift = 8 - high_bits_used - nbits;
    let mask = (1 << nbits) - 1;
    let shifted = if shift == 0 {
        x
    } else {
        shr(x, Expr::U8(shift))
    };
    bit_and(shifted, Expr::U8(mask))
}

fn mask_bits_u16(x: Expr, high_bits_used: u8, nbits: u8) -> Expr {
    assert!(
        nbits + high_bits_used <= 16,
        "mask_bits_u16 cannot create mask {nbits} bits out of available {}",
        16 - high_bits_used
    );
    let shift = 16 - (high_bits_used + nbits) as u16;
    let mask = (1u16 << nbits) - 1;
    let shifted = if shift == 0 {
        x
    } else {
        shr(x, Expr::U16(shift))
    };
    bit_and(shifted, Expr::U16(mask))
}

/// Ergonomic helper for parsing a 16-bit packed value into a multi-field record with more
/// context-awareness for determining the interpretation (and semantics) of the various
/// segments of contiguous bits.
pub fn bit_fields_u16<const N: usize>(bit_fields: [BitFieldKind; N]) -> Format {
    const BINDING_NAME: &str = "packed_bits";
    #[cfg(debug_assertions)]
    {
        let _len: u8 = bit_fields.iter().map(BitFieldKind::bit_width).sum();
        debug_assert_eq!(
            _len, 16,
            "bad packed-bits field-lengths: total width {_len} of {bit_fields:?} != 16"
        );
    }
    let mut fields = Vec::new();

    // mask value that should yield `0` when `&`-ed with the original u16
    // NOTE - currently, we set this value but don't directly use it
    let mut _zero_mask: u16 = 0;

    let mut high_bits_used = 0;
    for bit_field in bit_fields.into_iter() {
        let nbits = bit_field.bit_width();
        if let Some(name) = bit_field.field_name() {
            let raw = mask_bits_u16(var(BINDING_NAME), high_bits_used, nbits);
            let field_value = if bit_field.is_flag() {
                is_nonzero_u16(raw)
            } else {
                raw
            };
            fields.push((Label::Borrowed(name), field_value));
        } else if bit_field.check_zero() {
            _zero_mask &= ((1u16 << nbits) - 1) << (16 - (high_bits_used + nbits));
        }
        high_bits_used += nbits;
    }

    let u16be = map(
        tuple_repeat(2, Format::Byte(ByteSet::full())),
        lambda("x", Expr::U16Be(Box::new(var("x")))),
    );

    let packed = if _zero_mask != 0 {
        const PREPACKED: &str = "packed";
        // NOTE - only bother using where-lambda if zero_mask is non-vacuous
        where_lambda(
            u16be,
            PREPACKED,
            expr_eq(bit_and(var(PREPACKED), Expr::U16(_zero_mask)), Expr::U16(0)),
        )
    } else {
        u16be
    };

    map(packed, lambda(BINDING_NAME, Expr::Record(fields)))
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

/// Helper-function for [`Format::Sequence`] that can take any iterable container of [`Format`]s.
pub fn seq(formats: impl IntoIterator<Item = Format>) -> Format {
    Format::Sequence(formats.into_iter().collect())
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

/// Helper-function for [`Format::Match`] that accepts any iterable container `branches` of `(Pattern, Format)` pairs.
pub fn fmt_match(head: Expr, branches: impl IntoIterator<Item = (Pattern, Format)>) -> Format {
    Format::Match(Box::new(head), Vec::from_iter(branches))
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

/// Helper-function for [`Format::record`] taking any iterable container of
/// `(Name, Format)` pairs, which define each field's name and contents, in order.
///
/// # Notes
///
/// Care should be taken for any structure whose `IntoIterator` implementation
/// does not preserve the order of insertion, as record-like values within
/// binary formats must decode in the same order they were encoded, which must
/// conform to the specification and will typically be invariant for
/// non-self-describing formats.
pub fn record<Name: IntoLabel>(
    fields: impl IntoIterator<Item = (Name, Format), IntoIter: DoubleEndedIterator>,
) -> Format {
    Format::record(fields)
}

/// 'Smart' new-style record constructor that will discard labels starting with `__`,
/// capture labels starting with `_` without forcing in-record persistence (e.g. count-fields for
/// repeat arrays), and capture all other labels with in-record persistence.
///
/// # Examples
///
/// ```
/// # use doodle::helper::*;
/// # use doodle::Format;
/// record_auto([
///     ("_foo", Format::ANY_BYTE), // will be captured, but not persisted
///     ("bar", repeat_count(var("_foo"), Format::ANY_BYTE)), // will be captured and persisted
///     ("__baz", Format::ANY_BYTE), // will be discarded without ever being captured
/// ]); // yields `struct _ { bar: Vec<u8> }`
/// ```
pub fn record_auto<Name: IntoLabel + AsRef<str>>(
    fields: impl IntoIterator<Item = (Name, Format), IntoIter: DoubleEndedIterator>,
) -> Format {
    let fields_persist = fields.into_iter().map(|(label, format)| {
        if label.as_ref().starts_with("__") {
            (None, format)
        } else {
            let is_tmp = label.as_ref().starts_with("_");
            (Some((label, !is_tmp)), format)
        }
    });
    record_ext(fields_persist)
}

/// Bespoke record-constructor for new-style `Format`-level records.
///
/// Instead of a simple label, each format is given a synthetic marker for the field-capture
/// semantics corresponding to said format.
///
/// - `None` is to be used for non-fields that should be parsed but ignored (e.g. padding, alignment, leftover bytes).
/// - `Some((label, true))` will capture the field as `label` and persist it within the record under the same name and in the natural order-of-definition.
/// - `Some((label, false))` will capture the field as `label` but only for use in dependent formats in later fields, and it will not appear in the final record.
pub fn record_ext<Name: IntoLabel>(
    fields_persist: impl IntoIterator<
        Item = (Option<(Name, bool)>, Format),
        IntoIter: DoubleEndedIterator,
    >,
) -> Format {
    let mut rev_fields = fields_persist
        .into_iter()
        .rev()
        .collect::<Vec<(Option<(Name, bool)>, Format)>>();
    let accum = Vec::with_capacity(rev_fields.len());
    Format::Hint(
        StyleHint::Record { old_style: false },
        Box::new(Format::__chain_record(accum, &mut rev_fields)),
    )
}

/// Helper function that encloses a Format in an 'optional' context,
/// parsing it only if it its characteristic byte-pattern is detected,
/// and otherwise as a no-op parse.
///
/// Uses the in-model `Option` layer to avoid constructing a duplicate
/// version of `Option`.
pub fn optional(format: Format) -> Format {
    Format::Union([fmt_some(format), fmt_none()].to_vec())
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

/// Returns a format that matches a given byte-sequence and returns a tuple.
pub fn is_bytes(bytes: &[u8]) -> Format {
    tuple(bytes.iter().copied().map(is_byte))
}

/// Returns a format that matches a given byte-sequence and returns an array/vector.
pub fn byte_seq(bytes: &[u8]) -> Format {
    seq(bytes.iter().copied().map(is_byte))
}

/// Helper const for a format that matches every byte.
pub const ANY_BYTE: Format = Format::Byte(ByteSet::full());

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
pub fn record_lens(head: Expr, labels: &[&'static str]) -> Expr {
    if labels.is_empty() {
        head
    } else {
        record_lens(record_proj(head, labels[0]), &labels[1..])
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

pub fn pred(x: Expr) -> Expr {
    Expr::Unary(UnaryOp::IntPred, Box::new(x))
}

pub fn succ(x: Expr) -> Expr {
    Expr::Unary(UnaryOp::IntSucc, Box::new(x))
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
/// The `list` (tuple index 0) passed into `f` is initially empty, and will be post-extended with the output of each call to `f`.
/// Specifically, the second iteration will call `f` with `list` equal to the output of `f([], seq[0])`, and the third iteration
/// will call `f((f([], seq[0]) ++ f(f([], seq[0]), seq[1])), seq[2])`, and so on.
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
pub fn where_between<N>(format: Format, lower: N, upper: N, inject: impl Fn(N) -> Expr) -> Format
where
    N: ToPrimitive + Zero,
{
    let cond = if lower.is_zero() {
        expr_lte(var("x"), inject(upper))
    } else {
        expr_match(
            var("x"),
            [
                (
                    Pattern::Int(Bounds::new(
                        lower.to_usize().unwrap(),
                        upper.to_usize().unwrap(),
                    )),
                    Expr::Bool(true),
                ),
                (Pattern::Wildcard, Expr::Bool(false)),
            ],
        )
        // and(expr_gte(var("x"), lower), expr_lte(var("x"), upper))
    };
    where_lambda(format, "x", cond)
}

pub fn where_between_u8(format: Format, lower: u8, upper: u8) -> Format {
    where_between(format, lower, upper, Expr::U8)
}

pub fn where_between_u16(format: Format, lower: u16, upper: u16) -> Format {
    where_between(format, lower, upper, Expr::U16)
}

pub fn where_between_u32(format: Format, lower: u32, upper: u32) -> Format {
    where_between(format, lower, upper, Expr::U32)
}

/// Numeric validation helper that constrains a given format to yield a value that falls in an abstract range,
/// represented by a value `range` that a `Bounds` value can be constructed from via an `.into()` call.
///
/// Unlike [`where_between`], the range in question need not be closed (i.e. bounded both above and below).
/// In return, there is a loss of flexibility, as the range must be specified via numeric consts, rather than
/// arbitrary `Expr` values that are not required to be constants.
///
/// However, the complexity of the test will typically be higher for this helper than for [`where_between`];
/// this is doubly true for closed ranges whose minimum is `0`, in which case [`where_between`] tests a single
/// integer comparison.
pub fn where_within<R>(format: Format, range: R) -> Format
where
    R: Into<Bounds>,
{
    where_lambda(format, "x", is_within(var("x"), range.into()))
}

pub fn where_within_any<R>(format: Format, ranges: impl IntoIterator<Item = R>) -> Format
where
    R: Into<Bounds>,
{
    where_lambda(format, "x", is_within_any(var("x"), ranges))
}

/// Homogenous-format tuple whose elements are all `format`, repeating `count` times
pub fn tuple_repeat(count: usize, format: Format) -> Format {
    let iter = std::iter::repeat_n(format, count);
    Format::Tuple(iter.collect())
}

/// Homogenous-format record whose fields all have the same Format `format`, with each of the names of `field_names` in order
pub fn record_repeat<const N: usize>(field_names: [&'static str; N], format: Format) -> Format {
    let iter = field_names
        .iter()
        .map(|name| (Label::Borrowed(name), format.clone()));
    Format::record(iter)
}

pub trait ZeroMarker {
    fn mk_zero() -> Expr;
}

/// Marker type for [`Expr::U8`]-based generic trait impls
pub struct U8;
/// Marker type for [`Expr::U16`]-specific generic trait impls
pub struct U16;

/// Marker type for [`Expr::U32`]-specific generic trait impls
pub struct U32;

/// Marker type for [`Expr::U32`]-specific generic trait impls
pub struct U64;

macro_rules! impl_zeromarker {
    ( $( $t:ident ),+ $(,)? ) => {
        $(
            impl ZeroMarker for $t {
                fn mk_zero() -> Expr {
                    Expr::$t(0)
                }
            }
        )*
    };
}

impl_zeromarker!(U8, U16, U32, U64);

/// Given the appropriate Marker-type, returns an Expr that evaluates to `true` if the expression `expr` (of the appropriate type for the Marker passed in)
/// is non-zero.
pub fn is_nonzero<T: ZeroMarker>(expr: Expr) -> Expr {
    expr_ne(expr, T::mk_zero())
}

pub fn is_nonzero_u8(expr: Expr) -> Expr {
    expr_gt(expr, Expr::U8(0))
}

pub fn is_nonzero_u16(expr: Expr) -> Expr {
    expr_gt(expr, Expr::U16(0))
}

pub fn is_nonzero_u32(expr: Expr) -> Expr {
    expr_gt(expr, Expr::U32(0))
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
pub fn fmt_some(f: Format) -> Format {
    Format::LiftedOption(Some(Box::new(f)))
}

/// Helper for constructing `Option::None` within the Format model-language.
pub const fn fmt_none() -> Format {
    Format::LiftedOption(None)
}

/// Shortcut for `where_lambda` applied over the simple predicate [`is_nonzero`]
pub fn where_nonzero<T: ZeroMarker>(format: Format) -> Format {
    where_lambda(format, "x", is_nonzero::<T>(var("x")))
}

/// Helper for constructing `Format::ForEach`
pub fn for_each(seq: Expr, name: impl IntoLabel, inner: Format) -> Format {
    Format::ForEach(Box::new(seq), name.into(), Box::new(inner))
}

/// Helper for specifying a byte-aligned Format with a given byte-multiple `align`
pub fn aligned(f: Format, align: usize) -> Format {
    monad_seq(Format::Align(align), f)
}

/// Helper method for [`Format::LetFormat`]
#[inline]
pub fn chain(f0: Format, name: impl IntoLabel, f: Format) -> Format {
    Format::LetFormat(Box::new(f0), name.into(), Box::new(f))
}

/// Helper method for [`Format::MonadSeq`]
#[inline]
pub fn monad_seq(f0: Format, f: Format) -> Format {
    Format::MonadSeq(Box::new(f0), Box::new(f))
}

/// Helper for destructuring an `Expr`-level tuple-value into a set of locally bound variables.
///
/// # Notes
///
/// The entire tuple does not have to be enumerated, but there must not be more labels than the arity of the tuple allows for.
pub fn with_tuple<const N: usize>(
    tuple: Expr,
    labels: [&'static str; N],
    format: Format,
) -> Format {
    Format::Match(
        Box::new(tuple),
        vec![(
            Pattern::Tuple(labels.into_iter().map(bind).collect()),
            format,
        )],
    )
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
/// error at runtime due to a failed pattern-match.
pub fn expr_unwrap(expr: Expr) -> Expr {
    Expr::Match(Box::new(expr), vec![(pat_some(bind("x")), var("x"))])
}

/// Performs a fallible destructuring of the provided `expr` within the Expr layer,
/// either extracting the single element of a length-1 array, or resulting in an
/// ExcludedBranch error at runtime due to a failed pattern-match.
pub fn unwrap_singleton(expr: Expr) -> Expr {
    Expr::Match(
        Box::new(expr),
        vec![(Pattern::Seq(vec![bind("x")]), var("x"))],
    )
}

/// Parses a dependent format `opt_f(x)` if `expr` evaluates to `Some(x)`,
/// or `fmt_none` when `expr` evaluates to `None`.
///
/// Implicitly relies on the ValueType of the output of `opt_f` being `Option<U>`,
/// following the style of monadic bind operations in languages like Haskell.
///
/// # Notes
///
/// To offer more fine-tuning for the generated output, a `binding_name` parameter is required,
/// and used as the pattern-binding of `Some(_)` against `expr`, as well as the variable passed
/// into `opt_f`.
pub fn bind_option(
    expr: Expr,
    binding_name: &'static str,
    opt_f: impl FnOnce(Expr) -> Format,
) -> Format {
    Format::Match(
        Box::new(expr),
        vec![
            (pat_some(bind(binding_name)), opt_f(var(binding_name))),
            (pat_none(), fmt_none()),
        ],
    )
}

/// Parses a dependent format `fmt_some(f(x))` if `expr` evaluates to `Some(x)`,
/// or `fmt_none` when `expr` evaluates to `None`.
///
/// The output ValueType of `f` should be the parametric type `U` of whatever `Option<U>`
/// the call should evaluate to; this following the style of functor-map operations in
/// languages like Haskell.
pub fn map_option(
    expr: Expr,
    binding_name: &'static str,
    f: impl FnOnce(Expr) -> Format,
) -> Format {
    let opt_f = move |x: Expr| fmt_some(f(x));
    bind_option(expr, binding_name, opt_f)
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
        expr_destruct(
            var(HEAD_VAR),
            Pattern::Tuple(names.into_iter().map(bind).collect()),
            body,
        ),
    )
}

/// Boilerplate helper for [`Pattern`]-based destructuring of an `Expr` (`value`)
/// for post-processing in `body`.
///
/// Intended for cases where the `Pattern` is irrefutable.
///
/// Will result in runtime failure if the pattern does not match.
pub fn expr_destruct(value: Expr, pattern: Pattern, body: Expr) -> Expr {
    // FIXME - develop a first-class solution for this paradigm
    Expr::Destructure(Box::new(value), pattern, Box::new(body))
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

/// Given a single label-`Expr` pair and a record-kinded `Expr` with a list of field-labels,
/// constructs a unified record-kinded `Expr` whose first field is `field` and whose remaining
/// fields are the given list of `original` fields via record-projection.
///
/// Note that the list of field-labels given in `original.1` must not contain any field-labels
/// that are absent from `original.0`, but otherwise, may represent an arbitrary subset-permutation
/// of the actual record-field labels in `original.0`. It should not include any field more than once,
/// as this is not typically supported in the record model and may lead to breakage.
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
/// of each sub-list into a contiguous array whose elements appear in the natural order (e.g. `[[1,2,3],[4,5],[6]] -> [1,2,3,4,5,6]`)
#[inline]
pub fn concat(xs: Expr) -> Expr {
    flat_map(f_id(), xs)
}

/// Helper for the lambda-abstracted form of [`concat`].
pub fn f_concat() -> Expr {
    lambda("xs", concat(var("xs")))
}

/// Given a sequence `seq` of type `Seq(T)`, return an expression of type `Bool`
/// that is `true` if any element of `seq` yields `true` when `f` is called over it,
/// and `false` otherwise (including when the sequence is empty).
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
/// Given a default value `dft` of type `Expr@U`, and a callable `f` mapping `Expr@T -> Expr@U`,
/// as well as a value `x` of type `Expr@Option(T)`, computes the value of type `Expr@U`
/// corresponding to `f` applied to the `Some(_)` case, or dft if `x` is `None`.
pub fn expr_option_map_or(dft: Expr, f: impl FnOnce(Expr) -> Expr, x: Expr) -> Expr {
    expr_match(x, [(pat_some(bind("x")), f(var("x"))), (pat_none(), dft)])
}

/// Fused [`std::option::Option::or`] and [`std::option::Option::unwrap`], which takes two `Option@T`-kinded `Expr` values,
/// `a` and `b`, and returns the `T` value of the first one of them (left-to-right) that has a value.
///
/// If both are `Some(_)`, the value of `a` will be returned over the value of `b`.
///
/// # Notes
///
/// Will produce runtime parse-error if both are `None`.
pub fn expr_option_unwrap_first(a: Expr, b: Expr) -> Expr {
    expr_match(
        a,
        [
            (pat_some(bind("x")), var("x")),
            (pat_none(), expr_unwrap(b)),
        ],
    )
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

/// Constructs a balanced (i.e. minimized max depth) tree of `bitor`-joined
/// nodes of type Expr.
///
/// Will yield an unbalanced AST if there are more than 16 elements in `nodes`
pub fn balanced_bitor(nodes: Vec<Expr>) -> Expr {
    balance_merge((), move |_| nodes, bit_or)
}

/// Generic function for computing an N-way binary operation using a generic seed-value
/// and generation-function.
///
/// The `combine` operation should ideally be invariant under reordering and regrouping
/// (i.e. commutative and associative) as the internal tree structure of the Expr is not
/// specified.
///
/// Relies on the guarantee that the given seed and initialization function will together
/// produce a non-empty Vector, and will panic if the resulting vector is empty.
pub fn balance_merge<Seed, MkNodes, Combine>(
    seed: Seed,
    mk_nodes: MkNodes,
    combine: Combine,
) -> Expr
where
    MkNodes: FnOnce(Seed) -> Vec<Expr>,
    Combine: Fn(Expr, Expr) -> Expr,
{
    let nodes = mk_nodes(seed);

    if nodes.is_empty() {
        unreachable!("balance_merge: mk_nodes(seed) yielded empty vector");
    }

    let mut stratum = nodes;
    loop {
        match stratum.len() {
            0 => unreachable!("stratum cannot be empty"),
            1 => return stratum.drain(..).next().unwrap(),
            _ => {
                let mut tmp = Vec::with_capacity(stratum.len().div_ceil(2));
                let mut it = stratum.drain(..);
                while let Some(l) = it.next() {
                    if let Some(r) = it.next() {
                        tmp.push(combine(l, r));
                        continue;
                    } else {
                        tmp.push(l);
                        break;
                    }
                }
                std::mem::drop(it);
                stratum = tmp;
            }
        }
    }
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
pub fn seq_last_checked(seq: Expr) -> Expr {
    expr_opt_if(
        is_nonzero::<U32>(seq_length(seq.clone())),
        index_unchecked(seq.clone(), pred(seq_length(seq))),
    )
}

/// Computes the final element of a sequence-typed Expr provided that it is guaranteed to be non-empty.
///
/// Will result in a runtime panic if called on an empty sequence.
pub fn seq_last_unchecked(seq: Expr) -> Expr {
    index_unchecked(seq.clone(), pred(seq_length(seq)))
}

/// Returns `true` if the value of `x` is contained by `bounds` and false if it lies outside.
///
/// If `x` is not an integral-typed value, will cause a runtime error
/// when encountered by the interpreter or compiler.
pub fn is_within(x: Expr, bounds: Bounds) -> Expr {
    expr_match(
        x,
        [
            (Pattern::Int(bounds), Expr::Bool(true)),
            (Pattern::Wildcard, Expr::Bool(false)),
        ],
    )
}

/// Returns `true` if the value of `x` falls within any of the `Bounds` values constructed from
/// the items within an iterable container `ranges`.
///
/// If `x` is not an integral-typed value, will cause a runtime error when encountered
/// by the interpreter or compiler.
///
/// # Notes
///
/// `ranges` may be empty, but this will yield a trivially always-false conditional.
///
/// The items within `ranges` may overlap without issue, but typically should be disjoint.
///
/// The order in which the items in `ranges` are iterated through does not affect the
/// semantics, but may affect performance.
pub fn is_within_any<R>(x: Expr, ranges: impl IntoIterator<Item = R>) -> Expr
where
    R: Into<Bounds>,
{
    let branches = ranges
        .into_iter()
        .map(|r| (Pattern::Int(r.into()), Expr::Bool(true)))
        .chain(std::iter::once((Pattern::Wildcard, Expr::Bool(false))));
    expr_match(x, branches)
}

/// Helper function for constructing [`Format::WithRelativeOffset`] relative to the specified `base_address`, or defaulting
/// to the immediate buffer-position when `base_address` is `None`.
///
/// The offset `offset` is the position, relative to `base_address`, where the parse of `format` is performed.
///
/// For absolute addressing, `base_address` can be set to `Some(Expr::U32(0))` (or any other integer-kinded `Expr` variant over `0`).
pub fn with_relative_offset(base_address: Option<Expr>, offset: Expr, format: Format) -> Format {
    match base_address {
        Some(addr) => {
            Format::WithRelativeOffset(Box::new(addr), Box::new(offset), Box::new(format))
        }
        None => chain(
            Format::Pos,
            "__here",
            Format::WithRelativeOffset(Box::new(var("__here")), Box::new(offset), Box::new(format)),
        ),
    }
}

/// Gets the current stream-position and casts down from U64->U32
// REVIEW - Since the typechecker now infers a semi-auto type for Format::Pos rather than forcing U64, the cast may be extraneous...
pub fn pos32() -> Format {
    map(Format::Pos, lambda("x", Expr::AsU32(Box::new(var("x")))))
}

pub fn fmt_let(clone_varname: &'static str, orig: Expr, dep_format: Format) -> Format {
    Format::Let(
        Label::Borrowed(clone_varname),
        Box::new(orig),
        Box::new(dep_format),
    )
}

/// Helper for [`Expr::EnumFromTo`].
pub fn enum_from_to(start: Expr, end: Expr) -> Expr {
    Expr::EnumFromTo(Box::new(start), Box::new(end))
}

/// Helper for [`Expr::FindByKey`].
pub fn find_by_key(
    is_sorted: bool,
    key_fn: impl FnOnce(Expr) -> Expr,
    query: Expr,
    array: Expr,
) -> Expr {
    Expr::FindByKey(
        is_sorted,
        Box::new(lambda("elem", key_fn(var("elem")))),
        Box::new(query),
        Box::new(array),
    )
}

/// Repackages a `Seq(U8)` format as an ASCII-string format.
pub fn mk_ascii_string(x: Format) -> Format {
    Format::Hint(StyleHint::AsciiStr, Box::new(x))
}

/// Helper for [`Format::LetView`]
pub fn let_view<Name: IntoLabel>(name: Name, format: Format) -> Format {
    Format::LetView(name.into(), Box::new(format))
}

/// Helper for [`Format::WithView`]
pub fn with_view(view: ViewExpr, view_format: ViewFormat) -> Format {
    Format::WithView(view, view_format)
}

/// Helper for [`Format::ParseFromView`]
pub fn parse_from_view(view: ViewExpr, format: Format) -> Format {
    Format::ParseFromView(view, Box::new(format))
}

/// Helper for [`ViewFormat::CaptureBytes`]
pub fn capture_bytes(len: Expr) -> ViewFormat {
    ViewFormat::CaptureBytes(Box::new(len))
}

/// Helper for [`ViewFormat::ReadArray`]
pub fn read_array(len: Expr, kind: BaseKind<Endian>) -> ViewFormat {
    ViewFormat::ReadArray(Box::new(len), kind)
}

pub mod base {
    use super::*;
    use crate::CommonOp;

    macro_rules! endian {
        ( $( $fname:ident, $kind_endian:ident, $size:expr, $op:ident );* $(;)? ) => {
            $(
                pub fn $fname() -> Format {
                    Format::Hint(
                        StyleHint::Common(CommonOp::EndianParse(BaseKind::$kind_endian)),
                        Box::new(map(
                            tuple_repeat($size, Format::ANY_BYTE),
                            lambda("x", Expr::$op(Box::new(var("x")))),
                        ))
                    )
                }
            )*
        };
    }

    pub fn u8() -> Format {
        Format::Hint(
            StyleHint::Common(CommonOp::EndianParse(BaseKind::U8)),
            Box::new(Format::ANY_BYTE),
        )
    }

    endian! {
        u16be, U16BE, 2, U16Be;
        u16le, U16LE, 2, U16Le;
        u32be, U32BE, 4, U32Be;
        u32le, U32LE, 4, U32Le;
        u64be, U64BE, 8, U64Be;
        u64le, U64LE, 8, U64Le;
    }
}
pub use base::*;
