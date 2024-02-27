mod rust_ast;
mod typed_format;

use crate::{
    byte_set::ByteSet,
    decoder::{Decoder, Program},
    typecheck::{TypeChecker, UScope, UVar},
    Arith, BaseType, DynFormat, Expr, Format, FormatModule, IntRel, Label, MatchTree, Pattern,
    ValueType,
};

use std::{collections::HashMap, rc::Rc};

use rust_ast::*;

use typed_format::{GenType, TypedExpr, TypedFormat, TypedPattern};

use self::typed_format::TypedDynFormat;

/// Simple type for ad-hoc names using a counter value
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
struct IxLabel(usize);

impl IxLabel {
    fn to_usize(self) -> usize {
        self.0
    }
}

pub struct NameGen {
    ctr: usize,
    revmap: HashMap<RustTypeDef, IxLabel>,
}

impl NameGen {
    fn new() -> Self {
        Self {
            ctr: 0,
            revmap: HashMap::new(),
        }
    }

    /// Finds or generates a new name for a RustTypeDef.
    ///
    /// Returns `(old, (ix, false))` if the RustTypeDef was already in-scope with name `old` and index `ix``
    /// Returns `(new, (ix, true))` otherwise, where `ix` is the new index for the RustTypeDef, and `new` is a novel name
    fn get_name(&mut self, def: &RustTypeDef) -> (Label, (usize, bool)) {
        match self.revmap.get(def) {
            Some(ixlab) => (ixlab.into(), (ixlab.to_usize(), false)),
            None => {
                let ix = self.ctr;
                let ixlab = IxLabel::from(ix);
                self.ctr += 1;
                self.revmap.insert(def.clone(), ixlab);
                (ixlab.into(), (ix, true))
            }
        }
    }
}

pub struct Codegen {
    namegen: NameGen,
    defined_types: Vec<RustTypeDef>,
    decoder_types: Vec<GenType>,
}

impl Codegen {
    pub fn new() -> Self {
        let namegen = NameGen::new();
        let defined_types = Vec::new();
        let decoder_types = Vec::new();
        Codegen {
            namegen,
            defined_types,
            decoder_types,
        }
    }

