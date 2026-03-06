use crate::codegen::rust_ast::{AtomType, MachineSint, MachineUint, PrimType, RustType};
use crate::codegen::typed_format::GenType;
use crate::numeric::core::{BinOp, Expr, MachineRep, NumRep, TypedConst, UnaryOp};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum PrimInt {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

impl From<MachineUint> for PrimInt {
    fn from(value: MachineUint) -> Self {
        match value {
            MachineUint::U8 => PrimInt::U8,
            MachineUint::U16 => PrimInt::U16,
            MachineUint::U32 => PrimInt::U32,
            MachineUint::U64 => PrimInt::U64,
        }
    }
}

impl From<MachineSint> for PrimInt {
    fn from(value: MachineSint) -> Self {
        match value {
            MachineSint::I8 => PrimInt::I8,
            MachineSint::I16 => PrimInt::I16,
            MachineSint::I32 => PrimInt::I32,
            MachineSint::I64 => PrimInt::I64,
        }
    }
}

impl serde::Serialize for PrimInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_static_str())
    }
}

impl PrimInt {
    pub const fn to_static_str(self) -> &'static str {
        match self {
            PrimInt::U8 => "u8",
            PrimInt::U16 => "u16",
            PrimInt::U32 => "u32",
            PrimInt::U64 => "u64",
            PrimInt::I8 => "i8",
            PrimInt::I16 => "i16",
            PrimInt::I32 => "i32",
            PrimInt::I64 => "i64",
        }
    }
}

pub(crate) const PRIM_INTS: [PrimInt; 8] = [
    PrimInt::U8,
    PrimInt::U16,
    PrimInt::U32,
    PrimInt::U64,
    PrimInt::I8,
    PrimInt::I16,
    PrimInt::I32,
    PrimInt::I64,
];

#[derive(Debug)]
pub struct TryFromAutoError;

impl std::fmt::Display for TryFromAutoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot convert `NumRep::AUTO` to `PrimInt`")
    }
}

impl std::error::Error for TryFromAutoError {}

impl TryFrom<NumRep> for PrimInt {
    type Error = TryFromAutoError;

    fn try_from(value: NumRep) -> Result<Self, Self::Error> {
        match value {
            NumRep::Auto => Err(TryFromAutoError),
            NumRep::Concrete(mr) => Ok(mr.into()),
        }
    }
}

impl From<MachineRep> for PrimInt {
    fn from(value: MachineRep) -> Self {
        match value {
            MachineRep::U8 => PrimInt::U8,
            MachineRep::U16 => PrimInt::U16,
            MachineRep::U32 => PrimInt::U32,
            MachineRep::U64 => PrimInt::U64,
            MachineRep::I8 => PrimInt::I8,
            MachineRep::I16 => PrimInt::I16,
            MachineRep::I32 => PrimInt::I32,
            MachineRep::I64 => PrimInt::I64,
        }
    }
}

impl From<PrimInt> for MachineRep {
    fn from(value: PrimInt) -> Self {
        match value {
            PrimInt::U8 => MachineRep::U8,
            PrimInt::U16 => MachineRep::U16,
            PrimInt::U32 => MachineRep::U32,
            PrimInt::U64 => MachineRep::U64,
            PrimInt::I8 => MachineRep::I8,
            PrimInt::I16 => MachineRep::I16,
            PrimInt::I32 => MachineRep::I32,
            PrimInt::I64 => MachineRep::I64,
        }
    }
}

impl From<IntType> for MachineRep {
    fn from(value: IntType) -> Self {
        match value {
            IntType::Prim(prim) => MachineRep::from(prim),
        }
    }
}

impl From<IntType> for NumRep {
    fn from(value: IntType) -> Self {
        match value {
            IntType::Prim(prim) => NumRep::from(prim),
        }
    }
}

impl From<PrimInt> for NumRep {
    fn from(value: PrimInt) -> Self {
        match value {
            PrimInt::U8 => NumRep::U8,
            PrimInt::U16 => NumRep::U16,
            PrimInt::U32 => NumRep::U32,
            PrimInt::U64 => NumRep::U64,
            PrimInt::I8 => NumRep::I8,
            PrimInt::I16 => NumRep::I16,
            PrimInt::I32 => NumRep::I32,
            PrimInt::I64 => NumRep::I64,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum IntType {
    Prim(PrimInt),
}

impl IntType {
    pub fn to_prim(self) -> PrimInt {
        let IntType::Prim(ret) = self;
        ret
    }

    pub const fn to_static_str(self) -> &'static str {
        match self {
            Self::Prim(p) => p.to_static_str(),
        }
    }
}

// WIP - consider what extra information is required
#[derive(Debug)]
pub struct TryFromGenTypeError(GenType);

impl std::fmt::Display for IntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_static_str())
    }
}

