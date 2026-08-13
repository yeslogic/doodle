use std::hash::Hasher as _;

use crate::util::ErrTrace;

use super::offset::ByteOffset;

pub type PResult<T> = Result<T, ParseError>;

/// Type used to associate an error with a particular source-code location
pub type TraceHash = u64;

pub(crate) fn mk_trace(value: &impl std::hash::Hash) -> TraceHash {
    let mut hasher = std::hash::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug)]
pub struct CtxtParseError {
    pub err: ParseError,
    pub _trace: Vec<Box<dyn std::fmt::Debug + Sync + Send + 'static>>,
}

impl From<ParseError> for CtxtParseError {
    fn from(err: ParseError) -> Self {
        CtxtParseError {
            err,
            _trace: Vec::new(),
        }
    }
}

impl ErrTrace for CtxtParseError {
    fn with_trace<T>(mut self, ctxt: T) -> Self
    where
        T: std::fmt::Debug + Sync + Send + 'static,
    {
        self._trace.push(Box::new(ctxt));
        self
    }
}

impl std::fmt::Display for CtxtParseError {
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

impl std::error::Error for CtxtParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.err)
    }
}

/// General error type for both recoverable and unrecoverable errors encountered during parsing operations
#[derive(Clone, Debug)]
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
    /// A numeric evaluation error
    BadEval(crate::numeric::eval::EvalError),
}

impl From<crate::numeric::eval::EvalError> for ParseError {
    fn from(err: crate::numeric::eval::EvalError) -> Self {
        Self::BadEval(err)
    }
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
            ParseError::UnsoundOperation(Some(mesg), trace) => write!(
                f,
                "attempted unsound operation: {mesg} (trace-hash: {trace})"
            ),
            ParseError::UnsoundOperation(None, trace) => {
                write!(f, "attempted unsound operation (trace-hash: {trace})")
            }
            ParseError::FalsifiedWhere(trace) => write!(
                f,
                "parsed value deemed invalid by Where lambda (trace-hash: {trace})"
            ),
            ParseError::InsufficientRepeats => write!(
                f,
                "failed to find enough format repeats to satisfy requirement"
            ),
            ParseError::ExcludedBranch(trace) => write!(
                f,
                "buffer contents does not correspond to an expected branch of a MatchTree or Expr::Match (trace-hash: {trace})"
            ),
            ParseError::NegatedSuccess => write!(f, "sub-parse succeeded in negated context"),
            ParseError::NegativeIndex {
                abs_target,
                abs_buf_start,
            } => write!(
                f,
                "attempted to seek to negative index (target: {abs_target}, buffer-start: {abs_buf_start})"
            ),
            ParseError::IncompleteParse { bytes_remaining: n } => write!(
                f,
                "incomplete parse: expected end-of-stream, but {n} bytes remain unconsumed"
            ),
            ParseError::Overrun(k) => match k {
                OverrunKind::EndOfStream { offset, max_offset } => write!(
                    f,
                    "attempted offset-advance to {offset} would overrun end of stream[max-offset: {max_offset}]"
                ),
                OverrunKind::EndOfSlice { offset, max_offset } => write!(
                    f,
                    "attempted offset-advance to {offset} would overrun end of slice[max-offset: {max_offset}]"
                ),
            },
            ParseError::InternalError(e) => write!(f, "unrecoverable internal error: {e}"),
            ParseError::BadEval(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::InternalError(e) => Some(e),
            ParseError::BadEval(e) => Some(e),
            _ => None,
        }
    }
}

/// Sub-class of [`StateError`] for errors that arise purely from incoherent usage or improper
/// nesting/bracketing of the various state-manipulation methods on [`BufferOffset`].
///
/// These indicate a bug in the parsing logic itself (or in `doodle`'s code-generation), and can
/// never be triggered by a well-formed decoder processing any buffer, however malformed or
/// adversarial its contents.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParserStateError {
    /// Failed attempt to return to a fail-safe 'recovery-point', such as the starting offset of a `PeekNot` or `UnionNondet`.
    NoRecovery,
    /// Failed attempt to return to a neutral 'restoration-point', such as the starting offset of a `Peek` or `WithRelativeOffset`
    NoRestore,
    /// Attempt to enter bits-mode while already in bits-mode, or escape bits-mode while not in bits-mode
    BinaryModeError,
    /// Slice-close operation failed because there was no slice to close
    MissingSlice,
    /// The current offset somehow exceeded the limit of an extant slice
    SliceOverrun,
    /// Attempted to read a byte at an offset that is not just past the end of the buffer, but
    /// genuinely outside of it, which should be precluded by proactive enforcement elsewhere.
    IllegalOffsetRead,
    /// During an operation that closes a slice, an unclosed Lens was found within the slice-context we would close
    UnfinishedLensAboveSlice(super::offset::Lens),
}

