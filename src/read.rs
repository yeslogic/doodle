use serde::Serialize;

use crate::{BaseKind, Endian};

/// The kind of buffer that a Decoder is currently operating on, which can be used to provide more context in error messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferKind {
    /// Original input buffer (e.g. the entire input to a [`Decoder`](crate::decoder::Decoder))
    Input,
    /// Sub-buffer slice created by a [`Decoder::Slice`](crate::decoder::Decoder::Slice) operation
    Slice,
    /// Generated `View` buffer being read by a `ViewFormat`-derived `Decoder` (or [`Decoder::ParseFromView`](crate::decoder::Decoder::ParseFromView))
    View,
    /// Value-level byte-array buffer being decoded through a [`Decoder::DecodeBytes`](crate::decoder::Decoder::DecodeBytes) operation
    Value,
}

impl std::fmt::Display for BufferKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input => write!(f, "input-stream"),
            Self::Slice => write!(f, "slice"),
            Self::View => write!(f, "buffer-view"),
            Self::Value => write!(f, "byte-array"),
        }
    }
}

impl std::ops::BitAnd for BufferKind {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BufferKind::Value, _) | (_, BufferKind::Value) => BufferKind::Value,
            (BufferKind::Slice, _) | (_, BufferKind::Slice) => BufferKind::Slice,
            (BufferKind::View, _) | (_, BufferKind::View) => BufferKind::View,
            (BufferKind::Input, BufferKind::Input) => BufferKind::Input,
        }
    }
}

impl std::ops::BitAndAssign for BufferKind {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

#[derive(Copy, Clone, Serialize)]
pub struct ReadCtxt<'a> {
    #[serde(skip)]
    pub input: &'a [u8],
    pub offset: usize,
    #[serde(skip)]
    pub kind: BufferKind,
}

impl<'a> std::fmt::Debug for ReadCtxt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReadCtxt {{ input: [_; {}], offset: {} }}",
            self.input.len(),
            self.offset
        )
    }
}

impl<'a> ReadCtxt<'a> {
    pub fn new(input: &'a [u8]) -> ReadCtxt<'a> {
        let offset = 0;
        ReadCtxt {
            input,
            offset,
            kind: BufferKind::Input,
        }
    }

    /// Identical to [`ReadCtxt::new`], but sets the `kind` to `BufferKind::Value` instead of `BufferKind::Input`.
    pub fn from_value(bytes: &'a [u8]) -> ReadCtxt<'a> {
        let offset = 0;
        ReadCtxt {
            input: bytes,
            offset,
            kind: BufferKind::Value,
        }
    }

    /// Creates a new copy of `self` that denotes it is to be treated as a view-buffer.
    pub fn as_view(&self) -> ReadCtxt<'a> {
        ReadCtxt {
            kind: self.kind & BufferKind::View,
            ..*self
        }
    }

    pub fn remaining(&self) -> &'a [u8] {
        &self.input[self.offset..]
    }
    /// Creates a new `ReadCtxt` with the same `input` as the current `ReadCtxt`, but with an `offset` of `n`.
    ///
    /// The new `ReadCtxt` is only created if `n` is a valid offset into the `input` slice.
    pub fn seek_to(&self, n: usize) -> Option<ReadCtxt<'a>> {
        if n <= self.input.len() {
            Some(ReadCtxt { offset: n, ..*self })
        } else {
            None
        }
    }

    /// Splits the current `ReadCtxt` at the given position relative to the current offset,
    /// returning a tuple of two `ReadCtxt` instances if the split is valid.
    ///
    /// The first `ReadCtxt` contains the range from the current offset to `offset + n`,
    /// and the second `ReadCtxt` starts at `offset + n` and extends to the end of the input.
    ///
    /// Returns `None` if the specified position is out of bounds, i.e., if `offset + n`
    /// exceeds the length of the input.
    pub fn split_at(&self, n: usize) -> Option<(ReadCtxt<'a>, ReadCtxt<'a>)> {
        if self.offset + n <= self.input.len() {
            let fst = ReadCtxt {
                input: &self.input[..self.offset + n],
                ..*self
            };
            let snd = ReadCtxt {
                offset: self.offset + n,
                ..*self
            };
            Some((fst, snd))
        } else {
            None
        }
    }

    pub(crate) fn skip_remainder(&self) -> ReadCtxt<'a> {
        let offset = self.input.len();
        ReadCtxt { offset, ..*self }
    }
}

impl<'a> ReadCtxt<'a> {
    pub fn read_byte(&self) -> Option<(u8, ReadCtxt<'a>)> {
        if self.offset < self.input.len() {
            let b = self.input[self.offset];
            Some((
                b,
                ReadCtxt {
                    offset: self.offset + 1,
                    ..*self
                },
            ))
        } else {
            None
        }
    }

    pub fn read_u16be(&self) -> Option<(u16, ReadCtxt<'a>)> {
        const SZ: usize = std::mem::size_of::<u16>();
        if self.offset + SZ <= self.input.len() {
            let raw = &self.input[self.offset..self.offset + SZ];
            Some((
                u16::from_be_bytes(raw.try_into().unwrap()),
                ReadCtxt {
                    offset: self.offset + SZ,
                    ..*self
                },
            ))
        } else {
            None
        }
    }