    pub(crate) fn substitute_definition(&self, t: GenType) -> GenType {
        if let GenType::Inline(RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(ix, lbl)))) = t
        {
            GenType::Def((ix, lbl), self.defined_types[ix].clone())
        } else {
            t
        }
    }

    /// Converts a `ValueType` to a `RustType`, potentially creating new ad-hoc names
    /// for any records or unions encountered, and registering any new ad-hoc type definitions
    /// in `self`.
    fn lift_type(&mut self, vt: &ValueType) -> GenType {
        match vt {
            ValueType::Empty => RustType::UNIT.into(),
            ValueType::Base(BaseType::Bool) => PrimType::Bool.into(),
            ValueType::Base(BaseType::U8) => PrimType::U8.into(),
            ValueType::Base(BaseType::U16) => PrimType::U16.into(),
            ValueType::Base(BaseType::U32) => PrimType::U32.into(),
            ValueType::Base(BaseType::U64) => PrimType::U64.into(),
            ValueType::Base(BaseType::Char) => PrimType::Char.into(),
            ValueType::Tuple(vs) => {
                let mut buf = Vec::with_capacity(vs.len());
                for v in vs.iter() {
                    buf.push(self.lift_type(v).to_rust_type());
                }
                RustType::AnonTuple(buf).into()
            }
            ValueType::Seq(t) => {
                let inner = self.lift_type(t.as_ref()).to_rust_type();
                CompType::Vec(Box::new(inner)).into()
            }
            ValueType::Any => panic!("ValueType::Any"),
            ValueType::Record(fields) => {
                let mut rt_fields = Vec::new();
                for (lab, ty) in fields.iter() {
                    let rt_field = self.lift_type(ty);
                    rt_fields.push((lab.clone(), rt_field.to_rust_type()));
                }
                let rtdef = RustTypeDef::Struct(RustStruct::Record(rt_fields));
                let (tname, (ix, is_new)) = self.namegen.get_name(&rtdef);
                if is_new {
                    self.defined_types.push(rtdef.clone());
                }
                GenType::Def((ix, tname), rtdef)
            }
            ValueType::Union(vars) => {
                let mut rt_vars = Vec::new();
                for (vname, vdef) in vars.iter() {
                    let rt_var = match vdef {
                        ValueType::Empty => RustVariant::Unit(vname.clone()),
                        ValueType::Tuple(args) => {
                            if args.is_empty() {
                                RustVariant::Unit(vname.clone())
                            } else {
                                let mut rt_args = Vec::new();
                                for arg in args.iter() {
                                    rt_args.push(self.lift_type(arg).to_rust_type());
                                }
                                RustVariant::Tuple(vname.clone(), rt_args)
                            }
                        }
                        /* ValueType::Record(fields) => {
                            let mut rt_fields = Vec::new();
                            for (f_lab, f_ty) in fields.iter() {
                                rt_fields.push((f_lab.clone(), self.lift_type(f_ty)));
                            }
                            RustVariant::Record(vname.clone(), rt_fields)
                        } */
                        other => {
                            let inner = self.lift_type(other).to_rust_type();
                            RustVariant::Tuple(vname.clone(), vec![inner])
                        }
                    };
                    rt_vars.push(rt_var);
                }
                let rtdef = RustTypeDef::Enum(rt_vars);
                let (tname, (ix, is_new)) = self.namegen.get_name(&rtdef);
                if is_new {
                    self.defined_types.push(rtdef.clone());
                }
                GenType::Def((ix, tname), rtdef)
                // RustType::defined(ix, tname)
            }
        }
    }

    pub fn populate_decoder_types(&mut self, program: &Program) {
        self.decoder_types = Vec::with_capacity(program.decoders.len());
        for (_d, t) in &program.decoders {
            let r = self.lift_type(t);
            self.decoder_types.push(r);
        }
    }

    fn translate(&self, decoder: &Decoder, type_hint: Option<&GenType>) -> CaseLogic {
        match decoder {
            Decoder::Call(ix, args) => CaseLogic::Simple(SimpleLogic::Invoke(*ix, args.clone())),
            Decoder::Fail => CaseLogic::Simple(SimpleLogic::Fail),
            Decoder::EndOfInput => CaseLogic::Simple(SimpleLogic::ExpectEnd),
            Decoder::Align(n) => CaseLogic::Simple(SimpleLogic::SkipToNextMultiple(*n)),
            Decoder::Byte(bs) => CaseLogic::Simple(SimpleLogic::ByteIn(*bs)),
            Decoder::Variant(vname, inner) => {
                let (tname, tdef) = match type_hint {
                    Some(
                        GenType::Def((ix, lab), ..)
                        | GenType::Inline(RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(
                            ix,
                            lab,
                        )))),
                    ) => (lab.clone(), &self.defined_types[*ix]),
                    Some(other) => panic!("unexpected type_hint for Decoder::Variant: {:?}", other),
                    _ => unreachable!("must have type_hint to translate Decoder::Variant"),
                };

                let constr = Constructor::Compound(tname.clone(), vname.clone());

                match tdef {
                    RustTypeDef::Enum(vars) => {
                        let matching = vars
                            .iter()
                            .find(|var| var.get_label().as_ref() == vname.as_ref());
                        // REVIEW - should we force an exact match?
                        match matching {
                            Some(RustVariant::Unit(_)) => {
                                // FIXME - this is not quite correct, as it calls `Var(inner)` where `inner = ()` instead of `Var`
                                let inner = self.translate(inner, Some(&GenType::Inline(RustType::UNIT)));
                                CaseLogic::Derived(
                                    DerivedLogic::UnitVariantOf(constr, Box::new(inner))
                                )
                            }
                            Some(RustVariant::Tuple(_, typs)) => {
                                if typs.is_empty() {
                                    unreachable!(
                                        "unexpected Tuple-Variant with 0 positional arguments"
                                    );
                                }
                                match inner.as_ref() {
                                    Decoder::Tuple(decs) => {
                                        if decs.len() != typs.len() {
                                            if typs.len() == 1 {
                                                // REVIEW - allowance for 1-tuple variant whose argument type is itself an n-tuple
                                                match &typs[0] {
                                                    tt @ RustType::AnonTuple(..) => {
                                                        let cl_mono_tuple = self.translate(
                                                            inner,
                                                            Some(&tt.clone().into())
                                                        );
                                                        CaseLogic::Derived(
                                                            DerivedLogic::VariantOf(
                                                                constr,
                                                                Box::new(cl_mono_tuple)
                                                            )
                                                        )
                                                    }
                                                    other =>
                                                        panic!(
                                                            "unable to translate Decoder::Tuple with hint ({other:?}) implied by {tname}::{vname}"
                                                        ),
                                                }
                                            } else {
                                                unreachable!(
                                                    "mismatched arity between decoder (== {}) and variant {tname}::{vname} (== {})",
                                                    decs.len(),
                                                    typs.len()
                                                );
                                            }
                                        } else {
                                            let mut cl_args = Vec::new();
                                            for (dec, typ) in Iterator::zip(decs.iter(), typs.iter()) {
                                                let cl_arg = self.translate(dec, Some(&typ.clone().into()));
                                                cl_args.push(cl_arg);
                                            }
                                            CaseLogic::Sequential(SequentialLogic::AccumTuple {
                                                constructor: Some(constr),
                                                elements: cl_args,
                                            })
                                        }
                                    }
                                    _ => {
                                        if typs.len() == 1 {
                                            let cl_mono = self.translate(inner, Some(&typs[0].clone().into()));
                                            CaseLogic::Derived(
                                                DerivedLogic::VariantOf(constr, Box::new(cl_mono))
                                            )
                                        } else {
                                            panic!(
                                                "Variant {tname}::{vname}({typs:#?}) mismatches non-tuple Decoder {inner:?}"
                                            );
                                        }
                                    }
                                }
                            }
                            Some(RustVariant::Record(_, fields)) => {
                                match inner.as_ref() {
                                    Decoder::Record(inner_fields) => {
                                        let mut assocs = Vec::new();
                                        for (i, (l0, d)) in inner_fields.iter().enumerate() {
                                            let (l1, t) = &fields[i];
                                            assert_eq!(
                                                l0.as_ref(),
                                                l1.as_ref(),
                                                "Decoder field `{l0}` != RustTypeDef field `{l1}` (at index {i} in {decoder:?} | {tdef:?})"
                                            );
                                            assocs.push((l0.clone(), self.translate(d, Some(&t.clone().into()))));
                                        }
                                        CaseLogic::Sequential(SequentialLogic::AccumRecord {
                                            constructor: constr,
                                            fields: assocs,
                                        })
                                    }
                                    _ =>
                                        unreachable!(
                                            "Variant {tname}::{vname} expects record ({fields:#?}) but found {:?}",
                                            inner
                                        ),
                                }
                            }
                            None =>
                                unreachable!(
                                    "VariantOf called for nonexistent variant `{vname}` of enum-type `{tname}`"
                                ),
                        }
                    }
                    RustTypeDef::Struct(_) => {
                        unreachable!("Decoder::Variant incoherent against type defined as struct")
                    }
                }
            }
            Decoder::Parallel(alts) => CaseLogic::Parallel(ParallelLogic::Alts(
                alts.iter()
                    .map(|alt| self.translate(alt, type_hint))
                    .collect(),
            )),
            Decoder::Branch(tree, flat) => CaseLogic::Other(OtherLogic::Descend(
                tree.clone(),
                flat.iter()
                    .map(|alt| self.translate(alt, type_hint))
                    .collect(),
            )),
            Decoder::Tuple(elts) => match type_hint {
                None => CaseLogic::Sequential(SequentialLogic::AccumTuple {
                    constructor: None,
                    elements: elts.iter().map(|elt| self.translate(elt, None)).collect(),
                }),
                Some(GenType::Inline(RustType::AnonTuple(tys))) => {
                    CaseLogic::Sequential(SequentialLogic::AccumTuple {
                        constructor: None,
                        elements: elts
                            .iter()
                            .zip(tys)
                            .map(|(elt, ty)| self.translate(elt, Some(&ty.clone().into())))
                            .collect(),
                    })
                }
                Some(GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::Unit))))
                    if elts.is_empty() =>
                {
                    CaseLogic::Simple(SimpleLogic::Eval(RustExpr::UNIT))
                }
                Some(other) => unreachable!(
                    "Decoder::Tuple expected to have type RustType::AnonTuple(..), found {:?}",
                    other
                ),
            },
            Decoder::Record(flds) => {
                match type_hint {
                    Some(GenType::Def((ix, lab), ..) | GenType::Inline(RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(ix, lab))))) => {
                        let tdef = &self.defined_types[*ix];
                        let tfields = match tdef {
                            RustTypeDef::Struct(RustStruct::Record(tfields)) => tfields,
                            _ =>
                                unreachable!(
                                    "unexpected non-Struct::Record definition for type `{lab}`: {tdef:?}"
                                ),
                        };
                        // FIXME - for now we rely on a consistent order of field names between the decoder and type definition
                        let mut assocs = Vec::new();
                        for (i, (l0, d)) in flds.iter().enumerate() {
                            let (l1, t) = &tfields[i];
                            assert_eq!(
                                l0.as_ref(),
                                l1.as_ref(),
                                "Decoder field `{l0}` != RustTypeDef field `{l1}` (at index {i} in {decoder:?} | {tdef:?})"
                            );
                            assocs.push((l0.clone(), self.translate(d, Some(&t.clone().into()))));
                        }
                        CaseLogic::Sequential(SequentialLogic::AccumRecord {
                            constructor: Constructor::Simple(lab.clone()),
                            fields: assocs,
                        })
                    }

                    None =>
                        unreachable!(
                            "Cannot generate CaseLogic for a Record without a definite type-name"
                        ),
                    Some(other) =>
                        unreachable!(
                            "Decoder::Record expected to have type RustType::Atom(AtomType::TypeRef(..)), found {:?}",
                            other
                        ),
                }
            }
            Decoder::While(tree_continue, single) => match type_hint {
                Some(GenType::Inline(RustType::Atom(AtomType::Comp(CompType::Vec(t))))) => {
                    let cl_single = self.translate(single, Some(&t.as_ref().clone().into()));
                    CaseLogic::Repeat(RepeatLogic::ContinueOnMatch(
                        tree_continue.clone(),
                        Box::new(cl_single),
                    ))
                }
                Some(other) => {
                    unreachable!("Hint for Decoder::While should be Vec<_>, found {other:?}")
                }
                None => {
                    let cl_single = self.translate(single, None);
                    CaseLogic::Repeat(RepeatLogic::ContinueOnMatch(
                        tree_continue.clone(),
                        Box::new(cl_single),
                    ))
                }
            },
            Decoder::Until(tree_break, single) => match type_hint {
                Some(GenType::Inline(RustType::Atom(AtomType::Comp(CompType::Vec(t))))) => {
                    let cl_single = self.translate(single, Some(&t.as_ref().clone().into()));
                    CaseLogic::Repeat(RepeatLogic::BreakOnMatch(
                        tree_break.clone(),
                        Box::new(cl_single),
                    ))
                }
                Some(other) => {
                    unreachable!("Hint for Decoder::Until should be Vec<_>, found {other:?}")
                }
                None => {
                    let cl_single = self.translate(single, None);
                    CaseLogic::Repeat(RepeatLogic::BreakOnMatch(
                        tree_break.clone(),
                        Box::new(cl_single),
                    ))
                }
            },
            Decoder::RepeatCount(expr_count, single) => match type_hint {
                Some(GenType::Inline(RustType::Atom(AtomType::Comp(CompType::Vec(t))))) => {
                    let cl_single = self.translate(single, Some(&t.as_ref().clone().into()));
                    CaseLogic::Repeat(RepeatLogic::ExactCount(
                        embed_expr(expr_count),
                        Box::new(cl_single),
                    ))
                }
                Some(other) => {
                    unreachable!("Hint for Decoder::RepeatCount should be Vec<_>, found {other:?}")
                }
                None => {
                    let cl_single = self.translate(single, None);
                    CaseLogic::Repeat(RepeatLogic::ExactCount(
                        embed_expr(expr_count),
                        Box::new(cl_single),
                    ))
                }
            },
            Decoder::RepeatUntilLast(pred_terminal, single) => match type_hint {
                Some(GenType::Inline(RustType::Atom(AtomType::Comp(CompType::Vec(t))))) => {
                    let cl_single = self.translate(single, Some(&t.as_ref().clone().into()));
                    CaseLogic::Repeat(RepeatLogic::ConditionTerminal(
                        embed_expr(pred_terminal),
                        Box::new(cl_single),
                    ))
                }
                Some(other) => unreachable!(
                    "Hint for Decoder::RepeatUntilLast should be Vec<_>, found {other:?}"
                ),
                None => {
                    let cl_single = self.translate(single, None);
                    CaseLogic::Repeat(RepeatLogic::ConditionTerminal(
                        embed_expr(pred_terminal),
                        Box::new(cl_single),
                    ))
                }
            },
            Decoder::RepeatUntilSeq(pred_complete, single) => match type_hint {
                Some(GenType::Inline(RustType::Atom(AtomType::Comp(CompType::Vec(t))))) => {
                    let cl_single = self.translate(single, Some(&t.as_ref().clone().into()));
                    CaseLogic::Repeat(RepeatLogic::ConditionComplete(
                        embed_expr(pred_complete),
                        Box::new(cl_single),
                    ))
                }
                Some(other) => unreachable!(
                    "Hint for Decoder::RepeatUntilSeq should be Vec<_>, found {other:?}"
                ),
                None => {
                    let cl_single = self.translate(single, None);
                    CaseLogic::Repeat(RepeatLogic::ConditionComplete(
                        embed_expr(pred_complete),
                        Box::new(cl_single),
                    ))
                }
            },
            // FIXME - implement CaseLogic variants and translation rules for the remaining cases
            Decoder::Map(inner, f) => {
                // FIXME - we have no way of inferring a proper type-hint for inner
                let cl_inner = self.translate(inner, None);
                CaseLogic::Derived(DerivedLogic::MapOf(embed_expr(f), Box::new(cl_inner)))
            }
            Decoder::Compute(expr) => CaseLogic::Simple(SimpleLogic::Eval(embed_expr(expr))),
            Decoder::Let(name, expr, inner) => {
                let cl_inner = self.translate(inner, type_hint);
                CaseLogic::Derived(DerivedLogic::Let(
                    name.clone(),
                    embed_expr(expr),
                    Box::new(cl_inner),
                ))
            }
            Decoder::Match(scrutinee, cases) => {
                let mut cl_cases = Vec::new();
                for (pat, dec) in cases.iter() {
                    cl_cases.push((
                        // FIXME - add type_hint for scrutinee when possible
                        MatchCaseLHS::Pattern(embed_pattern(pat, None)),
                        self.translate(dec, type_hint),
                    ));
                }
                CaseLogic::Other(OtherLogic::ExprMatch(embed_expr(scrutinee), cl_cases))
            }
            Decoder::Dynamic(_lab, _f_dyn, _inner) => {
                CaseLogic::Unhandled("translate @ Decoder::Dynamic".into())
            }
            Decoder::Apply(_lab) => CaseLogic::Unhandled("translate @ Decoder::Apply".into()),
            Decoder::Peek(_inner) => CaseLogic::Unhandled("translate @ Decoder::Peek".into()),
            Decoder::PeekNot(_inner) => CaseLogic::Unhandled("translate @ Decoder::PeekNot".into()),
            Decoder::Slice(_width, _inner) => {
                CaseLogic::Unhandled("translate @ Decoder::Slice".into())
            }
            Decoder::Bits(_dec_bits) => CaseLogic::Unhandled("translate @ Decoder::Bits".into()),
            Decoder::WithRelativeOffset(_offset, _inner) => {
                CaseLogic::Unhandled("translate @ Decoder::WithRelativeOffset".into())
            }
        }
    }
}

fn get_enum_name(typ: &RustType) -> &Label {
    match typ {
        RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(_, name))) => name,
        other => unreachable!("get_enum_name: non-LocalDef type {other:?}"),
    }
}

