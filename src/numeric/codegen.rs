use std::borrow::Cow;

use crate::Label;
use crate::codegen::ToFragment;
use crate::codegen::rust_ast::RustPrimLit;
use crate::codegen::{
    rust_ast::{
        ClosureBody, FnEntity, NumType, RustClosure, RustClosureHead, RustEntity, RustExpr,
    },
    typed_format::GenType,
};
use crate::output::Fragment;

use super::{
    core::{BasicBinOp, BasicUnaryOp, BinOp, MachineRep, UnaryOp},
    elaborator::{IntType, MapType, PrimInt, Sig1, Sig2, TypedExpr},
};

#[derive(Clone, Copy, Debug)]
enum BinOpClass<T> {
    Pure(T),
    HomWide(T, T),
    HetWide(T, T, T),
    HomLossy(T, T),
    HetLossy(T, T, T),
}

#[derive(Clone, Copy, Debug)]
enum UnaryOpClass<T> {
    Pure(T),
    NonLossy(T, T),
    Lossy(T, T),
}

fn classify_binary(sig: Sig2<IntType>) -> BinOpClass<PrimInt> {
    let ((l, r), o) = sig;
    let l = l.to_prim();
    let r = r.to_prim();
    let o = o.to_prim();

    let lrep = MachineRep::from(l);
    let rrep = MachineRep::from(r);
    let orep = MachineRep::from(o);

    if lrep == rrep {
        if orep == lrep {
            BinOpClass::Pure(l)
        } else if orep.encompasses(lrep) {
            BinOpClass::HomWide(l, o)
        } else {
            BinOpClass::HomLossy(l, o)
        }
    } else if orep.encompasses(lrep) && orep.encompasses(rrep) {
        BinOpClass::HetWide(l, r, o)
    } else {
        BinOpClass::HetLossy(l, r, o)
    }
}

// NOTE - depending on the op, widening vs lossy might be affected (e.g. Abs(i8) fits in u8)
// FIXME - while this technically works, there is some fuzziness with regard to the intended semantics vs what we are effectively measuring (i.e. did we bother implementing a backend function we can call)
fn classify_unary(op: Option<BasicUnaryOp>, sig: Sig1<IntType>) -> UnaryOpClass<PrimInt> {
    let (i, o) = sig;
    let i = i.to_prim();
    let o = o.to_prim();

    let irep = MachineRep::from(i);
    let orep = MachineRep::from(o);

    match op {
        None => {
            // Cast - no operational influence on OpClass
            if irep == orep {
                UnaryOpClass::Pure(i)
            } else if orep.encompasses(irep) {
                UnaryOpClass::NonLossy(i, o)
            } else {
                UnaryOpClass::Lossy(i, o)
            }
        }
        Some(BasicUnaryOp::AbsVal) => {
            if irep == orep {
                UnaryOpClass::Pure(i)
            } else if absval_is_nonlossy(irep, orep) {
                UnaryOpClass::NonLossy(i, o)
            } else {
                UnaryOpClass::Lossy(i, o)
            }
        }
        Some(BasicUnaryOp::Negate) => {
            if irep == orep {
                UnaryOpClass::Pure(i)
            } else if negate_is_nonlossy(irep, orep) {
                UnaryOpClass::NonLossy(i, o)
            } else {
                UnaryOpClass::Lossy(i, o)
            }
        }
        Some(BasicUnaryOp::IntPred) => {
            // REVIEW - this might be misimplemented
            if irep == orep {
                UnaryOpClass::Pure(i)
            } else if pred_is_nonlossy(irep, orep) {
                UnaryOpClass::NonLossy(i, o)
            } else {
                UnaryOpClass::Lossy(i, o)
            }
        }
        Some(BasicUnaryOp::IntSucc) => {
            // REVIEW - this might be misimplemented
            if irep == orep {
                UnaryOpClass::Pure(i)
            } else if succ_is_nonlossy(irep, orep) {
                UnaryOpClass::NonLossy(i, o)
            } else {
                UnaryOpClass::Lossy(i, o)
            }
        }
    }
}

fn absval_is_nonlossy(source: MachineRep, target: MachineRep) -> bool {
    if source.is_signed() {
        // Signed -> Unsigned is non-lossy if the target-precision is greater-than-or-equal to target precision
        matches!(
            target.compare_width(source),
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater
        )
    } else {
        target.encompasses(source)
    }
}

