use crate::parser::error::ParseError as DoodleParseError;
use crate::{
    alt::prelude::allsorts::{
        binary::{
            read::{self, ReadArray, ReadScope},
            *,
        },
        error::ParseError as AllSortsParseError,
    },
    parser::{error::OverrunKind, offset::ByteOffset},
};

/// Isolated, state-free 'slice' of a `Parser` object, which can be processed using basic
/// view-centric methods, or reified back into a first-class `Parser` object for arbitrary
/// parsing operations.
///
/// # Note
///
/// This type is definitionally similar to [`ReadCtxt`](crate::read::ReadCtxt) and [`allsorts::ReadScope`](crate::alt::prelude::allsorts::binary::read::ReadScope),
/// but its role as a counterpart to `Parser` distinguishes it as the candidate-type in `prelude` for treating
/// `ViewFormat` and `ViewExpr` constructions, and direct conversion to and from `Parser`.
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

    /// Returns the logical start-offset of `self` within the original source buffer from which it was derived.
    ///
    /// Artificially constructed `View` objects (i.e. those which are not derived from a Parser or View object in turn) will default to a start-offset of `0`.
    pub fn get_offset(&self) -> usize {
        self.start_offset
    }

    /// Helper method for converting a relative offset to its absolute offset in the original source-buffer.
    ///
    /// When the relative-offset is well-formed (i.e. in-bounds of the current View), the returned absolute-offset
    /// should agree with `self.offset(relative_offset).unwrap().get_offset()`.
    ///
    /// However, this method is purely arithmetic and does not do any bounds-checking, so it may return an
    /// absolute offset that is out-of-bounds of the original source-buffer.
    ///
    /// # Notes
    ///
    /// This method is provided mainly to facilitate the construction of globally-unique and consistent identifiers,
    /// to use as keys when cacheing offset-indirected objects in downstream consumers (to avoid parsing the same
    /// data multiple times if it may be pointed to in more than one place).
    pub fn relative_to_absolute(&self, relative_offset: usize) -> usize {
        self.start_offset + relative_offset
    }

    /// Reads a slice of `len` bytes from the View, offset by `offset`.
    ///
    /// # Panics
    ///
    /// Panics if the requested len overruns the end of the internal slice held by `self`.
    pub fn read_len(&self, len: usize) -> &'a [u8] {
        &self.buffer[..len]
    }

    /// Returns the `View` derived by shiufting the start-offset of the current `View` by `offset` bytes.
    ///
    /// If there are fewer than `offset` bytes remaining in the current `View`, returns an error indicating the overrun.
    ///
    /// Designed for one-to-one conformity with the intensional semantics of [`ViewExpr::Offset`](crate::ViewExpr::Offset)
    pub fn offset(&self, offset: usize) -> Result<Self, DoodleParseError> {
        if offset > self.buffer.len() {
            Err(DoodleParseError::Overrun(OverrunKind::EndOfStream {
                offset: ByteOffset::from_bytes(self.start_offset + offset),
                max_offset: ByteOffset::from_bytes(self.start_offset + self.buffer.len()),
            }))
        } else {
            Ok(View {
                buffer: &self.buffer[offset..],
                start_offset: self.start_offset + offset,
            })
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
                // NOTE - because the error we return contains extra data that depends on local context, it can't be losslessly merged into the `ParseError::from_allsorts_error` case
                AllSortsParseError::BadEof => {
                    Err(DoodleParseError::Overrun(OverrunKind::EndOfStream {
                        offset: ByteOffset::from_bytes(self.start_offset + len * size),
                        max_offset: ByteOffset::from_bytes(self.start_offset + self.buffer.len()),
                    }))
                }
                other_err => Err(DoodleParseError::from_allsorts_error(other_err)),
            },
        }
    }

    pub fn read_u16be(&self) -> Result<u16, DoodleParseError> {
        if self.buffer.len() < 2 {
            return Err(DoodleParseError::Overrun(OverrunKind::EndOfStream {
                offset: ByteOffset::from_bytes(self.start_offset + 2),
                max_offset: ByteOffset::from_bytes(self.start_offset + self.buffer.len()),
            }));
        }
        Ok(u16::from_be_bytes([self.buffer[0], self.buffer[1]]))
    }

    // NOTE - MarkerType has no suppport for U24Be so this method cannot be specified generically in the codegen pipeline; however, it still isn't possible to name a Format that requires this, either
    pub fn read_array_u24be(&self, len: usize) -> Result<ReadArray<'a, U24Be>, DoodleParseError> {
        self.as_read_array::<U24Be>(len)
    }
}

impl DoodleParseError {
    /// Fallback method for casting an `AllSortsParseError` to a [`DoodleParseError`](crate::parser::error::ParseError)
    fn from_allsorts_error(err: AllSortsParseError) -> Self {
        // REVIEW - determine whether are any special-case correspondences we would like to map to custom ParseError variants rather than embedding indiscriminately
        Self::AllsortsError(err)
    }
}
