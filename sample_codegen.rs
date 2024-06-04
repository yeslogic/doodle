#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod codegen_tests;

use doodle::prelude::*;

#[derive(Debug, Clone)]
struct main_data {
signature: (u8, u8, u8),
version: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
screen_width: u16,
screen_height: u16,
flags: u8,
bg_color_index: u8,
pixel_aspect_ratio: u8
}

#[derive(Debug, Clone)]
struct main_data {
r: u8,
g: u8,
b: u8
}

#[derive(Debug, Clone)]
enum main_data { no, yes(Vec<main_data>) }

#[derive(Debug, Clone)]
struct main_data {
descriptor: main_data,
global_color_table: main_data
}

#[derive(Debug, Clone)]
struct main_data {
separator: u8,
label: u8,
block_size: u8,
flags: u8,
delay_time: u16,
transparent_color_index: u8,
terminator: u8
}

#[derive(Debug, Clone)]
enum main_data { none, some(main_data) }

#[derive(Debug, Clone)]
struct main_data {
len_bytes: u8,
data: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
separator: u8,
label: u8,
block_size: u8,
text_grid_left_position: u16,
text_grid_top_position: u16,
text_grid_width: u16,
text_grid_height: u16,
character_cell_width: u8,
character_cell_height: u8,
text_foreground_color_index: u8,
text_background_color_index: u8,
plain_text_data: Vec<main_data>,
terminator: u8
}

#[derive(Debug, Clone)]
struct main_data {
separator: u8,
image_left_position: u16,
image_top_position: u16,
image_width: u16,
image_height: u16,
flags: u8
}

#[derive(Debug, Clone)]
struct main_data {
lzw_min_code_size: u8,
image_data: Vec<main_data>,
terminator: u8
}

#[derive(Debug, Clone)]
struct main_data {
descriptor: main_data,
local_color_table: main_data,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { plain_text_extension(main_data), table_based_image(main_data) }

#[derive(Debug, Clone)]
struct main_data {
graphic_control_extension: main_data,
graphic_rendering_block: main_data
}

#[derive(Debug, Clone)]
struct main_data {
separator: u8,
label: u8,
block_size: u8,
identifier: Vec<u8>,
authentication_code: Vec<u8>,
application_data: Vec<main_data>,
terminator: u8
}

#[derive(Debug, Clone)]
struct main_data {
separator: u8,
label: u8,
comment_data: Vec<main_data>,
terminator: u8
}

#[derive(Debug, Clone)]
enum main_data { application_extension(main_data), comment_extension(main_data) }

#[derive(Debug, Clone)]
enum main_data { graphic_block(main_data), special_purpose_block(main_data) }

#[derive(Debug, Clone)]
struct main_data {
separator: u8
}

#[derive(Debug, Clone)]
struct main_data {
header: main_data,
logical_screen: main_data,
blocks: Vec<main_data>,
trailer: main_data
}

#[derive(Debug, Clone)]
struct main_data {
magic: (u8, u8),
method: u8,
file_flags: u8,
timestamp: u32,
compression_flags: u8,
os_id: u8
}

#[derive(Debug, Clone)]
struct main_data {
string: Vec<u8>,
null: u8
}

#[derive(Debug, Clone)]
enum main_data { no, yes(main_data) }

#[derive(Debug, Clone)]
struct main_data {
code: u16,
extra: u8
}

#[derive(Debug, Clone)]
struct main_data {
distance_extra_bits: u16,
distance: u16
}

#[derive(Debug, Clone)]
struct main_data {
length_extra_bits: u8,
length: u16,
distance_code: u16,
distance_record: main_data
}

#[derive(Debug, Clone)]
enum main_data { none, some(main_data) }

#[derive(Debug, Clone)]
struct main_data {
code: u16,
extra: main_data
}

#[derive(Debug, Clone)]
struct main_data {
length: u16,
distance: u16
}

#[derive(Debug, Clone)]
enum main_data { literal(u8), reference(main_data) }

#[derive(Debug, Clone)]
struct main_data {
hlit: u8,
hdist: u8,
hclen: u8,
code_length_alphabet_code_lengths: Vec<u8>,
literal_length_distance_alphabet_code_lengths: Vec<main_data>,
literal_length_distance_alphabet_code_lengths_value: Vec<u8>,
literal_length_alphabet_code_lengths_value: Vec<u8>,
distance_alphabet_code_lengths_value: Vec<u8>,
codes: Vec<main_data>,
codes_values: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
length_extra_bits: u8,
length: u16,
distance_code: u8,
distance_record: main_data
}

#[derive(Debug, Clone)]
enum main_data { none, some(main_data) }

#[derive(Debug, Clone)]
struct main_data {
code: u16,
extra: main_data
}

#[derive(Debug, Clone)]
struct main_data {
codes: Vec<main_data>,
codes_values: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
align: (),
len: u16,
nlen: u16,
bytes: Vec<u8>,
codes_values: Vec<main_data>
}

#[derive(Debug, Clone)]
enum main_data { dynamic_huffman(main_data), fixed_huffman(main_data), uncompressed(main_data) }

#[derive(Debug, Clone)]
struct main_data {
r#final: u8,
r#type: u8,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
blocks: Vec<main_data>,
codes: Vec<main_data>,
inflate: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
crc: u32,
length: u32
}

#[derive(Debug, Clone)]
struct main_data {
header: main_data,
fname: main_data,
data: main_data,
footer: main_data
}

#[derive(Debug, Clone)]
struct main_data {
ff: u8,
marker: u8
}

#[derive(Debug, Clone)]
struct main_data {
version_major: u8,
version_minor: u8,
density_units: u8,
density_x: u16,
density_y: u16,
thumbnail_width: u8,
thumbnail_height: u8,
thumbnail_pixels: Vec<Vec<main_data>>
}

#[derive(Debug, Clone)]
enum main_data { jfif(main_data), other(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
identifier: main_data,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
marker: main_data,
length: u16,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { be(u8, u8), le(u8, u8) }

#[derive(Debug, Clone)]
struct main_data {
tag: u16,
r#type: u16,
length: u32,
offset_or_data: u32
}

#[derive(Debug, Clone)]
struct main_data {
num_fields: u16,
fields: Vec<main_data>,
next_ifd_offset: u32,
next_ifd: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
byte_order: main_data,
magic: u16,
offset: u32,
ifd: main_data
}

#[derive(Debug, Clone)]
struct main_data {
padding: u8,
exif: main_data
}

#[derive(Debug, Clone)]
struct main_data {
xmp: Vec<u8>
}

#[derive(Debug, Clone)]
enum main_data { exif(main_data), other(Vec<u8>), xmp(main_data) }

#[derive(Debug, Clone)]
struct main_data {
identifier: main_data,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
marker: main_data,
length: u16,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { app0(main_data), app1(main_data) }

#[derive(Debug, Clone)]
struct main_data {
marker: main_data,
length: u16,
data: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
class_table_id: u8,
value: u8
}

#[derive(Debug, Clone)]
struct main_data {
marker: main_data,
length: u16,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
class_table_id: u8,
num_codes: Vec<u8>,
values: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
marker: main_data,
length: u16,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
precision_table_id: u8,
elements: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
marker: main_data,
length: u16,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
restart_interval: u16
}

#[derive(Debug, Clone)]
struct main_data {
marker: main_data,
length: u16,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { app0(main_data), app1(main_data), app10(main_data), app11(main_data), app12(main_data), app13(main_data), app14(main_data), app15(main_data), app2(main_data), app3(main_data), app4(main_data), app5(main_data), app6(main_data), app7(main_data), app8(main_data), app9(main_data), com(main_data), dac(main_data), dht(main_data), dqt(main_data), dri(main_data) }

#[derive(Debug, Clone)]
struct main_data {
id: u8,
sampling_factor: u8,
quantization_table_id: u8
}

#[derive(Debug, Clone)]
struct main_data {
sample_precision: u8,
num_lines: u16,
num_samples_per_line: u16,
num_image_components: u8,
image_components: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
marker: main_data,
length: u16,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { sof0(main_data), sof1(main_data), sof10(main_data), sof11(main_data), sof13(main_data), sof14(main_data), sof15(main_data), sof2(main_data), sof3(main_data), sof5(main_data), sof6(main_data), sof7(main_data), sof9(main_data) }

#[derive(Debug, Clone)]
struct main_data {
component_selector: u8,
entropy_coding_table_ids: u8
}

#[derive(Debug, Clone)]
struct main_data {
num_image_components: u8,
image_components: Vec<main_data>,
start_spectral_selection: u8,
end_spectral_selection: u8,
approximation_bit_position: u8
}

#[derive(Debug, Clone)]
struct main_data {
marker: main_data,
length: u16,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { mcu(u8), rst0(main_data), rst1(main_data), rst2(main_data), rst3(main_data), rst4(main_data), rst5(main_data), rst6(main_data), rst7(main_data) }

#[derive(Debug, Clone)]
struct main_data {
scan_data: Vec<main_data>,
scan_data_stream: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
segments: Vec<main_data>,
sos: main_data,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
num_lines: u16
}

#[derive(Debug, Clone)]
struct main_data {
marker: main_data,
length: u16,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { none, some(main_data) }

#[derive(Debug, Clone)]
struct main_data {
initial_segment: main_data,
segments: Vec<main_data>,
header: main_data,
scan: main_data,
dnl: main_data,
scans: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
soi: main_data,
frame: main_data,
eoi: main_data
}

#[derive(Debug, Clone)]
struct main_data {
major_brand: (u8, u8, u8, u8),
minor_version: u32,
compatible_brands: Vec<(u8, u8, u8, u8)>
}

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
number_of_entries: u32,
data: Vec<main_data>
}

#[derive(Debug, Clone)]
enum main_data { dref(main_data), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
predefined: u32,
handler_type: (u8, u8, u8, u8),
reserved: (u32, u32, u32),
name: main_data
}

#[derive(Debug, Clone)]
struct main_data {
content_type: main_data
}

#[derive(Debug, Clone)]
struct main_data {
item_uri_type: main_data
}

#[derive(Debug, Clone)]
enum main_data { mime(main_data), unknown, uri(main_data) }

#[derive(Debug, Clone)]
struct main_data {
item_ID: u32,
item_protection_index: u16,
item_type: (u8, u8, u8, u8),
item_name: main_data,
extra_fields: main_data
}

#[derive(Debug, Clone)]
struct main_data {
item_ID: u16,
item_protection_index: u16,
item_name: main_data,
content_type: main_data,
content_encoding: main_data
}

#[derive(Debug, Clone)]
enum main_data { no(main_data), yes(main_data) }

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
fields: main_data
}

#[derive(Debug, Clone)]
enum main_data { infe(main_data), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
item_info_entry: Vec<main_data>
}

#[derive(Debug, Clone)]
enum main_data { no, yes(u16) }

#[derive(Debug, Clone)]
struct main_data {
extent_index: u64,
extent_offset: u64,
extent_length: u64
}

#[derive(Debug, Clone)]
struct main_data {
item_ID: u32,
construction_method: main_data,
data_reference_index: u16,
base_offset: u64,
extent_count: u16,
extents: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
offset_size_length_size: u8,
base_offset_size_index_size: u8,
offset_size: u8,
length_size: u8,
base_offset_size: u8,
index_size: u8,
item_count: u32,
items: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
type_indicator: u32,
locale_indicator: u32,
value: Vec<u8>
}

#[derive(Debug, Clone)]
enum main_data { data(main_data), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { tool(Vec<main_data>), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
from_item_ID: u32,
reference_count: u16,
to_item_ID: Vec<u32>
}

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
from_item_ID: u16,
reference_count: u16,
to_item_ID: Vec<u16>
}

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { large(Vec<main_data>), small(Vec<main_data>) }

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
single_item_reference: main_data
}

#[derive(Debug, Clone)]
enum main_data { no(u32), yes(u16) }

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
item_ID: main_data
}

#[derive(Debug, Clone)]
enum main_data { dinf(Vec<main_data>), hdlr(main_data), idat(Vec<u8>), iinf(main_data), iloc(main_data), ilst(Vec<main_data>), iref(main_data), pitm(main_data), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
creation_time: u32,
modification_time: u32,
timescale: u32,
duration: u32
}

#[derive(Debug, Clone)]
struct main_data {
creation_time: u64,
modification_time: u64,
timescale: u32,
duration: u64
}

#[derive(Debug, Clone)]
enum main_data { version0(main_data), version1(main_data) }

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
fields: main_data,
rate: u32,
volume: u16,
reserved1: u16,
reserved2: (u32, u32),
matrix: Vec<u32>,
pre_defined: Vec<u32>,
next_track_ID: u32
}

#[derive(Debug, Clone)]
struct main_data {
track_duration: u32,
media_time: u32,
media_rate: u32
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
number_of_entries: u32,
edit_list_table: Vec<main_data>
}

#[derive(Debug, Clone)]
enum main_data { elst(main_data), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
component_type: u32,
component_subtype: (u8, u8, u8, u8),
component_manufacturer: u32,
component_flags: u32,
component_flags_mask: u32,
component_name: main_data
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
fields: main_data,
language: u16,
pre_defined: u16
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
balance: u16,
reserved: u16
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_offset: Vec<u64>
}

#[derive(Debug, Clone)]
struct main_data {
sample_count: u32,
sample_offset: u32
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<main_data>
}

#[derive(Debug, Clone)]
enum main_data { no, yes(u32) }

#[derive(Debug, Clone)]
struct main_data {
sample_count: u32,
group_description_index: u32
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
grouping_type: u32,
grouping_type_parameter: main_data,
entry_count: u32,
sample_groups: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
description_length: u32,
sample_group_entry: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
grouping_type: u32,
default_length: u32,
entry_count: u32,
sample_groups: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_offset: Vec<u32>
}

#[derive(Debug, Clone)]
struct main_data {
first_chunk: u32,
samples_per_chunk: u32,
sample_description_index: u32
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_entries: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_number: Vec<u32>
}

#[derive(Debug, Clone)]
enum main_data { no, yes(Vec<u32>) }

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
sample_size: u32,
sample_count: u32,
entry_size: main_data
}

#[derive(Debug, Clone)]
struct main_data {
sample_count: u32,
sample_delta: u32
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<main_data>
}

#[derive(Debug, Clone)]
enum main_data { co64(main_data), ctts(main_data), sbgp(main_data), sgpd(main_data), stco(main_data), stsc(main_data), stsd(main_data), stss(main_data), stsz(main_data), stts(main_data), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
graphicsmode: u16,
opcolor: Vec<u16>
}

#[derive(Debug, Clone)]
enum main_data { dinf(Vec<main_data>), smhd(main_data), stbl(Vec<main_data>), unknown(Vec<u8>), vmhd(main_data) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { hdlr(main_data), mdhd(main_data), minf(Vec<main_data>), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
creation_time: u32,
modification_time: u32,
track_ID: u32,
reserved: u32,
duration: u32
}

#[derive(Debug, Clone)]
struct main_data {
creation_time: u64,
modification_time: u64,
track_ID: u32,
reserved: u32,
duration: u64
}

#[derive(Debug, Clone)]
enum main_data { version0(main_data), version1(main_data) }

#[derive(Debug, Clone)]
struct main_data {
version: u8,
flags: (u8, u8, u8),
fields: main_data,
reserved2: (u32, u32),
layer: u16,
alternate_group: u16,
volume: u16,
reserved1: u16,
matrix: Vec<u32>,
width: u32,
height: u32
}

#[derive(Debug, Clone)]
enum main_data { edts(Vec<main_data>), mdia(Vec<main_data>), tkhd(main_data), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { meta(u32, Vec<main_data>), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { mvhd(main_data), trak(Vec<main_data>), udta(Vec<main_data>), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
enum main_data { free, ftyp(main_data), mdat, meta(u32, Vec<main_data>), moov(Vec<main_data>), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
struct main_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: main_data
}

#[derive(Debug, Clone)]
struct main_data {
atoms: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
width: u32,
height: u32,
bit_depth: u8,
color_type: u8,
compression_method: u8,
filter_method: u8,
interlace_method: u8
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: Vec<main_data>,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
greyscale: u16
}

#[derive(Debug, Clone)]
struct main_data {
red: u16,
green: u16,
blue: u16
}

#[derive(Debug, Clone)]
struct main_data {
palette_index: u8
}

#[derive(Debug, Clone)]
enum main_data { color_type_0(main_data), color_type_2(main_data), color_type_3(main_data), color_type_4(main_data), color_type_6(main_data) }

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
whitepoint_x: u32,
whitepoint_y: u32,
red_x: u32,
red_y: u32,
green_x: u32,
green_y: u32,
blue_x: u32,
blue_y: u32
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
gamma: u32
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
profile_name: Vec<u8>,
compression_method: u8,
compressed_profile: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
keyword: Vec<u8>,
compression_flag: u8,
compression_method: u8,
language_tag: main_data,
translated_keyword: main_data,
text: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
pixels_per_unit_x: u32,
pixels_per_unit_y: u32,
unit_specifier: u8
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
keyword: Vec<u8>,
text: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
year: u16,
month: u8,
day: u8,
hour: u8,
minute: u8,
second: u8
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
enum main_data { color_type_0(main_data), color_type_2(main_data), color_type_3(Vec<main_data>) }

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
keyword: Vec<u8>,
compression_method: u8,
compressed_text: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: main_data,
crc: u32
}

#[derive(Debug, Clone)]
enum main_data { PLTE(main_data), bKGD(main_data), cHRM(main_data), gAMA(main_data), iCCP(main_data), iTXt(main_data), pHYs(main_data), tEXt(main_data), tIME(main_data), tRNS(main_data), zTXt(main_data) }

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: Vec<u8>,
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
length: u32,
tag: (u8, u8, u8, u8),
data: (),
crc: u32
}

#[derive(Debug, Clone)]
struct main_data {
signature: (u8, u8, u8, u8, u8, u8, u8, u8),
ihdr: main_data,
chunks: Vec<main_data>,
idat: Vec<main_data>,
more_chunks: Vec<main_data>,
iend: main_data
}

#[derive(Debug, Clone)]
enum main_data { no(u8), yes }

#[derive(Debug, Clone)]
struct main_data {
tag: (u8, u8, u8, u8),
length: u32,
data: Vec<u8>,
pad: main_data
}

#[derive(Debug, Clone)]
struct main_data {
tag: (u8, u8, u8, u8),
chunks: Vec<main_data>
}

#[derive(Debug, Clone)]
struct main_data {
tag: (u8, u8, u8, u8),
length: u32,
data: main_data,
pad: main_data
}

#[derive(Debug, Clone)]
struct main_data {
string: Vec<u8>,
__padding: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
string: Vec<u8>,
__nul_or_wsp: u8,
__padding: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
string: Vec<u8>,
padding: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
name: main_data,
mode: main_data,
uid: main_data,
gid: main_data,
size: u32,
mtime: main_data,
chksum: main_data,
typeflag: u8,
linkname: main_data,
magic: (u8, u8, u8, u8, u8, u8),
version: (u8, u8),
uname: main_data,
gname: main_data,
devmajor: main_data,
devminor: main_data,
prefix: main_data,
pad: Vec<u8>
}

#[derive(Debug, Clone)]
struct main_data {
header: main_data,
file: Vec<u8>,
__padding: ()
}

#[derive(Debug, Clone)]
struct main_data {
contents: Vec<main_data>,
__padding: Vec<u8>,
__trailing: Vec<u8>
}

#[derive(Debug, Clone)]
enum main_data { gif(main_data), gzip(Vec<main_data>), jpeg(main_data), mpeg4(main_data), peano(Vec<u32>), png(main_data), riff(main_data), tar(main_data), text(Vec<char>) }

#[derive(Debug, Clone)]
enum deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths_denest_seq { none, some(u8) }

#[derive(Debug, Clone)]
struct tar_header_size {
oA: u8,
o9: u8,
o8: u8,
o7: u8,
o6: u8,
o5: u8,
o4: u8,
o3: u8,
o2: u8,
o1: u8,
o0: u8,
__nil: u8,
value: u32
}

#[derive(Debug, Clone)]
struct main {
data: main_data,
end: ()
}

fn Decoder0<'input>(_input: &mut Parser<'input>) -> Result<main, ParseError> {
PResult::Ok((Decoder1(_input))?)
}

fn Decoder1<'input>(_input: &mut Parser<'input>) -> Result<main, ParseError> {
let data = ((|| {
_input.start_alt();
{
let mut f_tmp = || PResult::Ok({
let inner = (Decoder2(_input))?;
main_data::peano(inner)
});
match f_tmp() {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
}
};
{
let mut f_tmp = || PResult::Ok({
let inner = (Decoder3(_input))?;
main_data::gif(inner)
});
match f_tmp() {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
}
};
{
let mut f_tmp = || PResult::Ok({
let inner = (Decoder4(_input))?;
main_data::gzip(inner)
});
match f_tmp() {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
}
};
{
let mut f_tmp = || PResult::Ok({
let inner = (Decoder5(_input))?;
main_data::jpeg(inner)
});
match f_tmp() {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
}
};
{
let mut f_tmp = || PResult::Ok({
let inner = (Decoder6(_input))?;
main_data::mpeg4(inner)
});
match f_tmp() {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
}
};
{
let mut f_tmp = || PResult::Ok({
let inner = (Decoder7(_input))?;
main_data::png(inner)
});
match f_tmp() {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
}
};
{
let mut f_tmp = || PResult::Ok({
let inner = (Decoder8(_input))?;
main_data::riff(inner)
});
match f_tmp() {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
}
};
{
let mut f_tmp = || PResult::Ok({
let inner = (Decoder9(_input))?;
main_data::tar(inner)
});
match f_tmp() {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(true)?;
}
}
};
{
let mut f_tmp = || PResult::Ok({
let inner = (Decoder10(_input))?;
main_data::text(inner)
});
match f_tmp() {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
return Err(_e);
}
}
};
})())?;
let end = ((|| PResult::Ok(_input.finish()?))())?;
PResult::Ok(main { data, end })
}

