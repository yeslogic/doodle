use crate::valuetype::BaseType;
use serde::Serialize;

// REVIEW - base-kind variants are all implicitly big-endian
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub enum BaseKind {
    U8,
    U16,
    U32,
    U64,
}

impl std::fmt::Display for BaseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl BaseKind {
    /// Returns the size for the given base-kind in bytes.
    pub const fn size(self) -> usize {
        match self {
            BaseKind::U8 => std::mem::size_of::<u8>(),
            BaseKind::U16 => std::mem::size_of::<u16>(),
            BaseKind::U32 => std::mem::size_of::<u32>(),
            BaseKind::U64 => std::mem::size_of::<u64>(),
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            BaseKind::U8 => "U8",
            BaseKind::U16 => "U16Be",
            BaseKind::U32 => "U32Be",
            BaseKind::U64 => "U64Be",
        }
    }
}

impl From<BaseKind> for BaseType {
    fn from(value: BaseKind) -> Self {
        match value {
            BaseKind::U8 => BaseType::U8,
            BaseKind::U16 => BaseType::U16,
            BaseKind::U32 => BaseType::U32,
            BaseKind::U64 => BaseType::U64,
        }
    }
}
