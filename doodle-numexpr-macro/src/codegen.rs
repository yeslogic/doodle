//! Lowers `ast::Expr` into a `TokenStream` that constructs the corresponding
//! `doodle::numeric::core::Expr` value.
//!
//! Every emitted path is absolute (`::doodle::...`), so the calling crate
//! only needs a regular `doodle` dependency in scope — no imports required
//! at the macro call site.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ast::{BinOpCode, CastSemantics, Expr, MRep, UnaryOpCode};

pub fn to_tokens(expr: &Expr) -> TokenStream {
    match expr {
        Expr::Const { value, rep } => {
            let rep_tokens = numrep_tokens(rep);
            quote! {
                ::doodle::numeric::core::Expr::Const(
                    ::doodle::numeric::core::TypedConst::new(#value, #rep_tokens)
                )
            }
        }
        Expr::Var(lit) => {
            quote! {
                ::doodle::numeric::core::Expr::NumVar(::std::borrow::Cow::Borrowed(#lit))
            }
        }
        Expr::UnaryOp {
            op,
            out_rep,
            operand,
        } => {
            let op_tokens = basic_unaryop_tokens(*op);
            let out_rep_tokens = opt_machinerep_tokens(out_rep);
            let operand_tokens = to_tokens(operand);
            quote! {
                ::doodle::numeric::core::Expr::UnaryOp(
                    ::doodle::numeric::core::UnaryOp::new(#op_tokens, #out_rep_tokens),
                    ::std::boxed::Box::new(#operand_tokens),
                )
            }
        }
        Expr::Cast {
            semantics,
            rep,
            inner,
        } => {
            let machine_rep = machinerep_tokens(*rep);
            let cast_op_tokens = match semantics {
                CastSemantics::Arithmetic => {
                    quote! { ::doodle::numeric::core::CastOp::arith(#machine_rep) }
                }
                CastSemantics::Bitwise => {
                    quote! { ::doodle::numeric::core::CastOp::bitwise(#machine_rep) }
                }
            };
            let inner_tokens = to_tokens(inner);
            quote! {
                ::doodle::numeric::core::Expr::Cast(
                    #cast_op_tokens,
                    ::std::boxed::Box::new(#inner_tokens),
                )
            }
        }
        Expr::BinOp {
            op,
            out_rep,
            lhs,
            rhs,
        } => {
            let op_tokens = basic_binop_tokens(*op);
            let out_rep_tokens = opt_machinerep_tokens(out_rep);
            let lhs_tokens = to_tokens(lhs);
            let rhs_tokens = to_tokens(rhs);
            quote! {
                ::doodle::numeric::core::Expr::BinOp(
                    ::doodle::numeric::core::BinOp::new(#op_tokens, #out_rep_tokens),
                    ::std::boxed::Box::new(#lhs_tokens),
                    ::std::boxed::Box::new(#rhs_tokens),
                )
            }
        }
    }
}

fn numrep_tokens(rep: &Option<MRep>) -> TokenStream {
    match rep {
        None => quote! { ::doodle::numeric::core::NumRep::AUTO },
        Some(rep) => {
            let machine_rep = machinerep_tokens(*rep);
            quote! { ::doodle::numeric::core::NumRep::Concrete(#machine_rep) }
        }
    }
}

fn opt_machinerep_tokens(rep: &Option<MRep>) -> TokenStream {
    match rep {
        None => quote! { ::std::option::Option::None },
        Some(rep) => {
            let machine_rep = machinerep_tokens(*rep);
            quote! { ::std::option::Option::Some(#machine_rep) }
        }
    }
}

fn machinerep_tokens(rep: MRep) -> TokenStream {
    match rep {
        MRep::U8 => quote! { ::doodle::numeric::core::MachineRep::U8 },
        MRep::U16 => quote! { ::doodle::numeric::core::MachineRep::U16 },
        MRep::U32 => quote! { ::doodle::numeric::core::MachineRep::U32 },
        MRep::U64 => quote! { ::doodle::numeric::core::MachineRep::U64 },
        MRep::I8 => quote! { ::doodle::numeric::core::MachineRep::I8 },
        MRep::I16 => quote! { ::doodle::numeric::core::MachineRep::I16 },
        MRep::I32 => quote! { ::doodle::numeric::core::MachineRep::I32 },
        MRep::I64 => quote! { ::doodle::numeric::core::MachineRep::I64 },
    }
}

fn basic_unaryop_tokens(op: UnaryOpCode) -> TokenStream {
    match op {
        UnaryOpCode::Negate => quote! { ::doodle::numeric::core::BasicUnaryOp::Negate },
        UnaryOpCode::AbsVal => quote! { ::doodle::numeric::core::BasicUnaryOp::AbsVal },
        UnaryOpCode::IntSucc => quote! { ::doodle::numeric::core::BasicUnaryOp::IntSucc },
        UnaryOpCode::IntPred => quote! { ::doodle::numeric::core::BasicUnaryOp::IntPred },
    }
}

fn basic_binop_tokens(op: BinOpCode) -> TokenStream {
    match op {
        BinOpCode::Add => quote! { ::doodle::numeric::core::BasicBinOp::Add },
        BinOpCode::Sub => quote! { ::doodle::numeric::core::BasicBinOp::Sub },
        BinOpCode::Mul => quote! { ::doodle::numeric::core::BasicBinOp::Mul },
        BinOpCode::Div => quote! { ::doodle::numeric::core::BasicBinOp::Div },
        BinOpCode::Rem => quote! { ::doodle::numeric::core::BasicBinOp::Rem },
    }
}