fn embed_pattern(pat: &Pattern, type_hint: Option<&RustType>) -> RustPattern {
    match pat {
        Pattern::Wildcard => RustPattern::CatchAll(None),
        Pattern::Binding(name) => RustPattern::CatchAll(Some(name.clone())),
        Pattern::Bool(b) => RustPattern::PrimLiteral(RustPrimLit::Boolean(*b)),
        Pattern::U8(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(*n as usize)),
        Pattern::U16(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(*n as usize)),
        Pattern::U32(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(*n as usize)),
        Pattern::U64(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(*n as usize)),
        Pattern::Char(c) => RustPattern::PrimLiteral(RustPrimLit::Char(*c)),
        Pattern::Tuple(pats) => {
            RustPattern::TupleLiteral(pats.iter().map(|x| embed_pattern(x, None)).collect())
        }
        Pattern::Seq(pats) => {
            RustPattern::ArrayLiteral(pats.iter().map(|x| embed_pattern(x, None)).collect())
        }
        Pattern::Variant(vname, pat) => {
            let constr = match type_hint {
                Some(ty) => {
                    let tname = get_enum_name(ty).clone();
                    Constructor::Compound(tname, vname.clone())
                }
                // FIXME - figure out a way to get around this
                None => Constructor::Simple(vname.clone()),
            };
            let inner = embed_pattern(pat, None);
            RustPattern::Variant(constr, Box::new(inner))
        }
    }
}

fn embed_expr(expr: &Expr) -> RustExpr {
    match expr {
        Expr::Var(vname) => {
            // FIXME - as currently implemented, the scoping is almost certainly not implemented to reference local assignments properly
            RustExpr::local(vname.clone())
        }
        Expr::Bool(b) => RustExpr::PrimitiveLit(RustPrimLit::Boolean(*b)),
        Expr::U8(n) => RustExpr::num_lit(*n as usize),
        Expr::U16(n) => RustExpr::num_lit(*n as usize),
        Expr::U32(n) => RustExpr::num_lit(*n as usize),
        Expr::U64(n) => RustExpr::num_lit(*n as usize),
        Expr::Tuple(tup) => RustExpr::Tuple(tup.iter().map(embed_expr).collect()),
        Expr::TupleProj(expr_tup, ix) => embed_expr(expr_tup).nth(*ix),
        Expr::Record(_fields) => unreachable!("Record not bound in Variant"),
        Expr::RecordProj(expr_rec, fld) => embed_expr(expr_rec).field(fld.clone()),
        Expr::Variant(vname, inner) => match inner.as_ref() {
            Expr::Record(fields) => RustExpr::Struct(
                RustEntity::Local(vname.clone()),
                fields
                    .iter()
                    .map(|(fname, fval)| (fname.clone(), Some(Box::new(embed_expr(fval)))))
                    .collect(),
            ),
            _ => RustExpr::local(vname.clone()).call_with([embed_expr(inner)]),
        },
        Expr::Seq(elts) => {
            RustExpr::ArrayLit(elts.iter().map(embed_expr).collect()).call_method("to_vec")
        }
        Expr::Match(scrutinee, cases) => RustExpr::Control(Box::new(RustControl::Match(
            embed_expr(scrutinee),
            cases
                .iter()
                .map(|(pat, rhs)| {
                    (
                        // FIXME - add actual type_hint when possible
                        MatchCaseLHS::Pattern(embed_pattern(pat, None)),
                        vec![RustStmt::Return(ReturnKind::Implicit, embed_expr(rhs))],
                    )
                })
                .collect(),
        ))),
        // FIXME - we probably need to apply precedence rules similar to tree-output, which will require a lot of refactoring in AST
        Expr::Arith(Arith::BitAnd, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::BitAnd, embed_expr(rhs))
        }
        Expr::Arith(Arith::BitOr, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::BitOr, embed_expr(rhs))
        }
        Expr::Arith(Arith::Mul, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Mul, embed_expr(rhs))
        }
        Expr::Arith(Arith::Div, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Div, embed_expr(rhs))
        }
        Expr::Arith(Arith::Rem, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Rem, embed_expr(rhs))
        }
        Expr::Arith(Arith::Shl, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Shl, embed_expr(rhs))
        }
        Expr::Arith(Arith::Shr, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Shr, embed_expr(rhs))
        }
        Expr::Arith(Arith::Add, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Add, embed_expr(rhs))
        }
        Expr::Arith(Arith::Sub, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Sub, embed_expr(rhs))
        }

        Expr::IntRel(IntRel::Eq, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Eq, embed_expr(rhs))
        }
        Expr::IntRel(IntRel::Ne, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Neq, embed_expr(rhs))
        }
        Expr::IntRel(IntRel::Lt, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Lt, embed_expr(rhs))
        }
        Expr::IntRel(IntRel::Gt, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Gt, embed_expr(rhs))
        }
        Expr::IntRel(IntRel::Lte, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Lte, embed_expr(rhs))
        }
        Expr::IntRel(IntRel::Gte, lhs, rhs) => {
            RustExpr::infix(embed_expr(lhs), Operator::Gte, embed_expr(rhs))
        }
        Expr::AsU8(x) => {
            RustExpr::Operation(RustOp::AsCast(Box::new(embed_expr(x)), PrimType::U8.into()))
        }
        Expr::AsU16(x) => RustExpr::Operation(RustOp::AsCast(
            Box::new(embed_expr(x)),
            PrimType::U16.into(),
        )),
        Expr::AsU32(x) => RustExpr::Operation(RustOp::AsCast(
            Box::new(embed_expr(x)),
            PrimType::U32.into(),
        )),
        Expr::AsU64(x) => RustExpr::Operation(RustOp::AsCast(
            Box::new(embed_expr(x)),
            PrimType::U64.into(),
        )),
        Expr::U16Be(be_bytes) => RustExpr::local("u16be").call_with([embed_expr(be_bytes)]),
        Expr::U16Le(le_bytes) => RustExpr::local("u16le").call_with([embed_expr(le_bytes)]),
        Expr::U32Be(be_bytes) => RustExpr::local("u32be").call_with([embed_expr(be_bytes)]),
        Expr::U32Le(le_bytes) => RustExpr::local("u32le").call_with([embed_expr(le_bytes)]),
        Expr::U64Be(be_bytes) => RustExpr::local("u64be").call_with([embed_expr(be_bytes)]),
        Expr::U64Le(le_bytes) => RustExpr::local("u64le").call_with([embed_expr(le_bytes)]),
        Expr::AsChar(codepoint) => RustExpr::scoped(["char"], "from_u32")
            .call_with([embed_expr(codepoint)])
            .call_method("unwrap"),
        Expr::SeqLength(seq) => embed_expr(seq).call_method("len"),
        Expr::SubSeq(seq, ix, len) => {
            let start_expr = embed_expr(ix);
            let bind_ix = RustStmt::assign("ix", start_expr);
            let end_expr = RustExpr::infix(RustExpr::local("ix"), Operator::Add, embed_expr(len));
            RustExpr::BlockScope(
                vec![bind_ix],
                Box::new(RustExpr::Slice(
                    Box::new(embed_expr(seq)),
                    Box::new(RustExpr::local("ix")),
                    Box::new(end_expr),
                )),
            )
        }
        Expr::FlatMap(f, seq) => embed_expr(seq)
            .call_method("into_iter")
            .call_method_with("flat_map", [embed_expr(f)])
            .call_method("collect"),
        Expr::FlatMapAccum(f, acc_init, _acc_type, seq) => embed_expr(seq)
            .call_method("into_iter")
            .call_method_with("fold", [embed_expr(acc_init), embed_expr(f)])
            .call_method("collect"),
        Expr::Dup(n, expr) => RustExpr::scoped(["Vec"], "from_iter").call_with([RustExpr::scoped(
            ["std", "iter"],
            "repeat",
        )
        .call_with([embed_expr(expr)])
        .call_method_with("take", [embed_expr(n)])]),
        Expr::Inflate(_) => {
            // FIXME - not clear what the proper thing to do here is
            RustExpr::local("unimplemented!").call_with([RustExpr::str_lit(
                "embed_expr is not implemented for Expr::Inflate",
            )])
        }
        Expr::Lambda(head, body) => RustExpr::Paren(Box::new(RustExpr::Closure(
            head.clone(),
            None,
            Box::new(embed_expr(body)),
        ))),
    }
}

#[allow(dead_code)]
type RustBlock = (Vec<RustStmt>, Option<RustExpr>);

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct ProdCtxt<'a> {
    input_varname: &'a Label,
    scope_varname: &'a Label,
}

impl CaseLogic {
    /// Produces an RustExpr-valued AST for the given CaseLogic instance.
    ///
    /// The Expr should have the bare type of the value being parsed (i.e. not Option-wrapped),
    /// but it is implicitly assumed to be contained in a block whose ultimate return value
    /// is `Option<_>`, allowing `return None` and `?` expressions to be used anyway.
    ///
    /// Local bindings and control flow are allowed, as long as an explicit return
    /// or a concrete, consistently-typed return value are used
    #[allow(dead_code)]
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            CaseLogic::Simple(s) => s.to_ast(ctxt),
            CaseLogic::Derived(d) => d.to_ast(ctxt),
            CaseLogic::Sequential(sq) => sq.to_ast(ctxt),
            CaseLogic::Repeat(r) => r.to_ast(ctxt),
            CaseLogic::Parallel(p) => p.to_ast(ctxt),
            CaseLogic::Other(o) => o.to_ast(ctxt),
            CaseLogic::Unhandled(msg) => (
                Vec::new(),
                Some(RustExpr::local("unimplemented!").call_with([RustExpr::str_lit(msg.clone())])),
            ),
        }
    }
}

