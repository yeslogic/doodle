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

impl CommonObject for peano {
type Args<'x> = ();

type Output<'x> = peano;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_peano(p)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#1)
#[derive(Debug, Clone)]
pub enum peano { S(u8, Box<peano>), Z(u8) }

impl CommonObject for ping {
type Args<'x> = ();

type Output<'x> = ping;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_ping(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#2)
#[derive(Debug, Clone)]
pub enum ping { Done(u8), More(u8, pong) }

impl CommonObject for pong {
type Args<'x> = ();

type Output<'x> = pong;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_pong(p)
}
}

/// expected size: 16
/// trait-ready: unique decoder function (d#3)
#[derive(Debug, Clone)]
pub struct pong {
tag: u8,
next: Box<ping>
}

/// d#0
fn Decoder0(input: &mut Parser<'_>) -> Result<(peano, ping), ParseError> {
PResult::Ok(((Decoder_peano(input))?, (Decoder_ping(input))?))
}

/// d#1
fn Decoder_peano(input: &mut Parser<'_>) -> Result<peano, ParseError> {
let tree_index = {
input.open_peek_context();
{
let ret = match input.read_byte()? {
90u8 => {
0
},

83u8 => {
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
if b == 90 {
b
} else {
return Err(ParseError::ExcludedBranch(13646096770106105413u64));
}
};
peano::Z(inner)
},

1 => {
let arg0 = {
let b = input.read_byte()?;
if b == 83 {
b
} else {
return Err(ParseError::ExcludedBranch(2206609067086327257u64));
}
};
let arg1 = Box::new((Decoder_peano(input))?);
peano::S(arg0, arg1)
},

_ => {
return Err(ParseError::ExcludedBranch(18270091135093349626u64));
}
})
}

/// d#2
fn Decoder_ping(input: &mut Parser<'_>) -> Result<ping, ParseError> {
let tree_index = {
input.open_peek_context();
{
let ret = match input.read_byte()? {
90u8 => {
0
},

65u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(18147521187885925800u64));
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
if b == 90 {
b
} else {
return Err(ParseError::ExcludedBranch(6185506036438099345u64));
}
};
ping::Done(inner)
},

1 => {
let arg0 = {
let b = input.read_byte()?;
if b == 65 {
b
} else {
return Err(ParseError::ExcludedBranch(15794382300316794652u64));
}
};
let arg1 = (Decoder_pong(input))?;
ping::More(arg0, arg1)
},

_ => {
return Err(ParseError::ExcludedBranch(7364705619221056123u64));
}
})
}

/// d#3
fn Decoder_pong(input: &mut Parser<'_>) -> Result<pong, ParseError> {
let tag = {
let b = input.read_byte()?;
if b == 66 {
b
} else {
return Err(ParseError::ExcludedBranch(2404222719611925354u64));
}
};
let next = (Decoder_ping(input))?;
PResult::Ok(pong { tag, next: Box::new(next) })
}

