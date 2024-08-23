use super::offset::ByteOffset;

pub type PResult<T> = Result<T, ParseError>;

/// General error type for both recoverable and unrecoverable errors encountered during parsing operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParseError {
    /// Explicit `Format::Fail` or any of its derived equivalents
    FailToken,
    /// Validation failure for a Format::Where
    FalsifiedWhere,
    /// For Repeat1, RepeatCount, or RepeatUntil*, indicates that an inadequate number of values were read before encountering end-of-buffer or end-of-slice.
    InsufficientRepeats,
    /// Indicates a successful parse within a negated context, as in the case of PeekNot
    NegatedSuccess,
    /// Used for any logical branch without a handler, such as a refuted Expr::Match or MatchTree descent; u64 value is a trace mechanic for determining which error was triggered
    ExcludedBranch(u64),
    /// Attempted offset-increment would run past the last legal offset of either the overall buffer, or a context-local `Format::Slice`.
    Overrun(OverrunKind),
    /// A `Format::EndOfInput` token occurring anywhere except the final offset of a Slice or the overall buffer.
    IncompleteParse { bytes_remaining: usize },
    /// Any unrecoverable error in the state of the Parser itself.
    InternalError(StateError),
    /// An operation performed on values derived via parsing is not sound, mostly due to a bad assumption of the format for what is being parsed
    UnsoundOperation(Option<&'static str>),
}

/// Error-kind indicator that distinguishes between different Overrun errors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverrunKind {
    /// Indicates that an overrun error occurred due to the absolute boundary of the full parse-buffer
    EndOfStream {
        offset: ByteOffset,
        max_offset: ByteOffset,
    },
    /// Indicates that an overrun error occurred due to the relative boundary of a context-local slice
    EndOfSlice {
        offset: ByteOffset,
        max_offset: ByteOffset,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::FailToken => write!(f, "reached Fail token"),
            ParseError::UnsoundOperation(Some(mesg)) => write!(f, "attempted unsound operation: {mesg}"),
            ParseError::UnsoundOperation(None) => write!(f, "attempted unsound operation"),
            ParseError::FalsifiedWhere => write!(f, "parsed value deemed invalid by Where lambda"),
            ParseError::InsufficientRepeats => write!(
                f,
                "failed to find enough format repeats to satisfy requirement"
            ),
            ParseError::ExcludedBranch(trace) => write!(f, "buffer contents does not correspond to an expected branch of a MatchTree or Expr::Match (trace-hash: {trace})"),
            ParseError::NegatedSuccess => write!(f, "sub-parse succeeded in negated context"),
            ParseError::IncompleteParse { bytes_remaining: n } => write!(
                f,
                "incomplete parse: expected end-of-stream, but {n} bytes remain unconsumed"
            ),
            ParseError::Overrun(k) => match k {
                OverrunKind::EndOfStream { offset, max_offset }=> write!(f, "attempted offset-advance to {offset} would overrun end of stream[max-offset: {max_offset}]"),
                OverrunKind::EndOfSlice { offset, max_offset } => write!(f, "attempted offset-advance to {offset} would overrun end of slice[max-offset: {max_offset}]"),
            },
            ParseError::InternalError(e) => write!(f, "unrecoverable internal error: {}", e)
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::InternalError(e) => Some(e),
            _ => None,
        }
    }
}

/// Sub-class of errors that only occur when an illegal operation is attempted,
/// due to incoherent usage or improperly nesting of various state-manipulation methods
/// within the [`BufferOffset`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StateError {
    /// Failed attempt to return to a fail-safe 'recovery-point', such as the starting offset of a `PeekNot` or `UnionNondet`.
    NoRecovery,
    /// Failed attempt to open a slice whose final offset overruns either an existing slice, or the buffer itself
    UnstackableSlices,
    /// Failed attempt to return to a neutral 'restoration-point', such as the starting offset of a `Peek` or `WithRelativeOffset`
    NoRestore,
    /// Attempt to enter bits-mode while already in bits-mode, or escape bits-mode while not in bits-mode
    BinaryModeError,
    /// Slice-close operation failed because there was no slice to close
    MissingSlice,
    /// The current offset somehow exceeded the limit of an extant slice
    SliceOverrun,
    /// Reading one byte is not possible because we are outside of the buffer bounds
    OutOfBoundsRead,
    /// No corresponding path for decoding a prefix code according to a constructed HuffmanNode-tree.
    HuffmanDescentError(crate::prelude::huffman::DescentError),
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::NoRecovery => write!(f, "unable to recover from parse failure"),
            StateError::NoRestore => write!(f, "unable to restore to a parsing checkpoint"),
            StateError::UnstackableSlices => write!(
                f,
                "unable to open slice that violates existing slice (or stream) boundary"
            ),
            StateError::BinaryModeError => write!(f, "illegal binary-mode switch operation"),
            StateError::MissingSlice => write!(f, "missing slice cannot be closed"),
            StateError::SliceOverrun => {
                write!(
                    f,
                    "cannot close slice properly, as it has already been overrun"
                )
            }
            StateError::OutOfBoundsRead => {
                write!(f, "unguarded illegal read operation (out of bounds)")
            }
            StateError::HuffmanDescentError(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for StateError {}

impl From<StateError> for ParseError {
    fn from(value: StateError) -> Self {
        ParseError::InternalError(value)
    }
}