impl SimpleLogic {
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            SimpleLogic::Fail => (
                vec![RustStmt::Return(ReturnKind::Keyword, RustExpr::NONE)],
                None,
            ),
            SimpleLogic::ExpectEnd => {
                let call = RustExpr::local(ctxt.input_varname.clone()).call_method("read_byte");
                let cond = call.call_method("is_none");
                let b_true = [RustStmt::Return(ReturnKind::Implicit, RustExpr::UNIT)];
                let b_false = [RustStmt::Return(ReturnKind::Keyword, RustExpr::NONE)];
                (
                    Vec::new(),
                    Some(RustExpr::Control(Box::new(RustControl::If(
                        cond,
                        b_true.to_vec(),
                        Some(b_false.to_vec()),
                    )))),
                )
            }
            // FIXME - not sure what should be done with _args
            SimpleLogic::Invoke(ix_dec, _args) => {
                let fname = format!("Decoder{ix_dec}");
                let call = RustExpr::local(fname).call_with([
                    RustExpr::local(ctxt.scope_varname.clone()),
                    RustExpr::local(ctxt.input_varname.clone()),
                ]);
                (Vec::new(), Some(call.wrap_try()))
            }
            SimpleLogic::SkipToNextMultiple(n) => {
                // FIXME - this currently produces correct but inefficient code
                // it is harder to write, but much more efficient, to cut the buffer at the right place
                // in order to do so, we would need a more advanced Parser model or more complex inline logic
                let cond = RustExpr::infix(
                    RustExpr::infix(
                        RustExpr::local("input").call_method("offset"),
                        Operator::Rem,
                        RustExpr::num_lit(*n),
                    ),
                    Operator::Neq,
                    RustExpr::num_lit(0usize),
                );
                let body = {
                    let let_tmp = RustStmt::assign(
                        "_",
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method("read_byte")
                            .wrap_try(),
                    );
                    vec![let_tmp]
                };
                (
                    vec![RustStmt::Control(RustControl::While(cond, body))],
                    Some(RustExpr::UNIT),
                )
            }
            SimpleLogic::ByteIn(bs) => {
                let call = RustExpr::local(ctxt.input_varname.clone())
                    .call_method("read_byte")
                    .wrap_try();
                let b_let = RustStmt::assign("b", call);
                let (cond, always_true) =
                    ByteCriterion::from(bs).as_predicate(RustExpr::local("b"));
                let logic = if always_true {
                    RustExpr::local("b")
                } else {
                    let b_true = vec![RustStmt::Return(ReturnKind::Implicit, RustExpr::local("b"))];
                    let b_false = vec![RustStmt::Return(
                        ReturnKind::Keyword,
                        RustExpr::local("None"),
                    )];
                    RustExpr::Control(Box::new(RustControl::If(cond, b_true, Some(b_false))))
                };
                ([b_let].to_vec(), Some(logic))
            }
            SimpleLogic::Eval(expr) => (vec![], Some(expr.clone())),
        }
    }
}

fn decoder_fn(ix: usize, t: &GenType, logic: CaseLogic) -> RustFn {
    let name = Label::from(format!("Decoder{ix}"));
    let params = {
        let mut tmp = DefParams::new();
        tmp.push_lifetime("'input");
        tmp
    };
    let sig = {
        let args = {
            let arg0 = {
                let name = "scope".into();
                let ty = RustType::borrow_of(None, Mut::Mutable, RustType::imported("Scope"));
                (name, ty)
            };
            let arg1 = {
                let name = "input".into();
                let ty = {
                    let mut params = RustParams::<RustLt, RustType>::new();
                    params.push_lifetime(RustLt::Parametric("'input".into()));
                    RustType::borrow_of(
                        None,
                        Mut::Mutable,
                        RustType::verbatim("ParseCtxt", Some(params)),
                    )
                };
                (name, ty)
            };
            [arg0, arg1].to_vec()
        };
        FnSig::new(args, Some(RustType::option_of(t.clone().to_rust_type())))
    };
    let ctxt = ProdCtxt {
        input_varname: &Label::from("input"),
        scope_varname: &Label::from("scope"),
    };
    let (stmts, ret) = logic.to_ast(ctxt);
    let body = stmts
        .into_iter()
        .chain(std::iter::once(RustStmt::Return(
            ReturnKind::Implicit,
            RustExpr::some(ret.unwrap()),
        )))
        .collect();
    RustFn::new(name, Some(params), sig, body)
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum ByteCriterion {
    Any,
    MustBe(u8),         // singleton
    OtherThan(u8),      // negated singleton
    WithinSet(ByteSet), // use embed_byteset to bridge to RustExpr
}

impl From<&ByteSet> for ByteCriterion {
    fn from(value: &ByteSet) -> Self {
        if value.is_full() {
            ByteCriterion::Any
        } else {
            match value.len() {
                1 => {
                    let elt = value.min_elem().expect("len == 1 but no min_elem");
                    ByteCriterion::MustBe(elt)
                }
                255 => {
                    let elt = (!value)
                        .min_elem()
                        .expect("len == 255 but no min_elem (on negation)");
                    ByteCriterion::OtherThan(elt)
                }
                2..=254 => ByteCriterion::WithinSet(*value),
                other => unreachable!("unexpected byteset len in catch-all: {other}"),
            }
        }
    }
}

impl ByteCriterion {
    /// Returns a tuple consisting of a RustExpr that evaluates to `true` if the argument satisfies the criterion
    /// that `self` represents, and whose second element is a flag indicating whether the expression
    /// is unconditionally true (and therefore may be elided in 'else-if' or case guard contexts)
    fn as_predicate(&self, arg: RustExpr) -> (RustExpr, bool) {
        match self {
            ByteCriterion::Any => (RustExpr::TRUE, true),
            ByteCriterion::MustBe(byte) => (
                RustExpr::Operation(RustOp::op_eq(arg, RustExpr::num_lit(*byte))),
                false,
            ),
            ByteCriterion::OtherThan(byte) => (
                RustExpr::Operation(RustOp::op_neq(arg, RustExpr::num_lit(*byte))),
                false,
            ),
            ByteCriterion::WithinSet(bs) => {
                (embed_byteset(bs).call_method_with("contains", [arg]), false)
            }
        }
    }
}

fn embed_byteset(bs: &ByteSet) -> RustExpr {
    if bs.is_full() {
        RustExpr::scoped(["ByteSet"], "full").call()
    } else if bs.len() == 1 {
        let Some(elt) = bs.min_elem() else {
            unreachable!("len == 1 but no min_elem")
        };
        RustExpr::scoped(["ByteSet"], "singleton").call_with([RustExpr::num_lit(elt)])
    } else {
        let [q0, q1, q2, q3] = bs.to_bits();
        RustExpr::scoped(["ByteSet"], "from_bits").call_with([RustExpr::ArrayLit(vec![
            RustExpr::num_lit(q0 as usize),
            RustExpr::num_lit(q1 as usize),
            RustExpr::num_lit(q2 as usize),
            RustExpr::num_lit(q3 as usize),
        ])])
    }
}

impl From<RustBlock> for RustExpr {
    fn from(value: RustBlock) -> Self {
        let (stmts, o_expr) = value;
        let expr = o_expr.unwrap_or(RustExpr::UNIT);
        if stmts.is_empty() {
            expr
        } else {
            RustExpr::BlockScope(stmts, Box::new(expr))
        }
    }
}

fn implicate_return(value: RustBlock) -> Vec<RustStmt> {
    let (mut stmts, o_expr) = value;
    if let Some(expr) = o_expr {
        stmts.push(RustStmt::Return(ReturnKind::Implicit, expr));
    }
    stmts
}

// follows the same rules as CaseLogic::to_ast as far as the expression type of the generated code
fn embed_matchtree(tree: &MatchTree, ctxt: ProdCtxt<'_>) -> RustBlock {
    fn expand_matchtree(tree: &MatchTree, ctxt: ProdCtxt<'_>) -> RustBlock {
        if tree.branches.is_empty() {
            if let Some(ix) = tree.accept {
                return (Vec::new(), Some(RustExpr::num_lit(ix)));
            } else {
                return (
                    vec![RustStmt::Return(ReturnKind::Keyword, RustExpr::NONE)],
                    None,
                );
            }
        }

        let bind = RustStmt::assign(
            "b",
            RustExpr::local(ctxt.input_varname.clone())
                .call_method("read_byte")
                .wrap_try(),
        );

        if tree.branches.len() == 1 {
            let (bs, branch) = tree.branches.first().unwrap();
            let (guard, always_true) = ByteCriterion::from(bs).as_predicate(RustExpr::local("b"));
            if always_true {
                // we have one non-accepting branch but it is unconditional
                return expand_matchtree(branch, ctxt);
            } else {
                let b_true: Vec<RustStmt> = implicate_return(expand_matchtree(branch, ctxt));
                let b_false = {
                    if let Some(ix) = tree.accept {
                        vec![RustStmt::Return(
                            ReturnKind::Implicit,
                            RustExpr::num_lit(ix),
                        )]
                    } else {
                        vec![RustStmt::Return(ReturnKind::Keyword, RustExpr::NONE)]
                    }
                };
                return (
                    vec![bind],
                    Some(RustExpr::Control(Box::new(RustControl::If(
                        guard,
                        b_true,
                        Some(b_false),
                    )))),
                );
            }
        }

        let mut cases = Vec::new();

        for (bs, branch) in tree.branches.iter() {
            let crit = ByteCriterion::from(bs);
            match crit {
                ByteCriterion::Any => {
                    unreachable!("unconditional descent with more than one branch");
                }
                ByteCriterion::MustBe(b) => {
                    let lhs = MatchCaseLHS::Pattern(RustPattern::PrimLiteral(
                        RustPrimLit::Numeric(b as usize),
                    ));
                    let rhs = implicate_return(expand_matchtree(branch, ctxt));
                    cases.push((lhs, rhs));
                }
                ByteCriterion::OtherThan(_) | ByteCriterion::WithinSet(_) => {
                    let (guard, _) = crit.as_predicate(RustExpr::local("tmp"));
                    let lhs = MatchCaseLHS::WithGuard(
                        RustPattern::CatchAll(Some(Label::from("tmp"))),
                        guard,
                    );
                    let rhs = implicate_return(expand_matchtree(branch, ctxt));
                    cases.push((lhs, rhs));
                }
            }
        }
        let matchblock = RustControl::Match(RustExpr::local("b"), cases);
        (vec![bind], Some(RustExpr::Control(Box::new(matchblock))))
    }

    let b_lookahead = RustStmt::assign(
        "lookahead",
        RustExpr::BorrowMut(Box::new(
            RustExpr::local(ctxt.input_varname.clone()).call_method("clone"),
        )),
    );
    let ll_context = ProdCtxt {
        input_varname: &Label::from("lookahead"),
        scope_varname: ctxt.scope_varname,
    };

    let (stmts, expr) = expand_matchtree(tree, ll_context);
    (std::iter::once(b_lookahead).chain(stmts).collect(), expr)
}

/// Abstraction type use to sub-categorize different Decoders and ensure that the codegen layer
/// is more resilient to changes both upstream (in the Decoder model)
/// and downstream (in the API made available for generated code to use)
#[derive(Clone, Debug)]
enum CaseLogic {
    Simple(SimpleLogic),
    Derived(DerivedLogic),
    Sequential(SequentialLogic),
    Parallel(ParallelLogic),
    Repeat(RepeatLogic),
    Other(OtherLogic),
    Unhandled(Label), // for generating a panic expression rather than panicking in codegen
}

/// Cases where a constant block of logic is repeated (0 or more times)
#[derive(Clone, Debug)]
enum RepeatLogic {
    ContinueOnMatch(MatchTree, Box<CaseLogic>), // evaluates a matchtree and continues if it is matched
    BreakOnMatch(MatchTree, Box<CaseLogic>),    // evaluates a matchtree and breaks if it is matched
    ExactCount(RustExpr, Box<CaseLogic>),       // repeats a specific numnber of times
    ConditionTerminal(RustExpr, Box<CaseLogic>), // stops when a predicate for 'terminal element' is satisfied
    ConditionComplete(RustExpr, Box<CaseLogic>), // stops when a predicate for 'complete sequence' is satisfied
}

impl RepeatLogic {
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            RepeatLogic::ContinueOnMatch(ctree, elt) => {
                let mut stmts = Vec::new();

                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(RustStmt::Let(
                    Mut::Mutable,
                    Label::from("accum"),
                    None,
                    RustExpr::scoped(["Vec"], "new").call(),
                ));
                let ctrl = {
                    let tree_index_expr: RustExpr = embed_matchtree(ctree, ctxt).into();
                    let bind_ix = RustStmt::assign("matching_ix", tree_index_expr);
                    let cond = RustExpr::infix(
                        RustExpr::local("matching_ix"),
                        Operator::Eq,
                        RustExpr::num_lit(0usize),
                    );
                    let b_continue = [
                        RustStmt::assign("next_elem", elt_expr),
                        RustStmt::Expr(
                            RustExpr::local("accum")
                                .call_method_with("push", [RustExpr::local("next_elem")]),
                        ),
                    ]
                    .to_vec();
                    let b_stop = [RustStmt::Control(RustControl::Break)].to_vec();
                    let escape_clause = RustControl::If(cond, b_continue, Some(b_stop));
                    RustStmt::Control(RustControl::While(
                        RustExpr::TRUE,
                        vec![bind_ix, RustStmt::Control(escape_clause)],
                    ))
                };
                stmts.push(ctrl);
                (stmts, Some(RustExpr::local("accum")))
            }
            RepeatLogic::BreakOnMatch(btree, elt) => {
                let mut stmts = Vec::new();

                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(RustStmt::Let(
                    Mut::Mutable,
                    Label::from("accum"),
                    None,
                    RustExpr::scoped(["Vec"], "new").call(),
                ));
                let ctrl = {
                    let tree_index_expr: RustExpr = embed_matchtree(btree, ctxt).into();
                    let bind_ix = RustStmt::assign("matching_ix", tree_index_expr);
                    let cond = RustExpr::infix(
                        RustExpr::local("matching_ix"),
                        Operator::Eq,
                        RustExpr::num_lit(0usize),
                    );
                    let b_continue = [
                        RustStmt::assign("next_elem", elt_expr),
                        RustStmt::Expr(
                            RustExpr::local("accum")
                                .call_method_with("push", [RustExpr::local("next_elem")]),
                        ),
                    ]
                    .to_vec();
                    let b_stop = [RustStmt::Control(RustControl::Break)].to_vec();
                    let escape_clause = RustControl::If(cond, b_stop, Some(b_continue));
                    RustStmt::Control(RustControl::While(
                        RustExpr::TRUE,
                        vec![bind_ix, RustStmt::Control(escape_clause)],
                    ))
                };
                stmts.push(ctrl);
                (stmts, Some(RustExpr::local("accum")))
            }
            RepeatLogic::ExactCount(expr_n, elt) => {
                let mut stmts = Vec::new();

                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(RustStmt::Let(
                    Mut::Mutable,
                    Label::from("accum"),
                    None,
                    RustExpr::scoped(["Vec"], "new").call(),
                ));
                // N non-loop blocks rather than 1 block representing an N-iteration loop
                let body = vec![RustStmt::Expr(
                    RustExpr::local("accum").call_method_with("push", [elt_expr]),
                )];
                stmts.push(RustStmt::Control(RustControl::ForRange0(
                    Label::from("_"),
                    expr_n.clone(),
                    body,
                )));

                (stmts, Some(RustExpr::local("accum")))
            }
            RepeatLogic::ConditionTerminal(tpred, elt) => {
                let mut stmts = Vec::new();
                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(RustStmt::Let(
                    Mut::Mutable,
                    Label::from("accum"),
                    None,
                    RustExpr::scoped(["Vec"], "new").call(),
                ));
                let ctrl = {
                    let elt_bind = RustStmt::assign("elem", elt_expr);
                    let cond = tpred
                        .clone()
                        .call()
                        .call_with([RustExpr::Borrow(Box::new(RustExpr::local("elem")))]);
                    let b_terminal = [
                        RustStmt::Expr(
                            RustExpr::local("accum")
                                .call_method_with("push", [RustExpr::local("elem")]),
                        ),
                        RustStmt::Control(RustControl::Break),
                    ]
                    .to_vec();
                    let b_else = [RustStmt::Expr(
                        RustExpr::local("accum")
                            .call_method_with("push", [RustExpr::local("elem")]),
                    )]
                    .to_vec();
                    let escape_clause = RustControl::If(cond, b_terminal, Some(b_else));
                    RustStmt::Control(RustControl::While(
                        RustExpr::TRUE,
                        vec![elt_bind, RustStmt::Control(escape_clause)],
                    ))
                };
                stmts.push(ctrl);
                (stmts, Some(RustExpr::local("accum")))
            }
            RepeatLogic::ConditionComplete(cpred, elt) => {
                let mut stmts = Vec::new();
                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(RustStmt::Let(
                    Mut::Mutable,
                    Label::from("accum"),
                    None,
                    RustExpr::scoped(["Vec"], "new").call(),
                ));
                let ctrl = {
                    let elt_bind = RustStmt::assign("elem", elt_expr);
                    let elt_push = RustStmt::Expr(
                        RustExpr::local("accum")
                            .call_method_with("push", [RustExpr::local("elem")]),
                    );
                    let cond = cpred
                        .clone()
                        .call()
                        .call_with([RustExpr::Borrow(Box::new(RustExpr::local("accum")))]);
                    let b_terminal = [RustStmt::Control(RustControl::Break)].to_vec();
                    let escape_clause = RustControl::If(cond, b_terminal, None);
                    RustStmt::Control(RustControl::While(
                        RustExpr::TRUE,
                        vec![elt_bind, elt_push, RustStmt::Control(escape_clause)],
                    ))
                };
                stmts.push(ctrl);
                (stmts, Some(RustExpr::local("accum")))
            }
        }
    }
}

