pub(crate) mod rust_ast;
pub mod typed_decoder;
pub mod typed_format;

use crate::{
    byte_set::ByteSet,
    typecheck::{TypeChecker, UScope, UVar},
    Arith, BaseType, DynFormat, Expr, Format, FormatModule, IntRel, Label, MatchTree, Pattern,
    ValueType,
};

use std::{borrow::Cow, collections::HashMap, rc::Rc};

use rust_ast::*;

use typed_format::{GenType, TypedExpr, TypedFormat, TypedPattern};

use self::{
    typed_decoder::{GTCompiler, GTDecoder, TypedDecoder},
    typed_format::TypedDynFormat,
};

pub(crate) mod ixlabel;
pub(crate) use ixlabel::IxLabel;

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
}

impl Codegen {
    pub fn new() -> Self {
        let namegen = NameGen::new();
        let defined_types = Vec::new();
        Codegen {
            namegen,
            defined_types,
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

    fn translate(&self, decoder: &GTDecoder) -> CaseLogic<GTExpr> {
        match decoder {
            TypedDecoder::Call(_gt, ix, args) =>
                CaseLogic::Simple(SimpleLogic::Invoke(*ix, args.clone())),
            TypedDecoder::Fail => CaseLogic::Simple(SimpleLogic::Fail),
            TypedDecoder::EndOfInput => CaseLogic::Simple(SimpleLogic::ExpectEnd),
            TypedDecoder::Align(n) => CaseLogic::Simple(SimpleLogic::SkipToNextMultiple(*n)),
            TypedDecoder::Byte(bs) => CaseLogic::Simple(SimpleLogic::ByteIn(*bs)),
            TypedDecoder::Variant(gt, vname, inner) => {
                let (tname, tdef) = match gt {
                    | GenType::Def((ix, lab), ..)
                    | GenType::Inline(
                          RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(ix, lab))),
                      ) => (lab.clone(), &self.defined_types[*ix]),
                    other => panic!("unexpected type_hint for Decoder::Variant: {:?}", other),
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
                                CaseLogic::Derived(
                                    DerivedLogic::UnitVariantOf(
                                        constr,
                                        Box::new(self.translate(inner))
                                    )
                                )
                            }
                            Some(RustVariant::Tuple(_, typs)) => {
                                if typs.is_empty() {
                                    unreachable!(
                                        "unexpected Tuple-Variant with 0 positional arguments"
                                    );
                                }
                                match inner.as_ref() {
                                    TypedDecoder::Tuple(_, decs) => {
                                        if decs.len() != typs.len() {
                                            if typs.len() == 1 {
                                                // REVIEW - allowance for 1-tuple variant whose argument type is itself an n-tuple
                                                match &typs[0] {
                                                    RustType::AnonTuple(..) => {
                                                        let cl_mono_tuple = self.translate(inner);
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
                                            for dec in decs.iter() {
                                                let cl_arg = self.translate(dec);
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
                                            let cl_mono = self.translate(inner);
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
                                    TypedDecoder::Record(_, inner_fields) => {
                                        let mut assocs = Vec::new();
                                        for (i, (l0, d)) in inner_fields.iter().enumerate() {
                                            let (l1, _t) = &fields[i];
                                            assert_eq!(
                                                l0.as_ref(),
                                                l1.as_ref(),
                                                "Decoder field `{l0}` != RustTypeDef field `{l1}` (at index {i} in {decoder:?} | {tdef:?})"
                                            );
                                            assocs.push((l0.clone(), self.translate(d)));
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
            TypedDecoder::Parallel(_, alts) =>
                CaseLogic::Parallel(
                    ParallelLogic::Alts(
                        alts
                            .iter()
                            .map(|alt| self.translate(alt))
                            .collect()
                    )
                ),
            TypedDecoder::Branch(_, tree, flat) =>
                CaseLogic::Other(
                    OtherLogic::Descend(
                        tree.clone(),
                        flat
                            .iter()
                            .map(|alt| self.translate(alt))
                            .collect()
                    )
                ),
            TypedDecoder::Tuple(gt, elts) =>
                match gt {
                    GenType::Inline(RustType::AnonTuple(_tys)) => {
                        CaseLogic::Sequential(SequentialLogic::AccumTuple {
                            constructor: None,
                            elements: elts
                                .iter()
                                .map(|elt| self.translate(elt))
                                .collect(),
                        })
                    }
                    GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::Unit))) if
                        elts.is_empty()
                    => {
                        CaseLogic::Simple(SimpleLogic::Eval(RustExpr::UNIT))
                    }
                    other =>
                        unreachable!(
                            "TypedDecoder::Tuple expected to have type RustType::AnonTuple(..) (or UNIT if empty), found {other:?}"
                        ),
                }
            TypedDecoder::Record(gt, flds) => {
                match gt {
                    | GenType::Def((_ix, lab), ..)
                    | GenType::Inline(
                          RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(_ix, lab))),
                      ) => {
                        let mut assocs = Vec::new();
                        for (l0, d) in flds.iter() {
                            assocs.push((l0.clone(), self.translate(d)));
                        }
                        CaseLogic::Sequential(SequentialLogic::AccumRecord {
                            constructor: Constructor::Simple(lab.clone()),
                            fields: assocs,
                        })
                    }
                    other =>
                        unreachable!(
                            "TypedDecoder::Record expected to have type Def(..) or Inline(Atom(TypeRef(..))), found {other:?}"
                        ),
                }
            }
            TypedDecoder::While(_gt, tree_continue, single) =>
                CaseLogic::Repeat(
                    RepeatLogic::ContinueOnMatch(
                        tree_continue.clone(),
                        Box::new(self.translate(single))
                    )
                ),

            TypedDecoder::Until(_gt, tree_break, single) =>
                CaseLogic::Repeat(
                    RepeatLogic::BreakOnMatch(tree_break.clone(), Box::new(self.translate(single)))
                ),

            TypedDecoder::RepeatCount(_gt, expr_count, single) =>
                CaseLogic::Repeat(
                    RepeatLogic::ExactCount(
                        embed_expr_t(expr_count),
                        Box::new(self.translate(single))
                    )
                ),
            TypedDecoder::RepeatUntilLast(_gt, pred_terminal, single) =>
                CaseLogic::Repeat(
                    RepeatLogic::ConditionTerminal(
                        embed_lambda_t(pred_terminal, ClosureKind::Predicate),
                        Box::new(self.translate(single))
                    )
                ),
            TypedDecoder::RepeatUntilSeq(_gt, pred_complete, single) => {
                CaseLogic::Repeat(
                    RepeatLogic::ConditionComplete(
                        embed_lambda_t(pred_complete, ClosureKind::Predicate),
                        Box::new(self.translate(single))
                    )
                )
            }
            TypedDecoder::Map(_gt, inner, f) => {
                let cl_inner = self.translate(inner);
                CaseLogic::Derived(
                    DerivedLogic::MapOf(
                        embed_lambda_t(f, ClosureKind::Transform),
                        Box::new(cl_inner)
                    )
                )
            }
            TypedDecoder::Compute(_t, expr) =>
                CaseLogic::Simple(SimpleLogic::Eval(embed_expr_t(expr))),
            TypedDecoder::Let(_t, name, expr, inner) => {
                let cl_inner = self.translate(inner);
                CaseLogic::Derived(
                    DerivedLogic::Let(name.clone(), embed_expr_t(expr), Box::new(cl_inner))
                )
            }
            TypedDecoder::Match(_t, scrutinee, cases) => {
                let scrutinized = embed_expr_t(scrutinee);
                let head = match scrutinee.get_type().unwrap().as_ref() {
                    GenType::Inline(RustType::Atom(AtomType::Comp(CompType::Box(..)))) =>
                        scrutinized.call_method("as_ref"),
                    GenType::Inline(RustType::Atom(AtomType::Comp(CompType::Vec(..)))) =>
                        scrutinized.call_method("as_slice"),
                    _ => scrutinized,
                };
                let mut cl_cases = Vec::new();
                for (pat, dec) in cases.iter() {
                    cl_cases.push((
                        MatchCaseLHS::Pattern(embed_pattern_t(pat)),
                        self.translate(dec),
                    ));
                }
                CaseLogic::Other(OtherLogic::ExprMatch(head, cl_cases))
            }
            TypedDecoder::Dynamic(_t, name, f_dyn, inner) => {
                let cl_inner = self.translate(inner);
                match f_dyn {
                    TypedDynFormat::Huffman(code_lengths, opt_values) => {
                        CaseLogic::Derived(
                            DerivedLogic::Dynamic(
                                DynamicLogic::Huffman(
                                    name.clone(),
                                    code_lengths.clone(),
                                    opt_values.clone()
                                ),
                                Box::new(cl_inner)
                            )
                        )
                    }
                }
            }
            TypedDecoder::Apply(_t, lab) => {
                CaseLogic::Simple(SimpleLogic::CallDynamic(lab.clone()))
            }
            // FIXME - missing logic
            TypedDecoder::Peek(_t, inner) => {
                let cl_inner = self.translate(inner);
                CaseLogic::Engine(EngineLogic::Peek(Box::new(cl_inner)))
            }
            // FIXME - missing logic
            TypedDecoder::PeekNot(_t, _inner) =>
                CaseLogic::Unhandled("translate @ Decoder::PeekNot".into()),
            TypedDecoder::Slice(_t, width, inner) => {
                let rexpr_width = embed_expr_t(width);
                let cl_inner = self.translate(inner);
                CaseLogic::Engine(EngineLogic::Slice(rexpr_width, Box::new(cl_inner)))
            }
            // FIXME - missing logic
            TypedDecoder::Bits(_t, _dec_bits) =>
                CaseLogic::Unhandled("translate @ Decoder::Bits".into()),
            // FIXME - missing logic
            TypedDecoder::WithRelativeOffset(_t, _offset, _inner) => {
                CaseLogic::Unhandled("translate @ Decoder::WithRelativeOffset".into())
            }
        }
    }
}

fn embed_pattern_t(pat: &GTPattern) -> RustPattern {
    match pat {
        TypedPattern::Tuple(_, elts) => match elts.as_slice() {
            [TypedPattern::Wildcard(..)] => RustPattern::Fill,
            _ => RustPattern::TupleLiteral(elts.iter().map(embed_pattern_t).collect()),
        },
        TypedPattern::Variant(gt, vname, inner) => match gt {
            GenType::Def((_, tname), _def) => {
                let constr = Constructor::Compound(tname.clone(), vname.clone());
                let inner_pat = match inner.as_ref() {
                    TypedPattern::Wildcard(..) => RustPattern::Fill,
                    _ => embed_pattern_t(inner),
                };
                RustPattern::Variant(constr, Box::new(inner_pat))
            }
            other => {
                unreachable!("cannot inline TypedPattern::Variant with abstract gentype: {other:?}")
            }
        },
        TypedPattern::Seq(_t, elts) => {
            RustPattern::ArrayLiteral(elts.iter().map(embed_pattern_t).collect())
        }
        TypedPattern::Wildcard(_) => RustPattern::CatchAll(None),
        TypedPattern::Binding(_, name) => RustPattern::CatchAll(Some(name.clone())),
        TypedPattern::Bool(b) => RustPattern::PrimLiteral(RustPrimLit::Boolean(*b)),
        TypedPattern::U8(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(*n as usize)),
        TypedPattern::U16(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(*n as usize)),
        TypedPattern::U32(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(*n as usize)),
        TypedPattern::U64(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(*n as usize)),
        TypedPattern::Char(c) => RustPattern::PrimLiteral(RustPrimLit::Char(*c)),
    }
}

fn embed_expr_t(expr: &TypedExpr<GenType>) -> RustExpr {
    match expr {
        TypedExpr::Record(gt, fields) => {
            let tname = match gt {
                GenType::Def((_, tname), _) => tname,
                GenType::Inline(
                    RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(_ix, tname))),
                ) => tname,
                other =>
                    unreachable!(
                        "TypedExpr::Record has unexpected type (looking for Def or Inline LocalDef): {other:?}"
                    ),
            };
            RustExpr::Struct(
                RustEntity::Local(tname.clone()),
                fields
                    .iter()
                    .map(|(fname, fval)| (fname.clone(), Some(Box::new(embed_expr_t(fval)))))
                    .collect()
            )
        }
        TypedExpr::Variant(gt, vname, inner) => {
            match gt {
                GenType::Def((_ix, tname), def) => {
                    match def {
                        RustTypeDef::Enum(vars) => {
                            let Some(this) = vars.iter().find(|var| var.get_label() == vname) else {
                                unreachable!("Variant not found: {:?}::{:?}", tname, vname)
                            };
                            let constr_ent = RustEntity::Scoped(vec![tname.clone()], vname.clone());
                            match this {
                                RustVariant::Unit(_vname) =>
                                    RustExpr::BlockScope(
                                        vec![RustStmt::Expr(embed_expr_t(inner))],
                                        Box::new(RustExpr::Entity(constr_ent))
                                    ),
                                RustVariant::Tuple(_vname, _elts) => {
                                    // FIXME - not sure how to avoid unary-over-tuple if inner becomes RustExpr::Tuple...
                                    RustExpr::Entity(constr_ent).call_with([embed_expr_t(inner)])
                                }
                                RustVariant::Record(_vname, _flds) =>
                                    match inner.as_ref() {
                                        TypedExpr::Record(_gt, fields) =>
                                            RustExpr::Struct(
                                                constr_ent,
                                                fields
                                                    .iter()
                                                    .map(|(fname, fval)| {
                                                        (
                                                            fname.clone(),
                                                            Some(Box::new(embed_expr_t(fval))),
                                                        )
                                                    })
                                                    .collect()
                                            ),
                                        other =>
                                            unreachable!(
                                                "Record variant found non-record inner Expr: {other:?}"
                                            ),
                                    }
                            }
                        }
                        RustTypeDef::Struct(_) => {
                            unreachable!("Variant has non-enum type information")
                        }
                    }
                }
                other =>
                    unreachable!(
                        "Cannot embed variant expression with inlined (abstract) GenType: {other:?}"
                    ),
            }
        }
        TypedExpr::Match(_t, scrutinee, cases) => {
            let scrutinized = embed_expr_t(scrutinee);
            let head = match scrutinee.get_type().unwrap().as_ref() {
                GenType::Inline(
                    RustType::Atom(
                        AtomType::Comp(
                            | CompType::Box(..)
                            | CompType::Vec(..)
                            | CompType::Array(..)
                            | CompType::Slice(..),
                        ),
                    ),
                ) => scrutinized.call_method("as_ref"),
                _ => scrutinized,
            };
            RustExpr::Control(
                Box::new(
                    RustControl::Match(
                        head,
                        add_error_catchall(
                            cases
                                .iter()
                                .map(|(pat, rhs)| {
                                    (
                                        MatchCaseLHS::Pattern(embed_pattern_t(pat)),
                                        vec![
                                            RustStmt::Return(
                                                ReturnKind::Implicit,
                                                embed_expr_t(rhs)
                                            )
                                        ],
                                    )
                                })
                        )
                    )
                )
            )
        }
        TypedExpr::Tuple(_t, tup) => RustExpr::Tuple(tup.iter().map(embed_expr_t).collect()),
        TypedExpr::TupleProj(_, expr_tup, ix) => embed_expr_t(expr_tup).nth(*ix),
        TypedExpr::RecordProj(_, expr_rec, fld) => embed_expr_t(expr_rec).field(fld.clone()),
        TypedExpr::Seq(_, elts) => {
            RustExpr::ArrayLit(elts.iter().map(embed_expr_t).collect()).call_method("to_vec")
        }
        TypedExpr::Arith(_, arith, lhs, rhs) => {
            let x = embed_expr_t(lhs);
            let y = embed_expr_t(rhs);
            let op = match arith {
                Arith::BitAnd => Operator::BitAnd,
                Arith::BitOr => Operator::BitOr,
                Arith::Add => Operator::Add,
                Arith::Sub => Operator::Sub,
                Arith::Mul => Operator::Mul,
                Arith::Div => Operator::Div,
                Arith::Rem => Operator::Rem,
                Arith::Shl => Operator::Shl,
                Arith::Shr => Operator::Shr,
            };
            RustExpr::infix(x, op, y)
        }

        TypedExpr::IntRel(_, rel, lhs, rhs) => {
            let x = embed_expr_t(lhs);
            let y = embed_expr_t(rhs);
            let op = match rel {
                IntRel::Eq => Operator::Eq,
                IntRel::Ne => Operator::Neq,
                IntRel::Lt => Operator::Lt,
                IntRel::Gt => Operator::Gt,
                IntRel::Lte => Operator::Lte,
                IntRel::Gte => Operator::Gte,
            };
            RustExpr::infix(x, op, y)
        }
        TypedExpr::AsU8(x) =>
            RustExpr::Operation(RustOp::AsCast(Box::new(embed_expr_t(x)), PrimType::U8.into())),
        TypedExpr::AsU16(x) =>
            RustExpr::Operation(RustOp::AsCast(Box::new(embed_expr_t(x)), PrimType::U16.into())),
        TypedExpr::AsU32(x) =>
            RustExpr::Operation(RustOp::AsCast(Box::new(embed_expr_t(x)), PrimType::U32.into())),
        TypedExpr::AsU64(x) =>
            RustExpr::Operation(RustOp::AsCast(Box::new(embed_expr_t(x)), PrimType::U64.into())),
        TypedExpr::U16Be(be_bytes) => RustExpr::local("u16be").call_with([embed_expr_t(be_bytes)]),
        TypedExpr::U16Le(le_bytes) => RustExpr::local("u16le").call_with([embed_expr_t(le_bytes)]),
        TypedExpr::U32Be(be_bytes) => RustExpr::local("u32be").call_with([embed_expr_t(be_bytes)]),
        TypedExpr::U32Le(le_bytes) => RustExpr::local("u32le").call_with([embed_expr_t(le_bytes)]),
        TypedExpr::U64Be(be_bytes) => RustExpr::local("u64be").call_with([embed_expr_t(be_bytes)]),
        TypedExpr::U64Le(le_bytes) => RustExpr::local("u64le").call_with([embed_expr_t(le_bytes)]),
        TypedExpr::AsChar(codepoint) =>
            RustExpr::scoped(["char"], "from_u32")
                .call_with([embed_expr_t(codepoint)])
                .call_method("unwrap"),
        TypedExpr::SeqLength(seq) => embed_expr_t(seq).call_method("len"),
        TypedExpr::SubSeq(_, seq, ix, len) => {
            let start_expr = embed_expr_t(ix);
            let bind_ix = RustStmt::assign("ix", start_expr);
            let end_expr = RustExpr::infix(RustExpr::local("ix"), Operator::Add, embed_expr_t(len));
            RustExpr::BlockScope(
                vec![bind_ix],
                Box::new(
                    RustExpr::Slice(
                        Box::new(embed_expr_t(seq)),
                        Box::new(RustExpr::local("ix")),
                        Box::new(end_expr)
                    )
                )
            )
        }
        TypedExpr::FlatMap(_, f, seq) =>
            embed_expr_t(seq)
                .call_method("iter")
                .call_method("cloned")
                .call_method_with("flat_map", [embed_lambda_t(f, ClosureKind::Transform)])
                .call_method("collect"),
        TypedExpr::FlatMapAccum(_, f, acc_init, _acc_type, seq) =>
            embed_expr_t(seq)
                .call_method("iter")
                .call_method("cloned")
                .call_method_with("fold", [
                    embed_expr_t(acc_init),
                    embed_lambda_t(f, ClosureKind::Transform),
                ])
                .call_method("collect"),
        TypedExpr::Dup(_, n, expr) => {
            RustExpr::local("dup32").call_with([embed_expr_t(n), embed_expr_t(expr)])
        }
        TypedExpr::Inflate(..) => {
            // FIXME - missing logic
            RustExpr::local("unimplemented!").call_with([
                RustExpr::str_lit("embed_expr is not implemented for Expr::Inflate"),
            ])
        }
        TypedExpr::Var(_, vname) => {
            // FIXME - lexical scopes, shadowing, and variable-name sanitization may not be quite right in the current implementation
            RustExpr::local(vname.clone())
        }
        TypedExpr::Bool(b) => RustExpr::PrimitiveLit(RustPrimLit::Boolean(*b)),
        TypedExpr::U8(n) => RustExpr::num_lit(*n as usize),
        TypedExpr::U16(n) => RustExpr::num_lit(*n as usize),
        TypedExpr::U32(n) => RustExpr::num_lit(*n as usize),
        TypedExpr::U64(n) => RustExpr::num_lit(*n as usize),
        TypedExpr::Lambda(_, _, _) =>
            unreachable!(
                "TypedExpr::Lambda unsupported as first-class embed (requires embed_lambda_t with proper ClosureKind argument)"
            ),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ClosureKind {
    Predicate,
    Transform,
}

fn embed_lambda_t(expr: &GTExpr, kind: ClosureKind) -> RustExpr {
    match expr {
        TypedExpr::Lambda((head_t, _), head, body) => match kind {
            ClosureKind::Predicate => {
                RustExpr::Paren(Box::new(RustExpr::Closure(RustClosure::new_predicate(
                    head.clone(),
                    Some(head_t.clone().to_rust_type()),
                    embed_expr_t(body),
                ))))
            }
            ClosureKind::Transform => {
                RustExpr::Paren(Box::new(RustExpr::Closure(RustClosure::new_transform(
                    head.clone(),
                    Some(head_t.clone().to_rust_type()),
                    embed_expr_t(body),
                ))))
            }
        },
        _other => unreachable!("embed_lambda_t expects a lambda, found {_other:?}"),
    }
}

type RustBlock = (Vec<RustStmt>, Option<RustExpr>);

#[derive(Clone, Copy)]
pub(crate) struct ProdCtxt<'a> {
    input_varname: &'a Label,
    scope_varname: &'a Label,
}

impl<'a> Default for ProdCtxt<'a> {
    fn default() -> Self {
        Self {
            input_varname: &Cow::Borrowed(""),
            scope_varname: &Cow::Borrowed(""),
        }
    }
}

macro_rules! impl_toast_caselogic {
    ($($t:ident),+) => {
        $(
        impl ToAst for CaseLogic<$t>
        {
            type AstElem = RustBlock;

            /// Produces an RustExpr-valued AST for the given CaseLogic instance.
            ///
            /// The ExprT should have the bare type of the value being parsed (i.e. not Option-wrapped),
            /// but it is implicitly assumed to be contained in a block whose ultimate return value
            /// is `Option<_>`, allowing `return None` and `?` expressions to be used anyway.
            ///
            /// Local bindings and control flow are allowed, as long as an explicit return
            /// or a concrete, consistently-typed return value are used
            #[allow(dead_code)]
            fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
                match self {
                    CaseLogic::Derived(d) => d.to_ast(ctxt),
                    CaseLogic::Engine(e) => e.to_ast(ctxt),
                    CaseLogic::Other(o) => o.to_ast(ctxt),
                    CaseLogic::Parallel(p) => p.to_ast(ctxt),
                    CaseLogic::Repeat(r) => r.to_ast(ctxt),
                    CaseLogic::Sequential(sq) => sq.to_ast(ctxt),
                    CaseLogic::Simple(s) => s.to_ast(ctxt),
                    CaseLogic::Unhandled(msg) => (
                        Vec::new(),
                        Some(RustExpr::local("unimplemented!").call_with([RustExpr::str_lit(msg.clone())])),
                    ),
                }
            }
        }
        )+
    };
}

impl_toast_caselogic!(Expr, GTExpr);

impl<ExprT> SimpleLogic<ExprT> {
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock
    where
        ExprT: Clone,
    {
        match self {
            SimpleLogic::Fail => (
                vec![RustStmt::Return(
                    ReturnKind::Keyword,
                    RustExpr::err(RustExpr::scoped(["ParseError"], "FailToken")),
                )],
                None,
            ),
            SimpleLogic::ExpectEnd => {
                let call = RustExpr::local(ctxt.input_varname.clone()).call_method("remaining");
                let cond = RustExpr::infix(call, Operator::Eq, RustExpr::num_lit(0usize));
                let b_true = [RustStmt::Return(ReturnKind::Implicit, RustExpr::UNIT)];
                let b_false = [RustStmt::Return(
                    ReturnKind::Keyword,
                    RustExpr::local("Err")
                        .call_with([RustExpr::scoped(["ParseError"], "IncompleteParse")]),
                )];
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
            SimpleLogic::CallDynamic(dynf_name) => {
                let call = RustExpr::local(dynf_name.clone()).call_with([
                    RustExpr::local(ctxt.scope_varname.clone()),
                    RustExpr::local(ctxt.input_varname.clone()),
                ]);
                (Vec::new(), Some(call.wrap_try()))
            }
            SimpleLogic::SkipToNextMultiple(n) => {
                // FIXME - this currently produces correct but inefficient code
                // it is harder to write, but much more efficient, to cut the buffer at the right place
                // in order to do so, we would need a more advanced Parser model or more complex inline logic
                (
                    vec![RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method_with("skip_align", [RustExpr::num_lit(*n)])
                            .wrap_try(),
                    )],
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
                        RustExpr::local("Err")
                            .call_with([RustExpr::scoped(["ParseError"], "ExcludedBranch")]),
                    )];
                    RustExpr::Control(Box::new(RustControl::If(cond, b_true, Some(b_false))))
                };
                ([b_let].to_vec(), Some(logic))
            }
            SimpleLogic::Eval(expr) => (vec![], Some(expr.clone())),
        }
    }
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

fn abstracted_try_block(block: RustBlock) -> RustExpr {
    let (stmts, ret) = block;
    RustExpr::Closure(RustClosure::new_thunk(
        RustExpr::scoped(["PResult"], "Ok").call_with([RustExpr::BlockScope(
            stmts,
            Box::new(ret.unwrap_or(RustExpr::UNIT)),
        )]),
    ))
}

// follows the same rules as CaseLogic::to_ast as far as the expression type of the generated code
fn embed_matchtree(tree: &MatchTree, ctxt: ProdCtxt<'_>) -> RustBlock {
    fn expand_matchtree(tree: &MatchTree, ctxt: ProdCtxt<'_>) -> RustBlock {
        if tree.branches.is_empty() {
            if let Some(ix) = tree.accept {
                return (Vec::new(), Some(RustExpr::num_lit(ix)));
            } else {
                let err_val = RustExpr::scoped(["ParseError"], "ExcludedBranch");
                return (
                    vec![RustStmt::Return(
                        ReturnKind::Keyword,
                        RustExpr::err(err_val),
                    )],
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
                        let err_val = RustExpr::scoped(["ParseError"], "ExcludedBranch");
                        vec![RustStmt::Return(
                            ReturnKind::Keyword,
                            RustExpr::err(err_val),
                        )]
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
        let matchblock = RustControl::Match(RustExpr::local("b"), add_error_catchall(cases));
        (vec![bind], Some(RustExpr::Control(Box::new(matchblock))))
    }

    let open_peek = RustStmt::Expr(
        RustExpr::local(ctxt.input_varname.clone()).call_method("open_peek_context"),
    );

    // this is a stub for non-ParseMonad models to replace the parser context with another
    let ll_context = ProdCtxt { ..ctxt };

    let (stmts, expr) = expand_matchtree(tree, ll_context);
    let close_peek = RustStmt::Expr(
        RustExpr::local(ctxt.input_varname.clone())
            .call_method("close_peek_context")
            .wrap_try(),
    );

    match expr {
        Some(expr) => (
            std::iter::once(open_peek)
                .chain(stmts.into_iter())
                .collect(),
            Some(RustExpr::BlockScope(
                vec![RustStmt::assign("ret", expr), close_peek],
                Box::new(RustExpr::local("ret")),
            )),
        ),
        None => (
            std::iter::once(open_peek)
                .chain(stmts.into_iter())
                .chain(std::iter::once(close_peek))
                .collect(),
            None,
        ),
    }
}

/// Abstraction type use to sub-categorize different Decoders and ensure that the codegen layer
/// is more resilient to changes both upstream (in the Decoder model)
/// and downstream (in the API made available for generated code to use)
#[derive(Clone, Debug)]
enum CaseLogic<ExprT = Expr> {
    Simple(SimpleLogic<ExprT>),
    Derived(DerivedLogic<ExprT>),
    Sequential(SequentialLogic<ExprT>),
    Parallel(ParallelLogic<ExprT>),
    Repeat(RepeatLogic<ExprT>),
    Engine(EngineLogic<ExprT>),
    Other(OtherLogic<ExprT>),
    Unhandled(Label),
}

#[derive(Clone, Debug)]
enum EngineLogic<ExprT> {
    Slice(RustExpr, Box<CaseLogic<ExprT>>),
    Peek(Box<CaseLogic<ExprT>>),
}

impl<ExprT> ToAst for EngineLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = RustBlock>,
{
    type AstElem = RustBlock;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            EngineLogic::Slice(sz, cl_inner) => (
                vec![
                    RustStmt::assign("sz", sz.clone()),
                    RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method_with("start_slice", [RustExpr::local("_sz")])
                            .wrap_try(),
                    ),
                    RustStmt::assign(
                        "ret",
                        abstracted_try_block(cl_inner.to_ast(ctxt))
                            .call()
                            .wrap_try(),
                    ),
                    RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method("end_slice")
                            .wrap_try(),
                    ),
                ],
                Some(RustExpr::local("ret")),
            ),
            EngineLogic::Peek(cl_inner) => (
                vec![
                    RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method("open_peek_context"),
                    ),
                    RustStmt::assign(
                        "ret",
                        abstracted_try_block(cl_inner.to_ast(ctxt))
                            .call()
                            .wrap_try(),
                    ),
                    RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method("close_peek_context")
                            .wrap_try(),
                    ),
                ],
                Some(RustExpr::local("ret")),
            ),
        }
    }
}

