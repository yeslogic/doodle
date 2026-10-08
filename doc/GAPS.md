# Design Gaps

Certain areas of the current implementation of `doodle` fall short of the desired behavior of various edge-case handling.

## TC: `Expr::AsU*` Tiebreaks as No-Op Over `UAny`

As currently implemented, the `TypeChecker` requires all metavariables given a `UintSet` constraint,
even those for interior-nodes whose actual types may not need to be unambiguous for the overall format's
type to be fully unambiguous, have an unambiguous resolution to a `BaseType`.

A new unit test in `typechecker.rs`, [`expr_as_cast_over_uany_forces_type`](/src/typecheck.rs#L4622), exercises a desired
specialization: if a node within an `AsU32` cast (or equivalently for the other fixed-output-type unsigned-int casts)
is constrained only by `BaseSet::UAny`, or similarly has multiple valid solutions without a clear tiebreaker, the presence
of the `AsU32` cast itself should resolve the ambiguity to force the inner and outer node-types to agree, such that the
`Expr::AsU*` wrappers act can act as a 'noop-cast for type disambiguation' even for `UAny`-constraints that have no other
source of disambiguation.

Running the test with the following command demonstrates the actual behavior on `UAny` nodes, with test-failure proving
that the gap exists:

```bash
cargo test --features tc_seq_length_polymorphic --lib -- expr_as_cast_over_uany_forces_type --no-capture
```

(`tc_seq_length_polymorphic` is a feature that applies `UAny` as the constraint for `Expr::SeqLength`, otherwise defaulting to `UAny32`).

Running normally doesn't show the actual behavior we are attempting to demonstrate, as `UAny32` forces a default of `U32` even in the
absence of outside information that narrows the choice.