/// Cases that apply other case-logic in sequence to an incrementally updated input
#[derive(Clone, Debug)]
enum SequentialLogic {
    AccumTuple {
        constructor: Option<Constructor>,
        elements: Vec<CaseLogic>,
    },
    AccumRecord {
        constructor: Constructor,
        fields: Vec<(Label, CaseLogic)>,
    },
}

impl SequentialLogic {
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            SequentialLogic::AccumTuple {
                constructor,
                elements,
            } => {
                if elements.is_empty() {
                    return (Vec::new(), Some(RustExpr::UNIT));
                }

                let mut names: Vec<Label> = Vec::new();
                let mut body = Vec::new();

                for (ix, elt_cl) in elements.iter().enumerate() {
                    let varname = format!("field{}", ix);
                    names.push(varname.clone().into());
                    let (mut preamble, o_val) = elt_cl.to_ast(ctxt);
                    if let Some(val) = o_val {
                        body.push(RustStmt::assign(
                            varname,
                            RustExpr::BlockScope(preamble, Box::new(val)),
                        ));
                    } else {
                        // FIXME - the logic here may be incorrect (we reach this branch if there is an unconditional 'return None' in the expansion of elt_cl)
                        body.append(&mut preamble);
                    }
                }

                if let Some(con) = constructor {
                    // FIXME - this may be incorrect since we don't always know the type-context (e.g. if we are in an enum)
                    (
                        body,
                        Some(
                            RustExpr::local(con.clone())
                                .call_with(names.into_iter().map(RustExpr::local)),
                        ),
                    )
                } else {
                    (
                        body,
                        Some(RustExpr::Tuple(
                            names.into_iter().map(RustExpr::local).collect(),
                        )),
                    )
                }
            }
            SequentialLogic::AccumRecord {
                constructor,
                fields,
            } => {
                if fields.is_empty() {
                    unreachable!(
                        "SequentialLogic::AccumRecord has no fields, which is not an expected case"
                    );
                }

                let mut names: Vec<Label> = Vec::new();
                let mut body = Vec::new();

                for (fname, fld_cl) in fields.iter() {
                    let varname = rust_ast::sanitize_label(fname);
                    names.push(varname.clone());
                    let (mut preamble, o_val) = fld_cl.to_ast(ctxt);
                    if let Some(val) = o_val {
                        body.push(RustStmt::assign(
                            varname,
                            RustExpr::BlockScope(preamble, Box::new(val)),
                        ));
                    } else {
                        // FIXME - the logic here may be incorrect (we reach this branch if there is an unconditional 'return None' in the expansion of fld_cl)
                        body.append(&mut preamble);
                    }
                }

                (
                    body,
                    Some(RustExpr::Struct(
                        constructor.clone().into(),
                        names.into_iter().map(|l| (l, None)).collect(),
                    )),
                )
            }
        }
    }
}

/// Catch-all for hard-to-classify cases
#[derive(Clone, Debug)]
enum OtherLogic {
    Descend(MatchTree, Vec<CaseLogic>),
    ExprMatch(RustExpr, Vec<(MatchCaseLHS, CaseLogic)>),
}