/// Cases where a constant block of logic is repeated (0 or more times)
#[derive(Clone, Debug)]
enum RepeatLogic<ExprT> {
    ContinueOnMatch(MatchTree, Box<CaseLogic<ExprT>>), // evaluates a matchtree and continues if it is matched
    BreakOnMatch(MatchTree, Box<CaseLogic<ExprT>>), // evaluates a matchtree and breaks if it is matched
    ExactCount(RustExpr, Box<CaseLogic<ExprT>>),    // repeats a specific numnber of times
    ConditionTerminal(RustExpr, Box<CaseLogic<ExprT>>), // stops when a predicate for 'terminal element' is satisfied
    ConditionComplete(RustExpr, Box<CaseLogic<ExprT>>), // stops when a predicate for 'complete sequence' is satisfied
}

pub(crate) trait ToAst {
    type AstElem;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> Self::AstElem;
}

impl<ExprT> ToAst for RepeatLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = RustBlock>,
{
    type AstElem = RustBlock;

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
                        RustExpr::infix(
                            RustExpr::local(ctxt.input_varname.clone()).call_method("remaining"),
                            Operator::Gt,
                            RustExpr::num_lit(0usize),
                        ),
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
                        RustExpr::infix(
                            RustExpr::local(ctxt.input_varname.clone()).call_method("remaining"),
                            Operator::Gt,
                            RustExpr::num_lit(0usize),
                        ),
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
                    RustStmt::Control(RustControl::Loop(vec![
                        elt_bind,
                        RustStmt::Control(escape_clause),
                    ]))
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
                    RustStmt::Control(RustControl::Loop(vec![
                        elt_bind,
                        elt_push,
                        RustStmt::Control(escape_clause),
                    ]))
                };
                stmts.push(ctrl);
                (stmts, Some(RustExpr::local("accum")))
            }
        }
    }
}

