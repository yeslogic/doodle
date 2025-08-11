#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt::skip)]

mod codegen_tests;
pub mod api_helper;

use doodle::prelude::*;
use doodle::try_sub;

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub struct opentype_table_record {
table_id: u32,
checksum: u32,
offset: u32,
length: u32
}

/// expected size: 64
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_axis_value_offsets<'input> {
axis_value_view: View<'input>,
axis_value_offsets: ReadArray<'input, U16Be>
}

/// expected size: 88
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

/// expected size: 88
#[derive(Debug, Copy, Clone)]
pub struct opentype_table_directory_table_links<'input> {
stat: Option<opentype_stat_table<'input>>
}

/// expected size: 128
#[derive(Debug, Clone)]
pub struct opentype_table_directory<'input> {
sfnt_version: u32,
num_tables: u16,
search_range: u16,
entry_selector: u16,
range_shift: u16,
table_records: Vec<opentype_table_record>,
table_links: opentype_table_directory_table_links<'input>
}

/// expected size: 136
#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version1_table_directories<'input> {
offset: u32,
link: Option<opentype_table_directory<'input>>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version1<'input> {
num_fonts: u32,
table_directories: Vec<opentype_ttc_header_header_Version1_table_directories<'input>>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version2<'input> {
num_fonts: u32,
table_directories: Vec<opentype_ttc_header_header_Version1_table_directories<'input>>,
dsig_tag: u32,
dsig_length: u32,
dsig_offset: u32
}

/// expected size: 48
#[derive(Debug, Clone)]
pub enum opentype_ttc_header_header<'input> { UnknownVersion(u16), Version1(opentype_ttc_header_header_Version1<'input>), Version2(opentype_ttc_header_header_Version2<'input>) }

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_ttc_header<'input> {
ttc_tag: u32,
major_version: u16,
minor_version: u16,
header: opentype_ttc_header_header<'input>
}

/// expected size: 136
#[derive(Debug, Clone)]
pub enum opentype_main_directory<'input> { TTCHeader(opentype_ttc_header<'input>), TableDirectory(opentype_table_directory<'input>) }

/// expected size: 144
#[derive(Debug, Clone)]
pub struct opentype_main<'input> {
file_start: u32,
magic: u32,
directory: opentype_main_directory<'input>
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

fn Decoder_opentype_main<'input>(_input: &mut Parser<'input>) -> Result<opentype_main<'input>, ParseError> {
Decoder1(_input)
}

fn Decoder1<'input>(_input: &mut Parser<'input>) -> Result<opentype_main<'input>, ParseError> {
let file_start = {
let x = _input.get_offset_u64();
x as u32
};
let magic = {
_input.open_peek_context();
let ret = (Decoder2(_input))?;
_input.close_peek_context()?;
ret
};
let directory = match magic {
65536u32 => {
let inner = (Decoder_opentype_table_directory(_input, file_start))?;
opentype_main_directory::TableDirectory(inner)
},

1330926671u32 => {
let inner = (Decoder_opentype_table_directory(_input, file_start))?;
opentype_main_directory::TableDirectory(inner)
},

1953784678u32 => {
let inner = (Decoder_opentype_ttc_header(_input, file_start))?;
opentype_main_directory::TTCHeader(inner)
},

1953658213u32 => {
let inner = (Decoder_opentype_table_directory(_input, file_start))?;
opentype_main_directory::TableDirectory(inner)
},

_ => {
return Err(ParseError::FailToken(13646096770106105413u64));
}
};
PResult::Ok(opentype_main { file_start, magic, directory })
}

fn Decoder2(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
let x = ((Decoder6(_input))?, (Decoder6(_input))?, (Decoder6(_input))?, (Decoder6(_input))?);
PResult::Ok(u32be(x))
}

