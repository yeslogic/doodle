use std::collections::BTreeMap as Map;

/// Helper trait for marking container-types that can be considered "null"
pub trait IsNull {
    /// Returns `true` if the container is empty.
    fn is_null(&self) -> bool;
}

impl<T> IsNull for Vec<T> {
    fn is_null(&self) -> bool {
        self.is_empty()
    }
}

impl<T> IsNull for Option<T> {
    fn is_null(&self) -> bool {
        self.is_none()
    }
}

/// Abstraction of a linear array of `Vec<T>` optimized for the vast majority of entries
/// being empty-vectors.
#[derive(Clone, Debug)]
pub(crate) struct IxHeap<T: IsNull> {
    // FIXME - implement a more efficient storage container
    _store: Map<usize, T>,
    _len: usize,
}

impl<T: IsNull + serde::Serialize> serde::Serialize for IxHeap<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self._store.serialize(serializer)
    }
}

impl<T: IsNull> IxHeap<T> {
    /// Constructs a new, empty `IxHeap`.
    pub const fn new() -> Self {
        Self {
            _store: Map::new(),
            _len: 0,
        }
    }

    /// Returns the logical length of an `IxHeap`, which includes empty entries.
    #[expect(dead_code)]
    pub const fn len(&self) -> usize {
        self._len
    }

    /// Pushes an element into the `IxHeap`.
    ///
    /// If the element is null, only the length is changed, and no element is stored.
    pub fn push(&mut self, elem: T) {
        if elem.is_null() {
            self.push_empty()
        } else {
            self.push_nonempty(elem)
        }
    }

    fn push_empty(&mut self) {
        self._len += 1;
    }

    fn push_nonempty(&mut self, elem: T) {
        self._store.insert(self._len, elem);
        self._len += 1;
    }
}

impl<T> IxHeap<Vec<T>> {
    /// Retrieves the element at index `ix`, or a null-value (i.e. empty slice) if the logical element at that index is null.
    ///
    /// # Panics
    ///
    /// Will panic if the specified index is out of bounds.
    pub fn get(&self, ix: usize) -> &[T] {
        assert!(ix < self._len, "index out of bounds");
        self._store.get(&ix).map(Vec::as_slice).unwrap_or(&[])
    }
}

impl<T> std::ops::Index<usize> for IxHeap<Vec<T>> {
    type Output = [T];

    fn index(&self, ix: usize) -> &[T] {
        self.get(ix)
    }
}

pub(crate) mod with_err {
    /// Product type for holding a normal value along with any number of error-values, as an alternative or extension to `Result`
    /// when certain errors are non-fatal under specific contexts.
    #[derive(Clone, Debug)]
    pub struct WithErr<T, E0> {
        value: T,
        errs: Vec<E0>,
    }

    impl<T, E0> WithErr<T, E0> {
        /// Constructs a new `WithErr` with the provided value, and no errors.
        pub const fn new(value: T) -> Self {
            Self {
                value,
                errs: Vec::new(),
            }
        }

        /// Constructs a new `WithErr` with the provided value and error.
        pub fn with_err(value: T, err: E0) -> Self {
            Self {
                value,
                errs: vec![err],
            }
        }

        /// Constructs a new `WithErr` with the provided value and error vector.
        pub const fn from_parts(value: T, errs: Vec<E0>) -> Self {
            Self { value, errs }
        }

        /// Transforms the value of `self` while preserving its held errors, if any.
        pub fn map<U>(self, f: impl FnOnce(T) -> U) -> WithErr<U, E0> {
            WithErr {
                value: f(self.value),
                errs: self.errs,
            }
        }

        /// Applies a closure `f` to the value held by `self`, aggregating the errors of `self` with those of the
        /// result.
        ///
        /// If `f` returns `Ok(other)`, this function returns `other` with any errors from `self` appended.
        /// Otherwise, returns the same `Err` that `f` returned.
        pub fn join<U, E1>(self, mut f: impl FnMut(T) -> EResult<U, E0, E1>) -> EResult<U, E0, E1> {
            let mut this_errs = self.errs;
            let mut ret = f(self.value)?;
            ret.errs.append(&mut this_errs);
            Ok(ret)
        }

