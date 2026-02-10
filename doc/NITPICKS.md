# Catalogued Gencode Anti-Patterns (according to clippy)

## Summary (3872 total)

Here is a summary of the issues that `cargo clippy --no-deps` reports for [the gencode crate](generated)

(There is one `clippy` suggestion that was for non-generated code, which has
been fixed locally but will show up on the current version of `main`. This has
been omitted from both the count and summary.)

### `Copy::clone()`-Related (612 of 3872)

```log
    229 `u8`
    164 `bool`
    101 `u32`
     91 `u16`
      6 `opentype_common_value_format_flags`
      5 `png_ihdr`
      5 `elf_types_elf_off`
      2 `Option<u8>`
      2 `opentype_glyf_composite_glyphs`
      1 `u64`
      1 `Option<deflate_fixed_huffman_codes_values>`
      1 `Option<deflate_dynamic_huffman_codes_values>`
      1 `opentype_hhea_table`
      1 `opentype_glyf_simple_flags`
      1 `opentype_glyf_simple_flags_raw`
      1 `elf_types_elf_full`
```

### Not `Copy::clone()` (3260 of 3872)

```log
   2385 warning: redundant field names in struct initialization
    622 warning: try not to call a closure in the expression where it is declared
    193 warning: this operation has no effect
     15 warning: this match could be written as a `let` statement
     13 warning: match expression looks like `matches!` macro
     10 warning: in an `if` condition, avoid complex blocks or closures with blocks; instead, move the block or closure higher and bind it with a `let`
      9 warning: this let-binding has unit value
      4 warning: unneeded `return` statement
      4 warning: manual `RangeInclusive::contains` implementation
      2 warning: large size difference between variants
      1 warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
      1 warning: very complex type used. Consider factoring parts into `type` definitions
      1 warning: this expression creates a reference which is immediately dereferenced by the compiler
```

## Samples

### `redundant_field_names`

```log
warning: redundant field names in struct initialization
    --> generated/gencode.rs:4549:20
     |
4549 | PResult::Ok(main { data: data, end: end })
     |                    ^^^^^^^^^^ help: replace it with: `data`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_field_names
     = note: `#[warn(clippy::redundant_field_names)]` on by default
```

This is a very trivial issue to fix, it can be done as a last-pass transformation at any point
without requiring intrusive or complex changes.

### `redundant_closure_call`

```log
warning: try not to call a closure in the expression where it is declared
    --> generated/gencode.rs:4795:11
     |
4795 | let ret = ((|| Decoder_deflate_main(_input))())?;
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try doing something like: `Decoder_deflate_main(_input)`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure_call
     = note: `#[warn(clippy::redundant_closure_call)]` on by default
```

This is not particularly difficult to solve, since all we have to do is detect particularly
small closure-bodies (mono-expressions) that are immediately invoked, and replace the overall
sub-term with the closure's contents.

It may be a bit fiddly to detect these patterns, however, so it is simple but not as trivial as
fixing field-name redundancies.

### `identity_op`

```log

warning: this operation has no effect
    --> generated/gencode.rs:5328:18
     |
5328 |   let tgt_offset = 0u64 + match header.phoff.clone() {
     |  __________________^
5329 | | elf_types_elf_off::Off32(x32) => {
5330 | | x32 as u64
5331 | | },
...    |
5336 | | };
     | |_^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#identity_op
     = note: `#[warn(clippy::identity_op)]` on by default
help: consider reducing it to
     |
5328 ~ let tgt_offset = match header.phoff.clone() {
5329 + elf_types_elf_off::Off32(x32) => {
5330 + x32 as u64
5331 + },
5332 +
5333 + elf_types_elf_off::Off64(x64) => {
5334 + x64
5335 + }
5336 ~ };
     |
