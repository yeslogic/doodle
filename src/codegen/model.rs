use crate::{BaseKind, Label};

use super::rust_ast::*;
use super::{GenBlock, GenExpr, GenStmt};
#[allow(unused_imports)]
use crate::Format as _;

// NOTE - this marks whether `allsorts::binary::read::ReadArray` is `Copy`
pub(crate) const READ_ARRAY_IS_COPY: bool = true;

// NOTE - based on the (temporary) hardcoding of `crate::parser::view::View` for target view-objects
// we wish to record certain properties of it abstractly without difficult-to-track hardcoding
pub(crate) const VIEW_OBJECT_IS_COPY: bool = true;

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
    ( fn $ident:ident $(, $arg:expr)* ) => {
        RustExpr::FunctionCall(
            Box::new(RustExpr::Entity(RustEntity::Local(lbl(stringify!($ident))))),
            vec![$($arg),*],
        )
    };
}

macro_rules! try_call {
    ( $x:expr, $y:ident $(, $z:expr )* $(,)? ) => {
        RustExpr::Try(Box::new(call!( $x, $y $(, $z)* )))
    };
    ( fn $ident:ident  $(, $arg:expr )*  ) => {
        RustExpr::Try(Box::new(
            call!( fn $ident $(, $arg)* )
        ))
    };
}

pub const fn lbl(x: &'static str) -> Label {
    Label::Borrowed(x)
}

// SECTION - magic strings for fixed identifiers as codegen artifacts
pub const DEFAULT_LT: &str = "'input";

/// General-purpose magic identifier for embedded-MatchTree branch-id bindings.
pub const MATCH_BRANCH_IX: &str = "matching_ix";

/// Magic identifier for temporary bindings used
pub const COMPLEX_COND_TMP: &str = "tmp_cond";

// SECTION - magic strings related to EngineLogic
pub const SLICE_LEN: &str = "sz";
pub const SLICE_RET: &str = "ret";

pub const OFFS_PEEK_TARGET: &str = "tgt_offset";
pub const OFFS_PEEK_DBG_ADV: &str = "_is_advance";
pub const OFFS_PEEK_RET: &str = "ret";

pub const PEEK_RET: &str = "ret";

/// Magic string for the `Result<_, E>` value produced and inspected during `Format::PeekNot` processing
pub const PEEK_NOT_RES: &str = "res";

/// Magic identifier used to record the number of bits read (upon escaping bits-mode), for debugging purposes
pub const BITS_NREAD: &str = "_bits_read";
pub const BITS_RET: &str = "ret";
// !SECTION

/// SECTION - magic strings related to RepeatLogic
pub const R0COM_ACCUM: &str = "accum";
pub const R0COM_ELEM: &str = "next_elem";

pub const R1BOM_ACCUM: &str = "accum";
pub const R1BOM_ELEM: &str = "next_elem";

/// Special variation of `MATCH_BRANCH_IX` for `RepeatLogic::BetweenCounts` due to the unique semantics of the branch-index
pub const BETWEEN_REPS_LEFT: &str = "reps_left";
pub const BETWEEN_ACCUM: &str = "accum";
pub const BETWEEN_ELEM: &str = "next_elem";

pub const FOREACH_ACCUM: &str = "accum";
pub const FOREACH_ELEM: &str = "next_elem";

pub const UNFOLD_SEQ: &str = "seq";
pub const UNFOLD_ACC: &str = "acc";
pub const UNFOLD_ELEM: &str = "elem";

pub const EXACT_ACCUM: &str = "accum";
pub const EXACT_ELEM: &str = "next_elem";

pub const UNTIL_LAST_ACCUM: &str = "accum";
pub const UNTIL_LAST_ELEM: &str = "next_elem";

pub const UNTIL_SEQ_ACCUM: &str = "accum";
pub const UNTIL_SEQ_ELEM: &str = "next_elem";
// !SECTION