impl OtherLogic {
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            OtherLogic::Descend(tree, cases) => {
                let mut branches = Vec::new();
                for (ix, case) in cases.iter().enumerate() {
                    let (mut rhs, o_val) = case.to_ast(ctxt);
                    if let Some(val) = o_val {
                        rhs.push(RustStmt::Return(ReturnKind::Implicit, val));
                    }
                    branches.push((
                        MatchCaseLHS::Pattern(RustPattern::PrimLiteral(RustPrimLit::Numeric(ix))),
                        rhs,
                    ));
                }
                let bind = RustStmt::assign("tree_index", invoke_matchtree(tree, ctxt));
                let ret = RustExpr::Control(Box::new(RustControl::Match(
                    RustExpr::local("tree_index"),
                    branches,
                )));
                (vec![bind], Some(ret))
            }
            OtherLogic::ExprMatch(expr, cases) => {
                let mut branches = Vec::new();
                for (lhs, logic) in cases.iter() {
                    let (mut rhs, o_val) = logic.to_ast(ctxt);
                    if let Some(val) = o_val {
                        rhs.push(RustStmt::Return(ReturnKind::Implicit, val));
                    }
                    branches.push((lhs.clone(), rhs));
                }
                let ret = RustExpr::Control(Box::new(RustControl::Match(expr.clone(), branches)));
                (vec![], Some(ret))
            }
        }
    }
}

/// this production should be a RustExpr whose compiled type is usize, and whose
/// runtime value is the index of the successful match relative to the input
fn invoke_matchtree(tree: &MatchTree, ctxt: ProdCtxt<'_>) -> RustExpr {
    embed_matchtree(tree, ctxt).into()
}

/// Cases that require processing of multiple cases in parallel (on the same input-state)
#[derive(Clone, Debug)]
enum ParallelLogic {
    Alts(Vec<CaseLogic>),
}

impl ParallelLogic {
    fn to_ast(&self, _ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            ParallelLogic::Alts(_alts) => {
                // FIMXE - no proper model for Parallel parsing yet
                (
                    Vec::new(),
                    Some(
                        RustExpr::local("unimplemented!")
                            .call_with([RustExpr::str_lit("ParallelLogic::Alts.to_ast(..)")]),
                    ),
                )
            }
        }
    }
}

/// Cases that require no recursion into other case-logic
#[derive(Clone, Debug)]
enum SimpleLogic {
    Fail,
    ExpectEnd,
    Invoke(usize, Vec<(Label, Expr)>),
    SkipToNextMultiple(usize),
    ByteIn(ByteSet),
    Eval(RustExpr),
}

/// Cases that recurse into other case-logic only once
#[derive(Clone, Debug)]
enum DerivedLogic {
    VariantOf(Constructor, Box<CaseLogic>),
    UnitVariantOf(Constructor, Box<CaseLogic>),
    MapOf(RustExpr, Box<CaseLogic>),
    Let(Label, RustExpr, Box<CaseLogic>),
}

impl DerivedLogic {
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            DerivedLogic::VariantOf(constr, inner) => {
                let assign_inner = RustStmt::assign("inner", RustExpr::from(inner.to_ast(ctxt)));
                (
                    vec![assign_inner],
                    Some(
                        RustExpr::local(Label::from(constr.clone()))
                            .call_with([RustExpr::local("inner")]),
                    ),
                )
            }
            DerivedLogic::UnitVariantOf(constr, inner) => {
                let assign_inner = RustStmt::assign("_", RustExpr::from(inner.to_ast(ctxt)));
                (
                    vec![assign_inner],
                    Some(RustExpr::local(Label::from(constr.clone()))),
                )
            }
            DerivedLogic::MapOf(f, inner) => {
                let assign_inner = RustStmt::assign("inner", RustExpr::from(inner.to_ast(ctxt)));
                (
                    vec![assign_inner],
                    Some(f.clone().call_with([RustExpr::local("inner")])),
                )
            }
            DerivedLogic::Let(name, expr, inner) => {
                let mut stmts = Vec::new();
                stmts.push(RustStmt::assign(name.clone(), expr.clone()));
                let (mut after, retval) = inner.to_ast(ctxt);
                stmts.append(&mut after);
                (stmts, retval)
            }
        }
    }
}

pub fn print_program(program: &Program) {
    let mut codegen = Codegen::new();
    let mut items = Vec::new();
    codegen.populate_decoder_types(program);
    let mut tdefs = Vec::from_iter(codegen.namegen.revmap.iter());
    tdefs.sort_by_key(|(_, ix)| *ix);
    for (tdef, ixlab) in tdefs.into_iter() {
        let it = RustItem::from_decl(RustDecl::TypeDef(ixlab.into(), tdef.clone()));
        items.push(it);
    }

    for (i, (d, _t)) in program.decoders.iter().enumerate() {
        let t = &codegen.decoder_types[i];
        let logic = codegen.translate(d, Some(t));
        let f = decoder_fn(i, t, logic);
        items.push(RustItem::from_decl(RustDecl::Function(f)));
    }

    let mut content = RustProgram::from_iter(items);
    content.add_import(RustImport {
        path: vec!["doodle".into(), "prelude".into()],
        uses: RustImportItems::Wildcard,
    });

    let extra = r#"
#[test]
fn test_decoder_27() {
    // PNG signature
    let input = b"\x89PNG\r\n\x1A\n";
    let mut parse_ctxt = ParseCtxt::new(input);
    let mut scope = Scope::Empty;
    let ret = Decoder27(&mut scope, &mut parse_ctxt);
    assert!(ret.is_some());
}"#;

    print!("{}{}", content.to_fragment(), extra)
}

/// COPYPASTA

/// Decoders with a fixed amount of lookahead

pub struct DecoderFn(CaseLogic, RustType);

pub struct SourceMap {
    adhoc_types: Vec<RustTypeDef>,
    decoder_skels: Vec<DecoderFn>,
}

impl SourceMap {
    pub fn new() -> SourceMap {
        SourceMap {
            adhoc_types: Vec::new(),
            decoder_skels: Vec::new(),
        }
    }
}

pub struct RustTypeScope<'a> {
    parent: Option<&'a RustTypeScope<'a>>,
    names: Vec<Label>,
    rtypes: Vec<RustType>,
}

impl<'a> RustTypeScope<'a> {
    fn new() -> Self {
        let parent = None;
        let names = Vec::new();
        let rtypes = Vec::new();
        RustTypeScope {
            parent,
            names,
            rtypes,
        }
    }

    fn child(parent: &'a RustTypeScope<'a>) -> Self {
        let parent = Some(parent);
        let names = Vec::new();
        let rtypes = Vec::new();
        RustTypeScope {
            parent,
            names,
            rtypes,
        }
    }

    fn push(&mut self, name: Label, rt: RustType) {
        self.names.push(name);
        self.rtypes.push(rt);
    }

    fn push_format(&mut self, name: Label, rt: RustType) {
        self.names.push(name);
        self.rtypes.push(rt);
    }

    fn get_type_by_name(&self, name: &str) -> &RustType {
        for (i, n) in self.names.iter().enumerate().rev() {
            if n == name {
                return &self.rtypes[i];
            }
        }
        if let Some(scope) = self.parent {
            scope.get_type_by_name(name)
        } else {
            panic!("variable not found: {name}");
        }
    }
}

pub struct Generator<'a> {
    module: &'a FormatModule,
    module_ftypes: HashMap<usize, ValueType>,
    codegen: Codegen,
    sourcemap: SourceMap,
}

pub(crate) enum TypedTreeNode {
    Format(GTFormat),
    Pattern(GTPattern),
    Expr(GTExpr),
}

impl<'a> Generator<'a> {
    pub(crate) fn new(module: &'a FormatModule) -> Self {
        let module_ftypes = HashMap::new();
        let codegen = Codegen::new();
        let sourcemap = SourceMap::new();
        Generator {
            module,
            module_ftypes,
            codegen,
            sourcemap,
        }
    }

    pub fn compile(module: &'a FormatModule, top_format: &Format) -> Self {
        let mut gen = Self::new(module);
        let mut tc = TypeChecker::new();
        let ctxt = crate::typecheck::Ctxt::new(module, &UScope::Empty);
        let _ = tc
            .infer_utype_format(top_format, ctxt)
            .unwrap_or_else(|err| panic!("Failed to infer topl-level format type: {err}"));
        let mut trav = Traversal::new(module, &tc, &mut gen.codegen);
        let top = trav.decorate_format(top_format, &TypedDynScope::Empty);
        gen.populate_from_top_format(&top);
        gen
    }

    fn populate_from_top_format(&mut self, top: &TypedFormat<GenType>) {}
}

pub(crate) struct Traversal<'a> {
    module: &'a FormatModule,
    next_index: usize,
    t_formats: HashMap<usize, Rc<GTFormat>>,
    tc: &'a TypeChecker,
    codegen: &'a mut Codegen,
}

impl<'a> Traversal<'a> {
    /// Increment the current `next_index` by 1 and return its un-incremented value.
    pub fn get_and_increment_index(&mut self) -> usize {
        let ret = self.next_index;
        self.next_index += 1;
        ret
    }

    /// Increment the current `tree_index` by 1.
    pub fn increment_index(&mut self) {
        self.next_index += 1;
    }

    /// Return the current `tree_index` without mutation.
    pub fn get_index(&self) -> usize {
        self.next_index
    }

