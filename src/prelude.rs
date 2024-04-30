use std::ops::{Bound, RangeBounds};

pub use crate::byte_set::ByteSet;
pub use crate::parser::{
    error::{PResult, ParseError},
    monad::ParseMonad,
};

pub fn u16le(input: (u8, u8)) -> u16 {
    u16::from_le_bytes([input.0, input.1])
}

pub fn u16be(input: (u8, u8)) -> u16 {
    u16::from_be_bytes([input.0, input.1])
}

pub fn u32le(input: (u8, u8, u8, u8)) -> u32 {
    u32::from_le_bytes([input.0, input.1, input.2, input.3])
}

pub fn try_flat_map_vec<T, U, E, F>(iter: impl Iterator<Item = T>, f: F) -> Result<Vec<U>, E>
where
    F: Fn(T) -> Result<Vec<U>, E>,
{
    let mut res: Vec<U> = Vec::new();
    for x in iter {
        let mut y = f(x)?;
        res.append(&mut y);
    }
    Ok(res)
}

pub fn try_fold_map_curried<T, U, V, E, F>(
    iter: impl Iterator<Item = T>,
    init: V,
    f: F,
) -> Result<Vec<U>, E>
where
    F: Fn((V, T)) -> Result<(V, Vec<U>), E>,
{
    let mut res: Vec<U> = Vec::new();
    let mut accum = init;
    for x in iter {
        let (new_accum, mut y) = f((accum, x))?;
        res.append(&mut y);
        accum = new_accum;
    }
    Ok(res)
}

pub fn u32be(input: (u8, u8, u8, u8)) -> u32 {
    u32::from_be_bytes([input.0, input.1, input.2, input.3])
}

pub fn u64le(input: (u8, u8, u8, u8, u8, u8, u8, u8)) -> u64 {
    u64::from_le_bytes([
        input.0, input.1, input.2, input.3, input.4, input.5, input.6, input.7,
    ])
}

pub fn u64be(input: (u8, u8, u8, u8, u8, u8, u8, u8)) -> u64 {
    u64::from_be_bytes([
        input.0, input.1, input.2, input.3, input.4, input.5, input.6, input.7,
    ])
}

pub fn dup32<T: Clone>(count: u32, value: T) -> Vec<T> {
    Vec::from_iter(std::iter::repeat(value).take(count as usize))
}

pub fn parse_huffman(
    lengths: impl AsRef<[u8]>,
    code_values: Option<Vec<u8>>,
) -> impl for<'a> Fn(&mut ParseMonad<'a>) -> PResult<u16> {
    let lengths = lengths
        .as_ref()
        .iter()
        .map(|n| *n as usize)
        .collect::<Vec<usize>>();
    let lengths = match code_values {
        None => lengths,
        Some(e) => {
            let values = e.iter().map(|n| (*n).into()).collect::<Vec<usize>>();
            let mut new_lengths = [0].repeat(values.len());
            for i in 0..lengths.len() {
                new_lengths[values[i]] = lengths[i];
            }
            new_lengths
        }
    };
    make_huffman_decoder(&lengths)
}

pub fn make_huffman_decoder(
    lengths: &[usize],
) -> impl for<'a> Fn(&mut ParseMonad<'a>) -> PResult<u16> {
    let max_length = *lengths.iter().max().unwrap();
    let mut bl_count = [0].repeat(max_length + 1);

    for len in lengths {
        bl_count[*len] += 1;
    }

    let mut next_code = [0].repeat(max_length + 1);
    let mut code = 0;
    bl_count[0] = 0;

    for bits in 1..max_length + 1 {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
    }

    let mut driver = huffman::HuffmanDriver::new();

    for (n, &len) in lengths.iter().enumerate() {
        if len != 0 {
            driver.push(bit_range(len, next_code[len]), n.try_into().unwrap());
            next_code[len] += 1;
        }
    }

    move |p: &mut ParseMonad<'_>| driver.parse_one(p)
}

mod huffman {
    use super::{PResult, ParseMonad};

    #[derive(Clone, Debug)]
    pub(super) struct HuffmanDriver {
        tree_root: HuffmanNode,
    }

    #[derive(Clone, Debug, Default)]
    pub(super) enum HuffmanNode {
        #[default]
        Empty,
        Leaf(u16),
        Branch {
            children: [Box<HuffmanNode>; 2],
        },
    }

    impl HuffmanNode {
        pub fn is_leaf(&self) -> bool {
            matches!(self, Self::Leaf(..))
        }

        pub fn unpack(&self) -> u16 {
            let &HuffmanNode::Leaf(value) = self else {
                panic!("unpack called on non-leaf")
            };
            value
        }

        pub fn insert(&mut self, suffix: &[u8], value: u16) {
            match (self, suffix) {
                (this @ &mut HuffmanNode::Empty, []) => {
                    *this = HuffmanNode::Leaf(value);
                }
                (_, []) | (HuffmanNode::Leaf(..), &[_, ..]) => {
                    unreachable!("Huffman tree generator encountered collision")
                }
                (this @ &mut HuffmanNode::Empty, &[b @ (0 | 1), ..]) => {
                    let mut children = [Box::new(HuffmanNode::Empty), Box::new(HuffmanNode::Empty)];
                    (&mut children[b as usize]).insert(&suffix[1..], value);
                    *this = HuffmanNode::Branch { children };
                }
                (HuffmanNode::Branch { children }, &[b @ (0 | 1), ..]) => {
                    (&mut children[b as usize]).insert(&suffix[1..], value)
                }
                _ => unreachable!("huffman non-bit value encountered"),
            }
        }

        pub fn follow(&self, b: u8) -> &Self {
            match self {
                HuffmanNode::Empty => unreachable!("empty tree encountered after construction"),
                HuffmanNode::Leaf(_) => unreachable!("impossible descent from leaf"),
                HuffmanNode::Branch { children } => &*children[b as usize],
            }
        }
    }

    impl HuffmanDriver {
        pub fn new() -> Self {
            Self {
                tree_root: HuffmanNode::Empty,
            }
        }

        pub fn push(&mut self, code: Vec<u8>, value: u16) {
            self.tree_root.insert(&code[..], value)
        }

        pub fn parse_one(&self, p: &mut ParseMonad<'_>) -> PResult<u16> {
            let mut node = &self.tree_root;
            while !node.is_leaf() {
                let b = p.read_byte()?;
                node = node.follow(b);
            }
            Ok(node.unpack())
        }
    }
}

fn bit_range(nbits: usize, value: usize) -> Vec<u8> {
    let mut fs = Vec::with_capacity(nbits);
    for i in 0..nbits {
        let r = nbits - 1 - i;
        let b = ((value & (1 << r)) >> r) != 0;
        fs.push(bit_as_u8(b));
    }
    fs
}

fn bit_as_u8(b: bool) -> u8 {
    if b {
        1
    } else {
        0
    }
}

pub fn extend_from_within_ext(vs: &mut Vec<u8>, range: std::ops::Range<usize>) {
    match range.end_bound() {
        Bound::Excluded(&rangemax) if rangemax <= vs.len() => vs.extend_from_within(range),
        Bound::Included(&lastix) if lastix < vs.len() => vs.extend_from_within(range),
        Bound::Unbounded => panic!("cannot extend indefinitely"),
        _ => {
            for i in range {
                vs.push(vs[i])
            }
        }
    }
}
