mod rust_ast;

use crate::codegen::rust_ast::{RustItem, RustDecl, RustControl, Mut, RustProgram};
use crate::decoder::{Decoder, Program};
use crate::{Label, ValueType};
use std::borrow::Cow;
use std::collections::HashMap;

use rust_ast::{CompType, PrimType, RustType, RustTypeDef, ToFragment};

use self::rust_ast::{RustStruct, RustVariant, RustFn, RustParams, RustLt, FnSig, AtomType, DefParams, RustExpr, RustStmt};

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

impl IxLabel {
    pub const fn to_usize(&self) -> usize {
        self.0
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
                        ValueType::Empty => {
                            RustVariant::Unit(vname.clone())
                        },
                        ValueType::Tuple(args) => {
                            let mut rt_args = Vec::new();
                            for arg in args.iter() {
                                rt_args.push(self.resolve(arg));
                            }
                            RustVariant::Tuple(vname.clone(), rt_args)
                        },
                        ValueType::Record(fields) => {
                            let mut rt_fields = Vec::new();
                            for (f_lab, f_ty) in fields.iter() {
                                rt_fields.push((f_lab.clone(), self.resolve(f_ty)));
                            }
                            RustVariant::Record(vname.clone(), rt_fields)
                        },
                        other => {
                            let inner = self.resolve(other);
                            RustVariant::Tuple(vname.clone(), vec![inner])
                        }
                    };
                    rt_vars.push(rt_var);
                };
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
                let ty = RustType::Atom(AtomType::Comp(CompType::Borrow(None, Mut::Mutable, Box::new(RustType::named("Scope")))));
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
    let body = decoder_body(decoder);
    RustFn::new(
        name,
        Some(params),
        sig,
        body,
    )
}

fn decoder_body(decoder: &Decoder) -> Vec<rust_ast::RustStmt> {
    match decoder {
        Decoder::Align(factor) => {
            let cond = RustExpr::infix(
                RustExpr::infix(
                    RustExpr::local("input").field("offset"),
                    " % ",
                    RustExpr::NumericLit(*factor)
                ),
                " != ",
                RustExpr::NumericLit(0)
            );
            let body = {
                let call = RustExpr::local("input").call_method("read_byte");
                vec![RustStmt::Let(Mut::Immutable, "_".into(), None, call)]
            };
            [RustStmt::Control(RustControl::While(cond, body)), RustStmt::Return(true, RustExpr::local("Some").call_with([RustExpr::UNIT]))].to_vec()
        },
        Decoder::Fail => {
            [RustStmt::Return(true, RustExpr::local("None"))].to_vec()
        }
        // FIXME - implement more gradually
        Decoder::Call(_, _) => todo!(),
        Decoder::EndOfInput => todo!(),
        Decoder::Byte(_) => todo!(),
        Decoder::Variant(_, _) => todo!(),
        Decoder::Parallel(_) => todo!(),
        Decoder::Branch(_, _) => todo!(),
        Decoder::Tuple(_) => todo!(),
        Decoder::Record(_) => todo!(),
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

pub fn print_program(program: &Program) {
    let mut codegen = Codegen::new();
    let mut items = Vec::new();
    codegen.populate_decoder_types(program);
    for (tdef, ixlab) in codegen.namegen.revmap.iter() {
        let it = RustItem::from_decl(
            RustDecl::TypeDef(ixlab.into(), tdef.clone()),
        );
        items.push(it);
    }

    for (i, (d, _t)) in program.decoders.iter().enumerate() {
        let t = &codegen.decoder_types[i];
        let f = decoder_fn(i, t, d);
        items.push(RustItem::from_decl(RustDecl::Function(f)));
    }
    let content = RustProgram::from_iter(items);
    print!("{}", content.to_fragment())
}
