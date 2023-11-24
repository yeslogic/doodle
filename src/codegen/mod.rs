mod rust_ast;

use crate::decoder::{Decoder, Program};
use crate::{Label, ValueType};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TypeRef {
    Var(usize),
    Empty,
    Bool,
    U8,
    U16,
    U32,
    Tuple(Vec<TypeRef>),
    Seq(Box<TypeRef>),
    Char,
}

// NOTE - same rhs but different semantics
type TypeUnion = Vec<(Label, TypeRef)>;
type TypeRecord = Vec<(Label, TypeRef)>;

pub enum TypeDef {
    Union(TypeUnion),
    Record(TypeRecord),
}

type IndexMap<T> = HashMap<T, usize>;

pub struct Codegen {
    typedefs: Vec<TypeDef>,
    typerefs: Vec<TypeRef>,
    record_map: IndexMap<TypeRecord>,
    union_map: IndexMap<TypeUnion>,
}

impl TypeRef {
    #[allow(dead_code)]
    fn to_value_type(&self, typedefs: &[TypeDef]) -> ValueType {
        match self {
            TypeRef::Var(n) => match &typedefs[*n] {
                TypeDef::Union(ts) => ValueType::Union(
                    ts.iter()
                        .map(|(name, t)| (name.clone(), t.to_value_type(typedefs)))
                        .collect(),
                ),
                TypeDef::Record(ts) => ValueType::Record(
                    ts.iter()
                        .map(|(name, t)| (name.clone(), t.to_value_type(typedefs)))
                        .collect(),
                ),
            },
            TypeRef::Empty => ValueType::Empty,
            TypeRef::Bool => ValueType::Bool,
            TypeRef::U8 => ValueType::U8,
            TypeRef::U16 => ValueType::U16,
            TypeRef::U32 => ValueType::U32,
            TypeRef::Char => ValueType::Char,
            TypeRef::Tuple(ts) => {
                ValueType::Tuple(ts.iter().map(|t| t.to_value_type(typedefs)).collect())
            }
            TypeRef::Seq(t) => ValueType::Seq(Box::new(t.to_value_type(typedefs))),
        }
    }
}

impl Codegen {
    pub fn new() -> Self {
        let typedefs = Vec::new();
        let typerefs = Vec::new();
        let record_map = HashMap::new();
        let union_map = HashMap::new();
        Codegen {
            typedefs,
            typerefs,
            record_map,
            union_map,
        }
    }

    pub fn make_typedefs(&mut self, program: &Program) {
        self.typerefs = Vec::with_capacity(program.decoders.len());
        for (_d, t) in &program.decoders {
            let r = self.typeref_from_value_type(t);
            self.typerefs.push(r);
        }
    }

    pub fn add_typedef(&mut self, t: TypeDef) -> TypeRef {
        let n = self.typedefs.len();
        self.typedefs.push(t);
        TypeRef::Var(n)
    }

    fn typeref_from_value_type(&mut self, t: &ValueType) -> TypeRef {
        match t {
            ValueType::Any => panic!("ValueType::Any"),
            ValueType::Empty => TypeRef::Empty,
            ValueType::Bool => TypeRef::Bool,
            ValueType::U8 => TypeRef::U8,
            ValueType::Char => TypeRef::Char,
            ValueType::U16 => TypeRef::U16,
            ValueType::U32 => TypeRef::U32,
            ValueType::Tuple(ts) => {
                TypeRef::Tuple(ts.iter().map(|t| self.typeref_from_value_type(t)).collect())
            }
            ValueType::Record(fields) => {
                let fs: Vec<_> = fields
                    .iter()
                    .map(|(label, t)| (label.clone(), self.typeref_from_value_type(t)))
                    .collect();
                let n = if let Some(n) = self.record_map.get(&fs) {
                    *n
                } else {
                    let t = TypeDef::Record(fs.clone());
                    let n = self.typedefs.len();
                    self.typedefs.push(t);
                    self.record_map.insert(fs, n);
                    n
                };
                TypeRef::Var(n)
            }
            ValueType::Union(branches) => {
                let bs: Vec<_> = branches
                    .iter()
                    .map(|(label, t)| (label.clone(), self.typeref_from_value_type(t)))
                    .collect();
                let n = if let Some(n) = self.union_map.get(&bs) {
                    *n
                } else {
                    let t = TypeDef::Union(bs.clone());
                    let n = self.typedefs.len();
                    self.typedefs.push(t);
                    self.union_map.insert(bs, n);
                    n
                };
                TypeRef::Var(n)
            }
            ValueType::Seq(t) => TypeRef::Seq(Box::new(self.typeref_from_value_type(t))),
        }
    }
}

pub fn print_program(program: &Program) {
    let mut codegen = Codegen::new();
    codegen.make_typedefs(program);
    for (i, t) in codegen.typedefs.iter().enumerate() {
        match t {
            /*
            TypeDef::Equiv(t) => {
                print!("type Type{i} = ");
                print_typeref(t);
                println!(";");
            }
            */
            TypeDef::Union(branches) => {
                println!("enum Type{i} {{");
                for (i, (label, t)) in branches.iter().enumerate() {
                    print!("    Branch{i}(");
                    print_typeref(t);
                    println!("), // {label}");
                }
                println!("}}");
            }
            TypeDef::Record(fields) => {
                println!("struct Type{i} {{");
                for (i, (label, t)) in fields.iter().enumerate() {
                    print!("    field{i}: ");
                    print_typeref(t);
                    println!(", // {label}");
                }
                println!("}}");
            }
        }
        println!();
    }

    for (i, (d, _t)) in program.decoders.iter().enumerate() {
        let t = &codegen.typerefs[i];
        print!("fn Decoder{i}<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<(");
        print_typeref(t);
        println!(", ReadCtxt<'input>)> {{");
        match d {
            Decoder::Call(_n, _args) => {}
            Decoder::Fail => println!("    return None;"),
            /*
            Decoder::EndOfInput => {}
            Decoder::Align(n) => {}
            Decoder::Byte(bs) => {}
            Decoder::Branch(tree, branches) => {}
            Decoder::Tuple(ds) => {}
            Decoder::Record(fields) => {}
            Decoder::While(tree, d) => {}
            Decoder::Until(tree, d) => {}
            Decoder::RepeatCount(expr, d) => {}
            Decoder::RepeatUntilLast(expr, d) => {}
            Decoder::RepeatUntilSeq(expr, d) => {}
            Decoder::Peek(d) => {}
            Decoder::Slice(expr, d) => {}
            Decoder::Bits(d) => {}
            Decoder::WithRelativeOffset(expr, d) => {}
            Decoder::Compute(expr) => {}
            Decoder::Match(expr, branches>) => {}
            Decoder::Dynamic(DynFormat::Huffman(_, _)) => {}
            */
            _ => println!("// FIXME"),
        }
        println!("}}");
        println!();
    }
}

fn print_typeref(t: &TypeRef) {
    match t {
        TypeRef::Var(n) => print!("Type{n}"),
        TypeRef::Empty => print!("Empty"),
        TypeRef::Bool => print!("bool"),
        TypeRef::U8 => print!("u8"),
        TypeRef::U16 => print!("u16"),
        TypeRef::U32 => print!("u32"),
        TypeRef::Char => print!("char"),
        TypeRef::Tuple(ts) => {
            print!("(");
            for t in ts {
                print_typeref(t);
                print!(",");
            }
            print!(")");
        }
        TypeRef::Seq(t) => {
            print!("Vec<");
            print_typeref(t);
            print!(">");
        }
    }
}