    fn decorate_dynamic_format<'s>(&mut self, dynf: &DynFormat) -> TypedDynFormat<GenType> {
        match dynf {
            DynFormat::Huffman(code_lengths, opt_values_expr) => {
                let t_codes = self.decorate_expr(code_lengths);
                self.increment_index();

                let t_values_expr = opt_values_expr.as_ref().map(|values_expr| {
                    let t_values = self.decorate_expr(values_expr);
                    self.increment_index();
                    t_values
                });
                GTDynFormat::Huffman(t_codes, t_values_expr)
            }
        }
    }

    fn decorate_pattern(&mut self, pat: &Pattern) -> TypedPattern<GenType> {
        let index = self.get_and_increment_index();

        match pat {
            Pattern::Binding(name) => {
                let gt = self.get_gt_from_index(index);
                GTPattern::Binding(gt, name.clone())
            }
            Pattern::Wildcard => GTPattern::Wildcard(self.get_gt_from_index(index)),
            Pattern::Bool(b) => GTPattern::Bool(*b),
            Pattern::U8(n) => GTPattern::U8(*n),
            Pattern::U16(n) => GTPattern::U16(*n),
            Pattern::U32(n) => GTPattern::U32(*n),
            Pattern::U64(n) => GTPattern::U64(*n),
            Pattern::Char(c) => GTPattern::Char(*c),
            Pattern::Tuple(elts) => {
                let mut t_elts = Vec::with_capacity(elts.len());
                for elt in elts {
                    let t_elt = self.decorate_pattern(elt);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                GTPattern::Tuple(gt, t_elts)
            }
            Pattern::Variant(name, inner) => {
                let t_inner = self.decorate_pattern(inner);
                let gt = self.get_gt_from_index(index);
                GTPattern::Variant(gt, name.clone(), Box::new(t_inner))
            }
            Pattern::Seq(elts) => {
                let mut t_elts = Vec::with_capacity(elts.len());
                for elt in elts {
                    let t_elt = self.decorate_pattern(elt);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                GTPattern::Seq(gt, t_elts)
            }
        }
    }

    pub(crate) fn new(
        module: &'a FormatModule,
        tc: &'a TypeChecker,
        codegen: &'a mut Codegen,
    ) -> Self {
        Self {
            module,
            next_index: 0,
            t_formats: HashMap::new(),
            tc,
            codegen,
        }
    }

    fn decorate_format_union(
        &mut self,
        branches: &[Format],
        dyns: &TypedDynScope<'_>,
        is_det: bool,
    ) -> GTFormat {
        let index = self.get_and_increment_index();
        let gt = self.get_gt_from_index(index);

        let mut t_branches = Vec::with_capacity(branches.len());
        for branch in branches {
            let t_branch = match branch {
                Format::Variant(name, inner) => {
                    let t_inner = self.decorate_format(inner, dyns);
                    GTFormat::Variant(gt.clone(), name.clone(), Box::new(t_inner))
                }
                _ => self.decorate_format(branch, dyns),
            };
            t_branches.push(t_branch);
        }

        if is_det {
            GTFormat::Union(gt, t_branches)
        } else {
            GTFormat::UnionNondet(gt, t_branches)
        }
    }

    fn decorate_format(&mut self, format: &Format, dyns: &TypedDynScope<'_>) -> GTFormat {
        match format {
            Format::ItemVar(level, args) => {
                let index = self.get_and_increment_index();
                let fm_args = &self.module.args[*level];
                let mut t_args = Vec::with_capacity(args.len());
                for ((lbl, _), arg) in Iterator::zip(fm_args.iter(), args.iter()) {
                    let t_arg = self.decorate_expr(arg);
                    t_args.push((lbl.clone(), t_arg));
                }
                let t_inner = if let Some(val) = self.t_formats.get(level) {
                    val.clone()
                } else {
                    let fmt = self.module.get_format(*level);
                    let tmp = self.decorate_format(fmt, &TypedDynScope::Empty);
                    let ret = Rc::new(tmp);
                    self.t_formats.insert(*level, ret.clone());
                    ret
                };
                let gt = self.get_gt_from_index(index);
                GTFormat::FormatCall(gt, t_args, t_inner)
            }
            Format::Fail => {
                self.increment_index();
                GTFormat::Fail
            }
            Format::EndOfInput => {
                self.increment_index();
                GTFormat::EndOfInput
            }
            Format::Align(n) => {
                self.increment_index();
                GTFormat::Align(*n)
            }
            Format::Byte(bs) => {
                self.increment_index();
                GTFormat::Byte(*bs)
            }
            Format::Union(branches) => self.decorate_format_union(branches, dyns, true),
            Format::UnionNondet(branches) => self.decorate_format_union(branches, dyns, false),
            Format::Tuple(elts) => {
                let index = self.get_and_increment_index();
                let mut t_elts = Vec::with_capacity(elts.len());
                for t in elts {
                    let t_elt = self.decorate_format(t, dyns);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                GTFormat::Tuple(gt, t_elts)
            }
            Format::Record(flds) => {
                let index = self.get_and_increment_index();
                let mut t_flds = Vec::with_capacity(flds.len());
                for (lbl, t) in flds {
                    let t_fld = self.decorate_format(t, dyns);
                    t_flds.push((lbl.clone(), t_fld));
                }
                let gt = self.get_gt_from_index(index);
                GTFormat::Record(gt, t_flds)
            }
            Format::Variant(label, inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Variant(gt, label.clone(), Box::new(t_inner))
            }
            Format::Repeat(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Repeat(gt, Box::new(t_inner))
            }
            Format::Repeat1(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Repeat1(gt, Box::new(t_inner))
            }
            Format::Bits(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Bits(gt, Box::new(t_inner))
            }
            Format::Peek(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.decorate_format(inner, dyns);
                let gt = {
                    let uvar = UVar::new(index);
                    let Some(vt) = self.tc.reify(uvar.into()) else {
                        unreachable!("unable to reify {uvar}")
                    };
                    self.codegen.lift_type(&vt)
                };
                GTFormat::Peek(gt, Box::new(t_inner))
            }
            Format::PeekNot(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::PeekNot(gt, Box::new(t_inner))
            }
            Format::Slice(expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.decorate_expr(expr);
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Slice(gt, t_expr, Box::new(t_inner))
            }
            Format::WithRelativeOffset(expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.decorate_expr(expr);
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::WithRelativeOffset(gt, t_expr, Box::new(t_inner))
            }
            Format::RepeatCount(expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.decorate_expr(expr);
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::RepeatCount(gt, t_expr, Box::new(t_inner))
            }
            Format::RepeatUntilLast(lambda, inner) => {
                let index = self.get_and_increment_index();
                // FIXME - figure out the pattern to apply here
                let t_lambda = self.decorate_expr_lambda(lambda);
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::RepeatUntilLast(gt, t_lambda, Box::new(t_inner))
            }
            Format::RepeatUntilSeq(lambda, inner) => {
                let index = self.get_and_increment_index();
                // FIXME - figure out the pattern to apply here
                let t_lambda = self.decorate_expr_lambda(lambda);
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::RepeatUntilSeq(gt, t_lambda, Box::new(t_inner))
            }
            Format::Map(inner, lambda) => {
                let index = self.get_and_increment_index();
                let t_inner = self.decorate_format(inner, dyns);
                let t_lambda = self.decorate_expr_lambda(lambda);
                let gt = self.get_gt_from_index(index);
                GTFormat::Map(gt, Box::new(t_inner), t_lambda)
            }
            Format::Compute(expr) => {
                let index = self.get_and_increment_index();
                let t_expr = self.decorate_expr(expr);
                let gt = self.get_gt_from_index(index);
                GTFormat::Compute(gt, t_expr)
            }
            Format::Let(lbl, expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.decorate_expr(expr);
                let t_inner = self.decorate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Let(gt, lbl.clone(), t_expr, Box::new(t_inner))
            }
            Format::Match(x, branches) => {
                let index = self.get_and_increment_index();
                let t_x = self.decorate_expr(x);
                let mut t_branches = Vec::with_capacity(branches.len());
                for (pat, rhs) in branches {
                    let t_pat = self.decorate_pattern(pat);
                    let t_rhs = self.decorate_format(rhs, dyns);
                    t_branches.push((t_pat, t_rhs));
                }
                let gt = self.get_gt_from_index(index);
                GTFormat::Match(gt, t_x, t_branches)
            }
            Format::Dynamic(lbl, dynf, inner) => {
                let index = self.get_and_increment_index();
                let t_dynf = self.decorate_dynamic_format(dynf);
                let newdyns = TypedDynScope::Binding(TypedDynBinding::new(
                    dyns,
                    lbl,
                    Rc::new(t_dynf.clone()),
                ));
                let t_inner = self.decorate_format(inner, &newdyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Dynamic(gt, lbl.clone(), t_dynf, Box::new(t_inner))
            }
            Format::Apply(lbl) => {
                let index = self.get_and_increment_index();
                let t_dynf = dyns
                    .get_typed_dynf_by_name(lbl)
                    .unwrap_or_else(|| panic!("missing dynformat {lbl}"));
                let gt = self.get_gt_from_index(index);
                GTFormat::Apply(gt, t_dynf)
            }
        }
    }

    fn get_gt_from_index(&mut self, index: usize) -> GenType {
        let uvar = UVar::new(index);
        let Some(vt) = self.tc.reify(uvar.into()) else {
            unreachable!("unable to reify {uvar}")
        };
        self.codegen.lift_type(&vt)
    }

    fn decorate_expr<'s>(&mut self, expr: &Expr) -> GTExpr {
        let index = self.get_and_increment_index();
        match expr {
            Expr::Var(lbl) => {
                let gt = self.get_gt_from_index(index);
                GTExpr::Var(gt, lbl.clone())
            }
            Expr::Bool(b) => GTExpr::Bool(*b),
            Expr::U8(n) => GTExpr::U8(*n),
            Expr::U16(n) => GTExpr::U16(*n),
            Expr::U32(n) => GTExpr::U32(*n),
            Expr::U64(n) => GTExpr::U64(*n),
            Expr::Tuple(elts) => {
                let mut t_elts = Vec::with_capacity(elts.len());
                for elt in elts {
                    let t_elt = self.decorate_expr(elt);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                GTExpr::Tuple(gt, t_elts)
            }
            Expr::Record(flds) => {
                // FIXME - integrate a multiscope (new) at this layer
                let mut t_flds = Vec::with_capacity(flds.len());
                for (lbl, fld) in flds {
                    let t_fld = self.decorate_expr(fld);
                    t_flds.push((lbl.clone(), t_fld));
                }
                let gt = self.get_gt_from_index(index);
                GTExpr::Record(gt, t_flds)
            }
            Expr::TupleProj(e, ix) => {
                let t_e = self.decorate_expr(e);
                let gt = self.get_gt_from_index(index);
                GTExpr::TupleProj(gt, Box::new(t_e), *ix)
            }
            Expr::RecordProj(e, fld) => {
                let t_e = self.decorate_expr(e);
                let gt = self.get_gt_from_index(index);
                GTExpr::RecordProj(gt, Box::new(t_e), fld.clone())
            }
            Expr::Variant(lbl, inner) => {
                let t_inner = self.decorate_expr(inner);
                let gt = self.get_gt_from_index(index);
                GTExpr::Variant(gt, lbl.clone(), Box::new(t_inner))
            }
            Expr::Seq(elts) => {
                let mut t_elts = Vec::with_capacity(elts.len());
                for elt in elts {
                    let t_elt = self.decorate_expr(elt);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                GTExpr::Seq(gt, t_elts)
            }
            Expr::Match(head, branches) => {
                let t_head = self.decorate_expr(head);
                let mut t_branches = Vec::with_capacity(branches.len());
                for (pat, rhs) in branches {
                    let t_pat = self.decorate_pattern(pat);
                    let t_rhs = self.decorate_expr(rhs);
                    t_branches.push((t_pat, t_rhs));
                }
                let gt = self.get_gt_from_index(index);
                GTExpr::Match(gt, Box::new(t_head), t_branches)
            }
            Expr::Lambda(..) => unreachable!(
                "Cannot decorate Expr::Lambda in neutral (i.e. not lambda-aware) context"
            ),
            Expr::IntRel(rel, x, y) => {
                let t_x = self.decorate_expr(x);
                let t_y = self.decorate_expr(y);
                let gt = self.get_gt_from_index(index);
                GTExpr::IntRel(gt, *rel, Box::new(t_x), Box::new(t_y))
            }
            Expr::Arith(op, x, y) => {
                let t_x = self.decorate_expr(x);
                let t_y = self.decorate_expr(y);
                let gt = self.get_gt_from_index(index);
                GTExpr::Arith(gt, *op, Box::new(t_x), Box::new(t_y))
            }
            Expr::AsU8(inner) => {
                let t_inner = self.decorate_expr(inner);
                GTExpr::AsU8(Box::new(t_inner))
            }
            Expr::AsU16(inner) => {
                let t_inner = self.decorate_expr(inner);
                GTExpr::AsU16(Box::new(t_inner))
            }
            Expr::AsU32(inner) => {
                let t_inner = self.decorate_expr(inner);
                GTExpr::AsU32(Box::new(t_inner))
            }
            Expr::AsU64(inner) => {
                let t_inner = self.decorate_expr(inner);
                GTExpr::AsU64(Box::new(t_inner))
            }
            Expr::AsChar(inner) => {
                let t_inner = self.decorate_expr(inner);
                GTExpr::AsChar(Box::new(t_inner))
            }
            Expr::U16Be(bytes) => {
                let t_bytes = self.decorate_expr(bytes);
                GTExpr::U16Be(Box::new(t_bytes))
            }
            Expr::U16Le(bytes) => {
                let t_bytes = self.decorate_expr(bytes);
                GTExpr::U16Le(Box::new(t_bytes))
            }
            Expr::U32Be(bytes) => {
                let t_bytes = self.decorate_expr(bytes);
                GTExpr::U32Be(Box::new(t_bytes))
            }
            Expr::U32Le(bytes) => {
                let t_bytes = self.decorate_expr(bytes);
                GTExpr::U32Le(Box::new(t_bytes))
            }
            Expr::U64Be(bytes) => {
                let t_bytes = self.decorate_expr(bytes);
                GTExpr::U64Be(Box::new(t_bytes))
            }
            Expr::U64Le(bytes) => {
                let t_bytes = self.decorate_expr(bytes);
                GTExpr::U64Le(Box::new(t_bytes))
            }
            Expr::SeqLength(seq) => {
                let t_seq = self.decorate_expr(seq);
                GTExpr::SeqLength(Box::new(t_seq))
            }
            Expr::SubSeq(seq, start, length) => {
                let t_seq = self.decorate_expr(seq);
                let t_start = self.decorate_expr(start);
                let t_length = self.decorate_expr(length);
                let gt = self.get_gt_from_index(index);
                GTExpr::SubSeq(gt, Box::new(t_seq), Box::new(t_start), Box::new(t_length))
            }
            Expr::FlatMap(seq, lambda) => {
                let t_seq = self.decorate_expr(seq);
                let t_lambda = self.decorate_expr_lambda(lambda);
                let gt = self.get_gt_from_index(index);
                GTExpr::FlatMap(gt, Box::new(t_seq), Box::new(t_lambda))
            }
            Expr::FlatMapAccum(lambda, acc, _acc_vt, seq) => {
                let t_lambda = self.decorate_expr_lambda(lambda);
                let t_acc = self.decorate_expr(acc);
                let t_seq = self.decorate_expr(seq);

                {
                    // account for two extra variables we generate in current TC implementation
                    self.increment_index();
                    self.increment_index();
                }

                let gt = self.get_gt_from_index(index);
                GTExpr::FlatMapAccum(
                    gt,
                    Box::new(t_lambda),
                    Box::new(t_acc),
                    _acc_vt.clone(),
                    Box::new(t_seq),
                )
            }
            Expr::Dup(count, x) => {
                let count_t = self.decorate_expr(count);
                let x_t = self.decorate_expr(x);
                let gt = self.get_gt_from_index(index);
                GTExpr::Dup(gt, Box::new(count_t), Box::new(x_t))
            }
            Expr::Inflate(seq) => {
                let seq_t = self.decorate_expr(seq);

                // increment for extra variable generated by TC logic implementation in this case
                self.increment_index();

                let gt = self.get_gt_from_index(index);

                GTExpr::Inflate(gt, Box::new(seq_t))
            }
        }
    }

    fn decorate_expr_lambda(&mut self, expr: &Expr) -> TypedExpr<GenType> {
        match expr {
            Expr::Lambda(head, body) => {
                let head_index = self.get_and_increment_index();
                // we don't increment here because it will be incremented by the rhs assignment on t_body
                let body_index = self.get_index();
                let t_body = self.decorate_expr(body);
                let gt_head = self.get_gt_from_index(head_index);
                let gt_body = self.get_gt_from_index(body_index);
                GTExpr::Lambda((gt_head, gt_body), head.clone(), Box::new(t_body))
            }
            _ => unreachable!("decorate_expr_lambda: unexpected non-lambda {expr:?}"),
        }
    }
}

type GTFormat = TypedFormat<GenType>;
type GTExpr = TypedExpr<GenType>;
type GTPattern = TypedPattern<GenType>;

#[derive(Clone, PartialEq, Debug)]
enum GenScope<'a> {
    Empty,
    Value(&'a GenScope<'a>, &'a str, Rc<GTExpr>),
}

type GTDynFormat = TypedDynFormat<GenType>;

#[derive(Clone, Debug, PartialEq)]
enum TypedDynScope<'a> {
    Empty,
    Binding(TypedDynBinding<'a>),
}

#[derive(Clone, Debug, PartialEq)]
struct TypedDynBinding<'a> {
    parent: &'a TypedDynScope<'a>,
    label: &'a str,
    t_dynf: Rc<GTDynFormat>,
}

impl<'a> TypedDynBinding<'a> {
    fn get_typed_dynf_by_name(&self, name: &str) -> Option<Rc<GTDynFormat>> {
        if self.label == name {
            Some(self.t_dynf.clone())
        } else {
            self.parent.get_typed_dynf_by_name(name)
        }
    }

    fn new(
        parent: &'a TypedDynScope<'a>,
        label: &'a str,
        t_dynf: Rc<TypedDynFormat<GenType>>,
    ) -> Self {
        Self {
            parent,
            label,
            t_dynf,
        }
    }
}

impl<'a> TypedDynScope<'a> {
    fn get_typed_dynf_by_name(&self, name: &'a str) -> Option<Rc<GTDynFormat>> {
        match self {
            TypedDynScope::Binding(binding) => binding.get_typed_dynf_by_name(name),
            TypedDynScope::Empty => None,
        }
    }
}

mod __impls {
    use super::IxLabel;
    use crate::Label;
    impl From<usize> for IxLabel {
        fn from(value: usize) -> Self {
            Self(value)
        }
    }

    impl From<IxLabel> for Label {
        fn from(value: IxLabel) -> Self {
            Label::Owned(format!("Type{}", value.0))
        }
    }

    impl From<&IxLabel> for Label {
        fn from(value: &IxLabel) -> Self {
            Label::Owned(format!("Type{}", value.0))
        }
    }

    impl AsRef<usize> for IxLabel {
        fn as_ref(&self) -> &usize {
            &self.0
        }
    }

    impl std::borrow::Borrow<usize> for IxLabel {
        fn borrow(&self) -> &usize {
            &self.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::typecheck::Ctxt;

    use super::*;

    fn population_check(module: &FormatModule, f: &Format) {
        let mut tc = TypeChecker::new();
        let _fv = tc.infer_var_format(f, Ctxt::new(module, &UScope::Empty));
        let tc_pop = tc.size();

        // println!("{tc:?}");

        let mut cg = Codegen::new();
        let mut tv = Traversal::new(module, &tc, &mut cg);
        let dec_f = tv.decorate_format(f, &TypedDynScope::Empty);
        let tv_pop = tv.next_index;

        println!("{f:?} => {dec_f:?}");
        assert_eq!(
            tv_pop, tc_pop,
            "failed population check ({} TC vs {} TV) on {:?}",
            tc_pop, tv_pop, dec_f
        );
    }

    fn run_popcheck(fs: &[(&'static str, Format)]) {
        let mut module = FormatModule::new();
        for (name, f) in fs.iter() {
            module.define_format(*name, f.clone());
            population_check(&module, f);
        }
    }

    #[test]
    fn test_popcheck_simple() {
        let formats = vec![
            ("test.fail", Format::Fail),
            ("test.eoi", Format::EndOfInput),
            ("test.align64", Format::Align(64)),
            ("test.any_byte", Format::Byte(ByteSet::full())),
        ];
        run_popcheck(&formats);
    }

    #[test]
    fn test_popcheck_record_simple() {
        let f = Format::Record(vec![
            ("any_byte".into(), Format::Byte(ByteSet::full())),
            ("align64".into(), Format::Align(64)),
            ("eoi".into(), Format::EndOfInput),
        ]);

        run_popcheck(&[("record_simple", f)]);
    }

    #[test]
    fn test_popcheck_adt_simple() {
        let f = Format::Union(vec![
            (Format::Variant("some".into(), Box::new(Format::Byte(ByteSet::full())))),
            (Format::Variant("none".into(), Box::new(Format::EMPTY))),
        ]);

        run_popcheck(&[("adt_simple", f)]);
    }
}
