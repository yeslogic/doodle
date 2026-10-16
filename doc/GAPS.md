# Design Gaps

Certain areas of the current implementation of `doodle` fall short of the desired behavior of various edge-case handling.

## TC: `Expr::AsU*` Tiebreaks as No-Op Over `UAny`

**Resolved.** Each `AsU8`–`AsU64` cast now records a *cast-hint* toward its own width on its operand
(`CastHints` in `src/typecheck/base_set.rs`, carried inside `UintSet`/`PrimIntSet`). A unique solution is
chosen by membership first, then `Rank`, then the earliest hint among the members tied at the top rank,
where a hint's stamp is the `UVar` of the cast that issued it (so "earliest" is TC traversal order).
A hint never overrides a rank-default (`AsU8` over a `UAny32` value still resolves to `U32`), and a tie
with no hinted member in the top tier remains a `MultipleSolutions` error.

The test below originally needed a `tc_seq_length_polymorphic` feature (which typed `Expr::SeqLength` as `UAny` instead
of `UAny32`) to reach an ambiguous operand at all. That feature has been removed; the test now uses an `Auto`-rep numeric
constant (`poly_zero()`) as its operand, which has no rank-default of its own, so it passes only because of the cast-hint
(with the hint tiebreak disabled, it fails with `MultipleIntSolutions` over all eight `PrimInt`s).
The original description of the gap follows.

As currently implemented, the `TypeChecker` requires all metavariables given a `UintSet` constraint,
even those for interior-nodes whose actual types may not need to be unambiguous for the overall format's
type to be fully unambiguous, have an unambiguous resolution to a `BaseType`.