fn negate_is_nonlossy(source: MachineRep, target: MachineRep) -> bool {
    if source.is_signed() {
        // Neg is effectively a no-op w.r.t the bounds of the input and output, so normal rules apply
        matches!(
            target.compare_width(source),
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater
        )
    } else {
        // Unsigned negation is never lossy because we avoid downcasting entirely
        true
    }
}

fn succ_is_nonlossy(source: MachineRep, target: MachineRep) -> bool {
    target.encompasses(source)
}

fn pred_is_nonlossy(source: MachineRep, target: MachineRep) -> bool {
    target.encompasses(source)
}

pub(crate) const SYNTHETIC_BINOP: &str = "eval_fallback";
pub(crate) const SYNTHETIC_UNARY: &str = "eval_unary_fallback";

fn induce_binary_fname(op: BinOp, class: BinOpClass<PrimInt>) -> Label {
    let base = match op.get_op() {
        BasicBinOp::Add => "add",
        BasicBinOp::Sub => "sub",
        BasicBinOp::Mul => "mul",
        BasicBinOp::Div => "div",
        BasicBinOp::Rem => "rem",
    };
    let str = match class {
        BinOpClass::Pure(t) => format!("{}_{}", base, t.to_static_str()),
        BinOpClass::HomWide(t0, t1) | BinOpClass::HomLossy(t0, t1) => {
            format!("{}_{}_{}", base, t0.to_static_str(), t1.to_static_str())
        }
        BinOpClass::HetWide(t0, t1, t2) | BinOpClass::HetLossy(t0, t1, t2) => format!(
            "{}_{}_{}_{}",
            base,
            t0.to_static_str(),
            t1.to_static_str(),
            t2.to_static_str()
        ),
    };
    Label::Owned(str)
}

fn induce_unary_fname(op: UnaryOp, class: UnaryOpClass<PrimInt>) -> Label {
    let base = match op.get_op() {
        BasicUnaryOp::Negate => "neg",
        BasicUnaryOp::AbsVal => "abs",
        BasicUnaryOp::IntPred => "pred",
        BasicUnaryOp::IntSucc => "succ",
    };
    let str = match class {
        UnaryOpClass::Pure(t) => format!("{}_{}", base, t.to_static_str()),
        UnaryOpClass::NonLossy(t0, t1) | UnaryOpClass::Lossy(t0, t1) => {
            format!("{}_{}_{}", base, t0.to_static_str(), t1.to_static_str())
        }
    };
    Label::Owned(str)
}

fn induce_cast_fname(class: UnaryOpClass<PrimInt>) -> Label {
    let base = "cast";
    let str = match class {
        UnaryOpClass::NonLossy(t0, t1) | UnaryOpClass::Lossy(t0, t1) => {
            format!("{}_{}_{}", base, t0.to_static_str(), t1.to_static_str())
        }
        UnaryOpClass::Pure(_) => unreachable!("pure casts should be elided during synthesis"),
    };
    Label::Owned(str)
}

fn synthesize_unary(op: UnaryOp) -> RustExpr {
    let ent = {
        let (qual, meth) = match op.get_op() {
            BasicUnaryOp::Negate => ("Neg", "neg"),
            BasicUnaryOp::AbsVal => ("Signed", "abs"),
            // WIP[epic=embedded-num] - trait does not yet exist
            BasicUnaryOp::IntPred => ("Numeric", "pred"),
            BasicUnaryOp::IntSucc => ("Numeric", "succ"),
        };
        RustEntity::Scoped(vec![Label::Borrowed(qual)], Label::Borrowed(meth))
    };
    let closure = {
        let input = Label::Borrowed("x");
        let invocation = {
            let fn_spec = FnEntity::Specific { fname: ent };
            let fn_args = vec![RustExpr::Entity(RustEntity::Local(input.clone()))];
            RustExpr::Invoke(fn_spec, fn_args)
        };
        RustClosure::new_transform(input, None, invocation)
    };
    RustExpr::Closure(closure)
}

