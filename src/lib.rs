#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]

use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::rc::Rc;

use serde::Serialize;

use crate::bounds::Bounds;
use crate::byte_set::ByteSet;

pub mod bounds;
pub mod byte_set;
pub mod error;
pub mod output;

use error::{ParseError, ParseResult};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Pattern {
    Binding(String),
    Wildcard,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<Pattern>),
    Variant(String, Box<Pattern>),
    Seq(Vec<Pattern>),
}

impl Pattern {
    pub const UNIT: Pattern = Pattern::Tuple(Vec::new());

    pub fn from_bytes(bs: &[u8]) -> Pattern {
        Pattern::Seq(bs.iter().copied().map(Pattern::U8).collect())
    }

    pub fn variant(label: impl Into<String>, value: impl Into<Box<Pattern>>) -> Pattern {
        Pattern::Variant(label.into(), value.into())
    }

    fn build_scope(&self, scope: &mut TypeScope, t: &ValueType) {
        match (self, t) {
            (Pattern::Binding(name), t) => {
                scope.push(name.clone(), t.clone());
            }
            (Pattern::Wildcard, _) => {}
            (Pattern::Bool(_b0), ValueType::Bool) => {}
            (Pattern::U8(_i0), ValueType::U8) => {}
            (Pattern::U16(_i0), ValueType::U16) => {}
            (Pattern::U32(_i0), ValueType::U32) => {}
            (Pattern::Tuple(ps), ValueType::Tuple(ts)) if ps.len() == ts.len() => {
                for (p, t) in Iterator::zip(ps.iter(), ts.iter()) {
                    p.build_scope(scope, t);
                }
            }
            (Pattern::Seq(ps), ValueType::Seq(t)) => {
                for p in ps {
                    p.build_scope(scope, t);
                }
            }
            (Pattern::Variant(label, p), ValueType::Union(branches)) => {
                if let Some((_l, t)) = branches.iter().find(|(l, _t)| label == l) {
                    p.build_scope(scope, t);
                } else {
                    panic!("no {label} in {branches:?}");
                }
            }
            _ => panic!("pattern build_scope failed"),
        }
    }

    fn infer_expr_branch_type(
        &self,
        scope: &mut TypeScope,
        head_type: &ValueType,
        expr: &Expr,
    ) -> Result<ValueType, String> {
        let initial_len = scope.len();
        self.build_scope(scope, head_type);
        let t = expr.infer_type(scope)?;
        scope.truncate(initial_len);
        Ok(t)
    }

    fn infer_format_branch_type(
        &self,
        scope: &mut TypeScope,
        head_type: &ValueType,
        module: &FormatModule,
        format: &Format,
    ) -> Result<ValueType, String> {
        let initial_len = scope.len();
        self.build_scope(scope, head_type);
        let t = module.infer_format_type(scope, format)?;
        scope.truncate(initial_len);
        Ok(t)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum ValueType {
    Any,
    Empty,
    Bool,
    U8,
    U16,
    U32,
    Tuple(Vec<ValueType>),
    Record(Vec<(String, ValueType)>),
    Union(Vec<(String, ValueType)>),
    Seq(Box<ValueType>),
    Format(Box<ValueType>),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<Value>),
    Record(Vec<(String, Value)>),
    Variant(String, Box<Value>),
    Seq(Vec<Value>),
    Format(Box<Format>),
}

impl Value {
    pub const UNIT: Value = Value::Tuple(Vec::new());

    fn record<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Value)>) -> Value {
        Value::Record(
            fields
                .into_iter()
                .map(|(label, value)| (label.into(), value))
                .collect(),
        )
    }

    fn variant(label: impl Into<String>, value: impl Into<Box<Value>>) -> Value {
        Value::Variant(label.into(), value.into())
    }

    fn record_proj(&self, label: &str) -> Value {
        match self {
            Value::Record(fields) => match fields.iter().find(|(l, _)| label == l) {
                Some((_, v)) => v.clone(),
                None => panic!("{label} not found in record"),
            },
            _ => panic!("expected record, found {self:?}"),
        }
    }

    /// Returns `true` if the pattern successfully matches the value, pushing
    /// any values bound by the pattern onto the scope
    fn matches(&self, scope: &mut Scope, pattern: &Pattern) -> bool {
        match (pattern, self.coerce_record_to_value()) {
            (Pattern::Binding(name), head) => {
                scope.push(name.clone(), head.clone());
                true
            }
            (Pattern::Wildcard, _) => true,
            (Pattern::Bool(b0), Value::Bool(b1)) => b0 == b1,
            (Pattern::U8(i0), Value::U8(i1)) => i0 == i1,
            (Pattern::U16(i0), Value::U16(i1)) => i0 == i1,
            (Pattern::U32(i0), Value::U32(i1)) => i0 == i1,
            (Pattern::Tuple(ps), Value::Tuple(vs)) | (Pattern::Seq(ps), Value::Seq(vs))
                if ps.len() == vs.len() =>
            {
                let initial_len = scope.len();
                for (p, v) in Iterator::zip(ps.iter(), vs.iter()) {
                    if !v.matches(scope, p) {
                        scope.truncate(initial_len);
                        return false;
                    }
                }
                true
            }
            (Pattern::Variant(label0, p), Value::Variant(label1, v)) if label0 == label1 => {
                v.matches(scope, p)
            }
            _ => false,
        }
    }

    fn coerce_record_to_value(&self) -> &Value {
        match self {
            Value::Record(fields) => {
                if let Some((_l, v)) = fields.iter().find(|(l, _)| l == "@value") {
                    &v
                } else {
                    &self
                }
            }
            v => v,
        }
    }

    fn unwrap_usize(self) -> usize {
        match self {
            Value::U8(n) => usize::from(n),
            Value::U16(n) => usize::from(n),
            Value::U32(n) => usize::try_from(n).unwrap(),
            _ => panic!("value is not a number"),
        }
    }

    fn unwrap_tuple(self) -> Vec<Value> {
        match self {
            Value::Tuple(values) => values,
            _ => panic!("value is not a tuple"),
        }
    }

    fn unwrap_bool(self) -> bool {
        match self {
            Value::Bool(b) => b,
            _ => panic!("value is not a bool"),
        }
    }
}

impl ValueType {
    fn record_proj(&self, label: &str) -> ValueType {
        match self {
            ValueType::Record(fields) => match fields.iter().find(|(l, _)| label == l) {
                Some((_, t)) => t.clone(),
                None => panic!("{label} not found in record type"),
            },
            _ => panic!("expected record type"),
        }
    }

    fn unwrap_tuple_type(self) -> Vec<ValueType> {
        match self {
            ValueType::Tuple(ts) => ts,
            _ => panic!("type is not a tuple"),
        }
    }

    fn is_numeric_type(&self) -> bool {
        match self {
            ValueType::U8 | ValueType::U16 | ValueType::U32 => true,
            _ => false,
        }
    }

