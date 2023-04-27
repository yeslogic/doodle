use crate::{Expr, Format, Type};

fn repr(format: &Format) -> Type {
    match format {
        Format::ItemVar(level) => {
            todo!() // TODO: Lookup repr in module
        }
        Format::Fail => Type::VOID,
        Format::EndOfInput => Type::UNIT,
        Format::Byte(_) => Type::U8,
        Format::Union(branches) => Type::Variant(
            branches
                .iter()
                .map(|(label, format)| (label.clone(), repr(format)))
                .collect(),
        ),
        Format::Tuple(formats) => Type::Tuple(formats.iter().map(repr).collect()),
        Format::Record(fields) => Type::Record(
            fields
                .iter()
                .map(|(label, format)| (label.clone(), repr(format)))
                .collect(),
        ),
        Format::Repeat(format)
        | Format::Repeat1(format)
        | Format::RepeatCount(_, format)
        | Format::RepeatUntil(_, format) => Type::Seq(Box::new(repr(format))),
        Format::Peek(format) => repr(format),
        Format::Slice(_, format) => repr(format),
        Format::WithRelativeOffset(_, format) => repr(format),
        Format::Map(expr, _) => {
            todo!() // TODO: `Format::Map` might require a type annotation
        }
        Format::Match(head, branches) => match branches.first() {
            Some((_, format)) => repr(format),
            None => todo!(), // TODO: return some type... perhaps `Type::VOID`?
        },
    }
}

pub fn check_format(context: &mut Vec<Type>, format: &Format) {
    match format {
        Format::ItemVar(level) => todo!(),
        Format::Fail => {}
        Format::EndOfInput => {}
        Format::Byte(_) => {}
        Format::Union(branches) => {
            // TODO: Check labels are unique
            for (label, format) in branches {
                check_format(context, format);
            }
        }
        Format::Tuple(formats) => {
            for format in formats {
                check_format(context, format);
            }
        }
        Format::Record(fields) => {
            let initial_len = context.len();
            // TODO: Check labels are unique
            for (label, format) in fields {
                check_format(context, format);
                context.push(repr(format));
            }
            context.truncate(initial_len);
        }
        Format::Repeat(format) => check_format(context, format),
        Format::Repeat1(format) => check_format(context, format),
        Format::RepeatCount(count, format) => {
            match synth_expr(context, count) {
                Type::U8 | Type::U16 | Type::U32 => {}
                _ => panic!("count was not a number"),
            }
            check_format(context, format);
        }
        Format::RepeatUntil(expr, format) => {
            check_format(context, format);
            context.push(repr(format));
            check_expr(context, expr, &Type::Bool);
            context.pop();
        }
        Format::Peek(format) => check_format(context, format),
        Format::Slice(len, format) => {
            match synth_expr(context, len) {
                Type::U8 | Type::U16 | Type::U32 => {}
                _ => panic!("length was not a number"),
            }
            check_format(context, format);
        }
        Format::WithRelativeOffset(offset, _) => {
            match synth_expr(context, offset) {
                Type::U8 | Type::U16 | Type::U32 => {}
                _ => panic!("offset was not a number"),
            }
            check_format(context, format);
        }
        Format::Map(expr, format) => {
            check_format(context, format);
            context.push(repr(format));
            synth_expr(context, expr);
            context.pop();
        }
        Format::Match(head, branches) => {
            let head_type = synth_expr(context, head);
            // TODO: Check branches have the same repr
            todo!();
        }
    }
}

fn check_expr(context: &mut Vec<Type>, expr: &Expr, r#type: &Type) {
    match (expr, r#type) {
        (Expr::Seq(exprs), Type::Seq(r#type)) => {
            for expr in exprs {
                check_expr(context, expr, r#type);
            }
        }
        (Expr::Variant(label, expr), Type::Variant(branches)) => {
            let r#type = match branches.iter().find(|(l, _)| l == label) {
                Some((_, r#type)) => r#type,
                None => panic!("no variant `{label}` in expected type"),
            };
            check_expr(context, expr, r#type);
        }
        (expr, r#type) => {
            let synth_type = synth_expr(context, expr);
            if *r#type != synth_type {
                panic!("type mismatch")
            }
        }
    }
}

fn synth_expr(context: &mut Vec<Type>, expr: &Expr) -> Type {
    match expr {
        Expr::Var(index) => context[context.len() - index - 1].clone(),
        Expr::Bool(_) => Type::Bool,
        Expr::U8(_) => Type::U8,
        Expr::U16(_) => Type::U16,
        Expr::U32(_) => Type::U32,
        Expr::Tuple(exprs) => {
            Type::Tuple(exprs.iter().map(|expr| synth_expr(context, expr)).collect())
        }
        Expr::TupleProj(head, index) => match synth_expr(context, head) {
            Type::Tuple(types) => match types.get(*index) {
                Some(r#type) => r#type.clone(),
                None => panic!("no element `{index}` in expected type"),
            },
            _ => panic!("not a tuple"),
        },
        Expr::Record(fields) => Type::Record(
            fields
                .iter()
                .map(|(label, expr)| (label.clone(), synth_expr(context, expr)))
                .collect(),
        ),
        Expr::RecordProj(head, label) => match synth_expr(context, head) {
            Type::Record(fields) => match fields.iter().find(|(l, _)| l == label) {
                Some((_, r#type)) => r#type.clone(),
                None => panic!("no field `{label}` in expected type"),
            },
            _ => panic!("not a record"),
        },
        Expr::Seq(_) => panic!("ambiguous sequence literal"),
        Expr::Variant(_, _) => panic!("ambiguous variant literal"),
        Expr::Match(head, branches) => {
            let head_type = synth_expr(context, head);
            // TODO: Check branches have the same type
            todo!()
        }

        Expr::Eq(expr0, expr1) | Expr::Ne(expr0, expr1) => {
            match (synth_expr(context, expr0), synth_expr(context, expr1)) {
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
        | Expr::Rem(expr0, expr1) => match (synth_expr(context, expr0), synth_expr(context, expr1))
        {
            (Type::U8, Type::U8) => Type::U8,
            (Type::U16, Type::U16) => Type::U16,
            (Type::U32, Type::U32) => Type::U32,
            (_, _) => panic!("unexpected operands"),
        },

        Expr::AsU16(expr) => match synth_expr(context, expr) {
            Type::U8 | Type::U16 => Type::U16,
            _ => panic!("unexpected operand"),
        },
        Expr::AsU32(expr) => match synth_expr(context, expr) {
            Type::U8 | Type::U16 | Type::U32 => Type::U32,
            _ => panic!("unexpected operand"),
        },

        Expr::U16Be(expr) | Expr::U16Le(expr) => {
            check_expr(context, expr, &Type::Tuple(vec![Type::U8; 2]));
            Type::U16
        }
        Expr::U32Be(expr) | Expr::U32Le(expr) => {
            check_expr(context, expr, &Type::Tuple(vec![Type::U8; 4]));
            Type::U32
        }
        Expr::Stream(expr) => match synth_expr(context, expr) {
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