A new unit test in `typecheck.rs`, [`expr_as_cast_over_uany_forces_type`](/src/typecheck.rs#L4791), exercises a desired
specialization: if a node within an `AsU32` cast (or equivalently for the other fixed-output-type unsigned-int casts)
is constrained only by `BaseSet::UAny`, or similarly has multiple valid solutions without a clear tiebreaker, the presence
of the `AsU32` cast itself should resolve the ambiguity to force the inner and outer node-types to agree, such that the
`Expr::AsU*` wrappers can act as a 'noop-cast for type disambiguation' even for `UAny`-constraints that have no other
source of disambiguation.

### Remaining: unhinted default-clash

Two unsigned sets with *different* tiebreak defaults, such as `UAny32` (`SeqLength`, `SeqIx`, `Slice`, ...) and
`UintSet::any_default(Bits64)` (`Format::Pos`), intersect to a top tier holding both defaults, so an unannotated `pos`
flowing into a `UAny32` site has no unique solution (`different_default_uany_unification_ambiguous` in `base_set.rs`).
A cast-hint toward either width breaks the tie (`cast_hint_breaks_default_clash`), but with no hint the clash remains a
`TCError`. A solve-time "widest wins" tiebreak is sketched as a TODO on `UintSet::get_unique_solution`, but is not
implemented, as the benefit is marginal (see `NUMERIC_GUIDELINE.md`).

## TC: Void-Judgement for Pruning Union Variants

**Resolved**, except for the `Repeat(Fail)` gap and the recursion caveat below.

When a union type is expanded (`TypeChecker::expand_union`, used by codegen elaboration) or reified
(`TypeChecker::reify_union`), variants that can never be constructed are pruned: a variant whose value could only be
produced by first decoding an unconditional `Format::Fail`. "Void" here means exactly that, *the value cannot manifest
without a `Fail` first*; it says nothing about decoding failures that depend on the input.

### Old behavior

There were two independent judgements, both shallow:

- `expand_union` called `is_empty_var`, which returned `true` only when the variant's meta-variable was canonically
  bound to `Equiv(UType::Empty)` — i.e. the variant was a bare `Fail`, and nothing else.
- `reify_union` did not call `is_empty_var` at all; it pruned a variant only when its reified type was literally
  `AugValueType::Empty`, which again only caught a bare `Fail`.

Any `Fail` one constructor deep was therefore missed. Each of the following was judged inhabited (not void), and is
now correctly flagged void (`mod is_empty` tests in `typecheck.rs`):

| Format | Type | Why it is void |
| --- | --- | --- |
| `tuple([Fail])` | `(Empty,)` | every field of a tuple must be decoded |
| `record([("fail", Fail)])` | `{ fail: Empty }` | likewise for records |
| `seq([Fail])` | `Seq(Empty)` | a fixed sequence must decode each of its elements |
| `alts_nondet([("every", Fail), ("branch", tuple([Fail])), ("fails", seq([Fail]))])` | union | every variant is void, so the union is too |

The union case was the worst: a meta-variable bound to a union (`Constraints::Variant`) was never judged void, however
its variants were judged, so voidness could not propagate *through* a union. At most the bare-`Fail` `every` variant
was pruned, leaving `branch` and `fails` behind. In turn, a wrapper like

```rust
alts_nondet([("Void", fail_union.call()), ("Singleton", Format::ANY_BYTE)])
```

kept its `Void` variant, in both `expand_union` and `reify_union` (`edge_case_is_empty_var_false_negative_on_union`).
It is now pruned, and the wrapper reifies to a single-variant union.

### Current design

- **One structural judgement.** `Expansion::instantiates_void` (and `WHNFSolution::instantiates_void`, which expands a
  meta-variable one step and recurses) is `true` for `Empty`, for a tuple or record with *any* void field, for a `Seq`
  whose element type is void, and for a union *all* of whose variants are void (so an empty union is void). `Base`,
  `Int`, `ViewObj`, `Option` and `PhantomData` are never void: `None` and a deferred parse need no `Fail` to manifest.
- **Shared by both users.** `reify_union` now calls `is_empty_var` too. Its own pruning only checks a variant after
  `reify_rec` has succeeded on it, which guarantees the judgement visits only meta-variables that `reify` could resolve,
  and so cannot hit `expand_var`'s panics on `Indefinite` or ambiguous variables.
- **Memoized.** The old judgement was a constant-time check; the structural one has to walk the type, and was
  initially re-run from scratch on every query. `is_empty_var` is now backed by a per-`UVar` memo-table
  (`TypeChecker::void_table`, a lazily initialized `OnceCell<Vec<Cell<VoidState>>>`), so each meta-variable is
  expanded at most once across all queries. The table needs no upkeep in `infer_*`/`unify_*`: void-queries are only
  ever made after full-module inference, when the meta-context is frozen (a `debug_assert` checks the table's length
  against the meta-context).
- **Cycles are judged optimistically.** A meta-variable reached again while its own judgement is in progress
  (`VoidState::InProgress`) is assumed non-void. A variable can only become void through `UType::Empty`, so an
  assumption of this kind can only *under*-report voidness: a constructible variant is never pruned, even when results
  computed under the assumption are memoized. Today, cycles only arise through `PhantomData` (never void anyway), so the
  judgement is exact.

### `compute_void_table`: the exact fallback for true recursion

Under true (non-`PhantomData`) recursion, the optimistic memo is sound but incomplete. A productive recursion whose
only base case is `Fail`, e.g.

```text
V := Stop(Fail) | More(u8, V)
```

is void (every finite `V` ends in `Stop`, which needs a `Fail`), but the memo judges it non-void: expanding `V` reaches
`More`, whose `V` field is `InProgress` and hence assumed non-void, so `More` is non-void, so `V` is. The answer depends
on traversal order, and cannot be fixed locally.

`TypeChecker::compute_void_table` computes the exact judgement for every meta-variable at once, as the complement of the
*least fixpoint of inhabitation*: every variable starts out uninhabited, and each pass marks a variable inhabited once
its constraint can be satisfied by the current marking (a union needs one inhabited variant; a tuple, record or
projection-shape needs all its components; a `Seq` needs its element type), until nothing changes. On the example
above, `V` is never marked, because `More` needs `V` to be marked first and `Stop` needs `Empty`.

- **Exact and order-independent**, unlike the memo.
- **Never panics.** `Indefinite` or ambiguous meta-variables are judged inhabited, which is the conservative choice for
  pruning.
- **Cost.** Each pass is linear in the size of the meta-context, and the number of passes is bounded by the longest
  dependency chain. If that proves too slow, a worklist driven by reverse-dependency edges makes it linear overall.
- **Status.** Not used outside tests yet (`#[cfg_attr(not(test), allow(dead_code))]`). Every `mod is_empty` test, plus
  `void_tables_agree_on_wrapper_adt`, cross-checks it against `is_empty_var` over every meta-variable. It is the intended
  replacement for (or fallback to) the memo once the true-recursion work is integrated, if a `Fail`-only base case
  needs to be caught there.

### Remaining: `Repeat(Fail)` is wrongly judged void

Both judgements treat every `Seq(Empty)` as void (`utype_inhabited_step` mirrors `instantiates_void` on purpose). That
is right for a fixed `seq([Fail])`, which must decode its `Fail`, but wrong for formats that may produce *zero*
elements: `Repeat(Fail)` never matches its element, and yields `[]` without failing. The same holds for other
repetitions that can stop before their first element (e.g. `RepeatUntil*`, a `RepeatCount` whose count is zero).
Since `Seq(Empty)` is all the type records, a variant wrapping such a repetition could be wrongly pruned.

The type level currently cannot tell these cases apart. The planned fix is to extend `SeqBorrowHint`
(`src/valuetype.rs`, which already carries a `TODO` to this effect) into a richer, format-dictated set of *sequence
properties*, recording among other things whether the producing format may yield `[]` (`Repeat`, `RepeatUntil*`, ...)
or must yield at least one element (a fixed `seq([...])`, `Repeat1`, ...). The void-judgement would then treat only the
latter kind of `Seq(Empty)` as void, in both `instantiates_void` and `utype_inhabited_step`.

One thing to watch: `SeqBorrowHint`'s variant *ordering* drives how two sequence types unify (the lesser hint is kept),
so any new properties must define how they combine under unification. For "may be empty", the safe combination is
"may be empty if either side may be", which keeps the judgement from ever pruning a constructible variant.
