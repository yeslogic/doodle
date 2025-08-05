pub mod error;
pub mod offset;
pub mod view;

use error::{PResult, ParseError, StateError};
use offset::{BufferOffset, ByteOffset};
pub use view::View;

/// Stateful parser with an associated buffer and offset-tracker.
pub struct Parser<'a> {
    pub(crate) buffer: View<'a>,
    pub(crate) offset: BufferOffset,
}

impl<'a> Parser<'a> {}

impl<'a> From<View<'a>> for Parser<'a> {
    fn from(view: View<'a>) -> Self {
        let max_offset = ByteOffset::from_bytes(view.buffer.len());
        Self {
            buffer: view,
            offset: BufferOffset::new(max_offset),
        }
    }
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser` from an immutably-borrowed buffer.
    pub fn new(buffer: &'a [u8]) -> Parser<'a> {
        let max_offset = ByteOffset::from_bytes(buffer.len());
        Self {
            buffer: View::new(buffer),
            offset: BufferOffset::new(max_offset),
        }
    }

    /// Creates a [`View`] starting from the current position in the Parser,
    /// provided it is in Bytes-mode.
    ///
    /// # Panics
    ///
    /// Will panic if the Parser is currently in bits-mode.
    pub fn view(&self) -> View<'a> {
        let (cur_offset, None) = self.offset.get_current_offset().as_bytes() else {
            panic!("cannot open view while in bits-mode processing");
        };
        self.buffer.offset(cur_offset).unwrap()
    }

    /// Advances the offset by `offset` positions, as if calling [`Self::read_byte`] that many
    /// times and ignoring the output beyond `Ok`/`Err` distinctions.
    ///
    /// In particular, this will advance the specified number of bits in bits-mode, and bytes in
    /// bytes-mode, and otherwise has no inherent unit attached to it.
    ///
    /// # Note
    ///
    /// For convenience of usage in generated code, the type of the `offset` parameter
    /// is `u32`, as that is much more commonly used in `Format` and `Expr` internally
    /// than `usize`, which would be the natural type for this parameter.
    pub fn advance_by<N>(&mut self, offset: N) -> Result<(), ParseError>
    where
        N: TryInto<usize, Error: std::fmt::Debug>,
    {
        let delta = offset.try_into().unwrap();
        self.offset.try_increment(delta)?;
        Ok(())
    }

    /// Performs the simplest offset-modifying operation required to reach the specified `dest_offset`,
    /// treated as an absolute byte-offset (or relative to the start of stream). Returns `Ok(true)` if
    /// the operation performed was a simple advance, `Ok(false)` if the operation performed was a
    /// random-access seek, and `Err` if an error occurred.
    ///
    /// Will default to a simple advance if possible, and fall back to random-access seek only when
    /// strictly necessary.
    ///
    /// # Note
    ///
    /// To avoid forcing conditional branches in generated code, an implicit peek-context is opened
    /// just before the advance-operation when not seeking. This means that the caller will not have
    /// to manually open a peek context, as well as not having to conditionally recover an extra time
    /// as a result if a seek is performed.
    pub fn advance_or_seek<N>(&mut self, dest_offset: N) -> Result<bool, ParseError>
    where
        N: TryInto<usize, Error: std::fmt::Debug> + Copy,
    {
        let dest = dest_offset.try_into().unwrap();
        if dest < self.buffer.start_offset {
            return Err(ParseError::NegativeIndex {
                abs_target: dest,
                abs_buf_start: self.buffer.start_offset,
            });
        }
        let o_dest = dest - self.buffer.start_offset;
        let dest_offset = ByteOffset::from_bytes(o_dest);
        let is_advance =
            if let Some(delta) = self.offset.get_current_offset().checked_delta(dest_offset) {
                self.offset.open_peek();
                self.offset.try_increment(delta)?;
                true
            } else {
                self.offset.seek_to_offset(o_dest, false)?;
                false
            };
        Ok(is_advance)
    }

    pub fn skip_remainder(&mut self) {
        let after_skip = self.offset.current_limit();
        unsafe {
            self.offset.set_offset(after_skip);
        }
    }

    /// Attempts to advance the buffer by one after capturing the value of the byte at the current logical
    /// offset into the buffer.
    ///
    /// In bits-mode, this will be a sub-indexed 0-or-1-valued `u8` of the bit in question,
    /// reading from LSB to MSB of each byte in turn.
    ///
    /// Otherwise, it will be an entire byte.
    pub fn read_byte(&mut self) -> Result<u8, ParseError> {
        let (ix, sub_bit) = self.offset.try_increment(1)?.as_bytes();
        let Some(&byte) = self.buffer.buffer.get(ix) else {
            return Err(ParseError::InternalError(StateError::OutOfBoundsRead));
        };
        let ret = if let Some(n) = sub_bit {
            let i = n as u8;
            (byte & (1 << i)) >> i
        } else {
            byte
        };
        Ok(ret)
    }

    /// Advances the current buffer-offset by the minimum number of positions,
    /// in the range `0..n`, such that it is aligned to the nearest greater-or-equal
    /// multiple of `n`.
    ///
    /// In bytes-mode, alignment is measured in terms of how many bytes have been read
    /// since the start of the entire buffer.
    ///
    /// In bits-mode, alignment is measured in terms of how many **bits** have been read
    /// since **entering** bits-mode, and the absolute position within the full buffer
    /// is not regarded.
    ///
    /// # Examples
    ///
    /// ```
    /// use doodle::parser::{Parser, offset::ByteOffset};
    /// let buf = [0u8; 64];
    /// let mut p = Parser::new(&buf);
    ///
    /// let _ = p.read_byte().unwrap(); // now at offset 1B
    /// p.enter_bits_mode().unwrap(); // now at offset 1B:+0b
    /// let _ = p.read_byte().unwrap(); // now at offset 1B:+1b
    /// p.skip_align(16).unwrap(); // now at offset 1B:+16b (and not 1B:+8b ~ 2B (% 16b = 0), or 1B:+120b ~ 16B (% 16B = 0))
    /// assert_eq!(p.get_current_offset(), ByteOffset::Bits { starting_byte: 1, bits_advanced: 16 });
    /// p.escape_bits_mode().unwrap(); // now at offset 3B
    /// p.skip_align(4).unwrap(); // now at offset 4B
    /// assert_eq!(p.get_current_offset(), ByteOffset::Bytes(4));
    /// ```
    pub fn skip_align(&mut self, n: usize) -> Result<(), ParseError> {
        let current_offset = self.offset.get_current_offset();
        let aligned_offset = match current_offset {
            ByteOffset::Bytes(nbytes) if nbytes % n == 0 => current_offset,
            ByteOffset::Bits { bits_advanced, .. } if bits_advanced % n == 0 => current_offset,

            ByteOffset::Bytes(nbytes) => ByteOffset::from_bytes(((nbytes / n) + 1) * n),
            ByteOffset::Bits {
                starting_byte,
                bits_advanced,
            } => ByteOffset::Bits {
                starting_byte,
                bits_advanced: (((bits_advanced / n) + 1) * n),
            },
        };
        let delta = current_offset.delta(aligned_offset);
        self.offset.try_increment(delta)?;
        Ok(())
    }

    /// While in bytes-mode (the implicit default), switches to bits-mode until we return to an
    /// earlier offset due to a `restore` or `recover` operation (corner-case),
    /// or until `escape_bits_mode` is called (more common).
    ///
    /// # Note
    ///
    /// Will return an `Err(ParseError::Internal)` value if the current binary mode is already
    /// bits-mode.
    pub fn enter_bits_mode(&mut self) -> Result<(), ParseError> {
        self.offset.enter_bits_mode()
    }

    /// Explicitly escapes from bits-mode and returns to bytes-mode, skipping to the next full byte
    /// if at least one bit of the immediate byte has already been consumed.
    ///
    /// # Note
    ///
    /// Will return an `Err(ParseError::Internal)` value if the current binary mode is not
    /// bits-mode.
    pub fn escape_bits_mode(&mut self) -> Result<usize, ParseError> {
        self.offset.escape_bits_mode()
    }

    /// Returns the number of bits that have been read thus-far in the current
    /// bits-mode context.
    ///
    /// Will return `None` instead if we are not in bits-mode.
    pub fn get_bits_read(&self) -> Option<usize> {
        self.offset.get_current_offset().bits_advanced()
    }

    /// Attempts to open a new slice starting at the current offset and spanning
    /// `size` bytes of buffer.
    ///
    /// Will fail with an appropriate `Err` value if the size of the slice is too large
    /// to fit into an existing slice-context, or would run past the end of the buffer itself.
    pub fn start_slice(&mut self, size: usize) -> Result<(), ParseError> {
        let _current_offset = self.offset.get_current_offset();
        let end = _current_offset.increment_by(size);
        let current_limit = self.offset.current_limit();
        if end > current_limit {
            return Err(ParseError::InternalError(StateError::UnstackableSlices {
                current_offset: _current_offset,
                current_limit,
                new_slice_end: end,
            }));
        }
        unsafe {
            self.offset.open_slice_unchecked(size);
        }
        Ok(())
    }

    /// Attempts to close the most recently-opened slice, skipping to the end of the slice
    /// if successful.
    ///
    /// Will only fail if there are no slices to close, or if the current offset-value somehow
    /// exceeded the slice end-point unexpectedly. In either case, the error cannot be recovered
    /// from.
    pub fn end_slice(&mut self) -> PResult<()> {
        self.offset.close_slice()?;
        Ok(())
    }

    /// Opens a new Peek context, marking the current offset and its modality to be restored
    /// when the matching [`Parser::close_peek_context`] call is reached.
    pub fn open_peek_context(&mut self) {
        self.offset.open_peek()
    }

    /// Closes the most recently-opened Peek context, restoring the offset and its modality
    /// at the time at which it was opened.
    ///
    /// Will return an error if there is no peek context to close.
    pub fn close_peek_context(&mut self) -> PResult<()> {
        self.offset.close_peek()?;
        Ok(())
    }

    /// Opens a new PeekNot context, marking the current offset and its modality to be recover3d
    /// when the matching [`Parser::close_peek_not_context`] call is reached.
    pub fn open_peek_not_context(&mut self) {
        self.offset.open_peek_not()
    }

    /// Closes the most recently-opened PeekNot context, recovering the offset and its modality
    /// at the time at which it was opened.
    ///
    /// Will return an error if there is no PeekNot context to close.
    pub fn close_peek_not_context(&mut self) -> PResult<()> {
        self.offset.recover()?;
        Ok(())
    }

    /// Opens an 'Alts' (non-deterministic alternation) context (as with `Format::UnionNondet`/`Decoder::Alts`),
    /// saving the immediate value of the offset to be recovered upon any branch of the alternation
    /// encountering a failed parse that is not caught by an intervening fail-safe recovery-point, as
    /// with another `Alts` or a `PeekNot` within any branch.
    pub fn start_alt(&mut self) {
        self.offset.open_parallel()
    }

    /// Rewinds to the branch-point of the latest 'Alts' context (e.g. after an individual branch could not be parsed successfully).
    ///
    /// According to the `is_last` parameter, may also re-create the branch-point to allow chaining with future branches.
    ///
    ///   - If `is_last == true`, it is assumed that the new branch we are proceeding to is the final one, and we should no longer
    ///     be able to recover to the branch-point if it encounters failure
    ///   - If `is_last == false`, it is assumed there is at least one more branch after the one we are proceeding to attempt to parse,
    ///     and we will be able to recover to the branch-point if the immediately successive branch-parse also fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Format: Format::UnionNondet(vec![("A", A), ("B", B), ("C", C)])
    /// // Derived Logic (without nesting, the following is desirable)
    /// let mut p = Parser::new(&buf);
    /// p.start_alt();
    /// match parse_a(&mut p) {
    ///     Ok(result_a) => return ..., // short circuit to escape future alternation branches
    ///     Err(_) => p.next_alt(false)?, // the next branch, B, is not the last
    /// }
    /// match parse_b(&mut p) {
    ///     Ok(result_b) => return ..., // short circuit to escape future alternation branches
    ///     Err(_) => p.next_alt(true)?, // the next branch, C, is the last
    /// }
    /// match parse_c(&mut p) {
    ///     Ok(result_c) => return ...,
    ///     Err(e) => return Err(e),
    /// }
    /// ```
    pub fn next_alt(&mut self, is_last: bool) -> Result<(), ParseError> {
        self.offset.recover()?;
        if !is_last {
            self.start_alt();
        }
        Ok(())
    }

    /// Returns the number of bytes that remain in the current, possibly slice-limited context.
    ///
    /// The return value will be `0` if and only if `self.read_byte` will return `Err(ParseError::Overrun(...))`.
    pub fn remaining(&self) -> usize {
        self.offset.rem_local()
    }

    /// Attempts to finish parsing as implied by an `EndOfInput` token, returning an error
    /// if we have not actually finished processing either a slice or the entire buffer.
    ///
    /// The error value will indicate internally how many unconsumed bytes remained at the time of failure.
    pub fn finish(&self) -> Result<(), ParseError> {
        match self.remaining() {
            0 => Ok(()),
            n => Err(ParseError::IncompleteParse { bytes_remaining: n }),
        }
    }

    /// Returns the current value of the `ByteOffset` stored internally.
    pub fn get_current_offset(&self) -> ByteOffset {
        self.offset.get_current_offset()
    }

    /// Returns a `u64`-valued byte-offset equivalent to the stored `ByteOffset`, for
    /// use in association with [`Format::Pos`].
    ///
    /// # Panics
    ///
    /// As currently defined, will panic if we are in bits-mode.
    pub fn get_offset_u64(&self) -> u64 {
        match self.get_current_offset() {
            ByteOffset::Bytes(n) => n as u64,
            ByteOffset::Bits { .. } => {
                // FIXME - this panic should perhaps be eliminated if possible
                unreachable!("no unequivocal way to compute a byte-offset while in bits-mode");
            }
        }
    }
}
