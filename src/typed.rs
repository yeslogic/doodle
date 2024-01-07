use crate::byte_set::ByteSet;
use crate::{DynFormat, Expr, Label, Pattern, TypeScope, ValueKind, VarType};

#[derive(Clone, Debug)]
pub struct TExpr {
    pub t: VarType,
    e: TExpr0,
}

#[derive(Clone, Debug)]
pub enum TExpr0 {
    Var(Label),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<TExpr>),
    TupleProj(Box<TExpr>, usize),
    Record(Vec<(Label, TExpr)>),
    RecordProj(Box<TExpr>, Label),
    Variant(Label, Box<TExpr>),
    Seq(Vec<TExpr>),
    Match(Box<TExpr>, Vec<(Pattern, TExpr)>),
    Lambda(Label, Box<TExpr>),

    BitAnd(Box<TExpr>, Box<TExpr>),
    BitOr(Box<TExpr>, Box<TExpr>),
    Eq(Box<TExpr>, Box<TExpr>),
    Ne(Box<TExpr>, Box<TExpr>),
    Lt(Box<TExpr>, Box<TExpr>),
    Gt(Box<TExpr>, Box<TExpr>),
    Lte(Box<TExpr>, Box<TExpr>),
    Gte(Box<TExpr>, Box<TExpr>),
    Mul(Box<TExpr>, Box<TExpr>),
    Div(Box<TExpr>, Box<TExpr>),
    Rem(Box<TExpr>, Box<TExpr>),
    Shl(Box<TExpr>, Box<TExpr>),
    Shr(Box<TExpr>, Box<TExpr>),
    Add(Box<TExpr>, Box<TExpr>),
    Sub(Box<TExpr>, Box<TExpr>),

    AsU8(Box<TExpr>),
    AsU16(Box<TExpr>),
    AsU32(Box<TExpr>),

    U16Be(Box<TExpr>),
    U16Le(Box<TExpr>),
    U32Be(Box<TExpr>),
    U32Le(Box<TExpr>),
    AsChar(Box<TExpr>),

    SeqLength(Box<TExpr>),
    SubSeq(Box<TExpr>, Box<TExpr>, Box<TExpr>),
    FlatMap(Box<TExpr>, Box<TExpr>),
    FlatMapAccum(Box<TExpr>, Box<TExpr>, VarType, Box<TExpr>),
    Dup(Box<TExpr>, Box<TExpr>),
    Inflate(Box<TExpr>),
}

#[derive(Clone, Debug)]
pub enum TFormat {
    /// Reference to a top-level item
    ItemVar(usize, Vec<TExpr>),
    /// A format that never matches
    Fail,
    /// Matches if the end of the input has been reached
    EndOfInput,
    /// Skips bytes if necessary to align the current offset to a multiple of N
    Align(usize),
    /// Matches a byte in the given byte set
    Byte(ByteSet),
    /// Wraps the value from the inner format in a variant
    Variant(Label, Box<TFormat>),
    /// Matches the union of all the formats, which must have the same type
    Union(Vec<TFormat>),
    /// Temporary hack for nondeterministic variant unions
    UnionNondet(Vec<(Label, TFormat)>),
    /// Matches a sequence of concatenated formats
    Tuple(Vec<TFormat>),
    /// Matches a sequence of named formats where later formats can depend on
    /// the decoded value of earlier formats
    Record(Vec<(Label, TFormat)>),
    /// Repeat a format zero-or-more times
    Repeat(Box<TFormat>),
    /// Repeat a format one-or-more times
    Repeat1(Box<TFormat>),
    /// Repeat a format an exact number of times
    RepeatCount(TExpr, Box<TFormat>),
    /// Repeat a format until a condition is satisfied by its last item
    RepeatUntilLast(TExpr, Box<TFormat>),
    /// Repeat a format until a condition is satisfied by the sequence
    RepeatUntilSeq(TExpr, Box<TFormat>),
    /// Parse a format without advancing the stream position afterwards
    Peek(Box<TFormat>),
    /// Attempt to parse a format and fail if it succeeds
    PeekNot(Box<TFormat>),
    /// Restrict a format to a sub-stream of a given number of bytes (skips any leftover bytes in the sub-stream)
    Slice(TExpr, Box<TFormat>),
    /// Parse bitstream
    Bits(Box<TFormat>),
    /// Matches a format at a byte offset relative to the current stream position
    WithRelativeOffset(TExpr, Box<TFormat>),
    /// Map a value with a lambda expression
    Map(Box<TFormat>, TExpr),
    /// Compute a value
    Compute(TExpr),
    /// Let binding
    Let(Label, TExpr, Box<TFormat>),
    /// Pattern match on an expression
    Match(TExpr, Vec<(Pattern, TFormat)>),
    /// TFormat generated dynamically
    Dynamic(Label, DynFormat, Box<TFormat>),
    /// Apply a dynamic format from a named variable in the scope
    Apply(Label),
}

