//! Fixed-shape analysis for `Format`s for [`crate::marker::FixedReadKind::FixedFormat`] validation and subsequent processing

use crate::record_fmt::RecordFormat;
use crate::{BaseKind, CommonOp, Endian, Format, FormatModule, FormatRef, Label, StyleHint};
use anyhow::{Result as AResult, anyhow};

// REVIEW - currently we are only able to compute the fixed-shape of a `Format` if it is record-shaped and uses raw commonops without any FormatRef indirection calls
// REVIEW - cases that are unhandled: tuples, formatref calls, bit-flags records, nested fixedformats, variant-wrappers around commmonops

impl From<BaseKind<Endian>> for SpineElem {
    fn from(kind: BaseKind<Endian>) -> Self {
        SpineElem::Raw(kind)
    }
}

// NOTE - because FormatRefs need to be validated, we deliberately do not implement `From<FormatRef>`

/// An 'atomic' fixed-size parse operation that consumes only one element of some `BaseKind<Endian>`, whether directly or through some indirection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpineElem {
    /// Simple base-kind parse via `Format::Hint(StyleHint::CommonOp(_), _)``
    Raw(BaseKind<Endian>),
    /// Indirect implicit base-kind parse that is performed but potentially wrapped in a Variant or mapped to a computed object (e.g. `bit_fields_u16`).
    ///
    /// The `BaseKind<Endian>` is the kind of the underlying primitive read, carried alongside the
    /// `FormatRef` because it is not recoverable from `RustType`/codegen state downstream (a
    /// `u32` field looks the same whether it was parsed as `u32be` or `u32le`).
    Indirect(FormatRef, BaseKind<Endian>),
}

///
/// This is the precondition for a `FormatRef` to be usable as the element-type of a strided
/// `ReadArray` (i.e. a candidate for `FixedReadKind::FixedFormat`): every field must have a
/// statically-known byte-width with no data-dependent control flow, so that `stride` can be
/// relied on to skip to the next element regardless of what an individual element-parse consumes.
#[derive(Debug, Clone)]
pub(crate) enum FixedShape {
    /// Single SpineElem with a statically-known stride
    ///
    /// We are not forced to store which `BaseKind<Endian>` it amounts to, but we at least store its stride so that we do not need to thread in `FormatModule` to determine how wide it is.
    Single { format: SpineElem, stride: usize },
    /// The flattened, order-preserving field-layout of a `Format` that is provably fixed-size and
    /// composed entirely of base-kind (`u8`/`u16be`/`u32be`/`u64be`, ...) primitive reads.
    Record {
        /// Ordered list of fields, alongside the persisted name of the field (if any -- ephemeral
        /// and anonymous fields have no name to report but still occupy space in the layout).
        fields: Vec<(Option<Label>, SpineElem)>,
        /// Total byte-length of one element, equal to the sum of `BaseKind::size()` over `fields`.
        stride: usize,
    },
}

/// Attempts to compute the [`FixedShape`] of `format`, for use as the element-type of a
/// fixed-format `ReadArray`.
///
/// Will return an error if `format` does not satisfy the requirements of a fixed-shape parse.
///
/// # Notes
///
/// - Nested `FixedFormat` fields (a field that is itself a fixed-shape record) are not yet
///   recognized as eligible; only the base case of purely primitive fields is supported.
/// - Little-endian base-kinds are structurally recognized here (since [`as_base_kind_read`]
///   defers to the `CommonOp::EndianParse` tag rather than re-deriving the kind from the
///   underlying byte-parse), but downstream consumers presently reject them (see the
///   `unimplemented!` arms in `codegen::model::read_array_from_view` and `decoder::read_base`).
/// - Because arbitrary tuple-formats do not end up with adhoc type bindings, we cannot predictably
///   use ReadArray over anonymous tuples and therefore tuple-types are rejected.
pub(crate) fn analyze_fixed_shape(
    module: &FormatModule,
    format_ref: FormatRef,
) -> AResult<FixedShape> {
    let format = module.get_format(format_ref.get_level());
    match RecordFormat::try_from(format) {
        Ok(record) => analyze_record(module, record, format),
        Err(_) => analyze_single(module, format_ref, format),
    }
}

fn analyze_single(
    module: &FormatModule,
    format_ref: FormatRef,
    format: &Format,
) -> AResult<FixedShape> {
    let Some((spine, size)) = as_spine_elem(module, Some(format_ref), format) else {
        return Err(anyhow!("unsupported spine-elem format: {format:?}"));
    };
    Ok(FixedShape::Single {
        format: spine,
        stride: size,
    })
}

