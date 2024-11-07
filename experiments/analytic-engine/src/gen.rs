use std::borrow::Cow;

use crate::core::{BasicBinOp, BasicUnaryOp, BinOp, NumRep, TypedConst, UnaryOp};
use crate::elaborator::{IntType, PrimInt, Sig1, Sig2, TypedBinOp, TypedExpr, TypedUnaryOp};
use crate::printer::{
    fragment::Fragment,
    precedence::{cond_paren, Precedence},
};

pub(crate) mod ast {
    use crate::{core::TypedConst, elaborator::PrimInt};

    pub type Label = std::borrow::Cow<'static, str>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SType {
        RustPrimInt(PrimInt),
    }

    #[derive(Clone, Debug)]
    pub enum AstEntity {
        Unqualified(Label),
        Qualified(Vec<Label>, Label),
    }

    impl From<Label> for AstEntity {
        fn from(l: Label) -> Self {
            AstEntity::Unqualified(l)
        }
    }

    #[derive(Clone, Debug)]
    pub(crate) enum FnEntity {
        Specific {
            fname: AstEntity,
        },
        Synthetic {
            fname: AstEntity,
            type_params: Vec<SType>,
        },
        Closure(AstClosure),
    }

    #[derive(Clone, Debug)]
    pub enum AstValue {
        Var(Label),
        StringLit(Label),
        Const(TypedConst, Option<SType>),
    }

    #[derive(Clone, Debug)]
    pub struct AstClosure {
        pub(super) head_args: Vec<Label>,
        pub(super) body: Box<AstExpr>,
    }

    #[derive(Clone, Debug)]
    pub enum AstExpr {
        Value(AstValue),
        Invoke(FnEntity, Vec<AstExpr>),
        ForceEval(Box<AstExpr>),
        Closure(AstClosure),
    }
}

use ast::{AstClosure, AstEntity, AstExpr, AstValue, FnEntity, Label, SType};

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
    Wide(T, T),
    Lossy(T, T),
}

fn classify_binary(sig: Sig2<IntType>) -> BinOpClass<PrimInt> {
    let ((l, r), o) = sig;
    let l = l.to_prim();
    let r = r.to_prim();
    let o = o.to_prim();

    let lrep = NumRep::from(l);
    let rrep = NumRep::from(r);
    let orep = NumRep::from(o);

    if lrep == rrep {
        if orep == lrep {
            BinOpClass::Pure(l)
        } else if orep.encompasses(&lrep) {
            BinOpClass::HomWide(l, o)
        } else {
            BinOpClass::HomLossy(l, o)
        }
    } else if orep.encompasses(&lrep) && orep.encompasses(&rrep) {
        BinOpClass::HetWide(l, r, o)
    } else {
        BinOpClass::HetLossy(l, r, o)
    }
}

fn classify_unary(sig: Sig1<IntType>) -> UnaryOpClass<PrimInt> {
    let (i, o) = sig;
    let i = i.to_prim();
    let o = o.to_prim();

    let irep = NumRep::from(i);
    let orep = NumRep::from(o);

    if irep == orep {
        UnaryOpClass::Pure(i)
    } else if orep.encompasses(&irep) {
        UnaryOpClass::Wide(i, o)
    } else {
        UnaryOpClass::Lossy(i, o)
    }
}

pub(crate) const SYNTHETIC_BINOP: &str = "eval_fallback";
pub(crate) const SYNTHETIC_UNARY: &str = "eval_unary_fallback";
pub(crate) const UNSIGNED_HOM_ABS: &str = "abs_noop";
pub(crate) const SYNTHETIC_CAST: &str = "cast_fallback";

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
    };
    let str = match class {
        UnaryOpClass::Pure(t) => format!("{}_{}", base, t.to_static_str()),
        UnaryOpClass::Wide(t0, t1) | UnaryOpClass::Lossy(t0, t1) => {
            format!("{}_{}_{}", base, t0.to_static_str(), t1.to_static_str())
        }
    };
    Label::Owned(str)
}

