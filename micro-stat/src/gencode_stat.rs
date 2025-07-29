#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt::skip)]

mod codegen_tests;
pub mod api_helper;

use doodle::prelude::*;
use doodle::try_sub;

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_stat_table_axis_value_offsets<'input> {
axis_value_offsets: ReadArray<'input, U16Be>
}

/// expected size: 64
#[derive(Debug, Clone)]
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

/// expected size: 32
#[derive(Debug, Clone)]
pub struct base_asciiz_string {
string: Vec<u8>,
null: u8
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_record {
axis_tag: u32,
axis_name_id: u16,
axis_ordering: u16
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_table_data_Format1_flags {
elidable_axis_value_name: bool,
older_sibling_font_attribute: bool
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub enum opentype_stat_axis_value_table_data_Format1_value { Fixed32(u32) }

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_table_data_Format1 {
axis_index: u16,
flags: opentype_stat_axis_value_table_data_Format1_flags,
value_name_id: u16,
value: opentype_stat_axis_value_table_data_Format1_value
}

/// expected size: 32
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_table_data_Format2 {
axis_index: u16,
flags: opentype_stat_axis_value_table_data_Format1_flags,
value_name_id: u16,
nominal_value: opentype_stat_axis_value_table_data_Format1_value,
range_min_value: opentype_stat_axis_value_table_data_Format1_value,
range_max_value: opentype_stat_axis_value_table_data_Format1_value
}

/// expected size: 24
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_table_data_Format3 {
axis_index: u16,
flags: opentype_stat_axis_value_table_data_Format1_flags,
value_name_id: u16,
value: opentype_stat_axis_value_table_data_Format1_value,
linked_value: opentype_stat_axis_value_table_data_Format1_value
}

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_table_data_Format4_axis_values {
axis_index: u16,
value: opentype_stat_axis_value_table_data_Format1_value
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_stat_axis_value_table_data_Format4 {
axis_count: u16,
flags: opentype_stat_axis_value_table_data_Format1_flags,
value_name_id: u16,
axis_values: Vec<opentype_stat_axis_value_table_data_Format4_axis_values>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum opentype_stat_axis_value_table_data { Format1(opentype_stat_axis_value_table_data_Format1), Format2(opentype_stat_axis_value_table_data_Format2), Format3(opentype_stat_axis_value_table_data_Format3), Format4(opentype_stat_axis_value_table_data_Format4) }

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_stat_axis_value_table {
format: u16,
data: opentype_stat_axis_value_table_data
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
table_scope.offset(_design_axes_offset as usize).read_len(len as usize)
};
let axis_value_count = (Decoder2(_input))?;
let _offset_to_axis_value_offsets = (Decoder3(_input))?;
let axis_value_offsets = {
let mut view_parser = Parser::from(table_scope.offset(_offset_to_axis_value_offsets as usize));
let view_input = &mut view_parser;
let axis_value_scope = view_input.view();
let axis_value_offsets = axis_value_scope.read_array_u16be(axis_value_count as usize)?;
opentype_stat_table_axis_value_offsets { axis_value_offsets }
};
let elided_fallback_name_id = (Decoder2(_input))?;
PResult::Ok(opentype_stat_table { major_version, minor_version, design_axis_size, design_axis_count, design_axes_array, axis_value_count, axis_value_offsets, elided_fallback_name_id })
}

fn Decoder2(_input: &mut Parser<'_>) -> Result<u16, ParseError> {
let x = ((Decoder4(_input))?, (Decoder4(_input))?);
PResult::Ok(u16be(x))
}

fn Decoder3(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
let x = ((Decoder4(_input))?, (Decoder4(_input))?, (Decoder4(_input))?, (Decoder4(_input))?);
PResult::Ok(u32be(x))
}

fn Decoder4(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
_input.read_byte()
}

fn Decoder_base_asciiz_string(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if (byte != 0) => {
0
},

0u8 => {
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
if b != 0 {
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
let null = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(6185506036438099345u64));
}
};
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_opentype_stat_axis_record(_input: &mut Parser<'_>) -> Result<opentype_stat_axis_record, ParseError> {
let axis_tag = (Decoder7(_input))?;
let axis_name_id = (Decoder2(_input))?;
let axis_ordering = (Decoder2(_input))?;
PResult::Ok(opentype_stat_axis_record { axis_tag, axis_name_id, axis_ordering })
}

fn Decoder7(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
Decoder3(_input)
}

fn Decoder_opentype_stat_axis_value_table(_input: &mut Parser<'_>) -> Result<opentype_stat_axis_value_table, ParseError> {
let format = {
let inner = (Decoder2(_input))?;
let is_valid = {
let x = inner;
matches!(x, 1u16..=4u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(15794382300316794652u64));
}
};
let data = match format {
1u16 => {
let inner = {
let axis_index = (Decoder2(_input))?;
let flags = {
let packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder2(_input))?;
let value = {
let x = (Decoder3(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
opentype_stat_axis_value_table_data_Format1 { axis_index, flags, value_name_id, value }
};
opentype_stat_axis_value_table_data::Format1(inner)
},

2u16 => {
let inner = {
let axis_index = (Decoder2(_input))?;
let flags = {
let packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder2(_input))?;
let nominal_value = {
let x = (Decoder3(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
let range_min_value = {
let x = (Decoder3(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
let range_max_value = {
let x = (Decoder3(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
opentype_stat_axis_value_table_data_Format2 { axis_index, flags, value_name_id, nominal_value, range_min_value, range_max_value }
};
opentype_stat_axis_value_table_data::Format2(inner)
},

3u16 => {
let inner = {
let axis_index = (Decoder2(_input))?;
let flags = {
let packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder2(_input))?;
let value = {
let x = (Decoder3(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
let linked_value = {
let x = (Decoder3(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
opentype_stat_axis_value_table_data_Format3 { axis_index, flags, value_name_id, value, linked_value }
};
opentype_stat_axis_value_table_data::Format3(inner)
},

4u16 => {
let inner = {
let axis_count = (Decoder2(_input))?;
let flags = {
let packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder2(_input))?;
let axis_values = {
let mut accum = Vec::new();
for _ in 0..axis_count {
let next_elem = {
let axis_index = (Decoder2(_input))?;
let value = {
let x = (Decoder3(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
opentype_stat_axis_value_table_data_Format4_axis_values { axis_index, value }
};
accum.push(next_elem)
};
accum
};
opentype_stat_axis_value_table_data_Format4 { axis_count, flags, value_name_id, axis_values }
};
opentype_stat_axis_value_table_data::Format4(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(opentype_stat_axis_value_table { format, data })
}
