pub use crate::byte_set::ByteSet;
pub use crate::decoder::Scope;
pub use crate::read::ReadCtxt;

// FIXME - this model does not support split_at or backtracking to previous ReadCtxt states
pub struct ParseCtxt<'a> {
    input: ReadCtxt<'a>,
}

impl<'a> ParseCtxt<'a> {
    pub fn new(bytes: &'a [u8]) -> ParseCtxt<'a> {
        Self { input: ReadCtxt::new(bytes) }
    }

    pub const fn offset(&self) -> usize {
        self.input.offset
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        let (ret, new_input) =  self.input.read_byte()?;
        self.input = new_input;
        Some(ret)
    }
}
