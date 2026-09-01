# Standalone context: ReadArray-eligibility audit (opentype)

**Task lineage:** Originally audited every `repeat_count(len, g)` array site in
`doodle-formats/src/format/opentype/**` for eligibility to become `ViewFormat::ReadArray`
(`doodle::helper::read_array(expr, kind)`). That first pass (commit `4bba34c`) was comment-only —
no `Format` logic changed. Substantial follow-up implementation work landed afterward (see below),
so a second session (this one) re-derived `READARRAY_AUDIT.md` from the live code rather than
trusting the original comment-only findings. **`READARRAY_AUDIT.md` is the source of truth — read
that file, not this one, for current status.** This file just orients a future session on what's
changed and where to look.

## Core mechanism (current, post-refactor)

- Fixed-shape analysis lives in **`src/fixed.rs`** now (renamed/expanded out of
  `src/record_fmt.rs`, which still exists but only holds the generic `RecordFormat` flattening
  logic `fixed.rs` builds on).
- `FixedReadKind` (`src/marker.rs`): `Base(BaseKind<Endian>)` for bare primitives, or
  `FixedFormat(FormatRef)` for a closed, flat, all-primitive-field record *or* a closed
  `FormatRef` whose body is itself a `Format::Variant`-wrapped primitive.
- `analyze_fixed_shape` (`src/fixed.rs`) is the entry point; `as_spine_elem`/`as_indirect`/
  `as_base_kind_read` (same file) do the per-field recognition. As of this session:
  - `FormatRef::call()` indirection (`Format::ItemVar`) to a primitive **is** resolved.
  - `Format::Variant`-wrapping (`f2dot14()`/`fixed32be()`) **is** resolved, both for a record
    field that's a `.call()` to a Variant-wrapped target, and for a `FormatRef` whose own body is
    the Variant, passed directly as the array elem-kind.
  - Signed primitives (`i8/i16be/...`) **are now resolved** too (commits `9e4e6bb` -> `f4d1e1a` ->
    `eb81afc` -> `9ce426a` -> `abe64ee`, after both of the above — see `BASEKIND_EXTENSION.md`).
    `BaseKind<X>` gained `I8`/`I16Ext`/`I32Ext`/`I64Ext` variants, and `ReadArray` codegen support
    landed for both `FixedFormat` record fields and bare-primitive arrays — big-endian only.
  - Bitflags-derived fields (`bit_fields_u16`/`u32`, `Compute`-derived), nested
    `FixedFormat`-in-record fields, and little-endian `ReadArray` codegen remain unsupported — see
    `READARRAY_AUDIT.md`'s "Genuine remaining gaps" section for live-example evidence on each (the
    LE gap's shape changed under `9ce426a`/`abe64ee`; re-read that section rather than assuming the
    old per-`BaseKind` `unimplemented!()` description still applies literally).
- `from_here` (`src/helper.rs`) = `let_view(NAME, with_view(vvar(NAME), fmt))`, needed when a site
  has no already-bound `ViewExpr`. Not needed when an array sits directly behind an offset field
  whose target reduces to *just* the array (`pseudo_record`+`with_view` form instead).
- Reference migrations to look at if extending this further: `cpal.rs::color_records_array`
  (offset-pointer, no `from_here`), `base.rs::baseline_tags` / `stat.rs::design_axes` /
  `colr.rs::color_line.color_stops` (all `from_here(read_array(..))`, covering the
  indirection/Variant-resolution cases specifically).

## What changed, across sessions

Commits `4a7e22d`, `8162781`, `74f8c2c`, `c4bc881`, `fe21d42`, `5eb4fbf` (in that order, all after
the original comment-only audit) closed both indirection/Variant checker gaps and migrated 5 of the
original 10 gap-blocked sites to real `read_array`/`from_here` calls, plus `hdmx.rs`'s
`device_record.widths` (a site the original audit missed entirely — false negative) and the
pre-existing `cpal.rs` reference migration. One site (`opentype.rs::table_directory.table_records`)
was deliberately left as `repeat_count` despite being eligible, because the parsed `Vec` needs to
stay eagerly in memory for `table_links`. Of the remaining 4 gap-blocked sites, 1 was genuinely
unmigrated (`mvar.rs`) and 3 were tried with `read_array` and reverted (`avar.rs`/`fvar.rs`/
`gvar.rs`, novel lifetime parametricity in the generated element type — see `READARRAY_AUDIT.md`'s
"Tried, reverted" section, each site now carries an inline `NOTE` explaining why).

A later commit batch (`9e4e6bb` -> `f4d1e1a` -> `eb81afc` -> `9ce426a` -> `abe64ee`) closed the
signed-primitive gap entirely (BE only) — see `BASEKIND_EXTENSION.md`. This turned 3 more live
sites (`gvar.rs::packed_deltas`, `hmtx.rs::left_side_bearings`+`long_metrics`, `post.rs::postv2dot5`)
from "blocked" into "low-hanging fruit," none of which are migrated yet. In the same window,
`aea08b6` reverted `hdmx.rs::device_record.widths` *away* from `read_array` back to `capture_bytes`
(deliberate opt-out for downstream infallibility in `otf_metrics.rs`, not a regression) — so
`hdmx.rs` moved from "already migrated" to "deliberately opted out."

The large ~50-site batch of always-eligible bare-primitive/closed-record sites was untouched by any
of this and remains un-migrated.

## Status

No outstanding work was requested in any session so far. `READARRAY_AUDIT.md` reflects the current
state as of commit `abe64ee`; re-derive again (rather than trusting either audit blindly) if a lot
more time has passed or more `read_array`-related commits have landed — check
`git log --oneline -- src/fixed.rs src/codegen/model.rs src/parser/view.rs src/marker.rs doodle-formats/src/format/opentype`
for anything newer than `abe64ee`.

## If resuming

Natural next steps (not yet requested, do not start without being asked):

1. The "low-hanging fruit" migrations in `READARRAY_AUDIT.md` (now 5: `mvar.rs`,
   `cpal.rs::color_record_indices`, plus the 3 newly-unblocked signed-primitive sites) — mechanical,
   same pattern as the already-migrated sites. `hmtx.rs::long_horizontal_metric` additionally needs
   promoting from an inline `record([..])` to a registered `FormatRef` first.
2. Bitflags-derived `ReadArray` support (`TODO[epic=adhoc-readarray]` in `cpal.rs`) — unblocks
   `cpal.rs::palette_types_array` and partially `fvar.rs::variation_axis_record`. Now the
   highest-value remaining *gap* (signed is closed), though still narrower than the low-hanging
   fruit above.
3. Little-endian `ReadArray` codegen — no live OpenType motivation, only worth doing if a LE-using
   format needs it. Re-read `READARRAY_AUDIT.md`'s current description first: `9ce426a`/`abe64ee`
   reshaped this from "missing per-`BaseKind` codegen arms" to "no `MarkerType`/`ReadUnchecked`
   impl exists for LE at all," so the old per-file pointers (`read_array_from_view`'s old
   `unimplemented!()` match, `view.rs`'s monomorphic `read_array_u*` methods) no longer describe the
   current code shape.
4. Re-attempting `avar.rs`/`fvar.rs`/`gvar.rs` would need the lifetime-parametricity issue in
   `src/codegen/model/traits.rs` addressed first — don't just re-apply `read_array` there.
