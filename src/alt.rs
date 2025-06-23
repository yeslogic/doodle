pub mod prelude;

use crate::{Format, Label, Expr};

pub mod marker {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum BaseKind {
        U8 = 1,
        U16 = 2,
        U32 = 4,
        U64 = 8,
    }

    impl BaseKind {
        /// Returns the size for the given base-kind in bytes.
        pub const fn size(self) -> usize {
            self as usize
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetaFormat {
    BindScopeTo(Label),
    WithScope(Label, ScopeFormat),
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeFormat {
    ReadArray(marker::BaseKind),
    ReadSliceLen(Expr),
}

#[derive(Debug, Clone)]
pub enum AltFormat {
    Ground(Format),
    Meta(MetaFormat),
}
