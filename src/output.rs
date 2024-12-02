use std::{
    fmt::{self, Write},
    rc::Rc,
};

use crate::Label;

pub mod flat;
pub mod tree;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    Vacuum,
    Elbow,
    Pipe,
    Junction,
}

impl fmt::Display for Symbol {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vacuum   => f.write_str("    "),
            Self::Pipe     => f.write_str("│   "),
            Self::Junction => f.write_str("├── "),
            Self::Elbow    => f.write_str("└── "),
        }
    }
}

// FIXME - add support for soft-newline (i.e. conditional line-break if no linebreak occurs before next printable character)
#[derive(Clone, Default)]
pub enum Fragment {
    #[default]
    Empty,
    Symbol(Symbol),
    Char(char),
    String(Label),
    DebugAtom(Rc<dyn fmt::Debug>),
    DisplayAtom(Rc<dyn fmt::Display>),
    Group(Box<Fragment>),
    Cat(Box<Fragment>, Box<Fragment>),
    Sequence {
        sep: Option<Box<Fragment>>,
        items: Vec<Fragment>,
    },
}

impl fmt::Debug for Fragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Char(c) => f.debug_tuple("Char").field(c).finish(),
            Self::Symbol(symbol) => f.debug_tuple("Symbol").field(symbol).finish(),
            Self::String(s) => f.debug_tuple("String").field(s).finish(),
            Self::DebugAtom(at) => f
                .debug_tuple("DebugAtom")
                .field(&format!("{:?}", at))
                .finish(),
            Self::DisplayAtom(at) => f
                .debug_tuple("DisplayAtom")
                .field(&format!("{}", at))
                .finish(),
            Self::Group(grp) => f.debug_tuple("Group").field(grp).finish(),
            Self::Cat(x, y) => f.debug_tuple("Cat").field(x).field(y).finish(),
            Self::Sequence { sep, items } => f
                .debug_struct("Sequence")
                .field("sep", sep)
                .field("items", items)
                .finish(),
        }
    }
}

impl Fragment {
    /// Returns true if a [Fragment] directly matches [`Fragment::Empty`].
    ///
    /// Note that this predicate will fail on certain pathological cases that happen to
    /// also render as zero-length strings. If a broader condition of vacuous rendering is
    /// needed, [`Fragment::is_vacuous`] may be used instead.
    ///
    /// In most cases, `is_empty` will be good enough, as we will avoid constructing vacuous
    /// Fragments other than [`Fragment::Empty`].
    fn is_empty(&self) -> bool {
        matches!(self, &Fragment::Empty)
    }

    /// Returns true if a [Fragment] produces a zero-length string when rendered
    ///
    /// The set of Fragments that satisfy this predicate is a super-set of those that satisfy [`Fragment::is_empty`].
    #[allow(dead_code)]
    fn is_vacuous(&self) -> bool {
        match self {
            Fragment::Empty => true,
            Fragment::Symbol(_) => false,
            Fragment::Char(_) => false,
            Fragment::String(s) => s.len() == 0,
            // in practice, we will not use DisplayAtom or DebugAtom if they entail zero-length output
            Fragment::DebugAtom(_) | Fragment::DisplayAtom(_) => false,
            Fragment::Group(g) => g.is_vacuous(),
            Fragment::Cat(x, y) => x.is_vacuous() && y.is_vacuous(),
            Fragment::Sequence { sep, items } => {
                match items.len() {
                    0 => true,
                    1 => items[0].is_vacuous(), // sep can be non-vacuous if there is only one item
                    _ => {
                        sep.as_ref().map_or(true, |frag| frag.is_vacuous())
                            && items.iter().all(Fragment::is_vacuous)
                    }
                }
            }
        }
    }

    /// Forms a compound fragment from a Fragment-valued iterable, with
    /// an optional Fragment separating each element in the output sequence.
    ///
    /// It is possibly more efficient to pass `sep := None` than `sep := Some(Fragment::Empty)`,
    /// but the resulting output will differ in performance alone, and not output.
    pub fn seq(items: impl IntoIterator<Item = Fragment>, sep: Option<Fragment>) -> Self {
        Self::Sequence {
            items: items.into_iter().collect(),
            sep: sep.map(Box::new),
        }
    }

    /// Concatenates two fragments into one.
    ///
    /// If either fragment is empty, will short-circuit to the other
    /// to avoid needless allocation.
    pub fn cat(self, frag1: Self) -> Self {
        if self.is_empty() {
            frag1
        } else if frag1.is_empty() {
            self
        } else {
            Self::Cat(Box::new(self), Box::new(frag1))
        }
    }

    /// Returns a new Fragment surrounded by two other Fragments
    ///
    /// Useful for parenthesizing function argument-lists, quoting string contents, and other such cases.
    pub fn delimit(self, before: Self, after: Self) -> Self {
        Self::cat(before, self).cat(after)
    }

