use super::*;
use crate::codegen::rust_ast::analysis::SourceContext;

pub(crate) struct Solution {
    is_copy: bool,
    is_ref: bool,
}

pub trait Resolvable {
    fn resolve(&mut self, ctx: &SourceContext<'_>);
}

impl<T: Resolvable> Resolvable for Box<T> {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        self.as_mut().resolve(ctx);
    }
}

impl<T: Resolvable> Resolvable for Vec<T> {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        self.iter_mut().for_each(|item| item.resolve(ctx));
    }
}

impl<T: Resolvable> Resolvable for Option<T> {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        if let Some(t) = self {
            t.resolve(ctx);
        }
    }
}

impl Resolvable for RustProgram {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        self.items.resolve(ctx)
    }
}

impl Resolvable for RustItem {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        self.decl.resolve(ctx)
    }
}

impl Resolvable for RustDecl {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        match self {
            RustDecl::Function(f) => f.resolve(ctx),
            RustDecl::TypeDef(..) => (),
        }
    }
}

impl Resolvable for RustFn {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        self.body.resolve(ctx)
    }
}

impl Resolvable for RustStmt {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        match self {
            RustStmt::Let(.., expr)
            | RustStmt::LetPattern(.., expr)
            | RustStmt::Reassign(.., expr)
            | RustStmt::Return(.., expr)
            | RustStmt::Expr(expr) => expr.resolve(ctx),
            RustStmt::Control(ctrl) => ctrl.resolve(ctx),
        }
    }
}

impl Resolvable for RustControl {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        match self {
            RustControl::Loop(stmts) => stmts.resolve(ctx),
            RustControl::While(expr, stmts)
            | RustControl::ForRange0(.., expr, stmts)
            | RustControl::ForIter(.., expr, stmts) => {
                expr.resolve(ctx);
                stmts.resolve(ctx);
            }
            RustControl::If(pred, b_true, b_false) => {
                pred.resolve(ctx);
                b_true.resolve(ctx);
                b_false.resolve(ctx);
            }
            RustControl::Match(expr, body) => {
                expr.resolve(ctx);
                match body {
                    RustMatchBody::Irrefutable(items) => {
                        items.iter_mut().for_each(|(_, item)| item.resolve(ctx))
                    }
                    RustMatchBody::Refutable(items, rust_catch_all) => match rust_catch_all {
                        RustCatchAll::PanicUnreachable { .. } => {
                            items.iter_mut().for_each(|(_, item)| item.resolve(ctx))
                        }
                        RustCatchAll::ReturnErrorValue { value } => {
                            value.resolve(ctx);
                            items.iter_mut().for_each(|(_, item)| item.resolve(ctx))
                        }
                    },
                }
            }
            RustControl::Break => (),
        }
    }
}

impl Resolvable for RustExpr {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        match self {
            RustExpr::Owned(owned) => {
                owned.resolve(ctx);
            }
            RustExpr::ArrayLit(arr) => arr.resolve(ctx),
            RustExpr::PrimitiveLit(..) | RustExpr::Entity(..) => (),
            RustExpr::MethodCall(recv, .., args) => {
                recv.resolve(ctx);
                args.resolve(ctx);
            }
            RustExpr::FunctionCall(fun, args) => {
                fun.resolve(ctx);
                args.resolve(ctx);
            }
            RustExpr::Tuple(args) => args.resolve(ctx),
            RustExpr::Struct(.., str) => str.resolve(ctx),
            RustExpr::Try(expr)
            | RustExpr::FieldAccess(expr, ..)
            | RustExpr::Borrow(expr)
            | RustExpr::ResultOk(_, expr)
            | RustExpr::ResultErr(expr)
            | RustExpr::Macro(RustMacro::Matches(expr, _))
            | RustExpr::BorrowMut(expr) => expr.resolve(ctx),
            RustExpr::Macro(RustMacro::Vec(v)) => match v {
                VecExpr::Nil => (),
                VecExpr::Single(expr) => expr.resolve(ctx),
                VecExpr::Repeat(expr, count) => {
                    expr.resolve(ctx);
                    count.resolve(ctx);
                }
                VecExpr::List(exprs) => exprs.resolve(ctx),
            },
            RustExpr::Operation(op) => op.resolve(ctx),
            RustExpr::BlockScope(stmts, expr) => {
                stmts.resolve(ctx);
                expr.resolve(ctx);
            }
            RustExpr::Control(ctrl) => ctrl.resolve(ctx),
            RustExpr::Closure(closure) => match &mut closure.1 {
                ClosureBody::Expression(expr) => expr.resolve(ctx),
                ClosureBody::Statements(stmts) => stmts.resolve(ctx),
            },
            RustExpr::Index(expr, ix) => {
                expr.resolve(ctx);
                ix.resolve(ctx);
            }
            RustExpr::Slice(expr0, expr1, expr2) => {
                expr0.resolve(ctx);
                expr1.resolve(ctx);
                expr2.resolve(ctx);
            }
            RustExpr::RangeExclusive(expr0, expr1) => {
                expr0.resolve(ctx);
                expr1.resolve(ctx);
            }
        }
    }
}

