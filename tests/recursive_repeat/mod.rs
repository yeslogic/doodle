#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]
#![cfg_attr(rustfmt, rustfmt::skip)]

mod codegen_tests;
pub mod api_helper;

use doodle::prelude::*;
use doodle::try_sub;

/// expected size: 40
/// trait-unready: multiple (2) decoders exist (d#{0, 1})
#[derive(Debug, Clone)]
pub enum test_tree { Leaf(u8), Node(u8, Vec<test_tree>) }

/// d#0
fn Decoder_test_tree(input: &mut Parser<'_>) -> Result<test_tree, ParseError> {
Decoder1(input)
}

/// d#1
fn Decoder1(input: &mut Parser<'_>) -> Result<test_tree, ParseError> {
let tree_index = {
input.open_peek_context();
{
let ret = match input.read_byte()? {
76u8 => {
0
},

78u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(11876854719037224982u64));
}
};
input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = {
let b = input.read_byte()?;
if b == 76 {
b
} else {
return Err(ParseError::ExcludedBranch(13646096770106105413u64));
}
};
test_tree::Leaf(inner)
},

1 => {
let arg0 = {
let b = input.read_byte()?;
if b == 78 {
b
} else {
return Err(ParseError::ExcludedBranch(2206609067086327257u64));
}
};
let arg1 = {
let mut accum = Vec::new();
for _ in 0..2u32 {
let next_elem = (Decoder1(input))?;
accum.push(next_elem)
};
accum
};
test_tree::Node(arg0, arg1)
},

_ => {
return Err(ParseError::ExcludedBranch(18270091135093349626u64));
}
})
}