    fn unify(&self, other: &ValueType) -> Result<ValueType, String> {
        match (self, other) {
            (ValueType::Any, rhs) => Ok(rhs.clone()),
            (lhs, ValueType::Any) => Ok(lhs.clone()),
            (ValueType::Empty, ValueType::Empty) => Ok(ValueType::Empty),
            (ValueType::Bool, ValueType::Bool) => Ok(ValueType::Bool),
            (ValueType::U8, ValueType::U8) => Ok(ValueType::U8),
            (ValueType::U16, ValueType::U16) => Ok(ValueType::U16),
            (ValueType::U32, ValueType::U32) => Ok(ValueType::U32),
            (ValueType::Tuple(ts1), ValueType::Tuple(ts2)) => {
                if ts1.len() != ts2.len() {
                    return Err(format!("tuples must have same length {ts1:?} vs. {ts2:?}"));
                }
                let mut ts = Vec::new();
                for (t1, t2) in Iterator::zip(ts1.iter(), ts2.iter()) {
                    ts.push(t1.unify(t2)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            (ValueType::Record(fs1), ValueType::Record(fs2)) => {
                if fs1.len() != fs2.len() {
                    return Err(format!(
                        "records must have same number of fields {fs1:?} vs. {fs2:?}"
                    ));
                }
                // FIXME field order
                let mut fs = Vec::new();
                for ((l1, t1), (l2, t2)) in Iterator::zip(fs1.iter(), fs2.iter()) {
                    if l1 != l2 {
                        return Err(format!("record fields do not match"));
                    }
                    fs.push((l1.clone(), t1.unify(t2)?));
                }
                Ok(ValueType::Record(fs))
            }
            (ValueType::Union(bs1), ValueType::Union(bs2)) => {
                let mut bs: Vec<(String, ValueType)> = Vec::new();
                for (label, t2) in bs2 {
                    let t = if let Some((_l, t1)) = bs.iter().find(|(l, _)| label == l) {
                        t1.unify(t2)?
                    } else {
                        t2.clone()
                    };
                    bs.push((label.clone(), t));
                }
                for (label, t1) in bs1 {
                    if bs.iter().find(|(l, _)| label == l).is_none() {
                        bs.push((label.clone(), t1.clone()));
                    }
                }
                Ok(ValueType::Union(bs))
            }
            (ValueType::Seq(t1), ValueType::Seq(t2)) => Ok(ValueType::Seq(Box::new(t1.unify(t2)?))),
            (t1, t2) => Err(format!("failed to unify types {t1:?} and {t2:?}")),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Expr {
    Var(String),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<Expr>),
    TupleProj(Box<Expr>, usize),
    Record(Vec<(String, Expr)>),
    RecordProj(Box<Expr>, String),
    Variant(String, Box<Expr>),
    Seq(Vec<Expr>),
    Match(Box<Expr>, Vec<(Pattern, Expr)>),
    Lambda(String, Box<Expr>),

    BitAnd(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Lte(Box<Expr>, Box<Expr>),
    Gte(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    Shl(Box<Expr>, Box<Expr>),
    Shr(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),

    AsU8(Box<Expr>),
    AsU16(Box<Expr>),
    AsU32(Box<Expr>),

    U16Be(Box<Expr>),
    U16Le(Box<Expr>),
    U32Be(Box<Expr>),
    U32Le(Box<Expr>),

    SeqLength(Box<Expr>),
    SubSeq(Box<Expr>, Box<Expr>, Box<Expr>),
    FlatMap(Box<Expr>, Box<Expr>),
    FlatMapAccum(Box<Expr>, Box<Expr>, ValueType, Box<Expr>),
    Dup(Box<Expr>, Box<Expr>),
    Inflate(Box<Expr>),
}

impl Expr {
    pub const UNIT: Expr = Expr::Tuple(Vec::new());

    pub fn record_proj(head: impl Into<Box<Expr>>, label: impl Into<String>) -> Expr {
        Expr::RecordProj(head.into(), label.into())
    }
}

/// Binary format descriptions
///
/// # Binary formats as regular expressions
///
/// Given a language of [regular expressions]:
///
/// ```text
/// r ∈ Regexp ::=
///   | ∅           empty set
///   | ε           empty byte string
///   | .           any byte
///   | b           literal byte
///   | r|r         alternation
///   | r r         concatenation
///   | r*          Kleene star
/// ```
///
/// We can use these to model a subset of our binary format descriptions:
///
/// ```text
/// ⟦ _ ⟧ : Format ⇀ Regexp
/// ⟦ Fail ⟧                                = ∅
/// ⟦ Byte({}) ⟧                            = ∅
/// ⟦ Byte(!{}) ⟧                           = .
/// ⟦ Byte({b}) ⟧                           = b
/// ⟦ Byte({b₀, ... bₙ}) ⟧                  = b₀ | ... | bₙ
/// ⟦ Union([]) ⟧                           = ∅
/// ⟦ Union([(l₀, f₀), ..., (lₙ, fₙ)]) ⟧    = ⟦ f₀ ⟧ | ... | ⟦ fₙ ⟧
/// ⟦ Tuple([]) ⟧                           = ε
/// ⟦ Tuple([f₀, ..., fₙ]) ⟧                = ⟦ f₀ ⟧ ... ⟦ fₙ ⟧
/// ⟦ Repeat(f) ⟧                           = ⟦ f ⟧*
/// ⟦ Repeat1(f) ⟧                          = ⟦ f ⟧ ⟦ f ⟧*
/// ⟦ RepeatCount(n, f) ⟧                   = ⟦ f ⟧ ... ⟦ f ⟧
///                                           ╰── n times ──╯
/// ```
///
/// Note that the data dependency present in record formats means that these
/// formats no longer describe regular languages.
///
/// [regular expressions]: https://en.wikipedia.org/wiki/Regular_expression#Formal_definition
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Format {
    /// Reference to a top-level item
    ItemVar(usize, Vec<Expr>),
    /// A format that never matches
    Fail,
    /// Matches if the end of the input has been reached
    EndOfInput,
    /// Skips bytes if necessary to align the current offset to a multiple of N
    Align(usize),
    /// Matches a byte in the given byte set
    Byte(ByteSet),
    /// Matches the union of the byte strings matched by all the formats
    Union(Vec<(String, Format)>),
    /// Temporary hack for nondeterministic unions
    NondetUnion(Vec<(String, Format)>),
    /// Matches a sequence of concatenated formats
    Tuple(Vec<Format>),
    /// Matches a sequence of named formats where later formats can depend on
    /// the decoded value of earlier formats
    Record(Vec<(String, Format)>),
    /// Repeat a format zero-or-more times
    Repeat(Box<Format>),
    /// Repeat a format one-or-more times
    Repeat1(Box<Format>),
    /// Repeat a format an exact number of times
    RepeatCount(Expr, Box<Format>),
    /// Repeat a format until a condition is satisfied by its last item
    RepeatUntilLast(Expr, Box<Format>),
    /// Repeat a format until a condition is satisfied by the sequence
    RepeatUntilSeq(Expr, Box<Format>),
    /// Parse a format without advancing the stream position afterwards
    Peek(Box<Format>),
    /// Attempt to parse a format and fail if it succeeds
    PeekNot(Box<Format>),
    /// Restrict a format to a sub-stream of a given number of bytes (skips any leftover bytes in the sub-stream)
    Slice(Expr, Box<Format>),
    /// Parse bitstream
    Bits(Box<Format>),
    /// Matches a format at a byte offset relative to the current stream position
    WithRelativeOffset(Expr, Box<Format>),
    /// Compute a value
    Compute(Expr),
    /// Pattern match on an expression
    Match(Expr, Vec<(Pattern, Format)>),
    /// Pattern match on an expression and return a variant
    MatchVariant(Expr, Vec<(Pattern, String, Format)>),
    /// Format generated dynamically
    Dynamic(DynFormat),
    /// Apply a dynamic format from a named variable in the scope
    Apply(String),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum DynFormat {
    Huffman(Expr, Option<Expr>),
}

impl Format {
    pub const EMPTY: Format = Format::Tuple(Vec::new());

    pub fn alts<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
        Format::Union(
            (fields.into_iter())
                .map(|(label, format)| (label.into(), format))
                .collect(),
        )
    }

    pub fn record<Label: Into<String>>(
        fields: impl IntoIterator<Item = (Label, Format)>,
    ) -> Format {
        Format::Record(
            (fields.into_iter())
                .map(|(label, format)| (label.into(), format))
                .collect(),
        )
    }
}

#[derive(Copy, Clone)]
pub struct FormatRef(usize);

impl FormatRef {
    pub fn get_level(&self) -> usize {
        self.0
    }

    pub fn call(&self) -> Format {
        Format::ItemVar(self.0, vec![])
    }

    pub fn call_args(&self, args: Vec<Expr>) -> Format {
        Format::ItemVar(self.0, args)
    }
}

#[derive(Debug, Serialize)]
pub struct FormatModule {
    names: Vec<String>,
    args: Vec<Vec<(String, ValueType)>>,
    formats: Vec<Format>,
    format_types: Vec<ValueType>,
}

impl FormatModule {
    pub fn new() -> FormatModule {
        FormatModule {
            names: Vec::new(),
            args: Vec::new(),
            formats: Vec::new(),
            format_types: Vec::new(),
        }
    }

    pub fn define_format(&mut self, name: impl Into<String>, format: Format) -> FormatRef {
        self.define_format_args(name, vec![], format)
    }

    pub fn define_format_args(
        &mut self,
        name: impl Into<String>,
        args: Vec<(String, ValueType)>,
        format: Format,
    ) -> FormatRef {
        let mut scope = TypeScope::new();
        for (arg_name, arg_type) in &args {
            scope.push(arg_name.clone(), arg_type.clone());
        }
        let format_type = match self.infer_format_type(&mut scope, &format) {
            Ok(t) => t,
            Err(msg) => panic!("{msg}"),
        };
        let level = self.names.len();
        self.names.push(name.into());
        self.args.push(args);
        self.formats.push(format);
        self.format_types.push(format_type);
        FormatRef(level)
    }

    fn get_name(&self, level: usize) -> &str {
        &self.names[level]
    }

    fn get_args(&self, level: usize) -> &[(String, ValueType)] {
        &self.args[level]
    }

    fn get_format(&self, level: usize) -> &Format {
        &self.formats[level]
    }

    pub fn get_format_type(&self, level: usize) -> &ValueType {
        &self.format_types[level]
    }

    fn infer_format_type(&self, scope: &mut TypeScope, f: &Format) -> Result<ValueType, String> {
        match f {
            Format::ItemVar(level, arg_exprs) => {
                let arg_names = self.get_args(*level);
                for ((_name, arg_type), expr) in Iterator::zip(arg_names.iter(), arg_exprs.iter()) {
                    let t = expr.infer_type(scope)?;
                    let _t = arg_type.unify(&t)?;
                }
                Ok(self.get_format_type(*level).clone())
            }
            Format::Fail => Ok(ValueType::Empty),
            Format::EndOfInput => Ok(ValueType::Tuple(vec![])),
            Format::Align(_n) => Ok(ValueType::Tuple(vec![])),
            Format::Byte(_bs) => Ok(ValueType::U8),
            Format::Union(branches) | Format::NondetUnion(branches) => {
                let mut ts = Vec::with_capacity(branches.len());
                for (label, f) in branches {
                    ts.push((label.clone(), self.infer_format_type(scope, f)?));
                }
                Ok(ValueType::Union(ts))
            }
            Format::Tuple(fields) => {
                let mut ts = Vec::with_capacity(fields.len());
                for f in fields {
                    ts.push(self.infer_format_type(scope, f)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            Format::Record(fields) => {
                let mut ts = Vec::with_capacity(fields.len());
                let initial_len = scope.len();
                for (label, f) in fields {
                    let t = self.infer_format_type(scope, f)?;
                    ts.push((label.clone(), t.clone()));
                    scope.push(label.clone(), t);
                }
                scope.truncate(initial_len);
                if let Some((_l, t)) = ts.iter().find(|(l, _)| l == "@value") {
                    Ok(t.clone())
                } else {
                    Ok(ValueType::Record(ts))
                }
            }
            Format::Repeat(a) | Format::Repeat1(a) => {
                let t = self.infer_format_type(scope, a)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            Format::RepeatCount(_expr, a)
            | Format::RepeatUntilLast(_expr, a)
            | Format::RepeatUntilSeq(_expr, a) => {
                let t = self.infer_format_type(scope, a)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            Format::Peek(a) => self.infer_format_type(scope, a),
            Format::PeekNot(_a) => Ok(ValueType::Tuple(vec![])),
            Format::Slice(_expr, a) => self.infer_format_type(scope, a),
            Format::Bits(a) => self.infer_format_type(scope, a),
            Format::WithRelativeOffset(_expr, a) => self.infer_format_type(scope, a),
            Format::Compute(expr) => expr.infer_type(scope),
            Format::Match(head, branches) => {
                if branches.is_empty() {
                    return Err(format!("infer_format_type: empty Match"));
                }
                let head_type = head.infer_type_coerce_value(scope)?;
                let mut t = ValueType::Any;
                for (pattern, branch) in branches {
                    t = t.unify(
                        &pattern.infer_format_branch_type(scope, &head_type, self, branch)?,
                    )?;
                }
                Ok(t)
            }
            Format::MatchVariant(head, branches) => {
                if branches.is_empty() {
                    return Err(format!("infer_format_type: empty MatchVariant"));
                }
                let head_type = head.infer_type_coerce_value(scope)?;
                let mut t = ValueType::Any;
                for (pattern, label, branch) in branches {
                    t = t.unify(&ValueType::Union(vec![(
                        label.clone(),
                        pattern.infer_format_branch_type(scope, &head_type, self, branch)?,
                    )]))?;
                }
                Ok(t)
            }
            Format::Dynamic(DynFormat::Huffman(lengths_expr, _opt_values_expr)) => {
                match lengths_expr.infer_type_coerce_value(scope)? {
                    ValueType::Seq(t) => match &*t {
                        ValueType::U8 | ValueType::U16 => {}
                        _ => return Err(format!("Huffman: expected U8 or U16")),
                    },
                    _ => return Err(format!("Huffman: expected Seq")),
                }
                // FIXME check opt_values_expr type
                let ts = vec![
                    // FIXME ("bits", alt???)
                    ("@value".to_string(), ValueType::U16),
                ];
                Ok(ValueType::Format(Box::new(ValueType::Record(ts))))
            }
            Format::Apply(name) => match scope.get_type_by_name(name) {
                ValueType::Format(t) => Ok(*t.clone()),
                _ => Err(format!("Apply: expected format")),
            },
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Next<'a> {
    Empty,
    Union(Rc<Next<'a>>, Rc<Next<'a>>),
    Cat(&'a Format, Rc<Next<'a>>),
    Tuple(&'a [Format], Rc<Next<'a>>),
    Record(&'a [(String, Format)], Rc<Next<'a>>),
    Repeat(&'a Format, Rc<Next<'a>>),
    RepeatCount(usize, &'a Format, Rc<Next<'a>>),
    Slice(usize, Rc<Next<'a>>, Rc<Next<'a>>),
    Peek(Rc<Next<'a>>, Rc<Next<'a>>),
    PeekNot(Rc<Next<'a>>, Rc<Next<'a>>),
}

#[derive(Clone, Debug)]
struct MatchTreeStep<'a> {
    accept: bool,
    branches: Vec<(ByteSet, Rc<Next<'a>>)>,
}

#[derive(Clone, Debug)]
struct MatchTreeLevel<'a> {
    accept: Option<usize>,
    branches: Vec<(ByteSet, HashSet<(usize, Rc<Next<'a>>)>)>,
}

#[derive(Clone, Debug)]
pub struct MatchTree {
    accept: Option<usize>,
    branches: Vec<(ByteSet, MatchTree)>,
}

/// Decoders with a fixed amount of lookahead
enum Decoder {
    Call(usize, Vec<(String, Expr)>),
    Fail,
    EndOfInput,
    Align(usize),
    Byte(ByteSet),
    Branch(MatchTree, Vec<(String, Decoder)>),
    Parallel(Vec<(String, Decoder)>),
    Tuple(Vec<Decoder>),
    Record(Vec<(String, Decoder)>),
    While(MatchTree, Box<Decoder>),
    Until(MatchTree, Box<Decoder>),
    RepeatCount(Expr, Box<Decoder>),
    RepeatUntilLast(Expr, Box<Decoder>),
    RepeatUntilSeq(Expr, Box<Decoder>),
    Peek(Box<Decoder>),
    PeekNot(Box<Decoder>),
    Slice(Expr, Box<Decoder>),
    Bits(Box<Decoder>),
    WithRelativeOffset(Expr, Box<Decoder>),
    Compute(Expr),
    Match(Expr, Vec<(Pattern, Decoder)>),
    MatchVariant(Expr, Vec<(Pattern, String, Decoder)>),
    Dynamic(DynFormat),
    Apply(String),
}

impl Expr {
    fn eval(&self, scope: &mut Scope) -> Value {
        match self {
            Expr::Var(name) => scope.get_value_by_name(name).clone(),
            Expr::Bool(b) => Value::Bool(*b),
            Expr::U8(i) => Value::U8(*i),
            Expr::U16(i) => Value::U16(*i),
            Expr::U32(i) => Value::U32(*i),
            Expr::Tuple(exprs) => Value::Tuple(exprs.iter().map(|expr| expr.eval(scope)).collect()),
            Expr::TupleProj(head, index) => match head.eval_value(scope) {
                Value::Tuple(vs) => vs[*index].clone(),
                _ => panic!("expected tuple"),
            },
            Expr::Record(fields) => {
                Value::record(fields.iter().map(|(label, expr)| (label, expr.eval(scope))))
            }
            Expr::RecordProj(head, label) => head.eval(scope).record_proj(label),
            Expr::Variant(label, expr) => Value::variant(label, expr.eval(scope)),
            Expr::Seq(exprs) => Value::Seq(exprs.iter().map(|expr| expr.eval(scope)).collect()),
            Expr::Match(head, branches) => {
                let head = head.eval(scope);
                let initial_len = scope.len();
                let (_, expr) = branches
                    .iter()
                    .find(|(pattern, _)| head.matches(scope, pattern))
                    .expect("exhaustive patterns");
                let value = expr.eval(scope);
                scope.truncate(initial_len);
                value
            }
            Expr::Lambda(_, _) => panic!("cannot eval lambda"),

            Expr::BitAnd(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(x & y),
                (Value::U16(x), Value::U16(y)) => Value::U16(x & y),
                (Value::U32(x), Value::U32(y)) => Value::U32(x & y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::BitOr(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(x | y),
                (Value::U16(x), Value::U16(y)) => Value::U16(x | y),
                (Value::U32(x), Value::U32(y)) => Value::U32(x | y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Eq(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x == y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x == y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x == y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Ne(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x != y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x != y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x != y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Lt(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x < y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x < y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x < y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Gt(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x > y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x > y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x > y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Lte(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x <= y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x <= y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x <= y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Gte(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x >= y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x >= y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x >= y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Mul(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_mul(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_mul(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_mul(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Div(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_div(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_div(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_div(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Rem(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_rem(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_rem(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_rem(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            #[rustfmt::skip]
            Expr::Shl(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_shl(x, u32::from(y)).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_shl(x, u32::from(y)).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_shl(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            #[rustfmt::skip]
            Expr::Shr(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_shr(x, u32::from(y)).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_shr(x, u32::from(y)).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_shr(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Add(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_add(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_add(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_add(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Sub(x, y) => match (x.eval_value(scope), y.eval_value(scope)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_sub(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_sub(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_sub(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },

            Expr::AsU8(x) => match x.eval_value(scope) {
                Value::U8(x) => Value::U8(x),
                Value::U16(x) if x < 256 => Value::U8(x as u8),
                Value::U32(x) if x < 256 => Value::U8(x as u8),
                x => panic!("cannot convert {x:?} to U8"),
            },
            Expr::AsU16(x) => match x.eval_value(scope) {
                Value::U8(x) => Value::U16(u16::from(x)),
                Value::U16(x) => Value::U16(x),
                Value::U32(x) if x < 65536 => Value::U16(x as u16),
                x => panic!("cannot convert {x:?} to U16"),
            },
            Expr::AsU32(x) => match x.eval_value(scope) {
                Value::U8(x) => Value::U32(u32::from(x)),
                Value::U16(x) => Value::U32(u32::from(x)),
                Value::U32(x) => Value::U32(x),
                x => panic!("cannot convert {x:?} to U32"),
            },

            Expr::U16Be(bytes) => match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                [Value::U8(hi), Value::U8(lo)] => Value::U16(u16::from_be_bytes([*hi, *lo])),
                _ => panic!("U16Be: expected (U8, U8)"),
            },
            Expr::U16Le(bytes) => match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                [Value::U8(lo), Value::U8(hi)] => Value::U16(u16::from_le_bytes([*lo, *hi])),
                _ => panic!("U16Le: expected (U8, U8)"),
            },
            Expr::U32Be(bytes) => match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                    Value::U32(u32::from_be_bytes([*a, *b, *c, *d]))
                }
                _ => panic!("U32Be: expected (U8, U8, U8, U8)"),
            },
            Expr::U32Le(bytes) => match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                    Value::U32(u32::from_le_bytes([*a, *b, *c, *d]))
                }
                _ => panic!("U32Le: expected (U8, U8, U8, U8)"),
            },
            Expr::SeqLength(seq) => match seq.eval(scope) {
                Value::Seq(values) => {
                    let len = values.len();
                    Value::U32(len as u32)
                }
                _ => panic!("SeqLength: expected Seq"),
            },
            Expr::SubSeq(seq, start, length) => match seq.eval(scope) {
                Value::Seq(values) => {
                    let start = start.eval_value(scope).unwrap_usize();
                    let length = length.eval_value(scope).unwrap_usize();
                    let values = &values[start..];
                    let values = &values[..length];
                    Value::Seq(values.to_vec())
                }
                _ => panic!("SubSeq: expected Seq"),
            },
            Expr::FlatMap(expr, seq) => match expr.as_ref() {
                Expr::Lambda(name, expr) => match seq.eval(scope) {
                    Value::Seq(values) => {
                        let mut vs = Vec::new();
                        for v in values {
                            scope.push(name.clone(), v);
                            if let Value::Seq(vn) = expr.eval(scope) {
                                vs.extend(vn);
                            } else {
                                panic!("FlatMap: expected Seq");
                            }
                            scope.pop();
                        }
                        Value::Seq(vs)
                    }
                    _ => panic!("FlatMap: expected Seq"),
                },
                _ => panic!("FlatMap: expected Lambda"),
            },
            Expr::FlatMapAccum(expr, accum, _accum_type, seq) => match expr.as_ref() {
                Expr::Lambda(name, expr) => match seq.eval(scope) {
                    Value::Seq(values) => {
                        let mut accum = accum.eval(scope);
                        let mut vs = Vec::new();
                        for v in values {
                            scope.push(name.clone(), Value::Tuple(vec![accum, v]));
                            accum = match expr.eval(scope).unwrap_tuple().as_mut_slice() {
                                [accum, Value::Seq(vn)] => {
                                    vs.extend_from_slice(&vn);
                                    accum.clone()
                                }
                                _ => panic!("FlatMapAccum: expected two values"),
                            };
                            scope.pop();
                        }
                        Value::Seq(vs)
                    }
                    _ => panic!("FlatMapAccum: expected Seq"),
                },
                _ => panic!("FlatMapAccum: expected Lambda"),
            },
            Expr::Dup(count, expr) => {
                let count = count.eval_value(scope).unwrap_usize();
                let v = expr.eval(scope);
                let mut vs = Vec::new();
                for _ in 0..count {
                    vs.push(v.clone());
                }
                Value::Seq(vs)
            }
            Expr::Inflate(seq) => match seq.eval(scope) {
                Value::Seq(values) => {
                    let vs = inflate(&values);
                    Value::Seq(vs)
                }
                _ => panic!("Inflate: expected Seq"),
            },
        }
    }

    fn eval_value(&self, scope: &mut Scope) -> Value {
        self.eval(scope).coerce_record_to_value().clone()
    }

    fn eval_lambda(&self, scope: &mut Scope, arg: Value) -> Value {
        match self {
            Expr::Lambda(name, expr) => {
                scope.push(name.clone(), arg);
                let v = expr.eval_value(scope);
                scope.pop();
                v
            }
            _ => panic!("expected Lambda"),
        }
    }

    fn infer_type(&self, scope: &mut TypeScope) -> Result<ValueType, String> {
        match self {
            Expr::Var(name) => Ok(scope.get_type_by_name(name).clone()),
            Expr::Bool(_b) => Ok(ValueType::Bool),
            Expr::U8(_i) => Ok(ValueType::U8),
            Expr::U16(_i) => Ok(ValueType::U16),
            Expr::U32(_i) => Ok(ValueType::U32),
            Expr::Tuple(exprs) => {
                let mut ts = Vec::new();
                for expr in exprs {
                    ts.push(expr.infer_type(scope)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            Expr::TupleProj(head, index) => match head.infer_type(scope)? {
                ValueType::Tuple(vs) => Ok(vs[*index].clone()),
                _ => Err(format!("expected tuple type")),
            },
            Expr::Record(fields) => {
                let mut fs = Vec::new();
                for (label, expr) in fields {
                    fs.push((label.clone(), expr.infer_type(scope)?));
                }
                Ok(ValueType::Record(fs))
            }
            Expr::RecordProj(head, label) => Ok(head.infer_type(scope)?.record_proj(label)),
            Expr::Variant(label, expr) => Ok(ValueType::Union(vec![(
                label.clone(),
                expr.infer_type(scope)?,
            )])),
            Expr::Seq(exprs) => {
                let mut t = ValueType::Any;
                for e in exprs {
                    t = t.unify(&e.infer_type(scope)?)?;
                }
                Ok(ValueType::Seq(Box::new(t)))
            }
            Expr::Match(head, branches) => {
                if branches.is_empty() {
                    return Err(format!("infer_type: empty Match"));
                }
                let head_type = head.infer_type_coerce_value(scope)?;
                let mut t = ValueType::Any;
                for (pattern, branch) in branches {
                    t = t.unify(&pattern.infer_expr_branch_type(scope, &head_type, branch)?)?;
                }
                Ok(t)
            }
            Expr::Lambda(_, _) => Err(format!("cannot infer_type lambda")),

            Expr::BitAnd(x, y) | Expr::BitOr(x, y) => match (
                x.infer_type_coerce_value(scope)?,
                y.infer_type_coerce_value(scope)?,
            ) {
                (ValueType::U8, ValueType::U8) => Ok(ValueType::U8),
                (ValueType::U16, ValueType::U16) => Ok(ValueType::U16),
                (ValueType::U32, ValueType::U32) => Ok(ValueType::U32),
                (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
            },
            Expr::Eq(x, y)
            | Expr::Ne(x, y)
            | Expr::Lt(x, y)
            | Expr::Gt(x, y)
            | Expr::Lte(x, y)
            | Expr::Gte(x, y) => match (
                x.infer_type_coerce_value(scope)?,
                y.infer_type_coerce_value(scope)?,
            ) {
                (ValueType::U8, ValueType::U8) => Ok(ValueType::Bool),
                (ValueType::U16, ValueType::U16) => Ok(ValueType::Bool),
                (ValueType::U32, ValueType::U32) => Ok(ValueType::Bool),
                (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
            },
            Expr::Add(x, y)
            | Expr::Sub(x, y)
            | Expr::Mul(x, y)
            | Expr::Div(x, y)
            | Expr::Rem(x, y)
            | Expr::Shl(x, y)
            | Expr::Shr(x, y) => match (
                x.infer_type_coerce_value(scope)?,
                y.infer_type_coerce_value(scope)?,
            ) {
                (ValueType::U8, ValueType::U8) => Ok(ValueType::U8),
                (ValueType::U16, ValueType::U16) => Ok(ValueType::U16),
                (ValueType::U32, ValueType::U32) => Ok(ValueType::U32),
                (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
            },

            Expr::AsU8(x) => match x.infer_type_coerce_value(scope)? {
                ValueType::U8 => Ok(ValueType::U8),
                ValueType::U16 => Ok(ValueType::U8),
                ValueType::U32 => Ok(ValueType::U8),
                x => Err(format!("cannot convert {x:?} to U8")),
            },
            Expr::AsU16(x) => match x.infer_type_coerce_value(scope)? {
                ValueType::U8 => Ok(ValueType::U16),
                ValueType::U16 => Ok(ValueType::U16),
                ValueType::U32 => Ok(ValueType::U16),
                x => Err(format!("cannot convert {x:?} to U16")),
            },
            Expr::AsU32(x) => match x.infer_type_coerce_value(scope)? {
                ValueType::U8 => Ok(ValueType::U32),
                ValueType::U16 => Ok(ValueType::U32),
                ValueType::U32 => Ok(ValueType::U32),
                x => Err(format!("cannot convert {x:?} to U32")),
            },

            Expr::U16Be(bytes) => match bytes.infer_type(scope)?.unwrap_tuple_type().as_slice() {
                [ValueType::U8, ValueType::U8] => Ok(ValueType::U16),
                _ => Err(format!("U16Be: expected (U8, U8)")),
            },
            Expr::U16Le(bytes) => match bytes.infer_type(scope)?.unwrap_tuple_type().as_slice() {
                [ValueType::U8, ValueType::U8] => Ok(ValueType::U16),
                _ => Err(format!("U16Le: expected (U8, U8)")),
            },
            Expr::U32Be(bytes) => match bytes.infer_type(scope)?.unwrap_tuple_type().as_slice() {
                [ValueType::U8, ValueType::U8, ValueType::U8, ValueType::U8] => Ok(ValueType::U32),
                _ => Err(format!("U32Be: expected (U8, U8, U8, U8)")),
            },
            Expr::U32Le(bytes) => match bytes.infer_type(scope)?.unwrap_tuple_type().as_slice() {
                [ValueType::U8, ValueType::U8, ValueType::U8, ValueType::U8] => Ok(ValueType::U32),
                _ => Err(format!("U32Le: expected (U8, U8, U8, U8)")),
            },
            Expr::SeqLength(seq) => match seq.infer_type(scope)? {
                ValueType::Seq(_t) => Ok(ValueType::U32),
                _ => Err(format!("SeqLength: expected Seq")),
            },
            Expr::SubSeq(seq, start, length) => match seq.infer_type(scope)? {
                ValueType::Seq(t) => {
                    let start_type = start.infer_type_coerce_value(scope)?;
                    let length_type = length.infer_type_coerce_value(scope)?;
                    if !start_type.is_numeric_type() {
                        return Err(format!("SubSeq start must be numeric"));
                    }
                    if !length_type.is_numeric_type() {
                        return Err(format!("SubSeq length must be numeric"));
                    }
                    Ok(ValueType::Seq(t))
                }
                _ => Err(format!("SubSeq: expected Seq")),
            },
            Expr::FlatMap(expr, seq) => match expr.as_ref() {
                Expr::Lambda(name, expr) => match seq.infer_type(scope)? {
                    ValueType::Seq(t) => {
                        scope.push(name.clone(), *t);
                        let t2 = if let ValueType::Seq(t2) = expr.infer_type(scope)? {
                            t2
                        } else {
                            return Err(format!("FlatMap: expected Seq"));
                        };
                        scope.pop();
                        Ok(ValueType::Seq(t2))
                    }
                    _ => Err(format!("FlatMap: expected Seq")),
                },
                _ => Err(format!("FlatMap: expected Lambda")),
            },
            Expr::FlatMapAccum(expr, accum, accum_type, seq) => match expr.as_ref() {
                Expr::Lambda(name, expr) => match seq.infer_type(scope)? {
                    ValueType::Seq(t) => {
                        let accum_type = accum.infer_type(scope)?.unify(&accum_type)?;
                        scope.push(name.clone(), ValueType::Tuple(vec![accum_type.clone(), *t]));
                        let t2 = match expr.infer_type(scope)?.unwrap_tuple_type().as_mut_slice() {
                            [accum_result, ValueType::Seq(t2)] => {
                                accum_result.unify(&accum_type)?;
                                t2.clone()
                            }
                            _ => panic!("FlatMapAccum: expected two values"),
                        };
                        scope.pop();
                        Ok(ValueType::Seq(t2))
                    }
                    _ => Err(format!("FlatMapAccum: expected Seq")),
                },
                _ => Err(format!("FlatMapAccum: expected Lambda")),
            },
            Expr::Dup(count, expr) => {
                if !count.infer_type_coerce_value(scope)?.is_numeric_type() {
                    return Err(format!("Dup count must be numeric"));
                }
                let t = expr.infer_type(scope)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            Expr::Inflate(seq) => match seq.infer_type(scope)? {
                // FIXME should check values are appropriate variants
                ValueType::Seq(_values) => Ok(ValueType::Seq(Box::new(ValueType::U8))),
                _ => Err(format!("Inflate: expected Seq")),
            },
        }
    }

    fn infer_type_coerce_value(&self, scope: &mut TypeScope) -> Result<ValueType, String> {
        match self.infer_type(scope)? {
            ValueType::Record(fields) => {
                if let Some((_l, t)) = fields.iter().find(|(l, _)| l == "@value") {
                    return Ok(t.clone());
                } else {
                    Ok(ValueType::Record(fields))
                }
            }
            t => Ok(t),
        }
    }

    /// Conservative bounds for unsigned numeric expressions
    fn bounds(&self) -> Bounds {
        match self {
            Expr::U8(n) => Bounds::exact(usize::from(*n)),
            Expr::U16(n) => Bounds::exact(usize::from(*n)),
            Expr::U32(n) => Bounds::exact(*n as usize),
            Expr::Add(a, b) => a.bounds() + b.bounds(),
            Expr::Mul(a, b) => a.bounds() * b.bounds(),
            _ => Bounds::new(0, None),
        }
    }
}

impl Format {
    /// Conservative bounds for number of bytes matched by a format
    fn match_bounds(&self, module: &FormatModule) -> Bounds {
        match self {
            Format::ItemVar(level, _args) => module.get_format(*level).match_bounds(module),
            Format::Fail => Bounds::exact(0),
            Format::EndOfInput => Bounds::exact(0),
            Format::Align(n) => Bounds::new(0, Some(n - 1)),
            Format::Byte(_) => Bounds::exact(1),
            Format::Union(branches) | Format::NondetUnion(branches) => branches
                .iter()
                .map(|(_, f)| f.match_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Tuple(fields) => fields
                .iter()
                .map(|f| f.match_bounds(module))
                .reduce(Bounds::add)
                .unwrap_or(Bounds::exact(0)),
            Format::Record(fields) => fields
                .iter()
                .map(|(_, f)| f.match_bounds(module))
                .reduce(Bounds::add)
                .unwrap_or(Bounds::exact(0)),
            Format::Repeat(_) => Bounds::new(0, None),
            Format::Repeat1(f) => f.match_bounds(module) * Bounds::new(1, None),
            Format::RepeatCount(expr, f) => f.match_bounds(module) * expr.bounds(),
            Format::RepeatUntilLast(_, f) => f.match_bounds(module) * Bounds::new(1, None),
            Format::RepeatUntilSeq(_, _f) => Bounds::new(0, None),
            Format::Peek(_) => Bounds::exact(0),
            Format::PeekNot(_) => Bounds::exact(0),
            Format::Slice(expr, _) => expr.bounds(),
            Format::Bits(f) => f.match_bounds(module).bits_to_bytes(),
            Format::WithRelativeOffset(_, _) => Bounds::exact(0),
            Format::Compute(_) => Bounds::exact(0),
            Format::Match(_, branches) => branches
                .iter()
                .map(|(_, f)| f.match_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::MatchVariant(_, branches) => branches
                .iter()
                .map(|(_, _, f)| f.match_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Dynamic(DynFormat::Huffman(_, _)) => Bounds::exact(0),
            Format::Apply(_) => Bounds::new(1, None),
        }
    }

    /// Returns `true` if the format could match the empty byte string
    fn is_nullable(&self, module: &FormatModule) -> bool {
        self.match_bounds(module).min == 0
    }

    /// True if the compilation of this format depends on the format that follows it
    fn depends_on_next(&self, module: &FormatModule) -> bool {
        match self {
            Format::ItemVar(level, _args) => module.get_format(*level).depends_on_next(module),
            Format::Fail => false,
            Format::EndOfInput => false,
            Format::Align(_) => false,
            Format::Byte(_) => false,
            Format::Union(branches) | Format::NondetUnion(branches) => {
                Format::union_depends_on_next(&branches, module)
            }
            Format::Tuple(fields) => fields.iter().any(|f| f.depends_on_next(module)),
            Format::Record(fields) => fields.iter().any(|(_, f)| f.depends_on_next(module)),
            Format::Repeat(_) => true,
            Format::Repeat1(_) => true,
            Format::RepeatCount(_, _f) => false,
            Format::RepeatUntilLast(_, _f) => false,
            Format::RepeatUntilSeq(_, _f) => false,
            Format::Peek(_) => false,
            Format::PeekNot(_) => false,
            Format::Slice(_, _) => false,
            Format::Bits(_) => false,
            Format::WithRelativeOffset(_, _) => false,
            Format::Compute(_) => false,
            Format::Match(_, branches) => branches.iter().any(|(_, f)| f.depends_on_next(module)),
            Format::MatchVariant(_, branches) => {
                branches.iter().any(|(_, _, f)| f.depends_on_next(module))
            }
            Format::Dynamic(_) => false,
            Format::Apply(_) => false,
        }
    }

    fn union_depends_on_next(branches: &[(String, Format)], module: &FormatModule) -> bool {
        let mut fs = Vec::with_capacity(branches.len());
        for (_label, f) in branches {
            if f.depends_on_next(module) {
                return true;
            }
            fs.push(f.clone());
        }
        MatchTree::build(module, &fs, Rc::new(Next::Empty)).is_none()
    }
}

impl Format {
    /// Returns `true` if values associated to this format should be handled as single ASCII characters
    pub fn is_ascii_char_format(&self, module: &FormatModule) -> bool {
        match self {
            // NOTE - currently only true for named formats matching 'base\.ascii-char.*'
            Format::ItemVar(level, _args) => module.get_name(*level).starts_with("base.ascii-char"),
            _ => false,
        }
    }

    /// Returns `true` if values associated to this format should be handled as multi-character ASCII strings
    pub fn is_ascii_string_format(&self, module: &FormatModule) -> bool {
        match self {
            Format::ItemVar(level, _args) => {
                let fmt_name = module.get_name(*level);
                // REVIEW - consider different heuristic for short-circuit
                if fmt_name.contains("ascii-string") || fmt_name.contains("asciiz-string") {
                    return true;
                }
                module.get_format(*level).is_ascii_string_format(module)
            }
            Format::Tuple(formats) => {
                !formats.is_empty() && formats.iter().all(|f| f.is_ascii_char_format(module))
            }
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::RepeatCount(_, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => format.is_ascii_char_format(module),
            Format::Slice(_, format) => format.is_ascii_string_format(module),
            // NOTE there may be other cases we should consider ASCII
            _ => false,
        }
    }
}

impl<'a> MatchTreeStep<'a> {
    fn reject() -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: false,
            branches: vec![],
        }
    }

    fn accept() -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: true,
            branches: vec![],
        }
    }

    fn branch(bs: ByteSet, next: Rc<Next<'a>>) -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: false,
            branches: vec![(bs, next)],
        }
    }

    fn union_branch(&mut self, mut bs: ByteSet, next: Rc<Next<'a>>) {
        let mut branches = Vec::new();
        for (bs0, next0) in self.branches.iter_mut() {
            let common = bs0.intersection(&bs);
            if !common.is_empty() {
                let orig = bs0.difference(&bs);
                if !orig.is_empty() {
                    branches.push((orig, next0.clone()));
                }
                *bs0 = common;
                *next0 = Rc::new(Next::Union(next0.clone(), next.clone()));
                bs = bs.difference(bs0);
            }
        }
        if !bs.is_empty() {
            self.branches.push((bs, next));
        }
        self.branches.append(&mut branches);
    }

    fn union(mut self, other: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        self.accept = self.accept || other.accept;
        for (bs, next) in other.branches {
            self.union_branch(bs, next);
        }
        self
    }

    fn peek(mut self, peek: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        self.accept = self.accept && peek.accept;
        if peek.accept {
            // do nothing
        } else if self.accept {
            self.branches = peek.branches;
        } else {
            let mut branches = Vec::new();
            for (bs1, next1) in self.branches {
                for (bs2, next2) in &peek.branches {
                    let bs = bs1.intersection(bs2);
                    if !bs.is_empty() {
                        let next = Rc::new(Next::Peek(next1.clone(), next2.clone()));
                        branches.push((bs, next));
                    }
                }
            }
            self.branches = branches;
        }
        self
    }

    fn peek_not(mut self, peek: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        self.accept = self.accept && !peek.accept;
        if peek.accept {
            self.branches = Vec::new();
        } else {
            let mut branches = Vec::new();
            for (bs1, next1) in self.branches.into_iter() {
                for (bs2, next2) in &peek.branches {
                    let common = bs1.intersection(bs2);
                    let diff = bs1.difference(bs2);
                    if !common.is_empty() {
                        let next = Rc::new(Next::PeekNot(next1.clone(), next2.clone()));
                        branches.push((common, next));
                    }
                    if !diff.is_empty() {
                        branches.push((diff, next1.clone()));
                    }
                }
            }
            self.branches = branches;
        }
        self
    }

    fn add_tuple(
        module: &'a FormatModule,
        fields: &'a [Format],
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        match fields.split_first() {
            None => Self::add_next(module, next),
            Some((f, fs)) => Self::add(module, f, Rc::new(Next::Tuple(fs, next))),
        }
    }

    fn add_record(
        module: &'a FormatModule,
        fields: &'a [(String, Format)],
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        match fields.split_first() {
            None => Self::add_next(module, next),
            Some(((_label, f), fs)) => Self::add(module, f, Rc::new(Next::Record(fs, next))),
        }
    }

    fn add_repeat_count(
        module: &'a FormatModule,
        n: usize,
        f: &'a Format,
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        if n > 0 {
            Self::add(module, f, Rc::new(Next::RepeatCount(n - 1, f, next)))
        } else {
            Self::add_next(module, next)
        }
    }

    pub fn add_slice(
        module: &'a FormatModule,
        n: usize,
        inside: Rc<Next<'a>>,
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        if n > 0 {
            let mut tree = Self::add_next(module, inside);
            tree.accept = false;
            if tree.branches.is_empty() {
                let next = Rc::new(Next::Slice(n - 1, Rc::new(Next::Empty), next.clone()));
                tree.branches.push((ByteSet::full(), next));
            } else {
                for (_bs, ref mut inside) in tree.branches.iter_mut() {
                    *inside = Rc::new(Next::Slice(n - 1, inside.clone(), next.clone()));
                }
            }
            tree
        } else {
            Self::add_next(module, next.clone())
        }
    }

    fn add_next(module: &'a FormatModule, next: Rc<Next<'a>>) -> MatchTreeStep<'a> {
        match next.as_ref() {
            Next::Empty => Self::accept(),
            Next::Union(next1, next2) => {
                let tree1 = Self::add_next(module, next1.clone());
                let tree2 = Self::add_next(module, next2.clone());
                tree1.union(tree2)
            }
            Next::Cat(f, next) => Self::add(module, f, next.clone()),
            Next::Tuple(fields, next) => Self::add_tuple(module, fields, next.clone()),
            Next::Record(fields, next) => Self::add_record(module, fields, next.clone()),
            Next::Repeat(a, next0) => {
                let tree = Self::add_next(module, next0.clone());
                tree.union(Self::add(module, a, next))
            }
            Next::RepeatCount(n, a, next0) => Self::add_repeat_count(module, *n, a, next0.clone()),
            Next::Slice(n, inside, next0) => {
                Self::add_slice(module, *n, inside.clone(), next0.clone())
            }
            Next::Peek(next1, next2) => {
                let tree1 = Self::add_next(module, next1.clone());
                let tree2 = Self::add_next(module, next2.clone());
                tree1.peek(tree2)
            }
            Next::PeekNot(next1, next2) => {
                let tree1 = Self::add_next(module, next1.clone());
                let tree2 = Self::add_next(module, next2.clone());
                tree1.peek_not(tree2)
            }
        }
    }

    pub fn add(module: &'a FormatModule, f: &'a Format, next: Rc<Next<'a>>) -> MatchTreeStep<'a> {
        match f {
            Format::ItemVar(level, _args) => Self::add(module, module.get_format(*level), next),
            Format::Fail => Self::reject(),
            Format::EndOfInput => Self::accept(),
            Format::Align(_) => {
                Self::accept() // FIXME
            }
            Format::Byte(bs) => Self::branch(*bs, next),
            Format::Union(branches) | Format::NondetUnion(branches) => {
                let mut tree = Self::reject();
                for (_, f) in branches {
                    tree = tree.union(Self::add(module, f, next.clone()));
                }
                tree
            }
            Format::Tuple(fields) => Self::add_tuple(module, fields, next),
            Format::Record(fields) => Self::add_record(module, fields, next),
            Format::Repeat(a) => {
                let tree = Self::add_next(module, next.clone());
                tree.union(Self::add(module, a, Rc::new(Next::Repeat(a, next.clone()))))
            }
            Format::Repeat1(a) => Self::add(module, a, Rc::new(Next::Repeat(a, next.clone()))),
            Format::RepeatCount(expr, a) => {
                let bounds = expr.bounds();
                if let Some(n) = bounds.is_exact() {
                    Self::add_repeat_count(module, n, a, next.clone())
                } else {
                    Self::add_repeat_count(module, bounds.min, a, Rc::new(Next::Empty))
                }
            }
            Format::RepeatUntilLast(_expr, _a) => {
                Self::accept() // FIXME
            }
            Format::RepeatUntilSeq(_expr, _a) => {
                Self::accept() // FIXME
            }
            Format::Peek(a) => {
                let tree = Self::add_next(module, next.clone());
                let peek = Self::add(module, a, Rc::new(Next::Empty));
                tree.peek(peek)
            }
            Format::PeekNot(a) => {
                let tree = Self::add_next(module, next.clone());
                let peek = Self::add(module, a, Rc::new(Next::Empty));
                tree.peek_not(peek)
            }
            Format::Slice(expr, f) => {
                let inside = Rc::new(Next::Cat(f, Rc::new(Next::Empty)));
                let bounds = expr.bounds();
                if let Some(n) = bounds.is_exact() {
                    Self::add_slice(module, n, inside, next)
                } else {
                    Self::add_slice(module, bounds.min, inside, Rc::new(Next::Empty))
                }
            }
            Format::Bits(_a) => {
                Self::accept() // FIXME
            }
            Format::WithRelativeOffset(_expr, _a) => {
                Self::accept() // FIXME
            }
            Format::Compute(_expr) => Self::add_next(module, next),
            Format::Match(_, branches) => {
                let mut tree = Self::reject();
                for (_, f) in branches {
                    tree = tree.union(Self::add(module, f, next.clone()));
                }
                tree
            }
            Format::MatchVariant(_, branches) => {
                let mut tree = Self::reject();
                for (_, _, f) in branches {
                    tree = tree.union(Self::add(module, f, next.clone()));
                }
                tree
            }
            Format::Dynamic(DynFormat::Huffman(_, _)) => Self::add_next(module, next),
            Format::Apply(_name) => Self::accept(),
        }
    }
}

impl<'a> MatchTreeLevel<'a> {
    fn reject() -> MatchTreeLevel<'a> {
        MatchTreeLevel {
            accept: None,
            branches: vec![],
        }
    }

    fn merge_accept(&mut self, index: usize) -> Result<(), ()> {
        match self.accept {
            None => {
                self.accept = Some(index);
                Ok(())
            }
            Some(i) if i == index => Ok(()),
            Some(_) => Err(()),
        }
    }

    fn merge_branch(
        &mut self,
        index: usize,
        mut bs: ByteSet,
        next: Rc<Next<'a>>,
    ) -> Result<(), ()> {
        let mut new_branches = Vec::new();
        for (bs0, nexts) in self.branches.iter_mut() {
            let common = bs0.intersection(&bs);
            if !common.is_empty() {
                let orig = bs0.difference(&bs);
                if !orig.is_empty() {
                    new_branches.push((orig, nexts.clone()));
                }
                *bs0 = common;
                nexts.insert((index, next.clone()));
                bs = bs.difference(bs0);
            }
        }
        if !bs.is_empty() {
            let mut nexts = HashSet::new();
            nexts.insert((index, next.clone()));
            self.branches.push((bs, nexts));
        }
        self.branches.append(&mut new_branches);
        Ok(())
    }

    fn merge(mut self, index: usize, other: MatchTreeStep<'a>) -> Result<MatchTreeLevel<'a>, ()> {
        if other.accept {
            self.merge_accept(index)?;
        }
        for (bs, next) in other.branches {
            self.merge_branch(index, bs, next)?;
        }
        Ok(self)
    }

    fn accepts(nexts: &HashSet<(usize, Rc<Next<'a>>)>) -> Option<MatchTree> {
        let mut tree = Self::reject();
        for (i, _next) in nexts.iter() {
            tree.merge_accept(*i).ok()?;
        }
        Some(MatchTree {
            accept: tree.accept,
            branches: vec![],
        })
    }

    fn grow(
        module: &'a FormatModule,
        nexts: HashSet<(usize, Rc<Next<'a>>)>,
        depth: usize,
    ) -> Option<MatchTree> {
        if let Some(tree) = Self::accepts(&nexts) {
            Some(tree)
        } else if depth > 0 {
            let mut tree = Self::reject();
            for (i, next) in nexts {
                let subtree = MatchTreeStep::add_next(module, next);
                tree = tree.merge(i, subtree).ok()?;
            }
            let mut branches = Vec::new();
            for (bs, nexts) in tree.branches {
                let t = Self::grow(module, nexts, depth - 1)?;
                branches.push((bs, t));
            }
            Some(MatchTree {
                accept: tree.accept,
                branches,
            })
        } else {
            None
        }
    }
}

impl MatchTree {
    fn matches<'a>(&self, input: ReadCtxt<'a>) -> Option<usize> {
        match input.read_byte() {
            None => self.accept,
            Some((b, input)) => {
                for (bs, s) in &self.branches {
                    if bs.contains(b) {
                        return s.matches(input);
                    }
                }
                self.accept
            }
        }
    }

    fn build(module: &FormatModule, branches: &[Format], next: Rc<Next<'_>>) -> Option<MatchTree> {
        let mut nexts = HashSet::new();
        for (i, f) in branches.iter().enumerate() {
            nexts.insert((i, Rc::new(Next::Cat(f, next.clone()))));
        }
        const MAX_DEPTH: usize = 32;
        MatchTreeLevel::grow(module, nexts, MAX_DEPTH)
    }
}

#[derive(Copy, Clone)]
pub struct ReadCtxt<'a> {
    input: &'a [u8],
    pub offset: usize,
}

impl<'a> ReadCtxt<'a> {
    pub fn new(input: &'a [u8]) -> ReadCtxt<'a> {
        let offset = 0;
        ReadCtxt { input, offset }
    }

    pub fn remaining(&self) -> &'a [u8] {
        &self.input[self.offset..]
    }

    pub fn read_byte(&self) -> Option<(u8, ReadCtxt<'a>)> {
        if self.offset < self.input.len() {
            let b = self.input[self.offset];
            Some((
                b,
                ReadCtxt {
                    input: self.input,
                    offset: self.offset + 1,
                },
            ))
        } else {
            None
        }
    }

    pub fn split_at(&self, n: usize) -> Option<(ReadCtxt<'a>, ReadCtxt<'a>)> {
        if self.offset + n <= self.input.len() {
            let fst = ReadCtxt {
                input: &self.input[..self.offset + n],
                offset: self.offset,
            };
            let snd = ReadCtxt {
                input: self.input,
                offset: self.offset + n,
            };
            Some((fst, snd))
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum TypeRef {
    Var(usize),
    Empty,
    Bool,
    U8,
    U16,
    U32,
    Tuple(Vec<TypeRef>),
    Seq(Box<TypeRef>),
    Format(Box<TypeRef>),
}

pub enum TypeDef {
    //Equiv(TypeRef),
    Union(Vec<(String, TypeRef)>),
    Record(Vec<(String, TypeRef)>),
}

pub struct Program {
    typedefs: Vec<TypeDef>,
    decoders: Vec<Decoder>,
}

impl Program {
    fn new() -> Self {
        let typedefs = Vec::new();
        let decoders = Vec::new();
        Program { typedefs, decoders }
    }

    pub fn run<'input>(&self, input: ReadCtxt<'input>) -> ParseResult<(Value, ReadCtxt<'input>)> {
        let mut scope = Scope::new();
        self.decoders[0].parse(self, &mut scope, input)
    }
}

pub struct Compiler<'a> {
    module: &'a FormatModule,
    program: Program,
    record_map: HashMap<Vec<(String, TypeRef)>, usize>,
    union_map: HashMap<Vec<(String, TypeRef)>, usize>,
    decoder_map: HashMap<(usize, Rc<Next<'a>>), usize>,
}

impl<'a> Compiler<'a> {
    fn new(module: &'a FormatModule) -> Self {
        let program = Program::new();
        let record_map = HashMap::new();
        let union_map = HashMap::new();
        let decoder_map = HashMap::new();
        Compiler {
            module,
            program,
            record_map,
            union_map,
            decoder_map,
        }
    }

    pub fn compile(module: &FormatModule, format: &Format) -> Result<Program, String> {
        let mut compiler = Compiler::new(module);
        // type
        /*
        let mut scope = TypeScope::new();
        let t = TypeRef::from_value_type(
            &mut compiler,
            &module.infer_format_type(&mut scope, format)?,
        );
        */
        // decoder
        let n = compiler.program.decoders.len();
        compiler.program.decoders.push(Decoder::Fail);
        let d = Decoder::compile(&mut compiler, format)?;
        compiler.program.decoders[n] = d;
        Ok(compiler.program)
    }

    pub fn add_typedef(&mut self, t: TypeDef) -> TypeRef {
        let n = self.program.typedefs.len();
        self.program.typedefs.push(t);
        TypeRef::Var(n)
    }
}

pub struct TypeScope {
    names: Vec<String>,
    types: Vec<ValueType>,
}

pub struct Scope {
    names: Vec<String>,
    values: Vec<Value>,
    decoders: Vec<Option<Decoder>>,
}

pub struct ScopeIter {
    name_iter: std::vec::IntoIter<String>,
    value_iter: std::vec::IntoIter<Value>,
}

impl Iterator for ScopeIter {
    type Item = (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.name_iter.next(), self.value_iter.next()) {
            (Some(name), Some(value)) => Some((name, value)),
            _ => None,
        }
    }
}

impl IntoIterator for &Scope {
    type Item = (String, Value);

    type IntoIter = ScopeIter;

    fn into_iter(self) -> Self::IntoIter {
        ScopeIter {
            name_iter: self.names.clone().into_iter(),
            value_iter: self.values.clone().into_iter(),
        }
    }
}

impl Scope {
    pub fn iter(&self) -> impl Iterator<Item = (String, Value)> {
        (&self).into_iter()
    }
}

impl TypeScope {
    fn new() -> Self {
        let names = Vec::new();
        let types = Vec::new();
        TypeScope { names, types }
    }

    fn push(&mut self, name: String, t: ValueType) {
        self.names.push(name);
        self.types.push(t);
    }

    fn pop(&mut self) -> ValueType {
        self.names.pop();
        self.types.pop().unwrap()
    }

    fn len(&self) -> usize {
        self.types.len()
    }

    fn truncate(&mut self, len: usize) {
        self.names.truncate(len);
        self.types.truncate(len);
    }

    fn get_type_by_name(&self, name: &str) -> &ValueType {
        for (i, n) in self.names.iter().enumerate().rev() {
            if n == name {
                return &self.types[i];
            }
        }
        panic!("variable not found: {name}");
    }
}

impl Scope {
    fn new() -> Self {
        let names = Vec::new();
        let values = Vec::new();
        let decoders = Vec::new();
        Scope {
            names,
            values,
            decoders,
        }
    }

    fn push(&mut self, name: String, v: Value) {
        self.names.push(name);
        self.values.push(v);
        self.decoders.push(None);
    }

    fn pop(&mut self) -> Value {
        self.names.pop();
        self.decoders.pop();
        self.values.pop().unwrap()
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn truncate(&mut self, len: usize) {
        self.names.truncate(len);
        self.values.truncate(len);
        self.decoders.truncate(len);
    }

    fn get_index_by_name(&self, name: &str) -> usize {
        for (i, n) in self.names.iter().enumerate().rev() {
            if n == name {
                return i;
            }
        }
        panic!("variable not found: {name}");
    }

    fn get_value_by_name(&self, name: &str) -> &Value {
        &self.values[self.get_index_by_name(name)]
    }

    fn call_decoder_by_name<'input>(
        &mut self,
        name: &str,
        program: &Program,
        input: ReadCtxt<'input>,
    ) -> ParseResult<(Value, ReadCtxt<'input>)> {
        let i = self.get_index_by_name(name);
        let mut od = std::mem::replace(&mut self.decoders[i], None);
        if od.is_none() {
            let d = match &self.values[i] {
                Value::Format(f) => Decoder::compile_one(&*f).unwrap(),
                _ => panic!("variable not format: {name}"),
            };
            od = Some(d);
        }
        let res = od.as_ref().unwrap().parse(program, self, input);
        self.decoders[i] = od;
        res
    }
}

impl TypeRef {
    #[allow(dead_code)]
    fn from_value_type<'a>(compiler: &mut Compiler<'a>, t: &ValueType) -> Self {
        match t {
            ValueType::Any => panic!("ValueType::Any"),
            ValueType::Empty => TypeRef::Empty,
            ValueType::Bool => TypeRef::Bool,
            ValueType::U8 => TypeRef::U8,
            ValueType::U16 => TypeRef::U16,
            ValueType::U32 => TypeRef::U32,
            ValueType::Tuple(ts) => TypeRef::Tuple(
                ts.iter()
                    .map(|t| Self::from_value_type(compiler, t))
                    .collect(),
            ),
            ValueType::Record(fields) => {
                let fs: Vec<_> = fields
                    .iter()
                    .map(|(label, t)| (label.clone(), Self::from_value_type(compiler, t)))
                    .collect();
                let n = if let Some(n) = compiler.record_map.get(&fs) {
                    *n
                } else {
                    let t = TypeDef::Record(fs.clone());
                    let n = compiler.program.typedefs.len();
                    compiler.program.typedefs.push(t);
                    compiler.record_map.insert(fs, n);
                    n
                };
                TypeRef::Var(n)
            }
            ValueType::Union(branches) => {
                let bs: Vec<_> = branches
                    .iter()
                    .map(|(label, t)| (label.clone(), Self::from_value_type(compiler, t)))
                    .collect();
                let n = if let Some(n) = compiler.union_map.get(&bs) {
                    *n
                } else {
                    let t = TypeDef::Union(bs.clone());
                    let n = compiler.program.typedefs.len();
                    compiler.program.typedefs.push(t);
                    compiler.union_map.insert(bs, n);
                    n
                };
                TypeRef::Var(n)
            }
            ValueType::Seq(t) => TypeRef::Seq(Box::new(Self::from_value_type(compiler, &*t))),
            ValueType::Format(t) => TypeRef::Format(Box::new(Self::from_value_type(compiler, &*t))),
        }
    }

    #[allow(dead_code)]
    fn to_value_type(&self, typedefs: &[TypeDef]) -> ValueType {
        match self {
            TypeRef::Var(n) => match &typedefs[*n] {
                //TypeDef::Equiv(t) => t.to_value_type(typedefs),
                TypeDef::Union(ts) => ValueType::Union(
                    ts.iter()
                        .map(|(name, t)| (name.clone(), t.to_value_type(typedefs)))
                        .collect(),
                ),
                TypeDef::Record(ts) => ValueType::Record(
                    ts.iter()
                        .map(|(name, t)| (name.clone(), t.to_value_type(typedefs)))
                        .collect(),
                ),
            },
            TypeRef::Empty => ValueType::Empty,
            TypeRef::Bool => ValueType::Bool,
            TypeRef::U8 => ValueType::U8,
            TypeRef::U16 => ValueType::U16,
            TypeRef::U32 => ValueType::U32,
            TypeRef::Tuple(ts) => {
                ValueType::Tuple(ts.iter().map(|t| t.to_value_type(typedefs)).collect())
            }
            TypeRef::Seq(t) => ValueType::Seq(Box::new(t.to_value_type(typedefs))),
            TypeRef::Format(t) => ValueType::Format(Box::new(t.to_value_type(typedefs))),
        }
    }
}

impl Decoder {
    pub fn compile_one(format: &Format) -> Result<Decoder, String> {
        let module = FormatModule::new();
        let mut compiler = Compiler::new(&module);
        Decoder::compile(&mut compiler, format)
    }

    pub fn compile<'a>(compiler: &mut Compiler<'a>, format: &'a Format) -> Result<Decoder, String> {
        Decoder::compile_next(compiler, format, Rc::new(Next::Empty))
    }

    fn compile_next<'a>(
        compiler: &mut Compiler<'a>,
        format: &'a Format,
        next: Rc<Next<'a>>,
    ) -> Result<Decoder, String> {
        match format {
            Format::ItemVar(level, arg_exprs) => {
                let next = if compiler
                    .module
                    .get_format(*level)
                    .depends_on_next(compiler.module)
                {
                    next
                } else {
                    Rc::new(Next::Empty)
                };
                let n = if let Some(n) = compiler.decoder_map.get(&(*level, next.clone())) {
                    *n
                } else {
                    let d = Decoder::compile_next(
                        compiler,
                        compiler.module.get_format(*level),
                        next.clone(),
                    )?;
                    let n = compiler.program.decoders.len();
                    compiler.program.decoders.push(d);
                    compiler.decoder_map.insert((*level, next.clone()), n);
                    n
                };
                let arg_names = compiler.module.get_args(*level);
                let mut args = Vec::new();
                for ((name, _type), expr) in Iterator::zip(arg_names.iter(), arg_exprs.iter()) {
                    args.push((name.clone(), expr.clone()));
                }
                Ok(Decoder::Call(n, args))
            }
            Format::Fail => Ok(Decoder::Fail),
            Format::EndOfInput => Ok(Decoder::EndOfInput),
            Format::Align(n) => Ok(Decoder::Align(*n)),
            Format::Byte(bs) => Ok(Decoder::Byte(*bs)),
            Format::Union(branches) => {
                let mut fs = Vec::with_capacity(branches.len());
                let mut ds = Vec::with_capacity(branches.len());
                for (label, f) in branches {
                    ds.push((
                        label.clone(),
                        Decoder::compile_next(compiler, f, next.clone())?,
                    ));
                    fs.push(f.clone());
                }
                if let Some(tree) = MatchTree::build(compiler.module, &fs, next) {
                    Ok(Decoder::Branch(tree, ds))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::NondetUnion(branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for (label, f) in branches {
                    ds.push((
                        label.clone(),
                        Decoder::compile_next(compiler, f, next.clone())?,
                    ));
                }
                Ok(Decoder::Parallel(ds))
            }
            Format::Tuple(fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Tuple(fields.as_slice(), next.clone()));
                    let df = Decoder::compile_next(compiler, f, next)?;
                    dfields.push(df);
                }
                Ok(Decoder::Tuple(dfields))
            }
            Format::Record(fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some((name, f)) = fields.next() {
                    let next = Rc::new(Next::Record(fields.as_slice(), next.clone()));
                    let df = Decoder::compile_next(compiler, f, next)?;
                    dfields.push((name.clone(), df));
                }
                Ok(Decoder::Record(dfields))
            }
            Format::Repeat(a) => {
                if a.is_nullable(compiler.module) {
                    return Err(format!("cannot repeat nullable format: {a:?}"));
                }
                let da =
                    Decoder::compile_next(compiler, a, Rc::new(Next::Repeat(a, next.clone())))?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::Tuple(vec![(**a).clone(), astar]);
                let fb = Format::EMPTY;
                if let Some(tree) = MatchTree::build(compiler.module, &[fa, fb], next) {
                    Ok(Decoder::While(tree, Box::new(da)))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::Repeat1(a) => {
                if a.is_nullable(compiler.module) {
                    return Err(format!("cannot repeat nullable format: {a:?}"));
                }
                let da =
                    Decoder::compile_next(compiler, a, Rc::new(Next::Repeat(a, next.clone())))?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::EMPTY;
                let fb = Format::Tuple(vec![(**a).clone(), astar]);
                if let Some(tree) = MatchTree::build(compiler.module, &[fa, fb], next) {
                    Ok(Decoder::Until(tree, Box::new(da)))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::RepeatCount(expr, a) => {
                // FIXME probably not right
                let da = Box::new(Decoder::compile_next(compiler, a, next)?);
                Ok(Decoder::RepeatCount(expr.clone(), da))
            }
            Format::RepeatUntilLast(expr, a) => {
                // FIXME probably not right
                let da = Box::new(Decoder::compile_next(compiler, a, next)?);
                Ok(Decoder::RepeatUntilLast(expr.clone(), da))
            }
            Format::RepeatUntilSeq(expr, a) => {
                // FIXME probably not right
                let da = Box::new(Decoder::compile_next(compiler, a, next)?);
                Ok(Decoder::RepeatUntilSeq(expr.clone(), da))
            }
            Format::Peek(a) => {
                let da = Box::new(Decoder::compile_next(compiler, a, Rc::new(Next::Empty))?);
                Ok(Decoder::Peek(da))
            }
            Format::PeekNot(a) => {
                const MAX_LOOKAHEAD: usize = 1024;
                match a.match_bounds(compiler.module).max {
                    None => return Err(format!("PeekNot cannot require unbounded lookahead")),
                    Some(n) if n > MAX_LOOKAHEAD => {
                        return Err(format!(
                            "PeekNot cannot require > {MAX_LOOKAHEAD} bytes lookahead"
                        ))
                    }
                    _ => {}
                }
                let da = Box::new(Decoder::compile_next(compiler, a, Rc::new(Next::Empty))?);
                Ok(Decoder::PeekNot(da))
            }
            Format::Slice(expr, a) => {
                let da = Box::new(Decoder::compile_next(compiler, a, Rc::new(Next::Empty))?);
                Ok(Decoder::Slice(expr.clone(), da))
            }
            Format::Bits(a) => {
                let da = Box::new(Decoder::compile_next(compiler, a, Rc::new(Next::Empty))?);
                Ok(Decoder::Bits(da))
            }
            Format::WithRelativeOffset(expr, a) => {
                let da = Box::new(Decoder::compile_next(compiler, a, Rc::new(Next::Empty))?);
                Ok(Decoder::WithRelativeOffset(expr.clone(), da))
            }
            Format::Compute(expr) => Ok(Decoder::Compute(expr.clone())),
            Format::Match(head, branches) => {
                let branches = branches
                    .iter()
                    .map(|(pattern, f)| {
                        Ok((
                            pattern.clone(),
                            Decoder::compile_next(compiler, f, next.clone())?,
                        ))
                    })
                    .collect::<Result<_, String>>()?;
                Ok(Decoder::Match(head.clone(), branches))
            }
            Format::MatchVariant(head, branches) => {
                let branches = branches
                    .iter()
                    .map(|(pattern, label, f)| {
                        Ok((
                            pattern.clone(),
                            label.clone(),
                            Decoder::compile_next(compiler, f, next.clone())?,
                        ))
                    })
                    .collect::<Result<_, String>>()?;
                Ok(Decoder::MatchVariant(head.clone(), branches))
            }
            Format::Dynamic(d) => Ok(Decoder::Dynamic(d.clone())),
            Format::Apply(name) => Ok(Decoder::Apply(name.clone())),
        }
    }

    pub fn parse<'input>(
        &self,
        program: &Program,
        scope: &mut Scope,
        input: ReadCtxt<'input>,
    ) -> ParseResult<(Value, ReadCtxt<'input>)> {
        match self {
            Decoder::Call(n, es) => {
                let mut new_scope = Scope::new();
                for (name, e) in es {
                    let v = e.eval(scope);
                    new_scope.push(name.clone(), v);
                }
                program.decoders[*n].parse(program, &mut new_scope, input)
            }
            Decoder::Fail => Err(ParseError::fail(scope, input)),
            Decoder::EndOfInput => match input.read_byte() {
                None => Ok((Value::UNIT, input)),
                Some((b, _)) => Err(ParseError::trailing(b, input.offset)),
            },
            Decoder::Align(n) => {
                let skip = (n - (input.offset % n)) % n;
                let (_, input) = input
                    .split_at(skip)
                    .ok_or(ParseError::overrun(skip, input.offset))?;
                Ok((Value::UNIT, input))
            }
            Decoder::Byte(bs) => {
                let (b, input) = input
                    .read_byte()
                    .ok_or(ParseError::overbyte(input.offset))?;
                if bs.contains(b) {
                    Ok((Value::U8(b), input))
                } else {
                    Err(ParseError::unexpected(b, bs.clone(), input.offset))
                }
            }
            Decoder::Branch(tree, branches) => {
                let index = tree.matches(input).ok_or(ParseError::NoValidBranch {
                    offset: input.offset,
                })?;
                let (label, d) = &branches[index];
                let (v, input) = d.parse(program, scope, input)?;
                Ok((Value::Variant(label.clone(), Box::new(v)), input))
            }
            Decoder::Parallel(branches) => {
                for (label, d) in branches {
                    let initial_len = scope.len();
                    let res = d.parse(program, scope, input);
                    if let Ok((v, input)) = res {
                        return Ok((Value::Variant(label.clone(), Box::new(v)), input));
                    }
                    scope.truncate(initial_len);
                }
                Err(ParseError::fail(scope, input))
            }
            Decoder::Tuple(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for f in fields {
                    let (vf, next_input) = f.parse(program, scope, input)?;
                    input = next_input;
                    v.push(vf.clone());
                }
                Ok((Value::Tuple(v), input))
            }
            Decoder::Record(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for (name, f) in fields {
                    let (vf, next_input) = f.parse(program, scope, input)?;
                    input = next_input;
                    v.push((name.clone(), vf.clone()));
                    scope.push(name.clone(), vf);
                }
                for _ in fields {
                    scope.pop();
                }
                Ok((Value::Record(v), input))
            }
            Decoder::While(tree, a) => {
                let mut input = input;
                let mut v = Vec::new();
                while tree.matches(input).ok_or(ParseError::NoValidBranch {
                    offset: input.offset,
                })? == 0
                {
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                }
                Ok((Value::Seq(v), input))
            }
            Decoder::Until(tree, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                    if tree.matches(input).ok_or(ParseError::NoValidBranch {
                        offset: input.offset,
                    })? == 0
                    {
                        break;
                    }
                }
                Ok((Value::Seq(v), input))
            }
            Decoder::RepeatCount(expr, a) => {
                let mut input = input;
                let count = expr.eval_value(scope).unwrap_usize();
                let mut v = Vec::with_capacity(count);
                for _ in 0..count {
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                }
                Ok((Value::Seq(v), input))
            }
            Decoder::RepeatUntilLast(expr, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    let done = expr.eval_lambda(scope, va.clone()).unwrap_bool();
                    v.push(va);
                    if done {
                        break;
                    }
                }
                Ok((Value::Seq(v), input))
            }
            Decoder::RepeatUntilSeq(expr, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                    let vs = Value::Seq(v.clone());
                    let done = expr.eval_lambda(scope, vs).unwrap_bool();
                    if done {
                        break;
                    }
                }
                Ok((Value::Seq(v), input))
            }
            Decoder::Peek(a) => {
                let (v, _next_input) = a.parse(program, scope, input)?;
                Ok((v, input))
            }
            Decoder::PeekNot(a) => {
                if a.parse(program, scope, input).is_ok() {
                    Err(ParseError::fail(scope, input))
                } else {
                    Ok((Value::Tuple(vec![]), input))
                }
            }
            Decoder::Slice(expr, a) => {
                let size = expr.eval_value(scope).unwrap_usize();
                let (slice, input) = input
                    .split_at(size)
                    .ok_or(ParseError::overrun(size, input.offset))?;
                let (v, _) = a.parse(program, scope, slice)?;
                Ok((v, input))
            }
            Decoder::Bits(a) => {
                let mut bits = Vec::with_capacity(input.remaining().len() * 8);
                for b in input.remaining() {
                    for i in 0..8 {
                        bits.push((b & (1 << i)) >> i);
                    }
                }
                let (v, bits) = a.parse(program, scope, ReadCtxt::new(&bits))?;
                let bytes_remain = bits.remaining().len() >> 3;
                let bytes_read = input.remaining().len() - bytes_remain;
                let (_, input) = input
                    .split_at(bytes_read)
                    .ok_or(ParseError::overrun(bytes_read, input.offset))?;
                Ok((v, input))
            }
            Decoder::WithRelativeOffset(expr, a) => {
                let offset = expr.eval_value(scope).unwrap_usize();
                let (_, slice) = input
                    .split_at(offset)
                    .ok_or(ParseError::overrun(offset, input.offset))?;
                let (v, _) = a.parse(program, scope, slice)?;
                Ok((v, input))
            }
            Decoder::Compute(expr) => {
                let v = expr.eval(scope);
                Ok((v, input))
            }
            Decoder::Match(head, branches) => {
                let head = head.eval(scope);
                let initial_len = scope.len();
                let (_, decoder) = branches
                    .iter()
                    .find(|(pattern, _)| head.matches(scope, pattern))
                    .expect("exhaustive patterns");
                let (v, input) = decoder.parse(program, scope, input)?;
                scope.truncate(initial_len);
                Ok((v, input))
            }
            Decoder::MatchVariant(head, branches) => {
                let head = head.eval(scope);
                let initial_len = scope.len();
                let (_, label, decoder) = branches
                    .iter()
                    .find(|(pattern, _, _)| head.matches(scope, pattern))
                    .expect("exhaustive patterns");
                let (v, input) = decoder.parse(program, scope, input)?;
                scope.truncate(initial_len);
                Ok((Value::Variant(label.clone(), Box::new(v)), input))
            }
            Decoder::Dynamic(DynFormat::Huffman(lengths_expr, opt_values_expr)) => {
                let lengths_val = lengths_expr.eval(scope);
                let lengths = value_to_vec_usize(&lengths_val);
                let lengths = match opt_values_expr {
                    None => lengths,
                    Some(e) => {
                        let values = value_to_vec_usize(&e.eval(scope));
                        let mut new_lengths = [0].repeat(values.len());
                        for i in 0..lengths.len() {
                            new_lengths[values[i]] = lengths[i];
                        }
                        new_lengths
                    }
                };
                let f = make_huffman_codes(&lengths);
                Ok((Value::Format(Box::new(f)), input))
            }
            Decoder::Apply(name) => scope.call_decoder_by_name(name, program, input),
        }
    }
}

fn value_to_vec_usize(v: &Value) -> Vec<usize> {
    let vs = match v {
        Value::Seq(vs) => vs,
        _ => panic!("expected Seq"),
    };
    vs.iter()
        .map(|v| match v.coerce_record_to_value() {
            Value::U8(n) => *n as usize,
            Value::U16(n) => *n as usize,
            _ => panic!("expected U8 or U16"),
        })
        .collect::<Vec<usize>>()
}

fn make_huffman_codes(lengths: &[usize]) -> Format {
    let max_length = *lengths.iter().max().unwrap();
    let mut bl_count = [0].repeat(max_length + 1);

    for len in lengths {
        bl_count[*len] += 1;
    }

    let mut next_code = [0].repeat(max_length + 1);
    let mut code = 0;
    bl_count[0] = 0;

    for bits in 1..max_length + 1 {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
    }

    let mut codes = Vec::with_capacity(lengths.len());
    let mut branches = Vec::new();

    for n in 0..lengths.len() {
        let len = lengths[n];
        if len != 0 {
            codes.push((n.to_string(), bit_range(len, next_code[len])));
            let pattern = Pattern::Variant(n.to_string(), Box::new(Pattern::Wildcard));
            let val = Expr::U16(n.try_into().unwrap());
            branches.push((pattern, val));
            //println!("{:?}", codes[codes.len()-1]);
            next_code[len] += 1;
        } else {
            //codes.push((n.to_string(), Format::Fail));
        }
    }

    Format::record([
        ("bits", Format::alts(codes)),
        (
            "@value",
            Format::Compute(Expr::Match(
                Box::new(Expr::Var("bits".to_string())),
                branches,
            )),
        ),
    ])
}

fn bit_range(n: usize, bits: usize) -> Format {
    let mut fs = Vec::with_capacity(n);
    for i in 0..n {
        let r = n - 1 - i;
        let b = (bits & (1 << r)) >> r != 0;
        fs.push(is_bit(b));
    }
    Format::Tuple(fs)
}

fn is_bit(b: bool) -> Format {
    Format::Byte(ByteSet::from([if b { 1 } else { 0 }]))
}

fn inflate(codes: &[Value]) -> Vec<Value> {
    let mut vs = Vec::new();
    for code in codes {
        match code {
            Value::Variant(name, v) => match (name.as_str(), v.as_ref()) {
                ("literal", v) => match v.coerce_record_to_value() {
                    Value::U8(b) => vs.push(Value::U8(*b)),
                    _ => panic!("inflate: expected U8"),
                },
                ("reference", Value::Record(fields)) => {
                    let length = &fields
                        .iter()
                        .find(|(label, _)| label == "length")
                        .unwrap()
                        .1;
                    let distance = &fields
                        .iter()
                        .find(|(label, _)| label == "distance")
                        .unwrap()
                        .1;
                    match (length, distance) {
                        (Value::U16(length), Value::U16(distance)) => {
                            let length = *length as usize;
                            let distance = *distance as usize;
                            if distance > vs.len() {
                                panic!("inflate: distance out of range");
                            }
                            let start = vs.len() - distance;
                            for i in 0..length {
                                vs.push(vs[start + i].clone());
                            }
                        }
                        _ => panic!(
                            "inflate: unexpected length/distance {:?} {:?}",
                            length, distance
                        ),
                    }
                }
                _ => panic!("inflate: unknown code"),
            },
            _ => panic!("inflate: expected variant"),
        }
    }
    vs
}

#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests {
    use super::*;

    fn alts<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
        Format::Union(
            (fields.into_iter())
                .map(|(label, format)| (label.into(), format))
                .collect(),
        )
    }

    fn record<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
        Format::Record(
            (fields.into_iter())
                .map(|(label, format)| (label.into(), format))
                .collect(),
        )
    }

    fn optional(format: Format) -> Format {
        alts([("some", format), ("none", Format::EMPTY)])
    }

    fn repeat(format: Format) -> Format {
        Format::Repeat(Box::new(format))
    }

    fn repeat1(format: Format) -> Format {
        Format::Repeat1(Box::new(format))
    }

    fn is_byte(b: u8) -> Format {
        Format::Byte(ByteSet::from([b]))
    }

    fn not_byte(b: u8) -> Format {
        Format::Byte(!ByteSet::from([b]))
    }

    fn accepts(d: &Decoder, input: &[u8], tail: &[u8], expect: Value) {
        let program = Program::new();
        let mut scope = Scope::new();
        let (val, remain) = d.parse(&program, &mut scope, ReadCtxt::new(input)).unwrap();
        assert_eq!(val, expect);
        assert_eq!(remain.remaining(), tail);
    }

    fn rejects(d: &Decoder, input: &[u8]) {
        let program = Program::new();
        let mut scope = Scope::new();
        assert!(d.parse(&program, &mut scope, ReadCtxt::new(input)).is_err());
    }

    #[test]
    fn compile_fail() {
        let f = Format::Fail;
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_empty() {
        let f = Format::EMPTY;
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::UNIT);
        accepts(&d, &[0x00], &[0x00], Value::UNIT);
    }

    #[test]
    fn compile_byte_is() {
        let f = is_byte(0x00);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[0x00], &[], Value::U8(0));
        accepts(&d, &[0x00, 0xFF], &[0xFF], Value::U8(0));
        rejects(&d, &[0xFF]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_byte_not() {
        let f = not_byte(0x00);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[0xFF], &[], Value::U8(0xFF));
        accepts(&d, &[0xFF, 0x00], &[0x00], Value::U8(0xFF));
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt() {
        let f = alts::<&str>([]);
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_alt_byte() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0xFF))]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[0x00], &[], Value::variant("a", Value::U8(0x00)));
        accepts(&d, &[0xFF], &[], Value::variant("b", Value::U8(0xFF)));
        rejects(&d, &[0x11]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_ambiguous() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0x00))]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_slice_byte() {
        let slice_a = Format::Slice(Expr::U8(1), Box::new(is_byte(0x00)));
        let slice_b = Format::Slice(Expr::U8(1), Box::new(is_byte(0xFF)));
        let f = alts([("a", slice_a), ("b", slice_b)]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[0x00], &[], Value::variant("a", Value::U8(0x00)));
        accepts(&d, &[0xFF], &[], Value::variant("b", Value::U8(0xFF)));
        rejects(&d, &[0x11]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_slice_ambiguous1() {
        let slice_a = Format::Slice(Expr::U8(1), Box::new(is_byte(0x00)));
        let slice_b = Format::Slice(Expr::U8(1), Box::new(is_byte(0x00)));
        let f = alts([("a", slice_a), ("b", slice_b)]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_slice_ambiguous2() {
        let tuple_a = Format::Tuple(vec![is_byte(0x00), is_byte(0x00)]);
        let tuple_b = Format::Tuple(vec![is_byte(0x00), is_byte(0xFF)]);
        let slice_a = Format::Slice(Expr::U8(1), Box::new(tuple_a));
        let slice_b = Format::Slice(Expr::U8(1), Box::new(tuple_b));
        let f = alts([("a", slice_a), ("b", slice_b)]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_fail() {
        let f = alts([("a", Format::Fail), ("b", Format::Fail)]);
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_end_of_input() {
        let f = alts([("a", Format::EndOfInput), ("b", Format::EndOfInput)]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_empty() {
        let f = alts([("a", Format::EMPTY), ("b", Format::EMPTY)]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_fail_end_of_input() {
        let f = alts([("a", Format::Fail), ("b", Format::EndOfInput)]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::variant("b", Value::UNIT));
    }

    #[test]
    fn compile_alt_end_of_input_or_byte() {
        let f = alts([("a", Format::EndOfInput), ("b", is_byte(0x00))]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::variant("a", Value::UNIT));
        accepts(&d, &[0x00], &[], Value::variant("b", Value::U8(0x00)));
        accepts(
            &d,
            &[0x00, 0x00],
            &[0x00],
            Value::variant("b", Value::U8(0x00)),
        );
        rejects(&d, &[0x11]);
    }

    #[test]
    fn compile_alt_opt() {
        let f = alts([("a", Format::EMPTY), ("b", is_byte(0x00))]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[0x00], &[], Value::variant("b", Value::U8(0x00)));
        accepts(&d, &[], &[], Value::variant("a", Value::UNIT));
        accepts(&d, &[0xFF], &[0xFF], Value::variant("a", Value::UNIT));
    }

    #[test]
    fn compile_alt_opt_next() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), is_byte(0xFF)]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::variant("some", Value::U8(0)), Value::U8(0xFF)]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![Value::variant("none", Value::UNIT), Value::U8(0xFF)]),
        );
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_opt_opt() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), optional(is_byte(0xFF))]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::variant("some", Value::U8(0)),
                Value::variant("some", Value::U8(0xFF)),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![
                Value::variant("some", Value::U8(0)),
                Value::variant("none", Value::UNIT),
            ]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![
                Value::variant("none", Value::UNIT),
                Value::variant("some", Value::U8(0xFF)),
            ]),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![
                Value::variant("none", Value::UNIT),
                Value::variant("none", Value::UNIT),
            ]),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![
                Value::variant("none", Value::UNIT),
                Value::variant("none", Value::UNIT),
            ]),
        );
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Tuple(vec![
                Value::variant("none", Value::UNIT),
                Value::variant("none", Value::UNIT),
            ]),
        );
    }

    #[test]
    fn compile_alt_opt_ambiguous() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), optional(is_byte(0x00))]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_opt_ambiguous_slow() {
        let alt = alts([
            ("0x00", is_byte(0x00)),
            ("0x01", is_byte(0x01)),
            ("0x02", is_byte(0x02)),
            ("0x03", is_byte(0x03)),
            ("0x04", is_byte(0x04)),
            ("0x05", is_byte(0x05)),
            ("0x06", is_byte(0x06)),
            ("0x07", is_byte(0x07)),
        ]);
        let rec = record([
            ("0", alt.clone()),
            ("1", alt.clone()),
            ("2", alt.clone()),
            ("3", alt.clone()),
            ("4", alt.clone()),
            ("5", alt.clone()),
            ("6", alt.clone()),
            ("7", alt.clone()),
        ]);
        let f = alts([("a", rec.clone()), ("b", rec.clone())]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_alt_repeat1_slow() {
        let f = repeat(alts([
            ("a", repeat1(is_byte(0x00))),
            ("b", is_byte(0x01)),
            ("c", is_byte(0x02)),
        ]));
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat() {
        let f = repeat(is_byte(0x00));
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::Seq(vec![]));
        accepts(&d, &[0xFF], &[0xFF], Value::Seq(vec![]));
        accepts(&d, &[0x00], &[], Value::Seq(vec![Value::U8(0x00)]));
        accepts(
            &d,
            &[0x00, 0x00],
            &[],
            Value::Seq(vec![Value::U8(0x00), Value::U8(0x00)]),
        );
    }

    #[test]
    fn compile_repeat_repeat() {
        let f = repeat(repeat(is_byte(0x00)));
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_cat_repeat() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), repeat(is_byte(0xFF))]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![Value::Seq(vec![]), Value::Seq(vec![])]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![Value::Seq(vec![Value::U8(0x00)]), Value::Seq(vec![])]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![Value::Seq(vec![]), Value::Seq(vec![Value::U8(0xFF)])]),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00)]),
                Value::Seq(vec![Value::U8(0xFF)]),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF, 0x00],
            &[0x00],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00)]),
                Value::Seq(vec![Value::U8(0xFF)]),
            ]),
        );
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Tuple(vec![Value::Seq(vec![]), Value::Seq(vec![])]),
        );
    }

    #[test]
    fn compile_cat_end_of_input() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::EndOfInput]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT]),
        );
        rejects(&d, &[]);
        rejects(&d, &[0x00, 0x00]);
    }

    #[test]
    fn compile_cat_repeat_end_of_input() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), Format::EndOfInput]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![Value::Seq(vec![]), Value::UNIT]),
        );
        accepts(
            &d,
            &[0x00, 0x00, 0x00],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00), Value::U8(0x00), Value::U8(0x00)]),
                Value::UNIT,
            ]),
        );
        rejects(&d, &[0x00, 0x10]);
    }

    #[test]
    fn compile_cat_repeat_ambiguous() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), repeat(is_byte(0x00))]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_fields() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x7F))),
        ]);
        assert!(Decoder::compile_one(&f).is_ok());
    }

