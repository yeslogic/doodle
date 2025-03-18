mod name;
mod rust_ast;
mod trace;
mod typed_decoder;
pub(crate) mod typed_format;
mod util;

use rebind::Rebindable;
pub use rust_ast::ToFragment;

use crate::{
    byte_set::ByteSet,
    decoder::extract_pair,
    parser::error::TraceHash,
    typecheck::{TypeChecker, UScope, UVar},
    Arith, BaseType, DynFormat, Expr, Format, FormatModule, IntRel, IntoLabel, Label, MatchTree,
    Pattern, StyleHint, UnaryOp, ValueType,
};

use std::{
    borrow::Cow,
    cell::OnceCell,
    collections::BTreeSet,
    hash::{Hash, Hasher},
    rc::Rc,
};
mod ixlabel;
mod path_names;

use ixlabel::IxLabel;
use name::{Derivation, NameAtom};
use path_names::NameGen;
use rust_ast::analysis::{
    heap_optimize::{HeapOptimize, HeapStrategy},
    CopyEligible,
};
use rust_ast::*;
use util::{BTree, MapLike, StableMap};

use trace::get_and_increment_seed;
use typed_decoder::{GTCompiler, GTDecoder, TypedDecoder};
use typed_format::{GenType, TypedDynFormat, TypedExpr, TypedFormat, TypedPattern};

/// Produces a probabilistically unique TraceHash based on the value of a thread-local counter-state
/// (and post-increments the counter).
///
/// In order to produce output values that are more lexically distinct than the initially small
/// seed-values, performs a hashing operation over the raw seed.
fn get_trace(_salt: &impl std::hash::Hash) -> TraceHash {
    let mut hasher = std::hash::DefaultHasher::new();
    let seed = get_and_increment_seed();

    // Because the seed will always be unique, salting is unnecessary in the current model
    // In a context where we want the seed to remain small, we might re-use it with different
    // locally-distinct salt values

    // _salt.hash(&mut hasher);
    seed.hash(&mut hasher);

    hasher.finish()
}

pub struct CodeGen {
    name_gen: NameGen,
    defined_types: Vec<RustTypeDef>,
}

impl CodeGen {
    pub fn new() -> Self {
        let name_gen = NameGen::new();
        let defined_types = Vec::new();
        CodeGen {
            name_gen,
            defined_types,
        }
    }

    /// Converts a `ValueType` to a `GenType`, potentially creating new ad-hoc names
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
            ValueType::Option(param_t) => GenType::Inline(
                CompType::Option(Box::new(self.lift_type(param_t).to_rust_type())).into(),
            ),
            ValueType::Tuple(vs) => match &vs[..] {
                [] => RustType::AnonTuple(Vec::new()).into(),
                [v] => RustType::AnonTuple(vec![self.lift_type(v).to_rust_type()]).into(),
                _ => {
                    let mut buf = Vec::with_capacity(vs.len());
                    self.name_gen.ctxt.push_atom(NameAtom::Positional(0));
                    for v in vs.iter() {
                        buf.push(self.lift_type(v).to_rust_type());
                        self.name_gen.ctxt.increment_index();
                    }
                    self.name_gen.ctxt.escape();
                    RustType::AnonTuple(buf).into()
                }
            },
            ValueType::Seq(t) => {
                let inner = self.lift_type(t.as_ref()).to_rust_type();
                CompType::Vec(Box::new(inner)).into()
            }
            ValueType::Any => panic!("ValueType::Any"),
            ValueType::Record(fields) => {
                let mut rt_fields = Vec::new();
                for (lab, ty) in fields.iter() {
                    self.name_gen
                        .ctxt
                        .push_atom(NameAtom::RecordField(lab.clone()));
                    let rt_field = self.lift_type(ty);
                    rt_fields.push((lab.clone(), rt_field.to_rust_type()));
                    self.name_gen.ctxt.escape();
                }
                let rt_def = RustTypeDef::Struct(RustStruct::Record(rt_fields));
                let (type_name, (ix, is_new)) = self.name_gen.get_name(&rt_def);
                if is_new {
                    self.defined_types.push(rt_def.clone());
                }
                GenType::Def((ix, type_name), rt_def)
            }
            ValueType::Union(vars) => {
                let mut rt_vars = Vec::new();
                for (name, def) in vars.iter() {
                    self.name_gen
                        .ctxt
                        .push_atom(NameAtom::Variant(name.clone()));
                    let name = name.clone();
                    let var = match def {
                        ValueType::Empty => RustVariant::Unit(name),
                        ValueType::Tuple(args) => match &args[..] {
                            [] => RustVariant::Unit(name),
                            [arg] => {
                                RustVariant::Tuple(name, vec![self.lift_type(arg).to_rust_type()])
                            }
                            _ => {
                                let mut v_args = Vec::new();
                                self.name_gen.ctxt.push_atom(NameAtom::Positional(0));
                                for arg in args {
                                    v_args.push(self.lift_type(arg).to_rust_type());
                                    self.name_gen.ctxt.increment_index();
                                }
                                self.name_gen.ctxt.escape();
                                RustVariant::Tuple(name, v_args)
                            }
                        },
                        other => {
                            let inner = self.lift_type(other).to_rust_type();
                            RustVariant::Tuple(name, vec![inner])
                        }
                    };
                    rt_vars.push(var);
                    self.name_gen.ctxt.escape();
                }
                let rt_def = RustTypeDef::Enum(rt_vars);
                let (tname, (ix, is_new)) = self.name_gen.get_name(&rt_def);
                if is_new {
                    self.defined_types.push(rt_def.clone());
                }
                GenType::Def((ix, tname), rt_def)
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
            TypedDecoder::Pos => CaseLogic::Simple(SimpleLogic::YieldCurrentOffset),
            TypedDecoder::SkipRemainder => CaseLogic::Simple(SimpleLogic::SkipRemainder),
            TypedDecoder::Byte(bs) => CaseLogic::Simple(SimpleLogic::ByteIn(*bs)),
            TypedDecoder::Variant(gt, name, inner) => {
                let (type_name, def) = {
                    let Some((ix, lab)) = gt.try_as_adhoc() else { panic!("unexpected type_hint for Decoder::Variant: {:?}", gt) };
                    (lab.clone(), &self.defined_types[ix])
                };
                let constr = Constructor::Compound(type_name.clone(), name.clone());
                match def {
                    RustTypeDef::Enum(vars) => {
                        let matching = vars
                            .iter()
                            .find(|var| var.get_label().as_ref() == name.as_ref());
                        // REVIEW - should we enforce exact matches (i.e. `inner` must conform to the exact specification of the defined type)?
                        match matching {
                            Some(RustVariant::Unit(_)) => {
                                CaseLogic::Derived(
                                    DerivedLogic::UnitVariantOf(
                                        constr,
                                        Box::new(self.translate(inner.get_dec()))
                                    )
                                )
                            }
                            Some(RustVariant::Tuple(_, types)) => {
                                if types.is_empty() {
                                    unreachable!(
                                        "unexpected Tuple-Variant with 0 positional arguments"
                                    );
                                }
                                match inner.get_dec() {
                                    TypedDecoder::Tuple(_, decs) => {
                                        if decs.len() != types.len() {
                                            if types.len() == 1 {
                                                // REVIEW - allowance for 1-tuple variant whose argument type is itself an n-tuple
                                                match &types[0] {
                                                    RustType::AnonTuple(..) => {
                                                        let cl_mono_tuple = self.translate(
                                                            inner.get_dec()
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
                                                            "unable to translate Decoder::Tuple with hint ({other:?}) implied by {type_name}::{name}"
                                                        ),
                                                }
                                            } else {
                                                unreachable!(
                                                    "mismatched arity between decoder (== {}) and variant {type_name}::{name} (== {})",
                                                    decs.len(),
                                                    types.len()
                                                );
                                            }
                                        } else {
                                            let mut cl_args = Vec::new();
                                            for dec in decs.iter() {
                                                let cl_arg = self.translate(dec.get_dec());
                                                cl_args.push(cl_arg);
                                            }
                                            CaseLogic::Sequential(SequentialLogic::AccumTuple {
                                                constructor: Some(constr),
                                                elements: cl_args,
                                            })
                                        }
                                    }
                                    _ => {
                                        if types.len() == 1 {
                                            let cl_mono = self.translate(inner.get_dec());
                                            CaseLogic::Derived(
                                                DerivedLogic::VariantOf(constr, Box::new(cl_mono))
                                            )
                                        } else {
                                            panic!(
                                                "Variant {type_name}::{name}({types:#?}) mismatches non-tuple Decoder {inner:?}"
                                            );
                                        }
                                    }
                                }
                            }
                            None =>
                                unreachable!(
                                    "VariantOf called for nonexistent variant `{name}` of enum-type `{type_name}`"
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
                            .map(|alt| self.translate(alt.get_dec()))
                            .collect()
                    )
                ),
            TypedDecoder::Branch(_, tree, flat) =>
                CaseLogic::Other(
                    OtherLogic::Descend(
                        tree.clone(),
                        flat
                            .iter()
                            .map(|alt| self.translate(alt.get_dec()))
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
                                .map(|elt| self.translate(elt.get_dec()))
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
            TypedDecoder::Repeat0While(_gt, tree_continue, single) =>
                CaseLogic::Repeat(
                    RepeatLogic::Repeat0ContinueOnMatch(
                        tree_continue.clone(),
                        Box::new(self.translate(single.get_dec()))
                    )
                ),

            TypedDecoder::Repeat1Until(_gt, tree_break, single) =>
                CaseLogic::Repeat(
                    RepeatLogic::Repeat1BreakOnMatch(
                        tree_break.clone(),
                        Box::new(self.translate(single.get_dec()))
                    )
                ),
            TypedDecoder::ForEach(_gt, expr, lbl, single) => {
                // REVIEW[epic=zealous-clone] - do we need to ensure this is cloned?
                let cl_expr = embed_expr(expr, ExprInfo::EmbedCloned);
                CaseLogic::Repeat(RepeatLogic::ForEach(cl_expr, lbl.clone(), Box::new(self.translate(single.get_dec()))))
            }
            TypedDecoder::DecodeBytes(_gt, expr, inner) => {
                let cl_expr = embed_expr_dft(expr);
                CaseLogic::Derived(DerivedLogic::DecodeBytes(cl_expr, Box::new(self.translate(inner.get_dec()))))
            }
            TypedDecoder::RepeatCount(_gt, expr_count, single) =>
                CaseLogic::Repeat(
                    RepeatLogic::ExactCount(
                        embed_expr_dft(expr_count),
                        Box::new(self.translate(single.get_dec()))
                    )
                ),
            TypedDecoder::RepeatBetween(_gt, tree, expr_min, expr_max, single) => {
                CaseLogic::Repeat(
                    RepeatLogic::BetweenCounts(
                        tree.clone(),
                        embed_expr_dft(expr_min),
                        embed_expr_dft(expr_max),
                        Box::new(self.translate(single.get_dec()))
                    )
                )
            }
            TypedDecoder::RepeatUntilLast(_gt, pred_terminal, single) =>
                CaseLogic::Repeat(
                    RepeatLogic::ConditionTerminal(
                        GenLambda::from_expr(*pred_terminal.clone(), ClosureKind::Predicate),
                        Box::new(self.translate(single.get_dec()))
                    )
                ),
            TypedDecoder::RepeatUntilSeq(_gt, pred_complete, single) => {
                CaseLogic::Repeat(
                    RepeatLogic::ConditionComplete(
                        GenLambda::from_expr(*pred_complete.clone(), ClosureKind::Predicate),
                        Box::new(self.translate(single.get_dec()))
                    )
                )
            }
            TypedDecoder::AccumUntil(_gt, f, g, init, single) => {
                CaseLogic::Repeat(
                    RepeatLogic::AccumUntil(
                        GenLambda::from_expr(*f.clone(), ClosureKind::PairOwnedBorrow),
                        GenLambda::from_expr(*g.clone(), ClosureKind::Transform),
                        embed_expr_dft(init),
                        Box::new(self.translate(single.get_dec()))
                    )
                )
            }
            TypedDecoder::Maybe(_gt, cond, inner) => {
                CaseLogic::Derived(
                    DerivedLogic::Maybe(
                        embed_expr_dft(cond),
                        Box::new(self.translate(inner.get_dec()))
                    )
                )
            }
            TypedDecoder::Map(_gt, inner, f) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Derived(
                    DerivedLogic::MapOf(
                        GenLambda::from_expr(*f.clone(), ClosureKind::Transform),
                        Box::new(cl_inner)
                    )
                )
            }
            TypedDecoder::Where(_gt, inner, f) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Derived(
                    DerivedLogic::Where(
                        GenLambda::from_expr(*f.clone(), ClosureKind::Transform),
                        Box::new(cl_inner)
                    )
                )
            }
            TypedDecoder::Compute(_t, expr) =>
                // REVIEW[epic=zealous-clone] - try to gate Clone when Move, Copy or reference is possible
                CaseLogic::Simple(SimpleLogic::Eval(embed_expr(expr, ExprInfo::EmbedCloned))),
            TypedDecoder::Let(_t, name, expr, inner) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Derived(
                    DerivedLogic::Let(
                        name.clone(),
                        // REVIEW[epic=zealous-clone] - gate cloning when reference or Copy is possible
                        embed_expr(expr, ExprInfo::EmbedCloned),
                        Box::new(cl_inner)
                    )
                )
            }
            TypedDecoder::LetFormat(_t, f0, name, f) => {
                let cl_f0 = self.translate(f0.get_dec());
                let cl_f = self.translate(f.get_dec());
                CaseLogic::Other(
                    OtherLogic::LetFormat(
                        Box::new(cl_f0),
                        name.clone(),
                        Box::new(cl_f),
                    )
                )
            }
            TypedDecoder::MonadSeq(_t, f0, f) => {
                let cl_f0 = self.translate(f0.get_dec());
                let cl_f = self.translate(f.get_dec());
                CaseLogic::Other(
                    OtherLogic::MonadSeq(
                        Box::new(cl_f0),
                        Box::new(cl_f),
                    )
                )
            }
            TypedDecoder::Hint(_t, hint, inner) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Other(OtherLogic::Hint(hint.clone(), Box::new(cl_inner)))
            }
            TypedDecoder::Match(_t, scrutinee, cases) => {
                let scrutinized = embed_expr_dft(scrutinee);
                let head = match scrutinee.get_type().unwrap().as_ref() {
                    GenType::Inline(RustType::Atom(AtomType::Comp(CompType::Vec(..)))) =>
                        scrutinized.vec_as_slice(),
                    _ => scrutinized,
                };
                let mut cl_cases = Vec::new();
                for (pat, dec) in cases.iter() {
                    cl_cases.push((
                        MatchCaseLHS::Pattern(embed_pattern(pat)),
                        self.translate(dec.get_dec()),
                    ));
                }
                let ck = refutability_check(
                    scrutinee.get_type().expect("bad lambda in scrutinee position").as_ref(),
                    cases
                );
                CaseLogic::Other(OtherLogic::ExprMatch(head, cl_cases, ck))
            }
            TypedDecoder::Dynamic(
                _t,
                name,
                TypedDynFormat::Huffman(lengths, opt_values),
                inner,
            ) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Derived(
                    DerivedLogic::Dynamic(
                        DynamicLogic::Huffman(name.clone(), lengths.as_ref().clone(), opt_values.as_deref().cloned()),
                        Box::new(cl_inner)
                    )
                )
            }
            TypedDecoder::Apply(_t, lab) => {
                CaseLogic::Simple(SimpleLogic::CallDynamic(lab.clone()))
            }
            TypedDecoder::Peek(_t, inner) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Engine(EngineLogic::Peek(Box::new(cl_inner)))
            }
            TypedDecoder::PeekNot(_t, inner) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Engine(EngineLogic::PeekNot(Box::new(cl_inner)))
            }
            TypedDecoder::Slice(_t, width, inner) => {
                let re_width = embed_expr_dft(width);
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Engine(EngineLogic::Slice(re_width, Box::new(cl_inner)))
            }
            TypedDecoder::Bits(_t, inner) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Engine(EngineLogic::Bits(Box::new(cl_inner)))
            }
            TypedDecoder::WithRelativeOffset(_t, base_addr, offset, inner) => {
                let re_base_addr = embed_expr_dft(base_addr);
                let re_offset = embed_expr_dft(offset);
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Engine(EngineLogic::OffsetPeek(re_base_addr, re_offset, Box::new(cl_inner)))
            }
            TypedDecoder::LiftedOption(_, None) => {
                // REVIEW - do we ever need to preserve the type of the None value?
                CaseLogic::Simple(SimpleLogic::ConstNone)
            }
            TypedDecoder::LiftedOption(_, Some(da)) => {
                let cl_inner = self.translate(da.get_dec());
                CaseLogic::Derived(DerivedLogic::WrapSome(Box::new(cl_inner)))
            }
        }
    }
}

fn embed_pattern(pat: &GTPattern) -> RustPattern {
    match pat {
        TypedPattern::Tuple(_, elts) => match elts.as_slice() {
            [TypedPattern::Wildcard(..)] => RustPattern::Fill,
            _ => RustPattern::TupleLiteral(elts.iter().map(embed_pattern).collect()),
        },
        TypedPattern::Variant(gt, vname, inner) => match gt {
            GenType::Def((_, tname), _def) => {
                let constr = Constructor::Compound(tname.clone(), vname.clone());
                let inner_pat = match inner.as_ref() {
                    TypedPattern::Wildcard(..) => RustPattern::Fill,
                    _ => {
                        let inner_t = inner.get_type();
                        let tmp = embed_pattern(inner);
                        // TODO[epic=multiphase] - replace with Phase2 copy_hint
                        if inner_t.is_copy() {
                            tmp
                        } else {
                            tmp.ref_hack()
                        }
                    }
                };
                RustPattern::Variant(constr, Box::new(inner_pat))
            }
            other => {
                unreachable!("cannot inline TypedPattern::Variant with abstract GenType: {other:?}")
            }
        },
        TypedPattern::Option(gt, inner) => match gt {
            GenType::Inline(RustType::Atom(AtomType::Comp(CompType::Option(t)))) => {
                match inner.as_ref() {
                    Some(inner_pat) => {
                        RustPattern::Option(Some(Box::new({
                            let tmp = embed_pattern(inner_pat);
                            // NOTE - when T is Copy, we want `Option<T>` to be destructed to allow direct arithmetic
                            if t.is_copy() {
                                tmp
                            } else {
                                // FIXME - this is a hack to get `Some(ref x)` when matching on Option over non-Copy types
                                tmp.ref_hack()
                            }
                        })))
                    }
                    None => RustPattern::Option(None),
                }
            }
            _ => unreachable!("cannot inline TypedPattern::Option with non-Option GenType: {gt:?}"),
        },
        TypedPattern::Seq(_t, elts) => {
            RustPattern::ArrayLiteral(elts.iter().map(embed_pattern).collect())
        }
        TypedPattern::Wildcard(_) => RustPattern::CatchAll(None),
        TypedPattern::Binding(_, name) => RustPattern::CatchAll(Some(name.clone())),
        TypedPattern::Bool(b) => RustPattern::PrimLiteral(RustPrimLit::Boolean(*b)),
        TypedPattern::U8(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(RustNumLit::U8(*n))),
        TypedPattern::U16(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(RustNumLit::U16(*n))),
        TypedPattern::U32(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(RustNumLit::U32(*n))),
        TypedPattern::U64(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(RustNumLit::U64(*n))),
        TypedPattern::Int(gt, bounds) => match bounds.is_exact() {
            Some(n) => RustPattern::PrimLiteral(RustPrimLit::Numeric(RustNumLit::Usize(n))),
            None => match gt {
                GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::U8))) => {
                    let Ok((min, opt_max)): Result<(u8, Option<u8>), _> = (*bounds).try_into()
                    else {
                        panic!("ascribed type PrimType::U8 does not match with inherent value-range of bounds: {bounds:?}")
                    };
                    RustPattern::PrimRange(
                        RustPrimLit::Numeric(RustNumLit::U8(min)),
                        opt_max.map(|n| RustPrimLit::Numeric(RustNumLit::U8(n))),
                    )
                }
                GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::U16))) => {
                    let Ok((min, opt_max)): Result<(u16, Option<u16>), _> = (*bounds).try_into()
                    else {
                        panic!("ascribed type PrimType::U16 does not match with inherent value-range of bounds: {bounds:?}")
                    };
                    RustPattern::PrimRange(
                        RustPrimLit::Numeric(RustNumLit::U16(min)),
                        opt_max.map(|n| RustPrimLit::Numeric(RustNumLit::U16(n))),
                    )
                }
                GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::U32))) => {
                    let Ok((min, opt_max)): Result<(u32, Option<u32>), _> = (*bounds).try_into()
                    else {
                        panic!("ascribed type PrimType::U32 does not match with inherent value-range of bounds: {bounds:?}")
                    };
                    RustPattern::PrimRange(
                        RustPrimLit::Numeric(RustNumLit::U32(min)),
                        opt_max.map(|n| RustPrimLit::Numeric(RustNumLit::U32(n))),
                    )
                }
                GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::U64))) => {
                    let Ok((min, opt_max)): Result<(u64, Option<u64>), _> = (*bounds).try_into()
                    else {
                        panic!("ascribed type PrimType::U64 does not match with inherent value-range of bounds: {bounds:?}")
                    };
                    RustPattern::PrimRange(
                        RustPrimLit::Numeric(RustNumLit::U64(min)),
                        opt_max.map(|n| RustPrimLit::Numeric(RustNumLit::U64(n))),
                    )
                }
                _ => unreachable!("incoherent type for integer bounds: {bounds:?}"),
            },
        },
        TypedPattern::Char(c) => RustPattern::PrimLiteral(RustPrimLit::Char(*c)),
    }
}

