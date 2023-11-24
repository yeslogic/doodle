use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use crate::output::{Fragment, FragmentBuilder};

use crate::{Label, ValueType};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) enum Visibility {
    #[default]
    Implicit,
    Pub,
}

impl Visibility {
    fn add_vis(&self, item: Fragment) -> Fragment {
        match self {
            Self::Implicit => item,
            Self::Pub => Fragment::cat(Fragment::string("pub "), item),
        }
    }
}

pub(crate) struct RustProgram {
    items: Vec<RustItem>,
}

impl ToFragment for RustProgram {
    fn to_fragment(&self) -> Fragment {
        let mut frags = FragmentBuilder::new();
        for item in self.items.iter() {
            frags.push(item.to_fragment().cat_break().cat_break());
        }
        frags.finalize()
    }
}

/// Top Level Item
pub struct RustItem {
    vis: Visibility,
    decl: RustDecl,
}

impl RustItem {
    pub fn to_fragment(&self) -> Fragment {
        let it_frag = self.decl.to_fragment();
        self.vis.add_vis(it_frag)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustDecl {
    TypeAlias(Label, RustType),
    TypeDef(Label, RustTypeDef),
    Function(RustFn),
}

impl RustDecl {
    pub fn to_fragment(&self) -> Fragment {
        match self {
            RustDecl::TypeAlias(aname, rhs) => Fragment::string("type")
                .cat(Fragment::String(aname.clone()))
                .cat(Fragment::string(" = "))
                .cat(rhs.to_fragment()),
            RustDecl::TypeDef(name, tdef) => {
                let frag_key = Fragment::string(tdef.keyword_for());
                Fragment::intervene(frag_key, Fragment::String(name.clone()), tdef.to_fragment())
            }
            RustDecl::Function(fdef) => fdef.to_fragment(),
        }
    }
}

/// NOTE: no support for relative longevity bounds on Lifetimes or trait bounds on parameters
#[derive(Clone, Debug, Default)]
pub(crate) struct RustParams {
    lt_params: Vec<Label>,
    ty_params: Vec<Label>,
}

impl ToFragment for RustParams {
    fn to_fragment(&self) -> Fragment {
        let all = self.lt_params.iter().chain(self.ty_params.iter());
        Fragment::seq(
            all.map(|lab| Fragment::String(lab.clone())),
            Some(Fragment::string(", ")),
        )
        .delimit(Fragment::Char('<'), Fragment::Char('>'))
    }
}

#[derive(Clone, Debug)]
pub struct FnSig {
    args: Vec<(Label, RustType)>,
    ret: Option<RustType>,
}

impl ToFragment for (Label, RustType) {
    fn to_fragment(&self) -> Fragment {
        Fragment::String(self.0.clone()).intervene(Fragment::string(": "), self.1.to_fragment())
    }
}

impl ToFragment for FnSig {
    fn to_fragment(&self) -> Fragment {
        ToFragment::paren_list(self.args.iter()).intervene(
            Fragment::string(" -> "),
            Fragment::opt(self.ret.as_ref(), RustType::to_fragment),
        )
    }
}

#[derive(Clone, Debug)]
pub struct RustFn {
    name: Label,
    params: Option<RustParams>,
    sig: FnSig,
    body: Vec<RustStmt>,
}

impl ToFragment for RustFn {
    fn to_fragment(&self) -> Fragment {
        let f_name = Fragment::string(self.name.clone());
        let f_params = Fragment::opt(self.params.as_ref(), RustParams::to_fragment);
        let f_sig = self.sig.to_fragment();
        let body = RustStmt::block(self.body.iter());
        Fragment::string("fn ")
            .cat(f_name)
            .cat(f_params)
            .cat(f_sig)
            .cat(Fragment::Char(' '))
            .cat(body)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustTypeDef {
    Enum(Vec<(Label, RustStruct)>),
    Struct(RustStruct),
}

impl RustTypeDef {
    pub fn keyword_for(&self) -> &'static str {
        match self {
            Self::Enum(..) => "enum",
            Self::Struct(..) => "struct",
        }
    }

    pub fn to_fragment(&self) -> Fragment {
        match self {
            RustTypeDef::Enum(vars) => {
                let iter = vars
                    .iter()
                    .map(|(vname, vdef)| Fragment::String(vname.clone()).cat(vdef.to_fragment()));
                let inner = Fragment::seq(iter, Some(Fragment::string(", ")));
                inner.delimit(Fragment::string("{ "), Fragment::string(" }"))
            }
            RustTypeDef::Struct(_) => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustType {
    Atom(AtomType),
    Generic(Label),
    ImplTrait(Label),
    AnonTuple(Vec<RustType>),
    SelfType,
}

impl RustType {
    pub const UNIT: RustType = RustType::Atom(AtomType::Prim(PrimType::Unit));

    pub fn named(name: impl Into<Label>) -> Self {
        Self::Atom(AtomType::Named(name.into()))
    }

    pub fn vec_of(inner: Self) -> Self {
        Self::Atom(AtomType::Comp(CompType::Vec(Box::new(inner))))
    }

    pub fn anon_tuple(elts: impl IntoIterator<Item = Self>) -> Self {
        Self::AnonTuple(elts.into_iter().collect())
    }
}

impl ToFragment for RustType {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustType::Atom(at) => at.to_fragment(),
            RustType::Generic(ident) => Fragment::String(ident.clone()),
            RustType::ImplTrait(ident) => {
                Fragment::cat(Fragment::string("impl "), Fragment::String(ident.clone()))
            }
            RustType::AnonTuple(params) => {
                let inner = params.iter().map(|elt| elt.to_fragment());
                Fragment::seq(inner, Some(Fragment::string(", ")))
                    .delimit(Fragment::Char('('), Fragment::Char(')'))
            }
            RustType::SelfType => Fragment::string("Self"),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustStruct {
    Unit(Label),
    Tuple(Label, Vec<RustType>),
    Struct(Label, Vec<(Label, RustType)>),
}

impl ToFragment for RustStruct {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustStruct::Unit(lab) => Fragment::String(lab.clone()),
            RustStruct::Tuple(lab, args) => {
                Fragment::String(lab.clone()).cat(RustType::paren_list(args.iter()))
            }
            RustStruct::Struct(lab, flds) => Fragment::String(lab.clone())
                .cat(Fragment::Char(' '))
                .cat(<(Label, RustType)>::block(flds.iter())),
        }
    }
}

type NarrowAtom = AtomType<Box<AtomType>>;

#[derive(Clone, Debug)]
pub(crate) enum AtomType<T = Box<RustType>, U = T>
where
    T: Sized,
    U: Sized,
{
    Named(Label),
    Prim(PrimType),
    Comp(CompType<T, U>),
}

impl<T, U> ToFragment for AtomType<T, U>
where
    T: Sized + ToFragment,
    U: Sized + ToFragment,
{
    fn to_fragment(&self) -> Fragment {
        match self {
            AtomType::Named(label) => Fragment::String(label.clone()),
            AtomType::Prim(pt) => pt.to_fragment(),
            AtomType::Comp(ct) => ct.to_fragment(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum PrimType {
    Unit,
    U8,
    U16,
    U32,
    Bool,
    Char,
    Usize,
}

impl ToFragment for PrimType {
    fn to_fragment(&self) -> Fragment {
        Fragment::string(match self {
            PrimType::Unit => "()",
            PrimType::U8 => "u8",
            PrimType::U16 => "u16",
            PrimType::U32 => "u32",
            PrimType::Bool => "bool",
            PrimType::Char => "char",
            PrimType::Usize => "usize",
        })
    }
}

/// AST type for Rust Lifetimes
#[derive(Clone, Debug, PartialEq)]
pub enum RustLt {
    /// `'static`
    Static,
    /// `'_``
    Wildcard,
    /// Label contents should contain leading `'`
    Parametric(Label),
}

impl ToFragment for RustLt {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustLt::Static => Fragment::string("'static"),
            RustLt::Wildcard => Fragment::string("'_"),
            RustLt::Parametric(lab) => Fragment::String(lab.clone()),
        }
    }
}

/// Compound type that is either unary over `T` or binary over `T, U`.
///
/// If not specified, `U` will implicitly have the same type as `T`
#[derive(Clone, Debug)]
pub(crate) enum CompType<T, U = T> {
    Vec(T),
    Option(T),
    Box(T),
    Result(T, U),
    Borrow(Option<RustLt>, Mut, T),
    Slice(Option<RustLt>, Mut, T),
    Array(T, usize),
}

impl<T, U> ToFragment for CompType<T, U>
where
    T: ToFragment,
    U: ToFragment,
{
    fn to_fragment(&self) -> Fragment {
        match self {
            CompType::Vec(inner) => {
                let tmp = inner.to_fragment();
                tmp.delimit(Fragment::string("Vec<"), Fragment::Char('>'))
            }
            CompType::Option(inner) => {
                let tmp = inner.to_fragment();
                tmp.delimit(Fragment::string("Option<"), Fragment::Char('>'))
            }
            CompType::Box(inner) => {
                let tmp = inner.to_fragment();
                tmp.delimit(Fragment::string("Box<"), Fragment::Char('>'))
            }
            CompType::Result(ok, err) => {
                let tmp = ok
                    .to_fragment()
                    .intervene(Fragment::string(", "), err.to_fragment());
                tmp.delimit(Fragment::string("Result<"), Fragment::Char('>'))
            }
            CompType::Borrow(lt, _mut, ty) => {
                let f_lt = Fragment::opt(lt.as_ref(), <RustLt as ToFragment>::to_fragment);
                let f_mut = _mut.to_fragment();
                let f_aux = Fragment::intervene(f_lt, Fragment::Char(' '), f_mut);
                let f_body = Fragment::intervene(f_aux, Fragment::Char(' '), ty.to_fragment());
                Fragment::cat(Fragment::Char('&'), f_body)
            }
            CompType::Slice(lt, _mut, ty) => {
                let f_lt = Fragment::opt(lt.as_ref(), <RustLt as ToFragment>::to_fragment);
                let f_mut = _mut.to_fragment();
                let f_aux = Fragment::intervene(f_lt, Fragment::Char(' '), f_mut);
                let f_inner = ty
                    .to_fragment()
                    .delimit(Fragment::Char('['), Fragment::Char(']'));
                let f_body = Fragment::intervene(f_aux, Fragment::Char(' '), f_inner);
                Fragment::cat(Fragment::Char('&'), f_body)
            }
            CompType::Array(ty, sz) => Fragment::delimit(
                Fragment::intervene(
                    ty.to_fragment(),
                    Fragment::string("; "),
                    Fragment::DisplayAtom(Rc::new(*sz)),
                ),
                Fragment::Char('['),
                Fragment::Char(']'),
            ),
        }
    }
}

pub(crate) struct TypeCompiler {
    next_ix: usize,
    revmap: HashMap<ValueType, Label>,
}

impl TypeCompiler {
    fn new() -> Self {
        Self {
            next_ix: 0,
            revmap: HashMap::new(),
        }
    }

    fn find_name_for(&mut self, vt: &ValueType) -> Label {
        if let Some(name) = self.revmap.get(vt) {
            name.clone()
        } else {
            let ix = self.next_ix;
            let name = format!("Anon{}", ix);
            self.next_ix += 1;
            let ret: Label = Cow::Owned(name);
            self.revmap.insert(vt.clone(), ret.clone());
            ret
        }
    }
}

impl From<PrimType> for AtomType {
    fn from(value: PrimType) -> Self {
        Self::Prim(value)
    }
}

impl From<CompType<Box<RustType>>> for AtomType {
    fn from(value: CompType<Box<RustType>>) -> Self {
        Self::Comp(value)
    }
}

impl From<AtomType> for RustType {
    fn from(value: AtomType) -> Self {
        Self::Atom(value)
    }
}

impl From<PrimType> for RustType {
    fn from(value: PrimType) -> Self {
        Self::from(AtomType::from(value))
    }
}

impl From<CompType<Box<RustType>>> for RustType {
    fn from(value: CompType<Box<RustType>>) -> Self {
        Self::from(AtomType::from(value))
    }
}

impl TryFrom<ValueType> for RustType {
    type Error = ValueType;

    fn try_from(value: ValueType) -> Result<Self, Self::Error> {
        match value {
            ValueType::Empty => Ok(RustType::UNIT),
            ValueType::Bool => Ok(PrimType::Bool.into()),
            ValueType::U8 => Ok(PrimType::U8.into()),
            ValueType::U16 => Ok(PrimType::U16.into()),
            ValueType::U32 => Ok(PrimType::U32.into()),
            ValueType::Char => Ok(PrimType::Char.into()),
            ValueType::Tuple(mut vs) => {
                let mut buf = Vec::with_capacity(vs.len());
                for v in vs.drain(..) {
                    buf.push(Self::try_from(v)?)
                }
                Ok(Self::AnonTuple(buf))
            }
            ValueType::Seq(t) => {
                let inner = Self::try_from(t.as_ref().clone())?;
                Ok(CompType::<Box<RustType>>::Vec(Box::new(inner)).into())
            }
            ValueType::Any | ValueType::Record(..) | ValueType::Union(..) => Err(value),
        }
    }
}

fn value_type_to_fragment_with(vt: &ValueType, tycom: &mut TypeCompiler) -> Fragment {
    match vt {
        ValueType::Any => Fragment::Char('_'),
        ValueType::Empty => Fragment::string("()"),
        ValueType::Bool => Fragment::string("bool"),
        ValueType::U8 => Fragment::string("u8"),
        ValueType::U16 => Fragment::string("u16"),
        ValueType::U32 => Fragment::string("u32"),
        ValueType::Char => Fragment::string("char"),
        ValueType::Tuple(ts) => Fragment::seq(
            ts.iter().map(|vt| value_type_to_fragment_with(vt, tycom)),
            Some(Fragment::string(", ")),
        )
        .delimit(Fragment::Char('('), Fragment::Char(')')),
        ValueType::Record(..) | ValueType::Union(..) => {
            let name = tycom.find_name_for(vt);
            Fragment::String(name)
        }
        ValueType::Seq(inner) => value_type_to_fragment_with(&inner, tycom)
            .delimit(Fragment::string("Vec<"), Fragment::Char('>')),
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug)]
pub(crate) enum Mut {
    #[default]
    Immutable,
    Mutable,
}

impl ToFragment for Mut {
    fn to_fragment(&self) -> Fragment {
        match self {
            Mut::Mutable => Fragment::string("mut"),
            Mut::Immutable => Fragment::Empty,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RustEntity {
    Local(Label),
    Scoped(Vec<Label>, Label),
    SelfEntity,
}

impl RustEntity {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustEntity::Local(v) => Fragment::String(v.clone()),
            RustEntity::Scoped(path, v) => Fragment::seq(
                path.iter()
                    .chain(std::iter::once(v))
                    .map(|scope| Fragment::String(scope.clone())),
                Some(Fragment::string("::")),
            ),
            RustEntity::SelfEntity => Fragment::string("self"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RustExpr {
    Entity(RustEntity),
    StringLit(Label),
    FieldAccess(Box<RustExpr>, Label), // can be used for receiver methods as well, with FunctionCall
    FunctionCall(Box<RustExpr>, Vec<RustExpr>), // can be used for tuple constructors as well
    Tuple(Vec<RustExpr>),
    Struct(Label, Vec<(Label, Option<Box<RustExpr>>)>),
    Paren(Box<RustExpr>),
    Borrow(Box<RustExpr>),
    BorrowMut(Box<RustExpr>),
    Try(Box<RustExpr>),
}

impl RustExpr {
    pub fn local(name: impl Into<Label>) -> Self {
        Self::Entity(RustEntity::Local(name.into()))
    }

    pub const SELF: Self = Self::Entity(RustEntity::SelfEntity);

    pub fn field(self, name: impl Into<Label>) -> Self {
        Self::FieldAccess(Box::new(self), name.into())
    }

    pub fn call_with(self, args: impl IntoIterator<Item = Self>) -> Self {
        Self::FunctionCall(Box::new(self), args.into_iter().collect())
    }

    pub fn call(self) -> Self {
        self.call_with(None)
    }

    pub fn call_method_with(
        self,
        name: impl Into<Label>,
        args: impl IntoIterator<Item = Self>,
    ) -> Self {
        self.field(name).call_with(args)
    }

    pub fn call_method(self, name: impl Into<Label>) -> Self {
        self.field(name).call()
    }
}

impl ToFragment for RustExpr {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustExpr::Entity(e) => e.to_fragment(),
            RustExpr::StringLit(s) => Fragment::Char('"')
                .cat(Fragment::String(s.clone()))
                .cat(Fragment::Char('"')),
            RustExpr::FieldAccess(x, name) => x
                .to_fragment()
                .intervene(Fragment::Char('.'), Fragment::String(name.clone())),
            RustExpr::FunctionCall(f, args) => f.to_fragment().cat(ToFragment::paren_list(args)),
            RustExpr::Tuple(elts) => Self::paren_list(elts),
            RustExpr::Struct(con, fields) => {
                let f_fields = Fragment::seq(
                    fields.iter().map(|(lab, expr)| {
                        Fragment::intervene(
                            Fragment::String(lab.clone()),
                            Fragment::string(": "),
                            expr.as_ref().map_or(Fragment::Empty, |x| x.to_fragment()),
                        )
                    }),
                    Some(Fragment::string(", ")),
                );
                Fragment::String(con.clone())
                    .cat(Fragment::Char(' '))
                    .cat(f_fields.delimit(Fragment::string("{ "), Fragment::string(" }")))
            }
            RustExpr::Paren(expr) => Self::paren_list([expr.as_ref()]),
            RustExpr::Borrow(expr) => Fragment::Char('&').cat(expr.to_fragment()),
            RustExpr::BorrowMut(expr) => Fragment::string("&mut ").cat(expr.to_fragment()),
            RustExpr::Try(expr) => expr.to_fragment().cat(Fragment::Char('?')),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustStmt {
    Let(Mut, Label, Option<RustType>, RustExpr),
    Expr(RustExpr),
    Return(bool, RustExpr), // bool: true for explicit return, false for implicit return
}

impl ToFragment for RustStmt {
    fn to_fragment(&self) -> Fragment {
        match self {
            Self::Let(_mut, binding, sig, value) => (match _mut {
                Mut::Mutable => Fragment::string("let mut "),
                Mut::Immutable => Fragment::string("let "),
            })
            .cat(Fragment::String(binding.clone()))
            .intervene(
                Fragment::string(": "),
                Fragment::opt(sig.as_ref(), RustType::to_fragment),
            )
            .cat(Fragment::string(" = "))
            .cat(value.to_fragment()),
            Self::Expr(expr) => expr.to_fragment().cat(Fragment::Char(';')),
            Self::Return(is_keyword, expr) => {
                let (before, after) = if *is_keyword {
                    (Fragment::String("return ".into()), Fragment::Char(';'))
                } else {
                    (Fragment::Empty, Fragment::Empty)
                };
                expr.to_fragment().delimit(before, after)
            }
        }
    }
}

pub trait ToFragment {
    fn to_fragment(&self) -> Fragment;

    fn paren_list<'a>(items: impl IntoIterator<Item = &'a Self>) -> Fragment
    where
        Self: 'a,
    {
        Fragment::seq(
            items.into_iter().map(Self::to_fragment),
            Some(Fragment::string(", ")),
        )
        .delimit(Fragment::Char('('), Fragment::Char(')'))
    }

    fn block<'a>(items: impl IntoIterator<Item = &'a Self>) -> Fragment
    where
        Self: 'a,
    {
        let lines = items.into_iter().map(Self::to_fragment);
        Fragment::seq(lines, Some(Fragment::Char('\n')))
            .delimit(Fragment::string("{\n"), Fragment::string("\n}"))
    }
}

impl<T> ToFragment for Box<T>
where
    T: ToFragment,
{
    fn to_fragment(&self) -> Fragment {
        self.as_ref().to_fragment()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn expect_fragment(value: &impl ToFragment, expected: &str) {
        assert_eq!(&format!("{}", value.to_fragment()), expected)
    }

    #[test]
    fn sample_type() {
        let rt = RustType::vec_of(RustType::anon_tuple([
            RustType::named("Label"),
            RustType::named("TypeRef"),
        ]));
        expect_fragment(&rt, "Vec<(Label, TypeRef)>");
    }

    #[test]
    fn sample_expr() {
        let re = RustExpr::SELF.call_method_with(
            "append",
            [RustExpr::BorrowMut(Box::new(RustExpr::local("other")))],
        );
        expect_fragment(&re, "self.append(&mut other)")
    }
}
