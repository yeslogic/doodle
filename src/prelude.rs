pub use crate::byte_set::ByteSet;
pub use crate::decoder::Scope;
pub use crate::parser::{
    error::{PResult, ParseError},
    monad::ParseMonad,
};

pub fn u16le(input: (u8, u8)) -> u16 {
    u16::from_le_bytes([input.0, input.1])
}

pub fn u16be(input: (u8, u8)) -> u16 {
    u16::from_be_bytes([input.0, input.1])
}

pub fn u32le(input: (u8, u8, u8, u8)) -> u32 {
    u32::from_le_bytes([input.0, input.1, input.2, input.3])
}

pub fn u32be(input: (u8, u8, u8, u8)) -> u32 {
    u32::from_be_bytes([input.0, input.1, input.2, input.3])
}

pub fn u64le(input: (u8, u8, u8, u8, u8, u8, u8, u8)) -> u64 {
    u64::from_le_bytes([
        input.0, input.1, input.2, input.3, input.4, input.5, input.6, input.7,
    ])
}

pub fn u64be(input: (u8, u8, u8, u8, u8, u8, u8, u8)) -> u64 {
    u64::from_be_bytes([
        input.0, input.1, input.2, input.3, input.4, input.5, input.6, input.7,
    ])
}

pub fn dup32<T: Clone>(count: u32, value: T) -> Vec<T> {
    Vec::from_iter(std::iter::repeat(value).take(count as usize))
}
