# ReadArray-eligibility audit — `doodle-formats/src/format/opentype/**`

Distilled re-assessment of `ViewFormat::ReadArray` eligibility (`src/fixed.rs`, formerly
`src/record_fmt.rs`) against `repeat_count(len, g)` sites in the OpenType format tree. This
supersedes the original comment-only audit: most of that audit's "blocked by a checker gap"
findings have since been either closed by implementation work or acted on directly in the format
definitions, and are pruned below rather than re-derived. Status verified against the current
source, cross-checked via `mcp__cclsp__find_references` on `doodle::helper::read_array`.

Re-checked as of commit `abe64ee` (previously last reviewed at `8d8ff0d`). The 5 commits in
between (`9e4e6bb`, `f4d1e1a`, `eb81afc`, `9ce426a`, `abe64ee`, plus `aea08b6` in passing) landed
full signed-BE-primitive support end to end — see "Signed-integer primitives" below, which moves
from "genuine remaining gap" to "closed".

## Already migrated (live `read_array(...)` call sites)

| Site | Pattern |
|---|---|
| `cpal.rs::table.color_records_array` | `pseudo_record` + `with_view` + `read_array` (the original reference migration) |
| `cpal.rs::palette_labels_array` / `palette_entry_labels_array` | generic `opentype.rs::read_array_view_offset32` helper |
| `base.rs::base_tag_list.baseline_tags` | `from_here(read_array(..., tag))` |
| `stat.rs::design_axes_array.design_axes` | `from_here(read_array(..., axis_record))` |
| `stat.rs::axis_value_table` Format4 `axis_values` | `from_here(read_array(..., axis_value_record))` |
| `colr.rs::color_line.color_stops` | `from_here(read_array(..., color_stop))` |
| `colr.rs::var_color_line.color_stops` | `from_here(read_array(..., var_color_stop))` |

`hdmx.rs` was misclassified as zero-qualifying in the original audit (false negative in the first
pass) — it was migrated to `read_array(..., BaseKind::U8)` for a while, but commit `aea08b6`
("Implement obj::PairS (PairSet) in otf_metrics") reimplemented `device_record.widths` using
`capture_bytes` instead, per its inline `NOTE`: `CaptureBytes` makes downstream processing in
`otf_metrics` infallible in a way `ReadArray` didn't. This is a deliberate opt-out, not a
regression — treat `hdmx.rs` as no longer carrying a live `read_array` site.

## Deliberately opted out

- **`opentype.rs::table_directory.table_records`** — genuinely eligible (`table_record.table_id`
  is `tag.call()`, resolvable since the indirection gap closed) but intentionally left as
  `repeat_count`, per the inline `NOTE` at the site: the parsed `Vec<TableRecord>` needs to stay
  fully materialized in memory for `table_links` to consume. `ReadArray` would trade that away for
  a lazy/strided view, which isn't wanted here. Do not re-flag this site.
- **`hdmx.rs::device_record.widths`** — genuinely eligible (`BaseKind::U8`) and was migrated for a
  time, but reverted to `capture_bytes` in `aea08b6` so that `generated/api_helper/otf_metrics.rs`
  can treat the byte-width data infallibly downstream. Do not re-flag this site either.

## Checker gaps that have been closed (historical, no longer blocking)

The original audit's largest section catalogued sites blocked *solely* by two gaps in
`analyze_fixed_shape`/`as_base_kind_read`. Both are now resolved in `src/fixed.rs`, and a third gap
(signed primitives) discovered afterward has since been closed too:

- **`FormatRef::call()` indirection** is now resolved (`as_spine_elem`'s `Format::ItemVar` arm,
  added in `74f8c2c`/`4a7e22d`) — a field like `tag.call()` correctly resolves to `BaseKind::U32BE`
  via `SpineElem::Indirect`.
- **`Format::Variant`-wrapping** (`f2dot14()`/`fixed32be()`) is now resolved for two shapes: (a) a
  record field that is a `FormatRef::call()` pointing at a Variant-wrapped primitive (resolved via
  the same `ItemVar` arm), and (b) a top-level `FormatRef` whose own body *is* the `Format::Variant`
  node, when passed directly as the array's `FixedReadKind::FixedFormat` (the "self-referencing"
  arm of `as_spine_elem`). It is still **not** resolved for a bare inline `Format::Variant` sitting
  directly in a record field with no `FormatRef` anchor — that arm deliberately requires `self_ref`
  and record fields always pass `None` (see the doc comment on `as_spine_elem`).
