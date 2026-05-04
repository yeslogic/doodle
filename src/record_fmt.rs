use anyhow::{Result as AResult, anyhow};

use crate::{Expr, Format, IntoLabel, Label, StyleHint};

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
