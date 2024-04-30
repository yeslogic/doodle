#[allow(dead_code)]
pub(crate) enum ParseTree<'a> {
    SimpleBuffer {
        buffer: &'a [u8],
    },
    SliceThen {
        slice: &'a ParseTree<'a>,
        after_slice: &'a ParseTree<'a>,
    },
    AsBits {
        as_bits: Box<dyn Iterator<Item = u8> + 'a>,
        raw_bytes: &'a ParseTree<'a>,
    },
    Peek {
        lookahead: &'a ParseTree<'a>,
        after_match: &'a ParseTree<'a>,
    },
    PeekNot {
        lookahead_window: &'a ParseTree<'a>,
        if_unmatched: &'a ParseTree<'a>,
    },
}
