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
            let ret: Label = Cow::Owned(name);
            self.revmap.insert(vt.clone(), ret.clone());
            ret
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
            ValueType::Empty => Fragment::string("()"),
            ValueType::Bool => Fragment::string("bool"),
            ValueType::U8 => Fragment::string("u8"),
            ValueType::U16 => Fragment::string("u16"),
            ValueType::U32 => Fragment::string("u32"),
            ValueType::Char => Fragment::string("char"),
            ValueType::Tuple(ts) => Fragment::seq(
                ts.iter()
                    .map(|vt| RustType::value_type_to_fragment_with(vt, tycom)),
                Some(Fragment::string(", ")),
            )
            .delimit(Fragment::Char('('), Fragment::Char(')')),
            ValueType::Record(..) | ValueType::Union(..) => {
                let name = tycom.find_name_for(vt);
                Fragment::String(name)
            }
            ValueType::Seq(inner) => RustType::value_type_to_fragment_with(&inner, tycom)
                .delimit(Fragment::string("Vec<"), Fragment::Char('>')),
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
                Some(Fragment::string("::")),
            ),
            RustEntity::SelfEntity => Fragment::string("self"),
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
        Fragment::seq(
            items.into_iter().map(RustExpr::to_fragment),
            Some(Fragment::string(", ")),
        )
        .delimit(Fragment::Char('('), Fragment::Char(')'))
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
            RustExpr::Struct(con, fields) => {
                let f_fields = Fragment::seq(
                    fields.iter().map(|(lab, expr)| {
                        Fragment::intervene(
                            Fragment::String(lab.clone()),
                            Fragment::string(": "),
                            expr.as_ref().map_or(Fragment::Empty, |x| x.to_fragment()),
                        )
                    }),
                    Some(Fragment::string(", ")),
                );
                Fragment::String(con.clone())
                    .cat(Fragment::Char(' '))
                    .cat(f_fields.delimit(Fragment::string("{ "), Fragment::string(" }")))
            }
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
            Self::Let(_mut, binding, sig, value) => (match _mut {
                Mut::Mutable => Fragment::string("let mut "),
                Mut::Immutable => Fragment::string("let "),
            })
            .cat(Fragment::String(binding.clone()))
            .intervene(
                Fragment::string(": "),
                Fragment::opt(sig.as_ref(), |rt| rt.to_fragment_with(tycom)),
            )
            .cat(Fragment::string(" = "))
            .cat(value.to_fragment()),
            Self::Expr(expr) => expr.to_fragment().cat(Fragment::Char(';')),
            Self::Return(is_keyword, expr) => {
                let (before, after) = if *is_keyword {
                    (Fragment::String("return ".into()), Fragment::Char(';'))
                } else {
                    (Fragment::Empty, Fragment::Empty)
                };
                expr.to_fragment().delimit(before, after)
            }
        }
    }
}
