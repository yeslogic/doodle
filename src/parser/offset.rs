use super::error::{OverrunKind, PResult, ParseError, StateError};
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ByteOffset {
    Bytes(usize),
    Bits {
        starting_byte: usize,
        bits_advanced: usize,
    },
}

impl std::fmt::Display for ByteOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_bytes() {
            (n, Some(k)) => write!(f, "{n}:{k}"),
            (n, None) => write!(f, "{n}"),
        }
    }
}

impl Default for ByteOffset {
    fn default() -> Self {
        ByteOffset::Bytes(0)
    }
}

impl ByteOffset {
    pub(crate) const fn from_bytes(nbytes: usize) -> Self {
        Self::Bytes(nbytes)
    }

    /// Calculates the increment value required for self to reach `other`,
    /// or returns `None` if the operation would constitute a decrement.
    pub(crate) fn checked_delta(self, other: Self) -> Option<usize> {
        if self.is_bit_mode() {
            other.abs_bit_offset().checked_sub(self.abs_bit_offset())
        } else if other.is_bit_mode() {
            unreachable!("cannot calculate delta-value from Byte-mode {self} to bit-mode {other}");
        } else {
            other.as_bytes().0.checked_sub(self.as_bytes().0)
        }
    }

    // Calculates the increment value required for self to reach `other`
    pub(crate) fn delta(self, other: Self) -> usize {
        if self.is_bit_mode() {
            other
                .abs_bit_offset()
                .checked_sub(self.abs_bit_offset())
                .unwrap_or_else(|| {
                    unreachable!("unrepresentable negative delta-value for {self}->{other}")
                })
        } else if other.is_bit_mode() {
            unreachable!("cannot calculate delta-value from Byte-mode {self} to bit-mode {other}");
        } else {
            other
                .as_bytes()
                .0
                .checked_sub(self.as_bytes().0)
                .unwrap_or_else(|| {
                    unreachable!("unrepresentable negative delta-value for {self}->{other}")
                })
        }
    }

    pub(crate) fn is_bit_mode(&self) -> bool {
        matches!(self, Self::Bits { .. })
    }

    pub(crate) fn increment_by(&self, delta: usize) -> Self {
        match *self {
            ByteOffset::Bytes(n_bytes) => Self::Bytes(n_bytes + delta),
            ByteOffset::Bits {
                starting_byte,
                bits_advanced,
            } => Self::Bits {
                starting_byte,
                bits_advanced: bits_advanced + delta,
            },
        }
    }

    /// Increments `self` by `delta` unconditionally, returning the old value
    /// of `self` before the offset.
    pub(crate) fn increment_assign_by(&mut self, delta: usize) -> Self {
        let ret = *self;
        match self {
            ByteOffset::Bytes(n_bytes) => {
                *n_bytes += delta;
            }
            ByteOffset::Bits {
                bits_advanced: n_bits,
                ..
            } => {
                *n_bits += delta;
            }
        }
        ret
    }

    pub(crate) fn enter_bits_mode(&mut self) -> Result<(), ParseError> {
        if let ByteOffset::Bytes(n_bytes) = *self {
            *self = ByteOffset::Bits {
                starting_byte: n_bytes,
                bits_advanced: 0,
            };
            Ok(())
        } else {
            Err(ParseError::InternalError(StateError::BinaryModeError))
        }
    }

    pub(crate) fn escape_bits_mode(&mut self) -> Result<usize, ParseError> {
        if let ByteOffset::Bits {
            starting_byte,
            bits_advanced,
        } = *self
        {
            let delta_major = bits_advanced / 8;
            let delta_minor = bits_advanced % 8;
            if delta_minor != 0 {
                *self = ByteOffset::Bytes(starting_byte + delta_major + 1);
            } else {
                *self = ByteOffset::Bytes(starting_byte + delta_major);
            }
            Ok(bits_advanced)
        } else {
            Err(ParseError::InternalError(StateError::BinaryModeError))
        }
    }

    pub(crate) fn abs_bit_offset(&self) -> usize {
        match *self {
            ByteOffset::Bytes(n) => n * 8,
            ByteOffset::Bits {
                starting_byte,
                bits_advanced,
            } => starting_byte * 8 + bits_advanced,
        }
    }

