use std::hash::Hasher as _;

use super::offset::ByteOffset;

pub type PResult<T> = Result<T, ParseError>;

/// Type used to associate an error with a particular source-code location
pub type TraceHash = u64;

pub(crate) fn mk_trace(value: &impl std::hash::Hash) -> TraceHash {
    let mut hasher = std::hash::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

/// General error type for both recoverable and unrecoverable errors encountered during parsing operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParseError {
    /// Explicit `Format::Fail` or any of its derived equivalents
    FailToken(TraceHash),
    /// Validation failure for a Format::Where
    FalsifiedWhere(TraceHash),
    /// For Repeat1, RepeatCount, or RepeatUntil*, indicates that an inadequate number of values were read before encountering end-of-buffer or end-of-slice.
    InsufficientRepeats,
    /// Indicates a successful parse within a negated context, as in the case of PeekNot
    NegatedSuccess,
    /// Used for any logical branch without a handler, such as a refuted Expr::Match or MatchTree descent; u64 value is a trace mechanic for determining which error was triggered
    ExcludedBranch(TraceHash),
    /// Attempted offset-increment would run past the last legal offset of either the overall buffer, or a context-local `Format::Slice`.
    Overrun(OverrunKind),
    /// Attempted random-access seek cannot be performed due to view-based truncation past the destination
    NegativeIndex {
        abs_target: usize,
        abs_buf_start: usize,
    },
    /// A `Format::EndOfInput` token occurring anywhere except the final offset of a Slice or the overall buffer.
    IncompleteParse { bytes_remaining: usize },
    /// Any unrecoverable error in the state of the Parser itself.
    InternalError(StateError),
    /// An operation performed on values derived via parsing is not sound, mostly due to a bad assumption of the format for what is being parsed
    UnsoundOperation(Option<&'static str>, TraceHash),
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
            ParseError::FailToken(trace) => write!(f, "reached Fail token (trace-hash: {trace})"),
            ParseError::UnsoundOperation(Some(mesg), trace) => write!(f, "attempted unsound operation: {mesg} (trace-hash: {trace})"),
            ParseError::UnsoundOperation(None, trace) => write!(f, "attempted unsound operation (trace-hash: {trace})"),
            ParseError::FalsifiedWhere(trace) => write!(f, "parsed value deemed invalid by Where lambda (trace-hash: {trace})"),
            ParseError::InsufficientRepeats => write!(
                f,
                "failed to find enough format repeats to satisfy requirement"
            ),
            ParseError::ExcludedBranch(trace) => write!(f, "buffer contents does not correspond to an expected branch of a MatchTree or Expr::Match (trace-hash: {trace})"),
            ParseError::NegatedSuccess => write!(f, "sub-parse succeeded in negated context"),
            ParseError::NegativeIndex { abs_target, abs_buf_start } => write!(f, "attempted to seek to negative index (target: {abs_target}, buffer-start: {abs_buf_start})"),
            ParseError::IncompleteParse { bytes_remaining: n } => write!(
                f,
                "incomplete parse: expected end-of-stream, but {n} bytes remain unconsumed"
            ),
            ParseError::Overrun(k) => match k {
                OverrunKind::EndOfStream { offset, max_offset }=> write!(f, "attempted offset-advance to {offset} would overrun end of stream[max-offset: {max_offset}]"),
                OverrunKind::EndOfSlice { offset, max_offset } => write!(f, "attempted offset-advance to {offset} would overrun end of slice[max-offset: {max_offset}]"),
            },
            ParseError::InternalError(e) => write!(f, "unrecoverable internal error: {e}")
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StateError {
    /// Failed attempt to return to a fail-safe 'recovery-point', such as the starting offset of a `PeekNot` or `UnionNondet`.
    NoRecovery,
    /// Failed attempt to open a slice whose final offset overruns either an existing slice, or the buffer itself
    UnstackableSlices {
        current_offset: ByteOffset,
        current_limit: ByteOffset,
        new_slice_end: ByteOffset,
    },
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
            StateError::UnstackableSlices {
                current_offset,
                current_limit,
                new_slice_end,
            } => write!(
                f,
                "unable to open slice due to limit-violation: to-be-constructed slice endpoint {new_slice_end} exceeds existing limit (slice or stream) of {current_limit} (current offset: {current_offset})",
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
