#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct View<'a> {
    pub(crate) buffer: &'a [u8],
    pub(crate) start_offset: usize,
}

impl<'a> View<'a> {
    /// Reads a slice of `len` bytes from the View, offset by `offset`.
    pub fn read_offset_len(&self, offset: usize, len: usize) -> &'a [u8] {
        &self.buffer[offset..(offset + len)]
    }
}