fn Decoder_opentype_table_directory<'input>(_input: &mut Parser<'input>, font_start: u32) -> Result<opentype_table_directory<'input>, ParseError> {
let sfnt_version = {
let inner = (Decoder2(_input))?;
let is_valid = {
let version = inner;
matches!(version, 65536u32 | 1330926671u32 | 1953658213u32)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2206609067086327257u64));
}
};
let num_tables = (Decoder5(_input))?;
let search_range = (Decoder5(_input))?;
let entry_selector = (Decoder5(_input))?;
let range_shift = (Decoder5(_input))?;
let table_records = {
let mut accum = Vec::new();
for _ in 0..num_tables {
let next_elem = (Decoder_opentype_table_record(_input))?;
accum.push(next_elem)
};
accum
};
let table_links = (Decoder_opentype_table_directory_table_links(_input, font_start, &table_records))?;
PResult::Ok(opentype_table_directory { sfnt_version, num_tables, search_range, entry_selector, range_shift, table_records, table_links })
}

fn Decoder_opentype_ttc_header<'input>(_input: &mut Parser<'input>, start: u32) -> Result<opentype_ttc_header<'input>, ParseError> {
let ttc_tag = {
let inner = (Decoder2(_input))?;
let is_valid = {
let tag = inner;
tag == 1953784678u32
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11876854719037224982u64));
}
};
let major_version = (Decoder5(_input))?;
let minor_version = (Decoder5(_input))?;
let header = match major_version {
1u16 => {
let inner = {
let num_fonts = (Decoder2(_input))?;
let table_directories = {
let mut accum = Vec::new();
for _ in 0..num_fonts {
let next_elem = {
let offset = (Decoder2(_input))?;
let link = match offset > 0u32 {
true => {
let tgt_offset = start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = Some((Decoder_opentype_table_directory(_input, start))?);
_input.close_peek_context()?;
ret
},

false => {
None
}
};
opentype_ttc_header_header_Version1_table_directories { offset, link }
};
accum.push(next_elem)
};
accum
};
opentype_ttc_header_header_Version1 { num_fonts, table_directories }
};
opentype_ttc_header_header::Version1(inner)
},

2u16 => {
let inner = {
let num_fonts = (Decoder2(_input))?;
let table_directories = {
let mut accum = Vec::new();
for _ in 0..num_fonts {
let next_elem = {
let offset = (Decoder2(_input))?;
let link = match offset > 0u32 {
true => {
let tgt_offset = start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = Some((Decoder_opentype_table_directory(_input, start))?);
_input.close_peek_context()?;
ret
},

false => {
None
}
};
opentype_ttc_header_header_Version1_table_directories { offset, link }
};
accum.push(next_elem)
};
accum
};
let dsig_tag = (Decoder2(_input))?;
let dsig_length = (Decoder2(_input))?;
let dsig_offset = (Decoder2(_input))?;
opentype_ttc_header_header_Version2 { num_fonts, table_directories, dsig_tag, dsig_length, dsig_offset }
};
opentype_ttc_header_header::Version2(inner)
},

unknown => {
let inner = unknown;
opentype_ttc_header_header::UnknownVersion(inner)
}
};
_input.skip_remainder();
PResult::Ok(opentype_ttc_header { ttc_tag, major_version, minor_version, header })
}

fn Decoder5(_input: &mut Parser<'_>) -> Result<u16, ParseError> {
let x = ((Decoder6(_input))?, (Decoder6(_input))?);
PResult::Ok(u16be(x))
}

fn Decoder6(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
_input.read_byte()
}

fn Decoder_opentype_table_record(_input: &mut Parser<'_>) -> Result<opentype_table_record, ParseError> {
let table_id = (Decoder10(_input))?;
let checksum = (Decoder2(_input))?;
let offset = (Decoder2(_input))?;
let length = (Decoder2(_input))?;
PResult::Ok(opentype_table_record { table_id, checksum, offset, length })
}

fn Decoder_opentype_table_directory_table_links<'input>(_input: &mut Parser<'input>, start: u32, tables: &[opentype_table_record]) -> Result<opentype_table_directory_table_links<'input>, ParseError> {
let stat = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1398030676u32, tables)).copied() {
Some(ref table) => {
let tgt_offset = start + table.offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = table.length as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_stat_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
Some(ret)
},

None => {
None
}
};
_input.skip_remainder();
PResult::Ok(opentype_table_directory_table_links { stat })
}

