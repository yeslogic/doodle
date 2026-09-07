//! Prototype microcosm of real doodle's `src/typecheck.rs` bidirectional-unification engine
//! (`UVar`/`UType` over a `Vec`-arena, with union-find aliasing) - built to work out ONE specific
//! design decision before touching the real ~4500-line file, per `TYPECHECKER.md`'s own
//! description of that engine's shape ("assigning unification-metavariables (`UVar`s) to each
//! node in the tree").
//!
//! Real doodle's occurs-check (`src/typecheck.rs::occurs_in`) is a deliberate, fully-working
//! fence against ANY self-referential type, with exactly one exemption: `UType::PhantomData`,
//! which is safe specifically because `Format::Phantom` is *guaranteed to never actually be
//! decoded* (see the `phantom_rec_*` regression tests in `src/codegen/mod.rs`, and the COLR paint
//! graph, which reads its own recursive structure out-of-band via explicit offset navigation
//! rather than by the `Format` engine actually recursing through bytes). That's a "this subtree
//! is inert, don't look inside it" exemption - it doesn't (and can't) generalize to a
//! self-reference that IS meant to be decoded at runtime, which is exactly what `doodle-rec`'s
//! `RecVar`/batch mechanism produces.
//!
//! The soundness question this module exists to answer is a genuinely different one from "is
//! this subtree ever walked": **is this self-reference expressible as a *finite Rust type*?**
//! That's exactly Rust's own E0072 rule (`error: recursive type X has infinite size`) - a
//! recursive type is legal precisely when every cycle passes through at least one indirection
//! (a `Tuple` element, a `Union` variant, a `Seq`/`Option` payload - the positions
//! `codegen.rs::rust_type` already `Box`-wraps at a `FormatType::Ref`). A direct, un-indirected
//! self-alias (`RecVar` as a format's *entire* own body, with nothing wrapping it) has no finite
//! representation, `Box` or no `Box` - matching how `type X = Box<X>;` is illegal in Rust even
//! though `enum X { Y(Box<X>) }` is fine.
//!
//! **This is conceptually a different axis from grammar-level left-recursion**, even though the
//! two turn out to be hard to pry apart empirically within `doodle-rec`'s current grammar.
//! `solve_determinations`'s `Traversal`/`Entry::LeftRecursive` (see `matchtree/determinations.rs`)
//! asks "does going around this cycle consume at least one byte" - a *decoder-termination*
//! question - while this module asks "can this cycle be laid out as a finite Rust type" - a
//! *type-representability* question (matching Rust's own distinction: `enum X { Y(Box<X>) }` with
//! no base case is a perfectly legal, if uninhabited, type - E0072 cares about finiteness, not
//! termination or inhabitedness). They're independent questions in principle, but two attempts to
//! find a `doodle-rec` construct that's grammar-safe-yet-type-unsafe (`Format::Slice(1,
//! Format::RecVar(0))` as a batch's whole body, and `Format::Tuple([Format::RecVar(0),
//! Format::Byte(..)])`, self-reference first) were BOTH independently rejected by the existing
//! left-recursion checker before reaching this module at all (verified empirically, not just
//! assumed - every currently-expressible way of reaching a `RecVar` without prior byte consumption
//! also happens to fail `solve_determinations`, whether or not it's wrapped in a `Tuple`/`Union`).
//! So today, within this grammar, this module doesn't yet have a working counterexample proving
//! its check catches something `solve_determinations` doesn't gate the SAME construct against
//! first - only that it catches what `infer_type` (the OLD, ungated type layer) misses. Whether a
//! genuine grammar-safe/type-unsafe split exists once `View`-style indirection or richer
//! constructs land is an open question, not a settled one - noted here rather than overclaimed.
//!
//! `doodle-rec`'s CURRENT `FormatType`/`infer_type` (`lib.rs`) doesn't attempt this distinction
//! at all - `Format::ItemVar`/`RecVar`'s arms unconditionally return `FormatType::Ref(level)` the
//! moment `visited` already contains the target level, regardless of whether anything wraps that
//! reference. It would silently "type" a bare `Format::RecVar(0)` (a batch's *entire* body) as
//! `Ref(0)`, an unrepresentable type. See the `direct_...` tests below for the empirical
//! before/after.
//!
//! Deliberately a NEW, separate module: does not touch `FormatType`/`infer_type`/`Compiler`/
//! `LL1Interpreter`/`codegen.rs` or their existing tests. `infer_module` is a standalone entry
//! point exercised only by this module's own tests.

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    rc::Rc,
};