        /// Given an initial accumulator `init` and an iterable container `values`,
        /// applies a left-fold using closure `f` to iteratively update the accumulator using each element in
        /// `values`, ultimately returning the final value of the accumulator.
        ///
        /// If at any point, an error is produced by `f`, it is immediately returned, short-circuiting
        /// past any remaining iteration.
        pub fn fold<U, E1>(
            init: U,
            values: impl IntoIterator<Item = T>,
            f: impl Fn(U, T) -> EResult<U, E0, E1>,
        ) -> EResult<U, E0, E1> {
            let mut acc = WithErr::new(init);
            for value in values {
                let mut this_errs = acc.errs;
                let mut new_acc = f(acc.value, value)?;
                new_acc.errs.append(&mut this_errs);
                acc = new_acc;
            }
            Ok(acc)
        }

        /// Extracts the value of `self`, discarding any errors it contained.
        pub fn into_inner(self) -> T {
            self.value
        }

        /// Extracts the value of `self`, logging any errors it contains at the `log::warn` level.
        pub fn extract_warn(self) -> T
        where
            E0: std::fmt::Display,
        {
            for err in self.errs.iter() {
                log::warn!("[non-fatal error]: {err}");
            }
            self.value
        }

        /// Returns `true` if there are any errors.
        pub const fn has_errs(&self) -> bool {
            !self.errs.is_empty()
        }

        /// Returns `Ok(self.value)` if there are no errors, and `Err(self.errs)` otherwise.
        pub fn into_strict(self) -> Result<T, Vec<E0>> {
            if !self.errs.is_empty() {
                Err(self.errs)
            } else {
                Ok(self.value)
            }
        }

        /// Returns an iterator over the errors held by `self`.
        pub fn iter_errs(&self) -> impl Iterator<Item = &E0> {
            self.errs.iter()
        }

        /// Given an external mutable reference to a vector of errors, appends the errors of `self` to it and returns the value of `self`.
        ///
        /// Primarily intended to avoid having to `join` many different `WithErr<_, E0>` values in succession when ultimately constructing a single
        /// `WithErr<T, E0>` value from their contents.
        pub fn lift(mut self, outer_errs: &mut Vec<E0>) -> T {
            outer_errs.append(&mut self.errs);
            self.value
        }
    }

    impl<T, E> AsRef<T> for WithErr<T, E> {
        fn as_ref(&self) -> &T {
            &self.value
        }
    }

    pub type EResult<T, E0, E1 = E0> = Result<WithErr<T, E0>, E1>;

    pub fn downgrade_error<T, E0>(val: EResult<T, E0>, default: T) -> WithErr<T, E0>
    where
        E0: std::fmt::Display,
    {
        match val {
            Ok(v) => v,
            Err(e) => {
                log::error!("downgraded error: {e}");
                WithErr::with_err(default, e)
            }
        }
    }
}
pub(crate) use with_err::{EResult, WithErr, downgrade_error};

pub trait ErrTrace {
    fn with_trace<T>(self, trace: T) -> Self
    where
        T: std::fmt::Debug + Send + Sync + 'static;
}

#[macro_export]
/// Perform a `?` operation but add additional trace-context to TCError values if encountered
///
/// # Syntax
///
/// ```ignore
/// try_with!( self.unify_var_pair(v1, v2) => ("unify_var_pair", v1, v2) );
/// try_with!( self.unify_var_pair(v1, v2) ); // equivalent to `?`
/// ```
#[allow(unused_macros)]
macro_rules! try_with {
    ($x:expr_2021 => $y:expr_2021) => {
        match $x {
            Ok(val) => val,
            Err(e) => return Err(e.with_trace($y)),
        }
    };
    ($x:expr_2021 $(=> ())?) => {
        $x?
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lift_syntax() {
        let x = {
            let mut errs = Vec::new();
            let a = WithErr::with_err("zero", 0).lift(&mut errs);
            let b = WithErr::with_err("one", 1).lift(&mut errs);
            let c = WithErr::with_err("two", 2).lift(&mut errs);
            let res = vec![a, b, c];
            WithErr::from_parts(res, errs)
        };
        let errs = x.iter_errs().copied().collect::<Vec<_>>();
        let val = x.into_inner();
        assert_eq!(&errs[..], &[0, 1, 2]);
        assert_eq!(&val[..], &["zero", "one", "two"]);
    }
}
