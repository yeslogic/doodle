//! Prototype microcosm of real doodle's `Elaborator`/`CodeGen::lift_uvar` layer: lifts a
//! [`typecheck::TypeChecker`]'s solved `UVar` graph for one recursive batch into named Rust type
//! declarations, using the SAME "reserve a placeholder before recursing, commit the real content
//! after" pattern real doodle's `lift_uvar` uses for its `in_progress`/`NameGen` reservation (see
//! this session's memory notes on that investigation) - proving it out for `doodle-rec`'s own
//! *genuinely decoded* recursion, not real doodle's inert `PhantomData`-only case.
//!
//! **Why this needed building at all, concretely**: `codegen.rs` (the step-6 prototype) already
//! solves Box-placement, but off the OLD `FormatType::Ref` model, which is asymmetric - a batch
//! member reached non-cyclically gets fully structurally INLINED rather than referenced by name
//! (see `codegen.rs`'s own `mutual_recursion_calls_the_correct_sibling_by_name` test comment).
//! This module's `TypeChecker::level_vars` (persisting across the whole top-level check, not
//! reset per batch member) fixes that: every batch member, cyclic or not, gets a real, shared
//! `UVar`, hence a real, nominal Rust reference here rather than inlined duplication.
//!
//! **A second, independently-motivating bug, found empirically before writing this module**:
//! `codegen.rs::write_type_decl` emits a transparent `pub type Name = ...;` ALIAS for any
//! non-`Union` batch member. A bare alias cannot close a recursive cycle, even behind `Box` -
//! confirmed with a minimal probe (`type A = (u8, B); type B = (u8, Box<A>);`) that `rustc`
//! rejects outright as `E0391 cycle detected when expanding type alias` - Rust's alias-expansion
//! check is purely syntactic and doesn't know `Box` is opaque. `codegen.rs`'s own test corpus
//! (peano, ping/pong) never hit this because a `Union` (which already compiles to a real `enum`)
//! happens to sit somewhere on both cycles. This module never emits a plain alias for anything in
//! `level_of` (a declared batch member) - every member becomes a real nominal Rust item, a
//! tuple-`struct` for non-`Union` shapes (matching `Union`'s existing `enum` treatment) - see the
//! `all_tuple_cycle_...` test for the exact previously-uncompilable shape this fixes.
//!
//! Deliberately narrower than real doodle's actual `lift_uvar` in two ways, both because
//! `doodle-rec` has nothing to exercise them against yet, not because they're hard:
//! - No `var_needs_lifetime`/lifetime-parameter machinery - `doodle-rec` has no borrowing/view
//!   construct (no `ReadArray`/`BufferView` equivalent) at all, so every generated type here is
//!   fully owned; there is nothing for that check to ever return `true` on yet.
//! - No content-based name deduplication (real doodle's `NameGen::get_name`/`rev_map`, keyed by a
//!   `RustTypeDecl`'s structural content). Every name-needing `UVar` here already corresponds to a
//!   declared batch-member level (`Union` only appears at a batch member's own top level, matching
//!   `codegen.rs`'s existing restriction - not lifted here either), so a stable name
//!   (`module.get_name(level)`) is always already known up front; nothing here is anonymous the
//!   way an `Expr`-side inline record/union can be in real doodle. Only the RESERVATION mechanism
//!   itself (what this module exists to prototype) is built.
//!
//! Originally did not attempt decode-function generation at all (that judgment call - and
//! `CaseLogic` - stands: see [`generate_combined_source`]'s own docs for why `CaseLogic` itself
//! turned out to need no new work). But leaving `codegen.rs` and this module as two
//! non-interoperating prototypes meant `doodle-rec` had no single pipeline that produced BOTH
//! correct types AND matching decode logic for the same batch - `codegen.rs`'s decode functions
//! construct values shaped for its OLD, asymmetric `FormatType::Ref` model (inlined siblings,
//! sometimes a bare alias), which no longer match this module's types at all.
//! [`generate_combined_source`] closes that gap: it drives `codegen.rs`'s existing
//! `Compiler`/`Decoder` compilation for the actual decode LOGIC (byte-level dispatch, `MatchTree`
//! disambiguation - none of that is a type-layer concern, nothing to redo), but consults THIS
//! module's `LiftedBatch` - not `codegen.rs`'s own `FormatType`-driven `rust_type`/`Box`-insertion
//! - for where a constructed value needs `Box`-wrapping or its own nominal-`struct` wrapper. A new
//! addition here rather than a modification of `codegen.rs`'s existing (tested) functions, for the
//! same "new parallel module over touching existing tested code" reasoning already used for
//! `typecheck.rs` itself.