/// Helper type that dictates the ownership model when transcribing a `GTExpr` into a `RustExpr`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ExprInfo {
    #[default]
    /// Uses implicit copy-or-move semantics on referenced local variables (i.e. as opposed to cloning)
    Natural,
    /// Applies a `clone` operation to any referenced local variables
    EmbedCloned,
}

fn embed_expr(expr: &GTExpr, info: ExprInfo) -> RustExpr {
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
                Constructor::Simple(tname.clone()),
                StructExpr::RecordExpr(fields
                    .iter()
                    .map(|(name, val)| (
                        name.clone(),
                        Some(embed_expr_dft(val)),
                    ))
                    .collect()
                )
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
                            let constr = Constructor::Compound(tname.clone(), vname.clone());
                            match this {
                                RustVariant::Unit(_vname) => {
                                    // FIXME - this leads to some '();' statements we might want to elide
                                    RustExpr::BlockScope(
                                        // REVIEW - we only need EmbedCloned if there are any potential reuse-after-move patterns within the `_ : ()` preamble...
                                        vec![RustStmt::Expr(embed_expr_dft(inner))],
                                        Box::new(RustExpr::Struct(constr, StructExpr::EmptyExpr))
                                    )
                                }
                                RustVariant::Tuple(_vname, _elts) => {
                                    // FIXME - not sure how to avoid 1 x N (unary-over-tuple) if inner becomes RustExpr::Tuple...
                                    RustExpr::Struct(constr, StructExpr::TupleExpr(vec![embed_expr_dft(inner)]))
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
            let scrutinized = embed_expr_dft(scrutinee);
            let head = match scrutinee.get_type().unwrap().as_ref() {
                GenType::Inline(
                    RustType::Atom(
                        AtomType::Comp(
                            | CompType::Vec(..)
                        ),
                    ),
                ) => scrutinized.make_persistent().into_owned().call_method("as_slice"),
                _ => scrutinized,
            };
            let ck = refutability_check(
                &scrutinee.get_type().expect("unexpected lambda in match-scrutinee position"),
                cases
            );

            let rust_cases = cases
                .iter()
                .map(|(pat, rhs)| {
                    (
                        MatchCaseLHS::Pattern(embed_pattern(pat)),
                        vec![RustStmt::Return(ReturnKind::Implicit, embed_expr(rhs, info))],
                    )
                })
                .collect::<Vec<RustMatchCase>>();
            let rust_body = match ck {
                Refutability::Refutable | Refutability::Indeterminate =>
                    RustMatchBody::Refutable(rust_cases, RustCatchAll::ReturnErrorValue {
                        value: RustExpr::err(RustExpr::scoped(["ParseError"], "ExcludedBranch").call_with([RustExpr::u64lit(get_trace(&expr))])),
                    }),
                Refutability::Irrefutable => RustMatchBody::Irrefutable(rust_cases),
            };
            RustExpr::Control(Box::new(RustControl::Match(head, rust_body)))
        }
        TypedExpr::Tuple(_t, tup) =>
            RustExpr::Tuple(
                tup
                    .iter()
                    .map(|x| embed_expr(x, info))
                    .collect()
            ),
        TypedExpr::TupleProj(_, expr_tup, ix) => {
            embed_expr(expr_tup, info /* ExprInfo::EmbedCloned */).nth(*ix)
        }
        TypedExpr::SeqIx(_, expr_seq, ix) => {
            let ix_expr = RustExpr::Operation(RustOp::AsCast(Box::new(embed_expr_dft(ix)), PrimType::Usize.into()));
            // REVIEW[epic=zealous-clone] - figure out under what circumstances we can avoid introducing cloning here
            embed_expr(expr_seq, ExprInfo::EmbedCloned).index(ix_expr)
        }
        TypedExpr::RecordProj(_, expr_rec, fld) => {
            // REVIEW[epic=zealous-clone] - figure out under what circumstances we can avoid introducing cloning here
            embed_expr(expr_rec, ExprInfo::EmbedCloned).field(fld.clone())
        }
        TypedExpr::Seq(_, elts) => {
            RustExpr::ArrayLit(
                elts
                    .iter()
                    .map(|x| embed_expr(x, info))
                    .collect()
            ).call_method("to_vec")
        }
        TypedExpr::Arith(_gt, arith, lhs, rhs) => {
            // NOTE - because arith only deals with Copy types, we oughtn't need any embedded clones
            let mut alt = None;
            let x = embed_expr_dft(lhs);
            let y = embed_expr_dft(rhs);
            let op = match arith {
                Arith::BitAnd => InfixOperator::BitAnd,
                Arith::BitOr => InfixOperator::BitOr,
                Arith::BoolAnd => InfixOperator::BoolAnd,
                Arith::BoolOr => InfixOperator::BoolOr,
                Arith::Add => InfixOperator::Add,
                Arith::Sub => {
                    alt.replace(RustExpr::local("try_sub!").call_with([x.clone(), y.clone(), RustExpr::u64lit(get_trace(&()))]));
                    InfixOperator::Sub
                }
                Arith::Mul => InfixOperator::Mul,
                Arith::Div => {
                    // TODO - implement try_div! to avoid panic on divide-by-zero
                    InfixOperator::Div
                }
                Arith::Rem => {
                    // TODO - implement try_rem! to avoid panic on remainder-by-zero
                    InfixOperator::Rem
                }
                Arith::Shl => InfixOperator::Shl,
                Arith::Shr => InfixOperator::Shr,
            };
            match alt {
                Some(alt) => alt,
                None => RustExpr::infix(x, op, y),
            }
        }
        TypedExpr::EnumFromTo(_, from, to) => {
            let start = embed_expr_dft(from);
            let stop = embed_expr_dft(to);
            // FIXME - currently, we have no optimization to pre-optimize SeqIx(EnumFromTo)...
            RustExpr::RangeExclusive(Box::new(start), Box::new(stop))
        }
        TypedExpr::IntRel(_, rel, lhs, rhs) => {
            // NOTE - because IntRel only deals with Copy types, we oughtn't need any embedded clones
            let x = embed_expr_dft(lhs);
            let y = embed_expr_dft(rhs);
            let op = match rel {
                IntRel::Eq => InfixOperator::Eq,
                IntRel::Ne => InfixOperator::Neq,
                IntRel::Lt => InfixOperator::Lt,
                IntRel::Gt => InfixOperator::Gt,
                IntRel::Lte => InfixOperator::Lte,
                IntRel::Gte => InfixOperator::Gte,
            };
            RustExpr::infix(x, op, y)
        }
        TypedExpr::Unary(_, op, inner) => {
            // NOTE - because Unary only deals with Copy types, we oughtn't need any embedded clones
            let x = embed_expr_dft(inner);
            match op {
                UnaryOp::BoolNot => {
                    let op = PrefixOperator::BoolNot;
                    RustExpr::Operation(RustOp::PrefixOp(op, Box::new(x)))
                }
                UnaryOp::IntPred => {
                    RustExpr::FunctionCall(Box::new(RustExpr::local("pred")), vec![x])
                }
                UnaryOp::IntSucc => {
                    RustExpr::FunctionCall(Box::new(RustExpr::local("succ")), vec![x])
                }
            }
        }
        TypedExpr::AsU8(x) =>
            RustExpr::Operation(RustOp::AsCast(Box::new(embed_expr_dft(x)), PrimType::U8.into())),
        TypedExpr::AsU16(x) =>
            RustExpr::Operation(RustOp::AsCast(Box::new(embed_expr_dft(x)), PrimType::U16.into())),
        TypedExpr::AsU32(x) =>
            RustExpr::Operation(RustOp::AsCast(Box::new(embed_expr_dft(x)), PrimType::U32.into())),
        TypedExpr::AsU64(x) =>
            RustExpr::Operation(RustOp::AsCast(Box::new(embed_expr_dft(x)), PrimType::U64.into())),
        TypedExpr::U16Be(be_bytes) =>
            RustExpr::local("u16be").call_with([embed_expr_dft(be_bytes)]),
        TypedExpr::U16Le(le_bytes) =>
            RustExpr::local("u16le").call_with([embed_expr_dft(le_bytes)]),
        TypedExpr::U32Be(be_bytes) =>
            RustExpr::local("u32be").call_with([embed_expr_dft(be_bytes)]),
        TypedExpr::U32Le(le_bytes) =>
            RustExpr::local("u32le").call_with([embed_expr_dft(le_bytes)]),
        TypedExpr::U64Be(be_bytes) =>
            RustExpr::local("u64be").call_with([embed_expr_dft(be_bytes)]),
        TypedExpr::U64Le(le_bytes) =>
            RustExpr::local("u64le").call_with([embed_expr_dft(le_bytes)]),
        TypedExpr::AsChar(codepoint) =>
            RustExpr::scoped(["char"], "from_u32")
                .call_with([embed_expr_dft(codepoint)])
                .call_method("unwrap"),
        TypedExpr::SeqLength(seq) => {
            // NOTE - SeqLength is treated as U32 in Format context, so any operations on it have to be done on a U32 value rather than the natural `.len(): _ -> usize` return-value
            RustExpr::Operation(
                RustOp::AsCast(
                    Box::new(embed_expr_dft(seq).vec_len()),
                    RustType::Atom(AtomType::Prim(PrimType::U32))
                )
            )
        }

        TypedExpr::SubSeq(_, seq, ix, len) => {
            let start_expr = embed_expr_dft(ix);
            let bind_ix = RustStmt::assign(
                "ix",
                RustExpr::Operation(RustOp::AsCast(Box::new(start_expr), PrimType::Usize.into()))
            );
            let end_expr = RustExpr::infix(
                RustExpr::local("ix"),
                InfixOperator::Add,
                RustExpr::Operation(
                    RustOp::AsCast(Box::new(embed_expr_dft(len)), PrimType::Usize.into())
                )
            );
            RustExpr::BlockScope(
                vec![bind_ix],
                Box::new(
                    // REVIEW - in some cases, we might be able to get away with slice-typed expressions, but in practice it is easier to vec everything for now and worry about performance later
                    RustExpr::scoped(["Vec"], "from").call_with([
                        RustExpr::Borrow(
                            Box::new(
                                RustExpr::Slice(
                                    Box::new(embed_expr_dft(seq)),
                                    Box::new(RustExpr::local("ix")),
                                    Box::new(end_expr)
                                )
                            )
                        ),
                    ])
                )
            )
        }
        TypedExpr::SubSeqInflate(_, seq, ix, len) => {
            let start_expr = embed_expr_dft(ix);

            let bind_ix = RustStmt::assign("ix", RustExpr::Operation(RustOp::AsCast(Box::new(start_expr), PrimType::Usize.into())));
            let end_expr = RustExpr::infix(
                RustExpr::local("ix"),
                InfixOperator::Add,
                RustExpr::Operation(
                    RustOp::AsCast(Box::new(embed_expr_dft(len)), PrimType::Usize.into())
                )
            );

            let range = RustExpr::RangeExclusive(Box::new(RustExpr::local("ix")), Box::new(end_expr));

            RustExpr::BlockScope(vec![bind_ix], Box::new(RustExpr::local("slice_ext").call_with(vec![RustExpr::borrow_of(embed_expr_dft(seq)), range]).call_method("to_vec")))
        }
        TypedExpr::FlatMap(_, f, seq) =>
            RustExpr::local("try_flat_map_vec")
                .call_with([
                    embed_expr_dft(seq).call_method("iter").call_method("cloned"),
                    embed_lambda(f, ClosureKind::Transform, true, ExprInfo::EmbedCloned),
                ])
                .wrap_try(),
        TypedExpr::FlatMapAccum(_, f, acc_init, _acc_type, seq) =>
            RustExpr::local("try_fold_map_curried")
                .call_with([
                    embed_expr_dft(seq).call_method("iter").call_method("cloned"),
                    embed_expr(acc_init, info /* ExprInfo::EmbedCloned */),
                    embed_lambda(f, ClosureKind::Transform, true, ExprInfo::EmbedCloned),
                ])
                .wrap_try(),
        TypedExpr::LeftFold(_, f, acc_init, _acc_type, seq) =>
            RustExpr::local("try_fold_left_curried")
                .call_with([
                    embed_expr_dft(seq).call_method("iter").call_method("cloned"),
                    embed_expr(acc_init, info /* ExprInfo::EmbedCloned */),
                    embed_lambda(f, ClosureKind::Transform, true, ExprInfo::EmbedCloned),
                ])
                .wrap_try(),
        TypedExpr::FlatMapList(_, f, _ret_type, seq) =>
            RustExpr::local("try_flat_map_append_vec")
                .call_with([
                    embed_expr_dft(seq).call_method("iter").call_method("cloned"),
                    embed_lambda_dft(f, ClosureKind::PairBorrowOwned, true),
                ])
                .wrap_try(),
        TypedExpr::FindByKey(_, is_sorted, f, query, seq) => {
            let method = if *is_sorted {
                "find_by_key_sorted"
            } else {
                "find_by_key_unsorted"
            };
            let seq = embed_expr_dft(seq).make_persistent().into_owned();
            RustExpr::local(method)
                .call_with([
                    embed_lambda_dft(f, ClosureKind::ExtractKey, false),
                    embed_expr(query, ExprInfo::Natural),
                    seq,
                ])
                // REVIEW[epic=zealous-clone] - do we need this? (we probably do)
                // TODO: gate 'copied' by whether the GenType is Copy/Clone; we may want to infer whether an owned value is needed at all, but that is harder to answer locally
                .call_method("cloned")
        }
        TypedExpr::Dup(_, n, expr) => {
            // NOTE - the dup count should be simple, but the duplicated expression must be move-safe
            RustExpr::local("dup32").call_with([
                embed_expr_dft(n),
                // REVIEW[epc=zealous-clone] - consider under what circumstances the clone can be eliminated
                embed_expr(expr, ExprInfo::EmbedCloned),
            ])
        }
        TypedExpr::Var(_, vname) => {
            // REVIEW - lexical scopes, shadowing, and variable-name sanitization may not be quite right in the current implementation
            let loc = RustExpr::local(vname.clone());
            // REVIEW[epic=zealous-clone] - figure out if there is a way to avoid cloning here...
            match info {
                ExprInfo::EmbedCloned => RustExpr::CloneOf(Box::new(loc)),
                ExprInfo::Natural => loc,
            }
        }
        TypedExpr::Bool(b) => RustExpr::bool_lit(*b),
        TypedExpr::U8(n) => RustExpr::u8lit(*n),
        TypedExpr::U16(n) => RustExpr::u16lit(*n),
        TypedExpr::U32(n) => RustExpr::u32lit(*n),
        TypedExpr::U64(n) => RustExpr::u64lit(*n),
        TypedExpr::Lambda(_, _, _) =>
            unreachable!(
                "TypedExpr::Lambda unsupported as first-class embed (requires embed_lambda with proper ClosureKind argument)"
            ),
        // TODO - determine if we need to type-annotate the Some call based on the gt we are currently ignoring
        TypedExpr::LiftOption(_, Some(x)) => embed_expr(x, info).wrap_some(),
        TypedExpr::LiftOption(_, None) => RustExpr::option_none(),
    }
}

/// Uses the default value of `ExprInfo` for [`embed_expr`]
fn embed_expr_dft(expr: &TypedExpr<GenType>) -> RustExpr {
    embed_expr(expr, ExprInfo::default())
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Default)]
enum Refutability {
    Refutable,
    #[default]
    Indeterminate,
    Irrefutable,
}

