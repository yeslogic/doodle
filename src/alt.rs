pub mod prelude;

use std::{collections::BTreeMap, rc::Rc};

use crate::{
    BaseType, ByteSet, DynFormat, Expr, Format, FormatModule, FormatRef, IntoLabel, Label, Pattern,
    StyleHint, TypeHint, TypeScope, ValueKind, ValueType,
};
use anyhow::{anyhow, Result as AResult};
use serde::Serialize;

pub mod marker {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
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

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum GroundFormat {
    /// Reference to a top-level item
    ItemVar(usize, Vec<Expr>), // FIXME - do the exprs here need type(+) info?
    /// A format that never matches
    Fail,
    /// Matches if the end of the input has been reached
    EndOfInput,
    /// Skips bytes if necessary to align the current offset to a multiple of N
    Align(usize),
    /// Matches a byte in the given byte set
    Byte(ByteSet),
    /// Apply a dynamic format from a named variable in the scope
    Apply(Label),
    /// Current byte-offset relative to start-of-buffer (as a U64(?))
    Pos,
    /// Skip the remainder of the stream, up until the end of input or the last available byte within a Slice
    SkipRemainder,
    /// Compute a value
    Compute(Box<Expr>),
}

/// Descent-patterns for Formats that hold a single `Box<Format>`
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum MonoKind {
    Variant(Label),
    Repeat,
    Repeat1,
    RepeatCount(Box<Expr>),
    RepeatBetween(Box<Expr>, Box<Expr>),
    RepeatUntilLast(Box<Expr>),
    RepeatUntilSeq(Box<Expr>),
    AccumUntil(Box<Expr>, Box<Expr>, Box<Expr>, TypeHint),
    ForEach(Box<Expr>, Label),
    Maybe(Box<Expr>),
    Peek,
    PeekNot,
    Slice(Box<Expr>),
    Bits,
    WithRelativeOffset(Box<Expr>, Box<Expr>),
    Map(Box<Expr>),
    Where(Box<Expr>),
    Let(Label, Box<Expr>),
    Dynamic(Label, DynFormat),
    DecodeBytes(Box<Expr>),
    Hint(StyleHint),
}

/// Descent-patterns for Formats that hold `Vec<Format>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum PolyKind {
    Union,
    UnionNondet,
    Tuple,
    Sequence,
}

/// Descent-patterns for Formats that hold `Vec<(Pattern, Format)>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum PatKind {
    Match(Box<Expr>),
}

/// Descent-patterns for Formats that hold an `Option<Box<Format>>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum OptKind {
    LiftedOption,
}

