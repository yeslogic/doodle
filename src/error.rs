use crate::byte_set::ByteSet;
use crate::decoder::{Scope, ScopeEntry, Value};
use crate::loc_decoder::{LocScope, ParsedValue};
use crate::read::ReadCtxt;
use crate::{Expr, Label};

pub type DecodeResult<T> = Result<T, DecodeError>;
pub type LocDecodeError<T> = Result<T, DecodeError<crate::loc_decoder::ParsedValue>>;

#[derive(Debug)]
pub enum DecodeError<V: Clone = Value> {
    Fail {
        bindings: Vec<(Label, ScopeEntry<V>)>,
        offset: usize,
        buffer: Vec<u8>,
    },
    BadWhere {
        bindings: Vec<(Label, ScopeEntry<V>)>,
        assertion: Box<Expr>,
        exception: Box<V>,
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
    SeekPastEnd {
        seek_offset: usize,
        buffer_len: usize,
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

impl<V: std::fmt::Debug + Clone> std::fmt::Display for DecodeError<V> {
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
            Self::SeekPastEnd {
                seek_offset,
                buffer_len,
            } => {
                write!(
                    f,
                    "attempt to seek to buffer offset {seek_offset} would overrun buffer (len: {buffer_len})"
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

impl<V: std::fmt::Debug + Clone> std::error::Error for DecodeError<V> {}

impl DecodeError<Value> {
    pub fn fail(scope: &Scope<'_>, input: ReadCtxt<'_>) -> DecodeError<Value> {
        let mut bindings = Vec::new();
        scope.get_bindings(&mut bindings);
        let offset = input.offset;
        let buffer = input.input.to_owned();
        DecodeError::Fail {
            bindings,
            offset,
            buffer,
        }
    }

    pub fn bad_where(
        scope: &Scope<'_>,
        assertion: Box<Expr>,
        exception: Box<Value>,
    ) -> DecodeError<Value> {
        let mut bindings = Vec::new();
        scope.get_bindings(&mut bindings);
        DecodeError::BadWhere {
            bindings,
            assertion,
            exception,
        }
    }
}

impl DecodeError<ParsedValue> {
    pub fn loc_fail(scope: &LocScope<'_>, input: ReadCtxt<'_>) -> DecodeError<ParsedValue> {
        let mut bindings = Vec::new();
        scope.get_bindings(&mut bindings);
        let buffer = input.input.to_owned();
        let offset = input.offset;
        DecodeError::Fail {
            bindings,
            buffer,
            offset,
        }
    }

    pub fn loc_bad_where(
        scope: &LocScope<'_>,
        assertion: Box<Expr>,
        exception: Box<ParsedValue>,
    ) -> DecodeError<ParsedValue> {
        let mut bindings = Vec::new();
        scope.get_bindings(&mut bindings);
        DecodeError::BadWhere {
            bindings,
            assertion,
            exception,
        }
    }
}

impl<V: Clone> DecodeError<V> {
    pub fn trailing(byte: u8, offset: usize) -> Self {
        Self::Trailing { byte, offset }
    }

    pub fn bad_seek(seek_offset: usize, buffer_len: usize) -> Self {
        DecodeError::SeekPastEnd {
            seek_offset,
            buffer_len,
        }
    }

    /// Constructs a DecodeError that indicates that a split attempt (of `nbytes` bytes) would overrun the buffer based
    /// on the immediate offset (`offset`)
    pub fn overrun(nbytes: usize, offset: usize) -> Self {
        Self::Overrun { nbytes, offset }
    }

    /// Constructs a DecodeError that indicates that a (one-byte) read attempt would overrun the buffer based on the immediate offset (`offset`)
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