impl Refutability {
    fn and(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn or(self, other: Self) -> Self {
        Ord::max(self, other)
    }
}

fn refutability_check<A: std::fmt::Debug + Clone>(
    head_type: &GenType,
    cases: &[(TypedPattern<GenType>, A)],
) -> Refutability {
    if contains_irrefutable_pattern(cases) {
        return Refutability::Irrefutable;
    }
    match head_type {
        GenType::Inline(rt) =>
            match rt {
                RustType::Atom(at) =>
                    match at {
                        AtomType::TypeRef(lt) =>
                            match lt {
                                LocalType::LocalDef(ix, lbl) =>
                                    unreachable!(
                                        "inline LocalDef ({ix}, {lbl}) cannot be resolved abstractly, use GenType::Def instead"
                                    ),
                                LocalType::External(t) =>
                                    unreachable!(
                                        "external type '{t}' cannot be resolved without further information"
                                    ),
                            }
                        AtomType::Prim(pt) =>
                            match pt {
                                PrimType::Unit => {
                                    if cases.is_empty() {
                                        Refutability::Refutable
                                    } else {
                                        Refutability::Irrefutable
                                    }
                                }
                                // these cases have too many values to practically cover...
                                | PrimType::U8
                                | PrimType::U16
                                | PrimType::U32
                                | PrimType::U64
                                | PrimType::Char => Refutability::Indeterminate,
                                //
                                PrimType::Bool => {
                                    // mask for inclusion with indices 0: false, 1: true
                                    let mut cover_mask = [false, false];
                                    for (pat, _) in cases {
                                        match pat {
                                            TypedPattern::Bool(b) => {
                                                let ix = if *b { 1 } else { 0 };
                                                cover_mask[ix] = true;
                                            }
                                            _ => {
                                                continue;
                                            }
                                        }
                                    }
                                    if cover_mask[0] && cover_mask[1] {
                                        Refutability::Irrefutable
                                    } else {
                                        Refutability::Refutable
                                    }
                                }
                                // any match on usize is only exhaustive with a catch-all, which we have precluded above
                                PrimType::Usize => Refutability::Refutable,
                            }
                        AtomType::Comp(ct) =>
                            match ct {
                                CompType::Vec(_) => Refutability::Refutable, // Vec can have any length, so no match can be exhaustive without catchalls
                                CompType::Option(t) => {
                                    let none_covered = cases.iter().any(|(pat, _)| matches!(pat, TypedPattern::Option(_, None)));
                                    if !none_covered {
                                        return Refutability::Refutable;
                                    }

                                    let some_cases: Vec<(TypedPattern<GenType>, A)> = cases.iter().filter_map(|(pat, rhs)| match pat { TypedPattern::Option(_, Some(x)) => Some(((**x).clone(), rhs.clone())), _ => None}).collect();
                                    let rust_type = (**t).clone();
                                    refutability_check(&GenType::Inline(rust_type), &some_cases)
                                }
                                CompType::Result(_, _) =>
                                    unreachable!("unexpected result in pattern head-type"),
                                CompType::Borrow(_, _, t) => {
                                    refutability_check(&GenType::Inline((**t).clone()), cases)
                                }
                            }
                    }
                RustType::AnonTuple(ts) => {
                    // we have already checked in contains_irrefutable_pattern that there is no (_x0, ..., _xN) pattern
                    if ts.is_empty() && !cases.is_empty() {
                        Refutability::Irrefutable
                    } else {
                        Refutability::Indeterminate
                    }
                }
                RustType::Verbatim(_, _) =>
                    unreachable!("verbatim types not expected in generated match-expressions"),
            }
        GenType::Def(_, def) => {
            match def {
                RustTypeDef::Enum(vars) => {
                    // NOTE - we encounter badness when attempting to check full-variant coverage using subtyped partial unions
                    // NOTE - we can only check for every possible value being covered for every possible variant
                    let mut variant_coverage: StableMap<Label, Refutability, BTree> =
                        vars
                            .iter()
                            .map(|x| (x.get_label().clone(), Refutability::Refutable))
                            .collect();
                    for (pat, _) in cases {
                        match pat {
                            TypedPattern::Variant(_, vname, inner_pat) => {
                                let entry = variant_coverage.entry(vname.clone());

                                if is_pattern_irrefutable(inner_pat) {
                                    entry.and_modify(|prior| {
                                        *prior = prior.or(Refutability::Irrefutable);
                                    });
                                } else {
                                    entry.and_modify(|prior| {
                                        *prior = prior.or(Refutability::Indeterminate);
                                    });
                                }
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    variant_coverage.values().cloned().reduce(Refutability::and).unwrap()
                }
                RustTypeDef::Struct(st) => {
                    unreachable!(
                        "there are no patterns that match simple structures in place: {st:?}, {cases:#?}"
                    );
                }
            }
        }
    }
}

fn is_pattern_irrefutable(pat: &TypedPattern<GenType>) -> bool {
    match pat {
        TypedPattern::Binding(..) | TypedPattern::Wildcard(..) => true,
        TypedPattern::Tuple(_, elts) => elts.iter().all(is_pattern_irrefutable),
        TypedPattern::Seq(..) => false, // there is no exhaustive pattern-set for sequences as they can have any length...
        TypedPattern::Variant(gt, lab, inner) => {
            // a variant pattern is irrefutable if there are no other variants and the inner expression is also irrefutable
            is_pattern_irrefutable(inner)
                && (match gt {
                    GenType::Def(_, def) => match def {
                        RustTypeDef::Enum(vars) => vars.len() == 1 && vars[0].get_label() == lab,
                        _ => unreachable!("variant pattern will never match struct-typed value"),
                    },
                    GenType::Inline(RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(..)))) => {
                        false
                    } // we cannot identify mono-variants at this layer
                    _ => unreachable!("variant pattern cannot match non-LocalDef type"),
                })
        }
        _ => false, // all the other cases are prim-types that cover only one of N > 1 possible values
    }
}

fn contains_irrefutable_pattern<A>(head_cases: &[(TypedPattern<GenType>, A)]) -> bool {
    for (pat, _) in head_cases {
        if is_pattern_irrefutable(pat) {
            return true;
        }
    }
    false
}

/// Marker-type for different syntactic categories of closure, with respect to what type of
/// capture they perform (i.e. move or borrow)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ClosureKind {
    /// Category for closures that take a single borrowed argument
    Predicate,
    /// Category for closures that take a single owned argument
    Transform,
    /// Hybrid category for closures taking an pair-argument, the first element of which is borrowed and the second of which is owned
    PairBorrowOwned,
    /// Hybrid category for closures taking an pair-argument, the first element of which is owned and the second of which is borrowed
    PairOwnedBorrow,
    /// Category for closures that take a single borrowed argument and extract an owned value of a `Copy` type, to use as a key for scanning arrays
    ExtractKey,
}

/// Transcribes `GTExpr::Lambda` instances into RustExpr values.
///
/// When `kind` is `ClosureKind::Predicate`, the resulting RustExpr will be a closure that operates on a reference to its associated argument-type.
///
/// When `kind` is `ClosureKind::Transform`, the resulting RustExpr will be a closure that operates on an owned value of its associated argument-type.
///
/// For hybrid `kind`s `PairBorrowOwned` and `PairOwnedBorrow`, the closure in question operates on a tuple with one borrowed and one owned value.
///
/// The `needs_ok` argument controls whether the overall body of the closure expression will be wrapped in `Ok` or not, which depends on whether
/// there are any short-circuiting code-paths within the embedded lambda body. If `true`, an `Ok(...)` will be produced. Otherwise, the body will be
/// transcribed as-is.
///
/// Additionally takes an argument `info` that dictates how the body is to be transcribed, according to the
/// implied semantics used in `embed_expr`
fn embed_lambda(expr: &GTExpr, kind: ClosureKind, needs_ok: bool, info: ExprInfo) -> RustExpr {
    match expr {
        TypedExpr::Lambda((head_t, _), head, body) => match kind {
            // REVIEW - while ExtractKind is very similar to Predicate semantics, we want to avoid hard-coding Predicate for cases where that descriptor is misleading
            ClosureKind::Predicate | ClosureKind::ExtractKey => {
                let expansion = embed_expr(body, info);
                RustExpr::Closure(RustClosure::new_predicate(
                    head.clone(),
                    Some(head_t.clone().to_rust_type()),
                    if needs_ok {
                        expansion.wrap_ok(Some("PResult"))
                    } else {
                        expansion
                    },
                ))
            }
            ClosureKind::Transform => {
                let expansion = embed_expr(body, info);
                RustExpr::Closure(RustClosure::new_transform(
                    head.clone(),
                    Some(head_t.clone().to_rust_type()),
                    if needs_ok {
                        expansion.wrap_ok(Some("PResult"))
                    } else {
                        expansion
                    },
                ))
            }
            ClosureKind::PairBorrowOwned => {
                let RustType::AnonTuple(args) = head_t.clone().to_rust_type() else {
                    panic!("type {head_t:?} does not look like a tuple...")
                };
                let point_t = match &args[..] {
                    [fst, snd] => RustType::AnonTuple(vec![
                        RustType::borrow_of(None, Mut::Immutable, fst.clone()),
                        snd.clone(),
                    ]),
                    other => unreachable!("tuple is not a pair: {other:?}"),
                };
                let expansion = embed_expr(body, info);
                RustExpr::Closure(RustClosure::new_transform(
                    head.clone(),
                    Some(point_t),
                    if needs_ok {
                        expansion.wrap_ok(Some("PResult"))
                    } else {
                        expansion
                    },
                ))
            }
            ClosureKind::PairOwnedBorrow => {
                let RustType::AnonTuple(args) = head_t.clone().to_rust_type() else {
                    panic!("type {head_t:?} does not look like a tuple...")
                };
                let point_t = match &args[..] {
                    [fst, snd] => RustType::AnonTuple(vec![
                        fst.clone(),
                        RustType::borrow_of(None, Mut::Immutable, snd.clone()),
                    ]),
                    other => unreachable!("tuple is not a pair: {other:?}"),
                };
                let expansion = embed_expr(body, info);
                RustExpr::Closure(RustClosure::new_transform(
                    head.clone(),
                    Some(point_t),
                    if needs_ok {
                        expansion.wrap_ok(Some("PResult"))
                    } else {
                        expansion
                    },
                ))
            }
        },
        _other => unreachable!("embed_lambda_t expects a lambda, found {_other:?}"),
    }
}

/// Version of [`embed_lambda`] that uses the implied-default `ExprInfo` argument.
fn embed_lambda_dft(expr: &GTExpr, kind: ClosureKind, needs_ok: bool) -> RustExpr {
    embed_lambda(expr, kind, needs_ok, ExprInfo::Natural)
}

#[derive(Debug, Clone)]
struct TypedLambda<TypeRep> {
    head: Label,
    head_type: TypeRep,
    kind: ClosureKind,
    // REVIEW - we might be able to deprecate this field and use is_short_circuiting instead
    body: Rc<TypedExpr<TypeRep>>,
    __beta_reducible: OnceCell<bool>,
    __needs_ok: OnceCell<bool>,
}

/// REVIEW - consider how GenLambda nad GenBlock interoperate (or, possibly, fail to)
type GenLambda = TypedLambda<GenType>;

impl GenLambda {
    pub fn get_head_var(&self) -> &Label {
        &self.head
    }

    /// Fallibly constructs a `GenLambda` from a `TypedExpr<GenType>`, panicking if it is not a Lambda variant.
    ///
    /// Additionally takes in the `kind` and `needs_ok` parameters that would normally be
    /// passed to [`GenLambda::new`] or [`embed_lambda_dft`].
    pub fn from_expr(expr: GTExpr, kind: ClosureKind) -> Self {
        let TypedExpr::Lambda((head_type, _), head, body) = expr else {
            unreachable!("GenLambda::from_expr expects a lambda, found {expr:?}")
        };
        let body = Rc::new(*body);
        Self::new(head, head_type, kind, body)
    }

    fn new(head: Label, head_type: GenType, kind: ClosureKind, body: Rc<GTExpr>) -> Self {
        Self {
            head,
            head_type,
            kind,
            body,
            __beta_reducible: OnceCell::new(),
            __needs_ok: OnceCell::new(),
        }
    }

    fn try_alpha_convert(&self, new_head: &Label) -> Option<Self> {
        assert_ne!(&self.head, new_head, "alpha conversion would be no-op");
        let _trial_conv = self.clone();
        // TOOD - implement alpha conversion
        None
    }

    fn beta_reduce(&self, param: RustExpr, body_info: ExprInfo) -> RustExpr {
        match param.as_local() {
            Some(outer) => {
                if &**outer == &*self.head {
                    embed_expr(&self.body, body_info)
                } else if let Some(rebound) = self.try_alpha_convert(outer) {
                    embed_expr(&rebound.body, body_info)
                } else {
                    let bind_param_and_head = match self.kind {
                        ClosureKind::Predicate | ClosureKind::ExtractKey => {
                            let bind_param_to_head =
                                RustStmt::assign(self.head.clone(), RustExpr::borrow_of(param));
                            vec![bind_param_to_head]
                        }
                        ClosureKind::Transform => {
                            let bind_param_to_head = RustStmt::assign(self.head.clone(), param);
                            vec![bind_param_to_head]
                        }
                        ClosureKind::PairBorrowOwned => {
                            use rust_ast::CaptureSemantics::*;
                            // REVIEW - This may lead to redundancy if the body itself de-structures the pair
                            const PAIR_BIND: [&str; 2] = ["fst", "snd"];
                            const SEMANTICS: [CaptureSemantics; 2] = [Ref, Owned];
                            let bind_param_to_tuple = RustStmt::destructure(
                                RustPattern::tuple_capture(PAIR_BIND, SEMANTICS),
                                param,
                            );
                            let bind_tuple_to_head = RustStmt::assign(
                                self.head.clone(),
                                RustExpr::local_tuple(PAIR_BIND),
                            );
                            vec![bind_param_to_tuple, bind_tuple_to_head]
                        }
                        ClosureKind::PairOwnedBorrow => {
                            use rust_ast::CaptureSemantics::*;
                            // REVIEW - This may lead to redundancy if the body itself de-structures the pair
                            const PAIR_BIND: [&str; 2] = ["fst", "snd"];
                            const SEMANTICS: [CaptureSemantics; 2] = [Owned, Ref];
                            let bind_param_to_tuple = RustStmt::destructure(
                                RustPattern::tuple_capture(PAIR_BIND, SEMANTICS),
                                param,
                            );
                            let bind_tuple_to_head = RustStmt::assign(
                                self.head.clone(),
                                RustExpr::local_tuple(PAIR_BIND),
                            );
                            vec![bind_param_to_tuple, bind_tuple_to_head]
                        }
                    };
                    let expansion = embed_expr(&self.body, body_info);
                    RustExpr::BlockScope(bind_param_and_head, Box::new(expansion))
                }
            }
            None => {
                let head_bind = RustStmt::assign(self.head.clone(), param);
                let expansion = embed_expr(&self.body, body_info);
                RustExpr::BlockScope(vec![head_bind], Box::new(expansion))
            }
        }
    }

    // FIXME - the logic here may be broken
    fn __apply_pair(&self, param0: RustExpr, param1: RustExpr, body_info: ExprInfo) -> RustExpr {
        let raw_expansion = embed_expr(&self.body, body_info);
        match raw_expansion {
            RustExpr::Control(ctrl) => match ctrl.as_ref() {
                RustControl::Match(scrutinee, match_body) => {
                    if scrutinee.as_local() != Some(&self.head) {
                        unreachable!("unexpected outer scrutinee in expansion of pair-lambda (head: {}): {scrutinee:?}", &self.head);
                    }
                    match match_body {
                        RustMatchBody::Irrefutable(cases) => match &cases[..] {
                            [(lhs, rhs)] => match lhs {
                                MatchCaseLHS::Pattern(RustPattern::TupleLiteral(pair)) => match &pair[..] {
                                    [fst, snd] => match (fst, snd) {
                                        (RustPattern::CatchAll(Some(fst_lbl)), RustPattern::CatchAll(Some(snd_lbl))) => {
                                            let fst_bind = RustStmt::assign(fst_lbl.clone(), param0);
                                            let snd_bind = RustStmt::assign(snd_lbl.clone(), param1);
                                            match stmts_to_block(Cow::Borrowed(rhs)) {
                                                Some((block, ret)) => {
                                                    let all_stmts = [fst_bind, snd_bind].into_iter().chain(block.iter().cloned()).collect();
                                                    RustExpr::BlockScope(
                                                        all_stmts,
                                                        Box::new(ret.into_owned())
                                                    )
                                                }
                                                None => unreachable!("unexpected short-circuit: {rhs:?}"),
                                            }
                                        }
                                        other => unreachable!("expected pair-var capture pattern in lhs, found {other:?}"),
                                    }
                                    other => unreachable!("expected 2-tuple, found {other:?}"),
                                }
                                other => unreachable!("unexpected if-guarded or non-tuple MatchCaseLHS {other:?}"),
                            }
                            [] => unreachable!("unexpected empty match-block"),
                            other => unreachable!("unexpected multi-branch RustMatchBody in pair-lambda expansion: {other:?}"),
                        }
                        other => unreachable!("unexpected non-irrefutable RustMatchBody in pair-lambda expansion: {other:?}"),
                    }
                }
                _ => {
                    let arg = RustExpr::Tuple(vec![param0, param1]);
                    return self.apply(arg, body_info);
                }
            },
            _ => {
                let arg = RustExpr::Tuple(vec![param0, param1]);
                return self.apply(arg, body_info);
            }
        }
    }

    // FIXME - the logic here may be broken
    pub fn apply_pair(&self, param0: RustExpr, param1: RustExpr, body_info: ExprInfo) -> RustExpr {
        let beta_reducible = self.is_beta_reducible();
        match (self.kind, beta_reducible) {
            (_, false) => {
                let arg = RustExpr::Tuple(vec![param0, param1]);
                self.embed(body_info).call_with([arg])
            }
            (ClosureKind::Transform, true) => self.__apply_pair(param0, param1, body_info),
            (ClosureKind::Predicate, true) => self.__apply_pair(
                RustExpr::borrow_of(param0),
                RustExpr::borrow_of(param1),
                body_info,
            ),
            (ClosureKind::PairBorrowOwned, true) => {
                self.__apply_pair(RustExpr::borrow_of(param0), param1, body_info)
            }
            (ClosureKind::PairOwnedBorrow, true) => {
                self.__apply_pair(param0, RustExpr::borrow_of(param1), body_info)
            }
            (other @ ClosureKind::ExtractKey, true) => {
                unreachable!("unexpected ClosureKind for apply_pair: {other:?}")
            }
        }
    }

    /// Abstraction layer that allows for conditional selection of embed-closure-then-call
    /// and beta-reduction strategies.
    pub fn apply(&self, param: RustExpr, body_info: ExprInfo) -> RustExpr {
        if self.should_beta_reduce(&param) {
            self.beta_reduce(param, body_info)
        } else {
            let raw = self.embed(body_info).call_with([param]);
            // REVIEW - double-check this is the right predicate to apply
            if self.needs_ok() {
                raw.wrap_try()
            } else {
                raw
            }
        }
    }

    /// Internal heuristic that returns `true` if the application of `self` to `arg` should be handled
    /// via beta-reduction rather than embed-and-call.
    fn should_beta_reduce(&self, _arg: &RustExpr) -> bool {
        // REVIEW - decide on what heuristics, if any, to replace this with
        self.is_beta_reducible()
    }

    pub fn is_beta_reducible(&self) -> bool {
        let __body = self.body.clone();
        *self
            .__beta_reducible
            .get_or_init(move || !embed_expr_dft(__body.as_ref()).is_short_circuiting())
    }

    /// Indicates whether the body, if it is found to have short-circuiting, needs to be wrapped in `Ok(..)`.
    ///
    /// May return true even when the body does not itself short-circuit, in which case the output should not
    /// mean the caller must wrap the body in `Ok`.
    pub fn needs_ok(&self) -> bool {
        let __body = self.body.clone();
        *self
            .__needs_ok
            .get_or_init(move || embed_expr_dft(__body.as_ref()).needs_ok())
    }

    fn embed(&self, info: ExprInfo) -> RustExpr {
        match self.kind {
            ClosureKind::Predicate | ClosureKind::ExtractKey => {
                let expansion = embed_expr(&self.body, info);
                RustExpr::Closure(RustClosure::new_predicate(
                    self.head.clone(),
                    Some(self.head_type.clone().to_rust_type()),
                    if self.needs_ok() {
                        expansion.wrap_ok(Some("PResult"))
                    } else {
                        expansion
                    },
                ))
            }
            ClosureKind::Transform => {
                let expansion = embed_expr(&self.body, info);
                RustExpr::Closure(RustClosure::new_transform(
                    self.head.clone(),
                    Some(self.head_type.clone().to_rust_type()),
                    if self.needs_ok() {
                        expansion.wrap_ok(Some("PResult"))
                    } else {
                        expansion
                    },
                ))
            }
            ClosureKind::PairBorrowOwned => {
                let RustType::AnonTuple(args) = self.head_type.clone().to_rust_type() else {
                    panic!("type {:?} does not look like a tuple...", &self.head_type)
                };
                let point_t = {
                    let (fst, snd) = extract_pair(args);
                    RustType::AnonTuple(vec![RustType::borrow_of(None, Mut::Immutable, fst), snd])
                };
                let expansion = embed_expr(&self.body, info);
                RustExpr::Closure(RustClosure::new_transform(
                    self.head.clone(),
                    Some(point_t),
                    if self.needs_ok() {
                        expansion.wrap_ok(Some("PResult"))
                    } else {
                        expansion
                    },
                ))
            }
            ClosureKind::PairOwnedBorrow => {
                let RustType::AnonTuple(args) = self.head_type.clone().to_rust_type() else {
                    panic!("type {:?} does not look like a tuple...", &self.head_type)
                };
                let point_t = {
                    let (fst, snd) = extract_pair(args);
                    RustType::AnonTuple(vec![fst, RustType::borrow_of(None, Mut::Immutable, snd)])
                };
                let expansion = embed_expr(&self.body, info);
                RustExpr::Closure(RustClosure::new_transform(
                    self.head.clone(),
                    Some(point_t),
                    if self.needs_ok() {
                        expansion.wrap_ok(Some("PResult"))
                    } else {
                        expansion
                    },
                ))
            }
        }
    }
}

type RustBlock = (Vec<RustStmt>, Option<RustExpr>);

#[derive(Debug, Clone)]
enum GenExpr {
    /// One-to-one lifted `RustExpr` without exposure for further modifications
    Embed(RustExpr),
    /// Construction of a value for an anon-tuple or a locally-defined type
    // TODO - split out RustExpr::Struct from this instead of using RustExpr as a proxy
    TyValCon(RustExpr),
    /// Analogue of `RustExpr::BlockScope`
    BlockScope(Box<GenBlock>),
    /// Value-producing Control expression (e.g. `if`, `match`)
    Control(Box<RustControl<GenBlock>>),
    /// Wrapping a `GenExpr` in an `Ok` value
    ResultOk(Option<Label>, Box<GenExpr>),
    /// Wrapping a `GenExpr` in a `Some` value
    WrapSome(Box<GenExpr>),
    /// Applies the `?` operator to a given `GenExpr`
    Try(Box<GenExpr>),
    /// Calls a value-producing thunk directly
    CallThunk(Box<GenThunk>),
}

impl GenExpr {
    fn wrap_ok<Name: IntoLabel>(self, qual: Option<Name>) -> GenExpr {
        match self {
            Self::Try(x) => *x,
            other => Self::ResultOk(qual.map(Name::into), Box::new(other)),
        }
    }

