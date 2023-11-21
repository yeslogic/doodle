use crate::{Format, FormatModule, TypeScope, ValueType};
use std::borrow::Cow;
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

pub enum TypeDef {
    Union(Vec<(Cow<'static, str>, TypeRef)>),
    Record(Vec<(Cow<'static, str>, TypeRef)>),
}

pub struct Codegen {
    typedefs: Vec<TypeDef>,
    record_map: HashMap<Vec<(Cow<'static, str>, TypeRef)>, usize>,
    union_map: HashMap<Vec<(Cow<'static, str>, TypeRef)>, usize>,
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
        let record_map = HashMap::new();
        let union_map = HashMap::new();
        Codegen {
            typedefs,
            record_map,
            union_map,
        }
    }

    pub fn make_typedefs(&mut self, module: &FormatModule, format: &Format) -> TypeRef {
        let t = module.infer_format_type(&TypeScope::new(), format).unwrap();
        self.typeref_from_value_type(&t)
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

pub fn print_program(module: &FormatModule, format: &Format) {
    let mut codegen = Codegen::new();
    let _t = codegen.make_typedefs(module, format);
    for (i, t) in codegen.typedefs.iter().enumerate() {
        match t {
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
