use crate::byte_set::ByteSet;
use crate::decoder::{Scope, ScopeEntry, Value};
use crate::loc_decoder::{LocScope, ParsedValue};
use crate::read::{BufferKind, ReadCtxt};
use crate::{Expr, Label, Pattern};

// FIXME - LocDecoder has yet to be re-implemented to use `DecodeError` instead of `DecodeErrorKind`, but until we do that, we need to keep the `DecodeErrorKind` type around for the time being.

#[derive(Debug)]
pub struct UnknownVarError(pub(crate) Label);

impl std::fmt::Display for UnknownVarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "reference to unknown variable: `{}`", self.0)
    }
}

impl std::error::Error for UnknownVarError {}

#[derive(Debug)]
pub struct DecodeError<V: Clone = Value> {
    pub err: Box<DecodeErrorKind<V>>,
    pub _trace: Vec<Box<dyn std::fmt::Debug + Sync + Send + 'static>>,
}

impl<V: Clone + std::fmt::Debug> crate::util::ErrTrace for DecodeError<V> {
    fn with_trace<T>(mut self, ctxt: T) -> Self
    where
        T: std::fmt::Debug + Sync + Send + 'static,
    {
        self._trace.push(Box::new(ctxt));
        self
    }
}

impl<V: Clone + std::fmt::Debug> From<BufferLimitError> for DecodeError<V> {
    fn from(err: BufferLimitError) -> Self {
        DecodeError {
            err: Box::new(err.into()),
            _trace: Vec::new(),
        }
    }
}

impl<V: Clone + std::fmt::Debug> From<DecodeErrorKind<V>> for DecodeError<V> {
    fn from(err: DecodeErrorKind<V>) -> Self {
        DecodeError {
            err: Box::new(err),
            _trace: Vec::new(),
        }
    }
}

impl<V: Clone + std::fmt::Debug> std::fmt::Display for DecodeError<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self._trace.is_empty() {
            write!(f, "{}", self.err)
        } else {
            writeln!(f, "{} (", self.err)?;
            for item in self._trace.iter() {
                writeln!(f, "\t{item:?}")?;
            }
            write!(f, ")")
        }
    }
}

impl<V: Clone + std::fmt::Debug + 'static> std::error::Error for DecodeError<V> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.err)
    }
}

#[derive(Debug)]
/// Sub-class of errors that are specifically related to violating the bounds of a buffer,
/// and which do not store or depend on the type used to represent the value-layer.
pub enum BufferLimitError {
    /// Attempted to perform an atomic multi-byte read-operation or otherwise advance the buffer-offset by `n` bytes would run past the end of the available data.
    Overrun {
        /// What kind of buffer was being read when the overrun occurred (input, slice, view, or value)
        buffer_kind: BufferKind,
        /// How many bytes the decoder attempted to read as a single operation, or otherwise advance the buffer-offset by
        nbytes: usize,
        /// Immediate offset in the buffer from which the advance was attempted, and subsequently failed
        offset: usize,
    },
    /// Attempted to read a byte at an offset that is past the end of the buffer-view (whether slice, the entire buffer itself, or a computed value-buffer)
    Overbyte {
        /// What kind of buffer was being read when the overbyte occurred (input, slice, view, or value)
        buffer_kind: BufferKind,
        /// Immediate offset in the buffer from which the read was attempted, and subsequently failed
        offset: usize,
    },
    /// Attempted to seek to a buffer-offset that is past the end of the buffer-view (whether slice, the entire buffer itself, or a computed value-buffer)
    SeekPastEnd {
        /// What kind of buffer was being read when the seek-past-end occurred (input, slice, view, or value)
        buffer_kind: BufferKind,
        /// The offset that the decoder attempted to seek to, which would have overrun the buffer
        seek_offset: usize,
        /// The length of the buffer we were seeking within (i.e. one more than the last legal offset)
        buffer_len: usize,
    },
}

