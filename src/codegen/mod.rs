mod rust_ast;

use crate::byte_set::ByteSet;
use crate::codegen::rust_ast::{
    LocalType, Mut, RustControl, RustDecl, RustImport, RustImportItems, RustItem, RustOp,
    RustProgram,
};
use crate::decoder::{Decoder, Program};
use crate::{Expr, Label, MatchTree, ValueType};
use std::borrow::Cow;
use std::collections::HashMap;

use rust_ast::{CompType, PrimType, RustType, RustTypeDef, ToFragment};

use self::rust_ast::{
    AtomType, DefParams, FnSig, RustExpr, RustFn, RustLt, RustParams, RustPattern, RustStmt,
    RustStruct, RustVariant,
};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
/// Simple type for ad-hoc names using a counter value
struct IxLabel(usize);

impl IxLabel {
    fn to_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for IxLabel {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<IxLabel> for Label {
    fn from(value: IxLabel) -> Self {
        Cow::Owned(format!("Type{}", value.0))
    }
}

impl From<&IxLabel> for Label {
    fn from(value: &IxLabel) -> Self {
        Cow::Owned(format!("Type{}", value.0))
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
    decoder_types: Vec<RustType>,
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

    /// Converts a `ValueType` to a `RustType`, potentially creating new ad-hoc names
    /// for any records or unions encountered, and registering any new ad-hoc type definitions
    /// in `self`.
    fn lift_type(&mut self, vt: &ValueType) -> RustType {
        match vt {
            ValueType::Empty => RustType::UNIT,
            ValueType::Bool => PrimType::Bool.into(),
            ValueType::U8 => PrimType::U8.into(),
            ValueType::U16 => PrimType::U16.into(),
            ValueType::U32 => PrimType::U32.into(),
            ValueType::Char => PrimType::Char.into(),
            ValueType::Tuple(vs) => {
                let mut buf = Vec::with_capacity(vs.len());
                for v in vs.iter() {
                    buf.push(self.lift_type(v))
                }
                RustType::AnonTuple(buf)
            }
            ValueType::Seq(t) => {
                let inner = self.lift_type(t.as_ref());
                CompType::Vec(Box::new(inner)).into()
            }
            ValueType::Any => panic!("ValueType::Any"),
            ValueType::Record(fields) => {
                let mut rt_fields = Vec::new();
                for (lab, ty) in fields.iter() {
                    let rt_field = self.lift_type(ty);
                    rt_fields.push((lab.clone(), rt_field));
                }
                let rtdef = RustTypeDef::Struct(RustStruct::Record(rt_fields));
                let (tname, (ix, is_new)) = self.namegen.get_name(&rtdef);
                if is_new {
                    self.defined_types.push(rtdef);
                }
                RustType::defined(ix, tname)
            }
            ValueType::Union(vars) => {
                let mut rt_vars = Vec::new();
                for (vname, vdef) in vars.iter() {
                    let rt_var = match vdef {
                        ValueType::Empty => RustVariant::Unit(vname.clone()),
                        ValueType::Tuple(args) => {
                            let mut rt_args = Vec::new();
                            for arg in args.iter() {
                                rt_args.push(self.lift_type(arg));
                            }
                            RustVariant::Tuple(vname.clone(), rt_args)
                        }
                        ValueType::Record(fields) => {
                            let mut rt_fields = Vec::new();
                            for (f_lab, f_ty) in fields.iter() {
                                rt_fields.push((f_lab.clone(), self.lift_type(f_ty)));
                            }
                            RustVariant::Record(vname.clone(), rt_fields)
                        }
                        other => {
                            let inner = self.lift_type(other);
                            RustVariant::Tuple(vname.clone(), vec![inner])
                        }
                    };
                    rt_vars.push(rt_var);
                }
                let rtdef = RustTypeDef::Enum(rt_vars);
                let (tname, (ix, is_new)) = self.namegen.get_name(&rtdef);
                if is_new {
                    self.defined_types.push(rtdef);
                }
                RustType::defined(ix, tname)
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

    #[allow(dead_code)]
    fn translate(&self, decoder: &Decoder, type_hint: Option<&RustType>) -> CaseLogic {
        match decoder {
            Decoder::Call(ix, args) => CaseLogic::Simple(SimpleLogic::Invoke(*ix, args.clone())),
            Decoder::Fail => CaseLogic::Simple(SimpleLogic::Fail),
            Decoder::EndOfInput => CaseLogic::Simple(SimpleLogic::ExpectEnd),
            Decoder::Align(n) => CaseLogic::Simple(SimpleLogic::SkipToNextMultiple(*n)),
            Decoder::Byte(bs) => CaseLogic::Simple(SimpleLogic::ByteIn(bs.clone())),
            Decoder::Variant(vname, inner) => {
                // FIXME - not sure quite what logic to employ for unwrapping type-hints on variants
                match type_hint {
                    Some(RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(ix, lab)))) => {
                        let qname = RustExpr::scoped([lab.clone()], vname.clone());
                        let tdef = &self.defined_types[*ix];
                        match tdef {
                            RustTypeDef::Enum(vars) => {
                                let matching = vars
                                    .iter()
                                    .find(|var| var.get_label().as_ref() == vname.as_ref());
                                // REVIEW - should we force an exact match?
                                match matching {
                                    Some(RustVariant::Unit(_)) => {
                                        let _inner = self.translate(inner, Some(&RustType::UNIT));
                                        CaseLogic::Derived(DerivedLogic::VariantOf(qname, Box::new(_inner)))
                                    }
                                    Some(RustVariant::Tuple(_, typs)) => {
                                        // FIXME - is this correct?
                                        let cl_inner = self.translate(inner, Some(&RustType::AnonTuple(typs.clone())));
                                        CaseLogic::Derived(DerivedLogic::VariantOf(qname, Box::new(cl_inner)))
                                    }
                                    Some(RustVariant::Record(_, _fields)) => {
                                        // FIXME - this is much harder to implement as Records cannot be anonymous
                                        todo!("VariantOf @ RustVariant::Record")
                                    }
                                    None => unreachable!("VariantOf called for nonexistent variant `{vname}` of enum-type `{lab}`"),
                                }
                            }
                            RustTypeDef::Struct(_) => {
                                unreachable!("VariantOf not coherent on type defined as struct")
                            }
                        }
                    }
                    _ => unreachable!(
                        "insufficient type information to translate Decoder::VariantOf"
                    ),
                }
            }
            Decoder::Parallel(alts) => CaseLogic::Parallel(ParallelLogic::Alts(
                alts.iter()
                    .map(|alt| self.translate(alt, type_hint.clone()))
                    .collect(),
            )),
            Decoder::Branch(tree, flat) => CaseLogic::Other(OtherLogic::Descend(
                tree.clone(),
                flat.iter()
                    .map(|alt| self.translate(alt, type_hint.clone()))
                    .collect(),
            )),
            Decoder::Tuple(elts) => match type_hint {
                None => CaseLogic::Sequential(SequentialLogic::AccumTuple {
                    constructor: None,
                    elements: elts.iter().map(|elt| self.translate(elt, None)).collect(),
                }),
                Some(RustType::AnonTuple(tys)) => {
                    CaseLogic::Sequential(SequentialLogic::AccumTuple {
                        constructor: None,
                        elements: elts
                            .iter()
                            .zip(tys)
                            .map(|(elt, ty)| self.translate(elt, Some(ty)))
                            .collect(),
                    })
                }
                Some(other) => unreachable!(
                    "Decoder::Tuple expected to have type RustType::AnonTuple(..), found {:?}",
                    other
                ),
            },
            Decoder::Record(flds) => {
                match type_hint {
                    Some(RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(ix, lab)))) => {
                        let ref tdef = self.defined_types[*ix];
                        let tfields = match tdef {
                            RustTypeDef::Struct(RustStruct::Record(tfields)) => tfields,
                            _ => unreachable!("unexpected non-Struct::Record definition for type `{lab}`: {tdef:?}"),
                        };
                        // FIXME - for now we rely on a consistent order of field names between the decoder and type definition
                        let mut assocs = Vec::new();
                        for (i, (l0, d)) in flds.iter().enumerate() {
                            let (l1, t) = &tfields[i];
                            assert_eq!(l0.as_ref(), l1.as_ref(), "Decoder field `{l0}` != RustTypeDef field `{l1}` (at index {i} in {decoder:?} | {tdef:?})");
                            assocs.push((l0.clone(), self.translate(d, Some(t))));
                        }
                        CaseLogic::Sequential(SequentialLogic::AccumRecord { constructor: lab.clone(), fields: assocs })
                    }
                    None => unreachable!("Cannot generate CaseLogic for a Record without a definite type-name"),
                    Some(other) => unreachable!("Decoder::Record expected to have type RustType::Atom(AtomType::TypeRef(..)), found {:?}", other),
                }
            }
            // FIXME - implement CaseLogic variants and translation rules for the remaining cases
            Decoder::While(_, _) => todo!(),
            Decoder::Until(_, _) => todo!(),
            Decoder::RepeatCount(_, _) => todo!(),
            Decoder::RepeatUntilLast(_, _) => todo!(),
            Decoder::RepeatUntilSeq(_, _) => todo!(),
            Decoder::Peek(_) => todo!(),
            Decoder::PeekNot(_) => todo!(),
            Decoder::Slice(_, _) => todo!(),
            Decoder::Bits(_) => todo!(),
            Decoder::WithRelativeOffset(_, _) => todo!(),
            Decoder::Map(_, _) => todo!(),
            Decoder::Compute(_) => todo!(),
            Decoder::Let(_, _, _) => todo!(),
            Decoder::Match(_, _) => todo!(),
            Decoder::Dynamic(_, _, _) => todo!(),
            Decoder::Apply(_) => todo!(),
        }
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
    /// or a concrete, consistently typed return value are used
    #[allow(dead_code)]
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            CaseLogic::Simple(s) => s.to_ast(ctxt),
            CaseLogic::Derived(d) => d.to_ast(ctxt),
            CaseLogic::Sequential(sq) => sq.to_ast(ctxt),
            CaseLogic::Parallel(p) => p.to_ast(ctxt),
            CaseLogic::Other(o) => o.to_ast(ctxt),
        }
    }
}

impl SimpleLogic {
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> RustBlock {
        match self {
            SimpleLogic::Fail => (vec![RustStmt::Return(true, RustExpr::NONE)], None),
            SimpleLogic::ExpectEnd => {
                let call = RustExpr::local(ctxt.input_varname.clone()).call_method("read_byte");
                let cond = call.call_method("is_none");
                let b_true = [RustStmt::Return(false, RustExpr::UNIT)];
                let b_false = [RustStmt::Return(true, RustExpr::NONE)];
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
                        " % ",
                        RustExpr::num_lit(*n),
                    ),
                    " != ",
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
                    let b_true = vec![RustStmt::Return(false, RustExpr::local("b"))];
                    let b_false = vec![RustStmt::Return(true, RustExpr::local("None"))];
                    RustExpr::Control(Box::new(RustControl::If(cond, b_true, Some(b_false))))
                };
                ([b_let].to_vec(), Some(logic))
            }
        }
    }
}

