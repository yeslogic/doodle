use super::NVar;
use super::base_set::IntSet;
use crate::numeric::{
    core::{Bounds, Expr, MachineRep, NumRep},
    elaborator::{IntType, PRIM_INTS, PrimInt},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum NUType {
    Var(NVar),
    Int(IntType),
}

impl From<IntType> for NUType {
    fn from(value: IntType) -> Self {
        NUType::Int(value)
    }
}

impl From<NVar> for NUType {
    fn from(value: NVar) -> Self {
        NUType::Var(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NVType {
    Int(IntType),
    Within(Bounds),
    // REVIEW - is this variant strictly necessary for InferenceEngine?
    Abstract(NUType),
}

use super::Alias;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum Constraints {
    #[default]
    Indefinite,
    Invariant(Constraint),
}

impl Constraints {
    pub fn new() -> Constraints {
        Constraints::Indefinite
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NumConstraint {
    Int(IntType),
    // Must be able to represent all values within the specified range
    Encompasses(crate::numeric::core::Bounds),
}

impl std::fmt::Display for NumConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumConstraint::Int(it) => write!(f, "= {it}"),
            NumConstraint::Encompasses(bounds) => write!(f, "∈ {}", bounds),
        }
    }
}

#[allow(dead_code)]
impl NumConstraint {
    /// Returns `Some(true)` if this constraint is definitely satisfiable by a given type-assignment.
    ///
    /// If the constraint is known to be unsatisfiable by the assignment, returns `Some(false)` instead.
    ///
    /// Otherwise, returns `None` if the constraint cannot be determined statically.
    pub(crate) fn is_satisfied_by(&self, candidate: IntType) -> Option<bool> {
        match self {
            NumConstraint::Int(int_type) => Some(int_type == &candidate),
            NumConstraint::Encompasses(bounds) => {
                let IntType::Prim(candidate) = candidate;
                Some(bounds.is_encompassed_by(
                    &<PrimInt as Into<crate::numeric::MachineRep>>::into(candidate).as_bounds(),
                ))
            }
        }
    }

    // NOTE - should only be called on Encompasses
    pub(crate) fn get_unique_solution(&self) -> InferenceResult<IntType> {
        debug_assert!(matches!(self, NumConstraint::Encompasses(_)));
        // REVIEW - there are smarter ways of calculating this
        let mut solutions = Vec::with_capacity(8);
        for prim_int in PRIM_INTS.iter() {
            match self.is_satisfied_by(IntType::Prim(*prim_int)) {
                Some(true) => {
                    solutions.push(*prim_int);
                }
                Some(false) => (),
                None => panic!(
                    "unexpected call to get_unique_solution on `{self}` (either trivial or insoluble)"
                ),
            }
        }
        match solutions.as_slice() {
            [] => Err(InferenceError::NoSolution),
            [uniq] => Ok(IntType::Prim(*uniq)),
            _ => Err(InferenceError::MultipleSolutions),
        }
    }

    pub(crate) fn to_int_set(&self) -> IntSet {
        match self {
            NumConstraint::Int(it) => IntSet::Single(it.to_prim()),
            NumConstraint::Encompasses(bounds) => IntSet::from_bounds(bounds),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Constraint {
    Equiv(NUType),
    // Must be able to represent all values within the specified range
    Encompasses(crate::numeric::core::Bounds),
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constraint::Equiv(utype) => write!(f, "= {utype:?}"),
            Constraint::Encompasses(bounds) => write!(f, "∈ {}", bounds),
        }
    }
}

impl From<NUType> for Constraint {
    fn from(value: NUType) -> Self {
        Constraint::Equiv(value)
    }
}

impl Constraint {
    /// Returns `Some(true)` if this constraint is definitely satisfiable by a given type-assignment.
    ///
    /// If the constraint is known to be unsatisfiable by the assignment, returns `Some(false)` instead.
    ///
    /// Otherwise, returns `None` if the constraint cannot be determined statically.
    pub(crate) fn is_satisfied_by(&self, candidate: IntType) -> Option<bool> {
        match self {
            Constraint::Equiv(utype) => match utype {
                NUType::Var(_) => None,
                NUType::Int(int_type) => Some(int_type == &candidate),
            },
            Constraint::Encompasses(bounds) => {
                let IntType::Prim(candidate) = candidate;
                Some(
                    bounds.is_encompassed_by(
                        &<PrimInt as Into<MachineRep>>::into(candidate).as_bounds(),
                    ),
                )
            }
        }
    }

    // NOTE - should only be called on Encompasses
    pub(crate) fn get_unique_solution(&self) -> InferenceResult<IntType> {
        debug_assert!(matches!(self, Constraint::Encompasses(_)));
        // REVIEW - there are smarter ways of calculating this
        let mut solutions = Vec::with_capacity(8);
        for prim_int in PRIM_INTS.iter() {
            match self.is_satisfied_by(IntType::Prim(*prim_int)) {
                Some(true) => {
                    solutions.push(*prim_int);
                }
                Some(false) => (),
                None => panic!(
                    "unexpected call to get_unique_solution on `{self}` (either trivial or insoluble)"
                ),
            }
        }
        match solutions.as_slice() {
            [] => Err(InferenceError::NoSolution),
            [uniq] => Ok(IntType::Prim(*uniq)),
            _ => Err(InferenceError::MultipleSolutions),
        }
    }
}

use super::error::{InferenceError, InferenceResult};

#[derive(Debug)]
pub struct InferenceEngine {
    constraints: Vec<Constraints>,
    aliases: Vec<Alias>,
    // global_deps: GlobalDeps,
}

impl InferenceEngine {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            aliases: Vec::new(),
            // global_deps: GlobalDeps::new(),
        }
    }

    pub fn init_var_simple(&mut self, typ: NUType) -> InferenceResult<(NVar, NUType)> {
        let newvar = self.get_new_nvar();
        let constr = Constraint::Equiv(typ);
        self.unify_var_constraint(newvar, constr)?;
        Ok((newvar, typ))
    }

    fn get_new_nvar(&mut self) -> NVar {
        let ret = NVar(self.constraints.len());
        self.constraints.push(Constraints::new());
        self.aliases.push(Alias::new());
        ret
    }
}

impl InferenceEngine {
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn check_uvar_sanity(&self) {
        assert_eq!(self.constraints.len(), self.aliases.len());
    }
}

impl InferenceEngine {
    fn get_canonical(&self, v: NVar) -> NVar {
        match self.aliases[v.0] {
            Alias::Canonical(_) | Alias::Ground => v,
            Alias::BackRef(ix) => NVar(ix),
        }
    }

    fn unify_var_constraint(
        &mut self,
        var: NVar,
        constraint: Constraint,
    ) -> InferenceResult<Constraint> {
        let can_ix = self.get_canonical(var).0;

        match &self.constraints[can_ix] {
            Constraints::Indefinite => {
                let ret = constraint.clone();
                self.constraints[can_ix] = Constraints::Invariant(constraint);
                Ok(ret)
            }
            Constraints::Invariant(prior) => {
                let c1 = prior.clone();
                if c1 == constraint {
                    return Ok(c1);
                }
                let ret = self.unify_constraint_pair(c1, constraint)?;
                self.constraints[can_ix] = Constraints::Invariant(ret.clone());
                Ok(ret)
            }
        }
    }

    /// Repoints `hi` back to `lo` and adds a forward reference from `lo` to `hi`.
    ///
    /// # Safety
    ///
    /// This method is not known to cause UB, but it is an internal-facing call that may lead to
    /// corrupted alias-state if called in a context that does not check certain preconditions. In particular,
    ///  - `lo` < `hi`
    ///  - `lo` must be canonical
    unsafe fn repoint(&mut self, lo: usize, hi: usize) {
        self.aliases[hi].set_backref(lo);
        self.aliases[lo].add_forward_ref(hi);
    }

    /// Given two indices `a1` and `a2` extracted from aliased `NVar`s, reconciles the constraints
    /// stored at both indices and updates the stored constraints-values accordingly.
    ///
    /// Will return an InferenceError if both indices point to `Invariant` constraints that cannot be reconciled.
    ///
    /// # Safety
    ///
    /// This method does not involve any UB, but is a low-level internal that should only be called in the context
    /// of a larger alias-and-unify operation, as in `unify_var_pair`. It is marked `unsafe` to provide a linting
    /// error when used malapropos.
    unsafe fn transfer_constraints(
        &mut self,
        a1: usize,
        a2: usize,
    ) -> InferenceResult<&Constraints> {
        if a1 == a2 {
            return Ok(&self.constraints[a1]);
        }

        match (&self.constraints[a1], &self.constraints[a2]) {
            (Constraints::Indefinite, Constraints::Indefinite) => Ok(&Constraints::Indefinite),
            (Constraints::Indefinite, _) => Ok(self.replace_constraints_from_index(a1, a2)),
            (_, Constraints::Indefinite) => Ok(self.replace_constraints_from_index(a2, a1)),
            (Constraints::Invariant(c1), Constraints::Invariant(c2)) => {
                let c0 = self.unify_constraint_pair(c1.clone(), c2.clone())?;
                let _ = self.replace_constraints_with_value(a1, Constraints::Invariant(c0.clone()));
                let _ = self.replace_constraints_with_value(a2, Constraints::Invariant(c0));
                Ok(&self.constraints[a1])
            }
        }
    }

    #[must_use]
    /// Overwrites the [`Constraints`] at the specified index `tgt_ix` with the immediate value at the specified index `src_ix`, returning
    /// a reference to the value at the rewritten index.
    fn replace_constraints_from_index(&mut self, tgt_ix: usize, src_ix: usize) -> &Constraints {
        assert_ne!(tgt_ix, src_ix);
        let val = self.constraints[src_ix].clone();
        self.replace_constraints_with_value(tgt_ix, val)
    }

    #[must_use]
    /// Overwrites the [`Constraints`] at the specified index with the provided value, returning
    /// a reference to the updated value.
    fn replace_constraints_with_value(&mut self, ix: usize, val: Constraints) -> &Constraints {
        self.constraints[ix] = val;
        &self.constraints[ix]
    }

    /// Performs unification between two variables, returning a reference to the resulting [`Constraints`]
    /// that now apply to both.
    ///
    /// Handles both aliasing and constraint-unification.
    fn unify_var_pair(&mut self, v1: NVar, v2: NVar) -> InferenceResult<&Constraints> {
        if v1 == v2 {
            return Ok(&self.constraints[v1.0]);
        }

        // short-circuit if already equated
        match (&self.aliases[v1.0], &self.aliases[v2.0]) {
            (Alias::Ground, Alias::Ground) => {
                if v1 < v2 {
                    unsafe {
                        self.repoint(v1.0, v2.0);
                        self.transfer_constraints(v1.0, v2.0)
                    }
                } else {
                    unsafe {
                        self.repoint(v2.0, v1.0);
                        self.transfer_constraints(v2.0, v1.0)
                    }
                }
            }
            (Alias::Ground, &Alias::BackRef(can_ix)) if v1.0 > can_ix => unsafe {
                self.repoint(can_ix, v1.0);
                self.transfer_constraints(can_ix, v1.0)
            },
            (Alias::Ground, &Alias::BackRef(can_ix)) if v1.0 < can_ix => {
                debug_assert!(
                    self.aliases[can_ix].is_canonical_nonempty(),
                    "half-alias ?{can_ix}-|<-{v1}"
                );
                debug_assert!(
                    !self.aliases[can_ix].contains_fwd_ref(v2.0),
                    "retrograde half-aliased 'forward' ref ?{can_ix}->|-{v2}"
                );
                unsafe { self.recanonicalize(v1.0, can_ix) }
            }
            (Alias::Ground, &Alias::BackRef(_can_ix)) => {
                unreachable!("unexpected half-alias {v1}-|<-{v2}");
            }
            (&Alias::BackRef(can_ix), Alias::Ground) if v2.0 > can_ix => unsafe {
                self.repoint(can_ix, v2.0);
                self.transfer_constraints(can_ix, v2.0)
            },
            (&Alias::BackRef(can_ix), Alias::Ground) if v2.0 < can_ix => {
                debug_assert!(
                    self.aliases[can_ix].is_canonical_nonempty(),
                    "half-alias ?{can_ix}-|<-{v1}"
                );
                debug_assert!(
                    !self.aliases[can_ix].contains_fwd_ref(v2.0),
                    "retrograde half-aliased 'forward' ref ?{can_ix}->|-{v2}"
                );
                unsafe { self.recanonicalize(v2.0, can_ix) }
            }
            (&Alias::BackRef(_can_ix), Alias::Ground) => {
                unreachable!("unexpected half-alias {v2}-|<-{v1}");
            }
            (Alias::Ground, Alias::Canonical(_)) => {
                if v1.0 < v2.0 {
                    debug_assert!(
                        !self.aliases[v2.0].contains_fwd_ref(v1.0),
                        "retrograde half-aliased 'forward' ref {v2}->|-{v1}"
                    );
                    unsafe { self.recanonicalize(v1.0, v2.0) }
                } else {
                    unsafe {
                        self.repoint(v2.0, v1.0);
                        self.transfer_constraints(v2.0, v1.0)
                    }
                }
            }
            (Alias::Canonical(_), Alias::Ground) => {
                if v2.0 < v1.0 {
                    debug_assert!(
                        !self.aliases[v1.0].contains_fwd_ref(v2.0),
                        "retrograde half-aliased 'forward' ref {v1}->|-{v2}"
                    );
                    unsafe { self.recanonicalize(v2.0, v1.0) }
                } else {
                    unsafe {
                        self.repoint(v1.0, v2.0);
                        self.transfer_constraints(v1.0, v2.0)
                    }
                }
            }
            (&Alias::BackRef(ix1), &Alias::BackRef(ix2)) if ix1 < ix2 => unsafe {
                self.recanonicalize(ix1, ix2)
            },
            (&Alias::BackRef(ix1), &Alias::BackRef(ix2)) if ix2 < ix1 => unsafe {
                self.recanonicalize(ix2, ix1)
            },
            (&Alias::BackRef(ix), &Alias::BackRef(_ix)) => {
                // the two are equal so nothing needs to be changed; we will check both are forward-aliased, however
                let common = &self.aliases[ix];
                debug_assert!(
                    common.contains_fwd_ref(v1.0),
                    "unexpected half-alias ?{ix}<-{v1}"
                );
                debug_assert!(
                    common.contains_fwd_ref(v2.0),
                    "unexpected half-alias ?{ix}<-{v2}"
                );
                Ok(&self.constraints[ix])
            }
            (a1 @ Alias::BackRef(tgt), a2 @ Alias::Canonical(fwds)) => {
                let left = fwds.contains(&v1.0);
                let right = *tgt == v2.0;

                match (left, right) {
                    (true, true) => {
                        return Ok(&self.constraints[v2.0]);
                    }
                    (true, false) | (false, true) => {
                        unreachable!(
                            "mismatched back- and forward-references for {v1} ({a1:?}) and {v2} ({a2:?})"
                        )
                    }
                    (false, false) => (),
                }

                let ix1 = *tgt;
                let ix2 = v2.0;

                // check not the actual indices, but the canonical indices for tie-breaking
                if ix1 < ix2 {
                    unsafe { self.recanonicalize(ix1, ix2) }
                } else {
                    unsafe { self.recanonicalize(ix2, ix1) }
                }
            }
            (a1 @ Alias::Canonical(fwds), a2 @ Alias::BackRef(tgt)) => {
                let left = fwds.contains(&v2.0);
                let right = *tgt == v1.0;

                match (left, right) {
                    (true, true) => {
                        return Ok(&self.constraints[v1.0]);
                    }
                    (true, false) | (false, true) => {
                        unreachable!(
                            "mismatched forward- and back-references for {v1} ({a1:?}) and {v2} ({a2:?})"
                        )
                    }
                    (false, false) => (),
                }

                let ix1 = v1.0;
                let ix2 = *tgt;

                // check not the actual indices, but the canonical indices for tie-breaking
                if ix1 < ix2 {
                    unsafe { self.recanonicalize(ix1, ix2) }
                } else {
                    unsafe { self.recanonicalize(ix2, ix1) }
                }
            }
            (Alias::Canonical(_), Alias::Canonical(_)) => {
                if v1 < v2 {
                    unsafe { self.recanonicalize(v1.0, v2.0) }
                } else {
                    unsafe { self.recanonicalize(v2.0, v1.0) }
                }
            }
        }
    }

    unsafe fn recanonicalize(&mut self, a1: usize, a2: usize) -> InferenceResult<&Constraints> {
        let tmp = self.aliases[a2].set_backref(a1);
        let iter = tmp.iter_fwd_refs();
        for a in iter {
            assert!(
                !self.aliases[a1].contains_fwd_ref(a),
                "forward ref of ?{a2} is also a forward_ref of ?{a1}, somehow"
            );
            unsafe { self.repoint(a1, a) };
        }
        self.aliases[a1].add_forward_ref(a2);
        unsafe { self.transfer_constraints(a1, a2) }
    }

    fn unify_utype(&mut self, left: NUType, right: NUType) -> InferenceResult<NUType> {
        match (left, right) {
            (NUType::Var(v1), NUType::Var(v2)) => {
                self.unify_var_pair(v1, v2)?;
                Ok(NUType::Var(Ord::min(v1, v2)))
            }
            (NUType::Var(v), _) => {
                let constraint = Constraint::Equiv(right);
                let after = self.unify_var_constraint(v, constraint)?;
                match after {
                    Constraint::Equiv(t) => Ok(t),
                    Constraint::Encompasses(_) => {
                        unreachable!("equiv should erase encompasses")
                    }
                }
            }
            (_, NUType::Var(v)) => {
                let constraint = Constraint::Equiv(left);
                let after = self.unify_var_constraint(v, constraint)?;
                match after {
                    Constraint::Equiv(t) => Ok(t),
                    Constraint::Encompasses(_) => {
                        unreachable!("equiv should erase encompasses")
                    }
                }
            }
            (NUType::Int(t0), NUType::Int(t1)) => {
                if t0 != t1 {
                    return Err(InferenceError::BadUnification(
                        Constraint::Equiv(left),
                        Constraint::Equiv(right),
                    ));
                }
                Ok(left)
            }
        }
    }

    fn unify_utype_bounds(
        &mut self,
        utype: NUType,
        bounds: &Bounds,
    ) -> InferenceResult<Constraint> {
        match utype {
            NUType::Var(var) => {
                self.unify_var_constraint(var, Constraint::Encompasses(bounds.clone()))
            }
            NUType::Int(int_type) => {
                let IntType::Prim(candidate) = int_type;
                let soluble = bounds
                    .is_encompassed_by(&<PrimInt as Into<MachineRep>>::into(candidate).as_bounds());
                if soluble {
                    Ok(Constraint::Equiv(utype))
                } else {
                    Err(InferenceError::BadUnification(
                        Constraint::Equiv(utype),
                        Constraint::Encompasses(bounds.clone()),
                    ))
                }
            }
        }
    }

    fn unify_var_utype(&mut self, var: NVar, utype: NUType) -> InferenceResult<()> {
        let _ = self.unify_var_constraint(var, Constraint::Equiv(utype))?;
        Ok(())
    }

    fn unify_var_rep(&mut self, var: NVar, rep: NumRep) -> InferenceResult<()> {
        if rep.is_auto() {
            return Ok(());
        }
        let t = NUType::Int(IntType::Prim(PrimInt::try_from(rep).unwrap()));
        self.unify_var_utype(var, t)
    }

    /// Unifies two constraints applying to the same metavariable and returns
    /// the resultant constraint.
    ///
    /// Returns an error if the constraints cannot be unified.
    fn unify_constraint_pair(
        &mut self,
        c1: Constraint,
        c2: Constraint,
    ) -> InferenceResult<Constraint> {
        match (c1, c2) {
            (Constraint::Equiv(t1), Constraint::Equiv(t2)) => {
                if t1 == t2 {
                    Ok(Constraint::Equiv(t1))
                } else {
                    let t0 = self.unify_utype(t1, t2)?;
                    Ok(Constraint::Equiv(t0))
                }
            }
            (Constraint::Equiv(utype), Constraint::Encompasses(bounds))
            | (Constraint::Encompasses(bounds), Constraint::Equiv(utype)) => {
                Ok(self.unify_utype_bounds(utype, &bounds)?)
            }
            (Constraint::Encompasses(bs1), Constraint::Encompasses(bs2)) => {
                let bs0 = bs1.unify(&bs2).into_owned();
                Ok(Constraint::Encompasses(bs0))
            }
        }
    }
}

impl InferenceEngine {
    pub(crate) fn infer_var_expr<'a>(&mut self, e: &Expr) -> InferenceResult<(NVar, NumRep)> {
        let (top_var, top_rep) = match e {
            Expr::NumVar(v_ident) => {
                return Err(InferenceError::UnscopedVariable(v_ident.clone()));
            }
            Expr::Const(typed_const) => {
                let rep = typed_const.get_rep();
                let var = match rep {
                    NumRep::AUTO => {
                        let this_var = self.get_new_nvar();
                        self.unify_var_constraint(
                            this_var,
                            Constraint::Encompasses(Bounds::singleton(
                                typed_const.as_raw_value().clone(),
                            )),
                        )?;
                        this_var
                    }
                    NumRep::U8 => {
                        self.init_var_simple(NUType::Int(IntType::Prim(PrimInt::U8)))?
                            .0
                    }
                    NumRep::U16 => {
                        self.init_var_simple(NUType::Int(IntType::Prim(PrimInt::U16)))?
                            .0
                    }
                    NumRep::U32 => {
                        self.init_var_simple(NUType::Int(IntType::Prim(PrimInt::U32)))?
                            .0
                    }
                    NumRep::U64 => {
                        self.init_var_simple(NUType::Int(IntType::Prim(PrimInt::U64)))?
                            .0
                    }
                    NumRep::I8 => {
                        self.init_var_simple(NUType::Int(IntType::Prim(PrimInt::I8)))?
                            .0
                    }
                    NumRep::I16 => {
                        self.init_var_simple(NUType::Int(IntType::Prim(PrimInt::I16)))?
                            .0
                    }
                    NumRep::I32 => {
                        self.init_var_simple(NUType::Int(IntType::Prim(PrimInt::I32)))?
                            .0
                    }
                    NumRep::I64 => {
                        self.init_var_simple(NUType::Int(IntType::Prim(PrimInt::I64)))?
                            .0
                    }
                };
                (var, rep)
            }
            Expr::BinOp(bin_op, lhs, rhs) => {
                let this_var = self.get_new_nvar();
                let (l_var, l_rep) = self.infer_var_expr(&lhs)?;
                let (r_var, r_rep) = self.infer_var_expr(&rhs)?;
                let cast_rep = bin_op.cast_rep();
                let this_rep = match (l_rep, r_rep) {
                    (NumRep::AUTO, NumRep::AUTO) => {
                        self.unify_var_pair(this_var, l_var)?;
                        self.unify_var_pair(this_var, r_var)?;
                        if let Some(rep) = cast_rep {
                            self.unify_var_rep(this_var, NumRep::Concrete(rep))?;
                            NumRep::Concrete(rep)
                        } else {
                            NumRep::AUTO
                        }
                    }
                    (rep0, rep1) if rep0 == rep1 => {
                        if let Some(rep) = cast_rep {
                            let rep = rep.into();
                            self.unify_var_rep(this_var, rep)?;
                            rep
                        } else {
                            self.unify_var_rep(this_var, rep0)?;
                            rep0
                        }
                    }
                    (rep0, rep1) => {
                        if let Some(rep) = cast_rep {
                            let rep = rep.into();
                            self.unify_var_rep(this_var, rep)?;
                            if l_rep.is_auto() {
                                debug_assert!(!r_rep.is_auto());
                                self.unify_var_pair(this_var, l_var)?;
                            }
                            if r_rep.is_auto() {
                                debug_assert!(!l_rep.is_auto());
                                self.unify_var_pair(this_var, r_var)?;
                            }
                            rep
                        } else {
                            if l_rep.is_auto() {
                                debug_assert!(!r_rep.is_auto());
                                self.unify_var_rep(this_var, rep1)?;
                                self.unify_var_rep(l_var, rep1)?;
                                rep1
                            } else if r_rep.is_auto() {
                                self.unify_var_rep(this_var, rep0)?;
                                self.unify_var_rep(r_var, rep0)?;
                                rep0
                            } else {
                                return Err(InferenceError::Ambiguous);
                            }
                        }
                    }
                };
                (this_var, this_rep)
            }
            Expr::UnaryOp(unary_op, expr) => {
                let this_var = self.get_new_nvar();
                let (inner_var, inner_rep) = self.infer_var_expr(&expr)?;
                let cast_rep = unary_op.cast_rep();
                let this_rep = match inner_rep {
                    NumRep::AUTO => {
                        self.unify_var_pair(this_var, inner_var)?;
                        if let Some(rep) = cast_rep {
                            let rep = rep.into();
                            self.unify_var_rep(this_var, rep)?;
                            rep
                        } else {
                            NumRep::AUTO
                        }
                    }
                    rep0 => {
                        if let Some(rep) = cast_rep {
                            let rep = rep.into();
                            self.unify_var_rep(this_var, rep)?;
                            rep
                        } else {
                            self.unify_var_rep(this_var, rep0)?;
                            rep0
                        }
                    }
                };
                (this_var, this_rep)
            }
            &Expr::Cast(rep, ref expr) => {
                let this_var = self.get_new_nvar();
                let (inner_var, inner_rep) = self.infer_var_expr(&expr)?;
                let rep = rep.into();
                if inner_rep.is_auto() {
                    self.unify_var_rep(inner_var, rep)?;
                }
                self.unify_var_rep(this_var, rep)?;
                (this_var, rep)
            }
        };
        Ok((top_var, top_rep))
    }

    fn to_whnf_vtype(&self, t: NUType) -> InferenceResult<NVType> {
        Ok(match t {
            NUType::Var(v) => {
                let v0 = self.get_canonical(v);
                match &self.constraints[v0.0] {
                    Constraints::Indefinite => {
                        // REVIEW - does this case ever happen in practice?
                        log::info!("var {} is indefinite", v0);
                        NVType::Abstract(v0.into())
                    }
                    Constraints::Invariant(Constraint::Equiv(ut)) => self.to_whnf_vtype(*ut)?,
                    Constraints::Invariant(Constraint::Encompasses(bounds)) => {
                        NVType::Within(bounds.clone())
                    }
                }
            }
            NUType::Int(int_type) => NVType::Int(int_type),
        })
    }

    pub(crate) fn substitute_nvar_nvtype(&self, v: NVar) -> InferenceResult<Option<NVType>> {
        match &self.constraints[self.get_canonical(v).0] {
            Constraints::Indefinite => Ok(None),
            Constraints::Invariant(cx) => Ok(match cx {
                Constraint::Equiv(ut) => Some(self.to_whnf_vtype(*ut)?),
                Constraint::Encompasses(bounds) => Some(NVType::Within(bounds.clone())),
            }),
        }
    }
}

impl InferenceEngine {}

impl InferenceEngine {
    pub fn reify_err(&self, t: NUType) -> InferenceResult<IntType> {
        match t {
            NUType::Var(uv) => {
                let v = self.get_canonical(uv);
                match self.substitute_nvar_nvtype(v)? {
                    Some(t0) => match t0 {
                        NVType::Int(int_type) => Ok(int_type),
                        NVType::Within(bounds) => Constraint::get_unique_solution(
                            &Constraint::Encompasses(bounds.clone()),
                        ),
                        NVType::Abstract(utype) => self.reify_err(utype),
                    },
                    None => match &self.constraints[v.0] {
                        Constraints::Indefinite => Err(InferenceError::UnconstrainedVar(v)),
                        Constraints::Invariant(..) => unreachable!(
                            "only Indefinite constraints should yield None for substitute_uvar_vtype"
                        ),
                    },
                }
            }
            NUType::Int(i) => Ok(i),
        }
    }

    #[inline(always)]
    pub fn reify(&self, t: NUType) -> Option<IntType> {
        self.reify_err(t).ok()
    }
}

#[cfg(any())]
mod __unused {
    use super::*;

    /// Global-dependency object for capturing relevant scope details and
    /// handling UVar-dependencies within `InferenceEngine`.
    #[derive(Debug, Default)]
    pub(crate) struct GlobalDeps {
        /// Mappings extracted from `UScope` associating each `NumVar` ident to the UVar it is bound to
        idents: HashMap<Label, UVar>,
        /// Memoized set of all `UVar`s whose solutions must be known for the numeric-tree to be solvable
        global_vars: OnceCell<BTreeSet<UVar>>,
        /// Incrementally collected store of solutions for `UVar`s
        solutions: RefCell<DepSolutions>,
    }

    impl GlobalDeps {
        /// Constructs a new `GlobalDeps` object.
        ///
        /// Will contain no bindings, variables, or solutions.
        pub fn new() -> Self {
            Self::default()
        }

        /// Returns `true` if this `GlobalDeps` object has any registered dependencies.
        pub fn has_any_deps(&self) -> bool {
            !self.idents.is_empty()
        }

        /// Inserts a new global-variable binding informed by a `UScope` parameter passed in at time-of-inference.
        ///
        /// # Panics
        ///
        /// Will panic if, for any reason, two different UVar values are inserted under
        /// the same label.
        fn insert(&mut self, label: Label, global: UVar) {
            use std::collections::hash_map::Entry;
            let entry = self.idents.entry(label);
            match entry {
                Entry::Occupied(occ) => {
                    let lab = occ.key();
                    let prior = *occ.get();
                    if prior != global {
                        unreachable!(
                            "GlobalDeps::insert: entry for identifier `{lab}` inserted twice with different values: {prior} != {global}"
                        );
                    }
                    return;
                }
                Entry::Vacant(vac) => {
                    vac.insert(global);
                }
            }
        }

        /// Returns (or initializes) a memoized set of all `UVar`s that the numeric-tree being
        /// typechecked depends on.
        pub fn get_all_dependencies(&self) -> &BTreeSet<UVar> {
            self.global_vars.get_or_init(|| {
                let mut vars = BTreeSet::new();
                for (_, var) in self.idents.iter() {
                    vars.insert(*var);
                }
                vars
            })
        }

        /// Returns `true` if there is at least one dependent `UVar` whose solution has not
        /// yet been determined.
        pub fn has_any_unsolved(&self) -> bool {
            let table = self.solutions.borrow();
            let deps = self.get_all_dependencies();

            if table.len() < self.idents.len() {
                // if there are fewer entries in the DepSolutions table than dependencies, we are necessarily missing some
                return true;
            }
            // due to keys being unique, if the number of keys in the table is equal to the number
            // of dependencies, we can assume that the table's keys are equiv

            for (var, sol) in table.iter() {
                if !sol.is_definite() {
                    return true;
                }
                debug_assert!(deps.contains(var));
            }
            return false;
        }

        pub fn get_indefinite_dependencies(&self) -> BTreeSet<UVar> {
            let mut vars = BTreeSet::new();
            let table = self.solutions.borrow();
            for (_, var) in self.idents.iter() {
                if !table.get(var).is_some_and(VarSolution::is_definite) {
                    vars.insert(*var);
                }
            }
            vars
        }

        pub fn update_solutions(&self, solutions: &DepSolutions) {
            let mut table = self.solutions.borrow_mut();
            for (var, solution) in solutions.iter() {
                if !solution.is_definite() {
                    continue;
                }
                if let Some(prior) = table.insert(*var, *solution)
                    && prior.is_definite()
                    && prior != *solution
                {
                    unreachable!(
                        "GlobalDeps::update_solutions: entry for metavariable {var} inserted twice with different values: {prior} != {solution}"
                    );
                }
            }
        }

        /// Performs a lookup for the solution of a metavariable.
        ///
        /// Returns `None` if the metavariable has not yet been solved.
        fn lookup(&self, v: UVar) -> Option<IntType> {
            #[cfg(debug_assertions)]
            {
                let deps = self.get_all_dependencies();
                debug_assert!(deps.contains(&v));
            }
            let table = self.solutions.borrow();
            table.get(&v).copied().and_then(VarSolution::coerce_int)
        }
    }

    impl InferenceEngine {
        pub(crate) fn has_global_deps(&self) -> bool {
            self.global_deps.has_any_deps()
        }

        pub(crate) fn get_dep_vars(&self) -> &BTreeSet<UVar> {
            self.global_deps.get_all_dependencies()
        }

        pub(crate) fn get_unsolved_deps(&self) -> BTreeSet<UVar> {
            self.global_deps.get_indefinite_dependencies()
        }

        pub(crate) fn has_unsolved_deps(&self) -> bool {
            self.global_deps.has_any_unsolved()
        }

        pub(crate) fn update_solutions(&self, solutions: &DepSolutions) {
            self.global_deps.update_solutions(solutions);
        }
    }
}