impl TExpr {
    fn new(t: VarType, e: TExpr0) -> TExpr {
        TExpr { t, e }
    }

    pub fn infer_type(scope: &TypeScope<'_>, expr: &Expr) -> Result<TExpr, String> {
        match expr {
            Expr::Var(name) => match scope.get_type_by_name(name) {
                ValueKind::Value(t) => Ok(TExpr::new(t.clone(), TExpr0::Var(name.clone()))),
                ValueKind::Format(_t) => Err("expected value type".to_string()),
            },
            Expr::Bool(b) => Ok(TExpr::new(VarType::Bool, TExpr0::Bool(*b))),
            Expr::U8(i) => Ok(TExpr::new(VarType::U8, TExpr0::U8(*i))),
            Expr::U16(i) => Ok(TExpr::new(VarType::U16, TExpr0::U16(*i))),
            Expr::U32(i) => Ok(TExpr::new(VarType::U32, TExpr0::U32(*i))),
            Expr::Tuple(exprs) => {
                let mut es = Vec::with_capacity(exprs.len());
                let mut ts = Vec::with_capacity(exprs.len());
                for expr in exprs {
                    let e = TExpr::infer_type(scope, expr)?;
                    let t = e.t.clone();
                    es.push(e);
                    ts.push(t);
                }
                Ok(TExpr::new(VarType::Tuple(ts), TExpr0::Tuple(es)))
            }
            Expr::TupleProj(head, index) => {
                let head = TExpr::infer_type(scope, head)?;
                match head.t.expand_var() {
                    VarType::Tuple(vs) => Ok(TExpr::new(
                        vs[*index].clone(),
                        TExpr0::TupleProj(Box::new(head), *index),
                    )),
                    _ => Err("expected tuple type".to_string()),
                }
            }
            Expr::Record(fields) => {
                let mut ts = Vec::with_capacity(fields.len());
                let mut fs = Vec::with_capacity(fields.len());
                for (label, expr) in fields {
                    let expr = TExpr::infer_type(scope, expr)?;
                    ts.push((label.clone(), expr.t.clone()));
                    fs.push((label.clone(), expr));
                }
                Ok(TExpr::new(VarType::Record(ts), TExpr0::Record(fs)))
            }
            Expr::RecordProj(head, label) => {
                let head = TExpr::infer_type(scope, head)?;
                let t = head.t.record_proj(label);
                Ok(TExpr::new(
                    t,
                    TExpr0::RecordProj(Box::new(head), label.clone()),
                ))
            }
            Expr::Variant(label, expr) => {
                let expr = TExpr::infer_type(scope, expr)?;
                Ok(TExpr::new(
                    VarType::union(vec![(label.clone(), expr.t.clone())]),
                    TExpr0::Variant(label.clone(), Box::new(expr)),
                ))
            }
            Expr::Seq(exprs) => {
                let mut t = VarType::var();
                let mut es = Vec::with_capacity(exprs.len());
                for e in exprs {
                    let e = TExpr::infer_type(scope, e)?;
                    t = t.unify(&e.t)?;
                    es.push(e);
                }
                Ok(TExpr::new(VarType::Seq(Box::new(t)), TExpr0::Seq(es)))
            }
            Expr::Match(head, branches) => {
                if branches.is_empty() {
                    return Err("infer_type: empty Match".to_string());
                }
                let head = TExpr::infer_type(scope, head)?;
                let mut bs = Vec::with_capacity(branches.len());
                let mut t = VarType::var();
                for (pattern, branch) in branches {
                    let branch =
                        TExpr::infer_expr_pattern_branch_type(scope, &head.t, pattern, branch)?;
                    t = t.unify(&branch.t)?;
                    bs.push((pattern.clone(), branch));
                }
                Ok(TExpr::new(t, TExpr0::Match(Box::new(head), bs)))
            }
            Expr::Lambda(_, _) => Err("cannot infer_type lambda".to_string()),

            Expr::BitAnd(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::U8,
                        TExpr0::BitAnd(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        TExpr0::BitAnd(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        TExpr0::BitAnd(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::BitOr(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::U8,
                        TExpr0::BitOr(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        TExpr0::BitOr(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        TExpr0::BitOr(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Eq(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Eq(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Eq(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Eq(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Ne(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Ne(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Ne(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Ne(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Lt(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Lt(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Lt(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Lt(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Gt(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Gt(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Gt(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Gt(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Lte(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Lte(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Lte(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Lte(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Gte(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Gte(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Gte(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        TExpr0::Gte(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Add(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::U8,
                        TExpr0::Add(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        TExpr0::Add(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        TExpr0::Add(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Sub(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::U8,
                        TExpr0::Sub(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        TExpr0::Sub(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        TExpr0::Sub(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Mul(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::U8,
                        TExpr0::Mul(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        TExpr0::Mul(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        TExpr0::Mul(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Div(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::U8,
                        TExpr0::Div(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        TExpr0::Div(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        TExpr0::Div(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Rem(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::U8,
                        TExpr0::Rem(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        TExpr0::Rem(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        TExpr0::Rem(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Shl(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::U8,
                        TExpr0::Shl(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        TExpr0::Shl(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        TExpr0::Shl(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Shr(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::U8,
                        TExpr0::Shr(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        TExpr0::Shr(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        TExpr0::Shr(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }

            Expr::AsU8(x) => {
                let x = TExpr::infer_type(scope, x)?;
                match x.t.expand_var() {
                    VarType::U8 => Ok(TExpr::new(VarType::U8, TExpr0::AsU8(Box::new(x)))),
                    VarType::U16 => Ok(TExpr::new(VarType::U8, TExpr0::AsU8(Box::new(x)))),
                    VarType::U32 => Ok(TExpr::new(VarType::U8, TExpr0::AsU8(Box::new(x)))),
                    x => Err(format!("cannot convert {x:?} to U8")),
                }
            }
            Expr::AsU16(x) => {
                let x = TExpr::infer_type(scope, x)?;
                match x.t.expand_var() {
                    VarType::U8 => Ok(TExpr::new(VarType::U16, TExpr0::AsU16(Box::new(x)))),
                    VarType::U16 => Ok(TExpr::new(VarType::U16, TExpr0::AsU16(Box::new(x)))),
                    VarType::U32 => Ok(TExpr::new(VarType::U16, TExpr0::AsU16(Box::new(x)))),
                    x => Err(format!("cannot convert {x:?} to U16")),
                }
            }
            Expr::AsU32(x) => {
                let x = TExpr::infer_type(scope, x)?;
                match x.t.expand_var() {
                    VarType::U8 => Ok(TExpr::new(VarType::U32, TExpr0::AsU32(Box::new(x)))),
                    VarType::U16 => Ok(TExpr::new(VarType::U32, TExpr0::AsU32(Box::new(x)))),
                    VarType::U32 => Ok(TExpr::new(VarType::U32, TExpr0::AsU32(Box::new(x)))),
                    x => Err(format!("cannot convert {x:?} to U32")),
                }
            }
            Expr::AsChar(x) => {
                let x = TExpr::infer_type(scope, x)?;
                match x.t.expand_var() {
                    VarType::U8 => Ok(TExpr::new(VarType::Char, TExpr0::AsChar(Box::new(x)))),
                    VarType::U16 => Ok(TExpr::new(VarType::Char, TExpr0::AsChar(Box::new(x)))),
                    VarType::U32 => Ok(TExpr::new(VarType::Char, TExpr0::AsChar(Box::new(x)))),
                    x => Err(format!("cannot convert {x:?} to Char")),
                }
            }

            Expr::U16Be(bytes) => {
                let bytes = TExpr::infer_type(scope, bytes)?;
                match bytes.t.clone().unwrap_tuple_type().as_slice() {
                    [VarType::U8, VarType::U8] => {
                        Ok(TExpr::new(VarType::U16, TExpr0::U16Le(Box::new(bytes))))
                    }
                    other => Err(format!("U16Be: expected (U8, U8), found {other:#?}")),
                }
            }
            Expr::U16Le(bytes) => {
                let bytes = TExpr::infer_type(scope, bytes)?;
                match bytes.t.clone().unwrap_tuple_type().as_slice() {
                    [VarType::U8, VarType::U8] => {
                        Ok(TExpr::new(VarType::U16, TExpr0::U16Le(Box::new(bytes))))
                    }
                    other => Err(format!("U16Le: expected (U8, U8), found {other:#?}")),
                }
            }
            Expr::U32Be(bytes) => {
                let bytes = TExpr::infer_type(scope, bytes)?;
                match bytes.t.clone().unwrap_tuple_type().as_slice() {
                    [VarType::U8, VarType::U8, VarType::U8, VarType::U8] => {
                        Ok(TExpr::new(VarType::U32, TExpr0::U32Be(Box::new(bytes))))
                    }
                    other => Err(format!(
                        "U32Be: expected (U8, U8, U8, U8), found {other:#?}"
                    )),
                }
            }
            Expr::U32Le(bytes) => {
                let bytes = TExpr::infer_type(scope, bytes)?;
                match bytes.t.clone().unwrap_tuple_type().as_slice() {
                    [VarType::U8, VarType::U8, VarType::U8, VarType::U8] => {
                        Ok(TExpr::new(VarType::U32, TExpr0::U32Le(Box::new(bytes))))
                    }
                    other => Err(format!(
                        "U32Le: expected (U8, U8, U8, U8), found {other:#?}"
                    )),
                }
            }
            Expr::SeqLength(seq) => {
                let seq = TExpr::infer_type(scope, seq)?;
                match seq.t.expand_var() {
                    VarType::Seq(_t) => {
                        Ok(TExpr::new(VarType::U32, TExpr0::SeqLength(Box::new(seq))))
                    }
                    other => Err(format!("SeqLength: expected Seq, found {other:?}")),
                }
            }
            Expr::SubSeq(seq, start, length) => {
                let seq = TExpr::infer_type(scope, seq)?;
                let start = TExpr::infer_type(scope, start)?;
                let length = TExpr::infer_type(scope, length)?;
                match seq.t.expand_var() {
                    VarType::Seq(t) => {
                        if !start.t.is_numeric_type() {
                            return Err(format!(
                                "SubSeq start must be numeric, found {:?}",
                                start.t
                            ));
                        }
                        if !length.t.is_numeric_type() {
                            return Err(format!(
                                "SubSeq length must be numeric, found {:?}",
                                length.t
                            ));
                        }
                        Ok(TExpr::new(
                            VarType::Seq(t.clone()),
                            TExpr0::SubSeq(Box::new(seq), Box::new(start), Box::new(length)),
                        ))
                    }
                    other => Err(format!("SubSeq: expected Seq, found {other:?}")),
                }
            }
            Expr::FlatMap(expr, seq) => {
                let seq = TExpr::infer_type(scope, seq)?;
                match seq.t.expand_var() {
                    VarType::Seq(t) => {
                        match expr.as_ref() {
                            Expr::Lambda(name, body) => {
                                let mut child_scope = TypeScope::child(scope);
                                child_scope.push(name.clone(), (**t).clone());
                                let body = TExpr::infer_type(&child_scope, body)?;
                                match body.t.expand_var().clone() {
                                    VarType::Seq(t2) => {
                                        // FIXME no lambda types yet
                                        let expr = TExpr::new(
                                            body.t.clone(),
                                            TExpr0::Lambda(name.clone(), Box::new(body)),
                                        );
                                        Ok(TExpr::new(
                                            VarType::Seq(t2),
                                            TExpr0::FlatMap(Box::new(expr), Box::new(seq)),
                                        ))
                                    }
                                    other => Err(format!("FlatMap: expected Seq, found {other:?}")),
                                }
                            }
                            other => Err(format!("FlatMap: expected Lambda, found {other:?}")),
                        }
                    }
                    other => Err(format!("FlatMap: expected Seq, found {other:?}")),
                }
            }
            Expr::FlatMapAccum(expr, accum, accum_type, seq) => {
                let accum = TExpr::infer_type(scope, accum)?;
                let seq = TExpr::infer_type(scope, seq)?;
                match seq.t.expand_var() {
                    VarType::Seq(t) => {
                        match expr.as_ref() {
                            Expr::Lambda(name, body) => {
                                let accum_type = accum.t.unify(&accum_type.to_var_type())?;
                                let mut child_scope = TypeScope::child(scope);
                                child_scope.push(
                                    name.clone(),
                                    VarType::Tuple(vec![accum_type.clone(), *t.clone()]),
                                );
                                let body = TExpr::infer_type(&child_scope, body)?;
                                match body.t.clone().unwrap_tuple_type().as_mut_slice() {
                                    [accum_result, VarType::Seq(t2)] => {
                                        accum_result.unify(&accum_type)?;
                                        // FIXME no lambda types yet
                                        let expr = TExpr::new(
                                            body.t.clone(),
                                            TExpr0::Lambda(name.clone(), Box::new(body)),
                                        );
                                        Ok(TExpr::new(
                                            VarType::Seq(t2.clone()),
                                            TExpr0::FlatMapAccum(
                                                Box::new(expr),
                                                Box::new(accum),
                                                accum_type,
                                                Box::new(seq),
                                            ),
                                        ))
                                    }
                                    _ => panic!("FlatMapAccum: expected two values"),
                                }
                            }
                            other => Err(format!("FlatMapAccum: expected Lambda, found {other:?}")),
                        }
                    }
                    other => Err(format!("FlatMapAccum: expected Seq, found {other:?}")),
                }
            }
            Expr::Dup(count, expr) => {
                let count = TExpr::infer_type(scope, count)?;
                let expr = TExpr::infer_type(scope, expr)?;
                if !count.t.is_numeric_type() {
                    return Err(format!("Dup: count is not numeric: {count:?}"));
                }
                Ok(TExpr::new(
                    VarType::Seq(Box::new(expr.t.clone())),
                    TExpr0::Dup(Box::new(count), Box::new(expr)),
                ))
            }
            Expr::Inflate(seq) => {
                let seq = TExpr::infer_type(scope, seq)?;
                match seq.t.expand_var() {
                    // FIXME should check values are appropriate variants
                    VarType::Seq(_values) => Ok(TExpr::new(
                        VarType::Seq(Box::new(VarType::U8)),
                        TExpr0::Inflate(Box::new(seq)),
                    )),
                    other => Err(format!("Inflate: expected Seq, found {other:?}")),
                }
            }
        }
    }

    fn infer_expr_pattern_branch_type(
        scope: &TypeScope<'_>,
        head_type: &VarType,
        pattern: &Pattern,
        expr: &Expr,
    ) -> Result<TExpr, String> {
        let mut pattern_scope = TypeScope::child(scope);
        pattern.build_scope(&mut pattern_scope, head_type);
        TExpr::infer_type(&pattern_scope, expr)
    }
}