impl std::fmt::Display for BufferLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overrun {
                buffer_kind,
                nbytes,
                offset,
            } => {
                write!(
                    f,
                    "attempt to split {nbytes} bytes ahead at offset {offset} would overrun {buffer_kind}"
                )
            }
            Self::SeekPastEnd {
                buffer_kind,
                seek_offset,
                buffer_len,
            } => {
                write!(
                    f,
                    "attempt to seek to buffer offset {seek_offset} would overrun buffer ({buffer_kind} had length {buffer_len})"
                )
            }
            Self::Overbyte {
                buffer_kind,
                offset,
            } => {
                write!(
                    f,
                    "attempted to read byte at offset {offset} in {buffer_kind}, but encountered {terminus}",
                    terminus = buffer_kind.terminus()
                )
            }
        }
    }
}

impl std::error::Error for BufferLimitError {}

#[derive(Debug)]
pub enum DecodeErrorKind<V: Clone = Value> {
    /// Explicit [`Format::Fail`] token encountered, or successful parse in a `PeekNot` context.
    Fail {
        bindings: Vec<(Label, ScopeEntry<V>)>,
        offset: usize,
        buffer: Vec<u8>,
    },
    /// `Format::Where` assertion failed on a parsed value
    BadWhere {
        /// The set of in-scope locals and their values at the time of the error
        bindings: Vec<(Label, ScopeEntry<V>)>,
        /// Predicate lambda that the parsed value failed to satisfy
        assertion: Box<Expr>,
        /// The parsed value that failed to satisfy the assertion
        exception: Box<V>,
    },
    /// Input contains at least one trailing byte beyond the scope of what the Decoder processed before being done.
    Trailing {
        buffer_kind: BufferKind,
        /// First byte read after the end of the parse
        byte: u8,
        /// Buffer-offset of the byte in question
        offset: usize,
    },
    BufferLimit(BufferLimitError),
    /// [`Decoder::Byte`](crate::decoder::Decoder::Byte) encountered a byte that was not a member of the expected set.
    Unexpected {
        /// The byte that was read, but not expected
        found: u8,
        /// The set of bytes accepted by the decoder at this point in the parse
        expected: ByteSet,
        /// The offset in the buffer at which the unexpected byte was read
        offset: usize,
    },
    /// Encountered byte-sequence that is not accepted by any of the valid MatchTree branches
    NoValidBranch {
        /// Offset in the buffer at which the invalid branch was encountered
        offset: usize,
    },
    /// Expression evaluated to a value that did not match any of the provided patterns in a [`Decoder::Match`](crate::decoder::Decoder::Match).
    RefutedPatternMatch {
        /// List of patterns that the value was matched against
        cases: Vec<Pattern>,
        /// The value that failed to match any of the provided patterns
        value: Box<V>,
    },
}

impl BufferLimitError {
    pub fn with_trace<V, T>(self, trace: T) -> DecodeError<V>
    where
        T: std::fmt::Debug + Sync + Send + 'static,
        V: Clone,
    {
        DecodeErrorKind::<V>::from(self).with_trace(trace)
    }
}

impl<V: Clone> DecodeErrorKind<V> {
    pub fn with_trace<T>(self, trace: T) -> DecodeError<V>
    where
        T: std::fmt::Debug + Sync + Send + 'static,
    {
        DecodeError {
            err: Box::new(self),
            _trace: vec![Box::new(trace)],
        }
    }
}

impl<V: std::fmt::Debug + Clone> std::fmt::Display for DecodeErrorKind<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fail {
                bindings,
                buffer,
                offset,
            } => {
                write!(
                    f,
                    "decode failure at Offset={offset}, Buffer={buffer:#?} (Scope: {bindings:?})"
                )
            }
            Self::BadWhere {
                bindings,
                assertion,
                exception,
            } => {
                write!(
                    f,
                    "invalidated Format::Where: assertion `{assertion:?}` does not hold for observed value `{exception:?} (Scope: {bindings:?})`"
                )
            }
            Self::Trailing {
                buffer_kind,
                byte,
                offset,
            } => {
                write!(
                    f,
                    "byte `{byte:02x}` found when {terminus} expected (offset = {offset})",
                    terminus = buffer_kind.terminus()
                )
            }
            Self::BufferLimit(err) => err.fmt(f),
            Self::Unexpected {
                found,
                expected,
                offset,
            } => {
                write!(
                    f,
                    "byte `{found:02x}` at offset {offset} not member of expected set {expected:?}"
                )
            }
            Self::NoValidBranch { offset } => {
                write!(
                    f,
                    "no valid branch found for content starting at offset {offset}"
                )
            }
            Self::RefutedPatternMatch { value, cases } => {
                write!(
                    f,
                    "value `{value:?}` failed to match any of the provided patterns: {cases:?}"
                )
            }
        }
    }
}