impl Resolvable for RustOp {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        match self {
            RustOp::InfixOp(.., lhs, rhs) => {
                lhs.resolve(ctx);
                rhs.resolve(ctx);
            }
            RustOp::PrefixOp(_, expr) | RustOp::AsCast(expr, _) => expr.resolve(ctx),
        }
    }
}

impl Resolvable for StructExpr {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        match self {
            StructExpr::EmptyExpr => (),
            StructExpr::TupleExpr(elts) => elts.resolve(ctx),
            StructExpr::RecordExpr(flds) => flds.iter_mut().for_each(|(_, fld)| fld.resolve(ctx)),
        }
    }
}

impl Resolvable for OwnedRustExpr {
    fn resolve(&mut self, ctx: &SourceContext<'_>) {
        match &mut self.kind {
            OwnedKind::Unresolved(lens) => {
                let sol = solve_lens(&lens, ctx);
                if sol.is_copy {
                    if sol.is_ref {
                        self.kind = OwnedKind::Deref;
                    } else {
                        self.kind = OwnedKind::Copied;
                    }
                } else {
                    self.kind = OwnedKind::Cloned;
                }
            }
            _ => (),
        }
    }
}

fn solve_type(ty: &RustType, ctx: &SourceContext<'_>) -> Solution {
    match ty {
        RustType::Atom(at) => match at {
            AtomType::Prim(_) => Solution {
                is_copy: true,
                is_ref: false,
            },
            AtomType::TypeRef(lt) => match lt {
                LocalType::LocalDef(ix, ..) => {
                    let is_copy = ctx.get_copy(*ix);
                    let is_ref = false;
                    Solution { is_copy, is_ref }
                }
                LocalType::External(..) => unreachable!("external type cannot be solved"),
            },
            AtomType::Comp(ct) => match ct {
                CompType::Vec(..) => Solution {
                    is_copy: false,
                    is_ref: false,
                },
                CompType::RawSlice(elt) => {
                    let Solution { is_copy, .. } = solve_type(&elt, ctx);
                    Solution {
                        is_copy,
                        is_ref: false,
                    }
                }
                CompType::Option(inner) | CompType::Result(inner, _) => {
                    let Solution { is_copy, .. } = solve_type(&inner, ctx);
                    Solution {
                        is_copy,
                        is_ref: false,
                    }
                }
                CompType::Borrow(.., t) => {
                    let Solution { is_copy, .. } = solve_type(&t, ctx);
                    Solution {
                        is_copy,
                        is_ref: true,
                    }
                }
            },
        },
        RustType::AnonTuple(rust_types) => {
            let mut is_copy = true;
            for ty in rust_types.iter() {
                let sol = solve_type(ty, ctx);
                is_copy &= sol.is_copy;
            }
            Solution {
                is_copy,
                is_ref: false,
            }
        }
        RustType::Verbatim(..) => unreachable!("unsolvable verbatim type: {ty:?}"),
    }
}

