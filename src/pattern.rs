use serde::Serialize;

use crate::{scope::TypeScope, Format, FormatModule, Expr, ValueType};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Pattern {
    Binding(String),
    Wildcard,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<Pattern>),
    Variant(String, Box<Pattern>),
    Seq(Vec<Pattern>),
}

impl Pattern {
    pub const UNIT: Pattern = Pattern::Tuple(Vec::new());

    pub fn from_bytes(bs: &[u8]) -> Pattern {
        Pattern::Seq(bs.iter().copied().map(Pattern::U8).collect())
    }

    pub fn variant(label: impl Into<String>, value: impl Into<Box<Pattern>>) -> Pattern {
        Pattern::Variant(label.into(), value.into())
    }

    pub(crate) fn build_scope(&self, scope: &mut TypeScope, t: &ValueType) {
        match (self, t) {
            (Pattern::Binding(name), t) => {
                scope.push(name.clone(), t.clone());
            }
            (Pattern::Wildcard, _) => {}
            (Pattern::Bool(_b0), ValueType::Bool) => {}
            (Pattern::U8(_i0), ValueType::U8) => {}
            (Pattern::U16(_i0), ValueType::U16) => {}
            (Pattern::U32(_i0), ValueType::U32) => {}
            (Pattern::Tuple(ps), ValueType::Tuple(ts)) if ps.len() == ts.len() => {
                for (p, t) in Iterator::zip(ps.iter(), ts.iter()) {
                    p.build_scope(scope, t);
                }
            }
            (Pattern::Seq(ps), ValueType::Seq(t)) => {
                for p in ps {
                    p.build_scope(scope, t);
                }
            }
            (Pattern::Variant(label, p), ValueType::Union(branches)) => {
                if let Some((_l, t)) = branches.iter().find(|(l, _t)| label == l) {
                    p.build_scope(scope, t);
                } else {
                    panic!("no {label} in {branches:?}");
                }
            }
            _ => panic!("pattern build_scope failed"),
        }
    }

    pub(crate) fn infer_expr_branch_type(
        &self,
        scope: &mut TypeScope,
        head_type: &ValueType,
        expr: &Expr,
    ) -> Result<ValueType, String> {
        let initial_len = scope.len();
        self.build_scope(scope, head_type);
        let t = expr.infer_type(scope)?;
        scope.truncate(initial_len);
        Ok(t)
    }

    pub(crate) fn infer_format_branch_type(
        &self,
        scope: &mut TypeScope,
        head_type: &ValueType,
        module: &FormatModule,
        format: &Format,
    ) -> Result<ValueType, String> {
        let initial_len = scope.len();
        self.build_scope(scope, head_type);
        let t = module.infer_format_type(scope, format)?;
        scope.truncate(initial_len);
        Ok(t)
    }
}