    pub fn read_u16le(&self) -> Option<(u16, ReadCtxt<'a>)> {
        const SZ: usize = std::mem::size_of::<u16>();
        if self.offset + SZ <= self.input.len() {
            let raw = &self.input[self.offset..self.offset + SZ];
            Some((
                u16::from_le_bytes(raw.try_into().unwrap()),
                ReadCtxt {
                    offset: self.offset + SZ,
                    ..*self
                },
            ))
        } else {
            None
        }
    }
    pub fn read_u32be(&self) -> Option<(u32, ReadCtxt<'a>)> {
        const SZ: usize = std::mem::size_of::<u32>();
        if self.offset + SZ <= self.input.len() {
            let raw = &self.input[self.offset..self.offset + SZ];
            Some((
                u32::from_be_bytes(raw.try_into().unwrap()),
                ReadCtxt {
                    offset: self.offset + SZ,
                    ..*self
                },
            ))
        } else {
            None
        }
    }

    pub fn read_u32le(&self) -> Option<(u32, ReadCtxt<'a>)> {
        const SZ: usize = std::mem::size_of::<u32>();
        if self.offset + SZ <= self.input.len() {
            let raw = &self.input[self.offset..self.offset + SZ];
            Some((
                u32::from_le_bytes(raw.try_into().unwrap()),
                ReadCtxt {
                    offset: self.offset + SZ,
                    ..*self
                },
            ))
        } else {
            None
        }
    }

    pub fn read_u64be(&self) -> Option<(u64, ReadCtxt<'a>)> {
        const SZ: usize = std::mem::size_of::<u64>();
        if self.offset + SZ <= self.input.len() {
            let raw = &self.input[self.offset..self.offset + SZ];
            Some((
                u64::from_be_bytes(raw.try_into().unwrap()),
                ReadCtxt {
                    offset: self.offset + SZ,
                    ..*self
                },
            ))
        } else {
            None
        }
    }

    pub fn read_u64le(&self) -> Option<(u64, ReadCtxt<'a>)> {
        const SZ: usize = std::mem::size_of::<u64>();
        if self.offset + SZ <= self.input.len() {
            let raw = &self.input[self.offset..self.offset + SZ];
            Some((
                u64::from_le_bytes(raw.try_into().unwrap()),
                ReadCtxt {
                    offset: self.offset + SZ,
                    ..*self
                },
            ))
        } else {
            None
        }
    }
}

impl<'a> ReadCtxt<'a> {
    pub fn mark_slice(self) -> Self {
        match self.kind {
            BufferKind::Input | BufferKind::View => ReadCtxt {
                kind: BufferKind::Slice,
                ..self
            },
            BufferKind::Slice => self,
            BufferKind::Value => self,
        }
    }

    /// Core utility for performing a base-kind read, usable as the internal implementation for
    /// the `read_base` functions defined in each of the modules `crate::decoder` and `crate::loc_decoder`.
    pub(crate) fn read_base(
        self,
        kind: BaseKind<Endian>,
    ) -> Result<(crate::decoder::Value, Self), crate::error::BufferLimitError> {
        match kind {
            BaseKind::U8 => {
                let Some((byte, new_buf)) = self.read_byte() else {
                    return Err(self.kind.overbyte(self.offset));
                };
                Ok((crate::decoder::Value::U8(byte), new_buf))
            }
            BaseKind::U16BE => {
                let Some((val, new_buf)) = self.read_u16be() else {
                    return Err(self.kind.overrun(kind.size(), self.offset));
                };
                Ok((crate::decoder::Value::U16(val), new_buf))
            }
            BaseKind::U16LE => {
                let Some((val, new_buf)) = self.read_u16le() else {
                    return Err(self.kind.overrun(kind.size(), self.offset));
                };
                Ok((crate::decoder::Value::U16(val), new_buf))
            }

            BaseKind::U32BE => {
                let Some((val, new_buf)) = self.read_u32be() else {
                    return Err(self.kind.overrun(kind.size(), self.offset));
                };
                Ok((crate::decoder::Value::U32(val), new_buf))
            }
            BaseKind::U32LE => {
                let Some((val, new_buf)) = self.read_u32le() else {
                    return Err(self.kind.overrun(kind.size(), self.offset));
                };
                Ok((crate::decoder::Value::U32(val), new_buf))
            }
            BaseKind::U64BE => {
                let Some((val, new_buf)) = self.read_u64be() else {
                    return Err(self.kind.overrun(kind.size(), self.offset));
                };
                Ok((crate::decoder::Value::U64(val), new_buf))
            }
            BaseKind::U64LE => {
                let Some((val, new_buf)) = self.read_u64le() else {
                    return Err(self.kind.overrun(kind.size(), self.offset));
                };
                Ok((crate::decoder::Value::U64(val), new_buf))
            }
        }
    }
}