fn expand_lens<'a>(lens: &'a Lens<RustType>, ctx: &SourceContext<'_>) -> Cow<'a, RustType> {
    match lens {
        Lens::Ground(ty) => Cow::Borrowed(ty),
        Lens::ElemOf(lens) => {
            let ty0 = expand_lens(lens.as_ref(), ctx);
            Cow::Owned(get_elem(ty0.as_ref(), ctx))
        }
        Lens::FieldAccess(SubIdent::ByName(lab), lens) => {
            let ty0 = expand_lens(lens.as_ref(), ctx);
            Cow::Owned(get_field(ty0.as_ref(), lab, ctx))
        }
        Lens::FieldAccess(SubIdent::ByPosition(ix), lens) => {
            let ty0 = expand_lens(lens.as_ref(), ctx);
            Cow::Owned(get_pos(ty0.as_ref(), *ix, ctx))
        }
        Lens::ParamOf(lens) => {
            let ty0 = expand_lens(lens.as_ref(), ctx);
            Cow::Owned(get_param(ty0.as_ref(), ctx))
        }
    }
}

fn get_field_def(def: &RustTypeDecl, lab: &str) -> RustType {
    match &def.def {
        RustTypeDef::Enum(..) => unreachable!("bad Field on enum: {def:?}"),
        RustTypeDef::Struct(str) => match str {
            RustStruct::Record(fields) => {
                fields
                    .iter()
                    .find(|f| f.0.as_ref() == lab)
                    .cloned()
                    .unwrap_or_else(|| panic!("missing field `{lab}` in {def:?}"))
                    .1
            }
        },
    }
}

fn get_field(ty: &RustType, lab: &str, ctx: &SourceContext<'_>) -> RustType {
    match ty {
        RustType::Atom(at) => match at {
            AtomType::TypeRef(lt) => match lt {
                LocalType::LocalDef(ix, ..) => {
                    let def = ctx.get_def(*ix);
                    get_field_def(def, lab)
                }
                LocalType::External(..) => unreachable!("external type cannot be solved: {ty:?}"),
            },
            AtomType::Comp(ct) => match ct {
                CompType::Borrow(.., ty0) => get_field(ty0, lab, ctx),
                _ => unreachable!("bad Field on non-record: {ty:?}"),
            },
            _ => unreachable!("bad Field on non-record: {ty:?}"),
        },
        _ => unreachable!("bad Field on non-record: {ty:?}"),
    }
}

fn get_elem(ty: &RustType, _ctx: &SourceContext<'_>) -> RustType {
    match ty {
        RustType::Atom(at) => match at {
            AtomType::Comp(ct) => match ct {
                CompType::RawSlice(ty0) | CompType::Vec(ty0) => ty0.as_ref().clone(),
                CompType::Borrow(.., ty) => get_elem(ty.as_ref(), _ctx),
                _ => unreachable!("bad ElemOf on non-array: {ty:?}"),
            },
            _ => unreachable!("bad ElemOf on non-array: {ty:?}"),
        },
        _ => unreachable!("bad ElemOf on non-array: {ty:?}"),
    }
}

fn get_param(ty: &RustType, _ctx: &SourceContext<'_>) -> RustType {
    match ty {
        RustType::Atom(at) => match at {
            AtomType::Comp(ct) => match ct {
                CompType::Option(ty0) => ty0.as_ref().clone(),
                CompType::Borrow(.., ty) => get_param(ty.as_ref(), _ctx),
                _ => unreachable!("bad ParamOf on non-generic: {ty:?}"),
            },
            _ => unreachable!("bad ParamOf on non-generic: {ty:?}"),
        },
        _ => unreachable!("bad ParamOf on non-generic: {ty:?}"),
    }
}

fn get_pos(ty: &RustType, ix: usize, ctx: &SourceContext<'_>) -> RustType {
    match ty {
        RustType::AnonTuple(elts) => elts[ix].clone(),
        RustType::Atom(at) => match at {
            AtomType::Comp(ct) => match ct {
                CompType::Borrow(.., ty) => get_pos(ty.as_ref(), ix, ctx),
                _ => unreachable!("bad Position on non-tuple: {ty:?}"),
            },
            _ => unreachable!("bad Position on non-tuple: {ty:?}"),
        },
        _ => unreachable!("bad Position on non-tuple: {ty:?}"),
    }
}

fn solve_lens(lens: &Lens<RustType>, ctx: &SourceContext<'_>) -> Solution {
    let ty = expand_lens(lens, ctx);
    solve_type(ty.as_ref(), ctx)
}