pub fn synthesize_unary(op: UnaryOp) -> AstExpr {
    let ent = {
        let (qual, meth) = match op.get_op() {
            BasicUnaryOp::Negate => ("Neg", "neg"),
            BasicUnaryOp::AbsVal => ("Signed", "abs"),
        };
        AstEntity::Qualified(vec![Label::Borrowed(qual)], Label::Borrowed(meth))
    };
    let closure = {
        let input = Label::Borrowed("x");
        let head_args = vec![input.clone()];
        let body = {
            let fn_spec = FnEntity::Specific { fname: ent };
            let fn_args = vec![AstExpr::Value(AstValue::Var(input))];
            let invocation = AstExpr::Invoke(fn_spec, fn_args);
            Box::new(invocation)
        };
        AstClosure { head_args, body }
    };
    AstExpr::Closure(closure)
}

pub fn synthesize_binop(op: BinOp) -> AstExpr {
    let ent = {
        let qual = Label::Borrowed("BigInt");
        let meth = match op.get_op() {
            BasicBinOp::Add => "checked_add",
            BasicBinOp::Sub => "checked_sub",
            BasicBinOp::Mul => "checked_mul",
            BasicBinOp::Div => "checked_div",
            BasicBinOp::Rem => "checked_rem",
        };
        AstEntity::Qualified(vec![qual], Label::Borrowed(meth))
    };
    let closure = {
        let lhs = Label::Borrowed("x");
        let rhs = Label::Borrowed("y");
        let head_args = vec![lhs.clone(), rhs.clone()];
        let body = {
            let fn_spec = FnEntity::Specific { fname: ent };
            let fn_args = vec![
                AstExpr::Value(AstValue::Var(lhs)),
                AstExpr::Value(AstValue::Var(rhs)),
            ];
            let invocation = AstExpr::Invoke(fn_spec, fn_args);
            Box::new(invocation)
        };
        AstClosure { head_args, body }
    };
    AstExpr::Closure(closure)
}

