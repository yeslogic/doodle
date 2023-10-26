use crate::{byte_set::ByteSet, ReadCtxt, Scope, Value};

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    Fail {
        bindings: Vec<(String, Value)>,
        buffer: Vec<u8>,
        offset: usize,
    },
    Trailing {
        byte: u8,
        offset: usize,
    },
    Overrun {
        nbytes: usize,
        offset: usize,
    },
    Overbyte {
        offset: usize,
    },
    Unexpected {
        found: u8,
        expected: ByteSet,
        offset: usize,
    },
    NoValidBranch {
        offset: usize,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fail {
                bindings,
                buffer,
                offset,
            } => {
                write!(
                    f,
                    "parse failure at Offset={offset}, Buffer={buffer:#?} (Scope: {bindings:?})"
                )
            }
            Self::Trailing { byte, offset } => {
                write!(
                    f,
                    "byte `{byte:02x}` found when end-of-input expected (offset = {offset})"
                )
            }
            Self::Overrun { nbytes, offset } => {
                write!(
                    f,
                    "attempt to split {nbytes} bytes ahead at offset {offset} would overrun buffer"
                )
            }
            Self::Overbyte { offset } => {
                write!(
                    f,
                    "attempted to read byte at offset {offset}, but encountered end-of-input"
                )
            }
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
        }
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    pub fn fail(scope: &Scope, input: ReadCtxt<'_>) -> Self {
        let bindings = scope.iter().collect::<Vec<_>>();
        let buffer = input.input.to_owned();
        let offset = input.offset;
        Self::Fail {
            bindings,
            buffer,
            offset,
        }
    }

    pub fn trailing(byte: u8, offset: usize) -> Self {
        Self::Trailing { byte, offset }
    }

    pub fn overrun(skip: usize, offset: usize) -> Self {
        Self::Overrun {
            nbytes: skip,
            offset,
        }
    }

    pub fn overbyte(offset: usize) -> Self {
        Self::Overbyte { offset }
    }

    pub fn unexpected(found: u8, expected: ByteSet, offset: usize) -> Self {
        Self::Unexpected {
            found,
            expected,
            offset,
        }
    }
}