/// Descent-patterns for `Formats` that hold two `Box<Format>`s
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum DuoKind {
    MonadSeq,
    LetFormat(Label),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum EpiFormat<B = Box<FormatExt>, V = Vec<FormatExt>, P = Vec<(Pattern, FormatExt)>> {
    Mono(MonoKind, B),
    Duo(DuoKind, B, B),
    Poly(PolyKind, V),
    Pat(PatKind, P),
    Opt(OptKind, Option<B>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum MetaFormat {
    BindScopeTo(Label),
    WithScope(Label, ScopeFormat),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum ScopeFormat {
    ReadArray(marker::BaseKind),
    ReadSliceLen(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum FormatExt {
    Ground(GroundFormat),
    Epi(EpiFormat),
    Meta(MetaFormat),
}

impl FormatExt {
    pub fn ground(f: GroundFormat) -> FormatExt {
        FormatExt::Ground(f)
    }

    pub fn epi(f: EpiFormat) -> FormatExt {
        FormatExt::Epi(f)
    }

    pub fn meta(f: MetaFormat) -> FormatExt {
        FormatExt::Meta(f)
    }

    pub fn is_ground(&self) -> bool {
        matches!(self, FormatExt::Ground(_))
    }

    pub fn is_epi(&self) -> bool {
        matches!(self, FormatExt::Epi(_))
    }

    pub fn is_meta(&self) -> bool {
        matches!(self, FormatExt::Meta(_))
    }

    pub fn contains_meta(&self) -> bool {
        match self {
            FormatExt::Ground(_) => false,
            FormatExt::Epi(f) => f.contains_meta(),
            FormatExt::Meta(_) => true,
        }
    }
}

impl EpiFormat {
    pub fn contains_meta(&self) -> bool {
        match self {
            EpiFormat::Mono(_, b) => b.contains_meta(),
            EpiFormat::Duo(_, b0, b1) => b0.contains_meta() || b1.contains_meta(),
            EpiFormat::Poly(_, v) => v.iter().any(FormatExt::contains_meta),
            EpiFormat::Pat(_, p) => p.iter().any(|(_, f)| f.contains_meta()),
            EpiFormat::Opt(_, opt_f) => opt_f.as_ref().is_some_and(|f| f.contains_meta()),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct FormatCompiler {
    params: CompilerParams,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct CompilerParams {
    pub alternate_processing_model: bool,
}

impl FormatCompiler {
    pub fn new(params: CompilerParams) -> Self {
        Self { params }
    }

    pub fn compile(&self, f: &FormatExt) -> Format {
        match f {
            FormatExt::Ground(gf) => Format::from(gf.clone()),
            FormatExt::Epi(ef) => self.compile_epi(ef),
            FormatExt::Meta(mf) => self.compile_meta(mf),
        }
    }

    pub fn compile_epi(&self, ef: &EpiFormat) -> Format {
        match ef {
            EpiFormat::Mono(mono_kind, fx) => {
                let f = Box::new(self.compile(fx));
                Format::from((mono_kind.clone(), f))
            }
            EpiFormat::Duo(duo_kind, fx0, fx1) => {
                let f0 = Box::new(self.compile(fx0));
                let f1 = Box::new(self.compile(fx1));
                Format::from((duo_kind.clone(), f0, f1))
            }
            EpiFormat::Poly(poly_kind, fxs) => {
                let fs = fxs.iter().map(|f| self.compile(f)).collect();
                Format::from((poly_kind.clone(), fs))
            }
            EpiFormat::Opt(opt_kind, opt_fx) => {
                let opt_f = opt_fx.as_ref().map(|f| Box::new(self.compile(f)));
                Format::from((opt_kind.clone(), opt_f))
            }
            EpiFormat::Pat(pat_kind, pat_fxs) => {
                let pat_fs = pat_fxs
                    .iter()
                    .map(|(p, f)| (p.clone(), self.compile(f)))
                    .collect();
                Format::from((pat_kind.clone(), pat_fs))
            }
        }
    }

    pub fn compile_meta(&self, mf: &MetaFormat) -> Format {
        match mf {
            MetaFormat::BindScopeTo(_lbl) => {
                todo!("implement compilation logic for MetaFormat::BindScopeTo");
            }
            MetaFormat::WithScope(_lbl, scope_format) => match scope_format {
                ScopeFormat::ReadArray(_base_kind) => {
                    todo!("implement compilation logic for ScopeFormat::ReadArray")
                }
                ScopeFormat::ReadSliceLen(_expr) => {
                    todo!("implement compilation logic for ScopeFormat::ReadSliceLen")
                }
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FormatModuleExt {
    names: Vec<Label>,
    args: Vec<Vec<(Label, ValueType)>>,
    formats: Vec<FormatExt>,
    format_types: Vec<ValueType>,
}

impl FormatModuleExt {
    pub fn compile(self, compiler: &FormatCompiler) -> FormatModule {
        let formats = self.formats.iter().map(|f| compiler.compile(f)).collect();
        FormatModule {
            names: self.names,
            args: self.args,
            formats,
            format_types: self.format_types,
        }
    }

    pub fn define_format<Name: IntoLabel>(
        &mut self,
        name: Name,
        format_ext: FormatExt,
    ) -> FormatRef {
        self.define_format_args(name, vec![], format_ext)
    }

    pub fn define_format_args<Name: IntoLabel>(
        &mut self,
        name: Name,
        args: Vec<(Label, ValueType)>,
        format_ext: FormatExt,
    ) -> FormatRef {
        let mut scope = TypeScope::new();
        for (arg_name, arg_type) in &args {
            scope.push(arg_name.clone(), arg_type.clone());
        }
        let format_type = match self.infer_format_type(&scope, &format_ext) {
            Ok(t) => t,
            Err(msg) => panic!("{msg}"),
        };
        let level = self.names.len();
        self.names.push(name.into());
        self.args.push(args);
        self.formats.push(format_ext);
        self.format_types.push(format_type);
        FormatRef(level)
    }

    fn get_args(&self, level: usize) -> &[(Label, ValueType)] {
        &self.args[level]
    }

    pub fn get_format_type(&self, level: usize) -> &ValueType {
        &self.format_types[level]
    }
}

impl FormatModuleExt {
    pub(crate) fn infer_format_type(
        &self,
        scope: &TypeScope<'_>,
        f: &FormatExt,
    ) -> AResult<ValueType> {
        match f {
            FormatExt::Ground(ground_format) => match ground_format {
                GroundFormat::ItemVar(level, args) => {
                    let arg_names = self.get_args(*level);
                    if arg_names.len() != args.len() {
                        return Err(anyhow!(
                            "Expected {} arguments, found {}",
                            arg_names.len(),
                            args.len()
                        ));
                    }
                    for ((_name, arg_type), expr) in Iterator::zip(arg_names.iter(), args.iter()) {
                        let t = expr.infer_type(scope)?;
                        let _t = arg_type.unify(&t)?;
                    }
                    Ok(self.get_format_type(*level).clone())
                }
                GroundFormat::Fail => Ok(ValueType::Empty),

                GroundFormat::SkipRemainder | GroundFormat::EndOfInput | GroundFormat::Align(_) => {
                    Ok(ValueType::UNIT)
                }

                GroundFormat::Byte(_bs) => Ok(ValueType::Base(BaseType::U8)),
                GroundFormat::Compute(expr) => expr.infer_type(scope),

                // REVIEW - do we want to hard-code this as U64 or make it a flexibly abstract integer type?
                GroundFormat::Pos => Ok(ValueType::Base(BaseType::U64)),

                GroundFormat::Apply(name) => match scope.get_type_by_name(name) {
                    ValueKind::Format(t) => Ok(t.clone()),
                    ValueKind::Value(t) => Err(anyhow!("Apply: expected format, found {t:?}")),
                },
            },
            FormatExt::Epi(epi_format) => match epi_format {
                EpiFormat::Mono(mono_kind, f) => {
                    match mono_kind {
                        MonoKind::Variant(label) => {
                            let t = self.infer_format_type(scope, f)?;
                            Ok(ValueType::Union(BTreeMap::from([(label.clone(), t)])))
                        }
                        MonoKind::Repeat | MonoKind::Repeat1 => {
                            let t = self.infer_format_type(scope, f)?;
                            Ok(ValueType::Seq(Box::new(t)))
                        }

                        MonoKind::RepeatCount(count) => {
                            match count.infer_type(scope)? {
                                ValueType::Base(b) if b.is_numeric() => {
                                    let t = self.infer_format_type(scope, f)?;
                                    Ok(ValueType::Seq(Box::new(t)))
                                }
                                other => Err(anyhow!("RepeatCount first argument type should be numeric, found {other:?} instead")),
                            }
                        }
                        MonoKind::RepeatBetween(min, max) => {
                            match min.infer_type(scope)? {
                                ref t0 @ ValueType::Base(b0) if b0.is_numeric() => {
                                    match max.infer_type(scope)? {
                                        ValueType::Base(b1) if b0 == b1 => {
                                            let t = self.infer_format_type(scope, f)?;
                                            Ok(ValueType::Seq(Box::new(t)))
                                        }
                                        other => Err(anyhow!("RepeatBetween second argument type should be the same as the first, found {other:?} (!= {t0:?})")),
                                    }
                                }
                                other => Err(anyhow!("RepeatBetween first argument type should be numeric, found {other:?} instead")),
                            }
                        }
                        MonoKind::RepeatUntilLast(lambda_elem) => {
                            match lambda_elem.as_ref() {
                                Expr::Lambda(head, expr) => {
                                    let t = self.infer_format_type(scope, f)?;
                                    let mut child_scope = TypeScope::child(scope);
                                    child_scope.push(head.clone(), t.clone());
                                    let ret_type = expr.infer_type(&child_scope)?;
                                    match ret_type {
                                        ValueType::Base(BaseType::Bool) => Ok(ValueType::Seq(Box::new(t))),
                                        other => Err(anyhow!("RepeatUntilLast first argument (lambda) return type should be Bool, found {other:?} instead")),
                                    }
                                }
                                other => Err(anyhow!("RepeatUntilLast first argument type should be lambda, found {other:?} instead")),
                            }
                        }
                        MonoKind::RepeatUntilSeq(lambda_seq) => {
                            match lambda_seq.as_ref() {
                                Expr::Lambda(head, expr) => {
                                    let t = self.infer_format_type(scope, f)?;
                                    let mut child_scope = TypeScope::child(scope);
                                    child_scope.push(head.clone(), ValueType::Seq(Box::new(t.clone())));
                                    let ret_type = expr.infer_type(&child_scope)?;
                                    match ret_type {
                                        ValueType::Base(BaseType::Bool) => Ok(ValueType::Seq(Box::new(t))),
                                        other => Err(anyhow!("RepeatUntilSeq first argument (lambda) return type should be Bool, found {other:?} instead")),
                                    }
                                }
                                other => Err(anyhow!("RepeatUntilSeq first argument type should be lambda, found {other:?} instead")),
                            }
                        }
                        MonoKind::AccumUntil(lambda_acc_seq, lambda_acc_val, init, vt) => {
                            match lambda_acc_seq.as_ref() {
                                Expr::Lambda(head, expr) => {
                                    let t = self.infer_format_type(scope, f)?;
                                    // Check that the initial accumulator value's type unifies with the type-claim
                                    let _acc_type = init.infer_type(&scope)?.unify(vt.as_ref())?;
                                    let mut child_scope = TypeScope::child(scope);
                                    let t_seq = ValueType::Seq(Box::new(t.clone()));
                                    let vt_acc_seq = ValueType::Tuple(vec![vt.as_ref().clone(), t_seq.clone()]);
                                    child_scope.push(head.clone(), vt_acc_seq.clone());
                                    let ret_type = expr.infer_type(&child_scope)?;
                                    match ret_type {
                                        ValueType::Base(BaseType::Bool) => {
                                            match lambda_acc_val.as_ref() {
                                                Expr::Lambda(head, expr) => {
                                                    let mut child_scope = TypeScope::child(&child_scope);
                                                    let vt_acc_elem = ValueType::Tuple(vec![vt.as_ref().clone(), t.clone()]);
                                                    child_scope.push(head.clone(), vt_acc_elem);
                                                    // we just need to check that these types unify, the value is unimportant
                                                    let _ret_type = expr.infer_type(&child_scope)?.unify(vt.as_ref())?;
                                                    Ok(vt_acc_seq)
                                                }
                                                other => return Err(anyhow!("AccumUntil second argument type should be lambda, found {other:?} instead")),
                                            }
                                        }
                                        other => Err(anyhow!("AccumUntil first argument (lambda) return type should be Bool, found {other:?} instead")),
                                    }

                                }
                                other => Err(anyhow!("AccumUntil first argument type should be lambda, found {other:?} instead")),
                            }
                        }
                        MonoKind::ForEach(expr, lbl) => {
                            let expr_t = expr.infer_type(scope)?;
                            let elem_t = match expr_t {
                                ValueType::Seq(elem_t) => (*elem_t).clone(),
                                _ => return Err(anyhow!("ForEach: expected Seq, found {expr_t:?}")),
                            };
                            let mut child_scope = TypeScope::child(scope);
                            child_scope.push(lbl.clone(), elem_t);
                            let inner_t = self.infer_format_type(&child_scope, f)?;
                            Ok(ValueType::Seq(Box::new(inner_t)))
                        }
                        MonoKind::Maybe(x) => match x.infer_type(scope)? {
                            ValueType::Base(BaseType::Bool) => {
                                let t = self.infer_format_type(scope, f)?;
                                Ok(ValueType::Option(Box::new(t)))
                            }
                            other => Err(anyhow!(
                                "Maybe-predicate is not a bool-type: {x:?} ~ {other:?}"
                            )),
                        },
                        MonoKind::Peek => self.infer_format_type(scope, f),
                        MonoKind::PeekNot => Ok(ValueType::UNIT),
                        MonoKind::Slice(expr) => {
                            debug_assert!(expr.infer_type(scope).is_ok_and(|vt| vt.as_base().is_some_and(BaseType::is_numeric)));
                            self.infer_format_type(scope, f)
                        }
                        MonoKind::Bits => self.infer_format_type(scope, f),
                        MonoKind::WithRelativeOffset(_base_addr, _offs) => {
                            self.infer_format_type(scope, f)
                        }
                        MonoKind::Map(expr) => {
                            let arg_t = self.infer_format_type(scope, f)?;
                            match expr.as_ref() {
                                Expr::Lambda(name, body) => {
                                    let mut child_scope = TypeScope::child(scope);
                                    child_scope.push(name.clone(), arg_t);
                                    body.infer_type(&child_scope)
                                }
                                other => Err(anyhow!("Map: expected lambda, found {other:?}")),
                            }
                        }
                        MonoKind::Where(expr) => {
                            let arg_type = self.infer_format_type(scope, f)?;
                            match expr.as_ref() {
                                Expr::Lambda(name, body) => {
                                    let mut child_scope = TypeScope::child(scope);
                                    child_scope.push(name.clone(), arg_type.clone());
                                    let t = body.infer_type(&child_scope)?;
                                    if t != ValueType::Base(BaseType::Bool) {
                                        return Err(anyhow!("Where: expected bool lambda, found {t:?}"));
                                    }
                                    Ok(arg_type)
                                }
                                other => Err(anyhow!("Where: expected lambda, found {other:?}")),
                            }
                        }
                        MonoKind::Let(name, expr) => {
                            let t = expr.infer_type(scope)?;
                            let mut child_scope = TypeScope::child(scope);
                            child_scope.push(name.clone(), t);
                            self.infer_format_type(&child_scope, f)
                        }
                        MonoKind::Dynamic(name, dyn_format) => {
                            match dyn_format {
                                DynFormat::Huffman(lengths_expr, _opt_values_expr) => match lengths_expr.infer_type(scope)? {
                                    ValueType::Seq(t) => match &*t {
                                        ValueType::Base(BaseType::U8 | BaseType::U16) => {
                                            let mut child_scope = TypeScope::child(scope);
                                            child_scope.push(name.clone(), ValueType::Base(BaseType::U16));
                                            self.infer_format_type(&child_scope, f)
                                        }
                                        other => Err(anyhow!("Huffman: expected U8 or U16, found {other:?}")),
                                    }
                                    other => Err(anyhow!("Huffman: expected Seq, found {other:?}")),
                                }
                            }
                        }
                        MonoKind::DecodeBytes(bytes) => {
                            let bytes_type = bytes.infer_type(scope)?;
                            match bytes_type {
                                ValueType::Seq(bt) if matches!(*bt, ValueType::Base(BaseType::U8)) => {
                                    self.infer_format_type(scope, f)
                                }
                                other => Err(anyhow!("DecodeBytes first argument type should be Seq(U8), found {other:?} instead")),
                            }
                        }
                        MonoKind::Hint(_) => {
                            self.infer_format_type(scope, f)
                        }
                    }
                }
                EpiFormat::Duo(duo_kind, f0, f1) => match duo_kind {
                    DuoKind::MonadSeq => {
                        let _ = self.infer_format_type(scope, f0)?;
                        self.infer_format_type(scope, f1)
                    }
                    DuoKind::LetFormat(name) => {
                        let t0 = self.infer_format_type(scope, f0)?;
                        let mut new_scope = TypeScope::child(scope);
                        new_scope.push(name.clone(), t0);
                        self.infer_format_type(&new_scope, f1)
                    }
                },
                EpiFormat::Poly(poly_kind, fs) => match poly_kind {
                    PolyKind::Union | PolyKind::UnionNondet => {
                        let mut t = ValueType::Empty;
                        for f in fs {
                            t = t.unify(&self.infer_format_type(scope, f)?)?;
                        }
                        Ok(t)
                    }
                    PolyKind::Tuple => {
                        let mut ts = Vec::new();
                        for f in fs {
                            ts.push(self.infer_format_type(scope, f)?);
                        }
                        Ok(ValueType::Tuple(ts))
                    }
                    PolyKind::Sequence => {
                        let mut elem_t = ValueType::Any;
                        for f in fs {
                            elem_t = elem_t.unify(&self.infer_format_type(scope, f)?)?;
                        }
                        Ok(ValueType::Seq(Box::new(elem_t)))
                    }
                },
                EpiFormat::Pat(pat_kind, pat_fs) => match pat_kind {
                    PatKind::Match(head) => {
                        if pat_fs.is_empty() {
                            return Err(anyhow!("infer_format_type: empty Match"));
                        }
                        let head_type = Rc::new(head.infer_type(scope)?);
                        let mut t = ValueType::Any;
                        for (pattern, branch) in pat_fs {
                            t = t.unify(&pattern.infer_format_branch_type_ext(
                                scope,
                                head_type.clone(),
                                self,
                                branch,
                            )?)?;
                        }
                        Ok(t)
                    }
                },
                EpiFormat::Opt(opt_kind, opt_f) => match opt_kind {
                    OptKind::LiftedOption => match opt_f {
                        None => Ok(ValueType::Option(Box::new(ValueType::Any))),
                        Some(f) => {
                            let t = self.infer_format_type(scope, f)?;
                            Ok(ValueType::Option(Box::new(t)))
                        }
                    },
                },
            },
            FormatExt::Meta(meta_format) => {
                match meta_format {
                    MetaFormat::BindScopeTo(_ident) => Ok(ValueType::UNIT),
                    MetaFormat::WithScope(_ident, scope_format) => match scope_format {
                        ScopeFormat::ReadArray(_base_kind) => {
                            todo!("type inference does not support ReadArray")
                        }
                        ScopeFormat::ReadSliceLen(expr) => {
                            // TODO: check that _ident is bound to a scope
                            let t = expr.infer_type(scope)?;
                            if t.as_base().is_some_and(BaseType::is_numeric) {
                                Ok(ValueType::Seq(Box::new(ValueType::Base(BaseType::U8))))
                            } else {
                                Err(anyhow!("ReadSliceLen: expected numeric type, found {t:?}"))
                            }
                        }
                    },
                }
            }
        }
    }
}

mod __impls {
    use super::*;

    impl From<GroundFormat> for FormatExt {
        fn from(f: GroundFormat) -> Self {
            FormatExt::Ground(f)
        }
    }

    impl From<EpiFormat> for FormatExt {
        fn from(f: EpiFormat) -> Self {
            FormatExt::Epi(f)
        }
    }

    impl From<MetaFormat> for FormatExt {
        fn from(f: MetaFormat) -> Self {
            FormatExt::Meta(f)
        }
    }

    impl From<GroundFormat> for Format {
        fn from(f: GroundFormat) -> Self {
            match f {
                GroundFormat::Fail => Format::Fail,
                GroundFormat::Pos => Format::Pos,
                GroundFormat::EndOfInput => Format::EndOfInput,
                GroundFormat::SkipRemainder => Format::SkipRemainder,
                GroundFormat::Align(n) => Format::Align(n),
                GroundFormat::Byte(bs) => Format::Byte(bs),
                GroundFormat::Apply(lbl) => Format::Apply(lbl),
                GroundFormat::ItemVar(level, exprs) => Format::ItemVar(level, exprs),
                GroundFormat::Compute(expr) => Format::Compute(expr),
            }
        }
    }

    impl From<(MonoKind, Box<Format>)> for Format {
        fn from(value: (MonoKind, Box<Format>)) -> Self {
            let (kind, inner) = value;
            match kind {
                MonoKind::Variant(lab) => Format::Variant(lab, inner),
                MonoKind::Repeat => Format::Repeat(inner),
                MonoKind::Repeat1 => Format::Repeat1(inner),
                MonoKind::RepeatCount(expr) => Format::RepeatCount(expr, inner),
                MonoKind::RepeatBetween(expr, expr1) => Format::RepeatBetween(expr, expr1, inner),
                MonoKind::RepeatUntilLast(expr) => Format::RepeatUntilLast(expr, inner),
                MonoKind::RepeatUntilSeq(expr) => Format::RepeatUntilSeq(expr, inner),
                MonoKind::AccumUntil(expr, expr1, expr2, type_hint) => {
                    Format::AccumUntil(expr, expr1, expr2, type_hint, inner)
                }
                MonoKind::ForEach(expr, lab) => Format::ForEach(expr, lab, inner),
                MonoKind::Maybe(expr) => Format::Maybe(expr, inner),
                MonoKind::Peek => Format::Peek(inner),
                MonoKind::PeekNot => Format::PeekNot(inner),
                MonoKind::Slice(expr) => Format::Slice(expr, inner),
                MonoKind::Bits => Format::Bits(inner),
                MonoKind::WithRelativeOffset(expr, expr1) => {
                    Format::WithRelativeOffset(expr, expr1, inner)
                }
                MonoKind::Map(expr) => Format::Map(inner, expr),
                MonoKind::Where(expr) => Format::Where(inner, expr),
                MonoKind::Let(lab, expr) => Format::Let(lab, expr, inner),
                MonoKind::Dynamic(lab, dyn_format) => Format::Dynamic(lab, dyn_format, inner),
                MonoKind::DecodeBytes(expr) => Format::DecodeBytes(expr, inner),
                MonoKind::Hint(style_hint) => Format::Hint(style_hint, inner),
            }
        }
    }

    impl From<(DuoKind, Box<Format>, Box<Format>)> for Format {
        fn from(value: (DuoKind, Box<Format>, Box<Format>)) -> Self {
            let (kind, f0, f1) = value;
            match kind {
                DuoKind::MonadSeq => Format::MonadSeq(f0, f1),
                DuoKind::LetFormat(lab) => Format::LetFormat(f0, lab, f1),
            }
        }
    }

    impl From<(PolyKind, Vec<Format>)> for Format {
        fn from(value: (PolyKind, Vec<Format>)) -> Self {
            let (kind, fs) = value;
            match kind {
                PolyKind::Union => Format::Union(fs),
                PolyKind::UnionNondet => Format::UnionNondet(fs),
                PolyKind::Tuple => Format::Tuple(fs),
                PolyKind::Sequence => Format::Sequence(fs),
            }
        }
    }

    impl From<(OptKind, Option<Box<Format>>)> for Format {
        fn from(value: (OptKind, Option<Box<Format>>)) -> Self {
            let (kind, f) = value;
            match kind {
                OptKind::LiftedOption => Format::LiftedOption(f),
            }
        }
    }

    impl From<(PatKind, Vec<(Pattern, Format)>)> for Format {
        fn from(value: (PatKind, Vec<(Pattern, Format)>)) -> Self {
            let (kind, fs) = value;
            match kind {
                PatKind::Match(expr) => Format::Match(expr, fs),
            }
        }
    }
}