fn analyze_record<'a>(
    module: &FormatModule,
    record: RecordFormat<'a>,
    _format: &'a Format,
) -> AResult<FixedShape> {
    let mut fields = Vec::with_capacity(record.len());
    let mut stride = 0usize;
    for (ix, (field_label, field_format)) in record.iter().enumerate() {
        // NOTE - even though anonymous and ephemeral fields do not interfere with fixed-size predictions,
        // they fundamentally violate the one-to-one correspondence between the field-layout of a defined
        // ad-hoc struct and what fixed-size parse operation it would require. As a result, we mandate
        // that all parses are persisted as permanent fields.
        let name = match field_label.to_option() {
            Some((name, true)) => Some(name.clone()),
            Some((name, false)) => {
                return Err(anyhow!(
                    "bad ephemeral-field label: ({name}: {field_format:?}) in {_format:?}"
                ));
            }
            None => {
                return Err(anyhow!(
                    "bad anonymous-field parse: (_{ix}: {field_format:?}) in {_format:?}"
                ));
            }
        };
        let (kind, size) = as_spine_elem(module, None, field_format).ok_or_else(|| {
            anyhow!(
                "field `{}` is not a fixed-size primitive read: {field_format:?}",
                name.as_deref().unwrap_or("<anonymous>"),
            )
        })?;
        stride += size;
        fields.push((name, kind));
    }
    Ok(FixedShape::Record { fields, stride })
}

/// Given a `format` and a `FormatModule` to resolve `ItemVar`, determine if `format` can be
/// expressed as a `SpineElem`, returning `Some` if so and `None` otherwise.
///
/// `self_ref`, if provided, is the `FormatRef` that `format` itself is the (raw, unwrapped) body
/// of -- i.e. `module.get_format(self_ref.get_level()) == format`. This is only meaningful (and
/// only ever `Some`) when analyzing the top-level format passed to [`analyze_fixed_shape`]
/// itself (via [`analyze_single`]): a bare `Format::Variant` has no `FormatRef` of its own to
/// indirect through unless it *is* the very format a `FormatRef` was already resolved to by the
/// caller, so record fields (which have no such anchor) always pass `None` and any bare
/// `Format::Variant` found there is correctly rejected.
///
/// Returns the `SpineElem` and the number of bytes it consumes from the buffer when being parsed.
fn as_spine_elem(
    module: &FormatModule,
    self_ref: Option<FormatRef>,
    format: &Format,
) -> Option<(SpineElem, usize)> {
    match format {
        &Format::Hint(StyleHint::Common(CommonOp::EndianParse(kind)), _) => {
            Some((kind.into(), kind.size()))
        }
        Format::ItemVar(level, exprs, views) if exprs.is_empty() && views.is_empty() => {
            let target = module.get_format(*level);
            let kind = as_indirect(target)?;
            Some((SpineElem::Indirect(FormatRef(*level), kind), kind.size()))
        }
        Format::Variant(..) => {
            let self_ref = self_ref?;
            let kind = as_indirect(format)?;
            Some((SpineElem::Indirect(self_ref, kind), kind.size()))
        }
        _ => None,
    }
}

/// Non-recursive analysis for a leaf-format to determine whether it is a valid target of
/// `FormatRef` indirection (i.e. a legal referent of `SpineElem::Indirect`).
///
/// It must only consume a static number of bytes one time, as a CommonOp, and should not have any dependent
/// or speculative buffer operations.
///
/// Returns the `BaseKind<Endian>` of the underlying primitive read.
fn as_indirect(format: &Format) -> Option<BaseKind<Endian>> {
    match format {
        Format::Variant(_, inner) => as_base_kind_read(inner),
        // TODO - figure out how to recognize `bit_fields_u16` reliably, ergonomically, and without false-positives
        other => {
            // fallthrough: all special cases are handled above and anything besides CommonOp will be rejected at this point
            as_base_kind_read(other)
        }
    }
}

