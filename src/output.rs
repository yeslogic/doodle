use std::{fmt::Write, borrow::Borrow};

pub mod flat;
pub mod tree;

pub enum Fragment {
    Empty,
    Char(char),
    String(std::borrow::Cow<'static, str>),
    DisplayAtom(Box<dyn std::fmt::Display>),
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
            Self::DisplayAtom(arg0) => f.debug_tuple("DisplayAtom").field(&format!("{}", arg0)).finish(),
            Self::Group(arg0) => f.debug_tuple("Group").field(arg0).finish(),
            Self::Cat(arg0, arg1) => f.debug_tuple("Cat").field(arg0).field(arg1).finish(),
            Self::Sequence { sep, items } => f.debug_struct("Sequence").field("sep", sep).field("items", items).finish(),
        }
    }
}

impl Fragment {
    fn seq(items: impl IntoIterator<Item = Fragment>, sep: Option<Fragment>) -> Self {
        Self::Sequence {
            items: items.into_iter().collect(),
            sep: sep.map(Box::new),
        }
    }

    fn cat(frag0: Self, frag1: Self) -> Self {
        Self::Cat(Box::new(frag0), Box::new(frag1))
    }

    fn grp(self) -> Self {
        Self::Group(Box::new(self))
    }
}

struct Snippet {
    depth: u8,
    contents: Fragment,
}

impl std::fmt::Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fragment::Empty => Ok(()),
            Fragment::Char(c) => f.write_char(*c),
            Fragment::String(s) => f.write_str(s.borrow()),
            Fragment::DisplayAtom(atom) => atom.fmt(f),
            Fragment::Group(frag) => {
                frag.fmt(f)
            }
            Fragment::Cat(frag0, frag1) => {
                frag0.fmt(f)?;
                frag1.fmt(f)
            }
            Fragment::Sequence { sep, items } => {
                if items.is_empty() {
                    Ok(())
                } else {
                    let mut is_first = true;
                    for item in items.iter() {
                        if !is_first {
                            sep.as_ref().map_or(Ok(()), |frag| frag.fmt(f))?;
                        } else {
                            is_first = false;
                        }
                        item.fmt(f)?;
                    }
                    Ok(())
                }
            }
        }
    }
}