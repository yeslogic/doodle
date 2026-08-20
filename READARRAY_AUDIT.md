# ReadArray-eligibility audit — `doodle-formats/src/format/opentype/**`

Distilled re-assessment of `ViewFormat::ReadArray` eligibility (`src/fixed.rs`, formerly
`src/record_fmt.rs`) against `repeat_count(len, g)` sites in the OpenType format tree. This
supersedes the original comment-only audit: most of that audit's "blocked by a checker gap"
findings have since been either closed by implementation work or acted on directly in the format
definitions, and are pruned below rather than re-derived. Status verified against the current
source, cross-checked via `mcp__cclsp__find_references` on `doodle::helper::read_array`.

## Already migrated (live `read_array(...)` call sites)

| Site | Pattern |
|---|---|
| `cpal.rs::table.color_records_array` | `pseudo_record` + `with_view` + `read_array` (the original reference migration) |
| `cpal.rs::palette_labels_array` / `palette_entry_labels_array` | generic `opentype.rs::read_array_view_offset32` helper |
| `hdmx.rs::device_record.widths` | `from_here(read_array(..., BaseKind::U8))` |
| `base.rs::base_tag_list.baseline_tags` | `from_here(read_array(..., tag))` |
| `stat.rs::design_axes_array.design_axes` | `from_here(read_array(..., axis_record))` |
| `stat.rs::axis_value_table` Format4 `axis_values` | `from_here(read_array(..., axis_value_record))` |
| `colr.rs::color_line.color_stops` | `from_here(read_array(..., color_stop))` |
| `colr.rs::var_color_line.color_stops` | `from_here(read_array(..., var_color_stop))` |

`hdmx.rs` was misclassified as zero-qualifying in the original audit (false negative in the first
pass) — it has since been both correctly identified and migrated.

## Deliberately opted out

- **`opentype.rs::table_directory.table_records`** — genuinely eligible (`table_record.table_id`
  is `tag.call()`, resolvable since the indirection gap closed) but intentionally left as
  `repeat_count`, per the inline `NOTE` at the site: the parsed `Vec<TableRecord>` needs to stay
  fully materialized in memory for `table_links` to consume. `ReadArray` would trade that away for
  a lazy/strided view, which isn't wanted here. Do not re-flag this site.

## Checker gaps that have been closed (historical, no longer blocking)

The original audit's largest section catalogued sites blocked *solely* by two gaps in
`analyze_fixed_shape`/`as_base_kind_read`. Both are now resolved in `src/fixed.rs`:

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

Of the original 10 "blocked solely by a gap" sites, 5 were subsequently migrated (`base.rs`
`baseline_tags`, `stat.rs` ×2, `colr.rs` ×2, all listed above), 1 was checked and explicitly
declined (`opentype.rs` `table_records`), and 4 remain open: 1 genuine not-yet-done quick win
(`mvar.rs`) and 3 that were tried and reverted for a real reason (`avar.rs`/`fvar.rs`/`gvar.rs`) —
see the two sections below. This accounts for all 10; nothing from that section is
unresolved-and-unaccounted-for.

## Remaining low-hanging fruit (gap-closed, not yet migrated)

These are now fully eligible under current rules — no remaining architectural blocker — but the
migration hasn't been done. Each just needs `from_here(read_array(len, elem))` in place of the
existing `repeat_count`:

| Site | Elem kind |
|---|---|
| `mvar.rs::table.value_records` | `value_record` (reuse existing `FormatRef`; drop `.call()`) |
| `cpal.rs::table.color_record_indices` | bare `BaseKind::U16BE` |

`mvar.rs` still carries the old audit's inline comment describing `tag.call()` as "the sole
blocker" — that's now stale (the blocker is closed; the site is just unmigrated) but harmless.

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

### 1. Signed-integer primitives — live examples, not yet tracked by any epic tag

`i8()`/`i16be()`/`i32be()`/`i64be()` (`src/helper.rs`) are `map_numeric`-wrapped around the
corresponding unsigned `EndianParse` read, so `as_base_kind_read` doesn't recognize them
(`fixed.rs`'s own `as_base_kind_read_accepts_signed` test is `#[should_panic]`, documenting this as
still-open). This is no longer a theoretical gap — there are concrete, currently-blocked array
sites:

- `gvar.rs` — `fmt_variant("Delta16", repeat_count(var("run_length"), i16be()))` and the `Delta8`
  sibling using `i8()` (glyph-variation-data point deltas).
- `hmtx.rs` — `long_metrics`' inline `long_horizontal_metric` record (`left_side_bearing: i16be()`,
  which would also need promoting from an inline `record([..])` to a registered `FormatRef` to
  qualify as `FixedFormat`) and the bare `left_side_bearings: repeat_count(.., i16be())` array.
- `post.rs` — `offset: repeat_count(var("num_glyphs"), i8())`.

`vmtx.rs` almost certainly mirrors `hmtx.rs` (top-side-bearings) but wasn't independently
re-checked. Of the remaining gaps, this is the one with the broadest, best-attested live impact —
worth prioritizing over the other three below if this work resumes.

### 2. Bitflags-derived fields — live examples, tracked via `TODO[epic=adhoc-readarray]`

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

### 3. Nested `FixedFormat` record fields — hypothetical, no live example found

`analyze_record`'s field-level `as_spine_elem` call only recognizes bare primitives and
primitive-resolving indirection/Variant; a field that is itself another fixed-shape *record*
(rather than a scalar) is unhandled (`fixed.rs`'s
`analyze_fixed_shape_accepts_record_with_record_field` test is `#[should_panic]`, documenting
this). A light search for a record-typed field reused inside another flat record (as opposed to
behind an offset/phantom indirection, which is by far the dominant pattern in this codebase) turned
up nothing concrete in the current OpenType tree. Given how often OpenType nests small fixed
sub-records (value records, anchor-adjacent tables), this is plausible to hit eventually, but
speculative — not worth deeper searching until a concrete site motivates it.

### 4. Little-endian `ReadArray` codegen — hypothetical for OpenType specifically

`fixed.rs` structurally recognizes LE `BaseKind`s (via the same `CommonOp::EndianParse` tag as BE),
but the codegen/runtime layer doesn't: `src/codegen/model.rs::read_array_from_view` still has
`unimplemented!("little-endian read-array parses not yet implemented")` for
`U16LE`/`U32LE`/`U64LE`, and `src/parser/view.rs` only defines `read_array_u{16,32,64}be` (no `le`
counterparts) — unlike scalar reads, where `5eb4fbf` added `read_u16le`/`read_u32le`/`read_u64le`
and removed the equivalent `unimplemented!()` for non-array LE reads. OpenType itself is
exclusively big-endian, so there is no live site anywhere in this tree that this would unblock;
flagged only because it's a concrete, already-documented asymmetry (scalar LE works, array LE
doesn't) that would surface immediately if a little-endian format ever wanted `ReadArray`.

### 5. Tuple-shaped elements — unchanged, no live example, not re-investigated

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

## Zero-qualifying files (re-confirmed, `hdmx.rs` removed — see "Already migrated")

`var_common.rs`, `svg.rs`, `dsig.rs`, `name.rs`, `head.rs`, `hvar.rs`, `colr.rs` (outside the two
migrated sites above), and `maxp.rs`/`vhea.rs`/`cvt.rs`/`fpgm.rs`/`prep.rs`. `hhea.rs` presumed
still empty (unchanged, not re-checked). `hmtx.rs`/`vmtx.rs` remain zero-qualifying *under current
rules* but are the best-attested live examples for gap 1 above (signed primitives) rather than
being architecturally dead ends.