    #[test]
    fn compile_repeat_fields_ambiguous() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x00))),
        ]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_fields_okay() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            (
                "second-and-third",
                optional(record([
                    (
                        "second",
                        Format::Tuple(vec![is_byte(0xFF), repeat(is_byte(0xFF))]),
                    ),
                    ("third", repeat(is_byte(0x00))),
                ])),
            ),
        ]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::record([
                ("first", Value::Seq(vec![])),
                ("second-and-third", Value::variant("none", Value::UNIT)),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::record([
                ("first", Value::Seq(vec![Value::U8(0x00)])),
                ("second-and-third", Value::variant("none", Value::UNIT)),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::record([
                ("first", Value::Seq(vec![Value::U8(0x00)])),
                (
                    "second-and-third",
                    Value::variant(
                        "some",
                        Value::record([
                            (
                                "second",
                                Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![])]),
                            ),
                            ("third", Value::Seq(vec![])),
                        ]),
                    ),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF, 0x00],
            &[],
            Value::record(vec![
                ("first", Value::Seq(vec![Value::U8(0x00)])),
                (
                    "second-and-third",
                    Value::variant(
                        "some",
                        Value::record(vec![
                            (
                                "second",
                                Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![])]),
                            ),
                            ("third", Value::Seq(vec![Value::U8(0x00)])),
                        ]),
                    ),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0x7F],
            &[0x7F],
            Value::record(vec![
                ("first", Value::Seq(vec![Value::U8(0x00)])),
                ("second-and-third", Value::variant("none", Value::UNIT)),
            ]),
        );
    }

    #[test]
    fn compile_repeat1() {
        let f = repeat1(is_byte(0x00));
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0xFF]);
        accepts(&d, &[0x00], &[], Value::Seq(vec![Value::U8(0x00)]));
        accepts(
            &d,
            &[0x00, 0xFF],
            &[0xFF],
            Value::Seq(vec![Value::U8(0x00)]),
        );
        accepts(
            &d,
            &[0x00, 0x00],
            &[],
            Value::Seq(vec![Value::U8(0x00), Value::U8(0x00)]),
        );
    }

    #[test]
    fn compile_align1() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::Align(1), is_byte(0xFF)]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT, Value::U8(0xFF)]),
        );
    }

    #[test]
    fn compile_align2() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::Align(2), is_byte(0xFF)]);
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[0x00, 0xFF]);
        rejects(&d, &[0x00, 0x99, 0x99, 0xFF]);
        accepts(
            &d,
            &[0x00, 0x99, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT, Value::U8(0xFF)]),
        );
    }

    #[test]
    fn compile_peek_not() {
        let any_byte = Format::Byte(ByteSet::full());
        let a = Format::Tuple(vec![is_byte(0xFF), is_byte(0xFF)]);
        let peek_not = Format::PeekNot(Box::new(a));
        let f = Format::Tuple(vec![peek_not, any_byte.clone(), any_byte.clone()]);
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0xFF]);
        rejects(&d, &[0xFF, 0xFF]);
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::Tuple(vec![]), Value::U8(0x00), Value::U8(0xFF)]),
        );
        accepts(
            &d,
            &[0xFF, 0x00],
            &[],
            Value::Tuple(vec![Value::Tuple(vec![]), Value::U8(0xFF), Value::U8(0x00)]),
        );
    }

    #[test]
    fn compile_peek_not_switch() {
        let any_byte = Format::Byte(ByteSet::full());
        let guard = Format::PeekNot(Box::new(Format::Tuple(vec![is_byte(0xFF), is_byte(0xFF)])));
        let a = Format::Tuple(vec![guard, Format::Repeat(Box::new(any_byte.clone()))]);
        let b = Format::Tuple(vec![is_byte(0xFF), is_byte(0xFF)]);
        let f = alts([("a", a), ("b", b)]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Variant(
                "a".to_string(),
                Box::new(Value::Tuple(vec![Value::Tuple(vec![]), Value::Seq(vec![])])),
            ),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Variant(
                "a".to_string(),
                Box::new(Value::Tuple(vec![
                    Value::Tuple(vec![]),
                    Value::Seq(vec![Value::U8(0xFF)]),
                ])),
            ),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Variant(
                "a".to_string(),
                Box::new(Value::Tuple(vec![
                    Value::Tuple(vec![]),
                    Value::Seq(vec![Value::U8(0x00), Value::U8(0xFF)]),
                ])),
            ),
        );
        accepts(
            &d,
            &[0xFF, 0x00],
            &[],
            Value::Variant(
                "a".to_string(),
                Box::new(Value::Tuple(vec![
                    Value::Tuple(vec![]),
                    Value::Seq(vec![Value::U8(0xFF), Value::U8(0x00)]),
                ])),
            ),
        );
        accepts(
            &d,
            &[0xFF, 0xFF],
            &[],
            Value::Variant(
                "b".to_string(),
                Box::new(Value::Tuple(vec![Value::U8(0xFF), Value::U8(0xFF)])),
            ),
        );
    }

    #[test]
    fn compile_peek_not_lookahead() {
        let peek_not = Format::PeekNot(Box::new(repeat1(is_byte(0x00))));
        let any_byte = Format::Byte(ByteSet::full());
        let f = Format::Tuple(vec![peek_not, repeat1(any_byte)]);
        assert!(Decoder::compile_one(&f).is_err());
    }
}
