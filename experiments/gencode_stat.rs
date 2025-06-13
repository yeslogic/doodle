#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
// #![cfg_attr(rustfmt, rustfmt::skip)]

pub mod api_helper;
mod codegen_tests;

use doodle::prelude::*;
use doodle::try_sub;

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_design_axes_offset_link_design_axes {
    axis_tag: u32,
    axis_name_id: u16,
    axis_ordering: u16,
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct opentype_stat_table_design_axes_offset_link {
    design_axes: Vec<opentype_stat_table_design_axes_offset_link_design_axes>,
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_stat_table_design_axes_offset {
    offset: u32,
    link: Option<opentype_stat_table_design_axes_offset_link>,
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags
{
    elidable_axis_value_name: bool,
    older_sibling_font_attribute: bool,
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub enum opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value
{
    Fixed32(u32),
}

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1 {
axis_index: u16,
flags: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags,
value_name_id: u16,
value: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value
}

/// expected size: 32
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format2 {
axis_index: u16,
flags: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags,
value_name_id: u16,
nominal_value: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value,
range_min_value: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value,
range_max_value: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value
}

/// expected size: 24
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format3 {
axis_index: u16,
flags: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags,
value_name_id: u16,
value: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value,
linked_value: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value
}

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format4_axis_values {
axis_index: u16,
value: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format4 {
axis_count: u16,
flags: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags,
value_name_id: u16,
axis_values: Vec<opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format4_axis_values>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data {
    Format1(
        opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1,
    ),
    Format2(
        opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format2,
    ),
    Format3(
        opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format3,
    ),
    Format4(
        opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format4,
    ),
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link {
    format: u16,
    data: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data,
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets {
    offset: u16,
    link: Option<opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link>,
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link {
    table_start: u32,
    axis_value_offsets:
        Vec<opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets>,
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets {
    offset: u32,
    link: Option<opentype_stat_table_offset_to_axis_value_offsets_link>,
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_stat_table {
    table_start: u32,
    major_version: u16,
    minor_version: u16,
    design_axis_size: u16,
    design_axis_count: u16,
    design_axes_offset: opentype_stat_table_design_axes_offset,
    axis_value_count: u16,
    offset_to_axis_value_offsets: opentype_stat_table_offset_to_axis_value_offsets,
    elided_fallback_name_id: u16,
}

fn Decoder_opentype_stat_table(_input: &mut Parser<'_>) -> Result<opentype_stat_table, ParseError> {
    Decoder1(_input)
}

fn Decoder1(_input: &mut Parser<'_>) -> Result<opentype_stat_table, ParseError> {
    let table_start = {
        let x = _input.get_offset_u64();
        x as u32
    };
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
    let design_axes_offset = {
        let offset = (Decoder3(_input))?;
        let link =
            match offset > 0u32 {
                true => {
                    let tgt_offset = table_start + offset;
                    let _is_advance = _input.advance_or_seek(tgt_offset)?;
                    let ret =
                        ((|| {
                            let design_axes =
                                {
                                    let mut accum = Vec::new();
                                    for _ in 0..design_axis_count {
                                        accum.push({
let axis_tag = (Decoder4(_input))?;
let axis_name_id = (Decoder2(_input))?;
let axis_ordering = (Decoder2(_input))?;
opentype_stat_table_design_axes_offset_link_design_axes { axis_tag, axis_name_id, axis_ordering }
});
                                    }
                                    accum
                                };
                            PResult::Ok(Some(opentype_stat_table_design_axes_offset_link {
                                design_axes,
                            }))
                        })())?;
                    _input.close_peek_context()?;
                    ret
                }

                false => None,
            };
        opentype_stat_table_design_axes_offset { offset, link }
    };
    let axis_value_count = (Decoder2(_input))?;
    let offset_to_axis_value_offsets = {
        let offset = (Decoder3(_input))?;
        let link = match offset > 0u32 {
            true => {
                let tgt_offset = table_start + offset;
                let _is_advance = _input.advance_or_seek(tgt_offset)?;
                let ret = ((|| {
                    let table_start = {
                        let x = _input.get_offset_u64();
                        x as u32
                    };
                    let axis_value_offsets = {
                        let mut accum = Vec::new();
                        for _ in 0..axis_value_count {
                            accum.push({
let offset = (Decoder2(_input))?;
let link = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
if table_start + (offset as u32) >= __here {
let tgt_offset = table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let format = {
let inner = (Decoder2(_input))?;
let is_valid = {
let x = inner;
matches!(x, 1u16..=4u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11876854719037224982u64));
}
};
let data = match format {
1u16 => {
let inner = {
let axis_index = (Decoder2(_input))?;
let flags = {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder2(_input))?;
let value = {
let x = (Decoder3(_input))?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value::Fixed32(x)
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1 { axis_index, flags, value_name_id, value }
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data::Format1(inner)
},

2u16 => {
let inner = {
let axis_index = (Decoder2(_input))?;
let flags = {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder2(_input))?;
let nominal_value = {
let x = (Decoder3(_input))?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value::Fixed32(x)
};
let range_min_value = {
let x = (Decoder3(_input))?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value::Fixed32(x)
};
let range_max_value = {
let x = (Decoder3(_input))?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value::Fixed32(x)
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format2 { axis_index, flags, value_name_id, nominal_value, range_min_value, range_max_value }
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data::Format2(inner)
},

3u16 => {
let inner = {
let axis_index = (Decoder2(_input))?;
let flags = {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder2(_input))?;
let value = {
let x = (Decoder3(_input))?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value::Fixed32(x)
};
let linked_value = {
let x = (Decoder3(_input))?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value::Fixed32(x)
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format3 { axis_index, flags, value_name_id, value, linked_value }
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data::Format3(inner)
},

4u16 => {
let inner = {
let axis_count = (Decoder2(_input))?;
let flags = {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits & 1u16 > 0u16 }
};
let value_name_id = (Decoder2(_input))?;
let axis_values = {
let mut accum = Vec::new();
for _ in 0..axis_count {
accum.push({
let axis_index = (Decoder2(_input))?;
let value = {
let x = (Decoder3(_input))?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_value::Fixed32(x)
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format4_axis_values { axis_index, value }
});
}
accum
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format4 { axis_count, flags, value_name_id, axis_values }
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data::Format4(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link { format, data })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
}
},

false => {
None
}
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets { offset, link }
});
                        }
                        accum
                    };
                    PResult::Ok(Some(
                        opentype_stat_table_offset_to_axis_value_offsets_link {
                            table_start,
                            axis_value_offsets,
                        },
                    ))
                })())?;
                _input.close_peek_context()?;
                ret
            }

            false => None,
        };
        opentype_stat_table_offset_to_axis_value_offsets { offset, link }
    };
    let elided_fallback_name_id = (Decoder2(_input))?;
    PResult::Ok(opentype_stat_table {
        table_start,
        major_version,
        minor_version,
        design_axis_size,
        design_axis_count,
        design_axes_offset,
        axis_value_count,
        offset_to_axis_value_offsets,
        elided_fallback_name_id,
    })
}

fn Decoder2(_input: &mut Parser<'_>) -> Result<u16, ParseError> {
    let x = {
        let field0 = (Decoder5(_input))?;
        let field1 = (Decoder5(_input))?;
        (field0, field1)
    };
    PResult::Ok(u16be(x))
}

fn Decoder3(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
    let x = {
        let field0 = (Decoder5(_input))?;
        let field1 = (Decoder5(_input))?;
        let field2 = (Decoder5(_input))?;
        let field3 = (Decoder5(_input))?;
        (field0, field1, field2, field3)
    };
    PResult::Ok(u32be(x))
}

fn Decoder4(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
    Decoder3(_input)
}

fn Decoder5(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
    _input.read_byte()
}