use anyhow::{Result as AResult, anyhow};

use crate::{BaseType, Format, FormatId, FormatModule, FormatType, Label, RecurseCtx, TypeShape};

pub type UVar = usize;

/// Mirrors real doodle's `UType`, scaled down to what `doodle-rec`'s grammar needs. One
/// deliberate simplification vs. the real engine: every child position (`Tuple` element, `Union`
/// variant, `Seq`/`Option` payload) is always `Var`-wrapped, one fresh `UVar` per `Format` AST
/// node - real doodle's `UType` allows a mix of `Var` and directly-embedded concrete shapes
/// (an optimization), but always-`Var` is a valid, simpler special case of the same design and
/// keeps unification uniform (every merge is a `UVar`-to-`UVar` operation).
#[derive(Debug, Clone)]
pub enum UType {
    /// Unconstrained placeholder - the initial state of every freshly allocated `UVar`.
    Hole,
    /// Forwarding reference to another arena slot.
    Var(UVar),
    /// Uninhabited (`Format::FailWith` / an empty `Format::Byte`) - same role as
    /// `FormatType::Void` in the existing `lib.rs` model.
    Void,
    Base(BaseType),
    Tuple(Vec<Rc<UType>>),
    Seq(Rc<UType>),
    Option(Rc<UType>),
    Union(BTreeMap<Label, Rc<UType>>),
}