fn Decoder2<'input>(_input: &mut Parser<'input>) -> Result<Vec<u32>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
90u8 => {
1
},

83u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(8880004939303506267u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
if accum.is_empty() {
return Err(ParseError::InsufficientRepeats);
} else {
break
}
} else {
let next_elem = (Decoder182(_input))?;
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder3<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let header = ((|| PResult::Ok((Decoder164(_input))?))())?;
let logical_screen = ((|| PResult::Ok((Decoder165(_input))?))())?;
let blocks = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
33u8 => {
0
},

44u8 => {
0
},

59u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(18325384555431379809u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder166(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let trailer = ((|| PResult::Ok((Decoder167(_input))?))())?;
PResult::Ok(main_data { header, logical_screen, blocks, trailer })
}

fn Decoder4<'input>(_input: &mut Parser<'input>) -> Result<Vec<main_data>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 31 {
1
} else {
0
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
if accum.is_empty() {
return Err(ParseError::InsufficientRepeats);
} else {
break
}
} else {
let next_elem = {
let header = ((|| PResult::Ok((Decoder152(_input))?))())?;
let fname = ((|| PResult::Ok(match header.clone().file_flags & 8u8 != 0u8 {
true => {
let inner = (Decoder153(_input))?;
main_data::yes(inner)
},

false => {
main_data::no
}
}))())?;
let data = ((|| PResult::Ok({
_input.enter_bits_mode()?;
let ret = ((|| PResult::Ok((Decoder154(_input))?))())?;
let _bits_read = _input.escape_bits_mode()?;
ret
}))())?;
let footer = ((|| PResult::Ok((Decoder155(_input))?))())?;
main_data { header, fname, data, footer }
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder5<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let soi = ((|| PResult::Ok((Decoder79(_input))?))())?;
let frame = ((|| PResult::Ok((Decoder80(_input))?))())?;
let eoi = ((|| PResult::Ok((Decoder81(_input))?))())?;
PResult::Ok(main_data { soi, frame, eoi })
}

fn Decoder6<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let atoms = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder58(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(main_data { atoms })
}

fn Decoder7<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let signature = ((|| PResult::Ok((Decoder27(_input))?))())?;
let ihdr = ((|| PResult::Ok((Decoder28(_input))?))())?;
let chunks = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
let b = _input.read_byte()?;
{
let ret = match b {
98u8 => {
0
},

99u8 => {
0
},

105u8 => {
0
},

103u8 => {
0
},

112u8 => {
0
},

80u8 => {
0
},

116u8 => {
0
},

122u8 => {
0
},

73u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(11139425453690521993u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder29(_input, ihdr.clone()))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let idat = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
let b = _input.read_byte()?;
{
let ret = match b {
73u8 => {
let b = _input.read_byte()?;
match b {
69u8 => {
0
},

68u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(8806068124070768035u64));
}
}
},

98u8 => {
0
},

99u8 => {
0
},

105u8 => {
0
},

103u8 => {
0
},

112u8 => {
0
},

80u8 => {
0
},

116u8 => {
0
},

122u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(14501281906258189277u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
if accum.is_empty() {
return Err(ParseError::InsufficientRepeats);
} else {
break
}
} else {
let next_elem = (Decoder30(_input))?;
accum.push(next_elem);
}
}
accum
}))())?;
let more_chunks = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
let b = _input.read_byte()?;
{
let ret = match b {
98u8 => {
0
},

99u8 => {
0
},

105u8 => {
0
},

103u8 => {
0
},

112u8 => {
0
},

80u8 => {
0
},

116u8 => {
0
},

122u8 => {
0
},

73u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(11139425453690521993u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder29(_input, ihdr.clone()))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let iend = ((|| PResult::Ok((Decoder31(_input))?))())?;
PResult::Ok(main_data { signature, ihdr, chunks, idat, more_chunks, iend })
}

fn Decoder8<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 82 {
b
} else {
return Err(ParseError::ExcludedBranch(4610689655322527862u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(17197161005512507961u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(14049552398800766371u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(14049552398800766371u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let length = ((|| PResult::Ok((Decoder23(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder24(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let pad = ((|| PResult::Ok(match length % 2u32 == 0u32 {
true => {
main_data::yes
},

false => {
let inner = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
main_data::no(inner)
}
}))())?;
PResult::Ok(main_data { tag, length, data, pad })
}

fn Decoder9<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let contents = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
0u8 => {
0
},

tmp if (tmp != 0) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(5876973260510944493u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
if accum.is_empty() {
return Err(ParseError::InsufficientRepeats);
} else {
break
}
} else {
let next_elem = (Decoder14(_input))?;
accum.push(next_elem);
}
}
accum
}))())?;
let __padding = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..1024u32 {
accum.push({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
});
}
accum
}))())?;
let __trailing = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
0
} else {
1
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(main_data { contents, __padding, __trailing })
}

fn Decoder10<'input>(_input: &mut Parser<'input>) -> Result<Vec<char>, ParseError> {
PResult::Ok((Decoder11(_input))?)
}

fn Decoder11<'input>(_input: &mut Parser<'input>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([18446744073709551615, 18446744073709551615, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(tmp)) => {
0
},

224u8 => {
0
},

tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => {
0
},

237u8 => {
0
},

tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => {
0
},

240u8 => {
0
},

tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => {
0
},

244u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(8259468294382519899u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder12(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
}

fn Decoder12<'input>(_input: &mut Parser<'input>) -> Result<char, ParseError> {
let inner = {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([18446744073709551615, 18446744073709551615, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(tmp)) => {
1
},

224u8 => {
2
},

tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => {
2
},

237u8 => {
2
},

tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => {
2
},

240u8 => {
3
},

tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => {
3
},

244u8 => {
3
},

_ => {
return Err(ParseError::ExcludedBranch(6535045935131258590u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let inner = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744073709551615, 18446744073709551615, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(11920168927252633217u64));
}
};
((|byte: u8| PResult::Ok(byte as u32))(inner))?
},

1 => {
let inner = {
let field0 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 0, 4294967292])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(17624589492623733874u64));
}
};
((|raw: u8| PResult::Ok(raw & 31u8))(inner))?
}))())?;
let field1 = ((|| PResult::Ok((Decoder13(_input))?))())?;
(field0, field1)
};
((|bytes: (u8, u8)| PResult::Ok(match bytes {
(x1, x0) => {
(x1 as u32) << 6u32 | (x0 as u32)
}
}))(inner))?
},

2 => {
let inner = {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
224u8 => {
0
},

tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => {
1
},

237u8 => {
2
},

tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => {
3
},

_ => {
return Err(ParseError::ExcludedBranch(7728581146653271998u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let field0 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if b == 224 {
b
} else {
return Err(ParseError::ExcludedBranch(5346911683359312959u64));
}
};
((|raw: u8| PResult::Ok(raw & 15u8))(inner))?
}))())?;
let field1 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 18446744069414584320, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(10020684034467804360u64));
}
};
((|raw: u8| PResult::Ok(raw & 63u8))(inner))?
}))())?;
let field2 = ((|| PResult::Ok((Decoder13(_input))?))())?;
(field0, field1, field2)
},

1 => {
let field0 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(15018012796466655710u64));
}
};
((|raw: u8| PResult::Ok(raw & 15u8))(inner))?
}))())?;
let field1 = ((|| PResult::Ok((Decoder13(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder13(_input))?))())?;
(field0, field1, field2)
},

2 => {
let field0 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if b == 237 {
b
} else {
return Err(ParseError::ExcludedBranch(4000866269867594892u64));
}
};
((|raw: u8| PResult::Ok(raw & 15u8))(inner))?
}))())?;
let field1 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 4294967295, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(11663367663089555181u64));
}
};
((|raw: u8| PResult::Ok(raw & 63u8))(inner))?
}))())?;
let field2 = ((|| PResult::Ok((Decoder13(_input))?))())?;
(field0, field1, field2)
},

3 => {
let field0 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(6041500870840229679u64));
}
};
((|raw: u8| PResult::Ok(raw & 15u8))(inner))?
}))())?;
let field1 = ((|| PResult::Ok((Decoder13(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder13(_input))?))())?;
(field0, field1, field2)
},

_ => {
return Err(ParseError::ExcludedBranch(16680599075360251934u64));
}
}
};
((|bytes: (u8, u8, u8)| PResult::Ok(match bytes {
(x2, x1, x0) => {
(x2 as u32) << 12u32 | (x1 as u32) << 6u32 | (x0 as u32)
}
}))(inner))?
},

3 => {
let inner = {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
240u8 => {
0
},

tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => {
1
},

244u8 => {
2
},

_ => {
return Err(ParseError::ExcludedBranch(7207241947967887206u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let field0 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if b == 240 {
b
} else {
return Err(ParseError::ExcludedBranch(4436478097112104593u64));
}
};
((|raw: u8| PResult::Ok(raw & 7u8))(inner))?
}))())?;
let field1 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 18446744073709486080, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(2326106400913054182u64));
}
};
((|raw: u8| PResult::Ok(raw & 63u8))(inner))?
}))())?;
let field2 = ((|| PResult::Ok((Decoder13(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder13(_input))?))())?;
(field0, field1, field2, field3)
},

1 => {
let field0 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(2405483008932899239u64));
}
};
((|raw: u8| PResult::Ok(raw & 7u8))(inner))?
}))())?;
let field1 = ((|| PResult::Ok((Decoder13(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder13(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder13(_input))?))())?;
(field0, field1, field2, field3)
},

2 => {
let field0 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if b == 244 {
b
} else {
return Err(ParseError::ExcludedBranch(7074153516412524481u64));
}
};
((|raw: u8| PResult::Ok(raw & 7u8))(inner))?
}))())?;
let field1 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 65535, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(7043438521252360401u64));
}
};
((|raw: u8| PResult::Ok(raw & 63u8))(inner))?
}))())?;
let field2 = ((|| PResult::Ok((Decoder13(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder13(_input))?))())?;
(field0, field1, field2, field3)
},

_ => {
return Err(ParseError::ExcludedBranch(10897670729404727847u64));
}
}
};
((|bytes: (u8, u8, u8, u8)| PResult::Ok(match bytes {
(x3, x2, x1, x0) => {
(x3 as u32) << 18u32 | (x2 as u32) << 12u32 | (x1 as u32) << 6u32 | (x0 as u32)
}
}))(inner))?
},

_ => {
return Err(ParseError::ExcludedBranch(7414759281301542086u64));
}
}
};
PResult::Ok(((|codepoint: u32| PResult::Ok((char::from_u32(codepoint)).unwrap()))(inner))?)
}

fn Decoder13<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let inner = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 18446744073709551615, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(15157310944304873712u64));
}
};
PResult::Ok(((|raw: u8| PResult::Ok(raw & 63u8))(inner))?)
}

fn Decoder14<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let header = ((|| PResult::Ok((Decoder15(_input))?))())?;
let file = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..header.clone().size {
accum.push((Decoder16(_input))?);
}
accum
}))())?;
let __padding = ((|| PResult::Ok(_input.skip_align(512)?))())?;
PResult::Ok(main_data { header, file, __padding })
}

fn Decoder15<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let sz = 512u32 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let name = ((|| PResult::Ok({
let sz = 100u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder17(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let mode = ((|| PResult::Ok({
let sz = 8u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2596090920165813952u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder18(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder19(_input))?))())?;
let __padding = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
0
} else {
1
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { string, __nul_or_wsp, __padding }
}))())?;
_input.end_slice()?;
ret
}))())?;
let uid = ((|| PResult::Ok({
let sz = 8u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2596090920165813952u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder18(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder19(_input))?))())?;
let __padding = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
0
} else {
1
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { string, __nul_or_wsp, __padding }
}))())?;
_input.end_slice()?;
ret
}))())?;
let gid = ((|| PResult::Ok({
let sz = 8u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2596090920165813952u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder18(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder19(_input))?))())?;
let __padding = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
0
} else {
1
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { string, __nul_or_wsp, __padding }
}))())?;
_input.end_slice()?;
ret
}))())?;
let size = ((|| PResult::Ok({
let inner = {
let oA = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let o9 = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let o8 = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let o7 = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let o6 = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let o5 = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let o4 = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let o3 = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let o2 = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let o1 = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let o0 = ((|| PResult::Ok({
let inner = (Decoder18(_input))?;
((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
}))())?;
let __nil = ((|| PResult::Ok((Decoder19(_input))?))())?;
let value = ((|| PResult::Ok((((0u8 as u32) << 3u32 | (oA as u32)) << 6u32 | (o9 as u32) << 3u32 | (o8 as u32)) << 24u32 | (((o7 as u32) << 3u32 | (o6 as u32)) << 6u32 | (o5 as u32) << 3u32 | (o4 as u32)) << 12u32 | ((o3 as u32) << 3u32 | (o2 as u32)) << 6u32 | (o1 as u32) << 3u32 | (o0 as u32)))())?;
tar_header_size { oA, o9, o8, o7, o6, o5, o4, o3, o2, o1, o0, __nil, value }
};
((|rec: tar_header_size| PResult::Ok(rec.clone().value))(inner))?
}))())?;
let mtime = ((|| PResult::Ok({
let sz = 12u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2596090920165813952u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder18(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder19(_input))?))())?;
let __padding = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
0
} else {
1
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { string, __nul_or_wsp, __padding }
}))())?;
_input.end_slice()?;
ret
}))())?;
let chksum = ((|| PResult::Ok({
let sz = 8u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2596090920165813952u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder18(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder19(_input))?))())?;
let __padding = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
0
} else {
1
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { string, __nul_or_wsp, __padding }
}))())?;
_input.end_slice()?;
ret
}))())?;
let typeflag = ((|| PResult::Ok((Decoder20(_input))?))())?;
let linkname = ((|| PResult::Ok({
let sz = 100u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder21(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let magic = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 117 {
b
} else {
return Err(ParseError::ExcludedBranch(2907868308058195485u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 115 {
b
} else {
return Err(ParseError::ExcludedBranch(17994192348199484624u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 116 {
b
} else {
return Err(ParseError::ExcludedBranch(1704008783145591213u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 97 {
b
} else {
return Err(ParseError::ExcludedBranch(5987471249031546739u64));
}
}))())?;
let field4 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 114 {
b
} else {
return Err(ParseError::ExcludedBranch(3985419300396206930u64));
}
}))())?;
let field5 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
(field0, field1, field2, field3, field4, field5)
}))())?;
let version = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 48 {
b
} else {
return Err(ParseError::ExcludedBranch(10502325800520584810u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 48 {
b
} else {
return Err(ParseError::ExcludedBranch(10502325800520584810u64));
}
}))())?;
(field0, field1)
}))())?;
let uname = ((|| PResult::Ok({
let sz = 32u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder22(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let gname = ((|| PResult::Ok({
let sz = 32u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder22(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let devmajor = ((|| PResult::Ok({
let sz = 8u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2596090920165813952u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder18(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder19(_input))?))())?;
let __padding = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
0
} else {
1
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { string, __nul_or_wsp, __padding }
}))())?;
_input.end_slice()?;
ret
}))())?;
let devminor = ((|| PResult::Ok({
let sz = 8u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2596090920165813952u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder18(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder19(_input))?))())?;
let __padding = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
0
} else {
1
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { string, __nul_or_wsp, __padding }
}))())?;
_input.end_slice()?;
ret
}))())?;
let prefix = ((|| PResult::Ok({
let sz = 155u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder21(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let pad = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..12u32 {
accum.push({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
});
}
accum
}))())?;
main_data { name, mode, uid, gid, size, mtime, chksum, typeflag, linkname, magic, version, uname, gname, devmajor, devminor, prefix, pad }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder16<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(b)
}

