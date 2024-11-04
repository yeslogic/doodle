use crate::core::{BinOp, Expr, NumRep, TypedConst, UnaryOp};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub(crate) enum PrimInt {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

pub(crate) const PRIM_INTS: [PrimInt; 8] = [
    PrimInt::U8,
    PrimInt::U16,
    PrimInt::U32,
    PrimInt::U64,
    PrimInt::I8,
    PrimInt::I16,
    PrimInt::I32,
    PrimInt::I64,
];

#[derive(Debug)]
pub struct TryFromAutoError;

impl std::fmt::Display for TryFromAutoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot convert `NumRep::AUTO` to `PrimInt`")
    }
}

impl std::error::Error for TryFromAutoError {}

impl TryFrom<NumRep> for PrimInt {
    type Error = TryFromAutoError;

    fn try_from(value: NumRep) -> Result<Self, Self::Error> {
        match value {
            NumRep::Auto => Err(TryFromAutoError),
            NumRep::U8 => Ok(PrimInt::U8),
            NumRep::U16 => Ok(PrimInt::U16),
            NumRep::U32 => Ok(PrimInt::U32),
            NumRep::U64 => Ok(PrimInt::U64),
            NumRep::I8 => Ok(PrimInt::I8),
            NumRep::I16 => Ok(PrimInt::I16),
            NumRep::I32 => Ok(PrimInt::I32),
            NumRep::I64 => Ok(PrimInt::I64),
        }
    }
}

impl From<IntType> for NumRep {
    fn from(value: IntType) -> Self {
        match value {
            IntType::Prim(prim) => NumRep::from(prim),
        }
    }
}

impl From<PrimInt> for NumRep {
    fn from(value: PrimInt) -> Self {
        match value {
            PrimInt::U8 => NumRep::U8,
            PrimInt::U16 => NumRep::U16,
            PrimInt::U32 => NumRep::U32,
            PrimInt::U64 => NumRep::U64,
            PrimInt::I8 => NumRep::I8,
            PrimInt::I16 => NumRep::I16,
            PrimInt::I32 => NumRep::I32,
            PrimInt::I64 => NumRep::I64,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum IntType {
    Prim(PrimInt),
}

impl IntType {
    pub const fn to_static_str(self) -> &'static str {
        match self {
            IntType::Prim(PrimInt::U8) => "u8",
            IntType::Prim(PrimInt::U16) => "u16",
            IntType::Prim(PrimInt::U32) => "u32",
            IntType::Prim(PrimInt::U64) => "u64",
            IntType::Prim(PrimInt::I8) => "i8",
            IntType::Prim(PrimInt::I16) => "i16",
            IntType::Prim(PrimInt::I32) => "i32",
            IntType::Prim(PrimInt::I64) => "i64",
        }
    }
}

impl std::fmt::Display for IntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}

#[derive(Clone, Debug)]
pub(crate) enum TypedExpr<TypeRep> {
    ElabConst(TypeRep, TypedConst),
    ElabBinOp(
        TypeRep,
        TypedBinOp<TypeRep>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
    ),
    ElabUnaryOp(TypeRep, TypedUnaryOp<TypeRep>, Box<TypedExpr<TypeRep>>),
    ElabCast(TypeRep, NumRep, Box<TypedExpr<TypeRep>>),
}

impl<T> TypedExpr<T> {
    pub fn get_type(&self) -> &T {
        match self {
            TypedExpr::ElabConst(t, _) => t,
            TypedExpr::ElabBinOp(t, _, _, _) => t,
            TypedExpr::ElabUnaryOp(t, _, _) => t,
            TypedExpr::ElabCast(t, _, _) => t,
        }
    }
}

type Sig1<T> = (T, T);
type Sig2<T> = ((T, T), T);

#[derive(Clone, Debug)]
pub(crate) struct TypedBinOp<TypeRep> {
    pub(crate) sig: Sig2<TypeRep>,
    pub(crate) inner: BinOp,
}

#[derive(Clone, Debug)]
pub(crate) struct TypedUnaryOp<TypeRep> {
    pub(crate) sig: Sig1<TypeRep>,
    pub(crate) inner: UnaryOp,
}

pub(crate) mod inference {
    use std::collections::HashSet;

    use crate::core::{Bounds, Expr, NumRep};

    use super::{IntType, PrimInt};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct UVar(usize);

