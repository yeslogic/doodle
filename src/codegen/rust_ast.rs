use std::borrow::Cow;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::output::{Fragment, FragmentBuilder};

use crate::precedence::{cond_paren, Precedence};
use crate::{BaseType, IntoLabel, Label, ValueType};

/// Enum-type (currently degenerate) for specifying the visibility of a top-level item
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) enum Visibility {
    /// Equivalent to leaving out any visibility keywords (i.e. as if `pub(self)`)
    #[default]
    Implicit,
    Public,
}

impl Visibility {
    fn add_vis(&self, item: Fragment) -> Fragment {
        match self {
            Self::Implicit => item,
            Self::Public => Fragment::cat(Fragment::string("pub "), item),
        }
    }
}

// FIXME - this shouldn't be open-coded but it will do for now
pub(crate) struct AllowAttr(Label);

impl From<Label> for AllowAttr {
    fn from(value: Label) -> Self {
        AllowAttr(value)
    }
}

impl ToFragment for AllowAttr {
    fn to_fragment(&self) -> Fragment {
        Fragment::cat(
            Fragment::string("allow"),
            Fragment::string(self.0.clone()).delimit(Fragment::Char('('), Fragment::Char(')')),
        )
    }
}

pub(crate) enum ModuleAttr {
    Allow(AllowAttr),
}

impl ToFragment for ModuleAttr {
    fn to_fragment(&self) -> Fragment {
        match self {
            ModuleAttr::Allow(allow_attr) => Fragment::string("#!").cat(
                allow_attr
                    .to_fragment()
                    .delimit(Fragment::Char('['), Fragment::Char(']')),
            ),
        }
    }
}

pub(crate) struct RustSubmodule(Visibility, Label);

impl RustSubmodule {
    pub fn new(label: impl IntoLabel) -> Self {
        RustSubmodule(Visibility::default(), label.into())
    }

    pub fn new_pub(label: impl IntoLabel) -> Self {
        RustSubmodule(Visibility::Public, label.into())
    }
}

impl ToFragment for RustSubmodule {
    fn to_fragment(&self) -> Fragment {
        self.0
            .add_vis(Fragment::cat(
                Fragment::string("mod "),
                self.1.to_fragment(),
            ))
            .cat(Fragment::Char(';'))
    }
}

#[derive(Default)]
pub(crate) struct RustProgram {
    mod_level_attrs: Vec<ModuleAttr>,
    submodules: Vec<RustSubmodule>,
    imports: Vec<RustImport>,
    items: Vec<RustItem>,
}

impl FromIterator<RustItem> for RustProgram {
    fn from_iter<T: IntoIterator<Item = RustItem>>(iter: T) -> Self {
        Self {
            imports: Vec::new(),
            items: Vec::from_iter(iter),
            ..Default::default()
        }
    }
}

impl RustProgram {
    // pub fn new() -> Self {
    //     RustProgram {
    //         mod_level_attrs: Vec::new(),
    //         submodules: Vec::new(),
    //         imports: Vec::new(),
    //         items: Vec::new(),
    //     }
    // }

    pub fn add_module_attr(&mut self, attr: ModuleAttr) {
        self.mod_level_attrs.push(attr)
    }

    pub fn add_submodule(&mut self, submodule: RustSubmodule) {
        self.submodules.push(submodule)
    }

    pub fn add_import(&mut self, import: RustImport) {
        self.imports.push(import)
    }
}

impl ToFragment for RustProgram {
    fn to_fragment(&self) -> Fragment {
        let mut frags = FragmentBuilder::new();
        for mod_level_attr in self.mod_level_attrs.iter() {
            frags.push(mod_level_attr.to_fragment().cat_break());
        }
        if !self.mod_level_attrs.is_empty() {
            frags.push(Fragment::Empty.cat_break());
        }
        for submodule in self.submodules.iter() {
            frags.push(submodule.to_fragment().cat_break());
        }
        if !self.submodules.is_empty() {
            frags.push(Fragment::Empty.cat_break());
        }

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

/// Representation for the specifications of what items should be imported from a module in a top-level or block-local `use` expression.
pub(crate) enum RustImportItems {
    /// Glob-imports from a single module
    Wildcard,
    Singleton(Label),
}

impl ToFragment for RustImportItems {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustImportItems::Wildcard => Fragment::Char('*'),
            RustImportItems::Singleton(lbl) => Fragment::String(lbl.clone()),
        }
    }
}

/// Top-level declared item (e.g. struct definitions and functions)
pub(crate) struct RustItem {
    vis: Visibility,
    attrs: Vec<RustAttr>,
    decl: RustDecl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
#[allow(dead_code)]
pub(crate) enum TraitSet {
    Empty = 0,
    Debug = 1,
    Clone = 2,
    #[default]
    DebugClone = 3,
    Copy = 6,
    DebugCopy = 7,
}

impl std::ops::BitAnd<TraitSet> for TraitSet {
    type Output = TraitSet;

