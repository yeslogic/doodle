mod name;
pub(crate) mod rust_ast;
pub mod typed_decoder;
pub mod typed_format;

use crate::{
    byte_set::ByteSet,
    typecheck::{TypeChecker, UScope, UVar},
    Arith, BaseType, DynFormat, Expr, Format, FormatModule, IntRel, Label, MatchTree, Pattern,
    ValueType,
};

use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{Hash, Hasher},
    rc::Rc,
};

use name::{NameAtom, WrapperKind};
use rust_ast::*;

use typed_format::{GenType, TypedExpr, TypedFormat, TypedPattern};

use self::{
    typed_decoder::{GTCompiler, GTDecoder, TypedDecoder},
    typed_format::TypedDynFormat,
};

pub(crate) mod ixlabel;
pub(crate) use ixlabel::IxLabel;

fn get_trace(state: &impl std::hash::Hash) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    state.hash(&mut hasher);
    let ret = hasher.finish();
    ret
}

mod path_names {
    use super::{
        name::{NameCtxt, PathLabel},
        rust_ast::RustTypeDef,
    };
    use crate::Label;
    use std::collections::HashMap;

    pub struct NameGen {
        pub(super) ctxt: NameCtxt,
        ctr: usize,
        pub(super) rev_map: HashMap<RustTypeDef, (usize, PathLabel)>,
    }

    impl NameGen {
        pub fn new() -> Self {
            Self {
                ctxt: NameCtxt::new(),
                ctr: 0,
                rev_map: HashMap::new(),
            }
        }

        /// Finds or generates a new name for a RustTypeDef
        ///
        /// Returns `(old, (ix, false))` if the RustTypeDef was already in-scope with name `old` where `ix` is paired with the given
        /// Returns `(new, (ix, true))` otherwise, where `path` is the local path for RustTypeDef at time-of-invocation, and `new` is a novel name
        pub fn get_name(&mut self, def: &RustTypeDef) -> (Label, (usize, bool)) {
            match self.rev_map.get(def) {
                Some((ix, path)) => match self.ctxt.find_name_for(path).ok() {
                    Some(name) => (name.clone(), (*ix, false)),
                    None => unreachable!("no identifier associated with path, but path is in use"),
                },
                None => {
                    let ix = self.ctr;
                    self.ctr += 1;
                    let (path, ret) = {
                        let path = self.ctxt.get_loc().clone();
                        let loc = self.ctxt.produce_name();
                        (path, self.ctxt.find_name_for(&loc).unwrap())
                    };
                    self.rev_map.insert(def.clone(), (ix, path));
                    (ret, (ix, true))
                }
            }
        }
    }
}

mod ix_names {
    #![allow(dead_code)]
    use super::IxLabel;
    use super::RustTypeDef;
    use crate::Label;
    use std::collections::HashMap;
    pub struct NameGen {
        pub(super) ctr: usize,
        rev_map: HashMap<RustTypeDef, IxLabel>,
    }

    impl NameGen {
        pub fn new() -> Self {
            Self {
                ctr: 0,
                rev_map: HashMap::new(),
            }
        }

        /// Finds or generates a new name for a RustTypeDef.
        ///
        /// Returns `(old, (ix, false))` if the RustTypeDef was already in-scope with name `old` and index `ix`
        /// Returns `(new, (ix, true))` otherwise, where `ix` is the new index for the RustTypeDef, and `new` is a novel name
        pub fn get_name(&mut self, def: &RustTypeDef) -> (Label, (usize, bool)) {
            match self.rev_map.get(def) {
                Some(ixlab) => (ixlab.into(), (ixlab.to_usize(), false)),
                None => {
                    let ix = self.ctr;
                    let ixlab = IxLabel::from(ix);
                    self.ctr += 1;
                    self.rev_map.insert(def.clone(), ixlab);
                    (ixlab.into(), (ix, true))
                }
            }
        }
    }
}

