use super::display::{
    Token::{self, LineBreak},
    TokenStream, tok, toks,
};
use crate::api_helper::util::{EnumLen, Wec, trisect_unchecked};

/// Generic helper for displaying an array of possibly-None elements, skipping over
/// all None-values and only showing up to the first, and last `N` elements, where
/// `N` is determined by `bookend` (shows all elements if the lenght is less than or equal to `2 * bookend`).
///
/// The `show_fn` function is used to display individual elements, and takes both the index and the value of the element in question.
///
/// The `ellipsis` function is used to signal information about the skipped middle-elements (if any), and takes two arguments:
/// the number of elements that were skipped over (after filtering out None-values), and a tuple `(start, stop)` where `start` is the first
/// element-index skipped and `stop` is the element-index where display resumes.
pub(crate) fn display_nullable<T>(
    opt_items: &[Option<T>],
    mut show_fn: impl FnMut(usize, &T) -> TokenStream<'static>,
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
            buffer.push(show_fn(*ix, it));
        }

        let n_skipped = count - bookend * 2;
        assert_eq!(middle.len(), n_skipped);
        buffer.push(ellipsis(n_skipped, (middle[0].0, middle[n_skipped - 1].0)));

        for (ix, it) in right_bookend {
            buffer.push(show_fn(*ix, it));
        }
    } else {
        for (ix, it) in items.into_iter() {
            buffer.push(show_fn(ix, it));
        }
    }
    TokenStream::join_with(buffer, LineBreak)
}

/// Generic helper for displaying an array of possibly-None elements, skipping over
/// all None-values and only showing up to the first, and last `N` elements, where
/// `N` is determined by `bookend` (shows all elements if the lenght is less than or equal to `2 * bookend`).
///
/// The `show_fn` function is used to display individual elements, and takes both the index and the value of the element in question.
///
/// The `ellipsis` function is used to signal information about the skipped middle-elements (if any), and takes two arguments:
/// the number of elements that were skipped over (after filtering out None-values), and a tuple `(start, stop)` where `start` is the first
/// element-index skipped and `stop` is the element-index where display resumes.
pub(crate) fn display_inline_nullable<T>(
    opt_items: &[Option<T>],
    mut show_fn: impl FnMut(usize, &T) -> TokenStream<'static>,
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
            buffer.push(show_fn(*ix, it));
        }

        let n_skipped = count - bookend * 2;
        assert_eq!(middle.len(), n_skipped);
        buffer.push(ellipsis(n_skipped, (middle[0].0, middle[n_skipped - 1].0)));

        for (ix, it) in right_bookend {
            buffer.push(show_fn(*ix, it));
        }
    } else {
        for (ix, it) in items.into_iter() {
            buffer.push(show_fn(ix, it));
        }
    }
    TokenStream::join_with(buffer, tok(", ")).bracket()
}

/// Generic helper for displaying an array of elements, showing at most `bookend` elements at the start and end of the array and a single ellipsis if the array is longer than `2 * bookend`.
///
/// Each element that is to be displayed is formatted using the provided closure `fmt_fn`.
///
/// All elements will be written on the same line, and so the `fmt_fn` closure should not include any line-breaks.
///
/// The `ellipsis` function is used to signal information about the skipped middle-elements (if any), and takes the number of elements that were skipped over.
pub(crate) fn display_items_inline<T>(
    items: &[T],
    mut fmt_fn: impl FnMut(&T) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl Fn(usize) -> TokenStream<'static>,
) -> TokenStream<'static> {
    // Allocate a buffer big enough to hold one string per item in the array, or enough items to show both bookends and one ellipsis-string
    let mut buffer =
        Vec::<TokenStream<'static>>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

    let count = items.len();
    if count > bookend * 2 {
        for item in &items[..bookend] {
            buffer.push(fmt_fn(item));
        }

        buffer.push(ellipsis(count - bookend * 2));

        for item in &items[count - bookend..] {
            buffer.push(fmt_fn(item));
        }
    } else {
        buffer.extend(items.iter().map(fmt_fn));
    }
    TokenStream::join_with(buffer, tok(", ")).bracket()
}

/// Enumerates the contents of a slice, showing only the first and last `bookend` items if the slice is long enough.
///
/// Each item is shown with `show_fn`, and `fn_message` is used to signal the range of indices skipped.
/// If the slice length is less than or equal to `2 * bookend`, all elements are displayed and `ellipsis` is not called.
pub(crate) fn display_items_elided<T>(
    items: &[T],
    show_fn: impl Fn(usize, &T) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl Fn(usize, usize) -> TokenStream<'static>,
) -> TokenStream<'static> {
    let mut buffer =
        Vec::<TokenStream<'static>>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

    let count = items.len();
    if count > bookend * 2 {
        for (ix, item) in items.iter().enumerate().take(bookend) {
            buffer.push(show_fn(ix, item));
        }

        buffer.push(ellipsis(bookend, count - bookend));

        for (ix, item) in items.iter().enumerate().skip(count - bookend) {
            buffer.push(show_fn(ix, item));
        }
    } else {
        buffer.extend(items.iter().enumerate().map(|(ix, item)| show_fn(ix, item)));
    }
    TokenStream::join_with(buffer, LineBreak)
}

