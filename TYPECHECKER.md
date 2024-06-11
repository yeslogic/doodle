# Type-Checker Extension Guide

This is a WIP guide for the current implementation of the [type-inference engine](src/typecheck.rs) and the accompanying elaborator
within the [code-generation module](src/codgen/mod.rs).

This is meant primarily as a rough reference document and not a design doc, but hopefully should help in future efforts to extend the `Format + Expr + Pattern` language with additional primitives over time, with respect to ensuring that the new primitives have full support in the type-inference and elaboration layers.

## Type-Inference

[`typecheck.rs`](src/typecheck.rs) is a bidirectional type-inference engine that uses a fixed-traversal-order indexing of a `Format`-rooted tree, by assigning unification-metavariables (`UVar`s) to each node in the tree, regardless of the sub-language (`Format` vs `Expr` vs `Pattern`) of the node in question.

At every stage, all inferred constraints on individual `UVar`s, or between pairs of `UVar`s are recorded, and gradually reified as the total structure of the top-level `Format` is expanded. If any `UVar`s remain insoluble by the time the top-level `Format` is fully processed, this is a type-level error and reification of the full tree is not possible.

### UVar and UType

We define a higher-level abstraction of `UType`, which is defined auto-recursively, and which contains zero or more `UVar`s.

As currently defined:

```rust
pub enum UType {
    Empty,     // Reserved for value-free Formats (Format::Fail only)
    Hole,      // ground type-hole for shape-only unifications
    Var(UVar), // type-hole
    Base(BaseType),
    Tuple(Vec<Rc<UType>>),
    Record(Vec<(Label, Rc<UType>)>),
    Seq(Rc<UType>),
}
```

For this document, we adopt the following notational conventions:

- `Var(X)` and `X` are both written as `?X`, e.g. `?0` and `?N`
- `Hole` is written as `??`, which is a placeholder that is erased as soon as it is unified with something other than another Hole.
- `Base(T)` is written as T, e.g. `Base(Char)` is simply `Char`
- `Tuple([T0, \ldots, TN])` is written as `(T0, ... TN)`, and the elements of $\mathtt{?X} \simeq (\mathtt{T_0}, ..., \mathtt{T_N})$ are written as `?X[i]` (signifying `Ti`)
- `Record([(Lab0, T0), ..., (LabN, TN)])` is written as `{ Lab0: T0, ..., LabN: TN }`
- `Seq(T)` is written as `[T]`
- `Empty` is written as $\varepsilon$

### Simple Constraints

There are three principal sub-classes of `Constraint` that can be recorded and
later enforced during reification. Note that these are prescriptive, in the
sense that they are demands that are placed on the eventual type of a given
node, that must be fulfilled in order for the tree to pass type-checking. In
particular, if a specific type is later found for a node, that violates the
constraints placed on it directly or by implication, the unification algorithm
will fail. In this sense, constraints are not type-discovery, but rather, selective
narrowing to the possible combinations that would yield a type-valid sub-tree.

- `Equiv` (Equivalence): Metavariable `?X` is equivalent to a given UType `T` ($\mathtt{?X} \simeq \mathtt{T}$)
- `Elem` (Element): Metavariable `?X` is some member of a (possibly singleton) set of ground-types `S` ($\mathtt{?X} \in \mathcal{S}$)
- `Proj` (Projection): Metavariable `?X` is one of the following:
  - A sequence whose element type is `??` (or, post-unification, `?Y`)
  - A tuple-type which can be selectively indexed into with new metavariable associations $\verb!?X![i] \simeq \verb!?Y!$ that will eventually cover all valid indices, but may start out vacuous.
  - A record-type with an unknown set of field-names, which can be selectively populated with new field name-type pairs `(Name, Type)`. If two implicit fields have the same label, their field-types are forced to unify, and otherwise the full shape of the record is delayed until no more fields could possibly be added (usually when the full tree is reified)

### Variant Constraints

Furthermore, there is also a higher-level `Constraints` type that models not
only these static `Constraint` values, but also supports a mutually-exclusive
path of enumerating over a set of Variants, when `?X` happens to be an algebraic
type. In this case, the value stored is a unique identifier for a particular partial union over the known variants of an abstract ADT, that may not be fully populated with its full set of reachable variants until the entire tree is fully processed.

