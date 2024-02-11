use crate::{Expr, IntRel, Label};

struct Constraint {
    name: Label,
    lower: Option<u64>,
    upper: Option<u64>,
}

impl Constraint {
    fn new(name: &Label, lower: Option<u64>, upper: Option<u64>) -> Constraint {
        Constraint {
            name: name.clone(),
            lower,
            upper,
        }
    }

    fn from_expr(expr: &Expr) -> Option<Constraint> {
        match expr {
            Expr::IntRel(op, lhs, rhs) => {
                match (op, lhs.as_ref(), rhs.as_ref()) {
                    (IntRel::Eq, Expr::Var(name), expr) | (IntRel::Eq, expr, Expr::Var(name)) => {
                        let val = Constraint::get_u64(expr)?;
                        Some(Constraint::new(name, Some(val), Some(val)))
                    }
                    (IntRel::Ne, _, _) => None, // FIXME
                    (IntRel::Lt, Expr::Var(name), expr) | (IntRel::Gt, expr, Expr::Var(name)) => {
                        let val = Constraint::get_u64(expr)?;
                        Some(Constraint::new(
                            name,
                            None,
                            Some(val.checked_sub(1).unwrap()),
                        ))
                    }
                    (IntRel::Gt, Expr::Var(name), expr) | (IntRel::Lt, expr, Expr::Var(name)) => {
                        let val = Constraint::get_u64(expr)?;
                        Some(Constraint::new(
                            name,
                            Some(val.checked_add(1).unwrap()),
                            None,
                        ))
                    }
                    (IntRel::Lte, Expr::Var(name), expr) | (IntRel::Gte, expr, Expr::Var(name)) => {
                        let val = Constraint::get_u64(expr)?;
                        Some(Constraint::new(name, None, Some(val)))
                    }
                    (IntRel::Gte, Expr::Var(name), expr) | (IntRel::Lte, expr, Expr::Var(name)) => {
                        let val = Constraint::get_u64(expr)?;
                        Some(Constraint::new(name, Some(val), None))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn get_u64(expr: &Expr) -> Option<u64> {
        match expr {
            Expr::U8(n) => Some(u64::from(*n)),
            Expr::U16(n) => Some(u64::from(*n)),
            Expr::U32(n) => Some(u64::from(*n)),
            Expr::U64(n) => Some(*n),
            _ => None,
        }
    }

    fn disjoint(&self, other: &Constraint) -> bool {
        if self.name == other.name {
            match (self.lower, self.upper, other.lower, other.upper) {
                (Some(lo), _, _, Some(hi)) if lo > hi => true,
                (Some(lo), _, _, Some(hi)) if lo > hi => true,

                _ => false,
            }
        } else {
            false
        }
    }
}

// both expr must be bool
// conservative approximation, if true then definitely disjoint
pub fn disjoint(a: &Expr, b: &Expr) -> bool {
    match (Constraint::from_expr(a), Constraint::from_expr(b)) {
        (Some(ac), Some(bc)) => ac.disjoint(&bc),
        _ => false,
    }
}