use std::{collections::BTreeMap, fmt::Write as _};

use anyhow::{Result as AResult, anyhow};

use crate::{
    BaseType, FormatId, FormatModule, Span,
    codegen::{self, CompiledBatch, type_name, variant_name},
    decoder::Decoder,
    typecheck::{TypeChecker, UType, UVar},
};

/// One lifted Rust type position - either an unnamed, structural shape, or a reference to a named
/// declaration recorded in [`Lifter::defined`] (by index).
#[derive(Debug, Clone)]
pub enum GenType {
    Inline(RustType),
    Def(usize),
}

#[derive(Debug, Clone)]
pub enum RustType {
    Unit,
    Base(BaseType),
    Tuple(Vec<RustType>),
    Option(Box<RustType>),
    Vec(Box<RustType>),
    /// Inserted exactly where [`Lifter::lift_ref`] finds the referenced `UVar` already
    /// `in_progress` (an ancestor still under construction) - i.e. exactly where a real cycle
    /// closes, the same structural signal `typecheck.rs`'s occurs-check calls "indirected".
    Boxed(Box<RustType>),
    /// A reference to `defined[ix]` by name.
    Named(usize),
}

#[derive(Debug, Clone)]
pub enum RustTypeDef {
    /// A non-`Union` batch member: always a real tuple-`struct` wrapper (`pub struct Name(pub
    /// T);`), never a transparent `type Name = T;` alias - see the module doc comment for why a
    /// bare alias can't safely close a cycle even behind `Box`.
    Struct(RustType),
    Enum(BTreeMap<String, RustType>),
}

#[derive(Debug, Clone)]
pub struct RustTypeDecl {
    pub name: String,
    pub def: RustTypeDef,
}

pub struct LiftedBatch {
    pub defined: Vec<RustTypeDecl>,
    /// `defined` index for each batch member, in span order.
    pub member_index: Vec<usize>,
}

struct Lifter<'a> {
    module: &'a FormatModule,
    defined: Vec<RustTypeDecl>,
    /// Memoizes a completed lift per canonical `UVar`, matching real doodle's `metavariables`.
    by_uvar: std::collections::HashMap<UVar, GenType>,
    /// `defined` index reserved for a name-needing `UVar` the moment its own `lift` call begins -
    /// present as a key for exactly as long as that `UVar` is on the current recursion stack.
    /// Unlike real doodle's `in_progress` (which reserves *lazily*, only once a self-reference is
    /// actually found, because its names are content-derived and not yet knowable), every entry
    /// here is reserved *eagerly* at the start of its own `lift` call - `doodle-rec`'s names are
    /// already known up front (see the module doc comment), so there's nothing to defer.
    in_progress: std::collections::HashMap<UVar, usize>,
    /// Canonical `UVar` -> declared batch-member level, populated for the WHOLE batch before any
    /// member is lifted (so a forward reference to a not-yet-visited sibling is still known to
    /// need a name).
    level_of: std::collections::HashMap<UVar, FormatId>,
}

impl<'a> Lifter<'a> {
    fn reserve(&mut self, level: FormatId) -> usize {
        let ix = self.defined.len();
        let name = type_name(self.module.get_name(level));
        // Placeholder content, overwritten once the real shape is known (below) - never read in
        // this placeholder state by anything (the reservation only ever needs to hand out a
        // stable name/index, matching real doodle's own "content is never read from the
        // placeholder" invariant for its `t_formats`/`level_vars` reservations).
        self.defined.push(RustTypeDecl {
            name,
            def: RustTypeDef::Struct(RustType::Unit),
        });
        ix
    }