pub struct TypeChecker<'a> {
    module: &'a FormatModule,
    /// `constraints[v]` is only meaningful when `v` is its own union-find root (`parent[v] ==
    /// None`) - the current best-known `UType` for that equivalence class.
    constraints: Vec<Rc<UType>>,
    /// Union-find parent pointers (`None` = root/canonical).
    parent: Vec<Option<UVar>>,
    /// `ItemVar`/`RecVar` reference cache, keyed by absolute `FormatId`, shared across the WHOLE
    /// transitive walk from one top-level `Format` - deliberately NOT reset per batch member the
    /// way `declare_rec_formats`'s per-member-fresh `visited: HashSet` is. This mirrors real
    /// doodle's own `level_vars` (`infer_var_format_level`, `src/typecheck.rs:999-1020`), which
    /// persists for one `TypeChecker`/`infer_module` call - and gives proper reference-sharing
    /// for ANY repeated level reference within that call, not just a literal self-cycle (fixing
    /// the asymmetry found during step 6's codegen work, where a non-cyclic sibling reference got
    /// fully structurally inlined instead of shared, purely because `declare_rec_formats` solves
    /// each batch member with its own fresh `visited` set).
    level_vars: HashMap<FormatId, UVar>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(module: &'a FormatModule) -> Self {
        Self {
            module,
            constraints: Vec::new(),
            parent: Vec::new(),
            level_vars: HashMap::new(),
        }
    }

    pub fn get_new_uvar(&mut self) -> UVar {
        let v = self.constraints.len();
        self.constraints.push(Rc::new(UType::Hole));
        self.parent.push(None);
        v
    }

    pub(crate) fn find(&mut self, v: UVar) -> UVar {
        match self.parent[v] {
            None => v,
            Some(p) => {
                let root = self.find(p);
                self.parent[v] = Some(root);
                root
            }
        }
    }

    /// One-step ("weak head") expansion: dereferences `v` to its union-find root and returns the
    /// shape stored there, WITHOUT recursing into child positions (those remain `Var`-wrapped,
    /// to be expanded on demand). This has to stop at one level, not fully substitute - a fully
    /// substituted legitimate recursive type is, definitionally, infinite; real doodle's own
    /// `expand_var`/`Expansion` does the same one-step expansion for the same reason.
    pub fn expand(&mut self, v: UVar) -> Rc<UType> {
        let v = self.find(v);
        self.constraints[v].clone()
    }

    /// Does resolving `v` (transitively, via plain `Var` forwarding only - not through any
    /// `Tuple`/`Seq`/`Option`/`Union` indirection) ever reach `target`? `indirected` tracks
    /// whether an indirection boundary has been crossed since the check for this particular
    /// `target` began; reaching `target` un-indirected is the actual infinite-type violation.
    /// `visited` is keyed by `(uvar, indirected)` and threaded through the WHOLE walk (never
    /// reset at indirection boundaries) so a pre-existing, already-legitimate cycle elsewhere in
    /// the graph (reached while searching for `target`) can't make this loop forever.
    fn occurs_in(
        &mut self,
        target: UVar,
        v: UVar,
        indirected: bool,
        visited: &mut HashSet<(UVar, bool)>,
    ) -> AResult<()> {
        let v = self.find(v);
        let target = self.find(target);
        if v == target {
            return if indirected {
                Ok(())
            } else {
                Err(anyhow!(
                    "infinite type: uvar {target} refers to itself with no Tuple/Seq/Option/Union \
                     indirection anywhere on the cycle - not representable as a finite Rust type"
                ))
            };
        }
        if !visited.insert((v, indirected)) {
            return Ok(());
        }
        let ty = self.constraints[v].clone();
        self.occurs_in_shape(target, &ty, indirected, visited)
    }

    fn occurs_in_shape(
        &mut self,
        target: UVar,
        ty: &UType,
        indirected: bool,
        visited: &mut HashSet<(UVar, bool)>,
    ) -> AResult<()> {
        match ty {
            UType::Hole | UType::Void | UType::Base(_) => Ok(()),
            UType::Var(v) => self.occurs_in(target, *v, indirected, visited),
            UType::Tuple(ts) => {
                for t in ts {
                    self.occurs_in_shape(target, t, true, visited)?;
                }
                Ok(())
            }
            UType::Seq(t) | UType::Option(t) => self.occurs_in_shape(target, t, true, visited),
            UType::Union(vs) => {
                for t in vs.values() {
                    self.occurs_in_shape(target, t, true, visited)?;
                }
                Ok(())
            }
        }
    }

    /// Installs `ty` as `v`'s constraint, occurs-checked first. If `v` already has a non-`Hole`
    /// constraint (only expected via [`Self::unify_var_pair`]'s merge path), the two shapes are
    /// structurally merged instead of overwritten.
    pub fn unify_var_utype(&mut self, v: UVar, ty: Rc<UType>) -> AResult<()> {
        let v = self.find(v);
        self.occurs_in_shape(v, &ty, false, &mut HashSet::new())?;
        let existing = self.constraints[v].clone();
        match &*existing {
            UType::Hole => {
                self.constraints[v] = ty;
                Ok(())
            }
            _ => self.merge_shapes(&existing, &ty),
        }
    }

    /// Unifies two `UVar`s (`a` and `b` denote the same type). Occurs-checks in both directions
    /// before committing, then aliases `b` to `a` and folds `b`'s prior content (if any) in.
    pub fn unify_var_pair(&mut self, a: UVar, b: UVar) -> AResult<UVar> {
        let a = self.find(a);
        let b = self.find(b);
        if a == b {
            return Ok(a);
        }
        self.occurs_in(a, b, false, &mut HashSet::new())?;
        self.occurs_in(b, a, false, &mut HashSet::new())?;
        let b_ty = self.constraints[b].clone();
        self.parent[b] = Some(a);
        if !matches!(&*b_ty, UType::Hole) {
            self.unify_var_utype(a, b_ty)?;
        }
        Ok(a)
    }

    fn merge_shapes(&mut self, a: &UType, b: &UType) -> AResult<()> {
        match (a, b) {
            (UType::Void, UType::Void) => Ok(()),
            (UType::Base(x), UType::Base(y)) if x == y => Ok(()),
            (UType::Tuple(xs), UType::Tuple(ys)) if xs.len() == ys.len() => {
                for (x, y) in xs.iter().zip(ys) {
                    self.merge_rc(x, y)?;
                }
                Ok(())
            }
            (UType::Seq(x), UType::Seq(y)) | (UType::Option(x), UType::Option(y)) => {
                self.merge_rc(x, y)
            }
            (UType::Union(xs), UType::Union(ys)) => {
                for (label, y) in ys {
                    if let Some(x) = xs.get(label) {
                        self.merge_rc(x, y)?;
                    }
                    // A label present only in `ys` is fine to leave out of this merge - callers
                    // that need the fully-grown label set (e.g. `Format::Union`'s own arm) build
                    // it themselves from the union of both sides before merging shared labels.
                }
                Ok(())
            }
            _ => Err(anyhow!("cannot unify incompatible shapes: {a:?} vs {b:?}")),
        }
    }

    fn merge_rc(&mut self, x: &Rc<UType>, y: &Rc<UType>) -> AResult<()> {
        match (&**x, &**y) {
            (UType::Var(vx), UType::Var(vy)) => {
                self.unify_var_pair(*vx, *vy)?;
                Ok(())
            }
            _ => self.merge_shapes(x, y),
        }
    }

    /// Reserve-before-recurse cache for a level reference (`ItemVar(level)`, or `RecVar(ix)`
    /// after resolving `ix` to an absolute level via the ambient `RecurseCtx`). Mirrors real
    /// doodle's `infer_var_format_level` exactly: check the cache, else reserve a fresh
    /// placeholder UVar *before* recursing into the body, then unify the placeholder with
    /// whatever the body actually infers to - any reference to `level` reached WHILE that body is
    /// still being solved (a genuine cycle) sees the placeholder and is occurs-checked against it
    /// in [`Self::unify_var_pair`] once the recursive call returns.
    pub(crate) fn infer_level(&mut self, level: FormatId) -> AResult<UVar> {
        if let Some(v) = self.level_vars.get(&level) {
            return Ok(*v);
        }
        let placeholder = self.get_new_uvar();
        self.level_vars.insert(level, placeholder);
        let body = self.module.get_format(level);
        let body_ctx = self.module.get_ctx(level);
        let result = self.infer_var_format(body, body_ctx)?;
        self.unify_var_pair(placeholder, result)?;
        Ok(placeholder)
    }

    /// Infers a `UVar` for one `Format` AST node under `ctx`. `ItemVar`/`RecVar` delegate
    /// straight to [`Self::infer_level`] (returning the level's own shared `UVar`, no extra
    /// wrapping); every other construct allocates a fresh `UVar` for itself and unifies it with
    /// its structurally-built shape.
    pub fn infer_var_format(&mut self, format: &'a Format, ctx: RecurseCtx<'a>) -> AResult<UVar> {
        match format {
            Format::ItemVar(level) => self.infer_level(*level),
            Format::RecVar(ix) => {
                let level = ctx
                    .convert_rec_var(*ix)
                    .ok_or_else(|| anyhow!("RecVar(~{ix}) outside a recursive context"))?;
                self.infer_level(level)
            }
            _ => {
                let v = self.get_new_uvar();
                let ty = self.infer_format_shape(format, ctx)?;
                self.unify_var_utype(v, ty)?;
                Ok(v)
            }
        }
    }

    fn infer_format_shape(
        &mut self,
        format: &'a Format,
        ctx: RecurseCtx<'a>,
    ) -> AResult<Rc<UType>> {
        match format {
            Format::ItemVar(_) | Format::RecVar(_) => {
                unreachable!("handled directly in infer_var_format")
            }
            Format::FailWith(_) => Ok(Rc::new(UType::Void)),
            Format::EndOfInput => Ok(Rc::new(UType::Tuple(vec![]))),
            Format::Byte(bs) if bs.is_empty() => Ok(Rc::new(UType::Void)),
            Format::Byte(_) => Ok(Rc::new(UType::Base(BaseType::U8))),
            Format::Compute(expr) => Ok(Rc::new(from_format_type(&expr.infer_type()?))),
            Format::Variant(label, inner) => {
                let iv = self.infer_var_format(inner, ctx)?;
                Ok(Rc::new(UType::Union(BTreeMap::from([(
                    label.clone(),
                    Rc::new(UType::Var(iv)),
                )]))))
            }
            Format::Union(branches) | Format::UnionNondet(branches) => {
                let mut merged: BTreeMap<Label, UVar> = BTreeMap::new();
                for f in branches {
                    let bv = self.infer_var_format(f, ctx)?;
                    for (label, uv) in self.expect_union(bv)? {
                        match merged.get(&label) {
                            Some(&existing) => {
                                self.unify_var_pair(existing, uv)?;
                            }
                            None => {
                                merged.insert(label, uv);
                            }
                        }
                    }
                }
                Ok(Rc::new(UType::Union(
                    merged
                        .into_iter()
                        .map(|(l, v)| (l, Rc::new(UType::Var(v))))
                        .collect(),
                )))
            }
            Format::Tuple(elts) => {
                let vs = elts
                    .iter()
                    .map(|f| self.infer_var_format(f, ctx))
                    .collect::<AResult<Vec<_>>>()?;
                Ok(Rc::new(UType::Tuple(
                    vs.into_iter().map(|v| Rc::new(UType::Var(v))).collect(),
                )))
            }
            Format::Seq(elts) => {
                let elem = self.get_new_uvar();
                for f in elts {
                    let fv = self.infer_var_format(f, ctx)?;
                    self.unify_var_pair(elem, fv)?;
                }
                Ok(Rc::new(UType::Seq(Rc::new(UType::Var(elem)))))
            }
            Format::Repeat(inner)
            | Format::RepeatCount(_, inner)
            | Format::RepeatBetween(_, _, inner) => {
                let iv = self.infer_var_format(inner, ctx)?;
                Ok(Rc::new(UType::Seq(Rc::new(UType::Var(iv)))))
            }
            Format::Maybe(_, inner) => {
                let iv = self.infer_var_format(inner, ctx)?;
                Ok(Rc::new(UType::Option(Rc::new(UType::Var(iv)))))
            }
            // Peek/PeekNot/Slice/WithRelativeOffset are all type-transparent (matching
            // `FormatType::infer_type`'s existing delegation) - none of them introduce a `Box`
            // point in codegen, so none of them count as an indirection boundary here either.
            // PeekNot's own type is unit regardless of its target, same as `infer_type`.
            Format::Peek(inner)
            | Format::Slice(_, inner)
            | Format::WithRelativeOffset(_, _, inner) => {
                let iv = self.infer_var_format(inner, ctx)?;
                Ok(Rc::new(UType::Var(iv)))
            }
            Format::PeekNot(_) => Ok(Rc::new(UType::Tuple(vec![]))),
        }
    }

    /// Resolves `v` (one step) and requires it to be a `Union`, returning its label->`UVar` map.
    fn expect_union(&mut self, v: UVar) -> AResult<BTreeMap<Label, UVar>> {
        match &*self.expand(v) {
            UType::Union(m) => m
                .iter()
                .map(|(l, t)| match &**t {
                    UType::Var(uv) => Ok((l.clone(), *uv)),
                    other => Err(anyhow!(
                        "expected a Union's variant to be Var-wrapped, found {other:?}"
                    )),
                })
                .collect(),
            other => Err(anyhow!("expected a Union shape, found {other:?}")),
        }
    }
}

