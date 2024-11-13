# Helper Function Definitions

```rust
/// Returns whichever of two pair-typed values has a larger first element, left-biased if equal
pub fn expr_max_keyval(a: Expr, b: Expr) -> Expr {
    expr_if_else(
        expr_gte(tuple_proj(a.clone(), 0), tuple_proj(b.clone(), 0)),
        a,
        b,
    )
}

/// Computes the largest element of a Seq(U*)-typed expression,
/// wrapped in an Option (None if the list is empty)
///
/// Requires an explicit parameter `t_param` to determine the monomorphic type of both the sequence
/// being processed and the Option being returned
pub fn expr_maximum(seq: Expr, t_param: ValueType) -> Expr {
    left_fold(
        lambda_tuple(
            ["acc", "y"],
            expr_match(
                var("acc"),
                [
                    (pat_some(bind("x")), expr_some(expr_max(var("x"), var("y")))),
                    (pat_none(), expr_some(var("y"))),
                ],
            ),
        ),
        expr_none(),
        ValueType::Option(Box::new(t_param)),
        seq,
    )
}

/// Given a keying function `f_key` that produces a value of type U*, along with type-hints `vt_key` and `vt_elem`,
/// returns the element of a sequence of type `vt_elem` with the largest key of type `vt_key` as computed by `f_key`.
///
/// The return-value is wrapped in an `Option` layer, or `None` if the sequence is empty.
// REVIEW - if we ever make it possible to call lambdas directly, we should use that instead of `impl Fn`
pub fn expr_maximum_by(
    f_key: impl FnOnce(Expr) -> Expr,
    vt_key: ValueType,
    vt_elem: ValueType,
    seq: Expr,
) -> Expr {
    let keyed = flat_map(lambda("x", singleton(pair(f_key(var("x")), var("x")))), seq);
    let max_keyed = left_fold(
        lambda_tuple(
            ["acc", "y"],
            expr_match(
                var("acc"),
                [
                    (
                        pat_some(bind("x")),
                        expr_some(expr_max_keyval(var("x"), var("y"))),
                    ),
                    (pat_none(), expr_some(var("y"))),
                ],
            ),
        ),
        expr_none(),
        ValueType::Option(Box::new(ValueType::Tuple(vec![vt_key, vt_elem]))),
        keyed,
    );
    expr_match(
        max_keyed,
        [
            (
                pat_some(Pattern::Tuple(vec![Pattern::Wildcard, bind("x")])),
                expr_some(var("x")),
            ),
            (pat_none(), expr_none()),
        ],
    )
}

/// Computes the summation of all elements in a Seq(U*)-typed expression,
/// unified to U64 to allay the immediate need for overflow-checking.
pub fn expr_sum_as_u64(seq: Expr) -> Expr {
    left_fold(
        lambda_tuple(
            ["acc", "y"],
            add(var("acc"), Expr::AsU64(Box::new(var("y")))),
        ),
        Expr::U64(0),
        ValueType::UMAX,
        seq,
    )
}

/// Evaluates to `true` if any element of `haystack` equals `needle`, otherwise `false`.
pub fn is_elem(needle: Expr, haystack: Expr) -> Expr {
    left_fold(
        lambda_tuple(
            ["acc", "stalk"],
            expr_if_else(var("acc"), Expr::Bool(true), expr_eq(var("stalk"), needle)),
        ),
        Expr::Bool(false),
        ValueType::Base(BaseType::Bool),
        haystack,
    )
}

/// Returns a copied sequence with only the first occurrence of any given element modulo integer equality.
pub fn dedup(seq: Expr, vt_elem: ValueType) -> Expr {
    flat_map_list(
        lambda_tuple(
            ["prefix", "x"],
            expr_if_else(
                is_elem(var("x"), var("prefix")),
                seq_empty(),
                singleton(var("x")),
            ),
        ),
        vt_elem,
        seq,
    )
}

/// Returns a duplicated sequence that elides all elements for which `f(x)` evaluates to `false`.
// REVIEW - if we ever make it possible to call lambdas directly, we should use that instead of `impl Fn`
pub fn filter(f: impl FnOnce(Expr) -> Expr, seq: Expr) -> Expr {
    flat_map(
        lambda(
            "x",
            expr_if_else(f(var("x")), singleton(var("x")), seq_empty()),
        ),
        seq,
    )
}

/// Returns a version of a list where each element appears after its index (U32), within a 2-tuple
///
/// Semantically equivalent to `Iterator::enumerate`
pub fn enumerate_u32(seq: Expr, vt_elem: ValueType) -> Expr {
    flat_map_list(
        lambda_tuple(
            ["prev", "elem"],
            singleton(pair(seq_length(var("prev")), var("elem"))),
        ),
        ValueType::Tuple(vec![ValueType::Base(BaseType::U32), vt_elem]),
        seq,
    )
}

/// Performs an infallible destructuring of the provided `expr` within the Expr layer,
/// either extracting the contents of a `Some(_)` value, or returning `default` if it is `None`.
pub fn expr_unwrap_or(expr: Expr, default: Expr) -> Expr {
    Expr::Match(
        Box::new(expr),
        vec![(pat_some(bind("x")), var("x")), (pat_none(), default)],
    )
}

/// Performs a fallible destructuring of the provided `expr` within the Format layer,
/// either extracting the contents of a `Some(_)` value, or poisoning the current parse-state
/// via `Format::Fail` if it is `None`.
pub fn fmt_unwrap(expr: Expr) -> Format {
    Format::Match(
        expr,
        vec![
            (pat_some(bind("x")), Format::Compute(var("x"))),
            (pat_none(), Format::Fail),
        ],
    )
}

/// Performs a table lookup operation over a sequence of entries `seq` of type `elem_type`.
///
/// Uses `f_getkey` to map from each entry to its key, and `query_key` as the query key.
///
/// Even if mulitple matching entries exist, only the first one is returned, as an Option.
pub fn table_find(
    seq: Expr,
    elem_type: ValueType,
    f_getkey: impl Fn(Expr) -> Expr,
    query_key: Expr,
) -> Expr {
    let match0_or1 = flat_map_list(
        lambda(
            "list-entry",
            expr_match(
                seq_length(tuple_proj(var("list-entry"), 0)),
                [
                    (Pattern::Int(Bounds::at_least(1)), Expr::Seq(Vec::new())),
                    (
                        Pattern::Wildcard,
                        expr_match(
                            expr_eq(f_getkey(tuple_proj(var("list-entry"), 1)), query_key),
                            [
                                (Pattern::Bool(true), Expr::Seq(vec![var("entry")])),
                                (Pattern::Bool(false), Expr::Seq(Vec::new())),
                            ],
                        ),
                    ),
                ],
            ),
        ),
        elem_type,
        seq,
    );
    seq_to_opt(match0_or1)
}

/// Performs the natural transformation from a singleton-or-empty `Expr::Seq` into `Option`.
///
/// # Notes
///
/// Not well-formed (i.e. will result in a runtime error) if the provided Expr can ever be a sequence of length 2 or higher.
pub fn seq_to_opt(empty_or_singleton: Expr) -> Expr {
    expr_match(
        empty_or_singleton,
        [
            (Pattern::Seq(vec![bind("x")]), expr_some(var("x"))),
            (Pattern::Seq(Vec::new()), expr_none()),
        ],
    )
}



/// Shortcut for discarding a Format's return value but perserving its effect on the overall parse
pub fn void(f: Format) -> Format {
    chain(f, "_", Format::EMPTY)
}

/// Shortcut for two-layer decoding of a Format that is embedded in the result of parsing another.
///
/// The first argument is the outer layer, whose immediate result is discarded after processing the
/// second layer, which is treated as the final result.
pub fn two_pass(primary: Format, secondary: Format) -> Format {
    chain(
        primary,
        "raw",
        Format::DecodeBytes(var("raw"), Box::new(secondary)),
    )
}

/// Constructs a format that will fallback to parsing an abstracted byte-sequence if the given Format `f`
/// fails to parse.
pub fn binary_fallback(f: Format) -> Format {
    union_nondet([
        ("valid", f),
        ("invalid", repeat(Format::Byte(ByteSet::full()))),
    ])
}

/// Shortcut for computing a standalone `Format::Pos` that we immediately consume without ever needing to reuse,
/// which discards the `Format::Pos` token via `map`
///
/// The `pos_varname` parameter is the verbatim name of the variable that `f` internally uses to refer to the parsed `Format::Pos`.
#[inline]
pub fn with_pos(pos_varname: &'static str, f: Format) -> Format {
    chain(Format::Pos, pos_varname, f)
}

/// Helper for parsing `(f, suffix)` where we only want to see the `f` component
#[inline]
pub fn discard_suffix(f: Format, suffix: Format) -> Format {
    map(tuple([f, suffix]), lambda("x", tuple_proj(var("x"), 0)))
}

/// Performs a guarded index operation within the Format layer, returning `elt` if the index is in bounds,
/// or causing parse-failure if it is out of bounds.
pub fn fmt_try_index(seq: Expr, index: Expr) -> Format {
    fmt_unwrap(index_checked(seq, index))
}

/// Similar to [`chain`], but for cases where the result of `f0` is not used.
#[inline]
pub fn keep_last(f0: Format, f: Format) -> Format {
    // TODO: implement a first-class Format variant for this case
    Format::LetFormat(Box::new(f0), Label::Borrowed("_"), Box::new(f))
}

/// Dual method to [`keep_last`], where the order of evaluation is the same
/// but the return value is that of the first format, not the last.
pub fn keep_first(f: Format, f1: Format) -> Format {
    // NOTE: compared to `keep_last`, there is far less of a reason to create a Format primitive to avoid this construction
    chain(f, "x", keep_last(f1, compute(var("x"))))
}

/// Helper function for splicing together two seperate records, with the option to filter the fields included from each
/// or reorder them internally, but not across the record as a whole (i.e. all fields from the first will strictly precede those of the second).
pub fn merge_record_subsets<const N: usize, const M: usize>(first: (Expr, [&'static str; N]), second: (Expr, [&'static str; M])) -> Expr {
    let (first_expr, first_fields) = first;
    let (second_expr, second_fields) = second;

    let mut accum_fields = Vec::with_capacity(N + M);
    let mut included_fields = BTreeSet::new();

    for field_name in first_fields.into_iter() {
        if !included_fields.insert(field_name) {
            unreachable!("duplicated field in merge_records: `{field_name}`");
        }
        accum_fields.push((
            Label::Borrowed(field_name),
            record_proj(first_expr.clone(), field_name),
        ));
    }
    for field_name in second_fields.into_iter() {
        if !included_fields.insert(field_name) {
            unreachable!("duplicated field in merge_records: `{field_name}`");
        }
        accum_fields.push((
            Label::Borrowed(field_name),
            record_proj(second_expr.clone(), field_name),
        ))
    }
    Expr::Record(accum_fields)
}
```