/// Cases that apply other case-logic in sequence to an incrementally updated input
#[derive(Clone, Debug)]
enum SequentialLogic<ExprT> {
    AccumTuple {
        constructor: Option<Constructor>,
        elements: Vec<CaseLogic<ExprT>>,
    },
    AccumRecord {
        constructor: Constructor,
        fields: Vec<(Label, CaseLogic<ExprT>)>,
    },
}

impl<ExprT> ToAst for SequentialLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = RustBlock>,
{
    type AstElem = RustBlock;

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
                    let elt_thunk = abstracted_try_block(elt_cl.to_ast(ctxt));
                    body.push(RustStmt::assign(varname, elt_thunk.call().wrap_try()));
                }

                if let Some(con) = constructor {
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
                    let fld_thunk = abstracted_try_block(fld_cl.to_ast(ctxt));
                    body.push(RustStmt::assign(varname, fld_thunk.call().wrap_try()));
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
enum OtherLogic<ExprT> {
    Descend(MatchTree, Vec<CaseLogic<ExprT>>),
    ExprMatch(RustExpr, Vec<(MatchCaseLHS, CaseLogic<ExprT>)>),
}

fn add_panic_catchall(
    cases: impl IntoIterator<Item = (MatchCaseLHS, Vec<RustStmt>)>,
) -> Vec<(MatchCaseLHS, Vec<RustStmt>)> {
    cases
        .into_iter()
        .chain(std::iter::once((
            MatchCaseLHS::Pattern(RustPattern::CatchAll(Some(Label::Borrowed("_other")))),
            vec![RustStmt::Expr(
                RustExpr::local("unreachable!")
                    .call_with([RustExpr::str_lit("bad value {_other:?}")]),
            )],
        )))
        .collect()
}

fn add_error_catchall(
    cases: impl IntoIterator<Item = (MatchCaseLHS, Vec<RustStmt>)>,
) -> Vec<(MatchCaseLHS, Vec<RustStmt>)> {
    cases
        .into_iter()
        .chain(std::iter::once((
            MatchCaseLHS::Pattern(RustPattern::CatchAll(Some(Label::Borrowed("_other")))),
            vec![RustStmt::Return(
                ReturnKind::Keyword,
                RustExpr::err(RustExpr::scoped(["ParseError"], "ExcludedBranch")),
            )],
        )))
        .collect()
}

impl<ExprT> ToAst for OtherLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = RustBlock>,
{
    type AstElem = RustBlock;

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
                    add_error_catchall(branches),
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
                let ret = RustExpr::Control(Box::new(RustControl::Match(
                    expr.clone(),
                    add_panic_catchall(branches),
                )));
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
enum ParallelLogic<ExprT> {
    Alts(Vec<CaseLogic<ExprT>>),
}

impl<ExprT> ToAst for ParallelLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = RustBlock>,
{
    type AstElem = RustBlock;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            ParallelLogic::Alts(alts) => {
                let l = alts.len();
                let stmts = Iterator::chain(
                    std::iter::once(RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone()).call_method("start_alt"),
                    )),
                    alts.iter().enumerate().map(|(ix, branch_cl)| {
                        let on_err = match l - ix {
                            0 => unreachable!("index matches overall length"),
                            1 => RustStmt::Return(
                                ReturnKind::Keyword,
                                RustExpr::err(RustExpr::local("_e")),
                            ),
                            2 => RustStmt::Expr(
                                RustExpr::local(ctxt.input_varname.clone())
                                    .call_method_with("next_alt", [RustExpr::TRUE]),
                            ),
                            3.. => RustStmt::Expr(
                                RustExpr::local(ctxt.input_varname.clone())
                                    .call_method_with("next_alt", [RustExpr::FALSE]),
                            ),
                            _ => unreachable!("usize bounds are weird???"),
                        };
                        let thunk = abstracted_try_block(branch_cl.to_ast(ctxt).into());
                        RustStmt::Expr(RustExpr::BlockScope(
                            [RustStmt::assign("f_tmp", thunk)].to_vec(),
                            Box::new(RustExpr::Control(Box::new(RustControl::Match(
                                RustExpr::local("f_tmp").call(),
                                vec![
                                    (
                                        MatchCaseLHS::Pattern(RustPattern::Variant(
                                            Constructor::Simple(Label::from("Ok")),
                                            Box::new(RustPattern::CatchAll(Some(Label::from(
                                                "inner",
                                            )))),
                                        )),
                                        [RustStmt::Return(
                                            ReturnKind::Keyword,
                                            RustExpr::ok(RustExpr::local("inner")),
                                        )]
                                        .to_vec(),
                                    ),
                                    (
                                        MatchCaseLHS::Pattern(RustPattern::Variant(
                                            Constructor::Simple(Label::from("Err")),
                                            Box::new(RustPattern::CatchAll(Some(Label::from(
                                                "_e",
                                            )))),
                                        )),
                                        [on_err].to_vec(),
                                    ),
                                ],
                            )))),
                        ))
                    }),
                )
                .collect();
                let failsafe = RustExpr::local("panic!").call_with([RustExpr::str_lit(
                    "last branch should return something unconditionally",
                )]);
                (stmts, Some(failsafe))
            }
        }
    }
}