    fn wrap_some(mut self) -> GenExpr {
        match self {
            Self::ResultOk(t, inner) => Self::ResultOk(t, Box::new(inner.wrap_some())),
            Self::BlockScope(ref mut block) => {
                // REVIEW - this may not be enough if non-`ret` (statements) can return values
                block.wrap_some_final_value();
                self
            }
            this => Self::WrapSome(Box::new(this)),
        }
    }

    fn wrap_try(self) -> GenExpr {
        match self {
            Self::ResultOk(_, inner) => *inner,
            Self::BlockScope(block) => {
                let block0 = GenBlock {
                    stmts: block.stmts,
                    ret: block.ret.map(GenExpr::wrap_try),
                };
                Self::BlockScope(Box::new(block0))
            }
            _ => Self::Try(Box::new(self)),
        }
    }

    /// Constructs the most appropriate `GenExpr` for a given nominal match-expression,
    /// namely a block-scope let-pattern destructuring for single-case infallible matches,
    /// or a natural embed of `RustControl::Match` for fallible or branching matches.
    fn embed_match(expr: RustExpr, body: RustMatchBody<GenBlock>) -> Self {
        match body {
            RustMatchBody::Irrefutable(mut cases) if cases.len() == 1 && cases[0].0.is_simple() => {
                // unwwrap is safe because we checked cases above
                let Some((MatchCaseLHS::Pattern(pat), mut block)) = cases.pop() else {
                    panic!("bad guard")
                };
                let let_bind = GenStmt::Embed(RustStmt::destructure(pat, expr));
                block.prepend_stmt(let_bind);
                GenExpr::BlockScope(Box::new(block))
            }
            _ => {
                let match_item = RustControl::Match(expr, body);
                GenExpr::Control(Box::new(match_item))
            }
        }
    }
}

impl From<RustExpr> for GenExpr {
    fn from(value: RustExpr) -> Self {
        GenExpr::Embed(value)
    }
}

impl From<GenExpr> for RustExpr {
    fn from(value: GenExpr) -> Self {
        match value {
            GenExpr::BlockScope(block) => RustExpr::from(*block),
            GenExpr::Embed(expr) | GenExpr::TyValCon(.., expr) => expr,
            GenExpr::Control(ctrl) => RustExpr::Control(Box::new(RustControl::translate(*ctrl))),
            GenExpr::WrapSome(expr) => RustExpr::from(*expr).wrap_some(),
            GenExpr::ResultOk(qual, expr) => RustExpr::from(*expr).wrap_ok(qual),
            GenExpr::Try(expr) => RustExpr::from(*expr).wrap_try(),
            GenExpr::CallThunk(thunk) => {
                let closure = if thunk.thunk_body.ret.is_none() {
                    RustClosure::thunk_body(thunk.thunk_body.flatten())
                } else {
                    RustClosure::thunk_expr(RustExpr::from(thunk.thunk_body))
                };
                RustExpr::Closure(closure).call()
            }
        }
    }
}

impl ShortCircuit for GenExpr {
    fn is_short_circuiting(&self) -> bool {
        match self {
            GenExpr::Embed(expr) => expr.is_short_circuiting(),
            GenExpr::TyValCon(expr) => expr.is_short_circuiting(),
            GenExpr::Control(ctrl) => ctrl.is_short_circuiting(),
            GenExpr::ResultOk(.., expr) | GenExpr::WrapSome(expr) => expr.is_short_circuiting(),
            GenExpr::BlockScope(block) => block.is_short_circuiting(),
            GenExpr::Try(..) => true,
            GenExpr::CallThunk(..) => false,
        }
    }
}

/// Abstraction layer between `TypedExpr` (internal AST) and `RustStmt` (external AST)
///
/// `GenStmt` is intended to be a bridge between `TypedExpr` and `RustStmt` that
/// preserves enough type-adjacent information about each expression and binding
/// to perform last-minute mutations of the content that would be much less easy
/// to accomplish on a pure `RustStmt` tree without annotations or introspection
/// rules that are very fragile and difficult to reason about.
#[derive(Clone, Debug)]
enum GenStmt {
    /// Evaluate a `GenExpr` without binding its value locally
    Expr(GenExpr),
    /// One-to-one lifted `RustStmt` without exposure for further modifications
    Embed(RustStmt),
    /// One-time immutable binding of an expression, modeled as `GenBlock`.
    BindOnce(Label, GenBlock),
}

impl GenBlock {
    /// Inserts the single statement `before` at the start of the statements contained in `self`.
    fn prepend_stmt(&mut self, before: GenStmt) {
        let mut stmts = Vec::with_capacity(self.stmts.len() + 1);
        stmts.push(before);
        stmts.append(&mut self.stmts);
        self.stmts = stmts;
    }

    /// Inserts the statements contained in the iterable `preamble`, in order, directly before
    /// the statements contained in `self`.
    fn prepend_stmts(&mut self, preamble: impl IntoIterator<Item = GenStmt>) {
        let stmts =
            Iterator::chain(preamble.into_iter(), self.stmts.drain(..)).collect::<Vec<GenStmt>>();
        self.stmts = stmts;
    }

    pub fn wrap_some_final_value(&mut self) {
        let fallback = || {
            Result::<_, std::convert::Infallible>::Ok(Some(GenExpr::from(
                RustExpr::UNIT.wrap_some(),
            )))
        };
        self.transform_return_value(GenExpr::wrap_some, fallback)
            .unwrap()
    }

    fn transform_return_value<E, F, G>(&mut self, f: F, fallback: G) -> Result<(), E>
    where
        F: Fn(GenExpr) -> GenExpr,
        G: FnOnce() -> Result<Option<GenExpr>, E>,
    {
        if let Some(val) = self.ret.take() {
            self.ret.replace(f(val));
        } else {
            self.ret = fallback()?;
        }
        Ok(())
    }
}

impl From<GenStmt> for RustStmt {
    fn from(value: GenStmt) -> Self {
        match value {
            GenStmt::Expr(gen_expr) => RustStmt::Expr(RustExpr::from(gen_expr)),
            GenStmt::Embed(stmt) => stmt,
            GenStmt::BindOnce(bind_name, block) => RustStmt::assign(bind_name, block.into()),
        }
    }
}

impl From<GenBlock> for Vec<RustStmt> {
    fn from(value: GenBlock) -> Vec<RustStmt> {
        value.flatten()
    }
}

impl From<RustStmt> for GenStmt {
    fn from(value: RustStmt) -> Self {
        GenStmt::Embed(value)
    }
}

impl ShortCircuit for GenStmt {
    fn is_short_circuiting(&self) -> bool {
        match self {
            GenStmt::Expr(gen_expr) => gen_expr.is_short_circuiting(),
            GenStmt::Embed(stmt) => stmt.is_short_circuiting(),
            GenStmt::BindOnce(_, block) => block.is_short_circuiting(),
        }
    }
}

impl GenStmt {
    /// Returns a mutable reference to the `GenBlock` assigned to a sigbind if the name of the binding matches `query`.
    ///
    /// Returns None if the binding-name is anon-match, or if `self` is not a significant binding.
    #[expect(unused)]
    pub fn get_bind_as_mut(&mut self, query: impl AsRef<str>) -> Option<&mut GenBlock> {
        match self {
            GenStmt::BindOnce(lab, rhs) if lab.as_ref() == query.as_ref() => Some(rhs),
            GenStmt::BindOnce(..) | GenStmt::Embed(..) | GenStmt::Expr(..) => None,
        }
    }

    pub const fn is_bind(&self) -> bool {
        matches!(self, GenStmt::BindOnce(..))
    }

    /// Returns the name for a distinguished binding (e.g. [`GenStmt::BindOnce`]), if there is one,
    /// or None if there is not.
    #[expect(unused)]
    pub const fn bind_name(&self) -> Option<&Label> {
        match self {
            GenStmt::BindOnce(lab, ..) => Some(lab),
            GenStmt::Embed(..) => None,
            GenStmt::Expr(..) => None,
        }
    }

    fn assign<Name: IntoLabel>(binding: Name, rhs: GenBlock) -> Self {
        GenStmt::BindOnce(binding.into(), rhs)
    }
}

/// Abstraction for thunk constructed directly from a body (`GenBlock`)
#[derive(Debug, Clone)]
struct GenThunk {
    thunk_body: GenBlock,
}

impl GenThunk {
    pub const fn new(thunk_body: GenBlock) -> Self {
        GenThunk { thunk_body }
    }

    pub fn call(self) -> GenExpr {
        GenExpr::CallThunk(Box::new(self))
    }
}

#[derive(Debug, Clone, Default)]
struct GenBlock {
    stmts: Vec<GenStmt>,
    ret: Option<GenExpr>,
}

impl ShortCircuit for GenBlock {
    fn is_short_circuiting(&self) -> bool {
        self.stmts.is_short_circuiting()
            || self.ret.as_ref().is_some_and(GenExpr::is_short_circuiting)
    }
}

impl GenBlock {
    /// Constructs a new, empty `GenBlock``.
    #[expect(unused)]
    pub const fn new() -> Self {
        GenBlock {
            stmts: Vec::new(),
            ret: None,
        }
    }

    pub fn mono_statement(stmt: RustStmt) -> Self {
        GenBlock {
            stmts: vec![GenStmt::Embed(stmt)],
            ret: None,
        }
    }

    pub const fn from_parts(stmts: Vec<GenStmt>, ret: Option<GenExpr>) -> Self {
        GenBlock { stmts, ret }
    }

    #[expect(unused)]
    pub fn has_binds(&self) -> bool {
        self.stmts.iter().any(GenStmt::is_bind)
    }

    /// Constructs a traditional `RustBlock` value from a `GenBlock`, consuming the original in the process.
    pub fn synthesize(self) -> RustBlock {
        let ret = self.ret.map(RustExpr::from);
        if self.stmts.is_empty() {
            (Vec::new(), ret)
        } else {
            let stmts = self.stmts.into_iter().map(RustStmt::from).collect();
            (stmts, ret)
        }
    }

    /// Constructs a `GenBlock` consisting of a single expression, without any preliminary bindings, extra logic, or control statements.
    pub const fn simple_expr(expr: RustExpr) -> Self {
        GenBlock {
            stmts: Vec::new(),
            ret: Some(GenExpr::Embed(expr)),
        }
    }

    /// Constructs a `GenBlock` consisting of a single expression, without any preliminary bindings, extra logic, or control statements.
    pub const fn single_expr(expr: GenExpr) -> Self {
        GenBlock {
            stmts: Vec::new(),
            ret: Some(expr),
        }
    }

    /// Constructs a `GenBlock` of the form `return <expr>`, as a standalone `RustStmt`.
    pub fn explicit_return(expr: RustExpr) -> Self {
        GenBlock {
            stmts: vec![GenStmt::Embed(RustStmt::Return(ReturnKind::Keyword, expr))],
            ret: None,
        }
    }

    /// Constructs a `GenBlock` without any promotion-eligible bindings, from a sequence of `RustStmt` and a `RustExpr`.
    pub fn lift_block(logic: impl IntoIterator<Item = RustStmt>, ret: RustExpr) -> Self {
        GenBlock {
            stmts: logic.into_iter().map(GenStmt::Embed).collect(),
            ret: Some(GenExpr::Embed(ret)),
        }
    }

    /// Converts a `GenBlock` into a `Vec<RustStmt>` (e.g. for use in `RustControl` constructs)
    /// by mapping the (optional) trailing `RustExpr` into an implicit-return `RustStmt` after
    /// fully manifesting any distinguished bindings.
    ///
    /// Should only be used in contexts where it is known (or strongly assumed) that there are no
    /// modifications that need to be made to the `GenBlock` before manifestation.
    pub fn flatten(self) -> Vec<RustStmt> {
        let (mut stmts, ret) = self.synthesize();
        if stmts.is_empty() {
            match ret {
                None => Vec::new(),
                Some(expr) => vec![RustStmt::Return(ReturnKind::Implicit, expr)],
            }
        } else {
            if let Some(expr) = ret {
                stmts.push(RustStmt::Return(ReturnKind::Implicit, expr));
            }
            stmts
        }
    }
}

impl GenBlock {
    /// Refactor a GenBlock so that any short-circuiting statements before the implicit-return-value
    /// are caught at the implicit-return site rather than escaping the current function (or closure)
    /// altogether.
    ///
    /// E.g.
    /// ```no_run
    /// fn incorrect() -> Result<bool, E> {
    ///     let x = {
    ///         let y = fallible_operation()?;
    ///         Ok(y)
    ///     };
    ///     match x {
    ///         Ok(_x) => return Ok(true),
    ///         Err(_e) => return Ok(false),
    ///     }
    /// }
    ///
    /// fn correct() -> Result<bool, E> {
    ///     let x = (||
    ///         let y = fallible_operation()?;
    ///         Ok(y)
    ///     )();
    ///     match x {
    ///         Ok(_x) => return Ok(true),
    ///         Err(_e) => return Ok(false),
    ///     }
    /// }
    /// ```
    pub fn local_try(self) -> GenBlock {
        if self.stmts.iter().any(GenStmt::is_short_circuiting) {
            let GenBlock { stmts, ret } = self;
            let thunk_body = GenBlock {
                stmts,
                ret: ret.map(|expr| expr.wrap_ok(Some("PResult"))),
            };
            let ret = GenThunk::new(thunk_body).call().wrap_try();
            GenBlock {
                stmts: Vec::new(),
                ret: Some(ret),
            }
        } else {
            let GenBlock { stmts, ret } = self;
            match ret {
                Some(expr) => match expr {
                    GenExpr::Try(inner) => GenBlock {
                        stmts,
                        ret: Some(GenExpr::Try(inner)),
                    },
                    _ => {
                        if expr.is_short_circuiting() {
                            let thunk_body = GenBlock {
                                stmts: Vec::new(),
                                ret: Some(expr.wrap_ok(Some("PResult"))),
                            };
                            GenBlock {
                                stmts,
                                ret: Some(GenThunk::new(thunk_body).call().wrap_try()),
                            }
                        } else {
                            GenBlock {
                                stmts,
                                ret: Some(expr),
                            }
                        }
                    }
                },
                None => GenBlock { stmts, ret: None },
            }
        }
    }

