#![allow(dead_code)]

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
    imports: Vec<RustImport>,
    items: Vec<RustItem>,
}

impl FromIterator<RustItem> for RustProgram {
    fn from_iter<T: IntoIterator<Item = RustItem>>(iter: T) -> Self {
        Self {
            imports: Vec::new(),
            items: Vec::from_iter(iter),
        }
    }
}

impl RustProgram {
    pub fn add_import(&mut self, import: RustImport) {
        self.imports.push(import)
    }
}

impl ToFragment for RustProgram {
    fn to_fragment(&self) -> Fragment {
        let mut frags = FragmentBuilder::new();
        for import in self.imports.iter() {
            frags.push(import.to_fragment().cat_break());
        }

        if !self.imports.is_empty() {
            frags.push(Fragment::Empty.cat_break());
        }

        for item in self.items.iter() {
            frags.push(item.to_fragment().cat_break().cat_break());
        }
        frags.finalize()
    }
}

pub(crate) struct RustImport {
    pub(crate) path: Vec<Label>,
    pub(crate) uses: RustImportItems,
}

impl ToFragment for RustImport {
    fn to_fragment(&self) -> Fragment {
        let keyword = Fragment::string("use");
        let spec = Fragment::seq(
            self.path
                .iter()
                .cloned()
                .map(Fragment::String)
                .chain(std::iter::once(self.uses.to_fragment())),
            Some(Fragment::string("::")),
        );
        keyword
            .intervene(Fragment::Char(' '), spec)
            .cat(Fragment::Char(';'))
    }
}

pub(crate) enum RustImportItems {
    Wildcard,
    // One(Label),
    // Group(Vec<Label>),
}

impl ToFragment for RustImportItems {
    fn to_fragment(&self) -> Fragment {
        match self {
            Self::Wildcard => Fragment::Char('*'),
        }
    }
}

/// Top Level Item
pub(crate) struct RustItem {
    vis: Visibility,
    decl: RustDecl,
}

impl RustItem {
    pub fn from_decl(decl: RustDecl) -> Self {
        Self {
            vis: Default::default(),
            decl,
        }
    }
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
                .cat(aname.to_fragment())
                .cat(Fragment::string(" = "))
                .cat(rhs.to_fragment()),
            RustDecl::TypeDef(name, tdef) => {
                let frag_key = Fragment::string(tdef.keyword_for());
                Fragment::intervene(frag_key, Fragment::Char(' '), name.to_fragment())
                    .intervene(Fragment::Char(' '), tdef.to_fragment())
            }
            RustDecl::Function(fdef) => fdef.to_fragment(),
        }
    }
}
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RustParams<Lt, Ty> {
    lt_params: Vec<Lt>,
    ty_params: Vec<Ty>,
}

impl<Lt, Ty> Default for RustParams<Lt, Ty> {
    fn default() -> Self {
        Self {
            lt_params: Default::default(),
            ty_params: Default::default(),
        }
    }
}

pub(crate) type DefParams = RustParams<Label, Label>;
pub(crate) type UseParams = RustParams<RustLt, RustType>;

impl<Lt, Ty> RustParams<Lt, Ty> {
    pub fn new() -> Self {
        Self {
            lt_params: Vec::new(),
            ty_params: Vec::new(),
        }
    }
}

impl<Lt, Ty> RustParams<Lt, Ty> {
    pub fn push_lifetime(&mut self, lt: impl Into<Lt>) {
        self.lt_params.push(lt.into())
    }

    pub fn push_type(&mut self, t: impl Into<Ty>) {
        self.ty_params.push(t.into())
    }
}

impl ToFragment for RustParams<Label, Label> {
    fn to_fragment(&self) -> Fragment {
        let all = self.lt_params.iter().chain(self.ty_params.iter());
        Fragment::seq(all.map(Label::to_fragment), Some(Fragment::string(", ")))
            .delimit(Fragment::Char('<'), Fragment::Char('>'))
    }
}

