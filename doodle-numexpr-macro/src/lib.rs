//! Proc-macro front-end for a small arithmetic-expression surface syntax
//! that lowers directly to `doodle::numeric::core::Expr`, so that
//! `doodle`-based format specs can write e.g. `numexpr!("width" * "height")`
//! instead of hand-building `Expr`/`BinOp`/`TypedConst` trees.
//!
//! Currently supported: numeric/string-literal terms, parenthesization,
//! left-associative binary operators (optionally with a forced output
//! representation, e.g. `"a" +u16 "b"`), unary prefix operators
//! (`abs`/`succ`/`pred`/`-`, also with an optional forced output
//! representation, e.g. `abs u16 "a"`), and casts: `"a" as u8` (bitwise,
//! matching real Rust `as`-cast semantics) and `"a" into u8`
//! (value-preserving). A leading `-` directly on an integer literal (e.g.
//! `-5`) is folded into the constant's own sign rather than becoming an
//! operator node.

mod ast;
mod codegen;
mod parse;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn numexpr(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as ast::Expr);
    codegen::to_tokens(&expr).into()
}