    /// Shorthand for [`Fragment::String`] that can be called on `String` and `&'static str` as well as `Label`.
    pub fn string(s: impl Into<Label>) -> Self {
        Self::String(s.into())
    }

    /// Apply a Fragment-producing closure to an `Option`, or return `Fragment::Empty` if `None`
    pub fn opt<T>(o_val: Option<T>, f: impl FnOnce(T) -> Self) -> Self {
        if let Some(val) = o_val {
            f(val)
        } else {
            Self::Empty
        }
    }

    /// Adds an intervening fragment between two others, but only if both the left and right halves are non-vacuous.
    ///
    /// This avoids situations where a separator is injected at the beginning or end of a string, or where multiple
    /// separators appear in turn with nothing in between them.
    ///
    /// # Examples
    ///
    /// ```
    /// use doodle::output::Fragment;
    /// let frg_hello = Fragment::String("hello".into());
    /// let frg_world = Fragment::String("world".into());
    /// let x = frg_hello.intervene(Fragment::Char(' '), frg_world);
    /// assert_eq!(&format!("{x}"), "hello world")
    /// ```
    ///
    /// ```
    /// use doodle::output::Fragment;
    /// let frg_hello = Fragment::String("hello".into());
    /// let frg_nothing = Fragment::String("".into());
    /// let x = frg_hello.intervene(Fragment::Char(' '), frg_nothing);
    /// assert_eq!(&format!("{x}"), "hello")
    /// ```
    #[allow(dead_code)]
    pub fn intervene(self, sep: Self, other: Self) -> Self {
        if self.is_vacuous() {
            other
        } else if other.is_vacuous() {
            self
        } else if sep.is_vacuous() {
            self.cat(other)
        } else {
            self.cat(sep).cat(other)
        }
    }

    /// Appends a given fragment to the receiver.
    ///
    /// Returns the same mutable reference as was passed in, to allow chaining of similar operations.
    #[inline]
    pub fn append(&mut self, other: Self) -> &mut Self {
        let this = std::mem::take(self);
        *self = Self::cat(this, other);
        self
    }

    /// Wraps the current fragment in a [`Fragment::Group`] and returns the result.
    ///
    /// # Note
    ///
    /// There are currently no display rules that differentiate [`Fragment::Group`](x) from `x` itself, but
    /// it is defined anyway and supported with this helper to allow for cleaner adoption of a more nuanced
    /// model in which logically-grouped fragments have their own display rules.
    pub(crate) fn group(self) -> Self {
        Self::Group(Box::new(self))
    }

    /// Like [Fragment::group], except that it modifies a mutable reference in-place and passes it back to the caller
    pub(crate) fn enclose(&mut self) -> &mut Self {
        let this = Box::new(std::mem::take(self));
        *self = Self::Group(this);
        self
    }

    /// Return a new Fragment consisting of `self` with a newline appended to the end.
    #[inline]
    pub fn cat_break(self) -> Self {
        Self::cat(self, Fragment::Char('\n'))
    }

    /// Append a newline character to the receiver.
    ///
    /// Returns the same mutable reference as was passed in, to allow chaining of similar operations.
    #[inline]
    fn append_break(&mut self) -> &mut Self {
        self.append(Fragment::Char('\n'))
    }

    /// Returns an empty fragment
    fn new() -> Self {
        Self::Empty
    }

    /// Returns `true` if the printable representation of `self` fits on a single line.
    ///
    /// Similar to [`Fragment::fits_inline`], but determines whether more than one line is displayed
    /// rather than whether inline concatenations are possible.
    ///
    /// Importantly, a single trailing newline character is permitted, and `Symbols` care allowed to appear anywhere
    pub(crate) fn is_single_line(&self, is_final: bool) -> bool {
        match self {
            Fragment::Empty => true,
            Fragment::Char('\n') => is_final,
            Fragment::Char(_) => true,
            Fragment::String(s) => {
                let ix_nl = s.find('\n');
                match ix_nl {
                    Some(n) if n == s.len() - 1 => is_final,
                    None => true,
                    _ => false,
                }
            }
            Fragment::Symbol(_) => true,
            Fragment::DisplayAtom(_) | Fragment::DebugAtom(_) => true,
            Fragment::Group(frag) => frag.is_single_line(is_final),
            Fragment::Cat(lhs, rhs) => lhs.is_single_line(false) && rhs.is_single_line(is_final),
            Fragment::Sequence { sep, items } => {
                match sep {
                    None => (),
                    Some(join) => {
                        if !items.is_empty() && !join.is_single_line(false) {
                            return false;
                        }
                    }
                }
                let l = items.len();
                items
                    .iter()
                    .enumerate()
                    .all(|(ix, frag)| frag.is_single_line(is_final && (ix == l - 1)))
            }
        }
    }

