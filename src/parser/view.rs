use crate::parser::error::ParseError as DoodleParseError;
use crate::{
    alt::prelude::allsorts::{
        binary::{
            read::{self, ReadArray, ReadScope},
            U16Be, U32Be, U64Be, U8,
        },
        error::ParseError as AllSortsParseError,
    },
    parser::{error::OverrunKind, offset::ByteOffset},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct View<'a> {
    pub(crate) buffer: &'a [u8],
    pub(crate) start_offset: usize,
}

impl<'a> View<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            start_offset: 0,
        }
    }

    /// Reads a slice of `len` bytes from the View, offset by `offset`.
    pub fn read_len(&self, len: usize) -> &'a [u8] {
        &self.buffer[..len]
    }

    pub fn offset(&self, offset: usize) -> Self {
        View {
            buffer: &self.buffer[offset..],
            start_offset: self.start_offset + offset,
        }
    }

    pub fn as_read_array<T>(&self, len: usize) -> Result<ReadArray<'a, T>, DoodleParseError>
    where
        T: read::ReadUnchecked,
    {
        let size = <T as read::ReadUnchecked>::SIZE;
        let scope = ReadScope::from_parts(self.buffer, self.start_offset);
        let mut ctxt = scope.ctxt();
        match ctxt.read_array::<T>(len) {
            Ok(ret) => Ok(ret),
            Err(e) => match e {
                AllSortsParseError::BadEof => {
                    Err(DoodleParseError::Overrun(OverrunKind::EndOfStream {
                        offset: ByteOffset::from_bytes(self.start_offset + len * size),
                        max_offset: ByteOffset::from_bytes(self.start_offset + self.buffer.len()),
                    }))
                }
                // FIXME - use a more robust conversion strategy
                _ => unreachable!("unexpected error-kind from `read_array`: {:?}", e),
            },
        }
    }

    // NOTE - we need these separate methods because RustExpr::MethodCall doesn't allow turbo-fish type-parameters
    pub fn read_array_u8(&self, len: usize) -> Result<ReadArray<'a, U8>, DoodleParseError> {
        self.as_read_array(len)
    }

    pub fn read_array_u16be(&self, len: usize) -> Result<ReadArray<'a, U16Be>, DoodleParseError> {
        self.as_read_array(len)
    }

    pub fn read_array_u32be(&self, len: usize) -> Result<ReadArray<'a, U32Be>, DoodleParseError> {
        self.as_read_array(len)
    }

    pub fn read_array_u64be(&self, len: usize) -> Result<ReadArray<'a, U64Be>, DoodleParseError> {
        self.as_read_array(len)
    }
}