    pub(crate) fn bits_advanced(&self) -> Option<usize> {
        match self {
            ByteOffset::Bytes(_n) => None,
            &ByteOffset::Bits { bits_advanced, .. } => Some(bits_advanced),
        }
    }

    pub fn as_bytes(&self) -> (usize, Option<usize>) {
        match *self {
            ByteOffset::Bytes(n) => (n, None),
            ByteOffset::Bits {
                starting_byte,
                bits_advanced,
            } => {
                let delta_major = bits_advanced / 8;
                let delta_minor = bits_advanced % 8;
                (starting_byte + delta_major, Some(delta_minor))
            }
        }
    }
}

impl PartialOrd for ByteOffset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (n1, o1) = self.as_bytes();
        let (n2, o2) = other.as_bytes();
        let partial = n1.cmp(&n2);
        match partial {
            Ordering::Equal => match (o1, o2) {
                (None, None) => Some(Ordering::Equal),
                (Some(m1), Some(m2)) => Some(m1.cmp(&m2)),
                _ => None,
            },
            _ => Some(partial),
        }
    }
}

/// Wrapper for a `Vec`-based FILO stack of [`Lens`]es
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ViewStack {
    stack: Vec<Lens>,
}

impl ViewStack {
    /// Creates a new, empty `ViewStack`.
    pub(crate) fn new() -> Self {
        ViewStack { stack: Vec::new() }
    }

    /// Performs a stack-push operation with the provided `Lens`
    pub(crate) fn push_lens(&mut self, lens: Lens) {
        self.stack.push(lens)
    }

    /// Returns the upper-bound [`ByteOffset`] implied by a LIFO-order slice of [`Lens`]es.
    ///
    /// # Note
    ///
    /// If the slice is in non-LIFO order, the result returned by this method will be biased
    /// to the rightmost `Lens` that imposes an upper-bound, and may be inaccurate.
    fn get_limit_from_slice(slice: &[Lens]) -> Answer<ByteOffset> {
        let (lens, rest) = match slice.split_last() {
            Some(it) => it,
            None => {
                return Answer {
                    val_or_keep_going: Err(true),
                }
            }
        };
        // NOTE - because slices must nest, if we find one at any point, nothing further down will occur earlier in the buffer, so we can short-circuit
        lens.get_endpoint()
            .or_else(|| Self::get_limit_from_slice(rest))
    }

    /// Returns the upper-bound [`ByteOffset`] implied  by a given `ViewStack`.
    pub(crate) fn get_limit(&self) -> Option<ByteOffset> {
        let ret = Self::get_limit_from_slice(self.stack.as_slice());
        // FIXME - introduce caching mechanic
        ret.as_option()
    }

    /// Performs a stack-pop operation on an owned `ViewStack`, returning the
    /// resulting `ViewStack` and the former topmost element.
    pub(crate) fn escape(mut self) -> (ViewStack, Option<Lens>) {
        let ret = self.stack.pop();
        (self, ret)
    }

    /// Unrolls a `ViewStack`, successively attempting to restore each `Lens` as it is popped.
    ///
    /// Returns the ByteOffset that should be restored, and remainder of the `ViewStack`, if any `Lens`
    /// is considered a 'restore'-point.
    ///
    /// Otherwise, returns `Err(StateError::NoRestore)`.
    ///
    /// # Note
    ///
    /// In this context, 'restore' is used in apposition to 'recovery'.
    ///
    /// When a speculative parse succeeds, the offset where it was initiated is 'restored' (i.e. after a successful Peek).
    ///
    /// When a speculative parse fails,  the offset where it was initiated is 'recovered' (i.e. after a failed parse within a PeekNot, or on some branch of a UnionNondet)
    ///
    /// This convention is adopted at the [`Lens`] and [`crate::parser::Parser`] layer as well.
    pub(crate) fn restore(mut self) -> Result<(ByteOffset, ViewStack), StateError> {
        for (ix, lens) in self.stack.iter().enumerate().rev() {
            match lens.restore() {
                Some(offset) => {
                    self.stack.truncate(ix);
                    return Ok((offset, self));
                }
                None => {
                    continue;
                }
            }
        }
        Err(StateError::NoRestore)
    }