impl ToFragment for RustParams<RustLt, RustType> {
    fn to_fragment(&self) -> Fragment {
        let all = self
            .lt_params
            .iter()
            .map(RustLt::to_fragment)
            .chain(self.ty_params.iter().map(RustType::to_fragment));
        Fragment::seq(all, Some(Fragment::string(", ")))
            .delimit(Fragment::Char('<'), Fragment::Char('>'))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FnSig {
    args: Vec<(Label, RustType)>,
    ret: Option<RustType>,
}

impl FnSig {
    pub fn new(args: Vec<(Label, RustType)>, ret: Option<RustType>) -> Self {
        Self { args, ret }
    }
}

impl ToFragment for (Label, RustType) {
    fn to_fragment(&self) -> Fragment {
        self.0
            .to_fragment()
            .intervene(Fragment::string(": "), self.1.to_fragment())
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
pub(crate) struct RustFn {
    name: Label,
    params: Option<DefParams>,
    sig: FnSig,
    body: Vec<RustStmt>,
}

impl RustFn {
    pub fn new(name: Label, params: Option<DefParams>, sig: FnSig, body: Vec<RustStmt>) -> Self {
        Self {
            name,
            params,
            sig,
            body,
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum RustTypeDef {
    Enum(Vec<RustVariant>),
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
                let iter = vars.iter().map(RustVariant::to_fragment);
                let inner = Fragment::seq(iter, Some(Fragment::string(", ")));
                inner.delimit(Fragment::string("{ "), Fragment::string(" }"))
            }
            RustTypeDef::Struct(str) => str.to_fragment(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum RustType {
    Atom(AtomType),
    Generic(Label),
    ImplTrait(Label),
    AnonTuple(Vec<RustType>),
    Verbatim(Label, UseParams), // Catch-all for generics that we may not be able or willing to hardcode
    SelfType,
}

impl RustType {
    pub const UNIT: RustType = RustType::Atom(AtomType::Prim(PrimType::Unit));

    pub fn imported(name: impl Into<Label>) -> Self {
        Self::Atom(AtomType::TypeRef(LocalType::External(name.into())))
    }

    pub fn defined(ix: usize, name: impl Into<Label>) -> Self {
        Self::Atom(AtomType::TypeRef(LocalType::LocalDef(ix, name.into())))
    }

    pub fn vec_of(inner: Self) -> Self {
        Self::Atom(AtomType::Comp(CompType::Vec(Box::new(inner))))
    }

    pub fn anon_tuple(elts: impl IntoIterator<Item = Self>) -> Self {
        Self::AnonTuple(elts.into_iter().collect())
    }

    pub fn verbatim(con: impl Into<Label>, params: Option<UseParams>) -> Self {
        Self::Verbatim(con.into(), params.unwrap_or_default())
    }

    pub fn option_of(inner: RustType) -> RustType {
        Self::Atom(AtomType::Comp(CompType::Option(Box::new(inner))))
    }
}

impl ToFragment for RustType {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustType::Atom(at) => at.to_fragment(),
            RustType::Generic(ident) => ident.to_fragment(),
            RustType::ImplTrait(ident) => {
                Fragment::cat(Fragment::string("impl "), ident.to_fragment())
            }
            RustType::AnonTuple(args) => {
                let inner = args.iter().map(|elt| elt.to_fragment());
                Fragment::seq(inner, Some(Fragment::string(", ")))
                    .delimit(Fragment::Char('('), Fragment::Char(')'))
            }
            RustType::Verbatim(con, params) => con.to_fragment().cat(params.to_fragment()),
            RustType::SelfType => Fragment::string("Self"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum RustStruct {
    Unit,
    Tuple(Vec<RustType>),
    Record(Vec<(Label, RustType)>),
}

impl ToFragment for RustStruct {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustStruct::Unit => Fragment::Empty,
            RustStruct::Tuple(args) => RustType::paren_list(args.iter()),
            RustStruct::Record(flds) => {
                <(Label, RustType)>::block_sep(flds.iter(), Fragment::Char(','))
            }
        }
    }
}

impl ToFragment for Label {
    fn to_fragment(&self) -> Fragment {
        Fragment::String(sanitize_label(self))
    }
}

pub(crate) fn sanitize_label(label: &Label) -> Label {
    if label.chars().enumerate().all(|(ix, c)| is_valid(ix, c)) {
        remap(label.clone())
    } else {
        remap(Label::from(replace_bad(label.as_ref())))
    }
}

fn remap(input: Label) -> Label {
    match input.as_ref() {
        "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn" | "else"
        | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop"
        | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self"
        | "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe" | "use"
        | "where" | "while" | "abstract" | "become" | "box" | "do" | "final" | "macro"
        | "override" | "priv" | "try" | "typeof" | "unsized" | "virtual" | "yield" => {
            Label::from(format!("r#{}", input))
        }
        _ => input,
    }
}

fn is_valid(ix: usize, c: char) -> bool {
    match c {
        '-' | '.' | ' ' | '\t' => false,
        '0'..='9' => ix != 0,
        _ => true,
    }
}

fn replace_bad(input: &str) -> String {
    let mut ret = String::new();
    let mut dashed = false;
    for c in input.chars() {
        if c.is_ascii_digit() && ret.is_empty() {
            ret.push('_');
            ret.push(c);
            dashed = false;
        } else if is_valid(ret.len(), c) {
            ret.push(c);
            dashed = false;
        } else if !dashed {
            ret.push('_');
            dashed = true;
        }
    }
    ret
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum RustVariant {
    Unit(Label),
    Tuple(Label, Vec<RustType>),
    Record(Label, Vec<(Label, RustType)>),
}

impl RustVariant {
    pub(crate) fn get_label(&self) -> &Label {
        match self {
            RustVariant::Unit(lab) | RustVariant::Tuple(lab, _) | RustVariant::Record(lab, _) => {
                lab
            }
        }
    }
}

impl ToFragment for RustVariant {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustVariant::Unit(lab) => lab.to_fragment(),
            RustVariant::Tuple(lab, args) => {
                lab.to_fragment().cat(RustType::paren_list(args.iter()))
            }
            RustVariant::Record(lab, flds) => lab.to_fragment().intervene(
                Fragment::Char(' '),
                <(Label, RustType)>::block_sep(flds.iter(), Fragment::Char(',')),
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum AtomType<T = Box<RustType>, U = T>
where
    T: Sized,
    U: Sized,
{
    TypeRef(LocalType),
    Prim(PrimType),
    Comp(CompType<T, U>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum LocalType {
    LocalDef(usize, Label),
    External(Label),
}

impl AsRef<Label> for LocalType {
    fn as_ref(&self) -> &Label {
        match self {
            LocalType::External(lab) | LocalType::LocalDef(_, lab) => lab,
        }
    }
}

impl LocalType {
    pub fn as_name(&self) -> &Label {
        self.as_ref()
    }

    pub fn try_as_index(&self) -> Option<usize> {
        match self {
            Self::LocalDef(ix, _) => Some(*ix),
            Self::External(_) => None,
        }
    }
}

impl ToFragment for LocalType {
    fn to_fragment(&self) -> Fragment {
        match self {
            Self::LocalDef(_, lab) | Self::External(lab) => lab.to_fragment(),
        }
    }
}

impl<T, U> ToFragment for AtomType<T, U>
where
    T: Sized + ToFragment,
    U: Sized + ToFragment,
{
    fn to_fragment(&self) -> Fragment {
        match self {
            AtomType::TypeRef(ltype) => ltype.to_fragment(),
            AtomType::Prim(pt) => pt.to_fragment(),
            AtomType::Comp(ct) => ct.to_fragment(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
            RustLt::Parametric(lab) => lab.to_fragment(),
        }
    }
}

/// Compound type that is either unary over `T` or binary over `T, U`.
///
/// If not specified, `U` will implicitly have the same type as `T`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum CompType<T = Box<RustType>, U = T> {
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
                    buf.push(Self::try_from(v)?);
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

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Hash)]
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
            RustEntity::Local(v) => v.to_fragment(),
            RustEntity::Scoped(path, v) => Fragment::seq(
                path.iter()
                    .chain(std::iter::once(v))
                    .map(|scope| scope.to_fragment()),
                Some(Fragment::string("::")),
            ),
            RustEntity::SelfEntity => Fragment::string("self"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SubIdent {
    ByIndex(usize),
    ByName(Label),
}

impl ToFragment for SubIdent {
    fn to_fragment(&self) -> Fragment {
        match self {
            SubIdent::ByIndex(ix) => Fragment::DisplayAtom(Rc::new(*ix)),
            SubIdent::ByName(lab) => lab.to_fragment(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RustPrimLit {
    BooleanLit(bool),
    NumericLit(usize),
    StringLit(Label),
}

impl ToFragment for RustPrimLit {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustPrimLit::BooleanLit(b) => Fragment::DisplayAtom(Rc::new(*b)),
            RustPrimLit::NumericLit(n) => Fragment::DisplayAtom(Rc::new(*n)),
            RustPrimLit::StringLit(s) => Fragment::String(s.clone())
                .delimit(Fragment::string("r#\""), Fragment::string("\"#")),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RustExpr {
    Entity(RustEntity),
    PrimitiveLit(RustPrimLit),
    ArrayLit(Vec<RustExpr>),
    FieldAccess(Box<RustExpr>, SubIdent), // can be used for receiver methods as well, with FunctionCall
    FunctionCall(Box<RustExpr>, Vec<RustExpr>), // can be used for tuple constructors as well
    Tuple(Vec<RustExpr>),
    Struct(Label, Vec<(Label, Option<Box<RustExpr>>)>),
    Paren(Box<RustExpr>),
    Borrow(Box<RustExpr>),
    BorrowMut(Box<RustExpr>),
    Try(Box<RustExpr>),
    Operation(RustOp),
    BlockScope(Vec<RustStmt>, Box<RustExpr>), // scoped block with a final value as an implicit return
    Control(Box<RustControl>),                // for control blocks that return a value
}

#[derive(Debug, Clone)]
pub(crate) enum RustOp {
    // scaffolding to allow for flexible infix operations from operator tokens; should contain spaces already
    InfixOp(&'static str, Box<RustExpr>, Box<RustExpr>),
}

impl RustOp {
    pub fn op_eq(lhs: RustExpr, rhs: RustExpr) -> Self {
        Self::InfixOp(" == ", Box::new(lhs), Box::new(rhs))
    }

    pub fn op_neq(lhs: RustExpr, rhs: RustExpr) -> Self {
        Self::InfixOp(" != ", Box::new(lhs), Box::new(rhs))
    }
}

impl ToFragment for RustOp {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustOp::InfixOp(op, lhs, rhs) => lhs
                .to_fragment()
                .intervene(Fragment::string(*op), rhs.to_fragment()),
        }
    }
}

impl RustExpr {
    pub const UNIT: Self = Self::Tuple(Vec::new());

    pub const NONE: Self = Self::Entity(RustEntity::Local(Label::Borrowed("None")));

    pub const TRUE: Self = Self::PrimitiveLit(RustPrimLit::BooleanLit(true));

    pub const FALSE: Self = Self::PrimitiveLit(RustPrimLit::BooleanLit(false));

    pub fn some(inner: Self) -> Self {
        Self::local("Some").call_with([inner])
    }

    pub fn local(name: impl Into<Label>) -> Self {
        Self::Entity(RustEntity::Local(name.into()))
    }

    pub fn num_lit<N: Into<usize>>(num: N) -> Self {
        Self::PrimitiveLit(RustPrimLit::NumericLit(num.into()))
    }

    pub fn scoped<Name: Into<Label>>(
        path: impl IntoIterator<Item = Name>,
        name: impl Into<Label>,
    ) -> Self {
        let lpath = path.into_iter().map(|x| x.into()).collect::<Vec<Label>>();
        Self::Entity(RustEntity::Scoped(lpath, name.into()))
    }

    pub const SELF: Self = Self::Entity(RustEntity::SelfEntity);

    pub fn field(self, name: impl Into<Label>) -> Self {
        Self::FieldAccess(Box::new(self), SubIdent::ByName(name.into()))
    }

    pub fn nth(self, ix: usize) -> Self {
        Self::FieldAccess(Box::new(self), SubIdent::ByIndex(ix))
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

    pub fn infix(lhs: Self, op: &'static str, rhs: Self) -> Self {
        Self::Operation(RustOp::InfixOp(op, Box::new(lhs), Box::new(rhs)))
    }

    pub fn wrap_try(self) -> Self {
        Self::Try(Box::new(self))
    }

    pub fn str_lit(str: impl Into<Label>) -> Self {
        Self::PrimitiveLit(RustPrimLit::StringLit(str.into()))
    }
}

impl ToFragment for RustExpr {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustExpr::Entity(e) => e.to_fragment(),
            RustExpr::PrimitiveLit(pl) => pl.to_fragment(),
            RustExpr::ArrayLit(elts) => Fragment::seq(
                elts.iter().map(RustExpr::to_fragment),
                Some(Fragment::string(", ")),
            )
            .delimit(Fragment::Char('['), Fragment::Char(']')),
            RustExpr::FieldAccess(x, name) => x
                .to_fragment()
                .intervene(Fragment::Char('.'), name.to_fragment()),
            RustExpr::FunctionCall(f, args) => f.to_fragment().cat(ToFragment::paren_list(args)),
            RustExpr::Tuple(elts) => Self::paren_list(elts),
            RustExpr::Struct(con, fields) => {
                let f_fields = Fragment::seq(
                    fields.iter().map(|(lab, expr)| {
                        Fragment::intervene(
                            lab.to_fragment(),
                            Fragment::string(": "),
                            expr.as_ref().map_or(Fragment::Empty, |x| x.to_fragment()),
                        )
                    }),
                    Some(Fragment::string(", ")),
                );
                con.to_fragment()
                    .cat(Fragment::Char(' '))
                    .cat(f_fields.delimit(Fragment::string("{ "), Fragment::string(" }")))
            }
            RustExpr::Paren(expr) => Self::paren_list([expr.as_ref()]),
            RustExpr::Borrow(expr) => Fragment::Char('&').cat(expr.to_fragment()),
            RustExpr::BorrowMut(expr) => Fragment::string("&mut ").cat(expr.to_fragment()),
            RustExpr::Try(expr) => expr.to_fragment().cat(Fragment::Char('?')),
            RustExpr::Operation(op) => op.to_fragment(),
            RustExpr::BlockScope(stmts, val) => RustStmt::block(stmts.iter().chain(
                std::iter::once(&RustStmt::Return(false, val.as_ref().clone())),
            )),
            RustExpr::Control(ctrl) => ctrl.to_fragment(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustStmt {
    Let(Mut, Label, Option<RustType>, RustExpr),
    Reassign(Label, RustExpr),
    #[allow(dead_code)]
    Expr(RustExpr),
    Return(bool, RustExpr), // bool: true for explicit return, false for implicit return
    Control(RustControl),
}

impl RustStmt {
    pub fn assign(name: impl Into<Label>, rhs: RustExpr) -> Self {
        Self::Let(Mut::Immutable, name.into(), None, rhs)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustControl {
    While(RustExpr, Vec<RustStmt>),
    If(RustExpr, Vec<RustStmt>, Option<Vec<RustStmt>>),
    Match(RustExpr, Vec<(RustPattern, Vec<RustStmt>)>),
}

#[derive(Clone, Debug)]
pub(crate) enum RustPattern {
    NumLiteral(usize),
    CatchAll(Option<Label>), // None <- `_`, Some("x") for `x`
}

impl ToFragment for RustPattern {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustPattern::NumLiteral(n) => Fragment::DisplayAtom(Rc::new(*n)),
            RustPattern::CatchAll(None) => Fragment::Char('_'),
            RustPattern::CatchAll(Some(lab)) => Fragment::String(lab.clone()),
        }
    }
}

impl ToFragment for RustControl {
    fn to_fragment(&self) -> Fragment {
        match self {
            Self::While(cond, body) => Fragment::string("while")
                .intervene(Fragment::Char(' '), cond.to_fragment())
                .intervene(Fragment::Char(' '), RustStmt::block(body.iter())),
            Self::If(cond, b_then, b_else) => Fragment::string("if")
                .intervene(Fragment::Char(' '), cond.to_fragment())
                .intervene(Fragment::Char(' '), RustStmt::block(b_then.iter()))
                .intervene(
                    Fragment::string(" else "),
                    Fragment::opt(b_else.as_ref(), |branch| RustStmt::block(branch.iter())),
                ),
            Self::Match(expr, cases) => Fragment::string("match")
                .intervene(Fragment::Char(' '), expr.to_fragment())
                .intervene(
                    Fragment::Char(' '),
                    <(RustPattern, Vec<RustStmt>)>::block_sep(cases, Fragment::string(",\n")),
                ),
        }
    }
}

impl ToFragment for (RustPattern, Vec<RustStmt>) {
    fn to_fragment(&self) -> Fragment {
        self.0
            .to_fragment()
            .intervene(Fragment::string(" => "), RustStmt::block(self.1.iter()))
    }
}

impl ToFragment for RustStmt {
    fn to_fragment(&self) -> Fragment {
        match self {
            Self::Let(_mut, binding, sig, value) => (match _mut {
                Mut::Mutable => Fragment::string("let mut "),
                Mut::Immutable => Fragment::string("let "),
            })
            .cat(binding.to_fragment())
            .intervene(
                Fragment::string(": "),
                Fragment::opt(sig.as_ref(), RustType::to_fragment),
            )
            .cat(Fragment::string(" = "))
            .cat(value.to_fragment())
            .cat(Fragment::Char(';')),
            Self::Reassign(lab, expr) => lab
                .to_fragment()
                .cat(Fragment::string(" = "))
                .cat(expr.to_fragment())
                .cat(Fragment::Char(';')),
            Self::Expr(expr) => expr.to_fragment().cat(Fragment::Char(';')),
            Self::Return(is_keyword, expr) => {
                let (before, after) = if *is_keyword {
                    (Fragment::String("return ".into()), Fragment::Char(';'))
                } else {
                    (Fragment::Empty, Fragment::Empty)
                };
                expr.to_fragment().delimit(before, after)
            }
            Self::Control(ctrl) => ctrl.to_fragment(),
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
        Self::block_sep(items, Fragment::Empty)
    }

    fn block_sep<'a>(items: impl IntoIterator<Item = &'a Self>, sep: Fragment) -> Fragment
    where
        Self: 'a,
    {
        let lines = items.into_iter().map(Self::to_fragment);
        Fragment::seq(lines, Some(Fragment::cat(sep, Fragment::Char('\n'))))
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
            RustType::imported("Label"),
            RustType::imported("TypeRef"),
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
