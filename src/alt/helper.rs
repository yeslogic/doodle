use super::{EpiFormat, FormatExt, MonoKind};
use crate::{IntoLabel, StyleHint};

pub fn record_auto<Name: IntoLabel + AsRef<str>>(
    fields: impl IntoIterator<Item = (Name, FormatExt), IntoIter: DoubleEndedIterator>,
) -> FormatExt {
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
        Item = (Option<(Name, bool)>, FormatExt),
        IntoIter: DoubleEndedIterator,
    >,
) -> FormatExt {
    let mut rev_fields = fields_persist
        .into_iter()
        .rev()
        .collect::<Vec<(Option<(Name, bool)>, FormatExt)>>();
    let accum = Vec::with_capacity(rev_fields.len());
    FormatExt::Epi(EpiFormat::Mono(
        MonoKind::Hint(StyleHint::Record { old_style: false }),
        Box::new(FormatExt::__chain_record(accum, &mut rev_fields)),
    ))
}
