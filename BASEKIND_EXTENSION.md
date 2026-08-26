# `BaseKind` Extension: Signed Integers (and punted U24)

Context file for resuming work on the `BaseKind` FIXMEs originally in `src/marker.rs`:

```rust
// FIXME[epic=exotic-int-parse] - add support for U24
// FIXME[epic=signed-parse] - add support for signed integers
```

## Status: signed-integer support landed

`BaseKind<X>` now has `I8`/`I16Ext(X)`/`I32Ext(X)`/`I64Ext(X)` alongside the existing unsigned
variants, following the codebase's existing "signedness is a sibling variant, never a flag"
precedent (`ValueType::Base`/`Signed`, `AtomType::Prim`/`Signed`). Parsed signed values land
in the existing `Value::Numeric(Rc<TypedConst>)` representation, matching what `i32be()`
already produced before this change — no new flat `Value::I*` variants. A new `BaseNumType`
sum type (`src/valuetype.rs`, mirroring `NumType`'s `{U(..), I(..)}` shape) replaced the old
`BaseKind -> BaseType` conversion, which could no longer stay total once signed variants
existed; its three call sites (`src/lib.rs`, `src/typecheck.rs`, `src/codegen/mod.rs`) plus a
fourth found only via full compilation (`src/alt.rs`) were all migrated. `MarkerType`
(`src/codegen/rust_ast/mod.rs`) gained `I8`/`I16Be`/`I32Be`/`I64Be` variants backing the
`ReadUnchecked`-trait-derivation codegen path (`src/codegen/model/traits.rs`) for
`FixedFormat` records with signed fields.

Verified: `cargo check --workspace` (excluding `smallsorts`/`analytic-*`), `cargo testall`,
and `cargo fmt -- --check` all pass clean. The `src/fixed.rs` regression test
(`as_base_kind_read_accepts_signed`) that used to be `#[should_panic]` now passes for real.

Remaining source anchors (`// ANCHOR - <name>`, findable via `grep -rn "ANCHOR - " src/`) are
pared down to just the two items below — everything else was resolved and its anchor removed.

## Undone work

### 1. Scalar signed `ReadArray`-over-`View` codegen is unimplemented

`view-signed-readarray-gap` (`src/codegen/model.rs`, in `read_array_from_view`). This is a
*different* codegen path than the `MarkerType`/`ReadUnchecked` one that now works (above) —
it handles `ViewFormat::ReadArray(len, kind)` where `kind: FixedReadKind::Base(..)` names a
**bare primitive** directly (`ReadArray<'_, i32>`, not a record containing an `i32` field), by
emitting a call to a named method on `View` (`view.read_array_u32be(len)`, etc.). No
`read_array_i8`/`i16be`/`i32be`/`i64be` methods exist on `View` (location not yet confirmed —
presumably `parser/view.rs`) to emit calls to, so the signed arms currently `unimplemented!()`,
mirroring the pre-existing LE-unimplemented arms in the same function. Nothing in
`doodle-formats` needs this today. To implement: add the four methods to `View`, then wire
them into the match at `view-signed-readarray-gap`.

The interpreter-side equivalent (non-codegen, i.e. running `doodle file`) has no such gap —
`ReadCtxt::read_base` (`src/read.rs`) handles all signed cases fully.

### 2. U24Be — fully punted

Investigated, then explicitly descoped ("let's punt on that issue"). Unlike signed, U24 has no
foothold anywhere above the parse layer: `BaseType` (`basetype-enum`, `src/valuetype.rs`) has
no `U24` variant, nor does the `PrimInt`/`MachineRep` 8-variant set. Per the user, this is
fine and expected: **U24Be's value-type is just `U32`** (no dedicated value-type needed).

The real complication is codegen-specific: **the value type and the read/wire type diverge
for U24, and nowhere else.** For every other `BaseKind` variant, "the type a scalar decode
function returns" and "the type `ReadArray`'s element parameter uses" are the same primitive.
For U24Be: a scalar read must yield a plain `u32` (only reading 3 bytes), while `ReadArray`'s
element type must specifically be `smallsorts::binary::U24Be` (not `u32`, not a wrapper reused
from U32). Whatever code derives "the Rust type for this `BaseKind`" — `markertype-enum`
(`src/codegen/rust_ast/mod.rs`) for the `ReadArray` element type, `PrimType`/`AtomType` for
the scalar value type — would need to bifurcate into two genuinely different queries that
happen to coincide for U8/U16/U32/U64 but diverge for U24. `smallsorts` already provides
`ReadArray` support for `U24Be` (marker-type + `ReadUnchecked` impl both already exist), so —
as with signed — no `smallsorts` changes would be needed; it's purely a `doodle`-side wiring
problem.

Not investigated: whether `src/fixed.rs`'s `SpineElem`/`FixedShape` stride computation needs
the same bifurcation, or can stay in terms of byte-widths throughout, unaffected. `basekind-enum`
(`src/marker.rs`) still carries the `FIXME[epic=exotic-int-parse]` marking where the U24
variant would go; `as-base-kind-read` (`src/fixed.rs`) still documents the U24Be gap in its
doc-comment; `markertype-enum` is the obvious place a `U24Be` marker-type arm would land.
