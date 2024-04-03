use super::error::{ParseError, StateError};
use anyhow::{anyhow, Result as AResult};
use std::cmp::Ordering;

type PResult<T> = Result<T, ParseError>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum ByteOffset {
    Bytes(usize),
    Bits { starting_byte: usize, bits_advanced: usize },
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

    pub(crate) const fn byte_plus_bits(starting_byte: usize, nbits: usize) -> Self {
        Self::Bits { starting_byte, bits_advanced: nbits }
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
        matches!(self, Self::Bits {..})
    }

    pub(crate) fn increment_assign(&mut self) -> Self {
        self.increment_assign_by(1)
    }

    pub(crate) fn increment_by(&self, delta: usize) -> Self {
        match self {
            &ByteOffset::Bytes(n_bytes) => Self::Bytes(n_bytes + delta),
            &ByteOffset::Bits {starting_byte, bits_advanced } => Self::Bits { starting_byte, bits_advanced: bits_advanced + delta },
        }
    }

    pub(crate) fn increment_assign_by(&mut self, delta: usize) -> Self {
        let ret = *self;
        match self {
            ByteOffset::Bytes(n_bytes) => {
                *n_bytes += delta;
            }
            ByteOffset::Bits { bits_advanced: n_bits, .. } => {
                *n_bits += delta;
            }
        }
        ret
    }

    pub(crate) fn enter_bits_mode(&mut self) -> Result<(), ParseError> {
        if let ByteOffset::Bytes(n_bytes) = *self {
            *self = ByteOffset::Bits { starting_byte: n_bytes, bits_advanced: 0 };
            Ok(())
        } else {
            Err(ParseError::InternalError(StateError::BinaryModeError))
        }
    }

    pub(crate) fn escape_bits_mode(&mut self) -> Result<usize, ParseError> {
        if let ByteOffset::Bits { starting_byte, bits_advanced } = *self {
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
        match self {
            &ByteOffset::Bytes(n) => n * 8,
            &ByteOffset::Bits { starting_byte, bits_advanced } => starting_byte * 8 + bits_advanced,
        }
    }

    pub(crate) fn bits_advanced(&self) -> Option<usize> {
        match self {
            ByteOffset::Bytes(_n) => None,
            &ByteOffset::Bits { bits_advanced, .. } => Some(bits_advanced),
        }
    }

    pub(crate) fn as_bytes(&self) -> (usize, Option<usize>) {
        match self {
            &ByteOffset::Bytes(n) => (n, None),
            &ByteOffset::Bits { starting_byte, bits_advanced } => {
                let delta_major = bits_advanced / 8;
                let delta_minor = bits_advanced % 8;
                (starting_byte + delta_major, Some(delta_minor))
            }
        }
    }
}

impl PartialOrd for ByteOffset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let partial = self.abs_bit_offset().cmp(&other.abs_bit_offset());
        match partial {
            // yield None instead of Some(Equal) if same bit-level offset but different modes
            Ordering::Equal if self.is_bit_mode() ^ other.is_bit_mode() => None,
            _ => Some(partial),
        }
    }
}

pub(crate) struct BufferOffset {
    current_offset: ByteOffset, // the 'true' value of the offset, reinterpreted according to the other fields
    view_stack: ViewStack,
    max_offset: ByteOffset,
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ViewStack {
    stack: Vec<Lens>,
}

impl ViewStack {
    pub(crate) fn new() -> Self {
        ViewStack { stack: Vec::new() }
    }

    pub(crate) fn push_lens(&mut self, lens: Lens) {
        self.stack.push(lens)
    }

    fn get_limit_from_slice(slice: &[Lens]) -> Option<ByteOffset> {
        let (lens, rest) = slice.split_last()?;
        match lens.get_endpoint() {
            None => Self::get_limit_from_slice(rest),
            // FIXME - this could be avoided by amortizing comparison-cost into the push-lens method, but that involves more complex work.
            // FIXME - consider memoizing this value until the stack is manipulated enough it might change?
            Some(end) => {
                Self::get_limit_from_slice(rest).map(|lim| if end > lim { lim } else { end })
            }
        }
    }

    pub(crate) fn get_limit(&self) -> Option<ByteOffset> {
        let ret = Self::get_limit_from_slice(self.stack.as_slice());
        // FIXME - introduce caching mechanic
        ret
    }

    pub(crate) fn escape(mut self) -> (ViewStack, Option<Lens>) {
        let ret = self.stack.pop();
        (self, ret)
    }

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) enum Lens {
    PeekNot { checkpoint: ByteOffset },
    Peek { checkpoint: ByteOffset },
    Slice { endpoint: ByteOffset },
    Alts { checkpoint: ByteOffset },
}

impl Lens {
    /// Returns the proper ByteOffset to 'recover' from parse-failure that occurred within the current Lens.
    ///
    /// If the current context does not have implicit allowance for failure, returns `None` instead, to allow
    /// recovery to propagate down the ViewStack to outer contexts that may permit recovery.
    pub(crate) fn recover(&self) -> Option<ByteOffset> {
        match self {
            Lens::Alts { checkpoint } | Lens::PeekNot { checkpoint } => Some(*checkpoint),
            Lens::Peek { .. } | Lens::Slice { .. } => None,
        }
    }