    /// Unrolls a `ViewStack`, successively attempting to recover each `Lens` as it is popped.
    ///
    /// Returns the ByteOffset that should be recovered, and remainder of the `ViewStack`, if any `Lens`
    /// is considered a 'recovery'-point.
    ///
    /// Otherwise, returns `Err(StateError::NoRecovery)`.
    ///
    /// # Note
    ///
    /// See the documentation of [`ViewStack::restore`] for the difference between 'restore' and 'recover',
    /// both as methods and as conventional terms.
    pub(crate) fn recover(mut self) -> Result<(ByteOffset, ViewStack), StateError> {
        for (ix, lens) in self.stack.iter().enumerate().rev() {
            match lens.recover() {
                Some(offset) => {
                    self.stack.truncate(ix);
                    return Ok((offset, self));
                }
                None => {
                    continue;
                }
            }
        }
        Err(StateError::NoRecovery)
    }
}

/// Enumeration over the (four) format-combinators that require special handling,
/// both for limited-view (Slice) and speculative (Peek, PeekNot, UnionNondet) parsing,
/// as well as free-context speculative random access (Seek).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Lens {
    Alts {
        checkpoint: ByteOffset,
    },
    Peek {
        checkpoint: ByteOffset,
    },
    PeekNot {
        checkpoint: ByteOffset,
    },
    /// Random-access Seek with a restore-point offset of where we were before, and a control parameter for whether the seek is transparent with respect to subordinate Slices
    Seek {
        is_transparent: bool,
        checkpoint: ByteOffset,
    },
    Slice {
        endpoint: ByteOffset,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub(crate) struct Answer<T> {
    val_or_keep_going: Result<T, bool>,
}

impl<T> Answer<T> {
    fn or_else(self, f: impl FnOnce() -> Answer<T>) -> Answer<T> {
        match self.val_or_keep_going {
            Err(false) => Self {
                val_or_keep_going: Err(false),
            },
            Err(true) => f(),
            Ok(val) => Self {
                val_or_keep_going: Ok(val),
            },
        }
    }

    fn as_option(self) -> Option<T> {
        self.val_or_keep_going.ok()
    }
}

impl Lens {
    /// Returns the proper ByteOffset to 'recover' from parse-failure that occurred within the current Lens.
    ///
    /// If the current context does not have implicit allowance for failure, returns `None` instead, to allow
    /// recovery to propagate down the ViewStack to outer contexts that may permit recovery.
    pub(crate) fn recover(&self) -> Option<ByteOffset> {
        match self {
            Lens::Alts { checkpoint } | Lens::PeekNot { checkpoint } => Some(*checkpoint),
            Lens::Peek { .. } | Lens::Slice { .. } | Lens::Seek { .. } => None,
        }
    }

    /// Returns the proper ByteOffset to 'restore' after a successful, non-consuming parse operation based on the current Lens.
    ///
    /// If there is no appropriate value to return, `None` is returned instead, to allow restoration to propagate down the ViewStack
    /// to outer contexts that may permit restoration.
    pub(crate) fn restore(&self) -> Option<ByteOffset> {
        match self {
            Lens::Peek { checkpoint } | Lens::Seek { checkpoint, .. } => Some(*checkpoint),
            Lens::Slice { .. } => None,
            // NOTE - despite having starting-offsets, Alts and PeekNot are return-on-fail rather than return-on-success
            Lens::Alts { .. } | Lens::PeekNot { .. } => None,
        }
    }

    /// Returns the local upper-bound beyond which parsing is not permissible in the current Lens.
    ///
    /// If the current Lens does not enforce such a limit, returns `None` instead, to allow back-propagation down the ViewStack
    /// until one is found or the ViewStack is exhausted.
    pub(crate) fn get_endpoint(&self) -> Answer<ByteOffset> {
        match self {
            Lens::Slice { endpoint } => Answer {
                val_or_keep_going: Ok(*endpoint),
            },
            Lens::Seek { is_transparent, .. } => Answer {
                val_or_keep_going: Err(*is_transparent),
            },
            _ => Answer {
                val_or_keep_going: Err(true),
            },
        }
    }
}

/// Comined state that tracks an index, or offset, into a buffer being parsed,
/// and stores a [`ViewStack`] to manage meta-contextual state about subarray-limited (Slice)
/// and speculative parsing (Peek, PeekNot, Alts/UnionNondet).
pub(crate) struct BufferOffset {
    /// The current value of the offset being tracked
    current_offset: ByteOffset,
    /// The stack of `Lens` objects in LIFO order
    view_stack: ViewStack,
    /// The maximum legal offset, which is one logical position past the final legal index of the buffer (i.e. equal to the buffer length when measured in bytes)
    max_offset: ByteOffset,
}

impl BufferOffset {
    /// Takes the maximum legal value for the offset (equal to the buffer's total length in bytes)
    /// and returns a new BufferOffset starting from 0.
    pub(crate) fn new(max_offset: ByteOffset) -> Self {
        Self {
            current_offset: ByteOffset::default(),
            view_stack: ViewStack::new(),
            max_offset,
        }
    }

    #[cfg(feature = "parser_from_read_ctxt")]
    pub(crate) fn with_offset(current_offset: ByteOffset, max_offset: ByteOffset) -> Self {
        Self {
            current_offset,
            view_stack: ViewStack::new(),
            max_offset,
        }
    }

    /// Returns the value of the offset being tracked
    pub(crate) fn get_current_offset(&self) -> ByteOffset {
        self.current_offset
    }

    /// Performs a seek operation, and returns the checkpoint offset if successful, or `Err` if the seek is not allowed.
    ///
    /// # Note
    ///
    /// This operation is fragile and may lead to unexpected conditions under normal parsing. If the seek-to offset
    /// is statically known to be ahead of the current offset, use [`try_increment`] instead.
    pub(crate) fn seek_to_offset(
        &mut self,
        abs_offset: usize,
        _is_transparent: bool,
    ) -> PResult<ByteOffset> {
        let destination = ByteOffset::from_bytes(abs_offset);

        if destination > self.max_offset {
            return Err(ParseError::Overrun(OverrunKind::EndOfStream {
                offset: destination,
                max_offset: self.max_offset,
            }));
        }

        let checkpoint = self.current_offset;
        if checkpoint.is_bit_mode() {
            // NOTE - this panic is a placeholder until we have a case where Seek and bit-mode parsing coincide, to inform the approach that fits this edge-case
            unreachable!("cannot perform random byte-access while in bit-parsing mode");
        }

        let is_transparent = match self.view_stack.get_limit() {
            Some(max_offset) => destination < max_offset && _is_transparent,
            None => _is_transparent,
        };

        self.current_offset = destination;
        self.view_stack.push_lens(Lens::Seek {
            is_transparent,
            checkpoint,
        });
        Ok(checkpoint)
    }

    /// Increments the current offset by `delta` if it is legal to do so.
    ///
    /// Returns the old offset if successful, or `Err` if the increment is not allowed.
    ///
    /// # Note
    ///
    /// The implicit unit of `delta` is whichever of 'bits' or 'bytes' is currently being processed.
    /// In most cases this will be bytes, but within a `Format::Bits` context, delta will measure
    /// bits within each byte.
    pub(crate) fn try_increment(&mut self, delta: usize) -> PResult<ByteOffset> {
        let slice_limit = self.view_stack.get_limit();
        let after_increment = self.current_offset.increment_by(delta);

        match slice_limit {
            Some(max_offset) => {
                if after_increment > max_offset {
                    return Err(ParseError::Overrun(OverrunKind::EndOfSlice {
                        offset: after_increment,
                        max_offset,
                    }));
                }
            }
            None => {
                if after_increment > self.max_offset {
                    return Err(ParseError::Overrun(OverrunKind::EndOfStream {
                        offset: after_increment,
                        max_offset: self.max_offset,
                    }));
                }
            }
        }
        Ok(self.current_offset.increment_assign_by(delta))
    }

    /// Switches from reading byte-by-byte to reading bit-by-bit.
    ///
    /// Whether the resulting bit-stream is in MSB-to-LSB or LSB-to-MSB order
    /// is determined by the operational semantics of the Parser in question.
    ///
    /// Will return an `Err` value if called when already in bit-by-bit mode.
    pub(crate) fn enter_bits_mode(&mut self) -> PResult<()> {
        self.current_offset.enter_bits_mode()
    }

    /// Escapes bit-by-bit mode and returns the number of bits read while in bits-mode.
    ///
    /// If at least one bit has been read since the last full-byte boundary, the remainder
    /// of that byte is skipped, and otherwise the offset remains in-place while switching
    /// between modes.
    ///
    /// Will return an `Err` value if called when already in byte-by-byte mode.
    pub(crate) fn escape_bits_mode(&mut self) -> PResult<usize> {
        self.current_offset.escape_bits_mode()
    }

    /// Pushes a `Lens` to the internal `ViewStack` without validation.
    ///
    /// # Safety
    ///
    /// When called on a `Lens::Slice` whose endpoint exceeds an extant `Slice` in the
    /// `ViewStack`, this method may lead to unexpected results, but will not be
    /// undefined behavior.
    ///
    /// When called on a `Lens::Slice` whose endpoint exceeds the maximum buffer offset,
    /// may lead to future panics due to OOB attempted reads.
    unsafe fn push_lens(&mut self, lens: Lens) {
        self.view_stack.push_lens(lens);
    }

    /// Pushes a new `Lens::Slice` to the top of the `ViewStack` that ends at offset-delta `slice_len`,
    /// without validating the upper-bound of said slice against either the most restrictive Slice on the
    /// ViewStack thusfar, or even the `max_offset` of the `BufferOffset` in question.
    ///
    /// # Note
    ///
    /// In bits-mode, the slice-len is implicitly assumed to specify a number of bits; in bytes-mode,
    /// it is implicitly assumed to specify a number of bytes.
    pub(crate) unsafe fn open_slice_unchecked(&mut self, slice_len: usize) {
        self.push_lens(Lens::Slice {
            endpoint: self.current_offset.increment_by(slice_len),
        })
    }

    /// Skips to the end of the most recently opened slice (if not there already) and removes
    /// the corresponding `Lens` from the `ViewStack`, popping any intervening `Lens`es that may
    /// occur.
    ///
    /// Returns the value of the offset after this operation is processed, which will be the upper-bound
    /// offset of the slice that was escaped.
    ///
    /// Will return an appropriate `Err` value if either of the conditions below are met:
    ///   - There is no slice to close
    ///   - The current `ByteOffset` has somehow violated the upper-bound imposed by the most recent slice
    ///
    /// # Note
    ///
    /// Closing a slice will inherently restore the byte-or-bit modality of the offset at the time
    /// the slice was opened. This means that if a slice was opened in bytes-mode, closing it will
    /// always return to bytes-mode, even if bits-mode was entered within the slice and never explicitly
    /// escaped via [`BufferOffset::escape_bits_mode`]. The same would be true in the converse,
    /// except there is no parsing meta-operation that that enters bytes-mode from within bits-mode.
    pub(crate) fn close_slice(&mut self) -> PResult<ByteOffset> {
        let mut stack = ViewStack::new();
        std::mem::swap(&mut stack, &mut self.view_stack);
        match stack.escape() {
            (stack, Some(Lens::Slice { endpoint })) => {
                if self.current_offset > endpoint {
                    return Err(ParseError::InternalError(StateError::SliceOverrun));
                }
                self.current_offset = endpoint;
                self.view_stack = stack;
                Ok(endpoint)
            }
            (stack, Some(Lens::Alts { .. })) => {
                // NOTE - if we nest a non-det union within a slice, we are closing it implicitly by precluding further fallthrough
                self.view_stack = stack;
                self.close_slice()
            }
            (_, None) => Err(ParseError::InternalError(StateError::MissingSlice)),
            (_, Some(Lens::PeekNot { checkpoint })) => {
                unreachable!(
                    "[STATE]: close-slice @{}: unexpected PeekNot <-@{}",
                    self.current_offset, checkpoint
                );
            }
            (_, Some(Lens::Peek { checkpoint })) => {
                unreachable!(
                    "[STATE]: close-slice @{}: unexpected Peek <-@{}",
                    self.current_offset, checkpoint
                );
            }
            (
                _,
                Some(Lens::Seek {
                    is_transparent,
                    checkpoint,
                }),
            ) => {
                // NOTE - this panic is here for the same reason as the Peek/PeekNot cases above, but may be removed if this case naturally crops up
                unreachable!(
                    "[STATE]: close-slice @{}: unexpected Seek[is_transparent: {}] <-@{}",
                    self.current_offset, is_transparent, checkpoint
                )
            }
        }
    }

    /// Creates and pushes a new [`Lens::Peek`] to the internal `ViewStack`.
    pub(crate) fn open_peek(&mut self) {
        let checkpoint = self.current_offset;
        let peek = Lens::Peek { checkpoint };
        self.view_stack.push_lens(peek);
    }

    /// Creates and pushes a new [`Lens::PeekNot`] to the internal `ViewStack`.
    pub(crate) fn open_peek_not(&mut self) {
        let checkpoint = self.current_offset;
        let peeknot = Lens::PeekNot { checkpoint };
        self.view_stack.push_lens(peeknot);
    }

    /// Creates and pushes a new [`Lens::Alts`] to the internal `ViewStack`.
    pub(crate) fn open_parallel(&mut self) {
        let checkpoint = self.current_offset;
        let parallel = Lens::Alts { checkpoint };
        self.view_stack.push_lens(parallel);
    }

    /// Performs a [`ViewStack::restore`] operation on the internal ViewStack, replacing
    /// the current offset and ViewStack's values with the return-value of that method call.
    ///
    /// If the `restore` operation returns an `Err`, will instead return the same error instead;
    /// if such an `Err` value is returned, `self` will be left in a semi-indeterminate state,
    /// and recovery from such an error is not possible.
    ///
    /// # Note
    ///
    /// Though the method is called `close_peek`, it is also applicable for closing `Seek` Lenses as well.
    /// As a result, it may need to be called more than once to close the `Peek`` below a `Seek`.
    pub(crate) fn close_peek(&mut self) -> Result<(), StateError> {
        let mut stack = ViewStack::new();
        std::mem::swap(&mut stack, &mut self.view_stack);
        let (offs, new_stack) = stack.restore()?;
        self.current_offset = offs;
        self.view_stack = new_stack;
        Ok(())
    }

    /// Performs an [`ViewStack::recover`] operation upon reaching a parse-failure, unwinding the internal ViewStack until a fail-safe `Lens` is popped.
    ///
    /// If the ViewStack is empty, or is exhausted before such a Lens is found, will return `Err` with the appropriate
    /// `StateError` value. In such a case, `self` will be left in a semi-indeterminate state, and there is no way to
    /// recover (in the colloquial sense) from such an error.
    pub(crate) fn recover(&mut self) -> Result<(), StateError> {
        let mut stack = ViewStack::new();
        std::mem::swap(&mut stack, &mut self.view_stack);
        let (offs, new_stack) = stack.recover()?;
        self.current_offset = offs;
        self.view_stack = new_stack;
        Ok(())
    }

    /// Returns the least-upper-bound for the offset implied by the internal state of `self`.
    ///
    /// If at least one `Lens::Slice` is active, the most-recently-added will be respected and its end-point returned.
    /// Otherwise, returns the registered `max_offset` passed in at time-of-creation via the [`BufferOffset::new`] method.
    pub(crate) fn current_limit(&self) -> ByteOffset {
        self.view_stack.get_limit().unwrap_or(self.max_offset)
    }

    /// Returns the number of bytes (or bits, in bits-mode) 'remaining'; this will be the largest value of `n`
    /// for which `self.try_increment(n)` will return an `Ok` value.
    ///
    /// If the mode changes between bits-mode and bytes-mode the return value of this method will almost always change, even if no incrementing operation is performed.
    pub(crate) fn rem_local(&self) -> usize {
        self.current_offset.delta(self.current_limit())
    }

    /// Unconditionally replaces the current offset with `offset`, returning the previous offset.
    ///
    /// # Safety
    ///
    /// This method is unsafe because it does not check that `offset` is a legal position to which we could advance without
    /// explicitly closing intervening slices, nor that it is a legal position in the overall buffer itself.
    ///
    /// It also does not check whether the new offset is greater-than-or-equal-to the current offset, meaning
    /// that we can move backwards in the buffer, which is normally impossible.
    ///
    /// Furthermore, it does not distinguish between the modality (bits vs bytes) of the original and new offset,
    /// meaning it can silently switch between two modes without any indication that the new mode is incorrect.
    ///
    /// This method should only be called with values of `offset` that are guaranteed to lie within the available
    /// view of the buffer being parsed, which are no less than the current offset, and which are in the same modality.
    pub(crate) unsafe fn set_offset(&mut self, offset: ByteOffset) -> ByteOffset {
        std::mem::replace(&mut self.current_offset, offset)
    }
}
