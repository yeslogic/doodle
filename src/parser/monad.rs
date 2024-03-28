use anyhow::Result as AResult;
use super::{error::{ParseError, StateError}, offset::{BufferOffset, ByteOffset}};

pub const PEEKNOT_LOOKAHEAD_DELTA: usize = 1024;

pub struct ParseMonad<'a> {
    pub(crate) buffer: &'a [u8],
    pub(crate) offset: BufferOffset,
}

impl<'a> ParseMonad<'a> {
    pub fn new(buffer: &'a [u8]) -> ParseMonad<'a> {
        let max_offset = ByteOffset::from_bytes(buffer.len());
        Self { buffer, offset: BufferOffset::new(max_offset) }
    }

    /// Attempts to advance ther buffer by one after capturing the value of the byte at the current logical
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

    pub fn start_slice(&mut self, size: usize) -> Result<(), ParseError> {
        let end = self.offset.get_current_offset().increment_by(size);
        let current_limit = self.offset.current_limit();
        if (end > current_limit) {
            return Err(ParseError::InternalError(StateError::UnstackableSlices));
        }
        unsafe { self.offset.open_slice_unchecked(size) };
        Ok(())
    }

    pub fn end_slice(&mut self) -> AResult<()> {
        self.offset.close_slice()?;
        Ok(())
    }

    pub fn open_peek_context(&mut self) {
        self.offset.open_peek()
    }

    pub fn close_peek_context(&mut self) -> Result<(), ParseError> {
        self.offset.close_peek()?;
        Ok(())
    }

    pub fn open_peek_not_context(&mut self) {
        self.offset.open_peeknot(PEEKNOT_LOOKAHEAD_DELTA)
    }

    pub fn close_peek_not_context(&mut self) -> Result<(), ParseError> {
        self.offset.recover()?;
        Ok(())
    }

    pub fn start_alt(&mut self) {
        self.offset.open_parallel()
    }

    pub fn next_alt(&mut self, is_last: bool) -> Result<(), ParseError> {
        self.offset.recover()?;
        if !is_last {
            self.start_alt();
        }
        Ok(())
    }

    pub fn recover(&mut self) -> Result<(), ParseError> {
        self.offset.recover()?;
        Ok(())
    }

    pub fn remaining(&self) -> usize {
        self.offset.rem_local()
    }
}
