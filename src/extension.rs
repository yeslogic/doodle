use crate::ValueType;

pub(crate) type NLInfo<X: Extension<NLKind>> = <X as Extension<NLKind>>::Info;
pub(crate) type HOInfo<X: Extension<HOKind>> = <X as Extension<HOKind>>::Info;
// pub(crate) type MCInfo<X: Extension<HOKind>> = <X as Extension<MCKind>>::Info;


/// Extension-Kind (type family subscript) for Non-Leaf constructors
pub struct NLKind;

/// Extension-Kind (type family subscript) for Higher-Order constructors
pub struct HOKind;

// / Extension-Kind (type family subscript) for Monomorphic Cast constructors
// pub struct MCKind;


mod sealed {
    pub trait ExtKind {}

    impl ExtKind for NLKind {}
    impl ExtKind for HOKind {}
    // impl ExtKind for MCKind {}
}

pub trait Extension<K: sealed::ExtKind> {
    type Info;

    fn info() -> Self::Info;
}

pub trait Follows<X: Extension<K>> where K: sealed::ExtKind, Self: Extension<K> {
    fn downcast(info: Self::Info) -> X::Info;
}

impl Follows<UD> for TC {
    fn downcast(info: Self::Info) -> <UD as Extension>::Info {
        ()
    }
}

impl<X: Extension<K>, K: ExtKind> Follows<X> for X {
    fn downcast(info: Self::Info) -> <X as Extension>::Info {
        info
    }
}

pub trait Magic<X: Extension<K>> where K: sealed::ExtKind, Self: Extension<K> {
    fn cast(info: Self::Info) -> Option<X::Info> {
        None
    }
}

impl<X, Y> Magic<X> for Y
where Y: Follows<X>,
{
    fn cast(info: Y::Info) -> Option<X::Info> {
        Some(Y::downcast(info))
    }
}

impl Magic<TC> for UD {}



pub struct UD;

impl<K: sealed::ExtKind> Extension<K> for UD {
    type Info = ();

    fn info() -> Self::Info {
        ()
    }
}

trait TotalExtension: Extension<NLKind> + Extension<HOKind> /* + Extension<MCKind> */ {}

impl<X> TotalExtension for X where X: Extension<NLKind> + Extension<HOKind> /* + Extension<MCKind> */ {}

pub struct TC;

impl Extension<NLKind> for TC {
    type Info = ValueType;

    fn info() -> Self::Info {
        ValueType::Hole
    }
}

impl Extension<HOKind> for TC {
    type Info = (ValueType, ValueType);

    fn info() -> Self::Info {
        (ValueType::Hole, ValueType::Hole)
    }
}
