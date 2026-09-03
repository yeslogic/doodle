/// Ternary result-type used when iteratively/recursively querying individual [`Lens`]es in a [`ViewStack`].
///
/// Any given query that yields an `Answer<T>` will either report the expected result `T` as [`Answer::Found`], or will
/// inform the driver whether it should continue to query the next [`Lens`] in the stack ([`Answer::Continue`]), or
/// stop the search early and report no answer ([`Answer::Blocked`]) — as certain lenses may not contain an answer,
/// but their presence alone can rule out the possibility of an answer being found further down.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Answer<T> {
    /// The query resolved to a definite answer.
    Found(T),
    /// This [`Lens`] has no answer of its own, but does not preclude one being found further down the stack.
    Continue,
    /// This [`Lens`] has no answer, and its mere presence rules out any answer that might otherwise be found further down the stack.
    Blocked,
}

impl<T> Answer<T> {
    /// Chaining operation to allow conditional continuation of a query, if it has not yet been answered or precluded.
    ///
    /// If `self` is [`Answer::Continue`], calls `f()` and returns its result.
    ///
    /// If an answer has been found or has been precluded (i.e. [`Answer::Found`] or [`Answer::Blocked`]), returns `self` without calling `f`.
    pub fn or_else(self, f: impl FnOnce() -> Answer<T>) -> Answer<T> {
        match self {
            Answer::Blocked => Answer::Blocked,
            Answer::Continue => f(),
            Answer::Found(val) => Answer::Found(val),
        }
    }

    /// If `self` is [`Answer::Found(val)`](Answer::Found), returns `Some(val)`.
    ///
    /// Otherwise, returns `None`, erasing the distinction betwween [`Answer::Continue`] and [`Answer::Blocked`].
    #[allow(clippy::wrong_self_convention)]
    pub fn to_option(self) -> Option<T> {
        match self {
            Answer::Found(val) => Some(val),
            Answer::Continue | Answer::Blocked => None,
        }
    }
}