fn Decoder_opentype_stat_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_stat_table<'input>, ParseError> {
let table_scope = _input.view();
let major_version = {
let inner = (Decoder5(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(18270091135093349626u64));
}
};
let minor_version = {
let inner = (Decoder5(_input))?;
let is_valid = {
let x = inner;
matches!(x, 1u16 | 2u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6185506036438099345u64));
}
};
let design_axis_size = (Decoder5(_input))?;
let design_axis_count = (Decoder5(_input))?;
let _design_axes_offset = (Decoder2(_input))?;
let design_axes_array = {
let len = design_axis_size * design_axis_count;
table_scope.offset(_design_axes_offset as usize)?.read_len(len as usize)
};
let axis_value_count = (Decoder5(_input))?;
let _offset_to_axis_value_offsets = (Decoder2(_input))?;
let axis_value_offsets = {
let mut view_parser = Parser::from(table_scope.offset(_offset_to_axis_value_offsets as usize)?);
let view_input = &mut view_parser;
let axis_value_scope = view_input.view();
let axis_value_view = axis_value_scope;
let axis_value_offsets = axis_value_scope.read_array_u16be(axis_value_count as usize)?;
opentype_stat_table_axis_value_offsets { axis_value_view, axis_value_offsets }
};
let elided_fallback_name_id = (Decoder5(_input))?;
PResult::Ok(opentype_stat_table { major_version, minor_version, design_axis_size, design_axis_count, design_axes_array, axis_value_count, axis_value_offsets, elided_fallback_name_id })
}

fn Decoder10(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
Decoder2(_input)
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
return Err(ParseError::ExcludedBranch(15794382300316794652u64));
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
return Err(ParseError::ExcludedBranch(18147521187885925800u64));
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
return Err(ParseError::ExcludedBranch(7364705619221056123u64));
}
};
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_opentype_stat_axis_record(_input: &mut Parser<'_>) -> Result<opentype_stat_axis_record, ParseError> {
let axis_tag = (Decoder10(_input))?;
let axis_name_id = (Decoder5(_input))?;
let axis_ordering = (Decoder5(_input))?;
PResult::Ok(opentype_stat_axis_record { axis_tag, axis_name_id, axis_ordering })
}

fn Decoder_opentype_stat_axis_value_table(_input: &mut Parser<'_>) -> Result<opentype_stat_axis_value_table, ParseError> {
let format = {
let inner = (Decoder5(_input))?;
let is_valid = {
let x = inner;
matches!(x, 1u16..=4u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2404222719611925354u64));
}
};
let data = match format {
1u16 => {
let inner = {
let axis_index = (Decoder5(_input))?;
let flags = {
let packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder5(_input))?;
let value = {
let x = (Decoder2(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
opentype_stat_axis_value_table_data_Format1 { axis_index, flags, value_name_id, value }
};
opentype_stat_axis_value_table_data::Format1(inner)
},

2u16 => {
let inner = {
let axis_index = (Decoder5(_input))?;
let flags = {
let packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder5(_input))?;
let nominal_value = {
let x = (Decoder2(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
let range_min_value = {
let x = (Decoder2(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
let range_max_value = {
let x = (Decoder2(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
opentype_stat_axis_value_table_data_Format2 { axis_index, flags, value_name_id, nominal_value, range_min_value, range_max_value }
};
opentype_stat_axis_value_table_data::Format2(inner)
},

3u16 => {
let inner = {
let axis_index = (Decoder5(_input))?;
let flags = {
let packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder5(_input))?;
let value = {
let x = (Decoder2(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
let linked_value = {
let x = (Decoder2(_input))?;
opentype_stat_axis_value_table_data_Format1_value::Fixed32(x)
};
opentype_stat_axis_value_table_data_Format3 { axis_index, flags, value_name_id, value, linked_value }
};
opentype_stat_axis_value_table_data::Format3(inner)
},

4u16 => {
let inner = {
let axis_count = (Decoder5(_input))?;
let flags = {
let packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder5(_input))?;
let axis_values = {
let mut accum = Vec::new();
for _ in 0..axis_count {
let next_elem = {
let axis_index = (Decoder5(_input))?;
let value = {
let x = (Decoder2(_input))?;
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

