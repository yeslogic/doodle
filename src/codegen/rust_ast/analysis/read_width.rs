use crate::codegen::rust_ast::{
    AtomType, CompType, LocalType, PrimType, RustStruct, RustType, RustTypeDecl, RustTypeDef,
    RustVariant, analysis::SourceContext,
};

use super::ASTContext;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub enum ValueWidth {
    Variable { is_dynamic: bool },
    Fixed(usize),
}

impl ValueWidth {
    pub const ZERO: ValueWidth = ValueWidth::Fixed(0);
    pub const DYN: ValueWidth = ValueWidth::Variable { is_dynamic: true };
    pub const VAR: ValueWidth = ValueWidth::Variable { is_dynamic: false };

    pub fn as_fixed(&self) -> Option<usize> {
        match self {
            ValueWidth::Variable { .. } => None,
            ValueWidth::Fixed(n) => Some(*n),
        }
    }

    /// Downcasts fixed-width to non-dynamic variable-width, for combinatorial use
    /// in additive contexts where Variable is forced.
    pub fn obfuscate(&self) -> Self {
        match self {
            ValueWidth::Fixed(_) => ValueWidth::Variable { is_dynamic: false },
            _ => *self,
        }
    }
}

impl std::ops::Add for ValueWidth {
    type Output = ValueWidth;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ValueWidth::Fixed(a), ValueWidth::Fixed(b)) => ValueWidth::Fixed(a + b),
            (lhs, rhs) => {
                let ValueWidth::Variable { is_dynamic: dyn0 } = lhs.obfuscate() else {
                    unreachable!()
                };
                let ValueWidth::Variable { is_dynamic: dyn1 } = rhs.obfuscate() else {
                    unreachable!()
                };
                ValueWidth::Variable {
                    is_dynamic: dyn0 || dyn1,
                }
            }
        }
    }
}

/// Unification of fixed-width variables that diverges to Variable if unequal
impl std::ops::BitAnd for ValueWidth {
    type Output = ValueWidth;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ValueWidth::Fixed(a), ValueWidth::Fixed(b)) => {
                if a == b {
                    ValueWidth::Fixed(a)
                } else {
                    ValueWidth::Variable { is_dynamic: true }
                }
            }
            (lhs, rhs) => {
                let ValueWidth::Variable { is_dynamic: dyn0 } = lhs.obfuscate() else {
                    unreachable!()
                };
                let ValueWidth::Variable { is_dynamic: dyn1 } = rhs.obfuscate() else {
                    unreachable!()
                };
                ValueWidth::Variable {
                    is_dynamic: dyn0 || dyn1,
                }
            }
        }
    }
}

/// Trait for types that are effectively hierarchical trees whose leaf-nodes are all primitive parses of fixed width that are read directly from buffered data, in order, without peeking, seeking, or other higher-level state-alteration
pub trait ReadWidth: ASTContext {
    fn read_width(&self, context: Self::Context<'_>) -> ValueWidth;
}

impl ReadWidth for RustType {
    fn read_width(&self, context: &SourceContext<'_>) -> ValueWidth {
        match self {
            RustType::Atom(at) => at.read_width(context),
            RustType::AnonTuple(ts) => {
                let mut total = ValueWidth::ZERO;
                for t in ts {
                    total = total + t.read_width(context);
                }
                total
            }
            RustType::ViewObject(..) => {
                // NOTE - even though ViewObject is a zero-width parse, we reject it because it doesn't meet our expected shape
                ValueWidth::Variable { is_dynamic: false }
            }
            RustType::ReadArray(..) => {
                // NOTE - even though ReadArray is a zero-width parse, we reject it because it doesn't meet our expected shape
                ValueWidth::Variable { is_dynamic: false }
            }
            RustType::Verbatim(..) => unreachable!("unexpected Verbatim in structural type"),
        }
    }
}

impl ReadWidth for AtomType {
    fn read_width(&self, context: &SourceContext<'_>) -> ValueWidth {
        match self {
            AtomType::Prim(p) => p.read_width(()),
            AtomType::Comp(c) => c.read_width(context),
            AtomType::TypeRef(lt) => lt.read_width(context),
        }
    }
}

impl ReadWidth for PrimType {
    fn read_width(&self, _: ()) -> ValueWidth {
        match self {
            PrimType::U8 => ValueWidth::Fixed(size_of::<u8>()),
            PrimType::U16 => ValueWidth::Fixed(size_of::<u16>()),
            PrimType::U32 => ValueWidth::Fixed(size_of::<u32>()),
            PrimType::U64 => ValueWidth::Fixed(size_of::<u64>()),
            PrimType::Unit => ValueWidth::ZERO,
            PrimType::Bool => ValueWidth::VAR,
            PrimType::Char => ValueWidth::VAR,
            PrimType::Usize => ValueWidth::VAR,
        }
    }
}

impl<T: ASTContext> ReadWidth for CompType<Box<T>> {
    fn read_width(&self, _: T::Context<'_>) -> ValueWidth {
        match self {
            CompType::PhantomData(..) => ValueWidth::ZERO,
            CompType::Vec(..) => ValueWidth::DYN,
            CompType::RawSlice(_) => ValueWidth::VAR,
            CompType::Option(_) => ValueWidth::VAR,
            CompType::Borrow(..) => ValueWidth::VAR,
            CompType::Result(..) => unreachable!("unexpected Result in structural type"),
        }
    }
}

impl ReadWidth for LocalType {
    fn read_width(&self, context: &SourceContext<'_>) -> ValueWidth {
        match self {
            LocalType::LocalDef(ix, ..) => context.get_width(*ix),
            LocalType::External(_) => unreachable!("unexpected External in structural type"),
        }
    }
}

impl ReadWidth for RustTypeDecl {
    fn read_width(&self, context: &SourceContext<'_>) -> ValueWidth {
        self.def.read_width(context)
    }
}

impl ReadWidth for RustTypeDef {
    fn read_width(&self, context: Self::Context<'_>) -> ValueWidth {
        match self {
            RustTypeDef::Struct(t) => t.read_width(context),
            RustTypeDef::Enum(vars) => match vars.as_slice() {
                [] => unreachable!("empty enum"),
                [a] => a.read_width(context),
                [a, rest @ ..] => {
                    let mut total = a.read_width(context);
                    for b in rest {
                        total = total & b.read_width(context);
                    }
                    total
                }
            },
        }
    }
}

impl ReadWidth for RustVariant {
    fn read_width(&self, context: &SourceContext<'_>) -> ValueWidth {
        match self {
            RustVariant::Unit(..) => ValueWidth::ZERO,
            RustVariant::Tuple(_, xs) => {
                let mut total = ValueWidth::ZERO;
                for x in xs {
                    total = total + x.read_width(context);
                }
                total
            }
        }
    }
}

impl ReadWidth for RustStruct {
    fn read_width(&self, context: &SourceContext<'_>) -> ValueWidth {
        let mut total = ValueWidth::ZERO;
        let RustStruct::Record(fields) = self;
        for (_, f) in fields {
            total = total + f.read_width(context);
        }
        total
    }
}
