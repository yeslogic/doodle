//! Fixed-shape analysis for `Format`s for [`crate::marker::FixedReadKind::FixedFormat`] validation and subsequent processing

use crate::record_fmt::RecordFormat;
use crate::{BaseKind, CommonOp, Endian, Format, Label, StyleHint};
use anyhow::{Result as AResult, anyhow};

// REVIEW - currently we are only able to compute the fixed-shape of a `Format` if it is record-shaped and uses raw commonops without any FormatRef indirection calls
// REVIEW - cases that are unhandled: tuples, formatref calls, bit-flags records, nested fixedformats, variant-wrappers around commmonops

/// The flattened, order-preserving field-layout of a `Format` that is provably fixed-size and
/// composed entirely of base-kind (`u8`/`u16be`/`u32be`/`u64be`, ...) primitive reads.
///
/// This is the precondition for a `FormatRef` to be usable as the element-type of a strided
/// `ReadArray` (i.e. a candidate for `FixedReadKind::FixedFormat`): every field must have a
/// statically-known byte-width with no data-dependent control flow, so that `stride` can be
/// relied on to skip to the next element regardless of what an individual element-parse consumes.
#[derive(Debug, Clone)]
pub(crate) struct FixedShape {
    /// Ordered list of fields, alongside the persisted name of the field (if any -- ephemeral
    /// and anonymous fields have no name to report but still occupy space in the layout).
    pub(crate) fields: Vec<(Option<Label>, BaseKind<Endian>)>,
    /// Total byte-length of one element, equal to the sum of `BaseKind::size()` over `fields`.
    pub(crate) stride: usize,
}

/// Attempts to compute the [`FixedShape`] of `format`, for use as the element-type of a
/// fixed-format `ReadArray`. Returns an error if the format is not a record, if
/// any of its fields are ephemeral or anonymous, or if any of its fields have non-primitive
/// parses.
///
/// # Notes
///
/// - Nested `FixedFormat` fields (a field that is itself a fixed-shape record) are not yet
///   recognized as eligible; only the base case of purely primitive fields is supported.
/// - Little-endian base-kinds are structurally recognized here (since [`as_base_kind_read`]
///   defers to the `CommonOp::EndianParse` tag rather than re-deriving the kind from the
///   underlying byte-parse), but downstream consumers presently reject them (see the
///   `unimplemented!` arms in `codegen::model::read_array_from_view` and `decoder::read_base`).
/// - Certain fixed-shape parses are not yet supported, such as [`crate::helper::bit_fields_u16`] and tuple-shape
///   formats whose constituents nevertheless satisfy the requirement of primitive CommonOp parses.
pub(crate) fn analyze_fixed_shape(format: &Format) -> AResult<FixedShape> {
    let record =
        RecordFormat::try_from(format).map_err(|e| anyhow!("not a record-shaped format: {e}"))?;
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
                    "bad ephemeral-field label: ({name}: {field_format:?}) in {format:?}"
                ));
            }
            None => {
                return Err(anyhow!(
                    "bad anonymous-field parse: (_{ix}: {field_format:?}) in {format:?}"
                ));
            }
        };
        let kind = as_base_kind_read(field_format).ok_or_else(|| {
            anyhow!(
                "field `{}` is not a fixed-size primitive read: {field_format:?}",
                name.as_deref().unwrap_or("<anonymous>"),
            )
        })?;
        stride += kind.size();
        fields.push((name, kind));
    }
    Ok(FixedShape { fields, stride })
}

/// Recognizes the `Format::Hint(StyleHint::Common(CommonOp::EndianParse(kind)), ..)` wrapper
/// that every base-kind constructor in [`crate::helper`] produces (see `helper::u8`, and the
/// `endian!`-macro-generated `u16be`/`u32be`/`u64be`/etc.), without needing to inspect or
/// re-derive the kind from the byte-tuple parse it wraps.
fn as_base_kind_read(format: &Format) -> Option<BaseKind<Endian>> {
    match format {
        Format::Hint(StyleHint::Common(CommonOp::EndianParse(kind)), _) => Some(*kind),
        Format::ItemVar(level, exprs, views) if exprs.is_empty() && views.is_empty() => {
            log::info!(
                "as_base_kind_read: found ItemVar({level}, [], []), which should be handled but isn't yet"
            );
            // FIXME - include module argument and do proper recursive analysis
            None
        }
        Format::Variant(_, inner) => {
            log::info!(
                "as_base_kind_read: found Variant(_, {inner:?}), which should be handled but isn't yet"
            );
            // FIXME - do proper recursive analysis after we have the means to report the variant-wrapping appropriately
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::*;

    /// Regression-test: are records correctly analyzed?
    #[test]
    fn fixed_regression_record_correct() {
        let f0 = Format::Record(vec![]);
        let fix_f0 = analyze_fixed_shape(&f0).unwrap();
        assert!(fix_f0.fields.is_empty());
        assert_eq!(fix_f0.stride as u8, 0);

        // STUB - add more meaningful cases
        let f1 = Format::Record(vec![
            ("a", u8()),
            ("b", u16be()),
            ("c", u32le()),
            ("d", u64be()),
        ]);
        let fix_f1 = analyze_fixed_shape(&f1).unwrap();
        assert_eq!(
            fix_f1.fields.as_slice(),
            &[
                (Some("a"), BaseKind::U8),
                (Some("b"), BaseKind::U16(Endian::BE)),
                (Some("c"), BaseKind::U32(Endian::LE)),
                (Some("d"), BaseKind::U64(Endian::BE))
            ]
        );
        assert_eq!(fix_f1.stride as u8, 15);
    }

    #[test]
    #[should_panic]
    fn fixed_regresision_tuple_incorrect() {
        let f0 = tuple([u8(), u32(), u32()]);
        assert!(analyze_fixed_shape(&f0).is_ok());
    }

    #[test]
    #[should_panic]
    fn base_regression_itemvar_incorrect() {
        let mut module = FormatModule::new();
        let word = module.define_format("word", u32be());
        let f = word.call();
        assert!(as_base_kind_read(&f).is_some());
    }

    #[test]
    #[should_panic]
    fn base_regression_variant_incorrect() {
        let mut module = FormatModule::new();
        let f = fmt_variant("Word", u32be());
        assert!(as_base_kind_read(&f).is_some());
    }

    #[test]
    #[should_panic]
    fn base_regression_signed_incorrect() {
        let f = i32be();
        assert!(as_base_kind_read(&f).is_some());
    }
}
