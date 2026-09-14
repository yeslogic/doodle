//! Recursive-descent (precedence-climbing) parser from the macro's
//! `TokenStream` into `ast::Expr`.
//!
//! Grammar:
//!
//! ```text
//! Expr    := Cast (BinOp MRep? Expr)*       -- left-associative, precedence-climbed
//! Cast    := Unary (("as" | "into") MRep)*  -- left-associative (chains: `x as u8 as i16`)
//! Unary   := UnaryKw MRep? Unary
//!          | "-" <int-literal>              -- folds into Const's inherent sign
//!          | "-" MRep? Unary
//!          | Term
//! Term    := <int-literal> | <str-literal> | "(" Expr ")"
//! UnaryKw := "abs" | "succ" | "pred"
//! BinOp   := "+" | "-" | "*" | "/" | "%"
//! MRep    := "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64"
//! ```
//!
//! Each layer binds tighter than the one above it, matching both the source
//! `.lalrpop` grammar's precedence declarations and real Rust's own operator
//! precedence (unary tighter than `as`, tighter than `* / %`, tighter than
//! `+ -`). `as` matches real Rust `as`-cast (bitwise/reinterpreting)
//! semantics; `into` is the value-preserving cast (the `.lalrpop` grammar's
//! default, unprefixed `COp` case).

use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, LitStr, Result, Token};

use crate::ast::{BinOpCode, CastSemantics, Expr, MRep, UnaryOpCode};

impl Parse for Expr {
    fn parse(input: ParseStream) -> Result<Self> {
        parse_expr(input, 0)
    }
}

fn parse_expr(input: ParseStream, min_bp: u8) -> Result<Expr> {
    let mut lhs = parse_cast_expr(input)?;

    loop {
        let Some((op, bp)) = peek_binop(input) else {
            break;
        };
        if bp < min_bp {
            break;
        }
        consume_binop(input, op)?;
        let out_rep = try_parse_mrep(input)?;
        // `bp + 1` (rather than `bp`) on the recursive call enforces
        // left-associativity: a same-precedence operator to the right is not
        // absorbed into the RHS, so it instead gets picked up by this loop.
        let rhs = parse_expr(input, bp + 1)?;
        lhs = Expr::BinOp {
            op,
            out_rep,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }

    Ok(lhs)
}

/// Parses a `Cast` (see the module-level grammar comment): a `Unary`
/// followed by zero or more chained `as`/`into` casts, left-associative
/// (`x as u8 as i16` == `(x as u8) as i16`).
fn parse_cast_expr(input: ParseStream) -> Result<Expr> {
    let mut expr = parse_unary_expr(input)?;

    loop {
        let semantics = if input.peek(Token![as]) {
            input.parse::<Token![as]>()?;
            CastSemantics::Bitwise
        } else if peek_keyword(input, "into") {
            let _keyword: Ident = input.parse()?;
            CastSemantics::Arithmetic
        } else {
            break;
        };

        let rep = parse_required_mrep(input)?;
        expr = Expr::Cast {
            semantics,
            rep,
            inner: Box::new(expr),
        };
    }

    Ok(expr)
}

/// Parses a `Unary` (see the module-level grammar comment): an optional
/// stack of prefix unary operators (`abs`/`succ`/`pred`/`-`), bottoming out
/// in a `Term`.
fn parse_unary_expr(input: ParseStream) -> Result<Expr> {
    if let Some(op) = peek_unary_keyword(input) {
        // Re-parse (rather than reusing the fork) now that we know it matches.
        let _keyword: Ident = input.parse()?;
        let out_rep = try_parse_mrep(input)?;
        let operand = parse_unary_expr(input)?;
        return Ok(Expr::UnaryOp {
            op,
            out_rep,
            operand: Box::new(operand),
        });
    }

    if input.peek(Token![-]) {
        input.parse::<Token![-]>()?;

        // `- <int-literal>` directly (no rep keyword in between) folds into
        // the literal's own inherent sign, rather than becoming an operator
        // node — see `UnaryOpCode::Negate`'s doc comment.
        if input.peek(LitInt) {
            let lit: LitInt = input.parse()?;
            let value: i64 = lit.base10_parse()?;
            let rep = lit_suffix_rep(&lit)?;
            return Ok(Expr::Const { value: -value, rep });
        }

        let out_rep = try_parse_mrep(input)?;
        let operand = parse_unary_expr(input)?;
        return Ok(Expr::UnaryOp {
            op: UnaryOpCode::Negate,
            out_rep,
            operand: Box::new(operand),
        });
    }

    parse_term(input)
}

fn parse_term(input: ParseStream) -> Result<Expr> {
    if input.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in input);
        let inner = parse_expr(&content, 0)?;
        if !content.is_empty() {
            return Err(content.error("unexpected token after expression"));
        }
        return Ok(inner);
    }

    if input.peek(LitStr) {
        let lit: LitStr = input.parse()?;
        return Ok(Expr::Var(lit));
    }

    if input.peek(LitInt) {
        let lit: LitInt = input.parse()?;
        let value: i64 = lit.base10_parse()?;
        let rep = lit_suffix_rep(&lit)?;
        return Ok(Expr::Const { value, rep });
    }

    Err(input.error(
        "expected a numeric literal, a quoted variable name (e.g. \"x\"), or a parenthesized expression",
    ))
}