    /// Applies a lambda-abstraction to a `GenBlock` so that engine logic isn't affected
    /// by short-circuiting behavior of `?` and `return Err(...)` within the block in question
    ///
    /// Used when the value of `Err` must be inspected (as in `PeekNot`` or `Alts`),
    /// rather than externally short-circuited on via `?` (in which case, [`local_try`] should be used).
    fn abstracted_try(self) -> GenBlock {
        if self.stmts.iter().any(GenStmt::is_short_circuiting) {
            let GenBlock { stmts, ret } = self;
            let thunk_body = GenBlock {
                stmts,
                ret: ret.map(|expr| expr.wrap_ok(Some("PResult"))),
            };
            let ret = GenThunk::new(thunk_body).call();
            GenBlock {
                stmts: Vec::new(),
                ret: Some(ret),
            }
        } else {
            let Some(ret) = self.ret else {
                unreachable!("unexpected: abstracted_try_block called with no return-value to wrap")
            };
            if ret.is_short_circuiting() {
                let thunk_ret = ret.wrap_ok(Some("PResult"));
                let ret = GenThunk::new(GenBlock {
                    stmts: Vec::new(),
                    ret: Some(thunk_ret),
                })
                .call();
                GenBlock {
                    stmts: self.stmts,
                    ret: Some(ret),
                }
            } else {
                Self {
                    ret: Some(ret),
                    ..self
                }
            }
        }
    }
}

impl From<GenBlock> for RustExpr {
    fn from(value: GenBlock) -> Self {
        let (stmts, ret) = value.synthesize();
        if stmts.is_empty() {
            // REVIEW - do we always want an explicit Unit here?
            ret.unwrap_or(RustExpr::UNIT)
        } else {
            // REVIEW - do we always want an explicit Unit here?
            RustExpr::BlockScope(stmts, Box::new(ret.unwrap_or(RustExpr::UNIT)))
        }
    }
}

impl From<RustExpr> for GenBlock {
    fn from(value: RustExpr) -> Self {
        GenBlock::simple_expr(value)
    }
}

impl From<GenExpr> for GenBlock {
    fn from(value: GenExpr) -> Self {
        GenBlock::single_expr(value)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ProdCtxt<'a> {
    input_varname: &'a Label,
}

impl<'a> Default for ProdCtxt<'a> {
    fn default() -> Self {
        Self {
            input_varname: &Cow::Borrowed(""),
        }
    }
}

macro_rules! impl_toast_caselogic {
    ($($t:ident),+) => {
        $(
        impl ToAst for CaseLogic<$t>
        {
            type AstElem = GenBlock;

            /// Produces an RustExpr-valued AST for the given CaseLogic instance.
            ///
            /// The ExprT should have the bare type of the value being parsed (i.e. not Option-wrapped),
            /// but it is implicitly assumed to be contained in a block whose ultimate return value
            /// is `Option<_>`, allowing `return None` and `?` expressions to be used anyway.
            ///
            /// Local bindings and control flow are allowed, as long as an explicit return
            /// or a concrete, consistently-typed return value are used
            fn to_ast(&self, ctxt: ProdCtxt<'_>) -> GenBlock {
                match self {
                    CaseLogic::Derived(d) => d.to_ast(ctxt),
                    CaseLogic::Engine(e) => e.to_ast(ctxt),
                    CaseLogic::Other(o) => o.to_ast(ctxt),
                    CaseLogic::Parallel(p) => p.to_ast(ctxt),
                    CaseLogic::Repeat(r) => r.to_ast(ctxt),
                    CaseLogic::Sequential(sq) => sq.to_ast(ctxt),
                    CaseLogic::Simple(s) => s.to_ast(ctxt),
                }
            }
        }
        )+
    };
}

impl_toast_caselogic!(GTExpr);

impl ToAst for SimpleLogic<GTExpr> {
    type AstElem = GenBlock;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> GenBlock {
        match self {
            SimpleLogic::Fail => GenBlock::explicit_return(RustExpr::err(
                RustExpr::scoped(["ParseError"], "FailToken")
                    .call_with([RustExpr::u64lit(get_trace(&()))]),
            )),
            SimpleLogic::ExpectEnd => GenBlock::simple_expr(
                RustExpr::local(ctxt.input_varname.clone())
                    .call_method("finish")
                    .wrap_try(),
            ),
            SimpleLogic::SkipRemainder => GenBlock::simple_expr(
                RustExpr::local(ctxt.input_varname.clone()).call_method("skip_remainder"),
            ),
            SimpleLogic::Invoke(ix_dec, args) => {
                let fname = format!("Decoder{ix_dec}");
                let call_args = {
                    let base_args = [RustExpr::local(ctxt.input_varname.clone())];
                    if args.is_empty() {
                        base_args.to_vec()
                    } else {
                        base_args
                            .into_iter()
                            .chain(args.iter().map(|(_lab, x)| {
                                let Some(t) = x.get_type() else {
                                    panic!("unexpected lambda in arg-list of SimpleLogic::Invoke")
                                };
                                if t.to_rust_type().should_borrow_for_arg() {
                                    RustExpr::borrow_of(embed_expr(x, ExprInfo::Natural))
                                } else {
                                    embed_expr(x, ExprInfo::EmbedCloned)
                                }
                            }))
                            .collect()
                    }
                };
                let call = RustExpr::local(fname).call_with(call_args);
                GenBlock::simple_expr(call.wrap_try())
            }
            SimpleLogic::CallDynamic(dynf_name) => {
                let call = RustExpr::local(dynf_name.clone())
                    .call_with([RustExpr::local(ctxt.input_varname.clone())]);
                GenBlock::simple_expr(call.wrap_try())
            }
            SimpleLogic::SkipToNextMultiple(n) => GenBlock::simple_expr(
                RustExpr::local(ctxt.input_varname.clone())
                    .call_method_with("skip_align", [RustExpr::num_lit(*n)])
                    .wrap_try(),
            ),
            SimpleLogic::YieldCurrentOffset => GenBlock::simple_expr(
                RustExpr::local(ctxt.input_varname.clone()).call_method("get_offset_u64"),
            ),
            SimpleLogic::ByteIn(bs) => {
                let call = RustExpr::local(ctxt.input_varname.clone())
                    .call_method("read_byte")
                    .wrap_try();
                let bc = ByteCriterion::from(bs);
                if bc.always_true() {
                    GenBlock::simple_expr(call)
                } else {
                    let b_let = RustStmt::assign("b", call);
                    let cond = bc.as_predicate(RustExpr::local("b"));
                    let b_true = vec![RustStmt::Return(ReturnKind::Implicit, RustExpr::local("b"))];
                    let b_false = vec![RustStmt::Return(
                        ReturnKind::Keyword,
                        RustExpr::err(
                            RustExpr::scoped(["ParseError"], "ExcludedBranch")
                                .call_with([RustExpr::u64lit(get_trace(bs))]),
                        ),
                    )];
                    let logic =
                        RustExpr::Control(Box::new(RustControl::If(cond, b_true, Some(b_false))));
                    GenBlock::lift_block([b_let], logic)
                }
            }
            SimpleLogic::Eval(expr) => GenBlock::simple_expr(expr.clone()),
            SimpleLogic::ConstNone => GenBlock::simple_expr(RustExpr::NONE),
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
    /// Returns `true` if the ByteCriterion is satisfied by every possible byte from 0 to 255.
    pub fn always_true(&self) -> bool {
        matches!(self, ByteCriterion::Any)
    }

    /// Returns a tuple a RustExpr that evaluates to `true` if the argument satisfies the criterion
    /// that `self` represents.
    fn as_predicate(&self, arg: RustExpr) -> RustExpr {
        match self {
            ByteCriterion::Any => RustExpr::TRUE,
            ByteCriterion::MustBe(byte) => {
                RustExpr::Operation(RustOp::op_eq(arg, RustExpr::num_lit(*byte)))
            }
            ByteCriterion::OtherThan(byte) => {
                RustExpr::Operation(RustOp::op_neq(arg, RustExpr::num_lit(*byte)))
            }
            ByteCriterion::WithinSet(bs) => embed_byteset(bs).call_method_with("contains", [arg]),
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

// follows the same rules as CaseLogic::to_ast as far as the expression type of the generated code
fn embed_matchtree(tree: &MatchTree, ctxt: ProdCtxt<'_>) -> GenBlock {
    fn expand_matchtree(tree: &MatchTree, ctxt: ProdCtxt<'_>) -> GenBlock {
        match &tree.branches[..] {
            [] => {
                if let Some(ix) = tree.accept {
                    return GenBlock::simple_expr(RustExpr::num_lit(ix));
                } else {
                    let err_val = RustExpr::scoped(["ParseError"], "ExcludedBranch")
                        .call_with([RustExpr::u64lit(get_trace(&(tree, "empty-non-accepting")))]);
                    return GenBlock::explicit_return(RustExpr::err(err_val));
                }
            }
            [(bs, branch)] => {
                let bc = ByteCriterion::from(bs);

                let call = RustExpr::local(ctxt.input_varname.clone())
                    .call_method("read_byte")
                    .wrap_try();

                if bc.always_true() {
                    // this always accepts, but needs to read a byte
                    let ignore_byte = GenStmt::Embed(RustStmt::Expr(call));
                    let branch_block = expand_matchtree(branch, ctxt);
                    // REVIEW - need append-stable indexing, or dedicated method for prepend/append
                    let all_stmts = Iterator::chain(
                        std::iter::once(ignore_byte),
                        branch_block.stmts.into_iter(),
                    )
                    .collect();
                    GenBlock {
                        stmts: all_stmts,
                        ..branch_block
                    }
                } else {
                    let bind = RustStmt::assign("b", call);
                    let guard = bc.as_predicate(RustExpr::local("b"));
                    let b_true: Vec<RustStmt> = expand_matchtree(branch, ctxt).flatten();
                    let b_false = {
                        if let Some(ix) = tree.accept {
                            vec![RustStmt::Return(
                                ReturnKind::Implicit,
                                RustExpr::num_lit(ix),
                            )]
                        } else {
                            let err_val = RustExpr::scoped(["ParseError"], "ExcludedBranch")
                                .call_with([RustExpr::u64lit(get_trace(&(
                                    tree,
                                    "failed-descent-condition",
                                )))]);
                            vec![RustStmt::Return(
                                ReturnKind::Keyword,
                                RustExpr::err(err_val),
                            )]
                        }
                    };
                    GenBlock::lift_block(
                        [bind],
                        RustExpr::Control(Box::new(RustControl::If(guard, b_true, Some(b_false)))),
                    )
                }
            }
            _ => {
                let call = RustExpr::local(ctxt.input_varname.clone())
                    .call_method("read_byte")
                    .wrap_try();
                let mut cases = Vec::new();

                for (bs, branch) in tree.branches.iter() {
                    let crit = ByteCriterion::from(bs);
                    match crit {
                        ByteCriterion::Any => {
                            unreachable!("unconditional descent with more than one branch");
                        }
                        ByteCriterion::MustBe(b) => {
                            let lhs = MatchCaseLHS::Pattern(RustPattern::PrimLiteral(
                                RustPrimLit::Numeric(RustNumLit::U8(b)),
                            ));
                            let rhs = expand_matchtree(branch, ctxt).flatten();
                            cases.push((lhs, rhs));
                        }
                        ByteCriterion::OtherThan(_) | ByteCriterion::WithinSet(_) => {
                            let guard = crit.as_predicate(RustExpr::local("byte"));
                            let lhs = MatchCaseLHS::WithGuard(
                                RustPattern::CatchAll(Some(Label::from("byte"))),
                                guard,
                            );
                            let rhs = expand_matchtree(branch, ctxt).flatten();
                            cases.push((lhs, rhs));
                        }
                    }
                }

                let value = RustExpr::err(
                    RustExpr::scoped(["ParseError"], "ExcludedBranch")
                        .call_with([RustExpr::u64lit(get_trace(&(tree, "catchall-nomatch")))]),
                );
                let match_block = RustControl::Match(
                    call,
                    RustMatchBody::Refutable(cases, RustCatchAll::ReturnErrorValue { value }),
                );
                GenBlock::simple_expr(RustExpr::Control(Box::new(match_block)))
            }
        }
    }

    let open_peek = GenStmt::Embed(RustStmt::Expr(
        RustExpr::local(ctxt.input_varname.clone()).call_method("open_peek_context"),
    ));

    // this is a stub for alternate parsing models to replace the `Parser` argument in the context of the expansion
    let ll_context = ProdCtxt { ..ctxt };

    let mut tree_block = expand_matchtree(tree, ll_context);
    let close_peek = GenStmt::Embed(RustStmt::Expr(
        RustExpr::local(ctxt.input_varname.clone())
            .call_method("close_peek_context")
            .wrap_try(),
    ));

    // REVIEW - we could definitely clean up the structural grouping of the pieces below
    let mut stmts =
        Vec::with_capacity(tree_block.stmts.len() + if tree_block.ret.is_some() { 1 } else { 2 });

    stmts.push(open_peek);
    stmts.append(&mut tree_block.stmts);
    let ret = match tree_block.ret {
        None => {
            stmts.push(close_peek);
            None
        }
        Some(expr) => Some(GenExpr::BlockScope(Box::new(GenBlock {
            stmts: vec![
                GenStmt::assign("ret", GenBlock::single_expr(expr)),
                close_peek,
            ],
            ret: Some(GenExpr::Embed(RustExpr::local("ret"))),
        }))),
    };
    GenBlock {
        stmts,
        ret,
        ..tree_block
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
}

#[derive(Clone, Debug)]
enum EngineLogic<ExprT> {
    Slice(RustExpr, Box<CaseLogic<ExprT>>),
    Peek(Box<CaseLogic<ExprT>>),
    Bits(Box<CaseLogic<ExprT>>),
    PeekNot(Box<CaseLogic<ExprT>>),
    /// OffsetPeek(BaseAddr, RelOffset, InnerLogic)
    OffsetPeek(RustExpr, RustExpr, Box<CaseLogic<ExprT>>),
}

impl<ExprT> ToAst for EngineLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = GenBlock>,
{
    type AstElem = GenBlock;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> GenBlock {
        match self {
            EngineLogic::Slice(sz, cl_inner) => {
                let bind_sz_var = RustStmt::assign(
                    Label::from("sz"),
                    RustExpr::Operation(RustOp::AsCast(
                        Box::new(sz.clone()),
                        RustType::from(PrimType::Usize),
                    )),
                )
                .into();
                let try_open_peek = GenStmt::Embed(RustStmt::Expr(
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method_with("start_slice", [RustExpr::local("sz")])
                        .wrap_try(),
                ));
                let bind_ret = GenStmt::assign("ret", cl_inner.to_ast(ctxt).local_try());
                let try_close_peek = GenStmt::Embed(RustStmt::Expr(
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method("end_slice")
                        .wrap_try(),
                ));
                let stmts = vec![bind_sz_var, try_open_peek, bind_ret, try_close_peek];
                let ret = Some(GenExpr::Embed(RustExpr::local("ret")));
                GenBlock::from_parts(stmts, ret)
            }
            EngineLogic::Peek(cl_inner) => {
                let start_peek = RustStmt::Expr(
                    RustExpr::local(ctxt.input_varname.clone()).call_method("open_peek_context"),
                )
                .into();
                let bind_ret = GenStmt::assign("ret", cl_inner.to_ast(ctxt).local_try());
                let try_close_peek = GenStmt::Embed(RustStmt::Expr(
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method("close_peek_context")
                        .wrap_try(),
                ));
                let stmts = vec![start_peek, bind_ret, try_close_peek];
                let ret = Some(GenExpr::Embed(RustExpr::local("ret")));
                GenBlock::from_parts(stmts, ret)
            }
            EngineLogic::OffsetPeek(base_addr, offs, cl_inner) => {
                let bind_tgt_offset_var = GenStmt::Embed(RustStmt::assign(
                    "tgt_offset",
                    RustExpr::infix(base_addr.clone(), InfixOperator::Add, offs.clone()),
                ));
                let advance_or_seek = GenStmt::Embed(RustStmt::assign(
                    "_is_advance",
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method_with("advance_or_seek", [RustExpr::local("tgt_offset")])
                        .wrap_try(),
                ));
                let bind_ret = GenStmt::assign("ret", cl_inner.to_ast(ctxt).local_try());
                let try_close_peek = GenStmt::Embed(RustStmt::Expr(
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method("close_peek_context")
                        .wrap_try(),
                ));
                let stmts = vec![
                    bind_tgt_offset_var,
                    advance_or_seek,
                    bind_ret,
                    try_close_peek,
                ];
                let ret = Some(GenExpr::Embed(RustExpr::local("ret")));
                GenBlock::from_parts(stmts, ret)
            }
            EngineLogic::PeekNot(cl_inner) => {
                let open_peek_not = RustStmt::Expr(
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method("open_peek_not_context"),
                )
                .into();
                let bind_res = GenStmt::assign("res", cl_inner.to_ast(ctxt).abstracted_try());
                let close_or_fail = GenExpr::Control(Box::new(RustControl::If(
                    RustExpr::local("res").call_method("is_err"),
                    GenBlock::simple_expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method("close_peek_not_context")
                            .wrap_try(),
                    ),
                    Some(GenBlock::explicit_return(RustExpr::err(RustExpr::scoped(
                        ["ParseError"],
                        "NegatedSuccess",
                    )))),
                )));
                let stmts = vec![open_peek_not, bind_res];
                GenBlock::from_parts(stmts, Some(close_or_fail))
            }
            EngineLogic::Bits(cl_inner) => {
                let enter_bits = RustStmt::Expr(
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method("enter_bits_mode")
                        .wrap_try(),
                )
                .into();
                let bind_ret = GenStmt::assign("ret", cl_inner.to_ast(ctxt).local_try());
                let escape_bits = GenStmt::Embed(RustStmt::assign(
                    // FIXME: promote to non-hardcoded identifier
                    "_bits_read",
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method("escape_bits_mode")
                        .wrap_try(),
                ));
                let stmts = vec![enter_bits, bind_ret, escape_bits];
                let ret = Some(GenExpr::Embed(RustExpr::local("ret")));
                GenBlock::from_parts(stmts, ret)
            }
        }
    }
}

/// Cases where a constant block of logic is repeated (0 or more times)
#[derive(Clone, Debug)]
enum RepeatLogic<ExprT> {
    /// Evaluates a matchtree and continues if it is matched
    Repeat0ContinueOnMatch(MatchTree, Box<CaseLogic<ExprT>>),
    /// evaluates a matchtree and breaks if it is matched
    Repeat1BreakOnMatch(MatchTree, Box<CaseLogic<ExprT>>),
    /// repeats a specific number of times
    ExactCount(RustExpr, Box<CaseLogic<ExprT>>),
    /// Repeats between N and M times
    BetweenCounts(MatchTree, RustExpr, RustExpr, Box<CaseLogic<ExprT>>),
    /// Repetition stops after a predicate for 'terminal element' is satisfied
    ConditionTerminal(GenLambda, Box<CaseLogic<ExprT>>),
    /// Repetition stops after a predicate for 'complete sequence' is satisfied (post-append)
    ConditionComplete(GenLambda, Box<CaseLogic<ExprT>>),
    /// Lifts an Expr to a sequence of parameters to apply to a format, once per element
    ForEach(RustExpr, Label, Box<CaseLogic<ExprT>>),
    /// Fused logic for a left-fold that is updated on each repeat, and contributes to the condition for termination
    ///
    /// Lambda order: termination-predicate, then update-function
    AccumUntil(GenLambda, GenLambda, RustExpr, Box<CaseLogic<ExprT>>),
}

pub(crate) trait ToAst {
    type AstElem;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> Self::AstElem;
}

impl<ExprT> ToAst for RepeatLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = GenBlock>,
{
    type AstElem = GenBlock;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> GenBlock {
        match self {
            RepeatLogic::Repeat0ContinueOnMatch(continue_tree, elt) => {
                let mut stmts = Vec::new();

                // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(
                    RustStmt::Let(
                        Mut::Mutable,
                        Label::from("accum"),
                        None,
                        RustExpr::scoped(["Vec"], "new").call(),
                    )
                    .into(),
                );
                let ctrl = {
                    let tree_index_expr: RustExpr = invoke_matchtree(continue_tree, ctxt);
                    let bind_ix = RustStmt::assign("matching_ix", tree_index_expr);
                    let cond = RustExpr::infix(
                        RustExpr::local("matching_ix"),
                        InfixOperator::Eq,
                        RustExpr::num_lit(0usize),
                    );
                    let b_continue = [
                        // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
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
                            InfixOperator::Gt,
                            RustExpr::num_lit(0usize),
                        ),
                        vec![bind_ix, RustStmt::Control(escape_clause)],
                    ))
                };
                stmts.push(ctrl.into());
                // FIXME - internal is misleading here but currently accurate due to the lack of interoperability between RustControl and GenContext
                GenBlock::lift_block(stmts, RustExpr::local("accum"))
            }
            RepeatLogic::Repeat1BreakOnMatch(break_tree, elt) => {
                let mut stmts = Vec::new();

                // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(
                    RustStmt::assign_mut("accum", RustExpr::scoped(["Vec"], "new").call()).into(),
                );
                let ctrl = {
                    let tree_index_expr: RustExpr = invoke_matchtree(break_tree, ctxt);
                    let bind_ix = RustStmt::assign("matching_ix", tree_index_expr);
                    let cond = RustExpr::infix(
                        RustExpr::local("matching_ix"),
                        InfixOperator::Eq,
                        RustExpr::num_lit(0usize),
                    );
                    let b_continue = [
                        // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                        RustStmt::assign("next_elem", elt_expr),
                        RustStmt::Expr(
                            RustExpr::local("accum")
                                .call_method_with("push", [RustExpr::local("next_elem")]),
                        ),
                    ]
                    .to_vec();
                    let b_stop = vec![RustStmt::Control(RustControl::If(
                        RustExpr::local("accum").call_method("is_empty"),
                        vec![RustStmt::Return(
                            ReturnKind::Keyword,
                            RustExpr::err(RustExpr::scoped(["ParseError"], "InsufficientRepeats")),
                        )],
                        Some(vec![RustStmt::Control(RustControl::Break)]),
                    ))];
                    let escape_clause = RustControl::If(cond, b_stop, Some(b_continue));
                    RustStmt::Control(RustControl::While(
                        RustExpr::infix(
                            RustExpr::local(ctxt.input_varname.clone()).call_method("remaining"),
                            InfixOperator::Gt,
                            RustExpr::num_lit(0usize),
                        ),
                        vec![bind_ix, RustStmt::Control(escape_clause)],
                    ))
                };
                stmts.push(ctrl.into());
                // FIXME - internal is misleading here but currently accurate due to the lack of interoperability between RustControl and GenContext
                GenBlock::lift_block(stmts, RustExpr::local("accum"))
            }
            RepeatLogic::BetweenCounts(btree, expr_min, expr_max, elt) => {
                let mut stmts = Vec::new();

                // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                let elt_expr = elt.to_ast(ctxt).into();
                stmts.push(
                    RustStmt::assign_mut("accum", RustExpr::scoped(["Vec"], "new").call()).into(),
                );
                let ctrl = {
                    let tree_index_expr: RustExpr = invoke_matchtree(btree, ctxt);
                    let bind_ix = RustStmt::assign("matching_ix", tree_index_expr);
                    let cond = {
                        let tree_cond = RustExpr::infix(
                            RustExpr::local("matching_ix"),
                            InfixOperator::Eq,
                            RustExpr::num_lit(0usize),
                        );
                        let min_cond = RustExpr::infix(
                            RustExpr::local("accum").vec_len(),
                            InfixOperator::Gte,
                            RustExpr::Operation(RustOp::AsCast(
                                Box::new(expr_min.clone()),
                                RustType::from(PrimType::Usize),
                            )),
                        );
                        let max_cond = RustExpr::infix(
                            RustExpr::local("accum").vec_len(),
                            InfixOperator::Eq,
                            RustExpr::Operation(RustOp::AsCast(
                                Box::new(expr_max.clone()),
                                RustType::from(PrimType::Usize),
                            )),
                        );
                        // Workaround for lack of boolean operations in RustOp
                        RustExpr::local("repeat_between_finished")
                            .call_with([tree_cond, min_cond, max_cond])
                            .wrap_try()
                    };
                    let b_continue = [
                        // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                        RustStmt::assign("next_elem", elt_expr),
                        RustStmt::Expr(
                            RustExpr::local("accum")
                                .call_method_with("push", [RustExpr::local("next_elem")]),
                        ),
                    ]
                    .to_vec();
                    let b_stop = vec![RustStmt::Control(RustControl::Break)];
                    let escape_clause = RustControl::If(cond, b_stop, Some(b_continue));
                    RustStmt::Control(RustControl::While(
                        RustExpr::infix(
                            RustExpr::local(ctxt.input_varname.clone()).call_method("remaining"),
                            InfixOperator::Gt,
                            RustExpr::num_lit(0usize),
                        ),
                        vec![bind_ix, RustStmt::Control(escape_clause)],
                    ))
                };
                stmts.push(ctrl.into());
                // FIXME - internal is misleading here but currently accurate due to the lack of interoperability between RustControl and GenContext
                GenBlock::lift_block(stmts, RustExpr::local("accum"))
            }
            RepeatLogic::ForEach(seq, lbl, inner) => {
                let mut stmts = Vec::new();

                // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                let inner_expr = inner.to_ast(ctxt).into();

                stmts.push(RustStmt::Let(
                    Mut::Mutable,
                    Label::from("accum"),
                    None,
                    RustExpr::scoped(["Vec"], "new").call(),
                ));

                let body = vec![RustStmt::Expr(
                    // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible - and we don't even capture it, currently
                    RustExpr::local("accum").call_method_with("push", [inner_expr]),
                )];
                stmts.push(RustStmt::Control(RustControl::ForIter(
                    lbl.clone(),
                    seq.clone(),
                    body,
                )));

                GenBlock::lift_block(stmts, RustExpr::local("accum"))
            }
            RepeatLogic::ExactCount(expr_n, elt) => {
                let mut stmts = Vec::new();

                // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(
                    RustStmt::Let(
                        Mut::Mutable,
                        Label::from("accum"),
                        None,
                        RustExpr::scoped(["Vec"], "new").call(),
                    )
                    .into(),
                );
                // N non-loop blocks rather than 1 block representing an N-iteration loop
                let body = vec![RustStmt::Expr(
                    // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible - and we don't even capture it, currently
                    RustExpr::local("accum").call_method_with("push", [elt_expr]),
                )];
                stmts.push(
                    RustStmt::Control(RustControl::ForRange0(
                        Label::from("_"),
                        expr_n.clone(),
                        body,
                    ))
                    .into(),
                );
                // FIXME - internal is misleading here but currently accurate due to the lack of interoperability between RustControl and GenContext

                GenBlock::lift_block(stmts, RustExpr::local("accum"))
            }
            RepeatLogic::ConditionTerminal(pred_last, elt) => {
                let mut stmts = Vec::new();
                // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(
                    RustStmt::Let(
                        Mut::Mutable,
                        Label::from("accum"),
                        None,
                        RustExpr::scoped(["Vec"], "new").call(),
                    )
                    .into(),
                );
                let ctrl = {
                    // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                    let elt_bind = RustStmt::assign("elem", elt_expr);
                    let cond = pred_last.beta_reduce(
                        RustExpr::Borrow(Box::new(RustExpr::local("elem"))),
                        ExprInfo::default(),
                    );
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
                stmts.push(ctrl.into());
                GenBlock::lift_block(stmts, RustExpr::local("accum"))
            }
            RepeatLogic::ConditionComplete(pred_full, elt) => {
                let mut stmts = Vec::new();
                // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(
                    RustStmt::assign_mut("accum", RustExpr::scoped(["Vec"], "new").call()).into(),
                );
                let ctrl = {
                    // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                    let elt_bind = RustStmt::assign("elem", elt_expr);
                    let elt_push = RustStmt::Expr(
                        RustExpr::local("accum")
                            .call_method_with("push", [RustExpr::local("elem")]),
                    );
                    let cond = pred_full.apply(
                        RustExpr::borrow_of(RustExpr::local("accum")),
                        ExprInfo::default(),
                    );
                    let b_terminal = [RustStmt::Control(RustControl::Break)].to_vec();
                    let escape_clause = RustControl::If(cond, b_terminal, None);
                    RustStmt::Control(RustControl::Loop(vec![
                        elt_bind,
                        elt_push,
                        RustStmt::Control(escape_clause),
                    ]))
                };
                stmts.push(ctrl.into());
                GenBlock::lift_block(stmts, RustExpr::local("accum"))
            }
            RepeatLogic::AccumUntil(cond, update, init, elt) => {
                let mut stmts = Vec::new();
                // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                let elt_expr = elt.to_ast(ctxt).into();
                let seq_type = match cond.head_type.to_rust_type() {
                    RustType::AnonTuple(ts) => ts[1].clone(),
                    other => unreachable!("bad type {other:?}"),
                };
                stmts.push(
                    RustStmt::Let(
                        Mut::Mutable,
                        Label::Borrowed("seq"),
                        Some(seq_type),
                        RustExpr::scoped(["Vec"], "new").call(),
                    )
                    .into(),
                );
                stmts.push(RustStmt::assign_mut("acc", init.clone()));
                let ctrl = {
                    let done_call = cond.apply_pair(
                        // REVIEW[epic=clone-of-copy] - figure out if we can avoid cloning copy-types here
                        RustExpr::CloneOf(Box::new(RustExpr::local("acc"))),
                        RustExpr::local("seq"),
                        ExprInfo::default(),
                    );
                    let break_if_done = RustStmt::Control(RustControl::If(
                        done_call,
                        vec![RustStmt::Control(RustControl::Break)],
                        None,
                    ));
                    // FIXME[epic=sigbind-missing] - We can't sigbind this due to GenStmt and RustControl being incompatible
                    let elt_bind = RustStmt::assign("elem", elt_expr);
                    let push_elt = RustStmt::Expr(RustExpr::local("seq").call_method_with(
                        "push",
                        [RustExpr::CloneOf(Box::new(RustExpr::local("elem")))],
                    ));
                    let new_acc = update.apply_pair(
                        RustExpr::local("acc"),
                        RustExpr::local("elem"),
                        ExprInfo::default(),
                    );
                    let update_acc = RustStmt::Reassign(Label::Borrowed("acc"), new_acc);
                    RustStmt::Control(RustControl::Loop(vec![
                        break_if_done,
                        elt_bind,
                        push_elt,
                        update_acc,
                    ]))
                };
                stmts.push(ctrl.into());
                GenBlock::lift_block(
                    stmts,
                    RustExpr::Tuple(vec![RustExpr::local("acc"), RustExpr::local("seq")]),
                )
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
}

impl<ExprT> ToAst for SequentialLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = GenBlock>,
{
    type AstElem = GenBlock;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> GenBlock {
        match self {
            SequentialLogic::AccumTuple {
                constructor,
                elements,
            } => {
                if elements.is_empty() {
                    return GenBlock::simple_expr(RustExpr::UNIT);
                }

                let mut names: Vec<Label> = Vec::new();
                let mut body = Vec::new();

                for (ix, elt_cl) in elements.iter().enumerate() {
                    let varname = format!("field{}", ix);
                    names.push(varname.clone().into());
                    body.push(GenStmt::assign(varname, elt_cl.to_ast(ctxt).local_try()));
                }

                if let Some(con) = constructor {
                    // FIXME - In addition to local rule-interpretation for each tuple positional, we also want to selectively box the elements, either at site-of-binding or in the struct expr
                    GenBlock::from_parts(
                        body,
                        Some(GenExpr::TyValCon(RustExpr::Struct(
                            con.clone(),
                            StructExpr::TupleExpr(names.into_iter().map(RustExpr::local).collect()),
                        ))),
                    )
                } else {
                    // FIXME - In addition to local rule-interpretation for each tuple positional, we also want to selectively box the elements, either at site-of-binding or in the struct expr
                    GenBlock::from_parts(
                        body,
                        Some(GenExpr::TyValCon(RustExpr::Tuple(
                            names.into_iter().map(RustExpr::local).collect(),
                        ))),
                    )
                }
            }
        }
    }
}

/// Catch-all for hard-to-classify cases
#[derive(Clone, Debug)]
enum OtherLogic<ExprT> {
    Descend(MatchTree, Vec<CaseLogic<ExprT>>),
    ExprMatch(
        RustExpr,
        Vec<(MatchCaseLHS, CaseLogic<ExprT>)>,
        Refutability,
    ),
    LetFormat(Box<CaseLogic<ExprT>>, Label, Box<CaseLogic<ExprT>>),
    MonadSeq(Box<CaseLogic<ExprT>>, Box<CaseLogic<ExprT>>),
    Hint(StyleHint, Box<CaseLogic<ExprT>>),
}

impl<ExprT> ToAst for OtherLogic<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = GenBlock>,
{
    type AstElem = GenBlock;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> GenBlock {
        match self {
            OtherLogic::Descend(tree, cases) => {
                let mut branches = Vec::new();
                for (ix, case) in cases.iter().enumerate() {
                    let case_block = case.to_ast(ctxt);

                    branches.push((
                        MatchCaseLHS::Pattern(RustPattern::PrimLiteral(RustPrimLit::Numeric(
                            RustNumLit::Usize(ix),
                        ))),
                        case_block,
                    ));
                }
                let bind =
                    GenStmt::Embed(RustStmt::assign("tree_index", invoke_matchtree(tree, ctxt)));
                let ctrl = {
                    let fallthrough = RustExpr::err(
                        RustExpr::scoped(["ParseError"], "ExcludedBranch")
                            .call_with([RustExpr::u64lit(get_trace(&(tree, "fallthrough")))]),
                    );
                    RustControl::Match(
                        RustExpr::local("tree_index"),
                        RustMatchBody::Refutable(
                            branches,
                            RustCatchAll::ReturnErrorValue { value: fallthrough },
                        ),
                    )
                };
                GenBlock::from_parts(vec![bind], Some(GenExpr::Control(Box::new(ctrl))))
            }
            OtherLogic::ExprMatch(expr, cases, ck) => {
                let mut branches = Vec::new();
                for (lhs, logic) in cases.iter() {
                    let case_block = logic.to_ast(ctxt);
                    branches.push((lhs.clone(), case_block));
                }

                let match_body = match ck {
                    Refutability::Refutable | Refutability::Indeterminate => {
                        RustMatchBody::Refutable(
                            branches,
                            RustCatchAll::PanicUnreachable {
                                message: Label::from("ExprMatch refuted: "),
                            },
                        )
                    }
                    Refutability::Irrefutable => RustMatchBody::Irrefutable(branches),
                };
                let match_expr = GenExpr::embed_match(expr.clone(), match_body);
                GenBlock::single_expr(match_expr)
            }
            OtherLogic::LetFormat(prior, name, inner) => {
                let prior_block = prior.to_ast(ctxt);
                let mut inner_block = inner.to_ast(ctxt);
                // REVIEW - indexing scheme must be resilient to prepend and append...
                inner_block
                    .stmts
                    .insert(0, GenStmt::assign(name.clone(), prior_block));
                inner_block
            }
            OtherLogic::MonadSeq(prior, inner) => {
                let prior_block = prior.to_ast(ctxt);
                let mut inner_block = inner.to_ast(ctxt);

                // REVIEW - is there a better construction we can use instead of this?
                let prior_stmt = GenStmt::Expr(GenExpr::BlockScope(Box::new(prior_block)));

                inner_block.prepend_stmt(prior_stmt);
                inner_block
            }
            OtherLogic::Hint(_hint, inner) => {
                let inner_block = inner.to_ast(ctxt);

                // REVIEW - do we want to perform any local modifications?
                inner_block
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
    CaseLogic<ExprT>: ToAst<AstElem = GenBlock>,
{
    type AstElem = GenBlock;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> GenBlock {
        match self {
            ParallelLogic::Alts(alts) => {
                let l = alts.len();

                let stmts = Iterator::chain(
                    std::iter::once(GenStmt::Embed(RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone()).call_method("start_alt"),
                    ))),
                    alts.iter().enumerate().flat_map(|(ix, branch_cl)| {
                        let on_err = match l - ix {
                            0 => unreachable!("index matches overall length"),
                            1 => GenBlock::explicit_return(RustExpr::err(RustExpr::local("_e"))),
                            2 => GenBlock::mono_statement(RustStmt::Expr(
                                RustExpr::local(ctxt.input_varname.clone())
                                    .call_method_with("next_alt", [RustExpr::TRUE])
                                    .wrap_try(),
                            )),
                            3.. => GenBlock::mono_statement(RustStmt::Expr(
                                RustExpr::local(ctxt.input_varname.clone())
                                    .call_method_with("next_alt", [RustExpr::FALSE])
                                    .wrap_try(),
                            )),
                        };
                        let branch_result = branch_cl.to_ast(ctxt).abstracted_try();
                        let bind_res = GenStmt::assign("res", branch_result);
                        let ctrl = RustControl::Match(
                            RustExpr::local("res"),
                            RustMatchBody::Irrefutable(vec![
                                (
                                    MatchCaseLHS::Pattern(RustPattern::Variant(
                                        Constructor::Simple(Label::from("Ok")),
                                        Box::new(RustPattern::CatchAll(Some(Label::from("inner")))),
                                    )),
                                    GenBlock::explicit_return(
                                        RustExpr::local("inner").wrap_ok(Some("PResult")),
                                    ),
                                ),
                                (
                                    MatchCaseLHS::Pattern(RustPattern::Variant(
                                        Constructor::Simple(Label::from("Err")),
                                        Box::new(RustPattern::CatchAll(Some(Label::from("_e")))),
                                    )),
                                    on_err,
                                ),
                            ]),
                        );
                        [bind_res, GenStmt::Expr(GenExpr::Control(Box::new(ctrl)))]
                    }),
                )
                .collect();
                GenBlock::from_parts(stmts, None)
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
    YieldCurrentOffset,
    SkipRemainder,
    ConstNone,
}

/// Cases that recurse into other case-logic only once
#[derive(Clone, Debug)]
enum DerivedLogic<ExprT> {
    WrapSome(Box<CaseLogic<ExprT>>),
    VariantOf(Constructor, Box<CaseLogic<ExprT>>),
    UnitVariantOf(Constructor, Box<CaseLogic<ExprT>>),
    MapOf(GenLambda, Box<CaseLogic<ExprT>>),
    Let(Label, RustExpr, Box<CaseLogic<ExprT>>),
    Dynamic(DynamicLogic<ExprT>, Box<CaseLogic<ExprT>>),
    Where(GenLambda, Box<CaseLogic<ExprT>>),
    Maybe(RustExpr, Box<CaseLogic<ExprT>>),
    DecodeBytes(RustExpr, Box<CaseLogic<ExprT>>),
}

#[derive(Clone, Debug)]
enum DynamicLogic<ExprT> {
    Huffman(Label, ExprT, Option<ExprT>),
}

impl ToAst for DynamicLogic<GTExpr> {
    type AstElem = RustStmt;

    fn to_ast(&self, _ctxt: ProdCtxt<'_>) -> Self::AstElem {
        match self {
            DynamicLogic::Huffman(lbl, code_lengths, opt_values_expr) => {
                let info = ExprInfo::EmbedCloned;
                let rhs = {
                    let opt_values_lifted = match opt_values_expr {
                        None => RustExpr::NONE,
                        Some(x) => RustExpr::some(embed_expr(x, info)),
                    };
                    RustExpr::local("parse_huffman")
                        .call_with([embed_expr(code_lengths, info), opt_values_lifted])
                };
                RustStmt::Let(Mut::Immutable, lbl.clone(), None, rhs)
            }
        }
    }
}

impl ToAst for DerivedLogic<GTExpr> {
    type AstElem = GenBlock;

    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> GenBlock {
        match self {
            DerivedLogic::Dynamic(dyn_logic, inner_cl) => {
                let GenBlock { mut stmts, ret } = inner_cl.to_ast(ctxt);
                let _stmts = &mut stmts;
                let mut stmts = Vec::with_capacity(_stmts.len() + 1);
                // REVIEW - we need an indexing model that is prepend/append-stable, or an internal method to add GenStmts before/after
                stmts.push(GenStmt::Embed(dyn_logic.to_ast(ctxt)));
                stmts.append(_stmts);
                GenBlock { stmts, ret }
            }
            DerivedLogic::DecodeBytes(bytes_expr, inner_cl) => {
                const INNER_NAME: &str = "reparser";

                // pre-generate local byte-context to parse, and logic to parse inner_cl within it
                let bytes_ctxt = ProdCtxt {
                    input_varname: &Cow::Borrowed(INNER_NAME),
                };
                let mut inner_block = inner_cl.to_ast(bytes_ctxt);

                // prepend boilerplate for second-phase byte-parsing
                let persist_parser = GenStmt::Embed(RustStmt::assign_mut(
                    "tmp",
                    RustExpr::scoped(["Parser"], "new")
                        .call_with([RustExpr::vec_as_slice(bytes_expr.clone())]),
                ));
                let bind_ref_mut_parser = GenStmt::Embed(RustStmt::assign(
                    INNER_NAME,
                    RustExpr::BorrowMut(Box::new(RustExpr::local("tmp"))),
                ));

                inner_block.prepend_stmts([persist_parser, bind_ref_mut_parser]);
                inner_block
            }
            DerivedLogic::Maybe(is_present, inner_cl) => {
                let mut if_true = inner_cl.to_ast(ctxt);
                match if_true.ret {
                    None => unreachable!("UNEXPECTED: inner logic of Format::Maybe context does not have return-value"),
                    Some(expr) => if_true.ret = Some(expr.wrap_some()),
                }
                let if_false = GenBlock::simple_expr(RustExpr::local("None"));
                let ctrl = RustControl::If(is_present.clone(), if_true, Some(if_false));
                GenBlock::single_expr(GenExpr::Control(Box::new(ctrl)))
            }
            DerivedLogic::VariantOf(constr, inner) => {
                const BIND_NAME: &str = "inner";
                let sigbind_inner = GenStmt::assign(
                    // REVIEW - consider adding a consts module for naming each used binding with greater global visibility
                    BIND_NAME,
                    inner.to_ast(ctxt),
                );
                let stmts = vec![sigbind_inner];
                let ret = Some(GenExpr::TyValCon(RustExpr::Struct(
                    constr.clone(),
                    StructExpr::TupleExpr(vec![RustExpr::local(BIND_NAME)]),
                )));
                GenBlock::from_parts(stmts, ret)
            }
            DerivedLogic::WrapSome(inner) => {
                let mut inner_block = inner.to_ast(ctxt);
                if let Some(ret) = inner_block.ret.take() {
                    inner_block.ret.replace(ret.wrap_some());
                } else {
                    unreachable!("SomeOf called on non-value-producing GenBlock: {inner_block:?}");
                };
                inner_block
            }
            DerivedLogic::UnitVariantOf(constr, inner) => {
                let inner_block = inner.to_ast(ctxt);
                if inner_block.stmts.last().is_some_and(|s| {
                    matches!(s, GenStmt::Embed(RustStmt::Return(ReturnKind::Keyword, _)))
                }) {
                    debug_assert!(inner_block.ret.is_none(), "explicit return precedes implicitly returned value in block-scope expression");
                    // NOTE - if the last statement is an explicit return, pass-through as-is because there is no variant to construct
                    inner_block
                } else {
                    match RustStmt::assign_and_forget(RustExpr::from(inner_block)) {
                        Some(inner) => GenBlock::lift_block(
                            [inner],
                            RustExpr::Struct(constr.clone(), StructExpr::EmptyExpr),
                        ),
                        None => GenBlock::simple_expr(RustExpr::Struct(
                            constr.clone(),
                            StructExpr::EmptyExpr,
                        )),
                    }
                }
            }
            DerivedLogic::MapOf(f, inner) => {
                let varname = f.get_head_var();
                let assign_inner = GenStmt::assign(varname.clone(), inner.to_ast(ctxt));
                GenBlock::from_parts(
                    vec![assign_inner],
                    // REVIEW - figure out how to mesh GenExpr/GenBlock and GenLambda models
                    Some(GenExpr::Embed(f.apply(
                        RustExpr::local(varname.clone()),
                        ExprInfo::default(),
                    ))),
                )
            }
            DerivedLogic::Where(f, inner) => {
                let assign_inner = GenStmt::assign("inner", inner.to_ast(ctxt));
                let arg = RustExpr::local("inner");
                let is_valid = f.apply(arg, ExprInfo::default());
                let bind_cond = GenStmt::Embed(RustStmt::assign("is_valid", is_valid));
                let ctrl = {
                    let b_valid = GenBlock::simple_expr(RustExpr::local("inner"));
                    let b_invalid = GenBlock::explicit_return(RustExpr::err(
                        RustExpr::scoped(["ParseError"], "FalsifiedWhere")
                            .call_with([RustExpr::u64lit(get_trace(&()))]),
                    ));
                    RustControl::If(RustExpr::local("is_valid"), b_valid, Some(b_invalid))
                };
                let stmts = vec![assign_inner, bind_cond];
                GenBlock::from_parts(stmts, Some(GenExpr::Control(Box::new(ctrl))))
            }
            DerivedLogic::Let(name, expr, inner) => {
                let mut stmts = Vec::new();
                stmts.push(GenStmt::Embed(RustStmt::assign(name.clone(), expr.clone())));
                let mut inner_block = inner.to_ast(ctxt);
                // REVIEW - figure out indexing model that doesn't break on modification
                stmts.append(&mut inner_block.stmts);
                GenBlock {
                    stmts,
                    ..inner_block
                }
            }
        }
    }
}

// ANCHOR[main-fn] - `generate_code` function
pub fn generate_code(module: &FormatModule, top_format: &Format) -> impl ToFragment {
    let mut items = Vec::new();

    let Generator {
        sourcemap,
        mut elaborator,
    } = Generator::compile(module, top_format);
    let mut table = elaborator.codegen.name_gen.manifest_renaming_table();
    // Set of identifiers we have picked as bespoke names for decoder functions based on the type they are parsing (rather than sequentially enumerated)
    let mut fn_renames = BTreeSet::<Label>::new();
    let type_context = &elaborator.codegen.defined_types[..];
    let src_context = rust_ast::analysis::SourceContext::from(type_context);
    let mut type_defs = Vec::from_iter(elaborator.codegen.defined_types.iter().map(|type_def| {
        elaborator
            .codegen
            .name_gen
            .rev_map
            .get_key_value(type_def)
            .unwrap()
    }));
    type_defs.sort_by_key(|(_, (ix, _))| ix);
    const HEAP_STRATEGY: HeapStrategy =
        HeapStrategy::new().variant_cutoff(128)
        // .absolute_cutoff(128)
        ;

    for (type_def, (_ix, path)) in type_defs.into_iter() {
        let name = elaborator
            .codegen
            .name_gen
            .ctxt
            .find_name_for(path)
            .expect("no name found");
        let traits = if type_def.copy_hint(&src_context) {
            // Derive `Copy` if we can statically infer the definition to be compatible with `Copy`
            // TODO - it might be possible to track which LocalDef items have already been marked Copy, but even that isn't perfect if the def follows its first reference
            TraitSet::DebugCopy
        } else {
            TraitSet::DebugClone
        };
        let it = RustItem::pub_decl_with_traits(RustDecl::type_def(name, type_def.clone()), traits);
        let comments = {
            let sz_comment = format!(
                "expected size: {}",
                rust_ast::analysis::MemSize::size_hint(type_def, &src_context)
            );
            let outcome = HeapOptimize::heap_hint(type_def, HEAP_STRATEGY, &src_context);
            if outcome.0.is_noop() {
                vec![sz_comment]
            } else {
                let heap_comment = format!("heap outcome ({:?}): {:?}", HEAP_STRATEGY, outcome);
                vec![sz_comment, heap_comment]
            }
        };
        items.push(it.with_comment(comments));
    }

    for mut decoder_fn in sourcemap.decoder_skels {
        decoder_fn.rebind(&table);
        match &decoder_fn.adhoc_name {
            Some(name) => {
                let replacement_name = Label::from(format!("Decoder_{}", sanitize_label(name)));
                // If the ideal name already exists, prevent it from being reused
                if fn_renames.contains(&replacement_name) {
                    let _ = decoder_fn.adhoc_name.take();
                } else {
                    fn_renames.insert(replacement_name.clone());
                    table.insert(decoder_fname(decoder_fn.ixlabel), replacement_name);
                }
            }
            None => (),
        };
        let func = decoder_fn.to_ast(ProdCtxt::default());
        items.push(RustItem::from_decl(RustDecl::Function(func)));
    }

    let mut content = RustProgram::from_iter(items);
    content.add_import(RustImport {
        path: vec!["doodle".into(), "prelude".into()],
        uses: RustImportItems::Wildcard,
    });
    content.add_import(RustImport {
        path: vec!["doodle".into()],
        uses: RustImportItems::Singleton(Label::Borrowed("try_sub")),
    });
    for attr_string in ["non_camel_case_types", "non_snake_case", "dead_code"].into_iter() {
        content.add_module_attr(ModuleAttr::Allow(AllowAttr::from(Label::from(attr_string))));
    }
    content.add_module_attr(ModuleAttr::RustFmtSkip);
    content.add_submodule(RustSubmodule::new("codegen_tests"));
    content.add_submodule(RustSubmodule::new_pub("api_helper"));
    content.rebind(&table);
    content
}

#[derive(Clone, Debug)]
pub struct DecoderFn<ExprT> {
    adhoc_name: Option<Label>,
    ixlabel: IxLabel,
    logic: CaseLogic<ExprT>,
    extra_args: Option<Vec<(Label, GenType)>>,
    ret_type: RustType,
}

impl<ExprT> Rebindable for DecoderFn<ExprT> {
    fn rebind(&mut self, table: &impl MapLike<Label, Label>) {
        if let Some(ref mut name) = self.adhoc_name {
            if table.contains_key(&*name) {
                *name = table.index(&*name).clone();
            }
        }
    }
}

fn decoder_fname(ixlabel: IxLabel) -> Label {
    Label::from(format!("Decoder{}", ixlabel.to_usize()))
}

impl<ExprT> ToAst for DecoderFn<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = GenBlock>,
    ExprT: std::fmt::Debug,
{
    type AstElem = RustFn;

    fn to_ast(&self, _ctxt: ProdCtxt<'_>) -> RustFn {
        let name = decoder_fname(self.ixlabel);
        let params = DefParams::new();
        let sig = {
            let args = {
                let arg0 = {
                    let name = "_input".into();
                    let ty = {
                        let mut params = RustParams::<RustLt, RustType>::new();
                        params.push_lifetime(RustLt::Parametric("'_".into()));
                        RustType::borrow_of(
                            None,
                            Mut::Mutable,
                            RustType::verbatim("Parser", Some(params)),
                        )
                    };
                    (name, ty)
                };
                if let Some(ref args) = self.extra_args {
                    Iterator::chain(
                        std::iter::once(arg0),
                        args.iter().map(|(lab, gt)| {
                            (
                                lab.clone(),
                                RustType::selective_borrow(None, Mut::Immutable, gt.to_rust_type()),
                            )
                        }),
                    )
                    .collect()
                } else {
                    [arg0].to_vec()
                }
            };
            FnSig::new(
                args,
                Some(RustType::result_of(
                    self.ret_type.clone(),
                    RustType::imported("ParseError"),
                )),
            )
        };
        let ctxt = ProdCtxt {
            input_varname: &Label::from("_input"),
        };
        // NOTE - this is the last place we can modify the GenBlock before it is manifested as pure RustAST constructs
        let self_block = self.logic.to_ast(ctxt);

        let (stmts, ret) = self_block.synthesize();
        let body = if let Some(ret) = ret {
            Iterator::chain(
                stmts.into_iter(),
                std::iter::once(RustStmt::Return(
                    ReturnKind::Implicit,
                    ret.wrap_ok(Some("PResult")),
                )),
            )
            .collect()
        } else {
            stmts
        };

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
            .unwrap_or_else(|err| panic!("Failed to infer top-level format type: {err}"));
        let mut gen = Self {
            elaborator: Elaborator::new(module, tc, CodeGen::new()),
            sourcemap: SourceMap::new(),
        };
        let elab = &mut gen.elaborator;

        let top = elab.elaborate_format(top_format, &TypedDynScope::Empty);
        // assert_eq!(elab.next_index, elab.tc.size());
        let prog = GTCompiler::compile_program(module, &top).expect("failed to compile program");
        for (ix, (dec_ext, t)) in prog.decoders.iter().enumerate() {
            let dec_fn = {
                let dec = dec_ext.get_dec();
                let args = dec_ext.get_args();
                let dec_gt = dec.get_type();
                let adhoc_name = dec_gt.and_then(|t| match t.as_ref() {
                    GenType::Def((_, name), ..) => Some(name.clone()),
                    _ => None,
                });
                let cl = elab.codegen.translate(dec);
                DecoderFn {
                    adhoc_name,
                    ixlabel: IxLabel::from(ix),
                    logic: cl,
                    extra_args: args.clone(),
                    ret_type: t.to_rust_type(),
                }
            };
            gen.sourcemap.decoder_skels.push(dec_fn);
        }

        gen
    }
}

pub struct Elaborator<'a> {
    module: &'a FormatModule,
    next_index: usize,
    t_formats: StableMap<usize, Rc<GTFormat>, BTree>,
    tc: TypeChecker,
    codegen: CodeGen,
}

impl<'a> Elaborator<'a> {
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

    fn elaborate_dynamic_format(&mut self, dynf: &DynFormat) -> TypedDynFormat<GenType> {
        match dynf {
            DynFormat::Huffman(code_lengths, opt_values_expr) => {
                // for dynf itself
                self.increment_index();
                let t_codes = self.elaborate_expr(code_lengths);
                // for the element-type of code_lengths
                self.increment_index();

                let boxed_t_values_expr = opt_values_expr.as_ref().map(|values_expr| {
                    let t_values = self.elaborate_expr(values_expr);
                    // for the element-type of opt_values_expr
                    self.increment_index();
                    Box::new(t_values)
                });
                GTDynFormat::Huffman(Box::new(t_codes), boxed_t_values_expr)
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
            Pattern::Int(bounds) => {
                let gt = self.get_gt_from_index(index);
                GTPattern::Int(gt, *bounds)
            }
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
            Pattern::Option(opt) => {
                let t_pat = if let Some(pat) = opt.as_ref() {
                    Some(Box::new(self.elaborate_pattern(pat)))
                } else {
                    self.increment_index();
                    None
                };
                let gt = self.get_gt_from_index(index);
                GTPattern::Option(gt, t_pat)
            }
        }
    }

    pub fn new(module: &'a FormatModule, tc: TypeChecker, codegen: CodeGen) -> Self {
        Self {
            module,
            next_index: 0,
            t_formats: Default::default(),
            tc,
            codegen,
        }
    }

    fn elaborate_format_union(
        &mut self,
        branches: &[Format],
        dyn_scope: &TypedDynScope<'_>,
        is_det: bool,
    ) -> GTFormat {
        let index = self.get_and_increment_index();
        let gt = self.get_gt_from_index(index);

        let mut t_branches = Vec::with_capacity(branches.len());
        for branch in branches.iter() {
            match branch {
                Format::Variant(name, inner) => {
                    self.codegen
                        .name_gen
                        .ctxt
                        .push_atom(NameAtom::Variant(name.clone()));
                    let t_inner = self.elaborate_format(inner, dyn_scope);
                    self.codegen.name_gen.ctxt.escape();
                    if t_inner.get_type().is_none() {
                        continue;
                    }
                    let t_branch =
                        TypedFormat::Variant(gt.clone(), name.clone(), Box::new(t_inner));
                    t_branches.push(t_branch);
                }
                _ => {
                    let t_branch = self.elaborate_format(branch, dyn_scope);
                    t_branches.push(t_branch);
                }
            }
        }

        if is_det {
            TypedFormat::Union(gt, t_branches)
        } else {
            TypedFormat::UnionNondet(gt, t_branches)
        }
    }

    fn elaborate_format(&mut self, format: &Format, dyn_scope: &TypedDynScope<'_>) -> GTFormat {
        match format {
            Format::ItemVar(level, args) => {
                self.codegen
                    .name_gen
                    .ctxt
                    .push_atom(NameAtom::Explicit(Label::from(
                        self.module.get_name(*level).to_string(),
                    )));
                let index = self.get_and_increment_index();
                let fm_args = &self.module.args[*level];

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let mut t_args = Vec::with_capacity(args.len());
                for ((lbl, _), arg) in Iterator::zip(fm_args.iter(), args.iter()) {
                    let t_arg = self.elaborate_expr(arg);
                    t_args.push((lbl.clone(), t_arg));
                }
                self.codegen.name_gen.ctxt.escape();

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
                self.codegen.name_gen.ctxt.escape();
                TypedFormat::FormatCall(gt, *level, t_args, t_inner)
            }
            Format::ForEach(expr, lbl, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.elaborate_expr(expr);
                self.increment_index();
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::ForEach(gt, Box::new(t_expr), lbl.clone(), Box::new(t_inner))
            }
            Format::DecodeBytes(expr, inner) => {
                let index = self.get_and_increment_index();

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_expr = self.elaborate_expr(expr);
                self.codegen.name_gen.ctxt.escape();

                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::DecodeBytes(gt, Box::new(t_expr), Box::new(t_inner))
            }
            Format::Fail => {
                self.increment_index();
                TypedFormat::Fail
            }
            Format::EndOfInput => {
                self.increment_index();
                TypedFormat::EndOfInput
            }
            Format::SkipRemainder => {
                self.increment_index();
                TypedFormat::SkipRemainder
            }
            Format::Align(n) => {
                self.increment_index();
                TypedFormat::Align(*n)
            }
            Format::Pos => {
                self.increment_index();
                TypedFormat::Pos
            }
            Format::Byte(bs) => {
                self.increment_index();
                TypedFormat::Byte(*bs)
            }
            Format::Variant(label, inner) => {
                let index = self.get_and_increment_index();
                self.codegen
                    .name_gen
                    .ctxt
                    .push_atom(NameAtom::Variant(label.clone()));
                let t_inner = self.elaborate_format(inner, dyn_scope);
                self.codegen.name_gen.ctxt.escape();
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
                TypedFormat::Variant(gt, label.clone(), Box::new(t_inner))
            }
            Format::Union(branches) => self.elaborate_format_union(branches, dyn_scope, true),
            Format::UnionNondet(branches) => {
                self.elaborate_format_union(branches, dyn_scope, false)
            }
            Format::Tuple(elts) => {
                let index = self.get_and_increment_index();
                let t_elts = match &elts[..] {
                    [] => Vec::new(),
                    [v] => vec![self.elaborate_format(v, dyn_scope)],
                    elts => {
                        let mut t_elts = Vec::with_capacity(elts.len());
                        self.codegen
                            .name_gen
                            .ctxt
                            .push_atom(NameAtom::Positional(0));
                        for t in elts {
                            let t_elt = self.elaborate_format(t, dyn_scope);
                            t_elts.push(t_elt);
                            self.codegen.name_gen.ctxt.increment_index();
                        }
                        self.codegen.name_gen.ctxt.escape();
                        t_elts
                    }
                };
                let gt = self.get_gt_from_index(index);
                TypedFormat::Tuple(gt, t_elts)
            }
            Format::Repeat(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::Repeat(gt, Box::new(t_inner))
            }
            Format::Repeat1(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::Repeat1(gt, Box::new(t_inner))
            }
            Format::RepeatCount(expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.elaborate_expr(expr);
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::RepeatCount(gt, Box::new(t_expr), Box::new(t_inner))
            }
            Format::RepeatBetween(min_expr, max_expr, inner) => {
                let index = self.get_and_increment_index();
                let t_min_expr = self.elaborate_expr(min_expr);
                let t_max_expr = self.elaborate_expr(max_expr);
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::RepeatBetween(
                    gt,
                    Box::new(t_min_expr),
                    Box::new(t_max_expr),
                    Box::new(t_inner),
                )
            }
            Format::RepeatUntilLast(lambda, inner) => {
                let index = self.get_and_increment_index();
                let t_lambda = self.elaborate_expr_lambda(lambda);
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::RepeatUntilLast(gt, Box::new(t_lambda), Box::new(t_inner))
            }
            Format::RepeatUntilSeq(lambda, inner) => {
                let index = self.get_and_increment_index();
                let t_lambda = self.elaborate_expr_lambda(lambda);
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::RepeatUntilSeq(gt, Box::new(t_lambda), Box::new(t_inner))
            }
            Format::AccumUntil(cond, update, init, _vt, inner) => {
                let index = self.get_and_increment_index();
                let t_cond = self.elaborate_expr_lambda(cond);
                let t_update = self.elaborate_expr_lambda(update);
                let t_init = self.elaborate_expr(init);
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::AccumUntil(
                    gt,
                    Box::new(t_cond),
                    Box::new(t_update),
                    Box::new(t_init),
                    _vt.clone(),
                    Box::new(t_inner),
                )
            }
            Format::Maybe(cond, inner) => {
                let index = self.get_and_increment_index();
                let t_cond = self.elaborate_expr(cond);
                self.codegen
                    .name_gen
                    .ctxt
                    .push_atom(NameAtom::Derived(Derivation::Yes));
                let t_inner = self.elaborate_format(inner, dyn_scope);
                self.codegen.name_gen.ctxt.escape();
                let gt = self.get_gt_from_index(index);
                TypedFormat::Maybe(gt, Box::new(t_cond), Box::new(t_inner))
            }
            Format::Peek(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::Peek(gt, Box::new(t_inner))
            }
            Format::PeekNot(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::PeekNot(gt, Box::new(t_inner))
            }
            Format::Slice(expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.elaborate_expr(expr);
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::Slice(gt, Box::new(t_expr), Box::new(t_inner))
            }
            Format::Bits(inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::Bits(gt, Box::new(t_inner))
            }
            Format::WithRelativeOffset(base_addr, expr, inner) => {
                let index = self.get_and_increment_index();
                let t_base_addr = self.elaborate_expr(base_addr);
                let t_expr = self.elaborate_expr(expr);
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::WithRelativeOffset(
                    gt,
                    Box::new(t_base_addr),
                    Box::new(t_expr),
                    Box::new(t_inner),
                )
            }
            Format::Map(inner, lambda) => {
                // FIXME - adhoc types introduced by Map are not properly path-named
                let index = self.get_and_increment_index();
                self.codegen
                    .name_gen
                    .ctxt
                    .push_atom(NameAtom::Derived(Derivation::Preimage));
                let t_inner = self.elaborate_format(inner, dyn_scope);
                self.codegen.name_gen.ctxt.escape();
                let t_lambda = self.elaborate_expr_lambda(lambda);
                let gt = self.get_gt_from_index(index);
                TypedFormat::Map(gt, Box::new(t_inner), Box::new(t_lambda))
            }
            Format::Where(inner, lambda) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let t_lambda = self.elaborate_expr_lambda(lambda);
                let gt = self.get_gt_from_index(index);
                TypedFormat::Where(gt, Box::new(t_inner), Box::new(t_lambda))
            }
            Format::Compute(expr) => {
                // FIXME - adhoc types introduced by Compute are not properly path-named
                let index = self.get_and_increment_index();
                let t_expr = self.elaborate_expr(expr);
                let gt = self.get_gt_from_index(index);
                TypedFormat::Compute(gt, Box::new(t_expr))
            }
            Format::Let(lbl, expr, inner) => {
                let index = self.get_and_increment_index();
                let t_expr = self.elaborate_expr(expr);
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::Let(gt, lbl.clone(), Box::new(t_expr), Box::new(t_inner))
            }
            Format::Match(x, branches) => {
                let index = self.get_and_increment_index();

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_x = self.elaborate_expr(x);
                self.codegen.name_gen.ctxt.escape();

                let mut t_branches = Vec::with_capacity(branches.len());
                for (pat, rhs) in branches {
                    self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                    let t_pat = self.elaborate_pattern(pat);
                    self.codegen.name_gen.ctxt.escape();

                    let t_rhs = self.elaborate_format(rhs, dyn_scope);
                    t_branches.push((t_pat, t_rhs));
                }
                let gt = self.get_gt_from_index(index);
                TypedFormat::Match(gt, Box::new(t_x), t_branches)
            }
            Format::Dynamic(lbl, dynf, inner) => {
                let index = self.get_and_increment_index();
                let t_dynf = self.elaborate_dynamic_format(dynf);
                let new_dyn_scope = TypedDynScope::Binding(TypedDynBinding::new(
                    dyn_scope,
                    lbl,
                    Rc::new(t_dynf.clone()),
                ));
                let t_inner = self.elaborate_format(inner, &new_dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::Dynamic(gt, lbl.clone(), t_dynf, Box::new(t_inner))
            }
            Format::Apply(lbl) => {
                let index = self.get_and_increment_index();
                let t_dynf = dyn_scope
                    .get_typed_dynf_by_name(lbl)
                    .unwrap_or_else(|| panic!("missing dynformat {lbl}"));
                let gt = self.get_gt_from_index(index);
                TypedFormat::Apply(gt, lbl.clone(), t_dynf)
            }
            Format::LetFormat(f0, name, f) => {
                // FIXME - adhoc types introduced in LetFormat are not properly path-named
                let index = self.get_and_increment_index();
                self.codegen
                    .name_gen
                    .ctxt
                    .push_atom(NameAtom::Bind(name.clone()));
                let t_f0 = self.elaborate_format(f0, dyn_scope);
                self.codegen.name_gen.ctxt.escape();
                let t_f = self.elaborate_format(f, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::LetFormat(gt, Box::new(t_f0), name.clone(), Box::new(t_f))
            }
            Format::MonadSeq(f0, f) => {
                // FIXME - adhoc types introduced in MonadSeq are not properly path-named
                let index = self.get_and_increment_index();
                self.codegen
                    .name_gen
                    .ctxt
                    .push_atom(NameAtom::Derived(Derivation::Preimage));
                let t_f0 = self.elaborate_format(f0, dyn_scope);
                self.codegen.name_gen.ctxt.escape();
                let t_f = self.elaborate_format(f, dyn_scope);
                let gt = self.get_gt_from_index(index);
                TypedFormat::MonadSeq(gt, Box::new(t_f0), Box::new(t_f))
            }
            Format::Hint(style_hint, inner) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyn_scope);
                let gt = self.get_gt_from_index(index);
                match style_hint {
                    StyleHint::Record { .. } => {
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
                    }
                }
                TypedFormat::Hint(gt, style_hint.clone(), Box::new(t_inner))
            }
            Format::LiftedOption(opt_f) => {
                let index = self.get_and_increment_index();
                let inner = match opt_f {
                    None => {
                        // increment to account for the 'ghost' index of the free type parameter
                        self.increment_index();
                        None
                    }
                    Some(inner_f) => Some(Box::new(self.elaborate_format(inner_f, dyn_scope))),
                };
                let gt = self.get_gt_from_index(index);
                TypedFormat::LiftedOption(gt, inner)
            }
        }
    }

    fn get_gt_from_index(&mut self, index: usize) -> GenType {
        let var = UVar::new(index);
        let Some(vt) = self.tc.reify(var.into()) else {
            unreachable!("unable to reify {var}")
        };
        self.codegen.lift_type(&vt)
    }

    fn elaborate_expr(&mut self, expr: &Expr) -> GTExpr {
        let index = self.get_and_increment_index();
        match expr {
            Expr::Var(lbl) => {
                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let gt = self.get_gt_from_index(index);
                self.codegen.name_gen.ctxt.escape();

                TypedExpr::Var(gt, lbl.clone())
            }
            Expr::Bool(b) => TypedExpr::Bool(*b),
            Expr::U8(n) => TypedExpr::U8(*n),
            Expr::U16(n) => TypedExpr::U16(*n),
            Expr::U32(n) => TypedExpr::U32(*n),
            Expr::U64(n) => TypedExpr::U64(*n),
            Expr::Tuple(elts) => {
                let mut t_elts = Vec::with_capacity(elts.len());
                self.codegen
                    .name_gen
                    .ctxt
                    .push_atom(NameAtom::Positional(0));
                for elt in elts {
                    let t_elt = self.elaborate_expr(elt);
                    t_elts.push(t_elt);
                    self.codegen.name_gen.ctxt.increment_index();
                }
                let gt = self.get_gt_from_index(index);
                self.codegen.name_gen.ctxt.escape();
                TypedExpr::Tuple(gt, t_elts)
            }
            Expr::TupleProj(e, ix) => {
                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_e = self.elaborate_expr(e);
                self.codegen.name_gen.ctxt.escape();

                // NOTE - by definition, the projected element has a known type, and we need to avoid two types with the same path
                let gt = self.get_gt_from_index(index);
                TypedExpr::TupleProj(gt, Box::new(t_e), *ix)
            }
            Expr::SeqIx(e, ix) => {
                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_e = self.elaborate_expr(e);
                self.codegen.name_gen.ctxt.escape();

                let t_ix = self.elaborate_expr(ix);
                let gt = self.get_gt_from_index(index);

                TypedExpr::SeqIx(gt, Box::new(t_e), Box::new(t_ix))
            }
            Expr::Record(flds) => {
                let mut t_flds = Vec::with_capacity(flds.len());
                for (lbl, fld) in flds {
                    self.codegen
                        .name_gen
                        .ctxt
                        .push_atom(NameAtom::RecordField(lbl.clone()));
                    let t_fld = self.elaborate_expr(fld);
                    self.codegen.name_gen.ctxt.escape();
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
                TypedExpr::Record(gt, t_flds)
            }
            Expr::Seq(elts) => {
                let mut t_elts = Vec::with_capacity(elts.len());
                self.increment_index();
                for elt in elts {
                    let t_elt = self.elaborate_expr(elt);
                    t_elts.push(t_elt);
                }
                let gt = self.get_gt_from_index(index);
                TypedExpr::Seq(gt, t_elts)
            }

            Expr::RecordProj(e, fld) => {
                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_e = self.elaborate_expr(e);
                self.codegen.name_gen.ctxt.escape();

                // NOTE - by definition, the field has its own type-name, so don't allow it to capture the local path
                let gt = self.get_gt_from_index(index);
                TypedExpr::RecordProj(gt, Box::new(t_e), fld.clone())
            }
            Expr::Match(head, branches) => {
                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_head = self.elaborate_expr(head);
                self.codegen.name_gen.ctxt.escape();

                let mut t_branches = Vec::with_capacity(branches.len());
                for (pat, rhs) in branches {
                    let t_pat = self.elaborate_pattern(pat);
                    let t_rhs = self.elaborate_expr(rhs);
                    t_branches.push((t_pat, t_rhs));
                }
                let gt = self.get_gt_from_index(index);
                TypedExpr::Match(gt, Box::new(t_head), t_branches)
            }
            Expr::Lambda(..) => unreachable!(
                "Cannot elaborate Expr::Lambda in neutral (i.e. not lambda-aware) context"
            ),
            Expr::Variant(lbl, inner) => {
                self.codegen
                    .name_gen
                    .ctxt
                    .push_atom(NameAtom::Variant(lbl.clone()));
                let t_inner = self.elaborate_expr(inner);
                self.codegen.name_gen.ctxt.escape();

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
                TypedExpr::Variant(gt, lbl.clone(), Box::new(t_inner))
            }
            Expr::IntRel(rel, x, y) => {
                let t_x = self.elaborate_expr(x);
                let t_y = self.elaborate_expr(y);
                let gt = self.get_gt_from_index(index);
                TypedExpr::IntRel(gt, *rel, Box::new(t_x), Box::new(t_y))
            }
            Expr::Arith(op, x, y) => {
                let t_x = self.elaborate_expr(x);
                let t_y = self.elaborate_expr(y);
                let gt = self.get_gt_from_index(index);
                TypedExpr::Arith(gt, *op, Box::new(t_x), Box::new(t_y))
            }
            Expr::Unary(op, inner) => {
                let t_inner = self.elaborate_expr(inner);
                let gt = self.get_gt_from_index(index);
                TypedExpr::Unary(gt, *op, Box::new(t_inner))
            }
            Expr::AsU8(inner) => {
                let t_inner = self.elaborate_expr(inner);
                TypedExpr::AsU8(Box::new(t_inner))
            }
            Expr::AsU16(inner) => {
                let t_inner = self.elaborate_expr(inner);
                TypedExpr::AsU16(Box::new(t_inner))
            }
            Expr::AsU32(inner) => {
                let t_inner = self.elaborate_expr(inner);
                TypedExpr::AsU32(Box::new(t_inner))
            }
            Expr::AsU64(inner) => {
                let t_inner = self.elaborate_expr(inner);
                TypedExpr::AsU64(Box::new(t_inner))
            }
            Expr::AsChar(inner) => {
                let t_inner = self.elaborate_expr(inner);
                TypedExpr::AsChar(Box::new(t_inner))
            }
            Expr::U16Be(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                TypedExpr::U16Be(Box::new(t_bytes))
            }
            Expr::U16Le(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                TypedExpr::U16Le(Box::new(t_bytes))
            }
            Expr::U32Be(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                TypedExpr::U32Be(Box::new(t_bytes))
            }
            Expr::U32Le(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                TypedExpr::U32Le(Box::new(t_bytes))
            }
            Expr::U64Be(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                TypedExpr::U64Be(Box::new(t_bytes))
            }
            Expr::U64Le(bytes) => {
                let t_bytes = self.elaborate_expr(bytes);
                TypedExpr::U64Le(Box::new(t_bytes))
            }
            Expr::SeqLength(seq) => {
                let t_seq = self.elaborate_expr(seq);
                // NOTE - for element type of sequence
                self.increment_index();
                TypedExpr::SeqLength(Box::new(t_seq))
            }
            Expr::SubSeq(seq, start, length) => {
                let t_seq = self.elaborate_expr(seq);
                let t_start = self.elaborate_expr(start);
                let t_length = self.elaborate_expr(length);
                // NOTE - for element type of sequence
                self.increment_index();
                let gt = self.get_gt_from_index(index);
                TypedExpr::SubSeq(gt, Box::new(t_seq), Box::new(t_start), Box::new(t_length))
            }
            Expr::SubSeqInflate(seq, start, length) => {
                let t_seq = self.elaborate_expr(seq);
                let t_start = self.elaborate_expr(start);
                let t_length = self.elaborate_expr(length);
                // NOTE - for element type of sequence
                self.increment_index();
                let gt = self.get_gt_from_index(index);
                TypedExpr::SubSeqInflate(gt, Box::new(t_seq), Box::new(t_start), Box::new(t_length))
            }
            Expr::FlatMap(lambda, seq) => {
                let t_lambda = self.elaborate_expr_lambda(lambda);

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_seq = self.elaborate_expr(seq);
                self.codegen.name_gen.ctxt.escape();

                self.increment_index();

                let gt = self.get_gt_from_index(index);
                TypedExpr::FlatMap(gt, Box::new(t_lambda), Box::new(t_seq))
            }
            Expr::FlatMapAccum(lambda, acc, _acc_vt, seq) => {
                let t_lambda = self.elaborate_expr_lambda(lambda);

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_acc = self.elaborate_expr(acc);
                self.codegen.name_gen.ctxt.escape();

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_seq = self.elaborate_expr(seq);
                self.codegen.name_gen.ctxt.escape();

                {
                    // account for two extra variables we generate in current TC implementation
                    self.increment_index();
                    self.increment_index();
                }

                let gt = self.get_gt_from_index(index);
                TypedExpr::FlatMapAccum(
                    gt,
                    Box::new(t_lambda),
                    Box::new(t_acc),
                    _acc_vt.clone(),
                    Box::new(t_seq),
                )
            }
            Expr::LeftFold(lambda, acc, _acc_vt, seq) => {
                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_lambda = self.elaborate_expr_lambda(lambda);
                self.codegen.name_gen.ctxt.escape();

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_acc = self.elaborate_expr(acc);
                self.codegen.name_gen.ctxt.escape();

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_seq = self.elaborate_expr(seq);
                self.codegen.name_gen.ctxt.escape();

                self.increment_index();
                let gt = self.get_gt_from_index(index);

                TypedExpr::LeftFold(
                    gt,
                    Box::new(t_lambda),
                    Box::new(t_acc),
                    _acc_vt.clone(),
                    Box::new(t_seq),
                )
            }
            Expr::FindByKey(_is_sorted, f_get_key, query_key, seq) => {
                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_f_get_key = self.elaborate_expr_lambda(f_get_key);
                self.codegen.name_gen.ctxt.escape();

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_query_key = self.elaborate_expr(query_key);
                self.codegen.name_gen.ctxt.escape();

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_seq = self.elaborate_expr(seq);
                self.codegen.name_gen.ctxt.escape();

                self.increment_index();

                let gt = self.get_gt_from_index(index);

                TypedExpr::FindByKey(
                    gt,
                    *_is_sorted,
                    Box::new(t_f_get_key),
                    Box::new(t_query_key),
                    Box::new(t_seq),
                )
            }
            Expr::FlatMapList(lambda, _ret_type, seq) => {
                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_lambda = self.elaborate_expr_lambda(lambda);
                self.codegen.name_gen.ctxt.escape();

                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let t_seq = self.elaborate_expr(seq);
                self.codegen.name_gen.ctxt.escape();

                {
                    // account for two extra variables we generate in current TC implementation
                    self.increment_index();
                    self.increment_index();
                }

                let gt = self.get_gt_from_index(index);

                TypedExpr::FlatMapList(gt, Box::new(t_lambda), _ret_type.clone(), Box::new(t_seq))
            }
            Expr::Dup(count, x) => {
                let t_count = self.elaborate_expr(count);
                let t_x = self.elaborate_expr(x);
                let gt = self.get_gt_from_index(index);
                TypedExpr::Dup(gt, Box::new(t_count), Box::new(t_x))
            }
            Expr::EnumFromTo(from, to) => {
                let t_from = self.elaborate_expr(from);
                let t_to = self.elaborate_expr(to);
                let gt = self.get_gt_from_index(index);
                TypedExpr::EnumFromTo(gt, Box::new(t_from), Box::new(t_to))
            }
            Expr::LiftOption(opt) => {
                let t_expr = if let Some(expr) = opt {
                    Some(Box::new(self.elaborate_expr(expr)))
                } else {
                    self.increment_index();
                    None
                };
                let gt = self.get_gt_from_index(index);
                TypedExpr::LiftOption(gt, t_expr)
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

                // NOTE - alternate path must already exist independently for lambda head-binding
                self.codegen.name_gen.ctxt.push_atom(NameAtom::DeadEnd);
                let gt_head = self.get_gt_from_index(head_index);
                self.codegen.name_gen.ctxt.escape();

                let gt_body = self.get_gt_from_index(body_index);
                TypedExpr::Lambda((gt_head, gt_body), head.clone(), Box::new(t_body))
            }
            _ => unreachable!("elaborate_expr_lambda: unexpected non-lambda {expr:?}"),
        }
    }
}

type GTFormat = TypedFormat<GenType>;
type GTExpr = TypedExpr<GenType>;
type GTPattern = TypedPattern<GenType>;

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
    use crate::helper::compute;
    use crate::{typecheck::Ctxt, TypeHint};

    fn population_check(module: &FormatModule, f: &Format, label: Option<&'static str>) {
        let mut tc = TypeChecker::new();
        let _fv = tc.infer_var_format(f, Ctxt::new(module, &UScope::Empty));
        let tc_pop = tc.size();

        // println!("{tc:?}");

        let cg = CodeGen::new();
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
            tv_pop,
            // dec_f,
            // serde_json::ser::to_string(&re_f).unwrap(),
            // serde_json::ser::to_string(&f).unwrap()
        );
    }

    fn produce_string_gencode(module: &FormatModule, f: &Format) -> String {
        generate_code(module, f).to_fragment().to_string()
    }

    fn run_headcount(fs: &[(&'static str, Format)]) {
        let mut module = FormatModule::new();
        for (name, f) in fs.iter() {
            module.define_format(*name, f.clone());
            population_check(&module, f, None);
        }
    }

    #[test]
    fn test_headcount_simple() {
        let formats = vec![
            ("test.fail", Format::Fail),
            ("test.eoi", Format::EndOfInput),
            ("test.align64", Format::Align(64)),
            ("test.any_byte", Format::Byte(ByteSet::full())),
        ];
        run_headcount(&formats);
    }

    #[test]
    fn test_headcount_record_simple() {
        let f = Format::record(vec![
            ("any_byte", Format::Byte(ByteSet::full())),
            ("align64", Format::Align(64)),
            ("eoi", Format::EndOfInput),
        ]);

        run_headcount(&[("record_simple", f)]);
    }

    #[test]
    fn test_headcount_adt_simple() {
        let f = Format::Union(vec![
            Format::Variant("some".into(), Box::new(Format::Byte(ByteSet::full()))),
            Format::Variant("none".into(), Box::new(Format::EMPTY)),
        ]);

        run_headcount(&[("adt_simple", f)]);
    }

    #[test]
    fn test_headcount_item_var() {
        let sub_f = Format::Byte(ByteSet::full());
        let mut module = FormatModule::new();
        let sub_ref = module.define_format("test.any_byte", sub_f);
        let f = sub_ref.call();
        module.define_format("test.call_any_byte", f.clone());
        population_check(&module, &f, None);
    }

    #[test]
    fn test_headcount_compute_simple() {
        let x = Format::Byte(ByteSet::full());
        let fx = compute(Expr::Var("x".into()));
        let gx = compute(Expr::Arith(
            Arith::Add,
            Box::new(Expr::Var("x".into())),
            Box::new(Expr::Var("x".into())),
        ));

        let f = Format::record(vec![("x", x), ("fx", fx), ("gx", gx)]);
        run_headcount(&[("test.compute_simple", f)]);
    }

    #[test]
    fn test_headcount_compute_complex() {
        let is_null = Expr::Lambda(
            "x".into(),
            Box::new(Expr::IntRel(
                IntRel::Eq,
                Box::new(Expr::U8(0)),
                Box::new(Expr::Var("x".into())),
            )),
        );
        let ix_dup = Expr::Lambda(
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

        let xs =
            Format::RepeatUntilLast(Box::new(is_null), Box::new(Format::Byte(ByteSet::full())));
        let fxs = compute(Expr::FlatMapAccum(
            Box::new(ix_dup),
            Box::new(Expr::U32(1)),
            TypeHint::from(ValueType::Base(BaseType::U32)),
            Box::new(Expr::Var("xs".into())),
        ));

        let f = Format::record(vec![("xs", xs), ("fxs", fxs)]);
        run_headcount(&[("test.compute_complex", f)]);
    }

    #[test]
    fn test_codegen_output() {
        let f = Format::Compute(Box::new(Expr::Unary(
            UnaryOp::IntPred,
            Box::new(Expr::U32(43)),
        )));
        let mut module = FormatModule::new();
        module.define_format("test.output", f.clone());
        let output = produce_string_gencode(&module, &f);
        println!("{}", output);
    }

    #[test]
    fn test_lambda_sanity() {
        const TU16: RustType = RustType::Atom(AtomType::Prim(PrimType::U16));
        const TU8: GenType = GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::U8)));
        const GTBOOL: GenType = GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::Bool)));
        let lambda = {
            let names = ["totlen", "seq"];
            let body = {
                let x = TypedExpr::Var(GenType::Inline(TU16), "totlen".into());
                let y = TypedExpr::AsU16(Box::new(TypedExpr::Var(
                    TU8,
                    Label::Borrowed("point_count"),
                )));
                TypedExpr::IntRel(GTBOOL, IntRel::Gte, Box::new(x), Box::new(y))
            };
            const HEAD_VAR: &str = "tuple_var";
            {
                let types = &[TU16, RustType::vec_of(TU16)];
                let head_type = GenType::from(RustType::AnonTuple(types.to_vec()));
                let body = {
                    let head = TypedExpr::Var(head_type.clone(), Label::Borrowed(HEAD_VAR));
                    let branches = [(
                        TypedPattern::Tuple(
                            head_type.clone(),
                            Iterator::zip(names.into_iter(), types.into_iter())
                                .map(|(pat, typ)| {
                                    TypedPattern::Binding(
                                        GenType::Inline(typ.clone()),
                                        Label::Borrowed(pat),
                                    )
                                })
                                .collect(),
                        ),
                        body,
                    )];
                    TypedExpr::Match(GTBOOL, Box::new(head), Vec::from_iter(branches))
                };

                GenLambda::new(
                    Label::Borrowed(HEAD_VAR),
                    head_type,
                    ClosureKind::PairOwnedBorrow,
                    Rc::new(body),
                )
            }
        };

        assert!(lambda.is_beta_reducible());
        assert!(lambda.needs_ok());

        assert_eq!(
            format!(
                "{}",
                lambda
                    .apply_pair(
                        RustExpr::CloneOf(Box::new(RustExpr::local("acc"))),
                        RustExpr::local("seq"),
                        ExprInfo::default()
                    )
                    .to_fragment()
            ),
            "{\nlet totlen = acc.clone();\nlet seq = &seq;\ntotlen >= (point_count as u16)\n}"
        );
    }
}
