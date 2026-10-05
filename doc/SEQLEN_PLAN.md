# Plan: the `seqlen-always-u32` project

Status: decisions D1–D4 are confirmed by the user; the step breakdown is proposed. Implemented: steps 1–4.

Follows `doc/NUMERIC_PLAN.md`, which deferred this work as its item 5, and its item 14 (`Format::Pos` evaluates to an Auto Numeric), whose approach this project reuses.

Terminology is as in `doc/NUMERIC_PLAN.md`: **registration** is `Expr::infer_type` and friends, **TC** is the `TypeChecker`, **Auto** is a Numeric with `NumRep::Auto` (`NumericHole` in registration).

## Goal

Remove the hardcoded `U32` assumptions around sequence lengths and indices (`FIXME[epic=seqlen-always-u32]`, `FIXME[epic=dup32]`), so that:

- `Expr::SeqLength` is typed `NumericHole` by registration and evaluates to an Auto Numeric, like `Format::Pos`.
- The index, start, length and count arguments of `SeqIx`, `SubSeq`, `SubSeqInflate` and `Dup` accept any unsigned type or Auto, as TC already does.

The guiding principle from `doc/NUMERIC_PLAN.md` still applies: whatever registration and TC both accept, the interpreter and codegen must both handle without panicking.

## Current state

| Expr | Registration | TC | Interpreter | Codegen |
|---|---|---|---|---|
| `SeqLength` | always `U32` (`ValueType::SEQ_LEN_T`) | `BaseSet::UAny32` | native `U32`, via `u32::try_from(len)?` | type hardcoded to `U32` in `TypedExpr::get_type`; emits `len() as u32` |
| `SeqIx` index | exactly `U32` | `UAny32` | `try_as_usize` (any width, Numerics OK) | `as usize` |
| `SubSeq` / `SubSeqInflate` start and length | exactly `U32` | `UAny32` | `try_as_usize` | `as usize` |
| `Dup` count | exactly `U32` | `UAny32` | `try_as_usize` | `dup32(count: u32, ..)` (prelude) |