```

This has several potential solutions:

- At the layer of `Expr` construction, where certain
operations with left- and/or right-identities (e.g. bit-shift, addition, subtraction, multiplication)
are pre-optimized to auto-reduce if a constant identity-term is used.
- At the layer of codegen, where the same optimization is employed, but later on in the process.

It is arguably easier to perform at the codegen layer, since sometimes the codegen process may
deal with synthetic value-expressions that may not have appeared in the format itself; but there
are also cases where the helper-fn we use to construct an `Expr` can pre-detect cases that can
be reduced/simplified given an identity element's inclusion.


### `match_single_binding`

```log

warning: this match could be written as a `let` statement
    --> generated/gencode.rs:5677:1
     |
5677 | / match tuple_var {
5678 | | (x1, x0) => {
5679 | | (x1 as u32) << 6u32 | (x0 as u32)
5680 | | }
5681 | | }
     | |_^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#match_single_binding
     = note: `#[warn(clippy::match_single_binding)]` on by default
help: consider using a `let` statement
     |
5677 + let (x1, x0) = tuple_var;
5678 + (x1 as u32) << 6u32 | (x0 as u32)
     |
```

This is already solved in principle, we just have to figure out where we need to apply
the tuple-bind construction where we do not already.

### `match_like_matches_macro`

```log
warning: match expression looks like `matches!` macro
    --> generated/gencode.rs:5964:1
     |
5964 | / match version {
5965 | | 65536u32 => {
5966 | | true
5967 | | },
...    |
5980 | | }
     | |_^ help: try: `matches!(version, 65536u32 | 1330926671u32 | 1953658213u32)`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#match_like_matches_macro
```

This is a bit more fiddly to detect, and requires a bit more syntactic nuance to pull off, since
we normally do not allow `RustPattern` to be used outside of a `RustControl::Match` context.

It is achievable in principle, though it may require a bit of trial-and-error to properly implement.

### `blocks_in_conditions`

```log

warning: in an `if` condition, avoid complex blocks or closures with blocks; instead, move the block or closure higher and bind it with a `let`
    --> generated/gencode.rs:9362:1
     |
9362 | / if {
9363 | | let totlen = acc.clone();
9364 | | let _seq = &seq;
9365 | | totlen >= (point_count as u16)
9366 | | } {
     | |_^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#blocks_in_conditions
     = note: `#[warn(clippy::blocks_in_conditions)]` on by default
help: try
     |
9362 + let res = {
9363 + let totlen = acc.clone();
9364 + let _seq = &seq;
9365 + totlen >= (point_count as u16)
9366 ~ }; if res {
     |
```

This effectively boils down to a lifting-problem where we want to take non-simple
`RustExpr::BlockScope` and bind it above `RustControl::If`. This also requires a
name-dodging algorithm, since we need to pick a binding-name that doesn't shadow
anything already in-use. Practically speaking, we might be able to get away with
`_predicate`, since that is unlikely to be bound elsewhere, but there are always
potential scenarios where we would run into trouble using a single hardcoded name.

### `let_unit_value`

```log
warning: this let-binding has unit value
    --> generated/gencode.rs:6089:1
     |
6089 | let __skip = _input.skip_remainder();
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#let_unit_value
help: omit the `let` binding and replace variable usages with `()`
     |
6089 + _input.skip_remainder();
6090 ~ PResult::Ok(opentype_ttc_header { ttc_tag: ttc_tag, major_version: major_version, minor_version: minor_version, header: header, __skip: () })
     |
```

In practice, this is virtually obsolete as a problem, since we now have the capability
to use new-style records to eliminate the need to capture such side-effect-only parses
under an identifier.

### `needless_return`

```log
warning: unneeded `return` statement
     --> generated/gencode.rs:19724:1
      |
19724 | return PResult::Ok(inner);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
      = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_return
help: remove `return`
      |
19724 ~ PResult::Ok(inner)
19725 | },
  ...
19729 | }
19730 ~ }
      |
```

This is a bit of a head-scratcher, since we don't currently keep track of what productions are
'top-level' and/or 'last-in-line', which we would have to know in order to determine whether a
given `return` statement is needless or needed.

We can probably take the hit since there are few enough of these and it would require a significant
amount of work to pass in context-awareness to each production that may use a `return` statement.

That being said, if we do end up introducing such context for other reasons, it wouldn't hurt to
tack this onto whatever effort that is in aid of.

### `manual_range_contains`

```log
warning: manual `RangeInclusive::contains` implementation
     --> generated/gencode.rs:29727:1
      |