- **Signed-integer primitives** (`i8()`/`i16be()`/`i32be()`/`i64be()`) are now resolved too, landed
  in the commit batch after this audit was first written (`9e4e6bb` -> `f4d1e1a` -> `eb81afc` ->
  `9ce426a` -> `abe64ee`, plus `aea08b6` in passing). `BaseKind<X>` gained `I8`/`I16Ext(X)`/
  `I32Ext(X)`/`I64Ext(X)` sibling variants (`src/marker.rs`); `as_base_kind_read`'s
  `as_base_kind_read_accepts_signed` test in `src/fixed.rs` — `#[should_panic]` at the time of the
  first audit — now passes for real. Codegen support landed too: `MarkerType` gained
  `I8`/`I16Be`/`I32Be`/`I64Be` variants backing both the `ReadUnchecked`-derivation path for
  `FixedFormat` records with signed fields (`src/codegen/model/traits.rs`) *and* the bare-primitive
  `ViewFormat::ReadArray` path (`model::read_array_from_view` -> `read_fixed_array_from_view` ->
  `View::as_read_array::<T>`, which `9ce426a` genericized over any `T: ReadUnchecked` instead of
  matching per-`BaseKind` as before). Full writeup: `BASEKIND_EXTENSION.md`. This closes the
  "signed primitives" gap from the original "Genuine remaining gaps" list below — but only its
  **big-endian** half; see "Little-endian `ReadArray` codegen" further down for the LE asymmetry
  this batch left untouched for arrays.

Of the original 10 "blocked solely by a gap" sites, 5 were subsequently migrated (`base.rs`
`baseline_tags`, `stat.rs` ×2, `colr.rs` ×2, all listed above), 1 was checked and explicitly
declined (`opentype.rs` `table_records`), and 4 remain open: 1 genuine not-yet-done quick win
(`mvar.rs`) and 3 that were tried and reverted for a real reason (`avar.rs`/`fvar.rs`/`gvar.rs`) —
see the two sections below. This accounts for all 10; nothing from that section is
unresolved-and-unaccounted-for.

## Remaining low-hanging fruit (gap-closed, not yet migrated)

These are now fully eligible under current rules — no remaining architectural blocker — but the
migration hasn't been done. Each just needs `from_here(read_array(len, elem))` (or the
`with_view`/`pseudo_record` variant, where there's no already-bound `ViewExpr` at the site) in
place of the existing `repeat_count`:

| Site | Elem kind |
|---|---|
| `mvar.rs::table.value_records` | `value_record` (reuse existing `FormatRef`; drop `.call()`) |
| `cpal.rs::table.color_record_indices` | bare `BaseKind::U16BE` |
| `post.rs::postv2dot5.offset` | bare `BaseKind::I8` (signed gap closed, see below) |
| `hmtx.rs::table.left_side_bearings` | bare `BaseKind::I16BE` (signed gap closed) |
| `gvar.rs::packed_deltas.run.deltas` (`Delta16`/`Delta8` arms) | bare `BaseKind::I16BE` / `BaseKind::I8` (signed gap closed) |

`mvar.rs` still carries the old audit's inline comment describing `tag.call()` as "the sole
blocker" — that's now stale (the blocker is closed; the site is just unmigrated) but harmless.

The three new signed-primitive rows became eligible only as of `eb81afc`/`9ce426a` — see "Signed-
integer primitives" under "Checker gaps that have been closed" below. None of them currently
carry a comment explaining why they're unmigrated (unlike the tried-and-reverted sites), since
until this commit batch there was a real, correctly-documented blocker. `hmtx.rs`'s
`long_horizontal_metric` (the `long_metrics` field, mixing `u16be()`+`i16be()`) would also need
promoting from an inline `record([..])` to a registered `FormatRef` to qualify as `FixedFormat`
before it can be migrated the same way; `left_side_bearings` (bare `i16be()`) needs no such
promotion. `vmtx.rs` almost certainly mirrors `hmtx.rs` (top-side-bearings) but wasn't
independently re-checked.

## Tried, reverted (real blocker, not just unmigrated)

