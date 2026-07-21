use anyhow::{Result as AResult, anyhow};

use crate::{BaseKind, CommonOp, Endian, Expr, Format, IntoLabel, Label, StyleHint};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum FieldLabel<Name> {
    /// Used for parsing values that do not need to be captured or persisted in any way (e.g. alignment/padding, reserved, 'skip remainder', etc.)
    Anonymous,
    /// Parsed value is captured under the given name and kept in-scope for dependent parses of later fields, but does not ultimately end up as a field in the final record value
    Ephemeral(Name),
    /// Parsed value is captured under `in_capture` for dependent parses of later fields, and also ends up as a field in the final record value under `in_value`
    Permanent { in_capture: Name, in_value: Name },
}

impl<'a, Name: Clone> FieldLabel<&'a Name> {
    pub fn cloned(self) -> FieldLabel<Name> {
        match self {
            FieldLabel::Anonymous => FieldLabel::Anonymous,
            FieldLabel::Ephemeral(name) => FieldLabel::Ephemeral(name.clone()),
            FieldLabel::Permanent {
                in_capture,
                in_value,
            } => FieldLabel::Permanent {
                in_capture: in_capture.clone(),
                in_value: in_value.clone(),
            },
        }
    }
}

impl<Name> FieldLabel<Name> {
    #[expect(dead_code)]
    pub fn into_label(self) -> FieldLabel<Label>
    where
        Name: IntoLabel,
    {
        match self {
            FieldLabel::Anonymous => FieldLabel::Anonymous,
            FieldLabel::Ephemeral(name) => FieldLabel::Ephemeral(name.into()),
            FieldLabel::Permanent {
                in_capture,
                in_value,
            } => FieldLabel::Permanent {
                in_capture: in_capture.into(),
                in_value: in_value.into(),
            },
        }
    }

    /// Performs a lossy conversion of `self` into `Some((ident, is_persistent))` if it is not `Anonymous`,
    /// or `None` otherwise.
    ///
    /// For permanent fields, the `ident` in the output tuple is the `in_value` name, and the `is_persistent` flag is `true`.
    ///
    /// For ephemeral fields, the `ident` in the output tuple is the ephemeral name, and the `is_persistent` flag is `false`.
    pub fn to_option(self) -> Option<(Name, bool)> {
        match self {
            FieldLabel::Anonymous => None,
            FieldLabel::Ephemeral(name) => Some((name, false)),
            FieldLabel::Permanent { in_value, .. } => Some((in_value, true)),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RecordFormat<'a> {
    pub(crate) flat: Vec<(FieldLabel<&'a Label>, &'a Format)>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct OwnedRecordFormat {
    pub(crate) flat: Vec<(FieldLabel<Label>, Format)>,
}

pub(crate) mod __impl {
    use super::{FieldLabel, Format, Label, OwnedRecordFormat, RecordFormat};
    impl<'a> std::ops::Deref for RecordFormat<'a> {
        type Target = Vec<(FieldLabel<&'a Label>, &'a Format)>;

        fn deref(&self) -> &Self::Target {
            &self.flat
        }
    }

    impl<'a> std::ops::DerefMut for RecordFormat<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.flat
        }
    }

    impl std::ops::Deref for OwnedRecordFormat {
        type Target = Vec<(FieldLabel<Label>, Format)>;

        fn deref(&self) -> &Self::Target {
            &self.flat
        }
    }

    impl std::ops::DerefMut for OwnedRecordFormat {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.flat
        }
    }

    impl<'a> From<RecordFormat<'a>> for OwnedRecordFormat {
        fn from(value: RecordFormat<'a>) -> Self {
            OwnedRecordFormat {
                flat: value
                    .flat
                    .into_iter()
                    .map(|(field_label, format)| (field_label.cloned(), format.clone()))
                    .collect(),
            }
        }
    }
}