Registration is stricter than TC at every site, so nothing that both accept fails today (the numeric survey's 15 "codegen looser" rows).

### Latent codegen issues exposed by relaxing registration

1. **`SeqLength`'s width.** TC can resolve a `SeqLength` to a width other than `U32` when the context pins it (e.g. `seq_length(x) == U8(3)`), but `TypedExpr::SeqLength` carries no type, `get_type` reports `U32`, and the lowering emits `len() as u32`. The generated code would not compile. Unreachable today only because registration rejects every such context.
2. **`Dup`'s count.** `dup32` takes `u32`, so a `Dup` whose count TC resolves to another width would not compile.

Both confirmed in step 1: for `seq_length(s) == U8(3)` codegen emits `((s.len()) as u32) == 3u8`, and for `dup(U8(4), U8(0))` it emits `dup32(4u8, 0u8)`. Both are Rust type errors.

### Live uses in `doodle-formats`

With `SeqLength` as Auto:

- `peano`: `seq_length(s)` is the format's value. Only its display changes (`N` → `N?`); no decode snapshot covers it.
- `deflate`: `seq_length(buffer) - as_u32(distance)` (Auto adopts `U32`), a `>=` comparison on a `seq_length`, and `seq_last_checked`'s `!= U32(0)` (`helper.rs`). Unaffected.
- `opentype` (two sites) and `helper::seq_last_(un)checked`: `pred(seq_length(seq))`. On an empty sequence this is currently an arithmetic error at `pred`; as Auto it evaluates to `-1`, and the error moves to where the value is used (`EnumFromTo` / `SeqIx`). Both outcomes are errors, so this is within the principle, but it should be pinned down by a test.
- `deflate` (two sites, `FIXME[epic=dup32]`): `dup(as_u32(add(extra, U8(3))), ..)`. The `as_u32` exists only because of registration's exact-`U32` rule.

## Decisions

| # | Question | Decision |
|---|---|---|
| D1 | Interpreter representation of `SeqLength` | An Auto Numeric, following `Format::Pos` (user's direction). The `u32::try_from` overflow check goes away; the width is checked wherever the value next meets a concrete type. |
| D2 | TC's constraint on `SeqLength` | Keep `UAny32` (any unsigned width, `U32` tiebreak). Codegen must honour the resolved width (fixes latent issue 1). |
| D3 | Codegen for a non-`u32` `Dup` count | Emit `count as usize`, as `SeqIx`/`SubSeq` already do, and call a `usize`-taking prelude function. Lossless for every unsigned width. Whether `dup32` is kept alongside it or replaced is an implementation detail to settle in step 4. |
| D4 | The `deflate` `FIXME[epic=dup32]` workarounds | Remove the `as_u32` casts as the final step, checking that the decode snapshots are unchanged. |

## Proposed steps

1. **(Done.) Regression tests** pinning current behaviour, like `doodle-formats/tests/pos.rs`: `SeqLength`'s value in both interpreters, the live patterns above (including `pred(seq_length(..))` on an empty sequence), and non-`U32` indices/counts in each layer.
2. **(Done.) Registration.** `SeqLength` → `NumericHole`; `SeqIx`, `SubSeq`, `SubSeqInflate` and `Dup` use `is_unsigned_or_auto` for their index/start/length/count arguments; remove `ValueType::SEQ_LEN_T`. Mirror in `src/alt.rs`.
3. **(Done.) Interpreter.** `SeqLength` evaluates to an Auto Numeric (a shared constructor, like `Value::from_pos`), in both interpreters via `eval_generic`.
4. **(Done.) Codegen.** Give `TypedExpr::SeqLength` its resolved type and emit `len() as <T>`; lower `Dup` per D3. Compile-check generated code for a non-`U32` `SeqLength` and `Dup` count, as was done for signed `IntRel`.
5. **Cleanup.** Remove the `deflate` `as_u32` workarounds (D4) and regenerate `gencode.rs`; update the "not `U32` may fail" notes on `index_unchecked`/`index_checked` and the `seq_length` doc in `helper.rs`; resolve item 5 in `doc/NUMERIC_PLAN.md`; extend the numeric survey and update `doc/NUMERIC.md` and `NUMERIC_GUIDELINE.md`.

## Open questions

- **Q1. Other `U32` assumptions downstream of `SeqLength`.** Step 1's tests and the survey should reveal whether anything else (e.g. `api_helper` code in `generated/`, or codegen's handling of a `SeqLength` bound to a variable) relies on `SeqLength` being a native `U32`. Not yet checked: `generated/api_helper/`, `output/tree.rs`.
- **Q2. `gencode.rs` churn.** With D2, every live `SeqLength` should still resolve to `U32` by tiebreak, so `cargo cg` output is expected to be unchanged until step 5's `deflate` cleanup. To be confirmed in step 4.

## Progress notes

- **Step 1.** `doodle-formats/tests/seqlen.rs` (15 tests) pins current behaviour in registration, both interpreters and codegen (codegen acceptance only; emitted code is not compiled):
  - `SeqLength` is a native `U32`; `- U32`, `>= U32` and `seq_last_(un)checked` on non-empty and empty sequences.
  - `pred(seq_length(..))` on an empty sequence errors at `IntPred`, both via `seq_last_unchecked` and the `opentype` `enum_from_to` pattern.
  - `seq_length(..) == U8(3)`, a `U8` `SeqIx` index, a `U16` `SubSeq` start and a `U8` `Dup` count are rejected by registration and accepted by codegen.
  - The `deflate` `dup(as_u32(..), ..)` pattern.
- **Steps 2–3** (done together: registration typing `SeqLength` as `NumericHole` while the interpreter still produced a native `U32` would have made e.g. `seq_length(s) == U8(3)` panic in the interpreter).
  - Registration and `alt.rs`: `SeqLength` → `NumericHole`; `SeqIx` index, `SubSeq`/`SubSeqInflate` start and length, and `Dup` count use `is_unsigned_or_auto`. `ValueType::SEQ_LEN_T` is removed.
  - Interpreter: `SeqLength` evaluates to `Value::from_seq_len(len)`, an Auto Numeric. The `u32::try_from` overflow check is gone (D1); `eval.rs`'s two `u32::MAX` boundary tests now check that the length is exact and that the width error is raised when it meets a `U32`.
  - `seqlen.rs` updated: `SeqLength` values are Auto; the four formerly rejected cases now evaluate; on an empty sequence, `pred(seq_length(..))` now errors with `NumericConvert` where `-1` is used (`SeqIx`/`EnumFromTo`), as predicted. (The baseline's `"IntPred"` assertion had matched the error's expression trace, not the error kind; the new assertions check the kind.)
  - No decode snapshot changed, and `cargo cg` output is unchanged (Q2 confirmed so far).
  - Survey: 43 of 286 cases inconsistent (was 55); the 15 "codegen looser" rows are gone. New "codegen stricter" rows are `-1auto` indices/counts, which TC rejects and the interpreter reports as errors (the item 9 class).
  - Until step 4 landed, codegen emitted Rust that did not compile for the newly accepted `seq_length(..) == U8(..)` and non-`U32` `Dup` counts (latent issues 1 and 2 were reachable).
- **Step 4.**
  - `TypedExpr::SeqLength` now carries its `GenType`, taken from the node's own TC variable in the elaborator; the lowering emits `len() as <that type>`. The hardcoded `U32` in `TypedExpr::get_type` is gone.
  - D3 settled as **replace**: the prelude's `dup32(count: u32, ..)` is replaced by `dup_n(count: usize, ..)`, and codegen emits `dup_n(<count> as usize, ..)`. `dup32` had no callers outside generated code.
  - `gencode.rs` changes only at the 7 `Dup` sites (`dup32(x as u32, ..)` → `dup_n((x as u32) as usize, ..)`); every live `SeqLength` still resolves to `u32`. Q2 is therefore answered: no `SeqLength` churn, only the `Dup` call sites.
  - Compile check (one-off, scratch crate against `doodle::prelude`): `seq_length(s) == U8(3)`, `seq_length(dup(U8(4), ..))` and `seq_length(dup(U64(2), ..))` emit `(s.len() as u8) == 3u8`, `dup_n(4u8 as usize, ..)` and `dup_n(2u64 as usize, ..)`, which compile and return `(true, 4, 2)`.
  - The `as u32` in `dup_n((x as u32) as usize, ..)` comes from the format definitions, not codegen; `u8 → u32 → usize` is two lossless widenings, not a round trip.
    - In `deflate` (6 sites) the cast is applied after a `u8` addition (`(extra + 3u8) as u32`), so it guards nothing and only satisfied registration's old exact-`U32` rule; D4 removes it.
    - In `opentype`'s `glyf` flags (1 site) the cast precedes the addition (`(repeats as u32) + 1u32`, with `repeats: u8`), so it is a real widening that prevents `255 + 1` overflowing `u8`. It stays.

