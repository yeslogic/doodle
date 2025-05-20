use std::{borrow::Cow, cell::OnceCell, collections::{BTreeMap, HashSet}, ops::RangeInclusive, rc::Rc};
use doodle::byte_set::ByteSet;
use anyhow::{anyhow, Result as AResult};


pub type Label = Cow<'static, str>;

pub type FormatId = usize;

pub type RecId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FormatRef(FormatId);

impl FormatRef {
    pub const fn get_level(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span<Idx> {
    pub start: Idx,
    pub end: Idx,
}

impl<Idx> Span<Idx> {
    pub const fn new(start: Idx, end: Idx) -> Self {
        Self { start, end }
    }
}

impl<Idx: Copy> From<RangeInclusive<Idx>> for Span<Idx> {
    fn from(value: RangeInclusive<Idx>) -> Self {
        Self { start: *value.start(), end: *value.end() }
    }
}

#[derive(Debug, Clone)]
pub struct FormatDecl {
    format: Format,
    fmt_id: FormatId,
    f_type: Rc<OnceCell<FormatType>>,
    batch: Option<Span<FormatId>>,
}

impl FormatDecl {
    pub fn solve_type(&self, module: &FormatModule) -> AResult<&FormatType> {
        let mut visited = HashSet::new();
        self.solve_type_with(module, &mut visited)
    }

    pub(crate) fn solve_type_with(&self, module: &FormatModule, visited: &mut HashSet<FormatId>) -> AResult<&FormatType> {
        match self.f_type.get() {
            None => {
                visited.insert(self.fmt_id);
                let f_type = self.format.infer_type(visited, module, self.batch)?;
                let Ok(_) = self.f_type.set(f_type) else { unreachable!("synchronous TOCTOU!?") };
                Ok(self.f_type.get().unwrap())
            }
            Some(f_type) => Ok(f_type),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    Char,
}

impl BaseType {
    pub fn is_numeric(&self) -> bool {
        matches!(self, BaseType::U8 | BaseType::U16 | BaseType::U32 | BaseType::U64)
    }
}

#[derive(Debug, Clone)]
pub enum FormatType {
    Any,
    Void,
    Base(BaseType),
    Ref(FormatId),
    Shape(TypeShape),
}

impl FormatType {
    pub const UNIT : FormatType = FormatType::Shape(TypeShape::Tuple(Vec::new()));

    pub fn is_numeric(&self) -> bool {
        match self {
            FormatType::Base(base) => base.is_numeric(),
            _ => false,
        }
    }

    fn unify(&self, other: &FormatType) -> AResult<FormatType> {
        match (self, other) {
            (FormatType::Any, _) => Ok(other.clone()),
            (_, FormatType::Any) => Ok(self.clone()),
            (FormatType::Ref(id0), FormatType::Ref(id1)) => {
                if id0 == id1 {
                    Ok(FormatType::Ref(*id0))
                } else {
                    unimplemented!("cross-ref unification not implemented");
                }
            }
            (FormatType::Void, _) | (_, FormatType::Void) => Ok(FormatType::Void),
            (FormatType::Base(b1), FormatType::Base(b2)) if b1 == b2 => Ok(FormatType::Base(*b1)),
            (FormatType::Shape(s1), FormatType::Shape(s2)) => {
                let s = s1.unify(s2)?;
                Ok(FormatType::Shape(s))
            }
            _ => Err(anyhow!("cannot unify incompatible types: {self:?}, {other:?}")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeShape {
    Tuple(Vec<FormatType>),
    Seq(Box<FormatType>),
    Option(Box<FormatType>),
    Union(BTreeMap<Label, FormatType>),
}

impl TypeShape {
    fn unify(&self, other: &Self) -> AResult<TypeShape> {
        match (self, other) {
            (TypeShape::Tuple(t1), TypeShape::Tuple(t2)) => {
                if t1.len() != t2.len() {
                    return Err(anyhow!("cannot unify tuples of different arity: {t1:?}, {t2:?}"));
                }
                let mut unified = Vec::with_capacity(t1.len());
                for (t1, t2) in t1.iter().zip(t2.iter()) {
                    unified.push(t1.unify(t2)?);
                }
                Ok(TypeShape::Tuple(unified))
            }
            (TypeShape::Seq(t1), TypeShape::Seq(t2)) => Ok(TypeShape::Seq(Box::new(t1.unify(t2)?))),
            (TypeShape::Option(t1), TypeShape::Option(t2)) => Ok(TypeShape::Option(Box::new(t1.unify(t2)?))),
            (TypeShape::Union(bs1), TypeShape::Union(bs2)) => {
                let mut bs = BTreeMap::new();

                let keys1 = bs1.keys().collect::<HashSet<_>>();
                let keys2 = bs2.keys().collect::<HashSet<_>>();

                let all_keys = HashSet::union(&keys1, &keys2).cloned();

                for key in all_keys.into_iter() {
                    match (bs1.get(key), bs2.get(key)) {
                        (Some(t1), Some(t2)) => {
                            let t = t1.unify(t2)?;
                            bs.insert(key.clone(), t);
                        }
                        (Some(t), None) | (None, Some(t)) => {
                            bs.insert(key.clone(), t.clone());
                        }
                        (None, None) => unreachable!("key must appear in at least one operand"),
                    }
                }
                Ok(TypeShape::Union(bs))
            }
            _ => Err(anyhow!("cannot unify shapes: {self:?}, {other:?}")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Format {
    // References to other formats
    ItemVar(FormatId),
    RecVar(RecId),

    // Basic Primitives
    FailWith(Label),
    EndOfInput,
    Byte(ByteSet),
    Compute(Box<Expr>),

    // Union-Based
    Variant(Label, Box<Format>),
    Union(Vec<Format>),

    // Sequential
    Repeat(Box<Format>),
    Seq(Vec<Format>),

    // Higher-Order
    Tuple(Vec<Format>),
    Maybe(Box<Expr>, Box<Format>),
}

impl Format {
    fn infer_type<'ctx>(&'ctx self, visited: &mut HashSet<FormatId>, module: &'ctx FormatModule, batch: Option<Span<FormatId>>) -> AResult<FormatType> {
        match self {
            Format::ItemVar(level) => {
                if visited.contains(level) {
                    Ok(FormatType::Ref(*level))
                } else {
                    let decl = &module.decls[*level];
                    Ok(decl.solve_type_with(module, visited)?.clone())
                }
            }
            Format::RecVar(batch_ix) => {
                match batch {
                    None => Err(anyhow!("Recursion without a batch")),
                    Some(range) => {
                        let level = range.start + batch_ix;
                        if level > range.end {
                            return Err(anyhow!("batch index out of range"))
                        }
                        if visited.contains(&level) {
                            Ok(FormatType::Ref(level))
                        } else {
                            let decl = &module.decls[level];
                            Ok(decl.solve_type_with(module, visited)?.clone())
                        }
                    }
                }
            }
            Format::FailWith(_msg) => Ok(FormatType::Void),
            Format::EndOfInput => Ok(FormatType::UNIT),
            Format::Byte(bs) if bs.is_empty() => Ok(FormatType::Void),
            Format::Byte(_) => Ok(FormatType::Base(BaseType::U8)),
            Format::Compute(expr) => expr.as_ref().infer_type(),
            Format::Variant(label, inner) => {
                let inner_type = inner.infer_type(visited, module, batch)?;
                Ok(FormatType::Shape(TypeShape::Union(BTreeMap::from([(label.clone(), inner_type)]))))
            }
            Format::Union(branches) => {
                let mut t = FormatType::Any;
                for f in branches {
                    t = t.unify(&f.infer_type(visited, module, batch)?)?;
                }
                Ok(t)
            }
            Format::Repeat(inner) => {
                let t = inner.infer_type(visited, module, batch)?;
                Ok(FormatType::Shape(TypeShape::Seq(Box::new(t))))
            }
            Format::Seq(elts) => {
                let mut elem_type = FormatType::Any;
                for elt in elts {
                    elem_type = elem_type.unify(&elt.infer_type(visited, module, batch)?)?;
                }
                Ok(FormatType::Shape(TypeShape::Seq(Box::new(elem_type))))
            }
            Format::Tuple(elts) => {
                let mut types = Vec::with_capacity(elts.len());
                for elt in elts {
                    types.push(elt.infer_type(visited, module, batch)?);
                }
                Ok(FormatType::Shape(TypeShape::Tuple(types)))
            }
            Format::Maybe(expr, format) => match expr.infer_type()? {
                FormatType::Base(BaseType::Bool) => {
                    let t = format.infer_type(visited, module, batch)?;
                    Ok(FormatType::Shape(TypeShape::Option(Box::new(t))))
                }
                other => Err(anyhow!("maybe expression type was inferred to be non-bool: {other:?}")),
            }
        }
    }


}

#[derive(Debug, Clone)]
pub enum Expr {
    // Primitive Values
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Bool(bool),

    // Primitive Value Casts
    AsChar(Box<Expr>),
    AsU8(Box<Expr>),
    AsU16(Box<Expr>),
    AsU32(Box<Expr>),
    AsU64(Box<Expr>),

    // Higher-Order Exprs
    Seq(Vec<Expr>),
    Tuple(Vec<Expr>),
    LiftMaybe(Option<Box<Expr>>),
    Variant(Label, Box<Expr>),

    // Operational
    IntRel(IntRel, Box<Expr>, Box<Expr>),
    Arith(Arith, Box<Expr>, Box<Expr>),
    Unary(Unary, Box<Expr>),
}

impl Expr {
    fn infer_type(&self) -> AResult<FormatType> {
        match self {
            Expr::U8(_) => Ok(FormatType::Base(BaseType::U8)),
            Expr::U16(_) => Ok(FormatType::Base(BaseType::U16)),
            Expr::U32(_) => Ok(FormatType::Base(BaseType::U32)),
            Expr::U64(_) => Ok(FormatType::Base(BaseType::U64)),
            Expr::Bool(_) => Ok(FormatType::Base(BaseType::Bool)),
            Expr::AsChar(expr) => {
                let expr_type = expr.infer_type()?;
                if expr_type.is_numeric() {
                    Ok(FormatType::Base(BaseType::Char))
                } else {
                    Err(anyhow!("invalid char type conversion from {expr_type:?}"))
                }
            }
            Expr::AsU8(expr) => {
                let expr_type = expr.infer_type()?;
                if expr_type.is_numeric() {
                    Ok(FormatType::Base(BaseType::U8))
                } else {
                    Err(anyhow!("invalid u8 type conversion from {expr_type:?}"))
                }
            }
            Expr::AsU16(expr) => {
                let expr_type = expr.infer_type()?;
                if expr_type.is_numeric() {
                    Ok(FormatType::Base(BaseType::U16))
                } else {
                    Err(anyhow!("invalid u16 type conversion from {expr_type:?}"))
                }
            }
            Expr::AsU32(expr) => {
                let expr_type = expr.infer_type()?;
                if expr_type.is_numeric() {
                    Ok(FormatType::Base(BaseType::U32))
                } else {
                    Err(anyhow!("invalid u32 type conversion from {expr_type:?}"))
                }
            }
            Expr::AsU64(expr) => {
                let expr_type = expr.infer_type()?;
                if expr_type.is_numeric() {
                    Ok(FormatType::Base(BaseType::U64))
                } else {
                    Err(anyhow!("invalid u64 type conversion from {expr_type:?}"))
                }
            }
            Expr::Seq(exprs) => {
                let mut elem_type = FormatType::Any;
                for expr in exprs {
                    elem_type = expr.infer_type()?.unify(&elem_type)?;
                }
                Ok(FormatType::Shape(TypeShape::Seq(Box::new(elem_type))))
            }
            Expr::Tuple(exprs) => {
                let mut elem_types = Vec::with_capacity(exprs.len());
                for expr in exprs {
                    elem_types.push(expr.infer_type()?);
                }
                Ok(FormatType::Shape(TypeShape::Tuple(elem_types)))
            }
            Expr::LiftMaybe(None) => Ok(FormatType::Shape(TypeShape::Option(Box::new(FormatType::Any)))),
            Expr::LiftMaybe(Some(expr)) => {
                let expr_type = expr.infer_type()?;
                Ok(FormatType::Shape(TypeShape::Option(Box::new(expr_type))))
            }
            Expr::Variant(lab, expr) => {
                let expr_type = expr.infer_type()?;
                Ok(FormatType::Shape(TypeShape::Union(BTreeMap::from([(lab.clone(), expr_type)]))))
            }
            Expr::IntRel(_rel, lhs, rhs) => {
                let lhs_type = lhs.infer_type()?;
                let rhs_type = rhs.infer_type()?;
                match (lhs_type, rhs_type) {
                    (FormatType::Base(b1), FormatType::Base(b2)) if b1 == b2 && b1.is_numeric() => Ok(FormatType::Base(BaseType::Bool)),
                    (lhs_type, rhs_type) => Err(anyhow!("invalid integer relation between {lhs_type:?} and {rhs_type:?}")),
                }
            }
            Expr::Arith(_arith, lhs, rhs) => {
                let lhs_type = lhs.infer_type()?;
                let rhs_type = rhs.infer_type()?;
                match (lhs_type, rhs_type) {
                    (FormatType::Base(b1), FormatType::Base(b2)) if b1 == b2 && b1.is_numeric() => Ok(FormatType::Base(b1)),
                    (lhs_type, rhs_type) => Err(anyhow!("invalid arithmetic operation between {lhs_type:?} and {rhs_type:?}")),
                }
            }
            Expr::Unary(Unary::BoolNot, expr) => {
                let expr_type = expr.infer_type()?;
                if matches!(expr_type, FormatType::Base(BaseType::Bool)) {
                    Ok(FormatType::Base(BaseType::Bool))
                } else {
                    Err(anyhow!("invalid bool-not on {expr_type:?}"))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IntRel {
    Eq, Neq,
    Gt, Gte,
    Lt, Lte,
}

#[derive(Debug, Clone, Copy)]
pub enum Arith {
    Add, Sub,
    Mul, Div, Rem,
    Shl, Shr,
    BitOr, BitAnd,
}

#[derive(Debug, Clone, Copy)]
pub enum Unary {
    BoolNot,
}

pub struct FormatModule {
    names: Vec<Label>,
    decls: Vec<FormatDecl>,
}

impl FormatModule {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            decls: Vec::new(),
        }
    }

    pub fn declare_rec_formats(&mut self, formats: Vec<(Label, Format)>) -> Vec<FormatRef> {
        let fmt_id = self.decls.len();
        let batch_size = formats.len();
        let batch_id = Span::from(fmt_id..=fmt_id + batch_size - 1);
        for (ix, (name, format)) in formats.into_iter().enumerate() {
            let decl = FormatDecl {
                format,
                fmt_id: fmt_id + ix,
                f_type: Rc::new(OnceCell::new()),
                batch: Some(batch_id.clone()),
            };
            self.names.push(name);
            self.decls.push(decl);
        }
        let mut accum = Vec::with_capacity(batch_size);
        for ix in batch_id.start..=batch_id.end {
            let decl = &self.decls[ix];
            match decl.solve_type_with(self, &mut HashSet::new()) {
                Ok(_) => {
                    accum.push(FormatRef(ix));
                }
                Err(e) => {
                    panic!("Failed to solve type for {name}: {e}", name = &self.names[ix]);
                }
            }
        }
        accum
    }

    pub fn declare_format(&mut self, name: Label, format: Format) -> FormatRef {
        let fmt_id = self.decls.len();
        let f_type = Rc::new(OnceCell::new());
        let decl = FormatDecl {
            format,
            fmt_id,
            f_type,
            batch: None,
        };
        match decl.solve_type(&self) {
            Ok(_) => {
                self.names.push(name);
                self.decls.push(decl);
                FormatRef(fmt_id)
            }
            Err(e) => {
                panic!("Failed to solve type for {name}: {e}");
            }
        }
    }

    pub fn get_format_type(&self, level: usize) -> &FormatType {
        &self.decls[level].solve_type(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_type_inference() -> AResult<()> {
        let mut module = FormatModule::new();
        let expr = Expr::IntRel(IntRel::Eq, Box::new(Expr::Arith(Arith::Add, Box::new(Expr::U8(1)), Box::new(Expr::U8(1)))), Box::new(Expr::U8(2)));
        let f = Format::Compute(Box::new(expr));
        let fref = module.declare_format(Label::Borrowed("static_math"), f);
        assert!(matches!(module.get_format_type(fref.get_level()), FormatType::Base(BaseType::Bool)));
        Ok(())
    }

    #[test]
    fn cons_list_any_byte() -> AResult<()> {
        let mut module = FormatModule::new();
        let format0 = Format::Union(vec![
            Format::Variant(Label::Borrowed("Cons"), Box::new(Format::Tuple(vec![
                Format::Byte(ByteSet::full()),
                Format::RecVar(0),
            ]))),
            Format::Variant(Label::Borrowed("Nil"), Box::new(Format::Tuple(vec![]))),
        ]);
        let fref = module.declare_rec_formats(vec![(Label::Borrowed("list_any_byte"), format0)])[0];
        let expected = FormatType::Shape(TypeShape::Union(BTreeMap::from([
            (Label::Borrowed("Cons"), FormatType::Shape(TypeShape::Tuple(vec![
                    FormatType::Base(BaseType::U8),
                    FormatType::Ref(0),
                ])
            )),
            (Label::Borrowed("Nil"), FormatType::UNIT),
        ])));
        let actual = module.get_format_type(fref.get_level());
        match actual.unify(&expected) {
            Ok(FormatType::Shape(TypeShape::Union(bs))) => assert_eq!(bs.len(), 2),
            Err(e) => panic!("unification failed: {e}"),
            other => panic!("unexpected type: {other:?}"),
        }
        eprintln!("cons_list_any_byte :: {actual:?}");
        Ok(())
    }
}