fn decoder_fn(ix: usize, t: &RustType, decoder: &Decoder) -> RustFn {
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
                let ty = RustType::Atom(AtomType::Comp(CompType::Borrow(
                    None,
                    Mut::Mutable,
                    Box::new(RustType::imported("Scope")),
                )));
                (name, ty)
            };
            let arg1 = {
                let name = "input".into();
                let ty = {
                    let mut params = RustParams::<RustLt, RustType>::new();
                    params.push_lifetime(RustLt::Parametric("'input".into()));
                    RustType::Atom(AtomType::Comp(CompType::Borrow(
                        None,
                        Mut::Mutable,
                        Box::new(RustType::verbatim("ParseCtxt", Some(params))),
                    )))
                };
                (name, ty)
            };
            [arg0, arg1].to_vec()
        };
        FnSig::new(args, Some(RustType::option_of(t.clone())))
    };
    let body = decoder_body(decoder, t);
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

fn name_for_decoder(dec: &Decoder) -> &'static str {
    match dec {
        Decoder::Call(_, _) => "Call",
        Decoder::Fail => "Fail",
        Decoder::EndOfInput => "EndOfInput",
        Decoder::Align(_) => "Align",
        Decoder::Byte(_) => "Byte",
        Decoder::Variant(_, _) => "Variant",
        Decoder::Parallel(_) => "Parallel",
        Decoder::Branch(_, _) => "Branch",
        Decoder::Tuple(_) => "Tuple",
        Decoder::Record(_) => "Record",
        Decoder::While(_, _) => "While",
        Decoder::Until(_, _) => "Until",
        Decoder::RepeatCount(_, _) => "RepeatCount",
        Decoder::RepeatUntilLast(_, _) => "RepeatUntilLast",
        Decoder::RepeatUntilSeq(_, _) => "RepeatUntilSeq",
        Decoder::Peek(_) => "Peek",
        Decoder::PeekNot(_) => "PeekNot",
        Decoder::Slice(_, _) => "Slice",
        Decoder::Bits(_) => "Bits",
        Decoder::WithRelativeOffset(_, _) => "WithRelativeOffset",
        Decoder::Map(_, _) => "Map",
        Decoder::Compute(_) => "Compute",
        Decoder::Let(_, _, _) => "Let",
        Decoder::Match(_, _) => "Match",
        Decoder::Dynamic(_, _, _) => "Dynamic",
        Decoder::Apply(_) => "Apply",
    }
}

