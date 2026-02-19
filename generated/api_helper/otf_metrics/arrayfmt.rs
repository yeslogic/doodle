use super::display::{
    Token::{self, LineBreak},
    TokenStream, tok, toks,
};
use crate::api_helper::util::{EnumLen, Wec, trisect_unchecked};

/// Multiline display helper for arrays of possibly-None values.
///
/// All elements are considered for the purposes of indexing, but no None-values are
/// displayed; the resulting output will show up to the first, and last `N` elements, where
/// `N` is determined by `bookend` (shows all elements if the number of non-None elements is less than or equal to `2 * bookend`).
///
/// The `display_fn` closure is used to display individual elements, and takes both the index and the value of the element in question.
/// In particular, the index is the original index in the input array, and not a re-indexing after filtering out None-values.
///
/// The `ellipsis` closure is used to signal information about the skipped middle-elements (if any), and takes two arguments:
/// the number of elements that were skipped over (after filtering out None-values), and a tuple `(start, stop)` where `start` is the
/// true index of the first element skipped and `stop` is the true index of the element where display resumes.
pub(crate) fn display_nullable<T>(
    opt_items: &[Option<T>],
    mut display_fn: impl FnMut(usize, &T) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl Fn(usize, (usize, usize)) -> TokenStream<'static>,
) -> TokenStream<'static> {
    let items: Vec<(usize, &T)> = opt_items
        .iter()
        .enumerate()
        .filter_map(|(ix, opt)| opt.as_ref().map(|v| (ix, v)))
        .collect();
    let mut buffer =
        Vec::<TokenStream<'static>>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

    let count = items.len();

    if count > bookend * 2 {
        let (left_bookend, middle, right_bookend) =
            unsafe { trisect_unchecked(&items, bookend, bookend) };

        for (ix, it) in left_bookend {
            buffer.push(display_fn(*ix, it));
        }

        let n_skipped = count - bookend * 2;
        assert_eq!(middle.len(), n_skipped);
        buffer.push(ellipsis(n_skipped, (middle[0].0, middle[n_skipped - 1].0)));

        for (ix, it) in right_bookend {
            buffer.push(display_fn(*ix, it));
        }
    } else {
        for (ix, it) in items.into_iter() {
            buffer.push(display_fn(ix, it));
        }
    }
    TokenStream::join_with(buffer, LineBreak)
}

/// Inline display helper for arrays of possibly-None values.
///
/// Uses the same semantics and signature as [`display_nullable`], but formats the resulting output as an inline list of items
/// separated by commas and bounded with brackets, rather than a vertical list of items separated by line-breaks.
///
/// As all elements will be formatted on the same line, the `display_fn` closure should not include any line-breaks or indentation.
pub(crate) fn display_inline_nullable<T>(
    opt_items: &[Option<T>],
    mut display_fn: impl FnMut(usize, &T) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl Fn(usize, (usize, usize)) -> TokenStream<'static>,
) -> TokenStream<'static> {
    let items: Vec<(usize, &T)> = opt_items
        .iter()
        .enumerate()
        .filter_map(|(ix, opt)| opt.as_ref().map(|v| (ix, v)))
        .collect();
    let mut buffer =
        Vec::<TokenStream<'static>>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

    let count = items.len();

    if count > bookend * 2 {
        let (left_bookend, middle, right_bookend) =
            unsafe { trisect_unchecked(&items, bookend, bookend) };

        for (ix, it) in left_bookend {
            buffer.push(display_fn(*ix, it));
        }

        let n_skipped = count - bookend * 2;
        assert_eq!(middle.len(), n_skipped);
        buffer.push(ellipsis(n_skipped, (middle[0].0, middle[n_skipped - 1].0)));

        for (ix, it) in right_bookend {
            buffer.push(display_fn(*ix, it));
        }
    } else {
        for (ix, it) in items.into_iter() {
            buffer.push(display_fn(ix, it));
        }
    }
    TokenStream::join_with(buffer, tok(", ")).bracket()
}