fn Decoder17<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
0u8 => {
0
},

tmp if (tmp != 0) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(9907092251485419402u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
if accum.is_empty() {
return Err(ParseError::InsufficientRepeats);
} else {
break
}
} else {
let next_elem = {
let b = _input.read_byte()?;
if b != 0 {
b
} else {
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
}
}
accum
}))())?;
let __padding = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
0
} else {
1
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(main_data { string, __padding })
}

fn Decoder18<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if (ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(16196330650984947656u64));
})
}

fn Decoder19<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if (ByteSet::from_bits([4294967297, 0, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9824667705306069359u64));
})
}

fn Decoder20<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(b)
}

fn Decoder21<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(10468509372044097033u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __padding = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
0
} else {
1
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(main_data { string, __padding })
}

fn Decoder22<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let padding = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 0 {
1
} else {
0
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
if accum.is_empty() {
return Err(ParseError::InsufficientRepeats);
} else {
break
}
} else {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
accum.push(next_elem);
}
}
accum
}))())?;
PResult::Ok(main_data { string, padding })
}

fn Decoder23<'input>(_input: &mut Parser<'input>) -> Result<u32, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2, field3)
};
PResult::Ok(((|x: (u8, u8, u8, u8)| PResult::Ok(u32le(x)))(inner))?)
}

fn Decoder24<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let tag = ((|| PResult::Ok((Decoder25(_input))?))())?;
let chunks = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder26(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(main_data { tag, chunks })
}

fn Decoder25<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = ((|| PResult::Ok((Decoder20(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder20(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder20(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder20(_input))?))())?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder26<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let tag = ((|| PResult::Ok((Decoder25(_input))?))())?;
let length = ((|| PResult::Ok((Decoder23(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
let pad = ((|| PResult::Ok(match length % 2u32 == 0u32 {
true => {
main_data::yes
},

false => {
let inner = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
main_data::no(inner)
}
}))())?;
PResult::Ok(main_data { tag, length, data, pad })
}

fn Decoder27<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8, u8, u8, u8, u8), ParseError> {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 137 {
b
} else {
return Err(ParseError::ExcludedBranch(10008271234946209065u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 80 {
b
} else {
return Err(ParseError::ExcludedBranch(11521109187063420822u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 78 {
b
} else {
return Err(ParseError::ExcludedBranch(8604468179520937907u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 71 {
b
} else {
return Err(ParseError::ExcludedBranch(690880023569680479u64));
}
}))())?;
let field4 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 13 {
b
} else {
return Err(ParseError::ExcludedBranch(10755821400739488603u64));
}
}))())?;
let field5 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 10 {
b
} else {
return Err(ParseError::ExcludedBranch(4202505692043699682u64));
}
}))())?;
let field6 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 26 {
b
} else {
return Err(ParseError::ExcludedBranch(349275303258878611u64));
}
}))())?;
let field7 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 10 {
b
} else {
return Err(ParseError::ExcludedBranch(4202505692043699682u64));
}
}))())?;
PResult::Ok((field0, field1, field2, field3, field4, field5, field6, field7))
}

