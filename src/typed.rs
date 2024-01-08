use crate::byte_set::ByteSet;
use crate::{DynFormat, Expr, Expr0, Label, Pattern, TypeScope, ValueKind, VarType};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct TExpr {
    pub t: VarType,
    e: Expr<TExpr>,
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
    fn new(t: VarType, e: Expr<TExpr>) -> TExpr {
        TExpr { t, e }
    }

    pub fn infer_type(scope: &TypeScope<'_>, expr: &Expr0) -> Result<TExpr, String> {
        match expr {
            Expr::Var(name) => match scope.get_type_by_name(name) {
                ValueKind::Value(t) => Ok(TExpr::new(t.clone(), Expr::Var(name.clone()))),
                ValueKind::Format(_t) => Err("expected value type".to_string()),
            },
            Expr::Bool(b) => Ok(TExpr::new(VarType::Bool, Expr::Bool(*b))),
            Expr::U8(i) => Ok(TExpr::new(VarType::U8, Expr::U8(*i))),
            Expr::U16(i) => Ok(TExpr::new(VarType::U16, Expr::U16(*i))),
            Expr::U32(i) => Ok(TExpr::new(VarType::U32, Expr::U32(*i))),
            Expr::Tuple(exprs) => {
                let mut es = Vec::with_capacity(exprs.len());
                let mut ts = Vec::with_capacity(exprs.len());
                for expr in exprs {
                    let e = TExpr::infer_type(scope, expr)?;
                    let t = e.t.clone();
                    es.push(e);
                    ts.push(t);
                }
                Ok(TExpr::new(VarType::Tuple(ts), Expr::Tuple(es)))
            }
            Expr::TupleProj(head, index) => {
                let head = TExpr::infer_type(scope, head)?;
                match head.t.expand_var() {
                    VarType::Tuple(vs) => Ok(TExpr::new(
                        vs[*index].clone(),
                        Expr::TupleProj(Box::new(head), *index),
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
                Ok(TExpr::new(VarType::Record(ts), Expr::Record(fs)))
            }
            Expr::RecordProj(head, label) => {
                let head = TExpr::infer_type(scope, head)?;
                let t = head.t.record_proj(label);
                Ok(TExpr::new(
                    t,
                    Expr::RecordProj(Box::new(head), label.clone()),
                ))
            }
            Expr::Variant(label, expr) => {
                let expr = TExpr::infer_type(scope, expr)?;
                Ok(TExpr::new(
                    VarType::union(vec![(label.clone(), expr.t.clone())]),
                    Expr::Variant(label.clone(), Box::new(expr)),
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
                Ok(TExpr::new(VarType::Seq(Box::new(t)), Expr::Seq(es)))
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
                Ok(TExpr::new(t, Expr::Match(Box::new(head), bs)))
            }
            Expr::Lambda(_, _) => Err("cannot infer_type lambda".to_string()),

            Expr::BitAnd(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => Ok(TExpr::new(
                        VarType::U8,
                        Expr::BitAnd(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        Expr::BitAnd(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        Expr::BitAnd(Box::new(x), Box::new(y)),
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
                        Expr::BitOr(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        Expr::BitOr(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        Expr::BitOr(Box::new(x), Box::new(y)),
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
                        Expr::Eq(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Eq(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Eq(Box::new(x), Box::new(y)),
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
                        Expr::Ne(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Ne(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Ne(Box::new(x), Box::new(y)),
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
                        Expr::Lt(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Lt(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Lt(Box::new(x), Box::new(y)),
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
                        Expr::Gt(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Gt(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Gt(Box::new(x), Box::new(y)),
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
                        Expr::Lte(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Lte(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Lte(Box::new(x), Box::new(y)),
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
                        Expr::Gte(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Gte(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::Bool,
                        Expr::Gte(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Add(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => {
                        Ok(TExpr::new(VarType::U8, Expr::Add(Box::new(x), Box::new(y))))
                    }
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        Expr::Add(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        Expr::Add(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Sub(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => {
                        Ok(TExpr::new(VarType::U8, Expr::Sub(Box::new(x), Box::new(y))))
                    }
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        Expr::Sub(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        Expr::Sub(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Mul(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => {
                        Ok(TExpr::new(VarType::U8, Expr::Mul(Box::new(x), Box::new(y))))
                    }
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        Expr::Mul(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        Expr::Mul(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Div(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => {
                        Ok(TExpr::new(VarType::U8, Expr::Div(Box::new(x), Box::new(y))))
                    }
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        Expr::Div(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        Expr::Div(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Rem(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => {
                        Ok(TExpr::new(VarType::U8, Expr::Rem(Box::new(x), Box::new(y))))
                    }
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        Expr::Rem(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        Expr::Rem(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Shl(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => {
                        Ok(TExpr::new(VarType::U8, Expr::Shl(Box::new(x), Box::new(y))))
                    }
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        Expr::Shl(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        Expr::Shl(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }
            Expr::Shr(x, y) => {
                let x = TExpr::infer_type(scope, x)?;
                let y = TExpr::infer_type(scope, y)?;
                match (x.t.expand_var(), y.t.expand_var()) {
                    (VarType::U8, VarType::U8) => {
                        Ok(TExpr::new(VarType::U8, Expr::Shr(Box::new(x), Box::new(y))))
                    }
                    (VarType::U16, VarType::U16) => Ok(TExpr::new(
                        VarType::U16,
                        Expr::Shr(Box::new(x), Box::new(y)),
                    )),
                    (VarType::U32, VarType::U32) => Ok(TExpr::new(
                        VarType::U32,
                        Expr::Shr(Box::new(x), Box::new(y)),
                    )),
                    (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
                }
            }

            Expr::AsU8(x) => {
                let x = TExpr::infer_type(scope, x)?;
                match x.t.expand_var() {
                    VarType::U8 => Ok(TExpr::new(VarType::U8, Expr::AsU8(Box::new(x)))),
                    VarType::U16 => Ok(TExpr::new(VarType::U8, Expr::AsU8(Box::new(x)))),
                    VarType::U32 => Ok(TExpr::new(VarType::U8, Expr::AsU8(Box::new(x)))),
                    x => Err(format!("cannot convert {x:?} to U8")),
                }
            }
            Expr::AsU16(x) => {
                let x = TExpr::infer_type(scope, x)?;
                match x.t.expand_var() {
                    VarType::U8 => Ok(TExpr::new(VarType::U16, Expr::AsU16(Box::new(x)))),
                    VarType::U16 => Ok(TExpr::new(VarType::U16, Expr::AsU16(Box::new(x)))),
                    VarType::U32 => Ok(TExpr::new(VarType::U16, Expr::AsU16(Box::new(x)))),
                    x => Err(format!("cannot convert {x:?} to U16")),
                }
            }
            Expr::AsU32(x) => {
                let x = TExpr::infer_type(scope, x)?;
                match x.t.expand_var() {
                    VarType::U8 => Ok(TExpr::new(VarType::U32, Expr::AsU32(Box::new(x)))),
                    VarType::U16 => Ok(TExpr::new(VarType::U32, Expr::AsU32(Box::new(x)))),
                    VarType::U32 => Ok(TExpr::new(VarType::U32, Expr::AsU32(Box::new(x)))),
                    x => Err(format!("cannot convert {x:?} to U32")),
                }
            }
            Expr::AsChar(x) => {
                let x = TExpr::infer_type(scope, x)?;
                match x.t.expand_var() {
                    VarType::U8 => Ok(TExpr::new(VarType::Char, Expr::AsChar(Box::new(x)))),
                    VarType::U16 => Ok(TExpr::new(VarType::Char, Expr::AsChar(Box::new(x)))),
                    VarType::U32 => Ok(TExpr::new(VarType::Char, Expr::AsChar(Box::new(x)))),
                    x => Err(format!("cannot convert {x:?} to Char")),
                }
            }

            Expr::U16Be(bytes) => {
                let bytes = TExpr::infer_type(scope, bytes)?;
                match bytes.t.clone().unwrap_tuple_type().as_slice() {
                    [VarType::U8, VarType::U8] => {
                        Ok(TExpr::new(VarType::U16, Expr::U16Le(Box::new(bytes))))
                    }
                    other => Err(format!("U16Be: expected (U8, U8), found {other:#?}")),
                }
            }
            Expr::U16Le(bytes) => {
                let bytes = TExpr::infer_type(scope, bytes)?;
                match bytes.t.clone().unwrap_tuple_type().as_slice() {
                    [VarType::U8, VarType::U8] => {
                        Ok(TExpr::new(VarType::U16, Expr::U16Le(Box::new(bytes))))
                    }
                    other => Err(format!("U16Le: expected (U8, U8), found {other:#?}")),
                }
            }
            Expr::U32Be(bytes) => {
                let bytes = TExpr::infer_type(scope, bytes)?;
                match bytes.t.clone().unwrap_tuple_type().as_slice() {
                    [VarType::U8, VarType::U8, VarType::U8, VarType::U8] => {
                        Ok(TExpr::new(VarType::U32, Expr::U32Be(Box::new(bytes))))
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
                        Ok(TExpr::new(VarType::U32, Expr::U32Le(Box::new(bytes))))
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
                        Ok(TExpr::new(VarType::U32, Expr::SeqLength(Box::new(seq))))
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
                            Expr::SubSeq(Box::new(seq), Box::new(start), Box::new(length)),
                        ))
                    }
                    other => Err(format!("SubSeq: expected Seq, found {other:?}")),
                }
            }
            Expr::FlatMap(expr, seq) => {
                let seq = TExpr::infer_type(scope, seq)?;
                match seq.t.expand_var() {
                    VarType::Seq(t) => {
                        match expr.as_ref().deref() {
                            Expr0::Lambda(name, body) => {
                                let mut child_scope = TypeScope::child(scope);
                                child_scope.push(name.clone(), (**t).clone());
                                let body = TExpr::infer_type(&child_scope, body)?;
                                match body.t.expand_var().clone() {
                                    VarType::Seq(t2) => {
                                        // FIXME no lambda types yet
                                        let expr = TExpr::new(
                                            body.t.clone(),
                                            Expr::Lambda(name.clone(), Box::new(body)),
                                        );
                                        Ok(TExpr::new(
                                            VarType::Seq(t2),
                                            Expr::FlatMap(Box::new(expr), Box::new(seq)),
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
                        match expr.as_ref().deref() {
                            Expr0::Lambda(name, body) => {
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
                                            Expr::Lambda(name.clone(), Box::new(body)),
                                        );
                                        Ok(TExpr::new(
                                            VarType::Seq(t2.clone()),
                                            Expr::FlatMapAccum(
                                                Box::new(expr),
                                                Box::new(accum),
                                                accum_type.to_value_type(),
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
                    Expr::Dup(Box::new(count), Box::new(expr)),
                ))
            }
            Expr::Inflate(seq) => {
                let seq = TExpr::infer_type(scope, seq)?;
                match seq.t.expand_var() {
                    // FIXME should check values are appropriate variants
                    VarType::Seq(_values) => Ok(TExpr::new(
                        VarType::Seq(Box::new(VarType::U8)),
                        Expr::Inflate(Box::new(seq)),
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
        expr: &Expr0,
    ) -> Result<TExpr, String> {
        let mut pattern_scope = TypeScope::child(scope);
        pattern.build_scope(&mut pattern_scope, head_type);
        TExpr::infer_type(&pattern_scope, expr)
    }
}
