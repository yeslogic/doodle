use std::ops::Add as _;
use std::rc::Rc;

use serde::Serialize;

use crate::{
    Bounds, DynFormat, Expr, IntoLabel, Label, MatchTree, Next, StyleHint, TypeHint, ViewExpr,
    ViewFormat,
};

use crate::FormatModule;
use crate::byte_set::ByteSet;
use crate::pattern::Pattern;
use crate::record_fmt::RecordFormat;
use crate::validation::Condition;

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
    ItemVar(usize, Vec<Expr>, Vec<ViewExpr>), // FIXME - do the exprs here need type(+) info?
    /// A format that never matches
    Fail,
    /// Matches if the end of the input has been reached
    EndOfInput,
    /// Skips bytes if necessary to align the current offset to a multiple of N
    Align(usize),
    /// Matches a byte in the given byte set
    Byte(ByteSet),
    /// Wraps the value from the inner format in a variant
    Variant(Label, Box<Format>),
    /// Matches the union of all the formats, which must have the same type
    Union(Vec<Format>),
    /// Nondeterministic unions, where the formats are not mutually exclusive
    UnionNondet(Vec<Format>),
    /// Matches a sequence of concatenated formats
    Tuple(Vec<Format>),
    /// Matches a fixed-length sequence of homogeneously-typed formats
    Sequence(Vec<Format>),
    /// Repeat a format zero-or-more times
    Repeat(Box<Format>),
    /// Repeat a format one-or-more times
    Repeat1(Box<Format>),
    /// Repeat a format an exact number of times
    RepeatCount(Box<Expr>, Box<Format>),
    /// Repeat a format at least N and at most M times
    RepeatBetween(Box<Expr>, Box<Expr>, Box<Format>),
    /// Repeat a format until a condition is satisfied by its last item (condition is only checked post-append)
    RepeatUntilLast(Box<Expr>, Box<Format>),
    /// Repeat a format until a condition is satisfied by the sequence (condition is only checked post-append)
    RepeatUntilSeq(Box<Expr>, Box<Format>),
    /// Repeat a format until a condition is satisfied by a tuple constructed from a left-fold accumulator and the sequence, returning both
    /// AccumUntil :: ((A, [T]) -> bool) -> ((A, T) -> A) -> A -> Vt(A) -> T -> (A, [T])
    AccumUntil(Box<Expr>, Box<Expr>, Box<Expr>, TypeHint, Box<Format>),
    /// Apply a parametric format for each element of a sequence-typed Expr using a fused lambda binding
    ForEach(Box<Expr>, Label, Box<Format>),
    /// Parse a format if and only if the given expression evaluates to true, otherwise skip
    Maybe(Box<Expr>, Box<Format>),
    /// Parse a format without advancing the stream position afterwards
    Peek(Box<Format>),
    /// Attempt to parse a format and fail if it succeeds
    PeekNot(Box<Format>),
    /// Restrict a format to a sub-stream of a given number of bytes (skips any leftover bytes in the sub-stream)
    Slice(Box<Expr>, Box<Format>),
    /// Parse bitstream
    Bits(Box<Format>),
    /// Matches a format at a byte offset relative to the given base address
    WithRelativeOffset(Box<Expr>, Box<Expr>, Box<Format>),
    /// Map a value with a lambda expression
    Map(Box<Format>, Box<Expr>),
    /// Assert that a boolean condition (as a lambda) holds on a value
    Where(Box<Format>, Condition),
    /// Compute a value
    Compute(Box<Expr>),
    /// Let binding
    Let(Label, Box<Expr>, Box<Format>),
    /// Pattern match on an expression
    Match(Box<Expr>, Vec<(Pattern, Format)>),
    /// Format generated dynamically
    Dynamic(Label, DynFormat, Box<Format>),
    /// Apply a dynamic format from a named variable in the scope
    Apply(Label),
    /// Current byte-offset relative to start-of-buffer (as a U64(?))
    Pos,
    /// Skip the remainder of the stream, up until the end of input or the last available byte within a Slice
    SkipRemainder,
    /// Given an expression corresponding to a byte-sequence, decode it again using the provided Format. This can be used to reparse the initial decode of formats that output Vec<u8> or similar
    DecodeBytes(Box<Expr>, Box<Format>),
    /// Process one format, bind the result to a label, and process a second format, discarding the result of the first
    LetFormat(Box<Format>, Label, Box<Format>),
    /// Process one format without capturing the resultant value, and then process the a second format as normal
    MonadSeq(Box<Format>, Box<Format>),
    /// Encapsulation of a Format with a structurally significant artifact of what it represents or how it was constructed
    Hint(StyleHint, Box<Format>),
    /// Wrap the result of `format` in `Some` if `Some(format)`, yield `None` if `None`
    LiftedOption(Option<Box<Format>>),
    // SECTION - APM-backing formats
    /// Binds a View to the specfied label and processes a format in the ensuing context
    LetView(Label, Box<Format>),
    /// Using a ViewExpr, apply a View-based parse or capture
    WithView(ViewExpr, ViewFormat),
    /// Parses a given format within the context of a View
    ParseFromView(ViewExpr, Box<Format>),
    // !SECTION
    // Include a format in the tree as a placeholder, without ever processing it
    Phantom(Box<Format>),
}