// Enumerates the contents of a Wec<T>, showing only the first and last `bookend` rows if the Wec is tall enough.
///
/// Each row is shown with `show_fn`, and the `elision_message` is printed (along with the range of indices skipped)
/// if the slice length exceeds than 2 * `bookend`, in between the initial and terminal span of `bookend` items.
pub(crate) fn display_wec_rows_elided<T>(
    matrix: &Wec<T>,
    show_fn: impl Fn(usize, &[T]) -> TokenStream<'static>,
    bookend: usize,
    fn_message: impl Fn(usize, usize) -> TokenStream<'static>,
) -> TokenStream<'static> {
    let count = matrix.rows();
    let mut lines = Vec::with_capacity(Ord::min(count, bookend * 2 + 1));

    if count > bookend * 2 {
        for ix in 0..bookend {
            lines.push(show_fn(ix, &matrix[ix]));
        }
        lines.push(fn_message(bookend, count - bookend));
        for ix in (count - bookend)..count {
            lines.push(show_fn(ix, &matrix[ix]));
        }
    } else {
        let mut lines = Vec::with_capacity(count);
        lines.extend(
            matrix
                .iter_rows()
                .enumerate()
                .map(|(ix, row)| show_fn(ix, row)),
        );
    }
    TokenStream::join_with(lines, LineBreak)
}

/// Like `display_items_inline`, but for arrays of items that are cross-indexed against glyph-ids in a coverage table.
///
/// Takes an additional parameter `coverage`, an iterator with the same cardinality as `items`; `fmt_fn` takes a `u16`
/// parameter equal to the GlyphId at the corresponding index for each displayed element in `items`.
pub(crate) fn display_coverage_linked_array<T>(
    items: &[T],
    coverage: impl Iterator<Item = u16>,
    mut fmt_fn: impl FnMut(u16, &T) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl FnOnce(usize) -> TokenStream<'static>,
) -> TokenStream<'static> {
    let count = items.len();
    let mut buffer = Vec::with_capacity(Ord::min(count, bookend * 2 + 1));

    let mut ix_iter = EnumLen::new(coverage, count);

    if count > bookend * 2 {
        for (ix, glyph_id) in ix_iter.iter_with().take(bookend) {
            buffer.push(fmt_fn(glyph_id, &items[ix]));
        }

        let n_skipped = count - bookend * 2;

        buffer.push(ellipsis(n_skipped));

        for (ix, glyph_id) in ix_iter.iter_with().skip(n_skipped).take(bookend) {
            buffer.push(fmt_fn(glyph_id, &items[ix]));
        }
    } else {
        for (ix, glyph_id) in ix_iter.iter_with() {
            buffer.push(fmt_fn(glyph_id, &items[ix]));
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

/// Formats a table column as an inline TokenStream for horizontally-oriented table-display.
///
/// The resulting `TokenStream` will begin with `heading` (which should be justified across lines of the same table),
/// and will contain up to `bookend` leading and trailing entries of the `items` array, each tokenized using `show_fn`,
/// and separated with a single whitespace character (`' '`).
///
/// Includes a conditional elision-string returned by `ellipsis` (parametrized by the number of entries elided).
///
/// # Notes
///
/// In order to ensure that each inline-column of a table has horizontally aligned entries, the tokenization of
/// each item, as well as the `ellipsis` string, should be fixed-width, e.g. by space-padding or 0-padding numeric values.
///
/// Furthermore, the `header` strings should all be of consistent width.
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
    mut show_fn: impl FnMut(&A) -> TokenStream<'static>,
    bookend: usize,
    ellipsis: impl FnOnce(usize) -> TokenStream<'static>,
) -> TokenStream<'static> {
    let count = items.len();
    let mut buf = Vec::with_capacity(Ord::min(count, 2 * bookend + 1));
    if count > 2 * bookend {
        let (left_bookend, _middle, right_bookend) =
            unsafe { trisect_unchecked(items, bookend, bookend) };

        for it in left_bookend {
            buf.push(show_fn(it));
        }

        let n_skipped = count - bookend * 2;
        buf.push(ellipsis(n_skipped));

        for it in right_bookend {
            buf.push(show_fn(it));
        }
    } else {
        buf.extend(items.iter().map(show_fn));
    }

    tok(heading).then(TokenStream::join_with(buf, tok(" ")))
}
