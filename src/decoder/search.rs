use crate::decoder::{seq_kind::SeqKind, Value};
use crate::Expr;

/// Helper trait to apply find_index_by_key_sorted to ParsedValue and Value generically
pub(crate) trait AsKey {
    /// Compares two values as keys, using natural order on the types being represented.
    ///
    /// Currently, only applies to strictly-numeric Value kinds (U8, etc.)
    fn compare_as_key(&self, other: &Self) -> std::cmp::Ordering;

    /// Tests equality over two values as keys, using natural equality on the types being represented.
    ///
    /// Currently, only applies to strictly-numeric Value kinds (U8, etc.), to mirror `compare_as_key`
    /// (even though more complex equalities can be established on value-kinds without natural ordering).
    fn eq_key(&self, other: &Self) -> bool;
}

impl AsKey for Value {
    fn compare_as_key(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::U8(a), Value::U8(b)) => a.cmp(b),
            (Value::U16(a), Value::U16(b)) => a.cmp(b),
            (Value::U32(a), Value::U32(b)) => a.cmp(b),
            (Value::U64(a), Value::U64(b)) => a.cmp(b),
            _ => panic!("Value::compare_as_key: Can't compare {self:?} and {other:?} as keys"),
        }
    }

    fn eq_key(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::U8(a), Value::U8(b)) => a == b,
            (Value::U16(a), Value::U16(b)) => a == b,
            (Value::U32(a), Value::U32(b)) => a == b,
            (Value::U64(a), Value::U64(b)) => a == b,
            _ => panic!("Value::eq_key: can't compare {self:?} and {other:?} as keys"),
        }
    }
}

/// Helper struct to avoid evaluating key-values at the same index more than once
pub(crate) struct KeyCache<T>(Vec<std::cell::OnceCell<T>>);

impl<T> KeyCache<T> {
    pub(crate) fn new(len: usize) -> Self {
        let store = (0..len).map(|_| std::cell::OnceCell::new()).collect();
        Self(store)
    }

    pub(crate) fn get_or_init(&self, index: usize, f: impl FnOnce() -> T) -> &T {
        self.0[index].get_or_init(f)
    }
}

pub(crate) fn find_index_by_key_sorted<'a, V, V0, Eval>(
    f_get_key: &Expr,
    query: &V,
    values: &SeqKind<V0>,
    evaluate: Eval,
) -> Option<usize>
where
    Eval: 'a + Fn(&Expr, &V0) -> V,
    V: AsKey,
    V0: Clone,
{
    use std::cmp::Ordering;
    // If values is empty, search is trivial
    if values.is_empty() {
        return None;
    }

    let len = values.len();

    // cache to store keys we have seen after computing them once
    let cache = KeyCache::<V>::new(len);
    // helper closure to keep code below lightweight and more implementation-agnostic
    let get_key_at_index = |ix: usize| cache.get_or_init(ix, || evaluate(f_get_key, &values[ix]));

    // check boundaries, once only, at very start
    let lower_bound = get_key_at_index(0);

    // don't bother evaluating upper_bound if query <= lower-bound
    match query.compare_as_key(lower_bound) {
        Ordering::Less => return None,
        Ordering::Equal => return Some(0),
        Ordering::Greater => {
            // skip computing 'upper bound' on singleton list
            if len <= 1 {
                return None;
            }
        }
    }

    // safe because values cannot be empty
    let last_ix = len - 1;
    let upper_bound = get_key_at_index(last_ix);

    match query.compare_as_key(&upper_bound) {
        Ordering::Greater => return None,
        Ordering::Equal => return Some(last_ix),
        Ordering::Less => {
            // skip entire loop when there are no middle values
            if len <= 2 {
                return None;
            }
        }
    }

    // binary search, after already checking edge-cases bounds

    let mut lower_bound_ix = 0;
    let mut upper_bound_ix = last_ix;

    while lower_bound_ix <= upper_bound_ix {
        // if lower_bound_ix == upper_bound_ix, this will converge to that
        // value, and any branch below besides return, will break the loop
        // condition
        let mid = (lower_bound_ix + upper_bound_ix) / 2;

        let mid_key = get_key_at_index(mid);
        match query.compare_as_key(&mid_key) {
            Ordering::Less => upper_bound_ix = mid - 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => lower_bound_ix = mid + 1,
        }
    }
    None
}

pub(crate) fn find_index_by_key_unsorted<'a, V, V0, Eval>(
    f_get_key: &'a Expr,
    query: &V,
    values: &SeqKind<V0>,
    evaluate: Eval,
) -> Option<usize>
where
    Eval: 'a + Fn(&Expr, &V0) -> V,
    V: AsKey,
    V0: Clone,
{
    for (ix, v) in values.iter().enumerate() {
        let key = evaluate(f_get_key, v);
        if query.eq_key(&key) {
            return Some(ix);
        }
    }
    None
}