    fn lift(&mut self, tc: &mut TypeChecker<'a>, v: UVar) -> AResult<GenType> {
        let v = tc.find(v);
        if let Some(gt) = self.by_uvar.get(&v) {
            return Ok(gt.clone());
        }
        match self.level_of.get(&v).copied() {
            None => {
                let ty = tc.expand(v);
                let rt = self.lift_shape(tc, &ty)?;
                let gt = GenType::Inline(rt);
                self.by_uvar.insert(v, gt.clone());
                Ok(gt)
            }
            Some(level) => {
                // A self/ancestor-reference reached `v` again while its own definition is still
                // being built further out on this very call stack - hand back the reservation
                // already in progress rather than starting a second, unbounded recursion into the
                // same level's body (the bug this exact check exists to prevent: dropping it
                // reproduces the historical `infer_var_format_level` stack-overflow real doodle
                // hit, empirically confirmed while writing this module - see the module's git
                // history / commit message for how it was caught).
                if let Some(&ix) = self.in_progress.get(&v) {
                    return Ok(GenType::Def(ix));
                }
                let ix = self.reserve(level);
                self.in_progress.insert(v, ix);
                let ty = tc.expand(v);
                let def = self.lift_def(tc, &ty)?;
                self.defined[ix].def = def;
                self.in_progress.remove(&v);
                let gt = GenType::Def(ix);
                self.by_uvar.insert(v, gt.clone());
                Ok(gt)
            }
        }
    }

    fn lift_def(&mut self, tc: &mut TypeChecker<'a>, ty: &UType) -> AResult<RustTypeDef> {
        match ty {
            UType::Union(m) => {
                let mut variants = BTreeMap::new();
                for (label, t) in m {
                    let rt = self.lift_child(tc, t)?;
                    variants.insert(variant_name(label), rt);
                }
                Ok(RustTypeDef::Enum(variants))
            }
            other => Ok(RustTypeDef::Struct(self.lift_shape(tc, other)?)),
        }
    }

    fn lift_shape(&mut self, tc: &mut TypeChecker<'a>, ty: &UType) -> AResult<RustType> {
        match ty {
            UType::Hole => Err(anyhow!(
                "elaborate: unconstrained Hole reached during lifting"
            )),
            UType::Void => Err(anyhow!(
                "elaborate: cannot lift an uninhabited Void type to Rust"
            )),
            UType::Base(b) => Ok(RustType::Base(*b)),
            UType::Var(v) => self.lift_ref(tc, *v),
            UType::Tuple(ts) if ts.is_empty() => Ok(RustType::Unit),
            UType::Tuple(ts) => Ok(RustType::Tuple(
                ts.iter()
                    .map(|t| self.lift_child(tc, t))
                    .collect::<AResult<Vec<_>>>()?,
            )),
            UType::Seq(t) => Ok(RustType::Vec(Box::new(self.lift_child(tc, t)?))),
            UType::Option(t) => Ok(RustType::Option(Box::new(self.lift_child(tc, t)?))),
            UType::Union(_) => Err(anyhow!(
                "elaborate: a Union type nested inside another type isn't supported by this \
                 prototype - matching codegen.rs's existing restriction, Union is only supported \
                 at a batch member's own top level"
            )),
        }
    }

    fn lift_child(
        &mut self,
        tc: &mut TypeChecker<'a>,
        t: &std::rc::Rc<UType>,
    ) -> AResult<RustType> {
        match &**t {
            UType::Var(v) => self.lift_ref(tc, *v),
            other => self.lift_shape(tc, other),
        }
    }

    /// Lifts a reference to `v` (not `v`'s own definition) - this is the one place `Box` gets
    /// decided: `v` is already `in_progress` exactly when it's an ancestor still under
    /// construction, i.e. exactly when this reference closes a real cycle.
    fn lift_ref(&mut self, tc: &mut TypeChecker<'a>, v: UVar) -> AResult<RustType> {
        let v = tc.find(v);
        let boxed = self.in_progress.contains_key(&v);
        let gt = self.lift(tc, v)?;
        let rt = match gt {
            GenType::Inline(rt) => rt,
            GenType::Def(ix) => RustType::Named(ix),
        };
        Ok(if boxed {
            RustType::Boxed(Box::new(rt))
        } else {
            rt
        })
    }
}

