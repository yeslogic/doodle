use std::collections::HashMap;

use crate::Label;

use super::*;

pub trait Rebindable {
    fn rebind(&mut self, table: &HashMap<Label, Label>);
}

impl<T: Rebindable> Rebindable for Box<T> {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        self.as_mut().rebind(table)
    }
}

impl<T: Rebindable> Rebindable for Vec<T> {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        self.iter_mut().for_each(|item| item.rebind(table));
    }
}

impl<T: Rebindable> Rebindable for Option<T> {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        if let Some(item) = self {
            item.rebind(table)
        }
    }
}

impl Rebindable for RustProgram {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        self.items.rebind(table)
    }
}

impl Rebindable for RustItem {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        self.decl.rebind(table)
    }
}

impl Rebindable for RustDecl {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustDecl::TypeDef(name, tdef) => {
                if table.contains_key(&*name) {
                    *name = table[&*name].clone();
                }
                tdef.rebind(table)
            }
            RustDecl::Function(fn_def) => fn_def.rebind(table),
        }
    }
}

impl Rebindable for RustTypeDef {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustTypeDef::Enum(vars) => vars.rebind(table),
            RustTypeDef::Struct(str) => str.rebind(table),
        }
    }
}

impl Rebindable for RustFn {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        if table.contains_key(&self.name) {
            self.name = table[&*self.name].clone();
        }
        self.sig.rebind(table);
        self.body.iter_mut().for_each(|stmt| stmt.rebind(table))
    }
}

impl Rebindable for RustStmt {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustStmt::Expr(expr) => expr.rebind(table),
            RustStmt::Control(ctrl) => ctrl.rebind(table),
            RustStmt::Let(_, _, otyp, rhs) => {
                otyp.rebind(table);
                rhs.rebind(table)
            }
            RustStmt::Reassign(_, rhs) => rhs.rebind(table),
            RustStmt::Return(_, rhs) => rhs.rebind(table),
        }
    }
}

impl Rebindable for RustEntity {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustEntity::Local(lab) => {
                if table.contains_key(lab) {
                    *lab = table[lab].clone();
                }
            }
            RustEntity::Scoped(path, _lab) => {
                for lab in path.iter_mut() {
                    if table.contains_key(lab) {
                        *lab = table[lab].clone();
                    }
                }
            }
        }
    }
}

impl Rebindable for RustExpr {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustExpr::Entity(ent) => ent.rebind(table),
            RustExpr::PrimitiveLit(..) => (),
            RustExpr::ArrayLit(arr) => arr.rebind(table),
            RustExpr::MethodCall(head, _, args) => {
                head.rebind(table);
                args.rebind(table);
            }
            RustExpr::FieldAccess(head, _) => head.rebind(table),
            RustExpr::FunctionCall(f, args) => {
                f.rebind(table);
                args.rebind(table);
            }
            RustExpr::Tuple(elts) => elts.rebind(table),
            RustExpr::Struct(con, expr) => {
                con.rebind(table);
                match expr {
                    StructExpr::RecordExpr(flds) => {
                        flds.iter_mut().for_each(|(_, fld)| fld.rebind(table))
                    }
                    StructExpr::TupleExpr(vals) => {
                        vals.iter_mut().for_each(|val| val.rebind(table))
                    }
                    StructExpr::EmptyExpr => (),
                }
            }
            RustExpr::CloneOf(inner)
            | RustExpr::Deref(inner)
            | RustExpr::Borrow(inner)
            | RustExpr::BorrowMut(inner)
            | RustExpr::Try(inner) => inner.rebind(table),
            RustExpr::Operation(oper) => oper.rebind(table),
            RustExpr::BlockScope(stmts, ret) => {
                stmts.rebind(table);
                ret.rebind(table);
            }
            RustExpr::Control(ctrl) => ctrl.rebind(table),
            RustExpr::Closure(f) => f.rebind(table),
            RustExpr::Index(head, ix) => {
                head.rebind(table);
                ix.rebind(table);
            }
            RustExpr::Slice(head, start, stop) => {
                head.rebind(table);
                start.rebind(table);
                stop.rebind(table);
            }
            RustExpr::RangeExclusive(start, stop) => {
                start.rebind(table);
                stop.rebind(table);
            }
        }
    }
}

impl Rebindable for RustClosure {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        self.0.rebind(table);
        self.1.rebind(table);
    }
}

impl Rebindable for RustClosureHead {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustClosureHead::Thunk => (),
            RustClosureHead::SimpleVar(_, otyp) => otyp.rebind(table),
        }
    }
}

