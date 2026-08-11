use super::error::{OverrunKind, PResult, ParseError, ParserStateError, StateError};
use super::util::Answer;
use std::cmp::Ordering;

// SECTION - ByteOffset
/// Pure offset-value used as the internal analogue for 'index into a buffer' when parsing.
///
/// Represents either a whole-byte offset when processing the buffer normally (in 'bytes-mode'), or a bit offset (in 'bits-mode').
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ByteOffset {
    /// Whole-byte offset from the start of the buffer in question, implicitly measured in bytes
    Bytes(usize),
    /// Bit-offset measured as a number of bits advanced relative to an initial byte-offset.
    Bits {
        /// Byte-offset we were at when we entered bits-mode
        starting_byte: usize,
        /// Number of individual bits advanced since entering bits-mode
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
    /// Constructs a byte-level offset ([`ByteOffset::Bytes`]) from a raw byte-offset amount.
    pub const fn from_bytes(nbytes: usize) -> Self {
        Self::Bytes(nbytes)
    }

    /// Returns the checked advance-distance from `self` to `other`.
    ///
    /// If `other` represents a location that is at or past `self`, the return-value will be the distance between the two locations,
    /// in whichever unit of advance (i.e. bytes or bits) is applicable to `self`.
    ///
    /// If `self` is past `other`, returns `None`.
    ///
    /// When the return value is `Some(n)`, it is guaranteed that `self.increment_by(n)` will yield an equivalent location to `other`.
    ///
    /// # Notes
    ///
    /// The measure of 'equivalence' we are using to determine if `self.increment_by(n)` will yield `other` is asymmetric.
    ///   - If `self` and `other` are both in bytes-mode, we use pure equality (i.e. `self.increment_by(n)` will yield `other` precisely).
    ///   - If `self` and `other` are both in bits-mode, the results will be quasi-equal (i.e. structural equality is not implied, but [`Self::abs_bit_offset`] will always agree).
    ///   - If `self` is in bits-mode and `other` is in bytes-mode, `self.increment_by(n)` will yield a bits-mode offset that is equivalent to `other` under [`Self::abs_bit_offset`].
    ///   - If `self` is in bytes-mode and `other` is in bits-mode, this method will panic even if `other` represents a whole-byte offset (i.e. `other.bits_advanced().unwrap() % 8 == 0`).
    ///
    /// # Panics
    ///
    /// If there is no whole-number advance distance from `self` that would yield `other` or equivalent (i.e. when `other` is in bits-mode
    ///
    /// and `self` is in bytes-mode), will result in a runtime panic.
    pub(crate) fn checked_delta(self, other: Self) -> Option<usize> {
        if self.is_bit_mode() {
            other.abs_bit_offset().checked_sub(self.abs_bit_offset())
        } else if other.is_bit_mode() {
            unreachable!("cannot calculate delta-value from Byte-mode {self} to bit-mode {other}");
        } else {
            other.as_bytes().0.checked_sub(self.as_bytes().0)
        }
    }

    /// Returns the unchecked advance-distance from `self` to `other`.
    ///
    /// If `other` represents a location that is at or past `self`, the return-value will be the distance between the two locations.
    ///
    /// # Panics
    /// When no valid answer is possible, as under the following cases:
    ///   - `self` is past `other`
    ///   - `other` is a non-whole-byte offset in bits-mode and `self` is in bytes-mode
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

    /// Returns `true` if `self` is in bits-mode.
    pub(crate) fn is_bit_mode(&self) -> bool {
        matches!(self, Self::Bits { .. })
    }

    /// Performs an 'increment by `delta`' operation on `self`, returning a new [`ByteOffset`] value.
    ///
    /// If `self` is in bits-mode, `delta` is treated as a number of bits to advance.
    ///
    /// If `self` is in bytes-mode, `delta` is treated as a number of bytes to advance.
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

    /// Mutates `self` by performing an 'increment by `delta`' operation, returning its original value before the increment.
    ///
    /// Adheres to the same unit semantics as [`Self::increment_by`].
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

    /// Switches `self` from bytes-mode to bits-mode without advancing.
    ///
    /// Returns an error if `self` is already in bits-mode.
    ///
    /// Otherwise, returns `Ok(())`.
    pub(crate) fn enter_bits_mode(&mut self) -> Result<(), ParseError> {
        if let ByteOffset::Bytes(n_bytes) = *self {
            *self = ByteOffset::Bits {
                starting_byte: n_bytes,
                bits_advanced: 0,
            };
            Ok(())
        } else {
            Err(ParseError::InternalError(StateError::Parser(
                ParserStateError::BinaryModeError,
            )))
        }
    }

    /// Converts a bits-mode `self` back into bytes-mode.
    ///
    /// Returns an error of `self` was not in bits-mode.
    ///
    /// If the buffer-location represented by `self` was somewhere in the middle
    /// of a byte, the new value will be the byte-offset of the following byte.
    ///
    /// Otherwise, the new value will be the byte-offset of the current byte.
    ///
    /// When successful, returns the number of bits advanced from the start of bits-mode.
    pub(crate) fn escape_bits_mode(&mut self) -> Result<usize, ParseError> {
        let ByteOffset::Bits {
            starting_byte,
            bits_advanced,
        } = *self
        else {
            return Err(ParseError::InternalError(StateError::Parser(
                ParserStateError::BinaryModeError,
            )));
        };

        let delta_major = bits_advanced / 8;
        let delta_minor = bits_advanced % 8;
        if delta_minor != 0 {
            *self = ByteOffset::Bytes(starting_byte + delta_major + 1);
        } else {
            *self = ByteOffset::Bytes(starting_byte + delta_major);
        }
        Ok(bits_advanced)
    }

    /// Returns the absolute bit offset of the buffer-location represented by `self`.
    pub(crate) fn abs_bit_offset(&self) -> usize {
        match *self {
            ByteOffset::Bytes(n) => n * 8,
            ByteOffset::Bits {
                starting_byte,
                bits_advanced,
            } => starting_byte * 8 + bits_advanced,
        }
    }

    /// Returns the number of bits advanced from the start of bits-mode, or `None` if `self` is in bytes-mode.
    pub(crate) fn bits_advanced(&self) -> Option<usize> {
        match self {
            ByteOffset::Bytes(_n) => None,
            &ByteOffset::Bits { bits_advanced, .. } => Some(bits_advanced),
        }
    }

    /// Normalizes a [`ByteOffset`] into a `(byte_offset, nbits)` tuple.
    ///
    /// If self is in bytes-mode, `nbits` will be `None`.
    ///
    /// If self is in bits-mode, `nbits` will be `Some(k)` where `k` is the number of bits past `byte_offset` that have been read (`k < 8`),
    /// and `byte_offset` is the offset of the byte that `self` is currently reading from.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use doodle::parser::offset::ByteOffset;
    /// assert_eq!(ByteOffset::Bytes(127).as_bytes(), (127, None));
    /// assert_eq!(ByteOffset::Bits { starting_byte: 42, bits_advanced: 3 }.as_bytes(), (42, Some(3)));
    /// assert_eq!(ByteOffset::Bits { starting_byte: 1, bits_advanced: 33 }.as_bytes(), (5, Some(1)));
    /// ```
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
// !SECTION - ByteOffset

/// Control structure that manages a stack of nested [`Lens`]es.
///
/// Used to enforce slice-based limits and restore the correct offset when escaping speculative parse contexts,
/// even when the [`Lens`] containing the endpoint we are enforcing, or checkpoint we are restoring to, is not the
/// topmost [`Lens`] on the stack.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ViewStack {
    stack: Vec<Lens>,
}

impl ViewStack {
    /// Creates a new, empty `ViewStack`.
    pub fn new() -> Self {
        ViewStack { stack: Vec::new() }
    }

    /// Pushes the provided `Lens` onto the top of the stack.
    pub fn push_lens(&mut self, lens: Lens) {
        self.stack.push(lens)
    }

    /// Static helper that determines the most-restrictive upper-bound [`ByteOffset`] implied by a slice of [`Lens`]es
    /// that is implicitly treated as a stack (LIFO order).
    ///
    /// Will produce incorrect answers if the slice is in non-LIFO order.
    fn get_limit_from_slice(slice: &[Lens]) -> Answer<ByteOffset> {
        let Some((lens, rest)) = slice.split_last() else {
            // NOTE - returning `None` is the desired outcome, and `Answer::Blocked` would be misleading, so we return `Answer::Continue` even though we can't actually continue from this point.
            return Answer::Continue;
        };
        // NOTE - because [`Lens::Slice`]s are always required to nest properly, we can safely assume that the first one we find will always be the most-restrictive, so we can short-circuit the search
        lens.get_endpoint()
            .or_else(|| Self::get_limit_from_slice(rest))
    }

    /// Returns the upper-bound [`ByteOffset`] implied by a given `ViewStack`.
    ///
    /// A return-value of `None` means that no artificial limits are being imposed by the `ViewStack`,
    /// and that the true end-of-buffer should be used to determine the upper-bound.
    ///
    /// If the stack is empty, or if it contains no [`Lens::Slice`]s, will return `None`.
    ///
    /// Otherwise, will return the upper-bound of the most-restrictive [`Lens::Slice`] in the stack,
    /// provided that there are no opaque [`Lens::Seek`] above it (in which case `None` will be returned).
    pub fn get_limit(&self) -> Option<ByteOffset> {
        let ret = Self::get_limit_from_slice(self.stack.as_slice());
        ret.to_option()
    }

    /// Internal helper for unstacking as many elements from the top as necessary to reach the first [`Lens::Slice`] from the top.
    ///
    /// Used by [`BufferOffset::close_slice`] to remove the need to recurse, and to avoid discarding the entire stack if
    /// no slices are found.
    ///
    /// This method splits the current stack, keeping only those elements below the topmost slice, and returning
    /// an iterator over the elements that were removed (with the deepest element at the front of the iterator, and the topmost at the end).
    fn unstack_slice_context(&mut self) -> impl DoubleEndedIterator<Item = Lens> + use<'_> {
        let topmost_slice_ix = self
            .stack
            .iter()
            .rposition(|lens| matches!(lens, Lens::Slice { .. }));
        match topmost_slice_ix {
            Some(ix) => self.stack.drain(ix..),
            None => self.stack.drain(0..0),
        }
    }

    /// Pops the topmost element of the stack, returning the updated stack and the popped element.
    ///
    /// If the stack is empty, will return `(self, None)`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use doodle::parser::offset::ViewStack;
    /// // let mut stack = ViewStack::new();
    /// // ...some operations that might push elements onto `stack`...
    /// // let lens: Lens = ...;
    /// let expected = (stack.clone(), Some(lens.clone()));
    /// stack.push_lens(lens);
    /// assert_eq!(stack.pop_lens(), expected);
    /// ```
    pub fn pop_lens(mut self) -> (ViewStack, Option<Lens>) {
        let ret = self.stack.pop();
        (self, ret)
    }

    /// Attempts to restore the return-on-success offset-checkpoint for a speculative parse that has succeeded.
    ///
    /// Pops elements from the top of the stack until a `Lens` with a valid restore-point (see [`Lens::restore`]) is found,
    /// returning the updated [`ViewStack`] along with the unwrapped result of [`Lens::restore`].
    ///
    /// All elements above the first `Lens` whose restore-point was found will be discarded.
    ///
    /// If the stack is exhausted before a valid restore-point is found, returns an error reporting
    /// that no restore-point was found.
    ///
    /// # Note
    ///
    /// In this context, 'restore' is the dual of 'recovery'.
    ///
    /// When a speculative parse succeeds, the offset where it was initiated is 'restored' (i.e. when the inner parse of a 'peek' or 'seek' operation is fully processed).
    ///
    /// When a speculative parse fails, the offset where it was initiated is 'recovered' (i.e. upon hitting a parse-failure within a 'peek-not' or 'union-nondet' operation)
    ///
    /// This convention is adopted at the [`Lens`] and [`Parser`](crate::parser::Parser) layer as well.
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
        Err(StateError::Parser(ParserStateError::NoRestore))
    }

    /// Attempts to recover to the return-on-fail offset-checkpoint for a speculative parse that has failed.
    ///
    /// Pops elements from the top of the stack until a `Lens` with a valid recovery-point (see [`Lens::recover`]) is found,
    /// returning the updated [`ViewStack`] along with the unwrapped result of [`Lens::recover`].
    ///
    /// All elements above the first `Lens` whose recovery-point was found will be discarded.
    ///
    /// If the stackis exhausted before a valid recovery-point is found, returns an error reporting
    /// that no recovery-point was found.
    ///
    /// # Note
    ///
    /// In this context, 'recovery' is the dual of 'restore'.
    ///
    /// When a speculative parse fails, the offset where it was initiated is 'recovered' (i.e. upon hitting a parse-failure within a 'peek-not' or 'union-nondet' operation).
    ///
    /// When a speculative parse succeeds, the offset where it was initiated is 'restored' (i.e. when the inner parse of a 'peek' or 'seek' operation is fully processed).
    ///
    /// This convention is adopted at the [`Lens`] and [`Parser`](crate::parser::Parser) layer as well.
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
        Err(StateError::Parser(ParserStateError::NoRecovery))
    }
}

