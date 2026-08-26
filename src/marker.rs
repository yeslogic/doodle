use crate::FormatRef;
use crate::valuetype::{BaseNumType, BaseType, SignedIntType};
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

// ANCHOR - basekind-enum
/// Marker-type for various widths of machine-integer parse-directives,
/// with support for generic decoration with either `()` or [`Endian`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub enum BaseKind<X: Copy = ()> {
    // FIXME[epic=exotic-int-parse] - add support for U24
    U8,
    U16Ext(X),
    U32Ext(X),
    U64Ext(X),
    I8,
    I16Ext(X),
    I32Ext(X),
    I64Ext(X),
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

    pub const I16: BaseKind<()> = BaseKind::I16Ext(());
    pub const I32: BaseKind<()> = BaseKind::I32Ext(());
    pub const I64: BaseKind<()> = BaseKind::I64Ext(());

    // NOTE - LE shortcuts elided for the signed widths: nothing in doodle-formats currently
    // needs them (OpenType, the only consumer so far, is all big-endian). Add I16LE/I32LE/I64LE
    // here if/when a little-endian signed format shows up.
    pub const I16BE: BaseKind<Endian> = BaseKind::I16Ext(Endian::Be);
    pub const I32BE: BaseKind<Endian> = BaseKind::I32Ext(Endian::Be);
    pub const I64BE: BaseKind<Endian> = BaseKind::I64Ext(Endian::Be);
}

impl BaseKind {
    pub const fn name(&self) -> &'static str {
        match self {
            BaseKind::U8 => "U8",
            BaseKind::U16Ext(_) => "U16",
            BaseKind::U32Ext(_) => "U32",
            BaseKind::U64Ext(_) => "U64",
            BaseKind::I8 => "I8",
            BaseKind::I16Ext(_) => "I16",
            BaseKind::I32Ext(_) => "I32",
            BaseKind::I64Ext(_) => "I64",
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

            BaseKind::I8 => "I8",

            BaseKind::I16Ext(Be) => "I16Be",
            BaseKind::I32Ext(Be) => "I32Be",
            BaseKind::I64Ext(Be) => "I64Be",

            BaseKind::I16Ext(Le) => "I16Le",
            BaseKind::I32Ext(Le) => "I32Le",
            BaseKind::I64Ext(Le) => "I64Le",
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
            BaseKind::I8 => std::mem::size_of::<i8>(),
            BaseKind::I16Ext(..) => std::mem::size_of::<i16>(),
            BaseKind::I32Ext(..) => std::mem::size_of::<i32>(),
            BaseKind::I64Ext(..) => std::mem::size_of::<i64>(),
        }
    }
}

impl<X: Copy> From<BaseKind<X>> for BaseNumType {
    fn from(value: BaseKind<X>) -> Self {
        match value {
            BaseKind::U8 => BaseNumType::Unsigned(BaseType::U8),
            BaseKind::U16Ext(..) => BaseNumType::Unsigned(BaseType::U16),
            BaseKind::U32Ext(..) => BaseNumType::Unsigned(BaseType::U32),
            BaseKind::U64Ext(..) => BaseNumType::Unsigned(BaseType::U64),
            BaseKind::I8 => BaseNumType::Signed(SignedIntType::I8),
            BaseKind::I16Ext(..) => BaseNumType::Signed(SignedIntType::I16),
            BaseKind::I32Ext(..) => BaseNumType::Signed(SignedIntType::I32),
            BaseKind::I64Ext(..) => BaseNumType::Signed(SignedIntType::I64),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum FixedReadKind {
    Base(BaseKind<Endian>),
    /// Reads a value of the format named by the given [`FormatRef`], which must be provably
    /// fixed-size and composed entirely of base-kind (`u8`/`u16be`/`u32be`/`u64be`, ...) fields
    /// with no data-dependent control flow -- see `record_fmt::analyze_fixed_shape`, which is
    /// used to validate this precondition (at typecheck-time) and to compute the field layout
    /// (at Decoder-compile-time and codegen-elaboration-time).
    FixedFormat(FormatRef),
}

impl From<BaseKind<Endian>> for FixedReadKind {
    fn from(kind: BaseKind<Endian>) -> Self {
        FixedReadKind::Base(kind)
    }
}

impl From<FormatRef> for FixedReadKind {
    fn from(format_ref: FormatRef) -> Self {
        FixedReadKind::FixedFormat(format_ref)
    }
}