// SECTION - magic strings related to SequentialLogic
pub const ACCUM_SEQ_PREFIX: &str = "ix";
pub const ACCUM_TUP_PREFIX: &str = "arg";
// !SECTION

// SECTION - magic strings related to OtherLogic
pub const DESCEND_IX: &str = "tree_index";
// !SECTION

// SECTION - magic strings related to ParallelLogic
pub const ALT_RES: &str = "res";
pub const ALT_OK_BIND: &str = "inner";
pub const ALT_ERR_BIND: &str = "_e";
// !SECTION

// SECTION - magic strings related to DerivedLogic
pub const DECODE_BUF_PARSER_OBJ: &str = "buf_parser";
pub const DECODE_BUF_PARSER_REF: &str = "buf_input";

pub const PARSE_VIEW_PARSER_OBJ: &str = "view_parser";
pub const PARSE_VIEW_PARSER_REF: &str = "view_input";

pub const VARIANT_INNER: &str = "inner";

pub const WHERE_INNER: &str = "inner";
pub const WHERE_CHECK: &str = "is_valid";
// !SECTION

// !SECTION

pub fn view_obj_type(lt: RustLt) -> RustType {
    RustType::ViewObject(lt)
}

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

pub fn err_fallthrough(trace: u64) -> RustExpr {
    RustExpr::ResultErr(Box::new(RustExpr::FunctionCall(
        Box::new(RustExpr::Entity(RustEntity::Scoped(
            vec![lbl("ParseError")],
            lbl("ExcludedBranch"),
        ))),
        vec![RustExpr::u64lit(trace)],
    )))
}

