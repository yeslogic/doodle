use doodle::bit_set::BitSet;
use doodle::{BitFormat, BitFormatModule};

use crate::format::base::*;

/// Deflate
///
#[allow(clippy::redundant_clone)]
pub fn main(module: &mut BitFormatModule, base: &BaseModule) -> BitFormat {
    let block = module.define_format(
        "deflate.block",
        record([
            ("final", BitFormat::Token(BitSet::full())),
            ("type0", BitFormat::Token(BitSet::full())),
            ("type1", BitFormat::Token(BitSet::full())),
        ]),
    );

    module.define_format("deflate.main", record([("blocks", repeat1(block))]))
}
