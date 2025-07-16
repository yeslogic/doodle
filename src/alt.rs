pub mod prelude;

use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
    rc::Rc,
};

use crate::{
    typecheck::UnificationError, valuetype::Container, BaseType, ByteSet, DynFormat, Expr, Format,
    FormatModule, FormatRef, IntoLabel, Label, Pattern, StyleHint, TypeScope, ValueKind, ValueType,
    ViewExpr,
};
use anyhow::{anyhow, Result as AResult};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ModelKind {
    #[default]
    BaseModel = 0,
    AltModel = 1,
}

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
// use marker::BaseKind;

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
    /// Apply a ViewFormat(Ext) to a specified view-level expression
    WithView(ViewExpr, ViewFormatExt),
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
    AccumUntil(Box<Expr>, Box<Expr>, Box<Expr>, TypeHintExt),
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
    LetView(Label),
    Dynamic(Label, DynFormat),
    DecodeBytes(Box<Expr>),
    ParseFromView(ViewExpr),
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
    // LetView(Label, Box<FormatExt>),
    // WithView(Label, ViewFormatExt),
    EngineSpecific {
        base_model: Box<FormatExt>,
        alt_model: Box<FormatExt>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum ReadKind {
    ByteSlice,
    ArrayOf(marker::BaseKind),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum ViewFormatExt {
    /// ReadLen(Len),
    CaptureBytes(Box<Expr>),
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
            FormatExt::Ground(gf) => Format::try_from(gf.clone()).expect("bad conversion"),
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
            MetaFormat::EngineSpecific {
                base_model,
                alt_model,
            } => {
                if self.params.alternate_processing_model {
                    self.compile(alt_model)
                } else {
                    self.compile(base_model)
                }
            }
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeHintExt(Container<ValueTypeExt>);

impl TypeHintExt {
    pub fn into_inner(&self) -> &Container<ValueTypeExt> {
        &self.0
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum ValueTypeExt {
    Any,
    Empty,
    Base(BaseType),
    Tuple(Vec<ValueTypeExt>),
    Record(Vec<(Label, ValueTypeExt)>),
    Union(BTreeMap<Label, ValueTypeExt>),
    Seq(Box<ValueTypeExt>),
    Option(Box<ValueTypeExt>),
    EngineSpecific {
        base_model: Box<ValueTypeExt>,
        alt_model: Box<ValueTypeExt>,
    },
}

impl From<ValueType> for ValueTypeExt {
    fn from(v: ValueType) -> Self {
        match v {
            ValueType::Any => ValueTypeExt::Any,
            ValueType::Empty => ValueTypeExt::Empty,
            ValueType::Base(b) => ValueTypeExt::Base(b),
            ValueType::Tuple(v) => {
                ValueTypeExt::Tuple(v.into_iter().map(ValueTypeExt::from).collect())
            }
            ValueType::Record(v) => ValueTypeExt::Record(
                v.into_iter()
                    .map(|(l, v)| (l, ValueTypeExt::from(v)))
                    .collect(),
            ),
            ValueType::Union(v) => ValueTypeExt::Union(
                v.into_iter()
                    .map(|(l, v)| (l, ValueTypeExt::from(v)))
                    .collect(),
            ),
            ValueType::Seq(v) => ValueTypeExt::Seq(Box::new(ValueTypeExt::from(*v))),
            ValueType::Option(v) => ValueTypeExt::Option(Box::new(ValueTypeExt::from(*v))),
        }
    }
}

impl ValueTypeExt {
    pub const UNIT: Self = ValueTypeExt::Tuple(Vec::new());

    pub fn is_numeric(&self) -> bool {
        match self {
            ValueTypeExt::Base(BaseType::U8) => true,
            ValueTypeExt::Base(BaseType::U16) => true,
            ValueTypeExt::Base(BaseType::U32) => true,
            ValueTypeExt::Base(BaseType::U64) => true,
            ValueTypeExt::EngineSpecific {
                base_model,
                alt_model,
            } => base_model.is_numeric() && alt_model.is_numeric(),
            _ => false,
        }
    }

    /// Returns `true` if the actual value type depends on the processing model.
    pub fn depends_on_model(&self) -> bool {
        match self {
            ValueTypeExt::Any => false,
            ValueTypeExt::Empty => false,
            ValueTypeExt::Base(_) => false,
            ValueTypeExt::Tuple(ts) => ts.iter().any(|t| t.depends_on_model()),
            ValueTypeExt::Record(items) => items.iter().any(|(_, t)| t.depends_on_model()),
            ValueTypeExt::Union(items) => items.iter().any(|(_, t)| t.depends_on_model()),
            ValueTypeExt::Seq(t) => t.depends_on_model(),
            ValueTypeExt::Option(t) => t.depends_on_model(),
            ValueTypeExt::EngineSpecific { .. } => true,
        }
    }

    /// Returns `true` if the only engine-specificity is found at the root nod
    pub fn is_canonical(&self) -> bool {
        match self {
            ValueTypeExt::EngineSpecific {
                base_model,
                alt_model,
            } => !base_model.depends_on_model() && !alt_model.depends_on_model(),
            other => !other.depends_on_model(),
        }
    }

    pub(crate) fn as_tuple_type(&self) -> &[ValueTypeExt] {
        match self {
            ValueTypeExt::Tuple(ts) => ts.as_slice(),
            other => panic!("not a tuple type: {other:?}"),
        }
    }

    /// Redistributes embedded model-dependent nodes to a single, top-level disjunction
    pub fn canonicalize(&self) -> Self {
        if self.depends_on_model() && !self.is_canonical() {
            let base_model = Box::new(self.normalize(ModelKind::BaseModel).into_owned());
            let alt_model = Box::new(self.normalize(ModelKind::AltModel).into_owned());
            ValueTypeExt::EngineSpecific {
                base_model,
                alt_model,
            }
        } else {
            self.clone()
        }
    }

    pub fn merge(base: &Self, alt: &Self) -> Self {
        match (
            base.normalize(ModelKind::BaseModel).as_ref(),
            alt.normalize(ModelKind::AltModel).as_ref(),
        ) {
            (ValueTypeExt::Any, _) => alt.clone(),
            (_, ValueTypeExt::Any) => base.clone(),
            (ValueTypeExt::Empty, _) => alt.clone(),
            (_, ValueTypeExt::Empty) => base.clone(),
            (ValueTypeExt::Base(b0), ValueTypeExt::Base(b1)) => {
                if b0 == b1 {
                    ValueTypeExt::Base(*b0)
                } else {
                    ValueTypeExt::EngineSpecific {
                        base_model: Box::new(base.clone()),
                        alt_model: Box::new(alt.clone()),
                    }
                }
            }
            (ValueTypeExt::Tuple(ts0), ValueTypeExt::Tuple(ts1)) => {
                if ts0.len() == ts1.len() {
                    let mut ts = Vec::with_capacity(ts0.len());
                    for (t0, t1) in Iterator::zip(ts0.iter(), ts1.iter()) {
                        ts.push(ValueTypeExt::merge(t0, t1));
                    }
                    ValueTypeExt::Tuple(ts)
                } else {
                    ValueTypeExt::EngineSpecific {
                        base_model: Box::new(base.clone()),
                        alt_model: Box::new(alt.clone()),
                    }
                }
            }
            (ValueTypeExt::Record(items0), ValueTypeExt::Record(items1)) => {
                if items0.len() == items1.len() {
                    let mut items = Vec::new();
                    for ((l0, t0), (l1, t1)) in Iterator::zip(items0.iter(), items1.iter()) {
                        if l0 != l1 {
                            return ValueTypeExt::EngineSpecific {
                                base_model: Box::new(base.clone()),
                                alt_model: Box::new(alt.clone()),
                            };
                        }
                        items.push((l0.clone(), ValueTypeExt::merge(t0, t1)));
                    }
                    ValueTypeExt::Record(items)
                } else {
                    ValueTypeExt::EngineSpecific {
                        base_model: Box::new(base.clone()),
                        alt_model: Box::new(alt.clone()),
                    }
                }
            }
            (ValueTypeExt::Union(items0), ValueTypeExt::Union(items1)) => {
                let keys0 = items0.keys().collect::<HashSet<_>>();
                let keys1 = items1.keys().collect::<HashSet<_>>();
                if keys0 == keys1 {
                    let mut items = BTreeMap::new();
                    for (k, t0) in items0.iter() {
                        let t1 = items1.get(k).unwrap();
                        let t = Self::merge(t0, t1);
                        items.insert(k.clone(), t);
                    }
                    ValueTypeExt::Union(items)
                } else {
                    ValueTypeExt::EngineSpecific {
                        base_model: Box::new(base.clone()),
                        alt_model: Box::new(alt.clone()),
                    }
                }
            }
            (ValueTypeExt::Seq(t0), ValueTypeExt::Seq(t1)) => {
                ValueTypeExt::Seq(Box::new(Self::merge(t0, t1)))
            }
            (ValueTypeExt::Option(t0), ValueTypeExt::Option(t1)) => {
                ValueTypeExt::Option(Box::new(Self::merge(t0, t1)))
            }
            (x @ ValueTypeExt::EngineSpecific { .. }, _)
            | (_, x @ ValueTypeExt::EngineSpecific { .. }) => {
                unreachable!("merge: normalized ValueTypeExt should not be EngineSpecific: {x:?}")
            }
            (x, y) => ValueTypeExt::EngineSpecific {
                base_model: Box::new(x.clone()),
                alt_model: Box::new(y.clone()),
            },
        }
    }

    /// Reduces top-level disjunctions to present a ValueType-compatible WHNF, if possible
    pub fn simplify(&self) -> Cow<'_, Self> {
        match self {
            ValueTypeExt::EngineSpecific {
                base_model,
                alt_model,
            } => {
                let base = base_model.normalize(ModelKind::BaseModel);
                let alt = alt_model.normalize(ModelKind::AltModel);
                Cow::Owned(Self::merge(base.as_ref(), alt.as_ref()))
            }
            _ => Cow::Borrowed(self),
        }
    }

    /// Solves all disjunctions using the argument `model`.
    pub fn normalize(&self, model: ModelKind) -> Cow<'_, Self> {
        if !self.depends_on_model() {
            return Cow::Borrowed(self);
        }
        match self {
            ValueTypeExt::EngineSpecific {
                base_model,
                alt_model,
            } => match model {
                ModelKind::BaseModel => base_model.normalize(model),
                ModelKind::AltModel => alt_model.normalize(model),
            },
            ValueTypeExt::Any | ValueTypeExt::Empty | ValueTypeExt::Base(_) => Cow::Borrowed(self),
            ValueTypeExt::Tuple(ts) => Cow::Owned(ValueTypeExt::Tuple(
                ts.iter().map(|t| t.normalize(model).into_owned()).collect(),
            )),
            ValueTypeExt::Record(fs) => Cow::Owned(ValueTypeExt::Record(
                fs.iter()
                    .map(|(l, t)| (l.clone(), t.normalize(model).into_owned()))
                    .collect(),
            )),
            ValueTypeExt::Union(branches) => Cow::Owned(ValueTypeExt::Union(
                branches
                    .iter()
                    .map(|(l, t)| (l.clone(), t.normalize(model).into_owned()))
                    .collect(),
            )),
            ValueTypeExt::Seq(elt) => Cow::Owned(ValueTypeExt::Seq(Box::new(
                elt.normalize(model).into_owned(),
            ))),
            ValueTypeExt::Option(inner) => Cow::Owned(ValueTypeExt::Option(Box::new(
                inner.normalize(model).into_owned(),
            ))),
        }
    }

    pub fn unify(
        &self,
        other: &ValueTypeExt,
    ) -> Result<ValueTypeExt, UnificationError<ValueTypeExt>> {
        match (self, other) {
            // NOTE - we have to specify these patterns before the similar cases for Empty because we want (Empty, Any) in either order to yield Empty
            (ValueTypeExt::Any, rhs) => Ok(rhs.clone()),
            (lhs, ValueTypeExt::Any) => Ok(lhs.clone()),

            (ValueTypeExt::Empty, rhs) => Ok(rhs.clone()),
            (lhs, ValueTypeExt::Empty) => Ok(lhs.clone()),

            (
                ValueTypeExt::EngineSpecific {
                    base_model,
                    alt_model,
                },
                other,
            ) => {
                let base_model =
                    Box::new(base_model.unify(&other.normalize(ModelKind::BaseModel))?);
                let alt_model = Box::new(alt_model.unify(&other.normalize(ModelKind::AltModel))?);
                Ok(ValueTypeExt::EngineSpecific {
                    base_model,
                    alt_model,
                })
            }
            (
                this,
                ValueTypeExt::EngineSpecific {
                    base_model,
                    alt_model,
                },
            ) => {
                let base_model = Box::new(this.normalize(ModelKind::BaseModel).unify(base_model)?);
                let alt_model = Box::new(this.normalize(ModelKind::AltModel).unify(alt_model)?);
                Ok(ValueTypeExt::EngineSpecific {
                    base_model,
                    alt_model,
                })
            }
            (ValueTypeExt::Base(b1), ValueTypeExt::Base(b2)) => {
                if b1 == b2 {
                    Ok(ValueTypeExt::Base(*b1))
                } else {
                    Err(UnificationError::Unsatisfiable(self.clone(), other.clone()))
                }
            }

            (ValueTypeExt::Tuple(ts1), ValueTypeExt::Tuple(ts2)) => {
                if ts1.len() != ts2.len() {
                    // tuple arity mismatch
                    return Err(UnificationError::Unsatisfiable(self.clone(), other.clone()));
                }
                let mut ts = Vec::new();
                for (t1, t2) in Iterator::zip(ts1.iter(), ts2.iter()) {
                    ts.push(t1.unify(t2)?);
                }
                Ok(ValueTypeExt::Tuple(ts))
            }
            (ValueTypeExt::Record(fs1), ValueTypeExt::Record(fs2)) => {
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
                Ok(ValueTypeExt::Record(fs))
            }
            (ValueTypeExt::Union(bs1), ValueTypeExt::Union(bs2)) => {
                let mut bs: BTreeMap<Label, ValueTypeExt> = BTreeMap::new();

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

                Ok(ValueTypeExt::Union(bs))
            }
            (ValueTypeExt::Seq(t1), ValueTypeExt::Seq(t2)) => {
                Ok(ValueTypeExt::Seq(Box::new(t1.unify(t2)?)))
            }
            (ValueTypeExt::Option(t1), ValueTypeExt::Option(t2)) => {
                Ok(ValueTypeExt::Option(Box::new(t1.unify(t2)?)))
            }
            (t1, t2) => Err(UnificationError::Unsatisfiable(t1.clone(), t2.clone())),
        }
    }

    pub(crate) fn tuple_proj(self, index: usize) -> AResult<ValueTypeExt> {
        match self {
            ValueTypeExt::Tuple(vs) => Ok(vs[index].clone()),
            ValueTypeExt::EngineSpecific {
                base_model,
                alt_model,
            } => {
                let base = base_model.tuple_proj(index)?;
                let alt = alt_model.tuple_proj(index)?;
                Ok(ValueTypeExt::merge(&base, &alt))
            }
            other => Err(anyhow!("tuple projection on non-tuple type {other:?}")),
        }
    }

    pub(crate) fn record_proj(&self, label: &str) -> AResult<ValueTypeExt> {
        match self {
            ValueTypeExt::Record(vs) => {
                for (l, v) in vs.iter() {
                    if l.as_ref() == label {
                        return Ok(v.clone());
                    }
                }
                unreachable!()
            }
            ValueTypeExt::EngineSpecific {
                base_model,
                alt_model,
            } => {
                let base = base_model.record_proj(label)?;
                let alt = alt_model.record_proj(label)?;
                Ok(ValueTypeExt::merge(&base, &alt))
            }
            other => Err(anyhow!("record projection on non-record type {other:?}")),
        }
    }

    pub fn try_to_valuetype(&self) -> Option<ValueType> {
        match self {
            ValueTypeExt::Any => Some(ValueType::Any),
            ValueTypeExt::Empty => Some(ValueType::Empty),
            ValueTypeExt::Base(b) => Some(ValueType::Base(b.clone())),
            ValueTypeExt::Tuple(v) => {
                let mut vs = Vec::with_capacity(v.len());
                for v in v {
                    vs.push(v.try_to_valuetype()?);
                }
                Some(ValueType::Tuple(vs))
            }
            ValueTypeExt::Record(v) => {
                let mut vs = Vec::with_capacity(v.len());
                for (l, v) in v {
                    vs.push((l.clone(), v.try_to_valuetype()?));
                }
                Some(ValueType::Record(vs))
            }
            ValueTypeExt::Union(v) => {
                let mut branches = BTreeMap::new();
                for (lab, t) in v {
                    branches.insert(lab.clone(), t.try_to_valuetype()?);
                }
                Some(ValueType::Union(branches))
            }
            ValueTypeExt::Seq(v) => Some(ValueType::Seq(Box::new(v.try_to_valuetype()?))),
            ValueTypeExt::Option(v) => Some(ValueType::Option(Box::new(v.try_to_valuetype()?))),
            ValueTypeExt::EngineSpecific { .. } => None,
        }
    }

    pub fn reify(self, compiler: &FormatCompiler) -> ValueType {
        match self {
            ValueTypeExt::EngineSpecific {
                base_model,
                alt_model,
            } => {
                if compiler.params.alternate_processing_model {
                    alt_model.reify(compiler)
                } else {
                    base_model.reify(compiler)
                }
            }
            ValueTypeExt::Any => ValueType::Any,
            ValueTypeExt::Empty => ValueType::Empty,
            ValueTypeExt::Base(b) => ValueType::Base(b),
            ValueTypeExt::Tuple(v) => {
                let mut vs = Vec::with_capacity(v.len());
                for v in v {
                    vs.push(v.reify(compiler));
                }
                ValueType::Tuple(vs)
            }
            ValueTypeExt::Record(v) => {
                let mut vs = Vec::with_capacity(v.len());
                for (l, v) in v {
                    vs.push((l.clone(), v.reify(compiler)));
                }
                ValueType::Record(vs)
            }
            ValueTypeExt::Union(v) => {
                let mut branches = BTreeMap::new();
                for (lab, t) in v {
                    branches.insert(lab, t.reify(compiler));
                }
                ValueType::Union(branches)
            }
            ValueTypeExt::Seq(v) => ValueType::Seq(Box::new(v.reify(compiler))),
            ValueTypeExt::Option(v) => ValueType::Option(Box::new(v.reify(compiler))),
        }
    }

    pub(crate) fn unwrap_tuple_type(&self) -> AResult<Vec<ValueTypeExt>> {
        match self.simplify().as_ref() {
            ValueTypeExt::Tuple(vs) => Ok(vs.clone()),
            other @ ValueTypeExt::EngineSpecific { .. } => Err(anyhow!(
                "tuple projection over irreducible disjunction: {other:?}"
            )),
            other => Err(anyhow!("tuple projection on non-tuple type {other:?}")),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FormatModuleExt {
    names: Vec<Label>,
    args: Vec<Vec<(Label, ValueTypeExt)>>,
    formats: Vec<FormatExt>,
    format_types: Vec<ValueTypeExt>,
}

impl FormatModuleExt {
    pub fn new() -> FormatModuleExt {
        FormatModuleExt {
            names: Vec::new(),
            args: Vec::new(),
            formats: Vec::new(),
            format_types: Vec::new(),
        }
    }

    pub fn compile(self, compiler: &FormatCompiler) -> FormatModule {
        FormatModule {
            names: self.names,
            args: self
                .args
                .into_iter()
                .map(|args| {
                    args.into_iter()
                        .map(|(name, t)| (name, t.reify(compiler)))
                        .collect()
                })
                .collect(),
            formats: self.formats.iter().map(|f| compiler.compile(f)).collect(),
            format_types: self
                .format_types
                .into_iter()
                .map(|t| t.reify(compiler))
                .collect(),
        }
    }

    pub fn define_format<Name: IntoLabel, F>(&mut self, name: Name, format: F) -> FormatRef
    where
        FormatExt: From<F>,
    {
        self.define_format_ext(name, FormatExt::from(format))
    }

    pub fn define_format_ext<Name: IntoLabel>(
        &mut self,
        name: Name,
        format_ext: FormatExt,
    ) -> FormatRef {
        self.define_format_args(name, vec![], format_ext)
    }

    pub fn define_format_args<Name: IntoLabel>(
        &mut self,
        name: Name,
        args: Vec<(Label, ValueTypeExt)>,
        format_ext: FormatExt,
    ) -> FormatRef {
        let mut scope = TypeScope::<'_, ValueTypeExt>::new();
        for (arg_name, arg_type) in &args {
            scope.push(arg_name.clone(), arg_type.clone());
        }
        let format_type = match self.infer_format_ext_type(&scope, &format_ext) {
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

    fn get_args(&self, level: usize) -> &[(Label, ValueTypeExt)] {
        &self.args[level]
    }

    pub fn get_format_type(&self, level: usize) -> &ValueTypeExt {
        &self.format_types[level]
    }
}

impl FormatModuleExt {
    pub(crate) fn infer_format_ext_type(
        &self,
        scope: &TypeScope<'_, ValueTypeExt>,
        f: &FormatExt,
    ) -> AResult<ValueTypeExt> {
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
                        let t = expr.infer_type_ext(scope)?;
                        let _t = arg_type.unify(&t)?;
                    }
                    Ok(self.get_format_type(*level).clone())
                }
                GroundFormat::Fail => Ok(ValueTypeExt::Empty),

                GroundFormat::SkipRemainder | GroundFormat::EndOfInput | GroundFormat::Align(_) => {
                    Ok(ValueTypeExt::UNIT)
                }

                GroundFormat::Byte(_bs) => Ok(ValueTypeExt::Base(BaseType::U8)),
                GroundFormat::Compute(expr) => Ok(expr.infer_type_ext(scope)?),

                // REVIEW - do we want to hard-code this as U64 or make it a flexibly abstract integer type?
                GroundFormat::Pos => Ok(ValueTypeExt::Base(BaseType::U64)),

                GroundFormat::Apply(name) => match scope.get_type_by_name(name) {
                    ValueKind::Format(t) => Ok(t.clone().into()),
                    ValueKind::Value(t) => Err(anyhow!("Apply: expected format, found {t:?}")),
                    ValueKind::View => Err(anyhow!("Apply: expected format, found View")),
                },
                GroundFormat::WithView(ident, vf) => {
                    ident.check_type_ext(scope)?;
                    match vf {
                        ViewFormatExt::CaptureBytes(len) => {
                            // confirm that length is of a numeric type
                            match len.infer_type_ext(scope)? {
                                t if t.is_numeric() => {}
                                other => {
                                    return Err(anyhow!(
                                        "ReadOffsetLen: expected numeric len, found {other:?}"
                                    ))
                                }
                            }
                            // TODO - consider if we need to add a valuetype for borrowed u8 slice in APM (?)
                            Ok(ValueTypeExt::Seq(Box::new(ValueTypeExt::Base(
                                BaseType::U8,
                            ))))
                        }
                    }
                }
            },
            FormatExt::Epi(epi_format) => match epi_format {
                EpiFormat::Mono(mono_kind, f) => {
                    match mono_kind {
                        MonoKind::Variant(label) => {
                            let t = self.infer_format_ext_type(scope, f)?;
                            Ok(ValueTypeExt::Union(BTreeMap::from([(label.clone(), t)])))
                        }
                        MonoKind::Repeat | MonoKind::Repeat1 => {
                            let t = self.infer_format_ext_type(scope, f)?;
                            Ok(ValueTypeExt::Seq(Box::new(t)))
                        }
                        MonoKind::RepeatCount(count) => {
                            match count.infer_type_ext(scope)? {
                                ValueTypeExt::Base(b) if b.is_numeric() => {
                                    let t = self.infer_format_ext_type(scope, f)?;
                                    Ok(ValueTypeExt::Seq(Box::new(t)))
                                }
                                ValueTypeExt::EngineSpecific { .. } => unreachable!("RepeatCount: unexpected engine-specific type in numeric position"),
                                other => Err(anyhow!("RepeatCount first argument type should be numeric, found {other:?} instead")),
                            }
                        }
                        MonoKind::RepeatBetween(min, max) => {
                            match min.infer_type_ext(scope)? {
                                ref t0 @ ValueTypeExt::Base(b0) if b0.is_numeric() => {
                                    match max.infer_type_ext(scope)? {
                                        ValueTypeExt::Base(b1) if b0 == b1 => {
                                            let t = self.infer_format_ext_type(scope, f)?;
                                            Ok(ValueTypeExt::Seq(Box::new(t)))
                                        }
                                    ValueTypeExt::EngineSpecific { .. } => unreachable!("RepeatBetween@1: unexpected engine-specific type in numeric position"),
                                        other => Err(anyhow!("RepeatBetween second argument type should be the same as the first, found {other:?} (!= {t0:?})")),
                                    }
                                }
                                ValueTypeExt::EngineSpecific { .. } => unreachable!("RepeatBetween@0: unexpected engine-specific type in numeric position"),
                                other => Err(anyhow!("RepeatBetween first argument type should be numeric, found {other:?} instead")),
                            }
                        }
                        MonoKind::LetView(ident) => {
                            let mut child_scope = TypeScope::child(scope);
                            child_scope.push_view(ident.clone());
                            self.infer_format_ext_type(scope, f)
                        }
                        MonoKind::RepeatUntilLast(lambda_elem) => {
                            match lambda_elem.as_ref() {
                                Expr::Lambda(head, expr) => {
                                    let t = self.infer_format_ext_type(scope, f)?;
                                    let mut child_scope = TypeScope::child(scope);
                                    child_scope.push(head.clone(), t.clone());
                                    let ret_type = expr.infer_type_ext(&child_scope)?;
                                    match ret_type {
                                        ValueTypeExt::Base(BaseType::Bool) => Ok(ValueTypeExt::Seq(Box::new(t))),
                                        ValueTypeExt::EngineSpecific { .. } => unreachable!("RepeatUntilLast@0.out: unexpected engine-specific type in boolean position"),
                                        other => Err(anyhow!("RepeatUntilLast first argument (lambda) return type should be Bool, found {other:?} instead")),
                                    }
                                }
                                other => Err(anyhow!("RepeatUntilLast first argument type should be lambda, found {other:?} instead")),
                            }
                        }
                        MonoKind::RepeatUntilSeq(lambda_seq) => {
                            match lambda_seq.as_ref() {
                                Expr::Lambda(head, expr) => {
                                    let t = self.infer_format_ext_type(scope, f)?;
                                    let mut child_scope = TypeScope::child(scope);
                                    child_scope.push(head.clone(), ValueTypeExt::Seq(Box::new(t.clone())));
                                    let ret_type = expr.infer_type_ext(&child_scope)?;
                                    match ret_type {
                                        ValueTypeExt::Base(BaseType::Bool) => Ok(ValueTypeExt::Seq(Box::new(t))),
                                        ValueTypeExt::EngineSpecific { .. } => unreachable!("RepeatUntilSeq@0.out: unexpected engine-specific type in boolean position"),
                                        other => Err(anyhow!("RepeatUntilSeq first argument (lambda) return type should be Bool, found {other:?} instead")),
                                    }
                                }
                                other => Err(anyhow!("RepeatUntilSeq first argument type should be lambda, found {other:?} instead")),
                            }
                        }
                        MonoKind::AccumUntil(lambda_acc_seq, lambda_acc_val, init, vt) => {
                            match lambda_acc_seq.as_ref() {
                                Expr::Lambda(head, expr) => {
                                    let t = self.infer_format_ext_type(scope, f)?;
                                    // Check that the initial accumulator value's type unifies with the type-claim
                                    let _acc_type = init.infer_type_ext(&scope)?.unify(vt.as_ref())?;
                                    let mut child_scope = TypeScope::child(scope);
                                    let t_seq = ValueTypeExt::Seq(Box::new(t.clone()));
                                    let vt_acc_seq = ValueTypeExt::Tuple(vec![vt.as_ref().clone(), t_seq.clone()]);
                                    child_scope.push(head.clone(), vt_acc_seq.clone());
                                    let ret_type = expr.infer_type_ext(&child_scope)?;
                                    match ret_type {
                                        ValueTypeExt::Base(BaseType::Bool) => {
                                            match lambda_acc_val.as_ref() {
                                                Expr::Lambda(head, expr) => {
                                                    let mut child_scope = TypeScope::child(&child_scope);
                                                    let vt_acc_elem = ValueTypeExt::Tuple(vec![vt.as_ref().clone(), t.clone()]);
                                                    child_scope.push(head.clone(), vt_acc_elem);
                                                    // we just need to check that these types unify, the value is unimportant
                                                    let _ret_type = expr.infer_type_ext(&child_scope)?.unify(vt.as_ref())?;
                                                    Ok(vt_acc_seq)
                                                }
                                                other => Err(anyhow!("AccumUntil second argument type should be lambda, found {other:?} instead")),
                                            }
                                        }
                                        ValueTypeExt::EngineSpecific { .. } => unreachable!("AccumUntil@0.out: unexpected engine-specific type in boolean position"),
                                        other => Err(anyhow!("AccumUntil first argument (lambda) return type should be Bool, found {other:?} instead")),
                                    }

                                }
                                other => Err(anyhow!("AccumUntil first argument type should be lambda, found {other:?} instead")),
                            }
                        }
                        MonoKind::ForEach(expr, lbl) => {
                            let expr_t = expr.infer_type_ext(scope)?;
                            let elem_t = match expr_t.simplify().as_ref() {
                                ValueTypeExt::Seq(elem_t) => (**elem_t).clone(),
                                _ => return Err(anyhow!("ForEach: expected Seq, found {expr_t:?}")),
                            };
                            let mut child_scope = TypeScope::child(scope);
                            child_scope.push(lbl.clone(), elem_t);
                            let inner_t = self.infer_format_ext_type(&child_scope, f)?;
                            Ok(ValueTypeExt::Seq(Box::new(inner_t)))
                        }
                        MonoKind::Maybe(x) => match x.infer_type_ext(scope)? {
                            ValueTypeExt::Base(BaseType::Bool) => {
                                let t = self.infer_format_ext_type(scope, f)?;
                                Ok(ValueTypeExt::Option(Box::new(t)))
                            }
                            other => Err(anyhow!(
                                "Maybe-predicate is not a bool-type: {x:?} ~ {other:?}"
                            )),
                        },
                        MonoKind::Peek => self.infer_format_ext_type(scope, f),
                        MonoKind::PeekNot => Ok(ValueTypeExt::UNIT),
                        MonoKind::Slice(expr) => {
                            debug_assert!(expr.infer_type_ext(scope).is_ok_and(|vt| vt.is_numeric()));
                            self.infer_format_ext_type(scope, f)
                        }
                        MonoKind::Bits => self.infer_format_ext_type(scope, f),
                        MonoKind::WithRelativeOffset(_base_addr, _offs) => {
                            self.infer_format_ext_type(scope, f)
                        }
                        MonoKind::Map(expr) => {
                            let arg_t = self.infer_format_ext_type(scope, f)?;
                            match expr.as_ref() {
                                Expr::Lambda(name, body) => {
                                    let mut child_scope = TypeScope::child(scope);
                                    child_scope.push(name.clone(), arg_t);
                                    body.infer_type_ext(&child_scope)
                                }
                                other => Err(anyhow!("Map: expected lambda, found {other:?}")),
                            }
                        }
                        MonoKind::Where(expr) => {
                            let arg_type = self.infer_format_ext_type(scope, f)?;
                            match expr.as_ref() {
                                Expr::Lambda(name, body) => {
                                    let mut child_scope = TypeScope::child(scope);
                                    child_scope.push(name.clone(), arg_type.clone());
                                    let t = body.infer_type_ext(&child_scope)?;
                                    if t != ValueTypeExt::Base(BaseType::Bool) {
                                        return Err(anyhow!("Where: expected bool lambda, found {t:?}"));
                                    }
                                    Ok(arg_type)
                                }
                                other => Err(anyhow!("Where: expected lambda, found {other:?}")),
                            }
                        }
                        MonoKind::Let(name, expr) => {
                            let t = expr.infer_type_ext(scope)?;
                            let mut child_scope = TypeScope::child(scope);
                            child_scope.push(name.clone(), t);
                            self.infer_format_ext_type(&child_scope, f)
                        }
                        MonoKind::Dynamic(name, dyn_format) => {
                            match dyn_format {
                                DynFormat::Huffman(lengths_expr, _opt_values_expr) => match lengths_expr.infer_type_ext(scope)? {
                                    ValueTypeExt::Seq(t) => match &*t {
                                        ValueTypeExt::Base(BaseType::U8 | BaseType::U16) => {
                                            let mut child_scope = TypeScope::child(scope);
                                            child_scope.push(name.clone(), ValueTypeExt::Base(BaseType::U16));
                                            self.infer_format_ext_type(&child_scope, f)
                                        }
                                        other => Err(anyhow!("Huffman: expected U8 or U16, found {other:?}")),
                                    }
                                    other => Err(anyhow!("Huffman: expected Seq, found {other:?}")),
                                }
                            }
                        }
                        MonoKind::DecodeBytes(bytes) => {
                            let bytes_type = bytes.infer_type_ext(scope)?;
                            match bytes_type {
                                ValueTypeExt::Seq(bt) if matches!(*bt, ValueTypeExt::Base(BaseType::U8)) => {
                                    self.infer_format_ext_type(scope, f)
                                }
                                other => Err(anyhow!("DecodeBytes first argument type should be Seq(U8), found {other:?} instead")),
                            }
                        }
                        MonoKind::ParseFromView(v_expr) => {
                            v_expr.check_type_ext(scope)?;
                            self.infer_format_ext_type(scope, f)
                        }
                        MonoKind::Hint(_) => {
                            self.infer_format_ext_type(scope, f)
                        }
                    }
                }
                EpiFormat::Duo(duo_kind, f0, f1) => match duo_kind {
                    DuoKind::MonadSeq => {
                        let _ = self.infer_format_ext_type(scope, f0)?;
                        self.infer_format_ext_type(scope, f1)
                    }
                    DuoKind::LetFormat(name) => {
                        let t0 = self.infer_format_ext_type(scope, f0)?;
                        let mut new_scope = TypeScope::child(scope);
                        new_scope.push(name.clone(), t0);
                        self.infer_format_ext_type(&new_scope, f1)
                    }
                },
                EpiFormat::Poly(poly_kind, fs) => match poly_kind {
                    PolyKind::Union | PolyKind::UnionNondet => {
                        let mut t = ValueTypeExt::Empty;
                        for f in fs {
                            let t0 = self.infer_format_ext_type(scope, f)?;
                            t = t.unify(&t0)?;
                        }
                        Ok(t)
                    }
                    PolyKind::Tuple => {
                        let mut ts = Vec::new();
                        for f in fs {
                            let t0 = self.infer_format_ext_type(scope, f)?;
                            ts.push(t0);
                        }
                        Ok(ValueTypeExt::Tuple(ts))
                    }
                    PolyKind::Sequence => {
                        let mut elem_t = ValueTypeExt::Any;
                        for f in fs {
                            let t0 = self.infer_format_ext_type(scope, f)?;
                            elem_t = elem_t.unify(&t0)?;
                        }
                        Ok(ValueTypeExt::Seq(Box::new(elem_t)))
                    }
                },
                EpiFormat::Pat(pat_kind, pat_fs) => match pat_kind {
                    PatKind::Match(head) => {
                        if pat_fs.is_empty() {
                            return Err(anyhow!("infer_format_type: empty Match"));
                        }
                        let head_type = Rc::new(head.infer_type_ext(scope)?);
                        let mut t = ValueTypeExt::Any;
                        for (pattern, branch) in pat_fs {
                            let t0 = pattern.infer_format_branch_type_ext(
                                scope,
                                head_type.clone(),
                                self,
                                branch,
                            )?;
                            t = t.unify(&t0)?;
                        }
                        Ok(t)
                    }
                },
                EpiFormat::Opt(opt_kind, opt_f) => match opt_kind {
                    OptKind::LiftedOption => match opt_f {
                        None => Ok(ValueTypeExt::Option(Box::new(ValueTypeExt::Any))),
                        Some(f) => {
                            let t = self.infer_format_ext_type(scope, f)?;
                            Ok(ValueTypeExt::Option(Box::new(t)))
                        }
                    },
                },
            },
            FormatExt::Meta(meta_format) => match meta_format {
                MetaFormat::EngineSpecific {
                    base_model,
                    alt_model,
                } => {
                    let base_t = self.infer_format_ext_type(scope, base_model)?;
                    let alt_t = self.infer_format_ext_type(scope, alt_model)?;
                    Ok(ValueTypeExt::EngineSpecific {
                        base_model: Box::new(base_t),
                        alt_model: Box::new(alt_t),
                    })
                }
            },
        }
    }
}

mod __impls {
    use super::*;
    use crate::{Arith, TypeHint, UnaryOp, ViewFormat};
    use std::rc::Rc;

    impl Expr {
        pub(crate) fn infer_type_ext(
            &self,
            scope: &TypeScope<'_, ValueTypeExt>,
        ) -> AResult<ValueTypeExt> {
            match self {
                Expr::Var(name) => match scope.get_type_by_name(name) {
                    ValueKind::Value(t) => Ok(t.clone()),
                    ValueKind::Format(_t) => Err(anyhow!(
                        "expected ValueKind::Value, found ValueKind::Format for var {name}"
                    )),
                    ValueKind::View => Err(anyhow!(
                        "expected ValueKind::Value, found ValueKind::View for var {name}"
                    )),
                },
                Expr::Bool(_b) => Ok(ValueTypeExt::Base(BaseType::Bool)),
                Expr::U8(_n) => Ok(ValueTypeExt::Base(BaseType::U8)),
                Expr::U16(_n) => Ok(ValueTypeExt::Base(BaseType::U16)),
                Expr::U32(_n) => Ok(ValueTypeExt::Base(BaseType::U32)),
                Expr::U64(_n) => Ok(ValueTypeExt::Base(BaseType::U64)),
                Expr::Tuple(exprs) => {
                    let mut ts = Vec::new();
                    for expr in exprs {
                        ts.push(expr.infer_type_ext(scope)?);
                    }
                    Ok(ValueTypeExt::Tuple(ts))
                }
                Expr::TupleProj(head, index) => {
                    let t = head.infer_type_ext(scope)?;
                    t.tuple_proj(*index)
                }
                Expr::Record(fields) => {
                    let mut fs = Vec::new();
                    for (label, expr) in fields {
                        fs.push((label.clone(), expr.infer_type_ext(scope)?));
                    }
                    Ok(ValueTypeExt::Record(fs))
                }
                Expr::RecordProj(head, label) => head.infer_type_ext(scope)?.record_proj(label),
                Expr::Variant(label, expr) => Ok(ValueTypeExt::Union(BTreeMap::from([(
                    label.clone(),
                    expr.infer_type_ext(scope)?,
                )]))),
                Expr::Seq(exprs) => {
                    let mut t = ValueTypeExt::Any;
                    for e in exprs {
                        t = t.unify(&e.infer_type_ext(scope)?)?;
                    }
                    Ok(ValueTypeExt::Seq(Box::new(t)))
                }
                Expr::Match(head, branches) => {
                    if branches.is_empty() {
                        return Err(anyhow!("cannot infer type of empty match expression"));
                    }
                    let head_type = Rc::new(head.infer_type_ext(scope)?);
                    let mut t = ValueTypeExt::Any;
                    for (pattern, branch) in branches {
                        t = t.unify(&pattern.infer_expr_branch_type_ext(
                            scope,
                            head_type.clone(),
                            branch,
                        )?)?;
                    }
                    Ok(t)
                }
                Expr::Destructure(head, pattern, body) => {
                    let head_type = Rc::new(head.infer_type_ext(scope)?);
                    pattern.infer_expr_branch_type_ext(scope, head_type, body)
                }
                Expr::Lambda(..) => Err(anyhow!("infer_type encountered unexpected lambda")),

                Expr::IntRel(_rel, x, y) => {
                    match (x.infer_type_ext(scope)?, y.infer_type_ext(scope)?) {
                        (ValueTypeExt::Base(b1), ValueTypeExt::Base(b2))
                            if b1 == b2 && b1.is_numeric() =>
                        {
                            Ok(ValueTypeExt::Base(BaseType::Bool))
                        }
                        (x, y) => Err(anyhow!(
                            "mismatched operand types for {_rel:?}: {x:?}, {y:?}"
                        )),
                    }
                }
                Expr::Arith(_arith @ (Arith::BoolAnd | Arith::BoolOr), x, y) => {
                    match (x.infer_type_ext(scope)?, y.infer_type_ext(scope)?) {
                        (
                            ValueTypeExt::Base(BaseType::Bool),
                            ValueTypeExt::Base(BaseType::Bool),
                        ) => Ok(ValueTypeExt::Base(BaseType::Bool)),
                        (x, y) => Err(anyhow!(
                            "mismatched operand types for {_arith:?}: {x:?}, {y:?}"
                        )),
                    }
                }

                Expr::Arith(_arith, x, y) => {
                    match (x.infer_type_ext(scope)?, y.infer_type_ext(scope)?) {
                        (ValueTypeExt::Base(b1), ValueTypeExt::Base(b2))
                            if b1 == b2 && b1.is_numeric() =>
                        {
                            Ok(ValueTypeExt::Base(b1))
                        }
                        (x, y) => Err(anyhow!(
                            "mismatched operand types for {_arith:?}: {x:?}, {y:?}"
                        )),
                    }
                }
                Expr::Unary(_op @ UnaryOp::BoolNot, x) => match x.infer_type_ext(scope)? {
                    ValueTypeExt::Base(BaseType::Bool) => Ok(ValueTypeExt::Base(BaseType::Bool)),
                    x => Err(anyhow!("unexpected operand type for {_op:?}: {x:?}")),
                },
                Expr::Unary(_op @ (UnaryOp::IntSucc | UnaryOp::IntPred), x) => {
                    match x.infer_type_ext(scope)? {
                        ValueTypeExt::Base(b) if b.is_numeric() => Ok(ValueTypeExt::Base(b)),
                        x => Err(anyhow!("unexpected operand type for {_op:?}: {x:?}")),
                    }
                }

                Expr::AsU8(x) => match x.infer_type_ext(scope)? {
                    ValueTypeExt::Base(b) if b.is_numeric() => Ok(ValueTypeExt::Base(BaseType::U8)),
                    x => Err(anyhow!("unsound type cast AsU8(_ : {x:?})")),
                },
                Expr::AsU16(x) => match x.infer_type_ext(scope)? {
                    ValueTypeExt::Base(b) if b.is_numeric() => {
                        Ok(ValueTypeExt::Base(BaseType::U16))
                    }
                    x => Err(anyhow!("unsound type cast AsU16(_ : {x:?})")),
                },
                Expr::AsU32(x) => match x.infer_type_ext(scope)? {
                    ValueTypeExt::Base(b) if b.is_numeric() => {
                        Ok(ValueTypeExt::Base(BaseType::U32))
                    }
                    x => Err(anyhow!("unsound type cast AsU32(_ : {x:?})")),
                },
                Expr::AsU64(x) => match x.infer_type_ext(scope)? {
                    ValueTypeExt::Base(b) if b.is_numeric() => {
                        Ok(ValueTypeExt::Base(BaseType::U64))
                    }
                    x => Err(anyhow!("cannot convert {x:?} to U64")),
                },
                Expr::AsChar(x) => match x.infer_type_ext(scope)? {
                    ValueTypeExt::Base(b) if b.is_numeric() => {
                        Ok(ValueTypeExt::Base(BaseType::Char))
                    }
                    x => Err(anyhow!("unsound type cast AsChar(_ : {x:?})")),
                },
                Expr::U16Be(bytes) => {
                    let _t = bytes.infer_type_ext(scope)?;
                    match _t.as_tuple_type() {
                        [ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8)] => {
                            Ok(ValueTypeExt::Base(BaseType::U16))
                        }
                        _ => Err(anyhow!("unsound byte-level type cast U16Be(_ : {_t:?})")),
                    }
                }
                Expr::U16Le(bytes) => {
                    let _t = bytes.infer_type_ext(scope)?;
                    match _t.as_tuple_type() {
                        [ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8)] => {
                            Ok(ValueTypeExt::Base(BaseType::U16))
                        }
                        _ => Err(anyhow!("unsound byte-level type cast U16Le(_ : {_t:?})")),
                    }
                }
                Expr::U32Be(bytes) => {
                    let _t = bytes.infer_type_ext(scope)?;
                    match _t.as_tuple_type() {
                        [ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8)] => {
                            Ok(ValueTypeExt::Base(BaseType::U32))
                        }
                        _ => Err(anyhow!("unsound byte-level type cast U32Be(_ : {_t:?})")),
                    }
                }
                Expr::U32Le(bytes) => {
                    let _t = bytes.infer_type_ext(scope)?;
                    match _t.as_tuple_type() {
                        [ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8)] => {
                            Ok(ValueTypeExt::Base(BaseType::U32))
                        }
                        _ => Err(anyhow!("unsound byte-level type cast U32Le(_ : {_t:?})")),
                    }
                }
                Expr::U64Be(bytes) | Expr::U64Le(bytes) => {
                    let _t = bytes.infer_type_ext(scope)?;
                    match _t.as_tuple_type() {
                        [ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8), ValueTypeExt::Base(BaseType::U8)] => {
                            Ok(ValueTypeExt::Base(BaseType::U64))
                        }
                        other => Err(anyhow!(
                            "U64Be/Le: expected (U8, U8, U8, U8, U8, U8, U8, U8), found {other:#?}"
                        )),
                    }
                }
                Expr::SeqLength(seq) => match seq.infer_type_ext(scope)? {
                    ValueTypeExt::Seq(_t) => Ok(ValueTypeExt::Base(BaseType::U32)),
                    other => Err(anyhow!("seq-length called on non-sequence type: {other:?}")),
                },
                Expr::SeqIx(seq, index) => match seq.infer_type_ext(scope)? {
                    ValueTypeExt::Seq(t) => {
                        let index_type = index.infer_type_ext(scope)?;
                        if index_type != ValueTypeExt::Base(BaseType::U32) {
                            return Err(anyhow!(
                                "SeqIx `index` param: expected U32, found {index_type:?}"
                            ));
                        }
                        Ok(ValueTypeExt::clone(&t))
                    }
                    other => Err(anyhow!("SeqIx: expected Seq, found {other:?}")),
                },
                Expr::SubSeq(seq, start, length) => match seq.infer_type_ext(scope)? {
                    ValueTypeExt::Seq(t) => {
                        let start_type = start.infer_type_ext(scope)?;
                        let length_type = length.infer_type_ext(scope)?;
                        if start_type != ValueTypeExt::Base(BaseType::U32) {
                            return Err(anyhow!(
                                "SubSeq `start` param: expected U32, found {start_type:?}"
                            ));
                        }
                        if length_type != ValueTypeExt::Base(BaseType::U32) {
                            return Err(anyhow!(
                                "SubSeq length must be numeric, found {length_type:?}"
                            ));
                        }
                        Ok(ValueTypeExt::Seq(t))
                    }
                    other => Err(anyhow!("SubSeq: expected Seq, found {other:?}")),
                },
                Expr::SubSeqInflate(seq, start, length) => match seq.infer_type_ext(scope)? {
                    ValueTypeExt::Seq(t) => {
                        let start_type = start.infer_type_ext(scope)?;
                        let length_type = length.infer_type_ext(scope)?;
                        if start_type != ValueTypeExt::Base(BaseType::U32) {
                            return Err(anyhow!(
                                "SubSeqInflate `start` param: expected U32, found {start_type:?}"
                            ));
                        }
                        if length_type != ValueTypeExt::Base(BaseType::U32) {
                            return Err(anyhow!(
                                "SubSeqInflate length must be numeric, found {length_type:?}"
                            ));
                        }
                        Ok(ValueTypeExt::Seq(t))
                    }
                    other => Err(anyhow!("SubSeqInflate: expected Seq, found {other:?}")),
                },
                Expr::FlatMap(expr, seq) => match expr.as_ref() {
                    Expr::Lambda(name, expr) => match seq.infer_type_ext(scope)? {
                        ValueTypeExt::Seq(t) => {
                            let mut child_scope = TypeScope::child(scope);
                            child_scope.push(name.clone(), *t);
                            match expr.infer_type_ext(&child_scope)? {
                                ValueTypeExt::Seq(t2) => Ok(ValueTypeExt::Seq(t2)),
                                other => Err(anyhow!("FlatMap: expected Seq, found {other:?}")),
                            }
                        }
                        other => Err(anyhow!("FlatMap: expected Seq, found {other:?}")),
                    },
                    other => Err(anyhow!("FlatMap: expected Lambda, found {other:?}")),
                },
                Expr::FlatMapAccum(expr, accum, accum_type, seq) => match expr.as_ref() {
                    Expr::Lambda(name, expr) => match seq.infer_type_ext(scope)? {
                        ValueTypeExt::Seq(t) => {
                            let accum_type: ValueTypeExt =
                                (**accum_type.into_inner()).clone().into();
                            let accum_type = accum.infer_type_ext(scope)?.unify(&accum_type)?;
                            let mut child_scope = TypeScope::child(scope);
                            child_scope.push(
                                name.clone(),
                                ValueTypeExt::Tuple(vec![accum_type.clone(), *t]),
                            );
                            match expr
                                .infer_type_ext(&child_scope)?
                                .unwrap_tuple_type()?
                                .as_mut_slice()
                            {
                                [accum_result, ValueTypeExt::Seq(t2)] => {
                                    accum_result.unify(&accum_type)?;
                                    Ok(ValueTypeExt::Seq(t2.clone()))
                                }
                                _ => Err(anyhow!("FlatMapAccum: expected two values")),
                            }
                        }
                        other => Err(anyhow!("FlatMapAccum: expected Seq, found {other:?}")),
                    },
                    other => Err(anyhow!("FlatMapAccum: expected Lambda, found {other:?}")),
                },
                Expr::LeftFold(expr, accum, accum_type, seq) => match expr.as_ref() {
                    Expr::Lambda(name, expr) => match seq.infer_type_ext(scope)? {
                        ValueTypeExt::Seq(t) => {
                            let accum_type: ValueTypeExt =
                                (**accum_type.into_inner()).clone().into();
                            let accum_type = accum.infer_type_ext(scope)?.unify(&accum_type)?;
                            let mut child_scope = TypeScope::child(scope);
                            child_scope.push(
                                name.clone(),
                                ValueTypeExt::Tuple(vec![accum_type.clone(), *t]),
                            );
                            Ok(expr.infer_type_ext(&child_scope)?.unify(&accum_type)?)
                        }
                        other => Err(anyhow!("LeftFold: expected Seq, found {other:?}")),
                    },
                    other => Err(anyhow!("LeftFold: expected Lambda, found {other:?}")),
                },
                Expr::FindByKey(_is_sorted, get_key, query_key, seq) => match get_key.as_ref() {
                    Expr::Lambda(name, expr) => match seq.infer_type_ext(scope)? {
                        ValueTypeExt::Seq(t) => {
                            let key_type = query_key.infer_type_ext(scope)?;
                            let mut child_scope = TypeScope::child(scope);
                            child_scope.push(name.clone(), t.as_ref().clone());
                            match expr.infer_type_ext(&child_scope)?.unify(&key_type)? {
                                ValueTypeExt::Base(b) if b.is_numeric() => {
                                    Ok(ValueTypeExt::Option(t))
                                }
                                other => Err(anyhow!(
                                    "FindByKey: expected numeric key type, found {other:?}"
                                )),
                            }
                        }
                        other => Err(anyhow!("FindByKey: expected Seq, found {other:?}")),
                    },
                    other => Err(anyhow!("FindByKey: Expected Lambda, found {other:?}")),
                },
                Expr::FlatMapList(expr, ret_type, seq) => match expr.as_ref() {
                    Expr::Lambda(name, expr) => match seq.infer_type_ext(scope)? {
                        ValueTypeExt::Seq(t) => {
                            let ret_type: ValueTypeExt = (**ret_type.into_inner()).clone().into();
                            let mut child_scope = TypeScope::child(scope);
                            child_scope.push(
                                name.clone(),
                                ValueTypeExt::Tuple(vec![
                                    ValueTypeExt::Seq(Box::new(ret_type)),
                                    *t,
                                ]),
                            );
                            match expr.infer_type_ext(&child_scope)? {
                                ValueTypeExt::Seq(t2) => Ok(ValueTypeExt::Seq(t2)),
                                other => Err(anyhow!("FlatMapList: expected Seq, found {other:?}")),
                            }
                        }
                        other => Err(anyhow!("FlatMapList: expected Seq, found {other:?}")),
                    },
                    other => Err(anyhow!("FlatMapList: expected Lambda, found {other:?}")),
                },
                Expr::EnumFromTo(start, end) => {
                    let start_type = start.infer_type_ext(scope)?;
                    let end_type = end.infer_type_ext(scope)?;

                    if !matches!(start_type, ValueTypeExt::Base(b) if b.is_numeric()) {
                        return Err(anyhow!("EnumFromTo: start is not numeric: {start_type:?}"));
                    } else if start_type != end_type {
                        return Err(anyhow!(
                            "EnumFromTo: start and end do not agree: {start_type:?} != {end_type:?}"
                        ));
                    }

                    Ok(ValueTypeExt::Seq(Box::new(start_type)))
                }
                Expr::Dup(count, expr) => {
                    if count.infer_type_ext(scope)? != ValueTypeExt::Base(BaseType::U32) {
                        return Err(anyhow!("Dup: count is not U32: {count:?}"));
                    }
                    let t = expr.infer_type_ext(scope)?;
                    Ok(ValueTypeExt::Seq(Box::new(t)))
                }
                Expr::LiftOption(expr) => match expr {
                    Some(expr) => Ok(ValueTypeExt::Option(Box::new(expr.infer_type_ext(scope)?))),
                    None => Ok(ValueTypeExt::Option(Box::new(ValueTypeExt::Any))),
                },
                Expr::Append(lhs, rhs) => {
                    let lhs_type = lhs.infer_type_ext(scope)?;
                    let rhs_type = rhs.infer_type_ext(scope)?;
                    match (&lhs_type, &rhs_type) {
                        (ValueTypeExt::Seq(t1), ValueTypeExt::Seq(t2)) => {
                            let elem_t = t1.unify(&t2)?;
                            Ok(ValueTypeExt::Seq(Box::new(elem_t)))
                        }
                        (ValueTypeExt::Seq(..), other) => {
                            Err(anyhow!("Append: rhs is not Seq: {other:?}"))
                        }
                        (other, ValueTypeExt::Seq(..)) => {
                            Err(anyhow!("Append: lhs is not Seq: {other:?}"))
                        }
                        (lhs_type, rhs_type) => {
                            return Err(anyhow!(
                                "Append: lhs and rhs must be Seq: {lhs_type:?}, {rhs_type:?} !~ Seq(_)"
                            ));
                        }
                    }
                }
            }
        }
    }

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

    impl From<Format> for FormatExt {
        fn from(value: Format) -> Self {
            match value {
                Format::ItemVar(level, exprs) => {
                    FormatExt::Ground(GroundFormat::ItemVar(level, exprs))
                }
                Format::Fail => FormatExt::Ground(GroundFormat::Fail),
                Format::EndOfInput => FormatExt::Ground(GroundFormat::EndOfInput),
                Format::Align(n) => FormatExt::Ground(GroundFormat::Align(n)),
                Format::Byte(byte_set) => FormatExt::Ground(GroundFormat::Byte(byte_set)),
                Format::Variant(name, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Variant(name),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Union(formats) => FormatExt::Epi(EpiFormat::Poly(
                    PolyKind::Union,
                    formats.into_iter().map(FormatExt::from).collect(),
                )),
                Format::UnionNondet(formats) => FormatExt::Epi(EpiFormat::Poly(
                    PolyKind::UnionNondet,
                    formats.into_iter().map(FormatExt::from).collect(),
                )),
                Format::Tuple(formats) => FormatExt::Epi(EpiFormat::Poly(
                    PolyKind::Tuple,
                    formats.into_iter().map(FormatExt::from).collect(),
                )),
                Format::Sequence(formats) => FormatExt::Epi(EpiFormat::Poly(
                    PolyKind::Sequence,
                    formats.into_iter().map(FormatExt::from).collect(),
                )),
                Format::Repeat(format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Repeat,
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Repeat1(format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Repeat1,
                    Box::new(FormatExt::from(*format)),
                )),
                Format::RepeatCount(expr, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::RepeatCount(expr),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::RepeatBetween(expr, expr1, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::RepeatBetween(expr, expr1),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::RepeatUntilLast(expr, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::RepeatUntilLast(expr),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::RepeatUntilSeq(expr, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::RepeatUntilSeq(expr),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::AccumUntil(expr, expr1, expr2, type_hint, format) => {
                    FormatExt::Epi(EpiFormat::Mono(
                        MonoKind::AccumUntil(expr, expr1, expr2, type_hint.into()),
                        Box::new(FormatExt::from(*format)),
                    ))
                }
                Format::ForEach(expr, name, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::ForEach(expr, name),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Maybe(expr, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Maybe(expr),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Peek(format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Peek,
                    Box::new(FormatExt::from(*format)),
                )),
                Format::PeekNot(format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::PeekNot,
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Slice(expr, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Slice(expr),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Bits(format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Bits,
                    Box::new(FormatExt::from(*format)),
                )),
                Format::WithRelativeOffset(expr, expr1, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::WithRelativeOffset(expr, expr1),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Map(format, expr) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Map(expr),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Where(format, expr) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Where(expr),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Compute(expr) => FormatExt::Ground(GroundFormat::Compute(expr)),
                Format::Let(name, expr, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Let(name, expr),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::LetView(name, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::LetView(name),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Match(expr, items) => FormatExt::Epi(EpiFormat::Pat(
                    PatKind::Match(expr),
                    items
                        .into_iter()
                        .map(|(p, f)| (p, FormatExt::from(f)))
                        .collect(),
                )),
                Format::Dynamic(name, dyn_format, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Dynamic(name, dyn_format),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::Apply(name) => FormatExt::Ground(GroundFormat::Apply(name)),
                Format::Pos => FormatExt::Ground(GroundFormat::Pos),
                Format::SkipRemainder => FormatExt::Ground(GroundFormat::SkipRemainder),
                Format::DecodeBytes(expr, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::DecodeBytes(expr),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::ParseFromView(v_expr, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::ParseFromView(v_expr),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::LetFormat(format, name, format1) => FormatExt::Epi(EpiFormat::Duo(
                    DuoKind::LetFormat(name),
                    Box::new(FormatExt::from(*format)),
                    Box::new(FormatExt::from(*format1)),
                )),
                Format::MonadSeq(format, format1) => FormatExt::Epi(EpiFormat::Duo(
                    DuoKind::MonadSeq,
                    Box::new(FormatExt::from(*format)),
                    Box::new(FormatExt::from(*format1)),
                )),
                Format::Hint(style_hint, format) => FormatExt::Epi(EpiFormat::Mono(
                    MonoKind::Hint(style_hint),
                    Box::new(FormatExt::from(*format)),
                )),
                Format::LiftedOption(Some(format)) => FormatExt::Epi(EpiFormat::Opt(
                    OptKind::LiftedOption,
                    Some(Box::new(FormatExt::from(*format))),
                )),
                Format::LiftedOption(None) => {
                    FormatExt::Epi(EpiFormat::Opt(OptKind::LiftedOption, None))
                }
                Format::WithView(name, view_format) => {
                    let view_format_ext = ViewFormatExt::from(view_format);
                    FormatExt::Ground(GroundFormat::WithView(name, view_format_ext))
                }
            }
        }
    }

    impl From<ViewFormat> for ViewFormatExt {
        fn from(value: ViewFormat) -> Self {
            match value {
                ViewFormat::CaptureBytes(len) => ViewFormatExt::CaptureBytes(len),
            }
        }
    }

    impl From<ViewFormatExt> for ViewFormat {
        fn from(value: ViewFormatExt) -> Self {
            match value {
                ViewFormatExt::CaptureBytes(len) => ViewFormat::CaptureBytes(len),
            }
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
                GroundFormat::WithView(v_expr, vfx) => Format::WithView(v_expr, vfx.into()),
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
                    // FIXME - we have to hardcode this based on the current design
                    let type_hint = TypeHint::from(
                        type_hint.as_ref().clone().reify(&FormatCompiler::default()),
                    );
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
                MonoKind::ParseFromView(v_expr) => Format::ParseFromView(v_expr, inner),
                MonoKind::Hint(style_hint) => Format::Hint(style_hint, inner),
                MonoKind::LetView(ident) => Format::LetView(ident, inner),
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

    impl AsRef<ValueTypeExt> for TypeHintExt {
        fn as_ref(&self) -> &ValueTypeExt {
            self.0.as_ref()
        }
    }

    impl Serialize for TypeHintExt {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    impl From<ValueTypeExt> for TypeHintExt {
        fn from(t: ValueTypeExt) -> Self {
            Self(Container::new(t))
        }
    }

    impl From<TypeHint> for TypeHintExt {
        fn from(t: TypeHint) -> Self {
            let inner: ValueTypeExt = t.as_ref().clone().into();
            Self(Container::new(inner))
        }
    }
}