impl std::fmt::Display for TryFromGenTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to convert GenType to IntType: {:?}", self.0)
    }
}

impl std::error::Error for TryFromGenTypeError {}

impl TryFrom<GenType> for IntType {
    type Error = TryFromGenTypeError;

    fn try_from(value: GenType) -> Result<Self, Self::Error> {
        match value {
            GenType::Inline(RustType::Atom(AtomType::Signed(sint))) => {
                Ok(IntType::Prim(PrimInt::from(sint)))
            }
            GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::Unsigned(uint)))) => {
                Ok(IntType::Prim(PrimInt::from(uint)))
            }
            _ => Err(TryFromGenTypeError(value)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum TypedExpr<TypeRep> {
    ElabConst(TypeRep, TypedConst),
    ElabBinOp(
        TypeRep,
        TypedBinOp<TypeRep>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
    ),
    ElabUnaryOp(TypeRep, TypedUnaryOp<TypeRep>, Box<TypedExpr<TypeRep>>),
    ElabCast(TypeRep, TypedCast<TypeRep>, Box<TypedExpr<TypeRep>>),
}

pub(crate) trait MapType: Sized {
    type Rep;
    type Output<A>;

    fn map_type<T>(self, f: &impl Fn(Self::Rep) -> T) -> Self::Output<T> {
        let g = |x: Self::Rep| Ok(f(x));
        let Ok(ret) = self.try_map_type::<T, std::convert::Infallible>(&g);
        ret
    }

    fn try_map_type<T, E>(
        self,
        f: &impl Fn(Self::Rep) -> Result<T, E>,
    ) -> Result<Self::Output<T>, E>;
}

impl<X> MapType for Box<X>
where
    X: MapType,
{
    type Rep = X::Rep;
    type Output<A> = Box<X::Output<A>>;

    fn try_map_type<T, E>(
        self,
        f: &impl Fn(Self::Rep) -> Result<T, E>,
    ) -> Result<Self::Output<T>, E> {
        Ok(Box::new((*self).try_map_type(f)?))
    }
}

impl<TypeRep> std::hash::Hash for TypedExpr<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            TypedExpr::ElabConst(_, c) => {
                c.hash(state);
            }
            TypedExpr::ElabBinOp(_, bop, l, r) => {
                bop.hash(state);
                l.hash(state);
                r.hash(state);
            }
            TypedExpr::ElabUnaryOp(_, uop, e) => {
                uop.hash(state);
                e.hash(state);
            }
            TypedExpr::ElabCast(_, c, e) => {
                c.hash(state);
                e.hash(state);
            }
        }
    }
}

impl<T> TypedExpr<T> {
    pub fn get_type(&self) -> &T {
        match self {
            TypedExpr::ElabConst(t, _) => t,
            TypedExpr::ElabBinOp(t, _, _, _) => t,
            TypedExpr::ElabUnaryOp(t, _, _) => t,
            TypedExpr::ElabCast(t, _, _) => t,
        }
    }
}

mod __impls {
    use super::*;

    #[allow(clippy::boxed_local)]
    fn rebox<T, U: From<T>>(b: Box<T>) -> Box<U> {
        Box::new(U::from(*b))
    }

    impl<TypeRep> From<TypedBinOp<TypeRep>> for BinOp {
        fn from(value: TypedBinOp<TypeRep>) -> Self {
            value.inner
        }
    }

    impl<TypeRep> From<TypedUnaryOp<TypeRep>> for UnaryOp {
        fn from(value: TypedUnaryOp<TypeRep>) -> Self {
            value.inner
        }
    }

    impl<TypeRep> From<TypedExpr<TypeRep>> for Expr {
        fn from(value: TypedExpr<TypeRep>) -> Self {
            match value {
                TypedExpr::ElabConst(_, c) => Expr::Const(c),
                TypedExpr::ElabBinOp(_, op, lhs, rhs) => {
                    Expr::BinOp(op.into(), rebox(lhs), rebox(rhs))
                }
                TypedExpr::ElabUnaryOp(_, op, inner) => Expr::UnaryOp(op.into(), rebox(inner)),
                TypedExpr::ElabCast(_, cast, inner) => Expr::Cast(cast.rep, rebox(inner)),
            }
        }
    }
}