    /// Returns `true` if this fragment can be appended to another as an inline continuation,
    /// without inadvertently adding linebreaks or mis-aligning leading `Symbol`s.
    ///
    /// Returns `false` if any of the following are true:
    ///   - The Display form of `self` contains any newline characters
    ///   - `self` contains any `Symbol` sub-fragments
    fn fits_inline(&self) -> bool {
        match self {
            Fragment::Empty => true,
            Fragment::Char(c) => *c != '\n',
            Fragment::String(s) => !s.contains('\n'),
            Fragment::Symbol(_) => false,
            Fragment::DisplayAtom(_) | Fragment::DebugAtom(_) => true,
            Fragment::Group(frag) => frag.fits_inline(),
            Fragment::Cat(lhs, rhs) => lhs.fits_inline() && rhs.fits_inline(),
            Fragment::Sequence { sep, items } => {
                match sep {
                    None => (),
                    Some(join) => {
                        if !items.is_empty() && !join.fits_inline() {
                            return false;
                        }
                    }
                }
                items.iter().all(Self::fits_inline)
            }
        }
    }

    /// Joins two fragments with appropriate whitespace:
    ///   - If `other` fits on a single line with no trailing newline, joins with `' '`, with a newline at the very end
    ///   - Otherwise, joins with `'\n'`
    pub(crate) fn join_with_wsp(self, other: Self) -> Self {
        if other.fits_inline() {
            self.cat(Self::Char(' ')).cat(other).cat_break()
        } else {
            self.cat_break().cat(other)
        }
    }

    /// Joins two fragments with the appropriate whitespace and a conditional line-ending `trailer`:
    ///    - If `other` does not require more than one line to print, joins with `' '`, with a newline at the very end (and without `trailer`).
    ///    - Otherwise, joins `self` and `trailer` with no separation, followed by `other` on the following line
    pub(crate) fn join_with_wsp_eol(self, other: Self, trailer: Self) -> Self {
        // FIXME - As currently implemented, if `other` contains Symbols, calling this method may break them
        if other.fits_inline() {
            self.cat(Self::Char(' ')).cat(other).cat_break()
        } else {
            self.cat(trailer).cat_break().cat(other)
        }
    }
}

/// Builder pattern helper-struct for accumulating up longer sequences of [Fragment]s.
pub(crate) struct FragmentBuilder {
    frozen: Vec<Fragment>,
    active: Fragment,
}

impl FragmentBuilder {
    pub fn new() -> Self {
        Self {
            frozen: Vec::new(),
            active: Fragment::new(),
        }
    }

    pub fn active_mut(&mut self) -> &mut Fragment {
        &mut self.active
    }

    /// Freezes the currently-active fragment and returns a mutable reference to a new active fragment,
    /// which will be reinitialized to [`Fragment::Empty`].
    pub fn renew(&mut self) -> &mut Fragment {
        let frag = std::mem::take(&mut self.active);
        if !frag.is_empty() {
            self.frozen.push(frag);
        }
        &mut self.active
    }

    /// Compound operation that freezes the current active and immediately adds a new frozen fragment, leaving
    /// the active fragment empty.
    pub fn push(&mut self, frag: Fragment) {
        let old = std::mem::take(&mut self.active);

        if old.is_empty() {
            self.frozen.push(frag);
        } else {
            self.frozen.push(old);
            if !frag.is_empty() {
                self.frozen.push(frag);
            }
            self.active = Fragment::new();
        }
    }

    pub fn finalize(mut self) -> Fragment {
        let _ = self.renew();
        Fragment::seq(self.frozen, None)
    }

    pub fn finalize_with_sep(mut self, sep: Fragment) -> Fragment {
        let _ = self.renew();
        Fragment::seq(self.frozen, Some(sep))
    }
}

impl fmt::Display for Fragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fragment::Empty => Ok(()),
            Fragment::Char(c) => f.write_char(*c),
            Fragment::Symbol(symbol) => symbol.fmt(f),
            Fragment::String(s) => f.write_str(s.as_ref()),
            Fragment::DebugAtom(atom) => fmt::Debug::fmt(&atom, f),
            Fragment::DisplayAtom(atom) => fmt::Display::fmt(&atom, f),
            Fragment::Group(frag) => frag.fmt(f),
            Fragment::Cat(frag0, frag1) => {
                frag0.fmt(f)?;
                frag1.fmt(f)
            }
            Fragment::Sequence { sep, items } => {
                let mut iter = items.iter();
                if let Some(head) = iter.next() {
                    head.fmt(f)?;
                } else {
                    return Ok(());
                }
                let f_sep: Box<dyn Fn(&mut fmt::Formatter<'_>) -> fmt::Result> =
                    if let Some(frag) = sep.as_deref() {
                        Box::new(|f| frag.fmt(f))
                    } else {
                        Box::new(|_| Ok(()))
                    };
                for item in iter {
                    f_sep(f)?;
                    item.fmt(f)?;
                }
                Ok(())
            }
        }
    }
}