/// Inline display helper for arrays.
///
/// Formats the elements of the input array as an inline list of items (each produced by `display_fn`) separated by the token `", "` and bounded with brackets,
/// with an elision-string based on the number of medial elements skipped when the array is long enough (i.e. when the number of elements exceeds `2 * bookend`).
///
/// The `ellipsis` closure is used to produce such an elision-string, which takes as its sole parameter the number of skipped middle-elements.
/// When no elements are skipped, `ellipsis` will not be called and no elision-string will be included in the output.
///
/// All elements will be written on the same line, and so the `display_fn` closure should not include any line-breaks or indentation.
pub(crate) fn display_items_inline<T>(
    items: &[T],
    mut display_fn: impl FnMut(&T) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl Fn(usize) -> TokenStream<'static>,
) -> TokenStream<'static> {
    // Allocate a buffer big enough to hold one string per item in the array, or enough items to show both bookends and one ellipsis-string
    let mut buffer =
        Vec::<TokenStream<'static>>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

    let count = items.len();
    if count > bookend * 2 {
        for item in &items[..bookend] {
            buffer.push(display_fn(item));
        }

        buffer.push(ellipsis(count - bookend * 2));

        for item in &items[count - bookend..] {
            buffer.push(display_fn(item));
        }
    } else {
        buffer.extend(items.iter().map(display_fn));
    }
    TokenStream::join_with(buffer, tok(", ")).bracket()
}

/// Multiline display helper for arrays.
///
/// Formats each element of the input array as a separate line (using `display_fn`), showing only the first and last `bookend` items if the array is long enough (i.e. when the number of elements exceeds `2 * bookend`).
///
/// The `display_fn` closure takes both the index and the value of each element, and should produce a `TokenStream` representing the intended multiline display of that element (including any line-breaks or indentation).
///
/// When the array is long enough to warrant elision, the `ellipsis` closure is used to produce a `TokenStream` representing the desired format of the elision-string, taking as parameters the indices of the first and last items to be elided.
/// Namely, the first parameter is the index immediately following the last item shown in the initial bookend, and the second parameter is the index preceding the first item shown in the terminal bookend.
///
/// If the length of `items` is less than or equal to `2 * bookend`, no elision is performed and the `ellipsis` closure will not be called.
pub(crate) fn display_items_elided<T>(
    items: &[T],
    display_fn: impl Fn(usize, &T) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl Fn(usize, usize) -> TokenStream<'static>,
) -> TokenStream<'static> {
    let mut buffer =
        Vec::<TokenStream<'static>>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

    let count = items.len();
    if count > bookend * 2 {
        for (ix, item) in items.iter().enumerate().take(bookend) {
            buffer.push(display_fn(ix, item));
        }

        buffer.push(ellipsis(bookend, count - bookend));

        for (ix, item) in items.iter().enumerate().skip(count - bookend) {
            buffer.push(display_fn(ix, item));
        }
    } else {
        buffer.extend(
            items
                .iter()
                .enumerate()
                .map(|(ix, item)| display_fn(ix, item)),
        );
    }
    TokenStream::join_with(buffer, LineBreak)
}

/// Multiline display helper for [`Wec`]-based 2D matrices.
///
/// For each row of the matrix, `display_fn` is used to produce a `TokenStream` corresponding to the intended display of that row, and the resulting output is the vertical concatenation of these per-row displays, with an optional elision-string in the middle when the number of rows exceeds `2 * bookend`.
///
/// As each row of the `Wec` is already a slice of items, the `display_fn` closure takes as parameters the row-index and a slice consisting of the elements of that row.
///
/// The closure `ellipsis` is called if the number of rows exceeds `2 * bookend`, and produces the elision-string that is displayed on its own line between the first and last `N` rows that are directly displayed, where `N` is the value passed to `bookend`.
/// This closure takes, as its parameters, the indices of the first and last rows to be elided. Namely, the first parameter is the index immediately following the last row shown in the initial bookend, and the second parameter is row-index immediately prior to
/// the first row shown in the terminal bookend.
///
/// # Notes
///
/// The closure passed in as `display_fn` will most often make use of one of the other display helpers in this module to format the individual rows. In such cases, care should be taken to either ensure that
/// the output is entirely inline, or to increment indentation of indexed items if multiline, to avoid ambiguity between the indexing of rows and the indexing of items within each row.
pub(crate) fn display_wec_rows_elided<T>(
    matrix: &Wec<T>,
    display_fn: impl Fn(usize, &[T]) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl Fn(usize, usize) -> TokenStream<'static>,
) -> TokenStream<'static> {
    let count = matrix.rows();
    let mut lines = Vec::with_capacity(Ord::min(count, bookend * 2 + 1));

    if count > bookend * 2 {
        for ix in 0..bookend {
            lines.push(display_fn(ix, &matrix[ix]));
        }
        lines.push(ellipsis(bookend, count - bookend));
        for ix in (count - bookend)..count {
            lines.push(display_fn(ix, &matrix[ix]));
        }
    } else {
        let mut lines = Vec::with_capacity(count);
        lines.extend(
            matrix
                .iter_rows()
                .enumerate()
                .map(|(ix, row)| display_fn(ix, row)),
        );
    }
    TokenStream::join_with(lines, LineBreak)
}

