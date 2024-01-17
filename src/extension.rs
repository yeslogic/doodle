#![allow(dead_code)]
use crate::ValueType;

pub(crate) type NLInfo<X> = <X as Extension<NLKind>>::Info;
pub(crate) type HOInfo<X> = <X as Extension<HOKind>>::Info;
// pub(crate) type MCInfo<X: Extension<HOKind>> = <X as Extension<MCKind>>::Info;


/// Extension-Kind (type family subscript) for Non-Leaf constructors
pub struct NLKind;

/// Extension-Kind (type family subscript) for Higher-Order constructors
pub struct HOKind;

// / Extension-Kind (type family subscript) for Monomorphic Cast constructors
// pub struct MCKind;


mod sealed {
    pub trait ExtKind {}

    impl ExtKind for super::NLKind {}
    impl ExtKind for super::HOKind {}
    // impl ExtKind for MCKind {}
}

pub trait Extension<K: sealed::ExtKind> {
    type Info;

    fn info() -> Self::Info;
}

pub trait Follows<X: Extension<K>, K> where K: sealed::ExtKind, Self: Extension<K> {
    fn downcast(info: Self::Info) -> X::Info;
}

impl<K> Follows<UD, K> for TC where UD: Extension<K>, K: sealed::ExtKind, TC: Extension<K> {
    fn downcast(_info: Self::Info) -> <UD as Extension<K>>::Info {
        UD::info()
    }
}

impl<X: Extension<K>, K: sealed::ExtKind> Follows<X, K> for X {
    fn downcast(info: Self::Info) -> <X as Extension<K>>::Info {
        info
    }
}

pub trait MaybeCast<X: Extension<K>, K> where K: sealed::ExtKind, Self: Extension<K> {
    #[allow(unused_variables)]
    fn cast(info: Self::Info) -> Option<X::Info> {
        None
    }
}

impl<X, Y, K> MaybeCast<X, K> for Y
where Y: Follows<X, K>, X: Extension<K>, K: sealed::ExtKind
{
    fn cast(info: Y::Info) -> Option<<X as Extension<K>>::Info> {
        Some(Y::downcast(info))
    }
}

impl MaybeCast<TC, HOKind> for UD {}
impl MaybeCast<TC, NLKind> for UD {}

/// Extension Index for Undecorated trees
pub struct UD;

impl<K: sealed::ExtKind> Extension<K> for UD {
    type Info = ();

    fn info() -> Self::Info {
        ()
    }
}

trait TotalExtension: Extension<NLKind> + Extension<HOKind> /* + Extension<MCKind> */ {}

impl<X> TotalExtension for X where X: Extension<NLKind> + Extension<HOKind> /* + Extension<MCKind> */ {}

/// Extension index for Typechecked trees
pub struct TC;

impl Extension<NLKind> for TC {
    type Info = ValueType;

    fn info() -> Self::Info {
        ValueType::Any
    }
}

impl Extension<HOKind> for TC {
    type Info = (ValueType, ValueType);

    fn info() -> Self::Info {
        (ValueType::Any, ValueType::Any)
    }
}