impl Rebindable for ClosureBody {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            ClosureBody::Expression(expr) => expr.rebind(table),
            ClosureBody::Statements(stmts) => stmts.rebind(table),
        }
    }
}

impl Rebindable for RustControl {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustControl::Loop(stmts) => stmts.rebind(table),
            RustControl::While(cond, stmts) => {
                cond.rebind(table);
                stmts.rebind(table);
            }
            RustControl::ForIter(_, iter, stmts) => {
                iter.rebind(table);
                stmts.rebind(table);
            }
            RustControl::ForRange0(_, max, stmts) => {
                max.rebind(table);
                stmts.rebind(table);
            }
            RustControl::If(cond, t_body, f_body) => {
                cond.rebind(table);
                t_body.rebind(table);
                f_body.rebind(table);
            }
            RustControl::Match(scrutinee, branches) => {
                scrutinee.rebind(table);
                branches.rebind(table);
            }
            RustControl::Break => (),
        }
    }
}

impl Rebindable for RustMatchBody {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustMatchBody::Irrefutable(branches) | RustMatchBody::Refutable(branches, _) => {
                branches.rebind(table);
            }
        }
    }
}

impl Rebindable for RustMatchCase {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        self.0.rebind(table);
        self.1.rebind(table);
    }
}

impl Rebindable for MatchCaseLHS {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            MatchCaseLHS::Pattern(pat) => pat.rebind(table),
            MatchCaseLHS::WithGuard(pat, cond) => {
                pat.rebind(table);
                cond.rebind(table);
            }
        }
    }
}

impl Rebindable for RustPattern {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustPattern::PrimLiteral(..)
            | RustPattern::PrimRange(..)
            | RustPattern::Fill
            | RustPattern::CatchAll(..)
            | RustPattern::BindRef(..) => (),
            RustPattern::TupleLiteral(tup) => tup.rebind(table),
            RustPattern::ArrayLiteral(arr) => arr.rebind(table),
            RustPattern::Option(o_pat) => o_pat.rebind(table),
            RustPattern::Variant(con, pat) => {
                con.rebind(table);
                pat.rebind(table);
            }
        }
    }
}

impl Rebindable for Constructor {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            Constructor::Simple(name) => {
                if table.contains_key(&*name) {
                    *name = table[&*name].clone();
                }
            }
            Constructor::Compound(name, _) => {
                if table.contains_key(&*name) {
                    *name = table[&*name].clone();
                }
            }
        }
    }
}

impl Rebindable for RustOp {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustOp::InfixOp(_, x, y) => {
                x.rebind(table);
                y.rebind(table);
            }
            RustOp::PrefixOp(_, x) => x.rebind(table),
            RustOp::AsCast(x, _) => x.rebind(table),
        }
    }
}

impl Rebindable for RustVariant {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustVariant::Unit(..) => (),
            RustVariant::Tuple(_, elts) => elts.iter_mut().for_each(|elt| elt.rebind(table)),
        }
    }
}

impl Rebindable for RustStruct {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustStruct::Record(flds) => flds.iter_mut().for_each(|(_, ftype)| ftype.rebind(table)),
        }
    }
}

impl Rebindable for FnSig {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        self.args.iter_mut().for_each(|(_, arg)| arg.rebind(table));
        match self.ret.as_mut() {
            Some(ret) => ret.rebind(table),
            None => (),
        }
    }
}

impl Rebindable for RustType {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            RustType::Atom(at) => at.rebind(table),
            RustType::AnonTuple(args) => args.iter_mut().for_each(|arg| arg.rebind(table)),
            RustType::Verbatim(..) => (),
        }
    }
}

impl Rebindable for AtomType {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            AtomType::TypeRef(tref) => tref.rebind(table),
            AtomType::Prim(_) => (),
            AtomType::Comp(comp_type) => comp_type.rebind(table),
        }
    }
}

impl Rebindable for LocalType {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            LocalType::LocalDef(_ix, lab) => {
                if table.contains_key(lab) {
                    *lab = table[lab].clone();
                }
            }
            LocalType::External(..) => (),
        }
    }
}

impl Rebindable for CompType {
    fn rebind(&mut self, table: &HashMap<Label, Label>) {
        match self {
            CompType::Vec(t)
            | CompType::Option(t)
            | CompType::Result(t, ..)
            | CompType::Borrow(.., t) => t.rebind(table),
        }
    }
}