/// Inline display helper for arrays of items that are cross-indexed against glyph-ids in a coverage table.
///
/// The `display_fn` closure takes both the GlyphId and the item value as parameters, and should produce a `TokenStream` representing the intended inline display of that item given the
/// glyphId it is associated with.
///
/// In terms of the formatting of the output, this function is functionally equivalent to `display_items_inline`.
///
/// Takes an additional parameter `coverage`, an iterator containing as many glyph-ids as there are items in the input array,
/// and the elements it yields are understood to be in the same order as the items in the input array.
pub(crate) fn display_coverage_linked_array<T>(
    items: &[T],
    coverage: impl Iterator<Item = u16>,
    mut display_fn: impl FnMut(u16, &T) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl FnOnce(usize) -> TokenStream<'static>,
) -> TokenStream<'static> {
    let count = items.len();
    let mut buffer = Vec::with_capacity(Ord::min(count, bookend * 2 + 1));

    let mut ix_iter = EnumLen::new(coverage, count);

    if count > bookend * 2 {
        for (ix, glyph_id) in ix_iter.iter_with().take(bookend) {
            buffer.push(display_fn(glyph_id, &items[ix]));
        }

        let n_skipped = count - bookend * 2;

        // REVIEW - do we care to mention the first and last glyphIds elided, or is the number of skipped elements good enough?
        buffer.push(ellipsis(n_skipped));

        for (ix, glyph_id) in ix_iter.iter_with().skip(n_skipped).take(bookend) {
            buffer.push(display_fn(glyph_id, &items[ix]));
        }
    } else {
        for (ix, glyph_id) in ix_iter.iter_with() {
            buffer.push(display_fn(glyph_id, &items[ix]));
        }
    }

    // NOTE - boolean control-flag for strictness; when false, will not return an error if there are leftover items in the coverage iterator
    const FORBID_LEFTOVER_COVERAGE: bool = false;

    match ix_iter.finish(FORBID_LEFTOVER_COVERAGE) {
        Ok(_) => {}
        Err(e) => panic!("format_coverage_linked_array found error: {e}"),
    }
    TokenStream::join_with(buffer, tok(", ")).bracket()
}

/// Inline  helper for horizontally-oriented display of an individual column of a table.
///
/// The resulting `TokenStream` will begin with `heading` (which should be justified across lines of the same table),
/// and will contain up to `bookend` leading and trailing entries of the `items` array, each tokenized using `display_fn`,
/// and separated with a single whitespace character (`' '`).
///
/// Conditionally includes an elision-string returned by `ellipsis`, called with the number of skipped middle-items.
///
/// # Notes
///
/// When used for per-column display of a table with more than one column, care should be taken to ensure
/// that the `heading` parameter is of consistent width and justification across all columns of the same table,
/// and that the closures used for `display_fn` and `ellipsis` produce fixed-width output
/// (e.g. by space-padding or 0-padding numeric values), so that entries in the same row of the overall
/// table are properly aligned.
///
/// # Example
///
/// Usage:
///
/// ```ignore
/// display_table_column_horiz(
///     "\tx |",
///     [2, 3, 4, 5, 6],
///     |x| toks(format!(" {x:02}")),
///     2,
///     |n| toks(format!("..({n:02})..")),
/// ).to_string()
/// ```
///
/// Output (text):
/// ```ignore
/// "\tx | 02  03 ..(01)..  05  06"
/// ```
pub fn display_table_column_horiz<A>(
    heading: &'static str,
    items: &[A],
    mut display_fn: impl FnMut(&A) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl FnOnce(usize) -> TokenStream<'static>,
) -> TokenStream<'static> {
    let count = items.len();
    let mut buf = Vec::with_capacity(Ord::min(count, 2 * bookend + 1));
    if count > 2 * bookend {
        let (left_bookend, _middle, right_bookend) =
            unsafe { trisect_unchecked(items, bookend, bookend) };

        for it in left_bookend {
            buf.push(display_fn(it));
        }

        let n_skipped = count - bookend * 2;
        buf.push(ellipsis(n_skipped));

        for it in right_bookend {
            buf.push(display_fn(it));
        }
    } else {
        buf.extend(items.iter().map(display_fn));
    }

    tok(heading).then(TokenStream::join_with(buf, tok(" ")))
}
