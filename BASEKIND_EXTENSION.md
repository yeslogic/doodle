# `BaseKind` Extension: Signed Integers (and punted U24)

Context file for resuming work on the `BaseKind` FIXMEs originally in `src/marker.rs`:

```rust
// FIXME[epic=exotic-int-parse] - add support for U24
// FIXME[epic=signed-parse] - add support for signed integers
```

## Status: signed-integer support fully landed (scalar reads *and* codegen)

`BaseKind<X>` now has `I8`/`I16Ext(X)`/`I32Ext(X)`/`I64Ext(X)` alongside the existing unsigned
variants, following the codebase's existing "signedness is a sibling variant, never a flag"
precedent (`ValueType::Base`/`Signed`, `AtomType::Prim`/`Signed`). Parsed signed values land in
the existing `Value::Numeric(Rc<TypedConst>)` representation, matching what `i32be()` already
produced before this change. A new `BaseNumType` sum type (`src/valuetype.rs`, mirroring
`NumType`'s `{U(..), I(..)}` shape) replaced the old `BaseKind -> BaseType` conversion, which
could no longer stay total once signed variants existed; all four call sites (`src/lib.rs`,
`src/typecheck.rs`, `src/codegen/mod.rs`, `src/alt.rs`) were migrated. `MarkerType`
(`src/codegen/rust_ast/mod.rs`) gained `I8`/`I16Be`/`I32Be`/`I64Be` variants.

What was still undone at the end of that pass — scalar signed `ReadArray`-over-`View` codegen —
has since landed too, in three follow-up commits (`eb81afc` "Add support for signed read-array
codegen", `9ce426a` "Clean up `model::read_array_from_view`", `abe64ee` "Cleanup
`parser/view.rs`"), done independently of the earlier assistant session. Rather than adding
`read_array_i8`/`i16be`/`i32be`/`i64be` methods to `View` one at a time as originally sketched,
the fix went generic: `View::as_read_array<T: ReadUnchecked>` (`src/parser/view.rs`) replaced
essentially all of the old monomorphic `read_array_uNN` methods, and
`model::read_array_from_view` (`src/codegen/model.rs`) now just calls
`read_fixed_array_from_view(view, len, MarkerType::from_base_kind_endian(kind))`, which works
for any `MarkerType` — signed included — with no signed-specific code path left at all. The one
survivor is `read_array_u24be`, kept as a named wrapper around `as_read_array::<U24Be>` because
`U24Be` isn't (and can't be) named generically from `BaseKind`/`MarkerType` today (see U24Be
section below). `BaseKind::is_be()` (`src/marker.rs`) was added as part of this cleanup and is
used by `read_array_from_view` to assert-guard the pre-existing, unrelated LE-unsupported case
(allsorts' marker types are all big-endian) before dispatching.

Verified (current tree): `cargo check --workspace` (excluding `smallsorts`/`analytic-*`) passes
clean. The `src/fixed.rs` regression test (`as_base_kind_read_accepts_signed`) that used to be
`#[should_panic]` passes for real.

Remaining source anchors (`// ANCHOR - <name>`, findable via `grep -rn "ANCHOR - " src/`) are all
tied to the one remaining open item, U24Be: `basekind-enum` (`src/marker.rs`), `basetype-enum`
(`src/valuetype.rs`), `markertype-enum` (`src/codegen/rust_ast/mod.rs`), `as-base-kind-read`
(`src/fixed.rs`).

One loose end unrelated to the above: `src/lib.rs:854` still carries a pre-existing (predates
this effort, added April 2026) `// FIXME[epic=signed-parse] - add in expressivity for
signed-integer parsing as commonop` on `CommonOp::EndianParse(BaseKind<Endian>)`. Since
`BaseKind<Endian>` itself now covers signed kinds, `CommonOp::EndianParse` already carries that
expressivity — this FIXME reads as stale but hasn't been swept up by either pass; worth
double-checking against its original intent before deleting it.

## Undone work

### U24Be — fully punted

Investigated, then explicitly descoped ("let's punt on that issue"). Unlike signed, U24 has no
foothold anywhere above the parse layer: `BaseType` (`basetype-enum`, `src/valuetype.rs`) has no
`U24` variant, nor does the `PrimInt`/`MachineRep` 8-variant set. Per the user, this is fine and
expected: **U24Be's value-type is just `U32`** (no dedicated value-type needed).

The real complication is codegen-specific: **the value type and the read/wire type diverge for
U24, and nowhere else.** For every other `BaseKind` variant, "the type a scalar decode function
returns" and "the type `ReadArray`'s element parameter uses" are the same primitive. For U24Be: a
scalar read must yield a plain `u32` (only reading 3 bytes), while `ReadArray`'s element type
must specifically be `smallsorts::binary::U24Be` (not `u32`, not a wrapper reused from U32).
Whatever code derives "the Rust type for this `BaseKind`" — `markertype-enum`
(`src/codegen/rust_ast/mod.rs`) for the `ReadArray` element type, `PrimType`/`AtomType` for the
scalar value type — would need to bifurcate into two genuinely different queries that happen to
coincide for U8/U16/U32/U64 but diverge for U24. `smallsorts` already provides `ReadArray`
support for `U24Be` (marker-type + `ReadUnchecked` impl both already exist), so — as with signed
— no `smallsorts` changes would be needed; it's purely a `doodle`-side wiring problem.

The generic `View::as_read_array::<T: ReadUnchecked>` refactor (see above) actually makes this
gap narrower than it looked before: `src/parser/view.rs::read_array_u24be` already exists as the
one hand-written wrapper around `as_read_array::<U24Be>`, so the `View`-level plumbing for U24Be
`ReadArray` is done. What's still missing is entirely on the type-derivation side: nothing maps a
`BaseKind`/scalar-U32-producing U24 read to `MarkerType::U24Be` (there is no such `MarkerType`
variant) so that `read_array_from_view`/`read_fixed_array_from_view` could dispatch to it
generically instead of needing the special-cased method at all.

Not investigated: whether `src/fixed.rs`'s `SpineElem`/`FixedShape` stride computation needs the
same bifurcation, or can stay in terms of byte-widths throughout, unaffected. `basekind-enum`
(`src/marker.rs`) still carries the `FIXME[epic=exotic-int-parse]` marking where the U24 variant
would go; `as-base-kind-read` (`src/fixed.rs`) still documents the U24Be gap in its doc-comment;
`markertype-enum` is the obvious place a `U24Be` marker-type arm would land.