29727 | (x >= 1u8) && (x <= 4u8)
      | ^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `(1u8..=4u8).contains(&x)`
      |
      = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_range_contains
```

This is probably best-solved at the construction layer, introducing a synthetic
expression like `Expr::IsBetween(Expr, (Expr, Expr))` that amounts to the same
semantics as `(X >= Y) && (X <= Z)`.

Otherwise, we could attempt to manually detect `BoolAnd(GTE, LTE)` nodes, but that
is a bit more hackish and could easily break if we flip the order in which the
terms are `BoolAnd`-ed together.

### `large_enum_variant`

```log

warning: large size difference between variants
    --> generated/gencode.rs:3907:1
     |
3907 | pub enum opentype_main_directory { TTCHeader(opentype_ttc_header), TableDirectory(opentype_table_directory) }
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------------------------------^^----------------------------------------^^
     | |                                  |                               |
     | |                                  |                               the largest variant contains at least 1992 bytes
     | |                                  the second-largest variant contains at least 48 bytes
     | the entire enum is at least 1992 bytes
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
     = note: `#[warn(clippy::large_enum_variant)]` on by default
help: consider boxing the large fields to reduce the total size of the enum
     |
3907 - pub enum opentype_main_directory { TTCHeader(opentype_ttc_header), TableDirectory(opentype_table_directory) }
3907 + pub enum opentype_main_directory { TTCHeader(opentype_ttc_header), TableDirectory(Box<opentype_table_directory>) }
     |
```

This is a noted issue, which we have a basic handle on, but not enough power
to actually achieve at the moment. For now, we are limited by our inability
to introduce boxing at the type-layer in complete concord with boxing at the
expression (value) layer to match it.

For this to work, we would need to not only record what `Box`es we introduce
after-the-fact, but also know exactly what nodes in the Expression-tree would
require modification to achieve that.

This might benefit from a sandbox experiment in type-transformation and expression-transformation
to figure out a proper solution to.

It helps that this happens in only two places (for now), meaning that once we have a candidate
solution, there would likely be far fewer niche-cases to account for in subsequent iterations
on the design.

### `ptr_arg`

```log
warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
    --> generated/gencode.rs:6114:96
     |
6114 | ...ser<'_>, start: u32, tables: &Vec<opentype_table_record>) -> Result<opentype_table_directory_table_links, ParseError> {
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: change this to: `&[opentype_table_record]`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg
     = note: `#[warn(clippy::ptr_arg)]` on by default
```

This is a relatively simple case to account for, since we only have to modify types in function signatures.

We could add a rule that makes `Vec<T>` turn into `&[T]` when wrapped in `BorrowOf(..)`, or just
do that manually at the DecoderFn-definition layer.

### `type_complexity`

```log
warning: very complex type used. Consider factoring parts into `type` definitions
     --> generated/gencode.rs:17161:45
      |
17161 | fn Decoder158<>(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8, u8, u8, u8, u8), ParseError> {
      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
      = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity
      = note: `#[warn(clippy::type_complexity)]` on by default
```

This may be possible to avoid using `MonadSeq` to avoid returning magic-numbers we don't need
to hold onto. But worth looking into, in case it isn't so trivial.

### `needless_borrow`

```log

warning: this expression creates a reference which is immediately dereferenced by the compiler
     --> generated/gencode.rs:17566:12
      |
17566 | (slice_ext(&buffer, ix..ix + (((r.length.clone()) as u32) as usize))).to_vec()
      |            ^^^^^^^ help: change this to: `buffer`
      |
      = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_borrow
      = note: `#[warn(clippy::needless_borrow)]` on by default
```

This is a one-off issue that we have to look into to figure out what is going on, and attempted
fixes may break things in other places. So really, TBD what the solution (or complexity of finding
one) might be.