impl<TypeRep> MapType for TypedExpr<TypeRep> {
    type Rep = TypeRep;
    type Output<A> = TypedExpr<A>;

    fn try_map_type<T, E>(
        self,
        f: &impl Fn(Self::Rep) -> Result<T, E>,
    ) -> Result<Self::Output<T>, E> {
        match self {
            TypedExpr::ElabConst(t, c) => Ok(TypedExpr::ElabConst(f(t)?, c)),
            TypedExpr::ElabBinOp(t, op, l, r) => Ok(TypedExpr::ElabBinOp(
                f(t)?,
                op.try_map_type(f)?,
                l.try_map_type(f)?,
                r.try_map_type(f)?,
            )),
            TypedExpr::ElabUnaryOp(t, op, x) => Ok(TypedExpr::ElabUnaryOp(
                f(t)?,
                op.try_map_type(f)?,
                x.try_map_type(f)?,
            )),
            TypedExpr::ElabCast(t, c, x) => {
                Ok(TypedExpr::ElabCast(f(t)?, c.try_map_type(f)?, x.try_map_type(f)?))
            }
        }
    }
}

pub(crate) type Sig1<T> = (T, T);
pub(crate) type Sig2<T> = ((T, T), T);

impl<X> MapType for Sig1<X> {
    type Rep = X;
    type Output<A> = Sig1<A>;

    fn try_map_type<T, E>(
        self,
        f: &impl Fn(Self::Rep) -> Result<T, E>,
    ) -> Result<Self::Output<T>, E> {
        let (i, o) = self;
        Ok((f(i)?, f(o)?))
    }
}

impl<X> MapType for Sig2<X> {
    type Rep = X;
    type Output<A> = Sig2<A>;

    fn try_map_type<T, E>(
        self,
        f: &impl Fn(Self::Rep) -> Result<T, E>,
    ) -> Result<Self::Output<T>, E> {
        let ((i0, i1), o) = self;
        Ok(((f(i0)?, f(i1)?), f(o)?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypedBinOp<TypeRep> {
    pub sig: Sig2<TypeRep>,
    pub inner: BinOp,
}

impl<TypeRep> MapType for TypedBinOp<TypeRep> {
    type Rep = TypeRep;
    type Output<A> = TypedBinOp<A>;

    fn try_map_type<T, E>(self, f: &impl Fn(TypeRep) -> Result<T, E>) -> Result<TypedBinOp<T>, E> {
        let sig = self.sig.try_map_type(f)?;
        Ok(TypedBinOp {
            inner: self.inner,
            sig,
        })
    }
}

impl<TypeRep> std::hash::Hash for TypedBinOp<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedUnaryOp<TypeRep> {
    pub sig: Sig1<TypeRep>,
    pub inner: UnaryOp,
}

impl<TypeRep> MapType for TypedUnaryOp<TypeRep> {
    type Rep = TypeRep;
    type Output<A> = TypedUnaryOp<A>;

    fn try_map_type<T, E>(self, f: &impl Fn(TypeRep) -> Result<T, E>) -> Result<TypedUnaryOp<T>, E> {
        let sig = self.sig.try_map_type(f)?;
        Ok(TypedUnaryOp {
            inner: self.inner,
            sig,
        })
    }
}

impl<TypeRep> std::hash::Hash for TypedUnaryOp<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypedCast<TypeRep> {
    pub sig: Sig1<TypeRep>,
    pub rep: MachineRep,
}

impl<TypeRep> MapType for TypedCast<TypeRep> {
    type Rep = TypeRep;
    type Output<A> = TypedCast<A>;

    fn try_map_type<T, E>(
        self,
        f: &impl Fn(Self::Rep) -> Result<T, E>,
    ) -> Result<Self::Output<T>, E> {
        let sig = self.sig.try_map_type(f)?;
        Ok(TypedCast {
            rep: self.rep,
            sig,
        })
    }
}

impl<TypeRep> std::hash::Hash for TypedCast<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rep.hash(state);
    }
}

use crate::typecheck::{UVar, inference::InferenceEngine};

/// Alias for whatever value-type we use to associate a failed reification with some indication of what went wrong, or where
type Hint = usize;

#[derive(Debug)]
pub enum ElaborationError {
    BadReification(Hint),
}

impl std::fmt::Display for ElaborationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElaborationError::BadReification(hint) => {
                write!(f, "bad reification on UVar ?{}", hint)
            }
        }
    }
}