fn Decoder28<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok((Decoder56(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder57(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder29<'input>(_input: &mut Parser<'input>, ihdr: main_data) -> Result<main_data, ParseError> {
let tree_index = {
_input.open_peek_context();
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
let b = _input.read_byte()?;
{
let ret = match b {
98u8 => {
0
},

99u8 => {
1
},

105u8 => {
let b = _input.read_byte()?;
match b {
67u8 => {
2
},

84u8 => {
3
},

_ => {
return Err(ParseError::ExcludedBranch(15600701245169182301u64));
}
}
},

103u8 => {
4
},

112u8 => {
5
},

80u8 => {
6
},

116u8 => {
let b = _input.read_byte()?;
match b {
69u8 => {
7
},

73u8 => {
8
},

82u8 => {
9
},

_ => {
return Err(ParseError::ExcludedBranch(10893936849216007982u64));
}
}
},

122u8 => {
10
},

_ => {
return Err(ParseError::ExcludedBranch(4370585114495553186u64));
}
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = (Decoder37(_input, ihdr.clone()))?;
main_data::bKGD(inner)
},

1 => {
let inner = (Decoder38(_input))?;
main_data::cHRM(inner)
},

2 => {
let inner = (Decoder39(_input))?;
main_data::iCCP(inner)
},

3 => {
let inner = (Decoder40(_input))?;
main_data::iTXt(inner)
},

4 => {
let inner = (Decoder41(_input))?;
main_data::gAMA(inner)
},

5 => {
let inner = (Decoder42(_input))?;
main_data::pHYs(inner)
},

6 => {
let inner = (Decoder43(_input))?;
main_data::PLTE(inner)
},

7 => {
let inner = (Decoder44(_input))?;
main_data::tEXt(inner)
},

8 => {
let inner = (Decoder45(_input))?;
main_data::tIME(inner)
},

9 => {
let inner = (Decoder46(_input, ihdr.clone()))?;
main_data::tRNS(inner)
},

10 => {
let inner = (Decoder47(_input))?;
main_data::zTXt(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(13044660050275045301u64));
}
})
}

fn Decoder30<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok((Decoder35(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder36(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder31<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok((Decoder33(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder34(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder32<'input>(_input: &mut Parser<'input>) -> Result<u32, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2, field3)
};
PResult::Ok(((|x: (u8, u8, u8, u8)| PResult::Ok(u32be(x)))(inner))?)
}

fn Decoder33<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(17197161005512507961u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 69 {
b
} else {
return Err(ParseError::ExcludedBranch(4321719390811047443u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 78 {
b
} else {
return Err(ParseError::ExcludedBranch(8604468179520937907u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(11087183532096489351u64));
}
}))())?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder34<'input>(_input: &mut Parser<'input>) -> Result<(), ParseError> {
PResult::Ok(())
}

fn Decoder35<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(17197161005512507961u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(11087183532096489351u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 65 {
b
} else {
return Err(ParseError::ExcludedBranch(5168475411614401238u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 84 {
b
} else {
return Err(ParseError::ExcludedBranch(145148447135656575u64));
}
}))())?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder36<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
}

fn Decoder37<'input>(_input: &mut Parser<'input>, ihdr: main_data) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 98 {
b
} else {
return Err(ParseError::ExcludedBranch(17793564444344969327u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 75 {
b
} else {
return Err(ParseError::ExcludedBranch(6039736239144529496u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 71 {
b
} else {
return Err(ParseError::ExcludedBranch(690880023569680479u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(11087183532096489351u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match ihdr.clone().data.color_type {
0 => {
let inner = {
let greyscale = ((|| PResult::Ok((Decoder50(_input))?))())?;
main_data { greyscale }
};
main_data::color_type_0(inner)
},

4 => {
let inner = {
let greyscale = ((|| PResult::Ok((Decoder50(_input))?))())?;
main_data { greyscale }
};
main_data::color_type_4(inner)
},

2 => {
let inner = {
let red = ((|| PResult::Ok((Decoder50(_input))?))())?;
let green = ((|| PResult::Ok((Decoder50(_input))?))())?;
let blue = ((|| PResult::Ok((Decoder50(_input))?))())?;
main_data { red, green, blue }
};
main_data::color_type_2(inner)
},

6 => {
let inner = {
let red = ((|| PResult::Ok((Decoder50(_input))?))())?;
let green = ((|| PResult::Ok((Decoder50(_input))?))())?;
let blue = ((|| PResult::Ok((Decoder50(_input))?))())?;
main_data { red, green, blue }
};
main_data::color_type_6(inner)
},

3 => {
let inner = {
let palette_index = ((|| PResult::Ok((Decoder16(_input))?))())?;
main_data { palette_index }
};
main_data::color_type_3(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder38<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 99 {
b
} else {
return Err(ParseError::ExcludedBranch(11313607538540189010u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 72 {
b
} else {
return Err(ParseError::ExcludedBranch(13017675598322041426u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 82 {
b
} else {
return Err(ParseError::ExcludedBranch(4610689655322527862u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 77 {
b
} else {
return Err(ParseError::ExcludedBranch(1661485880725065159u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let whitepoint_x = ((|| PResult::Ok((Decoder32(_input))?))())?;
let whitepoint_y = ((|| PResult::Ok((Decoder32(_input))?))())?;
let red_x = ((|| PResult::Ok((Decoder32(_input))?))())?;
let red_y = ((|| PResult::Ok((Decoder32(_input))?))())?;
let green_x = ((|| PResult::Ok((Decoder32(_input))?))())?;
let green_y = ((|| PResult::Ok((Decoder32(_input))?))())?;
let blue_x = ((|| PResult::Ok((Decoder32(_input))?))())?;
let blue_y = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { whitepoint_x, whitepoint_y, red_x, red_y, green_x, green_y, blue_x, blue_y }
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder39<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 105 {
b
} else {
return Err(ParseError::ExcludedBranch(15111945935260023152u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 67 {
b
} else {
return Err(ParseError::ExcludedBranch(10318298496630049506u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 67 {
b
} else {
return Err(ParseError::ExcludedBranch(10318298496630049506u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 80 {
b
} else {
return Err(ParseError::ExcludedBranch(11521109187063420822u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let profile_name = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder55(_input))?))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
(field0, field1)
};
((|x: (Vec<u8>, u8)| PResult::Ok(x.clone().0))(inner))?
}))())?;
let compression_method = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
let compressed_profile = ((|| PResult::Ok((Decoder49(_input))?))())?;
main_data { profile_name, compression_method, compressed_profile }
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder40<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 105 {
b
} else {
return Err(ParseError::ExcludedBranch(15111945935260023152u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 84 {
b
} else {
return Err(ParseError::ExcludedBranch(145148447135656575u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 88 {
b
} else {
return Err(ParseError::ExcludedBranch(17869373923605816126u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 116 {
b
} else {
return Err(ParseError::ExcludedBranch(1704008783145591213u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let keyword = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder52(_input))?))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
(field0, field1)
};
((|x: (Vec<u8>, u8)| PResult::Ok(x.clone().0))(inner))?
}))())?;
let compression_flag = ((|| PResult::Ok({
let b = _input.read_byte()?;
if (ByteSet::from_bits([3, 0, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(13168638698998618208u64));
}
}))())?;
let compression_method = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
let language_tag = ((|| PResult::Ok((Decoder53(_input))?))())?;
let translated_keyword = ((|| PResult::Ok((Decoder54(_input))?))())?;
let text = ((|| PResult::Ok(match compression_flag == 1u8 {
true => {
(Decoder49(_input))?
},

false => {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}
}))())?;
main_data { keyword, compression_flag, compression_method, language_tag, translated_keyword, text }
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder41<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 103 {
b
} else {
return Err(ParseError::ExcludedBranch(1468953881245131984u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 65 {
b
} else {
return Err(ParseError::ExcludedBranch(5168475411614401238u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 77 {
b
} else {
return Err(ParseError::ExcludedBranch(1661485880725065159u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 65 {
b
} else {
return Err(ParseError::ExcludedBranch(5168475411614401238u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let gamma = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { gamma }
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder42<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 112 {
b
} else {
return Err(ParseError::ExcludedBranch(11580992248901122101u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 72 {
b
} else {
return Err(ParseError::ExcludedBranch(13017675598322041426u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 89 {
b
} else {
return Err(ParseError::ExcludedBranch(8653514599897871365u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 115 {
b
} else {
return Err(ParseError::ExcludedBranch(17994192348199484624u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let pixels_per_unit_x = ((|| PResult::Ok((Decoder32(_input))?))())?;
let pixels_per_unit_y = ((|| PResult::Ok((Decoder32(_input))?))())?;
let unit_specifier = ((|| PResult::Ok((Decoder16(_input))?))())?;
main_data { pixels_per_unit_x, pixels_per_unit_y, unit_specifier }
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder43<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 80 {
b
} else {
return Err(ParseError::ExcludedBranch(11521109187063420822u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 76 {
b
} else {
return Err(ParseError::ExcludedBranch(7343583089148506132u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 84 {
b
} else {
return Err(ParseError::ExcludedBranch(145148447135656575u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 69 {
b
} else {
return Err(ParseError::ExcludedBranch(4321719390811047443u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 1;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
if accum.is_empty() {
return Err(ParseError::InsufficientRepeats);
} else {
break
}
} else {
let next_elem = {
let r = ((|| PResult::Ok((Decoder16(_input))?))())?;
let g = ((|| PResult::Ok((Decoder16(_input))?))())?;
let b = ((|| PResult::Ok((Decoder16(_input))?))())?;
main_data { r, g, b }
};
accum.push(next_elem);
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder44<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 116 {
b
} else {
return Err(ParseError::ExcludedBranch(1704008783145591213u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 69 {
b
} else {
return Err(ParseError::ExcludedBranch(4321719390811047443u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 88 {
b
} else {
return Err(ParseError::ExcludedBranch(17869373923605816126u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 116 {
b
} else {
return Err(ParseError::ExcludedBranch(1704008783145591213u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let keyword = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder51(_input))?))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
(field0, field1)
};
((|x: (Vec<u8>, u8)| PResult::Ok(x.clone().0))(inner))?
}))())?;
let text = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder20(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { keyword, text }
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder45<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 116 {
b
} else {
return Err(ParseError::ExcludedBranch(1704008783145591213u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(17197161005512507961u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 77 {
b
} else {
return Err(ParseError::ExcludedBranch(1661485880725065159u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 69 {
b
} else {
return Err(ParseError::ExcludedBranch(4321719390811047443u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let year = ((|| PResult::Ok((Decoder50(_input))?))())?;
let month = ((|| PResult::Ok((Decoder16(_input))?))())?;
let day = ((|| PResult::Ok((Decoder16(_input))?))())?;
let hour = ((|| PResult::Ok((Decoder16(_input))?))())?;
let minute = ((|| PResult::Ok((Decoder16(_input))?))())?;
let second = ((|| PResult::Ok((Decoder16(_input))?))())?;
main_data { year, month, day, hour, minute, second }
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder46<'input>(_input: &mut Parser<'input>, ihdr: main_data) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 116 {
b
} else {
return Err(ParseError::ExcludedBranch(1704008783145591213u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 82 {
b
} else {
return Err(ParseError::ExcludedBranch(4610689655322527862u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 78 {
b
} else {
return Err(ParseError::ExcludedBranch(8604468179520937907u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 83 {
b
} else {
return Err(ParseError::ExcludedBranch(16554645260058031671u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match ihdr.clone().data.color_type {
0 => {
let inner = {
let greyscale = ((|| PResult::Ok((Decoder50(_input))?))())?;
main_data { greyscale }
};
main_data::color_type_0(inner)
},

2 => {
let inner = {
let red = ((|| PResult::Ok((Decoder50(_input))?))())?;
let green = ((|| PResult::Ok((Decoder50(_input))?))())?;
let blue = ((|| PResult::Ok((Decoder50(_input))?))())?;
main_data { red, green, blue }
};
main_data::color_type_2(inner)
},

3 => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let palette_index = ((|| PResult::Ok((Decoder16(_input))?))())?;
main_data { palette_index }
};
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::color_type_3(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder47<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let tag = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 122 {
b
} else {
return Err(ParseError::ExcludedBranch(10490665823433481845u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 84 {
b
} else {
return Err(ParseError::ExcludedBranch(145148447135656575u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 88 {
b
} else {
return Err(ParseError::ExcludedBranch(17869373923605816126u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 116 {
b
} else {
return Err(ParseError::ExcludedBranch(1704008783145591213u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let keyword = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder48(_input))?))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
(field0, field1)
};
((|x: (Vec<u8>, u8)| PResult::Ok(x.clone().0))(inner))?
}))())?;
let compression_method = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
let compressed_text = ((|| PResult::Ok((Decoder49(_input))?))())?;
main_data { keyword, compression_method, compressed_text }
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder32(_input))?))())?;
PResult::Ok(main_data { length, tag, data, crc })
}

fn Decoder48<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
0u8 => {
0
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
1
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
2
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
3
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
4
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
5
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
6
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
7
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
8
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
9
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
10
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
11
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
12
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
13
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
14
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
15
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
16
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
17
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
18
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
19
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
20
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
21
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
22
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
23
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
24
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
25
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
26
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
27
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
28
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
29
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
30
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
31
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
32
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
33
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
34
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
35
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
36
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
37
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
38
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
39
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
40
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
41
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
42
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
43
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
44
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
45
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
46
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
47
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
48
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
49
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
50
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
51
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
52
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
53
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
54
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
55
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
56
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
57
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
58
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
59
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
60
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
61
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
62
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
63
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
64
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
65
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
66
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
67
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
68
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
69
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
70
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
71
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
72
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
73
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
74
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
75
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
76
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
77
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
78
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
79
},

_ => {
return Err(ParseError::ExcludedBranch(1156600997808834721u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(686893874203959698u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5647302839925181930u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12596085444683110489u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(698004606122880289u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17484571581640965095u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6540530436842500333u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1885078274385301600u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2760313249059646380u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6864494491173992330u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12031893906322850579u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3325605041662679663u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18127647179293822299u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1757426024726163624u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(178440331724964330u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7701642596783457452u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5889869058337734004u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(448960124419894112u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17968063285163593108u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6658979120527716197u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14070492394437254859u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12094341563934969777u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14447745632705856537u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2878199988418950426u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9981078220607407536u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13979884363466418441u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14697219789812534349u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1922183737469204535u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4564459967526826259u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6295454573086855551u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8829268174072585826u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17289562280941389130u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15187733903820788036u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8945233277383628939u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17055677057026365051u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2711657847979741650u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7960086128827310454u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17243432721042925232u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2492433356894874420u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2412963531573765251u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13615115093049678240u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12861038934173916684u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15290638577266507314u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13771803105164343178u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8440227624652964490u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11749686089473367822u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14128049364035057882u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5772944606843372240u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12877845223200621278u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18429737009659339382u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5056695690378482781u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18175410941244882664u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14689827387576631959u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13752824778747682586u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15099715940097679920u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7570592576744298472u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9011326107999450601u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18413916726623917222u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11077716068559322830u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16761545731695489821u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1796681571676370638u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12634645130304766428u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10851594797972925398u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2877758930083196789u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4021154774029150054u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4872726929046804051u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13578407048997150968u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14985643526348759689u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13195439938299117823u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14643378569655829231u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16890725544144972486u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5759023799041458604u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7279863718715306056u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1929389086973805060u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16960558233825067461u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18079708419564968323u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13745914803581094198u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6362830467043337482u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5206670497493022146u64));
}
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(matching_ix == 0, accum.len() >= (1u32 as usize), accum.len() == (79u32 as usize)))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(15306540504651776134u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder49<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
}

fn Decoder50<'input>(_input: &mut Parser<'input>) -> Result<u16, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1)
};
PResult::Ok(((|x: (u8, u8)| PResult::Ok(u16be(x)))(inner))?)
}

fn Decoder51<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
0u8 => {
0
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
1
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
2
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
3
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
4
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
5
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
6
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
7
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
8
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
9
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
10
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
11
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
12
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
13
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
14
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
15
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
16
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
17
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
18
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
19
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
20
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
21
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
22
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
23
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
24
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
25
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
26
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
27
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
28
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
29
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
30
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
31
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
32
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
33
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
34
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
35
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
36
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
37
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
38
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
39
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
40
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
41
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
42
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
43
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
44
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
45
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
46
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
47
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
48
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
49
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
50
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
51
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
52
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
53
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
54
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
55
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
56
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
57
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
58
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
59
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
60
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
61
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
62
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
63
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
64
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
65
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
66
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
67
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
68
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
69
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
70
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
71
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
72
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
73
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
74
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
75
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
76
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
77
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
78
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
79
},

_ => {
return Err(ParseError::ExcludedBranch(1156600997808834721u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(686893874203959698u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5647302839925181930u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12596085444683110489u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(698004606122880289u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17484571581640965095u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6540530436842500333u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1885078274385301600u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2760313249059646380u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6864494491173992330u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12031893906322850579u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3325605041662679663u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18127647179293822299u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1757426024726163624u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(178440331724964330u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7701642596783457452u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5889869058337734004u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(448960124419894112u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17968063285163593108u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6658979120527716197u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14070492394437254859u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12094341563934969777u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14447745632705856537u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2878199988418950426u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9981078220607407536u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13979884363466418441u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14697219789812534349u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1922183737469204535u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4564459967526826259u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6295454573086855551u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8829268174072585826u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17289562280941389130u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15187733903820788036u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8945233277383628939u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17055677057026365051u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2711657847979741650u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7960086128827310454u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17243432721042925232u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2492433356894874420u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2412963531573765251u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13615115093049678240u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12861038934173916684u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15290638577266507314u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13771803105164343178u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8440227624652964490u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11749686089473367822u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14128049364035057882u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5772944606843372240u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12877845223200621278u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18429737009659339382u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5056695690378482781u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18175410941244882664u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14689827387576631959u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13752824778747682586u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15099715940097679920u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7570592576744298472u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9011326107999450601u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18413916726623917222u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11077716068559322830u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16761545731695489821u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1796681571676370638u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12634645130304766428u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10851594797972925398u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2877758930083196789u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4021154774029150054u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4872726929046804051u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13578407048997150968u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14985643526348759689u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13195439938299117823u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14643378569655829231u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16890725544144972486u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5759023799041458604u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7279863718715306056u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1929389086973805060u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16960558233825067461u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18079708419564968323u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13745914803581094198u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6362830467043337482u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5206670497493022146u64));
}
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(matching_ix == 0, accum.len() >= (1u32 as usize), accum.len() == (79u32 as usize)))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(15306540504651776134u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder52<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
0u8 => {
0
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
1
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
2
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
3
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
4
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
5
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
6
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
7
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
8
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
9
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
10
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
11
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
12
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
13
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
14
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
15
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
16
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
17
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
18
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
19
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
20
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
21
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
22
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
23
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
24
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
25
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
26
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
27
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
28
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
29
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
30
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
31
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
32
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
33
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
34
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
35
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
36
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
37
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
38
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
39
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
40
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
41
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
42
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
43
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
44
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
45
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
46
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
47
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
48
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
49
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
50
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
51
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
52
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
53
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
54
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
55
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
56
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
57
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
58
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
59
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
60
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
61
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
62
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
63
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
64
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
65
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
66
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
67
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
68
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
69
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
70
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
71
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
72
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
73
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
74
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
75
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
76
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
77
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
78
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
79
},

_ => {
return Err(ParseError::ExcludedBranch(1156600997808834721u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(686893874203959698u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5647302839925181930u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12596085444683110489u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(698004606122880289u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17484571581640965095u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6540530436842500333u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1885078274385301600u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2760313249059646380u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6864494491173992330u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12031893906322850579u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3325605041662679663u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18127647179293822299u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1757426024726163624u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(178440331724964330u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7701642596783457452u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5889869058337734004u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(448960124419894112u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17968063285163593108u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6658979120527716197u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14070492394437254859u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12094341563934969777u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14447745632705856537u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2878199988418950426u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9981078220607407536u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13979884363466418441u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14697219789812534349u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1922183737469204535u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4564459967526826259u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6295454573086855551u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8829268174072585826u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17289562280941389130u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15187733903820788036u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8945233277383628939u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17055677057026365051u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2711657847979741650u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7960086128827310454u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17243432721042925232u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2492433356894874420u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2412963531573765251u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13615115093049678240u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12861038934173916684u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15290638577266507314u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13771803105164343178u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8440227624652964490u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11749686089473367822u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14128049364035057882u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5772944606843372240u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12877845223200621278u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18429737009659339382u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5056695690378482781u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18175410941244882664u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14689827387576631959u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13752824778747682586u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15099715940097679920u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7570592576744298472u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9011326107999450601u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18413916726623917222u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11077716068559322830u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16761545731695489821u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1796681571676370638u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12634645130304766428u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10851594797972925398u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2877758930083196789u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4021154774029150054u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4872726929046804051u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13578407048997150968u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14985643526348759689u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13195439938299117823u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14643378569655829231u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16890725544144972486u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5759023799041458604u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7279863718715306056u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1929389086973805060u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16960558233825067461u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18079708419564968323u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13745914803581094198u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6362830467043337482u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5206670497493022146u64));
}
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(matching_ix == 0, accum.len() >= (1u32 as usize), accum.len() == (79u32 as usize)))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(15306540504651776134u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder53<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder54<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder55<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
0u8 => {
0
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
1
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
2
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
3
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
4
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
5
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
6
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
7
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
8
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
9
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
10
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
11
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
12
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
13
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
14
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
15
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
16
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
17
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
18
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
19
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
20
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
21
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
22
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
23
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
24
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
25
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
26
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
27
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
28
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
29
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
30
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
31
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
32
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
33
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
34
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
35
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
36
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
37
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
38
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
39
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
40
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
41
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
42
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
43
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
44
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
45
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
46
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
47
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
48
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
49
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
50
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
51
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
52
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
53
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
54
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
55
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
56
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
57
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
58
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
59
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
60
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
61
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
62
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
63
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
64
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
65
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
66
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
67
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
68
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
69
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
70
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
71
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
72
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
73
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
74
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
75
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
76
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
77
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
let b = _input.read_byte()?;
match b {
0u8 => {
78
},

tmp if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(tmp)) => {
79
},

_ => {
return Err(ParseError::ExcludedBranch(1156600997808834721u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(686893874203959698u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5647302839925181930u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12596085444683110489u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(698004606122880289u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17484571581640965095u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6540530436842500333u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1885078274385301600u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2760313249059646380u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6864494491173992330u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12031893906322850579u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3325605041662679663u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18127647179293822299u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1757426024726163624u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(178440331724964330u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7701642596783457452u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5889869058337734004u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(448960124419894112u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17968063285163593108u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6658979120527716197u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14070492394437254859u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12094341563934969777u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14447745632705856537u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2878199988418950426u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9981078220607407536u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13979884363466418441u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14697219789812534349u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1922183737469204535u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4564459967526826259u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6295454573086855551u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8829268174072585826u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17289562280941389130u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15187733903820788036u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8945233277383628939u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17055677057026365051u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2711657847979741650u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7960086128827310454u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17243432721042925232u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2492433356894874420u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2412963531573765251u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13615115093049678240u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12861038934173916684u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15290638577266507314u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13771803105164343178u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8440227624652964490u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11749686089473367822u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14128049364035057882u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5772944606843372240u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12877845223200621278u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18429737009659339382u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5056695690378482781u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18175410941244882664u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14689827387576631959u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13752824778747682586u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15099715940097679920u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7570592576744298472u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9011326107999450601u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18413916726623917222u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11077716068559322830u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16761545731695489821u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1796681571676370638u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12634645130304766428u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10851594797972925398u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2877758930083196789u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4021154774029150054u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4872726929046804051u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13578407048997150968u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14985643526348759689u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13195439938299117823u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14643378569655829231u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16890725544144972486u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5759023799041458604u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7279863718715306056u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1929389086973805060u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16960558233825067461u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18079708419564968323u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13745914803581094198u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6362830467043337482u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5206670497493022146u64));
}
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(matching_ix == 0, accum.len() >= (1u32 as usize), accum.len() == (79u32 as usize)))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(15306540504651776134u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder56<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(17197161005512507961u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 72 {
b
} else {
return Err(ParseError::ExcludedBranch(13017675598322041426u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(11087183532096489351u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 82 {
b
} else {
return Err(ParseError::ExcludedBranch(4610689655322527862u64));
}
}))())?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder57<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let width = ((|| PResult::Ok((Decoder32(_input))?))())?;
let height = ((|| PResult::Ok((Decoder32(_input))?))())?;
let bit_depth = ((|| PResult::Ok((Decoder16(_input))?))())?;
let color_type = ((|| PResult::Ok((Decoder16(_input))?))())?;
let compression_method = ((|| PResult::Ok((Decoder16(_input))?))())?;
let filter_method = ((|| PResult::Ok((Decoder16(_input))?))())?;
let interlace_method = ((|| PResult::Ok((Decoder16(_input))?))())?;
PResult::Ok(main_data { width, height, bit_depth, color_type, compression_method, filter_method, interlace_method })
}

fn Decoder58<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(102, 116, 121, 112) => {
let inner = {
let major_brand = ((|| PResult::Ok((Decoder59(_input))?))())?;
let minor_version = ((|| PResult::Ok((Decoder32(_input))?))())?;
let compatible_brands = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder59(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { major_brand, minor_version, compatible_brands }
};
main_data::ftyp(inner)
},

(102, 114, 101, 101) => {
main_data::free
},

(109, 100, 97, 116) => {
main_data::mdat
},

(109, 101, 116, 97) => {
let field0 = ((|| PResult::Ok((Decoder32(_input))?))())?;
let field1 = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder61(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data::meta(field0, field1)
},

(109, 111, 111, 118) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder62(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::moov(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder59<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = ((|| PResult::Ok((Decoder20(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder20(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder20(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder20(_input))?))())?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder60<'input>(_input: &mut Parser<'input>) -> Result<u64, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
PResult::Ok(((|x: (u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(u64be(x)))(inner))?)
}

fn Decoder61<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(100, 105, 110, 102) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder69(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::dinf(inner)
},

(104, 100, 108, 114) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let predefined = ((|| PResult::Ok((Decoder32(_input))?))())?;
let handler_type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let reserved = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder32(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder32(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder32(_input))?))())?;
(field0, field1, field2)
}))())?;
let name = ((|| PResult::Ok((Decoder67(_input))?))())?;
main_data { version, flags, predefined, handler_type, reserved, name }
};
main_data::hdlr(inner)
},

(112, 105, 116, 109) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let item_ID = ((|| PResult::Ok(match version == 0u8 {
true => {
let inner = (Decoder50(_input))?;
main_data::yes(inner)
},

false => {
let inner = (Decoder32(_input))?;
main_data::no(inner)
}
}))())?;
main_data { version, flags, item_ID }
};
main_data::pitm(inner)
},

(105, 105, 110, 102) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok(match version == 0u8 {
true => {
let inner = (Decoder50(_input))?;
((|x: u16| PResult::Ok(x as u32))(inner))?
},

false => {
(Decoder32(_input))?
}
}))())?;
let item_info_entry = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder71(_input))?);
}
accum
}))())?;
main_data { version, flags, entry_count, item_info_entry }
};
main_data::iinf(inner)
},

(105, 114, 101, 102) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let single_item_reference = ((|| PResult::Ok(match version {
0 => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let from_item_ID = ((|| PResult::Ok((Decoder50(_input))?))())?;
let reference_count = ((|| PResult::Ok((Decoder50(_input))?))())?;
let to_item_ID = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..reference_count {
accum.push((Decoder50(_input))?);
}
accum
}))())?;
main_data { from_item_ID, reference_count, to_item_ID }
}))())?;
_input.end_slice()?;
ret
}))())?;
main_data { size_field, r#type, size, data }
};
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::small(inner)
},

1 => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let from_item_ID = ((|| PResult::Ok((Decoder32(_input))?))())?;
let reference_count = ((|| PResult::Ok((Decoder50(_input))?))())?;
let to_item_ID = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..reference_count {
accum.push((Decoder32(_input))?);
}
accum
}))())?;
main_data { from_item_ID, reference_count, to_item_ID }
}))())?;
_input.end_slice()?;
ret
}))())?;
main_data { size_field, r#type, size, data }
};
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::large(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
main_data { version, flags, single_item_reference }
};
main_data::iref(inner)
},

(105, 108, 111, 99) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let offset_size_length_size = ((|| PResult::Ok((Decoder16(_input))?))())?;
let base_offset_size_index_size = ((|| PResult::Ok((Decoder16(_input))?))())?;
let offset_size = ((|| PResult::Ok(offset_size_length_size >> 4u8))())?;
let length_size = ((|| PResult::Ok(offset_size_length_size & 7u8))())?;
let base_offset_size = ((|| PResult::Ok(base_offset_size_index_size >> 4u8))())?;
let index_size = ((|| PResult::Ok(match version > 0u8 {
true => {
base_offset_size_index_size & 7u8
},

false => {
0u8
}
}))())?;
let item_count = ((|| PResult::Ok(match version < 2u8 {
true => {
let inner = (Decoder50(_input))?;
((|x: u16| PResult::Ok(x as u32))(inner))?
},

false => {
(Decoder32(_input))?
}
}))())?;
let items = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..item_count {
accum.push({
let item_ID = ((|| PResult::Ok(match version < 2u8 {
true => {
let inner = (Decoder50(_input))?;
((|x: u16| PResult::Ok(x as u32))(inner))?
},

false => {
(Decoder32(_input))?
}
}))())?;
let construction_method = ((|| PResult::Ok(match version > 0u8 {
true => {
let inner = (Decoder50(_input))?;
main_data::yes(inner)
},

false => {
main_data::no
}
}))())?;
let data_reference_index = ((|| PResult::Ok((Decoder50(_input))?))())?;
let base_offset = ((|| PResult::Ok(match base_offset_size {
0 => {
0u64
},

4 => {
let inner = (Decoder32(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8 => {
(Decoder60(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let extent_count = ((|| PResult::Ok((Decoder50(_input))?))())?;
let extents = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..extent_count {
accum.push({
let extent_index = ((|| PResult::Ok(match index_size {
0 => {
0u64
},

4 => {
let inner = (Decoder32(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8 => {
(Decoder60(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let extent_offset = ((|| PResult::Ok(match offset_size {
0 => {
0u64
},

4 => {
let inner = (Decoder32(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8 => {
(Decoder60(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let extent_length = ((|| PResult::Ok(match length_size {
0 => {
0u64
},

4 => {
let inner = (Decoder32(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8 => {
(Decoder60(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
main_data { extent_index, extent_offset, extent_length }
});
}
accum
}))())?;
main_data { item_ID, construction_method, data_reference_index, base_offset, extent_count, extents }
});
}
accum
}))())?;
main_data { version, flags, offset_size_length_size, base_offset_size_index_size, offset_size, length_size, base_offset_size, index_size, item_count, items }
};
main_data::iloc(inner)
},

(105, 108, 115, 116) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder72(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::ilst(inner)
},

(105, 100, 97, 116) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::idat(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder62<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(109, 118, 104, 100) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let fields = ((|| PResult::Ok(match version {
0 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder32(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder32(_input))?))())?;
let timescale = ((|| PResult::Ok((Decoder32(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { creation_time, modification_time, timescale, duration }
};
main_data::version0(inner)
},

1 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder60(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder60(_input))?))())?;
let timescale = ((|| PResult::Ok((Decoder32(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder60(_input))?))())?;
main_data { creation_time, modification_time, timescale, duration }
};
main_data::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let rate = ((|| PResult::Ok((Decoder32(_input))?))())?;
let volume = ((|| PResult::Ok((Decoder50(_input))?))())?;
let reserved1 = ((|| PResult::Ok((Decoder50(_input))?))())?;
let reserved2 = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder32(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder32(_input))?))())?;
(field0, field1)
}))())?;
let matrix = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..9u8 {
accum.push((Decoder32(_input))?);
}
accum
}))())?;
let pre_defined = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..6u8 {
accum.push((Decoder32(_input))?);
}
accum
}))())?;
let next_track_ID = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { version, flags, fields, rate, volume, reserved1, reserved2, matrix, pre_defined, next_track_ID }
};
main_data::mvhd(inner)
},

(116, 114, 97, 107) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder63(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::trak(inner)
},

(117, 100, 116, 97) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder64(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::udta(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder63<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(116, 107, 104, 100) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let fields = ((|| PResult::Ok(match version {
0 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder32(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder32(_input))?))())?;
let track_ID = ((|| PResult::Ok((Decoder32(_input))?))())?;
let reserved = ((|| PResult::Ok((Decoder32(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { creation_time, modification_time, track_ID, reserved, duration }
};
main_data::version0(inner)
},

1 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder60(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder60(_input))?))())?;
let track_ID = ((|| PResult::Ok((Decoder32(_input))?))())?;
let reserved = ((|| PResult::Ok((Decoder32(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder60(_input))?))())?;
main_data { creation_time, modification_time, track_ID, reserved, duration }
};
main_data::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let reserved2 = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder32(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder32(_input))?))())?;
(field0, field1)
}))())?;
let layer = ((|| PResult::Ok((Decoder50(_input))?))())?;
let alternate_group = ((|| PResult::Ok((Decoder50(_input))?))())?;
let volume = ((|| PResult::Ok((Decoder50(_input))?))())?;
let reserved1 = ((|| PResult::Ok((Decoder50(_input))?))())?;
let matrix = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..9u8 {
accum.push((Decoder32(_input))?);
}
accum
}))())?;
let width = ((|| PResult::Ok((Decoder32(_input))?))())?;
let height = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { version, flags, fields, reserved2, layer, alternate_group, volume, reserved1, matrix, width, height }
};
main_data::tkhd(inner)
},

(101, 100, 116, 115) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder65(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::edts(inner)
},

(109, 100, 105, 97) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder66(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::mdia(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder64<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(109, 101, 116, 97) => {
let field0 = ((|| PResult::Ok((Decoder32(_input))?))())?;
let field1 = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder61(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data::meta(field0, field1)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder65<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(101, 108, 115, 116) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let number_of_entries = ((|| PResult::Ok((Decoder32(_input))?))())?;
let edit_list_table = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..number_of_entries {
accum.push({
let track_duration = ((|| PResult::Ok((Decoder32(_input))?))())?;
let media_time = ((|| PResult::Ok((Decoder32(_input))?))())?;
let media_rate = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { track_duration, media_time, media_rate }
});
}
accum
}))())?;
main_data { version, flags, number_of_entries, edit_list_table }
};
main_data::elst(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder66<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(104, 100, 108, 114) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let component_type = ((|| PResult::Ok((Decoder32(_input))?))())?;
let component_subtype = ((|| PResult::Ok((Decoder59(_input))?))())?;
let component_manufacturer = ((|| PResult::Ok((Decoder32(_input))?))())?;
let component_flags = ((|| PResult::Ok((Decoder32(_input))?))())?;
let component_flags_mask = ((|| PResult::Ok((Decoder32(_input))?))())?;
let component_name = ((|| PResult::Ok((Decoder67(_input))?))())?;
main_data { version, flags, component_type, component_subtype, component_manufacturer, component_flags, component_flags_mask, component_name }
};
main_data::hdlr(inner)
},

(109, 100, 104, 100) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let fields = ((|| PResult::Ok(match version {
0 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder32(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder32(_input))?))())?;
let timescale = ((|| PResult::Ok((Decoder32(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { creation_time, modification_time, timescale, duration }
};
main_data::version0(inner)
},

1 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder60(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder60(_input))?))())?;
let timescale = ((|| PResult::Ok((Decoder32(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder60(_input))?))())?;
main_data { creation_time, modification_time, timescale, duration }
};
main_data::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let language = ((|| PResult::Ok((Decoder50(_input))?))())?;
let pre_defined = ((|| PResult::Ok((Decoder50(_input))?))())?;
main_data { version, flags, fields, language, pre_defined }
};
main_data::mdhd(inner)
},

(109, 105, 110, 102) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder68(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::minf(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder67<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder68<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(118, 109, 104, 100) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let graphicsmode = ((|| PResult::Ok((Decoder50(_input))?))())?;
let opcolor = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..3u8 {
accum.push((Decoder50(_input))?);
}
accum
}))())?;
main_data { version, flags, graphicsmode, opcolor }
};
main_data::vmhd(inner)
},

(115, 109, 104, 100) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let balance = ((|| PResult::Ok((Decoder50(_input))?))())?;
let reserved = ((|| PResult::Ok((Decoder50(_input))?))())?;
main_data { version, flags, balance, reserved }
};
main_data::smhd(inner)
},

(100, 105, 110, 102) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder69(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::dinf(inner)
},

(115, 116, 98, 108) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder70(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::stbl(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder69<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(100, 114, 101, 102) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let number_of_entries = ((|| PResult::Ok((Decoder32(_input))?))())?;
let data = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
main_data { size_field, r#type, size, data }
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { version, flags, number_of_entries, data }
};
main_data::dref(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder70<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(115, 116, 115, 100) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let sample_entries = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
main_data { size_field, r#type, size, data }
});
}
accum
}))())?;
main_data { version, flags, entry_count, sample_entries }
};
main_data::stsd(inner)
},

(115, 116, 116, 115) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let sample_entries = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let sample_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let sample_delta = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { sample_count, sample_delta }
});
}
accum
}))())?;
main_data { version, flags, entry_count, sample_entries }
};
main_data::stts(inner)
},

(99, 116, 116, 115) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let sample_entries = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let sample_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let sample_offset = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { sample_count, sample_offset }
});
}
accum
}))())?;
main_data { version, flags, entry_count, sample_entries }
};
main_data::ctts(inner)
},

(115, 116, 115, 115) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let sample_number = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder32(_input))?);
}
accum
}))())?;
main_data { version, flags, entry_count, sample_number }
};
main_data::stss(inner)
},

(115, 116, 115, 99) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let chunk_entries = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let first_chunk = ((|| PResult::Ok((Decoder32(_input))?))())?;
let samples_per_chunk = ((|| PResult::Ok((Decoder32(_input))?))())?;
let sample_description_index = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { first_chunk, samples_per_chunk, sample_description_index }
});
}
accum
}))())?;
main_data { version, flags, entry_count, chunk_entries }
};
main_data::stsc(inner)
},

(115, 116, 115, 122) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let sample_size = ((|| PResult::Ok((Decoder32(_input))?))())?;
let sample_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let entry_size = ((|| PResult::Ok(match sample_size == 0u32 {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..sample_count {
accum.push((Decoder32(_input))?);
}
accum
};
main_data::yes(inner)
},

false => {
main_data::no
}
}))())?;
main_data { version, flags, sample_size, sample_count, entry_size }
};
main_data::stsz(inner)
},

(115, 116, 99, 111) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let chunk_offset = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder32(_input))?);
}
accum
}))())?;
main_data { version, flags, entry_count, chunk_offset }
};
main_data::stco(inner)
},

(99, 111, 54, 52) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let chunk_offset = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder60(_input))?);
}
accum
}))())?;
main_data { version, flags, entry_count, chunk_offset }
};
main_data::co64(inner)
},

(115, 103, 112, 100) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let grouping_type = ((|| PResult::Ok((Decoder32(_input))?))())?;
let default_length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let entry_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let sample_groups = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let description_length = ((|| PResult::Ok(match default_length == 0u32 {
true => {
(Decoder32(_input))?
},

false => {
default_length.clone()
}
}))())?;
let sample_group_entry = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..description_length {
accum.push((Decoder16(_input))?);
}
accum
}))())?;
main_data { description_length, sample_group_entry }
});
}
accum
}))())?;
main_data { version, flags, grouping_type, default_length, entry_count, sample_groups }
};
main_data::sgpd(inner)
},