    impl std::fmt::Display for UVar {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "?{}", self.0)
        }
    }

    impl UVar {
        pub fn new(ix: usize) -> Self {
            Self(ix)
        }

        // pub fn to_usize(self) -> usize {
        //     self.0
        // }
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub enum UType {
        Var(UVar),
        Int(IntType),
    }

    impl From<IntType> for UType {
        fn from(value: IntType) -> Self {
            UType::Int(value)
        }
    }

    impl From<UVar> for UType {
        fn from(value: UVar) -> Self {
            UType::Var(value)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(crate) enum VType {
        Int(IntType),
        Within(Bounds),
        Abstract(UType),
    }

    #[derive(Clone, Debug, Default)]
    enum Alias {
        #[default]
        Ground,
        BackRef(usize),
        Canonical(HashSet<usize>),
    }

    impl Alias {
        pub const fn new() -> Alias {
            Alias::Ground
        }

        pub fn is_canonical_nonempty(&self) -> bool {
            match self {
                Alias::Canonical(x) => !x.is_empty(),
                _ => false,
            }
        }

        // pub fn as_backref(&self) -> Option<usize> {
        //     match self {
        //         Alias::Ground | Alias::Canonical(_) => None,
        //         Alias::BackRef(ix) => Some(*ix),
        //     }
        // }

        pub fn add_forward_ref(&mut self, tgt: usize) {
            match self {
                Alias::Ground => {
                    let _ = std::mem::replace(self, Alias::Canonical(HashSet::from([tgt])));
                }
                Alias::BackRef(_ix) => panic!("cannot add forward-ref to Alias::BackRef"),
                Alias::Canonical(fwds) => {
                    fwds.insert(tgt);
                }
            }
        }

        fn set_backref(&mut self, tgt: usize) -> Alias {
            std::mem::replace(self, Alias::BackRef(tgt))
        }

        fn iter_fwd_refs<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
            match self {
                Alias::Ground | Alias::BackRef(_) => Box::new(std::iter::empty()),
                Alias::Canonical(fwds) => Box::new(fwds.iter().copied()),
            }
        }

        fn contains_fwd_ref(&self, tgt: usize) -> bool {
            match self {
                Alias::Ground | Alias::BackRef(_) => false,
                Alias::Canonical(fwds) => fwds.contains(&tgt),
            }
        }
    }

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

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Constraint {
        Equiv(UType),
        // Must be able to represent all values within the specified range
        Encompasses(crate::core::Bounds),
    }

    impl std::fmt::Display for Constraint {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Constraint::Equiv(utype) => write!(f, "= {utype:?}"),
                Constraint::Encompasses(bounds) => write!(f, "∈ {}", bounds),
            }
        }
    }

    impl From<UType> for Constraint {
        fn from(value: UType) -> Self {
            Constraint::Equiv(value)
        }
    }

    impl Constraint {
        /// Speculatively checks if this constraint is definiitely satisfiable (as-is) by a given type-assignment.
        ///
        /// If this is not statically deterministic, returns `None`.
        /// Returns `Some(true)` if the constraint is satisfiable by the assignment, and `Some(false)` otherwise.
        pub(crate) fn is_satisfied_by(&self, candidate: IntType) -> Option<bool> {
            match self {
                Constraint::Equiv(utype) => match utype {
                    UType::Var(_) => None,
                    UType::Int(int_type) => Some(int_type == &candidate),
                },
                Constraint::Encompasses(bounds) => {
                    let IntType::Prim(candidate) = candidate;
                    Some(
                        bounds.is_encompassed_by(
                            &<PrimInt as Into<NumRep>>::into(candidate)
                                .as_bounds()
                                .unwrap(),
                        ),
                    )
                }
            }
        }

        // pub(crate) fn has_unique_assignment(&self) -> bool {
        //     // REVIEW - there are smarter ways of calculating this
        //     let mut solutions = 0;
        //     for prim_int in super::PRIM_INTS.iter() {
        //         match self.is_satisfied_by(IntType::Prim(*prim_int)) {
        //             Some(true) => {
        //                 solutions += 1;
        //             }
        //             Some(false) => (),
        //             None => return false,
        //         }
        //     }
        //     solutions == 1
        // }

        // NOTE - should only be called on Encompasses
        pub(crate) fn get_unique_solution(&self) -> InferenceResult<IntType> {
            // REVIEW - there are smarter ways of calculating this
            let mut solutions = Vec::with_capacity(8);
            for prim_int in super::PRIM_INTS.iter() {
                match self.is_satisfied_by(IntType::Prim(*prim_int)) {
                    Some(true) => {
                        solutions.push(*prim_int);
                    }
                    Some(false) => (),
                    None => panic!("unexpected call to get_unique_solution on `{self}` (either trivial or insoluble)"),
                }
            }
            match solutions.as_slice() {
                [] => Err(InferenceError::NoSolution),
                [uniq] => Ok(IntType::Prim(*uniq)),
                _ => Err(InferenceError::MultipleSolutions),
            }
        }
    }

    #[derive(Debug)]
    pub struct InferenceEngine {
        constraints: Vec<Constraints>,
        aliases: Vec<Alias>,
    }

    #[derive(Debug)]
    pub enum InferenceError {
        // Unrepresentable(TypedConst, IntType),
        BadUnification(Constraint, Constraint),
        AbstractCast,
        Ambiguous,
        NoSolution,
        MultipleSolutions,
        Eval(crate::core::EvalError),
    }

    impl std::fmt::Display for InferenceError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                // InferenceError::Unrepresentable(c, int_type) => write!(f, "inference requires that `{}` be assigned type `{}`, which cannot represent it", c, int_type),
                InferenceError::BadUnification(cx1, cx2) => write!(f, "constraints `{}` and `{}` cannot be unified", cx1, cx2),
                InferenceError::AbstractCast => write!(f, "casts and operations cannot explicitly produce abstract NumReps"),
                InferenceError::Ambiguous => write!(f, "mixed-type binary operation must have out_rep on operation to avoid ambiguity"),
                InferenceError::Eval(e) => write!(f, "inference abandoned due to evaluation error: {}", e),
                InferenceError::NoSolution => write!(f, "no valid assignment of PrimInt types produce a fully representable tree"),
                InferenceError::MultipleSolutions => write!(f, "multiple assignments of PrimInt produce a fully representable tree, in absence of tie-breaking mechanism"),
            }
        }
    }

    impl std::error::Error for InferenceError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                InferenceError::Eval(e) => Some(e),
                _ => None,
            }
        }
    }

    pub type InferenceResult<T> = Result<T, InferenceError>;

    impl InferenceEngine {
        pub fn new() -> Self {
            Self {
                constraints: Vec::new(),
                aliases: Vec::new(),
            }
        }

        pub fn init_var_simple(&mut self, typ: UType) -> InferenceResult<(UVar, UType)> {
            let newvar = self.get_new_uvar();
            let constr = Constraint::Equiv(typ);
            self.unify_var_constraint(newvar, constr)?;
            Ok((newvar, typ))
        }

        fn get_new_uvar(&mut self) -> UVar {
            let ret = UVar(self.constraints.len());
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
        fn get_canonical_uvar(&self, v: UVar) -> UVar {
            match self.aliases[v.0] {
                Alias::Canonical(_) | Alias::Ground => v,
                Alias::BackRef(ix) => UVar(ix),
            }
        }

        fn unify_var_constraint(
            &mut self,
            uvar: UVar,
            constraint: Constraint,
        ) -> InferenceResult<Constraint> {
            let can_ix = self.get_canonical_uvar(uvar).0;

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

        unsafe fn repoint(&mut self, lo: usize, hi: usize) {
            self.aliases[hi].set_backref(lo);
            self.aliases[lo].add_forward_ref(hi);
        }

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
                    let _ =
                        self.replace_constraints_with_value(a1, Constraints::Invariant(c0.clone()));
                    let _ = self.replace_constraints_with_value(a2, Constraints::Invariant(c0));
                    Ok(&self.constraints[a1])
                }
            }
        }

        #[must_use]
        fn replace_constraints_from_index(&mut self, tgt_ix: usize, src_ix: usize) -> &Constraints {
            assert_ne!(tgt_ix, src_ix);
            let val = self.constraints[src_ix].clone();
            self.replace_constraints_with_value(tgt_ix, val)
        }

        #[must_use]
        fn replace_constraints_with_value(&mut self, ix: usize, val: Constraints) -> &Constraints {
            self.constraints[ix] = val;
            &self.constraints[ix]
        }

        fn unify_var_pair(&mut self, v1: UVar, v2: UVar) -> InferenceResult<&Constraints> {
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
                self.repoint(a1, a);
            }
            self.aliases[a1].add_forward_ref(a2);
            self.transfer_constraints(a1, a2)
        }

        fn unify_utype(&mut self, left: UType, right: UType) -> InferenceResult<UType> {
            match (left, right) {
                (UType::Var(v1), UType::Var(v2)) => {
                    self.unify_var_pair(v1, v2)?;
                    Ok(UType::Var(Ord::min(v1, v2)))
                }
                (UType::Var(v), _) => {
                    let constraint = Constraint::Equiv(right);
                    let after = self.unify_var_constraint(v, constraint)?;
                    match after {
                        Constraint::Equiv(t) => Ok(t),
                        Constraint::Encompasses(_) => {
                            unreachable!("equiv should erase encompasses")
                        }
                    }
                }
                (_, UType::Var(v)) => {
                    let constraint = Constraint::Equiv(left);
                    let after = self.unify_var_constraint(v, constraint)?;
                    match after {
                        Constraint::Equiv(t) => Ok(t),
                        Constraint::Encompasses(_) => {
                            unreachable!("equiv should erase encompasses")
                        }
                    }
                }
                (UType::Int(t0), UType::Int(t1)) => {
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
            utype: UType,
            bounds: &Bounds,
        ) -> InferenceResult<Constraint> {
            match utype {
                UType::Var(uvar) => {
                    self.unify_var_constraint(uvar, Constraint::Encompasses(bounds.clone()))
                }
                UType::Int(int_type) => {
                    let IntType::Prim(candidate) = int_type;
                    let soluble = bounds.is_encompassed_by(
                        &<PrimInt as Into<NumRep>>::into(candidate)
                            .as_bounds()
                            .unwrap(),
                    );
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

        fn unify_var_utype(&mut self, uvar: UVar, utype: UType) -> InferenceResult<()> {
            let _ = self.unify_var_constraint(uvar, Constraint::Equiv(utype))?;
            Ok(())
        }

        fn unify_var_rep(&mut self, uvar: UVar, rep: NumRep) -> InferenceResult<()> {
            if rep.is_auto() {
                return Ok(());
            }
            let t = UType::Int(IntType::Prim(PrimInt::try_from(rep).unwrap()));
            self.unify_var_utype(uvar, t)
        }

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
        pub(crate) fn infer_var_expr(&mut self, e: &Expr) -> InferenceResult<(UVar, NumRep)> {
            let (top_var, top_rep) = match e {
                Expr::Const(typed_const) => {
                    let rep = typed_const.get_rep();
                    let var = match rep {
                        NumRep::AUTO => {
                            let this_var = self.get_new_uvar();
                            self.unify_var_constraint(
                                this_var,
                                Constraint::Encompasses(Bounds::singleton(
                                    typed_const.as_raw_value().clone(),
                                )),
                            )?;
                            this_var
                        }
                        NumRep::U8 => {
                            self.init_var_simple(UType::Int(IntType::Prim(PrimInt::U8)))?
                                .0
                        }
                        NumRep::U16 => {
                            self.init_var_simple(UType::Int(IntType::Prim(PrimInt::U16)))?
                                .0
                        }
                        NumRep::U32 => {
                            self.init_var_simple(UType::Int(IntType::Prim(PrimInt::U32)))?
                                .0
                        }
                        NumRep::U64 => {
                            self.init_var_simple(UType::Int(IntType::Prim(PrimInt::U64)))?
                                .0
                        }
                        NumRep::I8 => {
                            self.init_var_simple(UType::Int(IntType::Prim(PrimInt::I8)))?
                                .0
                        }
                        NumRep::I16 => {
                            self.init_var_simple(UType::Int(IntType::Prim(PrimInt::I16)))?
                                .0
                        }
                        NumRep::I32 => {
                            self.init_var_simple(UType::Int(IntType::Prim(PrimInt::I32)))?
                                .0
                        }
                        NumRep::I64 => {
                            self.init_var_simple(UType::Int(IntType::Prim(PrimInt::I64)))?
                                .0
                        }
                    };
                    (var, rep)
                }
                Expr::BinOp(bin_op, lhs, rhs) => {
                    let this_var = self.get_new_uvar();
                    let (l_var, l_rep) = self.infer_var_expr(&lhs)?;
                    let (r_var, r_rep) = self.infer_var_expr(&rhs)?;
                    if bin_op.is_cast_and(NumRep::is_auto) {
                        return Err(InferenceError::AbstractCast);
                    }
                    let cast_rep = bin_op.cast_rep();
                    let this_rep = match (l_rep, r_rep) {
                        (NumRep::AUTO, NumRep::AUTO) => {
                            self.unify_var_pair(this_var, l_var)?;
                            self.unify_var_pair(this_var, r_var)?;
                            if let Some(rep) = cast_rep {
                                self.unify_var_rep(this_var, rep)?;
                                rep
                            } else {
                                {
                                    // REVIEW - do we need to go this far?
                                    match e.eval() {
                                        Ok(v) => match v.as_const() {
                                            Some(c) => {
                                                // NOTE - if there is a const-evaluable result for the computation, use it to refine our constraints on which types satisfy the aliased Auto
                                                let bounds =
                                                    Bounds::singleton(c.as_raw_value().clone());
                                                self.unify_var_constraint(
                                                    this_var,
                                                    Constraint::Encompasses(bounds),
                                                )?;
                                            }
                                            None => {
                                                // FIXME - this isn't a hard error necessarily, but our model isn't complex enough for non-TypedConst values to emerge
                                                unimplemented!("Value::AsConst returned None (unexpectedly) when called from InferenceEngine::infer_var_expr");
                                            }
                                        },
                                        Err(e) => {
                                            // NOTE - If the computation will fail regardless, there is no need to infer the type-information of the AST
                                            // REVIEW - make sure that we are confident in EvalErrors being sound reasons to fail type-inference, both now and going forward
                                            return Err(InferenceError::Eval(e));
                                        }
                                    }
                                }
                                NumRep::AUTO
                            }
                        }
                        (rep0, rep1) if rep0 == rep1 => {
                            if let Some(rep) = cast_rep {
                                self.unify_var_rep(this_var, rep)?;
                                rep
                            } else {
                                self.unify_var_rep(this_var, rep0)?;
                                rep0
                            }
                        }
                        (rep0, rep1) => {
                            if let Some(rep) = cast_rep {
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
                    let this_var = self.get_new_uvar();
                    let (inner_var, inner_rep) = self.infer_var_expr(&expr)?;
                    if unary_op.is_cast_and(NumRep::is_auto) {
                        return Err(InferenceError::AbstractCast);
                    }
                    let cast_rep = unary_op.cast_rep();
                    let this_rep = match inner_rep {
                        NumRep::AUTO => {
                            self.unify_var_pair(this_var, inner_var)?;
                            if let Some(rep) = cast_rep {
                                self.unify_var_rep(this_var, rep)?;
                                rep
                            } else {
                                {
                                    // REVIEW - do we need to go this far?
                                    match e.eval() {
                                        Ok(v) => match v.as_const() {
                                            Some(c) => {
                                                // NOTE - if there is a const-evaluable result for the computation, use it to refine our constraints on which types satisfy the aliased Auto
                                                let bounds =
                                                    Bounds::singleton(c.as_raw_value().clone());
                                                self.unify_var_constraint(
                                                    this_var,
                                                    Constraint::Encompasses(bounds),
                                                )?;
                                            }
                                            None => {
                                                // FIXME - this isn't a hard error necessarily, but our model isn't complex enough for non-TypedConst values to emerge
                                                unimplemented!("Value::AsConst returned None (unexpectedly) when called from InferenceEngine::infer_var_expr");
                                            }
                                        },
                                        Err(e) => {
                                            // NOTE - If the computation will fail regardless, there is no need to infer the type-information of the AST
                                            // REVIEW - make sure that we are confident in EvalErrors being sound reasons to fail type-inference, both now and going forward
                                            return Err(InferenceError::Eval(e));
                                        }
                                    }
                                }
                                NumRep::AUTO
                            }
                        }
                        rep0 => {
                            if let Some(rep) = cast_rep {
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
                Expr::Cast(rep, expr) => {
                    let this_var = self.get_new_uvar();
                    let (inner_var, inner_rep) = self.infer_var_expr(&expr)?;
                    if rep.is_auto() {
                        return Err(InferenceError::AbstractCast);
                    }
                    if inner_rep.is_auto() {
                        self.unify_var_rep(inner_var, *rep)?;
                    }
                    self.unify_var_rep(this_var, *rep)?;
                    (this_var, *rep)
                }
            };
            Ok((top_var, top_rep))
        }

        fn to_whnf_vtype(&self, t: UType) -> VType {
            match t {
                UType::Var(v) => {
                    let v0 = self.get_canonical_uvar(v);
                    match &self.constraints[v0.0] {
                        Constraints::Indefinite => VType::Abstract(v0.into()),
                        Constraints::Invariant(Constraint::Equiv(ut)) => self.to_whnf_vtype(*ut),
                        Constraints::Invariant(Constraint::Encompasses(bounds)) => {
                            VType::Within(bounds.clone())
                        }
                    }
                }
                UType::Int(int_type) => VType::Int(int_type),
            }
        }

        pub(crate) fn substitute_uvar_vtype(&self, v: UVar) -> InferenceResult<Option<VType>> {
            match &self.constraints[self.get_canonical_uvar(v).0] {
                Constraints::Indefinite => Ok(None),
                Constraints::Invariant(cx) => Ok(match cx {
                    Constraint::Equiv(ut) => Some(self.to_whnf_vtype(*ut)),
                    Constraint::Encompasses(bounds) => Some(VType::Within(bounds.clone())),
                }),
            }
        }
    }

    impl InferenceEngine {
        pub fn reify(&self, t: UType) -> Option<IntType> {
            match t {
                UType::Var(uv) => {
                    let v = self.get_canonical_uvar(uv);
                    match self.substitute_uvar_vtype(v) {
                        Ok(Some(t0)) => match t0 {
                            VType::Int(int_type) => Some(int_type),
                            VType::Within(bounds) => match Constraint::get_unique_solution(
                                &Constraint::Encompasses(bounds.clone()),
                            ) {
                                Ok(int_type) => Some(int_type),
                                Err(_) => None,
                            },
                            VType::Abstract(utype) => self.reify(utype),
                        },
                        Err(_) => None,
                        Ok(None) => match &self.constraints[v.0] {
                            _ => None,
                        },
                    }
                }
                UType::Int(i) => Some(i),
            }
        }
    }
}

use inference::{InferenceEngine, UVar};

/// Alias for whatever value-type we use to associate a failed reification with some indication of what went wrong, or where
type Hint = usize;

#[derive(Debug)]
pub enum ElaborationError {
    BadReification(Hint),
}

impl std::fmt::Display for ElaborationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElaborationError::BadReification(hint) => {
                write!(f, "bad reification on UVar ?{}", hint)
            }
        }
    }
}

impl std::error::Error for ElaborationError {}

pub(crate) type ElaborationResult<T> = Result<T, ElaborationError>;

pub struct Elaborator {
    next_index: usize,
    ie: InferenceEngine,
}

impl Elaborator {
    pub(crate) fn new(ie: InferenceEngine) -> Self {
        Self { next_index: 0, ie }
    }

    fn get_and_increment_index(&mut self) -> usize {
        let ret = self.next_index;
        self.next_index += 1;
        ret
    }

    fn get_type_from_index(&self, index: usize) -> ElaborationResult<IntType> {
        let uvar = UVar::new(index);
        let Some(t) = self.ie.reify(uvar.into()) else {
            return Err(ElaborationError::BadReification(index));
        };
        Ok(t)
    }

    pub(crate) fn elaborate_expr(&mut self, expr: &Expr) -> ElaborationResult<TypedExpr<IntType>> {
        let index = self.get_and_increment_index();
        match expr {
            Expr::Const(typed_const) => {
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabConst(t, typed_const.clone()))
            }
            Expr::BinOp(bin_op, x, y) => {
                let t_x = self.elaborate_expr(x)?;
                let t_y = self.elaborate_expr(y)?;
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabBinOp(
                    t,
                    TypedBinOp {
                        sig: ((*t_x.get_type(), *t_y.get_type()), t),
                        inner: *bin_op,
                    },
                    Box::new(t_x),
                    Box::new(t_y),
                ))
            }
            Expr::UnaryOp(unary_op, inner) => {
                let t_inner = self.elaborate_expr(inner)?;
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabUnaryOp(
                    t,
                    TypedUnaryOp {
                        sig: (*t_inner.get_type(), t),
                        inner: *unary_op,
                    },
                    Box::new(t_inner),
                ))
            }
            Expr::Cast(rep, inner) => {
                let t_inner = self.elaborate_expr(inner)?;
                let t = self.get_type_from_index(index)?;
                Ok(TypedExpr::ElabCast(t, *rep, Box::new(t_inner)))
            }
        }
    }
}
