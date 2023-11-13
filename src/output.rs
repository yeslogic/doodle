use std::{
    borrow::{Borrow, Cow},
    fmt::{self, Write},
    rc::Rc,
};

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

#[derive(Clone, Default)]
pub enum Fragment {
    #[default]
    Empty,
    Symbol(Symbol),
    Char(char),
    String(Cow<'static, str>),
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
            Self::Symbol(symb) => f.debug_tuple("Symbol").field(symb).finish(),
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
    fn is_empty(&self) -> bool {
        matches!(self, &Fragment::Empty)
    }

    /// Joins two fragments with appropriate whitespace:
    ///   - If `other` fits on a single line with no trailing newline, joins with `' '`
    ///   - Otherwise, joins with `'\n'`
    fn join_with_wsp(self, other: Self) -> Self {
        if other.fits_inline() {
            self.cat(Self::Char(' ')).cat(other).cat_break()
        } else {
            self.cat_break().cat(other)
        }
    }

    /// Predicate for whether a fragment, when displayed, includes any line breaks
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
                        if items.len() >= 1 && !join.fits_inline() {
                            return false;
                        }
                    }
                }
                items.iter().all(Self::fits_inline)
            }
        }
    }

    /// Forms a compound fragment from a Fragment-valued iterable, with
    /// an optional Fragment separating each element in the output sequence.
    ///
    /// It is more efficient to pass `sep := None` than `sep := Some(Fragment::Empty)`,
    /// but the resulting output will differ in performance alone, and not output.
    fn seq(items: impl IntoIterator<Item = Fragment>, sep: Option<Fragment>) -> Self {
        Self::Sequence {
            items: items.into_iter().collect(),
            sep: sep.map(Box::new),
        }
    }

    /// Concatenates two fragments into one.
    ///
    /// If either fragment is empty, will short-circuit to the other
    /// to avoid needless allocation.
    fn cat(self, frag1: Self) -> Self {
        if self.is_empty() {
            frag1
        } else if frag1.is_empty() {
            self
        } else {
            Self::Cat(Box::new(self), Box::new(frag1))
        }
    }

    #[allow(dead_code)]
    fn opt(frag: Option<Fragment>) -> Fragment {
        frag.unwrap_or(Fragment::Empty)
    }

    /// Appends a given fragment to the receiver.
    ///
    /// Returns the same mutable reference as was passed in, to allow chaining of similar operations.
    #[inline]
    fn encat(&mut self, other: Self) -> &mut Self {
        let this = std::mem::take(self);
        *self = Self::cat(this, other);
        self
    }

    /// Wraps the current fragment in a [`Fragment::Group`] and returns the result.
    fn group(self) -> Self {
        Self::Group(Box::new(self))
    }

    /// Like [Fragment::group], except that it modifies a mutable reference in-place and passes it back to the caller
    fn engroup(&mut self) -> &mut Self {
        let this = Box::new(std::mem::take(self));
        *self = Self::Group(this);
        self
    }

    /// Return a new Fragment consisting of `self` with a newline appended to the end.
    #[inline]
    fn cat_break(self) -> Self {
        Self::cat(self, Fragment::Char('\n'))
    }

    /// Append a newline character to the receiver.
    ///
    /// Returns the same mutable reference as was passed in, to allow chaining of similar operations.
    #[inline]
    fn encat_break(&mut self) -> &mut Self {
        self.encat(Fragment::Char('\n'))
    }

    /// Returns an empty fragment
    fn new() -> Self {
        Self::Empty
    }

    /// Similar to `fits_inline`, but determines whether more than one line is displayed
    /// rather than whether inline concatenations are possible.
    ///
    /// Importantly, newline characters are permitted if only one appears at the very end,
    /// and Symbols are legal
    fn is_single_line(&self, is_final: bool) -> bool {
        match self {
            Fragment::Empty => true,
            Fragment::Char('\n') => is_final,
            Fragment::Char(_) => true,
            Fragment::String(s) =>  {
                let ix_nl = s.find('\n');
                match ix_nl {
                    Some(n) if n == s.len() - 1 => is_final,
                    None => true,
                    _ => false
                }
            },
            Fragment::Symbol(_) => true,
            Fragment::DisplayAtom(_) | Fragment::DebugAtom(_) => true,
            Fragment::Group(frag) => frag.is_single_line(is_final),
            Fragment::Cat(lhs, rhs) => lhs.is_single_line(false) && rhs.is_single_line(is_final),
            Fragment::Sequence { sep, items } => {
                match sep {
                    None => (),
                    Some(join) => {
                        if items.len() >= 1 && !join.is_single_line(false) {
                            return false;
                        }
                    }
                }
                let l = items.len();
                items.iter().enumerate().all(|(ix, frag)| frag.is_single_line(is_final && (ix == l - 1)))
            }
        }
    }
}

/// Builder pattern helper-struct for accumulating up longer sequences of [Fragment]s.
struct FragmentBuilder {
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
            Fragment::Symbol(symb) => symb.fmt(f),
            Fragment::String(s) => f.write_str(s.borrow()),
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