/// Recognizes the `Format::Hint(StyleHint::Common(CommonOp::EndianParse(kind)), ..)` wrapper
/// that every base-kind constructor in [`crate::helper`] produces (see `helper::u8`, and the
/// `endian!`-macro-generated `u16be`/`u32be`/`u64be`/etc.), without needing to inspect or
/// re-derive the kind from the byte-tuple parse it wraps.
///
/// # Notes
///
/// Currently does not work properly for signed-integer types, as well as atypical endian-reads
/// like `U24Be`.
fn as_base_kind_read(format: &Format) -> Option<BaseKind<Endian>> {
    match format {
        Format::Hint(StyleHint::Common(CommonOp::EndianParse(kind)), _) => Some(*kind),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::*;

    /// Regression-test: are records correctly analyzed?
    #[test]
    fn analyze_fixed_shape_accepts_record() {
        let mut module = FormatModule::new();
        let empty = module.define_format(
            "empty",
            Format::Compute(Box::new(crate::Expr::Record(vec![]))),
        );
        let FixedShape::Record { fields, stride } = analyze_fixed_shape(&module, empty).unwrap()
        else {
            panic!("expected FixedShape::Record");
        };
        assert!(fields.is_empty());
        assert_eq!(stride, 0);

        // STUB - add more meaningful cases
        let rec = module.define_format(
            "rec",
            record([
                ("x8", u8()),
                ("x16be", u16be()),
                ("x32le", u32le()),
                ("x64be", u64be()),
            ]),
        );
        let FixedShape::Record { fields, stride } = analyze_fixed_shape(&module, rec).unwrap()
        else {
            panic!("expected FixedShape::Record");
        };
        assert_eq!(
            fields.as_slice(),
            &[
                (Some(Label::Borrowed("x8")), SpineElem::Raw(BaseKind::U8)),
                (
                    Some(Label::Borrowed("x16be")),
                    SpineElem::Raw(BaseKind::U16BE)
                ),
                (
                    Some(Label::Borrowed("x32le")),
                    SpineElem::Raw(BaseKind::U32LE)
                ),
                (
                    Some(Label::Borrowed("x64be")),
                    SpineElem::Raw(BaseKind::U64BE)
                ),
            ]
        );
        assert_eq!(stride, 15);
    }

    #[test]
    fn analyze_fixed_shape_rejects_tuple() {
        let mut module = FormatModule::new();
        let tup = module.define_format("tup", tuple([u8(), u32be(), u32be()]));
        assert!(analyze_fixed_shape(&module, tup).is_err());
    }

    #[test]
    fn analyze_fixed_shape_accepts_itemvar() {
        let mut module = FormatModule::new();
        let word = module.define_format("word", u32be());
        // `alias`'s own body is `Format::ItemVar(word.level(), ..)`, exercising the
        // ItemVar-indirection arm of `as_spine_elem` (as opposed to the self-referencing arm,
        // covered by `analyze_fixed_shape_accepts_variant` below).
        let alias = module.define_format("alias", word.call());
        let FixedShape::Single { format, stride } = analyze_fixed_shape(&module, alias).unwrap()
        else {
            panic!("expected FixedShape::Single");
        };
        assert_eq!(format, SpineElem::Indirect(word, BaseKind::U32BE));
        assert_eq!(stride, 4);
    }

    #[test]
    fn analyze_fixed_shape_accepts_variant() {
        let mut module = FormatModule::new();
        // `word_variant`'s own body *is* `Format::Variant(..)` directly (the natural, expected
        // usage of `FixedReadKind::FixedFormat` for a Variant-wrapped primitive read) -- this
        // exercises the self-referencing arm of `as_spine_elem`.
        let word_variant = module.define_format("word_variant", fmt_variant("Word", u32be()));
        let shape = analyze_fixed_shape(&module, word_variant).unwrap();
        let FixedShape::Single { format, stride } = shape else {
            panic!("expected FixedShape::Single");
        };
        assert_eq!(format, SpineElem::Indirect(word_variant, BaseKind::U32BE));
        assert_eq!(stride, 4);
    }

    #[test]
    fn analyze_fixed_shape_accepts_record_with_indirect_field() {
        let mut module = FormatModule::new();
        let word_variant = module.define_format("word_variant", fmt_variant("Word", u32be()));
        let rec = module.define_format("rec", record([("x", u8()), ("w", word_variant.call())]));
        let FixedShape::Record { fields, stride } = analyze_fixed_shape(&module, rec).unwrap()
        else {
            panic!("expected FixedShape::Record");
        };
        assert_eq!(
            fields.as_slice(),
            &[
                (Some(Label::Borrowed("x")), SpineElem::Raw(BaseKind::U8)),
                (
                    Some(Label::Borrowed("w")),
                    SpineElem::Indirect(word_variant, BaseKind::U32BE)
                ),
            ]
        );
        assert_eq!(stride, 5);
    }

    #[test]
    // NOTE - This is a regression so the panic is documenting the current behavior - the eventual goal is for this to pass
    #[should_panic]
    fn analyze_fixed_shape_accepts_record_with_record_field() {
        let mut module = FormatModule::new();
        let inner = module.define_format("inner", record([("x", u8()), ("y", u8())]));
        let outer = module.define_format("outer", record([("x", u8()), ("y", inner.call())]));
        let outcome = analyze_fixed_shape(&module, outer);
        // outcome will be Err because we don't have recursive support for fixed-record fields within prospective fixed-records.
        assert!(outcome.is_ok());
    }

    #[test]
    // NOTE - This is a regression so the panic is documenting the current behavior - the eventual goal is for this to pass
    #[should_panic]
    fn as_base_kind_read_accepts_signed() {
        let f = i32be();
        assert!(as_base_kind_read(&f).is_some());
    }
}