    fn bitand(self, rhs: TraitSet) -> Self::Output {
        unsafe { std::mem::transmute(self as u8 & rhs as u8) }
    }
}

impl std::ops::BitOr<TraitSet> for TraitSet {
    type Output = TraitSet;

    fn bitor(self, rhs: TraitSet) -> Self::Output {
        unsafe { std::mem::transmute(self as u8 | rhs as u8) }
    }
}

impl TraitSet {
    pub fn into_label_vec(self) -> Vec<Label> {
        match self {
            TraitSet::Empty => vec![],
            TraitSet::Debug => vec![Label::from("Debug")],
            TraitSet::Clone => vec![Label::from("Clone")],
            TraitSet::DebugClone => vec![Label::from("Debug"), Label::from("Clone")],
            TraitSet::Copy => vec![Label::from("Copy"), Label::from("Clone")],
            TraitSet::DebugCopy => vec![
                Label::from("Debug"),
                Label::from("Copy"),
                Label::from("Clone"),
            ],
        }
    }

    pub fn into_attr(self) -> RustAttr {
        RustAttr::DeriveTraits(DeclDerives(self.into_label_vec()))
    }
}

impl RustItem {
    /// Promotes a standalone declaration to a top-level item with implicitly 'default' visibility (i.e. `pub(self)`).
    ///
    /// Attaches the specified set of derive-traits `traits` to the declaration if it is a type definition.
    ///
    /// Currently, this argument is ignored for functions.
    pub fn from_decl_with_traits(decl: RustDecl, traits: TraitSet) -> Self {
        let attrs = match decl {
            RustDecl::TypeDef(..) => vec![traits.into_attr()],
            RustDecl::Function(_) => Vec::new(),
        };
        Self {
            attrs,
            vis: Default::default(),
            decl,
        }
    }

    /// Promotes a standalone declaration to a top-level item with explicit 'pub' visibility.
    ///
    /// Attaches the specified set of derive-traits `traits` to the declaration if it is a type definition.
    ///
    /// Currently, this argument is ignored for functions.
    pub fn pub_decl_with_traits(decl: RustDecl, traits: TraitSet) -> Self {
        let attrs = match decl {
            RustDecl::TypeDef(..) => vec![traits.into_attr()],
            RustDecl::Function(_) => Vec::new(),
        };
        Self {
            attrs,
            vis: Visibility::Public,
            decl,
        }
    }

    /// Promotes a type declaration to a top-level item with implicit 'pub(self)' visibility and the default set of derive-traits
    /// (currently, `Debug` and `Clone`).
    ///
    /// For more fine-control over the traits that are derived, use [`from_decl_with_traits`](Self::from_decl_with_traits).
    #[inline]
    pub fn from_decl(decl: RustDecl) -> Self {
        Self::from_decl_with_traits(decl, TraitSet::default())
    }

