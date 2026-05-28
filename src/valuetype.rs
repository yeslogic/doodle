use std::collections::{BTreeMap, HashSet};

use anyhow::{Result as AResult, anyhow};
use serde::Serialize;

use crate::codegen::rust_ast::MachineSint;
use crate::{
    Label,
    numeric::{
        core::{MachineRep, NumRep},
        elaborator::{IntType, PrimInt},
    },
    typecheck::error::UnificationError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash, PartialOrd, Ord)]
pub enum BaseType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    Char,
}

impl BaseType {
    pub const fn to_static_str(self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::Char => "char",
        }
    }
}

impl std::fmt::Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}

impl BaseType {
    pub(crate) fn is_numeric(&self) -> bool {
        matches!(self, Self::U8 | Self::U16 | Self::U32 | Self::U64)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash, PartialOrd, Ord)]
pub enum SignedIntType {
    I8,
    I16,
    I32,
    I64,
}

impl SignedIntType {
    pub const fn to_static_str(self) -> &'static str {
        match self {
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
        }
    }
}

impl std::fmt::Display for SignedIntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}

macro_rules! from_signed_int {
    ( $( $tgt:ident ),+ ) => {
        $(
            impl From<SignedIntType> for $tgt {
                fn from(s: SignedIntType) -> Self {
                    match s {
                        SignedIntType::I8 => $tgt::I8,
                        SignedIntType::I16 => $tgt::I16,
                        SignedIntType::I32 => $tgt::I32,
                        SignedIntType::I64 => $tgt::I64,
                    }
                }
            }
        )+
    }
}

from_signed_int!(MachineRep, PrimInt, MachineSint);

impl From<SignedIntType> for IntType {
    fn from(s: SignedIntType) -> Self {
        IntType::Prim(PrimInt::from(s))
    }
}

impl From<SignedIntType> for NumRep {
    fn from(s: SignedIntType) -> Self {
        NumRep::Concrete(MachineRep::from(s))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default, Serialize)]
pub enum SeqBorrowHint {
    #[default]
    /// Hint for arbitrary sequences that may either be constructed synthetically or dynamically,
    /// whether requiring variable-width or context-dependent decoding of a section of
    /// buffer-data, or whose source-bytes are non-contiguous.
    Constructed,
    /// Hint specific to [`ViewFormat::ReadArray`], which is backed by a fixed slice of the source-buffer
    /// and interpreted dynamically as a series of fixed-width values with a common type and unambiguous
    /// mapping from bytes to values (e.g. `U16Be`).
    ReadArray,
    /// Hint for an implied view (slice) of the buffer-data, e.g. `ViewFormat::CaptureBytes`.
    BufferView,
}

impl SeqBorrowHint {
    /// Returns `true` if the hint is [`SeqBorrowHint::Constructed`].
    pub fn is_constructed(&self) -> bool {
        matches!(self, Self::Constructed)
    }

    /// Returns `true` if the hint is [`SeqBorrowHint::BufferView`].
    pub fn is_buffer_view(&self) -> bool {
        matches!(self, Self::BufferView)
    }

