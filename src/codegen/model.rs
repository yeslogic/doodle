use crate::Label;

use super::rust_ast::*;
// use super::{GenBlock, GenExpr, GenStmt, ProdCtxt};
#[allow(unused_imports)]
use crate::Format as _;

macro_rules! call {
    ( $parser:expr, $method:ident ) => {
        RustExpr::MethodCall(
            Box::new($parser),
            mk_method(stringify!($method)),
            Vec::new(),
        )
    };
    ( $parser:expr, $method:ident, $( $arg:tt ),+ ) => {
        RustExpr::MethodCall(
            Box::new($parser),
            mk_method(stringify!($method)),
            vec![$($arg),+]
        )
    };
}

macro_rules! try_call {
    ( $x:expr, $y:ident $(, $z:expr )* $(,)? ) => {
        RustExpr::Try(Box::new(call!( $x, $y $(, $z)* )))
    };
}

pub const fn lbl(x: &'static str) -> Label {
    Label::Borrowed(x)
}

// SECTION - magic strings for fixed identifiers as codegen artifacts

/// General-purpose magic identifier for embedded-MatchTree branch-id bindings.
pub const MATCH_BRANCH_IX: &'static str = "matching_ix";

pub const SLICE_LEN: &'static str = "sz";
pub const SLICE_RET: &'static str = "ret";
pub const PEEK_RET: &'static str = "ret";
pub const OFFS_PEEK_TARGET: &'static str = "tgt_offset";
pub const OFFS_PEEK_DBG_ADV: &'static str = "_is_advance";
pub const OFFS_PEEK_RET: &'static str = "ret";
pub const PEEK_NOT_RES: &'static str = "res";
pub const BITS_RET: &'static str = "ret";

/// Magic identifier used to record the number of bits read (upon escaping bits-mode), for debugging purposes
pub const BITS_NREAD: &'static str = "_bits_read";

pub const R0COM_ACCUM: &'static str = "accum";
pub const R0COM_ELEM: &'static str = "next_elem";

pub const R1BOM_ACCUM: &'static str = "accum";
pub const R1BOM_ELEM: &'static str = "next_elem";

pub const BETWEEN_ACCUM: &'static str = "accum";
pub const BETWEEN_NEXT: &'static str = "next_elem";

/// Special variation of `MATCH_BRANCH_IX` for `RepeatLogic::BetweenCounts` due to the unique semantics of the branch-index
pub const BETWEEN_REPS_LEFT: &'static str = "reps_left";

pub const UNFOLD_DONE: &'static str = "is_done";
pub const UNFOLD_ELEM: &'static str = "elem";

// !SECTION

/// Helper function for promoting string-constants to method specifiers
const fn mk_method(ident: &'static str) -> MethodSpecifier {
    MethodSpecifier::Arbitrary(SubIdent::ByName(Label::Borrowed(ident)))
}

// SECTION - helper functions for constructing `Err(ParseError::..)`-values corresponding to various kinds of failure-state
/// Model RustExpr for handling [`Format::Fail`], as an `Err(<..>)`-value with accompanying trace-hash.
pub fn err_fail(trace: u64) -> RustExpr {
    RustExpr::ResultErr(Box::new(RustExpr::FunctionCall(
        Box::new(RustExpr::Entity(RustEntity::Scoped(
            vec![lbl("ParseError")],
            lbl("FailToken"),
        ))),
        vec![RustExpr::u64lit(trace)],
    )))
}

/// Model RustExpr for the failure-branch of [`Format::PeekNot`] (i.e. when the excluded parse is observed).
pub fn err_bad_peek_not() -> RustExpr {
    RustExpr::ResultErr(Box::new(RustExpr::Entity(RustEntity::Scoped(
        vec![lbl("ParseError")],
        lbl("NegatedSuccess"),
    ))))
}

/// model RustExpr for the failure-branch of [`Format::Repeat1`] and [`Format::RepeatBetween`] when the promised minimum number
/// of repeats cannot be fulfilled.
pub fn err_too_few() -> RustExpr {
    RustExpr::ResultErr(Box::new(RustExpr::Entity(RustEntity::Scoped(
        vec![lbl("ParseError")],
        lbl("InsufficientRepeats"),
    ))))
}
// !SECTION

// SECTION - Helper functions for prelude-specific AST-constructions
/// Model RustExpr for handling [`Format::EndOfInput`] as a Try-pattern (`<..>?`).
pub fn try_enforce_eos(parser: RustExpr) -> RustExpr {
    try_call!(parser, finish)
}

/// Model RustExpr for handling `Format::SkipRemainder` in the Parser model.
pub fn skip_remainder(parser: RustExpr) -> RustExpr {
    call!(parser, skip_remainder)
}