fn synthesize_binop(op: BinOp) -> RustExpr {
    let ent = {
        let qual = Label::Borrowed("BigInt");
        let meth = match op.get_op() {
            BasicBinOp::Add => "checked_add",
            BasicBinOp::Sub => "checked_sub",
            BasicBinOp::Mul => "checked_mul",
            BasicBinOp::Div => "checked_div",
            BasicBinOp::Rem => "checked_rem",
        };
        RustEntity::Scoped(vec![qual], Label::Borrowed(meth))
    };
    let closure = {
        let lhs = Label::Borrowed("x");
        let rhs = Label::Borrowed("y");
        let head_args = vec![(lhs.clone(), None), (rhs.clone(), None)];
        let body = {
            let fn_spec = FnEntity::Specific { fname: ent };
            let fn_args = vec![RustExpr::local(lhs), RustExpr::local(rhs)];
            let invocation = RustExpr::Invoke(fn_spec, fn_args);
            Box::new(invocation)
        };
        RustClosure::from_parts(
            RustClosureHead::VarList(head_args),
            ClosureBody::Expression(body),
        )
    };
    RustExpr::Closure(closure)
}

pub(crate) fn synthesize(model: &TypedExpr<GenType>) -> RustExpr {
    match model {
        TypedExpr::ElabConst(t, typed_const) => {
            let const_val = typed_const.clone();
            let num_type = match t.try_to_num_type() {
                None => unreachable!("numeric constants must be numeric types"),
                Some(t) => Some(t),
            };
            RustExpr::ConstNum(const_val, num_type)
        }
        TypedExpr::ElabBinOp(_, op, lhs, rhs) => {
            let coerce = |t: GenType| IntType::try_from(t);
            let lhs = synthesize(lhs);
            let rhs = synthesize(rhs);
            let sig = op
                .sig
                .clone()
                .try_map_type(&coerce)
                .expect("failed to coerce binop signature");
            match classify_binary(sig) {
                class @ (BinOpClass::Pure(..)
                | BinOpClass::HomWide(..)
                | BinOpClass::HetWide(..)) => RustExpr::Invoke(
                    FnEntity::Specific {
                        fname: RustEntity::Local(induce_binary_fname(op.inner, class)),
                    },
                    vec![lhs, rhs],
                ),
                class @ (BinOpClass::HomLossy(t1 @ t0, t2) | BinOpClass::HetLossy(t0, t1, t2)) => {
                    RustExpr::Invoke(
                        FnEntity::Synthetic {
                            fname: RustEntity::Local(Label::Borrowed(SYNTHETIC_BINOP)),
                            type_params: vec![
                                NumType::from(t0),
                                NumType::from(t1),
                                NumType::from(t2),
                            ],
                        },
                        vec![
                            lhs,
                            rhs,
                            RustExpr::PrimitiveLit(RustPrimLit::String(induce_binary_fname(
                                op.inner, class,
                            ))),
                            synthesize_binop(op.inner),
                        ],
                    )
                }
            }
        }
        TypedExpr::ElabUnaryOp(_, op, input) => {
            let input = synthesize(input);
            let coerce = |t: GenType| IntType::try_from(t);
            let sig = op
                .sig
                .clone()
                .try_map_type(&coerce)
                .expect("failed to coerce unaryop signature");
            match classify_unary(Some(op.inner.get_op()), sig) {
                class @ (UnaryOpClass::Pure(..) | UnaryOpClass::NonLossy(..)) => RustExpr::Invoke(
                    FnEntity::Specific {
                        fname: RustEntity::Local(induce_unary_fname(op.inner, class)),
                    },
                    vec![input],
                ),
                class @ UnaryOpClass::Lossy(t0, t1) => RustExpr::Invoke(
                    FnEntity::Synthetic {
                        fname: RustEntity::Local(Label::Borrowed(SYNTHETIC_UNARY)),
                        type_params: vec![NumType::from(t0), NumType::from(t1)],
                    },
                    vec![
                        input,
                        RustExpr::PrimitiveLit(RustPrimLit::String(induce_unary_fname(op.inner, class))),
                        synthesize_unary(op.inner),
                    ],
                ),
            }
        }
        TypedExpr::ElabCast(_, cast, input) => {
            let input = synthesize(input);
            let coerce = |t: GenType| IntType::try_from(t);
            let sig = cast
                .sig
                .clone()
                .try_map_type(&coerce)
                .expect("failed to coerce cast signature");
            match classify_unary(None, sig) {
                // NOTE - we avoid function stubbing for no-op casts (i.e. T -> T)
                UnaryOpClass::Pure(_) => input,
                class @ (UnaryOpClass::Lossy(..) | UnaryOpClass::NonLossy(..)) => RustExpr::Invoke(
                    FnEntity::Specific {
                        fname: RustEntity::Local(induce_cast_fname(class)),
                    },
                    vec![input],
                ),
            }
        }
    }
}
