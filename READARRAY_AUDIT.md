# ReadArray-eligibility audit — `doodle-formats/src/format/opentype/**`

Survey of every `repeat_count(len, g)` site in the OpenType format tree, checked against the
eligibility rules for `ViewFormat::ReadArray` / `FixedReadKind` (see `src/record_fmt.rs`,
`src/marker.rs`). Sites that qualified were annotated in place with `//` comments covering:
whether a new `FormatRef` is needed, whether the site needs `from_here(...)` or can be rewritten
from an offset+phantom-parse into `pseudo_record(...) + with_view(..., read_array(...))`, and (for
potentially-eligible sites) what missing category blocks them. A second pass then specifically
targeted sites blocked *only* by a checker gap (`FormatRef::call()` indirection to a primitive, or
`Format::Variant`-wrapped primitives like `f2dot14()`/`fixed32be()`) rather than by a genuine
shape/architecture mismatch — see the third section below. 21 files touched in total, no logic
changes; `cargo check -p doodle-formats` passes clean.

## Fully readarray-eligible sites (~50 annotated)

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
| `cpal.rs` | 1 new (`color_record_indices`) + pre-existing migrated example (`color_records_array`) |

## Potentially-eligible (needs new machinery)

- `cpal.rs`, `palette_types_array` — category **bitflags** (`bit_fields_u32` record; exposed fields
  are derived `Compute` bit-masks, not raw reads, so `analyze_fixed_shape` rejects it as-is).

No `Format::Tuple`-shaped array elements were found anywhere in the tree, so the **tuple** category
never actually applied in practice.

## Blocked solely by a checker gap — otherwise eligible (10 annotated)

These sites are **not** annotated as "readarray-eligible" outright — they're blocked by a real gap
in `analyze_fixed_shape`/`as_base_kind_read`, not by anything architectural — but would become
eligible immediately if that specific gap were closed, with no other blocker in play. Two gaps
account for all of them:

**`FormatRef::call()` indirection isn't resolved.** `as_base_kind_read` pattern-matches each
field's `Format` node directly and does not recurse through a nested `FormatRef::call()`
(`Format::ItemVar`). So a field like `tag.call()` — which itself resolves to a bare `u32be()` — is
disqualified today, despite being fixed-width in practice. This silently rules out any record field
reusing a shared primitive `FormatRef`, most commonly the ubiquitous 4-byte OpenType `tag`:

| Site | New `FormatRef` needed? | Needs `from_here`? |
|---|---|---|
| `opentype.rs`: `table_directory.table_records` (`table_record.table_id`) | no — reuse `table_record` | yes |
| `base.rs`: `base_tag_list.baseline_tags` (bare `tag.call()` element) | no — bare `BaseKind::U32BE` | yes (see note below) |
| `mvar.rs`: `table.value_records` (`value_record.value_tag`) | no — reuse `value_record` | yes |
| `stat.rs`: `design_axes_array.design_axes` (`axis_record.axis_tag`) | yes — `axis_record` is inline | **no** — pure offset-pointer, `pseudo_record`+`with_view` rewrite applies directly |

`base.rs`'s `baseline_tags` sits behind an offset dereference (`read_phantom_view_offset16` into
`base_tag_list`), but the array isn't the *whole* offset target — it's preceded by the sibling
`base_tag_count` field in the same record — so it can't skip straight to the `pseudo_record`
rewrite the way `stat.rs`'s `design_axes` can; it would still need `from_here` applied one level
down, inside the offset-jumped-to format. `stat.rs`'s `design_axes_array` is the one case whose
offset target reduces to *nothing but* the array (count supplied by the caller), structurally
identical to `cpal.rs`'s pre-migration `color_record_array` — so it gets the free ride straight to
the `pseudo_record([("offset", ..)], with_view(view.offset(var("offset")), read_array(..)))` form.

Several other `tag.call()` sites (`base.rs`'s `base_lang_sys_records`/`base_script_records`/
`feat_min_max_records`, `layout.rs`'s `feature_records`/`script_records`/`lang_sys_records`,
`fvar.rs`'s `variation_axis_record`-array) were checked and **excluded** — they're compounded by an
independent blocker (view-dependent `.call_views()`, a nested phantom-offset field, or a nested
`bit_fields_u16` sub-record) that fixing the indirection gap alone wouldn't remove.

**`Format::Variant`-wrapped primitives aren't recognized.** `f2dot14()`/`fixed32be()` (i.e.
`fmt_variant("F2Dot14"/"Fixed32", u16be()/u32be())`) are real fixed-width primitive reads, but the
`Format::Variant` wrapper isn't the literal `Hint(EndianParse(..))` node `as_base_kind_read`
matches on:

| Site | New `FormatRef` needed? | Needs `from_here`? |
|---|---|---|
| `avar.rs`: `segment_maps.axis_value_maps` (both fields `f2dot14()`) | no — reuse `axis_value_map` | yes |
| `fvar.rs`: `user_tuple.coordinates` (bare `fixed32be()` element) | no — bare `BaseKind::U32BE` | yes |
| `gvar.rs`: `tuple_record.coordinates` (bare `f2dot14()` element) | no — bare `BaseKind::U16BE` | yes |
| `colr.rs`: `color_line.color_stops` (`color_stop()`'s `stop_offset`/`alpha`) | yes — `color_stop` is inline | yes |
| `colr.rs`: `var_color_line.color_stops` (`var_color_stop()`'s `stop_offset`/`alpha`) | yes — `var_color_stop` is inline | yes |
| `stat.rs`: `axis_value_table` Format4 `axis_values` (`axis_value_record.value`) | yes — `axis_value_record` is inline | yes |

None of these six sit behind an offset target that reduces to *just* the array, so all still need
`from_here` even once the variant-wrapping gap is fixed.

Sites checked and excluded for the same reason as above (compounded by an independent, unrelated
blocker): `gvar.rs`'s `shared_tuples`/`tuple_variation_header`'s tuple arrays (element record's
*own* field is itself a nested array, not a scalar), `avar.rs`'s outer `axis_segment_maps` (element
`segment_maps` has a nested array field), `colr.rs`'s `affine2x3`/`var_affine2x3` (not used as
array elements at all), and `fvar.rs`'s `variation_axis_record`-array (also blocked by `tag.call()`
and a nested `bit_fields_u16` field, as above).

## Zero qualifying sites (checked, confirmed empty, left unmodified)

`var_common.rs`, `svg.rs`, `dsig.rs`, `hdmx.rs`, `name.rs`, `head.rs`, `hvar.rs`, `colr.rs`
(outside the two gap-blocked sites above), and all of
`hhea.rs`/`hmtx.rs`/`maxp.rs`/`vhea.rs`/`vmtx.rs`/`cvt.rs`/`fpgm.rs`/`prep.rs`.

Common disqualifying reasons, in rough frequency order:
- element format called via `.call_args(...)`/`.call_views(...)`/`.invoke_*` — depends on outer
  scope or a view, so it isn't a *closed* format `ReadArray` can repeat per-index.
- element wraps a nested offset/phantom sub-parse (`util::read_phantom_view_offset{16,24,32}`,
  `parse_view_offset`) — ephemeral/anonymous field, rejected by `analyze_fixed_shape`.
- element field uses a **signed** primitive (`i8`/`i16be`/`i32be`/`i64be`) — these are
  `map_numeric`-wrapped and lose the recognizable `EndianParse` tag.
- element field is `f2dot14()`/`fixed32be()` or a `FormatRef::call()` indirection, *plus* at least
  one other independent blocker (see the gap-blocked table above for the sites where this was the
  *only* issue).