/// Cases that require no recursion into other case-logic
#[derive(Clone, Debug)]
enum SimpleLogic<ExprT> {
    Fail,
    ExpectEnd,
    Invoke(usize, Vec<(Label, ExprT)>),
    SkipToNextMultiple(usize),
    ByteIn(ByteSet),
    Eval(RustExpr),
    CallDynamic(Label),
}

/// Cases that recurse into other case-logic only once
#[derive(Clone, Debug)]
enum DerivedLogic<ExprT> {
    VariantOf(Constructor, Box<CaseLogic<ExprT>>),
    UnitVariantOf(Constructor, Box<CaseLogic<ExprT>>),
    MapOf(RustExpr, Box<CaseLogic<ExprT>>),
    Let(Label, RustExpr, Box<CaseLogic<ExprT>>),
    Dynamic(DynamicLogic<ExprT>, Box<CaseLogic<TypedExpr<GenType>>>),
}

#[derive(Clone, Debug)]
enum DynamicLogic<ExprT> {
    Huffman(Label, ExprT, Option<ExprT>),
}

impl<ExprT> ToAst for DynamicLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = RustBlock>,
{
    type AstElem = RustStmt;

    fn to_ast(&self, _ctxt: ProdCtxt<'_>) -> Self::AstElem {
        match self {
            DynamicLogic::Huffman(lbl, _code_lengths, _opt_values_expr) => {
                let rhs = {
                    // FIXME - missing logic
                    let logic = RustExpr::local("unimplemented!").call_with([RustExpr::str_lit(
                        "no implementation for for DynamicLogic::Huffman AST-transcription",
                    )]);
                    logic
                };
                RustStmt::Let(Mut::Immutable, lbl.clone(), None, rhs)
            }
        }
    }
}