use path_names::NameGen;

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
                match &vs[..] {
                    [] => RustType::AnonTuple(Vec::new()).into(),
                    [v] => RustType::AnonTuple(vec![self.lift_type(v).to_rust_type()]).into(),
                    _ => {
                        let mut buf = Vec::with_capacity(vs.len());
                        // FIXME - hard-coded path_names version
                        self.name_gen.ctxt.push_atom(NameAtom::Positional(0));
                        for v in vs.iter() {
                            buf.push(self.lift_type(v).to_rust_type());
                            // FIXME - hardcoded path_names version
                            self.name_gen.ctxt.increment_index();
                        }
                        // FIXME - hard-coded path_names version
                        self.name_gen.ctxt.escape();
                        RustType::AnonTuple(buf).into()
                    }
                }
            }
            ValueType::Seq(t) => {
                // FIXME - hard-coded path_names version
                self.name_gen
                    .ctxt
                    .push_atom(NameAtom::Wrapped(WrapperKind::Sequence));
                let inner = self.lift_type(t.as_ref()).to_rust_type();
                // FIXME - hard-coded path_names version
                self.name_gen.ctxt.escape();
                CompType::Vec(Box::new(inner)).into()
            }
            ValueType::Any => panic!("ValueType::Any"),
            ValueType::Record(fields) => {
                let mut rt_fields = Vec::new();
                for (lab, ty) in fields.iter() {
                    // FIXME - hard-coded path_names version
                    self.name_gen
                        .ctxt
                        .push_atom(NameAtom::RecordField(lab.clone()));
                    let rt_field = self.lift_type(ty);
                    rt_fields.push((lab.clone(), rt_field.to_rust_type()));
                    // FIXME - hard-coded path_names version
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
                    // FIXME - hardcoded path_names version
                    self.name_gen
                        .ctxt
                        .push_atom(NameAtom::Variant(name.clone()));
                    let name = name.clone();
                    let var = match def {
                        ValueType::Empty => RustVariant::Unit(name),
                        ValueType::Tuple(args) => {
                            match &args[..] {
                                [] => RustVariant::Unit(name),
                                [arg] => RustVariant::Tuple(
                                    name,
                                    vec![self.lift_type(arg).to_rust_type()],
                                ),
                                _ => {
                                    let mut v_args = Vec::new();
                                    // FIXME - hardcoded path_names version
                                    self.name_gen.ctxt.push_atom(NameAtom::Positional(0));
                                    for arg in args {
                                        v_args.push(self.lift_type(arg).to_rust_type());
                                        // FIXME - hardcoded path_names version
                                        self.name_gen.ctxt.increment_index();
                                    }
                                    // FIXME - hardcoded path_names version
                                    self.name_gen.ctxt.escape();
                                    RustVariant::Tuple(name, v_args)
                                }
                            }
                        }
                        other => {
                            let inner = self.lift_type(other).to_rust_type();
                            RustVariant::Tuple(name, vec![inner])
                        }
                    };
                    rt_vars.push(var);
                    // FIXME - hardcoded path_names version
                    self.name_gen.ctxt.escape();
                }
                let rtdef = RustTypeDef::Enum(rt_vars);
                let (tname, (ix, is_new)) = self.name_gen.get_name(&rtdef);
                if is_new {
                    self.defined_types.push(rtdef.clone());
                }
                GenType::Def((ix, tname), rtdef)
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
            TypedDecoder::Record(gt, fields) => {
                match gt.try_as_adhoc() {
                    Some((_, lab)) =>  {
                        let constructor = Constructor::Simple(lab.clone());
                        let fields = fields.iter().map(|(l0, d)| (l0.clone(), self.translate(d.get_dec()))).collect();
                        CaseLogic::Sequential(SequentialLogic::AccumRecord { constructor, fields, })
                    }
                    None =>
                        unreachable!(
                            "TypedDecoder::Record expected to have type Def(..) or Inline(Atom(TypeRef(..))), found {gt:?}"
                        ),
                }
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
                        embed_lambda_dft(pred_terminal, ClosureKind::Predicate, true),
                        Box::new(self.translate(single.get_dec()))
                    )
                ),
            TypedDecoder::RepeatUntilSeq(_gt, pred_complete, single) => {
                CaseLogic::Repeat(
                    RepeatLogic::ConditionComplete(
                        embed_lambda_dft(pred_complete, ClosureKind::Predicate, true),
                        Box::new(self.translate(single.get_dec()))
                    )
                )
            }
            TypedDecoder::Map(_gt, inner, f) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Derived(
                    DerivedLogic::MapOf(
                        embed_lambda_dft(f, ClosureKind::Transform, true),
                        Box::new(cl_inner)
                    )
                )
            }
            TypedDecoder::Where(_gt, inner, f) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Derived(
                    DerivedLogic::Where(
                        embed_lambda_dft(f, ClosureKind::Predicate, true),
                        Box::new(cl_inner)
                    )
                )
            }
            TypedDecoder::Compute(_t, expr) =>
                CaseLogic::Simple(SimpleLogic::Eval(embed_expr(expr, ExprInfo::EmbedCloned))),
            TypedDecoder::Let(_t, name, expr, inner) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Derived(
                    DerivedLogic::Let(
                        name.clone(),
                        embed_expr(expr, ExprInfo::EmbedCloned),
                        Box::new(cl_inner)
                    )
                )
            }
            TypedDecoder::Match(_t, scrutinee, cases) => {
                let scrutinized = embed_expr(scrutinee, ExprInfo::Natural);
                let head = match scrutinee.get_type().unwrap().as_ref() {
                    GenType::Inline(RustType::Atom(AtomType::Comp(CompType::Vec(..)))) =>
                        scrutinized.call_method("as_slice"),
                    _ => scrutinized,
                };
                let mut cl_cases = Vec::new();
                for (pat, dec) in cases.iter() {
                    cl_cases.push((
                        MatchCaseLHS::Pattern(embed_pattern_t(pat)),
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
                        DynamicLogic::Huffman(name.clone(), lengths.clone(), opt_values.clone()),
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
                let re_width = embed_expr(width, ExprInfo::Natural);
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Engine(EngineLogic::Slice(re_width, Box::new(cl_inner)))
            }
            TypedDecoder::Bits(_t, inner) => {
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Engine(EngineLogic::Bits(Box::new(cl_inner)))
            }
            TypedDecoder::WithRelativeOffset(_t, offset, inner) => {
                let re_offset = embed_expr(offset, ExprInfo::Natural);
                let cl_inner = self.translate(inner.get_dec());
                CaseLogic::Engine(EngineLogic::OffsetPeek(re_offset, Box::new(cl_inner)))
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
        TypedPattern::U8(n) => {
            RustPattern::PrimLiteral(RustPrimLit::Numeric(RustNumLit::Usize(*n as usize)))
        }
        TypedPattern::U16(n) => {
            RustPattern::PrimLiteral(RustPrimLit::Numeric(RustNumLit::Usize(*n as usize)))
        }
        TypedPattern::U32(n) => {
            RustPattern::PrimLiteral(RustPrimLit::Numeric(RustNumLit::Usize(*n as usize)))
        }
        TypedPattern::U64(n) => {
            RustPattern::PrimLiteral(RustPrimLit::Numeric(RustNumLit::Usize(*n as usize)))
        }
        TypedPattern::Char(c) => RustPattern::PrimLiteral(RustPrimLit::Char(*c)),
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ExprInfo {
    #[default]
    Natural,
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
                RustEntity::Local(tname.clone()),
                fields
                    .iter()
                    .map(|(name, val)| (
                        name.clone(),
                        Some(Box::new(embed_expr(val, ExprInfo::Natural))),
                    ))
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
                                RustVariant::Unit(_vname) => {
                                    // FIXME - this leads to some '();' statements we might want to elide
                                    RustExpr::BlockScope(
                                        // REVIEW - we only need EmbedCloned if there are any potential reuse-after-move patterns within the `_ : ()` preamble...
                                        vec![RustStmt::Expr(embed_expr_dft(inner))],
                                        Box::new(RustExpr::Entity(constr_ent))
                                    )
                                }
                                RustVariant::Tuple(_vname, _elts) => {
                                    // FIXME - not sure how to avoid 1 x N (unary-over-tuple) if inner becomes RustExpr::Tuple...
                                    RustExpr::Entity(constr_ent).call_with([
                                        embed_expr(inner, ExprInfo::Natural),
                                    ])
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
                ) => scrutinized.call_method("as_ref"),
                _ => scrutinized,
            };
            let ck = refutability_check(
                &*scrutinee.get_type().expect("unexpected lambda in match-scrutinee position"),
                cases
            );

            let rust_cases = cases
                .iter()
                .map(|(pat, rhs)| {
                    (
                        MatchCaseLHS::Pattern(embed_pattern_t(pat)),
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
            // FIXME - field and index projections should be optimized around whole-object clone avoidance, when possible
            embed_expr(expr_tup, ExprInfo::EmbedCloned).nth(*ix)
        }
        TypedExpr::RecordProj(_, expr_rec, fld) => {
            // FIXME - field and index projections should be optimized around whole-object clone avoidance, when possible
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
        TypedExpr::Arith(_, arith, lhs, rhs) => {
            // NOTE - because arith only deals with Copy types, we oughtn't need any embedded clones
            let x = embed_expr_dft(lhs);
            let y = embed_expr_dft(rhs);
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
            // NOTE - because IntRel only deals with Copy types, we oughtn't need any embedded clones
            let x = embed_expr_dft(lhs);
            let y = embed_expr_dft(rhs);
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
                    Box::new(embed_expr_dft(seq).call_method("len")),
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
                Operator::Add,
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
                Operator::Add,
                RustExpr::Operation(
                    RustOp::AsCast(Box::new(embed_expr_dft(len)), PrimType::Usize.into())
                )
            );

            let range = RustExpr::RangeExclusive(Box::new(RustExpr::local("ix")), Box::new(end_expr));

            RustExpr::BlockScope(vec![bind_ix], Box::new(RustExpr::local("slice_ext").call_with(vec![RustExpr::Borrow(Box::new(embed_expr(seq, ExprInfo::Natural))), range]).call_method("to_vec")))
        }
        TypedExpr::FlatMap(_, f, seq) =>
            RustExpr::local("try_flat_map_vec")
                .call_with([
                    embed_expr(seq, ExprInfo::Natural).call_method("iter").call_method("cloned"),
                    embed_lambda(f, ClosureKind::Transform, true, ExprInfo::EmbedCloned),
                ])
                .wrap_try(),
        TypedExpr::FlatMapAccum(_, f, acc_init, _acc_type, seq) =>
            RustExpr::local("try_fold_map_curried")
                .call_with([
                    embed_expr(seq, ExprInfo::Natural).call_method("iter").call_method("cloned"),
                    embed_expr(acc_init, ExprInfo::EmbedCloned),
                    embed_lambda(f, ClosureKind::Transform, true, ExprInfo::EmbedCloned),
                ])
                .wrap_try(),
        TypedExpr::FlatMapList(_, f, _ret_type, seq) =>
            RustExpr::local("try_flat_map_append_vec")
                .call_with([
                    embed_expr(seq, ExprInfo::Natural).call_method("iter").call_method("cloned"),
                    embed_lambda_dft(f, ClosureKind::PairBorrowOwned, true),
                ])
                .wrap_try(),
        TypedExpr::Dup(_, n, expr) => {
            // NOTE - the dup count should be simple, but the duplicated expression must be move-safe
            RustExpr::local("dup32").call_with([
                embed_expr(n, ExprInfo::Natural),
                embed_expr(expr, ExprInfo::EmbedCloned),
            ])
        }
        TypedExpr::Var(_, vname) => {
            // REVIEW - lexical scopes, shadowing, and variable-name sanitization may not be quite right in the current implementation
            let loc = RustExpr::local(vname.clone());
            match info {
                ExprInfo::EmbedCloned => loc.call_method("clone"),
                ExprInfo::Natural => loc,
            }
        }
        TypedExpr::Bool(b) => RustExpr::PrimitiveLit(RustPrimLit::Boolean(*b)),
        TypedExpr::U8(n) => RustExpr::u8lit(*n),
        TypedExpr::U16(n) => RustExpr::u16lit(*n),
        TypedExpr::U32(n) => RustExpr::u32lit(*n),
        TypedExpr::U64(n) => RustExpr::u64lit(*n),
        TypedExpr::Lambda(_, _, _) =>
            unreachable!(
                "TypedExpr::Lambda unsupported as first-class embed (requires embed_lambda with proper ClosureKind argument)"
            ),
    }
}

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

fn refutability_check<A: std::fmt::Debug>(
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
                                CompType::Result(_, _) =>
                                    unreachable!("unexpected result in pattern head-type"),
                                CompType::Borrow(_, _, t) => {
                                    refutability_check(&GenType::Inline((&**t).clone()), cases)
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
                    // NOTE - attempts to check full-variant coverage using subtyped partial unions leads to unforeseen badness; we can only check for every possible value being covered for every possible variant
                    let mut variant_coverage: HashMap<Label, Refutability> = HashMap::from_iter(
                        vars.iter().map(|x| (x.get_label().clone(), Refutability::Refutable))
                    );
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ClosureKind {
    Predicate,
    Transform,
    PairBorrowOwned,
}

fn embed_lambda(expr: &GTExpr, kind: ClosureKind, needs_ok: bool, info: ExprInfo) -> RustExpr {
    match expr {
        TypedExpr::Lambda((head_t, _), head, body) => match kind {
            ClosureKind::Predicate => {
                let expansion = embed_expr(body, info);
                RustExpr::Closure(RustClosure::new_predicate(
                    head.clone(),
                    Some(head_t.clone().to_rust_type()),
                    if needs_ok {
                        RustExpr::scoped(["PResult"], "Ok").call_with([expansion])
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
                        RustExpr::scoped(["PResult"], "Ok").call_with([expansion])
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
                        RustExpr::scoped(["PResult"], "Ok").call_with([expansion])
                    } else {
                        expansion
                    },
                ))
            }
        },
        _other => unreachable!("embed_lambda_t expects a lambda, found {_other:?}"),
    }
}

/// Transcribes a lambda-kinded `GTExpr` into a RustExpr value.
///
/// When `kind` is `ClosureKind::Predicate`, the resulting RustExpr will be a closure that operates on a reference to its associated argument-type
/// When `kind` is `ClosureKind::Transform`, the resulting RustExpr will be a closure that operates on an owned value of its associated argument-type
///
/// The `needs_ok` argument controls whether the overall body of the closure expression will be wrapped in `Ok` or not, which depends on whether
/// there are any short-circuiting code-paths within the embedded lambda body. If `true`, an `Ok(...)` will be produced. Otherwise, the body will be
/// transcribed as-is.
fn embed_lambda_dft(expr: &GTExpr, kind: ClosureKind, needs_ok: bool) -> RustExpr {
    embed_lambda(expr, kind, needs_ok, ExprInfo::Natural)
}

type RustBlock = (Vec<RustStmt>, Option<RustExpr>);

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
            type AstElem = RustBlock;

            /// Produces an RustExpr-valued AST for the given CaseLogic instance.
            ///
            /// The ExprT should have the bare type of the value being parsed (i.e. not Option-wrapped),
            /// but it is implicitly assumed to be contained in a block whose ultimate return value
            /// is `Option<_>`, allowing `return None` and `?` expressions to be used anyway.
            ///
            /// Local bindings and control flow are allowed, as long as an explicit return
            /// or a concrete, consistently-typed return value are used
            fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
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

impl SimpleLogic<GTExpr> {
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            SimpleLogic::Fail => (
                vec![RustStmt::Return(
                    ReturnKind::Keyword,
                    RustExpr::err(RustExpr::scoped(["ParseError"], "FailToken")),
                )],
                None,
            ),
            SimpleLogic::ExpectEnd => (
                Vec::new(),
                Some(
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method("finish")
                        .wrap_try(),
                ),
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
                            .chain(
                                args.iter()
                                    .map(|(_lab, x)| embed_expr(x, ExprInfo::EmbedCloned)),
                            )
                            .collect()
                    }
                };
                let call = RustExpr::local(fname).call_with(call_args);
                (Vec::new(), Some(call.wrap_try()))
            }
            SimpleLogic::CallDynamic(dynf_name) => {
                let call = RustExpr::local(dynf_name.clone())
                    .call_with([RustExpr::local(ctxt.input_varname.clone())]);
                (Vec::new(), Some(call.wrap_try()))
            }
            SimpleLogic::SkipToNextMultiple(n) => (
                Vec::new(),
                Some(
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method_with("skip_align", [RustExpr::num_lit(*n)])
                        .wrap_try(),
                ),
            ),
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
                        RustExpr::err(
                            RustExpr::scoped(["ParseError"], "ExcludedBranch")
                                .call_with([RustExpr::u64lit(get_trace(bs))]),
                        ),
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
    match o_expr {
        Some(RustExpr::Tuple(t)) if t.is_empty() => stmts,
        None => stmts,
        Some(expr) => {
            stmts.push(RustStmt::Return(ReturnKind::Implicit, expr));
            stmts
        }
    }
}

/// Applies a lambda-abstraction to a `RustBlock` for internal logic to simultaneously
/// allow for local-only short-circuiting behavior of `?` and `return Err(...)`.
fn abstracted_try_block(block: RustBlock) -> RustExpr {
    let (stmts, ret) = block;
    match ret {
        Some(ret) if stmts.is_empty() => RustExpr::Closure(RustClosure::thunk_expr(
            RustExpr::scoped(["PResult"], "Ok").call_with([ret]),
        )),
        Some(ret) => RustExpr::Closure(RustClosure::thunk_expr(
            RustExpr::scoped(["PResult"], "Ok")
                .call_with([RustExpr::BlockScope(stmts, Box::new(ret))]),
        )),
        None => RustExpr::Closure(RustClosure::thunk_body(stmts)),
    }
}

// follows the same rules as CaseLogic::to_ast as far as the expression type of the generated code
fn embed_matchtree(tree: &MatchTree, ctxt: ProdCtxt<'_>) -> RustBlock {
    fn expand_matchtree(tree: &MatchTree, ctxt: ProdCtxt<'_>) -> RustBlock {
        if tree.branches.is_empty() {
            if let Some(ix) = tree.accept {
                return (Vec::new(), Some(RustExpr::num_lit(ix)));
            } else {
                let err_val = RustExpr::scoped(["ParseError"], "ExcludedBranch")
                    .call_with([RustExpr::u64lit(get_trace(&(tree, "empty-non-accepting")))]);
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
                // this always accepts but needs to read a byte
                let ignore_byte = RustStmt::Expr(
                    RustExpr::local(ctxt.input_varname.clone())
                        .call_method("read_byte")
                        .wrap_try(),
                );
                let (stmts, opt_ret) = expand_matchtree(branch, ctxt);
                let all_stmts =
                    Iterator::chain(std::iter::once(ignore_byte), stmts.into_iter()).collect();
                return (all_stmts, opt_ret);
            } else {
                let b_true: Vec<RustStmt> = implicate_return(expand_matchtree(branch, ctxt));
                let b_false = {
                    if let Some(ix) = tree.accept {
                        vec![RustStmt::Return(
                            ReturnKind::Implicit,
                            RustExpr::num_lit(ix),
                        )]
                    } else {
                        let err_val =
                            RustExpr::scoped(["ParseError"], "ExcludedBranch").call_with([
                                RustExpr::u64lit(get_trace(&(tree, "failed-descent-condition"))),
                            ]);
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
                        RustPrimLit::Numeric(RustNumLit::U8(b)),
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
        let value = RustExpr::err(
            RustExpr::scoped(["ParseError"], "ExcludedBranch")
                .call_with([RustExpr::u64lit(get_trace(&(tree, "catchall-nomatch")))]),
        );
        let match_block = RustControl::Match(
            RustExpr::local("b"),
            RustMatchBody::Refutable(cases, RustCatchAll::ReturnErrorValue { value }),
        );
        (vec![bind], Some(RustExpr::Control(Box::new(match_block))))
    }

    let open_peek = RustStmt::Expr(
        RustExpr::local(ctxt.input_varname.clone()).call_method("open_peek_context"),
    );

    // this is a stub for alternate parsing models to replace the `Parser` argument in the context of the expansion
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
}

#[derive(Clone, Debug)]
enum EngineLogic<ExprT> {
    Slice(RustExpr, Box<CaseLogic<ExprT>>),
    Peek(Box<CaseLogic<ExprT>>),
    Bits(Box<CaseLogic<ExprT>>),
    PeekNot(Box<CaseLogic<ExprT>>),
    OffsetPeek(RustExpr, Box<CaseLogic<ExprT>>),
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
                    RustStmt::assign(
                        Label::from("sz"),
                        RustExpr::Operation(RustOp::AsCast(
                            Box::new(sz.clone()),
                            RustType::verbatim("usize", None),
                        )),
                    ),
                    // // FIXME - remove or gate this
                    // RustStmt::Expr(
                    //     RustExpr::local("eprintln!").call_with([
                    //         RustExpr::str_lit("Opening slice at offset {} with length {}"),
                    //         RustExpr::local(ctxt.input_varname.clone())
                    //             .call_method("get_current_offset"),
                    //         RustExpr::local("sz"),
                    //     ]),
                    // ),
                    RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method_with("start_slice", [RustExpr::local("sz")])
                            .wrap_try(),
                    ),
                    RustStmt::assign(
                        "ret",
                        abstracted_try_block(cl_inner.to_ast(ctxt))
                            .call()
                            .wrap_try(),
                    ),
                    // // FIXME - remove or gate this
                    // RustStmt::Expr(
                    //     RustExpr::local("eprintln!").call_with([
                    //         RustExpr::str_lit("Closing latest slice at offset {}, skipping to end"),
                    //         RustExpr::local(ctxt.input_varname.clone())
                    //             .call_method("get_current_offset"),
                    //     ]),
                    // ),
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

            EngineLogic::OffsetPeek(offs, cl_inner) => (
                vec![
                    RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method("open_peek_context"),
                    ),
                    RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method_with("advance_by", [offs.clone()])
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
                            .call_method("close_peek_context")
                            .wrap_try(),
                    ),
                ],
                Some(RustExpr::local("ret")),
            ),

            EngineLogic::PeekNot(cl_inner) => (
                vec![
                    RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method("open_peek_not_context"),
                    ),
                    RustStmt::assign("_res", abstracted_try_block(cl_inner.to_ast(ctxt)).call()),
                    RustStmt::Control(RustControl::If(
                        RustExpr::local("_res").call_method("is_err"),
                        vec![RustStmt::Expr(
                            RustExpr::local(ctxt.input_varname.clone())
                                .call_method("close_peek_not_context")
                                .wrap_try(),
                        )],
                        Some(vec![RustStmt::Return(
                            ReturnKind::Keyword,
                            RustExpr::err(RustExpr::scoped(["ParseError"], "NegatedSuccess")),
                        )]),
                    )),
                ],
                None,
            ),

            EngineLogic::Bits(cl_inner) => (
                vec![
                    RustStmt::Expr(
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method("enter_bits_mode")
                            .wrap_try(),
                    ),
                    RustStmt::assign(
                        "ret",
                        abstracted_try_block(cl_inner.to_ast(ctxt))
                            .call()
                            .wrap_try(),
                    ),
                    RustStmt::assign(
                        "_bits_read", // FIXME: promote to non-hardcoded identifier
                        RustExpr::local(ctxt.input_varname.clone())
                            .call_method("escape_bits_mode")
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
    /// Evaluates a matchtree and continues if it is matched
    Repeat0ContinueOnMatch(MatchTree, Box<CaseLogic<ExprT>>),
    /// evaluates a matchtree and breaks if it is matched
    Repeat1BreakOnMatch(MatchTree, Box<CaseLogic<ExprT>>),
    /// repeats a specific number of times
    ExactCount(RustExpr, Box<CaseLogic<ExprT>>),
    /// Repeats between N and M times
    BetweenCounts(MatchTree, RustExpr, RustExpr, Box<CaseLogic<ExprT>>),
    /// Repetition stops after a predicate for 'terminal element' is satisfied
    ConditionTerminal(RustExpr, Box<CaseLogic<ExprT>>),
    /// Repetition stops after a predicate for 'complete sequence' is satisfied (post-append)
    ConditionComplete(RustExpr, Box<CaseLogic<ExprT>>),
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
            RepeatLogic::Repeat0ContinueOnMatch(continue_tree, elt) => {
                let mut stmts = Vec::new();

                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(RustStmt::Let(
                    Mut::Mutable,
                    Label::from("accum"),
                    None,
                    RustExpr::scoped(["Vec"], "new").call(),
                ));
                let ctrl = {
                    let tree_index_expr: RustExpr = invoke_matchtree(continue_tree, ctxt);
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
            RepeatLogic::Repeat1BreakOnMatch(break_tree, elt) => {
                let mut stmts = Vec::new();

                let elt_expr = elt.to_ast(ctxt).into();

                stmts.push(RustStmt::assign_mut(
                    "accum",
                    RustExpr::scoped(["Vec"], "new").call(),
                ));
                let ctrl = {
                    let tree_index_expr: RustExpr = invoke_matchtree(break_tree, ctxt);
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
                            Operator::Gt,
                            RustExpr::num_lit(0usize),
                        ),
                        vec![bind_ix, RustStmt::Control(escape_clause)],
                    ))
                };
                stmts.push(ctrl);
                (stmts, Some(RustExpr::local("accum")))
            }
            RepeatLogic::BetweenCounts(btree, expr_min, expr_max, elt) => {
                let mut stmts = Vec::new();

                let elt_expr = elt.to_ast(ctxt).into();
                stmts.push(RustStmt::assign_mut(
                    "accum",
                    RustExpr::scoped(["Vec"], "new").call(),
                ));
                let ctrl = {
                    let tree_index_expr: RustExpr = invoke_matchtree(btree, ctxt);
                    let bind_ix = RustStmt::assign("matching_ix", tree_index_expr);
                    let cond = {
                        let tree_cond = RustExpr::infix(
                            RustExpr::local("matching_ix"),
                            Operator::Eq,
                            RustExpr::num_lit(0usize),
                        );
                        let min_cond = RustExpr::infix(
                            RustExpr::local("accum").call_method("len"),
                            Operator::Gte,
                            RustExpr::Operation(RustOp::AsCast(
                                Box::new(expr_min.clone()),
                                RustType::from(PrimType::Usize),
                            )),
                        );
                        let max_cond = RustExpr::infix(
                            RustExpr::local("accum").call_method("len"),
                            Operator::Eq,
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
                        .call_with([RustExpr::Borrow(Box::new(RustExpr::local("elem")))])
                        .wrap_try();
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
                        .call_with([RustExpr::Borrow(Box::new(RustExpr::local("accum")))])
                        .wrap_try();
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
    ExprMatch(
        RustExpr,
        Vec<(MatchCaseLHS, CaseLogic<ExprT>)>,
        Refutability,
    ),
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
                        MatchCaseLHS::Pattern(RustPattern::PrimLiteral(RustPrimLit::Numeric(
                            RustNumLit::Usize(ix),
                        ))),
                        rhs,
                    ));
                }
                let bind = RustStmt::assign("tree_index", invoke_matchtree(tree, ctxt));
                let fallthrough = RustExpr::err(
                    RustExpr::scoped(["ParseError"], "ExcludedBranch")
                        .call_with([RustExpr::u64lit(get_trace(&(tree, "fallthrough")))]),
                );
                let ret = RustExpr::Control(Box::new(RustControl::Match(
                    RustExpr::local("tree_index"),
                    RustMatchBody::Refutable(
                        branches,
                        RustCatchAll::ReturnErrorValue { value: fallthrough },
                    ),
                )));
                (vec![bind], Some(ret))
            }
            OtherLogic::ExprMatch(expr, cases, ck) => {
                let mut branches = Vec::new();
                for (lhs, logic) in cases.iter() {
                    let (mut rhs, o_val) = logic.to_ast(ctxt);
                    if let Some(val) = o_val {
                        rhs.push(RustStmt::Return(ReturnKind::Implicit, val));
                    }
                    branches.push((lhs.clone(), rhs));
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
                let ret = RustExpr::Control(Box::new(RustControl::Match(expr.clone(), match_body)));
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
                                    .call_method_with("next_alt", [RustExpr::TRUE])
                                    .wrap_try(),
                            ),
                            3.. => RustStmt::Expr(
                                RustExpr::local(ctxt.input_varname.clone())
                                    .call_method_with("next_alt", [RustExpr::FALSE])
                                    .wrap_try(),
                            ),
                        };
                        let thunk = abstracted_try_block(branch_cl.to_ast(ctxt).into());
                        RustStmt::Expr(RustExpr::BlockScope(
                            [RustStmt::assign_mut("f_tmp", thunk)].to_vec(),
                            Box::new(RustExpr::Control(Box::new(RustControl::Match(
                                RustExpr::local("f_tmp").call(),
                                RustMatchBody::Irrefutable(vec![
                                    (
                                        MatchCaseLHS::Pattern(RustPattern::Variant(
                                            Constructor::Simple(Label::from("Ok")),
                                            Box::new(RustPattern::CatchAll(Some(Label::from(
                                                "inner",
                                            )))),
                                        )),
                                        [RustStmt::Return(
                                            ReturnKind::Keyword,
                                            RustExpr::scoped(["PResult"], "Ok")
                                                .call_with([RustExpr::local("inner")]),
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
                                ]),
                            )))),
                        ))
                    }),
                )
                .collect();
                (stmts, None)
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
    Where(RustExpr, Box<CaseLogic<TypedExpr<GenType>>>),
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
                match RustStmt::assign_and_forget(RustExpr::from(inner.to_ast(ctxt))) {
                    Some(inner) => (
                        vec![inner],
                        Some(RustExpr::local(Label::from(constr.clone()))),
                    ),
                    None => (vec![], Some(RustExpr::local(Label::from(constr.clone())))),
                }
            }
            DerivedLogic::MapOf(f, inner) => {
                let assign_inner = RustStmt::assign("inner", RustExpr::from(inner.to_ast(ctxt)));
                (
                    vec![assign_inner],
                    Some(f.clone().call_with([RustExpr::local("inner")]).wrap_try()),
                )
            }
            DerivedLogic::Where(f, inner) => {
                let assign_inner = RustStmt::assign("inner", RustExpr::from(inner.to_ast(ctxt)));
                let ctrl = {
                    let cond_valid = f.clone().call_with([RustExpr::local("inner")]).wrap_try();
                    let b_valid = vec![RustStmt::Return(
                        ReturnKind::Implicit,
                        RustExpr::local("inner"),
                    )];
                    let b_invalid = vec![RustStmt::Return(
                        ReturnKind::Keyword,
                        RustExpr::err(RustExpr::scoped(["ParseError"], "FalsifiedWhere")),
                    )];
                    RustExpr::Control(Box::new(RustControl::If(
                        cond_valid,
                        b_valid,
                        Some(b_invalid),
                    )))
                };
                (vec![assign_inner], Some(ctrl))
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

pub fn print_generated_code(
    module: &FormatModule,
    top_format: &Format,
    dest: Option<std::path::PathBuf>,
) {
    let mut items = Vec::new();

    let Generator {
        sourcemap,
        elaborator,
    } = Generator::compile(module, top_format);
    let mut tdefs = Vec::from_iter(elaborator.codegen.defined_types.iter().map(|tdef| {
        elaborator
            .codegen
            .name_gen
            .rev_map
            .get_key_value(tdef)
            .unwrap()
    }));
    tdefs.sort_by_key(|(_, (ix, _))| ix);

    for (tdef, (_ix, path)) in tdefs.into_iter() {
        let name = elaborator
            .codegen
            .name_gen
            .ctxt
            .find_name_for(&path)
            .expect("no name found");
        let it = RustItem::pub_decl(RustDecl::type_def(name, tdef.clone()));
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
    for attr_string in ["non_camel_case_types", "non_snake_case", "dead_code"].into_iter() {
        content.add_module_attr(ModuleAttr::Allow(AllowAttr::from(Label::from(attr_string))));
    }
    content.add_submodule(RustSubmodule::new("codegen_tests"));
    content.add_submodule(RustSubmodule::new_pub("api_helper"));

    fn write_to(mut f: impl std::io::Write, content: impl ToFragment) -> std::io::Result<()> {
        write!(f, "{}", content.to_fragment())
    }

    match dest {
        None => write_to(std::io::stdout().lock(), content).expect("failed to write"),
        Some(path) => {
            if !path.exists()
                || (path.is_file()
                    && path
                        .file_name()
                        .is_some_and(|s| s.to_string_lossy().contains("codegen.rs")))
            {
                let f = std::fs::File::create(path).unwrap_or_else(|err| panic!("error: {err}"));
                write_to(f, content).expect("failed to write");
            } else {
                panic!(
                    "will not overwrite directory or protected file: {}",
                    path.to_string_lossy()
                );
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DecoderFn<ExprT> {
    ixlabel: IxLabel,
    logic: CaseLogic<ExprT>,
    extra_args: Option<Vec<(Label, GenType)>>,
    ret_type: RustType,
}

impl<ExprT> ToAst for DecoderFn<ExprT>
where
    CaseLogic<ExprT>: ToAst<AstElem = RustBlock>,
    ExprT: std::fmt::Debug,
{
    type AstElem = RustFn;

    fn to_ast(&self, _ctxt: ProdCtxt<'_>) -> RustFn {
        let name = Label::from(format!("Decoder{}", self.ixlabel.to_usize()));
        let params = {
            let mut tmp = DefParams::new();
            tmp.push_lifetime("'input");
            tmp
        };
        let sig = {
            let args = {
                let arg0 = {
                    let name = "_input".into();
                    let ty = {
                        let mut params = RustParams::<RustLt, RustType>::new();
                        params.push_lifetime(RustLt::Parametric("'input".into()));
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
                        args.iter()
                            .map(|(lab, gt)| (lab.clone(), gt.to_rust_type())),
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
        let (stmts, ret) = self.logic.to_ast(ctxt);
        let body = if let Some(ret) = ret {
            Iterator::chain(
                stmts.into_iter(),
                std::iter::once(RustStmt::Return(
                    ReturnKind::Implicit,
                    RustExpr::scoped(["PResult"], "Ok").call_with([ret]),
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
                let cl = elab.codegen.translate(dec);
                DecoderFn {
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
    t_formats: HashMap<usize, Rc<GTFormat>>,
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

    pub fn new(module: &'a FormatModule, tc: TypeChecker, codegen: CodeGen) -> Self {
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
        for branch in branches.iter() {
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
                // FIXME - hieronym hardcode
                self.codegen
                    .name_gen
                    .ctxt
                    .push_atom(NameAtom::Explicit(Label::from(
                        self.module.get_name(*level).to_string(),
                    )));
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
                let (gt, t_elts) = if !elts.is_empty() {
                    let mut t_elts = Vec::with_capacity(elts.len());
                    for t in elts {
                        let t_elt = self.elaborate_format(t, dyns);
                        t_elts.push(t_elt);
                    }
                    let ret = self.get_gt_from_index(index);
                    (ret, t_elts)
                } else {
                    (self.get_gt_from_index(index), Vec::new())
                };
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
            Format::RepeatBetween(min_expr, max_expr, inner) => {
                let index = self.get_and_increment_index();
                let t_min_expr = self.elaborate_expr(min_expr);
                let t_max_expr = self.elaborate_expr(max_expr);
                let t_inner = self.elaborate_format(inner, dyns);
                let gt = self.get_gt_from_index(index);
                GTFormat::RepeatBetween(gt, t_min_expr, t_max_expr, Box::new(t_inner))
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
            Format::Where(inner, lambda) => {
                let index = self.get_and_increment_index();
                let t_inner = self.elaborate_format(inner, dyns);
                let t_lambda = self.elaborate_expr_lambda(lambda);
                let gt = self.get_gt_from_index(index);
                GTFormat::Where(gt, Box::new(t_inner), t_lambda)
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
            Expr::SubSeqInflate(_seq, _start, _length) => {
                let t_seq = self.elaborate_expr(_seq);
                let t_start = self.elaborate_expr(_start);
                let t_length = self.elaborate_expr(_length);
                // NOTE - for element type of sequence
                self.increment_index();
                let gt = self.get_gt_from_index(index);
                TypedExpr::SubSeqInflate(gt, Box::new(t_seq), Box::new(t_start), Box::new(t_length))
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
            Expr::FlatMapList(_lambda, _ret_type, _seq) => {
                let t_lambda = self.elaborate_expr_lambda(_lambda);
                let t_seq = self.elaborate_expr(_seq);

                {
                    // account for two extra variables we generate in current TC implementation
                    self.increment_index();
                    self.increment_index();
                }

                let gt = self.get_gt_from_index(index);

                GTExpr::FlatMapList(gt, Box::new(t_lambda), _ret_type.clone(), Box::new(t_seq))
            }
            Expr::Dup(count, x) => {
                let count_t = self.elaborate_expr(count);
                let x_t = self.elaborate_expr(x);
                let gt = self.get_gt_from_index(index);
                GTExpr::Dup(gt, Box::new(count_t), Box::new(x_t))
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
            Format::Variant("some".into(), Box::new(Format::Byte(ByteSet::full()))),
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
