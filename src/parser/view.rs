#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct View<'a> {
    pub(crate) buffer: &'a [u8],
    pub(crate) start_offset: usize,
}

impl<'a> View<'a> {
    /// Reads a slice of `len` bytes from the View, offset by `offset`.
    pub fn read_len(&self, len: usize) -> &'a [u8] {
        &self.buffer[..len]
    }

    pub fn offset(&self, offset: usize) -> Self {
        View {
            buffer: &self.buffer[offset..],
            start_offset: self.start_offset + offset,
        }
    }
}
