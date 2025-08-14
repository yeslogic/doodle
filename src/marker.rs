use crate::valuetype::BaseType;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Default)]
pub enum Endian {
    #[default]
    Be,
    Le,
}

impl Endian {
    pub const fn as_camel(&self) -> &'static str {
        match self {
            Endian::Be => "Be",
            Endian::Le => "Le",
        }
    }

    pub const fn as_lower(&self) -> &'static str {
        match self {
            Endian::Be => "be",
            Endian::Le => "le",
        }
    }

    pub const fn as_upper(&self) -> &'static str {
        match self {
            Endian::Be => "BE",
            Endian::Le => "LE",
        }
    }
}

/// Marker-type for various widths of machine-integer parse-directives,
/// with support for generic decoration with either `()` or [`Endian`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub enum BaseKind<X: Copy = ()> {
    U8,
    U16Ext(X),
    U32Ext(X),
    U64Ext(X),
}

impl BaseKind {
    pub const U16: BaseKind<()> = BaseKind::U16Ext(());
    pub const U32: BaseKind<()> = BaseKind::U32Ext(());
    pub const U64: BaseKind<()> = BaseKind::U64Ext(());

    pub const U16BE: BaseKind<Endian> = BaseKind::U16Ext(Endian::Be);
    pub const U32BE: BaseKind<Endian> = BaseKind::U32Ext(Endian::Be);
    pub const U64BE: BaseKind<Endian> = BaseKind::U64Ext(Endian::Be);

    pub const U16LE: BaseKind<Endian> = BaseKind::U16Ext(Endian::Le);
    pub const U32LE: BaseKind<Endian> = BaseKind::U32Ext(Endian::Le);
    pub const U64LE: BaseKind<Endian> = BaseKind::U64Ext(Endian::Le);
}

impl BaseKind {
    pub const fn name(&self) -> &'static str {
        match self {
            BaseKind::U8 => "U8",
            BaseKind::U16Ext(_) => "U16",
            BaseKind::U32Ext(_) => "U32",
            BaseKind::U64Ext(_) => "U64",
        }
    }
}

impl BaseKind<Endian> {
    pub const fn name(&self) -> &'static str {
        use Endian::*;
        match self {
            BaseKind::U8 => "U8",

            BaseKind::U16Ext(Be) => "U16Be",
            BaseKind::U32Ext(Be) => "U32Be",
            BaseKind::U64Ext(Be) => "U64Be",

            BaseKind::U16Ext(Le) => "U16Le",
            BaseKind::U32Ext(Le) => "U32Le",
            BaseKind::U64Ext(Le) => "U64Le",
        }
    }
}

impl std::fmt::Display for BaseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl<X: Copy> BaseKind<X> {
    /// Returns the size for the given base-kind in bytes.
    pub const fn size(&self) -> usize {
        match self {
            BaseKind::U8 => std::mem::size_of::<u8>(),
            BaseKind::U16Ext(..) => std::mem::size_of::<u16>(),
            BaseKind::U32Ext(..) => std::mem::size_of::<u32>(),
            BaseKind::U64Ext(..) => std::mem::size_of::<u64>(),
        }
    }
}

impl<X: Copy> From<BaseKind<X>> for BaseType {
    fn from(value: BaseKind<X>) -> Self {
        match value {
            BaseKind::U8 => BaseType::U8,
            BaseKind::U16Ext(..) => BaseType::U16,
            BaseKind::U32Ext(..) => BaseType::U32,
            BaseKind::U64Ext(..) => BaseType::U64,
        }
    }
}
