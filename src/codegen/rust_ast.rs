use std::borrow::Cow;
use std::collections::HashMap;

use crate::output::Fragment;
use crate::{Label, ValueType};

// TODO: decouple these if we expect non-one-to-one correspondence
struct RustType(ValueType);

struct TypeCompiler {
    next_ix: usize,
    revmap: HashMap<ValueType, Label>,
}

impl TypeCompiler {
    fn new() -> Self {
        Self {
            next_ix: 0,
            revmap: HashMap::new(),
        }
    }

    fn find_name_for(&mut self, vt: &ValueType) -> Label {
        if let Some(name) = self.revmap.get(vt) {
            name.clone()
        } else {
            let ix = self.next_ix;
            let name = format!("Anon{}", ix);
            self.next_ix += 1;
            Cow::Owned(name)
        }
    }
}

impl RustType {
    fn to_fragment_with(&self, tycom: &mut TypeCompiler) -> Fragment {
        Self::value_type_to_fragment_with(&self.0, tycom)
    }

    fn value_type_to_fragment_with(vt: &ValueType, tycom: &mut TypeCompiler) -> Fragment {
        match vt {
            ValueType::Any => Fragment::Char('_'),
            ValueType::Empty => Fragment::String("()".into()),
            ValueType::Bool => Fragment::String("bool".into()),
            ValueType::U8 => Fragment::String("u8".into()),
            ValueType::U16 => Fragment::String("u16".into()),
            ValueType::U32 => Fragment::String("u32".into()),
            ValueType::Char => Fragment::String("char".into()),
            ValueType::Tuple(ts) => Fragment::Char('(')
                .cat(Fragment::seq(
                    ts.iter()
                        .map(|vt| RustType::value_type_to_fragment_with(vt, tycom)),
                    Some(Fragment::String(", ".into())),
                ))
                .cat(Fragment::Char(')')),
            ValueType::Record(..) | ValueType::Union(..) => {
                let name = tycom.find_name_for(vt);
                Fragment::String(name)
            }
            ValueType::Seq(inner) => Fragment::String("Vec<".into())
                .cat(RustType::value_type_to_fragment_with(&inner, tycom))
                .cat(Fragment::Char('>')),
        }
    }
}

enum Mut {
    Mutable,
    Immutable,
}

enum RustEntity {
    Local(Label),
    Scoped(Vec<Label>, Label),
    SelfEntity,
}
impl RustEntity {
    fn to_fragment(&self) -> Fragment {
        match self {
            RustEntity::Local(v) => Fragment::String(v.clone()),
            RustEntity::Scoped(path, v) => Fragment::seq(
                path.iter()
                    .chain(std::iter::once(v))
                    .map(|scope| Fragment::String(scope.clone())),
                Some(Fragment::String("::".into())),
            ),
            RustEntity::SelfEntity => Fragment::String("self".into()),
        }
    }
}

enum RustExpr {
    Entity(RustEntity),
    StringLit(Label),
    FieldAccess(Box<RustExpr>, Label),
    FunctionCall(Box<RustExpr>, Vec<RustExpr>),
    Tuple(Vec<RustExpr>),
    Struct(Label, Vec<(Label, Option<Box<RustExpr>>)>),
    Paren(Box<RustExpr>),
    Borrow(Box<RustExpr>),
    BorrowMut(Box<RustExpr>),
    Try(Box<RustExpr>),
}

impl RustExpr {
    fn paren_list_to_fragment<'a>(items: impl IntoIterator<Item = &'a Self>) -> Fragment {
        Fragment::Char('(')
            .cat(Fragment::seq(
                items.into_iter().map(RustExpr::to_fragment),
                Some(Fragment::String(", ".into())),
            ))
            .cat(Fragment::Char(')'))
    }

    fn to_fragment(&self) -> Fragment {
        match self {
            RustExpr::Entity(e) => e.to_fragment(),
            RustExpr::StringLit(s) => Fragment::Char('"')
                .cat(Fragment::String(s.clone()))
                .cat(Fragment::Char('"')),
            RustExpr::FieldAccess(x, name) => x
                .to_fragment()
                .intervene(Fragment::Char('.'), Fragment::String(name.clone())),
            RustExpr::FunctionCall(f, args) => {
                f.to_fragment().cat(Self::paren_list_to_fragment(args))
            }
            RustExpr::Tuple(elts) => Self::paren_list_to_fragment(elts),
            RustExpr::Struct(con, fields) => Fragment::String(con.clone())
                .cat(Fragment::String("{ ".into()).cat(Fragment::seq(
                    fields.iter().map(|(lab, expr)| {
                        Fragment::intervene(
                            Fragment::String(lab.clone()),
                            Fragment::String(": ".into()),
                            expr.as_ref().map_or(Fragment::Empty, |x| x.to_fragment()),
                        )
                    }),
                    Some(Fragment::String(", ".into())),
                )))
                .cat(Fragment::String(" }".into())),
            RustExpr::Paren(expr) => Self::paren_list_to_fragment([expr.as_ref()]),
            RustExpr::Borrow(expr) => Fragment::Char('&').cat(expr.to_fragment()),
            RustExpr::BorrowMut(expr) => Fragment::String("&mut ".into()).cat(expr.to_fragment()),
            RustExpr::Try(expr) => expr.to_fragment().cat(Fragment::Char('?')),
        }
    }
}

enum RustStmt {
    Let(Mut, Label, Option<RustType>, RustExpr),
    Expr(RustExpr),
    Return(bool, RustExpr),
}

impl RustStmt {
    pub fn to_fragment_with(&self, tycom: &mut TypeCompiler) -> Fragment {
        match self {
            Self::Let(_mut, binding, sig, value) => Fragment::String("let".into())
                .cat(match _mut {
                    Mut::Mutable => Fragment::String(" mut ".into()),
                    Mut::Immutable => Fragment::Char(' '),
                })
                .cat(Fragment::String(binding.clone()))
                .intervene(
                    Fragment::String(": ".into()),
                    sig.as_ref()
                        .map_or(Fragment::Empty, |rt| rt.to_fragment_with(tycom)),
                )
                .cat(Fragment::String(" = ".into()))
                .cat(value.to_fragment()),
            Self::Expr(expr) => expr.to_fragment().cat(Fragment::Char(';')),
            Self::Return(is_keyword, expr) => {
                let (head, tail) = if *is_keyword {
                    (Fragment::String("return ".into()), Fragment::Char(';'))
                } else {
                    (Fragment::Empty, Fragment::Empty)
                };
                head.cat(expr.to_fragment()).cat(tail)
            }
        }
    }
}