`avar.rs::segment_maps.axis_value_maps`, `fvar.rs::user_tuple.coordinates`, and
`gvar.rs::tuple_record.coordinates` had their old eligibility comments deleted in commit `c4bc881`
without a migration landing — but unlike the two sites above, that wasn't just neglect. These three
were tried with `read_array` and reverted back to `repeat_count`: the generated code ended up with
novel lifetime parameters on the element type that hadn't been needed before, and the follow-up
fixes attempted in `generated/api_helper/*` didn't fully resolve it. No commit in the reachable
history captures the attempt-and-revert directly (checked `f9835b9..b22dfe4` on both the rebased
`main` copy and the original `archaephyrryx/fixed-size-readarray` branch — every diff touching
these three files only ever *removes* the eligibility comment, never adds/removes an actual
`read_array` call), so this is recorded from firsthand recollection rather than a citable commit.
Each site now carries an inline `// NOTE - despite being readarray-eligible, implementing with
`read_array` introduces complications due to novel lifetime parametricity in codegen output`
comment. Don't re-attempt these without also addressing the lifetime issue in the `ReadUnchecked`/
`View` codegen layer (`src/codegen/model/traits.rs`).

## Large unaffected batch (bare primitives / closed flat-primitive records, ~50 sites)

Unchanged by any of the above — these never depended on either closed gap, they were always
directly eligible (`Format::Hint(EndianParse(..), _)` or an all-primitive closed record). Spot-checked
(`glyf.rs::simple.end_points_of_contour`/`instructions`) and confirmed still present as
un-migrated `repeat_count` calls with the original audit's eligibility comments intact.

| File | Sites |
|---|---|
| `cmap.rs` | 13 — subtable format 0/2/4/6/8/10/12/13 glyph/group arrays |
| `layout.rs` | 11 — chained-sequence/sequence rule arrays, `sequence_lookup_record` arrays, `feature_table`/`lang_sys` index arrays, `condition_offsets` |
| `gsub.rs` | 6 — substitute/alternate/component glyph-ID arrays |
| `common.rs` | 6 — coverage/class-def range records, item-variation/device-table deltas |
| `glyf.rs` / `loca.rs` | 5 — contour end-points, instruction bytes, loca offset tables |
| `gpos.rs` | 4 — mark/base/ligature anchor-offset arrays |
| `gvar.rs` | 2 (4 branches) — glyph-variation-data offsets, point-number runs |
| `os2.rs` / `post.rs` | 2 — `panose`, `glyph_name_index` |
| `gdef.rs` / `vdmx.rs` | 2 — `attach_point.point_indices`, `vdmx::ratio_range` |

No individual re-verification was done per-site beyond the `glyf.rs` spot check; treat this table
as "presumed still valid" rather than freshly re-audited.

## Genuine remaining gaps, by evidence strength

Signed-integer primitives (formerly the top item here) are no longer a gap — see "Signed-integer
primitives" under "Checker gaps that have been closed" above; the live sites it used to block
(`gvar.rs`, `hmtx.rs`, `post.rs`) moved to "Remaining low-hanging fruit". `post.rs::table`'s
`postv2dot5` still carries a stale inline comment (`// TODO - ReadArray<'_, I8> would work here if
we had a model compatible with it`) that should be replaced with a migrated-or-unmigrated note
once that site is next touched.

### 1. Bitflags-derived fields — live examples, tracked via `TODO[epic=adhoc-readarray]`

A `bit_fields_u16`/`bit_fields_u32` record's only raw read is the packed-bits primitive; every
exposed field is a derived `Format::Compute` bit-mask, so `analyze_fixed_shape` rejects it outright
regardless of the indirection/Variant fixes.

- `cpal.rs::palette_types_array` — the one site already carrying the tracking tag
  (`TODO[epic=adhoc-readarray]`, `cpal.rs:110`).
- `fvar.rs::variation_axis_record` (the `#_axes` array) — now blocked *only* by its inline
  `axis_qual_flags: bit_fields_u16(..)` field, since its other two blockers (`tag.call()`,
  `fixed32be.call()`) are resolved. Not a clean "sole blocker" case, though: the array itself sits
  inside `phantom(parse_from_view(.., slice(.., repeat_count(..))))`, so even fixing bitflags
  wouldn't make this a trivial migration the way the other quick-wins above are.

### 2. Nested `FixedFormat` record fields — hypothetical, no live example found

