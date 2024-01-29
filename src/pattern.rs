use crate::{BaseType, Expr, Format, FormatModule, IntoLabel, Label, TypeScope, ValueType};
use anyhow::Result as AResult;
use serde::Serialize;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Pattern {
    Binding(Label),
    Wildcard,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Char(char),
    Tuple(Vec<Pattern>),
    Variant(Label, Box<Pattern>),
    Seq(Vec<Pattern>),
}

impl Pattern {
    pub const UNIT: Pattern = Pattern::Tuple(Vec::new());

    pub fn from_bytes(bs: &[u8]) -> Pattern {
        Pattern::Seq(bs.iter().copied().map(Pattern::U8).collect())
    }

    pub fn variant(label: impl IntoLabel, value: impl Into<Box<Self>>) -> Self {
        Pattern::Variant(label.into(), value.into())
    }

    pub fn binding(name: impl IntoLabel) -> Pattern {
        Pattern::Binding(name.into())
    }

    pub(crate) fn build_scope(&self, scope: &mut TypeScope<'_>, t: Rc<ValueType>) {
        match (self, t.as_ref()) {
            (Pattern::Binding(name), t) => {
                // FIXME - do we want to store an Rc<ValueType> in the scope instead, perhaps...?
                scope.push(name.clone(), t.clone());
            }
            (Pattern::Wildcard, _) => {}
            (Pattern::Bool(..), ValueType::Base(BaseType::Bool)) => {}
            (Pattern::U8(..), ValueType::Base(BaseType::U8)) => {}
            (Pattern::U16(..), ValueType::Base(BaseType::U16)) => {}
            (Pattern::U32(..), ValueType::Base(BaseType::U32)) => {}
            (Pattern::Tuple(ps), ValueType::Tuple(ts)) if ps.len() == ts.len() => {
                for (p, t) in Iterator::zip(ps.iter(), ts.iter()) {
                    p.build_scope(scope, Rc::new(t.clone()));
                }
            }
            (Pattern::Seq(ps), ValueType::Seq(t)) => {
                for p in ps {
                    p.build_scope(scope, Rc::new((**t).clone()));
                }
            }
            (Pattern::Variant(label, p), ValueType::Union(branches)) => {
                if let Some((_l, t)) = branches.iter().find(|(l, _t)| label == l) {
                    // FIXME - this is pretty bad, but it is hard to do better without more destructive changes
                    let tmp = Rc::new(t.clone());
                    p.build_scope(scope, tmp);
                } else {
                    panic!("no {label} in {branches:?}");
                }
            }
            _ => panic!("pattern build_scope failed"),
        }
    }

    pub(crate) fn infer_expr_branch_type(
        &self,
        scope: &TypeScope<'_>,
        head_type: Rc<ValueType>,
        expr: &Expr,
    ) -> AResult<ValueType> {
        let mut pattern_scope = TypeScope::child(scope);
        self.build_scope(&mut pattern_scope, head_type);
        expr.infer_type(&pattern_scope)
    }

    pub(crate) fn infer_format_branch_type(
        &self,
        scope: &TypeScope<'_>,
        head_type: Rc<ValueType>,
        module: &FormatModule,
        format: &Format,
    ) -> AResult<ValueType> {
        let mut pattern_scope = TypeScope::child(scope);
        self.build_scope(&mut pattern_scope, head_type);
        module.infer_format_type(&pattern_scope, format)
    }
}
