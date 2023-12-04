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
    AtomType, DefParams, FnSig, RustExpr, RustFn, RustLt, RustParams, RustStmt, RustStruct,
    RustVariant,
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
                    _ => CaseLogic::Derived(DerivedLogic::VariantOf(
                        vname.clone(),
                        Box::new(self.translate(inner.as_ref(), type_hint.clone())),
                    )),
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

fn embed_byteset(bs: &ByteSet) -> RustExpr {
    if bs.is_full() {
        RustExpr::scoped(["ByteSet"], "full").call()
    } else if bs.len() == 1 {
        let Some(elt) = bs.min_elem() else {
            unreachable!("len == 1 but no min_elem")
        };
        RustExpr::scoped(["ByteSet"], "singleton").call_with([RustExpr::NumericLit(elt as usize)])
    } else {
        let [q0, q1, q2, q3] = bs.to_bits();
        RustExpr::scoped(["ByteSet"], "from_bits").call_with([RustExpr::ArrayLit(vec![
            RustExpr::NumericLit(q0 as usize),
            RustExpr::NumericLit(q1 as usize),
            RustExpr::NumericLit(q2 as usize),
            RustExpr::NumericLit(q3 as usize),
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
                    RustExpr::NumericLit(*factor),
                ),
                " != ",
                RustExpr::NumericLit(0),
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

            let logic = if bs.is_full() {
                RustExpr::local("b")
            } else {
                let cond = {
                    if bs.len() == 1 {
                        let Some(elt) = bs.min_elem() else {
                            unreachable!("len == 1 but no min_elem")
                        };
                        RustExpr::Operation(RustOp::InfixOp(
                            " == ",
                            Box::new(RustExpr::local("b")),
                            Box::new(RustExpr::NumericLit(elt as usize)),
                        ))
                    } else {
                        embed_byteset(bs).call_method_with("contains", [RustExpr::local("b")])
                    }
                };

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
pub enum CaseLogic {
    Simple(SimpleLogic),
    Derived(DerivedLogic),
    Sequential(SequentialLogic),
    Parallel(ParallelLogic),
    Other(OtherLogic),
}

/// Cases that apply other case-logic in sequence to an incrementally updated input
#[derive(Clone, Debug)]
pub enum SequentialLogic {
    AccumTuple {
        constructor: Option<Label>,
        elements: Vec<CaseLogic>,
    },
    AccumRecord {
        constructor: Label,
        fields: Vec<(Label, CaseLogic)>,
    },
}

/// Catch-all for hard-to-classify cases
#[derive(Clone, Debug)]
pub enum OtherLogic {
    Descend(MatchTree, Vec<CaseLogic>),
}

/// Cases that require processing of multiple cases in parallel (on the same input-state)
#[derive(Clone, Debug)]
pub enum ParallelLogic {
    Alts(Vec<CaseLogic>),
}

/// Cases that require no recursion into other case-logic
#[derive(Clone, Debug)]
pub enum SimpleLogic {
    Fail,
    ExpectEnd,
    Invoke(usize, Vec<(Label, Expr)>),
    SkipToNextMultiple(usize),
    ByteIn(ByteSet),
}

/// Cases that recurse into other case-logic only once
#[derive(Clone, Debug)]
pub enum DerivedLogic {
    VariantOf(Label, Box<CaseLogic>),
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