(115, 98, 103, 112) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let grouping_type = ((|| PResult::Ok((Decoder32(_input))?))())?;
let grouping_type_parameter = ((|| PResult::Ok(match version == 1u8 {
true => {
let inner = (Decoder32(_input))?;
main_data::yes(inner)
},

false => {
main_data::no
}
}))())?;
let entry_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let sample_groups = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let sample_count = ((|| PResult::Ok((Decoder32(_input))?))())?;
let group_description_index = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { sample_count, group_description_index }
});
}
accum
}))())?;
main_data { version, flags, grouping_type, grouping_type_parameter, entry_count, sample_groups }
};
main_data::sbgp(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder71<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(105, 110, 102, 101) => {
let inner = {
let version = ((|| PResult::Ok((Decoder16(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1, field2)
}))())?;
let fields = ((|| PResult::Ok(match version < 2u8 {
true => {
let inner = {
let item_ID = ((|| PResult::Ok((Decoder50(_input))?))())?;
let item_protection_index = ((|| PResult::Ok((Decoder50(_input))?))())?;
let item_name = ((|| PResult::Ok((Decoder74(_input))?))())?;
let content_type = ((|| PResult::Ok((Decoder75(_input))?))())?;
let content_encoding = ((|| PResult::Ok((Decoder76(_input))?))())?;
main_data { item_ID, item_protection_index, item_name, content_type, content_encoding }
};
main_data::yes(inner)
},

false => {
let inner = {
let item_ID = ((|| PResult::Ok(match version == 2u8 {
true => {
let inner = (Decoder50(_input))?;
((|x: u16| PResult::Ok(x as u32))(inner))?
},

false => {
(Decoder32(_input))?
}
}))())?;
let item_protection_index = ((|| PResult::Ok((Decoder50(_input))?))())?;
let item_type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let item_name = ((|| PResult::Ok((Decoder77(_input))?))())?;
let extra_fields = ((|| PResult::Ok(match item_type {
(109, 105, 109, 101) => {
let inner = {
let content_type = ((|| PResult::Ok((Decoder78(_input))?))())?;
main_data { content_type }
};
main_data::mime(inner)
},

(117, 114, 105, 32) => {
let inner = {
let item_uri_type = ((|| PResult::Ok((Decoder78(_input))?))())?;
main_data { item_uri_type }
};
main_data::uri(inner)
},

_ => {
main_data::unknown
}
}))())?;
main_data { item_ID, item_protection_index, item_type, item_name, extra_fields }
};
main_data::no(inner)
}
}))())?;
main_data { version, flags, fields }
};
main_data::infe(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder72<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(169, 116, 111, 111) => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder73(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::tool(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder73<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let size_field = ((|| PResult::Ok((Decoder32(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder59(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder60(_input))?;
((|x: u64| PResult::Ok(x - 16u64))(inner))?
},

_ => {
(size_field - 8u32) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(100, 97, 116, 97) => {
let inner = {
let type_indicator = ((|| PResult::Ok((Decoder32(_input))?))())?;
let locale_indicator = ((|| PResult::Ok((Decoder32(_input))?))())?;
let value = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder20(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { type_indicator, locale_indicator, value }
};
main_data::data(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { size_field, r#type, size, data })
}

fn Decoder74<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder75<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder76<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder77<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder78<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder79<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 216 {
b
} else {
return Err(ParseError::ExcludedBranch(5637435011420551755u64));
}
}))())?;
PResult::Ok(main_data { ff, marker })
}

fn Decoder80<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let initial_segment = ((|| PResult::Ok({
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
let b = _input.read_byte()?;
match b {
224u8 => {
0
},

225u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(4308326862885139660u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(7148064920671428636u64));
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let inner = (Decoder82(_input))?;
main_data::app0(inner)
},

1 => {
let inner = (Decoder83(_input))?;
main_data::app1(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(3642042507085222192u64));
}
}
}))())?;
let segments = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
let b = _input.read_byte()?;
match b {
219u8 => {
0
},

196u8 => {
0
},

204u8 => {
0
},

221u8 => {
0
},

224u8 => {
0
},

225u8 => {
0
},

226u8 => {
0
},

227u8 => {
0
},

228u8 => {
0
},

229u8 => {
0
},

230u8 => {
0
},

231u8 => {
0
},

232u8 => {
0
},

233u8 => {
0
},

234u8 => {
0
},

235u8 => {
0
},

236u8 => {
0
},

237u8 => {
0
},

238u8 => {
0
},

239u8 => {
0
},

254u8 => {
0
},

192u8 => {
1
},

193u8 => {
1
},

194u8 => {
1
},

195u8 => {
1
},

197u8 => {
1
},

198u8 => {
1
},

199u8 => {
1
},

201u8 => {
1
},

202u8 => {
1
},

203u8 => {
1
},

205u8 => {
1
},

206u8 => {
1
},

207u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2627803341941537249u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(3984559770787002987u64));
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder84(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let header = ((|| PResult::Ok((Decoder85(_input))?))())?;
let scan = ((|| PResult::Ok((Decoder86(_input))?))())?;
let dnl = ((|| PResult::Ok({
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
let b = _input.read_byte()?;
match b {
220u8 => {
0
},

217u8 => {
1
},

218u8 => {
1
},

219u8 => {
1
},

196u8 => {
1
},

204u8 => {
1
},

221u8 => {
1
},

224u8 => {
1
},

225u8 => {
1
},

226u8 => {
1
},

227u8 => {
1
},

228u8 => {
1
},

229u8 => {
1
},

230u8 => {
1
},

231u8 => {
1
},

232u8 => {
1
},

233u8 => {
1
},

234u8 => {
1
},

235u8 => {
1
},

236u8 => {
1
},

237u8 => {
1
},

238u8 => {
1
},

239u8 => {
1
},

254u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2381939729225554952u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(5771732241052508004u64));
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let inner = (Decoder87(_input))?;
main_data::some(inner)
},

1 => {
main_data::none
},

_ => {
return Err(ParseError::ExcludedBranch(11678103101816798445u64));
}
}
}))())?;
let scans = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
let b = _input.read_byte()?;
match b {
218u8 => {
0
},

219u8 => {
0
},

196u8 => {
0
},

204u8 => {
0
},

221u8 => {
0
},

224u8 => {
0
},

225u8 => {
0
},

226u8 => {
0
},

227u8 => {
0
},

228u8 => {
0
},

229u8 => {
0
},

230u8 => {
0
},

231u8 => {
0
},

232u8 => {
0
},

233u8 => {
0
},

234u8 => {
0
},

235u8 => {
0
},

236u8 => {
0
},

237u8 => {
0
},

238u8 => {
0
},

239u8 => {
0
},

254u8 => {
0
},

217u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(18361368374853160051u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(12701987380979683068u64));
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder88(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(main_data { initial_segment, segments, header, scan, dnl, scans })
}

fn Decoder81<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 217 {
b
} else {
return Err(ParseError::ExcludedBranch(16574347298383600551u64));
}
}))())?;
PResult::Ok(main_data { ff, marker })
}

fn Decoder82<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 224 {
b
} else {
return Err(ParseError::ExcludedBranch(5346911683359312959u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder148(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder83<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 225 {
b
} else {
return Err(ParseError::ExcludedBranch(301524255299452508u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder142(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder84<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
let b = _input.read_byte()?;
match b {
219u8 => {
0
},

196u8 => {
1
},

204u8 => {
2
},

221u8 => {
3
},

224u8 => {
4
},

225u8 => {
5
},

226u8 => {
6
},

227u8 => {
7
},

228u8 => {
8
},

229u8 => {
9
},

230u8 => {
10
},

231u8 => {
11
},

232u8 => {
12
},

233u8 => {
13
},

234u8 => {
14
},

235u8 => {
15
},

236u8 => {
16
},

237u8 => {
17
},

238u8 => {
18
},

239u8 => {
19
},

254u8 => {
20
},

_ => {
return Err(ParseError::ExcludedBranch(6831883527687906764u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(17358231491816636887u64));
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = (Decoder119(_input))?;
main_data::dqt(inner)
},

1 => {
let inner = (Decoder120(_input))?;
main_data::dht(inner)
},

2 => {
let inner = (Decoder121(_input))?;
main_data::dac(inner)
},

3 => {
let inner = (Decoder122(_input))?;
main_data::dri(inner)
},

4 => {
let inner = (Decoder82(_input))?;
main_data::app0(inner)
},

5 => {
let inner = (Decoder83(_input))?;
main_data::app1(inner)
},

6 => {
let inner = (Decoder123(_input))?;
main_data::app2(inner)
},

7 => {
let inner = (Decoder124(_input))?;
main_data::app3(inner)
},

8 => {
let inner = (Decoder125(_input))?;
main_data::app4(inner)
},

9 => {
let inner = (Decoder126(_input))?;
main_data::app5(inner)
},

10 => {
let inner = (Decoder127(_input))?;
main_data::app6(inner)
},

11 => {
let inner = (Decoder128(_input))?;
main_data::app7(inner)
},

12 => {
let inner = (Decoder129(_input))?;
main_data::app8(inner)
},

13 => {
let inner = (Decoder130(_input))?;
main_data::app9(inner)
},

14 => {
let inner = (Decoder131(_input))?;
main_data::app10(inner)
},

15 => {
let inner = (Decoder132(_input))?;
main_data::app11(inner)
},

16 => {
let inner = (Decoder133(_input))?;
main_data::app12(inner)
},

17 => {
let inner = (Decoder134(_input))?;
main_data::app13(inner)
},

18 => {
let inner = (Decoder135(_input))?;
main_data::app14(inner)
},

19 => {
let inner = (Decoder136(_input))?;
main_data::app15(inner)
},

20 => {
let inner = (Decoder137(_input))?;
main_data::com(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(5858366816005674364u64));
}
})
}

fn Decoder85<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
let b = _input.read_byte()?;
match b {
192u8 => {
0
},

193u8 => {
1
},

194u8 => {
2
},

195u8 => {
3
},

197u8 => {
4
},

198u8 => {
5
},

199u8 => {
6
},

201u8 => {
7
},

202u8 => {
8
},

203u8 => {
9
},

205u8 => {
10
},

206u8 => {
11
},

207u8 => {
12
},

_ => {
return Err(ParseError::ExcludedBranch(6713649261753762975u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(5127673444229506389u64));
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = (Decoder104(_input))?;
main_data::sof0(inner)
},

1 => {
let inner = (Decoder105(_input))?;
main_data::sof1(inner)
},

2 => {
let inner = (Decoder106(_input))?;
main_data::sof2(inner)
},

3 => {
let inner = (Decoder107(_input))?;
main_data::sof3(inner)
},

4 => {
let inner = (Decoder108(_input))?;
main_data::sof5(inner)
},

5 => {
let inner = (Decoder109(_input))?;
main_data::sof6(inner)
},

6 => {
let inner = (Decoder110(_input))?;
main_data::sof7(inner)
},

7 => {
let inner = (Decoder111(_input))?;
main_data::sof9(inner)
},

8 => {
let inner = (Decoder112(_input))?;
main_data::sof10(inner)
},

9 => {
let inner = (Decoder113(_input))?;
main_data::sof11(inner)
},

10 => {
let inner = (Decoder114(_input))?;
main_data::sof13(inner)
},

11 => {
let inner = (Decoder115(_input))?;
main_data::sof14(inner)
},

12 => {
let inner = (Decoder116(_input))?;
main_data::sof15(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(13734934310846663046u64));
}
})
}

fn Decoder86<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let segments = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
let b = _input.read_byte()?;
match b {
219u8 => {
0
},

196u8 => {
0
},

204u8 => {
0
},

221u8 => {
0
},

224u8 => {
0
},

225u8 => {
0
},

226u8 => {
0
},

227u8 => {
0
},

228u8 => {
0
},

229u8 => {
0
},

230u8 => {
0
},

231u8 => {
0
},

232u8 => {
0
},

233u8 => {
0
},

234u8 => {
0
},

235u8 => {
0
},

236u8 => {
0
},

237u8 => {
0
},

238u8 => {
0
},

239u8 => {
0
},

254u8 => {
0
},

218u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(9981528058996288466u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(12045452821827788867u64));
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder84(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let sos = ((|| PResult::Ok((Decoder89(_input))?))())?;
let data = ((|| PResult::Ok((Decoder103(_input))?))())?;
PResult::Ok(main_data { segments, sos, data })
}

fn Decoder87<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 220 {
b
} else {
return Err(ParseError::ExcludedBranch(2912073318189654678u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder102(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder88<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let segments = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
let b = _input.read_byte()?;
match b {
219u8 => {
0
},

196u8 => {
0
},

204u8 => {
0
},

221u8 => {
0
},

224u8 => {
0
},

225u8 => {
0
},

226u8 => {
0
},

227u8 => {
0
},

228u8 => {
0
},

229u8 => {
0
},

230u8 => {
0
},

231u8 => {
0
},

232u8 => {
0
},

233u8 => {
0
},

234u8 => {
0
},

235u8 => {
0
},

236u8 => {
0
},

237u8 => {
0
},

238u8 => {
0
},

239u8 => {
0
},

254u8 => {
0
},

218u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(9981528058996288466u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(12045452821827788867u64));
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder84(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let sos = ((|| PResult::Ok((Decoder89(_input))?))())?;
let data = ((|| PResult::Ok((Decoder90(_input))?))())?;
PResult::Ok(main_data { segments, sos, data })
}

fn Decoder89<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 218 {
b
} else {
return Err(ParseError::ExcludedBranch(5297104498937034880u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder100(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder90<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let scan_data = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 255) => {
0
},

255u8 => {
let b = _input.read_byte()?;
match b {
0u8 => {
0
},

208u8 => {
0
},

209u8 => {
0
},

210u8 => {
0
},

211u8 => {
0
},

212u8 => {
0
},

213u8 => {
0
},

214u8 => {
0
},

215u8 => {
0
},

217u8 => {
1
},

218u8 => {
1
},

219u8 => {
1
},

196u8 => {
1
},

204u8 => {
1
},

221u8 => {
1
},

224u8 => {
1
},

225u8 => {
1
},

226u8 => {
1
},

227u8 => {
1
},

228u8 => {
1
},

229u8 => {
1
},

230u8 => {
1
},

231u8 => {
1
},

232u8 => {
1
},

233u8 => {
1
},

234u8 => {
1
},

235u8 => {
1
},

236u8 => {
1
},

237u8 => {
1
},

238u8 => {
1
},

239u8 => {
1
},

254u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(9445433320207076674u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14334550274612271578u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 255) => {
0
},

255u8 => {
let b = _input.read_byte()?;
match b {
0u8 => {
0
},

208u8 => {
1
},

209u8 => {
2
},

210u8 => {
3
},

211u8 => {
4
},

212u8 => {
5
},

213u8 => {
6
},

214u8 => {
7
},

215u8 => {
8
},

_ => {
return Err(ParseError::ExcludedBranch(2047945967620228231u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3966792236320797464u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let inner = (Decoder91(_input))?;
main_data::mcu(inner)
},

1 => {
let inner = (Decoder92(_input))?;
main_data::rst0(inner)
},

2 => {
let inner = (Decoder93(_input))?;
main_data::rst1(inner)
},

3 => {
let inner = (Decoder94(_input))?;
main_data::rst2(inner)
},

4 => {
let inner = (Decoder95(_input))?;
main_data::rst3(inner)
},

5 => {
let inner = (Decoder96(_input))?;
main_data::rst4(inner)
},

6 => {
let inner = (Decoder97(_input))?;
main_data::rst5(inner)
},

7 => {
let inner = (Decoder98(_input))?;
main_data::rst6(inner)
},

8 => {
let inner = (Decoder99(_input))?;
main_data::rst7(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(16335009692206494675u64));
}
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let scan_data_stream = ((|| PResult::Ok((try_flat_map_vec(scan_data.iter().cloned(), |x: main_data| PResult::Ok(match x {
main_data::mcu(v) => {
[v.clone()].to_vec()
},

main_data::rst0(..) => {
[].to_vec()
},

main_data::rst1(..) => {
[].to_vec()
},

main_data::rst2(..) => {
[].to_vec()
},

main_data::rst3(..) => {
[].to_vec()
},

main_data::rst4(..) => {
[].to_vec()
},

main_data::rst5(..) => {
[].to_vec()
},

main_data::rst6(..) => {
[].to_vec()
},

main_data::rst7(..) => {
[].to_vec()
}
})))?))())?;
PResult::Ok(main_data { scan_data, scan_data_stream })
}

fn Decoder91<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 255) => {
0
},

255u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(5885932633650161961u64));
}
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let b = _input.read_byte()?;
if b != 255 {
b
} else {
return Err(ParseError::ExcludedBranch(4029318947293129738u64));
}
},

1 => {
let inner = {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
(field0, field1)
};
((|_: (u8, u8)| PResult::Ok(255u8))(inner))?
},

_ => {
return Err(ParseError::ExcludedBranch(4297833600800538456u64));
}
})
}

fn Decoder92<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 208 {
b
} else {
return Err(ParseError::ExcludedBranch(5421268784727520761u64));
}
}))())?;
PResult::Ok(main_data { ff, marker })
}

fn Decoder93<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 209 {
b
} else {
return Err(ParseError::ExcludedBranch(10069632627653602280u64));
}
}))())?;
PResult::Ok(main_data { ff, marker })
}

fn Decoder94<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 210 {
b
} else {
return Err(ParseError::ExcludedBranch(7941505592535629367u64));
}
}))())?;
PResult::Ok(main_data { ff, marker })
}

