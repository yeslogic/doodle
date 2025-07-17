#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt::skip)]

mod codegen_tests;
pub mod api_helper;

use doodle::prelude::*;
use doodle::try_sub;

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_axis_value_offsets<'input> {
axis_value_offsets: &'input [u8]
}

/// expected size: 32
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table<'input> {
major_version: u16,
minor_version: u16,
design_axis_size: u16,
design_axis_count: u16,
design_axes_array: &'input [u8],
axis_value_count: u16,
axis_value_offsets: opentype_stat_table_axis_value_offsets<'input>,
elided_fallback_name_id: u16
}

fn Decoder_opentype_stat_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_stat_table<'input>, ParseError> {
Decoder1(_input)
}

fn Decoder1<'input>(_input: &mut Parser<'input>) -> Result<opentype_stat_table<'input>, ParseError> {
let table_scope = _input.view();
let major_version = {
let inner = (Decoder2(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13646096770106105413u64));
}
};
let minor_version = {
let inner = (Decoder2(_input))?;
let is_valid = {
let x = inner;
matches!(x, 1u16 | 2u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2206609067086327257u64));
}
};
let design_axis_size = (Decoder2(_input))?;
let design_axis_count = (Decoder2(_input))?;
let _design_axes_offset = (Decoder3(_input))?;
let design_axes_array = {
let len = design_axis_size * design_axis_count;
match _design_axes_offset > 0u32 {
true => {
table_scope.offset(_design_axes_offset as usize).read_len(len as usize)
},

false => {
[].to_vec()
}
}
};
let axis_value_count = (Decoder2(_input))?;
let _offset_to_axis_value_offsets = (Decoder3(_input))?;
let axis_value_offsets = {
let mut tmp = Parser::from(table_scope.offset(_offset_to_axis_value_offsets as usize));
let view_parser = &mut tmp;
let axis_value_scope = view_parser.view();
let axis_value_offsets = axis_value_scope.read_len((2u16 * axis_value_count) as usize);
opentype_stat_table_axis_value_offsets { axis_value_offsets }
};
let elided_fallback_name_id = (Decoder2(_input))?;
PResult::Ok(opentype_stat_table { major_version, minor_version, design_axis_size, design_axis_count, design_axes_array, axis_value_count, axis_value_offsets, elided_fallback_name_id })
}

fn Decoder2(_input: &mut Parser<'_>) -> Result<u16, ParseError> {
let x = {
let field0 = (Decoder4(_input))?;
let field1 = (Decoder4(_input))?;
(field0, field1)
};
PResult::Ok(u16be(x))
}

fn Decoder3(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
let x = {
let field0 = (Decoder4(_input))?;
let field1 = (Decoder4(_input))?;
let field2 = (Decoder4(_input))?;
let field3 = (Decoder4(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(u32be(x))
}

fn Decoder4(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
_input.read_byte()
}

