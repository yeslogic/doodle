use crate::Label;
use doodle::{prelude::ByteSet, read::ReadCtxt};

#[derive(Debug)]
pub enum DecodeError {
    Fail {
        message: Label,
        offset: usize,
        buffer: Vec<u8>,
    },
    Trailing {
        byte: u8,
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

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fail {
                message,
                offset,
                buffer,
            } => {
                write!(
                    f,
                    "failed with message \"{message}\" at Offset={offset}, Buffer={buffer:#?}"
                )
            }
            Self::Trailing { byte, offset } => {
                write!(
                    f,
                    "byte `{byte:02x}` found when end-of-input expected (offset = {offset})"
                )
            }
            Self::Overbyte { offset } => {
                write!(
                    f,
                    "attempted read of byte would overrun buffer (offset = {offset})"
                )
            }
            Self::Unexpected {
                found,
                expected,
                offset,
            } => {
                write!(
                    f,
                    "byte `{found:02x}` found when {expected:?} expected (offset = {offset})"
                )
            }
            Self::NoValidBranch { offset } => {
                write!(f, "no valid branch at offset {offset}")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

impl DecodeError {
    pub fn fail(message: Label, input: ReadCtxt<'_>) -> Self {
        let offset = input.offset;
        let buffer = input.input.to_owned();
        Self::Fail {
            message,
            offset,
            buffer,
        }
    }
}