fn Decoder95<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 211 {
b
} else {
return Err(ParseError::ExcludedBranch(4842764822111760355u64));
}
}))())?;
PResult::Ok(main_data { ff, marker })
}

fn Decoder96<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 212 {
b
} else {
return Err(ParseError::ExcludedBranch(172561454190383201u64));
}
}))())?;
PResult::Ok(main_data { ff, marker })
}

fn Decoder97<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 213 {
b
} else {
return Err(ParseError::ExcludedBranch(12052389963453405046u64));
}
}))())?;
PResult::Ok(main_data { ff, marker })
}

fn Decoder98<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 214 {
b
} else {
return Err(ParseError::ExcludedBranch(14545630498792155294u64));
}
}))())?;
PResult::Ok(main_data { ff, marker })
}

fn Decoder99<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 215 {
b
} else {
return Err(ParseError::ExcludedBranch(10573988543901039080u64));
}
}))())?;
PResult::Ok(main_data { ff, marker })
}

fn Decoder100<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let num_image_components = ((|| PResult::Ok((Decoder16(_input))?))())?;
let image_components = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_image_components {
accum.push((Decoder101(_input))?);
}
accum
}))())?;
let start_spectral_selection = ((|| PResult::Ok((Decoder16(_input))?))())?;
let end_spectral_selection = ((|| PResult::Ok((Decoder16(_input))?))())?;
let approximation_bit_position = ((|| PResult::Ok((Decoder16(_input))?))())?;
PResult::Ok(main_data { num_image_components, image_components, start_spectral_selection, end_spectral_selection, approximation_bit_position })
}

fn Decoder101<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let component_selector = ((|| PResult::Ok((Decoder16(_input))?))())?;
let entropy_coding_table_ids = ((|| PResult::Ok((Decoder16(_input))?))())?;
PResult::Ok(main_data { component_selector, entropy_coding_table_ids })
}

fn Decoder102<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let num_lines = ((|| PResult::Ok((Decoder50(_input))?))())?;
PResult::Ok(main_data { num_lines })
}

fn Decoder103<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let scan_data = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 255) => {
0
},