impl std::fmt::Display for ParserStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserStateError::NoRecovery => write!(f, "unable to recover from parse failure"),
            ParserStateError::NoRestore => write!(f, "unable to restore to a parsing checkpoint"),
            ParserStateError::BinaryModeError => {
                write!(f, "illegal binary-mode switch operation")
            }
            ParserStateError::MissingSlice => write!(f, "missing slice cannot be closed"),
            ParserStateError::SliceOverrun => {
                write!(
                    f,
                    "cannot close slice properly, as it has already been overrun"
                )
            }
            ParserStateError::IllegalOffsetRead => {
                write!(f, "attempted read at an offset outside of the buffer")
            }
            ParserStateError::UnfinishedLensAboveSlice(lens) => {
                write!(
                    f,
                    "attempted slice-close operation cannot proceed due to an unfinished lens above it: {lens:?}"
                )
            }
        }
    }
}

impl std::error::Error for ParserStateError {}

/// Sub-class of [`StateError`] for errors that can be triggered by contrarian, adversarial, or
/// otherwise non-conformant buffer-data being processed by an otherwise correctly-implemented
/// decoder.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataStateError {
    /// Failed attempt to open a slice whose final offset overruns either an existing slice, or the buffer itself
    UnstackableSlices {
        current_offset: ByteOffset,
        current_limit: ByteOffset,
        new_slice_end: ByteOffset,
    },
    /// Attempted to read a byte having already reached the final legal offset of the buffer.
    EndOfStreamRead,
    /// No corresponding path for decoding a prefix code according to a constructed HuffmanNode-tree.
    HuffmanDescentError(crate::prelude::huffman::DescentError),
}

impl std::fmt::Display for DataStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataStateError::UnstackableSlices {
                current_offset,
                current_limit,
                new_slice_end,
            } => write!(
                f,
                "unable to open slice due to limit-violation: to-be-constructed slice endpoint {new_slice_end} exceeds existing limit (slice or stream) of {current_limit} (current offset: {current_offset})",
            ),
            DataStateError::EndOfStreamRead => {
                write!(f, "unable to read past the end of the buffer")
            }
            DataStateError::HuffmanDescentError(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for DataStateError {}

/// Sub-class of errors that only occur when an illegal operation is attempted,
/// due to incoherent usage or improperly nesting of various state-manipulation methods
/// within the [`BufferOffset`], or due to contrarian, adversarial, or otherwise malformed
/// buffer-data being processed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StateError {
    /// Errors that indicate a bug in the parsing logic itself, and can never be triggered by any
    /// buffer-data, however malformed.
    Parser(ParserStateError),
    /// Errors that can be triggered by contrarian, adversarial, or otherwise non-conformant
    /// buffer-data, even in an otherwise correctly-implemented decoder.
    Data(DataStateError),
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::Parser(e) => write!(f, "{e}"),
            StateError::Data(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for StateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StateError::Parser(e) => Some(e),
            StateError::Data(e) => Some(e),
        }
    }
}

impl From<ParserStateError> for StateError {
    fn from(value: ParserStateError) -> Self {
        StateError::Parser(value)
    }
}

impl From<DataStateError> for StateError {
    fn from(value: DataStateError) -> Self {
        StateError::Data(value)
    }
}

impl From<StateError> for ParseError {
    fn from(value: StateError) -> Self {
        ParseError::InternalError(value)
    }
}

/// Trait for conditionally suppressing errors that can be triggered by non-conformant buffer-data,
/// without silencing more fundamental state errors related to the integrity of a parser definition.
pub trait Permissible: Sized {
    /// Returns `Ok(self)` if the error is eligible for suppression (e.g. [`Format::Permit`]), and `Err(self)`
    /// if it cannot be triggered by non-conformant buffer-data alone.
    fn permit(self) -> Result<Self, Self>;

    /// If `self` is eligible for suppression, calls `log_fn` on `self` and returns `Ok(value)`. Otherwise, returns `Err(self)`.
    fn fallback_value<T: Sized>(self, value: T, log_fn: impl FnOnce(Self)) -> Result<T, Self> {
        log_fn(self.permit()?);
        Ok(value)
    }
}

impl Permissible for StateError {
    fn permit(self) -> Result<Self, Self> {
        match &self {
            StateError::Parser(_) => Err(self),
            StateError::Data(_) => Ok(self),
        }
    }
}

#[inline]
fn map_ok_err<T, U>(res: Result<T, T>, f: impl FnOnce(T) -> U) -> Result<U, U> {
    match res {
        Ok(v) => Ok(f(v)),
        Err(e) => Err(f(e)),
    }
}

impl Permissible for ParseError {
    fn permit(self) -> Result<Self, Self> {
        match &self {
            ParseError::InternalError(e) => map_ok_err(e.permit(), ParseError::InternalError),
            _ => Ok(self),
        }
    }
}

impl Permissible for CtxtParseError {
    fn permit(self) -> Result<Self, Self> {
        let CtxtParseError { err, _trace } = self;
        let res = err.permit();
        map_ok_err(res, |err| CtxtParseError { err, _trace })
    }
}