// FIXME - implement something that actually works
fn invoke_decoder(decoder: &Decoder, input_varname: &Label) -> RustExpr {
    match decoder {
        Decoder::Align(factor) => {
            // FIXME - this currently produces correct but inefficient code
            // it is harder to write, but much more efficient, to cut the buffer at the right place
            let cond = RustExpr::infix(
                RustExpr::infix(
                    RustExpr::local("input").call_method("offset"),
                    " % ",
                    RustExpr::num_lit(*factor),
                ),
                " != ",
                RustExpr::num_lit(0usize),
            );
            let body = {
                let let_tmp = RustStmt::assign(
                    "_",
                    RustExpr::local(input_varname.clone())
                        .call_method("read_byte")
                        .wrap_try(),
                );
                vec![let_tmp]
            };
            RustExpr::BlockScope(
                vec![RustStmt::Control(RustControl::While(cond, body))],
                Box::new(RustExpr::UNIT),
            )
        }
        Decoder::Fail => RustExpr::BlockScope(
            vec![RustStmt::Return(true, RustExpr::NONE)],
            Box::new(RustExpr::UNIT),
        ),
        Decoder::EndOfInput => {
            let call = RustExpr::local(input_varname.clone()).call_method("read_byte");
            let cond = call.call_method("is_none");
            let b_true = [RustStmt::Return(false, RustExpr::UNIT)];
            let b_false = [RustStmt::Return(true, RustExpr::NONE)];
            RustExpr::Control(Box::new(RustControl::If(
                cond,
                b_true.to_vec(),
                Some(b_false.to_vec()),
            )))
        }
        Decoder::Byte(bs) => {
            let call = RustExpr::local(input_varname.clone())
                .call_method("read_byte")
                .wrap_try();
            let b_let = RustStmt::assign("b", call);
            let (cond, always_true) = ByteCriterion::from(bs).as_predicate(RustExpr::local("b"));
            let logic = if always_true {
                RustExpr::local("b")
            } else {
                let b_true = vec![RustStmt::Return(false, RustExpr::local("b"))];
                let b_false = vec![RustStmt::Return(true, RustExpr::local("None"))];
                RustExpr::Control(Box::new(RustControl::If(cond, b_true, Some(b_false))))
            };
            RustExpr::BlockScope([b_let].to_vec(), Box::new(logic))
        }
        // FIXME - not sure what to do with _args ...
        Decoder::Call(ix_dec, _args) => {
            let fname = format!("Decoder{ix_dec}");
            let call = RustExpr::local(fname).call_with([
                RustExpr::local("scope"),
                RustExpr::local(input_varname.clone()),
            ]);
            call.wrap_try()
        }
        Decoder::Tuple(elts) if elts.is_empty() => RustExpr::UNIT,
        // FIXME - there are still cases to split out of here
        other => {
            let let_tmp = RustStmt::assign(
                "tmp",
                RustExpr::str_lit(format!("invoke_decoder @ {}", name_for_decoder(other))),
            );
            RustExpr::BlockScope(
                vec![let_tmp],
                Box::new(
                    RustExpr::local("unimplemented!")
                        .call_with([RustExpr::str_lit("{}"), RustExpr::local("tmp")]),
                ),
            )
        }
    }
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
    Other(OtherLogic),
}