255u8 => {
let b = _input.read_byte()?;
match b {
0u8 => {
0
},

208u8 => {
0
},

209u8 => {
0
},

210u8 => {
0
},

211u8 => {
0
},

212u8 => {
0
},

213u8 => {
0
},

214u8 => {
0
},

215u8 => {
0
},

220u8 => {
1
},

217u8 => {
1
},

218u8 => {
1
},

219u8 => {
1
},

196u8 => {
1
},

204u8 => {
1
},

221u8 => {
1
},

224u8 => {
1
},

225u8 => {
1
},

226u8 => {
1
},

227u8 => {
1
},

228u8 => {
1
},

229u8 => {
1
},

230u8 => {
1
},

231u8 => {
1
},

232u8 => {
1
},

233u8 => {
1
},

234u8 => {
1
},

235u8 => {
1
},

236u8 => {
1
},

237u8 => {
1
},

238u8 => {
1
},

239u8 => {
1
},

254u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(9741508811552252074u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4565915750535274488u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 255) => {
0
},

255u8 => {
let b = _input.read_byte()?;
match b {
0u8 => {
0
},

208u8 => {
1
},

209u8 => {
2
},

210u8 => {
3
},

211u8 => {
4
},

212u8 => {
5
},

213u8 => {
6
},

214u8 => {
7
},

215u8 => {
8
},

_ => {
return Err(ParseError::ExcludedBranch(2047945967620228231u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3966792236320797464u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let inner = (Decoder91(_input))?;
main_data::mcu(inner)
},

1 => {
let inner = (Decoder92(_input))?;
main_data::rst0(inner)
},

2 => {
let inner = (Decoder93(_input))?;
main_data::rst1(inner)
},

3 => {
let inner = (Decoder94(_input))?;
main_data::rst2(inner)
},

4 => {
let inner = (Decoder95(_input))?;
main_data::rst3(inner)
},

5 => {
let inner = (Decoder96(_input))?;
main_data::rst4(inner)
},

6 => {
let inner = (Decoder97(_input))?;
main_data::rst5(inner)
},

7 => {
let inner = (Decoder98(_input))?;
main_data::rst6(inner)
},

8 => {
let inner = (Decoder99(_input))?;
main_data::rst7(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(16335009692206494675u64));
}
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let scan_data_stream = ((|| PResult::Ok((try_flat_map_vec(scan_data.iter().cloned(), |x: main_data| PResult::Ok(match x {
main_data::mcu(v) => {
[v.clone()].to_vec()
},

main_data::rst0(..) => {
[].to_vec()
},

main_data::rst1(..) => {
[].to_vec()
},

main_data::rst2(..) => {
[].to_vec()
},

main_data::rst3(..) => {
[].to_vec()
},

main_data::rst4(..) => {
[].to_vec()
},

main_data::rst5(..) => {
[].to_vec()
},

main_data::rst6(..) => {
[].to_vec()
},

main_data::rst7(..) => {
[].to_vec()
}
})))?))())?;
PResult::Ok(main_data { scan_data, scan_data_stream })
}

fn Decoder104<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 192 {
b
} else {
return Err(ParseError::ExcludedBranch(8297024098414101254u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder105<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 193 {
b
} else {
return Err(ParseError::ExcludedBranch(8756819601933520429u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder106<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 194 {
b
} else {
return Err(ParseError::ExcludedBranch(11080817261996913520u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder107<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 195 {
b
} else {
return Err(ParseError::ExcludedBranch(12909450577628061793u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder108<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 197 {
b
} else {
return Err(ParseError::ExcludedBranch(5274098556056955310u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder109<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 198 {
b
} else {
return Err(ParseError::ExcludedBranch(5472222913557901985u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder110<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 199 {
b
} else {
return Err(ParseError::ExcludedBranch(935456091642960999u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder111<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 201 {
b
} else {
return Err(ParseError::ExcludedBranch(17091795488609854789u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder112<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 202 {
b
} else {
return Err(ParseError::ExcludedBranch(14420220630934832328u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder113<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 203 {
b
} else {
return Err(ParseError::ExcludedBranch(10502663948806018262u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder114<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 205 {
b
} else {
return Err(ParseError::ExcludedBranch(5170411260421882161u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder115<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 206 {
b
} else {
return Err(ParseError::ExcludedBranch(8862644040087288472u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder116<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 207 {
b
} else {
return Err(ParseError::ExcludedBranch(6282714738219454149u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder117(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder117<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let sample_precision = ((|| PResult::Ok((Decoder16(_input))?))())?;
let num_lines = ((|| PResult::Ok((Decoder50(_input))?))())?;
let num_samples_per_line = ((|| PResult::Ok((Decoder50(_input))?))())?;
let num_image_components = ((|| PResult::Ok((Decoder16(_input))?))())?;
let image_components = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_image_components {
accum.push((Decoder118(_input))?);
}
accum
}))())?;
PResult::Ok(main_data { sample_precision, num_lines, num_samples_per_line, num_image_components, image_components })
}

fn Decoder118<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let id = ((|| PResult::Ok((Decoder16(_input))?))())?;
let sampling_factor = ((|| PResult::Ok((Decoder16(_input))?))())?;
let quantization_table_id = ((|| PResult::Ok((Decoder16(_input))?))())?;
PResult::Ok(main_data { id, sampling_factor, quantization_table_id })
}

fn Decoder119<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 219 {
b
} else {
return Err(ParseError::ExcludedBranch(11201713527929929098u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder141(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder120<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 196 {
b
} else {
return Err(ParseError::ExcludedBranch(13231341950566326183u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder140(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder121<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 204 {
b
} else {
return Err(ParseError::ExcludedBranch(10217556179496943797u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder139(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder122<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 221 {
b
} else {
return Err(ParseError::ExcludedBranch(8814285897505247341u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder138(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder123<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 226 {
b
} else {
return Err(ParseError::ExcludedBranch(12140482413237234104u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder124<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 227 {
b
} else {
return Err(ParseError::ExcludedBranch(2795443158724701367u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder125<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 228 {
b
} else {
return Err(ParseError::ExcludedBranch(3355559118720108211u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder126<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 229 {
b
} else {
return Err(ParseError::ExcludedBranch(14742198720488010940u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder127<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 230 {
b
} else {
return Err(ParseError::ExcludedBranch(6277645557415946825u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder128<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 231 {
b
} else {
return Err(ParseError::ExcludedBranch(2176159342917065583u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder129<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 232 {
b
} else {
return Err(ParseError::ExcludedBranch(6957547562921215229u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder130<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 233 {
b
} else {
return Err(ParseError::ExcludedBranch(3756953894146529854u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder131<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 234 {
b
} else {
return Err(ParseError::ExcludedBranch(12608692552323012024u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder132<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 235 {
b
} else {
return Err(ParseError::ExcludedBranch(2716996167109240019u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder133<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 236 {
b
} else {
return Err(ParseError::ExcludedBranch(6641423197242755780u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder134<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 237 {
b
} else {
return Err(ParseError::ExcludedBranch(4000866269867594892u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder135<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 238 {
b
} else {
return Err(ParseError::ExcludedBranch(7832938568744868798u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder136<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 239 {
b
} else {
return Err(ParseError::ExcludedBranch(52255437925028600u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder137<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let marker = ((|| PResult::Ok({
let ff = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let marker = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 254 {
b
} else {
return Err(ParseError::ExcludedBranch(5705528789532761578u64));
}
}))())?;
main_data { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder50(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (length - 2u16) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(main_data { marker, length, data })
}

fn Decoder138<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let restart_interval = ((|| PResult::Ok((Decoder50(_input))?))())?;
PResult::Ok(main_data { restart_interval })
}

fn Decoder139<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let class_table_id = ((|| PResult::Ok((Decoder16(_input))?))())?;
let value = ((|| PResult::Ok((Decoder16(_input))?))())?;
PResult::Ok(main_data { class_table_id, value })
}

fn Decoder140<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let class_table_id = ((|| PResult::Ok((Decoder16(_input))?))())?;
let num_codes = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..16u8 {
accum.push((Decoder16(_input))?);
}
accum
}))())?;
let values = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(main_data { class_table_id, num_codes, values })
}

fn Decoder141<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let precision_table_id = ((|| PResult::Ok((Decoder16(_input))?))())?;
let elements = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(main_data { precision_table_id, elements })
}

fn Decoder142<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let identifier = ((|| PResult::Ok((Decoder143(_input))?))())?;
let data = ((|| PResult::Ok(match identifier.clone().string.as_slice() {
[69, 120, 105, 102] => {
let inner = (Decoder144(_input))?;
main_data::exif(inner)
},

[104, 116, 116, 112, 58, 47, 47, 110, 115, 46, 97, 100, 111, 98, 101, 46, 99, 111, 109, 47, 120, 97, 112, 47, 49, 46, 48, 47] => {
let inner = (Decoder145(_input))?;
main_data::xmp(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::other(inner)
}
}))())?;
PResult::Ok(main_data { identifier, data })
}

fn Decoder143<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder144<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let padding = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
let exif = ((|| PResult::Ok((Decoder146(_input))?))())?;
PResult::Ok(main_data { padding, exif })
}

fn Decoder145<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let xmp = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(main_data { xmp })
}

fn Decoder146<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let byte_order = ((|| PResult::Ok({
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
73u8 => {
0
},

77u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2568666803637249590u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(17197161005512507961u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(17197161005512507961u64));
}
}))())?;
main_data::le(field0, field1)
},

1 => {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 77 {
b
} else {
return Err(ParseError::ExcludedBranch(1661485880725065159u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 77 {
b
} else {
return Err(ParseError::ExcludedBranch(1661485880725065159u64));
}
}))())?;
main_data::be(field0, field1)
},

_ => {
return Err(ParseError::ExcludedBranch(8662494850867647108u64));
}
}
}))())?;
let magic = ((|| PResult::Ok(match byte_order {
main_data::le(..) => {
(Decoder147(_input))?
},

main_data::be(..) => {
(Decoder50(_input))?
}
}))())?;
let offset = ((|| PResult::Ok(match byte_order {
main_data::le(..) => {
(Decoder23(_input))?
},

main_data::be(..) => {
(Decoder32(_input))?
}
}))())?;
let ifd = ((|| PResult::Ok({
_input.open_peek_context();
_input.advance_by(offset - 8u32)?;
let ret = ((|| PResult::Ok(match byte_order {
main_data::le(..) => {
let num_fields = ((|| PResult::Ok((Decoder147(_input))?))())?;
let fields = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_fields {
accum.push({
let tag = ((|| PResult::Ok((Decoder147(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder147(_input))?))())?;
let length = ((|| PResult::Ok((Decoder23(_input))?))())?;
let offset_or_data = ((|| PResult::Ok((Decoder23(_input))?))())?;
main_data { tag, r#type, length, offset_or_data }
});
}
accum
}))())?;
let next_ifd_offset = ((|| PResult::Ok((Decoder23(_input))?))())?;
let next_ifd = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { num_fields, fields, next_ifd_offset, next_ifd }
},

main_data::be(..) => {
let num_fields = ((|| PResult::Ok((Decoder50(_input))?))())?;
let fields = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_fields {
accum.push({
let tag = ((|| PResult::Ok((Decoder50(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder50(_input))?))())?;
let length = ((|| PResult::Ok((Decoder32(_input))?))())?;
let offset_or_data = ((|| PResult::Ok((Decoder32(_input))?))())?;
main_data { tag, r#type, length, offset_or_data }
});
}
accum
}))())?;
let next_ifd_offset = ((|| PResult::Ok((Decoder32(_input))?))())?;
let next_ifd = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
main_data { num_fields, fields, next_ifd_offset, next_ifd }
}
}))())?;
_input.close_peek_context()?;
ret
}))())?;
PResult::Ok(main_data { byte_order, magic, offset, ifd })
}

fn Decoder147<'input>(_input: &mut Parser<'input>) -> Result<u16, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder16(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder16(_input))?))())?;
(field0, field1)
};
PResult::Ok(((|x: (u8, u8)| PResult::Ok(u16le(x)))(inner))?)
}

fn Decoder148<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let identifier = ((|| PResult::Ok((Decoder149(_input))?))())?;
let data = ((|| PResult::Ok(match identifier.clone().string.as_slice() {
[74, 70, 73, 70] => {
let inner = (Decoder150(_input))?;
main_data::jfif(inner)
},

_ => {
let inner = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
{
let ret = 0;
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder16(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
main_data::other(inner)
}
}))())?;
PResult::Ok(main_data { identifier, data })
}

fn Decoder149<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder150<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let version_major = ((|| PResult::Ok((Decoder16(_input))?))())?;
let version_minor = ((|| PResult::Ok((Decoder16(_input))?))())?;
let density_units = ((|| PResult::Ok((Decoder16(_input))?))())?;
let density_x = ((|| PResult::Ok((Decoder50(_input))?))())?;
let density_y = ((|| PResult::Ok((Decoder50(_input))?))())?;
let thumbnail_width = ((|| PResult::Ok((Decoder16(_input))?))())?;
let thumbnail_height = ((|| PResult::Ok((Decoder16(_input))?))())?;
let thumbnail_pixels = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..thumbnail_height {
accum.push({
let mut accum = Vec::new();
for _ in 0..thumbnail_width {
accum.push((Decoder151(_input))?);
}
accum
});
}
accum
}))())?;
PResult::Ok(main_data { version_major, version_minor, density_units, density_x, density_y, thumbnail_width, thumbnail_height, thumbnail_pixels })
}

fn Decoder151<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let r = ((|| PResult::Ok((Decoder16(_input))?))())?;
let g = ((|| PResult::Ok((Decoder16(_input))?))())?;
let b = ((|| PResult::Ok((Decoder16(_input))?))())?;
PResult::Ok(main_data { r, g, b })
}

fn Decoder152<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let magic = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 31 {
b
} else {
return Err(ParseError::ExcludedBranch(6728817869016996251u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 139 {
b
} else {
return Err(ParseError::ExcludedBranch(12646530475123667541u64));
}
}))())?;
(field0, field1)
}))())?;
let method = ((|| PResult::Ok((Decoder16(_input))?))())?;
let file_flags = ((|| PResult::Ok((Decoder16(_input))?))())?;
let timestamp = ((|| PResult::Ok((Decoder23(_input))?))())?;
let compression_flags = ((|| PResult::Ok((Decoder16(_input))?))())?;
let os_id = ((|| PResult::Ok((Decoder16(_input))?))())?;
PResult::Ok(main_data { magic, method, file_flags, timestamp, compression_flags, os_id })
}

fn Decoder153<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
PResult::Ok((Decoder163(_input))?)
}

fn Decoder154<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let blocks = ((|| PResult::Ok({
let mut accum = Vec::new();
loop {
let elem = (Decoder156(_input))?;
if ((|x: &main_data| PResult::Ok(x.clone().r#final == 1u8))(&elem))? {
accum.push(elem);
break
} else {
accum.push(elem);
}
}
accum
}))())?;
let codes = ((|| PResult::Ok((try_flat_map_vec(blocks.iter().cloned(), |x: main_data| PResult::Ok(match x.clone().data {
main_data::uncompressed(y) => {
y.clone().codes_values
},

main_data::fixed_huffman(y) => {
y.clone().codes_values
},

main_data::dynamic_huffman(y) => {
y.clone().codes_values
}
})))?))())?;
let inflate = ((|| PResult::Ok({
fn inflate(codes: &Vec<main_data>) -> Vec<u8> {
let mut vs = Vec::new();
for code in codes.iter() {
match code {
main_data::literal(v) => {
vs.push(*v);
},

main_data::reference(fields) => {
let length = fields.length as usize;
let distance = fields.distance as usize;
if distance > vs.len() {
panic!();
}
let start = vs.len() - distance;
let range = start..start + length;
extend_from_within_ext(&mut vs, range);
}
}
}
vs
}
inflate(&codes)
}))())?;
PResult::Ok(main_data { blocks, codes, inflate })
}

fn Decoder155<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let crc = ((|| PResult::Ok((Decoder23(_input))?))())?;
let length = ((|| PResult::Ok((Decoder23(_input))?))())?;
PResult::Ok(main_data { crc, length })
}

fn Decoder156<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let r#final = ((|| PResult::Ok((Decoder157(_input))?))())?;
let r#type = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let data = ((|| PResult::Ok(match r#type {
0 => {
let inner = (Decoder158(_input))?;
main_data::uncompressed(inner)
},

1 => {
let inner = (Decoder159(_input))?;
main_data::fixed_huffman(inner)
},

2 => {
let inner = (Decoder160(_input))?;
main_data::dynamic_huffman(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
PResult::Ok(main_data { r#final, r#type, data })
}

fn Decoder157<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(b)
}

fn Decoder158<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let align = ((|| PResult::Ok(_input.skip_align(8)?))())?;
let len = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field10 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field11 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field12 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field13 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field14 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field15 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().15 as u16) << 15u16 | (bits.clone().14 as u16) << 14u16 | (bits.clone().13 as u16) << 13u16 | (bits.clone().12 as u16) << 12u16 | (bits.clone().11 as u16) << 11u16 | (bits.clone().10 as u16) << 10u16 | (bits.clone().9 as u16) << 9u16 | (bits.clone().8 as u16) << 8u16 | (bits.clone().7 as u16) << 7u16 | (bits.clone().6 as u16) << 6u16 | (bits.clone().5 as u16) << 5u16 | (bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
}))())?;
let nlen = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field10 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field11 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field12 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field13 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field14 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field15 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().15 as u16) << 15u16 | (bits.clone().14 as u16) << 14u16 | (bits.clone().13 as u16) << 13u16 | (bits.clone().12 as u16) << 12u16 | (bits.clone().11 as u16) << 11u16 | (bits.clone().10 as u16) << 10u16 | (bits.clone().9 as u16) << 9u16 | (bits.clone().8 as u16) << 8u16 | (bits.clone().7 as u16) << 7u16 | (bits.clone().6 as u16) << 6u16 | (bits.clone().5 as u16) << 5u16 | (bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
}))())?;
let bytes = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..len {
accum.push({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().7 << 7u8 | bits.clone().6 << 6u8 | bits.clone().5 << 5u8 | bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
});
}
accum
}))())?;
let codes_values = ((|| PResult::Ok((try_flat_map_vec(bytes.iter().cloned(), |x: u8| PResult::Ok([main_data::literal(x)].to_vec())))?))())?;
PResult::Ok(main_data { align, len, nlen, bytes, codes_values })
}

fn Decoder159<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let codes = ((|| PResult::Ok({
let format = parse_huffman([8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8].to_vec(), None);
let mut accum = Vec::new();
loop {
let elem = {
let code = ((|| PResult::Ok((format(_input))?))())?;
let extra = ((|| PResult::Ok(match code {
257 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(3u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

258 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(4u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

259 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(5u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

260 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(6u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

261 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(7u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

262 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(8u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

263 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(9u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

264 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(10u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

265 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(11u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

266 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(13u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

267 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(15u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

268 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(17u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

269 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(19u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

270 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(23u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

271 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(27u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

272 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(31u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

273 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(35u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

274 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(43u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

275 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(51u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

276 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(59u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

277 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(67u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

278 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(83u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

279 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(99u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

280 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(115u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

281 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(131u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

282 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(163u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

283 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(195u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

284 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(227u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

285 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(258u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code as u16))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

_ => {
main_data::none
}
}))())?;
main_data { code, extra }
};
if ((|x: &main_data| PResult::Ok((x.clone().code as u16) == 256u16))(&elem))? {
accum.push(elem);
break
} else {
accum.push(elem);
}
}
accum
}))())?;
let codes_values = ((|| PResult::Ok((try_flat_map_vec(codes.iter().cloned(), |x: main_data| PResult::Ok(match x.clone().code {
256 => {
[].to_vec()
},

257 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

258 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

259 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

260 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

261 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

262 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

263 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

264 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

265 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

266 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

267 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

268 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

269 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

270 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

271 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

272 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

273 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

274 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

275 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

276 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

277 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

278 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

279 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

280 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

281 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

282 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

283 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

284 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

285 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(14530426097121235493u64));
}
}
},

_ => {
[main_data::literal(x.clone().code as u8)].to_vec()
}
})))?))())?;
PResult::Ok(main_data { codes, codes_values })
}

fn Decoder160<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let hlit = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let hdist = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let hclen = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let code_length_alphabet_code_lengths = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..hclen + 4u8 {
accum.push({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
});
}
accum
}))())?;
let literal_length_distance_alphabet_code_lengths = ((|| PResult::Ok({
let code_length_alphabet_format = parse_huffman(code_length_alphabet_code_lengths.clone(), Some([16u8, 17u8, 18u8, 0u8, 8u8, 7u8, 9u8, 6u8, 10u8, 5u8, 11u8, 4u8, 12u8, 3u8, 13u8, 2u8, 14u8, 1u8, 15u8].to_vec()));
let mut accum = Vec::new();
loop {
let elem = {
let code = ((|| PResult::Ok((code_length_alphabet_format(_input))?))())?;
let extra = ((|| PResult::Ok(match code as u8 {
16 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
},

17 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
},

18 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().6 << 6u8 | bits.clone().5 << 5u8 | bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
},

_ => {
0u8
}
}))())?;
main_data { code, extra }
};
accum.push(elem);
if ((|y: &Vec<main_data>| PResult::Ok((((try_fold_map_curried(y.iter().cloned(), {
();
deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths_denest_seq::none
}, |x: (deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths_denest_seq, main_data)| PResult::Ok(match x.clone().1.code as u8 {
16 => {
(x.clone().0, dup32((x.clone().1.extra + 3u8) as u32, match x.clone().0 {
deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths_denest_seq::some(y) => {
y.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(3187827975347532847u64));
}
}))
},

17 => {
(x.clone().0, dup32((x.clone().1.extra + 3u8) as u32, 0u8))
},

18 => {
(x.clone().0, dup32((x.clone().1.extra + 11u8) as u32, 0u8))
},

v => {
(deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths_denest_seq::some(v), [v.clone()].to_vec())
}
})))?.len()) as u32) >= ((hlit + hdist) as u32) + 258u32))(&accum))? {
break
}
}
accum
}))())?;
let literal_length_distance_alphabet_code_lengths_value = ((|| PResult::Ok((try_fold_map_curried(literal_length_distance_alphabet_code_lengths.iter().cloned(), {
();
deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths_denest_seq::none
}, |x: (deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths_denest_seq, main_data)| PResult::Ok(match x.clone().1.code as u8 {
16 => {
(x.clone().0, dup32((x.clone().1.extra + 3u8) as u32, match x.clone().0 {
deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths_denest_seq::some(y) => {
y.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(3187827975347532847u64));
}
}))
},

17 => {
(x.clone().0, dup32((x.clone().1.extra + 3u8) as u32, 0u8))
},

18 => {
(x.clone().0, dup32((x.clone().1.extra + 11u8) as u32, 0u8))
},

v => {
(deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths_denest_seq::some(v), [v.clone()].to_vec())
}
})))?))())?;
let literal_length_alphabet_code_lengths_value = ((|| PResult::Ok({
let ix = 0u32 as usize;
Vec::from(&literal_length_distance_alphabet_code_lengths_value[ix..(ix + (((hlit as u32) + 257u32) as usize))])
}))())?;
let distance_alphabet_code_lengths_value = ((|| PResult::Ok({
let ix = ((hlit as u32) + 257u32) as usize;
Vec::from(&literal_length_distance_alphabet_code_lengths_value[ix..(ix + (((hdist as u32) + 1u32) as usize))])
}))())?;
let codes = ((|| PResult::Ok({
let distance_alphabet_format = parse_huffman(distance_alphabet_code_lengths_value.clone(), None);
let literal_length_alphabet_format = parse_huffman(literal_length_alphabet_code_lengths_value.clone(), None);
let mut accum = Vec::new();
loop {
let elem = {
let code = ((|| PResult::Ok((literal_length_alphabet_format(_input))?))())?;
let extra = ((|| PResult::Ok(match code {
257 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(3u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

258 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(4u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

259 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(5u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

260 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(6u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

261 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(7u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

262 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(8u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

263 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(9u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

264 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(10u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

265 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(11u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

266 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(13u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

267 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(15u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

268 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(17u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

269 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(19u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

270 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(23u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

271 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(27u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

272 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(31u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

273 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(35u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

274 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(43u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

275 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(51u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

276 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(59u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

277 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(67u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

278 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(83u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

279 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(99u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

280 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(115u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

281 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(131u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

282 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(163u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

283 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(195u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

284 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.clone().4 << 4u8 | bits.clone().3 << 3u8 | bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0))(inner))?
}))())?;
let length = ((|| PResult::Ok(227u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

285 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(258u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder161(_input, distance_code.clone()))?))())?;
main_data { length_extra_bits, length, distance_code, distance_record }
};
main_data::some(inner)
},

_ => {
main_data::none
}
}))())?;
main_data { code, extra }
};
if ((|x: &main_data| PResult::Ok((x.clone().code as u16) == 256u16))(&elem))? {
accum.push(elem);
break
} else {
accum.push(elem);
}
}
accum
}))())?;
let codes_values = ((|| PResult::Ok((try_flat_map_vec(codes.iter().cloned(), |x: main_data| PResult::Ok(match x.clone().code {
256 => {
[].to_vec()
},

257 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

258 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

259 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

260 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

261 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

262 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

263 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

264 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

265 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

266 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

267 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

268 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

269 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

270 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

271 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

272 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

273 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

274 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

275 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

276 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

277 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

278 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

279 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

280 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

281 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

282 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

283 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

284 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

285 => {
match x.clone().extra {
main_data::some(rec) => {
[main_data::reference(main_data { length: rec.clone().length, distance: rec.clone().distance_record.distance })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(11706052143427407410u64));
}
}
},

_ => {
[main_data::literal(x.clone().code as u8)].to_vec()
}
})))?))())?;
PResult::Ok(main_data { hlit, hdist, hclen, code_length_alphabet_code_lengths, literal_length_distance_alphabet_code_lengths, literal_length_distance_alphabet_code_lengths_value, literal_length_alphabet_code_lengths_value, distance_alphabet_code_lengths_value, codes, codes_values })
}

fn Decoder161<'input>(_input: &mut Parser<'input>, distance_code: u16) -> Result<main_data, ParseError> {
PResult::Ok(match distance_code as u8 {
0 => {
(Decoder162(_input, 0u8, 1u16))?
},

1 => {
(Decoder162(_input, 0u8, 2u16))?
},

2 => {
(Decoder162(_input, 0u8, 3u16))?
},

3 => {
(Decoder162(_input, 0u8, 4u16))?
},

4 => {
(Decoder162(_input, 1u8, 5u16))?
},

5 => {
(Decoder162(_input, 1u8, 7u16))?
},

6 => {
(Decoder162(_input, 2u8, 9u16))?
},

7 => {
(Decoder162(_input, 2u8, 13u16))?
},

8 => {
(Decoder162(_input, 3u8, 17u16))?
},

9 => {
(Decoder162(_input, 3u8, 25u16))?
},

10 => {
(Decoder162(_input, 4u8, 33u16))?
},

11 => {
(Decoder162(_input, 4u8, 49u16))?
},

12 => {
(Decoder162(_input, 5u8, 65u16))?
},

13 => {
(Decoder162(_input, 5u8, 97u16))?
},

14 => {
(Decoder162(_input, 6u8, 129u16))?
},

15 => {
(Decoder162(_input, 6u8, 193u16))?
},

16 => {
(Decoder162(_input, 7u8, 257u16))?
},

17 => {
(Decoder162(_input, 7u8, 385u16))?
},

18 => {
(Decoder162(_input, 8u8, 513u16))?
},

19 => {
(Decoder162(_input, 8u8, 769u16))?
},

20 => {
(Decoder162(_input, 9u8, 1025u16))?
},

21 => {
(Decoder162(_input, 9u8, 1537u16))?
},

22 => {
(Decoder162(_input, 10u8, 2049u16))?
},

23 => {
(Decoder162(_input, 10u8, 3073u16))?
},

24 => {
(Decoder162(_input, 11u8, 4097u16))?
},

25 => {
(Decoder162(_input, 11u8, 6145u16))?
},

26 => {
(Decoder162(_input, 12u8, 8193u16))?
},

27 => {
(Decoder162(_input, 12u8, 12289u16))?
},

28 => {
(Decoder162(_input, 13u8, 16385u16))?
},

29 => {
(Decoder162(_input, 13u8, 24577u16))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

fn Decoder162<'input>(_input: &mut Parser<'input>, extra_bits: u8, start: u16) -> Result<main_data, ParseError> {
let distance_extra_bits = ((|| PResult::Ok(match extra_bits {
0 => {
0u16
},

1 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.clone().0 as u16))(inner))?
},

2 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok((bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

3 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok((bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

4 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok((bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

5 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

6 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5)
};
((|bits: (u8, u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().5 as u16) << 5u16 | (bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

7 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().6 as u16) << 6u16 | (bits.clone().5 as u16) << 5u16 | (bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().7 as u16) << 7u16 | (bits.clone().6 as u16) << 6u16 | (bits.clone().5 as u16) << 5u16 | (bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

9 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().8 as u16) << 8u16 | (bits.clone().7 as u16) << 7u16 | (bits.clone().6 as u16) << 6u16 | (bits.clone().5 as u16) << 5u16 | (bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

10 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().9 as u16) << 9u16 | (bits.clone().8 as u16) << 8u16 | (bits.clone().7 as u16) << 7u16 | (bits.clone().6 as u16) << 6u16 | (bits.clone().5 as u16) << 5u16 | (bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

11 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field10 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().10 as u16) << 10u16 | (bits.clone().9 as u16) << 9u16 | (bits.clone().8 as u16) << 8u16 | (bits.clone().7 as u16) << 7u16 | (bits.clone().6 as u16) << 6u16 | (bits.clone().5 as u16) << 5u16 | (bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

12 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field10 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field11 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().11 as u16) << 11u16 | (bits.clone().10 as u16) << 10u16 | (bits.clone().9 as u16) << 9u16 | (bits.clone().8 as u16) << 8u16 | (bits.clone().7 as u16) << 7u16 | (bits.clone().6 as u16) << 6u16 | (bits.clone().5 as u16) << 5u16 | (bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

13 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field10 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field11 = ((|| PResult::Ok((Decoder157(_input))?))())?;
let field12 = ((|| PResult::Ok((Decoder157(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok((bits.clone().12 as u16) << 12u16 | (bits.clone().11 as u16) << 11u16 | (bits.clone().10 as u16) << 10u16 | (bits.clone().9 as u16) << 9u16 | (bits.clone().8 as u16) << 8u16 | (bits.clone().7 as u16) << 7u16 | (bits.clone().6 as u16) << 6u16 | (bits.clone().5 as u16) << 5u16 | (bits.clone().4 as u16) << 4u16 | (bits.clone().3 as u16) << 3u16 | (bits.clone().2 as u16) << 2u16 | (bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16)))(inner))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let distance = ((|| PResult::Ok(start + distance_extra_bits))())?;
PResult::Ok(main_data { distance_extra_bits, distance })
}

fn Decoder163<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let string = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
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
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let null = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
PResult::Ok(main_data { string, null })
}

fn Decoder164<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let signature = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 71 {
b
} else {
return Err(ParseError::ExcludedBranch(690880023569680479u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(17197161005512507961u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(14049552398800766371u64));
}
}))())?;
(field0, field1, field2)
}))())?;
let version = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..3u8 {
accum.push((Decoder20(_input))?);
}
accum
}))())?;
PResult::Ok(main_data { signature, version })
}

fn Decoder165<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let descriptor = ((|| PResult::Ok((Decoder181(_input))?))())?;
let global_color_table = ((|| PResult::Ok(match descriptor.clone().flags & 128u8 != 0u8 {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..2u8 << (descriptor.clone().flags & 7u8) {
accum.push((Decoder179(_input))?);
}
accum
};
main_data::yes(inner)
},

false => {
main_data::no
}
}))())?;
PResult::Ok(main_data { descriptor, global_color_table })
}

fn Decoder166<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
33u8 => {
let b = _input.read_byte()?;
match b {
249u8 => {
0
},

1u8 => {
0
},

255u8 => {
1
},

254u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(5009412587336832230u64));
}
}
},

44u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(3181733884495644307u64));
}
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = (Decoder168(_input))?;
main_data::graphic_block(inner)
},

1 => {
let inner = (Decoder169(_input))?;
main_data::special_purpose_block(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(14056621334159770744u64));
}
})
}

fn Decoder167<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let separator = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 59 {
b
} else {
return Err(ParseError::ExcludedBranch(15783818897979407630u64));
}
}))())?;
PResult::Ok(main_data { separator })
}

fn Decoder168<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let graphic_control_extension = ((|| PResult::Ok({
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
33u8 => {
let b = _input.read_byte()?;
match b {
249u8 => {
0
},

1u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(16676828686615745155u64));
}
}
},

44u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(4699571722508458381u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let inner = (Decoder174(_input))?;
main_data::some(inner)
},

1 => {
main_data::none
},

_ => {
return Err(ParseError::ExcludedBranch(15496895076277599409u64));
}
}
}))())?;
let graphic_rendering_block = ((|| PResult::Ok((Decoder175(_input))?))())?;
PResult::Ok(main_data { graphic_control_extension, graphic_rendering_block })
}

fn Decoder169<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 33 {
let b = _input.read_byte()?;
match b {
255u8 => {
0
},

254u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(6088842714593122773u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(8240896963323767603u64));
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = (Decoder170(_input))?;
main_data::application_extension(inner)
},

1 => {
let inner = (Decoder171(_input))?;
main_data::comment_extension(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(7565262198115782210u64));
}
})
}

fn Decoder170<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let separator = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(12638618761954708471u64));
}
}))())?;
let label = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
}))())?;
let block_size = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 11 {
b
} else {
return Err(ParseError::ExcludedBranch(16286797724653440122u64));
}
}))())?;
let identifier = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..8u8 {
accum.push((Decoder16(_input))?);
}
accum
}))())?;
let authentication_code = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..3u8 {
accum.push((Decoder16(_input))?);
}
accum
}))())?;
let application_data = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder172(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let terminator = ((|| PResult::Ok((Decoder173(_input))?))())?;
PResult::Ok(main_data { separator, label, block_size, identifier, authentication_code, application_data, terminator })
}

fn Decoder171<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let separator = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(12638618761954708471u64));
}
}))())?;
let label = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 254 {
b
} else {
return Err(ParseError::ExcludedBranch(5705528789532761578u64));
}
}))())?;
let comment_data = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder172(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let terminator = ((|| PResult::Ok((Decoder173(_input))?))())?;
PResult::Ok(main_data { separator, label, comment_data, terminator })
}

fn Decoder172<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let len_bytes = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b != 0 {
b
} else {
return Err(ParseError::ExcludedBranch(8606461246239977862u64));
}
}))())?;
let data = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..len_bytes {
accum.push((Decoder16(_input))?);
}
accum
}))())?;
PResult::Ok(main_data { len_bytes, data })
}

fn Decoder173<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
})
}

fn Decoder174<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let separator = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(12638618761954708471u64));
}
}))())?;
let label = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 249 {
b
} else {
return Err(ParseError::ExcludedBranch(2007898731924533432u64));
}
}))())?;
let block_size = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 4 {
b
} else {
return Err(ParseError::ExcludedBranch(12797785829236001664u64));
}
}))())?;
let flags = ((|| PResult::Ok((Decoder16(_input))?))())?;
let delay_time = ((|| PResult::Ok((Decoder147(_input))?))())?;
let transparent_color_index = ((|| PResult::Ok((Decoder16(_input))?))())?;
let terminator = ((|| PResult::Ok((Decoder173(_input))?))())?;
PResult::Ok(main_data { separator, label, block_size, flags, delay_time, transparent_color_index, terminator })
}