/// Reads an integer literal's suffix (e.g. the `u16` in `5u16`) as an `MRep`,
/// or `None` if the literal has no suffix.
fn lit_suffix_rep(lit: &LitInt) -> Result<Option<MRep>> {
    let suffix = lit.suffix();
    if suffix.is_empty() {
        return Ok(None);
    }
    match MRep::from_suffix(suffix) {
        Some(rep) => Ok(Some(rep)),
        None => Err(syn::Error::new(
            lit.span(),
            format!("unknown numeric representation suffix `{suffix}`"),
        )),
    }
}

/// Optionally consumes one of the unary-operator keywords (`abs`/`succ`/`pred`),
/// without consuming anything if the next token isn't one of them.
fn peek_unary_keyword(input: ParseStream) -> Option<UnaryOpCode> {
    if !input.peek(Ident) {
        return None;
    }
    let fork = input.fork();
    let ident: Ident = fork.parse().ok()?;
    match ident.to_string().as_str() {
        "abs" => Some(UnaryOpCode::AbsVal),
        "succ" => Some(UnaryOpCode::IntSucc),
        "pred" => Some(UnaryOpCode::IntPred),
        _ => None,
    }
}

/// Returns `true` (without consuming) if the next token is a bare identifier
/// spelled exactly `kw`. Used for the `into` cast keyword, which (unlike
/// `as`) isn't a reserved Rust keyword and so has no dedicated `syn::Token!`.
fn peek_keyword(input: ParseStream, kw: &str) -> bool {
    if !input.peek(Ident) {
        return false;
    }
    let fork = input.fork();
    match fork.parse::<Ident>() {
        Ok(ident) => ident == kw,
        Err(_) => false,
    }
}

/// Parses an `MRep` keyword, erroring (rather than leaving it unconsumed) if
/// the next token isn't one. Used for `as`/`into`, where a representation is
/// mandatory, unlike the optional forced-output-rep after a unary/binary op.
fn parse_required_mrep(input: ParseStream) -> Result<MRep> {
    let ident: Ident = input.parse()?;
    MRep::from_suffix(&ident.to_string()).ok_or_else(|| {
        syn::Error::new(
            ident.span(),
            format!(
                "expected a representation keyword (u8/u16/u32/u64/i8/i16/i32/i64), found `{ident}`"
            ),
        )
    })
}

fn peek_binop(input: ParseStream) -> Option<(BinOpCode, u8)> {
    if input.peek(Token![+]) {
        Some((BinOpCode::Add, 1))
    } else if input.peek(Token![-]) {
        Some((BinOpCode::Sub, 1))
    } else if input.peek(Token![*]) {
        Some((BinOpCode::Mul, 2))
    } else if input.peek(Token![/]) {
        Some((BinOpCode::Div, 2))
    } else if input.peek(Token![%]) {
        Some((BinOpCode::Rem, 2))
    } else {
        None
    }
}

fn consume_binop(input: ParseStream, op: BinOpCode) -> Result<()> {
    match op {
        BinOpCode::Add => {
            input.parse::<Token![+]>()?;
        }
        BinOpCode::Sub => {
            input.parse::<Token![-]>()?;
        }
        BinOpCode::Mul => {
            input.parse::<Token![*]>()?;
        }
        BinOpCode::Div => {
            input.parse::<Token![/]>()?;
        }
        BinOpCode::Rem => {
            input.parse::<Token![%]>()?;
        }
    }
    Ok(())
}

/// Optionally consumes a forced-output-representation keyword (e.g. the
/// `u16` in `"a" +u16 "b"`), without consuming anything if the next token
/// isn't one of the known `MRep` keywords.
///
/// Note: since `Var` is spelled as a string literal rather than a bare
/// identifier, no other surface-syntax construct in this first slice starts
/// with a bare `Ident`, so this lookahead is currently unambiguous. This will
/// need revisiting once unary-op keywords (`abs`/`succ`/`pred`) are added,
/// since those are also bare identifiers.
fn try_parse_mrep(input: ParseStream) -> Result<Option<MRep>> {
    if !input.peek(Ident) {
        return Ok(None);
    }
    let fork = input.fork();
    let ident: Ident = fork.parse()?;
    match MRep::from_suffix(&ident.to_string()) {
        Some(rep) => {
            input.advance_to(&fork);
            Ok(Some(rep))
        }
        None => Ok(None),
    }
}
