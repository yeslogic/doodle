use crate::Label;

use super::rust_ast::*;
#[allow(unused_imports)]
use crate::Format as _;


// SECTION - magic strings for fixed identifiers as codegen artifacts
pub const SLICE_LEN_BIND_IDENT: &'static str = "sz";
// !SECTION


// SECTION - Helper functions for prelude-specific AST-constructions

/// Helper function for promoting string-constants to method specifiers
const fn mk_method(ident: &'static str) -> MethodSpecifier {
    MethodSpecifier::Arbitrary(SubIdent::ByName(Label::Borrowed(ident)))
}

/// Model RustExpr for handling [`Format::Fail`], as an `Err(<..>)`-value with accompanying trace-hash.
pub fn err_fail(trace: u64) -> RustExpr {
    RustExpr::ResultErr(Box::new(
        RustExpr::scoped(["ParseError"], "FailToken").call_with([RustExpr::PrimitiveLit(
            RustPrimLit::Numeric(RustNumLit::U64(trace)),
        )]),
    ))
}

/// Model RustExpr for handling [`Format::EndOfInput`] as a Try-pattern (`<..>?`).
pub fn parser_expect_eos(parser: RustExpr) -> RustExpr {
    const EXPECT_EOS_METHOD: &'static str = "finish";
    RustExpr::Try(Box::new(RustExpr::MethodCall(
        Box::new(parser),
        mk_method(EXPECT_EOS_METHOD),
        Vec::new(),
    )))
}

/// Model RustExpr for handling `Format::SkipRemainder` in the Parser model.
pub fn skip_remainder(parser: RustExpr) -> RustExpr {
    const SKIP_REMAINDER_METHOD: &'static str = "skip_remainder";
    RustExpr::MethodCall(
        Box::new(parser),
        mk_method(SKIP_REMAINDER_METHOD),
        Vec::new(),
    )
}

/// Model RustExpr for handling `Format::Align(n)` in the Parser model.
pub fn skip_until_aligned(parser: RustExpr, n: usize) -> RustExpr {
    const SKIP_ALIGN_METHOD: &'static str = "skip_align";
    RustExpr::Try(Box::new(
        RustExpr::MethodCall(
            Box::new(parser),
            mk_method(SKIP_ALIGN_METHOD),
            vec![RustExpr::num_lit(n)],
        )
    ))
}

pub fn yield_offset_as_u64(parser: RustExpr) -> RustExpr {
    const GET_OFFSET_U64: &'static str = "get_offset_u64";
    RustExpr::MethodCall(
        Box::new(parser),
        mk_method(GET_OFFSET_U64),
        Vec::new(),
    )
}

// !SECTION