impl<'a> TryFrom<&'a Format> for RecordFormat<'a> {
    type Error = anyhow::Error;

    fn try_from(format: &'a Format) -> Result<Self, Self::Error> {
        let mut builder = RecordBuilder::init();
        builder.accum(format)?;
        Ok(builder.finish())
    }
}

impl<'a> RecordFormat<'a> {
    pub(crate) fn lookup_value_field(&self, field_name: &Label) -> Option<(&'a Format, &'a Label)> {
        for (label, format) in &self.flat {
            match label {
                FieldLabel::Permanent {
                    in_value,
                    in_capture,
                } if *in_value == field_name => {
                    return Some((format, in_capture));
                }
                _ => continue,
            }
        }
        None
    }
}

impl OwnedRecordFormat {
    pub(crate) fn into_format(self) -> Format {
        let mut rev_fields = self
            .flat
            .into_iter()
            .rev()
            .map(|(fld_label, format)| (fld_label.to_option(), format))
            .collect::<Vec<(Option<(Label, bool)>, Format)>>();
        let accum = Vec::with_capacity(rev_fields.len());
        let old_style = rev_fields.iter().all(|(opt, _)| {
            opt.as_ref()
                .is_some_and(|(_, is_persistent)| *is_persistent)
        });
        Format::Hint(
            StyleHint::Record { old_style },
            Box::new(Format::__chain_record(accum, &mut rev_fields)),
        )
    }
}

#[derive(Debug)]
pub(crate) struct RecordBuilder<'a> {
    pub(crate) labels: Vec<Option<&'a Label>>,
    pub(crate) formats: Vec<&'a Format>,
    pub(crate) res: Option<&'a Vec<(Label, Expr)>>,
}

impl<'a> RecordBuilder<'a> {
    pub const fn init() -> Self {
        Self {
            labels: Vec::new(),
            formats: Vec::new(),
            res: None,
        }
    }

    pub fn step(&mut self, format: &'a Format) -> AResult<Option<&'a Format>> {
        match format {
            Format::Hint(StyleHint::Record { .. }, inner) => self.step(inner),
            Format::LetFormat(f, name, inner) => {
                self.labels.push(Some(name));
                self.formats.push(f);
                Ok(Some(inner))
            }
            Format::MonadSeq(f, inner) => {
                self.labels.push(None);
                self.formats.push(f);
                Ok(Some(inner))
            }
            Format::Compute(expr) => match &**expr {
                Expr::Record(res) => {
                    assert!(self.res.replace(res).is_none());
                    Ok(None)
                }
                other => Err(anyhow!("expected Record, found {other:?}")),
            },
            other => Err(anyhow!("unexpected non-Record-shape format: {other:?}")),
        }
    }

    pub fn accum(&mut self, format: &'a Format) -> AResult<()> {
        let mut node = format;
        while let Some(inner) = self.step(node)? {
            node = inner;
        }
        Ok(())
    }

    pub fn finish(self) -> RecordFormat<'a> {
        let mut flat = Vec::with_capacity(self.labels.len());
        let mut kept = std::collections::BTreeMap::new();
        for (lab, r_expr) in self.res.unwrap() {
            match r_expr {
                Expr::Var(var) => kept.insert(var, lab),
                other => {
                    unreachable!("non-variable expression in format-record construction: {other:?}")
                }
            };
        }
        for (label, format) in Iterator::zip(self.labels.into_iter(), self.formats.into_iter()) {
            let f_label = match label {
                None => FieldLabel::Anonymous,
                Some(in_capture) => {
                    // there is no check for shadowing here, so we hope that is avoided.
                    match kept.get(in_capture) {
                        Some(in_value) => FieldLabel::Permanent {
                            in_capture,
                            in_value,
                        },
                        None => FieldLabel::Ephemeral(in_capture),
                    }
                }
            };
            flat.push((f_label, format));
        }
        RecordFormat { flat }
    }
}

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
        _ => None,
    }
}