fn from_format_type(ft: &FormatType) -> UType {
    match ft {
        FormatType::Any => UType::Hole,
        FormatType::Void => UType::Void,
        FormatType::Base(b) => UType::Base(*b),
        FormatType::Ref(level) => {
            unreachable!("Expr::infer_type never produces a Ref cycle placeholder (level {level})")
        }
        FormatType::Shape(TypeShape::Tuple(ts)) => {
            UType::Tuple(ts.iter().map(|t| Rc::new(from_format_type(t))).collect())
        }
        FormatType::Shape(TypeShape::Seq(t)) => UType::Seq(Rc::new(from_format_type(t))),
        FormatType::Shape(TypeShape::Option(t)) => UType::Option(Rc::new(from_format_type(t))),
        FormatType::Shape(TypeShape::Union(m)) => UType::Union(
            m.iter()
                .map(|(l, t)| (l.clone(), Rc::new(from_format_type(t))))
                .collect(),
        ),
    }
}

/// Type-checks the format at `top_level` (and everything transitively reachable from it) in one
/// fresh `TypeChecker`, mirroring real doodle's `TypeChecker::infer_module` entry point.
pub fn infer_module<'a>(
    module: &'a FormatModule,
    top_level: FormatId,
) -> AResult<(TypeChecker<'a>, UVar)> {
    let mut tc = TypeChecker::new(module);
    let v = tc.infer_level(top_level)?;
    Ok((tc, v))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Label;
    use doodle::byte_set::ByteSet;

    // --- Direct engine-level tests: the core occurs-check claim, isolated from grammar-level
    // left-recursion concerns (declare_rec_formats's own eager checks would otherwise reject a
    // trivial un-indirected self-reference for an unrelated reason - see the module doc comment).

    #[test]
    fn direct_self_alias_with_no_indirection_is_rejected() {
        let module = FormatModule::new();
        let mut tc = TypeChecker::new(&module);
        let v = tc.get_new_uvar();
        let err = tc.unify_var_utype(v, Rc::new(UType::Var(v))).expect_err(
            "a bare self-alias with no Tuple/Seq/Option/Union in between must be rejected",
        );
        assert!(
            err.to_string().contains("infinite type"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn self_reference_behind_a_tuple_is_accepted() {
        let module = FormatModule::new();
        let mut tc = TypeChecker::new(&module);
        let v = tc.get_new_uvar();
        tc.unify_var_utype(v, Rc::new(UType::Tuple(vec![Rc::new(UType::Var(v))])))
            .expect("a self-reference indirected through a Tuple must be accepted");
    }

    #[test]
    fn self_reference_behind_a_union_variant_is_accepted() {
        let module = FormatModule::new();
        let mut tc = TypeChecker::new(&module);
        let v = tc.get_new_uvar();
        let mut m = BTreeMap::new();
        m.insert(Label::Borrowed("S"), Rc::new(UType::Var(v)));
        tc.unify_var_utype(v, Rc::new(UType::Union(m)))
            .expect("a self-reference indirected through a Union variant must be accepted");
    }

    #[test]
    fn an_unrelated_pre_existing_cycle_does_not_hang_occurs_check() {
        // w1 <-> w2 form their own legitimate cycle (each behind a Tuple), unrelated to `target`.
        // Checking whether `target` (fresh, unconstrained) occurs in w1's expansion must
        // terminate, not loop forever chasing the w1/w2 cycle.
        let module = FormatModule::new();
        let mut tc = TypeChecker::new(&module);
        let w1 = tc.get_new_uvar();
        let w2 = tc.get_new_uvar();
        tc.unify_var_utype(w1, Rc::new(UType::Tuple(vec![Rc::new(UType::Var(w2))])))
            .unwrap();
        tc.unify_var_utype(w2, Rc::new(UType::Tuple(vec![Rc::new(UType::Var(w1))])))
            .unwrap();
        let target = tc.get_new_uvar();
        tc.unify_var_utype(target, Rc::new(UType::Var(w1)))
            .expect("target is unrelated to the w1/w2 cycle and should unify with it cleanly");
    }

    // --- Wired up against doodle-rec's real Format/RecVar grammar.

    fn peano_module() -> (FormatModule, FormatId) {
        let peano = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("Z"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'S'])),
                    Format::RecVar(0),
                ])),
            ),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![(Label::Borrowed("test.peano"), peano)]);
        let level = frefs[0].get_level();
        (module, level)
    }

    #[test]
    fn peano_recurses_through_indirection_and_typechecks() -> AResult<()> {
        let (module, level) = peano_module();
        let (mut tc, v) = infer_module(&module, level)?;
        match &*tc.expand(v) {
            UType::Union(m) => assert_eq!(m.len(), 2, "expected the Z/S variants: {m:?}"),
            other => panic!("expected a Union, got {other:?}"),
        }
        Ok(())
    }

    fn ping_pong_module() -> (FormatModule, FormatId) {
        let ping = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("Done"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("More"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'A'])),
                    Format::RecVar(1), // -> pong
                ])),
            ),
        ]);
        let pong = Format::Tuple(vec![Format::Byte(ByteSet::from([b'B'])), Format::RecVar(0)]); // -> ping
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![
            (Label::Borrowed("ping"), ping),
            (Label::Borrowed("pong"), pong),
        ]);
        let level = frefs[0].get_level();
        (module, level)
    }

    #[test]
    fn mutual_recursion_typechecks() -> AResult<()> {
        let (module, level) = ping_pong_module();
        let (mut tc, v) = infer_module(&module, level)?;
        match &*tc.expand(v) {
            UType::Union(m) => assert_eq!(m.len(), 2, "expected Done/More: {m:?}"),
            other => panic!("expected a Union, got {other:?}"),
        }
        Ok(())
    }

    #[test]
    fn shared_level_vars_reuses_the_same_uvar_for_a_repeated_non_cyclic_reference() -> AResult<()> {
        // Unlike `declare_rec_formats`'s per-member-fresh `visited` (which fully re-inlines a
        // non-cyclic sibling's structure - see step 6's codegen finding), this engine's
        // module-spanning `level_vars` cache must hand back the exact same UVar for a second,
        // non-cyclic reference to an already-fully-solved level.
        let (module, level) = ping_pong_module();
        let pong_level = level + 1;
        let mut tc = TypeChecker::new(&module);
        let first = tc.infer_level(pong_level)?;
        let second = tc.infer_level(pong_level)?;
        assert_eq!(
            tc.find(first),
            tc.find(second),
            "a second reference to the same already-solved level should share its UVar, not re-solve"
        );
        Ok(())
    }

    #[test]
    fn old_infer_type_silently_accepts_a_bare_self_reference_the_new_checker_rejects() {
        // Empirical before/after: `Format::RecVar(0)` as a batch's ENTIRE body has no possible
        // finite Rust representation (matching `type X = Box<X>;` being illegal even though
        // `enum X { Y(Box<X>) }` is fine). The OLD `infer_type` (lib.rs) doesn't attempt this
        // distinction and silently succeeds; this is the module-doc-comment claim, verified.
        //
        // Can't go through `declare_rec_formats` for this - its own eager `determinations` check
        // already independently rejects a zero-consumption cycle as left-recursion (a different,
        // unrelated axis - see the module doc comment), panicking before the type layer is even
        // reached. So this drives the OLD model's `infer_type` directly instead, bypassing that
        // grammar check entirely, to isolate what the OLD type layer specifically does: `visited`
        // already containing level 0 here simulates exactly what `FormatDecl::solve_type_with`
        // sets up (`visited.insert(self.fmt_id)` before recursing into the body) at the moment it
        // would recurse into a batch member whose own body is directly `Format::RecVar(0)`.
        let module = FormatModule::new();
        let bad = Format::RecVar(0);
        let mut visited: HashSet<FormatId> = HashSet::new();
        visited.insert(0);
        let batch = Some(crate::Span::new(0, 0));
        let ty = bad
            .infer_type(&mut visited, &module, batch)
            .expect("OLD model accepts a bare, un-indirected self-reference silently");
        assert!(
            matches!(ty, FormatType::Ref(0)),
            "OLD model produces a bogus, unrepresentable Ref(0) placeholder as this format's type: {ty:?}"
        );
    }
}
