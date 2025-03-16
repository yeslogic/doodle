#![allow(dead_code)]
use super::TypedExpr;
use crate::Label;
use cons_rs::List;

impl<Rep: Clone> TypedExpr<Rep, Label> {
    pub(crate) fn to_nameless(&self) -> TypedExpr<Rep, u32> {
        let mut stack = VarStack::new();
        self.__to_nameless(&mut stack)
    }

    fn __to_nameless(&self, vars: &mut VarStack<'_>) -> TypedExpr<Rep, u32> {
        match self {
            TypedExpr::Var(gt, var) => {
                let Some(index) = vars.get_index(var) else {
                    panic!("reference to var {:?} not found in env: {:?}", var, vars)
                };
                TypedExpr::Var(gt.clone(), index as u32)
            }

            TypedExpr::U8(n) => TypedExpr::U8(*n),
            TypedExpr::U16(n) => TypedExpr::U16(*n),
            TypedExpr::U32(n) => TypedExpr::U32(*n),
            TypedExpr::U64(n) => TypedExpr::U64(*n),
            TypedExpr::Arith(t, op, a, b) => TypedExpr::Arith(
                t.clone(),
                *op,
                Box::new(a.__to_nameless(vars)),
                Box::new(b.__to_nameless(vars)),
            ),
            TypedExpr::Record(_gt, _fields) => {
                todo!()
            }
            _ => todo!(),
        }
    }
}

#[derive(Default)]
struct VarStack<'a> {
    vars: List<&'a Label>,
}

impl<'a> VarStack<'a> {
    pub fn new() -> Self {
        Self { vars: List::new() }
    }

    pub fn push(&'a mut self, var: &'a Label) {
        self.vars.push(var)
    }

    pub fn pop(&mut self) -> Option<&Label> {
        self.vars.pop()
    }

    pub fn len(&self) -> usize {
        self.vars.len()
    }

    pub fn get_index(&self, var: &Label) -> Option<usize> {
        for (dist, var0) in self.vars.iter().copied().enumerate() {
            if var0 == var {
                return Some(dist);
            }
        }
        return None;
    }
}

impl<'a> std::fmt::Debug for VarStack<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VarStack: ")?;
        for (dist, var) in self.vars.iter().copied().enumerate() {
            write!(f, "${}: {}, ", dist, var)?
        }
        Ok(())
    }
}