impl<ExprT> ToAst for DerivedLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = RustBlock>,
{
    type AstElem = RustBlock;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            DerivedLogic::Dynamic(dynl, inner_cl) => {
                let (init, last) = inner_cl.to_ast(ctxt);
                (
                    Iterator::chain(std::iter::once(dynl.to_ast(ctxt)), init.into_iter()).collect(),
                    last,
                )
            }
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

pub fn print_generated_code(module: &FormatModule, top_format: &Format) {
    let mut items = Vec::new();

    let Generator {
        sourcemap,
        elaborator,
    } = Generator::compile(module, top_format);
    let tdefs = Vec::from_iter(elaborator.codegen.defined_types.iter());
    for (ix, tdef) in tdefs.into_iter().enumerate() {
        let it = RustItem::from_decl(RustDecl::TypeDef(IxLabel::from(ix).into(), tdef.clone()));
        items.push(it);
    }

    for decfn in sourcemap.decoder_skels.iter() {
        items.push(RustItem::from_decl(RustDecl::Function(
            decfn.to_ast(ProdCtxt::default()),
        )));
    }

    let mut content = RustProgram::from_iter(items);
    content.add_import(RustImport {
        path: vec!["doodle".into(), "prelude".into()],
        uses: RustImportItems::Wildcard,
    });

    let extra = r#"
#[test]

fn test_decoder_28() {
    // PNG signature
    let input = b"\x89PNG\r\n\x1A\n";
    let mut parse_ctxt = ParseCtxt::new(input);
    let mut scope = Scope::Empty;
    let ret = Decoder28(&mut scope, &mut parse_ctxt);
    assert!(ret.is_some());
}"#;

    print!(
        r#"
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    {}
    {}"#,
        content.to_fragment(),
        extra
    )
}

#[derive(Clone, Debug)]
pub struct DecoderFn<ExprT>(IxLabel, CaseLogic<ExprT>, RustType);

impl<ExprT> ToAst for DecoderFn<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = RustBlock>,
    ExprT: std::fmt::Debug,
{
    type AstElem = RustFn;

    fn to_ast(&self, _ctxt: ProdCtxt<'_>) -> RustFn {
        let name = Label::from(format!("Decoder{}", self.0.to_usize()));
        let params = {
            let mut tmp = DefParams::new();
            tmp.push_lifetime("'input");
            tmp
        };
        let sig = {
            let args = {
                let arg0 = {
                    let name = "scope".into();
                    let ty = {
                        let mut params = RustParams::<RustLt, RustType>::new();
                        params.push_lifetime(RustLt::Parametric("'input".into()));
                        RustType::borrow_of(
                            None,
                            Mut::Mutable,
                            RustType::verbatim("Scope", Some(params)),
                        )
                    };
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
                            RustType::verbatim("ParseMonad", Some(params)),
                        )
                    };
                    (name, ty)
                };
                [arg0, arg1].to_vec()
            };
            FnSig::new(
                args,
                Some(RustType::result_of(
                    self.2.clone(),
                    RustType::imported("ParseError"),
                )),
            )
        };
        let ctxt = ProdCtxt {
            input_varname: &Label::from("input"),
            scope_varname: &Label::from("scope"),
        };
        let (stmts, ret) = self.1.to_ast(ctxt);
        let body = Iterator::chain(
            stmts.into_iter(),
            std::iter::once(RustStmt::Return(
                ReturnKind::Implicit,
                RustExpr::ok(ret.unwrap()),
            )),
        )
        .collect();
        RustFn::new(name, Some(params), sig, body)
    }
}

