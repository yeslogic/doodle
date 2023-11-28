mod rust_ast;

use crate::byte_set::ByteSet;
use crate::codegen::rust_ast::{
    Mut, RustControl, RustDecl, RustImport, RustImportItems, RustItem, RustProgram,
};
use crate::decoder::{Decoder, Program};
use crate::{Label, ValueType};
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

    fn get_name(&mut self, def: &RustTypeDef) -> Label {
        match self.revmap.get(def) {
            Some(ixlab) => ixlab.into(),
            None => {
                let ixlab = IxLabel::from(self.ctr);
                self.ctr += 1;
                self.revmap.insert(def.clone(), ixlab);
                ixlab.into()
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
    fn resolve(&mut self, vt: &ValueType) -> RustType {
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
                    buf.push(self.resolve(v))
                }
                RustType::AnonTuple(buf)
            }
            ValueType::Seq(t) => {
                let inner = self.resolve(t.as_ref());
                CompType::Vec(Box::new(inner)).into()
            }
            ValueType::Any => panic!("ValueType::Any"),
            ValueType::Record(fields) => {
                let mut rt_fields = Vec::new();
                for (lab, ty) in fields.iter() {
                    let rt_field = self.resolve(ty);
                    rt_fields.push((lab.clone(), rt_field));
                }
                let rtdef = RustTypeDef::Struct(RustStruct::Record(rt_fields));
                let old_ctr = self.namegen.ctr;
                let tname = self.namegen.get_name(&rtdef);
                let new_ctr = self.namegen.ctr;
                if new_ctr > old_ctr {
                    self.defined_types.push(rtdef);
                }
                RustType::named(tname)
            }
            ValueType::Union(vars) => {
                let mut rt_vars = Vec::new();
                for (vname, vdef) in vars.iter() {
                    let rt_var = match vdef {
                        ValueType::Empty => RustVariant::Unit(vname.clone()),
                        ValueType::Tuple(args) => {
                            let mut rt_args = Vec::new();
                            for arg in args.iter() {
                                rt_args.push(self.resolve(arg));
                            }
                            RustVariant::Tuple(vname.clone(), rt_args)
                        }
                        ValueType::Record(fields) => {
                            let mut rt_fields = Vec::new();
                            for (f_lab, f_ty) in fields.iter() {
                                rt_fields.push((f_lab.clone(), self.resolve(f_ty)));
                            }
                            RustVariant::Record(vname.clone(), rt_fields)
                        }
                        other => {
                            let inner = self.resolve(other);
                            RustVariant::Tuple(vname.clone(), vec![inner])
                        }
                    };
                    rt_vars.push(rt_var);
                }
                let rtdef = RustTypeDef::Enum(rt_vars);
                let old_ctr = self.namegen.ctr;
                let tname = self.namegen.get_name(&rtdef);
                let new_ctr = self.namegen.ctr;
                if new_ctr > old_ctr {
                    self.defined_types.push(rtdef);
                }
                RustType::named(tname)
            }
        }
    }

    pub fn populate_decoder_types(&mut self, program: &Program) {
        self.decoder_types = Vec::with_capacity(program.decoders.len());
        for (_d, t) in &program.decoders {
            let r = self.resolve(t);
            self.decoder_types.push(r);
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
                    Box::new(RustType::named("Scope")),
                )));
                (name, ty)
            };
            let arg1 = {
                let name = "input".into();
                let ty = {
                    let mut params = RustParams::<RustLt, RustType>::new();
                    params.push_lifetime(RustLt::Parametric("'input".into()));
                    RustType::verbatim("ReadCtxt", Some(params))
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
    let [q0, q1, q2, q3] = bs.to_bits();
    RustExpr::scoped(["ByteSet"], "from_bits").call_with([RustExpr::ArrayLit(vec![
        RustExpr::NumericLit(q0 as usize),
        RustExpr::NumericLit(q1 as usize),
        RustExpr::NumericLit(q2 as usize),
        RustExpr::NumericLit(q3 as usize),
    ])])
}

// FIXME - implement something that actually works
fn invoke_decoder(decoder: &Decoder, input_varname: &Label) -> RustExpr {
    match decoder {
        Decoder::Align(factor) => {
            // FIXME - this currently produces correct but inefficient code
            // it is harder to write, but much more efficient, to cut the buffer at the right place
            let cond = RustExpr::infix(
                RustExpr::infix(
                    RustExpr::local("input").field("offset"),
                    " % ",
                    RustExpr::NumericLit(*factor),
                ),
                " != ",
                RustExpr::NumericLit(0),
            );
            let body = {
                let let_tmp = RustStmt::assign(
                    "tmp",
                    RustExpr::local(input_varname.clone())
                        .call_method("read_byte")
                        .wrap_try(),
                );
                let rebind =
                    RustStmt::Reassign(input_varname.clone(), RustExpr::local("tmp").nth(1));
                vec![let_tmp, rebind]
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
            let bind = RustStmt::assign("tmp", call);
            let cond = RustExpr::local("tmp").call_method("is_none");
            let b_true = [
                RustStmt::Reassign(input_varname.clone(), RustExpr::local("tmp").nth(1)),
                RustStmt::Return(false, RustExpr::UNIT),
            ];
            let b_false = [RustStmt::Return(true, RustExpr::NONE)];
            RustExpr::BlockScope(
                vec![bind],
                Box::new(RustExpr::Control(Box::new(RustControl::If(
                    cond,
                    b_true.to_vec(),
                    Some(b_false.to_vec()),
                )))),
            )
        }
        Decoder::Byte(bs) => {
            // FIXME - we have multiple options to handle this, none of them simple
            let bs_let = RustStmt::assign("bs", embed_byteset(bs));

            let call = RustExpr::local("input").call_method("read_byte").wrap_try();

            let bind = RustStmt::assign("tmp", call);
            let b_let = RustStmt::assign("b", RustExpr::local("tmp").nth(0));

            let logic = {
                let cond =
                    RustExpr::local("bs").call_method_with("contains", [RustExpr::local("b")]);
                let b_true = vec![
                    RustStmt::Reassign(input_varname.clone(), RustExpr::local("tmp").nth(1)),
                    RustStmt::Return(false, RustExpr::local("b")),
                ];
                let b_false = vec![RustStmt::Return(true, RustExpr::local("None"))];
                RustExpr::Control(Box::new(RustControl::If(cond, b_true, Some(b_false))))
            };

            RustExpr::BlockScope([bs_let, bind, b_let].to_vec(), Box::new(logic))
        }
        // FIXME - not sure what to do with _args ...
        Decoder::Call(ix_dec, _args) => {
            let fname = format!("Decoder{ix_dec}");
            let call = RustExpr::local(fname).call_with([
                RustExpr::local("scope"),
                RustExpr::local(input_varname.clone()),
            ]);
            let bind = RustStmt::assign("tmp", call.wrap_try());
            let replace = RustStmt::Reassign(input_varname.clone(), RustExpr::local("tmp").nth(1));
            let ret = RustExpr::local("tmp").nth(0);
            RustExpr::BlockScope(vec![bind, replace], Box::new(ret))
        }
        _ => RustExpr::local("unimplemented!").call_with([RustExpr::str_lit("invoke_decoder")]),
    }
}

fn decoder_body(decoder: &Decoder, return_type: &RustType) -> Vec<rust_ast::RustStmt> {
    // FIXME - double-check this won't clash with any local assignments in Decoder expansion
    let inp_varname: Label = "inp".into();
    let mut body = Vec::new();

    let init = RustStmt::Let(
        Mut::Mutable,
        inp_varname.clone(),
        None,
        RustExpr::local("input"),
    );
    body.push(init);

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
                RustType::Atom(AtomType::Named(tyname)) => tyname.clone(),
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

    print!("{}", content.to_fragment())
}
