# MatchTree Design Documentation

This document is to serve as an external explanation of the design, model, and API of the
`MatchTree` infrastructure within `doodle`. It should be viewed as a living document, and may not
precisely reflect the most up-to-date implementation as details change. However, it is easier to include
longer digressions on the model of a given type or operation within the system, in a separate document,
rather than crowd the definition sites in the library source file itself.

## Terminology

Abstractly, we may refer to the model of a `MatchTree` in terms of a trie whose nodes are *choice points* and whose children are *conditioned* on a particular set of values for the immediate byte of input at the current lookahead-offset into the buffer being processed.

By 'choice point', we mean an individual multi-way decision between one of K possible one-step descents.
Each such descent is associated (and conditioned on) a particular set of byte-values, which are mutually disjoint across any two branches of the same choice-point.

We may refer to one or more of the byte-sets upon which a choice-point is conditioned, as the *branching-predicates* of said node.

## Core Types

The `MatchTree` design consists of three core types, and one auxiliary type:

* [`MatchTree`](#matchtree-type-definition): a finalized prefix-tree evaluated to an abstract, but consistent depth across all branches (modulo short-circuit pruning in cases where acceptance or rejection is unconditional)
* `MatchTreeLevel`: an aggregation of choice-points at a common descent-depth into a `MatchTree`; alternatively, the unified logic for matching a byte at a given lookahead distance into the current buffer, which may be accumulated for each depth from 0 up to the maximal lookahead and thereby form a full `MatchTree`.
* `MatchTreeStep`: a single choice-point somewhere within a `MatchTree` that specifies the follow-set for each possible one-byte prefix of unprocessed input; alternatively, a deterministic choice-point of which a `MatchTreeLevel` models a superposition

Additionally, the `Next` type acts as a simplified deconstruction of a `Format` that may be partially processed (consumed).

### MatchTree Type Definition

Though subject to change, `MatchTree` is defined as:

```rust
pub struct MatchTree {
    accept: Option<usize>,
    branches: Vec<(ByteSet, MatchTree)>,
}
```

It is the highest-level structure within the `MatchTree` family of types, and is typically constructed
from a recursive build-up of `MatchTreeLevel`s corresponding to each depth from `0` to the maximal look-ahead distance; each such `MatchTreeLevel` may in turn be constructed from an unification over a
collection of `MatchTreeStep`s, representing each choice-point that can be found at the corresponding depth.

#### API Notes

The `MatchTree` type currently has two primary methods:

* `MatchTree::build(module: &FormatModule, branches: &[Format], next: Rc<Next<'_>>) -> Option<Self>`
* `MatchTree::matches(&self, input: ReadCtxt<'_>) -> Option<usize>`

`MatchTree::build` accepts an array slice `branches`, which is implicitly
referenced by the `accept` field of the `MatchTree` struct, as well as in the
branch-indices stored within the transient `MatchTreeLevel`s that are built up
into the `MatchTree` that is eventually returned.

`MatchTree::matches` indicates what unambiguous branch-index, if one exists, corresponds to the immediate byte-sequence yielded by looking into `input` up to the first unconditional choice-point, or
running out of look-ahead. In the former case, `Some(n)` is returned where `n` is the original index of the `Format` in the `branches` parameter passed to `MatchTree::build`.

### MatchTreeStep[^1]

[^1]: Although `MatchTreeLevel` is the next-highest-level type in the hierarchy, it is actually easier to describe in the context of `MatchTreeStep`, and so we will cover that type first.

A `MatchTreeStep` models the choice-point of possible one-byte prefixes and
subsequent follow-sets resulting from an arbitrary committed descent into a `Format`.

In the absence of `Union` or `Repeat` (or any variation thereof), this
is purely a matter of reading the static sequence of bytes that the format
expects for each position `0..=N`, for the arbitrary depth `N >= 0` at which the `MatchTreeStep` is found.

If the top-level format, or any of its sub-terms, have multiple alternative paths (e.g. explicitly as in a union, or implicitly as in an unbounded repetition), then each `MatchTreeStep` will correspond to a committed choice of a certain path, and will therefore stem from a node with a branching factor >= 2.

The concrete implementation, subject to change, is

```rust
struct MatchTreeStep<'a> {
    accept: bool,
    branches: Vec<(ByteSet, Rc<Next<'a>>)>,
}
```

where `branches` is an arbitrarily-ordered list of pairs `(s, n)`, where each `s` is a set of possible values for the next byte of input, and `n` is the residual format (`Next`s) entailed by that choice of prefix.

The field `accept` is set to `true` in order to signal that any subsequent input, including the lack of any remaining bytes, is acceptable under the given format, and that no further processing is necessary to disambiguate possible alternatives. If `accept` is `true`, it is to be expected that `branches` is empty; in the case that `accept` is false and `branches` is empty, it is to be understood that the committed choice of prefix has no valid interpretation within the original format, regardless of what input follows. This is a special case of the general rule that, if `accept` is `false`, any byte-values that are not members of any byte-set in `branches`, or end-of-input, would both mark the current input as a non-match for the chosen descent that led to a the current `MatchTreeStep`.

### MatchTreeLevel

Concisely, a `MatchTreeLevel` represents the unified branching conditions of all
choice-points at a consistent lookahead distance. That is, in a tree diagram
with a constant vertical height for all edges between nodes and their children,
a `MatchTreeLevel` can be thought of as a stratum found at a fixed depth.

Though subject to change, it is currently defined as:

```rust
#[derive(Clone, Debug)]
struct MatchTreeLevel<'a> {
    accept: Option<usize>,
    branches: Vec<(ByteSet, LevelBranch<'a>)>,
}
```

where the `accept` field has the same semantics the like-named field in `MatchTree`.

The `branches` field is defined in terms of a type-alias `LevelBranch<'_>`,
outlined in the subsection found below

#### LevelBranch

The type-alias `LevelBranch` is defined as follows:

```rust
type LevelBranch<'a> = HashSet<(usize, Rc<Next<'a>>)>;
```

To understand the significance of this type, we make the following observation:

In order for a `MatchTree` to be well-formed, it and all of its descendants
must have mutually disjoint branching-predicates. That is, for all well-formed `tree : MatchTree`, it must be the case that for all values of `b : u8`, the iterator generated by `tree.branches.filter(|(s, _)| s.contains(b)))` must yield zero or one elements only.

However, the same does not hold for `MatchTreeLevels`, as they represent the superposition of all choice-points at a common depth. If that depth is non-zero, the `MatchTree` may still be well-formed even if two choice-points within the `MatchTreeLevel` have some non-null intersection among the cartesian product of their respective lists of branching-predicates.

For this reason, a `MatchTreeLevel` cannot represent a single, unambiguous descent into a follow-set for any given byte of input, but rather, must form a disjunction over the smallest set of mutually disjoint subsets of all the branching predicates of its constituent choice-points, such as in the following algorithm (in pseudo-code):

```none
UnifySteps(Steps: [(FormatIx, MatchTreeStep)]):
    Instantiate a variable X : MatchTreeLevel, with X.branches := [] and X.accept = None
    For each element (Ix, Step) in Steps:
        If Step.accept
        Then
            If X.accept ≣ None || X.accept ≣ Some(Ix)
            Then
                X.accept := Some(Ix)
            Else
                Return None
        Else
            For each element (BS, Next) in Step.branches:
                Create a new temporary buffer NewBranches := []
                For each element (BS0, Branch) in X.branches:
                    Let Common := BS ∩ BS0
                    If Common ≠ ∅
                    Then
                        Replace (BS0, Branch) with (Common, Branch ⊕ [(Ix, Next)]), mutably in-place
                        Push (BS0 ∖ Common, Branch) to the end of NewBranches
                        Remove all elements of Common from BS, mutably in-place
                    Else
                        Do nothing
                If BS ≠ ∅
                Then
                    Push (BS, [(Ix, Next)]) to the end of X.branches
                Else
                    Do nothing
                Append all elements of NewBranches to the end of X.branches
    Return Some(X)
```

(This is effectively an outline of a a left-fold expansion of the method `MatchTreeLevel::merge_step`).

## Cycle guard for self-referential formats

Before recursive-format support was added to `doodle`, the eager descent performed by `MatchTreeStep::from_format`
when encountering `Format::ItemVar` could never form cycles, as the only way for a format to refer to itself
(outside of a phantom context that is exempt from MatchTree analysis) or form mutual cycles would be to artificially
create an `ItemVar` pointing to an as-yet-uninhabited level, which would break at time of registration when the
out-of-bounds level is used as an index into the module's `formats` field.

However, the introduction of support for auto- and mutually-recursive formats breaks this invariant by allowing
batch-definition with `RecVar`, which makes it possible to have self-referential or co-referential cycles when
expanding `Format::ItemVar` on a properly defined format. Because of this, any time a `MatchTreeStep` would
normally descend on a `Format::ItemVar` whose level it has already witnessed in the same call-stack, it needs
to detect the cycle and escape it rather than recurse indefinitely.

To address this, `MatchTreeLevel::grow` instantiates a fresh mutable state-object, of novel type `CycleGuard`,
which it threads through each call into `MatchTreeStep::from_next`. All of the related methods of `MatchTreeStep`
that can call into `from_format` (e.g. `from_format`, `from_sequential`, `from_repeat_count`) now take an additional
parameter of type `&mut CycleGuard`.

`CycleGuard`, defined in `src/lib.rs`, is a struct with two fields:

* `open_levels`: a set of 'open' `ItemVar` levels
* `has_cycle`: a boolean flag used to indicate that a cycle was detected, to avoid having to panic or introduce fallibility to `MatchTreeStep::from_format`

The concept of an 'open' level is somewhat circular, but it is best summed up as follows:

```pseudocode
MatchTreeStep::from_format(Module, F, Next, CycleGuard):
    if Format::ItemVar(Level, ..) == F:
        if Level is marked open in CycleGuard:
            CycleGuard->has_cycle := true
            return MatchTreeStep::reject()
        else:
            mark Level as open in CycleGuard
            let Ret = from_format(Module, Module.get_format(Level), Next, CycleGuard)
            mark Level as closed in CycleGuard
            return Ret
    ...
```

Namely, a call that sees `ItemVar` opens the corresponding level just before, and closes it just after, it performs the usual
recursive call on `module.get_format(level)`. Within that call-stack, `level` remains open and is only closed once the
recursive call returns a value.

Upon reaching `Format::ItemVar(level, ..)`, `from_format` inserts `level` into the open-set of the `CycleGuard` it is holding,
using the return-value to determine whether a cycle has been found. Provided the exit-condition is not already met,
it then performs the usual recursive introspection into the expanded format at `level`, and once that call terminates,
removes `level` from the `CycleGuard` to 'close' it out.

Because `MatchTreeStep` constitues exactly one byte of lookahead down a given parse-branch,
any path where a byte is unconditionally consumed (most commonly `Format::Byte`) forms a base-case
for the recursive definition of `from_format`, and the remainder of the format is delegated to delayed
processing via `Next`. Because each call that `MatchTreeLevel::grow` makes to `MatchTreeStep::from_next`
instantiates a fresh `CycleGuard`, any cycle witnessed during `from_format` processing is necessarily
an instance of unguarded left-recursion, as it would not have reached the second occurrence if even one
byte had been consumed since that level was opened.

After the call to `MatchTreeStep::from_next` returns to the original `MatchTreeLevel::grow` that called it,
the `CycleGuard` that was mutably passed into that call is inspected, and if the `has_cycle` flag
is set to `true`, the function returns early with a value of `None` (`MatchTreeLevel::grow` returns `Option<MatchTree>`).