    /// Returns `true` if the hint is [`SeqBorrowHint::ReadArray`].
    pub fn is_read_array(&self) -> bool {
        matches!(self, Self::ReadArray)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum ValueType {
    /// Polymorphic hole used for unconstrained parameter types (e.g. the element type for an empty sequence)
    Any,
    /// Bottom type representing value-less modes of failure (e.g. `Format::Fail`)
    Empty,
    /// Model-level type for whatever object type is used to represent the `View` abstraction
    ViewObj,
    /// Place-holding marker type to associate a selectively deferred parse with its proper type
    PhantomData(Box<ValueType>),
    Base(BaseType),
    /// Like [`Any`], but for polymorphic numeric nodes or embedded NumTrees with an `Auto` rep.
    NumericHole,
    Signed(SignedIntType),
    Tuple(Vec<ValueType>),
    Record(Vec<(Label, ValueType)>),
    Union(BTreeMap<Label, ValueType>),
    Seq(Box<ValueType>),
    Option(Box<ValueType>),
}

impl From<BaseType> for ValueType {
    fn from(b: BaseType) -> Self {
        ValueType::Base(b)
    }
}

impl From<SignedIntType> for ValueType {
    fn from(s: SignedIntType) -> Self {
        ValueType::Signed(s)
    }
}

mod __impl {
    use super::{BaseType, SignedIntType, ValueType};
    use crate::numeric::{MachineRep, NumRep, PrimInt, elaborator::IntType};

    macro_rules! from_uin {
        ( $( $src:ident ),+ ) => {
            $(
                impl From<$src> for ValueType {
                    fn from(value: $src) -> Self {
                        match value {
                            $src::U8 => ValueType::Base(BaseType::U8),
                            $src::U16 => ValueType::Base(BaseType::U16),
                            $src::U32 => ValueType::Base(BaseType::U32),
                            $src::U64 => ValueType::Base(BaseType::U64),
                            $src::I8 => ValueType::Signed(SignedIntType::I8),
                            $src::I16 => ValueType::Signed(SignedIntType::I16),
                            $src::I32 => ValueType::Signed(SignedIntType::I32),
                            $src::I64 => ValueType::Signed(SignedIntType::I64),
                        }
                    }
                }
            )+
        }
    }

    from_uin!(PrimInt, MachineRep);

    impl From<NumRep> for ValueType {
        fn from(value: NumRep) -> Self {
            match value {
                NumRep::Auto => ValueType::NumericHole,
                NumRep::Concrete(m) => m.into(),
            }
        }
    }

    impl From<IntType> for ValueType {
        fn from(value: IntType) -> Self {
            let IntType::Prim(prim) = value;
            prim.into()
        }
    }
}

impl ValueType {
    pub const BOOL: ValueType = ValueType::Base(BaseType::Bool);
    pub const UNIT: ValueType = ValueType::Tuple(Vec::new());

    pub const U8: ValueType = ValueType::Base(BaseType::U8);
    pub const U16: ValueType = ValueType::Base(BaseType::U16);
    pub const U32: ValueType = ValueType::Base(BaseType::U32);
    pub const U64: ValueType = ValueType::Base(BaseType::U64);

    pub const I8: ValueType = ValueType::Signed(SignedIntType::I8);
    pub const I16: ValueType = ValueType::Signed(SignedIntType::I16);
    pub const I32: ValueType = ValueType::Signed(SignedIntType::I32);
    pub const I64: ValueType = ValueType::Signed(SignedIntType::I64);

    /// Formalization of the hard-coded `u32` type for sequence lengths to avoid hardcoding U32 directly over multiple modules.
    pub const SEQ_LEN_T: ValueType = ValueType::Base(BaseType::U32);

    /// Helper function for constructing `ValueType::Option`.
    pub fn option(ty: Self) -> ValueType {
        ValueType::Option(Box::new(ty))
    }

    /// Given a `ValueType::Record` as `self` along with an identifier `label` that is a field of `self`,
    /// returns the corresponding type for said field.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not a record type, or if `label` is not a field of `self`.
    pub(crate) fn record_proj(&self, label: &str) -> ValueType {
        match self {
            ValueType::Record(fields) => match fields.iter().find(|(l, _)| label == l) {
                Some((_, t)) => t.clone(),
                None => panic!(
                    "ValueType::record_proj: field `{label}` not found in record type: {self:?}"
                ),
            },
            _ => panic!("projection `_.{label}` failed: expected record type, found {self:?}"),
        }
    }

    /// Deconstructs a `ValueType::Tuple` into its component types, preserving the original order.
    ///
    /// Returns `Err` if `self` is not a tuple type.
    pub(crate) fn try_into_tuple_type(self) -> AResult<Vec<ValueType>> {
        match self {
            ValueType::Tuple(ts) => Ok(ts),
            t => Err(anyhow!("type is not a tuple: {t:?}")),
        }
    }

    /// Returns a borrowed slice of the component types of a `ValueType::Tuple`.
    ///
    /// Returns `Err` if `self` is not a tuple type.`
    pub(crate) fn try_as_tuple_type(&self) -> AResult<&[ValueType]> {
        match self {
            ValueType::Tuple(ts) => Ok(ts.as_slice()),
            t => Err(anyhow!("type is not a tuple: {t:?}")),
        }
    }

    /// Performs standalone unification between `self` and `other`.
    pub fn unify(&self, other: &ValueType) -> Result<ValueType, UnificationError<ValueType>> {
        match (self, other) {
            (ValueType::Empty, ValueType::Empty) => Ok(ValueType::Empty),

            // NOTE - we have to specify these patterns before the similar cases for Empty because we want (Empty, Any) in either order to yield Empty
            (ValueType::Any, rhs) => Ok(rhs.clone()),
            (lhs, ValueType::Any) => Ok(lhs.clone()),

            (ValueType::Empty, rhs) => Ok(rhs.clone()),
            (lhs, ValueType::Empty) => Ok(lhs.clone()),

            (ValueType::NumericHole, rhs) if rhs.is_numeric() => Ok(rhs.clone()),
            (lhs, ValueType::NumericHole) if lhs.is_numeric() => Ok(lhs.clone()),

            (ValueType::ViewObj, ValueType::ViewObj) => Ok(ValueType::ViewObj),
            (ValueType::Base(b1), ValueType::Base(b2)) => {
                if b1 == b2 {
                    Ok(ValueType::Base(*b1))
                } else {
                    Err(UnificationError::Unsatisfiable(self.clone(), other.clone()))
                }
            }
            (ValueType::Signed(s1), ValueType::Signed(s2)) => {
                if s1 == s2 {
                    Ok(ValueType::Signed(*s1))
                } else {
                    Err(UnificationError::Unsatisfiable(self.clone(), other.clone()))
                }
            }
            (ValueType::Tuple(ts1), ValueType::Tuple(ts2)) => {
                if ts1.len() != ts2.len() {
                    // tuple arity mismatch
                    return Err(UnificationError::Unsatisfiable(self.clone(), other.clone()));
                }
                let mut ts = Vec::new();
                for (t1, t2) in Iterator::zip(ts1.iter(), ts2.iter()) {
                    ts.push(t1.unify(t2)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            (ValueType::Record(fs1), ValueType::Record(fs2)) => {
                if fs1.len() != fs2.len() {
                    // field count mismatch
                    return Err(UnificationError::Unsatisfiable(self.clone(), other.clone()));
                }
                // NOTE - because fields are parsed in declared order, two records with conflicting field orders are not operationally equivalent
                let mut fs = Vec::new();
                for ((l1, t1), (l2, t2)) in Iterator::zip(fs1.iter(), fs2.iter()) {
                    if l1 != l2 {
                        // field label mismatch
                        return Err(UnificationError::Unsatisfiable(self.clone(), other.clone()));
                    }
                    fs.push((l1.clone(), t1.unify(t2)?));
                }
                Ok(ValueType::Record(fs))
            }
            (ValueType::Union(bs1), ValueType::Union(bs2)) => {
                let mut bs: BTreeMap<Label, ValueType> = BTreeMap::new();

                let keys1 = bs1.keys().collect::<HashSet<_>>();
                let keys2 = bs2.keys().collect::<HashSet<_>>();

                let keys_common = HashSet::union(&keys1, &keys2).cloned();

                for key in keys_common.into_iter() {
                    match (bs1.get(key), bs2.get(key)) {
                        (Some(t1), Some(t2)) => {
                            let t = t1.unify(t2)?;
                            bs.insert(key.clone(), t);
                        }
                        (Some(t), None) | (None, Some(t)) => {
                            bs.insert(key.clone(), t.clone());
                        }
                        (None, None) => unreachable!("key must appear in at least one operand"),
                    }
                }

                Ok(ValueType::Union(bs))
            }
            (ValueType::Seq(t1), ValueType::Seq(t2)) => Ok(ValueType::Seq(Box::new(t1.unify(t2)?))),
            (ValueType::Option(t1), ValueType::Option(t2)) => {
                Ok(ValueType::Option(Box::new(t1.unify(t2)?)))
            }
            (ValueType::PhantomData(t1), ValueType::PhantomData(t2)) => {
                Ok(ValueType::PhantomData(Box::new(t1.unify(t2)?)))
            }
            (t1, t2) => Err(UnificationError::Unsatisfiable(t1.clone(), t2.clone())),
        }
    }

    pub(crate) fn is_numeric(&self) -> bool {
        match self {
            ValueType::Base(b) => b.is_numeric(),
            ValueType::NumericHole | ValueType::Signed(_) => true,
            _ => false,
        }
    }
}

/// Alias to reduce the number of code-sites we need to update if we pick a different Smart-Pointer type
/// as the backer of `TypeHint`
pub(crate) type Container<T> = std::rc::Rc<T>; // Box<T>;

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeHint(Container<ValueType>);

impl TypeHint {
    pub fn into_inner(&self) -> &Container<ValueType> {
        &self.0
    }
}

impl AsRef<ValueType> for TypeHint {
    fn as_ref(&self) -> &ValueType {
        self.0.as_ref()
    }
}

impl Serialize for TypeHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl From<ValueType> for TypeHint {
    fn from(t: ValueType) -> Self {
        Self(Container::new(t))
    }
}

pub(crate) mod augmented {
    use super::*;
    use crate::numeric::elaborator::PrimInt;

    #[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
    pub enum AugValueType {
        Any,
        Empty,
        ViewObj,
        NumericHole,
        Bool,
        Char,
        Int(PrimInt),
        Tuple(Vec<AugValueType>),
        Record(Vec<(Label, AugValueType)>),
        Union(BTreeMap<Label, AugValueType>),
        Seq(Box<AugValueType>, SeqBorrowHint),
        Option(Box<AugValueType>),
        PhantomData(Box<AugValueType>),
    }

    impl AugValueType {
        pub const UNIT: Self = AugValueType::Tuple(Vec::new());
    }

    impl From<BaseType> for AugValueType {
        fn from(t: BaseType) -> Self {
            match t {
                BaseType::Bool => AugValueType::Bool,
                BaseType::Char => AugValueType::Char,
                BaseType::U8 => AugValueType::Int(PrimInt::U8),
                BaseType::U16 => AugValueType::Int(PrimInt::U16),
                BaseType::U32 => AugValueType::Int(PrimInt::U32),
                BaseType::U64 => AugValueType::Int(PrimInt::U64),
            }
        }
    }

    impl From<ValueType> for AugValueType {
        fn from(t: ValueType) -> Self {
            match t {
                ValueType::NumericHole => AugValueType::NumericHole,
                ValueType::Any => AugValueType::Any,
                ValueType::Empty => AugValueType::Empty,
                ValueType::ViewObj => AugValueType::ViewObj,
                ValueType::Base(BaseType::Bool) => AugValueType::Bool,
                ValueType::Base(BaseType::Char) => AugValueType::Char,
                ValueType::Base(BaseType::U8) => AugValueType::Int(PrimInt::U8),
                ValueType::Base(BaseType::U16) => AugValueType::Int(PrimInt::U16),
                ValueType::Base(BaseType::U32) => AugValueType::Int(PrimInt::U32),
                ValueType::Base(BaseType::U64) => AugValueType::Int(PrimInt::U64),
                ValueType::Signed(s) => AugValueType::Int(PrimInt::from(s)),
                ValueType::Tuple(ts) => {
                    AugValueType::Tuple(ts.into_iter().map(From::from).collect())
                }
                ValueType::Record(fs) => {
                    AugValueType::Record(fs.into_iter().map(|(l, t)| (l, From::from(t))).collect())
                }
                ValueType::Union(bs) => {
                    AugValueType::Union(bs.into_iter().map(|(l, t)| (l, From::from(t))).collect())
                }
                ValueType::Seq(t) => {
                    AugValueType::Seq(Box::new(From::from(*t)), SeqBorrowHint::default())
                }
                ValueType::Option(t) => AugValueType::Option(Box::new(From::from(*t))),
                ValueType::PhantomData(t) => AugValueType::PhantomData(Box::new(From::from(*t))),
            }
        }
    }

    impl From<AugValueType> for ValueType {
        fn from(t: AugValueType) -> Self {
            match t {
                AugValueType::Any => ValueType::Any,
                AugValueType::Empty => ValueType::Empty,
                AugValueType::ViewObj => ValueType::ViewObj,
                AugValueType::Int(int_t) => ValueType::from(int_t),
                AugValueType::NumericHole => ValueType::NumericHole,
                AugValueType::Bool => ValueType::Base(BaseType::Bool),
                AugValueType::Char => ValueType::Base(BaseType::Char),
                AugValueType::Tuple(ts) => {
                    ValueType::Tuple(ts.into_iter().map(From::from).collect())
                }
                AugValueType::Record(fs) => {
                    ValueType::Record(fs.into_iter().map(|(l, t)| (l, From::from(t))).collect())
                }
                AugValueType::Union(bs) => {
                    ValueType::Union(bs.into_iter().map(|(l, t)| (l, From::from(t))).collect())
                }
                AugValueType::Seq(t, _) => ValueType::Seq(Box::new(From::from(*t))),
                AugValueType::Option(t) => ValueType::Option(Box::new(From::from(*t))),
                AugValueType::PhantomData(t) => ValueType::PhantomData(Box::new(From::from(*t))),
            }
        }
    }
}