pub(crate) fn synthesize(model: &TypedExpr<IntType>) -> ast::AstExpr {
    match model {
        TypedExpr::ElabConst(t, typed_const) => AstExpr::Value(AstValue::Const(
            typed_const.clone(),
            Some(ast::SType::RustPrimInt(t.to_prim())),
        )),
        TypedExpr::ElabBinOp(_, op, lhs, rhs) => {
            let lhs = synthesize(lhs);
            let rhs = synthesize(rhs);
            match classify_binary(op.sig) {
                class @ (BinOpClass::Pure(..)
                | BinOpClass::HomWide(..)
                | BinOpClass::HetWide(..)) => AstExpr::Invoke(
                    FnEntity::Specific {
                        fname: induce_binary_fname(op.inner, class).into(),
                    },
                    vec![lhs, rhs],
                ),
                class @ (BinOpClass::HomLossy(t1 @ t0, t2) | BinOpClass::HetLossy(t0, t1, t2)) => {
                    AstExpr::Invoke(
                        FnEntity::Synthetic {
                            fname: Label::Borrowed(SYNTHETIC_BINOP).into(),
                            type_params: vec![
                                SType::RustPrimInt(t0),
                                SType::RustPrimInt(t1),
                                SType::RustPrimInt(t2),
                            ],
                        },
                        vec![
                            lhs,
                            rhs,
                            AstExpr::Value(AstValue::StringLit(induce_binary_fname(
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
            match classify_unary(op.sig) {
                // FIXME - add in cases for predefined unary ops where applicable, as they are implemented
                UnaryOpClass::Pure(t)
                    if t.is_unsigned() && matches!(op.inner.get_op(), BasicUnaryOp::AbsVal) =>
                {
                    // NOTE - we can use the `abs_noop` function for unsigned abs that preserves type
                    AstExpr::Invoke(
                        FnEntity::Specific {
                            fname: Label::Borrowed(UNSIGNED_HOM_ABS).into(),
                        },
                        vec![input],
                    )
                }
                class @ (UnaryOpClass::Pure(t1 @ t0)
                | UnaryOpClass::Wide(t0, t1)
                | UnaryOpClass::Lossy(t0, t1)) => {
                    // FIXME - refine the case guard and body as we add in specific unary operations (besides `abs_noop`)
                    AstExpr::Invoke(
                        FnEntity::Synthetic {
                            fname: AstEntity::Unqualified(Label::Borrowed(SYNTHETIC_UNARY)),
                            type_params: vec![SType::RustPrimInt(t0), SType::RustPrimInt(t1)],
                        },
                        vec![
                            input,
                            AstExpr::Value(AstValue::StringLit(induce_unary_fname(
                                op.inner, class,
                            ))),
                            synthesize_unary(op.inner),
                        ],
                    )
                }
            }
        }
        TypedExpr::ElabCast(_, cast, input) => {
            let input = synthesize(input);
            match classify_unary(cast.sig) {
                // NOTE - we avoid function stubbing for truly noop casts (c.f. abs_noop where we *do* want to keep a record of the operation)
                UnaryOpClass::Pure(_) => input,
                UnaryOpClass::Lossy(t0, t1) | UnaryOpClass::Wide(t0, t1) => AstExpr::Invoke(
                    FnEntity::Synthetic {
                        fname: AstEntity::Unqualified(Label::Borrowed(SYNTHETIC_CAST)),
                        type_params: vec![SType::RustPrimInt(t0), SType::RustPrimInt(t1)],
                    },
                    vec![input],
                ),
            }
        }
    }
}

pub(crate) trait ToFragment {
    fn to_fragment(&self) -> Fragment;
}

impl ToFragment for AstClosure {
    fn to_fragment(&self) -> Fragment {
        let head_args = Fragment::seq(
            self.head_args.iter().cloned().map(Fragment::String),
            Some(Fragment::String(Cow::Borrowed(", "))),
        );
        let preamble = head_args.delimit(Fragment::Char('|'), Fragment::Char('|'));
        let body = self
            .body
            .to_fragment()
            .delimit(Fragment::Char('{'), Fragment::Char('}'));
        preamble.cat(Fragment::Char(' ')).cat(body)
    }
}

impl ToFragment for AstEntity {
    fn to_fragment(&self) -> Fragment {
        match self {
            AstEntity::Unqualified(lbl) => Fragment::String(lbl.clone()),
            AstEntity::Qualified(path_elts, lbl) => Fragment::seq(
                path_elts
                    .iter()
                    .chain(std::iter::once(lbl))
                    .cloned()
                    .map(Fragment::String),
                Some(Fragment::String(Cow::Borrowed("::"))),
            ),
        }
    }
}

impl ToFragment for AstValue {
    fn to_fragment(&self) -> Fragment {
        match self {
            AstValue::StringLit(s) => {
                Fragment::String(s.clone()).delimit(Fragment::Char('"'), Fragment::Char('"'))
            }
            AstValue::Var(vname) => Fragment::String(vname.clone()),
            AstValue::Const(typed_const, stype) => Fragment::String(Cow::Owned(format!(
                "{}{}",
                typed_const.as_raw_value(),
                match stype {
                    None => "",
                    Some(t) => match t {
                        SType::RustPrimInt(prim_int) => prim_int.to_static_str(),
                    },
                }
            ))),
        }
    }
}

impl ToFragment for FnEntity {
    fn to_fragment(&self) -> Fragment {
        match self {
            FnEntity::Specific { fname } => fname.to_fragment(),
            FnEntity::Synthetic { fname, type_params } => fname.to_fragment().cat(
                Fragment::seq(
                    type_params.iter().map(|t| match t {
                        SType::RustPrimInt(p) => Fragment::String(Cow::Borrowed(p.to_static_str())),
                    }),
                    Some(Fragment::String(Cow::Borrowed(", "))),
                )
                .delimit(Fragment::Char('<'), Fragment::Char('>')),
            ),
            FnEntity::Closure(ast_closure) => ast_closure
                .to_fragment()
                .delimit(Fragment::Char('('), Fragment::Char(')')),
        }
    }
}

impl ToFragment for AstExpr {
    fn to_fragment(&self) -> Fragment {
        match self {
            AstExpr::Closure(closure) => closure.to_fragment(),
            AstExpr::Invoke(fn_spec, args) => {
                let args = args.iter().map(ToFragment::to_fragment);
                let paren_list = Fragment::seq(
                    args,
                    Some(Fragment::String(std::borrow::Cow::Borrowed(", "))),
                )
                .delimit(Fragment::Char('('), Fragment::Char(')'));
                let fn_spec = fn_spec.to_fragment();
                fn_spec.cat(paren_list)
            }
            AstExpr::Value(ast_value) => ast_value.to_fragment(),
            // FIXME - think about this a bit more since it is a bit fiddly
            AstExpr::ForceEval(ast_expr) => ast_expr
                .to_fragment()
                .cat(Fragment::String(Cow::Borrowed(".eval()?"))),
        }
    }
}
