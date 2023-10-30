use std::{borrow::Borrow, fmt::Write};

pub mod flat;
pub mod tree;

#[derive(Clone, Default)]
pub enum Fragment {
    #[default]
    Empty,
    Char(char),
    String(std::borrow::Cow<'static, str>),
    DisplayAtom(std::rc::Rc<dyn std::fmt::Display>),
    Group(Box<Fragment>),
    Cat(Box<Fragment>, Box<Fragment>),
    Sequence {
        sep: Option<Box<Fragment>>,
        items: Vec<Fragment>,
    },
}

impl std::fmt::Debug for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Char(arg0) => f.debug_tuple("Char").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::DisplayAtom(arg0) => f
                .debug_tuple("DisplayAtom")
                .field(&format!("{}", arg0))
                .finish(),
            Self::Group(arg0) => f.debug_tuple("Group").field(arg0).finish(),
            Self::Cat(arg0, arg1) => f.debug_tuple("Cat").field(arg0).field(arg1).finish(),
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

    /// Append a newline character to the receiver.
    ///
    /// Returns the same mutable reference as was passed in, to allow chaining of similar operations.
    #[inline]
    fn enbreak(&mut self) -> &mut Self {
        self.encat(Fragment::Char('\n'))
    }

    /// Like [Fragment::group], except that it modifies a mutable reference in-place and passes it back to the caller
    fn engroup(&mut self) -> &mut Self {
        let this = Box::new(std::mem::take(self));
        *self = Self::Group(this);
        self
    }

    /// Returns an empty fragment
    fn new() -> Self {
        Self::Empty
    }
}

/// helper struct for building up longer sequences of [Fragment]s
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

impl std::fmt::Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fragment::Empty => Ok(()),
            Fragment::Char(c) => f.write_char(*c),
            Fragment::String(s) => f.write_str(s.borrow()),
            Fragment::DisplayAtom(atom) => atom.fmt(f),
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
                let f_sep: Box<dyn Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result> =
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
