use super::*;

// STUB[epic=horizontal-for-vertical] - this currently works, but the field-names are misleading because they are implicitly biased for hhea
pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    module.define_format("opentype.vhea.table", super::hhea::table_def())
}