impl Format {
    pub const EMPTY: Format = Format::Tuple(Vec::new());

    pub const ANY_BYTE: Format = Format::Byte(ByteSet::full());

    pub fn alts<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Format)>) -> Format {
        Format::Union(
            fields
                .into_iter()
                .map(|(label, format)| Format::Variant(label.into(), Box::new(format)))
                .collect(),
        )
    }

    pub(crate) fn to_record_format(&self) -> RecordFormat<'_> {
        RecordFormat::try_from(self).unwrap()
    }

    /// Old-style record where every field is preserved and the constructed record uses the same names as the individual parse-bindings.
    pub fn record<Name: IntoLabel, I>(fields: I) -> Format
    where
        I: IntoIterator<Item = (Name, Format), IntoIter: DoubleEndedIterator>,
    {
        // NOTE - reverse-order so `.pop()` removes the earliest remaining entry
        let mut rev_fields = fields
            .into_iter()
            .rev()
            .map(|(name, format)| (Some((name, true)), format))
            .collect::<Vec<(Option<(Name, bool)>, Format)>>();
        let accum = Vec::with_capacity(rev_fields.len());
        Format::Hint(
            StyleHint::Record { old_style: true },
            Box::new(Format::__chain_record(accum, &mut rev_fields)),
        )
    }

    pub(crate) fn __chain_record<Name: IntoLabel>(
        mut captured: Vec<(Label, Expr)>,
        remaining: &mut Vec<(Option<(Name, bool)>, Format)>,
    ) -> Format {
        if remaining.is_empty() {
            if captured.is_empty() {
                // NOTE - avoid constructing 'empty records' by returning a unit
                Format::Compute(Box::new(Expr::UNIT))
            } else {
                Format::Compute(Box::new(Expr::Record(captured)))
            }
        } else {
            let this = remaining.pop().unwrap();
            let (label, format) = this;
            match label {
                None => Format::MonadSeq(
                    Box::new(format),
                    Box::new(Format::__chain_record(captured, remaining)),
                ),
                Some((name, is_persist)) => {
                    let name: Label = name.into();
                    if is_persist {
                        captured.push((name.clone(), Expr::Var(name.clone())));
                    }
                    Format::LetFormat(
                        Box::new(format),
                        name,
                        Box::new(Format::__chain_record(captured, remaining)),
                    )
                }
            }
        }
    }

    pub fn chaining<Name: IntoLabel>(
        formats: impl IntoIterator<Item = (Option<Name>, Format), IntoIter: DoubleEndedIterator>,
        format: Format,
    ) -> Format {
        let mut remaining = formats.into_iter().rev().collect::<Vec<_>>();
        Format::__chain_format(&mut remaining, format)
    }

    pub(crate) fn __chain_format<Name: IntoLabel>(
        remaining: &mut Vec<(Option<Name>, Format)>,
        ret: Format,
    ) -> Format {
        if remaining.is_empty() {
            ret
        } else {
            let this = remaining.pop().unwrap();
            let (label, format) = this;
            match label {
                None => Format::MonadSeq(
                    Box::new(format),
                    Box::new(Format::__chain_format(remaining, ret)),
                ),
                Some(name) => {
                    let name: Label = name.into();
                    Format::LetFormat(
                        Box::new(format),
                        name,
                        Box::new(Format::__chain_format(remaining, ret)),
                    )
                }
            }
        }
    }
}

