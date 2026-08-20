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
  - Signed primitives (`i8/i16be/...`, `map_numeric`-wrapped), bitflags-derived fields
    (`bit_fields_u16`/`u32`, `Compute`-derived), nested `FixedFormat`-in-record fields, and
    little-endian `ReadArray` codegen are all still unsupported — see `READARRAY_AUDIT.md`'s "Genuine
    remaining gaps" section for live-example evidence on each.
- `from_here` (`src/helper.rs`) = `let_view(NAME, with_view(vvar(NAME), fmt))`, needed when a site
  has no already-bound `ViewExpr`. Not needed when an array sits directly behind an offset field
  whose target reduces to *just* the array (`pseudo_record`+`with_view` form instead).
- Reference migrations to look at if extending this further: `cpal.rs::color_records_array`
  (offset-pointer, no `from_here`), `base.rs::baseline_tags` / `stat.rs::design_axes` /
  `colr.rs::color_line.color_stops` (all `from_here(read_array(..))`, covering the
  indirection/Variant-resolution cases specifically).

## What changed between the two sessions

Commits `4a7e22d`, `8162781`, `74f8c2c`, `c4bc881`, `fe21d42`, `5eb4fbf` (in that order, all after
the original comment-only audit) closed both checker gaps described above and migrated 5 of the
original 10 gap-blocked sites to real `read_array`/`from_here` calls, plus `hdmx.rs`'s
`device_record.widths` (a site the original audit missed entirely — false negative) and the
pre-existing `cpal.rs` reference migration. One site (`opentype.rs::table_directory.table_records`)
was deliberately left as `repeat_count` despite being eligible, because the parsed `Vec` needs to
stay eagerly in memory for `table_links`. 4 sites remain eligible-but-unmigrated (`mvar.rs`,
`cpal.rs::color_record_indices`, `avar.rs`, `fvar.rs`, `gvar.rs` — 5 actually, see the audit's
"Remaining low-hanging fruit" table). The large ~50-site batch of always-eligible bare-primitive/
closed-record sites was untouched by any of this and remains un-migrated.

## Status

No outstanding work was requested in either session. `READARRAY_AUDIT.md` reflects the current
state as of the commits above; re-derive again (rather than trusting either audit blindly) if a lot
more time has passed or more `read_array`-related commits have landed — check
`git log --oneline -- src/fixed.rs doodle-formats/src/format/opentype` for anything newer than
`5eb4fbf`.

## If resuming

Natural next steps (not yet requested, do not start without being asked):

1. The 5 "low-hanging fruit" migrations in `READARRAY_AUDIT.md` — mechanical, same pattern as the
   already-migrated sites.
2. Extending `as_base_kind_read`/`as_spine_elem` to recognize signed primitives — highest-value
   remaining gap by live-example count (`gvar.rs`, `hmtx.rs`, `vmtx.rs`?, `post.rs`).
3. Bitflags-derived `ReadArray` support (`TODO[epic=adhoc-readarray]` in `cpal.rs`) — unblocks
   `cpal.rs::palette_types_array` and partially `fvar.rs::variation_axis_record`.
4. Little-endian `ReadArray` codegen (`src/codegen/model.rs::read_array_from_view`,
   `src/parser/view.rs`) — no live OpenType motivation, only worth doing if a LE-using format needs
   it.