fn Decoder175<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
44u8 => {
0
},

33u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2513620722346358705u64));
}
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = (Decoder176(_input))?;
main_data::table_based_image(inner)
},

1 => {
let inner = (Decoder177(_input))?;
main_data::plain_text_extension(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(14120387546690436687u64));
}
})
}

fn Decoder176<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let descriptor = ((|| PResult::Ok((Decoder178(_input))?))())?;
let local_color_table = ((|| PResult::Ok(match descriptor.clone().flags & 128u8 != 0u8 {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..2u8 << (descriptor.clone().flags & 7u8) {
accum.push((Decoder179(_input))?);
}
accum
};
main_data::yes(inner)
},

false => {
main_data::no
}
}))())?;
let data = ((|| PResult::Ok((Decoder180(_input))?))())?;
PResult::Ok(main_data { descriptor, local_color_table, data })
}

fn Decoder177<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let separator = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(12638618761954708471u64));
}
}))())?;
let label = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 1 {
b
} else {
return Err(ParseError::ExcludedBranch(2974505448909915409u64));
}
}))())?;
let block_size = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 12 {
b
} else {
return Err(ParseError::ExcludedBranch(15268554964885899593u64));
}
}))())?;
let text_grid_left_position = ((|| PResult::Ok((Decoder147(_input))?))())?;
let text_grid_top_position = ((|| PResult::Ok((Decoder147(_input))?))())?;
let text_grid_width = ((|| PResult::Ok((Decoder147(_input))?))())?;
let text_grid_height = ((|| PResult::Ok((Decoder147(_input))?))())?;
let character_cell_width = ((|| PResult::Ok((Decoder16(_input))?))())?;
let character_cell_height = ((|| PResult::Ok((Decoder16(_input))?))())?;
let text_foreground_color_index = ((|| PResult::Ok((Decoder16(_input))?))())?;
let text_background_color_index = ((|| PResult::Ok((Decoder16(_input))?))())?;
let plain_text_data = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder172(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let terminator = ((|| PResult::Ok((Decoder173(_input))?))())?;
PResult::Ok(main_data { separator, label, block_size, text_grid_left_position, text_grid_top_position, text_grid_width, text_grid_height, character_cell_width, character_cell_height, text_foreground_color_index, text_background_color_index, plain_text_data, terminator })
}

fn Decoder178<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let separator = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 44 {
b
} else {
return Err(ParseError::ExcludedBranch(957865226307229178u64));
}
}))())?;
let image_left_position = ((|| PResult::Ok((Decoder147(_input))?))())?;
let image_top_position = ((|| PResult::Ok((Decoder147(_input))?))())?;
let image_width = ((|| PResult::Ok((Decoder147(_input))?))())?;
let image_height = ((|| PResult::Ok((Decoder147(_input))?))())?;
let flags = ((|| PResult::Ok((Decoder16(_input))?))())?;
PResult::Ok(main_data { separator, image_left_position, image_top_position, image_width, image_height, flags })
}

fn Decoder179<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let r = ((|| PResult::Ok((Decoder16(_input))?))())?;
let g = ((|| PResult::Ok((Decoder16(_input))?))())?;
let b = ((|| PResult::Ok((Decoder16(_input))?))())?;
PResult::Ok(main_data { r, g, b })
}

fn Decoder180<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let lzw_min_code_size = ((|| PResult::Ok((Decoder16(_input))?))())?;
let image_data = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if (tmp != 0) => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(13862338712518612949u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder172(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let terminator = ((|| PResult::Ok((Decoder173(_input))?))())?;
PResult::Ok(main_data { lzw_min_code_size, image_data, terminator })
}

fn Decoder181<'input>(_input: &mut Parser<'input>) -> Result<main_data, ParseError> {
let screen_width = ((|| PResult::Ok((Decoder147(_input))?))())?;
let screen_height = ((|| PResult::Ok((Decoder147(_input))?))())?;
let flags = ((|| PResult::Ok((Decoder16(_input))?))())?;
let bg_color_index = ((|| PResult::Ok((Decoder16(_input))?))())?;
let pixel_aspect_ratio = ((|| PResult::Ok((Decoder16(_input))?))())?;
PResult::Ok(main_data { screen_width, screen_height, flags, bg_color_index, pixel_aspect_ratio })
}

fn Decoder182<'input>(_input: &mut Parser<'input>) -> Result<u32, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
90u8 => {
0
},

83u8 => {
let b = _input.read_byte()?;
match b {
90u8 => {
1
},

83u8 => {
let b = _input.read_byte()?;
match b {
90u8 => {
2
},

83u8 => {
let b = _input.read_byte()?;
match b {
90u8 => {
3
},

83u8 => {
let b = _input.read_byte()?;
match b {
90u8 => {
4
},

83u8 => {
let b = _input.read_byte()?;
match b {
90u8 => {
5
},

83u8 => {
let b = _input.read_byte()?;
match b {
90u8 => {
6
},

83u8 => {
let b = _input.read_byte()?;
match b {
90u8 => {
7
},

83u8 => {
let b = _input.read_byte()?;
match b {
90u8 => {
8
},

83u8 => {
9
},

_ => {
return Err(ParseError::ExcludedBranch(8001216093308031977u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15039885765796097429u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1933468383562631430u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16102628130774122918u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10928719624476144722u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7193796329588642972u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1105552943422416259u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4697947408157727853u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9798043767426199682u64));
}
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(matching_ix == 0, accum.len() >= (0u16 as usize), accum.len() == (9u16 as usize)))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if b == 83 {
b
} else {
return Err(ParseError::ExcludedBranch(16554645260058031671u64));
}
};
accum.push(next_elem);
}
}
accum
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 90 {
b
} else {
return Err(ParseError::ExcludedBranch(2948356503678068618u64));
}
}))())?;
(field0, field1)
};
PResult::Ok(((|x: (Vec<u8>, u8)| PResult::Ok((x.clone().0.len()) as u32))(inner))?)
}

