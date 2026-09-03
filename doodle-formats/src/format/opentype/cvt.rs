use super::*;

pub(crate) fn table(_module: &mut FormatModule) -> Format {
    repeat(i16be())
}