impl std::error::Error for ElaborationError {}

pub(crate) type ElaborationResult<T> = Result<T, ElaborationError>;

pub type ElabRc = Elaborator<std::rc::Rc<InferenceEngine>>;

pub struct Elaborator<E = Box<InferenceEngine>>
where
    E: std::ops::Deref<Target = InferenceEngine> + Sized + 'static,
{
    next_index: usize,
    engine: E,
}

impl<E> Elaborator<E>
where
    E: std::ops::Deref<Target = InferenceEngine> + Sized + 'static,
{
    pub(crate) fn new(engine: E) -> Self {
        Self {
            next_index: 0,
            engine,
        }
    }

    fn get_and_increment_index(&mut self) -> usize {
        let ret = self.next_index;
        self.next_index += 1;
        ret
    }

    fn get_type_from_index(&self, index: usize) -> ElaborationResult<IntType> {
        let uvar = UVar::new(index);
        let Some(t) = self.engine.reify(uvar.into()) else {
            return Err(ElaborationError::BadReification(index));
        };
        Ok(t)
    }

    pub(crate) fn elaborate_expr_as<T>(&mut self, expr: &Expr) -> ElaborationResult<TypedExpr<T>>
    where
        T: From<IntType> + Clone,
    {
        let index = self.get_and_increment_index();
        match expr {
            Expr::Const(typed_const) => {
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabConst(T::from(t), typed_const.clone()))
            }
            Expr::BinOp(bin_op, x, y) => {
                let t_x = self.elaborate_expr_as::<T>(x)?;
                let t_y = self.elaborate_expr_as::<T>(y)?;
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabBinOp(
                    T::from(t),
                    TypedBinOp {
                        sig: ((t_x.get_type().clone(), t_y.get_type().clone()), T::from(t)),
                        inner: *bin_op,
                    },
                    Box::new(t_x),
                    Box::new(t_y),
                ))
            }
            Expr::UnaryOp(unary_op, inner) => {
                let t_inner = self.elaborate_expr_as::<T>(inner)?;
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabUnaryOp(
                    T::from(t),
                    TypedUnaryOp {
                        sig: (t_inner.get_type().clone(), T::from(t)),
                        inner: *unary_op,
                    },
                    Box::new(t_inner),
                ))
            }
            &Expr::Cast(rep, ref inner) => {
                let t_inner = self.elaborate_expr_as::<T>(inner)?;
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabCast(
                    T::from(t),
                    TypedCast {
                        sig: (t_inner.get_type().clone(), T::from(t)),
                        rep,
                    },
                    Box::new(t_inner),
                ))
            }
        }
    }

    pub(crate) fn elaborate_expr(&mut self, expr: &Expr) -> ElaborationResult<TypedExpr<IntType>> {
        let index = self.get_and_increment_index();
        match expr {
            Expr::Const(typed_const) => {
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabConst(t, typed_const.clone()))
            }
            Expr::BinOp(bin_op, x, y) => {
                let t_x = self.elaborate_expr(x)?;
                let t_y = self.elaborate_expr(y)?;
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabBinOp(
                    t,
                    TypedBinOp {
                        sig: ((*t_x.get_type(), *t_y.get_type()), t),
                        inner: *bin_op,
                    },
                    Box::new(t_x),
                    Box::new(t_y),
                ))
            }
            Expr::UnaryOp(unary_op, inner) => {
                let t_inner = self.elaborate_expr(inner)?;
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabUnaryOp(
                    t,
                    TypedUnaryOp {
                        sig: (*t_inner.get_type(), t),
                        inner: *unary_op,
                    },
                    Box::new(t_inner),
                ))
            }
            &Expr::Cast(rep, ref inner) => {
                let t_inner = self.elaborate_expr(inner)?;
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabCast(
                    t,
                    TypedCast {
                        sig: (*t_inner.get_type(), t),
                        rep,
                    },
                    Box::new(t_inner),
                ))
            }
        }
    }
}
