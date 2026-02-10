# Hieronyms (Hierachical Names) Design Pattern

While traversing the `main` rooted Format-tree, each node we encounter has an associated ValueType.

Whenever that ValueType necessitates an ad-hoc type declaration, we must decide on what identifier
to use for that declaration.

The current scheme uses a hierarchical naming convention, where the name a node's type is informed solely by the descent-path from the root of the Format tree to reach that node. In certain cases, only the tail of the path is used, with the top node being the closest ancestor that
is given an explicit identifier via FormatModule registration.

## Concepts

### Poisoning

In certain cases where the node we are currently sitting at is unrelated to a local descent, such as the argument `Expr`s in a `Format::ItemVar`, or for `Expr::Var` where we know the binding must already exist elsewhere and so the local path is not a true path to reach the type in question.

For this purpose, we temporarily insert a `DeadEnd` token to the path-stack, which prevents any path-suffix containing it from being considered a valid candidate. That way, only true paths to the type remain in consideration.

## Deduplication Model


Priors

- `RustTypeDecl` (implements Hash)
- `UVar` (currently not used beyond elaboration)
- `ix: usize` (index in ad-hoc type inventory)

Model

- `PathLabel`: have to properly book-keep
- `Label`: serialized identifier that at least one PathLabel is in contention for
- `PHeap`: deferred Priority-Heap where assignment of priority is order-of-reification

### Goals

1. Two declarations must not share the same name
2. Every named type should have the most concise available name (fewest number of NameAtoms)
3. Name-candidates that aren't manifest should not sneak ahead of manifest name-candidates

### One-to-Many

- Each `RustTypeDecl` can have multiple PathLabels leading to it
- Each `Label` can have multiple PathLabels that serialize to it

### Process

During elaboration, we instantiate a new GenType
