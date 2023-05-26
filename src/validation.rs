use crate::{Expr, Format, FormatModule, Type};

pub struct Context<'module> {
    module: &'module FormatModule,
    types: Vec<Type>,
}

impl<'module> Context<'module> {
    pub fn new(module: &'module FormatModule) -> Context<'module> {
        Context {
            module,
            types: Vec::new(),
        }
    }

    fn repr(&self, format: &Format) -> Type {
        match format {
            Format::ItemVar(level) => self.repr(self.module.get_format(*level)),
            Format::Fail => Type::VOID,
            Format::EndOfInput => Type::UNIT,
            Format::Byte(_) => Type::U8,
            Format::Union(branches) => Type::Variant(
                branches
                    .iter()
                    .map(|(label, format)| (label.clone(), self.repr(format)))
                    .collect(),
            ),
            Format::Tuple(formats) => {
                Type::Tuple(formats.iter().map(|format| self.repr(format)).collect())
            }
            Format::Record(fields) => Type::Record(
                fields
                    .iter()
                    .map(|(label, format)| (label.clone(), self.repr(format)))
                    .collect(),
            ),
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::RepeatCount(_, format)
            | Format::RepeatUntil(_, format) => Type::Seq(Box::new(self.repr(format))),
            Format::Peek(format) => self.repr(format),
            Format::Slice(_, format) => self.repr(format),
            Format::WithRelativeOffset(_, format) => self.repr(format),
            Format::Map(expr, _) => {
                todo!() // TODO: `Format::Map` might require a type annotation
            }
            Format::Match(head, branches) => match branches.first() {
                Some((_, format)) => self.repr(format),
                None => todo!(), // TODO: return some type... perhaps `Type::VOID`?
            },
        }
    }

    pub fn check_type(&self, r#type: &Type) {
        match r#type {
            Type::Bool | Type::U8 | Type::U16 | Type::U32 => {}
            Type::Tuple(types) => {
                for r#type in types {
                    self.check_type(r#type);
                }
            }
            Type::Record(fields) => {
                // TODO: Check labels are unique
                for (label, r#type) in fields {
                    self.check_type(r#type);
                }
            }
            Type::Variant(branches) => {
                // TODO: Check labels are unique
                for (label, r#type) in branches {
                    self.check_type(r#type);
                }
            }
            Type::Seq(r#type) => self.check_type(r#type),
        }
    }

    pub fn check_format(&mut self, format: &Format) {
        match format {
            Format::ItemVar(level) => todo!(),
            Format::Fail => {}
            Format::EndOfInput => {}
            Format::Byte(_) => {}
            Format::Union(branches) => {
                // TODO: Check labels are unique
                for (label, format) in branches {
                    self.check_format(format);
                }
            }
            Format::Tuple(formats) => {
                for format in formats {
                    self.check_format(format);
                }
            }
            Format::Record(fields) => {
                let initial_len = self.types.len();
                // TODO: Check labels are unique
                for (label, format) in fields {
                    self.check_format(format);
                    self.types.push(self.repr(format));
                }
                self.types.truncate(initial_len);
            }
            Format::Repeat(format) => self.check_format(format),
            Format::Repeat1(format) => self.check_format(format),
            Format::RepeatCount(count, format) => {
                match self.synth_expr(count) {
                    Type::U8 | Type::U16 | Type::U32 => {}
                    _ => panic!("count was not a number"),
                }
                self.check_format(format);
            }
            Format::RepeatUntil(expr, format) => {
                self.check_format(format);
                self.types.push(self.repr(format));
                self.check_expr(expr, &Type::Bool);
                self.types.pop();
            }
            Format::Peek(format) => self.check_format(format),
            Format::Slice(len, format) => {
                match self.synth_expr(len) {
                    Type::U8 | Type::U16 | Type::U32 => {}
                    _ => panic!("length was not a number"),
                }
                self.check_format(format);
            }
            Format::WithRelativeOffset(offset, _) => {
                match self.synth_expr(offset) {
                    Type::U8 | Type::U16 | Type::U32 => {}
                    _ => panic!("offset was not a number"),
                }
                self.check_format(format);
            }
            Format::Map(expr, format) => {
                self.check_format(format);
                self.types.push(self.repr(format));
                self.synth_expr(expr);
                self.types.pop();
            }
            Format::Match(head, branches) => {
                let head_type = self.synth_expr(head);
                // TODO: Check branches have the same repr
                todo!();
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr, r#type: &Type) {
        match (expr, r#type) {
            (Expr::Seq(exprs), Type::Seq(r#type)) => {
                for expr in exprs {
                    self.check_expr(expr, r#type);
                }
            }
            (Expr::Variant(label, expr), Type::Variant(branches)) => {
                let r#type = match branches.iter().find(|(l, _)| l == label) {
                    Some((_, r#type)) => r#type,
                    None => panic!("no variant `{label}` in expected type"),
                };
                self.check_expr(expr, r#type);
            }
            (expr, r#type) => {
                let synth_type = self.synth_expr(expr);
                if *r#type != synth_type {
                    panic!("type mismatch")
                }
            }
        }
    }

    fn synth_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Var(index) => self.types[self.types.len() - index - 1].clone(),
            Expr::Bool(_) => Type::Bool,
            Expr::U8(_) => Type::U8,
            Expr::U16(_) => Type::U16,
            Expr::U32(_) => Type::U32,
            Expr::Tuple(exprs) => {
                Type::Tuple(exprs.iter().map(|expr| self.synth_expr(expr)).collect())
            }
            Expr::TupleProj(head, index) => match self.synth_expr(head) {
                Type::Tuple(types) => match types.get(*index) {
                    Some(r#type) => r#type.clone(),
                    None => panic!("no element `{index}` in expected type"),
                },
                _ => panic!("not a tuple"),
            },
            Expr::Record(fields) => Type::Record(
                fields
                    .iter()
                    .map(|(label, expr)| (label.clone(), self.synth_expr(expr)))
                    .collect(),
            ),
            Expr::RecordProj(head, label) => match self.synth_expr(head) {
                Type::Record(fields) => match fields.iter().find(|(l, _)| l == label) {
                    Some((_, r#type)) => r#type.clone(),
                    None => panic!("no field `{label}` in expected type"),
                },
                _ => panic!("not a record"),
            },
            Expr::Seq(_) => panic!("ambiguous sequence literal"),
            Expr::Variant(_, _) => panic!("ambiguous variant literal"),
            Expr::Match(head, branches) => {
                let head_type = self.synth_expr(head);
                // TODO: Check branches have the same type
                todo!()
            }

            Expr::Eq(expr0, expr1) | Expr::Ne(expr0, expr1) => {
                match (self.synth_expr(expr0), self.synth_expr(expr1)) {
                    (Type::U8, Type::U8) => Type::Bool,
                    (Type::U16, Type::U16) => Type::Bool,
                    (Type::U32, Type::U32) => Type::Bool,
                    (_, _) => panic!("unexpected operands"),
                }
            }
            Expr::BitAnd(expr0, expr1)
            | Expr::BitOr(expr0, expr1)
            | Expr::Shl(expr0, expr1)
            | Expr::Add(expr0, expr1)
            | Expr::Sub(expr0, expr1)
            | Expr::Rem(expr0, expr1) => match (self.synth_expr(expr0), self.synth_expr(expr1)) {
                (Type::U8, Type::U8) => Type::U8,
                (Type::U16, Type::U16) => Type::U16,
                (Type::U32, Type::U32) => Type::U32,
                (_, _) => panic!("unexpected operands"),
            },

            Expr::AsU16(expr) => match self.synth_expr(expr) {
                Type::U8 | Type::U16 => Type::U16,
                _ => panic!("unexpected operand"),
            },
            Expr::AsU32(expr) => match self.synth_expr(expr) {
                Type::U8 | Type::U16 | Type::U32 => Type::U32,
                _ => panic!("unexpected operand"),
            },

            Expr::U16Be(expr) | Expr::U16Le(expr) => {
                self.check_expr(expr, &Type::Tuple(vec![Type::U8; 2]));
                Type::U16
            }
            Expr::U32Be(expr) | Expr::U32Le(expr) => {
                self.check_expr(expr, &Type::Tuple(vec![Type::U8; 4]));
                Type::U32
            }
            Expr::Stream(expr) => match self.synth_expr(expr) {
                Type::Seq(elem_type) => match elem_type.as_ref() {
                    Type::Variant(branches) => match branches.as_slice() {
                        [(some_label, some_type), (none_label, Type::Tuple(none_types))]
                        | [(none_label, Type::Tuple(none_types)), (some_label, some_type)]
                            if some_label == "some"
                                && none_label == "none"
                                && none_types.is_empty() =>
                        {
                            Type::Seq(Box::new(some_type.clone()))
                        }
                        _ => panic!("not an option type"),
                    },
                    _ => panic!("not an option type"),
                },
                _ => panic!("not a sequence"),
            },
        }
    }
}