/// Lifts every member of the recursive batch spanning `span` into named Rust type declarations.
pub fn lift_batch<'a>(
    tc: &mut TypeChecker<'a>,
    module: &'a FormatModule,
    span: Span<FormatId>,
) -> AResult<LiftedBatch> {
    let mut lifter = Lifter {
        module,
        defined: Vec::new(),
        by_uvar: std::collections::HashMap::new(),
        in_progress: std::collections::HashMap::new(),
        level_of: std::collections::HashMap::new(),
    };
    // Pass 1: populate level_of for the WHOLE batch before lifting any one member, so a forward
    // reference to a not-yet-visited sibling is still recognized as name-needing.
    let mut uvars = Vec::with_capacity(span.end - span.start + 1);
    for level in span.start..=span.end {
        let uvar = tc.infer_level(level)?;
        let uvar = tc.find(uvar);
        lifter.level_of.insert(uvar, level);
        uvars.push(uvar);
    }
    // Pass 2: lift each member in turn.
    let mut member_index = Vec::with_capacity(uvars.len());
    for uvar in uvars {
        match lifter.lift(tc, uvar)? {
            GenType::Def(ix) => member_index.push(ix),
            GenType::Inline(_) => unreachable!(
                "every batch member is registered in level_of, so lift() always returns Def for it"
            ),
        }
    }
    Ok(LiftedBatch {
        defined: lifter.defined,
        member_index,
    })
}

