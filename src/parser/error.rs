#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseError {
    FailToken,
    NegatedSuccess,
    ExcludedBranch,
    Overrun(OverrunKind),
    InternalError(StateError),
    IncompleteParse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OverrunKind {
    EndOfStream,
    EndOfSlice,
    EndOfLookahead,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::FailToken => write!(f, "reached Fail token"),
            ParseError::ExcludedBranch => write!(f, "no MatchTree branch matches buffer contents"),
            ParseError::NegatedSuccess => write!(f, "successful parse in negated context"),
            ParseError::IncompleteParse => write!(
                f,
                "incomplete parse: expected end-of-stream, but >0 bytes remain unconsumed"
            ),
            ParseError::Overrun(k) => match k {
                OverrunKind::EndOfStream => write!(f, "offset would extend past end of stream"),
                OverrunKind::EndOfSlice => write!(f, "offset would extend past end of slice"),
                OverrunKind::EndOfLookahead => {
                    write!(f, "offset would extend past end of lookahead-window")
                }
            },
            ParseError::InternalError(_) => todo!(),
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

impl ParseError {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::InternalError(StateError::NoRecovery) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StateError {
    NoRecovery,
    UnstackableSlices,
    NoRestore,
    BinaryModeError,
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::NoRecovery => write!(f, "unable to recover from parse failure"),
            StateError::NoRestore => write!(f, "unable to restore to a parsing checkpoint"),
            StateError::UnstackableSlices => write!(
                f,
                "unable to open slice that violates existing slice boundaries"
            ),
            StateError::BinaryModeError => write!(f, "illegal binary-mode switch operation"),
        }
    }
}

impl std::error::Error for StateError {}

impl From<StateError> for ParseError {
    fn from(value: StateError) -> Self {
        ParseError::InternalError(value)
    }
}