impl<V: std::fmt::Debug + Clone> std::error::Error for DecodeErrorKind<V> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BufferLimit(err) => Some(err),
            _ => None,
        }
    }
}

impl<V: Clone> From<BufferLimitError> for DecodeErrorKind<V> {
    fn from(err: BufferLimitError) -> Self {
        Self::BufferLimit(err)
    }
}

impl DecodeErrorKind<Value> {
    pub fn fail(scope: &Scope<'_>, input: ReadCtxt<'_>) -> DecodeErrorKind<Value> {
        let mut bindings = Vec::new();
        scope.get_bindings(&mut bindings);
        let offset = input.offset;
        let buffer = input.input.to_owned();
        DecodeErrorKind::Fail {
            bindings,
            offset,
            buffer,
        }
    }

    pub fn bad_where(
        scope: &Scope<'_>,
        assertion: Box<Expr>,
        exception: Box<Value>,
    ) -> DecodeErrorKind<Value> {
        let mut bindings = Vec::new();
        scope.get_bindings(&mut bindings);
        DecodeErrorKind::BadWhere {
            bindings,
            assertion,
            exception,
        }
    }
}

impl DecodeErrorKind<ParsedValue> {
    pub fn loc_fail(scope: &LocScope<'_>, input: ReadCtxt<'_>) -> DecodeErrorKind<ParsedValue> {
        let mut bindings = Vec::new();
        scope.get_bindings(&mut bindings);
        let buffer = input.input.to_owned();
        let offset = input.offset;
        DecodeErrorKind::Fail {
            bindings,
            buffer,
            offset,
        }
    }

    pub fn loc_bad_where(
        scope: &LocScope<'_>,
        assertion: Box<Expr>,
        exception: Box<ParsedValue>,
    ) -> DecodeErrorKind<ParsedValue> {
        let mut bindings = Vec::new();
        scope.get_bindings(&mut bindings);
        DecodeErrorKind::BadWhere {
            bindings,
            assertion,
            exception,
        }
    }
}

impl<V: Clone> DecodeErrorKind<V> {
    pub fn unexpected(found: u8, expected: ByteSet, offset: usize) -> Self {
        Self::Unexpected {
            found,
            expected,
            offset,
        }
    }
}

impl BufferKind {
    /// Returns a string that describes the "terminus" of the buffer, i.e. what kind of end-of-buffer condition was encountered when an overrun or overbyte error occurred.
    pub fn terminus(&self) -> &'static str {
        match self {
            Self::Input => "end-of-stream",
            Self::Slice => "end-of-slice",
            Self::View => "end-of-input",
            Self::Value => "end-of-array",
        }
    }

    pub fn trailing<V: Clone>(self, byte: u8, offset: usize) -> DecodeErrorKind<V> {
        DecodeErrorKind::Trailing {
            buffer_kind: self,
            byte,
            offset,
        }
    }

    pub fn bad_seek(self, seek_offset: usize, buffer_len: usize) -> BufferLimitError {
        BufferLimitError::SeekPastEnd {
            buffer_kind: self,
            seek_offset,
            buffer_len,
        }
    }

    /// Constructs a DecodeError that indicates that a split attempt (of `nbytes` bytes) would overrun the buffer based
    /// on the immediate offset (`offset`)
    pub fn overrun(self, nbytes: usize, offset: usize) -> BufferLimitError {
        BufferLimitError::Overrun {
            buffer_kind: self,
            nbytes,
            offset,
        }
    }

    /// Constructs a DecodeError that indicates that a (one-byte) read attempt would overrun the buffer based on the immediate offset (`offset`)
    pub fn overbyte(self, offset: usize) -> BufferLimitError {
        BufferLimitError::Overbyte {
            buffer_kind: self,
            offset,
        }
    }
}
