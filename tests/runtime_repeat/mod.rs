#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt::skip)]

mod codegen_tests;
pub mod api_helper;

use doodle::prelude::*;
use doodle::try_sub;

impl CommonObject for test_inner {
type Args<'x> = ();

type Output<'x> = test_inner;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_test_inner(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#2)
#[derive(Debug, Clone)]
pub struct test_inner {
a: u8,
bs: Vec<u8>
}

/// expected size: 32
/// trait-unready: multiple (2) decoders exist (d#{0, 1})
#[derive(Debug, Clone)]
pub struct test_outer {
pairs: Vec<test_inner>,
end: u8
}

/// d#0
fn Decoder_test_outer(_input: &mut Parser<'_>) -> Result<test_outer, ParseError> {
Decoder1(_input)
}

/// d#1
fn Decoder1(_input: &mut Parser<'_>) -> Result<test_outer, ParseError> {
let pairs = {
let mut accum = Vec::new();
for _ in 0..2u32 {
let next_elem = (Decoder_test_inner(_input))?;
accum.push(next_elem)
};
accum
};
let end = {
let b = _input.read_byte()?;
if b == 204 {
b
} else {
return Err(ParseError::ExcludedBranch(13646096770106105413u64));
}
};
PResult::Ok(test_outer { pairs, end })
}

/// d#2
fn Decoder_test_inner(_input: &mut Parser<'_>) -> Result<test_inner, ParseError> {
let a = {
let b = _input.read_byte()?;
if b == 170 {
b
} else {
return Err(ParseError::ExcludedBranch(2206609067086327257u64));
}
};
let bs = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
187u8 => {
0
},

204u8 => {
1
},

170u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(11876854719037224982u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 187 {
b
} else {
return Err(ParseError::ExcludedBranch(18270091135093349626u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(test_inner { a, bs })
}
