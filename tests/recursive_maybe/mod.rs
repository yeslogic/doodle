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

/// expected size: 16
/// trait-unready: multiple (2) decoders exist (d#{0, 1})
#[derive(Debug, Clone)]
pub struct test_chain {
tag: u8,
next: Option<Box<test_chain>>
}

/// d#0
fn Decoder_test_chain(input: &mut Parser<'_>) -> Result<test_chain, ParseError> {
Decoder1(input)
}

/// d#1
fn Decoder1(input: &mut Parser<'_>) -> Result<test_chain, ParseError> {
let tag = {
let b = input.read_byte()?;
if (ByteSet::from_bits([0u64, 40u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(13646096770106105413u64));
}
};
let next = if tag == 67u8 {
Some((Decoder1(input))?)
} else {
None
};
PResult::Ok(test_chain { tag, next: next.map(Box::new) })
}

