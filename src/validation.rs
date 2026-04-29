use serde::Serialize;

use crate::{
    Expr,
    codegen::typed_format::{GenType, TypedExpr},
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct Condition<X = Expr> {
    pub(crate) expr: Box<X>,
    pub(crate) severity: Severity,
}

impl<T> Condition<T> {
    pub fn new(expr: T, severity: Severity) -> Condition<T> {
        Condition {
            expr: Box::new(expr),
            severity,
        }
    }

    pub fn is_required(&self) -> bool {
        self.severity.is_strict()
    }
}

impl Condition {
    pub fn from_lambda(body: Expr) -> Self {
        Condition {
            expr: Box::new(body),
            severity: Severity::default(),
        }
    }
}

impl<T> AsRef<T> for Condition<T> {
    fn as_ref(&self) -> &T {
        self.expr.as_ref()
    }
}

pub type TypedCondition<TypeRep = GenType> = Condition<TypedExpr<TypeRep>>;

impl<TypeRep> TypedCondition<TypeRep> {
    pub fn forget(self) -> Condition<Expr> {
        Condition {
            expr: Box::new((*self.expr).into()),
            severity: self.severity,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    Expect,
    #[default]
    Require,
}

impl Severity {
    pub const fn is_strict(&self) -> bool {
        matches!(self, Severity::Require)
    }
}