/// Control structure that manages state for individual parse-operations that change how the buffer is viewed or processed.
///
/// Used to provide support for various kinds of speculative parses, as well as [`Format::Slice`].
///
/// This is the element-type for [`ViewStack`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lens {
    /// Lens generated when processing [`Format::UnionNondet`], to record the original start-offset to re-attempt an alternate branch-parse
    /// from whenever a given branch-parse fails.
    Alts {
        /// The original offset at the time when we began processing [`Format::UnionNondet`]
        ///
        /// This is a 'recovery'-point, in that we return to it if the current parse we are attempting hits a failure at any point.
        checkpoint: ByteOffset,
    },
    /// Lens generated when processing [`Format::Peek`], to record the offset we will continue processing the buffer-data from after the peek succeeds
    Peek {
        /// The original offset at the time when we began processing [`Format::Peek`]
        ///
        /// This is a 'restore'-point, in that we return to it when the current parse we are attempting fully succeeds.
        checkpoint: ByteOffset,
    },
    /// Lens generated when processing [`Format::PeekNot`], to record the offset we will continue processing the buffer-data from after we have confirmed that the inner parse is unsuccessful
    PeekNot {
        /// The original offset at the time when we began processing [`Format::PeekNot`]
        ///
        /// This is a 'recovery'-point, in that we return to it if the current parse we are attempting hits a failure at any point.
        checkpoint: ByteOffset,
    },
    /// Random-access Seek, pushed onto the `ViewStack` whenever a seek is performed.
    ///
    /// `Seek` is an engine-level operation (with no one-to-one mapping to a [`Format`]) that
    /// is conditionally performed by [`BufferOffset::seek_to_offset`]. Otherwise, it is effectively
    /// a [`Peek`] that happens to start reading from somewhere other than the current checkpointed
    /// byte-offset.
    ///
    /// It contains an extra boolean flag-value, `is_transparent`, which determines whether
    /// any enclosing [`Lens::Slice`] further down the `ViewStack` are still effectively
    /// limiting how far into the buffer we are allowed to advance or read.
    Seek {
        /// Control-flag that determines whether this seek is transparent (when true) or opaque (when false).
        ///
        /// A 'transparent' seek is treated as a proximal [`Lens::Peek`], and any [`Lens::Slice`]
        /// enclosing it (i.e. lower down in the stack) is still actively limiting the upper-bound
        /// reported by a [`ViewStack`], and enforced by a [`BufferOffset`]. [`ViewStack::get_limit`]
        /// will ignore any transparent seeks it finds, and continue down the stack until it either
        /// finds a slice or reaches the bottom of the stack.
        ///
        /// An 'opaque' seek is treated as a random-access jump to 'elsewhere' in the
        /// buffer, and is unrestricted by any enclosing [`Lens::Slice`]. Any [`Lens::Slice`]
        /// that is constructed **after** the opaque seek (i.e. higher up in the stack) will
        /// still be active, but the requirement that all [`Lens::Slice`]s in the [`ViewStack`]
        /// must nest into any further down, will be interrupted by the opaque seek, though
        /// each half of the stack (above and below the opque seek) will still independently
        /// enforce their own nesting invariants. More importantly, the reported 'current limit'
        /// of a [`ViewStack`] will defer to the global maximum-buffer-offset if an opaque
        /// seek is encountered before any slices when traversing the stack from top-to-bottom.
        is_transparent: bool,
        checkpoint: ByteOffset,
    },
    Slice {
        endpoint: ByteOffset,
    },
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
    /// If the current Lens directly imposes such a limit (as with [`Lens::Slice`]), returns [`Answer::Found`] with that endpoint.
    ///
    /// If the current Lens does not impose a limit, but its presence should not prevent one from being found further
    /// down the `ViewStack`, returns [`Answer::Continue`] to allow the search to propagate down the stack until one is
    /// found or the stack is exhausted.
    ///
    /// If the current Lens does not impose a limit, but its presence should mask any limit that might otherwise be
    /// found further down the stack (as with an opaque [`Lens::Seek`]), returns [`Answer::Blocked`] to end the search early.
    pub(crate) fn get_endpoint(&self) -> Answer<ByteOffset> {
        match self {
            Lens::Slice { endpoint } => Answer::Found(*endpoint),
            Lens::Seek {
                is_transparent: false,
                ..
            } => Answer::Blocked,
            _ => Answer::Continue,
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

    /// Returns the value of the offset being tracked
    pub(crate) fn get_current_offset(&self) -> ByteOffset {
        self.current_offset
    }

    /// Performs a seek operation, and returns the checkpoint offset if successful, or `Err` if the seek is not allowed.
    ///
    /// # Panics
    ///
    /// Panics if `self` is currently in bit-parsing mode.
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
            log::warn!("edge-case: seek_to_offset called while in bit-parsing mode");
            // NOTE - this panic is a placeholder until we have a case where Seek and bit-mode parsing coincide, to inform the approach that fits this edge-case
            unimplemented!("encountered unhandled edge-case of seek-to-offset in bit-parsing mode");
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
    /// No validation is performed to check whether the `Lens` is well-formed with respect
    /// to the overall buffer, or the nesting-invariant enforced when multiple [`Lens::Slice`]
    /// are on the stack simultaneously.
    ///
    /// If the [`BufferOffset`] recorded in a [`Lens`] (checkpoint or endpoint) is not a valid
    /// offset within the corresponding buffer, various standard parsing or state-manipulation
    /// methods may hit panics or potentially even undefined behavior.
    unsafe fn push_lens(&mut self, lens: Lens) {
        self.view_stack.push_lens(lens);
    }

    /// Pushes an unvalidated `Lens::Slice` to the top of the `ViewStack` with an overall length of `slice_len` (i.e.
    /// whose endpoint is `self.current_offset.increment_by(slice_len)`).
    ///
    /// This method does not check that upper-bound of this slice properly nests within any other slices in the stack,
    /// or even against `self.max_offset` (the true upper-bound of the overall buffer).
    ///
    /// # Note
    ///
    /// In bits-mode, `slice_len` is implicitly assumed to specify a number of bits; in bytes-mode,
    /// it is implicitly assumed to specify a number of bytes.
    pub(super) unsafe fn open_slice_unchecked(&mut self, slice_len: usize) {
        let lens = Lens::Slice {
            endpoint: self.current_offset.increment_by(slice_len),
        };
        unsafe { self.push_lens(lens) }
    }

    /// Escapes the context of a [`Format::Slice`] by skipping to its endpoint, popping the corresponding
    /// [`Lens::Slice`] (and any lenses above it) from the [`ViewStack`].
    ///
    /// Performs a top-down search of the `ViewStack` to find the most recent [`Lens::Slice`],
    /// setting `self.current_offset` to its endpoint. This invariably restores the byte-or-bit modality
    /// inherited from the [`ByteOffset`] at the time the slice was created.
    ///
    /// Any lenses found above that [`Lens::Slice`] are discarded from the `ViewStack` along with it.
    ///
    /// Returns the new value of `current_offset`, which will be the endpoint of the slice that was closed.
    ///
    /// # Errors
    ///
    /// Returns an appropriate `Err` value if either of the conditions below are met:
    ///   - No active slices were found in the `ViewStack`
    ///   - A slice was found, but its endpoint is strictly lower than the current `ByteOffset`
    ///
    /// # Panics
    ///
    /// The lenses discarded above the topmost [`Lens::Slice`] are expected to have already been closed
    /// by the time `close_slice` is called; if any of them still represents an open speculative parse-state
    /// (e.g. [`Lens::Peek`], [`Lens::PeekNot`]) or an opaque [`Lens::Seek`], this method panics instead of
    /// silently discarding it. For example, an unclosed `Lens::Peek` above the topmost slice triggers a
    /// panic, since `close_peek` should always be called to remove it before the enclosing slice is closed.
    ///
    /// Such a state can only arise from mismatched open/close calls elsewhere in `Parser` (or the
    /// `BufferOffset` methods they delegate to) -- never from malformed or unexpected buffer data -- so it
    /// signals a bug in the parsing logic itself rather than a recoverable data-inconsistency. Panicking
    /// here is deliberate: downgrading this to an ordinary `Err` would let it be silently absorbed by the
    /// fallback/`Permit` machinery meant for recoverable data-errors, hiding the underlying bug instead of
    /// surfacing it.
    ///
    /// # Note
    ///
    /// Closing a slice will inherently restore the byte-or-bit modality of the offset at the time
    /// the slice was opened. This means that if a slice was opened in bytes-mode, closing it will
    /// always return to bytes-mode, even if bits-mode was entered within the slice and never explicitly
    /// escaped via [`BufferOffset::escape_bits_mode`]. The same would be true in the converse,
    /// except there is no parsing meta-operation that that enters bytes-mode from within bits-mode.
    pub(crate) fn close_slice(&mut self) -> PResult<ByteOffset> {
        // Extract the iterator that holds the top-most slice on the stack and any elements above it
        let mut topmost = self.view_stack.unstack_slice_context();

        // extract the slice we are closing, which should be the first element in the iterator
        let Some(slice) = topmost.next() else {
            return Err(ParseError::InternalError(StateError::Parser(
                ParserStateError::MissingSlice,
            )));
        };
        let Lens::Slice { endpoint } = slice else {
            unreachable!(
                "first element in iterator returned by unstack_slice_context should always be Lens::Slice"
            );
        };

        // iterate through the elements above the slice, from top to bottom, and ensure that our parsing-state isn't currently invalid
        for top in topmost.rev() {
            match top {
                Lens::Slice { .. } => unreachable!(
                    "only one slice should appear in the iterator returned by unstack_slice_context"
                ),
                Lens::Alts { checkpoint } => {
                    // NOTE - we are handling non-det unions transparently, because we don't ever explicitly escape their state unless we fail a branch and need to try the next one
                    log::info!(
                        "BufferOffset::close_slice: found a non-det union (<-@{checkpoint}) within the slice we are closing (@{endpoint}->) (@{})",
                        self.current_offset
                    );
                    continue;
                }
                Lens::Peek { checkpoint } => {
                    log::error!(
                        "BufferOffset::close_slice: unexpected, unclosed Peek (<-@{checkpoint}) found while closing slice (@{endpoint}->) (@{})",
                        self.current_offset
                    );
                    panic!(
                        "[STATE]: encountered unfinished Peek lens above the slice we are closing (@{})",
                        self.current_offset
                    );
                }
                Lens::PeekNot { checkpoint } => {
                    log::error!(
                        "BufferOffset::close_slice: unexpected, unclosed PeekNot (<-@{checkpoint}) found while closing slice (@{endpoint}->) (@{})",
                        self.current_offset
                    );
                    panic!(
                        "[STATE]: encountered unfinished PeekNot lens above the slice we are closing (@{})",
                        self.current_offset
                    );
                }
                Lens::Seek {
                    is_transparent,
                    checkpoint,
                } => {
                    if is_transparent {
                        log::info!(
                            "BufferOffset::close_slice: found a transparent Seek (<-@{checkpoint}) within the slice we are closing (@{endpoint}->) (@{})",
                            self.current_offset
                        );
                        continue;
                    } else {
                        log::error!(
                            "BufferOffset::close_slice: unexpected, unclosed Seek (<-@{checkpoint}) found while closing slice (@{endpoint}->) (@{})",
                            self.current_offset
                        );
                        panic!(
                            "[STATE]: encountered unfinished Seek lens above the slice we are closing (@{})",
                            self.current_offset
                        );
                    }
                }
            }
        }

        if self.current_offset > endpoint {
            // return the appropriate state-error if we somehow managed to overrun the slice, which should typically never happen
            return Err(ParseError::InternalError(StateError::Parser(
                ParserStateError::SliceOverrun,
            )));
        }
        self.current_offset = endpoint;
        Ok(endpoint)
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

    /// Gracefully closes a speculative parse (generally, [`Lens::Peek`]).
    ///
    /// Internally, this method calls [`ViewStack::restore`] on its held view-stack. Upon success,
    /// `current_offset` will be set to the value returned by [`ViewStack::restore`], namely the
    /// buffer-offset as of the time that the speculative parse was first opened.
    ///
    /// # Errors
    ///
    /// Proppagates any error returned by `ViewStack::restore` back to the caller.
    ///
    /// If an error is returned, no guarantees can be made about the internal state of the `BufferOffset`,
    /// and the caller should not assume that the error-state can be recovered from.
    ///
    /// # Note
    ///
    /// Despite the name `close_peek`, this method is the proper close-method for both [`Lens::Peek`] and [`Lens::Seek`] (the latter of which has
    /// no open- close-method of its own).
    ///
    /// This design ensures that, no matter which internal decision is made by [`Parser::advance_or_seek`](crate::parser::Parser::advance_or_seek),
    /// [`close_peek`] is the correct method to call after the corresponding speculative parse is finished.
    pub(crate) fn close_peek(&mut self) -> Result<(), StateError> {
        let mut stack = ViewStack::new();
        std::mem::swap(&mut stack, &mut self.view_stack);
        let (offs, new_stack) = stack.restore()?;
        self.current_offset = offs;
        self.view_stack = new_stack;
        Ok(())
    }

    /// Performs an [`ViewStack::recover`] operation upon reaching a parse-failure, unwinding the internal ViewStack until a fail-safe (recovery-point) `Lens` is popped.
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
