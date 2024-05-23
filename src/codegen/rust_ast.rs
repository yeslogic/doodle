use std::cmp::Ordering;
use std::rc::Rc;

use crate::output::{Fragment, FragmentBuilder};

use crate::precedence::{cond_paren, Precedence};
use crate::{BaseType, IntoLabel, Label, ValueType};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) enum Visibility {
    #[default]
    Implicit,
}

impl Visibility {
    fn add_vis(&self, item: Fragment) -> Fragment {
        match self {
            Self::Implicit => item,
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
    TypeDef(Label, RustTypeDef),
    Function(RustFn),
}

impl RustDecl {
    pub fn to_fragment(&self) -> Fragment {
        match self {
            RustDecl::TypeDef(name, tdef) => {
                let frag_key = Fragment::string(tdef.keyword_for());
                let def = Fragment::intervene(frag_key, Fragment::Char(' '), name.to_fragment())
                    .intervene(Fragment::Char(' '), tdef.to_fragment());
                // FIXME - this is a hack to allow debug in adhoc-typed non-exhaustive matches
                Fragment::intervene(
                    Fragment::string("#[derive(Debug, Clone)]"),
                    Fragment::Char('\n'),
                    def,
                )
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
    AnonTuple(Vec<RustType>),
    Verbatim(Label, UseParams), // Catch-all for generics that we may not be able or willing to hardcode
}

impl RustType {
    pub const UNIT: RustType = RustType::Atom(AtomType::Prim(PrimType::Unit));

    /// Returns the RustType representation of an externally-defined and imported type `<name>`.
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

    pub fn borrow_of(lt: Option<RustLt>, m: Mut, ty: RustType) -> Self {
        Self::Atom(AtomType::Comp(CompType::Borrow(lt, m, Box::new(ty))))
    }

    pub fn result_of(ok_type: RustType, err_type: RustType) -> RustType {
        Self::Atom(AtomType::Comp(CompType::Result(
            Box::new(ok_type),
            Box::new(err_type),
        )))
    }

    fn try_as_primtype(&self) -> Option<PrimType> {
        match self {
            RustType::Atom(at) => match at {
                AtomType::Prim(pt) => Some(*pt),
                _ => None,
            },
            _ => None,
        }
    }
}

impl ToFragment for RustType {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustType::Atom(at) => at.to_fragment(),
            RustType::AnonTuple(args) => {
                let inner = args.iter().map(|elt| elt.to_fragment());
                let mut elems = Fragment::seq(inner, Some(Fragment::string(", ")));
                // NOTE - Rust 1-tuples need an explicit ',' after the sole element
                if args.len() == 1 {
                    elems.encat(Fragment::Char(','));
                }
                elems.delimit(Fragment::Char('('), Fragment::Char(')'))
            }
            RustType::Verbatim(con, params) => con.to_fragment().cat(params.to_fragment()),
        }
    }
}

impl ToFragmentExt for RustType {
    // FIXME - this impl is only to fix test cases
    fn to_fragment_precedence(&self, _prec: Precedence) -> Fragment {
        self.to_fragment()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum RustStruct {
    Record(Vec<(Label, RustType)>),
}

impl ToFragment for RustStruct {
    fn to_fragment(&self) -> Fragment {
        match self {
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
}

impl RustVariant {
    pub(crate) fn get_label(&self) -> &Label {
        match self {
            RustVariant::Unit(lab) | RustVariant::Tuple(lab, _) => lab,
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
    U64,
    Bool,
    Char,
    Usize,
}

impl PrimType {
    fn is_numeric(&self) -> bool {
        matches!(
            self,
            PrimType::U8 | PrimType::U16 | PrimType::U32 | PrimType::U64 | PrimType::Usize
        )
    }

    fn compare_width(pt0: PrimType, pt1: PrimType) -> Option<Ordering> {
        match (pt0, pt1) {
            (PrimType::Unit, _) | (_, PrimType::Unit) => None,
            (PrimType::Char, _) | (_, PrimType::Char) => None,
            (PrimType::Bool, _) | (_, PrimType::Bool) => None,
            (PrimType::U8, PrimType::U8) => Some(Ordering::Equal),
            (PrimType::U8, _) => Some(Ordering::Less),
            (_, PrimType::U8) => Some(Ordering::Greater),
            (PrimType::U16, PrimType::U16) => Some(Ordering::Equal),
            (PrimType::U16, _) => Some(Ordering::Less),
            (_, PrimType::U16) => Some(Ordering::Greater),
            (PrimType::U32, PrimType::U32) => Some(Ordering::Equal),
            (PrimType::U32, _) => Some(Ordering::Less),
            (_, PrimType::U32) => Some(Ordering::Greater),
            (PrimType::U64 | PrimType::Usize, PrimType::U64 | PrimType::Usize) => {
                Some(Ordering::Equal)
            }
        }
    }
}

impl From<BaseType> for PrimType {
    fn from(value: BaseType) -> Self {
        match value {
            BaseType::Bool => PrimType::Bool,
            BaseType::U8 => PrimType::U8,
            BaseType::U16 => PrimType::U16,
            BaseType::U32 => PrimType::U32,
            BaseType::U64 => PrimType::U64,
            BaseType::Char => PrimType::Char,
        }
    }
}

impl ToFragment for PrimType {
    fn to_fragment(&self) -> Fragment {
        Fragment::string(match self {
            PrimType::Unit => "()",
            PrimType::U8 => "u8",
            PrimType::U16 => "u16",
            PrimType::U32 => "u32",
            PrimType::U64 => "u64",
            PrimType::Bool => "bool",
            PrimType::Char => "char",
            PrimType::Usize => "usize",
        })
    }
}

/// AST type for Rust Lifetimes
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustLt {
    /// Label contents should contain leading `'`
    Parametric(Label),
}

impl ToFragment for RustLt {
    fn to_fragment(&self) -> Fragment {
        match self {
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
    Result(T, U),
    Borrow(Option<RustLt>, Mut, T),
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
            ValueType::Base(BaseType::Bool) => Ok(PrimType::Bool.into()),
            ValueType::Base(BaseType::U8) => Ok(PrimType::U8.into()),
            ValueType::Base(BaseType::U16) => Ok(PrimType::U16.into()),
            ValueType::Base(BaseType::U32) => Ok(PrimType::U32.into()),
            ValueType::Base(BaseType::U64) => Ok(PrimType::U64.into()),
            ValueType::Base(BaseType::Char) => Ok(PrimType::Char.into()),
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
    Boolean(bool),
    Numeric(RustNumLit),
    Char(char),
    String(Label),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum RustNumLit {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
}

impl ToFragment for RustNumLit {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustNumLit::U8(n) => Fragment::string(format!("{n}u8")),
            RustNumLit::U16(n) => Fragment::string(format!("{n}u16")),
            RustNumLit::U32(n) => Fragment::string(format!("{n}u32")),
            RustNumLit::U64(n) => Fragment::string(format!("{n}u64")),
            RustNumLit::Usize(n) => Fragment::string(format!("{n}")),
        }
    }
}

impl ToFragment for RustPrimLit {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustPrimLit::Boolean(b) => Fragment::DisplayAtom(Rc::new(*b)),
            RustPrimLit::Numeric(n) => n.to_fragment(),
            RustPrimLit::Char(c) => Fragment::DisplayAtom(Rc::new(*c))
                .delimit(Fragment::Char('\''), Fragment::Char('\'')),
            RustPrimLit::String(s) => Fragment::String(s.clone())
                .delimit(Fragment::string("r#\""), Fragment::string("\"#")),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RustExpr {
    Entity(RustEntity),
    PrimitiveLit(RustPrimLit),
    ArrayLit(Vec<RustExpr>),
    MethodCall(Box<RustExpr>, SubIdent, Vec<RustExpr>), // used for specifically calling methods to assign a constant precedence to avoid parenthetical nesting
    FieldAccess(Box<RustExpr>, SubIdent), // can be used for receiver methods as well, with FunctionCall
    FunctionCall(Box<RustExpr>, Vec<RustExpr>), // can be used for tuple constructors as well
    Tuple(Vec<RustExpr>),
    Struct(RustEntity, Vec<(Label, Option<Box<RustExpr>>)>),
    Deref(Box<RustExpr>),
    Borrow(Box<RustExpr>),
    BorrowMut(Box<RustExpr>),
    Try(Box<RustExpr>),
    Operation(RustOp),
    BlockScope(Vec<RustStmt>, Box<RustExpr>), // scoped block with a final value as an implicit return
    Control(Box<RustControl>),                // for control blocks that return a value
    Closure(RustClosure),                     // only simple lambdas for now
    Slice(Box<RustExpr>, Box<RustExpr>, Box<RustExpr>), // object, start ix, end ix (exclusive)
    RangeExclusive(Box<RustExpr>, Box<RustExpr>),
}

#[derive(Clone, Debug)]
pub(crate) struct RustClosure(RustClosureHead, ClosureBody);

#[derive(Clone, Debug)]
pub(crate) enum ClosureBody {
    Expression(Box<RustExpr>),
    Statements(Vec<RustStmt>),
}

impl ToFragmentExt for ClosureBody {
    fn to_fragment_precedence(&self, prec: Precedence) -> Fragment {
        match self {
            ClosureBody::Expression(expr) => expr.to_fragment_precedence(prec),
            ClosureBody::Statements(..) => self.to_fragment(),
        }
    }
}

impl ToFragment for ClosureBody {
    fn to_fragment(&self) -> Fragment {
        match self {
            ClosureBody::Expression(expr) => expr.to_fragment_precedence(Precedence::TOP),
            ClosureBody::Statements(stmts) => <RustStmt as ToFragment>::block(stmts),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustClosureHead {
    Thunk,
    SimpleVar(Label, Option<RustType>),
}

impl RustClosure {
    pub fn thunk_expr(expr: RustExpr) -> RustClosure {
        RustClosure(
            RustClosureHead::Thunk,
            ClosureBody::Expression(Box::new(expr)),
        )
    }

    pub fn thunk_body(body: impl IntoIterator<Item = RustStmt>) -> RustClosure {
        RustClosure(
            RustClosureHead::Thunk,
            ClosureBody::Statements(Vec::from_iter(body)),
        )
    }

    /// Constructs a new closure with 'predicate' (ref) semantics.
    pub fn new_predicate(
        head: impl IntoLabel,
        deref_t: Option<RustType>,
        body: RustExpr,
    ) -> RustClosure {
        RustClosure(
            RustClosureHead::SimpleVar(
                head.into(),
                deref_t.map(|ty| RustType::borrow_of(None, Mut::Immutable, ty)),
            ),
            ClosureBody::Expression(Box::new(body)),
        )
    }

    /// Constructs a new closure with 'transform' (value) semantics
    pub fn new_transform(
        head: impl IntoLabel,
        value_t: Option<RustType>,
        body: RustExpr,
    ) -> RustClosure {
        RustClosure(
            RustClosureHead::SimpleVar(head.into(), value_t),
            ClosureBody::Expression(Box::new(body)),
        )
    }
}

impl ToFragment for RustClosureHead {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustClosureHead::Thunk => Fragment::string("||"),
            RustClosureHead::SimpleVar(lbl, sig) => lbl
                .to_fragment()
                .intervene(
                    Fragment::string(": "),
                    Fragment::opt(sig.as_ref(), RustType::to_fragment),
                )
                .delimit(Fragment::Char('|'), Fragment::Char('|')),
        }
    }
}

impl ToFragment for RustClosure {
    fn to_fragment(&self) -> Fragment {
        self.to_fragment_precedence(Precedence::ARROW)
    }
}

impl ToFragmentExt for RustClosure {
    fn to_fragment_precedence(&self, prec: Precedence) -> Fragment {
        match self {
            RustClosure(head, body) => cond_paren(
                head.to_fragment().intervene(
                    Fragment::Char(' '),
                    body.to_fragment_precedence(Precedence::ARROW),
                ),
                prec,
                Precedence::ARROW,
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Operator {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Div,
    Rem,
    Add,
    Sub,
    Mul,
    Shl,
    Shr,
    BitOr,
    BitAnd,
}

impl Operator {
    pub(crate) fn precedence(&self) -> Precedence {
        match self {
            Operator::Eq | Operator::Neq => Precedence::EQUALITY,
            Operator::Lt | Operator::Lte | Operator::Gt | Operator::Gte => Precedence::COMPARE,
            Operator::Div | Operator::Rem => Precedence::DIVREM,
            Operator::Add | Operator::Sub => Precedence::ADDSUB,
            Operator::Mul => Precedence::MUL,
            Operator::Shl | Operator::Shr => Precedence::BITSHIFT,
            Operator::BitOr => Precedence::BITOR,
            Operator::BitAnd => Precedence::BITAND,
        }
    }

    pub(crate) fn out_type(&self, lhs_type: PrimType, rhs_type: PrimType) -> Option<PrimType> {
        match self {
            Operator::Eq | Operator::Neq => {
                if lhs_type == rhs_type {
                    Some(PrimType::Bool)
                } else {
                    None
                }
            }
            Operator::Lt | Operator::Lte | Operator::Gt | Operator::Gte => {
                if lhs_type == rhs_type && lhs_type.is_numeric() {
                    Some(PrimType::Bool)
                } else {
                    None
                }
            }
            Operator::BitOr
            | Operator::BitAnd
            | Operator::Div
            | Operator::Rem
            | Operator::Add
            | Operator::Sub
            | Operator::Mul => {
                if lhs_type == rhs_type && lhs_type.is_numeric() {
                    Some(lhs_type)
                } else {
                    None
                }
            }
            Operator::Shl | Operator::Shr => {
                if lhs_type.is_numeric() && rhs_type.is_numeric() {
                    Some(lhs_type)
                } else {
                    None
                }
            }
        }
    }
}

impl Operator {
    pub(crate) fn token(&self) -> &'static str {
        match self {
            Operator::Eq => " == ",
            Operator::Neq => " != ",
            Operator::Lt => " < ",
            Operator::Lte => " <= ",
            Operator::Gt => " > ",
            Operator::Gte => " >= ",
            Operator::Div => " / ",
            Operator::Rem => " % ",
            Operator::Add => " + ",
            Operator::Sub => " - ",
            Operator::Mul => " * ",
            Operator::Shl => " << ",
            Operator::Shr => " >> ",
            Operator::BitOr => " | ",
            Operator::BitAnd => " & ",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RustOp {
    // scaffolding to allow for flexible infix operations from operator tokens; should contain spaces already
    InfixOp(Operator, Box<RustExpr>, Box<RustExpr>),
    AsCast(Box<RustExpr>, RustType),
}

impl RustOp {
    pub(crate) fn precedence(&self) -> Precedence {
        match self {
            Self::InfixOp(op, _, _) => op.precedence(),
            Self::AsCast(_, _) => Precedence::CAST_INFIX,
        }
    }

    /// Basic heuristic to determine whether a given operation is 'sound' at the type-level, i.e.
    /// that the operation in question is defined on the type of the operands and that the operands conform
    /// to the expectations of the operation, and are homogenous if that is required.
    ///
    /// If the operation might be unsound, may conservatively return false even if soundness is not ruled out.
    pub fn is_sound(&self) -> bool {
        match self {
            RustOp::InfixOp(op, lhs, rhs) => {
                match (op, lhs.try_get_primtype(), rhs.try_get_primtype()) {
                    (Operator::Eq | Operator::Neq, Some(ltype), Some(rtype)) => ltype == rtype,
                    (_, Some(ltype), Some(rtype)) => ltype == rtype && ltype.is_numeric(),
                    (_, None, _) | (_, _, None) => false,
                }
            }
            RustOp::AsCast(expr, typ) => match (expr.try_get_primtype(), typ.try_as_primtype()) {
                (Some(pt0), Some(pt1)) => match PrimType::compare_width(pt0, pt1) {
                    None | Some(Ordering::Greater) => false,
                    _ => true,
                },
                _ => false,
            },
        }
    }
}

impl RustOp {
    pub fn op_eq(lhs: RustExpr, rhs: RustExpr) -> Self {
        Self::InfixOp(Operator::Eq, Box::new(lhs), Box::new(rhs))
    }

    pub fn op_neq(lhs: RustExpr, rhs: RustExpr) -> Self {
        Self::InfixOp(Operator::Neq, Box::new(lhs), Box::new(rhs))
    }
}

impl ToFragmentExt for RustOp {
    fn to_fragment_precedence(&self, prec: Precedence) -> Fragment {
        let inherent = self.precedence();
        match self {
            RustOp::InfixOp(op, lhs, rhs) => cond_paren(
                lhs.to_fragment_precedence(inherent)
                    .cat(Fragment::string(op.token()))
                    .cat(rhs.to_fragment_precedence(inherent)),
                prec,
                inherent,
            ),
            RustOp::AsCast(expr, typ) => cond_paren(
                expr.to_fragment()
                    .intervene(Fragment::string(" as "), typ.to_fragment()),
                prec,
                inherent,
            ),
        }
    }
}

impl ToFragment for RustOp {
    fn to_fragment(&self) -> Fragment {
        self.to_fragment_precedence(Precedence::ATOM)
    }
}

impl RustExpr {
    pub const UNIT: Self = Self::Tuple(Vec::new());

    pub const NONE: Self = Self::Entity(RustEntity::Local(Label::Borrowed("None")));

    pub const TRUE: Self = Self::PrimitiveLit(RustPrimLit::Boolean(true));

    pub const FALSE: Self = Self::PrimitiveLit(RustPrimLit::Boolean(false));

    pub fn some(inner: Self) -> Self {
        Self::local("Some").call_with([inner])
    }

    pub fn local(name: impl Into<Label>) -> Self {
        Self::Entity(RustEntity::Local(name.into()))
    }

    pub fn num_lit<N: Into<usize>>(num: N) -> Self {
        Self::PrimitiveLit(RustPrimLit::Numeric(RustNumLit::Usize(num.into())))
    }

    pub fn u8lit(num: u8) -> Self {
        Self::PrimitiveLit(RustPrimLit::Numeric(RustNumLit::U8(num)))
    }

    pub fn u16lit(num: u16) -> Self {
        Self::PrimitiveLit(RustPrimLit::Numeric(RustNumLit::U16(num)))
    }

    pub fn u32lit(num: u32) -> Self {
        Self::PrimitiveLit(RustPrimLit::Numeric(RustNumLit::U32(num)))
    }

    pub fn u64lit(num: u64) -> RustExpr {
        Self::PrimitiveLit(RustPrimLit::Numeric(RustNumLit::U64(num)))
    }

    pub fn scoped<Name: Into<Label>>(
        path: impl IntoIterator<Item = Name>,
        name: impl Into<Label>,
    ) -> Self {
        let lpath = path.into_iter().map(|x| x.into()).collect::<Vec<Label>>();
        Self::Entity(RustEntity::Scoped(lpath, name.into()))
    }

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
        RustExpr::MethodCall(
            Box::new(self),
            SubIdent::ByName(name.into()),
            args.into_iter().collect(),
        )
    }

    pub fn call_method(self, name: impl Into<Label>) -> Self {
        self.call_method_with(name, None)
    }

    pub fn infix(lhs: Self, op: Operator, rhs: Self) -> Self {
        Self::Operation(RustOp::InfixOp(op, Box::new(lhs), Box::new(rhs)))
    }

    pub fn wrap_try(self) -> Self {
        Self::Try(Box::new(self))
    }

    pub fn str_lit(str: impl Into<Label>) -> Self {
        Self::PrimitiveLit(RustPrimLit::String(str.into()))
    }

    pub fn err(err_val: RustExpr) -> RustExpr {
        Self::local("Err").call_with([err_val])
    }

    pub fn try_get_primtype(&self) -> Option<PrimType> {
        match self {
            RustExpr::Entity(_) => None,
            RustExpr::PrimitiveLit(p_lit) => match p_lit {
                RustPrimLit::Boolean(..) => Some(PrimType::Bool),
                RustPrimLit::Numeric(n_lit) => match n_lit {
                    RustNumLit::U8(..) => Some(PrimType::U8),
                    RustNumLit::U16(..) => Some(PrimType::U16),
                    RustNumLit::U32(..) => Some(PrimType::U32),
                    RustNumLit::U64(..) => Some(PrimType::U64),
                    RustNumLit::Usize(..) => Some(PrimType::Usize),
                },
                RustPrimLit::Char(..) => Some(PrimType::Char),
                RustPrimLit::String(..) => None,
            },
            RustExpr::ArrayLit(..) => None,
            RustExpr::MethodCall(_obj, _method, _vars) => {
                match _method {
                    SubIdent::ByIndex(_) => {
                        unreachable!("unexpected method call using numeric subident")
                    }
                    SubIdent::ByName(name) => {
                        // REVIEW - is this the best idea?
                        if name.as_ref() == "len" && _vars.is_empty() {
                            Some(PrimType::Usize)
                        } else {
                            None
                        }
                    }
                }
            }
            // FIXME - this is complicated enough we won't bother implementing anything for this for now
            RustExpr::FieldAccess(..) => None,
            // FIXME - there may be some functions we can predict the return values of, but for now we can leave this alone
            RustExpr::FunctionCall(..) => None,
            RustExpr::Tuple(tuple) => match &tuple[..] {
                [] => Some(PrimType::Unit),
                [x] => x.try_get_primtype(),
                [_, ..] => None,
            },
            RustExpr::Struct(..) => None,
            RustExpr::Deref(x) => match &**x {
                // FIXME - intervening parentheses (RustExpr::Paren) break this match but we can't do much about that.
                RustExpr::Borrow(y) | RustExpr::BorrowMut(y) => y.try_get_primtype(),
                _ => None,
            },
            RustExpr::Borrow(_) | RustExpr::BorrowMut(_) => None,
            RustExpr::Try(..) => None,
            RustExpr::Operation(op) => match op {
                RustOp::InfixOp(op, lhs, rhs) => {
                    match (lhs.try_get_primtype(), rhs.try_get_primtype()) {
                        (Some(lhs_type), Some(rhs_type)) => op.out_type(lhs_type, rhs_type),
                        _ => None,
                    }
                }
                RustOp::AsCast(expr, typ) => {
                    let Some(out_typ) = typ.try_as_primtype() else {
                        return None;
                    };
                    if expr
                        .try_get_primtype()
                        .as_ref()
                        .map_or(false, PrimType::is_numeric)
                        && out_typ.is_numeric()
                    {
                        Some(out_typ)
                    } else {
                        None
                    }
                }
            },
            RustExpr::BlockScope(_stmts, ret) => ret.try_get_primtype(),
            RustExpr::Control(..)
            | RustExpr::Closure(..)
            | RustExpr::Slice(..)
            | RustExpr::RangeExclusive(..) => None,
        }
    }

    /// Basic heuristic to determine whether a `RustExpr` is free of any side-effects, and therefore can be fully elided
    /// if its direct evaluation would be immediately discarded (as with a RustStmt::Expr or RustStmt::Let over the `_` identifier).
    pub fn is_pure(&self) -> bool {
        match self {
            RustExpr::Entity(..) => true,
            RustExpr::PrimitiveLit(..) => true,
            RustExpr::ArrayLit(arr) => arr.iter().all(Self::is_pure),
            // NOTE - there is no guaranteed-accurate static heuristic to distinguish pure fn's from those with possible side-effects
            RustExpr::FunctionCall(..) | RustExpr::MethodCall(..) => false,
            RustExpr::FieldAccess(expr, ..) => expr.is_pure(),
            RustExpr::Tuple(tuple) => tuple.iter().all(Self::is_pure),
            RustExpr::Struct(_, assigns) => assigns
                .iter()
                .all(|(_, val)| val.as_deref().map_or(true, Self::is_pure)),
            RustExpr::Deref(expr) | RustExpr::Borrow(expr) | RustExpr::BorrowMut(expr) => {
                expr.is_pure()
            }
            // NOTE - Without static analysis to determine whether the value is always Some(x) where x is a pure calculation, try can always cause the block-scope to short-circuit to Err and therefore is impure by default
            RustExpr::Try(..) => false,
            RustExpr::Operation(op) => match op {
                RustOp::InfixOp(.., lhs, rhs) => lhs.is_pure() && rhs.is_pure() && op.is_sound(),
                // NOTE - illegal casts like `x as u8` where x >= 256 are language-level errors that are neither pure nor impure
                RustOp::AsCast(expr, ..) => expr.is_pure() && op.is_sound(),
            },
            RustExpr::BlockScope(stmts, tail) => stmts.is_empty() && tail.is_pure(),
            // NOTE - there may be some pure control expressions but those will be relatively rare as natural occurrences
            RustExpr::Control(..) => false,
            RustExpr::Closure(..) => false,
            // NOTE - slices exprs can always be out-of-bounds so they cannot be elided without changing program behavior
            RustExpr::Slice(..) => false,
            // NOTE - ranges can be inverted
            RustExpr::RangeExclusive(..) => false,
        }
    }
}

impl ToFragmentExt for RustExpr {
    // REVIEW - make sure we aren't leaving anything by the wayside
    fn to_fragment_precedence(&self, prec: Precedence) -> Fragment {
        match self {
            RustExpr::Entity(e) => e.to_fragment(),
            RustExpr::PrimitiveLit(pl) => pl.to_fragment(),
            RustExpr::ArrayLit(elts) => Fragment::seq(
                elts.iter()
                    .map(|x| RustExpr::to_fragment_precedence(x, Precedence::Top)),
                Some(Fragment::string(", ")),
            )
            .delimit(Fragment::Char('['), Fragment::Char(']')),
            RustExpr::MethodCall(x, name, args) => cond_paren(
                x.to_fragment_precedence(Precedence::Projection)
                    .intervene(Fragment::Char('.'), name.to_fragment())
                    .cat(ToFragmentExt::paren_list_prec(args, Precedence::Top)),
                prec,
                Precedence::Projection,
            ),
            RustExpr::FieldAccess(x, name) => x
                .to_fragment_precedence(Precedence::Projection)
                .intervene(Fragment::Char('.'), name.to_fragment()),
            RustExpr::FunctionCall(f, args) => cond_paren(
                f.to_fragment_precedence(prec)
                    .cat(ToFragmentExt::paren_list_prec(args, Precedence::Top)),
                prec,
                Precedence::Calculus,
            ),
            RustExpr::Tuple(elts) => match elts.as_slice() {
                [elt] => elt
                    .to_fragment_precedence(Precedence::Top)
                    .delimit(Fragment::Char('('), Fragment::string(",)")),
                _ => Self::paren_list_prec(elts, Precedence::Top),
            },
            RustExpr::Struct(con, fields) => {
                let f_fields = Fragment::seq(
                    fields.iter().map(|(lab, expr)| {
                        Fragment::intervene(
                            lab.to_fragment(),
                            Fragment::string(": "),
                            expr.as_ref().map_or(Fragment::Empty, |x| {
                                x.to_fragment_precedence(Precedence::Top)
                            }),
                        )
                    }),
                    Some(Fragment::string(", ")),
                );
                con.to_fragment()
                    .cat(Fragment::Char(' '))
                    .cat(f_fields.delimit(Fragment::string("{ "), Fragment::string(" }")))
            }
            RustExpr::Deref(expr) => {
                Fragment::Char('*').cat(expr.to_fragment_precedence(Precedence::Prefix))
            }
            RustExpr::Borrow(expr) => {
                Fragment::Char('&').cat(expr.to_fragment_precedence(Precedence::Prefix))
            }
            RustExpr::BorrowMut(expr) => {
                Fragment::string("&mut ").cat(expr.to_fragment_precedence(Precedence::Prefix))
            }
            RustExpr::Try(expr) => expr
                .to_fragment_precedence(Precedence::Projection)
                .cat(Fragment::Char('?')),
            RustExpr::Operation(op) => op.to_fragment_precedence(prec),
            RustExpr::BlockScope(stmts, val) => {
                RustStmt::block(stmts.iter().chain(std::iter::once(&RustStmt::Return(
                    ReturnKind::Implicit,
                    val.as_ref().clone(),
                ))))
            }
            RustExpr::Control(ctrl) => ctrl.to_fragment(),
            RustExpr::Closure(cl) => cl.to_fragment_precedence(prec),
            RustExpr::Slice(expr, start, stop) => expr
                .to_fragment_precedence(Precedence::Projection)
                .cat(Fragment::seq(
                    [
                        Fragment::Char('['),
                        start.to_fragment(),
                        Fragment::string(".."),
                        stop.to_fragment(),
                        Fragment::Char(']'),
                    ],
                    None,
                )),
            RustExpr::RangeExclusive(start, stop) => cond_paren(
                start.to_fragment_precedence(Precedence::Top).intervene(
                    Fragment::string(".."),
                    stop.to_fragment_precedence(Precedence::Top),
                ),
                prec,
                Precedence::Top,
            ),
        }
    }
}

impl ToFragment for RustExpr {
    fn to_fragment(&self) -> Fragment {
        self.to_fragment_precedence(Precedence::Atomic)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub(crate) enum ReturnKind {
    #[default]
    Implicit,
    Keyword,
}
impl ReturnKind {
    pub(crate) const fn is_keyword(&self) -> bool {
        matches!(self, Self::Keyword)
    }
}

impl From<bool> for ReturnKind {
    fn from(value: bool) -> Self {
        if value {
            Self::Keyword
        } else {
            Self::Implicit
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustStmt {
    Let(Mut, Label, Option<RustType>, RustExpr),
    Expr(RustExpr),
    Return(ReturnKind, RustExpr), // bool: true for explicit return, false for implicit return
    Control(RustControl),
    LocalFn(RustFn),
}

impl RustStmt {
    pub fn assign(name: impl Into<Label>, rhs: RustExpr) -> Self {
        Self::Let(Mut::Immutable, name.into(), None, rhs)
    }

    pub fn assign_mut(name: impl Into<Label>, rhs: RustExpr) -> Self {
        Self::Let(Mut::Mutable, name.into(), None, rhs)
    }

    pub fn assign_and_forget(rhs: RustExpr) -> Option<Self> {
        if rhs.is_pure() {
            None
        } else {
            Some(Self::Let(Mut::Immutable, Label::from("_"), None, rhs))
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustControl {
    Loop(Vec<RustStmt>),
    While(RustExpr, Vec<RustStmt>),
    ForIter(Label, RustExpr, Vec<RustStmt>), // element variable name, iterator expression (verbatim), loop contents
    ForRange0(Label, RustExpr, Vec<RustStmt>), // index variable name, upper bound (exclusive), loop contents (0..N)
    If(RustExpr, Vec<RustStmt>, Option<Vec<RustStmt>>),
    Match(RustExpr, RustMatchBody),
    Break, // no support for break values or loop labels, yet
}

pub(crate) type RustMatchCase = (MatchCaseLHS, Vec<RustStmt>);

#[derive(Clone, Debug)]
pub(crate) enum RustCatchAll {
    PanicUnreachable { message: Label },
    ReturnErrorValue { value: RustExpr },
}

impl RustCatchAll {
    pub fn to_match_case(&self) -> RustMatchCase {
        match self {
            RustCatchAll::PanicUnreachable { message } => (
                MatchCaseLHS::Pattern(RustPattern::CatchAll(Some(Label::Borrowed("_other")))),
                [RustStmt::Expr(RustExpr::local("unreachable!").call_with([
                    RustExpr::str_lit(format!(
                        "{message}match refuted with unexpected value {{_other:?}}"
                    )),
                ]))]
                .to_vec(),
            ),
            RustCatchAll::ReturnErrorValue { value } => (
                MatchCaseLHS::Pattern(RustPattern::CatchAll(None)),
                [RustStmt::Return(ReturnKind::Keyword, value.clone())].to_vec(),
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustMatchBody {
    Irrefutable(Vec<RustMatchCase>),
    Refutable(Vec<RustMatchCase>, RustCatchAll),
}

impl ToFragment for RustMatchBody {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustMatchBody::Irrefutable(cases) => {
                <RustMatchCase>::block_sep(cases, Fragment::string(",\n"))
            }
            RustMatchBody::Refutable(cases, catchall) => <RustMatchCase>::block_sep(
                cases
                    .iter()
                    .chain(std::iter::once(&catchall.to_match_case())),
                Fragment::string(",\n"),
            ),
        }
    }
}

impl From<Vec<RustMatchCase>> for RustMatchBody {
    fn from(value: Vec<RustMatchCase>) -> Self {
        RustMatchBody::Refutable(
            value,
            RustCatchAll::ReturnErrorValue {
                value: RustExpr::scoped(["ParseError"], "ExcludedBranch"),
            },
        )
    }
}

#[derive(Clone, Debug)]
pub(crate) enum MatchCaseLHS {
    Pattern(RustPattern),
    WithGuard(RustPattern, RustExpr),
}

impl ToFragment for MatchCaseLHS {
    fn to_fragment(&self) -> Fragment {
        match self {
            MatchCaseLHS::Pattern(pat) => pat.to_fragment(),
            MatchCaseLHS::WithGuard(pat, guard) => pat
                .to_fragment()
                .intervene(Fragment::string(" if "), guard.to_fragment()),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustPattern {
    PrimLiteral(RustPrimLit),
    TupleLiteral(Vec<RustPattern>),
    ArrayLiteral(Vec<RustPattern>),
    Fill,                                   // `..`
    CatchAll(Option<Label>),                // None <- `_`, Some("x") for `x`
    Variant(Constructor, Box<RustPattern>), // FIXME - need to attach enum scope
}

#[derive(Debug, Clone)]
pub(crate) enum Constructor {
    // Simple struct constructor
    Simple(Label),
    // Compound: Variant with intervening `::` between labels
    Compound(Label, Label),
}

impl From<Constructor> for RustEntity {
    fn from(value: Constructor) -> Self {
        match value {
            Constructor::Simple(lab) => RustEntity::Local(lab),
            Constructor::Compound(path, lab) => RustEntity::Scoped(vec![path], lab),
        }
    }
}

impl From<Constructor> for Label {
    fn from(value: Constructor) -> Self {
        match value {
            Constructor::Simple(lab) => lab,
            Constructor::Compound(path, var) => format!("{path}::{var}").into(),
        }
    }
}

impl ToFragment for RustPattern {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustPattern::PrimLiteral(pl) => pl.to_fragment(),
            RustPattern::TupleLiteral(tup) => RustPattern::paren_list(tup),
            RustPattern::ArrayLiteral(tup) => RustPattern::brace_list(tup),
            RustPattern::Variant(constr, inner) => {
                RustExpr::Entity(RustEntity::from(constr.clone()))
                    .to_fragment()
                    .cat(
                        inner
                            .to_fragment()
                            .delimit(Fragment::Char('('), Fragment::Char(')')),
                    )
            }
            RustPattern::Fill => Fragment::String("..".into()),
            RustPattern::CatchAll(None) => Fragment::Char('_'),
            RustPattern::CatchAll(Some(lab)) => Fragment::String(lab.clone()),
        }
    }
}

impl ToFragment for RustControl {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustControl::Loop(body) => Fragment::string("loop")
                .intervene(Fragment::Char(' '), RustStmt::block(body.iter())),
            RustControl::While(cond, body) => Fragment::string("while")
                .intervene(
                    Fragment::Char(' '),
                    cond.to_fragment_precedence(Precedence::TOP),
                )
                .intervene(Fragment::Char(' '), RustStmt::block(body.iter())),
            RustControl::If(cond, b_then, b_else) => Fragment::string("if")
                .intervene(
                    Fragment::Char(' '),
                    cond.to_fragment_precedence(Precedence::TOP),
                )
                .intervene(Fragment::Char(' '), RustStmt::block(b_then.iter()))
                .intervene(
                    Fragment::string(" else "),
                    Fragment::opt(b_else.as_ref(), |branch| RustStmt::block(branch.iter())),
                ),
            RustControl::Match(expr, body) => Fragment::string("match")
                .intervene(
                    Fragment::Char(' '),
                    expr.to_fragment_precedence(Precedence::TOP),
                )
                .intervene(Fragment::Char(' '), body.to_fragment()),
            RustControl::ForRange0(ctr_name, ubound, body) => Fragment::string("for")
                .intervene(Fragment::Char(' '), Fragment::String(ctr_name.clone()))
                .intervene(
                    Fragment::string(" in "),
                    Fragment::cat(
                        Fragment::string("0.."),
                        ubound.to_fragment_precedence(Precedence::TOP),
                    ),
                )
                .intervene(Fragment::Char(' '), RustStmt::block(body.iter())),
            RustControl::ForIter(elt_name, iterable, body) => Fragment::string("for")
                .intervene(Fragment::Char(' '), Fragment::String(elt_name.clone()))
                .intervene(
                    Fragment::string(" in "),
                    iterable.to_fragment_precedence(Precedence::TOP),
                )
                .intervene(Fragment::Char(' '), RustStmt::block(body.iter())),
            RustControl::Break => Fragment::string("break"),
        }
    }
}

impl ToFragment for (MatchCaseLHS, Vec<RustStmt>) {
    fn to_fragment(&self) -> Fragment {
        self.0
            .to_fragment()
            .intervene(Fragment::string(" => "), RustStmt::block(self.1.iter()))
    }
}

impl ToFragment for RustStmt {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustStmt::Let(_mut, binding, sig, value) => (match _mut {
                Mut::Mutable => Fragment::string("let mut "),
                Mut::Immutable => Fragment::string("let "),
            })
            .cat(binding.to_fragment())
            .intervene(
                Fragment::string(": "),
                Fragment::opt(sig.as_ref(), RustType::to_fragment),
            )
            .cat(Fragment::string(" = "))
            .cat(value.to_fragment_precedence(Precedence::TOP))
            .cat(Fragment::Char(';')),
            RustStmt::Expr(expr) => expr
                .to_fragment_precedence(Precedence::TOP)
                .cat(Fragment::Char(';')),
            RustStmt::Return(kind, expr) => {
                let (before, after) = if kind.is_keyword() {
                    (Fragment::String("return ".into()), Fragment::Char(';'))
                } else {
                    (Fragment::Empty, Fragment::Empty)
                };
                expr.to_fragment_precedence(Precedence::TOP)
                    .delimit(before, after)
            }
            RustStmt::Control(ctrl) => ctrl.to_fragment(),
            RustStmt::LocalFn(f) => f.to_fragment(),
        }
    }
}

pub trait ToFragment {
    fn to_fragment(&self) -> Fragment;

    fn delim_list<'a>(
        items: impl IntoIterator<Item = &'a Self>,
        before: Fragment,
        after: Fragment,
    ) -> Fragment
    where
        Self: 'a,
    {
        Fragment::seq(
            items.into_iter().map(Self::to_fragment),
            Some(Fragment::string(", ")),
        )
        .delimit(before, after)
    }

    fn paren_list<'a>(items: impl IntoIterator<Item = &'a Self>) -> Fragment
    where
        Self: 'a,
    {
        Self::delim_list(items, Fragment::Char('('), Fragment::Char(')'))
    }

    fn brace_list<'a>(items: impl IntoIterator<Item = &'a Self>) -> Fragment
    where
        Self: 'a,
    {
        Self::delim_list(items, Fragment::Char('['), Fragment::Char(']'))
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

trait ToFragmentExt: ToFragment {
    fn to_fragment_precedence(&self, prec: Precedence) -> Fragment;

    fn delim_list_prec<'a>(
        items: impl IntoIterator<Item = &'a Self>,
        prec: Precedence,
        before: Fragment,
        after: Fragment,
    ) -> Fragment
    where
        Self: 'a,
    {
        Fragment::seq(
            items.into_iter().map(|x| x.to_fragment_precedence(prec)),
            Some(Fragment::string(", ")),
        )
        .delimit(before, after)
    }

    fn paren_list_prec<'a>(items: impl IntoIterator<Item = &'a Self>, prec: Precedence) -> Fragment
    where
        Self: 'a,
    {
        Self::delim_list_prec(items, prec, Fragment::Char('('), Fragment::Char(')'))
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

impl<T> ToFragmentExt for Box<T>
where
    T: ToFragmentExt,
{
    fn to_fragment_precedence(&self, prec: Precedence) -> Fragment {
        self.as_ref().to_fragment_precedence(prec)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn expect_fragment(value: &impl ToFragmentExt, expected: &str) {
        assert_eq!(
            &format!("{}", value.to_fragment_precedence(Precedence::TOP)),
            expected
        )
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
        let re = RustExpr::local("this").call_method_with(
            "append",
            [RustExpr::BorrowMut(Box::new(RustExpr::local("other")))],
        );
        expect_fragment(&re, "this.append(&mut other)")
    }
}