/// Model RustExpr for handling `Format::Align(n)` in the Parser model (Try-call).
pub fn try_skip_align(parser: RustExpr, n: usize) -> RustExpr {
    try_call!(parser, skip_align, RustExpr::num_lit(n))
}

/// Model RustExpr for handling `Format::Pos` in the Parser model.
pub fn yield_offset_as_u64(parser: RustExpr) -> RustExpr {
    call!(parser, get_offset_u64)
}

/// Model RustExpr for handling `Format::LetView` in the Parser model.
pub fn get_view(parser: RustExpr) -> RustExpr {
    call!(parser, view)
}

/// Model RustExpr for handling `ViewFormat::CaptureBytes(len)` in the Parser (View) model.
pub fn read_from_view(view: RustExpr, len: RustExpr) -> RustExpr {
    call!(view, read_len, len)
}

/// Model RustExpr for setup of `Format::Slice` parse-context in the Parser model.
pub fn try_open_slice(parser: RustExpr, sz: RustExpr) -> RustExpr {
    try_call!(parser, start_slice, sz)
}

/// Model RustExpr for post-parse teardown of `Format::Slice` parse-context in the Parser model.
pub fn try_close_slice(parser: RustExpr) -> RustExpr {
    try_call!(parser, end_slice)
}

/// Model RustExpr for setup of `Format::Peek` parse-context in the Parser model.
pub fn open_peek(parser: RustExpr) -> RustExpr {
    call!(parser, open_peek_context)
}

/// Model RustExpr for post-parse teardown of `Format::Peek` in the Parser model.
pub fn try_close_peek(parser: RustExpr) -> RustExpr {
    try_call!(parser, close_peek_context)
}

/// Model RustExpr or setup of `Format::PeekNot` parse-context in the Parser model.
pub fn open_peek_not(parser: RustExpr) -> RustExpr {
    call!(parser, open_peek_not_context)
}

/// Model RustExpr for post-parse teardown of `Format::PeekNot` in the Parser model (if the anti-pattern is not seen).
pub fn try_close_peek_not(parser: RustExpr) -> RustExpr {
    try_call!(parser, close_peek_not_context)
}

/// Model RustExpr for handling `Format::WithRelativeOffset` in the Parser model (seeking to absolute-offset `target`).
pub fn try_seek_to_target(parser: RustExpr, target: RustExpr) -> RustExpr {
    try_call!(parser, advance_or_seek, target)
}

/// Model RustExpr for setup of `Format::Bits` parse-context in the Parser model.
pub fn ent_bits(parser: RustExpr) -> RustExpr {
    try_call!(parser, enter_bits_mode)
}

/// Model RustExpr for post-parse teardown of `Format::Bits` in the Parser model.
pub fn esc_bits(parser: RustExpr) -> RustExpr {
    try_call!(parser, escape_bits_mode)
}

pub fn rem_bytes(parser: RustExpr) -> RustExpr {
    call!(parser, remaining)
}

// !SECTION

// SECTION - corollary functions for prelude-defined helpers

/// Corollary for the helper-function [`crate::prelude::repeat_between_finished`], as a RustExpr.
///
/// # Note
///
/// The helper is only necessary due to the absence of boolean operators in the `rust_ast` model.
pub fn repeat_between_finished(
    out_of_reps: RustExpr,
    len: RustExpr,
    min: RustExpr,
    max: RustExpr,
) -> RustExpr {
    RustExpr::Try(Box::new(RustExpr::FunctionCall(
        Box::new(RustExpr::Entity(RustEntity::Local(lbl(
            "repeat_between_finished",
        )))),
        vec![out_of_reps, len, min, max],
    )))
}
// !SECTION

// SECTION - helpers that are used in the model but themselves do not depend on the model

/// RustExpr for `Vec::new()`
pub fn vec_new() -> RustExpr {
    RustExpr::FunctionCall(
        Box::new(RustExpr::Entity(RustEntity::Scoped(
            vec![lbl("Vec")],
            lbl("new"),
        ))),
        Vec::new(),
    )
}

/// RustStmt for `let mut <ident> = Vec::new()`
pub fn let_mut_vec_new(ident: &'static str) -> RustStmt {
    RustStmt::Let(Mut::Mutable, lbl(ident), None, vec_new())
}

/// RustExpr for `Vec::push(self, elem)`
pub fn vec_push(arr: RustExpr, elem: RustExpr) -> RustExpr {
    call!(arr, push, elem)
}

pub fn gt0(val: RustExpr) -> RustExpr {
    RustExpr::infix(val, InfixOperator::Gt, RustExpr::num_lit(0usize))
}

pub fn eq0(val: RustExpr) -> RustExpr {
    RustExpr::infix(val, InfixOperator::Eq, RustExpr::num_lit(0usize))
}

// !SECTION
