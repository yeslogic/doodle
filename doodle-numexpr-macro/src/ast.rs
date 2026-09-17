//! Surface-syntax AST for the `numexpr!` macro, parsed directly out of the
//! invocation's `TokenStream` (see `parse.rs`) and lowered to a
//! `doodle::numeric::core::Expr`-construction `TokenStream` (see `codegen.rs`).
//!
//! This is a first working slice covering only numeric/string literals,
//! parenthesization, and left-associative binary operators with an optional
//! forced output representation. Unary prefix operators (`abs`/`succ`/`pred`/`-`)
//! and casts (`as`/`into`) are not yet implemented.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MRep {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

impl MRep {
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        match suffix {
            "u8" => Some(MRep::U8),
            "u16" => Some(MRep::U16),
            "u32" => Some(MRep::U32),
            "u64" => Some(MRep::U64),
            "i8" => Some(MRep::I8),
            "i16" => Some(MRep::I16),
            "i32" => Some(MRep::I32),
            "i64" => Some(MRep::I64),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinOpCode {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

/// Operator-level unary operations. Note that `-` on a bare integer literal
/// (e.g. `-5`) does *not* produce a `Negate` node — see the parser's
/// `parse_unary_expr`, which folds that case into `Expr::Const`'s inherent
/// sign instead. `Negate` only arises from `-` applied to anything else
/// (a variable, a parenthesized expression, another unary op, ...).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnaryOpCode {
    Negate,
    AbsVal,
    IntSucc,
    IntPred,
}

/// Cast semantics, mirroring `doodle::numeric::core::CastSemantics`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CastSemantics {
    /// `into MRep` — value-preserving cast (the surface-syntax default in
    /// the source `.lalrpop` grammar).
    Arithmetic,
    /// `as MRep` — matches real Rust `as`-cast (reinterpreting) semantics.
    Bitwise,
}

pub enum Expr {
    /// A numeric constant. Its sign is inherent to the constant (not an
    /// applied operator) — see `UnaryOpCode::Negate`'s doc comment.
    Const { value: i64, rep: Option<MRep> },
    /// A numeric variable reference, spelled as a string literal (e.g. `"x"`).
    Var(syn::LitStr),
    UnaryOp {
        op: UnaryOpCode,
        out_rep: Option<MRep>,
        operand: Box<Expr>,
    },
    Cast {
        semantics: CastSemantics,
        rep: MRep,
        inner: Box<Expr>,
    },
    BinOp {
        op: BinOpCode,
        out_rep: Option<MRep>,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}
