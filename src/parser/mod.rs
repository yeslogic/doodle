use self::offset::{ BufferOffset, ByteOffset };
use anyhow::Result as AResult;

pub mod offset;

pub struct ParseMonad<'a> {
    buffer: &'a [u8],
    offset: BufferOffset,
}

impl<'a> ParseMonad<'a> {
    pub fn new(buffer: &'a [u8]) -> ParseMonad<'a> {
        let max_offset = ByteOffset::from_bytes(buffer.len());
        Self { buffer, offset: BufferOffset::new(max_offset) }
    }

    /// Attempts to advance the buffer by one after capturing the value of the byte at the current logical
    /// offset into the buffer. In bits-mode, this will be a sub-indexed 0-or-1-valued `u8` of the bit in question,
    /// reading from LSB to MSB of each byte in turn. Otherwise, it will be an entire byte.
    pub fn read_byte(&mut self) -> AResult<u8> {
        let (ix, sub_bit) = self.offset.try_increment(1)?.as_bytes();
        let byte = self.buffer[ix];
        let ret = if let Some(n) = sub_bit {
            let i = n as u8;
            byte & ((1 << i) >> i)
        } else {
            byte
        };
        Ok(ret)
    }

    pub fn enter_bits_mode(&mut self) -> AResult<()> {
        self.offset.enter_bits_mode()
    }

    pub fn escape_bits_mode(&mut self) -> AResult<()> {
        self.offset.escape_bits_mode()
    }

    pub fn start_slice(&mut self, size: usize) -> AResult<()> {
        let end = self.offset.get_current_offset().increment_by(size);
        self.offset.push_limit(end)
    }

    pub fn end_slice(&mut self) -> AResult<()> {
        self.offset.escape_limit()?;
        Ok(())
    }

    pub fn open_peek_context(&mut self) {
        self.offset.set_checkpoint(false)
    }

    pub fn close_peek_context(&mut self) -> AResult<()> {
        self.offset.return_checkpoint()
    }

    pub fn open_peek_not_context(&mut self) {
        self.offset.set_checkpoint(true)
    }

    pub fn close_peek_not_context(&mut self) -> AResult<()> {
        self.offset.recover_checkpoint()
    }

    pub fn open_parallel_context(&mut self) {
        self.offset.set_checkpoint(true)
    }

    pub fn next_parallel_branch(&mut self, has_more: bool) -> AResult<()> {
        self.offset.recover_checkpoint()?;
        Ok(self.offset.set_checkpoint(has_more))
    }

    pub fn recover(&mut self) -> AResult<()> {
        self.offset.recover_checkpoint()
    }
}