pub fn err_where_unsatisfied(trace: u64) -> RustExpr {
    RustExpr::ResultErr(Box::new(RustExpr::FunctionCall(
        Box::new(RustExpr::Entity(RustEntity::Scoped(
            vec![lbl("ParseError")],
            lbl("FalsifiedWhere"),
        ))),
        vec![RustExpr::u64lit(trace)],
    )))
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

/// Model RustExpr for handling `ViewFormat::ReadArray(len, kind)` in the Parser (View) model.
pub fn read_array_from_view(view: RustExpr, len: RustExpr, kind: BaseKind) -> RustExpr {
    // NOTE - we need these separate methods because RustExpr::MethodCall doesn't allow turbo-fish type-parameters
    match kind {
        BaseKind::U8 => try_call!(view, read_array_u8, len),
        BaseKind::U16 => try_call!(view, read_array_u16be, len),
        BaseKind::U32 => try_call!(view, read_array_u32be, len),
        BaseKind::U64 => try_call!(view, read_array_u64be, len),
    }
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

/// Model RustExpr for setup of `Format::UnionNondet` parse-context in the Parser model.
pub fn start_alt(parser: RustExpr) -> RustExpr {
    call!(parser, start_alt)
}

pub fn try_next_alt(parser: RustExpr, next_is_final: bool) -> RustExpr {
    try_call!(parser, next_alt, RustExpr::bool_lit(next_is_final))
}

// !SECTION

// SECTION - boilerplate patterns based on incidental implementation details of codegen process
pub(super) fn simplifying_if(
    predicate: RustExpr,
    then_branch: GenBlock,
    else_branch: Option<GenBlock>,
) -> GenExpr {
    if !predicate.is_complex() {
        GenExpr::Control(Box::new(RustControl::If(
            Box::new(predicate),
            then_branch,
            else_branch,
        )))
    } else {
        // REVIEW - we might need a strategy to avoid the temporary variable accidentally shadowing locally-external bindings
        GenExpr::BlockScope(Box::new(GenBlock::from(vec![
            GenStmt::Embed(RustStmt::assign(COMPLEX_COND_TMP, predicate)),
            RustControl::If(
                Box::new(RustExpr::local(COMPLEX_COND_TMP)),
                then_branch,
                else_branch,
            )
            .into(),
        ])))
    }
}

/// Pushes a `GenBlock` to the end of a sequence of terms, possibly via a local variable-assignment if it is too complex
/// to inline (via pushing an assignment to a list of statements and referring to the lhs-bound variable instead of pushing
/// directly to `terms`).
///
/// If a name is required for the lhs-bound variable, it is generated via the `mk_name` closure.
pub(super) fn push_seq_term(
    term: GenBlock,
    stmts: &mut Vec<GenStmt>,
    terms: &mut Vec<RustExpr>,
    mk_name: impl FnOnce() -> Label,
) {
    if term.is_simple() {
        let expr = RustExpr::from(term);
        terms.push(expr);
    } else {
        let name = mk_name();
        stmts.push(GenStmt::BindOnce(name.clone(), term));
        terms.push(RustExpr::local(name));
    }
}

pub const fn match_case_usize(n: usize) -> MatchCaseLHS {
    MatchCaseLHS::Pattern(RustPattern::PrimLiteral(RustPrimLit::Numeric(
        RustNumLit::Usize(n),
    )))
}

pub fn match_case_ok_bind(lab: &'static str) -> MatchCaseLHS {
    MatchCaseLHS::Pattern(RustPattern::Variant(
        Constructor::Simple(lbl("Ok")),
        Box::new(RustPattern::CatchAll(Some(lbl(lab)))),
    ))
}

pub fn match_case_err_bind(lab: &'static str) -> MatchCaseLHS {
    MatchCaseLHS::Pattern(RustPattern::Variant(
        Constructor::Simple(lbl("Err")),
        Box::new(RustPattern::CatchAll(Some(lbl(lab)))),
    ))
}

/// RustExpr for `Parser::new(<buffer>)`
///
/// `buffer` should already be appropriately coerced into `&[u8]` (not `Vec<u8>` or `&Vec<u8>`).
pub fn parser_new(buffer: RustExpr) -> RustExpr {
    RustExpr::FunctionCall(
        Box::new(RustExpr::Entity(RustEntity::Scoped(
            vec![lbl("Parser")],
            lbl("new"),
        ))),
        vec![buffer],
    )
}

/// RustStmt for `let mut <ident> = Parser::new(<buffer>)`.
///
/// The argument `buffer` should already be appropriately coerced into `&[u8]` (not `Vec<u8>` or `&Vec<u8>`).
pub fn let_mut_parser_new(ident: &'static str, buffer: RustExpr) -> RustStmt {
    RustStmt::Let(Mut::Mutable, lbl(ident), None, parser_new(buffer))
}

/// RustExpr for `Parser::from(<view>)`
pub fn parser_from(view: RustExpr) -> RustExpr {
    RustExpr::FunctionCall(
        Box::new(RustExpr::Entity(RustEntity::Scoped(
            vec![lbl("Parser")],
            lbl("from"),
        ))),
        vec![view],
    )
}

/// RustStmt for `let mut <ident> = Parser::from(<view>)`.
pub fn let_mut_parser_from(ident: &'static str, view: RustExpr) -> RustStmt {
    RustStmt::Let(Mut::Mutable, lbl(ident), None, parser_from(view))
}

pub fn reify_view(view_raw: RustExpr) -> RustExpr {
    // NOTE - View used both for transient parses and persisted values
    view_raw
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
    try_call!(fn repeat_between_finished, out_of_reps, len, min, max )
}

pub fn parse_huffman(code_lengths: RustExpr, opt_values_expr: Option<RustExpr>) -> RustExpr {
    let values = RustExpr::lift_option(opt_values_expr);
    call!(fn parse_huffman, code_lengths, values)
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

/// RustStmt for `let mut <ident> = Vec::new()`
pub fn let_mut_sig_vec_new(ident: &'static str, seq_ty: RustType) -> RustStmt {
    RustStmt::Let(Mut::Mutable, lbl(ident), Some(seq_ty), vec_new())
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