fn render_type(rt: &RustType, defined: &[RustTypeDecl]) -> String {
    match rt {
        RustType::Unit => "()".to_string(),
        RustType::Base(BaseType::Bool) => "bool".to_string(),
        RustType::Base(BaseType::U8) => "u8".to_string(),
        RustType::Base(BaseType::U16) => "u16".to_string(),
        RustType::Base(BaseType::U32) => "u32".to_string(),
        RustType::Base(BaseType::U64) => "u64".to_string(),
        RustType::Base(BaseType::Char) => "char".to_string(),
        RustType::Tuple(ts) => {
            if ts.is_empty() {
                "()".to_string()
            } else {
                format!(
                    "({})",
                    ts.iter()
                        .map(|t| render_type(t, defined))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
        RustType::Option(t) => format!("Option<{}>", render_type(t, defined)),
        RustType::Vec(t) => format!("Vec<{}>", render_type(t, defined)),
        RustType::Boxed(t) => format!("Box<{}>", render_type(t, defined)),
        RustType::Named(ix) => defined[*ix].name.clone(),
    }
}

/// Renders a [`LiftedBatch`]'s declarations as plain Rust source - types only, no decode
/// functions (see the module doc comment on why that's out of scope for this pass).
pub fn generate_type_source(batch: &LiftedBatch) -> AResult<String> {
    let mut out = String::new();
    for decl in &batch.defined {
        match &decl.def {
            RustTypeDef::Struct(rt) => {
                writeln!(
                    out,
                    "pub struct {}(pub {});",
                    decl.name,
                    render_type(rt, &batch.defined)
                )?;
            }
            RustTypeDef::Enum(variants) => {
                writeln!(out, "pub enum {} {{", decl.name)?;
                for (variant, rt) in variants {
                    writeln!(out, "    {variant}({}),", render_type(rt, &batch.defined))?;
                }
                writeln!(out, "}}")?;
            }
        }
    }
    Ok(out)
}

/// Compiles and lifts the recursive batch containing `entry_level` and generates ONE Rust source
/// string containing both this module's correct, always-nominal type declarations and matching
/// `decode_*` functions - the actual decode LOGIC (byte-level dispatch, `MatchTree`
/// disambiguation, `RecVar` -> direct sibling call) comes from `codegen.rs`'s existing
/// `Compiler`/`Decoder` compilation unchanged; only the type-driven decisions (where a constructed
/// value needs `Box`-wrapping, whether the whole function's result needs a nominal `Name(v)`
/// wrapper) are redirected to consult THIS module's `LiftedBatch` instead of `codegen.rs`'s own
/// `FormatType`-driven equivalents. Confirms real doodle's own finding (from this session's
/// `CaseLogic` investigation) that decode-time recursion needs no new `CaseLogic`-shaped machinery
/// at all: `RecVar` already compiles to a direct, unconditionally-queued sibling function call
/// (`codegen.rs`'s `Decoder::CallRec` handling) - the only thing that needed fixing was ever the
/// TYPE side, which is exactly what's addressed here.
pub fn generate_combined_source(module: &FormatModule, entry_level: FormatId) -> AResult<String> {
    let compiled = codegen::compile_batch(module, entry_level)?;
    let mut tc = TypeChecker::new(module);
    let lifted = lift_batch(&mut tc, module, compiled.span)?;

    let mut out = String::new();
    writeln!(
        out,
        "// Generated by doodle-rec's combined codegen/elaborate prototype - do not hand-edit."
    )?;
    writeln!(out)?;
    out.push_str(&generate_type_source(&lifted)?);
    for ix in 0..compiled.names.len() {
        writeln!(out)?;
        write_decode_fn(&mut out, &compiled, &lifted, ix)?;
    }
    Ok(out)
}

fn write_decode_fn(
    out: &mut String,
    compiled: &CompiledBatch<'_>,
    lifted: &LiftedBatch,
    ix: usize,
) -> AResult<()> {
    let fn_name = codegen::fn_name(compiled.names[ix]);
    let decl_ix = lifted.member_index[ix];
    let decl = &lifted.defined[decl_ix];
    let ty_name = &decl.name;
    writeln!(
        out,
        "pub fn {fn_name}(input: &[u8]) -> Result<({ty_name}, &[u8]), String> {{"
    )?;
    let body = match &decl.def {
        RustTypeDef::Enum(variants) => gen_decoder(
            compiled,
            lifted,
            &compiled.decoders[ix],
            &RustType::Unit, // unused: every arm here is a Decoder::Variant, which looks its own
            // payload type up from `variants` via `enclosing_enum` instead of using `rt` at all.
            Some((decl_ix, variants)),
        )?,
        RustTypeDef::Struct(inner_rt) => {
            let body = gen_decoder(compiled, lifted, &compiled.decoders[ix], inner_rt, None)?;
            format!("{{ let (v, input) = {body}?; Ok::<_, String>(({ty_name}(v), input)) }}")
        }
    };
    writeln!(out, "    {body}")?;
    writeln!(out, "}}")?;
    Ok(())
}

/// Mirrors `codegen.rs::gen_decoder` exactly, except keyed on this module's own `RustType`
/// (already fully resolved - `Box`-decisions baked in) instead of `codegen.rs`'s `FormatType`.
/// `rt` is this exact node's elaborated type, threaded down in lockstep with the `Decoder` tree -
/// both ultimately derive from the same `Format` tree via independent passes (`Compiler`/`Decoder`
/// for decode logic, `TypeChecker`/`UType` for types), so they agree structurally at every
/// position by construction; only `Decoder::CallRec`'s `Box`-decision and this function's own
/// top-level caller (see [`write_decode_fn`]) actually consult `rt`'s *content*.
fn gen_decoder(
    compiled: &CompiledBatch<'_>,
    lifted: &LiftedBatch,
    d: &Decoder,
    rt: &RustType,
    enclosing_enum: Option<(usize, &BTreeMap<String, RustType>)>,
) -> AResult<String> {
    match d {
        Decoder::FailWith(msg) => Ok(format!("{{ Err(format!({msg:?})) }}")),
        Decoder::EndOfInput => Ok(
            "{ if input.is_empty() { Ok(((), input)) } else { Err(\"expected end of input\".to_string()) } }"
                .to_string(),
        ),
        Decoder::Byte(bs) => {
            let pattern = bs
                .iter()
                .map(|b| format!("{b}u8"))
                .collect::<Vec<_>>()
                .join(" | ");
            if pattern.is_empty() {
                Ok("{ Err(\"unsatisfiable byte set\".to_string()) }".to_string())
            } else {
                Ok(format!(
                    "{{ match input.first().copied() {{ Some(b) if matches!(b, {pattern}) => Ok((b, &input[1..])), _ => Err(\"byte mismatch\".to_string()) }} }}"
                ))
            }
        }
        Decoder::Compute(expr) => {
            let lit = codegen::value_to_rust_literal(&expr.eval())?;
            Ok(format!("{{ Ok::<_, String>(({lit}, input)) }}"))
        }
        Decoder::Variant(label, d) => {
            let (enum_ix, variants) = enclosing_enum
                .ok_or_else(|| anyhow!("elaborate: Variant outside a batch member's own Union"))?;
            let ctor = variant_name(label);
            let inner_rt = variants
                .get(&ctor)
                .ok_or_else(|| anyhow!("elaborate: variant {ctor} not found in enclosing Union"))?;
            let inner = gen_decoder(compiled, lifted, d, inner_rt, None)?;
            let enum_name = &lifted.defined[enum_ix].name;
            Ok(format!(
                "{{ let (v, input) = {inner}?; Ok::<_, String>(({enum_name}::{ctor}(v), input)) }}"
            ))
        }
        Decoder::Branch(tree, ds) => {
            let dispatch = codegen::gen_match_tree(tree, 0);
            let mut arms = String::new();
            for (i, d) in ds.iter().enumerate() {
                let arm = gen_decoder(compiled, lifted, d, rt, enclosing_enum)?;
                writeln!(arms, "        {i} => {arm},")?;
            }
            Ok(format!(
                "{{ let branch_ix: Result<usize, String> = {dispatch}; match branch_ix? {{\n{arms}        _ => unreachable!(\"MatchTree produced an out-of-range branch index\"),\n    }} }}"
            ))
        }
        Decoder::Tuple(ds) => {
            let RustType::Tuple(field_tys) = rt else {
                return Err(anyhow!("elaborate: Tuple decoder without a Tuple RustType"));
            };
            if ds.is_empty() {
                return Ok("{ Ok::<_, String>(((), input)) }".to_string());
            }
            let mut block = String::from("{\n");
            for (i, (d, field_ty)) in ds.iter().zip(field_tys).enumerate() {
                let sub = gen_decoder(compiled, lifted, d, field_ty, None)?;
                writeln!(block, "        let (v{i}, input) = {sub}?;")?;
            }
            let elems = (0..ds.len())
                .map(|i| format!("v{i}"))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(block, "        Ok::<_, String>((({elems}), input))\n    }}")?;
            Ok(block)
        }
        Decoder::Seq(ds) => {
            let RustType::Vec(elem_ty) = rt else {
                return Err(anyhow!("elaborate: Seq decoder without a Vec RustType"));
            };
            let mut block = String::from("{\n        let mut v = Vec::new();\n");
            for d in ds {
                let sub = gen_decoder(compiled, lifted, d, elem_ty, None)?;
                writeln!(block, "        let (vi, input) = {sub}?; v.push(vi);")?;
            }
            writeln!(block, "        Ok::<_, String>((v, input))\n    }}")?;
            Ok(block)
        }
        Decoder::CallRec(_slot, batch_ix) => {
            let target = *batch_ix;
            if target >= compiled.names.len() {
                return Err(anyhow!("elaborate: RecVar({target}) out of range for this batch"));
            }
            let callee = codegen::fn_name(compiled.names[target]);
            match rt {
                RustType::Boxed(_) => Ok(format!(
                    "{{ let (v, input) = {callee}(input)?; Ok::<_, String>((Box::new(v), input)) }}"
                )),
                _ => Ok(format!("{{ {callee}(input) }}")),
            }
        }
        Decoder::Call(_) => Err(anyhow!(
            "elaborate: Decoder::Call (a reference to a format outside this batch) is not supported by this prototype"
        )),
        other => Err(anyhow!(
            "elaborate: unsupported decoder shape (out of scope for this prototype): {other:?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Format, FormatId, Label};
    use doodle::byte_set::ByteSet;

    fn peano_module() -> (FormatModule, FormatId, Span<FormatId>) {
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
        (module, level, Span::new(level, level))
    }

    #[test]
    fn peano_lifts_to_a_boxed_self_referential_enum() -> AResult<()> {
        let (module, _level, span) = peano_module();
        let mut tc = TypeChecker::new(&module);
        let batch = lift_batch(&mut tc, &module, span)?;
        assert_eq!(batch.defined.len(), 1);
        let decl = &batch.defined[batch.member_index[0]];
        assert_eq!(decl.name, "TestPeano");
        match &decl.def {
            RustTypeDef::Enum(variants) => {
                assert_eq!(variants.len(), 2, "expected Z/S: {variants:?}");
                match &variants["S"] {
                    RustType::Tuple(fields) => match &fields[1] {
                        RustType::Boxed(inner) => {
                            assert!(
                                matches!(**inner, RustType::Named(ix) if ix == batch.member_index[0])
                            )
                        }
                        other => panic!("expected the self-reference to be Boxed, got {other:?}"),
                    },
                    other => panic!("expected S's payload to be a Tuple, got {other:?}"),
                }
            }
            other => panic!("expected an Enum, got {other:?}"),
        }
        let src = generate_type_source(&batch)?;
        assert!(src.contains("pub enum TestPeano"));
        assert!(src.contains("Box<TestPeano>"), "source was:\n{src}");
        Ok(())
    }

    fn ping_pong_module() -> (FormatModule, FormatId, Span<FormatId>) {
        let ping = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("Done"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("More"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'A'])),
                    Format::RecVar(1),
                ])),
            ),
        ]);
        let pong = Format::Tuple(vec![Format::Byte(ByteSet::from([b'B'])), Format::RecVar(0)]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![
            (Label::Borrowed("ping"), ping),
            (Label::Borrowed("pong"), pong),
        ]);
        let level = frefs[0].get_level();
        (module, level, Span::new(level, level + 1))
    }

    #[test]
    fn mutual_recursion_boxes_exactly_one_edge_of_the_cycle() -> AResult<()> {
        // ping is lifted FIRST (span order): its own RecVar(1)->pong reference is NOT boxed
        // (pong isn't in_progress yet at that point - it's a real, ordinary nominal reference,
        // proving the non-cyclic-sibling-sharing fix over codegen.rs's old inlining behavior).
        // pong's OWN RecVar(0)->ping reference IS boxed (ping is still in_progress, an ancestor -
        // this is where the cycle actually closes). Which edge gets boxed is determined purely by
        // traversal/reservation order, not any special-casing - exactly what this test checks.
        let (module, _level, span) = ping_pong_module();
        let mut tc = TypeChecker::new(&module);
        let batch = lift_batch(&mut tc, &module, span)?;
        let ping_ix = batch.member_index[0];
        let pong_ix = batch.member_index[1];

        let ping_decl = &batch.defined[ping_ix];
        assert_eq!(ping_decl.name, "Ping");
        let RustTypeDef::Enum(variants) = &ping_decl.def else {
            panic!("expected Ping to be an Enum: {:?}", ping_decl.def)
        };
        match &variants["More"] {
            RustType::Tuple(fields) => assert!(
                matches!(fields[1], RustType::Named(ix) if ix == pong_ix),
                "expected ping's reference to pong to be a bare (unboxed) Named ref: {:?}",
                fields[1]
            ),
            other => panic!("expected More's payload to be a Tuple, got {other:?}"),
        }

        let pong_decl = &batch.defined[pong_ix];
        assert_eq!(pong_decl.name, "Pong");
        let RustTypeDef::Struct(RustType::Tuple(fields)) = &pong_decl.def else {
            panic!(
                "expected Pong to be a Struct wrapping a Tuple: {:?}",
                pong_decl.def
            )
        };
        assert!(
            matches!(&fields[1], RustType::Boxed(inner) if matches!(**inner, RustType::Named(ix) if ix == ping_ix)),
            "expected pong's reference back to ping to be Boxed: {:?}",
            fields[1]
        );

        let src = generate_type_source(&batch)?;
        assert!(src.contains("pub enum Ping"));
        assert!(
            src.contains("pub struct Pong(pub (u8, Box<Ping>));"),
            "source was:\n{src}"
        );
        Ok(())
    }

    /// The concrete, previously-uncompilable case this module exists to fix: a two-member cycle
    /// with NO `Union` anywhere on it (so `codegen.rs`'s old model would emit a plain, mutually
    /// self-referential `type` alias pair - confirmed separately, before writing this module, to
    /// be rejected outright by `rustc` as E0391 even with `Box` in between, since alias expansion
    /// is purely syntactic). This module always emits a real nominal `struct` for a non-`Union`
    /// batch member, so the same shape must compile.
    #[test]
    fn all_tuple_cycle_gets_real_structs_not_aliases_and_compiles() -> AResult<()> {
        let pair_a = Format::Tuple(vec![Format::Byte(ByteSet::from([b'A'])), Format::RecVar(1)]);
        let pair_b = Format::Tuple(vec![Format::Byte(ByteSet::from([b'B'])), Format::RecVar(0)]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![
            (Label::Borrowed("pair_a"), pair_a),
            (Label::Borrowed("pair_b"), pair_b),
        ]);
        let level = frefs[0].get_level();
        let mut tc = TypeChecker::new(&module);
        let batch = lift_batch(&mut tc, &module, Span::new(level, level + 1))?;

        for decl in &batch.defined {
            assert!(
                matches!(decl.def, RustTypeDef::Struct(_)),
                "expected every non-Union member to be a real struct, not an alias: {decl:?}"
            );
        }
        let src = generate_type_source(&batch)?;
        assert!(src.contains("pub struct PairA"), "source was:\n{src}");
        assert!(src.contains("pub struct PairB"), "source was:\n{src}");
        assert!(
            !src.contains("type "),
            "must never emit a transparent alias: {src}"
        );

        // Real rustc round-trip: this exact shape (mutual recursion through Tuples alone, no
        // enum anywhere) is exactly what a plain-alias version of this codegen cannot compile.
        let dir = std::env::temp_dir();
        let path = dir.join("doodle_rec_elaborate_all_tuple_cycle_probe.rs");
        std::fs::write(&path, &src).expect("write probe source");
        let output = std::process::Command::new("rustc")
            .args(["--edition", "2021", "--crate-type", "lib", "-o"])
            .arg(dir.join("doodle_rec_elaborate_all_tuple_cycle_probe.rlib"))
            .arg(&path)
            .output()
            .expect("run rustc");
        assert!(
            output.status.success(),
            "generated source failed to compile:\n{src}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }

    /// Compiles `src` (appending `harness`, a `fn main()`) with a bare `rustc` and runs it,
    /// asserting a zero exit code - the same real-compile-and-run verification method used
    /// throughout this whole session (step 6's `codegen.rs`, this module's own type-only tests),
    /// now applied to the COMBINED type+decode-function output.
    fn compile_and_run(name: &str, src: &str, harness: &str) {
        let dir = std::env::temp_dir();
        let full_src = format!("{src}\n{harness}\n");
        let src_path = dir.join(format!("doodle_rec_combined_{name}.rs"));
        let bin_path = dir.join(format!("doodle_rec_combined_{name}"));
        std::fs::write(&src_path, &full_src).expect("write combined source");
        let compile = std::process::Command::new("rustc")
            .args(["--edition", "2021", "-o"])
            .arg(&bin_path)
            .arg(&src_path)
            .output()
            .expect("run rustc");
        assert!(
            compile.status.success(),
            "generated source failed to compile:\n{full_src}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&compile.stderr)
        );
        let run = std::process::Command::new(&bin_path)
            .output()
            .expect("run compiled binary");
        assert!(
            run.status.success(),
            "compiled binary exited non-zero:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr)
        );
    }

    #[test]
    fn combined_peano_compiles_and_decodes_correctly() -> AResult<()> {
        let (module, level, _span) = peano_module();
        let src = generate_combined_source(&module, level)?;
        assert!(src.contains("pub enum TestPeano"));
        assert!(src.contains("pub fn decode_test_peano"));
        assert!(src.contains("Box<TestPeano>"), "source was:\n{src}");

        let harness = r#"
fn main() {
    let input = b"SSSZtrailing";
    let (tree, rest) = decode_test_peano(input).expect("decode should succeed");
    fn depth(t: &TestPeano) -> u32 {
        match t {
            TestPeano::Z(_) => 0,
            TestPeano::S((_, inner)) => 1 + depth(inner),
        }
    }
    assert_eq!(depth(&tree), 3, "expected peano depth 3 for input SSSZ");
    assert_eq!(rest, b"trailing");
    assert!(decode_test_peano(b"X").is_err(), "malformed input must be rejected");
}
"#;
        compile_and_run("peano", &src, harness);
        Ok(())
    }

    #[test]
    fn combined_ping_pong_compiles_and_decodes_correctly() -> AResult<()> {
        let (module, level, _span) = ping_pong_module();
        let src = generate_combined_source(&module, level)?;
        assert!(src.contains("pub enum Ping"));
        assert!(
            src.contains("pub struct Pong(pub (u8, Box<Ping>));"),
            "source was:\n{src}"
        );
        assert!(src.contains("pub fn decode_ping"));
        assert!(src.contains("pub fn decode_pong"));

        // "ABZ" -> Ping::More('A', Pong('B', Box::new(Ping::Done('Z')))), proving decode_pong's
        // constructed value is correctly wrapped in the new Pong(..) struct constructor, and that
        // the Box::new(..) inserted at pong's own reference back to ping (the one edge of the
        // cycle elaborate.rs decided needs it) round-trips through an ACTUAL decode, not just a
        // type-level compile check.
        let harness = r#"
fn main() {
    let input = b"ABZtrailing";
    let (val, rest) = decode_ping(input).expect("decode should succeed");
    match val {
        Ping::More((a, pong)) => {
            assert_eq!(a, b'A');
            let (b, boxed_ping) = pong.0;
            assert_eq!(b, b'B');
            match *boxed_ping {
                Ping::Done(z) => assert_eq!(z, b'Z'),
                _ => panic!("expected Done"),
            }
        }
        Ping::Done(_) => panic!("expected More, got Done at the top level"),
    }
    assert_eq!(rest, b"trailing");
    assert!(decode_ping(b"X").is_err(), "malformed input must be rejected");
}
"#;
        compile_and_run("ping_pong", &src, harness);
        Ok(())
    }
}
