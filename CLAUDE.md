# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`doodle` is a custom DDL (data description language) / parser-combinator library for describing
binary file formats declaratively as data (`Format`, `Expr`, `Pattern` trees), plus a code-generation
pipeline that compiles those declarative descriptions into standalone, dependency-light Rust decoders.
It is related work to [Fathom](https://github.com/yeslogic/fathom), and gradual efforts are being
made towards increasing interoperability with the parsing-model and data-types used in OTF processing
library [allsorts](https://github.com/yeslogic/allsorts).

## Workspace layout

- `src/` — the `doodle` crate itself: the `Format`/`Expr`/`Pattern` grammar, the interpreter
  (`decoder.rs`), the bidirectional type-checker (`typecheck.rs`, see `TYPECHECKER.md`), and the
  code-generation pipeline (`codegen/`).
- `doodle-formats/` — format *definitions* built with `doodle`, under `src/format/`: PNG, JPEG,
  GIF, (u)star, gzip/zlib/deflate, TIFF, RIFF, ELF, MPEG4, and the much larger `opentype/` tree
  (`.otf`/`.ttf`/`.ttc`), plus small synthetic formats used for testing (`peano`, `numbers`,
  `waldo`, `run_length`). Also provides the `doodle` CLI binary (`src/main.rs`).
- `generated/` — `doodle_gencode`: the machine-generated output of `doodle format --output rust`
  (checked-in file `gencode.rs`, **do not hand-edit** — see `generated/CLAUDE.md`), plus a hand-written
  support layer that gives the generated code a more stable/ergonomic API (`api_helper/`, `bin/*.rs`
  inspection CLIs).
- `smallsorts/` — compacted snapshot of the core binary-read functionality of `allsorts`, which the generated
  code relating to OpenType is gradually evolving toward compatibility with; intended to expose the core API
  of `allsorts` without having to pull that crate in as a dependency, which would be prohibitively expensive.
  This should never be edited.
- `experiments/analytic-engine/`, `experiments/analytic-parser/` — artifacts from early experiments into mixed-type
  arithmetic, kept as separate workspace members even after the useful code has landed in `src/numeric/`.
- `tests/` — integration tests: `decode.rs` runs the `doodle` binary against files in the repo root
  and `test-fonts/`/`test-images/` and diffs stdout against `tests/expected/decode/*.stdout` snapshots
  (via `expect-test`); `runtime_repeat/` and `permit_state_error/` are self-contained mini codegen
  fixtures (each has its own `mod.rs`, `codegen_tests.rs`, `api_helper.rs`) exercising specific
  codegen edge cases outside the main `generated/` crate.
- `custom/` — non-Rust generators (Haskell/C) for some of the synthetic test-format fixtures.
- `doc/` — design notes (`DESIGN.md` covers the `MatchTree` lookahead-disambiguation model), a
  features backlog (`Features.md`), and misc notes.
- Root-level `*.md` are living design/guide docs, not historical records — consult them when touching
  the relevant area: `HELPERS.md` (catalog of `Expr`/`Format` helper combinators in `src/helper.rs`),
  `TYPECHECKER.md` (guide to extending the type-inference engine with new primitives), `RECORDS.md`
  (guide to old-style vs. new-style record-`Format` construction post-#239).

## Common commands

```sh
# Build
cargo build

# Run the full test suite (mirrors CI; smallsorts/analytic-* are excluded)
cargo testall
# equivalent to:
cargo test --workspace --exclude smallsorts --exclude analytic-engine --exclude analytic-parser

# Run a single test
cargo test -p doodle-formats test_decode_test_png
cargo test --test decode gif::

# Update decode-test snapshots after an intentional output change
env UPDATE_EXPECT=1 cargo test

# Format check (CI enforces this)
cargo fmt -- --check

# Regenerate generated/gencode.rs from the current format definitions
cargo cg
# equivalent to:
cargo run --bin doodle -- format --output rust --dest generated/gencode.rs

# Decode a file with the CLI
cargo run --bin doodle -- file test2.jpg
cargo run --bin doodle -- file --output tree test.mp4
cargo run --bin doodle -- file --as-format opentype some-font.ttf

# Dump the registered format tree (debug/json) or check types
cargo run --bin doodle -- format --output debug
cargo run --bin doodle -- typecheck

# Coverage (requires cargo-tarpaulin)
cargo coverage
```

CI (`.github/workflows/ci.yml`) runs `cargo fmt -- --check`, then `cargo testall`, then `cargo build`,
on stable Rust. `RUST_MIN_STACK=8388608` is set project-wide via `.cargo/config.toml` — deep `Format`
trees (notably OpenType) can blow the default stack in debug builds without it.

## Architecture

### The Format/Expr/Pattern language

Binary formats are described as data, not code: `Format` (in `src/format.rs`, re-exported plus
extended in `src/lib.rs`) is the parsing-description language — sequencing, alternation
(`Union`/`UnionNondet`), repetition (`Repeat`/`RepeatCount`/`RepeatUntil*`/`AccumUntil`), offsets,
bit-level parsing, views, etc. `Expr` is the pure computation language used inside formats (e.g. to
compute a length from a previously-parsed field), and `Pattern` is used for destructuring in
`Match`/`Let`-style binding. Format definitions in `doodle-formats/src/format/*.rs` are built by
calling combinator functions (see `HELPERS.md` for the higher-level ones) rather than writing raw
enum literals directly.

A `FormatModule` (`src/lib.rs`) holds the flat table of named top-level formats
(`module.define_format(name, format)`), referenced elsewhere via `Format::ItemVar(level, ...)`
(`FormatRef`/`FormatRef::call()`). This indirection is what lets formats be *conditionally* auto-recursive
(truly format-level recursion is not yet possible due to the complications that would impose on MatchTree)
and shared/reused (e.g. `deflate` reused by both `zlib` and `gzip`).

### Two ways to run a Format

1. **Interpret directly** (`src/decoder.rs` + `src/read.rs`): `Compiler::compile_program` compiles a
   `FormatModule` + top-level `Format` into a `Program`, which can then be `run`/`run_with_loc`
   against a `ReadCtxt` over an in-memory byte buffer — this is what the `doodle` CLI's `file`
   subcommand does. `MatchTree` (see `doc/DESIGN.md`) is the bounded-lookahead disambiguation engine
   used to pick between `Union` branches without backtracking.
2. **Generate Rust code** (`src/codegen/`): `generate_code(module, top_format)` walks the (typed)
   format tree and emits standalone Rust decoder functions/types with no runtime dependency on
   `doodle` itself beyond the small `doodle_gencode::prelude` support layer. This is
   what backs `cargo cg` / `generated/gencode.rs`. Key stages: `src/typecheck.rs` (bidirectional
   unification over `Format`/`Expr`/`Pattern`, assigning a `UType` per node — see `TYPECHECKER.md`
   for the extension guide) enumerates and solves a predictably deterministic traversal of the grammar,
   allowing an `Elaborator` to subsequently fill in the solved types for each node when recursively
   elaborating the root `Format` into the corresponding `TypedFormat` (`src/codegen/typed_format.rs`).
   After compiling this to `TypedDecoder` (`src/codegen/typed_decoder`) and translating each decoder into
   an abstracted instruction-tree of high-level parse-directives (`CaseLogic` and its subtypes, from `src/codegen/mod.rs`),
   these instructions are expanded into Rust-AST representations via the `ToAst` trait, whose impls often use
   template constructions (`src/codegen/model.rs`) to avoid open-coding or hard-coding implementation-specific
   details of `src/codegen/rust_ast/` or the API exposed via the `prelude` import.
   The final AST can then be written to file or stdout through the `ToFragment` trait.
   `src/marker.rs`/`src/record_fmt.rs`/`src/fixed.rs` support `ViewFormat::ReadArray`, an
   optimization for reading fixed-shape record arrays directly instead of decoding element-by-element
   (see `READARRAY_AUDIT.md` for where it does/doesn't currently apply).

### Record-format construction

`Format::Record` was removed (#239) in favor of building records via `chain`/`monad_seq` sequencing
plus a final `Format::Compute(Expr::Record(...))`, wrapped in `Format::Hint` so downstream passes can
still recognize "this subtree is a record." See `RECORDS.md` for the old-style/new-style distinction
and the reasoning behind `Hint`.

## Project Rules

### Scoping discipline

Only read files I explicitly name or point to. Do not read additional files to "get context", "understand the project",
or "see how things connect" unless I ask you to.

If you think that reading more files would help, ask first. One sentence: "Want me to also read X?". Wait for my answer.
This applies to every task in this project. No exceptions for "just checking" or "quick look".