`analyze_record`'s field-level `as_spine_elem` call only recognizes bare primitives and
primitive-resolving indirection/Variant; a field that is itself another fixed-shape *record*
(rather than a scalar) is unhandled (`fixed.rs`'s
`analyze_fixed_shape_accepts_record_with_record_field` test is `#[should_panic]`, documenting
this). A light search for a record-typed field reused inside another flat record (as opposed to
behind an offset/phantom indirection, which is by far the dominant pattern in this codebase) turned
up nothing concrete in the current OpenType tree. Given how often OpenType nests small fixed
sub-records (value records, anchor-adjacent tables), this is plausible to hit eventually, but
speculative — not worth deeper searching until a concrete site motivates it.

### 3. Little-endian `ReadArray` codegen — hypothetical for OpenType specifically, gap reshaped but not closed

Still open, but the shape of the gap changed under `9ce426a`/`abe64ee` and is worth re-describing
accurately. `read_array_from_view` (`src/codegen/model.rs`) no longer has per-`BaseKind`
`unimplemented!()` match arms at all — `9ce426a` collapsed it to a single generic call through
`MarkerType::from_base_kind_endian(kind)` into `read_fixed_array_from_view`, which emits
`view.as_read_array::<elem_ty>(len)`, and `View::as_read_array<T>` (`src/parser/view.rs`) is fully
generic over any `T: ReadUnchecked` — no more monomorphic `read_array_u16be`/`u32be`/`u64be`
methods to individually extend with `le` counterparts (`abe64ee` deleted those; only
`read_array_u24be` survives, kept as a thin wrapper since `MarkerType` has no `U24Be` case to route
through generically). The LE gap now lives one layer up: `read_array_from_view` asserts
`kind.is_be()` before calling in, and `MarkerType::from_base_kind_endian` itself panics on any
`Endian::Le` variant with `"MarkerType models allsorts::binary marker-types, which are all
big-endian"`. So this is no longer really a "missing codegen arm" so much as "no `MarkerType`/
`ReadUnchecked` impl exists for LE at all" — an `allsorts`/`smallsorts` limitation (LE isn't a
concept in OpenType binary data) rather than a `doodle`-side oversight, mirroring how scalar LE
reads (`5eb4fbf`'s `read_u16le`/`read_u32le`/`read_u64le`) work at the *value* level but were never
plumbed into the marker-type/`ReadUnchecked` machinery `ReadArray` depends on. OpenType itself is
exclusively big-endian, so there is still no live site anywhere in this tree that this would
unblock; flagged only because it's a concrete, already-documented asymmetry that would surface
immediately if a little-endian format ever wanted `ReadArray`.

### 4. Tuple-shaped elements — unchanged, no live example, not re-investigated

`analyze_fixed_shape` still rejects `Format::Tuple` outright (`analyze_fixed_shape_rejects_tuple`
test, by design — ad-hoc tuples get no generated type to hang a `ReadUnchecked` impl off). No
tuple-shaped array elements were found in the OpenType tree during the original audit; not
re-checked here since nothing about tuple handling has changed and no new evidence suggests this
status is stale.

## Unchanged architectural exclusions (compounded blockers, still excluded)

Confirmed still present and still blocked by the same *other* reasons documented in the original
audit (none of these were touched by the migrations above):

- `base.rs`'s `base_lang_sys_records`/`base_script_records`/`feat_min_max_records` — view-dependent
  (`.invoke_view(vvar("table_view"))`, formerly `.call_views()`).
- `layout.rs`'s `feature_records`/`script_records`/`lang_sys_records` — same view-dependency shape.
- `gvar.rs`'s `shared_tuples`/`tuple_variation_header` tuple arrays, `avar.rs`'s outer
  `axis_segment_maps` — element's own field is itself a nested array, not a scalar.
- `colr.rs`'s `affine2x3`/`var_affine2x3` — not used as array elements at all.

## Zero-qualifying files (re-confirmed; `hdmx.rs` excluded — see "Deliberately opted out", not zero-qualifying)

`var_common.rs`, `svg.rs`, `dsig.rs`, `name.rs`, `head.rs`, `hvar.rs`, `colr.rs` (outside the two
migrated sites above), and `maxp.rs`/`vhea.rs`/`cvt.rs`/`fpgm.rs`/`prep.rs`. `hhea.rs` presumed
still empty (unchanged, not re-checked). `hmtx.rs`/`vmtx.rs` are no longer zero-qualifying: the
signed-primitive gap that used to block them is closed (see "Remaining low-hanging fruit"), they
just haven't been migrated yet.
