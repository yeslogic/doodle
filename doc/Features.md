# Missing Features

## Validation

- `Expr::UnaryOp` primitive  with associated enum including certain base-2 related transformations
  - `UnaryOp::ILog2` (would be enough)
  - Suggested by: `table_directory.search_range` and subsequent fields
- `Expr`-level construct for detecting and asserting relative sort-order of a sequence, either raw-value or with a key-getter `Expr::Lambda` (or `impl Fn(Expr) -> Expr`)
  - We can do this with a primitive Expr that evaluates to an Option-wrapped copy of the last iteration of a `Format::Repeat*`, but this might be complicated; alternatively, a Format primitive that bakes such a value into its parsing logic
  - Another way would be to have an Expr primitive that takes `Expr::Lambda :: (Expr@T, Expr@T) -> Expr@Bool` and applies it to every pair of adjacent elements of a sequence, aggregating with `BoolAnd`
- `Expr` that computes the maximum value within a sequence
  - Technically speaking we can define this with what we have, but it might be better off as a primitive, or at least with an extra primitive to simplify the definition
- Multi-field delimited Slice, or record-flattening/-concatenating constructions
  - This isn't quite necessary as of current implementation, but is hinted at by cmap subtable formats that just so happen to be implementable using other constructions as fallbacks
- Support for signed numbers
  - I16 (s16be in fathom)
  - I32 (s32be in fathom)
  - I64 (s64be in fathom)
  - (Signed) Fixed-Point 16.16 numbers
- Identifying strings for const-enum-like numeral values
  - Even without necessarily generating const-enum definitions in generated code, we may want to start signalling
  the semantics of the expected values of a given numeric format-token with strings or identifiers at the output layer;
  similar to the proposal for a type-preserving wrapper that merely attaches content of a given type (whether string or enum)
  to a Value as it is processed, we could have type-erased annotations that augment the display output without disrupting
  the computation model.
- Accumulator-predicated termination condition for repeat
  - For the variable-length array `flags` in glyf table simple descriptions,
  the condition to stop repeating is "sum over a given projected field of the record being repeated == final element of end_points_of_contours array"
  This is currently inefficient to define given our existing vocabulary, as while we have Expr::LeftFold and Format::RepeatUntilSeq, we have no way of preserving the sum over the first N elements to add the N+1 repetition to to avoid quadratic cost for a linear computation. Something like `RepeatUntilAccum(Expr{A -> Bool}, Expr{A -> B -> A}, AccInit{A}, ValueType{:= A}, Format{B})` would make this possible, though niche
- Expr::Dup variation for producing Seq-typed values of given length
  - Required to avoid a needlessly complicated workaround for expanding the repetitions of simple glyf flags