/// Cases that apply other case-logic in sequence to an incrementally updated input
#[derive(Clone, Debug)]
enum SequentialLogic {
    AccumTuple {
        constructor: Option<Label>,
        elements: Vec<CaseLogic>,
    },
    AccumRecord {
        constructor: Label,
        fields: Vec<(Label, CaseLogic)>,
    },
}

impl SequentialLogic {
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> (Vec<RustStmt>, Option<RustExpr>) {
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
                        Some(RustExpr::local(con.clone()).call_with([RustExpr::Tuple(
                            names.into_iter().map(RustExpr::local).collect(),
                        )])),
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
                        constructor.clone(),
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
}
impl OtherLogic {
    fn to_ast(&self, ctxt: ProdCtxt<'_>) -> (Vec<RustStmt>, Option<RustExpr>) {
        match self {
            OtherLogic::Descend(tree, cases) => {
                // FIXME - we don't have a transformation from MatchTree to AST, so this is incomplete
                let mut branches = Vec::new();
                for (ix, case) in cases.iter().enumerate() {
                    let (mut rhs, o_val) = case.to_ast(ctxt);
                    match o_val {
                        Some(val) => {
                            rhs.push(RustStmt::Return(false, val));
                        }
                        None => (),
                    };
                    branches.push((RustPattern::NumLiteral(ix), rhs));
                }
                let ret = RustExpr::Control(Box::new(RustControl::Match(
                    invoke_matchtree(tree, ctxt),
                    branches,
                )));
                (Vec::new(), Some(ret))
            }
        }
    }
}

/// this production should be a RustExpr whose compiled type is usize, and whose
/// runtime value is the index of the successful match relative to the input
fn invoke_matchtree(_tree: &MatchTree, _ctxt: ProdCtxt<'_>) -> RustExpr {
    RustExpr::local("unimplemented!").call_with([RustExpr::str_lit("invoke_matchtree")])
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
}

/// Cases that recurse into other case-logic only once
#[derive(Clone, Debug)]
enum DerivedLogic {
    VariantOf(RustExpr, Box<CaseLogic>),
}

impl DerivedLogic {
    fn to_ast(&self, _ctxt: ProdCtxt<'_>) -> (Vec<RustStmt>, Option<RustExpr>) {
        match self {
            // FIXME - variants cannot be modelled as atomic operations within the current framework
            DerivedLogic::VariantOf(_vname, _logic) => (
                Vec::new(),
                Some(
                    RustExpr::local("unimplemented!")
                        .call_with([RustExpr::str_lit("DerivedLogic::VariantOf.to_ast(..)")]),
                ),
            ),
        }
    }
}

fn decoder_body(decoder: &Decoder, return_type: &RustType) -> Vec<rust_ast::RustStmt> {
    // FIXME - double-check this won't clash with any local assignments in Decoder expansion
    let inp_varname: Label = "input".into();
    let mut body = Vec::new();

    match decoder {
        Decoder::Tuple(elems) => {
            if elems.is_empty() {
                return vec![RustStmt::Return(false, RustExpr::some(RustExpr::UNIT))];
            }

            let mut names: Vec<Label> = Vec::new();

            for (ix, dec) in elems.iter().enumerate() {
                let varname = format!("field{}", ix);
                names.push(varname.clone().into());
                let assign = {
                    let rhs = invoke_decoder(dec, &inp_varname);
                    RustStmt::assign(varname, rhs)
                };
                body.push(assign);
            }

            let ret = RustStmt::Return(
                true,
                RustExpr::some(RustExpr::Tuple(
                    names.into_iter().map(RustExpr::local).collect(),
                )),
            );

            body.push(ret);
        }
        Decoder::Record(fields) => {
            if fields.is_empty() {
                unreachable!("Decoder::Record has no fields, which is not an expected case");
            }
            let constr = match return_type {
                RustType::Atom(AtomType::TypeRef(lt)) => lt.as_name().clone(),
                _ => unreachable!(
                    "decoder_body found Decoder::Record with a non-`Named` return type {:?}",
                    return_type
                ),
            };

            let mut names: Vec<Label> = Vec::new();

            for (fname, dec) in fields.iter() {
                let varname = rust_ast::sanitize_label(fname);
                names.push(varname.clone());
                let assign = {
                    let rhs = invoke_decoder(dec, &inp_varname);
                    RustStmt::assign(varname, rhs)
                };
                body.push(assign);
            }

            let ret = RustStmt::Return(
                true,
                RustExpr::some(RustExpr::Struct(
                    constr,
                    names.into_iter().map(|l| (l, None)).collect(),
                )),
            );

            body.push(ret);
        }
        // FIXME - cover the other non-simple cases before the catch-all
        _ => {
            let ret = RustStmt::Return(true, RustExpr::some(invoke_decoder(decoder, &inp_varname)));
            body.push(ret);
        }
    }

    body
}

pub fn print_program(program: &Program) {
    let mut codegen = Codegen::new();
    let mut items = Vec::new();
    codegen.populate_decoder_types(program);
    for (tdef, ixlab) in codegen.namegen.revmap.iter() {
        let it = RustItem::from_decl(RustDecl::TypeDef(ixlab.into(), tdef.clone()));
        items.push(it);
    }

    for (i, (d, _t)) in program.decoders.iter().enumerate() {
        let t = &codegen.decoder_types[i];
        let f = decoder_fn(i, t, d);
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
