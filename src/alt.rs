pub mod prelude;

use crate::{
    ByteSet, DynFormat, Expr, Format, FormatModule, Label, Pattern, StyleHint, TypeHint, ValueType,
};
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