#[derive(Clone, Debug)]
pub struct SourceMap<ExprT> {
    pub(crate) decoder_skels: Vec<DecoderFn<ExprT>>,
}

impl<TypeRep> SourceMap<TypeRep> {
    pub const fn new() -> SourceMap<TypeRep> {
        SourceMap {
            decoder_skels: Vec::new(),
        }
    }
}

// pub struct RustTypeScope<'a> {
//     parent: Option<&'a RustTypeScope<'a>>,
//     names: Vec<Label>,
//     rtypes: Vec<RustType>,
// }

// impl<'a> RustTypeScope<'a> {
//     fn new() -> Self {
//         let parent = None;
//         let names = Vec::new();
//         let rtypes = Vec::new();
//         RustTypeScope {
//             parent,
//             names,
//             rtypes,
//         }
//     }

//     fn child(parent: &'a RustTypeScope<'a>) -> Self {
//         let parent = Some(parent);
//         let names = Vec::new();
//         let rtypes = Vec::new();
//         RustTypeScope {
//             parent,
//             names,
//             rtypes,
//         }
//     }

//     fn push(&mut self, name: Label, rt: RustType) {
//         self.names.push(name);
//         self.rtypes.push(rt);
//     }

//     fn push_format(&mut self, name: Label, rt: RustType) {
//         self.names.push(name);
//         self.rtypes.push(rt);
//     }

//     fn get_type_by_name(&self, name: &str) -> &RustType {
//         for (i, n) in self.names.iter().enumerate().rev() {
//             if n == name {
//                 return &self.rtypes[i];
//             }
//         }
//         if let Some(scope) = self.parent {
//             scope.get_type_by_name(name)
//         } else {
//             panic!("variable not found: {name}");
//         }
//     }
// }

pub struct Generator<'a> {
    pub(crate) elaborator: Elaborator<'a>,
    pub(crate) sourcemap: SourceMap<GTExpr>,
}

impl<'a> Generator<'a> {
    pub fn compile(module: &'a FormatModule, top_format: &Format) -> Self {
        let mut tc = TypeChecker::new();
        let ctxt = crate::typecheck::Ctxt::new(module, &UScope::Empty);
        let _ = tc
            .infer_utype_format(top_format, ctxt)
            .unwrap_or_else(|err| panic!("Failed to infer topl-level format type: {err}"));
        let mut gen = Self {
            elaborator: Elaborator::new(module, tc, Codegen::new()),
            sourcemap: SourceMap::new(),
        };
        let elab = &mut gen.elaborator;

        let top = elab.elaborate_format(top_format, &TypedDynScope::Empty);
        // assert_eq!(elab.next_index, elab.tc.size());
        let prog = GTCompiler::compile_program(module, &top).expect("failed to compile program");
        for (ix, (dec, t)) in prog.decoders.iter().enumerate() {
            let dec_fn = {
                let cl = elab.codegen.translate(dec);
                DecoderFn(IxLabel::from(ix), cl, t.clone().to_rust_type())
            };
            gen.sourcemap.decoder_skels.push(dec_fn);
        }
        gen
    }
}

pub struct Elaborator<'a> {
    module: &'a FormatModule,
    next_index: usize,
    t_formats: HashMap<usize, Rc<GTFormat>>,
    tc: TypeChecker,
    codegen: Codegen,
}

impl<'a> Elaborator<'a> {
    /// Increment the current `next_index` by 1 and return its un-incremented value.
    pub fn get_and_increment_index(&mut self) -> usize {
        let ret = self.next_index;
        self.next_index += 1;
        ret
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn iter_defined_types<'b: 'a>(
        &'b self,
    ) -> impl Iterator<Item = &'a RustTypeDef> + 'b {
        self.codegen.defined_types.iter()
    }

    /// Increment the current `tree_index` by 1.
    pub fn increment_index(&mut self) {
        self.next_index += 1;
    }

    /// Return the current `tree_index` without mutation.
    pub fn get_index(&self) -> usize {
        self.next_index
    }

    fn elaborate_dynamic_format<'s>(&mut self, dynf: &DynFormat) -> TypedDynFormat<GenType> {
        match dynf {
            DynFormat::Huffman(code_lengths, opt_values_expr) => {
                // for dynf itself
                self.increment_index();
                let t_codes = self.elaborate_expr(code_lengths);
                // for the element-type of code_lengths
                self.increment_index();

                let t_values_expr = opt_values_expr.as_ref().map(|values_expr| {
                    let t_values = self.elaborate_expr(values_expr);
                    // for the element-type of opt_values_expr
                    self.increment_index();
                    t_values
                });
                GTDynFormat::Huffman(t_codes, t_values_expr)
            }
        }
    }