If two variables with such constraints are ever unified, the contents of these
partial sets are reconciled by unifying the `UType`s associated with any
variants with a common name between the two sets, and otherwise taking the union
of the two sets, and repointing the two metavariables to the same identifier
(`VMId`) to ensure that by the end of the process, all metavariables that point
to the same union-type agree on the full set of variants in that union.

### Basic Unification Guide

Below is a guide for a variety of 'shapes' of `Format`, `Pattern`, and `Expr`
primitives that might be added, with a description and example of how the rules
for said primitive ought to be implemented, with an actual example from the
source-code if one exists.

(As patterns are low-level, there is a lot more context-sensitivitiy to implementing them, and so we focus mainly on `Expr` and `Format` in the first draft).

- Extend `infer_var_XXX` where XXX is the appropriate one of `format`, `expr`, or `scope_pattern`, with a new match-case
- Within the body of the match-case:
  - Always generate the UVar for the current node first

    ```rust
    let newvar = self.get_new_uvar();
    ```

  - Recurse into the appropriate `infer_var_YYY` calls with try (`?`) and the correct contextual argument (either `Ctxt` or `UScope`). The order in which the arguments of super-unary primitives are subjected to recursive inference is arbitrary but must be consistent, and is the order in which they appear in the definition by convention, though any order would be valid as long as it is reflected appropriately in the elaborator layer as well.

    ```rust
    let fmt_var = self.infer_var_format(fmt, ctxt)?;
    let expr_var = self.infer_var_expr(expr, ctxt.scope)?; // from infer_var_format
    let expr_var = self.infer_var_expr(expr, scope)?; // from infer_var_expr
    let (in_var, out_var) = self.infer_vars_expr_lambda(lambda_expr, scope)?; // from infer_var_expr
    ```

  - Perform any local unifications with constraints, UTypes, or UVars as appropriate based on the semantics of the primitive in question:
    - Use projective unifications for tuples of unknown arity, records of unknown field-sets, and sequences of unknown element-type

    ```rust
    self.unify_var_proj_index(tup_var, index, var_at_index)?; // requires that ?X[i] ~ ?Y
    self.unify_var_proj_field(rec_var, fieldname, var_in_field)?; // requires that ?X.field ~ ?Y
    self.unify_var_proj_elem(seq_var, elem_var)?; // requires that ?X ~ [?Y]
    ```

    - Use BaseSet unifications for types that must be numeric in order to unify properly (either as `self.unify_utype_baseset` or `self.unify_var_constraint`)

    ```rust
    let itype = self.infer_utype_expr(i, scope)?;
    let jtype = self.infer_utype_expr(j, scope)?;
    let ktype = self.infer_utype_expr(k, scope)?;
    let _ = self.unify_utype_baseset(itype, BaseSet::UAny); // any member of U* (no default)
    let _ = self.unify_utype_baseset(jtype, BaseSet::USome); // any member of U* (U32 by default)
    let _ = self.unify_utype_baseset(ktype, BaseSet::U(UintSet::Short8)); // U8 or U16, U8 default
    ```

    (If required by novel primitives, extending BaseSet or UintSet may be appropriate).

    - Use bidirectional unifications when direct-variable equivalence or shapeful relation is known

    ```rust
    let varx = self.infer_var_expr(x, scope)?;
    let vary = self.infer_var_expr(y, scope)?;
    let varz = self.infer_var_expr(z, scope)?;
    self.unify_var_pair(x, y)?; // if ?X ~ ?Y
    self.unify_var_utype(x, Rc::new(UType::Seq(Rc::new(UType::Var(z)))))?; // if ?X ~ [?Z]
    ```

    - Generate ancillary variables as necessary to populate holes in shapeful types that require an extra constraint in terms of novel meta-variables.

    ```rust
    let seq_var = self.inver_var_expr(seq, scope)?; //
    let elem_var = self.get_new_utype();
    self.unify_var_proj_elem(seq_var, elem_var)?;
    ```

  - Always return `Ok(newvar)` at the end of the block

- Respect the same order in the Elaboraator
  - Whenever `get_new_uvar()` is called, either directly or implicitly through an embedded call to `infer_var_XXX`, ensure the index is incremented accordingly (and in the same order)
  - Call `get_gt_from_index` on the indices as appropriate to generate the corresponding types for `TypedFormat`, `TypedExpr`, and `TypedPattern` primitives (which should be added as appropriate)