    /// Promotes a type declaration to a top-level item with implicit 'pub(self)' visibility and the default set of derive-traits
    /// (currently, `Debug` and `Clone`).
    ///
    /// For more fine-control over the traits that are derived, use [`pub_decl_with_traits`](Self::pub_decl_with_traits).
    #[inline]
    #[allow(dead_code)]
    pub fn pub_decl(decl: RustDecl) -> Self {
        Self::pub_decl_with_traits(decl, TraitSet::default())
    }
}

impl RustItem {
    pub fn to_fragment(&self) -> Fragment {
        let mut builder = FragmentBuilder::new();
        for attr in self.attrs.iter() {
            builder.push(attr.to_fragment().cat_break());
        }
        builder
            .finalize()
            .cat(self.vis.add_vis(self.decl.to_fragment()))
    }
}

type TraitName = Label;

#[derive(Debug, Clone)]
pub enum RustAttr {
    DeriveTraits(DeclDerives),
}

impl ToFragment for RustAttr {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustAttr::DeriveTraits(derives) => derives.to_fragment(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct DeclDerives(Vec<TraitName>);

impl ToFragment for DeclDerives {
    fn to_fragment(&self) -> Fragment {
        let DeclDerives(traits) = self;
        if traits.is_empty() {
            Fragment::Empty
        } else {
            ToFragment::paren_list(traits)
                .delimit(Fragment::string("#[derive"), Fragment::Char(']'))
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RustDecl {
    TypeDef(Label, RustTypeDef),
    Function(RustFn),
}

impl RustDecl {
    pub fn type_def(lab: impl IntoLabel, def: RustTypeDef) -> Self {
        Self::TypeDef(lab.into(), def)
    }
}

impl ToFragment for RustDecl {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustDecl::TypeDef(name, tdef) => {
                let frag_key = Fragment::string(tdef.keyword_for());
                Fragment::intervene(frag_key, Fragment::Char(' '), name.to_fragment())
                    .intervene(Fragment::Char(' '), tdef.to_fragment())
            }
            RustDecl::Function(fn_def) => fn_def.to_fragment(),
        }
    }
}

/// Generic representation for a list of lifetime- and type-parameters, generic over the types used to represent
/// each of those two components
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

/// Representation of the abstract, name-only parameters used in the definition of a type or function
pub(crate) type DefParams = RustParams<Label, Label>;
/// Representation of the concrete, specific parameters used when locally invoking a function or qualifying a type
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

/// Representation for the signature, both arguments and return type, for a non-closure function
#[derive(Clone, Debug)]
pub(crate) struct FnSig {
    /// List of arguments with accompanying type annotations
    args: Vec<(Label, RustType)>,
    /// Return type (assumed to be unit if omitted)
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

/// Representation for standalone functions declared either inline or top-level in Rust
#[derive(Clone, Debug)]
pub(crate) struct RustFn {
    /// Function name
    name: Label,
    /// Optional list of generic lifetimes and types for the function declaration
    params: Option<DefParams>,
    /// Signature, including both input parameters and return type
    sig: FnSig,
    /// List of statements comprising the body of the function
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

/// Representation for both `struct` and `enum`-keyword declarations.
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

    /// Rough heuristic to determine whether a `RustTypeDef` can derive `Copy` without resulting in a compiler error.
    pub(crate) fn can_be_copy(&self) -> bool {
        match self {
            RustTypeDef::Enum(variants) => variants.iter().all(|v| v.can_be_copy()),
            RustTypeDef::Struct(struct_def) => struct_def.can_be_copy(),
        }
    }
}

/// Entry-type for representing type-level constructions in Rust, for use in function signatures and return types,
/// the field-types of struct definitions, and expression-level type annotations.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum RustType {
    Atom(AtomType),
    AnonTuple(Vec<RustType>),
    /// Catch-all for generics that we may not be able or willing to hardcode
    Verbatim(Label, UseParams),
}

impl RustType {
    pub const UNIT: RustType = RustType::Atom(AtomType::Prim(PrimType::Unit));

    /// Returns the RustType representation of an externally-defined and imported type `<name>`.
    pub fn imported(name: impl Into<Label>) -> Self {
        Self::Atom(AtomType::TypeRef(LocalType::External(name.into())))
    }

    /// Returns the RustType representation of a locally-defined type whose index in the code-generation table
    /// is `ix` and whose identifier is `name`.
    pub fn defined(ix: usize, name: impl Into<Label>) -> Self {
        Self::Atom(AtomType::TypeRef(LocalType::LocalDef(ix, name.into())))
    }

    /// Maps the provided RustType according to the transformation `T -> Vec<T>`
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn vec_of(inner: Self) -> Self {
        Self::Atom(AtomType::Comp(CompType::Vec(Box::new(inner))))
    }

    /// Constructs an anonymous tuple-type representative over an iterable collection of RustType elements.
    pub fn anon_tuple(elts: impl IntoIterator<Item = Self>) -> Self {
        Self::AnonTuple(elts.into_iter().collect())
    }

    /// Returns a RustType given a verbatim string-form of the type-level constructor to use,
    /// with an optional list of generic arguments to parameterize it with.
    pub fn verbatim(con: impl Into<Label>, params: Option<UseParams>) -> Self {
        Self::Verbatim(con.into(), params.unwrap_or_default())
    }

    /// Constructs a `RustType` representing `&'a (mut|) T` from parameters representing `'a` (optional),
    /// the mutability of the reference, and `T`, respectively.
    pub fn borrow_of(lt: Option<RustLt>, m: Mut, ty: RustType) -> Self {
        Self::Atom(AtomType::Comp(CompType::Borrow(lt, m, Box::new(ty))))
    }

    /// Constructs a `RustType` representing `Result<T, E>` from parameters representing `T` and `E`, respectively.
    pub fn result_of(ok_type: RustType, err_type: RustType) -> RustType {
        Self::Atom(AtomType::Comp(CompType::Result(
            Box::new(ok_type),
            Box::new(err_type),
        )))
    }

    fn try_as_primtype(&self) -> Option<PrimType> {
        match self {
            RustType::Atom(AtomType::Prim(pt)) => Some(*pt),
            _ => None,
        }
    }

    /// Returns `true` if `self` is a known-`Copy` `RustType`.
    pub(crate) fn is_copy(&self) -> bool {
        match self.try_as_primtype() {
            // NOTE - all PrimTypes are Copy, and only PrimTypes are statically determinable to be Copy
            Some(_pt) => true,
            _ => false,
        }
    }
}

impl RustType {
    /// Conservative heuristic for determining whether it is possible to implement `Copy` on a `RustTypeDef` containing embedded values of this `RustType` without
    /// resulting in a compiler error.
    ///
    /// Returns `true` if `self` is a primitive type, an immutable reference, or if it is an anonymous tuple or `Result` consisting only of such value-types.
    ///
    /// Because inference is performed locally, no embedded `LocalDef` values are considered to be Copyable, even when they are locally-defined with a `#[derive(Copy)]` attribute.
    pub(crate) fn can_be_copy(&self) -> bool {
        match self {
            RustType::Atom(at) => match at {
                AtomType::Prim(..) => true,
                AtomType::TypeRef(..) => false,
                AtomType::Comp(ct) => match ct {
                    CompType::Vec(_) => false,
                    CompType::Option(t) => t.can_be_copy(),
                    CompType::Result(t_ok, t_err) => t_ok.can_be_copy() && t_err.can_be_copy(),
                    CompType::Borrow(_lt, m, _t) => !m.is_mutable(),
                },
            },
            RustType::AnonTuple(args) => args.iter().all(|t| t.can_be_copy()),
            RustType::Verbatim(..) => false,
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

impl RustStruct {
    /// Rough heuristic to determine whether a `RustStruct` can derive `Copy` without resulting in a compiler error.
    pub(crate) fn can_be_copy(&self) -> bool {
        match self {
            RustStruct::Record(flds) => flds.iter().all(|(_, t)| t.can_be_copy()),
        }
    }
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
    /// Special-case for sanitization of labels-as-identifiers rather than a direct identity-function.
    fn to_fragment(&self) -> Fragment {
        Fragment::String(sanitize_label(self))
    }
}

/// Sanitizes a label such that it can be used as an identifier.
///
/// Crucially, this function is invariant and deterministic, so any two instances
/// of a common pre-image will always yield identical images, both within each
/// run of the code-generation phase and between such runs.
pub(crate) fn sanitize_label(label: &Label) -> Label {
    if label.chars().enumerate().all(|(ix, c)| is_valid(ix, c)) {
        remap(label.clone())
    } else {
        remap(Label::from(replace_bad(label.as_ref())))
    }
}

/// Adds a `r#` prefix to any reserved Rust keywords that would be invalid as identifiers.
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

/// Returns `true` if the given character at the given index is valid in Rust-compatible identifiers
fn is_valid(ix: usize, c: char) -> bool {
    match c {
        '-' | '.' | ' ' | '\t' => false,
        '0'..='9' => ix != 0,
        _ => true,
    }
}

/// Sanitizes a given identifier by replacing all runs of one or more disallowed characters with a single underscore,
/// and preceding any initial ASCII digits with a leading underscore
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

    /// Rough heuristic to determine whether an enum containing the given `RustVariant` can derive `Copy` without resulting in a compiler error.
    ///
    /// As a heuristic, this function is local-only, meaning a result of `true` merely indicates that the provided `RustVariant` itself is Copy-able,
    /// but not that the overall enum is necessarily Copyable given its other variants.
    pub(crate) fn can_be_copy(&self) -> bool {
        match self {
            RustVariant::Unit(_) => true,
            RustVariant::Tuple(_, elts) => elts.iter().all(RustType::can_be_copy),
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
    Option(T),
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
            CompType::Option(inner) => {
                let tmp = inner.to_fragment();
                tmp.delimit(Fragment::string("Option<"), Fragment::Char('>'))
            }
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
            ValueType::Option(t) => {
                let inner = Self::try_from(t.as_ref().clone())?;
                Ok(RustType::Atom(AtomType::Comp(CompType::Option(Box::new(
                    inner,
                )))))
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

impl Mut {
    pub fn is_mutable(&self) -> bool {
        matches!(self, Self::Mutable)
    }
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MethodSpecifier {
    Arbitrary(SubIdent),
    Common(CommonMethod),
}

impl MethodSpecifier {
    pub const LEN: Self = Self::Common(CommonMethod::Len);
}

impl From<SubIdent> for MethodSpecifier {
    fn from(v: SubIdent) -> Self {
        Self::Arbitrary(v)
    }
}

impl From<CommonMethod> for MethodSpecifier {
    fn from(v: CommonMethod) -> Self {
        Self::Common(v)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CommonMethod {
    Len,
}

impl CommonMethod {
    pub(crate) fn try_get_return_primtype(&self) -> Option<PrimType> {
        match self {
            CommonMethod::Len => Some(PrimType::Usize),
        }
    }
}

impl ToFragment for CommonMethod {
    fn to_fragment(&self) -> Fragment {
        match self {
            CommonMethod::Len => Fragment::string("len"),
        }
    }
}

impl ToFragment for MethodSpecifier {
    fn to_fragment(&self) -> Fragment {
        match self {
            MethodSpecifier::Arbitrary(v) => v.to_fragment(),
            MethodSpecifier::Common(v) => v.to_fragment(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RustExpr {
    Entity(RustEntity),
    PrimitiveLit(RustPrimLit),
    ArrayLit(Vec<RustExpr>),
    MethodCall(Box<RustExpr>, MethodSpecifier, Vec<RustExpr>), // used for specifically calling methods to assign a constant precedence to avoid parenthetical nesting
    FieldAccess(Box<RustExpr>, SubIdent), // can be used for receiver methods as well, with FunctionCall
    FunctionCall(Box<RustExpr>, Vec<RustExpr>), // can be used for tuple constructors as well
    Tuple(Vec<RustExpr>),
    Struct(RustEntity, Vec<(Label, Option<Box<RustExpr>>)>),
    CloneOf(Box<RustExpr>),
    // FIXME: this variant is unused
    #[allow(dead_code)]
    Deref(Box<RustExpr>),
    Borrow(Box<RustExpr>),
    // FIXME: this variant is unused
    #[allow(dead_code)]
    BorrowMut(Box<RustExpr>),
    Try(Box<RustExpr>),
    Operation(RustOp),
    BlockScope(Vec<RustStmt>, Box<RustExpr>), // scoped block with a final value as an implicit return
    Control(Box<RustControl>),                // for control blocks that return a value
    Closure(RustClosure),                     // only simple lambdas for now
    Index(Box<RustExpr>, Box<RustExpr>),      // object, index
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
pub(crate) enum InfixOperator {
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
    BoolOr,
    BoolAnd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PrefixOperator {
    BoolNot,
}
impl PrefixOperator {
    fn precedence(&self) -> Precedence {
        match self {
            PrefixOperator::BoolNot => Precedence::LOGNEGATE,
        }
    }

    pub(crate) fn out_type(&self, inner_type: PrimType) -> Option<PrimType> {
        match self {
            PrefixOperator::BoolNot => match inner_type {
                PrimType::Bool => Some(PrimType::Bool),
                _ => None,
            },
        }
    }
}

impl InfixOperator {
    pub(crate) fn precedence(&self) -> Precedence {
        match self {
            InfixOperator::Eq | InfixOperator::Neq => Precedence::EQUALITY,
            InfixOperator::Lt | InfixOperator::Lte | InfixOperator::Gt | InfixOperator::Gte => {
                Precedence::COMPARE
            }
            InfixOperator::Div | InfixOperator::Rem => Precedence::DIVREM,
            InfixOperator::Add | InfixOperator::Sub => Precedence::ADDSUB,
            InfixOperator::Mul => Precedence::MUL,
            InfixOperator::Shl | InfixOperator::Shr => Precedence::BITSHIFT,
            InfixOperator::BitOr => Precedence::BITOR,
            InfixOperator::BitAnd => Precedence::BITAND,
            InfixOperator::BoolAnd => Precedence::LOGAND,
            InfixOperator::BoolOr => Precedence::LOGOR,
        }
    }

    pub(crate) fn out_type(&self, lhs_type: PrimType, rhs_type: PrimType) -> Option<PrimType> {
        match self {
            InfixOperator::Eq | InfixOperator::Neq => {
                if lhs_type == rhs_type {
                    Some(PrimType::Bool)
                } else {
                    None
                }
            }
            InfixOperator::BoolAnd | InfixOperator::BoolOr => match (lhs_type, rhs_type) {
                (PrimType::Bool, PrimType::Bool) => Some(PrimType::Bool),
                _ => None,
            },
            InfixOperator::Lt | InfixOperator::Lte | InfixOperator::Gt | InfixOperator::Gte => {
                if lhs_type == rhs_type && lhs_type.is_numeric() {
                    Some(PrimType::Bool)
                } else {
                    None
                }
            }
            InfixOperator::BitOr
            | InfixOperator::BitAnd
            | InfixOperator::Div
            | InfixOperator::Rem
            | InfixOperator::Add
            | InfixOperator::Sub
            | InfixOperator::Mul => {
                if lhs_type == rhs_type && lhs_type.is_numeric() {
                    Some(lhs_type)
                } else {
                    None
                }
            }
            // NOTE - the types of a SHR or SHL do not have to be the same, but both must be numeric at the very least
            InfixOperator::Shl | InfixOperator::Shr => {
                if lhs_type.is_numeric() && rhs_type.is_numeric() {
                    Some(lhs_type)
                } else {
                    None
                }
            }
        }
    }
}

impl InfixOperator {
    pub(crate) fn token(&self) -> &'static str {
        match self {
            InfixOperator::Eq => " == ",
            InfixOperator::Neq => " != ",
            InfixOperator::Lt => " < ",
            InfixOperator::Lte => " <= ",
            InfixOperator::Gt => " > ",
            InfixOperator::Gte => " >= ",
            InfixOperator::Div => " / ",
            InfixOperator::Rem => " % ",
            InfixOperator::Add => " + ",
            InfixOperator::Sub => " - ",
            InfixOperator::Mul => " * ",
            InfixOperator::Shl => " << ",
            InfixOperator::Shr => " >> ",
            InfixOperator::BitOr => " | ",
            InfixOperator::BitAnd => " & ",
            InfixOperator::BoolOr => " || ",
            InfixOperator::BoolAnd => " && ",
        }
    }
}

impl PrefixOperator {
    pub(crate) fn token(&self) -> &'static str {
        match self {
            PrefixOperator::BoolNot => "!",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RustOp {
    InfixOp(InfixOperator, Box<RustExpr>, Box<RustExpr>),
    PrefixOp(PrefixOperator, Box<RustExpr>),
    AsCast(Box<RustExpr>, RustType),
}

impl RustOp {
    pub(crate) fn precedence(&self) -> Precedence {
        match self {
            Self::InfixOp(op, _, _) => op.precedence(),
            Self::PrefixOp(op, _) => op.precedence(),
            Self::AsCast(_, _) => Precedence::CAST_INFIX,
        }
    }

    /// Basic heuristic to determine whether a given operation is 'sound' at the type-level, i.e.
    /// that the operation in question is defined on the type of the operands and that the operands conform
    /// to the expectations of the operation, and are homogenous if that is required.
    ///
    /// If the operation could possibly be unsound, this method may conservatively return false even if it happens to be sound
    /// for the given operation, in practice.
    pub fn is_sound(&self) -> bool {
        match self {
            RustOp::InfixOp(op, lhs, rhs) => {
                match (op, lhs.try_get_primtype(), rhs.try_get_primtype()) {
                    (InfixOperator::Eq | InfixOperator::Neq, Some(ltype), Some(rtype)) => {
                        ltype == rtype
                    }
                    // NOTE - we need to filter out BoolAnd and BoolOr from the next catchall branch, so we can't merely match on the literal PrimType::Bool in the case-pattern
                    (InfixOperator::BoolAnd | InfixOperator::BoolOr, Some(ltype), Some(rtype)) => {
                        matches!((ltype, rtype), (PrimType::Bool, PrimType::Bool))
                    }
                    (_, Some(ltype), Some(rtype)) => ltype == rtype && ltype.is_numeric(),
                    (_, None, _) | (_, _, None) => false,
                }
            }
            RustOp::PrefixOp(op, inner) => match (op, inner.try_get_primtype()) {
                (PrefixOperator::BoolNot, Some(PrimType::Bool)) => true,
                (PrefixOperator::BoolNot, _) => false,
            },
            RustOp::AsCast(expr, typ) => match (expr.try_get_primtype(), typ.try_as_primtype()) {
                (Some(pt0), Some(pt1)) => !matches!(
                    PrimType::compare_width(pt0, pt1),
                    None | Some(Ordering::Greater)
                ),
                _ => false,
            },
        }
    }
}

impl RustOp {
    pub fn op_eq(lhs: RustExpr, rhs: RustExpr) -> Self {
        Self::InfixOp(InfixOperator::Eq, Box::new(lhs), Box::new(rhs))
    }

    pub fn op_neq(lhs: RustExpr, rhs: RustExpr) -> Self {
        Self::InfixOp(InfixOperator::Neq, Box::new(lhs), Box::new(rhs))
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
            RustOp::PrefixOp(op, inner) => cond_paren(
                Fragment::string(op.token()).cat(inner.to_fragment_precedence(inherent)),
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

    pub fn borrow_of(self) -> Self {
        match self {
            Self::CloneOf(this) => *this,
            other => Self::Borrow(Box::new(other)),
        }
    }

    pub fn field(self, name: impl Into<Label>) -> Self {
        match self {
            Self::CloneOf(this) => Self::CloneOf(Box::new(this.field(name))),
            other => Self::FieldAccess(Box::new(other), SubIdent::ByName(name.into())),
        }
    }

    pub fn nth(self, ix: usize) -> Self {
        match self {
            Self::CloneOf(this) => Self::CloneOf(Box::new(this.nth(ix))),
            other => Self::FieldAccess(Box::new(other), SubIdent::ByIndex(ix)),
        }
    }

    pub fn index(self, ix: RustExpr) -> RustExpr {
        match self {
            Self::CloneOf(this) => Self::CloneOf(Box::new(this.index(ix))),
            other => Self::Index(Box::new(other), Box::new(ix)),
        }
    }

    pub fn call_with(self, args: impl IntoIterator<Item = Self>) -> Self {
        Self::FunctionCall(Box::new(self), args.into_iter().collect())
    }

    pub fn call(self) -> Self {
        self.call_with(None)
    }

    /// Helper method that calls the `as_slice` method on the expression passed in,
    /// unpacking any top-level `RustExpr::CloneOf` variants to avoid inefficient (and unnecessary)
    /// clone-then-borrow constructs in the generated code.
    pub fn vec_as_slice(self) -> Self {
        let this = match self {
            Self::CloneOf(this) => *this,
            other => other,
        };
        this.call_method("as_slice")
    }

    /// Helper method that calls the `len` method on the expression passed in,
    /// unpacking any top-level `RustExpr::CloneOf` variants to avoid inefficient (and unnecessary)
    /// clone-then-borrow constructs in the generated code.
    pub fn vec_len(self) -> Self {
        let this = match self {
            Self::CloneOf(this) => this,
            other => Box::new(other),
        };
        RustExpr::MethodCall(this, MethodSpecifier::LEN, Vec::new())
    }

    pub fn call_method(self, name: impl Into<Label>) -> Self {
        self.call_method_with(name, None)
    }

    pub fn call_method_with(
        self,
        name: impl Into<Label>,
        args: impl IntoIterator<Item = Self>,
    ) -> Self {
        RustExpr::MethodCall(
            Box::new(self),
            SubIdent::ByName(name.into()).into(),
            args.into_iter().collect(),
        )
    }

    pub fn infix(lhs: Self, op: InfixOperator, rhs: Self) -> Self {
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

    /// Attempts to infer and return the (primitive) type of the given `RustExpr`,
    /// returning `None` if the expression is not a primitive type or otherwise
    /// cannot be inferred without further context or more complicated heuristics.
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
                    // REVIEW - the current only CommonMethod, Len, is not well-defined over non-empty argument lists but we don't check this
                    MethodSpecifier::Common(cm) => cm.try_get_return_primtype(),
                    MethodSpecifier::Arbitrary(SubIdent::ByIndex(_)) => {
                        unreachable!("unexpected method call using numeric subident")
                    }
                    MethodSpecifier::Arbitrary(SubIdent::ByName(name)) => {
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
            RustExpr::Index(seq, index) => {
                match &**seq {
                    RustExpr::ArrayLit(lits) => {
                        if index.try_get_primtype() == Some(PrimType::Usize) {
                            lits[0].try_get_primtype()
                        } else {
                            None
                        }
                    }
                    // FIXME - Anything more complex than an array literal may not be worth sinking excess effort for a minor QoL improvement
                    _ => None,
                }
            }
            // FIXME - there may be some functions we can predict the return values of, but for now we can leave this alone
            RustExpr::FunctionCall(..) => None,
            RustExpr::Tuple(tuple) => match &tuple[..] {
                [] => Some(PrimType::Unit),
                [x] => x.try_get_primtype(),
                [_, ..] => None,
            },
            RustExpr::Struct(..) => None,
            RustExpr::CloneOf(x) | RustExpr::Deref(x) => match &**x {
                RustExpr::Borrow(y) | RustExpr::BorrowMut(y) => y.try_get_primtype(),
                other => other.try_get_primtype(),
            },
            RustExpr::Borrow(_) | RustExpr::BorrowMut(_) => None,
            RustExpr::Try(..) => None,
            RustExpr::Operation(op) => match op {
                RustOp::InfixOp(op, lhs, rhs) => {
                    let lhs_type = lhs.try_get_primtype()?;
                    let rhs_type = rhs.try_get_primtype()?;
                    op.out_type(lhs_type, rhs_type)
                }
                RustOp::PrefixOp(op, expr) => {
                    let expr_type = expr.try_get_primtype()?;
                    op.out_type(expr_type)
                }
                RustOp::AsCast(expr, typ) => {
                    let out_typ = typ.try_as_primtype()?;
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
            // REVIEW - over types we have no control over, clone itself can be impure, but it should never be so for the code we ourselves are generating
            RustExpr::CloneOf(expr) => expr.is_pure(),
            RustExpr::MethodCall(x, MethodSpecifier::LEN, args) => {
                if args.is_empty() {
                    x.is_pure()
                } else {
                    unreachable!("unexpected method call on `len` with args: {:?}", args);
                }
            }
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
            // NOTE - while we can construct pure Try-expressions manually, the intent of `?` is to have potential side-effects and so we judge them de-facto impure
            RustExpr::Try(..) => false,
            RustExpr::Operation(op) => match op {
                RustOp::InfixOp(.., lhs, rhs) => lhs.is_pure() && rhs.is_pure() && op.is_sound(),
                RustOp::PrefixOp(.., inner) => inner.is_pure() && op.is_sound(),
                // NOTE - illegal casts like `x as u8` where x >= 256 are language-level errors that are neither pure nor impure
                RustOp::AsCast(expr, ..) => expr.is_pure() && op.is_sound(),
            },
            // NOTE - we can have block-scopes with non-empty statements that are pure, but that is a bit too much work for our purposes right now.
            RustExpr::BlockScope(stmts, tail) => stmts.is_empty() && tail.is_pure(),
            // NOTE - there may be some pure control expressions but those will be relatively rare as natural occurrences
            RustExpr::Control(..) => false,
            // NOTE - closures never appear in a context where elision is a possibility to consider so this result doesn't actually need to be refined further
            RustExpr::Closure(..) => false,
            // NOTE - slice/index exprs can always be out-of-bounds so they cannot be elided without changing program behavior
            RustExpr::Slice(..) | RustExpr::Index(..) => false,
            // NOTE - ranges can only ever be language-level errors if the endpoint types are not the same
            RustExpr::RangeExclusive(start, end) => {
                start.is_pure()
                    && end.is_pure()
                    && match (start.try_get_primtype(), end.try_get_primtype()) {
                        (Some(pt0), Some(pt1)) => pt0 == pt1,
                        // NOTE - there are legal cases for ranges involving unknown types (i.e. those with untyped variables) but we cannot rule one way or another on those
                        _ => false,
                    }
            }
        }
    }

    /// Embed a RustExpr into a new non-temporary value, or return it if it is already non-temporary
    pub(crate) fn make_persistent(&self) -> Cow<'_, Self> {
        match self {
            RustExpr::Entity(..) => Cow::Borrowed(self),
            // REVIEW - consider which non-entity cases are already 'peristent'
            _ => Cow::Owned(RustExpr::BlockScope(
                vec![RustStmt::assign("tmp", self.clone())],
                Box::new(RustExpr::local("tmp")),
            )),
        }
    }
}

impl ToFragmentExt for RustExpr {
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
            RustExpr::CloneOf(x) => cond_paren(
                x.to_fragment_precedence(Precedence::Projection)
                    .intervene(Fragment::Char('.'), Fragment::string("clone()")),
                prec,
                Precedence::Projection,
            ),
            RustExpr::FieldAccess(x, name) => x
                .to_fragment_precedence(Precedence::Projection)
                .intervene(Fragment::Char('.'), name.to_fragment()),
            RustExpr::FunctionCall(f, args) => cond_paren(
                f.to_fragment_precedence(Precedence::INVOKE)
                    .cat(ToFragmentExt::paren_list_prec(args, Precedence::Top)),
                prec,
                Precedence::INVOKE,
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
            RustExpr::Index(expr, ix) => expr.to_fragment_precedence(Precedence::Projection).cat(
                ix.to_fragment_precedence(Precedence::Top)
                    .delimit(Fragment::Char('['), Fragment::Char(']')),
            ),
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
    Reassign(Label, RustExpr),
    Expr(RustExpr),
    Return(ReturnKind, RustExpr), // bool: true for explicit return, false for implicit return
    Control(RustControl),
    // LocalFn(RustFn),
}

impl RustStmt {
    pub fn assign(name: impl Into<Label>, rhs: RustExpr) -> Self {
        Self::Let(Mut::Immutable, name.into(), None, rhs)
    }

    pub fn assign_mut(name: impl Into<Label>, rhs: RustExpr) -> Self {
        Self::Let(Mut::Mutable, name.into(), None, rhs)
    }

    /// Classifies the provided Expr using [`RustExpr::is_pure`], and returns a [`RustStmt`]
    /// that performs a vacuous let-assignment if it is effect-ful. Otherwise,
    /// returns None.
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
    // FIXME: this variant is unused
    #[allow(dead_code)]
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

impl RustPattern {
    /// Manually replaces a direct capture of a variable with a `ref` binding under the same name, or preserves
    /// the original value if some other pattern.
    pub(crate) fn ref_hack(self) -> Self {
        match self {
            RustPattern::CatchAll(Some(label)) => RustPattern::BindRef(label),
            _ => self,
        }
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
    PrimRange(RustPrimLit, Option<RustPrimLit>),
    TupleLiteral(Vec<RustPattern>),
    ArrayLiteral(Vec<RustPattern>),
    Option(Option<Box<RustPattern>>),
    Fill,                                   // `..`
    CatchAll(Option<Label>),                // Wildcard when None, otherwise a variable-binding
    BindRef(Label),                         // "x" => `ref x`
    Variant(Constructor, Box<RustPattern>), // FIXME - need to attach enum scope
}

#[derive(Debug, Clone)]
pub(crate) enum Constructor {
    // Simple struct constructor (mostly used for in-scope-by-default variants like `Ok` and `None`)
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
            RustPattern::PrimRange(pl0, Some(pl1)) => pl0
                .to_fragment()
                .intervene(Fragment::string("..="), pl1.to_fragment()),
            RustPattern::PrimRange(pl0, None) => pl0.to_fragment().cat(Fragment::string("..")),
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
            RustPattern::CatchAll(Some(lab)) => lab.to_fragment(),
            RustPattern::BindRef(lab) => Fragment::string("ref ").cat(lab.to_fragment()),
            RustPattern::Option(None) => Fragment::string("None"),
            RustPattern::Option(Some(pat)) => Fragment::string("Some").cat(
                pat.to_fragment()
                    .delimit(Fragment::Char('('), Fragment::Char(')')),
            ),
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
            RustStmt::Reassign(binding, value) => binding
                .to_fragment()
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
            // RustStmt::LocalFn(f) => f.to_fragment(),
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