impl Format {
    /// Conservative bounds for number of byte-positions advanced after a format is matched (i.e. parsed)
    pub(crate) fn match_bounds(&self, module: &FormatModule) -> Bounds {
        match self {
            Format::ItemVar(level, _args, _views) => module.get_format(*level).match_bounds(module),
            Format::Fail => Bounds::exact(0),
            Format::EndOfInput => Bounds::exact(0),
            Format::SkipRemainder => Bounds::any(),
            Format::Align(0) => unreachable!("illegal Format::Align modulus (== 0)"),
            Format::Align(n) => Bounds::new(0, n - 1),
            Format::Byte(_) => Bounds::exact(1),
            Format::Variant(_label, f) => f.match_bounds(module),
            Format::Union(branches) | Format::UnionNondet(branches) => branches
                .iter()
                .map(|f| f.match_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Tuple(fields) => fields
                .iter()
                .map(|f| f.match_bounds(module))
                .reduce(Bounds::add)
                .unwrap_or(Bounds::exact(0)),
            Format::Repeat(_) => Bounds::any(),
            Format::Repeat1(f) => f.match_bounds(module) * Bounds::at_least(1),
            Format::RepeatCount(expr, f) => f.match_bounds(module) * expr.bounds(),
            Format::RepeatBetween(xmin, xmax, f) => {
                f.match_bounds(module) * (Bounds::union(xmin.bounds(), xmax.bounds()))
            }
            Format::RepeatUntilLast(_, f) => f.match_bounds(module) * Bounds::at_least(1),
            Format::RepeatUntilSeq(_, _f) | Format::AccumUntil(.., _f) => Bounds::any(),
            Format::Maybe(_, f) => Bounds::union(Bounds::exact(0), f.match_bounds(module)),
            Format::Peek(_) => Bounds::exact(0),
            Format::PeekNot(_) => Bounds::exact(0),
            Format::Slice(expr, _) => expr.bounds(),
            Format::Bits(f) => f.match_bounds(module).bits_to_bytes(),
            Format::WithRelativeOffset(..) => Bounds::exact(0),
            Format::Map(f, _expr) => f.match_bounds(module),
            Format::Where(f, _expr) => f.match_bounds(module),
            Format::Compute(_) | Format::Pos => Bounds::exact(0),
            Format::Let(_name, _expr, f) => f.match_bounds(module),
            Format::LetView(_name, f) => f.match_bounds(module),
            Format::Match(_, branches) => branches
                .iter()
                .map(|(_, f)| f.match_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Dynamic(_name, _dynformat, f) => f.match_bounds(module),
            Format::Apply(_) => Bounds::at_least(1),
            // FIXME - do we have any way of approximating this better?
            Format::ForEach(_expr, _lbl, _f) => Bounds::any(),
            // NOTE - because we are parsing a sequence of bytes, we do not interact with the actual buffer
            Format::DecodeBytes(_bytes, _f) => Bounds::exact(0),
            Format::LetFormat(first, _, second) | Format::MonadSeq(first, second) => {
                first.match_bounds(module) + second.match_bounds(module)
            }
            Format::Hint(.., inner) => inner.match_bounds(module),
            Format::LiftedOption(opt) => match opt {
                None => Bounds::exact(0),
                Some(f) => f.match_bounds(module),
            },
            Format::Sequence(fmts) => {
                let mut total = Bounds::exact(0);
                for fmt in fmts.iter() {
                    let bounds = fmt.match_bounds(module);
                    total = total + bounds;
                }
                total
            }
            Format::WithView(_v_expr, vf) => match vf {
                ViewFormat::CaptureBytes(_len) => Bounds::exact(0),
                ViewFormat::ReadArray(_len, _kind) => Bounds::exact(0),
                ViewFormat::ReifyView => Bounds::exact(0),
            },
            Format::ParseFromView(_v_expr, _inner) => Bounds::exact(0),
            Format::Phantom(_) => Bounds::exact(0),
        }
    }

    /// Conservative bounds for number of bytes that may be read in order to fully parse the given Format, regardless of how many
    /// are consumed as opposed to being left untouched in the buffer.
    pub(crate) fn lookahead_bounds(&self, module: &FormatModule) -> Bounds {
        match self {
            Format::ItemVar(level, _args, _views) => {
                module.get_format(*level).lookahead_bounds(module)
            }
            Format::Fail => Bounds::exact(0),
            Format::EndOfInput => Bounds::exact(0),
            // NOTE - for PeekNot purposes it is not fully clear how to treat SkipRemainder, but we want to mirror the behavior of `Repeat(Byte)`
            Format::SkipRemainder => Bounds::any(),
            Format::Align(0) => unreachable!("illegal Format::Align modulus (== 0)"),
            Format::Align(n) => Bounds::new(0, n - 1),
            Format::Byte(_) => Bounds::exact(1),
            Format::Variant(_label, f) => f.lookahead_bounds(module),
            Format::Union(branches) | Format::UnionNondet(branches) => branches
                .iter()
                .map(|f| f.lookahead_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Tuple(fields) => fields
                .iter()
                .map(|f| f.lookahead_bounds(module))
                .reduce(Bounds::add)
                .unwrap_or(Bounds::exact(0)),
            Format::Repeat(_) => Bounds::any(),
            // FIXME - do we have any way of approximating this better?
            Format::ForEach(_expr, _lbl, _f) => Bounds::any(),
            Format::Repeat1(f) => f.lookahead_bounds(module) * Bounds::at_least(1),
            Format::RepeatCount(expr, f) => f.lookahead_bounds(module) * expr.bounds(),
            Format::RepeatBetween(xmin, xmax, f) => {
                f.lookahead_bounds(module) * Bounds::union(xmin.bounds(), xmax.bounds())
            }
            Format::RepeatUntilLast(_, f) => f.lookahead_bounds(module) * Bounds::at_least(1),
            Format::RepeatUntilSeq(_, _f) | Format::AccumUntil(.., _f) => Bounds::any(),
            Format::Maybe(_, f) => Bounds::union(Bounds::exact(0), f.lookahead_bounds(module)),
            Format::Peek(f) => f.lookahead_bounds(module),
            Format::PeekNot(f) => f.lookahead_bounds(module),
            Format::Slice(expr, _) => expr.bounds(),
            Format::Bits(f) => f.lookahead_bounds(module).bits_to_bytes(),
            // REVIEW - do we have a way of approximating this better?
            Format::WithRelativeOffset(..) => Bounds::any(),
            Format::Map(f, _expr) => f.lookahead_bounds(module),
            Format::Where(f, _expr) => f.lookahead_bounds(module),
            Format::Compute(_) | Format::Pos => Bounds::exact(0),
            Format::Let(_name, _expr, f) => f.lookahead_bounds(module),
            Format::LetView(_name, f) => f.lookahead_bounds(module),
            Format::Match(_, branches) => branches
                .iter()
                .map(|(_, f)| f.lookahead_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Dynamic(_name, _dynformat, f) => f.lookahead_bounds(module),
            Format::Apply(_) => Bounds::at_least(1),
            Format::DecodeBytes(_bytes, _f) => Bounds::exact(0),
            Format::MonadSeq(f0, f) | Format::LetFormat(f0, _, f) => Bounds::union(
                f0.lookahead_bounds(module),
                f0.match_bounds(module) + f.lookahead_bounds(module),
            ),
            Format::Hint(_, f) => f.lookahead_bounds(module),
            Format::LiftedOption(opt) => match opt {
                None => Bounds::exact(0),
                Some(f) => f.lookahead_bounds(module),
            },
            Format::Sequence(fmts) => {
                let mut sum_match = Bounds::exact(0);
                let mut max_lookahead = Bounds::exact(0);
                for fmt in fmts.iter() {
                    max_lookahead =
                        Bounds::union(max_lookahead, sum_match + fmt.lookahead_bounds(module));
                    sum_match = sum_match + fmt.match_bounds(module);
                }
                max_lookahead
            }
            Format::WithView(_v_expr, vf) => match vf {
                ViewFormat::CaptureBytes(_len) => Bounds::exact(0),
                ViewFormat::ReadArray(_len, _kind) => Bounds::exact(0),
                ViewFormat::ReifyView => Bounds::exact(0),
            },
            Format::ParseFromView(_v_expr, _f) => Bounds::exact(0),
            Format::Phantom(_) => Bounds::exact(0),
        }
    }

    /// Returns `true` if the format could match the empty byte string
    pub(crate) fn is_nullable(&self, module: &FormatModule) -> bool {
        self.match_bounds(module).min == 0
    }

    /// True if the compilation of this format depends on the format that follows it
    pub(crate) fn depends_on_next(&self, module: &FormatModule) -> bool {
        match self {
            Format::ItemVar(level, _args, _views) => {
                module.get_format(*level).depends_on_next(module)
            }
            Format::Fail => false,
            Format::EndOfInput => false,
            // NOTE - compiling SkipRemainder doesn't depend on the next format because the next format can only ever match the empty byte string at that point
            Format::SkipRemainder => false,
            Format::Align(..) => false,
            Format::Byte(..) => false,
            Format::WithView(..) => false,
            Format::Variant(_label, f) => f.depends_on_next(module),
            Format::Union(branches) | Format::UnionNondet(branches) => {
                Format::union_depends_on_next(branches, module)
            }
            Format::Tuple(fmts) | Format::Sequence(fmts) => {
                fmts.iter().any(|f| f.depends_on_next(module))
            }
            Format::Repeat(..) | Format::Repeat1(..) | Format::RepeatBetween(..) => true,
            Format::RepeatCount(..) | Format::RepeatUntilLast(..) | Format::RepeatUntilSeq(..) => {
                false
            }
            Format::AccumUntil(..) => false,
            Format::Maybe(..) => true,
            Format::Peek(..) | Format::PeekNot(..) => false,
            Format::Slice(..) => false,
            Format::Bits(..) => false,
            Format::WithRelativeOffset(..) => false,
            Format::Map(f, _expr) => f.depends_on_next(module),
            Format::Where(f, _expr) => f.depends_on_next(module),
            Format::Compute(..) | Format::Pos => false,
            Format::Let(_name, _expr, f) => f.depends_on_next(module),
            Format::LetView(_name, f) => f.depends_on_next(module),
            Format::Match(_, branches) => branches.iter().any(|(_, f)| f.depends_on_next(module)),
            Format::Dynamic(_name, _dynformat, f) => f.depends_on_next(module),
            Format::Apply(..) => false,
            Format::ForEach(_expr, _lbl, f) => f.depends_on_next(module),
            Format::DecodeBytes(..) | Format::ParseFromView(..) => false,
            Format::MonadSeq(first, second) | Format::LetFormat(first, _, second) => {
                first.depends_on_next(module) || second.depends_on_next(module)
            }
            Format::Hint(_, f) => f.depends_on_next(module),
            Format::LiftedOption(opt) => opt.as_ref().is_some_and(|f| f.depends_on_next(module)),
            Format::Phantom(_) => false,
        }
    }

    pub(crate) fn union_depends_on_next(branches: &[Format], module: &FormatModule) -> bool {
        let mut fs = Vec::with_capacity(branches.len());
        for f in branches {
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
            Format::Hint(StyleHint::AsciiChar, _) => true,
            Format::ItemVar(level, ..) => module.get_format(*level).is_ascii_char_format(module),
            Format::Let(.., f) | Format::LetFormat(.., f) | Format::MonadSeq(_, f) => {
                f.is_ascii_char_format(module)
            }
            // FIXME - there may be other recursive cases to consider
            _ => false,
        }
    }

    /// Returns `true` if values associated to this format should be handled as multi-character ASCII strings
    pub fn is_ascii_string_format(&self, module: &FormatModule) -> bool {
        match self {
            Format::Hint(StyleHint::AsciiStr, _) => true,
            Format::ItemVar(level, ..) => module.get_format(*level).is_ascii_string_format(module),
            Format::Tuple(formats) | Format::Sequence(formats) => {
                !formats.is_empty() && formats.iter().all(|f| f.is_ascii_char_format(module))
            }
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::RepeatCount(_, format)
            | Format::RepeatBetween(_, _, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => format.is_ascii_char_format(module),
            Format::Let(.., f) | Format::LetFormat(.., f) | Format::MonadSeq(_, f) => {
                f.is_ascii_string_format(module)
            }
            Format::Slice(_, format) => format.is_ascii_string_format(module),
            // FIXME - there may be other cases we should consider ASCII
            _ => false,
        }
    }

    pub fn is_record_format(&self) -> bool {
        // we take it on faith that a format is a record iff it is hinted as such
        matches!(self, Format::Hint(StyleHint::Record { .. }, _))
    }
}