    /// Returns the proper ByteOffset to 'restore' after a successful, non-consuming parse operation based on the current Lens.
    ///
    /// If there is no appropriate value to return, `None` is returned instead, to allow restoration to propagate down the ViewStack
    /// to outer contexts that may permit restoration.
    pub(crate) fn restore(&self) -> Option<ByteOffset> {
        match self {
            Lens::Peek { checkpoint } => Some(*checkpoint),
            Lens::Slice { .. } => None,
            // NOTE - despite having starting-offsets, Alts and PeekNot are return-on-fail rather than return-on-success
            Lens::Alts { .. } | Lens::PeekNot { .. } => None,
        }
    }

    /// Returns the local upper-bound beyond which parsing is not permissible in the current Lens.
    ///
    /// If the current Lens does not enforce such a limit, returns `None` instead, to allow back-propagation down the ViewStack
    /// until one is found or the ViewStack is exhausted.
    pub(crate) fn get_endpoint(&self) -> Option<ByteOffset> {
        match self {
            Lens::Slice { endpoint } => Some(*endpoint),
            _ => None,
        }
    }
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

    pub(crate) fn get_current_offset(&self) -> ByteOffset {
        self.current_offset
    }

    pub(crate) fn can_increment(&self, delta: usize) -> bool {
        let lim = self.current_limit();
        let after_increment = self.current_offset.increment_by(delta);
        !(after_increment > lim)
    }

    /// Increments the current offset by `delta` if it is legal to do so, and returns the old offset.
    ///
    /// Instead returns `Err` if it is not legal to increment by `delta`.
    ///
    /// # Note
    ///
    /// The implicit unit of `delta` is whichever of 'bits' or 'bytes' is currently being processed.
    /// In most cases this will be bytes, but within a `Format::Bits` context, delta will measure
    /// bits within each byte.
    pub(crate) fn try_increment(&mut self, delta: usize) -> PResult<ByteOffset> {
        let lim = self.current_limit();
        let after_increment = self.current_offset.increment_by(delta);
        if !(after_increment > lim) {
            Ok(self.current_offset.increment_assign_by(delta))
        } else {
            if self.current_limit() < self.max_offset {
                Err(ParseError::Overrun(super::error::OverrunKind::EndOfSlice))
            } else {
                Err(ParseError::Overrun(super::error::OverrunKind::EndOfStream))
            }
        }
    }

    pub(crate) fn enter_bits_mode(&mut self) -> PResult<()> {
        self.current_offset.enter_bits_mode()
    }

    /// Escapes bits mode and returns the number of bits read since entering bits mode
    pub(crate) fn escape_bits_mode(&mut self) -> PResult<usize> {
        self.current_offset.escape_bits_mode()
    }

    pub(crate) unsafe fn push_lens(&mut self, lens: Lens) {
        self.view_stack.push_lens(lens);
    }

    pub(crate) unsafe fn open_slice_unchecked(&mut self, slice_len: usize) {
        self.push_lens(Lens::Slice {
            endpoint: self.current_offset.increment_by(slice_len),
        })
    }

    pub(crate) fn close_slice(&mut self) -> AResult<ByteOffset> {
        let mut stack = ViewStack::new();
        std::mem::swap(&mut stack, &mut self.view_stack);
        match stack.escape() {
            (stack, Some(Lens::Slice { endpoint })) => {
                if self.current_offset > endpoint {
                    return Err(anyhow!(
                        "Current offset exceeds limit being escaped: {} > {}",
                        self.current_offset,
                        endpoint
                    ));
                }
                self.current_offset = endpoint;
                Ok(endpoint)
            }
            (_, other) => Err(anyhow!(
                "ViewStack expected to have a Slice on top, found {:?} instead",
                other
            )),
        }
    }

    pub(crate) fn open_peek(&mut self) {
        let checkpoint = self.current_offset;
        let peek = Lens::Peek { checkpoint };
        self.view_stack.push_lens(peek);
    }

    pub(crate) fn open_peeknot(&mut self) {
        let checkpoint = self.current_offset;
        let peeknot = Lens::PeekNot { checkpoint };
        self.view_stack.push_lens(peeknot);
    }

    pub(crate) fn open_parallel(&mut self) {
        let checkpoint = self.current_offset;
        let parallel = Lens::Alts { checkpoint };
        self.view_stack.push_lens(parallel);
    }

    pub(crate) fn close_peek(&mut self) -> Result<(), StateError> {
        let mut stack = ViewStack::new();
        std::mem::swap(&mut stack, &mut self.view_stack);
        let (offs, new_stack) = stack.restore()?;
        self.current_offset = offs;
        self.view_stack = new_stack;
        Ok(())
    }

    /// 'Recovers' from a failed parse by unwinding the internal ViewStack until a fail-safe lens is popped.
    ///
    /// If the ViewStack is empty, or is exhausted before such a Lens is found, will return `Err` with the appropriate
    /// `StateError` value
    pub(crate) fn recover(&mut self) -> Result<(), StateError> {
        let mut stack = ViewStack::new();
        std::mem::swap(&mut stack, &mut self.view_stack);
        let (offs, new_stack) = stack.recover()?;
        self.current_offset = offs;
        self.view_stack = new_stack;
        Ok(())
    }

    pub(crate) fn current_limit(&self) -> ByteOffset {
        self.view_stack.get_limit().unwrap_or(self.max_offset)
    }

    /// Number of bytes that can be consumed until reaching a limit,
    /// based on the current ByteOffset mode (i.e. can become stale if the mode changes)
    pub(crate) fn rem_local(&self) -> usize {
        self.current_offset.delta(self.current_limit())
    }

    /// Number of bytes that can be consumed in the buffer overall, based on the current ByteOffset mode (i.e. can become stale if the mode changes)
    pub(crate) fn rem_absolute(&self) -> usize {
        self.current_offset.delta(self.max_offset)
    }
}