    fn elaborate_pattern(&mut self, pat: &Pattern) -> TypedPattern<GenType> {
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
                    let t_elt = self.elaborate_pattern(elt);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                GTPattern::Tuple(gt, t_elts)
            }
            Pattern::Variant(name, inner) => {
                let t_inner = self.elaborate_pattern(inner);
                let gt = self.get_gt_from_index(index);
                GTPattern::Variant(gt, name.clone(), Box::new(t_inner))
            }
            Pattern::Seq(elts) => {
                // for type of element
                self.increment_index();
                let mut t_elts = Vec::with_capacity(elts.len());
                for elt in elts {
                    let t_elt = self.elaborate_pattern(elt);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                GTPattern::Seq(gt, t_elts)
            }
        }
    }

    pub fn new(module: &'a FormatModule, tc: TypeChecker, codegen: Codegen) -> Self {
        Self {
            module,
            next_index: 0,
            t_formats: HashMap::new(),
            tc,
            codegen,
        }
    }

    fn elaborate_format_union(
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
                    let t_inner = self.elaborate_format(inner, dyns);
                    GTFormat::Variant(gt.clone(), name.clone(), Box::new(t_inner))
                }
                _ => self.elaborate_format(branch, dyns),
            };
            t_branches.push(t_branch);
        }

        if is_det {
            GTFormat::Union(gt, t_branches)
        } else {
            GTFormat::UnionNondet(gt, t_branches)
        }
    }

    fn elaborate_format(&mut self, format: &Format, dyns: &TypedDynScope<'_>) -> GTFormat {
        match format {
            Format::ItemVar(level, args) => {
                let index = self.get_and_increment_index();
                let fm_args = &self.module.args[*level];
                let mut t_args = Vec::with_capacity(args.len());
                for ((lbl, _), arg) in Iterator::zip(fm_args.iter(), args.iter()) {
                    let t_arg = self.elaborate_expr(arg);
                    t_args.push((lbl.clone(), t_arg));
                }
                let t_inner = if let Some(val) = self.t_formats.get(level) {
                    val.clone()
                } else {
                    let fmt = self.module.get_format(*level);
                    let tmp = self.elaborate_format(fmt, &TypedDynScope::Empty);
                    let ret = Rc::new(tmp);
                    self.t_formats.insert(*level, ret.clone());
                    ret
                };
                let gt = self.get_gt_from_index(index);
                GTFormat::FormatCall(gt, *level, t_args, t_inner)
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
            Format::Variant(label, inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                match gt.try_as_adhoc() {
                    Some(_) => (),
                    None => {
                        let before = self.get_gt_from_index(index - 1);
                        let after = self.get_gt_from_index(index + 1);
                        eprintln!("Possible frame-shift error around {index} (looking for Enum)");
                        eprintln!("[{}]: {before:?}", index - 1);
                        eprintln!("[{}]: {gt:?}", index);
                        eprintln!("[{}]: {after:?}", index + 1);
                        // unreachable!("found non-adhoc type for variant format elaboration: {gt:?} @ {index} ({label}({inner:?})");
                    }
                }
                GTFormat::Variant(gt, label.clone(), Box::new(t_inner))
            }
            Format::Union(branches) => self.elaborate_format_union(branches, dyns, true),
            Format::UnionNondet(branches) => self.elaborate_format_union(branches, dyns, false),
            Format::Tuple(elts) => {
                let index = self.get_and_increment_index();
                let mut t_elts = Vec::with_capacity(elts.len());
                for t in elts {
                    let t_elt = self.elaborate_format(t, dyns);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                GTFormat::Tuple(gt, t_elts)
            }
            Format::Record(flds) => {
                let index = self.get_and_increment_index();
                let mut t_flds = Vec::with_capacity(flds.len());
                for (lbl, t) in flds {
                    let t_fld = self.elaborate_format(t, dyns);
                    t_flds.push((lbl.clone(), t_fld));
                }
                let gt = self.get_gt_from_index(index);
                match gt.try_as_adhoc() {
                    Some(_) => (),
                    None => {
                        let before = self.get_gt_from_index(index - 1);
                        let after = self.get_gt_from_index(index + 1);
                        eprintln!("Possible frame-shift error around {index} (looking for Struct)");
                        eprintln!("[{}]: {before:?}", index - 1);
                        eprintln!("[{}]: {gt:?}", index);
                        eprintln!("[{}]: {after:?}", index + 1);
                        // unreachable!("found non-adhoc type for record format elaboration: {gt:?} @ {index} ({flds:#?})");
                    }
                }
                GTFormat::Record(gt, t_flds)
            }
            Format::Repeat(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Repeat(gt, Box::new(t_inner))
            }
            Format::Repeat1(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Repeat1(gt, Box::new(t_inner))
            }
            Format::RepeatCount(expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.elaborate_expr(expr);
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::RepeatCount(gt, t_expr, Box::new(t_inner))
            }
            Format::RepeatUntilLast(lambda, inner) => {
                let index = self.get_and_increment_index();
                let t_lambda = self.elaborate_expr_lambda(lambda);
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::RepeatUntilLast(gt, t_lambda, Box::new(t_inner))
            }
            Format::RepeatUntilSeq(lambda, inner) => {
                let index = self.get_and_increment_index();
                let t_lambda = self.elaborate_expr_lambda(lambda);
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::RepeatUntilSeq(gt, t_lambda, Box::new(t_inner))
            }
            Format::Peek(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Peek(gt, Box::new(t_inner))
            }
            Format::PeekNot(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::PeekNot(gt, Box::new(t_inner))
            }
            Format::Slice(expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.elaborate_expr(expr);
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Slice(gt, t_expr, Box::new(t_inner))
            }
            Format::Bits(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Bits(gt, Box::new(t_inner))
            }
            Format::WithRelativeOffset(expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.elaborate_expr(expr);
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::WithRelativeOffset(gt, t_expr, Box::new(t_inner))
            }
            Format::Map(inner, lambda) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyns);
                let t_lambda = self.elaborate_expr_lambda(lambda);
                let gt = self.get_gt_from_index(index);
                GTFormat::Map(gt, Box::new(t_inner), t_lambda)
            }
            Format::Compute(expr) => {
                let index = self.get_and_increment_index();
                let t_expr = self.elaborate_expr(expr);
                let gt = self.get_gt_from_index(index);
                GTFormat::Compute(gt, t_expr)
            }
            Format::Let(lbl, expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.elaborate_expr(expr);
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Let(gt, lbl.clone(), t_expr, Box::new(t_inner))
            }
            Format::Match(x, branches) => {
                let index = self.get_and_increment_index();
                let t_x = self.elaborate_expr(x);
                let mut t_branches = Vec::with_capacity(branches.len());
                for (pat, rhs) in branches {
                    let t_pat = self.elaborate_pattern(pat);
                    let t_rhs = self.elaborate_format(rhs, dyns);
                    t_branches.push((t_pat, t_rhs));
                }
                let gt = self.get_gt_from_index(index);
                GTFormat::Match(gt, t_x, t_branches)
            }
            Format::Dynamic(lbl, dynf, inner) => {
                let index = self.get_and_increment_index();
                let t_dynf = self.elaborate_dynamic_format(dynf);
                let newdyns = TypedDynScope::Binding(TypedDynBinding::new(
                    dyns,
                    lbl,
                    Rc::new(t_dynf.clone()),
                ));
                let t_inner = self.elaborate_format(inner, &newdyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::Dynamic(gt, lbl.clone(), t_dynf, Box::new(t_inner))
            }
            Format::Apply(lbl) => {
                let index = self.get_and_increment_index();
                let t_dynf = dyns
                    .get_typed_dynf_by_name(lbl)
                    .unwrap_or_else(|| panic!("missing dynformat {lbl}"));
                let gt = self.get_gt_from_index(index);
                GTFormat::Apply(gt, lbl.clone(), t_dynf)
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

    fn elaborate_expr<'s>(&mut self, expr: &Expr) -> GTExpr {
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
                    let t_elt = self.elaborate_expr(elt);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                GTExpr::Tuple(gt, t_elts)
            }
            Expr::TupleProj(e, ix) => {
                let t_e = self.elaborate_expr(e);
                let gt = self.get_gt_from_index(index);
                GTExpr::TupleProj(gt, Box::new(t_e), *ix)
            }
            Expr::Record(flds) => {
                let mut t_flds = Vec::with_capacity(flds.len());
                for (lbl, fld) in flds {
                    let t_fld = self.elaborate_expr(fld);
                    t_flds.push((lbl.clone(), t_fld));
                }
                let gt = self.get_gt_from_index(index);
                match gt.try_as_adhoc() {
                    Some(_) => (),
                    None => {
                        let before = self.get_gt_from_index(index - 1);
                        let after = self.get_gt_from_index(index + 1);
                        eprintln!("Possible frame-shift error around {index} (looking for Struct)");
                        eprintln!("[{}]: {before:?}", index - 1);
                        eprintln!("[{}]: {gt:?}", index);
                        eprintln!("[{}]: {after:?}", index + 1);
                        // unreachable!("found non-adhoc type for expr record elaboration: {gt:?} @ {index} ({flds:#?})");
                    }
                }
                GTExpr::Record(gt, t_flds)
            }
            Expr::Seq(elts) => {
                let mut t_elts = Vec::with_capacity(elts.len());
                self.increment_index();
                for elt in elts {
                    let t_elt = self.elaborate_expr(elt);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                GTExpr::Seq(gt, t_elts)
            }
            Expr::RecordProj(e, fld) => {
                let t_e = self.elaborate_expr(e);
                let gt = self.get_gt_from_index(index);
                GTExpr::RecordProj(gt, Box::new(t_e), fld.clone())
            }
            Expr::Match(head, branches) => {
                let t_head = self.elaborate_expr(head);
                let mut t_branches = Vec::with_capacity(branches.len());
                for (pat, rhs) in branches {
                    let t_pat = self.elaborate_pattern(pat);
                    let t_rhs = self.elaborate_expr(rhs);
                    t_branches.push((t_pat, t_rhs));
                }
                let gt = self.get_gt_from_index(index);
                GTExpr::Match(gt, Box::new(t_head), t_branches)
            }
            Expr::Lambda(..) => unreachable!(
                "Cannot elabora
               te Expr::Lambda in neutral (i.e. not lambda-aware) context"
            ),
            Expr::Variant(lbl, inner) => {
                let t_inner = self.elaborate_expr(inner);
                let gt = self.get_gt_from_index(index);
                match gt.try_as_adhoc() {
                    Some(_) => (),
                    None => {
                        let before = self.get_gt_from_index(index - 1);
                        let after = self.get_gt_from_index(index + 1);
                        eprintln!("Possible frame-shift error around {index} (looking for Enum)");
                        eprintln!("[{}]: {before:?}", index - 1);
                        eprintln!("[{}]: {gt:?}", index);
                        eprintln!("[{}]: {after:?}", index + 1);
                        // unreachable!("found non-adhoc type for expr variant elaboration: {gt:?} @ {index} ({lbl}({inner?}))");
                    }
                }
                GTExpr::Variant(gt, lbl.clone(), Box::new(t_inner))
            }
            Expr::IntRel(rel, x, y) => {
                let t_x = self.elaborate_expr(x);
                let t_y = self.elaborate_expr(y);
                let gt = self.get_gt_from_index(index);
                GTExpr::IntRel(gt, *rel, Box::new(t_x), Box::new(t_y))
            }
            Expr::Arith(op, x, y) => {
                let t_x = self.elaborate_expr(x);
                let t_y = self.elaborate_expr(y);
                let gt = self.get_gt_from_index(index);
                GTExpr::Arith(gt, *op, Box::new(t_x), Box::new(t_y))
            }
            Expr::AsU8(inner) => {
                let t_inner = self.elaborate_expr(inner);
                GTExpr::AsU8(Box::new(t_inner))
            }
            Expr::AsU16(inner) => {
                let t_inner = self.elaborate_expr(inner);
                GTExpr::AsU16(Box::new(t_inner))
            }
            Expr::AsU32(inner) => {
                let t_inner = self.elaborate_expr(inner);
                GTExpr::AsU32(Box::new(t_inner))
            }
            Expr::AsU64(inner) => {
                let t_inner = self.elaborate_expr(inner);
                GTExpr::AsU64(Box::new(t_inner))
            }
            Expr::AsChar(inner) => {
                let t_inner = self.elaborate_expr(inner);
                GTExpr::AsChar(Box::new(t_inner))
            }
            Expr::U16Be(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                GTExpr::U16Be(Box::new(t_bytes))
            }
            Expr::U16Le(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                GTExpr::U16Le(Box::new(t_bytes))
            }
            Expr::U32Be(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                GTExpr::U32Be(Box::new(t_bytes))
            }
            Expr::U32Le(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                GTExpr::U32Le(Box::new(t_bytes))
            }
            Expr::U64Be(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                GTExpr::U64Be(Box::new(t_bytes))
            }
            Expr::U64Le(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                GTExpr::U64Le(Box::new(t_bytes))
            }
            Expr::SeqLength(seq) => {
                let t_seq = self.elaborate_expr(seq);
                // NOTE - for element type of sequence
                self.increment_index();
                GTExpr::SeqLength(Box::new(t_seq))
            }
            Expr::SubSeq(seq, start, length) => {
                let t_seq = self.elaborate_expr(seq);
                let t_start = self.elaborate_expr(start);
                let t_length = self.elaborate_expr(length);
                // NOTE - for element type of sequence
                self.increment_index();
                let gt = self.get_gt_from_index(index);
                GTExpr::SubSeq(gt, Box::new(t_seq), Box::new(t_start), Box::new(t_length))
            }
            Expr::FlatMap(lambda, seq) => {
                let t_lambda = self.elaborate_expr_lambda(lambda);

                let t_seq = self.elaborate_expr(seq);
                self.increment_index();

                let gt = self.get_gt_from_index(index);
                GTExpr::FlatMap(gt, Box::new(t_lambda), Box::new(t_seq))
            }
            Expr::FlatMapAccum(lambda, acc, _acc_vt, seq) => {
                let t_lambda = self.elaborate_expr_lambda(lambda);
                let t_acc = self.elaborate_expr(acc);
                let t_seq = self.elaborate_expr(seq);

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
                let count_t = self.elaborate_expr(count);
                let x_t = self.elaborate_expr(x);
                let gt = self.get_gt_from_index(index);
                GTExpr::Dup(gt, Box::new(count_t), Box::new(x_t))
            }
            Expr::Inflate(seq) => {
                let seq_t = self.elaborate_expr(seq);

                // increment for extra variable generated by TC logic implementation in this case
                self.increment_index();

                let gt = self.get_gt_from_index(index);

                GTExpr::Inflate(gt, Box::new(seq_t))
            }
        }
    }

    fn elaborate_expr_lambda(&mut self, expr: &Expr) -> TypedExpr<GenType> {
        match expr {
            Expr::Lambda(head, body) => {
                let head_index = self.get_and_increment_index();
                // we don't increment here because it will be incremented by the rhs assignment on t_body
                let body_index = self.get_index();
                let t_body = self.elaborate_expr(body);
                let gt_head = self.get_gt_from_index(head_index);
                let gt_body = self.get_gt_from_index(body_index);
                GTExpr::Lambda((gt_head, gt_body), head.clone(), Box::new(t_body))
            }
            _ => unreachable!("elaborate_expr_lambda: unexpected non-lambda {expr:?}"),
        }
    }
}

type GTFormat = TypedFormat<GenType>;
type GTExpr = TypedExpr<GenType>;
type GTPattern = TypedPattern<GenType>;

// #[derive(Clone, PartialEq, Debug)]
// enum GenScope<'a> {
//     Empty,
//     Value(&'a GenScope<'a>, &'a str, Rc<GTExpr>),
// }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typecheck::Ctxt;

    fn population_check(module: &FormatModule, f: &Format, label: Option<&'static str>) {
        let mut tc = TypeChecker::new();
        let _fv = tc.infer_var_format(f, Ctxt::new(module, &UScope::Empty));
        let tc_pop = tc.size();

        // println!("{tc:?}");

        let cg = Codegen::new();
        let mut tv = Elaborator::new(module, tc, cg);
        let dec_f = tv.elaborate_format(f, &TypedDynScope::Empty);
        let re_f = Format::from(dec_f.clone());
        assert_eq!(
            &re_f,
            f,
            "post-elaboration format mismatch: {} != {}",
            serde_json::ser::to_string(&re_f).unwrap(),
            serde_json::ser::to_string(&f).unwrap()
        );
        let tv_pop = tv.next_index;

        // println!("{f:?} => {dec_f:?}");
        assert_eq!(
            tv_pop,
            tc_pop,
            "failed population check {} ({} TC vs {} TV)", // on {:?}\n{}\n{}",
            label.unwrap_or_default(),
            tc_pop,
            tv_pop // dec_f,
                   // serde_json::ser::to_string(&re_f).unwrap(),
                   // serde_json::ser::to_string(&f).unwrap()
        );
    }

    fn run_popcheck(fs: &[(&'static str, Format)]) {
        let mut module = FormatModule::new();
        for (name, f) in fs.iter() {
            module.define_format(*name, f.clone());
            population_check(&module, f, None);
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
            Format::Variant(
                "s
            ome"
                .into(),
                Box::new(Format::Byte(ByteSet::full())),
            ),
            Format::Variant("none".into(), Box::new(Format::EMPTY)),
        ]);

        run_popcheck(&[("adt_simple", f)]);
    }

    #[test]
    fn test_popcheck_itemvar() {
        let sub_f = Format::Byte(ByteSet::full());
        let mut module = FormatModule::new();
        let sub_ref = module.define_format("test.anybyte", sub_f);
        let f = sub_ref.call();
        module.define_format("test.call_anybyte", f.clone());
        population_check(&module, &f, None);
    }

    #[test]
    fn test_popcheck_compute_simple() {
        let x = Format::Byte(ByteSet::full());
        let fx = Format::Compute(Expr::Var("x".into()));
        let gx = Format::Compute(Expr::Arith(
            Arith::Add,
            Box::new(Expr::Var("x".into())),
            Box::new(Expr::Var("x".into())),
        ));

        let f = Format::Record(vec![("x".into(), x), ("fx".into(), fx), ("gx".into(), gx)]);
        run_popcheck(&[("test.compute_simple", f)]);
    }

    #[test]
    fn test_popcheck_compute_complex() {
        let is_null = Expr::Lambda(
            "x".into(),
            Box::new(Expr::IntRel(
                IntRel::Eq,
                Box::new(Expr::U8(0)),
                Box::new(Expr::Var("x".into())),
            )),
        );
        let ixdup = Expr::Lambda(
            "acc_x".into(),
            Box::new(Expr::Tuple(vec![
                Expr::Arith(
                    Arith::Add,
                    Box::new(Expr::U32(1)),
                    Box::new(Expr::TupleProj(Box::new(Expr::Var("acc_x".into())), 0)),
                ),
                Expr::Dup(
                    Box::new(Expr::TupleProj(Box::new(Expr::Var("acc_x".into())), 0)),
                    Box::new(Expr::TupleProj(Box::new(Expr::Var("acc_x".into())), 1)),
                ),
            ])),
        );

        let xs = Format::RepeatUntilLast(is_null, Box::new(Format::Byte(ByteSet::full())));
        let fxs = Format::Compute(Expr::FlatMapAccum(
            Box::new(ixdup),
            Box::new(Expr::U32(1)),
            ValueType::Base(BaseType::U32),
            Box::new(Expr::Var("xs".into())),
        ));

        let f = Format::Record(vec![("xs".into(), xs), ("fxs".into(), fxs)]);
        run_popcheck(&[("test.compute_complex", f)]);
    }
}
