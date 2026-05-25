use super::*;

// REVIEW - this function breaks the convention of `-> FormatRef` but it's an edge-case already
pub(crate) fn table(_module: &mut FormatModule) -> Format {
    // REVIEW[epic=view-opaque-bytes] - we may wish to use ViewFormat in place of opaque_bytes to avoid vector allocation
    opaque_bytes()
}
