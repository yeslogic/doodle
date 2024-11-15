#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod codegen_tests;
pub mod api_helper;

use doodle::prelude::*;
use doodle::try_sub;

#[derive(Debug, Clone)]
pub struct elf_header_ident {
magic: (u8, u8, u8, u8),
class: u8,
data: u8,
version: u8,
os_abi: u8,
abi_version: u8,
__pad: Vec<u8>
}

#[derive(Debug, Copy, Clone)]
pub enum elf_types_elf_addr { Addr32(u32), Addr64(u64) }

#[derive(Debug, Copy, Clone)]
pub enum elf_types_elf_off { Off32(u32), Off64(u64) }

#[derive(Debug, Clone)]
pub struct elf_header {
ident: elf_header_ident,
r#type: u16,
machine: u16,
version: u32,
entry: elf_types_elf_addr,
phoff: elf_types_elf_off,
shoff: elf_types_elf_off,
flags: u32,
ehsize: u16,
phentsize: u16,
phnum: u16,
shentsize: u16,
shnum: u16,
shstrndx: u16
}

#[derive(Debug, Copy, Clone)]
pub enum elf_types_elf_full { Full32(u32), Full64(u64) }

#[derive(Debug, Clone)]
pub struct elf_phdr_table {
r#type: u32,
flags64: Option<u32>,
offset: elf_types_elf_off,
vaddr: elf_types_elf_addr,
paddr: elf_types_elf_addr,
filesz: elf_types_elf_full,
memsz: elf_types_elf_full,
flags32: Option<u32>,
align: elf_types_elf_full
}

#[derive(Debug, Clone)]
pub struct elf_shdr_table {
name: u32,
r#type: u32,
flags: elf_types_elf_full,
addr: elf_types_elf_addr,
offset: elf_types_elf_off,
size: elf_types_elf_full,
link: u32,
info: u32,
addralign: elf_types_elf_full,
entsize: elf_types_elf_full
}

#[derive(Debug, Clone)]
pub struct elf_main {
header: elf_header,
__eoh: u64,
program_headers: Option<Vec<elf_phdr_table>>,
section_headers: Option<Vec<elf_shdr_table>>,
sections: Option<Vec<Option<Vec<u8>>>>,
__skip: ()
}

#[derive(Debug, Clone)]
pub struct gif_header {
signature: (u8, u8, u8),
version: Vec<u8>
}

#[derive(Debug, Copy, Clone)]
pub struct gif_logical_screen_descriptor_flags {
table_flag: u8,
color_resolution: u8,
sort_flag: u8,
table_size: u8
}

#[derive(Debug, Clone)]
pub struct gif_logical_screen_descriptor {
screen_width: u16,
screen_height: u16,
flags: gif_logical_screen_descriptor_flags,
bg_color_index: u8,
pixel_aspect_ratio: u8
}

#[derive(Debug, Copy, Clone)]
pub struct png_plte {
r: u8,
g: u8,
b: u8
}

#[derive(Debug, Clone)]
pub struct gif_logical_screen {
descriptor: gif_logical_screen_descriptor,
global_color_table: Option<Vec<png_plte>>
}

#[derive(Debug, Copy, Clone)]
pub struct gif_graphic_control_extension_flags {
reserved: u8,
disposal_method: u8,
user_input_flag: u8,
transparent_color_flag: u8
}

#[derive(Debug, Clone)]
pub struct gif_graphic_control_extension {
separator: u8,
label: u8,
block_size: u8,
flags: gif_graphic_control_extension_flags,
delay_time: u16,
transparent_color_index: u8,
terminator: u8
}

#[derive(Debug, Clone)]
pub enum gif_graphic_block_graphic_control_extension { none, some(gif_graphic_control_extension) }

#[derive(Debug, Clone)]
pub struct gif_subblock {
len_bytes: u8,
data: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct gif_plain_text_extension {
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
plain_text_data: Vec<gif_subblock>,
terminator: u8
}

#[derive(Debug, Copy, Clone)]
pub struct gif_image_descriptor_flags {
table_flag: u8,
interlace_flag: u8,
sort_flag: u8,
reserved: u8,
table_size: u8
}

#[derive(Debug, Clone)]
pub struct gif_image_descriptor {
separator: u8,
image_left_position: u16,
image_top_position: u16,
image_width: u16,
image_height: u16,
flags: gif_image_descriptor_flags
}

#[derive(Debug, Clone)]
pub struct gif_table_based_image_data {
lzw_min_code_size: u8,
image_data: Vec<gif_subblock>,
terminator: u8
}

#[derive(Debug, Clone)]
pub struct gif_table_based_image {
descriptor: gif_image_descriptor,
local_color_table: Option<Vec<png_plte>>,
data: gif_table_based_image_data
}

#[derive(Debug, Clone)]
pub enum gif_graphic_rendering_block { plain_text_extension(gif_plain_text_extension), table_based_image(gif_table_based_image) }

#[derive(Debug, Clone)]
pub struct gif_graphic_block {
graphic_control_extension: gif_graphic_block_graphic_control_extension,
graphic_rendering_block: gif_graphic_rendering_block
}

#[derive(Debug, Clone)]
pub struct gif_application_extension {
separator: u8,
label: u8,
block_size: u8,
identifier: Vec<u8>,
authentication_code: Vec<u8>,
application_data: Vec<gif_subblock>,
terminator: u8
}

#[derive(Debug, Clone)]
pub struct gif_comment_extension {
separator: u8,
label: u8,
comment_data: Vec<gif_subblock>,
terminator: u8
}

#[derive(Debug, Clone)]
pub enum gif_special_purpose_block { application_extension(gif_application_extension), comment_extension(gif_comment_extension) }

#[derive(Debug, Clone)]
pub enum gif_block { graphic_block(gif_graphic_block), special_purpose_block(gif_special_purpose_block) }

#[derive(Debug, Copy, Clone)]
pub struct gif_trailer {
separator: u8
}

#[derive(Debug, Clone)]
pub struct gif_main {
header: gif_header,
logical_screen: gif_logical_screen,
blocks: Vec<gif_block>,
trailer: gif_trailer
}

#[derive(Debug, Copy, Clone)]
pub struct gzip_header_file_flags {
fcomment: bool,
fname: bool,
fextra: bool,
fhcrc: bool,
ftext: bool
}

#[derive(Debug, Clone)]
pub struct gzip_header {
magic: (u8, u8),
method: u8,
file_flags: gzip_header_file_flags,
timestamp: u32,
compression_flags: u8,
os_id: u8
}

#[derive(Debug, Clone)]
pub struct gzip_fextra_subfield {
si1: u8,
si2: u8,
len: u16,
data: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct gzip_fextra {
xlen: u16,
subfields: Vec<gzip_fextra_subfield>
}

#[derive(Debug, Clone)]
pub struct base_asciiz_string {
string: Vec<u8>,
null: u8
}

#[derive(Debug, Clone)]
pub struct gzip_fcomment {
comment: base_asciiz_string
}

#[derive(Debug, Copy, Clone)]
pub struct gzip_fhcrc {
crc: u16
}

#[derive(Debug, Copy, Clone)]
pub struct deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths {
code: u16,
extra: u8
}

#[derive(Debug, Copy, Clone)]
pub struct deflate_distance_record {
distance_extra_bits: u16,
distance: u16
}

#[derive(Debug, Clone)]
pub struct deflate_dynamic_huffman_codes_values {
length_extra_bits: u8,
length: u16,
distance_code: u16,
distance_record: deflate_distance_record
}

#[derive(Debug, Clone)]
pub struct deflate_dynamic_huffman_codes {
code: u16,
extra: Option<deflate_dynamic_huffman_codes_values>
}

#[derive(Debug, Copy, Clone)]
pub struct deflate_main_codes_reference {
length: u16,
distance: u16
}

#[derive(Debug, Clone)]
pub enum deflate_main_codes__dupX1 { literal(u8), reference(deflate_main_codes_reference) }

#[derive(Debug, Clone)]
pub struct deflate_dynamic_huffman {
hlit: u8,
hdist: u8,
hclen: u8,
code_length_alphabet_code_lengths: Vec<u8>,
literal_length_distance_alphabet_code_lengths: Vec<deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths>,
literal_length_distance_alphabet_code_lengths_value: Vec<u8>,
literal_length_alphabet_code_lengths_value: Vec<u8>,
distance_alphabet_code_lengths_value: Vec<u8>,
codes: Vec<deflate_dynamic_huffman_codes>,
codes_values: Vec<deflate_main_codes__dupX1>
}

#[derive(Debug, Clone)]
pub struct deflate_fixed_huffman_codes_values {
length_extra_bits: u8,
length: u16,
distance_code: u8,
distance_record: deflate_distance_record
}

#[derive(Debug, Clone)]
pub struct deflate_fixed_huffman_codes {
code: u16,
extra: Option<deflate_fixed_huffman_codes_values>
}

#[derive(Debug, Clone)]
pub struct deflate_fixed_huffman {
codes: Vec<deflate_fixed_huffman_codes>,
codes_values: Vec<deflate_main_codes__dupX1>
}

#[derive(Debug, Clone)]
pub struct deflate_uncompressed {
align: (),
len: u16,
nlen: u16,
bytes: Vec<u8>,
codes_values: Vec<deflate_main_codes__dupX1>
}

#[derive(Debug, Clone)]
pub enum deflate_main_codes { dynamic_huffman(deflate_dynamic_huffman), fixed_huffman(deflate_fixed_huffman), uncompressed(deflate_uncompressed) }

#[derive(Debug, Clone)]
pub struct deflate_block {
r#final: u8,
r#type: u8,
data: deflate_main_codes
}

#[derive(Debug, Clone)]
pub struct deflate_main {
blocks: Vec<deflate_block>,
codes: Vec<deflate_main_codes__dupX1>,
inflate: Vec<u8>
}

#[derive(Debug, Copy, Clone)]
pub struct gzip_footer {
crc: u32,
length: u32
}

#[derive(Debug, Clone)]
pub struct gzip_main {
header: gzip_header,
fextra: Option<gzip_fextra>,
fname: Option<base_asciiz_string>,
fcomment: Option<gzip_fcomment>,
fhcrc: Option<gzip_fhcrc>,
data: deflate_main,
footer: gzip_footer
}

#[derive(Debug, Copy, Clone)]
pub struct jpeg_eoi {
ff: u8,
marker: u8
}

#[derive(Debug, Clone)]
pub struct jpeg_app0_jfif {
version_major: u8,
version_minor: u8,
density_units: u8,
density_x: u16,
density_y: u16,
thumbnail_width: u8,
thumbnail_height: u8,
thumbnail_pixels: Vec<Vec<png_plte>>
}

#[derive(Debug, Clone)]
pub enum jpeg_app0_data_data { jfif(jpeg_app0_jfif), other(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct jpeg_app0_data {
identifier: base_asciiz_string,
data: jpeg_app0_data_data
}

#[derive(Debug, Clone)]
pub struct jpeg_app0 {
marker: jpeg_eoi,
length: u16,
data: jpeg_app0_data
}

#[derive(Debug, Copy, Clone)]
pub enum tiff_main_byte_order { be(u8, u8), le(u8, u8) }

#[derive(Debug, Copy, Clone)]
pub struct tiff_main_ifd_fields {
tag: u16,
r#type: u16,
length: u32,
offset_or_data: u32
}

#[derive(Debug, Clone)]
pub struct tiff_main_ifd {
num_fields: u16,
fields: Vec<tiff_main_ifd_fields>,
next_ifd_offset: u32,
next_ifd: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct tiff_main {
byte_order: tiff_main_byte_order,
magic: u16,
offset: u32,
ifd: tiff_main_ifd
}

#[derive(Debug, Clone)]
pub struct jpeg_app1_exif {
padding: u8,
exif: tiff_main
}

#[derive(Debug, Clone)]
pub struct jpeg_app1_xmp {
xmp: Vec<u8>
}

#[derive(Debug, Clone)]
pub enum jpeg_app1_data_data { exif(jpeg_app1_exif), other(Vec<u8>), xmp(jpeg_app1_xmp) }

#[derive(Debug, Clone)]
pub struct jpeg_app1_data {
identifier: base_asciiz_string,
data: jpeg_app1_data_data
}

#[derive(Debug, Clone)]
pub struct jpeg_app1 {
marker: jpeg_eoi,
length: u16,
data: jpeg_app1_data
}

#[derive(Debug, Clone)]
pub enum jpeg_frame_initial_segment { app0(jpeg_app0), app1(jpeg_app1) }

#[derive(Debug, Clone)]
pub struct jpeg_com {
marker: jpeg_eoi,
length: u16,
data: Vec<u8>
}

#[derive(Debug, Copy, Clone)]
pub struct jpeg_dac_data_class_table_id {
class: u8,
table_id: u8
}

#[derive(Debug, Clone)]
pub struct jpeg_dac_data {
class_table_id: jpeg_dac_data_class_table_id,
value: u8
}

#[derive(Debug, Clone)]
pub struct jpeg_dac {
marker: jpeg_eoi,
length: u16,
data: jpeg_dac_data
}

#[derive(Debug, Clone)]
pub struct jpeg_dht_data {
class_table_id: jpeg_dac_data_class_table_id,
num_codes: Vec<u8>,
values: Vec<Vec<u8>>
}

#[derive(Debug, Clone)]
pub struct jpeg_dht {
marker: jpeg_eoi,
length: u16,
data: jpeg_dht_data
}

#[derive(Debug, Copy, Clone)]
pub struct jpeg_dqt_data_precision_table_id {
precision: u8,
table_id: u8
}

#[derive(Debug, Clone)]
pub enum jpeg_dqt_data_elements { Bytes(Vec<u8>), Shorts(Vec<u16>) }

#[derive(Debug, Clone)]
pub struct jpeg_dqt_data {
precision_table_id: jpeg_dqt_data_precision_table_id,
elements: jpeg_dqt_data_elements
}

#[derive(Debug, Clone)]
pub struct jpeg_dqt {
marker: jpeg_eoi,
length: u16,
data: Vec<jpeg_dqt_data>
}

#[derive(Debug, Copy, Clone)]
pub struct jpeg_dri_data {
restart_interval: u16
}

#[derive(Debug, Clone)]
pub struct jpeg_dri {
marker: jpeg_eoi,
length: u16,
data: jpeg_dri_data
}

#[derive(Debug, Clone)]
pub enum jpeg_table_or_misc { app0(jpeg_app0), app1(jpeg_app1), app10(jpeg_com), app11(jpeg_com), app12(jpeg_com), app13(jpeg_com), app14(jpeg_com), app15(jpeg_com), app2(jpeg_com), app3(jpeg_com), app4(jpeg_com), app5(jpeg_com), app6(jpeg_com), app7(jpeg_com), app8(jpeg_com), app9(jpeg_com), com(jpeg_com), dac(jpeg_dac), dht(jpeg_dht), dqt(jpeg_dqt), dri(jpeg_dri) }

#[derive(Debug, Copy, Clone)]
pub struct jpeg_sof_image_component_sampling_factor {
horizontal: u8,
vertical: u8
}

#[derive(Debug, Clone)]
pub struct jpeg_sof_image_component {
id: u8,
sampling_factor: jpeg_sof_image_component_sampling_factor,
quantization_table_id: u8
}

#[derive(Debug, Clone)]
pub struct jpeg_sof_data {
sample_precision: u8,
num_lines: u16,
num_samples_per_line: u16,
num_image_components: u8,
image_components: Vec<jpeg_sof_image_component>
}

#[derive(Debug, Clone)]
pub struct jpeg_sof15 {
marker: jpeg_eoi,
length: u16,
data: jpeg_sof_data
}

#[derive(Debug, Clone)]
pub enum jpeg_frame_header { sof0(jpeg_sof15), sof1(jpeg_sof15), sof10(jpeg_sof15), sof11(jpeg_sof15), sof13(jpeg_sof15), sof14(jpeg_sof15), sof15(jpeg_sof15), sof2(jpeg_sof15), sof3(jpeg_sof15), sof5(jpeg_sof15), sof6(jpeg_sof15), sof7(jpeg_sof15), sof9(jpeg_sof15) }

#[derive(Debug, Copy, Clone)]
pub struct jpeg_sos_image_component_entropy_coding_table_ids {
dc_entropy_coding_table_id: u8,
ac_entropy_coding_table_id: u8
}

#[derive(Debug, Clone)]
pub struct jpeg_sos_image_component {
component_selector: u8,
entropy_coding_table_ids: jpeg_sos_image_component_entropy_coding_table_ids
}

#[derive(Debug, Copy, Clone)]
pub struct jpeg_sos_data_approximation_bit_position {
high: u8,
low: u8
}

#[derive(Debug, Clone)]
pub struct jpeg_sos_data {
num_image_components: u8,
image_components: Vec<jpeg_sos_image_component>,
start_spectral_selection: u8,
end_spectral_selection: u8,
approximation_bit_position: jpeg_sos_data_approximation_bit_position
}

#[derive(Debug, Clone)]
pub struct jpeg_sos {
marker: jpeg_eoi,
length: u16,
data: jpeg_sos_data
}

#[derive(Debug, Clone)]
pub enum jpeg_scan_data_scan_data { mcu(u8), rst0(jpeg_eoi), rst1(jpeg_eoi), rst2(jpeg_eoi), rst3(jpeg_eoi), rst4(jpeg_eoi), rst5(jpeg_eoi), rst6(jpeg_eoi), rst7(jpeg_eoi) }

#[derive(Debug, Clone)]
pub struct jpeg_scan_data {
scan_data: Vec<jpeg_scan_data_scan_data>,
scan_data_stream: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct jpeg_scan {
segments: Vec<jpeg_table_or_misc>,
sos: jpeg_sos,
data: jpeg_scan_data
}

#[derive(Debug, Copy, Clone)]
pub struct jpeg_dnl_data {
num_lines: u16
}

#[derive(Debug, Clone)]
pub struct jpeg_dnl {
marker: jpeg_eoi,
length: u16,
data: jpeg_dnl_data
}

#[derive(Debug, Clone)]
pub enum jpeg_frame_dnl { none, some(jpeg_dnl) }

#[derive(Debug, Clone)]
pub struct jpeg_frame {
initial_segment: jpeg_frame_initial_segment,
segments: Vec<jpeg_table_or_misc>,
header: jpeg_frame_header,
scan: jpeg_scan,
dnl: jpeg_frame_dnl,
scans: Vec<jpeg_scan>
}

#[derive(Debug, Clone)]
pub struct jpeg_main {
soi: jpeg_eoi,
frame: jpeg_frame,
eoi: jpeg_eoi
}

#[derive(Debug, Clone)]
pub struct mpeg4_atom_data_ftyp {
major_brand: (u8, u8, u8, u8),
minor_version: u32,
compatible_brands: Vec<(u8, u8, u8, u8)>
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stsd_sample_entries {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct mpeg4_dinf_atom_data_dref {
version: u8,
flags: (u8, u8, u8),
number_of_entries: u32,
data: Vec<mpeg4_stbl_atom_data_stsd_sample_entries>
}

#[derive(Debug, Clone)]
pub enum mpeg4_dinf_atom_data { dref(mpeg4_dinf_atom_data_dref), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_dinf_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_dinf_atom_data
}

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_hdlr {
version: u8,
flags: (u8, u8, u8),
predefined: u32,
handler_type: (u8, u8, u8, u8),
reserved: (u32, u32, u32),
name: base_asciiz_string
}

#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe_fields_no_extra_fields_mime {
content_type: base_asciiz_string
}

#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe_fields_no_extra_fields_uri {
item_uri_type: base_asciiz_string
}

#[derive(Debug, Clone)]
pub enum mpeg4_iinf_atom_data_infe_fields_no_extra_fields { mime(mpeg4_iinf_atom_data_infe_fields_no_extra_fields_mime), unknown, uri(mpeg4_iinf_atom_data_infe_fields_no_extra_fields_uri) }

#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe_fields_no {
item_ID: u32,
item_protection_index: u16,
item_type: (u8, u8, u8, u8),
item_name: base_asciiz_string,
extra_fields: mpeg4_iinf_atom_data_infe_fields_no_extra_fields
}

#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe_fields_yes {
item_ID: u16,
item_protection_index: u16,
item_name: base_asciiz_string,
content_type: base_asciiz_string,
content_encoding: base_asciiz_string
}

#[derive(Debug, Clone)]
pub enum mpeg4_iinf_atom_data_infe_fields { no(mpeg4_iinf_atom_data_infe_fields_no), yes(mpeg4_iinf_atom_data_infe_fields_yes) }

#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe {
version: u8,
flags: (u8, u8, u8),
fields: mpeg4_iinf_atom_data_infe_fields
}

#[derive(Debug, Clone)]
pub enum mpeg4_iinf_atom_data { infe(mpeg4_iinf_atom_data_infe), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_iinf_atom_data
}

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iinf {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
item_info_entry: Vec<mpeg4_iinf_atom>
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_meta_atom_data_iloc_items_extents {
extent_index: u64,
extent_offset: u64,
extent_length: u64
}

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iloc_items {
item_ID: u32,
construction_method: Option<u16>,
data_reference_index: u16,
base_offset: u64,
extent_count: u16,
extents: Vec<mpeg4_meta_atom_data_iloc_items_extents>
}

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iloc {
version: u8,
flags: (u8, u8, u8),
offset_size_length_size: u8,
base_offset_size_index_size: u8,
offset_size: u8,
length_size: u8,
base_offset_size: u8,
index_size: u8,
item_count: u32,
items: Vec<mpeg4_meta_atom_data_iloc_items>
}

#[derive(Debug, Clone)]
pub struct mpeg4_tool_atom_data_data {
type_indicator: u32,
locale_indicator: u32,
value: Vec<u8>
}

#[derive(Debug, Clone)]
pub enum mpeg4_tool_atom_data { data(mpeg4_tool_atom_data_data), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_tool_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_tool_atom_data
}

#[derive(Debug, Clone)]
pub enum mpeg4_ilst_atom_data { tool(Vec<mpeg4_tool_atom>), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_ilst_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_ilst_atom_data
}

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iref_single_item_reference_large_data {
from_item_ID: u32,
reference_count: u16,
to_item_ID: Vec<u32>
}

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iref_single_item_reference_large {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_meta_atom_data_iref_single_item_reference_large_data
}

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iref_single_item_reference_small_data {
from_item_ID: u16,
reference_count: u16,
to_item_ID: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iref_single_item_reference_small {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_meta_atom_data_iref_single_item_reference_small_data
}

#[derive(Debug, Clone)]
pub enum mpeg4_meta_atom_data_iref_single_item_reference { large(Vec<mpeg4_meta_atom_data_iref_single_item_reference_large>), small(Vec<mpeg4_meta_atom_data_iref_single_item_reference_small>) }

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iref {
version: u8,
flags: (u8, u8, u8),
single_item_reference: mpeg4_meta_atom_data_iref_single_item_reference
}

#[derive(Debug, Copy, Clone)]
pub enum mpeg4_meta_atom_data_pitm_item_ID { no(u32), yes(u16) }

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_pitm {
version: u8,
flags: (u8, u8, u8),
item_ID: mpeg4_meta_atom_data_pitm_item_ID
}

#[derive(Debug, Clone)]
pub enum mpeg4_meta_atom_data { dinf(Vec<mpeg4_dinf_atom>), hdlr(mpeg4_meta_atom_data_hdlr), idat(Vec<u8>), iinf(mpeg4_meta_atom_data_iinf), iloc(mpeg4_meta_atom_data_iloc), ilst(Vec<mpeg4_ilst_atom>), iref(mpeg4_meta_atom_data_iref), pitm(mpeg4_meta_atom_data_pitm), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_meta_atom_data
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_moov_atom_data_mvhd_fields_version0 {
creation_time: u32,
modification_time: u32,
timescale: u32,
duration: u32
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_moov_atom_data_mvhd_fields_version1 {
creation_time: u64,
modification_time: u64,
timescale: u32,
duration: u64
}

#[derive(Debug, Clone)]
pub enum mpeg4_moov_atom_data_mvhd_fields { version0(mpeg4_moov_atom_data_mvhd_fields_version0), version1(mpeg4_moov_atom_data_mvhd_fields_version1) }

#[derive(Debug, Clone)]
pub struct mpeg4_moov_atom_data_mvhd {
version: u8,
flags: (u8, u8, u8),
fields: mpeg4_moov_atom_data_mvhd_fields,
rate: u32,
volume: u16,
reserved1: u16,
reserved2: (u32, u32),
matrix: Vec<u32>,
pre_defined: Vec<u32>,
next_track_ID: u32
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_edts_atom_data_elst_edit_list_table {
track_duration: u32,
media_time: u32,
media_rate: u32
}

#[derive(Debug, Clone)]
pub struct mpeg4_edts_atom_data_elst {
version: u8,
flags: (u8, u8, u8),
number_of_entries: u32,
edit_list_table: Vec<mpeg4_edts_atom_data_elst_edit_list_table>
}

#[derive(Debug, Clone)]
pub enum mpeg4_edts_atom_data { elst(mpeg4_edts_atom_data_elst), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_edts_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_edts_atom_data
}

#[derive(Debug, Clone)]
pub struct mpeg4_mdia_atom_data_hdlr {
version: u8,
flags: (u8, u8, u8),
component_type: u32,
component_subtype: (u8, u8, u8, u8),
component_manufacturer: u32,
component_flags: u32,
component_flags_mask: u32,
component_name: base_asciiz_string
}

#[derive(Debug, Clone)]
pub struct mpeg4_mdia_atom_data_mdhd {
version: u8,
flags: (u8, u8, u8),
fields: mpeg4_moov_atom_data_mvhd_fields,
language: u16,
pre_defined: u16
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_minf_atom_data_smhd {
version: u8,
flags: (u8, u8, u8),
balance: u16,
reserved: u16
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_co64 {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_offset: Vec<u64>
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_stbl_atom_data_ctts_sample_entries {
sample_count: u32,
sample_offset: u32
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_ctts {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<mpeg4_stbl_atom_data_ctts_sample_entries>
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_stbl_atom_data_sbgp_sample_groups {
sample_count: u32,
group_description_index: u32
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_sbgp {
version: u8,
flags: (u8, u8, u8),
grouping_type: u32,
grouping_type_parameter: Option<u32>,
entry_count: u32,
sample_groups: Vec<mpeg4_stbl_atom_data_sbgp_sample_groups>
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_sgpd_sample_groups {
description_length: u32,
sample_group_entry: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_sgpd {
version: u8,
flags: (u8, u8, u8),
grouping_type: u32,
default_length: u32,
entry_count: u32,
sample_groups: Vec<mpeg4_stbl_atom_data_sgpd_sample_groups>
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stco {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_offset: Vec<u32>
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_stbl_atom_data_stsc_chunk_entries {
first_chunk: u32,
samples_per_chunk: u32,
sample_description_index: u32
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stsc {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_entries: Vec<mpeg4_stbl_atom_data_stsc_chunk_entries>
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stsd {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<mpeg4_stbl_atom_data_stsd_sample_entries>
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stss {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_number: Vec<u32>
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stsz {
version: u8,
flags: (u8, u8, u8),
sample_size: u32,
sample_count: u32,
entry_size: Option<Vec<u32>>
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_stbl_atom_data_stts_sample_entries {
sample_count: u32,
sample_delta: u32
}

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stts {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<mpeg4_stbl_atom_data_stts_sample_entries>
}

#[derive(Debug, Clone)]
pub enum mpeg4_stbl_atom_data { co64(mpeg4_stbl_atom_data_co64), ctts(mpeg4_stbl_atom_data_ctts), sbgp(mpeg4_stbl_atom_data_sbgp), sgpd(mpeg4_stbl_atom_data_sgpd), stco(mpeg4_stbl_atom_data_stco), stsc(mpeg4_stbl_atom_data_stsc), stsd(mpeg4_stbl_atom_data_stsd), stss(mpeg4_stbl_atom_data_stss), stsz(mpeg4_stbl_atom_data_stsz), stts(mpeg4_stbl_atom_data_stts), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_stbl_atom_data
}

#[derive(Debug, Clone)]
pub struct mpeg4_minf_atom_data_vmhd {
version: u8,
flags: (u8, u8, u8),
graphicsmode: u16,
opcolor: Vec<u16>
}

#[derive(Debug, Clone)]
pub enum mpeg4_minf_atom_data { dinf(Vec<mpeg4_dinf_atom>), smhd(mpeg4_minf_atom_data_smhd), stbl(Vec<mpeg4_stbl_atom>), unknown(Vec<u8>), vmhd(mpeg4_minf_atom_data_vmhd) }

#[derive(Debug, Clone)]
pub struct mpeg4_minf_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_minf_atom_data
}

#[derive(Debug, Clone)]
pub enum mpeg4_mdia_atom_data { hdlr(mpeg4_mdia_atom_data_hdlr), mdhd(mpeg4_mdia_atom_data_mdhd), minf(Vec<mpeg4_minf_atom>), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_mdia_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_mdia_atom_data
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_trak_atom_data_tkhd_fields_version0 {
creation_time: u32,
modification_time: u32,
track_ID: u32,
reserved: u32,
duration: u32
}

#[derive(Debug, Copy, Clone)]
pub struct mpeg4_trak_atom_data_tkhd_fields_version1 {
creation_time: u64,
modification_time: u64,
track_ID: u32,
reserved: u32,
duration: u64
}

#[derive(Debug, Clone)]
pub enum mpeg4_trak_atom_data_tkhd_fields { version0(mpeg4_trak_atom_data_tkhd_fields_version0), version1(mpeg4_trak_atom_data_tkhd_fields_version1) }

#[derive(Debug, Clone)]
pub struct mpeg4_trak_atom_data_tkhd {
version: u8,
flags: (u8, u8, u8),
fields: mpeg4_trak_atom_data_tkhd_fields,
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
pub enum mpeg4_trak_atom_data { edts(Vec<mpeg4_edts_atom>), mdia(Vec<mpeg4_mdia_atom>), tkhd(mpeg4_trak_atom_data_tkhd), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_trak_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_trak_atom_data
}

#[derive(Debug, Clone)]
pub enum mpeg4_udta_atom_data { meta(u32, Vec<mpeg4_meta_atom>), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_udta_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_udta_atom_data
}

#[derive(Debug, Clone)]
pub enum mpeg4_moov_atom_data { mvhd(mpeg4_moov_atom_data_mvhd), trak(Vec<mpeg4_trak_atom>), udta(Vec<mpeg4_udta_atom>), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_moov_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_moov_atom_data
}

#[derive(Debug, Clone)]
pub enum mpeg4_atom_data { free, ftyp(mpeg4_atom_data_ftyp), mdat, meta(u32, Vec<mpeg4_meta_atom>), moov(Vec<mpeg4_moov_atom>), unknown(Vec<u8>) }

#[derive(Debug, Clone)]
pub struct mpeg4_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_atom_data
}

#[derive(Debug, Clone)]
pub struct mpeg4_main {
atoms: Vec<mpeg4_atom>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_table_record {
table_id: u32,
checksum: u32,
offset: u32,
length: u32
}

#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format0 {
format: u16,
length: u16,
language: u16,
glyph_id_array: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format10 {
format: u16,
__reserved: u16,
length: u32,
language: u32,
start_char_code: u32,
num_chars: u32,
glyph_id_array: Vec<u16>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_types_sequential_map_record {
start_char_code: u32,
end_char_code: u32,
start_glyph_id: u32
}

#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format13 {
format: u16,
__reserved: u16,
length: u32,
language: u32,
num_groups: u32,
groups: Vec<opentype_types_sequential_map_record>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_variation_selector_default_uvs_offset_link_ranges {
start_unicode_value: u32,
additional_count: u8
}

#[derive(Debug, Clone)]
pub struct opentype_variation_selector_default_uvs_offset_link {
num_unicode_value_ranges: u32,
ranges: Vec<opentype_variation_selector_default_uvs_offset_link_ranges>
}

#[derive(Debug, Clone)]
pub struct opentype_variation_selector_default_uvs_offset {
offset: u32,
link: Option<opentype_variation_selector_default_uvs_offset_link>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_variation_selector_non_default_uvs_offset_link_uvs_mappings {
unicode_value: u32,
glyph_id: u16
}

#[derive(Debug, Clone)]
pub struct opentype_variation_selector_non_default_uvs_offset_link {
num_uvs_mappings: u32,
uvs_mappings: Vec<opentype_variation_selector_non_default_uvs_offset_link_uvs_mappings>
}

#[derive(Debug, Clone)]
pub struct opentype_variation_selector_non_default_uvs_offset {
offset: u32,
link: Option<opentype_variation_selector_non_default_uvs_offset_link>
}

#[derive(Debug, Clone)]
pub struct opentype_variation_selector {
var_selector: u32,
default_uvs_offset: opentype_variation_selector_default_uvs_offset,
non_default_uvs_offset: opentype_variation_selector_non_default_uvs_offset
}

#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format14 {
format: u16,
length: u32,
num_var_selector_records: u32,
var_selector: Vec<opentype_variation_selector>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_cmap_subtable_format2_sub_headers {
first_code: u16,
entry_count: u16,
id_delta: u16,
id_range_offset: u16
}

#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format2 {
format: u16,
length: u16,
language: u16,
sub_header_keys: Vec<u16>,
sub_headers: Vec<opentype_cmap_subtable_format2_sub_headers>,
glyph_array: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format4 {
format: u16,
length: u16,
language: u16,
seg_count: u16,
search_range: u16,
entry_selector: u16,
range_shift: u16,
end_code: Vec<u16>,
__reserved_pad: u16,
start_code: Vec<u16>,
id_delta: Vec<u16>,
id_range_offset: Vec<u16>,
glyph_array: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format6 {
format: u16,
length: u16,
language: u16,
first_code: u16,
entry_count: u16,
glyph_id_array: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format8 {
format: u16,
__reserved: u16,
length: u32,
language: u32,
is32: Vec<u8>,
num_groups: u32,
groups: Vec<opentype_types_sequential_map_record>
}

#[derive(Debug, Clone)]
pub enum opentype_cmap_subtable_data { Format0(opentype_cmap_subtable_format0), Format10(opentype_cmap_subtable_format10), Format12(opentype_cmap_subtable_format13), Format13(opentype_cmap_subtable_format13), Format14(opentype_cmap_subtable_format14), Format2(opentype_cmap_subtable_format2), Format4(opentype_cmap_subtable_format4), Format6(opentype_cmap_subtable_format6), Format8(opentype_cmap_subtable_format8) }

#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable {
table_start: u32,
format: u16,
data: opentype_cmap_subtable_data
}

#[derive(Debug, Clone)]
pub struct opentype_encoding_record_subtable_offset {
offset: u32,
link: Option<opentype_cmap_subtable>
}

#[derive(Debug, Clone)]
pub struct opentype_encoding_record {
platform: u16,
encoding: u16,
subtable_offset: opentype_encoding_record_subtable_offset
}

#[derive(Debug, Clone)]
pub struct opentype_cmap_table {
table_start: u32,
version: u16,
num_tables: u16,
encoding_records: Vec<opentype_encoding_record>
}

#[derive(Debug, Copy, Clone)]
pub enum opentype_post_table_italic_angle { Fixed32(u32) }

#[derive(Debug, Copy, Clone)]
pub struct opentype_head_table_glyph_extents {
x_min: u16,
y_min: u16,
x_max: u16,
y_max: u16
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_head_table_mac_style {
extended: bool,
condensed: bool,
shadow: bool,
outline: bool,
underline: bool,
italic: bool,
bold: bool
}

#[derive(Debug, Clone)]
pub struct opentype_head_table {
major_version: u16,
minor_version: u16,
font_revision: opentype_post_table_italic_angle,
checksum_adjustment: u32,
magic_number: (u8, u8, u8, u8),
flags: u16,
units_per_em: u16,
created: u64,
modified: u64,
glyph_extents: opentype_head_table_glyph_extents,
mac_style: opentype_head_table_mac_style,
lowest_rec_ppem: u16,
font_direction_hint: u16,
index_to_loc_format: u16,
glyph_data_format: u16
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_hhea_table_caret_slope {
rise: u16,
run: u16
}

#[derive(Debug, Clone)]
pub struct opentype_hhea_table {
major_version: u16,
minor_version: u16,
ascent: u16,
descent: u16,
line_gap: u16,
advance_width_max: u16,
min_left_side_bearing: u16,
min_right_side_bearing: u16,
x_max_extent: u16,
caret_slope: opentype_hhea_table_caret_slope,
caret_offset: u16,
__reservedX4: (u16, u16, u16, u16),
metric_data_format: u16,
number_of_long_horizontal_metrics: u16
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_maxp_table_version1 {
max_points: u16,
max_contours: u16,
max_composite_points: u16,
max_composite_contours: u16,
max_zones: u16,
max_twilight_points: u16,
max_storage: u16,
max_function_defs: u16,
max_instruction_defs: u16,
max_stack_elements: u16,
max_size_of_instructions: u16,
max_component_elements: u16,
max_component_depth: u16
}

#[derive(Debug, Clone)]
pub enum opentype_maxp_table_data { MaxpPostScript, MaxpUnknown(u32), MaxpV1(opentype_maxp_table_version1) }

#[derive(Debug, Clone)]
pub struct opentype_maxp_table {
version: u32,
num_glyphs: u16,
data: opentype_maxp_table_data
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_hmtx_table_h_metrics {
advance_width: u16,
left_side_bearing: u16
}

#[derive(Debug, Clone)]
pub struct opentype_hmtx_table {
h_metrics: Vec<opentype_hmtx_table_h_metrics>,
left_side_bearings: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct opentype_name_table_name_records_offset {
offset: u16,
link: Option<Vec<u8>>
}

#[derive(Debug, Clone)]
pub struct opentype_name_table_name_records {
platform: u16,
encoding: u16,
language: u16,
name_id: u16,
length: u16,
offset: opentype_name_table_name_records_offset
}

#[derive(Debug, Clone)]
pub struct opentype_name_table_name_version_1_lang_tag_records {
length: u16,
offset: opentype_name_table_name_records_offset
}

#[derive(Debug, Clone)]
pub struct opentype_name_table_name_version_1 {
lang_tag_count: u16,
lang_tag_records: Vec<opentype_name_table_name_version_1_lang_tag_records>
}

#[derive(Debug, Clone)]
pub enum opentype_name_table_data { NameVersion0, NameVersion1(opentype_name_table_name_version_1), NameVersionUnknown(u16) }

#[derive(Debug, Clone)]
pub struct opentype_name_table {
table_start: u32,
version: u16,
name_count: u16,
storage_offset: u16,
name_records: Vec<opentype_name_table_name_records>,
data: opentype_name_table_data
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_os2_table_data_extra_fields_v1_extra_fields_v2_extra_fields_v5 {
us_lower_optical_point_size: u16,
us_upper_optical_point_size: u16
}

#[derive(Debug, Clone)]
pub struct opentype_os2_table_data_extra_fields_v1_extra_fields_v2 {
sx_height: u16,
s_cap_height: u16,
us_default_char: u16,
us_break_char: u16,
us_max_context: u16,
extra_fields_v5: Option<opentype_os2_table_data_extra_fields_v1_extra_fields_v2_extra_fields_v5>
}

#[derive(Debug, Clone)]
pub struct opentype_os2_table_data_extra_fields_v1 {
ul_code_page_range_1: u32,
ul_code_page_range_2: u32,
extra_fields_v2: Option<opentype_os2_table_data_extra_fields_v1_extra_fields_v2>
}

#[derive(Debug, Clone)]
pub struct opentype_os2_table_data {
s_typo_ascender: u16,
s_typo_descender: u16,
s_typo_line_gap: u16,
us_win_ascent: u16,
us_win_descent: u16,
extra_fields_v1: Option<opentype_os2_table_data_extra_fields_v1>
}

#[derive(Debug, Clone)]
pub struct opentype_os2_table {
version: u16,
x_avg_char_width: u16,
us_weight_class: u16,
us_width_class: u16,
fs_type: u16,
y_subscript_x_size: u16,
y_subscript_y_size: u16,
y_subscript_x_offset: u16,
y_subscript_y_offset: u16,
y_superscript_x_size: u16,
y_superscript_y_size: u16,
y_superscript_x_offset: u16,
y_superscript_y_offset: u16,
y_strikeout_size: u16,
y_strikeout_position: u16,
s_family_class: u16,
panose: Vec<u8>,
ul_unicode_range1: u32,
ul_unicode_range2: u32,
ul_unicode_range3: u32,
ul_unicode_range4: u32,
ach_vend_id: u32,
fs_selection: u16,
us_first_char_index: u16,
us_last_char_index: u16,
data: Option<opentype_os2_table_data>
}

#[derive(Debug, Clone)]
pub struct opentype_post_table_names_Version2 {
num_glyphs: u16,
glyph_name_index: Vec<u16>,
string_data: u32
}

#[derive(Debug, Clone)]
pub struct opentype_post_table_names_Version2Dot5 {
num_glyphs: u16,
offset: Vec<u8>
}

#[derive(Debug, Clone)]
pub enum opentype_post_table_names { Version1, Version2(opentype_post_table_names_Version2), Version2Dot5(opentype_post_table_names_Version2Dot5), Version3, VersionUnknown(u32) }

#[derive(Debug, Clone)]
pub struct opentype_post_table {
version: u32,
italic_angle: opentype_post_table_italic_angle,
underline_position: u16,
underline_thickness: u16,
is_fixed_pitch: u32,
min_mem_type42: u32,
max_mem_type42: u32,
min_mem_type1: u32,
max_mem_type1: u32,
names: opentype_post_table_names
}

#[derive(Debug, Clone)]
pub enum opentype_loca_table_offsets { Offsets16(Vec<u16>), Offsets32(Vec<u32>) }

#[derive(Debug, Clone)]
pub struct opentype_loca_table {
offsets: opentype_loca_table_offsets
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_composite_raw_flags {
unscaled_component_offset: bool,
scaled_component_offset: bool,
overlap_compound: bool,
use_my_metrics: bool,
we_have_instructions: bool,
we_have_a_two_by_two: bool,
we_have_an_x_and_y_scale: bool,
more_components: bool,
__reserved_bit4: bool,
we_have_a_scale: bool,
round_xy_to_grid: bool,
args_are_xy_values: bool,
arg_1_and_2_are_words: bool
}

#[derive(Debug, Copy, Clone)]
pub enum opentype_glyf_composite_raw_argument1 { Int16(u16), Int8(u8), Uint16(u16), Uint8(u8) }

#[derive(Debug, Copy, Clone)]
pub enum opentype_glyf_composite_raw_scale_Scale { F2Dot14(u16) }

#[derive(Debug, Clone)]
pub struct opentype_glyf_composite_raw_scale_XY {
x_scale: opentype_glyf_composite_raw_scale_Scale,
y_scale: opentype_glyf_composite_raw_scale_Scale
}

#[derive(Debug, Clone)]
pub enum opentype_glyf_composite_raw_scale { Matrix((opentype_glyf_composite_raw_scale_Scale, opentype_glyf_composite_raw_scale_Scale), (opentype_glyf_composite_raw_scale_Scale, opentype_glyf_composite_raw_scale_Scale)), Scale(opentype_glyf_composite_raw_scale_Scale), XY(opentype_glyf_composite_raw_scale_XY) }

#[derive(Debug, Clone)]
pub struct opentype_glyf_composite_raw {
flags: opentype_glyf_composite_raw_flags,
glyph_index: u16,
argument1: opentype_glyf_composite_raw_argument1,
argument2: opentype_glyf_composite_raw_argument1,
scale: Option<opentype_glyf_composite_raw_scale>
}

#[derive(Debug, Clone)]
pub struct opentype_glyf_composite {
glyphs: Vec<opentype_glyf_composite_raw>,
instructions: Vec<u8>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_simple_flags {
on_curve_point: bool,
x_short_vector: bool,
y_short_vector: bool,
x_is_same_or_positive_x_short_vector: bool,
y_is_same_or_positive_y_short_vector: bool,
overlap_simple: bool
}

#[derive(Debug, Clone)]
pub struct opentype_glyf_simple {
end_points_of_contour: Vec<u16>,
instruction_length: u16,
instructions: Vec<u8>,
number_of_coordinates: u16,
flags: Vec<opentype_glyf_simple_flags>,
x_coordinates: Vec<u16>,
y_coordinates: Vec<u16>
}

#[derive(Debug, Clone)]
pub enum opentype_glyf_description { Composite(opentype_glyf_composite), HeaderOnly, Simple(opentype_glyf_simple) }

#[derive(Debug, Clone)]
pub struct opentype_glyf_table_Glyph {
number_of_contours: u16,
x_min: u16,
y_min: u16,
x_max: u16,
y_max: u16,
description: opentype_glyf_description
}

#[derive(Debug, Clone)]
pub enum opentype_glyf_table { EmptyGlyph, Glyph(opentype_glyf_table_Glyph) }

#[derive(Debug, Copy, Clone)]
pub struct opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version0 {
dogray: bool,
gridfit: bool
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version1 {
symmetric_smoothing: bool,
symmetric_gridfit: bool,
dogray: bool,
gridfit: bool
}

#[derive(Debug, Clone)]
pub enum opentype_gasp_table_gasp_ranges_range_gasp_behavior { Version0(opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version0), Version1(opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version1) }

#[derive(Debug, Clone)]
pub struct opentype_gasp_table_gasp_ranges {
range_max_ppem: u16,
range_gasp_behavior: opentype_gasp_table_gasp_ranges_range_gasp_behavior
}

#[derive(Debug, Clone)]
pub struct opentype_gasp_table {
version: u16,
num_ranges: u16,
gasp_ranges: Vec<opentype_gasp_table_gasp_ranges>
}

#[derive(Debug, Clone)]
pub struct opentype_class_def_data_Format1 {
start_glyph_id: u16,
glyph_count: u16,
class_value_array: Vec<u16>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_class_def_data_Format2_class_range_records {
start_glyph_id: u16,
end_glyph_id: u16,
class: u16
}

#[derive(Debug, Clone)]
pub struct opentype_class_def_data_Format2 {
class_range_count: u16,
class_range_records: Vec<opentype_class_def_data_Format2_class_range_records>
}

#[derive(Debug, Clone)]
pub enum opentype_class_def_data { Format1(opentype_class_def_data_Format1), Format2(opentype_class_def_data_Format2) }

#[derive(Debug, Clone)]
pub struct opentype_class_def {
class_format: u16,
data: opentype_class_def_data
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_glyph_class_def {
offset: u16,
link: Option<opentype_class_def>
}

#[derive(Debug, Clone)]
pub struct opentype_coverage_table_data_Format1 {
glyph_count: u16,
glyph_array: Vec<u16>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_coverage_table_data_Format2_range_records {
start_glyph_id: u16,
end_glyph_id: u16,
start_coverage_index: u16
}

#[derive(Debug, Clone)]
pub struct opentype_coverage_table_data_Format2 {
range_count: u16,
range_records: Vec<opentype_coverage_table_data_Format2_range_records>
}

#[derive(Debug, Clone)]
pub enum opentype_coverage_table_data { Format1(opentype_coverage_table_data_Format1), Format2(opentype_coverage_table_data_Format2) }

#[derive(Debug, Clone)]
pub struct opentype_coverage_table {
coverage_format: u16,
data: opentype_coverage_table_data
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_attach_list_link_coverage {
offset: u16,
link: Option<opentype_coverage_table>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_attach_list_link_attach_point_offsets_link {
point_count: u16,
point_indices: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_attach_list_link_attach_point_offsets {
offset: u16,
link: Option<opentype_gdef_table_attach_list_link_attach_point_offsets_link>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_attach_list_link {
table_start: u32,
coverage: opentype_gdef_table_attach_list_link_coverage,
glyph_count: u16,
attach_point_offsets: Vec<opentype_gdef_table_attach_list_link_attach_point_offsets>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_attach_list {
offset: u16,
link: Option<opentype_gdef_table_attach_list_link>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format1 {
coordinate: u16
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format2 {
caret_value_point_index: u16
}

#[derive(Debug, Clone)]
pub struct opentype_common_device_or_variation_index_table_DeviceTable {
start_size: u16,
end_size: u16,
delta_format: u16,
delta_values: Vec<u16>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_common_device_or_variation_index_table_VariationIndexTable {
delta_set_outer_index: u16,
delta_set_inner_index: u16,
delta_format: (u8, u8)
}

#[derive(Debug, Clone)]
pub enum opentype_common_device_or_variation_index_table { DeviceTable(opentype_common_device_or_variation_index_table_DeviceTable), VariationIndexTable(opentype_common_device_or_variation_index_table_VariationIndexTable) }

#[derive(Debug, Clone)]
pub struct opentype_common_value_record_x_advance_device {
offset: u16,
link: Option<opentype_common_device_or_variation_index_table>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format3 {
coordinate: u16,
table: opentype_common_value_record_x_advance_device
}

#[derive(Debug, Clone)]
pub enum opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data { Format1(opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format1), Format2(opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format2), Format3(opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format3) }

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link {
table_start: u32,
caret_value_format: u16,
data: opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values {
offset: u16,
link: Option<opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link {
table_start: u32,
caret_count: u16,
caret_values: Vec<opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets {
offset: u16,
link: Option<opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link {
table_start: u32,
coverage: opentype_gdef_table_attach_list_link_coverage,
lig_glyph_count: u16,
lig_glyph_offsets: Vec<opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list {
offset: u16,
link: Option<opentype_gdef_table_lig_caret_list_link>
}

#[derive(Debug, Clone)]
pub struct opentype_mark_glyph_set_coverage {
offset: u32,
link: Option<opentype_coverage_table>
}

#[derive(Debug, Clone)]
pub struct opentype_mark_glyph_set {
table_start: u32,
format: u16,
mark_glyph_set_count: u16,
coverage: Vec<opentype_mark_glyph_set_coverage>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_data_Version1_2_mark_glyph_sets_def {
offset: u16,
link: Option<opentype_mark_glyph_set>
}

#[derive(Debug, Clone)]
pub struct opentype_gdef_table_data_Version1_2 {
mark_glyph_sets_def: opentype_gdef_table_data_Version1_2_mark_glyph_sets_def
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_data_Version1_3 {
item_var_store: u32
}

#[derive(Debug, Clone)]
pub enum opentype_gdef_table_data { Version1_0, Version1_2(opentype_gdef_table_data_Version1_2), Version1_3(opentype_gdef_table_data_Version1_3) }

#[derive(Debug, Clone)]
pub struct opentype_gdef_table {
table_start: u32,
major_version: u16,
minor_version: u16,
glyph_class_def: opentype_gdef_table_glyph_class_def,
attach_list: opentype_gdef_table_attach_list,
lig_caret_list: opentype_gdef_table_lig_caret_list,
mark_attach_class_def: opentype_gdef_table_glyph_class_def,
data: opentype_gdef_table_data
}

#[derive(Debug, Clone)]
pub struct opentype_common_langsys {
lookup_order_offset: u16,
required_feature_index: u16,
feature_index_count: u16,
feature_indices: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct opentype_common_script_table_default_lang_sys {
offset: u16,
link: Option<opentype_common_langsys>
}

#[derive(Debug, Clone)]
pub struct opentype_common_script_table_lang_sys_records {
lang_sys_tag: u32,
lang_sys: opentype_common_script_table_default_lang_sys
}

#[derive(Debug, Clone)]
pub struct opentype_common_script_table {
table_start: u32,
default_lang_sys: opentype_common_script_table_default_lang_sys,
lang_sys_count: u16,
lang_sys_records: Vec<opentype_common_script_table_lang_sys_records>
}

#[derive(Debug, Clone)]
pub struct opentype_common_script_list_script_records_script {
offset: u16,
link: Option<opentype_common_script_table>
}

#[derive(Debug, Clone)]
pub struct opentype_common_script_list_script_records {
script_tag: u32,
script: opentype_common_script_list_script_records_script
}

#[derive(Debug, Clone)]
pub struct opentype_common_script_list {
table_start: u32,
script_count: u16,
script_records: Vec<opentype_common_script_list_script_records>
}

#[derive(Debug, Clone)]
pub struct opentype_gpos_table_script_list {
offset: u16,
link: Option<opentype_common_script_list>
}

#[derive(Debug, Clone)]
pub struct opentype_common_feature_table {
table_start: u32,
feature_params: u16,
lookup_index_count: u16,
lookup_list_indices: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct opentype_common_feature_list_feature_records_feature {
offset: u16,
link: Option<opentype_common_feature_table>
}

#[derive(Debug, Clone)]
pub struct opentype_common_feature_list_feature_records {
feature_tag: u32,
feature: opentype_common_feature_list_feature_records_feature
}

#[derive(Debug, Clone)]
pub struct opentype_common_feature_list {
table_start: u32,
feature_count: u16,
feature_records: Vec<opentype_common_feature_list_feature_records>
}

#[derive(Debug, Clone)]
pub struct opentype_gpos_table_feature_list {
offset: u16,
link: Option<opentype_common_feature_list>
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_gpos_table_lookup_list_link_lookups_link_lookup_flag {
mark_attachment_class_filter: u8,
right_to_left: bool,
ignore_base_glyphs: bool,
ignore_ligatures: bool,
ignore_marks: bool,
use_mark_filtering_set: bool
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_common_value_format_flags {
y_advance_device: bool,
x_advance_device: bool,
y_placement_device: bool,
x_placement_device: bool,
y_advance: bool,
x_advance: bool,
y_placement: bool,
x_placement: bool
}

#[derive(Debug, Clone)]
pub struct opentype_common_value_record {
x_placement: Option<u16>,
y_placement: Option<u16>,
x_advance: Option<u16>,
y_advance: Option<u16>,
x_placement_device: Option<opentype_common_value_record_x_advance_device>,
y_placement_device: Option<opentype_common_value_record_x_advance_device>,
x_advance_device: Option<opentype_common_value_record_x_advance_device>,
y_advance_device: Option<opentype_common_value_record_x_advance_device>
}

#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust_subtable_Format1 {
coverage_offset: opentype_gdef_table_attach_list_link_coverage,
value_format: opentype_common_value_format_flags,
value_record: opentype_common_value_record
}

#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust_subtable_Format2 {
coverage_offset: opentype_gdef_table_attach_list_link_coverage,
value_format: opentype_common_value_format_flags,
value_count: u16,
value_records: Vec<opentype_common_value_record>
}

#[derive(Debug, Clone)]
pub enum opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust_subtable { Format1(opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust_subtable_Format1), Format2(opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust_subtable_Format2) }

#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust {
table_start: u32,
pos_format: u16,
subtable: opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust_subtable
}

#[derive(Debug, Clone)]
pub enum opentype_gpos_table_lookup_list_link_lookups_link_subtables_link { ChainCtxPos, CtxPos, CursAttach, ExtPos, MarkBaseAttach, MarkLigAttach, MarkMarkAttach, PairAdjust, SingleAdjust(opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust) }

#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link_lookups_link_subtables {
offset: u16,
link: Option<opentype_gpos_table_lookup_list_link_lookups_link_subtables_link>
}

#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link_lookups_link {
table_start: u32,
lookup_type: u16,
lookup_flag: opentype_gpos_table_lookup_list_link_lookups_link_lookup_flag,
sub_table_count: u16,
subtables: Vec<opentype_gpos_table_lookup_list_link_lookups_link_subtables>,
mark_filtering_set: Option<u16>
}

#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link_lookups {
offset: u16,
link: Option<opentype_gpos_table_lookup_list_link_lookups_link>
}

#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link {
table_start: u32,
lookup_count: u16,
lookups: Vec<opentype_gpos_table_lookup_list_link_lookups>
}

#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list {
offset: u16,
link: Option<opentype_gpos_table_lookup_list_link>
}

#[derive(Debug, Clone)]
pub struct opentype_gpos_table {
table_start: u32,
major_version: u16,
minor_version: u16,
script_list: opentype_gpos_table_script_list,
feature_list: opentype_gpos_table_feature_list,
lookup_list: opentype_gpos_table_lookup_list
}

#[derive(Debug, Clone)]
pub struct opentype_table_directory_table_links {
cmap: opentype_cmap_table,
head: opentype_head_table,
hhea: opentype_hhea_table,
maxp: opentype_maxp_table,
hmtx: opentype_hmtx_table,
name: opentype_name_table,
os2: opentype_os2_table,
post: opentype_post_table,
cvt: Option<Vec<u16>>,
fpgm: Option<Vec<u8>>,
loca: Option<opentype_loca_table>,
glyf: Option<Vec<opentype_glyf_table>>,
prep: Option<Vec<u8>>,
gasp: Option<opentype_gasp_table>,
base: Option<()>,
gdef: Option<opentype_gdef_table>,
gpos: Option<opentype_gpos_table>,
__skip: ()
}

#[derive(Debug, Clone)]
pub struct opentype_table_directory {
sfnt_version: u32,
num_tables: u16,
search_range: u16,
entry_selector: u16,
range_shift: u16,
table_records: Vec<opentype_table_record>,
table_links: opentype_table_directory_table_links
}

#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version1_table_directories {
offset: u32,
link: Option<opentype_table_directory>
}

#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version1 {
num_fonts: u32,
table_directories: Vec<opentype_ttc_header_header_Version1_table_directories>
}

#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version2 {
num_fonts: u32,
table_directories: Vec<opentype_ttc_header_header_Version1_table_directories>,
dsig_tag: u32,
dsig_length: u32,
dsig_offset: u32
}

#[derive(Debug, Clone)]
pub enum opentype_ttc_header_header { UnknownVersion(u16), Version1(opentype_ttc_header_header_Version1), Version2(opentype_ttc_header_header_Version2) }

#[derive(Debug, Clone)]
pub struct opentype_ttc_header {
ttc_tag: u32,
major_version: u16,
minor_version: u16,
header: opentype_ttc_header_header,
__skip: ()
}

#[derive(Debug, Clone)]
pub enum opentype_font_directory { TTCHeader(opentype_ttc_header), TableDirectory(opentype_table_directory) }

#[derive(Debug, Clone)]
pub struct opentype_font {
magic: u32,
directory: opentype_font_directory
}

#[derive(Debug, Clone)]
pub struct opentype_main {
start: u32,
font: opentype_font
}

#[derive(Debug, Copy, Clone)]
pub struct png_ihdr_data {
width: u32,
height: u32,
bit_depth: u8,
color_type: u8,
compression_method: u8,
filter_method: u8,
interlace_method: u8
}

#[derive(Debug, Clone)]
pub struct png_ihdr {
length: u32,
tag: (u8, u8, u8, u8),
data: png_ihdr_data,
crc: u32
}

#[derive(Debug, Copy, Clone)]
pub struct png_bkgd_color_type_0 {
greyscale: u16
}

#[derive(Debug, Copy, Clone)]
pub struct png_bkgd_color_type_2 {
red: u16,
green: u16,
blue: u16
}

#[derive(Debug, Copy, Clone)]
pub struct png_bkgd_color_type_3 {
palette_index: u8
}

#[derive(Debug, Clone)]
pub enum png_bkgd { color_type_0(png_bkgd_color_type_0), color_type_2(png_bkgd_color_type_2), color_type_3(png_bkgd_color_type_3), color_type_4(png_bkgd_color_type_0), color_type_6(png_bkgd_color_type_2) }

#[derive(Debug, Copy, Clone)]
pub struct png_chrm {
whitepoint_x: u32,
whitepoint_y: u32,
red_x: u32,
red_y: u32,
green_x: u32,
green_y: u32,
blue_x: u32,
blue_y: u32
}

#[derive(Debug, Copy, Clone)]
pub struct png_gama {
gamma: u32
}

#[derive(Debug, Clone)]
pub struct png_hist {
histogram: Vec<u16>
}

#[derive(Debug, Copy, Clone)]
pub struct zlib_main_compression_method_flags {
compression_info: u8,
compression_method: u8
}

#[derive(Debug, Copy, Clone)]
pub struct zlib_main_flags {
flevel: u8,
fdict: u8,
fcheck: u8
}

#[derive(Debug, Clone)]
pub struct zlib_main {
compression_method_flags: zlib_main_compression_method_flags,
flags: zlib_main_flags,
dict_id: Option<u32>,
data: deflate_main,
adler32: u32
}

#[derive(Debug, Clone)]
pub struct png_iccp {
profile_name: Vec<u8>,
compression_method: u8,
compressed_profile: zlib_main
}

#[derive(Debug, Clone)]
pub enum png_itxt_text_compressed { invalid(Vec<u8>), valid(Vec<char>) }

#[derive(Debug, Clone)]
pub enum png_itxt_text { compressed(png_itxt_text_compressed), uncompressed(Vec<char>) }

#[derive(Debug, Clone)]
pub struct png_itxt {
keyword: Vec<u8>,
compression_flag: u8,
compression_method: u8,
language_tag: base_asciiz_string,
translated_keyword: Vec<char>,
text: png_itxt_text
}

#[derive(Debug, Copy, Clone)]
pub struct png_phys {
pixels_per_unit_x: u32,
pixels_per_unit_y: u32,
unit_specifier: u8
}

#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_0 {
sig_greyscale_bits: u8
}

#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_2 {
sig_red_bits: u8,
sig_green_bits: u8,
sig_blue_bits: u8
}

#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_4 {
sig_greyscale_bits: u8,
sig_alpha_bits: u8
}

#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_6 {
sig_red_bits: u8,
sig_green_bits: u8,
sig_blue_bits: u8,
sig_alpha_bits: u8
}

#[derive(Debug, Clone)]
pub enum png_sbit { color_type_0(png_sbit_color_type_0), color_type_2(png_sbit_color_type_2), color_type_3(png_sbit_color_type_2), color_type_4(png_sbit_color_type_4), color_type_6(png_sbit_color_type_6) }

#[derive(Debug, Copy, Clone)]
pub struct png_splt_pallette_sample_depth_u16 {
red: u16,
green: u16,
blue: u16,
alpha: u16,
frequency: u16
}

#[derive(Debug, Copy, Clone)]
pub struct png_splt_pallette_sample_depth_u8 {
red: u8,
green: u8,
blue: u8,
alpha: u8,
frequency: u16
}

#[derive(Debug, Clone)]
pub enum png_splt_pallette { sample_depth_u16(Vec<png_splt_pallette_sample_depth_u16>), sample_depth_u8(Vec<png_splt_pallette_sample_depth_u8>) }

#[derive(Debug, Clone)]
pub struct png_splt {
palette_name: Vec<u8>,
sample_depth: u8,
pallette: png_splt_pallette
}

#[derive(Debug, Copy, Clone)]
pub struct png_srgb {
rendering_intent: u8
}

#[derive(Debug, Clone)]
pub struct png_text {
keyword: Vec<u8>,
text: Vec<u8>
}

#[derive(Debug, Copy, Clone)]
pub struct png_time {
year: u16,
month: u8,
day: u8,
hour: u8,
minute: u8,
second: u8
}

#[derive(Debug, Clone)]
pub enum png_trns { color_type_0(png_bkgd_color_type_0), color_type_2(png_bkgd_color_type_2), color_type_3(Vec<png_bkgd_color_type_3>) }

#[derive(Debug, Clone)]
pub struct png_ztxt {
keyword: Vec<u8>,
compression_method: u8,
compressed_text: Vec<char>
}

#[derive(Debug, Clone)]
pub enum png_chunk_data { PLTE(Vec<png_plte>), bKGD(png_bkgd), cHRM(png_chrm), gAMA(png_gama), hIST(png_hist), iCCP(png_iccp), iTXt(png_itxt), pHYs(png_phys), sBIT(png_sbit), sPLT(png_splt), sRGB(png_srgb), tEXt(png_text), tIME(png_time), tRNS(png_trns), unknown(Vec<u8>), zTXt(png_ztxt) }

#[derive(Debug, Clone)]
pub struct png_chunk {
length: u32,
tag: Vec<u8>,
data: png_chunk_data,
crc: u32
}

#[derive(Debug, Copy, Clone)]
pub struct png_iend {
length: u32,
tag: (u8, u8, u8, u8),
data: (),
crc: u32
}

#[derive(Debug, Clone)]
pub struct png_main {
signature: (u8, u8, u8, u8, u8, u8, u8, u8),
ihdr: png_ihdr,
chunks: Vec<png_chunk>,
idat: zlib_main,
more_chunks: Vec<png_chunk>,
iend: png_iend
}

#[derive(Debug, Clone)]
pub struct riff_chunk {
tag: (u8, u8, u8, u8),
length: u32,
data: Vec<u8>,
pad: Option<u8>
}

#[derive(Debug, Clone)]
pub struct riff_subchunks {
tag: (u8, u8, u8, u8),
chunks: Vec<riff_chunk>
}

#[derive(Debug, Clone)]
pub struct riff_main {
tag: (u8, u8, u8, u8),
length: u32,
data: riff_subchunks,
pad: Option<u8>
}

#[derive(Debug, Clone)]
pub struct tar_ascii_string_opt0 {
string: Vec<u8>,
__padding: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct tar_header_uid {
string: Vec<u8>,
__nul_or_wsp: u8,
__padding: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct tar_ascii_string {
string: Vec<u8>,
padding: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct tar_header {
name: tar_ascii_string_opt0,
mode: tar_header_uid,
uid: tar_header_uid,
gid: tar_header_uid,
size: u32,
mtime: tar_header_uid,
chksum: tar_header_uid,
typeflag: u8,
linkname: tar_ascii_string_opt0,
magic: (u8, u8, u8, u8, u8, u8),
version: (u8, u8),
uname: tar_ascii_string,
gname: tar_ascii_string,
devmajor: tar_header_uid,
devminor: tar_header_uid,
prefix: tar_ascii_string_opt0,
pad: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct tar_header_with_data {
header: tar_header,
file: Vec<u8>,
__padding: ()
}

#[derive(Debug, Clone)]
pub struct tar_main {
contents: Vec<tar_header_with_data>,
__padding: Vec<u8>,
__trailing: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct waldo_main {
r#where: u64,
noise: Vec<u8>,
sep: u8,
here: u64,
waldo: (u8, u8, u8, u8, u8),
__rem: ()
}

#[derive(Debug, Clone)]
pub enum main_data { elf(elf_main), gif(gif_main), gzip(Vec<gzip_main>), jpeg(jpeg_main), mpeg4(mpeg4_main), opentype(opentype_main), peano(Vec<u32>), png(png_main), riff(riff_main), tar(tar_main), text(Vec<char>), tgz(Vec<tar_main>), tiff(tiff_main), waldo(waldo_main) }

#[derive(Debug, Copy, Clone)]
pub struct tar_header_size_raw {
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
pub struct png_idat {
length: u32,
tag: (u8, u8, u8, u8),
data: Vec<u8>,
crc: u32
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_cmap_subtable_format14_raw_raw {
format: u16
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_cmap_subtable_format13_raw_raw {
format: u16,
__reserved: u16
}

#[derive(Debug, Clone)]
pub struct opentype_glyf_simple_flags_raw {
repeats: u8,
field_set: opentype_glyf_simple_flags
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_glyph_description_simple_flags_raw {
overlap_simple: bool,
y_is_same_or_positive_y_short_vector: bool,
x_is_same_or_positive_x_short_vector: bool,
repeat_flag: bool,
y_short_vector: bool,
x_short_vector: bool,
on_curve_point: bool
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_common_device_or_variation_index_table_raw_raw {
__skipped0: u16,
__skipped1: u16
}

#[derive(Debug, Copy, Clone)]
pub struct opentype_gpos_table_lookup_list_link_raw_lookups_link_raw_lookup_flag {
use_mark_filtering_set: bool,
ignore_marks: bool,
ignore_ligatures: bool,
ignore_base_glyphs: bool,
right_to_left: bool
}

#[derive(Debug, Clone)]
pub struct main {
data: main_data,
end: ()
}

fn Decoder_main<'input>(_input: &mut Parser<'input>) -> Result<main, ParseError> {
PResult::Ok((Decoder1(_input))?)
}

fn Decoder1<'input>(_input: &mut Parser<'input>) -> Result<main, ParseError> {
let data = ((|| {
_input.start_alt();
{
let mut f_tmp = || PResult::Ok({
let inner = (Decoder_waldo_main(_input))?;
main_data::waldo(inner)
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
let inner = (Decoder_gif_main(_input))?;
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
let inner = (Decoder5(_input))?;
main_data::tgz(inner)
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
let inner = (Decoder_jpeg_main(_input))?;
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
let inner = (Decoder_mpeg4_main(_input))?;
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
let inner = (Decoder_png_main(_input))?;
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
let inner = (Decoder_riff_main(_input))?;
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
let inner = (Decoder_tiff_main(_input))?;
main_data::tiff(inner)
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
let inner = (Decoder_tar_main(_input))?;
main_data::tar(inner)
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
let inner = (Decoder_elf_main(_input))?;
main_data::elf(inner)
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
let inner = (Decoder_opentype_main(_input))?;
main_data::opentype(inner)
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
let inner = (Decoder15(_input))?;
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

fn Decoder_waldo_main<'input>(_input: &mut Parser<'input>) -> Result<waldo_main, ParseError> {
let r#where = ((|| PResult::Ok((Decoder61(_input))?))())?;
let noise = ((|| PResult::Ok({
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
255u8 => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(3150449634277112009u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10618271977672484401u64));
}
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let sep = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
let here = ((|| PResult::Ok(_input.get_offset_u64()))())?;
let waldo = ((|| PResult::Ok({
_input.open_peek_context();
_input.advance_by(try_sub!(r#where, here))?;
let ret = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 87 {
b
} else {
return Err(ParseError::ExcludedBranch(5345789703123215803u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 97 {
b
} else {
return Err(ParseError::ExcludedBranch(5987471249031546739u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 108 {
b
} else {
return Err(ParseError::ExcludedBranch(1563474733986838367u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 100 {
b
} else {
return Err(ParseError::ExcludedBranch(12927144697814012286u64));
}
}))())?;
let field4 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 111 {
b
} else {
return Err(ParseError::ExcludedBranch(16448567210866195308u64));
}
}))())?;
(field0, field1, field2, field3, field4)
}))())?;
_input.close_peek_context()?;
ret
}))())?;
let __rem = ((|| PResult::Ok(_input.skip_remainder()))())?;
PResult::Ok(waldo_main { r#where, noise, sep, here, waldo, __rem })
}

fn Decoder3<'input>(_input: &mut Parser<'input>) -> Result<Vec<u32>, ParseError> {
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
let next_elem = (Decoder290(_input))?;
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder_gif_main<'input>(_input: &mut Parser<'input>) -> Result<gif_main, ParseError> {
let header = ((|| PResult::Ok((Decoder_gif_header(_input))?))())?;
let logical_screen = ((|| PResult::Ok((Decoder_gif_logical_screen(_input))?))())?;
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
let next_elem = (Decoder_gif_block(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let trailer = ((|| PResult::Ok((Decoder_gif_trailer(_input))?))())?;
PResult::Ok(gif_main { header, logical_screen, blocks, trailer })
}

fn Decoder5<'input>(_input: &mut Parser<'input>) -> Result<Vec<tar_main>, ParseError> {
let gzip_raw = (Decoder267(_input))?;
let mut accum = Vec::new();
for item in gzip_raw.clone() {
accum.push({
let mut tmp = Parser::new(item.data.inflate.as_slice());
let reparser = &mut tmp;
(Decoder_tar_main(reparser))?
});
}
PResult::Ok(accum)
}

fn Decoder6<'input>(_input: &mut Parser<'input>) -> Result<Vec<gzip_main>, ParseError> {
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
let header = ((|| PResult::Ok((Decoder_gzip_header(_input))?))())?;
let fextra = ((|| PResult::Ok(if header.file_flags.fextra.clone() {
Some((Decoder_gzip_fextra(_input))?)
} else {
None
}))())?;
let fname = ((|| PResult::Ok(if header.file_flags.fname.clone() {
Some((Decoder260(_input))?)
} else {
None
}))())?;
let fcomment = ((|| PResult::Ok(if header.file_flags.fcomment.clone() {
Some((Decoder_gzip_fcomment(_input))?)
} else {
None
}))())?;
let fhcrc = ((|| PResult::Ok(if header.file_flags.fhcrc.clone() {
Some((Decoder_gzip_fhcrc(_input))?)
} else {
None
}))())?;
let data = ((|| PResult::Ok({
_input.enter_bits_mode()?;
let ret = ((|| PResult::Ok((Decoder_deflate_main(_input))?))())?;
let _bits_read = _input.escape_bits_mode()?;
ret
}))())?;
let footer = ((|| PResult::Ok((Decoder_gzip_footer(_input))?))())?;
gzip_main { header, fextra, fname, fcomment, fhcrc, data, footer }
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder_jpeg_main<'input>(_input: &mut Parser<'input>) -> Result<jpeg_main, ParseError> {
let soi = ((|| PResult::Ok((Decoder_jpeg_eoi(_input))?))())?;
let frame = ((|| PResult::Ok((Decoder_jpeg_frame(_input))?))())?;
let eoi = ((|| PResult::Ok((Decoder189(_input))?))())?;
PResult::Ok(jpeg_main { soi, frame, eoi })
}

fn Decoder_mpeg4_main<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_main, ParseError> {
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
let next_elem = (Decoder_mpeg4_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(mpeg4_main { atoms })
}

fn Decoder_png_main<'input>(_input: &mut Parser<'input>) -> Result<png_main, ParseError> {
let signature = ((|| PResult::Ok((Decoder118(_input))?))())?;
let ihdr = ((|| PResult::Ok((Decoder_png_ihdr(_input))?))())?;
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
73u8 => {
let b = _input.read_byte()?;
if b == 68 {
1
} else {
return Err(ParseError::ExcludedBranch(7585197821406313821u64));
}
},

tmp if ((ByteSet::from_bits([18446744069414594048, 18446744073709551103, 0, 0])).contains(tmp)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(4734424520366274749u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_png_chunk(_input, ihdr.clone()))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let idat = ((|| PResult::Ok({
let idat = {
let inner = {
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

tmp if ((ByteSet::from_bits([18446744069414594048, 18446744073709551103, 0, 0])).contains(tmp)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(17936225909659650574u64));
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
let next_elem = (Decoder_png_idat(_input))?;
accum.push(next_elem);
}
}
accum
};
((|xs: Vec<png_idat>| PResult::Ok((try_flat_map_vec(xs.iter().cloned(), |x: png_idat| PResult::Ok(x.data.clone())))?))(inner))?
};
let mut tmp = Parser::new(idat.as_slice());
let reparser = &mut tmp;
(Decoder_zlib_main(reparser))?
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
73u8 => {
let b = _input.read_byte()?;
if b == 69 {
1
} else {
return Err(ParseError::ExcludedBranch(8433953923663074423u64));
}
},

tmp if ((ByteSet::from_bits([18446744069414594048, 18446744073709551103, 0, 0])).contains(tmp)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(15821618850664709184u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_png_chunk(_input, ihdr.clone()))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let iend = ((|| PResult::Ok((Decoder_png_iend(_input))?))())?;
PResult::Ok(png_main { signature, ihdr, chunks, idat, more_chunks, iend })
}

fn Decoder_riff_main<'input>(_input: &mut Parser<'input>) -> Result<riff_main, ParseError> {
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
let length = ((|| PResult::Ok((Decoder89(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_riff_subchunks(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let pad = ((|| PResult::Ok(if length % 2u32 == 1u32 {
let b = _input.read_byte()?;
Some(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
})
} else {
None
}))())?;
PResult::Ok(riff_main { tag, length, data, pad })
}

fn Decoder_tiff_main<'input>(_input: &mut Parser<'input>) -> Result<tiff_main, ParseError> {
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
tiff_main_byte_order::le(field0, field1)
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
tiff_main_byte_order::be(field0, field1)
},

_ => {
return Err(ParseError::ExcludedBranch(8662494850867647108u64));
}
}
}))())?;
let magic = ((|| PResult::Ok(match byte_order {
tiff_main_byte_order::le(..) => {
(Decoder101(_input))?
},

tiff_main_byte_order::be(..) => {
(Decoder24(_input))?
}
}))())?;
let offset = ((|| PResult::Ok(match byte_order {
tiff_main_byte_order::le(..) => {
(Decoder89(_input))?
},

tiff_main_byte_order::be(..) => {
(Decoder21(_input))?
}
}))())?;
let ifd = ((|| PResult::Ok({
_input.open_peek_context();
_input.advance_by(try_sub!(offset, 8u32))?;
let ret = ((|| PResult::Ok(match byte_order {
tiff_main_byte_order::le(..) => {
let num_fields = ((|| PResult::Ok((Decoder101(_input))?))())?;
let fields = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_fields {
accum.push({
let tag = ((|| PResult::Ok((Decoder101(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder101(_input))?))())?;
let length = ((|| PResult::Ok((Decoder89(_input))?))())?;
let offset_or_data = ((|| PResult::Ok((Decoder89(_input))?))())?;
tiff_main_ifd_fields { tag, r#type, length, offset_or_data }
});
}
accum
}))())?;
let next_ifd_offset = ((|| PResult::Ok((Decoder89(_input))?))())?;
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
tiff_main_ifd { num_fields, fields, next_ifd_offset, next_ifd }
},

tiff_main_byte_order::be(..) => {
let num_fields = ((|| PResult::Ok((Decoder24(_input))?))())?;
let fields = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_fields {
accum.push({
let tag = ((|| PResult::Ok((Decoder24(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder24(_input))?))())?;
let length = ((|| PResult::Ok((Decoder21(_input))?))())?;
let offset_or_data = ((|| PResult::Ok((Decoder21(_input))?))())?;
tiff_main_ifd_fields { tag, r#type, length, offset_or_data }
});
}
accum
}))())?;
let next_ifd_offset = ((|| PResult::Ok((Decoder21(_input))?))())?;
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
tiff_main_ifd { num_fields, fields, next_ifd_offset, next_ifd }
}
}))())?;
_input.close_peek_context()?;
ret
}))())?;
PResult::Ok(tiff_main { byte_order, magic, offset, ifd })
}

fn Decoder_tar_main<'input>(_input: &mut Parser<'input>) -> Result<tar_main, ParseError> {
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
let next_elem = (Decoder_tar_header_with_data(_input))?;
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
PResult::Ok(tar_main { contents, __padding, __trailing })
}

fn Decoder_elf_main<'input>(_input: &mut Parser<'input>) -> Result<elf_main, ParseError> {
let header = ((|| PResult::Ok((Decoder_elf_header(_input))?))())?;
let __eoh = ((|| PResult::Ok(_input.get_offset_u64()))())?;
let program_headers = ((|| PResult::Ok(if match header.phoff.clone() {
elf_types_elf_off::Off32(0u32) => {
false
},

elf_types_elf_off::Off64(0u64) => {
false
},

_ => {
true
}
} {
_input.open_peek_context();
_input.advance_by(try_sub!(match header.phoff.clone() {
elf_types_elf_off::Off32(x32) => {
x32 as u64
},

elf_types_elf_off::Off64(x64) => {
x64
}
}, __eoh))?;
let ret = ((|| PResult::Ok((Decoder76(_input, header.ident.data.clone() == 2u8, header.ident.class.clone(), header.phnum.clone()))?))())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
}))())?;
let section_headers = ((|| PResult::Ok(if match header.shoff.clone() {
elf_types_elf_off::Off32(0u32) => {
false
},

elf_types_elf_off::Off64(0u64) => {
false
},

_ => {
true
}
} {
_input.open_peek_context();
_input.advance_by(try_sub!(match header.shoff.clone() {
elf_types_elf_off::Off32(x32) => {
x32 as u64
},

elf_types_elf_off::Off64(x64) => {
x64
}
}, __eoh))?;
let ret = ((|| PResult::Ok((Decoder77(_input, header.ident.data.clone() == 2u8, header.ident.class.clone(), header.shnum.clone()))?))())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
}))())?;
let sections = ((|| PResult::Ok(match section_headers {
Some(ref shdrs) => {
let inner = {
let mut accum = Vec::new();
for shdr in shdrs.clone() {
accum.push(if (shdr.r#type.clone() != 8u32) && (shdr.r#type.clone() != 0u32) {
_input.open_peek_context();
_input.advance_by(try_sub!(match shdr.offset.clone() {
elf_types_elf_off::Off32(x32) => {
x32 as u64
},

elf_types_elf_off::Off64(x64) => {
x64
}
}, __eoh))?;
let ret = ((|| PResult::Ok((Decoder78(_input, shdr.r#type.clone(), match shdr.size.clone() {
elf_types_elf_full::Full32(x32) => {
x32 as u64
},

elf_types_elf_full::Full64(x64) => {
x64.clone()
}
}))?))())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
});
}
accum
};
((|val: Vec<Option<Vec<u8>>>| PResult::Ok(Some(val)))(inner))?
},

None => {
None
}
}))())?;
let __skip = ((|| PResult::Ok(_input.skip_remainder()))())?;
PResult::Ok(elf_main { header, __eoh, program_headers, section_headers, sections, __skip })
}

fn Decoder_opentype_main<'input>(_input: &mut Parser<'input>) -> Result<opentype_main, ParseError> {
let start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let font = ((|| PResult::Ok((Decoder_opentype_font(_input, start.clone()))?))())?;
PResult::Ok(opentype_main { start, font })
}

fn Decoder15<'input>(_input: &mut Parser<'input>) -> Result<Vec<char>, ParseError> {
PResult::Ok((Decoder16(_input))?)
}

fn Decoder16<'input>(_input: &mut Parser<'input>) -> Result<Vec<char>, ParseError> {
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

tmp if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(tmp)) => {
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
return Err(ParseError::ExcludedBranch(975831965879443532u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder17(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
}

fn Decoder17<'input>(_input: &mut Parser<'input>) -> Result<char, ParseError> {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
0u8 => {
0
},

tmp if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(tmp)) => {
1
},

tmp if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(tmp)) => {
1
},

224u8 => {
1
},

tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => {
1
},

237u8 => {
1
},

tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => {
1
},

240u8 => {
1
},

tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => {
1
},

244u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(7131706841387856848u64));
}
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
};
((|_: u8| PResult::Ok((char::from_u32(0u32)).unwrap()))(inner))?
},

1 => {
(Decoder18(_input))?
},

_ => {
return Err(ParseError::ExcludedBranch(11321684005377502486u64));
}
})
}

fn Decoder18<'input>(_input: &mut Parser<'input>) -> Result<char, ParseError> {
let inner = {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(tmp)) => {
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
return Err(ParseError::ExcludedBranch(7852662122060720972u64));
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
if (ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(5751201182572520699u64));
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
let field1 = ((|| PResult::Ok((Decoder19(_input))?))())?;
(field0, field1)
};
((|tuple_var: (u8, u8)| PResult::Ok(match tuple_var {
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
let field2 = ((|| PResult::Ok((Decoder19(_input))?))())?;
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
let field1 = ((|| PResult::Ok((Decoder19(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder19(_input))?))())?;
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
let field2 = ((|| PResult::Ok((Decoder19(_input))?))())?;
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
let field1 = ((|| PResult::Ok((Decoder19(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder19(_input))?))())?;
(field0, field1, field2)
},

_ => {
return Err(ParseError::ExcludedBranch(16680599075360251934u64));
}
}
};
((|tuple_var: (u8, u8, u8)| PResult::Ok(match tuple_var {
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
let field2 = ((|| PResult::Ok((Decoder19(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder19(_input))?))())?;
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
let field1 = ((|| PResult::Ok((Decoder19(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder19(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder19(_input))?))())?;
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
let field2 = ((|| PResult::Ok((Decoder19(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder19(_input))?))())?;
(field0, field1, field2, field3)
},

_ => {
return Err(ParseError::ExcludedBranch(10897670729404727847u64));
}
}
};
((|tuple_var: (u8, u8, u8, u8)| PResult::Ok(match tuple_var {
(x3, x2, x1, x0) => {
(x3 as u32) << 18u32 | (x2 as u32) << 12u32 | (x1 as u32) << 6u32 | (x0 as u32)
}
}))(inner))?
},

_ => {
return Err(ParseError::ExcludedBranch(12705355269156555156u64));
}
}
};
PResult::Ok(((|codepoint: u32| PResult::Ok((char::from_u32(codepoint)).unwrap()))(inner))?)
}

fn Decoder19<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
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

fn Decoder_opentype_font<'input>(_input: &mut Parser<'input>, start: u32) -> Result<opentype_font, ParseError> {
let magic = ((|| PResult::Ok({
_input.open_peek_context();
let ret = ((|| PResult::Ok((Decoder21(_input))?))())?;
_input.close_peek_context()?;
ret
}))())?;
let directory = ((|| PResult::Ok(match magic {
65536u32 => {
let inner = (Decoder_opentype_table_directory(_input, start.clone()))?;
opentype_font_directory::TableDirectory(inner)
},

1330926671u32 => {
let inner = (Decoder_opentype_table_directory(_input, start.clone()))?;
opentype_font_directory::TableDirectory(inner)
},

1953784678u32 => {
let inner = (Decoder_opentype_ttc_header(_input, start.clone()))?;
opentype_font_directory::TTCHeader(inner)
},

_ => {
return Err(ParseError::FailToken);
}
}))())?;
PResult::Ok(opentype_font { magic, directory })
}

fn Decoder21<'input>(_input: &mut Parser<'input>) -> Result<u32, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2, field3)
};
PResult::Ok(((|x: (u8, u8, u8, u8)| PResult::Ok(u32be(x)))(inner))?)
}

fn Decoder_opentype_table_directory<'input>(_input: &mut Parser<'input>, start: u32) -> Result<opentype_table_directory, ParseError> {
let sfnt_version = ((|| PResult::Ok({
let inner = (Decoder21(_input))?;
if ((|v: u32| PResult::Ok((v == 65536u32) || (v == 1330926671u32)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let num_tables = ((|| PResult::Ok((Decoder24(_input))?))())?;
let search_range = ((|| PResult::Ok((Decoder24(_input))?))())?;
let entry_selector = ((|| PResult::Ok((Decoder24(_input))?))())?;
let range_shift = ((|| PResult::Ok((Decoder24(_input))?))())?;
let table_records = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_tables {
accum.push((Decoder_opentype_table_record(_input, start.clone()))?);
}
accum
}))())?;
let table_links = ((|| PResult::Ok((Decoder_opentype_table_directory_table_links(_input, start.clone(), table_records.clone()))?))())?;
PResult::Ok(opentype_table_directory { sfnt_version, num_tables, search_range, entry_selector, range_shift, table_records, table_links })
}

fn Decoder_opentype_ttc_header<'input>(_input: &mut Parser<'input>, start: u32) -> Result<opentype_ttc_header, ParseError> {
let ttc_tag = ((|| PResult::Ok({
let inner = (Decoder21(_input))?;
if ((|tag: u32| PResult::Ok(tag == 1953784678u32))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let major_version = ((|| PResult::Ok((Decoder24(_input))?))())?;
let minor_version = ((|| PResult::Ok((Decoder24(_input))?))())?;
let header = ((|| PResult::Ok(match major_version {
1u16 => {
let inner = {
let num_fonts = ((|| PResult::Ok((Decoder21(_input))?))())?;
let table_directories = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_fonts {
accum.push({
let offset = ((|| PResult::Ok((Decoder21(_input))?))())?;
let link = ((|| PResult::Ok(if offset != 0u32 {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + offset, __here))?;
let ret = ((|| PResult::Ok((Decoder_opentype_table_directory(_input, start.clone()))?))())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
}))())?;
opentype_ttc_header_header_Version1_table_directories { offset, link }
});
}
accum
}))())?;
opentype_ttc_header_header_Version1 { num_fonts, table_directories }
};
opentype_ttc_header_header::Version1(inner)
},

2u16 => {
let inner = {
let num_fonts = ((|| PResult::Ok((Decoder21(_input))?))())?;
let table_directories = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_fonts {
accum.push({
let offset = ((|| PResult::Ok((Decoder21(_input))?))())?;
let link = ((|| PResult::Ok(if offset != 0u32 {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + offset, __here))?;
let ret = ((|| PResult::Ok((Decoder_opentype_table_directory(_input, start.clone()))?))())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
}))())?;
opentype_ttc_header_header_Version1_table_directories { offset, link }
});
}
accum
}))())?;
let dsig_tag = ((|| PResult::Ok((Decoder21(_input))?))())?;
let dsig_length = ((|| PResult::Ok((Decoder21(_input))?))())?;
let dsig_offset = ((|| PResult::Ok((Decoder21(_input))?))())?;
opentype_ttc_header_header_Version2 { num_fonts, table_directories, dsig_tag, dsig_length, dsig_offset }
};
opentype_ttc_header_header::Version2(inner)
},

unknown => {
let inner = unknown.clone();
opentype_ttc_header_header::UnknownVersion(inner)
}
}))())?;
let __skip = ((|| PResult::Ok(_input.skip_remainder()))())?;
PResult::Ok(opentype_ttc_header { ttc_tag, major_version, minor_version, header, __skip })
}

fn Decoder24<'input>(_input: &mut Parser<'input>) -> Result<u16, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1)
};
PResult::Ok(((|x: (u8, u8)| PResult::Ok(u16be(x)))(inner))?)
}

fn Decoder25<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(b)
}

fn Decoder_opentype_table_record<'input>(_input: &mut Parser<'input>, start: u32) -> Result<opentype_table_record, ParseError> {
let table_id = ((|| PResult::Ok((Decoder48(_input))?))())?;
let checksum = ((|| PResult::Ok((Decoder21(_input))?))())?;
let offset = ((|| PResult::Ok((Decoder21(_input))?))())?;
let length = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(opentype_table_record { table_id, checksum, offset, length })
}

fn Decoder_opentype_table_directory_table_links<'input>(_input: &mut Parser<'input>, start: u32, tables: Vec<opentype_table_record>) -> Result<opentype_table_directory_table_links, ParseError> {
let cmap = ((|| PResult::Ok({
let matching_table = match match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1668112752u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1668112752u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
} {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(2381717495940604803u64));
}
};
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + matching_table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (matching_table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_cmap_table(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
}))())?;
let head = ((|| PResult::Ok({
let matching_table = match match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1751474532u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1751474532u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
} {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(13414249021383648927u64));
}
};
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + matching_table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (matching_table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_head_table(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
}))())?;
let hhea = ((|| PResult::Ok({
let matching_table = match match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1751672161u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1751672161u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
} {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(2388446886758135460u64));
}
};
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + matching_table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (matching_table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_hhea_table(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
}))())?;
let maxp = ((|| PResult::Ok({
let matching_table = match match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1835104368u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1835104368u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
} {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(3272934374665651290u64));
}
};
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + matching_table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (matching_table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_maxp_table(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
}))())?;
let hmtx = ((|| PResult::Ok({
let matching_table = match match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1752003704u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1752003704u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
} {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(10983496763888046882u64));
}
};
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + matching_table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (matching_table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_hmtx_table(_input, hhea.number_of_long_horizontal_metrics.clone(), maxp.num_glyphs.clone()))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
}))())?;
let name = ((|| PResult::Ok({
let matching_table = match match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1851878757u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1851878757u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
} {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(11387878137723461222u64));
}
};
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + matching_table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (matching_table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_name_table(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
}))())?;
let os2 = ((|| PResult::Ok({
let matching_table = match match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1330851634u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1330851634u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
} {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(16053312509972943637u64));
}
};
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + matching_table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (matching_table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_os2_table(_input, matching_table.length.clone()))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
}))())?;
let post = ((|| PResult::Ok({
let matching_table = match match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1886352244u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1886352244u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
} {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(6462592155365958452u64));
}
};
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + matching_table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (matching_table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_post_table(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
}))())?;
let cvt = ((|| PResult::Ok({
let matching_table = match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1668707360u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1668707360u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
};
match matching_table {
Some(ref table) => {
let inner = {
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (table.length.clone()) as usize<>;
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
let next_elem = (Decoder24(_input))?;
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
_input.close_peek_context()?;
ret
};
((|val: Vec<u16>| PResult::Ok(Some(val)))(inner))?
},

None => {
None
}
}
}))())?;
let fpgm = ((|| PResult::Ok({
let matching_table = match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1718642541u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1718642541u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
};
match matching_table {
Some(ref table) => {
let inner = {
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (table.length.clone()) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
_input.close_peek_context()?;
ret
};
((|val: Vec<u8>| PResult::Ok(Some(val)))(inner))?
},

None => {
None
}
}
}))())?;
let loca = ((|| PResult::Ok({
let matching_table = match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1819239265u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1819239265u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
};
match matching_table {
Some(ref table) => {
let inner = {
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_loca_table(_input, maxp.num_glyphs.clone(), head.index_to_loc_format.clone()))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
};
((|val: opentype_loca_table| PResult::Ok(Some(val)))(inner))?
},

None => {
None
}
}
}))())?;
let glyf = ((|| PResult::Ok({
let matching_table = match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1735162214u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1735162214u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
};
match matching_table {
Some(ref table) => {
let inner = {
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder37(_input, match loca {
Some(ref x) => {
(try_fold_map_curried(match x.offsets.clone() {
opentype_loca_table_offsets::Offsets16(half16s) => {
(try_flat_map_vec(half16s.iter().cloned(), |half16: u16| PResult::Ok([(half16 as u32) * 2u32].to_vec())))?
},

opentype_loca_table_offsets::Offsets32(off32s) => {
off32s
}
}.iter().cloned(), None, |tuple_var: (Option<u32>, u32)| PResult::Ok(match tuple_var {
(last_value, value) => {
(Some(value.clone()), match last_value {
Some(x) => {
[(x.clone(), value.clone())].to_vec()
},

None => {
[].to_vec()
}
})
}
})))?
},

None => {
[].to_vec()
}
}))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
};
((|val: Vec<opentype_glyf_table>| PResult::Ok(Some(val)))(inner))?
},

None => {
None
}
}
}))())?;
let prep = ((|| PResult::Ok({
let matching_table = match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1886545264u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1886545264u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
};
match matching_table {
Some(ref table) => {
let inner = {
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (table.length.clone()) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
_input.close_peek_context()?;
ret
};
((|val: Vec<u8>| PResult::Ok(Some(val)))(inner))?
},

None => {
None
}
}
}))())?;
let gasp = ((|| PResult::Ok({
let matching_table = match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1734439792u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1734439792u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
};
match matching_table {
Some(ref table) => {
let inner = {
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_gasp_table(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
};
((|val: opentype_gasp_table| PResult::Ok(Some(val)))(inner))?
},

None => {
None
}
}
}))())?;
let base = ((|| PResult::Ok({
let matching_table = match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1111577413u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1111577413u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
};
match matching_table {
Some(ref table) => {
let inner = {
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder39(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
};
((|val: ()| PResult::Ok(Some(val)))(inner))?
},

None => {
None
}
}
}))())?;
let gdef = ((|| PResult::Ok({
let matching_table = match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1195656518u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1195656518u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
};
match matching_table {
Some(ref table) => {
let inner = {
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_gdef_table(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
};
((|val: opentype_gdef_table| PResult::Ok(Some(val)))(inner))?
},

None => {
None
}
}
}))())?;
let gpos = ((|| PResult::Ok({
let matching_table = match 0u32 < (((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1196445523u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?.len()) as u32) {
true => {
Some((try_flat_map_vec(tables.iter().cloned(), |table: opentype_table_record| PResult::Ok(match table.table_id.clone() == 1196445523u32 {
true => {
[table.clone()].to_vec()
},

false => {
[].to_vec()
}
})))?[0u32 as usize])
},

false => {
None
}
};
match matching_table {
Some(ref table) => {
let inner = {
let offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + table.offset.clone(), offset))?;
let ret = ((|| PResult::Ok({
let sz = (table.length.clone()) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_opentype_gpos_table(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
_input.close_peek_context()?;
ret
};
((|val: opentype_gpos_table| PResult::Ok(Some(val)))(inner))?
},

None => {
None
}
}
}))())?;
let __skip = ((|| PResult::Ok(_input.skip_remainder()))())?;
PResult::Ok(opentype_table_directory_table_links { cmap, head, hhea, maxp, hmtx, name, os2, post, cvt, fpgm, loca, glyf, prep, gasp, base, gdef, gpos, __skip })
}

fn Decoder_opentype_cmap_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_cmap_table, ParseError> {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let version = ((|| PResult::Ok((Decoder24(_input))?))())?;
let num_tables = ((|| PResult::Ok((Decoder24(_input))?))())?;
let encoding_records = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_tables {
accum.push((Decoder_opentype_encoding_record(_input, table_start.clone()))?);
}
accum
}))())?;
PResult::Ok(opentype_cmap_table { table_start, version, num_tables, encoding_records })
}

fn Decoder_opentype_head_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_head_table, ParseError> {
let major_version = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 1u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let minor_version = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let font_revision = ((|| PResult::Ok({
let inner = (Decoder21(_input))?;
((|x: u32| PResult::Ok(opentype_post_table_italic_angle::Fixed32(x)))(inner))?
}))())?;
let checksum_adjustment = ((|| PResult::Ok((Decoder21(_input))?))())?;
let magic_number = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 95 {
b
} else {
return Err(ParseError::ExcludedBranch(3297054456412973289u64));
}
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 15 {
b
} else {
return Err(ParseError::ExcludedBranch(11987894655311254510u64));
}
}))())?;
let field2 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 60 {
b
} else {
return Err(ParseError::ExcludedBranch(17424275857002596811u64));
}
}))())?;
let field3 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 245 {
b
} else {
return Err(ParseError::ExcludedBranch(1867519841059747314u64));
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let flags = ((|| PResult::Ok((Decoder24(_input))?))())?;
let units_per_em = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok((x >= 16u16) && (x <= 16384u16)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let created = ((|| PResult::Ok((Decoder60(_input))?))())?;
let modified = ((|| PResult::Ok((Decoder60(_input))?))())?;
let glyph_extents = ((|| PResult::Ok({
let x_min = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_min = ((|| PResult::Ok((Decoder24(_input))?))())?;
let x_max = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_max = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_head_table_glyph_extents { x_min, y_min, x_max, y_max }
}))())?;
let mac_style = ((|| PResult::Ok({
let inner = {
let inner = {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
b
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
b
}))())?;
(field0, field1)
};
((|x: (u8, u8)| PResult::Ok(u16be(x)))(inner))?
};
((|flagbits: u16| PResult::Ok(opentype_head_table_mac_style { extended: flagbits >> 6u16 & 1u16 != 0u16, condensed: flagbits >> 5u16 & 1u16 != 0u16, shadow: flagbits >> 4u16 & 1u16 != 0u16, outline: flagbits >> 3u16 & 1u16 != 0u16, underline: flagbits >> 2u16 & 1u16 != 0u16, italic: flagbits >> 1u16 & 1u16 != 0u16, bold: flagbits >> 0u16 & 1u16 != 0u16 }))(inner))?
}))())?;
let lowest_rec_ppem = ((|| PResult::Ok((Decoder24(_input))?))())?;
let font_direction_hint = ((|| PResult::Ok((Decoder24(_input))?))())?;
let index_to_loc_format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x <= 1u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let glyph_data_format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
PResult::Ok(opentype_head_table { major_version, minor_version, font_revision, checksum_adjustment, magic_number, flags, units_per_em, created, modified, glyph_extents, mac_style, lowest_rec_ppem, font_direction_hint, index_to_loc_format, glyph_data_format })
}

fn Decoder_opentype_hhea_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_hhea_table, ParseError> {
let major_version = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 1u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let minor_version = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let ascent = ((|| PResult::Ok((Decoder24(_input))?))())?;
let descent = ((|| PResult::Ok((Decoder24(_input))?))())?;
let line_gap = ((|| PResult::Ok((Decoder24(_input))?))())?;
let advance_width_max = ((|| PResult::Ok((Decoder24(_input))?))())?;
let min_left_side_bearing = ((|| PResult::Ok((Decoder24(_input))?))())?;
let min_right_side_bearing = ((|| PResult::Ok((Decoder24(_input))?))())?;
let x_max_extent = ((|| PResult::Ok((Decoder24(_input))?))())?;
let caret_slope = ((|| PResult::Ok({
let rise = ((|| PResult::Ok((Decoder24(_input))?))())?;
let run = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_hhea_table_caret_slope { rise, run }
}))())?;
let caret_offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let __reservedX4 = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let field1 = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let field2 = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let field3 = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
(field0, field1, field2, field3)
}))())?;
let metric_data_format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let number_of_long_horizontal_metrics = ((|| PResult::Ok((Decoder24(_input))?))())?;
PResult::Ok(opentype_hhea_table { major_version, minor_version, ascent, descent, line_gap, advance_width_max, min_left_side_bearing, min_right_side_bearing, x_max_extent, caret_slope, caret_offset, __reservedX4, metric_data_format, number_of_long_horizontal_metrics })
}

fn Decoder_opentype_maxp_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_maxp_table, ParseError> {
let version = ((|| PResult::Ok((Decoder21(_input))?))())?;
let num_glyphs = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok(match version {
65536u32 => {
let inner = (Decoder_opentype_maxp_table_version1(_input))?;
opentype_maxp_table_data::MaxpV1(inner)
},

20480u32 => {
opentype_maxp_table_data::MaxpPostScript
},

unknown => {
let inner = unknown.clone();
opentype_maxp_table_data::MaxpUnknown(inner)
}
}))())?;
PResult::Ok(opentype_maxp_table { version, num_glyphs, data })
}

fn Decoder_opentype_hmtx_table<'input>(_input: &mut Parser<'input>, num_h_metrics: u16, num_glyphs: u16) -> Result<opentype_hmtx_table, ParseError> {
let h_metrics = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_h_metrics {
accum.push({
let advance_width = ((|| PResult::Ok((Decoder24(_input))?))())?;
let left_side_bearing = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_hmtx_table_h_metrics { advance_width, left_side_bearing }
});
}
accum
}))())?;
let left_side_bearings = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..try_sub!(num_glyphs, num_h_metrics) {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
PResult::Ok(opentype_hmtx_table { h_metrics, left_side_bearings })
}

fn Decoder_opentype_name_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_name_table, ParseError> {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let version = ((|| PResult::Ok((Decoder24(_input))?))())?;
let name_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let storage_offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let name_records = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..name_count {
accum.push({
let platform = ((|| PResult::Ok((Decoder24(_input))?))())?;
let encoding = ((|| PResult::Ok((Decoder24(_input))?))())?;
let language = ((|| PResult::Ok((Decoder24(_input))?))())?;
let name_id = ((|| PResult::Ok((Decoder24(_input))?))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let offset = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (storage_offset as u32) + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..length {
accum.push((Decoder25(_input))?);
}
accum
};
((|val: Vec<u8>| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_name_table_name_records_offset { offset, link }
}))())?;
opentype_name_table_name_records { platform, encoding, language, name_id, length, offset }
});
}
accum
}))())?;
let data = ((|| PResult::Ok(match version {
0u16 => {
opentype_name_table_data::NameVersion0
},

1u16 => {
let inner = (Decoder_opentype_name_table_name_version_1(_input, table_start + (storage_offset as u32)))?;
opentype_name_table_data::NameVersion1(inner)
},

unknown => {
let inner = unknown.clone();
opentype_name_table_data::NameVersionUnknown(inner)
}
}))())?;
PResult::Ok(opentype_name_table { table_start, version, name_count, storage_offset, name_records, data })
}

fn Decoder_opentype_os2_table<'input>(_input: &mut Parser<'input>, table_length: u32) -> Result<opentype_os2_table, ParseError> {
let version = ((|| PResult::Ok((Decoder24(_input))?))())?;
let x_avg_char_width = ((|| PResult::Ok((Decoder24(_input))?))())?;
let us_weight_class = ((|| PResult::Ok((Decoder24(_input))?))())?;
let us_width_class = ((|| PResult::Ok((Decoder24(_input))?))())?;
let fs_type = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_subscript_x_size = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_subscript_y_size = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_subscript_x_offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_subscript_y_offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_superscript_x_size = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_superscript_y_size = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_superscript_x_offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_superscript_y_offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_strikeout_size = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_strikeout_position = ((|| PResult::Ok((Decoder24(_input))?))())?;
let s_family_class = ((|| PResult::Ok((Decoder24(_input))?))())?;
let panose = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..10u8 {
accum.push((Decoder25(_input))?);
}
accum
}))())?;
let ul_unicode_range1 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let ul_unicode_range2 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let ul_unicode_range3 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let ul_unicode_range4 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let ach_vend_id = ((|| PResult::Ok((Decoder48(_input))?))())?;
let fs_selection = ((|| PResult::Ok((Decoder24(_input))?))())?;
let us_first_char_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
let us_last_char_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok(if (version != 0u16) || (table_length >= 78u32) {
let s_typo_ascender = ((|| PResult::Ok((Decoder24(_input))?))())?;
let s_typo_descender = ((|| PResult::Ok((Decoder24(_input))?))())?;
let s_typo_line_gap = ((|| PResult::Ok((Decoder24(_input))?))())?;
let us_win_ascent = ((|| PResult::Ok((Decoder24(_input))?))())?;
let us_win_descent = ((|| PResult::Ok((Decoder24(_input))?))())?;
let extra_fields_v1 = ((|| PResult::Ok(if version >= 1u16 {
let ul_code_page_range_1 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let ul_code_page_range_2 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let extra_fields_v2 = ((|| PResult::Ok(if version >= 2u16 {
let sx_height = ((|| PResult::Ok((Decoder24(_input))?))())?;
let s_cap_height = ((|| PResult::Ok((Decoder24(_input))?))())?;
let us_default_char = ((|| PResult::Ok((Decoder24(_input))?))())?;
let us_break_char = ((|| PResult::Ok((Decoder24(_input))?))())?;
let us_max_context = ((|| PResult::Ok((Decoder24(_input))?))())?;
let extra_fields_v5 = ((|| PResult::Ok(if version >= 5u16 {
let us_lower_optical_point_size = ((|| PResult::Ok((Decoder24(_input))?))())?;
let us_upper_optical_point_size = ((|| PResult::Ok((Decoder24(_input))?))())?;
Some(opentype_os2_table_data_extra_fields_v1_extra_fields_v2_extra_fields_v5 { us_lower_optical_point_size, us_upper_optical_point_size })
} else {
None
}))())?;
Some(opentype_os2_table_data_extra_fields_v1_extra_fields_v2 { sx_height, s_cap_height, us_default_char, us_break_char, us_max_context, extra_fields_v5 })
} else {
None
}))())?;
Some(opentype_os2_table_data_extra_fields_v1 { ul_code_page_range_1, ul_code_page_range_2, extra_fields_v2 })
} else {
None
}))())?;
Some(opentype_os2_table_data { s_typo_ascender, s_typo_descender, s_typo_line_gap, us_win_ascent, us_win_descent, extra_fields_v1 })
} else {
None
}))())?;
PResult::Ok(opentype_os2_table { version, x_avg_char_width, us_weight_class, us_width_class, fs_type, y_subscript_x_size, y_subscript_y_size, y_subscript_x_offset, y_subscript_y_offset, y_superscript_x_size, y_superscript_y_size, y_superscript_x_offset, y_superscript_y_offset, y_strikeout_size, y_strikeout_position, s_family_class, panose, ul_unicode_range1, ul_unicode_range2, ul_unicode_range3, ul_unicode_range4, ach_vend_id, fs_selection, us_first_char_index, us_last_char_index, data })
}

fn Decoder_opentype_post_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_post_table, ParseError> {
let version = ((|| PResult::Ok((Decoder21(_input))?))())?;
let italic_angle = ((|| PResult::Ok({
let inner = (Decoder21(_input))?;
((|x: u32| PResult::Ok(opentype_post_table_italic_angle::Fixed32(x)))(inner))?
}))())?;
let underline_position = ((|| PResult::Ok((Decoder24(_input))?))())?;
let underline_thickness = ((|| PResult::Ok((Decoder24(_input))?))())?;
let is_fixed_pitch = ((|| PResult::Ok((Decoder21(_input))?))())?;
let min_mem_type42 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let max_mem_type42 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let min_mem_type1 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let max_mem_type1 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let names = ((|| PResult::Ok(match version {
65536u32 => {
opentype_post_table_names::Version1
},

131072u32 => {
let inner = {
let num_glyphs = ((|| PResult::Ok((Decoder24(_input))?))())?;
let glyph_name_index = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_glyphs {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
let string_data = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
opentype_post_table_names_Version2 { num_glyphs, glyph_name_index, string_data }
};
opentype_post_table_names::Version2(inner)
},

151552u32 => {
let inner = {
let num_glyphs = ((|| PResult::Ok((Decoder24(_input))?))())?;
let offset = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_glyphs {
accum.push((Decoder25(_input))?);
}
accum
}))())?;
opentype_post_table_names_Version2Dot5 { num_glyphs, offset }
};
opentype_post_table_names::Version2Dot5(inner)
},

196608u32 => {
opentype_post_table_names::Version3
},

unknown => {
let inner = unknown.clone();
opentype_post_table_names::VersionUnknown(inner)
}
}))())?;
PResult::Ok(opentype_post_table { version, italic_angle, underline_position, underline_thickness, is_fixed_pitch, min_mem_type42, max_mem_type42, min_mem_type1, max_mem_type1, names })
}

fn Decoder_opentype_loca_table<'input>(_input: &mut Parser<'input>, num_glyphs: u16, index_to_loc_format: u16) -> Result<opentype_loca_table, ParseError> {
let offsets = ((|| PResult::Ok(match index_to_loc_format {
0u16 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..num_glyphs + 1u16 {
accum.push((Decoder24(_input))?);
}
accum
};
opentype_loca_table_offsets::Offsets16(inner)
},

1u16 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..num_glyphs + 1u16 {
accum.push((Decoder21(_input))?);
}
accum
};
opentype_loca_table_offsets::Offsets32(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
PResult::Ok(opentype_loca_table { offsets })
}

fn Decoder37<'input>(_input: &mut Parser<'input>, offset_pairs: Vec<(u32, u32)>) -> Result<Vec<opentype_glyf_table>, ParseError> {
let start_offset = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
let mut accum = Vec::new();
for offset_pair in offset_pairs.clone() {
accum.push(match offset_pair.1.clone() > offset_pair.0.clone() {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start_offset + offset_pair.0.clone(), __here))?;
let ret = ((|| PResult::Ok({
let inner = {
let number_of_contours = ((|| PResult::Ok((Decoder24(_input))?))())?;
let x_min = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_min = ((|| PResult::Ok((Decoder24(_input))?))())?;
let x_max = ((|| PResult::Ok((Decoder24(_input))?))())?;
let y_max = ((|| PResult::Ok((Decoder24(_input))?))())?;
let description = ((|| PResult::Ok((Decoder_opentype_glyf_description(_input, number_of_contours.clone()))?))())?;
opentype_glyf_table_Glyph { number_of_contours, x_min, y_min, x_max, y_max, description }
};
opentype_glyf_table::Glyph(inner)
}))())?;
_input.close_peek_context()?;
ret
},

false => {
opentype_glyf_table::EmptyGlyph
}
});
}
PResult::Ok(accum)
}

fn Decoder_opentype_gasp_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_gasp_table, ParseError> {
let version = ((|| PResult::Ok((Decoder24(_input))?))())?;
let num_ranges = ((|| PResult::Ok((Decoder24(_input))?))())?;
let gasp_ranges = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_ranges {
accum.push({
let range_max_ppem = ((|| PResult::Ok((Decoder24(_input))?))())?;
let range_gasp_behavior = ((|| PResult::Ok(match version {
0u16 => {
let inner = {
let inner = {
let inner = {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
b
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
b
}))())?;
(field0, field1)
};
((|x: (u8, u8)| PResult::Ok(u16be(x)))(inner))?
};
((|flagbits: u16| PResult::Ok(opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version0 { dogray: flagbits >> 1u16 & 1u16 != 0u16, gridfit: flagbits >> 0u16 & 1u16 != 0u16 }))(inner))?
};
opentype_gasp_table_gasp_ranges_range_gasp_behavior::Version0(inner)
},

1u16 => {
let inner = {
let inner = {
let inner = {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
b
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
b
}))())?;
(field0, field1)
};
((|x: (u8, u8)| PResult::Ok(u16be(x)))(inner))?
};
((|flagbits: u16| PResult::Ok(opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version1 { symmetric_smoothing: flagbits >> 3u16 & 1u16 != 0u16, symmetric_gridfit: flagbits >> 2u16 & 1u16 != 0u16, dogray: flagbits >> 1u16 & 1u16 != 0u16, gridfit: flagbits >> 0u16 & 1u16 != 0u16 }))(inner))?
};
opentype_gasp_table_gasp_ranges_range_gasp_behavior::Version1(inner)
},

_ => {
return Err(ParseError::FailToken);
}
}))())?;
opentype_gasp_table_gasp_ranges { range_max_ppem, range_gasp_behavior }
});
}
accum
}))())?;
PResult::Ok(opentype_gasp_table { version, num_ranges, gasp_ranges })
}

fn Decoder39<'input>(_input: &mut Parser<'input>) -> Result<(), ParseError> {
PResult::Ok(())
}

fn Decoder_opentype_gdef_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_gdef_table, ParseError> {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let major_version = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 1u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let minor_version = ((|| PResult::Ok((Decoder24(_input))?))())?;
let glyph_class_def = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_class_def(_input))?;
((|val: opentype_class_def| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_glyph_class_def { offset, link }
}))())?;
let attach_list = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let coverage = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_coverage_table(_input))?;
((|val: opentype_coverage_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_attach_list_link_coverage { offset, link }
}))())?;
let glyph_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let attach_point_offsets = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = {
let point_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let point_indices = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..point_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
opentype_gdef_table_attach_list_link_attach_point_offsets_link { point_count, point_indices }
};
((|val: opentype_gdef_table_attach_list_link_attach_point_offsets_link| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_attach_list_link_attach_point_offsets { offset, link }
});
}
accum
}))())?;
opentype_gdef_table_attach_list_link { table_start, coverage, glyph_count, attach_point_offsets }
};
((|val: opentype_gdef_table_attach_list_link| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_attach_list { offset, link }
}))())?;
let lig_caret_list = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let coverage = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_coverage_table(_input))?;
((|val: opentype_coverage_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_attach_list_link_coverage { offset, link }
}))())?;
let lig_glyph_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let lig_glyph_offsets = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..lig_glyph_count {
accum.push({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let caret_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let caret_values = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..caret_count {
accum.push({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let caret_value_format = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok(match caret_value_format {
1u16 => {
let inner = {
let coordinate = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format1 { coordinate }
};
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data::Format1(inner)
},

2u16 => {
let inner = {
let caret_value_point_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format2 { caret_value_point_index }
};
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data::Format2(inner)
},

3u16 => {
let inner = {
let coordinate = ((|| PResult::Ok((Decoder24(_input))?))())?;
let table = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
((|val: opentype_common_device_or_variation_index_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_common_value_record_x_advance_device { offset, link }
}))())?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format3 { coordinate, table }
};
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data::Format3(inner)
},

_ => {
return Err(ParseError::FailToken);
}
}))())?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link { table_start, caret_value_format, data }
};
((|val: opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values { offset, link }
});
}
accum
}))())?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link { table_start, caret_count, caret_values }
};
((|val: opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets { offset, link }
});
}
accum
}))())?;
opentype_gdef_table_lig_caret_list_link { table_start, coverage, lig_glyph_count, lig_glyph_offsets }
};
((|val: opentype_gdef_table_lig_caret_list_link| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_lig_caret_list { offset, link }
}))())?;
let mark_attach_class_def = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_class_def(_input))?;
((|val: opentype_class_def| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_glyph_class_def { offset, link }
}))())?;
let data = ((|| PResult::Ok(match minor_version {
0u16 => {
opentype_gdef_table_data::Version1_0
},

1u16 => {
return Err(ParseError::FailToken);
},

2u16 => {
let inner = {
let mark_glyph_sets_def = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_mark_glyph_set(_input))?;
((|val: opentype_mark_glyph_set| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_data_Version1_2_mark_glyph_sets_def { offset, link }
}))())?;
opentype_gdef_table_data_Version1_2 { mark_glyph_sets_def }
};
opentype_gdef_table_data::Version1_2(inner)
},

3u16 => {
let inner = {
let item_var_store = ((|| PResult::Ok((Decoder21(_input))?))())?;
opentype_gdef_table_data_Version1_3 { item_var_store }
};
opentype_gdef_table_data::Version1_3(inner)
},

_ => {
let inner = {
let item_var_store = ((|| PResult::Ok((Decoder21(_input))?))())?;
opentype_gdef_table_data_Version1_3 { item_var_store }
};
opentype_gdef_table_data::Version1_3(inner)
}
}))())?;
PResult::Ok(opentype_gdef_table { table_start, major_version, minor_version, glyph_class_def, attach_list, lig_caret_list, mark_attach_class_def, data })
}

fn Decoder_opentype_gpos_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_gpos_table, ParseError> {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let major_version = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 1u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let minor_version = ((|| PResult::Ok((Decoder24(_input))?))())?;
let script_list = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_script_list(_input))?;
((|val: opentype_common_script_list| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gpos_table_script_list { offset, link }
}))())?;
let feature_list = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_feature_list(_input))?;
((|val: opentype_common_feature_list| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gpos_table_feature_list { offset, link }
}))())?;
let lookup_list = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let lookup_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let lookups = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..lookup_count {
accum.push({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let lookup_type = ((|| PResult::Ok((Decoder24(_input))?))())?;
let lookup_flag = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|flagbits: u8| PResult::Ok(opentype_gpos_table_lookup_list_link_raw_lookups_link_raw_lookup_flag { use_mark_filtering_set: flagbits >> 4u8 & 1u8 != 0u8, ignore_marks: flagbits >> 3u8 & 1u8 != 0u8, ignore_ligatures: flagbits >> 2u8 & 1u8 != 0u8, ignore_base_glyphs: flagbits >> 1u8 & 1u8 != 0u8, right_to_left: flagbits >> 0u8 & 1u8 != 0u8 }))(inner))?
}))())?;
(field0, field1)
};
((|tuple_var: (u8, opentype_gpos_table_lookup_list_link_raw_lookups_link_raw_lookup_flag)| PResult::Ok(match tuple_var {
(macf, lo) => {
opentype_gpos_table_lookup_list_link_lookups_link_lookup_flag { mark_attachment_class_filter: macf, right_to_left: lo.right_to_left.clone(), ignore_base_glyphs: lo.ignore_base_glyphs.clone(), ignore_ligatures: lo.ignore_ligatures.clone(), ignore_marks: lo.ignore_marks.clone(), use_mark_filtering_set: lo.use_mark_filtering_set.clone() }
}
}))(inner))?
}))())?;
let sub_table_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let subtables = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..sub_table_count {
accum.push({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = match lookup_type {
1u16 => {
let inner = {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let pos_format = ((|| PResult::Ok((Decoder24(_input))?))())?;
let subtable = ((|| PResult::Ok(match pos_format {
1u16 => {
let inner = {
let coverage_offset = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_coverage_table(_input))?;
((|val: opentype_coverage_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_attach_list_link_coverage { offset, link }
}))())?;
let value_format = ((|| PResult::Ok((Decoder_opentype_common_value_format_flags(_input))?))())?;
let value_record = ((|| PResult::Ok((Decoder_opentype_common_value_record(_input, table_start.clone(), value_format.clone()))?))())?;
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust_subtable_Format1 { coverage_offset, value_format, value_record }
};
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust_subtable::Format1(inner)
},

2u16 => {
let inner = {
let coverage_offset = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_coverage_table(_input))?;
((|val: opentype_coverage_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gdef_table_attach_list_link_coverage { offset, link }
}))())?;
let value_format = ((|| PResult::Ok((Decoder_opentype_common_value_format_flags(_input))?))())?;
let value_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let value_records = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..value_count {
accum.push((Decoder_opentype_common_value_record(_input, table_start.clone(), value_format.clone()))?);
}
accum
}))())?;
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust_subtable_Format2 { coverage_offset, value_format, value_count, value_records }
};
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust_subtable::Format2(inner)
},

_ => {
return Err(ParseError::FailToken);
}
}))())?;
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SingleAdjust { table_start, pos_format, subtable }
};
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::SingleAdjust(inner)
},

2u16 => {
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::PairAdjust
},

3u16 => {
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::CursAttach
},

4u16 => {
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::MarkBaseAttach
},

5u16 => {
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::MarkLigAttach
},

6u16 => {
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::MarkMarkAttach
},

7u16 => {
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::CtxPos
},

8u16 => {
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::ChainCtxPos
},

9u16 => {
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::ExtPos
},

_ => {
return Err(ParseError::FailToken);
}
};
((|val: opentype_gpos_table_lookup_list_link_lookups_link_subtables_link| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gpos_table_lookup_list_link_lookups_link_subtables { offset, link }
});
}
accum
}))())?;
let mark_filtering_set = ((|| PResult::Ok(match lookup_flag.use_mark_filtering_set.clone() {
true => {
let inner = (Decoder24(_input))?;
((|val: u16| PResult::Ok(Some(val)))(inner))?
},

false => {
None
}
}))())?;
opentype_gpos_table_lookup_list_link_lookups_link { table_start, lookup_type, lookup_flag, sub_table_count, subtables, mark_filtering_set }
};
((|val: opentype_gpos_table_lookup_list_link_lookups_link| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gpos_table_lookup_list_link_lookups { offset, link }
});
}
accum
}))())?;
opentype_gpos_table_lookup_list_link { table_start, lookup_count, lookups }
};
((|val: opentype_gpos_table_lookup_list_link| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_gpos_table_lookup_list { offset, link }
}))())?;
PResult::Ok(opentype_gpos_table { table_start, major_version, minor_version, script_list, feature_list, lookup_list })
}

fn Decoder_opentype_common_script_list<'input>(_input: &mut Parser<'input>) -> Result<opentype_common_script_list, ParseError> {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let script_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let script_records = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..script_count {
accum.push({
let script_tag = ((|| PResult::Ok((Decoder48(_input))?))())?;
let script = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_script_table(_input))?;
((|val: opentype_common_script_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_common_script_list_script_records_script { offset, link }
}))())?;
opentype_common_script_list_script_records { script_tag, script }
});
}
accum
}))())?;
PResult::Ok(opentype_common_script_list { table_start, script_count, script_records })
}

fn Decoder_opentype_common_feature_list<'input>(_input: &mut Parser<'input>) -> Result<opentype_common_feature_list, ParseError> {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let feature_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let feature_records = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..feature_count {
accum.push({
let feature_tag = ((|| PResult::Ok((Decoder48(_input))?))())?;
let feature = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_feature_table(_input))?;
((|val: opentype_common_feature_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_common_feature_list_feature_records_feature { offset, link }
}))())?;
opentype_common_feature_list_feature_records { feature_tag, feature }
});
}
accum
}))())?;
PResult::Ok(opentype_common_feature_list { table_start, feature_count, feature_records })
}

fn Decoder_opentype_coverage_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_coverage_table, ParseError> {
let coverage_format = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok(match coverage_format {
1u16 => {
let inner = {
let glyph_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let glyph_array = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
opentype_coverage_table_data_Format1 { glyph_count, glyph_array }
};
opentype_coverage_table_data::Format1(inner)
},

2u16 => {
let inner = {
let range_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let range_records = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..range_count {
accum.push({
let start_glyph_id = ((|| PResult::Ok((Decoder24(_input))?))())?;
let end_glyph_id = ((|| PResult::Ok((Decoder24(_input))?))())?;
let start_coverage_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_coverage_table_data_Format2_range_records { start_glyph_id, end_glyph_id, start_coverage_index }
});
}
accum
}))())?;
opentype_coverage_table_data_Format2 { range_count, range_records }
};
opentype_coverage_table_data::Format2(inner)
},

_ => {
return Err(ParseError::FailToken);
}
}))())?;
PResult::Ok(opentype_coverage_table { coverage_format, data })
}

fn Decoder_opentype_common_value_format_flags<'input>(_input: &mut Parser<'input>) -> Result<opentype_common_value_format_flags, ParseError> {
let inner = {
let inner = {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
b
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
b
}))())?;
(field0, field1)
};
((|x: (u8, u8)| PResult::Ok(u16be(x)))(inner))?
};
PResult::Ok(((|flagbits: u16| PResult::Ok(opentype_common_value_format_flags { y_advance_device: flagbits >> 7u16 & 1u16 != 0u16, x_advance_device: flagbits >> 6u16 & 1u16 != 0u16, y_placement_device: flagbits >> 5u16 & 1u16 != 0u16, x_placement_device: flagbits >> 4u16 & 1u16 != 0u16, y_advance: flagbits >> 3u16 & 1u16 != 0u16, x_advance: flagbits >> 2u16 & 1u16 != 0u16, y_placement: flagbits >> 1u16 & 1u16 != 0u16, x_placement: flagbits >> 0u16 & 1u16 != 0u16 }))(inner))?)
}

fn Decoder_opentype_common_value_record<'input>(_input: &mut Parser<'input>, table_start: u32, flags: opentype_common_value_format_flags) -> Result<opentype_common_value_record, ParseError> {
let x_placement = ((|| PResult::Ok(if flags.x_placement.clone() {
Some((Decoder24(_input))?)
} else {
None
}))())?;
let y_placement = ((|| PResult::Ok(if flags.y_placement.clone() {
Some((Decoder24(_input))?)
} else {
None
}))())?;
let x_advance = ((|| PResult::Ok(if flags.x_advance.clone() {
Some((Decoder24(_input))?)
} else {
None
}))())?;
let y_advance = ((|| PResult::Ok(if flags.y_advance.clone() {
Some((Decoder24(_input))?)
} else {
None
}))())?;
let x_placement_device = ((|| PResult::Ok(if flags.x_placement_device.clone() {
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
((|val: opentype_common_device_or_variation_index_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
}))())?;
let y_placement_device = ((|| PResult::Ok(if flags.y_placement_device.clone() {
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
((|val: opentype_common_device_or_variation_index_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
}))())?;
let x_advance_device = ((|| PResult::Ok(if flags.x_advance_device.clone() {
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
((|val: opentype_common_device_or_variation_index_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
}))())?;
let y_advance_device = ((|| PResult::Ok(if flags.y_advance_device.clone() {
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
((|val: opentype_common_device_or_variation_index_table| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
}))())?;
PResult::Ok(opentype_common_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

fn Decoder_opentype_common_device_or_variation_index_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_common_device_or_variation_index_table, ParseError> {
let delta_format = {
_input.open_peek_context();
let ret = ((|| PResult::Ok({
let _ = {
let __skipped0 = ((|| PResult::Ok((Decoder24(_input))?))())?;
let __skipped1 = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_common_device_or_variation_index_table_raw_raw { __skipped0, __skipped1 }
};
(Decoder24(_input))?
}))())?;
_input.close_peek_context()?;
ret
};
PResult::Ok(match delta_format {
1u16..=3u16 => {
let inner = {
let start_size = ((|| PResult::Ok((Decoder24(_input))?))())?;
let end_size = ((|| PResult::Ok((Decoder24(_input))?))())?;
let delta_format = ((|| PResult::Ok((Decoder24(_input))?))())?;
let delta_values = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..match delta_format {
1u16 => {
match (((try_sub!(end_size, start_size)) + 1u16) / 8u16) * 8u16 < (try_sub!(end_size, start_size)) + 1u16 {
true => {
((try_sub!(end_size, start_size)) + 1u16) / 8u16 + 1u16
},

false => {
((try_sub!(end_size, start_size)) + 1u16) / 8u16
}
}
},

2u16 => {
match (((try_sub!(end_size, start_size)) + 1u16) / 4u16) * 4u16 < (try_sub!(end_size, start_size)) + 1u16 {
true => {
((try_sub!(end_size, start_size)) + 1u16) / 4u16 + 1u16
},

false => {
((try_sub!(end_size, start_size)) + 1u16) / 4u16
}
}
},

3u16 => {
match (((try_sub!(end_size, start_size)) + 1u16) / 2u16) * 2u16 < (try_sub!(end_size, start_size)) + 1u16 {
true => {
((try_sub!(end_size, start_size)) + 1u16) / 2u16 + 1u16
},

false => {
((try_sub!(end_size, start_size)) + 1u16) / 2u16
}
}
},

_ => {
0u16
}
} {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
opentype_common_device_or_variation_index_table_DeviceTable { start_size, end_size, delta_format, delta_values }
};
opentype_common_device_or_variation_index_table::DeviceTable(inner)
},

32768u16 => {
let inner = {
let delta_set_outer_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
let delta_set_inner_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
let delta_format = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 128 {
b
} else {
return Err(ParseError::ExcludedBranch(4742104188500697913u64));
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
}))())?;
opentype_common_device_or_variation_index_table_VariationIndexTable { delta_set_outer_index, delta_set_inner_index, delta_format }
};
opentype_common_device_or_variation_index_table::VariationIndexTable(inner)
},

_ => {
return Err(ParseError::FailToken);
}
})
}

fn Decoder48<'input>(_input: &mut Parser<'input>) -> Result<u32, ParseError> {
PResult::Ok((Decoder21(_input))?)
}

fn Decoder_opentype_common_feature_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_common_feature_table, ParseError> {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let feature_params = ((|| PResult::Ok((Decoder24(_input))?))())?;
let lookup_index_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let lookup_list_indices = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..lookup_index_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
PResult::Ok(opentype_common_feature_table { table_start, feature_params, lookup_index_count, lookup_list_indices })
}

fn Decoder_opentype_common_script_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_common_script_table, ParseError> {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let default_lang_sys = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_langsys(_input))?;
((|val: opentype_common_langsys| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_common_script_table_default_lang_sys { offset, link }
}))())?;
let lang_sys_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let lang_sys_records = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..lang_sys_count {
accum.push({
let lang_sys_tag = ((|| PResult::Ok((Decoder48(_input))?))())?;
let lang_sys = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = (Decoder_opentype_common_langsys(_input))?;
((|val: opentype_common_langsys| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_common_script_table_default_lang_sys { offset, link }
}))())?;
opentype_common_script_table_lang_sys_records { lang_sys_tag, lang_sys }
});
}
accum
}))())?;
PResult::Ok(opentype_common_script_table { table_start, default_lang_sys, lang_sys_count, lang_sys_records })
}

fn Decoder_opentype_common_langsys<'input>(_input: &mut Parser<'input>) -> Result<opentype_common_langsys, ParseError> {
let lookup_order_offset = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let required_feature_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
let feature_index_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let feature_indices = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..feature_index_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
PResult::Ok(opentype_common_langsys { lookup_order_offset, required_feature_index, feature_index_count, feature_indices })
}

fn Decoder_opentype_class_def<'input>(_input: &mut Parser<'input>) -> Result<opentype_class_def, ParseError> {
let class_format = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok(match class_format {
1u16 => {
let inner = {
let start_glyph_id = ((|| PResult::Ok((Decoder24(_input))?))())?;
let glyph_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let class_value_array = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
opentype_class_def_data_Format1 { start_glyph_id, glyph_count, class_value_array }
};
opentype_class_def_data::Format1(inner)
},

2u16 => {
let inner = {
let class_range_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let class_range_records = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..class_range_count {
accum.push({
let start_glyph_id = ((|| PResult::Ok((Decoder24(_input))?))())?;
let end_glyph_id = ((|| PResult::Ok((Decoder24(_input))?))())?;
let class = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_class_def_data_Format2_class_range_records { start_glyph_id, end_glyph_id, class }
});
}
accum
}))())?;
opentype_class_def_data_Format2 { class_range_count, class_range_records }
};
opentype_class_def_data::Format2(inner)
},

_ => {
return Err(ParseError::FailToken);
}
}))())?;
PResult::Ok(opentype_class_def { class_format, data })
}

fn Decoder_opentype_mark_glyph_set<'input>(_input: &mut Parser<'input>) -> Result<opentype_mark_glyph_set, ParseError> {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 1u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let mark_glyph_set_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let coverage = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..mark_glyph_set_count {
accum.push({
let offset = ((|| PResult::Ok((Decoder21(_input))?))())?;
let link = ((|| PResult::Ok(if offset != 0u32 {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + offset, __here))?;
let ret = ((|| PResult::Ok((Decoder_opentype_coverage_table(_input))?))())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
}))())?;
opentype_mark_glyph_set_coverage { offset, link }
});
}
accum
}))())?;
PResult::Ok(opentype_mark_glyph_set { table_start, format, mark_glyph_set_count, coverage })
}

fn Decoder_opentype_glyf_description<'input>(_input: &mut Parser<'input>, n_contours: u16) -> Result<opentype_glyf_description, ParseError> {
PResult::Ok(match n_contours {
0u16 => {
opentype_glyf_description::HeaderOnly
},

1u16..=32767u16 => {
let inner = (Decoder_opentype_glyf_simple(_input, n_contours.clone()))?;
opentype_glyf_description::Simple(inner)
},

_ => {
let inner = (Decoder_opentype_glyf_composite(_input))?;
opentype_glyf_description::Composite(inner)
}
})
}

fn Decoder_opentype_glyf_simple<'input>(_input: &mut Parser<'input>, n_contours: u16) -> Result<opentype_glyf_simple, ParseError> {
let end_points_of_contour = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..n_contours {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
let instruction_length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let instructions = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..instruction_length {
accum.push((Decoder25(_input))?);
}
accum
}))())?;
let number_of_coordinates = ((|| PResult::Ok(1u16 + end_points_of_contour[(try_sub!((end_points_of_contour.len()) as u32, 1u32)) as usize].clone()))())?;
let flags = ((|| PResult::Ok({
let inner = {
let inner = {
let mut seq = Vec::new();
let mut acc = 0u16;
loop {
if ((|tuple_var: (u16, &Vec<opentype_glyf_simple_flags_raw>)| PResult::Ok(match tuple_var {
(totlen, _seq) => {
totlen >= number_of_coordinates
}
}))((acc.clone(), &seq)))? {
break
}
let elem = {
let flags = (Decoder_opentype_glyph_description_simple_flags_raw(_input))?;
let repeats = ((|| PResult::Ok(match flags.repeat_flag.clone() {
true => {
(Decoder25(_input))?
},

false => {
0u8
}
}))())?;
let field_set = ((|| PResult::Ok(opentype_glyf_simple_flags { on_curve_point: flags.on_curve_point.clone(), x_short_vector: flags.x_short_vector.clone(), y_short_vector: flags.y_short_vector.clone(), x_is_same_or_positive_x_short_vector: flags.x_is_same_or_positive_x_short_vector.clone(), y_is_same_or_positive_y_short_vector: flags.y_is_same_or_positive_y_short_vector.clone(), overlap_simple: flags.overlap_simple.clone() }))())?;
opentype_glyf_simple_flags_raw { repeats, field_set }
};
seq.push(elem.clone());
acc = ((|tuple_var: (u16, opentype_glyf_simple_flags_raw)| PResult::Ok(match tuple_var {
(acc, flags) => {
acc + ((flags.repeats.clone()) as u16) + 1u16
}
}))((acc, elem)))?;
}
(acc, seq)
};
((|tuple_var: (u16, Vec<opentype_glyf_simple_flags_raw>)| PResult::Ok(match tuple_var {
(_len, flags) => {
flags
}
}))(inner))?
};
((|arr_flags: Vec<opentype_glyf_simple_flags_raw>| PResult::Ok((try_flat_map_vec(arr_flags.iter().cloned(), |packed: opentype_glyf_simple_flags_raw| PResult::Ok(dup32(((packed.repeats.clone()) as u32) + 1u32, packed.field_set.clone()))))?))(inner))?
}))())?;
let x_coordinates = ((|| PResult::Ok({
let mut accum = Vec::new();
for flag_vals in flags.clone() {
accum.push(match flag_vals.x_short_vector.clone() {
true => {
let inner = (Decoder25(_input))?;
((|abs: u8| PResult::Ok(match flag_vals.x_is_same_or_positive_x_short_vector.clone() {
true => {
abs as u16
},

false => {
match abs {
0u8 => {
0u16
},

n => {
try_sub!(65535u16, try_sub!(n as u16, 1u16))
}
}
}
}))(inner))?
},

false => {
match flag_vals.x_is_same_or_positive_x_short_vector.clone() {
true => {
0u16
},

false => {
(Decoder24(_input))?
}
}
}
});
}
accum
}))())?;
let y_coordinates = ((|| PResult::Ok({
let mut accum = Vec::new();
for flag_vals in flags.clone() {
accum.push(match flag_vals.y_short_vector.clone() {
true => {
let inner = (Decoder25(_input))?;
((|abs: u8| PResult::Ok(match flag_vals.y_is_same_or_positive_y_short_vector.clone() {
true => {
abs as u16
},

false => {
match abs {
0u8 => {
0u16
},

n => {
try_sub!(65535u16, try_sub!(n as u16, 1u16))
}
}
}
}))(inner))?
},

false => {
match flag_vals.y_is_same_or_positive_y_short_vector.clone() {
true => {
0u16
},

false => {
(Decoder24(_input))?
}
}
}
});
}
accum
}))())?;
PResult::Ok(opentype_glyf_simple { end_points_of_contour, instruction_length, instructions, number_of_coordinates, flags, x_coordinates, y_coordinates })
}

fn Decoder_opentype_glyf_composite<'input>(_input: &mut Parser<'input>) -> Result<opentype_glyf_composite, ParseError> {
let acc_glyphs = {
let mut seq = Vec::new();
let mut acc = false;
loop {
if ((|tuple_var: (bool, &Vec<opentype_glyf_composite_raw>)| PResult::Ok(match tuple_var {
(_has_instructions, seq) => {
match match ((seq.len()) as u32) > 0u32 {
true => {
Some(seq[(try_sub!((seq.len()) as u32, 1u32)) as usize].clone())
},

false => {
None
}
} {
Some(ref x) => {
!x.flags.more_components.clone()
},

None => {
false
}
}
}
}))((acc.clone(), &seq)))? {
break
}
let elem = {
let flags = ((|| PResult::Ok({
let inner = {
let inner = {
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
b
}))())?;
let field1 = ((|| PResult::Ok({
let b = _input.read_byte()?;
b
}))())?;
(field0, field1)
};
((|x: (u8, u8)| PResult::Ok(u16be(x)))(inner))?
};
((|flagbits: u16| PResult::Ok(opentype_glyf_composite_raw_flags { unscaled_component_offset: flagbits >> 12u16 & 1u16 != 0u16, scaled_component_offset: flagbits >> 11u16 & 1u16 != 0u16, overlap_compound: flagbits >> 10u16 & 1u16 != 0u16, use_my_metrics: flagbits >> 9u16 & 1u16 != 0u16, we_have_instructions: flagbits >> 8u16 & 1u16 != 0u16, we_have_a_two_by_two: flagbits >> 7u16 & 1u16 != 0u16, we_have_an_x_and_y_scale: flagbits >> 6u16 & 1u16 != 0u16, more_components: flagbits >> 5u16 & 1u16 != 0u16, __reserved_bit4: flagbits >> 4u16 & 1u16 != 0u16, we_have_a_scale: flagbits >> 3u16 & 1u16 != 0u16, round_xy_to_grid: flagbits >> 2u16 & 1u16 != 0u16, args_are_xy_values: flagbits >> 1u16 & 1u16 != 0u16, arg_1_and_2_are_words: flagbits >> 0u16 & 1u16 != 0u16 }))(inner))?
}))())?;
let glyph_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
let argument1 = ((|| PResult::Ok(match flags.arg_1_and_2_are_words.clone() {
true => {
match flags.args_are_xy_values.clone() {
true => {
let inner = (Decoder24(_input))?;
opentype_glyf_composite_raw_argument1::Int16(inner)
},

false => {
let inner = (Decoder24(_input))?;
opentype_glyf_composite_raw_argument1::Uint16(inner)
}
}
},

false => {
match flags.args_are_xy_values.clone() {
true => {
let inner = (Decoder25(_input))?;
opentype_glyf_composite_raw_argument1::Int8(inner)
},

false => {
let inner = (Decoder25(_input))?;
opentype_glyf_composite_raw_argument1::Uint8(inner)
}
}
}
}))())?;
let argument2 = ((|| PResult::Ok(match flags.arg_1_and_2_are_words.clone() {
true => {
match flags.args_are_xy_values.clone() {
true => {
let inner = (Decoder24(_input))?;
opentype_glyf_composite_raw_argument1::Int16(inner)
},

false => {
let inner = (Decoder24(_input))?;
opentype_glyf_composite_raw_argument1::Uint16(inner)
}
}
},

false => {
match flags.args_are_xy_values.clone() {
true => {
let inner = (Decoder25(_input))?;
opentype_glyf_composite_raw_argument1::Int8(inner)
},

false => {
let inner = (Decoder25(_input))?;
opentype_glyf_composite_raw_argument1::Uint8(inner)
}
}
}
}))())?;
let scale = ((|| PResult::Ok(match flags.we_have_a_scale.clone() {
true => {
let inner = {
let inner = {
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(opentype_glyf_composite_raw_scale_Scale::F2Dot14(x)))(inner))?
};
opentype_glyf_composite_raw_scale::Scale(inner)
};
((|val: opentype_glyf_composite_raw_scale| PResult::Ok(Some(val)))(inner))?
},

false => {
match flags.we_have_an_x_and_y_scale.clone() {
true => {
let inner = {
let inner = {
let x_scale = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(opentype_glyf_composite_raw_scale_Scale::F2Dot14(x)))(inner))?
}))())?;
let y_scale = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(opentype_glyf_composite_raw_scale_Scale::F2Dot14(x)))(inner))?
}))())?;
opentype_glyf_composite_raw_scale_XY { x_scale, y_scale }
};
opentype_glyf_composite_raw_scale::XY(inner)
};
((|val: opentype_glyf_composite_raw_scale| PResult::Ok(Some(val)))(inner))?
},

false => {
match flags.we_have_a_two_by_two.clone() {
true => {
let inner = {
let field0 = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(opentype_glyf_composite_raw_scale_Scale::F2Dot14(x)))(inner))?
}))())?;
let field1 = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(opentype_glyf_composite_raw_scale_Scale::F2Dot14(x)))(inner))?
}))())?;
(field0, field1)
}))())?;
let field1 = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(opentype_glyf_composite_raw_scale_Scale::F2Dot14(x)))(inner))?
}))())?;
let field1 = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(opentype_glyf_composite_raw_scale_Scale::F2Dot14(x)))(inner))?
}))())?;
(field0, field1)
}))())?;
opentype_glyf_composite_raw_scale::Matrix(field0, field1)
};
((|val: opentype_glyf_composite_raw_scale| PResult::Ok(Some(val)))(inner))?
},

false => {
None
}
}
}
}
}
}))())?;
opentype_glyf_composite_raw { flags, glyph_index, argument1, argument2, scale }
};
seq.push(elem.clone());
acc = ((|tuple_var: (bool, opentype_glyf_composite_raw)| PResult::Ok(match tuple_var {
(acc, glyph) => {
acc || glyph.flags.we_have_instructions.clone()
}
}))((acc, elem)))?;
}
(acc, seq)
};
let glyphs = ((|| PResult::Ok(acc_glyphs.1.clone()))())?;
let instructions = ((|| PResult::Ok(match acc_glyphs.0.clone() {
true => {
let instructions_length = (Decoder24(_input))?;
let mut accum = Vec::new();
for _ in 0..instructions_length {
accum.push((Decoder25(_input))?);
}
accum
},

false => {
[].to_vec()
}
}))())?;
PResult::Ok(opentype_glyf_composite { glyphs, instructions })
}

fn Decoder_opentype_glyph_description_simple_flags_raw<'input>(_input: &mut Parser<'input>) -> Result<opentype_glyph_description_simple_flags_raw, ParseError> {
let inner = {
let b = _input.read_byte()?;
b
};
PResult::Ok(((|flagbits: u8| PResult::Ok(opentype_glyph_description_simple_flags_raw { overlap_simple: flagbits >> 6u8 & 1u8 != 0u8, y_is_same_or_positive_y_short_vector: flagbits >> 5u8 & 1u8 != 0u8, x_is_same_or_positive_x_short_vector: flagbits >> 4u8 & 1u8 != 0u8, repeat_flag: flagbits >> 3u8 & 1u8 != 0u8, y_short_vector: flagbits >> 2u8 & 1u8 != 0u8, x_short_vector: flagbits >> 1u8 & 1u8 != 0u8, on_curve_point: flagbits >> 0u8 & 1u8 != 0u8 }))(inner))?)
}

fn Decoder_opentype_name_table_name_version_1<'input>(_input: &mut Parser<'input>, storage_start: u32) -> Result<opentype_name_table_name_version_1, ParseError> {
let lang_tag_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let lang_tag_records = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..lang_tag_count {
accum.push({
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let offset = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
let link = ((|| PResult::Ok(match offset != 0u16 {
true => {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(storage_start + (offset as u32), __here))?;
let ret = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..length {
accum.push((Decoder25(_input))?);
}
accum
};
((|val: Vec<u8>| PResult::Ok(Some(val)))(inner))?
}))())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
}))())?;
opentype_name_table_name_records_offset { offset, link }
}))())?;
opentype_name_table_name_version_1_lang_tag_records { length, offset }
});
}
accum
}))())?;
PResult::Ok(opentype_name_table_name_version_1 { lang_tag_count, lang_tag_records })
}

fn Decoder_opentype_maxp_table_version1<'input>(_input: &mut Parser<'input>) -> Result<opentype_maxp_table_version1, ParseError> {
let max_points = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_contours = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_composite_points = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_composite_contours = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_zones = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok((x >= 1u16) && (x <= 2u16)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let max_twilight_points = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_storage = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_function_defs = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_instruction_defs = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_stack_elements = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_size_of_instructions = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_component_elements = ((|| PResult::Ok((Decoder24(_input))?))())?;
let max_component_depth = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x <= 16u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
PResult::Ok(opentype_maxp_table_version1 { max_points, max_contours, max_composite_points, max_composite_contours, max_zones, max_twilight_points, max_storage, max_function_defs, max_instruction_defs, max_stack_elements, max_size_of_instructions, max_component_elements, max_component_depth })
}

fn Decoder60<'input>(_input: &mut Parser<'input>) -> Result<u64, ParseError> {
PResult::Ok((Decoder61(_input))?)
}

fn Decoder61<'input>(_input: &mut Parser<'input>) -> Result<u64, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
PResult::Ok(((|x: (u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(u64be(x)))(inner))?)
}

fn Decoder_opentype_encoding_record<'input>(_input: &mut Parser<'input>, start: u32) -> Result<opentype_encoding_record, ParseError> {
let platform = ((|| PResult::Ok((Decoder24(_input))?))())?;
let encoding = ((|| PResult::Ok((Decoder24(_input))?))())?;
let subtable_offset = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder21(_input))?))())?;
let link = ((|| PResult::Ok(if offset != 0u32 {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(start + offset, __here))?;
let ret = ((|| PResult::Ok((Decoder_opentype_cmap_subtable(_input, platform.clone()))?))())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
}))())?;
opentype_encoding_record_subtable_offset { offset, link }
}))())?;
PResult::Ok(opentype_encoding_record { platform, encoding, subtable_offset })
}

fn Decoder_opentype_cmap_subtable<'input>(_input: &mut Parser<'input>, _platform: u16) -> Result<opentype_cmap_subtable, ParseError> {
let table_start = ((|| PResult::Ok({
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
}))())?;
let format = ((|| PResult::Ok({
_input.open_peek_context();
let ret = ((|| PResult::Ok((Decoder24(_input))?))())?;
_input.close_peek_context()?;
ret
}))())?;
let data = ((|| PResult::Ok(match format {
0u16 => {
let inner = (Decoder_opentype_cmap_subtable_format0(_input, _platform.clone()))?;
opentype_cmap_subtable_data::Format0(inner)
},

2u16 => {
let inner = (Decoder_opentype_cmap_subtable_format2(_input, _platform.clone()))?;
opentype_cmap_subtable_data::Format2(inner)
},

4u16 => {
let inner = (Decoder_opentype_cmap_subtable_format4(_input, _platform.clone()))?;
opentype_cmap_subtable_data::Format4(inner)
},

6u16 => {
let inner = (Decoder_opentype_cmap_subtable_format6(_input, _platform.clone()))?;
opentype_cmap_subtable_data::Format6(inner)
},

8u16 => {
let inner = (Decoder_opentype_cmap_subtable_format8(_input, _platform.clone()))?;
opentype_cmap_subtable_data::Format8(inner)
},

10u16 => {
let inner = (Decoder_opentype_cmap_subtable_format10(_input, _platform.clone()))?;
opentype_cmap_subtable_data::Format10(inner)
},

12u16 => {
let inner = (Decoder_opentype_cmap_subtable_format13(_input, _platform.clone()))?;
opentype_cmap_subtable_data::Format12(inner)
},

13u16 => {
let inner = (Decoder71(_input, _platform.clone()))?;
opentype_cmap_subtable_data::Format13(inner)
},

14u16 => {
let inner = (Decoder_opentype_cmap_subtable_format14(_input, table_start.clone()))?;
opentype_cmap_subtable_data::Format14(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
PResult::Ok(opentype_cmap_subtable { table_start, format, data })
}

fn Decoder_opentype_cmap_subtable_format0<'input>(_input: &mut Parser<'input>, _platform: u16) -> Result<opentype_cmap_subtable_format0, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| PResult::Ok({
let _ = {
let format = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_cmap_subtable_format14_raw_raw { format }
};
(Decoder24(_input))?
}))())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let format = ((|| PResult::Ok((Decoder24(_input))?))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let language = ((|| PResult::Ok((Decoder24(_input))?))())?;
let glyph_id_array = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..256u16 {
accum.push((Decoder25(_input))?);
}
accum
}))())?;
opentype_cmap_subtable_format0 { format, length, language, glyph_id_array }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format2<'input>(_input: &mut Parser<'input>, _platform: u16) -> Result<opentype_cmap_subtable_format2, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| PResult::Ok({
let _ = {
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 2u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
opentype_cmap_subtable_format14_raw_raw { format }
};
let inner = (Decoder24(_input))?;
if ((|l: u16| PResult::Ok((l >= 518u16) && (l % 2u16 == 0u16)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 2u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let length = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|l: u16| PResult::Ok((l >= 518u16) && (l % 2u16 == 0u16)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let language = ((|| PResult::Ok((Decoder24(_input))?))())?;
let sub_header_keys = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..256u16 {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
let sub_headers = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..1u16 + match (try_fold_left_curried(sub_header_keys.iter().cloned(), None, |tuple_var: (Option<u16>, u16)| PResult::Ok(match tuple_var {
(acc, y) => {
match acc {
Some(x) => {
Some(match x >= y / 8u16 {
true => {
x.clone()
},

false => {
y / 8u16
}
})
},

None => {
Some(y / 8u16)
}
}
}
})))? {
Some(x) => {
x
},

_ => {
return Err(ParseError::ExcludedBranch(10165057510572328669u64));
}
} {
accum.push({
let first_code = ((|| PResult::Ok((Decoder24(_input))?))())?;
let entry_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let id_delta = ((|| PResult::Ok((Decoder24(_input))?))())?;
let id_range_offset = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_cmap_subtable_format2_sub_headers { first_code, entry_count, id_delta, id_range_offset }
});
}
accum
}))())?;
let glyph_array = ((|| PResult::Ok({
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
opentype_cmap_subtable_format2 { format, length, language, sub_header_keys, sub_headers, glyph_array }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format4<'input>(_input: &mut Parser<'input>, _platform: u16) -> Result<opentype_cmap_subtable_format4, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| PResult::Ok({
let _ = {
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 4u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
opentype_cmap_subtable_format14_raw_raw { format }
};
(Decoder24(_input))?
}))())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 4u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let language = ((|| PResult::Ok((Decoder24(_input))?))())?;
let seg_count = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
((|seg_count_x2: u16| PResult::Ok(seg_count_x2 / 2u16))(inner))?
}))())?;
let search_range = ((|| PResult::Ok((Decoder24(_input))?))())?;
let entry_selector = ((|| PResult::Ok((Decoder24(_input))?))())?;
let range_shift = ((|| PResult::Ok((Decoder24(_input))?))())?;
let end_code = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..seg_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
let __reserved_pad = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let start_code = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..seg_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
let id_delta = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..seg_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
let id_range_offset = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..seg_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
let glyph_array = ((|| PResult::Ok({
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
opentype_cmap_subtable_format4 { format, length, language, seg_count, search_range, entry_selector, range_shift, end_code, __reserved_pad, start_code, id_delta, id_range_offset, glyph_array }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format6<'input>(_input: &mut Parser<'input>, _platform: u16) -> Result<opentype_cmap_subtable_format6, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| PResult::Ok({
let _ = {
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 6u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
opentype_cmap_subtable_format14_raw_raw { format }
};
(Decoder24(_input))?
}))())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 6u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let language = ((|| PResult::Ok((Decoder24(_input))?))())?;
let first_code = ((|| PResult::Ok((Decoder24(_input))?))())?;
let entry_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let glyph_id_array = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
opentype_cmap_subtable_format6 { format, length, language, first_code, entry_count, glyph_id_array }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format8<'input>(_input: &mut Parser<'input>, _platform: u16) -> Result<opentype_cmap_subtable_format8, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| PResult::Ok({
let _ = {
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 8u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let __reserved = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
opentype_cmap_subtable_format13_raw_raw { format, __reserved }
};
(Decoder21(_input))?
}))())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 8u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let __reserved = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let length = ((|| PResult::Ok((Decoder21(_input))?))())?;
let language = ((|| PResult::Ok((Decoder21(_input))?))())?;
let is32 = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..8192u16 {
accum.push((Decoder25(_input))?);
}
accum
}))())?;
let num_groups = ((|| PResult::Ok((Decoder21(_input))?))())?;
let groups = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_groups {
accum.push((Decoder_opentype_types_sequential_map_record(_input))?);
}
accum
}))())?;
opentype_cmap_subtable_format8 { format, __reserved, length, language, is32, num_groups, groups }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format10<'input>(_input: &mut Parser<'input>, _platform: u16) -> Result<opentype_cmap_subtable_format10, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| PResult::Ok({
let _ = {
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 10u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let __reserved = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
opentype_cmap_subtable_format13_raw_raw { format, __reserved }
};
(Decoder21(_input))?
}))())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 10u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let __reserved = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let length = ((|| PResult::Ok((Decoder21(_input))?))())?;
let language = ((|| PResult::Ok((Decoder21(_input))?))())?;
let start_char_code = ((|| PResult::Ok((Decoder21(_input))?))())?;
let num_chars = ((|| PResult::Ok((Decoder21(_input))?))())?;
let glyph_id_array = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_chars {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
opentype_cmap_subtable_format10 { format, __reserved, length, language, start_char_code, num_chars, glyph_id_array }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format13<'input>(_input: &mut Parser<'input>, _platform: u16) -> Result<opentype_cmap_subtable_format13, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| PResult::Ok({
let _ = {
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 12u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let __reserved = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
opentype_cmap_subtable_format13_raw_raw { format, __reserved }
};
(Decoder21(_input))?
}))())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 12u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let __reserved = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let length = ((|| PResult::Ok((Decoder21(_input))?))())?;
let language = ((|| PResult::Ok((Decoder21(_input))?))())?;
let num_groups = ((|| PResult::Ok((Decoder21(_input))?))())?;
let groups = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_groups {
accum.push((Decoder_opentype_types_sequential_map_record(_input))?);
}
accum
}))())?;
opentype_cmap_subtable_format13 { format, __reserved, length, language, num_groups, groups }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder71<'input>(_input: &mut Parser<'input>, _platform: u16) -> Result<opentype_cmap_subtable_format13, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| PResult::Ok({
let _ = {
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 13u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let __reserved = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
opentype_cmap_subtable_format13_raw_raw { format, __reserved }
};
(Decoder21(_input))?
}))())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 13u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let __reserved = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let length = ((|| PResult::Ok((Decoder21(_input))?))())?;
let language = ((|| PResult::Ok((Decoder21(_input))?))())?;
let num_groups = ((|| PResult::Ok((Decoder21(_input))?))())?;
let groups = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_groups {
accum.push((Decoder_opentype_types_sequential_map_record(_input))?);
}
accum
}))())?;
opentype_cmap_subtable_format13 { format, __reserved, length, language, num_groups, groups }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format14<'input>(_input: &mut Parser<'input>, table_start: u32) -> Result<opentype_cmap_subtable_format14, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| PResult::Ok({
let _ = {
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 14u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
opentype_cmap_subtable_format14_raw_raw { format }
};
(Decoder21(_input))?
}))())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let format = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x == 14u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let length = ((|| PResult::Ok((Decoder21(_input))?))())?;
let num_var_selector_records = ((|| PResult::Ok((Decoder21(_input))?))())?;
let var_selector = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_var_selector_records {
accum.push((Decoder_opentype_variation_selector(_input, table_start.clone()))?);
}
accum
}))())?;
opentype_cmap_subtable_format14 { format, length, num_var_selector_records, var_selector }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_variation_selector<'input>(_input: &mut Parser<'input>, table_start: u32) -> Result<opentype_variation_selector, ParseError> {
let var_selector = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok(0u8))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2, field3)
};
((|x: (u8, u8, u8, u8)| PResult::Ok(u32be(x)))(inner))?
}))())?;
let default_uvs_offset = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder21(_input))?))())?;
let link = ((|| PResult::Ok(if offset != 0u32 {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + offset, __here))?;
let ret = ((|| PResult::Ok({
let num_unicode_value_ranges = ((|| PResult::Ok((Decoder21(_input))?))())?;
let ranges = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_unicode_value_ranges {
accum.push({
let start_unicode_value = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok(0u8))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2, field3)
};
((|x: (u8, u8, u8, u8)| PResult::Ok(u32be(x)))(inner))?
}))())?;
let additional_count = ((|| PResult::Ok((Decoder25(_input))?))())?;
opentype_variation_selector_default_uvs_offset_link_ranges { start_unicode_value, additional_count }
});
}
accum
}))())?;
opentype_variation_selector_default_uvs_offset_link { num_unicode_value_ranges, ranges }
}))())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
}))())?;
opentype_variation_selector_default_uvs_offset { offset, link }
}))())?;
let non_default_uvs_offset = ((|| PResult::Ok({
let offset = ((|| PResult::Ok((Decoder21(_input))?))())?;
let link = ((|| PResult::Ok(if offset != 0u32 {
let __here = {
let inner = _input.get_offset_u64();
((|x: u64| PResult::Ok(x as u32))(inner))?
};
_input.open_peek_context();
_input.advance_by(try_sub!(table_start + offset, __here))?;
let ret = ((|| PResult::Ok({
let num_uvs_mappings = ((|| PResult::Ok((Decoder21(_input))?))())?;
let uvs_mappings = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_uvs_mappings {
accum.push({
let unicode_value = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok(0u8))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2, field3)
};
((|x: (u8, u8, u8, u8)| PResult::Ok(u32be(x)))(inner))?
}))())?;
let glyph_id = ((|| PResult::Ok((Decoder24(_input))?))())?;
opentype_variation_selector_non_default_uvs_offset_link_uvs_mappings { unicode_value, glyph_id }
});
}
accum
}))())?;
opentype_variation_selector_non_default_uvs_offset_link { num_uvs_mappings, uvs_mappings }
}))())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
}))())?;
opentype_variation_selector_non_default_uvs_offset { offset, link }
}))())?;
PResult::Ok(opentype_variation_selector { var_selector, default_uvs_offset, non_default_uvs_offset })
}

fn Decoder_opentype_types_sequential_map_record<'input>(_input: &mut Parser<'input>) -> Result<opentype_types_sequential_map_record, ParseError> {
let start_char_code = ((|| PResult::Ok((Decoder21(_input))?))())?;
let end_char_code = ((|| PResult::Ok((Decoder21(_input))?))())?;
let start_glyph_id = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(opentype_types_sequential_map_record { start_char_code, end_char_code, start_glyph_id })
}

fn Decoder_elf_header<'input>(_input: &mut Parser<'input>) -> Result<elf_header, ParseError> {
let ident = ((|| PResult::Ok({
let sz = 16u32 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_elf_header_ident(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let r#type = ((|| PResult::Ok((Decoder97(_input, ident.data.clone() == 2u8))?))())?;
let machine = ((|| PResult::Ok((Decoder98(_input, ident.data.clone() == 2u8))?))())?;
let version = ((|| PResult::Ok((Decoder99(_input, ident.data.clone() == 2u8))?))())?;
let entry = ((|| PResult::Ok((Decoder_elf_types_elf_addr(_input, ident.data.clone() == 2u8, ident.class.clone()))?))())?;
let phoff = ((|| PResult::Ok((Decoder_elf_types_elf_off(_input, ident.data.clone() == 2u8, ident.class.clone()))?))())?;
let shoff = ((|| PResult::Ok((Decoder_elf_types_elf_off(_input, ident.data.clone() == 2u8, ident.class.clone()))?))())?;
let flags = ((|| PResult::Ok((Decoder80(_input, ident.data.clone() == 2u8))?))())?;
let ehsize = ((|| PResult::Ok((Decoder100(_input, ident.data.clone() == 2u8))?))())?;
let phentsize = ((|| PResult::Ok((Decoder100(_input, ident.data.clone() == 2u8))?))())?;
let phnum = ((|| PResult::Ok((Decoder100(_input, ident.data.clone() == 2u8))?))())?;
let shentsize = ((|| PResult::Ok((Decoder100(_input, ident.data.clone() == 2u8))?))())?;
let shnum = ((|| PResult::Ok((Decoder100(_input, ident.data.clone() == 2u8))?))())?;
let shstrndx = ((|| PResult::Ok((Decoder100(_input, ident.data.clone() == 2u8))?))())?;
PResult::Ok(elf_header { ident, r#type, machine, version, entry, phoff, shoff, flags, ehsize, phentsize, phnum, shentsize, shnum, shstrndx })
}

fn Decoder76<'input>(_input: &mut Parser<'input>, is_be: bool, class: u8, phnum: u16) -> Result<Vec<elf_phdr_table>, ParseError> {
let mut accum = Vec::new();
for _ in 0..phnum {
accum.push((Decoder_elf_phdr_table(_input, is_be.clone(), class.clone()))?);
}
PResult::Ok(accum)
}

fn Decoder77<'input>(_input: &mut Parser<'input>, is_be: bool, class: u8, shnum: u16) -> Result<Vec<elf_shdr_table>, ParseError> {
let mut accum = Vec::new();
for _ in 0..shnum {
accum.push((Decoder_elf_shdr_table(_input, is_be.clone(), class.clone()))?);
}
PResult::Ok(accum)
}

fn Decoder78<'input>(_input: &mut Parser<'input>, r#type: u32, size: u64) -> Result<Vec<u8>, ParseError> {
PResult::Ok(match r#type {
_ => {
let mut accum = Vec::new();
for _ in 0..size {
accum.push((Decoder25(_input))?);
}
accum
}
})
}

fn Decoder_elf_shdr_table<'input>(_input: &mut Parser<'input>, is_be: bool, class: u8) -> Result<elf_shdr_table, ParseError> {
let name = ((|| PResult::Ok((Decoder80(_input, is_be.clone()))?))())?;
let r#type = ((|| PResult::Ok((Decoder81(_input, is_be.clone()))?))())?;
let flags = ((|| PResult::Ok((Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?))())?;
let addr = ((|| PResult::Ok((Decoder_elf_types_elf_addr(_input, is_be.clone(), class.clone()))?))())?;
let offset = ((|| PResult::Ok((Decoder_elf_types_elf_off(_input, is_be.clone(), class.clone()))?))())?;
let size = ((|| PResult::Ok((Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?))())?;
let link = ((|| PResult::Ok((Decoder80(_input, is_be.clone()))?))())?;
let info = ((|| PResult::Ok((Decoder85(_input, is_be.clone()))?))())?;
let addralign = ((|| PResult::Ok((Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?))())?;
let entsize = ((|| PResult::Ok((Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?))())?;
PResult::Ok(elf_shdr_table { name, r#type, flags, addr, offset, size, link, info, addralign, entsize })
}

fn Decoder80<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u32, ParseError> {
let _ = _input.skip_align(4)?;
PResult::Ok(match is_be {
true => {
(Decoder21(_input))?
},

false => {
(Decoder89(_input))?
}
})
}

fn Decoder81<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u32, ParseError> {
let inner = (Decoder80(_input, is_be.clone()))?;
PResult::Ok(if ((|sh_type: u32| PResult::Ok(match sh_type {
0u32..=11u32 => {
true
},

14u32..=18u32 => {
true
},

1610612736u32..=4294967295u32 => {
true
},

_ => {
false
}
}))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
})
}

fn Decoder_elf_types_elf_full<'input>(_input: &mut Parser<'input>, is_be: bool, class: u8) -> Result<elf_types_elf_full, ParseError> {
PResult::Ok(match class {
1u8 => {
let inner = (Decoder80(_input, is_be.clone()))?;
elf_types_elf_full::Full32(inner)
},

2u8 => {
let inner = (Decoder92(_input, is_be.clone()))?;
elf_types_elf_full::Full64(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

fn Decoder_elf_types_elf_addr<'input>(_input: &mut Parser<'input>, is_be: bool, class: u8) -> Result<elf_types_elf_addr, ParseError> {
PResult::Ok(match class {
1u8 => {
let inner = (Decoder90(_input, is_be.clone()))?;
elf_types_elf_addr::Addr32(inner)
},

2u8 => {
let inner = (Decoder91(_input, is_be.clone()))?;
elf_types_elf_addr::Addr64(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

fn Decoder_elf_types_elf_off<'input>(_input: &mut Parser<'input>, is_be: bool, class: u8) -> Result<elf_types_elf_off, ParseError> {
PResult::Ok(match class {
1u8 => {
let inner = (Decoder86(_input, is_be.clone()))?;
elf_types_elf_off::Off32(inner)
},

2u8 => {
let inner = (Decoder87(_input, is_be.clone()))?;
elf_types_elf_off::Off64(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

fn Decoder85<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u32, ParseError> {
PResult::Ok((Decoder80(_input, is_be.clone()))?)
}

fn Decoder86<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u32, ParseError> {
let _ = _input.skip_align(4)?;
PResult::Ok(match is_be {
true => {
(Decoder21(_input))?
},

false => {
(Decoder89(_input))?
}
})
}

fn Decoder87<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u64, ParseError> {
let _ = _input.skip_align(8)?;
PResult::Ok(match is_be {
true => {
(Decoder61(_input))?
},

false => {
(Decoder88(_input))?
}
})
}

fn Decoder88<'input>(_input: &mut Parser<'input>) -> Result<u64, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
PResult::Ok(((|x: (u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(u64le(x)))(inner))?)
}

fn Decoder89<'input>(_input: &mut Parser<'input>) -> Result<u32, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2, field3)
};
PResult::Ok(((|x: (u8, u8, u8, u8)| PResult::Ok(u32le(x)))(inner))?)
}

fn Decoder90<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u32, ParseError> {
let _ = _input.skip_align(4)?;
PResult::Ok(match is_be {
true => {
(Decoder21(_input))?
},

false => {
(Decoder89(_input))?
}
})
}

fn Decoder91<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u64, ParseError> {
let _ = _input.skip_align(8)?;
PResult::Ok(match is_be {
true => {
(Decoder61(_input))?
},

false => {
(Decoder88(_input))?
}
})
}

fn Decoder92<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u64, ParseError> {
let _ = _input.skip_align(8)?;
PResult::Ok(match is_be {
true => {
(Decoder61(_input))?
},

false => {
(Decoder88(_input))?
}
})
}

fn Decoder_elf_phdr_table<'input>(_input: &mut Parser<'input>, is_be: bool, class: u8) -> Result<elf_phdr_table, ParseError> {
let r#type = ((|| PResult::Ok((Decoder80(_input, is_be.clone()))?))())?;
let flags64 = ((|| PResult::Ok((Decoder94(_input, is_be.clone(), class.clone()))?))())?;
let offset = ((|| PResult::Ok((Decoder_elf_types_elf_off(_input, is_be.clone(), class.clone()))?))())?;
let vaddr = ((|| PResult::Ok((Decoder_elf_types_elf_addr(_input, is_be.clone(), class.clone()))?))())?;
let paddr = ((|| PResult::Ok((Decoder_elf_types_elf_addr(_input, is_be.clone(), class.clone()))?))())?;
let filesz = ((|| PResult::Ok((Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?))())?;
let memsz = ((|| PResult::Ok((Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?))())?;
let flags32 = ((|| PResult::Ok((Decoder95(_input, is_be.clone(), class.clone()))?))())?;
let align = ((|| PResult::Ok((Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?))())?;
PResult::Ok(elf_phdr_table { r#type, flags64, offset, vaddr, paddr, filesz, memsz, flags32, align })
}

fn Decoder94<'input>(_input: &mut Parser<'input>, is_be: bool, class: u8) -> Result<Option<u32>, ParseError> {
PResult::Ok(if class == 2u8 {
Some((Decoder80(_input, is_be.clone()))?)
} else {
None
})
}

fn Decoder95<'input>(_input: &mut Parser<'input>, is_be: bool, class: u8) -> Result<Option<u32>, ParseError> {
PResult::Ok(if class == 1u8 {
Some((Decoder80(_input, is_be.clone()))?)
} else {
None
})
}

fn Decoder_elf_header_ident<'input>(_input: &mut Parser<'input>) -> Result<elf_header_ident, ParseError> {
let magic = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 127 {
b
} else {
return Err(ParseError::ExcludedBranch(13298811867334449791u64));
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
if b == 76 {
b
} else {
return Err(ParseError::ExcludedBranch(7343583089148506132u64));
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
let class = ((|| PResult::Ok((Decoder102(_input))?))())?;
let data = ((|| PResult::Ok((Decoder103(_input))?))())?;
let version = ((|| PResult::Ok((Decoder104(_input))?))())?;
let os_abi = ((|| PResult::Ok((Decoder105(_input))?))())?;
let abi_version = ((|| PResult::Ok((Decoder106(_input))?))())?;
let __pad = ((|| PResult::Ok({
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
PResult::Ok(elf_header_ident { magic, class, data, version, os_abi, abi_version, __pad })
}

fn Decoder97<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u16, ParseError> {
let inner = (Decoder100(_input, is_be.clone()))?;
PResult::Ok(if ((|r#type: u16| PResult::Ok(match r#type {
0u16..=4u16 => {
true
},

65024u16..=65279u16 => {
true
},

65280u16..=65535u16 => {
true
},

_ => {
false
}
}))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
})
}

fn Decoder98<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u16, ParseError> {
PResult::Ok((Decoder100(_input, is_be.clone()))?)
}

fn Decoder99<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u32, ParseError> {
let inner = (Decoder80(_input, is_be.clone()))?;
PResult::Ok(if ((|x: u32| PResult::Ok(x <= 1u32))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
})
}

fn Decoder100<'input>(_input: &mut Parser<'input>, is_be: bool) -> Result<u16, ParseError> {
let _ = _input.skip_align(2)?;
PResult::Ok(match is_be {
true => {
(Decoder24(_input))?
},

false => {
(Decoder101(_input))?
}
})
}

fn Decoder101<'input>(_input: &mut Parser<'input>) -> Result<u16, ParseError> {
let inner = {
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1)
};
PResult::Ok(((|x: (u8, u8)| PResult::Ok(u16le(x)))(inner))?)
}

fn Decoder102<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let inner = (Decoder25(_input))?;
PResult::Ok(if ((|x: u8| PResult::Ok(x <= 2u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
})
}

fn Decoder103<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let inner = (Decoder25(_input))?;
PResult::Ok(if ((|x: u8| PResult::Ok(x <= 2u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
})
}

fn Decoder104<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let inner = (Decoder25(_input))?;
PResult::Ok(if ((|x: u8| PResult::Ok(x <= 1u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
})
}

fn Decoder105<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
PResult::Ok((Decoder25(_input))?)
}

fn Decoder106<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
PResult::Ok((Decoder25(_input))?)
}

fn Decoder_tar_header_with_data<'input>(_input: &mut Parser<'input>) -> Result<tar_header_with_data, ParseError> {
let header = ((|| PResult::Ok((Decoder_tar_header(_input))?))())?;
let file = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..header.size.clone() {
accum.push((Decoder25(_input))?);
}
accum
}))())?;
let __padding = ((|| PResult::Ok(_input.skip_align(512)?))())?;
PResult::Ok(tar_header_with_data { header, file, __padding })
}

fn Decoder_tar_header<'input>(_input: &mut Parser<'input>) -> Result<tar_header, ParseError> {
let sz = 512u32 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let name = ((|| PResult::Ok({
let sz = 100u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_tar_ascii_string_opt0(_input))?))())?;
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
let next_elem = (Decoder110(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder111(_input))?))())?;
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
tar_header_uid { string, __nul_or_wsp, __padding }
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
let next_elem = (Decoder110(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder111(_input))?))())?;
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
tar_header_uid { string, __nul_or_wsp, __padding }
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
let next_elem = (Decoder110(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder111(_input))?))())?;
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
tar_header_uid { string, __nul_or_wsp, __padding }
}))())?;
_input.end_slice()?;
ret
}))())?;
let size = ((|| PResult::Ok({
let inner = {
let oA = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let o9 = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let o8 = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let o7 = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let o6 = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let o5 = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let o4 = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let o3 = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let o2 = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let o1 = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let o0 = ((|| PResult::Ok({
let inner = (Decoder110(_input))?;
((|bit: u8| PResult::Ok(try_sub!(bit as u8, 48u8)))(inner))?
}))())?;
let __nil = ((|| PResult::Ok((Decoder111(_input))?))())?;
let value = ((|| PResult::Ok((((0u8 as u32) << 3u32 | (oA as u32)) << 6u32 | (o9 as u32) << 3u32 | (o8 as u32)) << 24u32 | (((o7 as u32) << 3u32 | (o6 as u32)) << 6u32 | (o5 as u32) << 3u32 | (o4 as u32)) << 12u32 | ((o3 as u32) << 3u32 | (o2 as u32)) << 6u32 | (o1 as u32) << 3u32 | (o0 as u32)))())?;
tar_header_size_raw { oA, o9, o8, o7, o6, o5, o4, o3, o2, o1, o0, __nil, value }
};
((|rec: tar_header_size_raw| PResult::Ok(rec.value.clone()))(inner))?
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
let next_elem = (Decoder110(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder111(_input))?))())?;
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
tar_header_uid { string, __nul_or_wsp, __padding }
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
let next_elem = (Decoder110(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder111(_input))?))())?;
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
tar_header_uid { string, __nul_or_wsp, __padding }
}))())?;
_input.end_slice()?;
ret
}))())?;
let typeflag = ((|| PResult::Ok((Decoder112(_input))?))())?;
let linkname = ((|| PResult::Ok({
let sz = 100u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder113(_input))?))())?;
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
let ret = ((|| PResult::Ok((Decoder_tar_ascii_string(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let gname = ((|| PResult::Ok({
let sz = 32u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_tar_ascii_string(_input))?))())?;
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
let next_elem = (Decoder110(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder111(_input))?))())?;
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
tar_header_uid { string, __nul_or_wsp, __padding }
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
let next_elem = (Decoder110(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let __nul_or_wsp = ((|| PResult::Ok((Decoder111(_input))?))())?;
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
tar_header_uid { string, __nul_or_wsp, __padding }
}))())?;
_input.end_slice()?;
ret
}))())?;
let prefix = ((|| PResult::Ok({
let sz = 155u16 as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder113(_input))?))())?;
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
tar_header { name, mode, uid, gid, size, mtime, chksum, typeflag, linkname, magic, version, uname, gname, devmajor, devminor, prefix, pad }
}))())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_tar_ascii_string_opt0<'input>(_input: &mut Parser<'input>) -> Result<tar_ascii_string_opt0, ParseError> {
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
PResult::Ok(tar_ascii_string_opt0 { string, __padding })
}

fn Decoder110<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if (ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(16196330650984947656u64));
})
}

fn Decoder111<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if (ByteSet::from_bits([4294967297, 0, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9824667705306069359u64));
})
}

fn Decoder112<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(b)
}

fn Decoder113<'input>(_input: &mut Parser<'input>) -> Result<tar_ascii_string_opt0, ParseError> {
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
PResult::Ok(tar_ascii_string_opt0 { string, __padding })
}

fn Decoder_tar_ascii_string<'input>(_input: &mut Parser<'input>) -> Result<tar_ascii_string, ParseError> {
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
PResult::Ok(tar_ascii_string { string, padding })
}

fn Decoder_riff_subchunks<'input>(_input: &mut Parser<'input>) -> Result<riff_subchunks, ParseError> {
let tag = ((|| PResult::Ok((Decoder116(_input))?))())?;
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
let next_elem = (Decoder_riff_chunk(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(riff_subchunks { tag, chunks })
}

fn Decoder116<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = ((|| PResult::Ok((Decoder112(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder112(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder112(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder112(_input))?))())?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder_riff_chunk<'input>(_input: &mut Parser<'input>) -> Result<riff_chunk, ParseError> {
let tag = ((|| PResult::Ok((Decoder116(_input))?))())?;
let length = ((|| PResult::Ok((Decoder89(_input))?))())?;
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
let next_elem = (Decoder25(_input))?;
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
let pad = ((|| PResult::Ok(if length % 2u32 == 1u32 {
let b = _input.read_byte()?;
Some(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
})
} else {
None
}))())?;
PResult::Ok(riff_chunk { tag, length, data, pad })
}

fn Decoder118<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8, u8, u8, u8, u8), ParseError> {
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

fn Decoder_png_ihdr<'input>(_input: &mut Parser<'input>) -> Result<png_ihdr, ParseError> {
let length = ((|| PResult::Ok({
let inner = (Decoder21(_input))?;
if ((|length: u32| PResult::Ok(length <= 2147483647u32))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let tag = ((|| PResult::Ok((Decoder165(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_png_ihdr_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(png_ihdr { length, tag, data, crc })
}

fn Decoder_png_chunk<'input>(_input: &mut Parser<'input>, ihdr: png_ihdr) -> Result<png_chunk, ParseError> {
let length = ((|| PResult::Ok({
let inner = (Decoder21(_input))?;
if ((|length: u32| PResult::Ok(length <= 2147483647u32))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let tag = ((|| PResult::Ok({
let _ = {
_input.open_peek_not_context();
let _res = (|| PResult::Ok({
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(17197161005512507961u64));
}
}))();
if _res.is_err() {
_input.close_peek_not_context()?;
} else {
return Err(ParseError::NegatedSuccess);
}
()
};
let mut accum = Vec::new();
for _ in 0..4u32 {
accum.push((Decoder136(_input))?);
}
accum
}))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match tag.as_slice() {
[80u8, 76u8, 84u8, 69u8] => {
let inner = (Decoder137(_input))?;
png_chunk_data::PLTE(inner)
},

[116u8, 82u8, 78u8, 83u8] => {
let inner = (Decoder_png_trns(_input, ihdr.clone()))?;
png_chunk_data::tRNS(inner)
},

[99u8, 72u8, 82u8, 77u8] => {
let inner = (Decoder_png_chrm(_input))?;
png_chunk_data::cHRM(inner)
},

[103u8, 65u8, 77u8, 65u8] => {
let inner = (Decoder_png_gama(_input))?;
png_chunk_data::gAMA(inner)
},

[105u8, 67u8, 67u8, 80u8] => {
let inner = (Decoder_png_iccp(_input))?;
png_chunk_data::iCCP(inner)
},

[115u8, 66u8, 73u8, 84u8] => {
let inner = (Decoder_png_sbit(_input, ihdr.clone()))?;
png_chunk_data::sBIT(inner)
},

[115u8, 82u8, 71u8, 66u8] => {
let inner = (Decoder_png_srgb(_input))?;
png_chunk_data::sRGB(inner)
},

[105u8, 84u8, 88u8, 116u8] => {
let inner = (Decoder_png_itxt(_input))?;
png_chunk_data::iTXt(inner)
},

[116u8, 69u8, 88u8, 116u8] => {
let inner = (Decoder_png_text(_input))?;
png_chunk_data::tEXt(inner)
},

[122u8, 84u8, 88u8, 116u8] => {
let inner = (Decoder_png_ztxt(_input))?;
png_chunk_data::zTXt(inner)
},

[98u8, 75u8, 71u8, 68u8] => {
let inner = (Decoder_png_bkgd(_input, ihdr.clone()))?;
png_chunk_data::bKGD(inner)
},

[104u8, 73u8, 83u8, 84u8] => {
let inner = (Decoder_png_hist(_input))?;
png_chunk_data::hIST(inner)
},

[112u8, 72u8, 89u8, 115u8] => {
let inner = (Decoder_png_phys(_input))?;
png_chunk_data::pHYs(inner)
},

[115u8, 80u8, 76u8, 84u8] => {
let inner = (Decoder_png_splt(_input))?;
png_chunk_data::sPLT(inner)
},

[116u8, 73u8, 77u8, 69u8] => {
let inner = (Decoder_png_time(_input))?;
png_chunk_data::tIME(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
png_chunk_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(png_chunk { length, tag, data, crc })
}

fn Decoder_png_idat<'input>(_input: &mut Parser<'input>) -> Result<png_idat, ParseError> {
let length = ((|| PResult::Ok({
let inner = (Decoder21(_input))?;
if ((|length: u32| PResult::Ok(length <= 2147483647u32))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let tag = ((|| PResult::Ok((Decoder134(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder135(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(png_idat { length, tag, data, crc })
}

fn Decoder_zlib_main<'input>(_input: &mut Parser<'input>) -> Result<zlib_main, ParseError> {
let compression_method_flags = ((|| PResult::Ok({
let inner = {
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(zlib_main_compression_method_flags { compression_info: packedbits >> 4u8 & 15u8, compression_method: packedbits >> 0u8 & 15u8 }))(inner))?
};
if ((|method_info: zlib_main_compression_method_flags| PResult::Ok(method_info.compression_method.clone() == 8u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let flags = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(zlib_main_flags { flevel: packedbits >> 6u8 & 3u8, fdict: packedbits >> 5u8 & 1u8, fcheck: packedbits >> 0u8 & 31u8 }))(inner))?
}))())?;
let dict_id = ((|| PResult::Ok(if flags.fdict.clone() != 0u8 {
Some((Decoder21(_input))?)
} else {
None
}))())?;
let data = ((|| PResult::Ok({
_input.enter_bits_mode()?;
let ret = ((|| PResult::Ok((Decoder_deflate_main(_input))?))())?;
let _bits_read = _input.escape_bits_mode()?;
ret
}))())?;
let adler32 = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

fn Decoder_png_iend<'input>(_input: &mut Parser<'input>) -> Result<png_iend, ParseError> {
let length = ((|| PResult::Ok({
let inner = (Decoder21(_input))?;
if ((|length: u32| PResult::Ok(length <= 2147483647u32))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let tag = ((|| PResult::Ok((Decoder124(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = length as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder125(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
let crc = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(png_iend { length, tag, data, crc })
}

fn Decoder124<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
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

fn Decoder125<'input>(_input: &mut Parser<'input>) -> Result<(), ParseError> {
PResult::Ok(())
}

fn Decoder_deflate_main<'input>(_input: &mut Parser<'input>) -> Result<deflate_main, ParseError> {
let blocks = ((|| PResult::Ok({
let mut accum = Vec::new();
loop {
let elem = (Decoder_deflate_block(_input))?;
if ((|x: &deflate_block| PResult::Ok(x.r#final.clone() == 1u8))(&elem))? {
accum.push(elem);
break
} else {
accum.push(elem);
}
}
accum
}))())?;
let codes = ((|| PResult::Ok((try_flat_map_vec(blocks.iter().cloned(), |x: deflate_block| PResult::Ok(match x.data.clone() {
deflate_main_codes::uncompressed(y) => {
y.codes_values.clone()
},

deflate_main_codes::fixed_huffman(y) => {
y.codes_values.clone()
},

deflate_main_codes::dynamic_huffman(y) => {
y.codes_values.clone()
}
})))?))())?;
let inflate = ((|| PResult::Ok((try_flat_map_append_vec(codes.iter().cloned(), |tuple_var: (&Vec<u8>, deflate_main_codes__dupX1)| PResult::Ok(match tuple_var {
(buffer, symbol) => {
match symbol {
deflate_main_codes__dupX1::literal(b) => {
[b].to_vec()
},

deflate_main_codes__dupX1::reference(r) => {
{
let ix = (try_sub!((buffer.len()) as u32, (r.distance.clone()) as u32)) as usize;
(slice_ext(&buffer, ix..ix + (((r.length.clone()) as u32) as usize))).to_vec()
}
}
}
}
})))?))())?;
PResult::Ok(deflate_main { blocks, codes, inflate })
}

fn Decoder_deflate_block<'input>(_input: &mut Parser<'input>) -> Result<deflate_block, ParseError> {
let r#final = ((|| PResult::Ok((Decoder128(_input))?))())?;
let r#type = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8))(inner))?
}))())?;
let data = ((|| PResult::Ok(match r#type {
0u8 => {
let inner = (Decoder_deflate_uncompressed(_input))?;
deflate_main_codes::uncompressed(inner)
},

1u8 => {
let inner = (Decoder_deflate_fixed_huffman(_input))?;
deflate_main_codes::fixed_huffman(inner)
},

2u8 => {
let inner = (Decoder_deflate_dynamic_huffman(_input))?;
deflate_main_codes::dynamic_huffman(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
PResult::Ok(deflate_block { r#final, r#type, data })
}

fn Decoder128<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(b)
}

fn Decoder_deflate_uncompressed<'input>(_input: &mut Parser<'input>) -> Result<deflate_uncompressed, ParseError> {
let align = ((|| PResult::Ok(_input.skip_align(8)?))())?;
let len = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field10 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field11 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field12 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field13 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field14 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field15 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16 | ((bits.5.clone()) as u16) << 5u16 | ((bits.6.clone()) as u16) << 6u16 | ((bits.7.clone()) as u16) << 7u16 | ((bits.8.clone()) as u16) << 8u16 | ((bits.9.clone()) as u16) << 9u16 | ((bits.10.clone()) as u16) << 10u16 | ((bits.11.clone()) as u16) << 11u16 | ((bits.12.clone()) as u16) << 12u16 | ((bits.13.clone()) as u16) << 13u16 | ((bits.14.clone()) as u16) << 14u16 | ((bits.15.clone()) as u16) << 15u16))(inner))?
}))())?;
let nlen = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field10 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field11 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field12 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field13 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field14 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field15 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16 | ((bits.5.clone()) as u16) << 5u16 | ((bits.6.clone()) as u16) << 6u16 | ((bits.7.clone()) as u16) << 7u16 | ((bits.8.clone()) as u16) << 8u16 | ((bits.9.clone()) as u16) << 9u16 | ((bits.10.clone()) as u16) << 10u16 | ((bits.11.clone()) as u16) << 11u16 | ((bits.12.clone()) as u16) << 12u16 | ((bits.13.clone()) as u16) << 13u16 | ((bits.14.clone()) as u16) << 14u16 | ((bits.15.clone()) as u16) << 15u16))(inner))?
}))())?;
let bytes = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..len {
accum.push({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8 | bits.5.clone() << 5u8 | bits.6.clone() << 6u8 | bits.7.clone() << 7u8))(inner))?
});
}
accum
}))())?;
let codes_values = ((|| PResult::Ok((try_flat_map_vec(bytes.iter().cloned(), |x: u8| PResult::Ok([deflate_main_codes__dupX1::literal(x)].to_vec())))?))())?;
PResult::Ok(deflate_uncompressed { align, len, nlen, bytes, codes_values })
}

fn Decoder_deflate_fixed_huffman<'input>(_input: &mut Parser<'input>) -> Result<deflate_fixed_huffman, ParseError> {
let codes = ((|| PResult::Ok({
let format = parse_huffman([8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8].to_vec(), None);
let mut accum = Vec::new();
loop {
let elem = {
let code = ((|| PResult::Ok((format(_input))?))())?;
let extra = ((|| PResult::Ok(match code {
257u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(3u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

258u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(4u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

259u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(5u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

260u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(6u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

261u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(7u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

262u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(8u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

263u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(9u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

264u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(10u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

265u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.0.clone()))(inner))?
}))())?;
let length = ((|| PResult::Ok(11u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

266u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.0.clone()))(inner))?
}))())?;
let length = ((|| PResult::Ok(13u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

267u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.0.clone()))(inner))?
}))())?;
let length = ((|| PResult::Ok(15u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

268u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.0.clone()))(inner))?
}))())?;
let length = ((|| PResult::Ok(17u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

269u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(19u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

270u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(23u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

271u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(27u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

272u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(31u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

273u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(35u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

274u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(43u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

275u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(51u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

276u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(59u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

277u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(67u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

278u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(83u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

279u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(99u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

280u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(115u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

281u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(131u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

282u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(163u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

283u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(195u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

284u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(227u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

285u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(258u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok({
let inner = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder128(_input))?);
}
accum
};
((|bits: Vec<u8>| PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone()))(inner))?
}))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code as u16))?))())?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_fixed_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

286u16..=287u16 => {
None
},

_ => {
None
}
}))())?;
deflate_fixed_huffman_codes { code, extra }
};
if ((|x: &deflate_fixed_huffman_codes| PResult::Ok(((x.code.clone()) as u16) == 256u16))(&elem))? {
accum.push(elem);
break
} else {
accum.push(elem);
}
}
accum
}))())?;
let codes_values = ((|| PResult::Ok((try_flat_map_vec(codes.iter().cloned(), |x: deflate_fixed_huffman_codes| PResult::Ok(match x.code.clone() {
256u16 => {
[].to_vec()
},

257u16..=285u16 => {
match x.extra.clone() {
Some(ref rec) => {
[deflate_main_codes__dupX1::reference(deflate_main_codes_reference { length: rec.length.clone(), distance: rec.distance_record.distance.clone() })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(4350808036978594792u64));
}
}
},

286u16..=287u16 => {
[].to_vec()
},

_ => {
[deflate_main_codes__dupX1::literal((x.code.clone()) as u8)].to_vec()
}
})))?))())?;
PResult::Ok(deflate_fixed_huffman { codes, codes_values })
}

fn Decoder_deflate_dynamic_huffman<'input>(_input: &mut Parser<'input>) -> Result<deflate_dynamic_huffman, ParseError> {
let hlit = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8))(inner))?
}))())?;
let hdist = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8))(inner))?
}))())?;
let hclen = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8))(inner))?
}))())?;
let code_length_alphabet_code_lengths = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..hclen + 4u8 {
accum.push({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8))(inner))?
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
16u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8))(inner))?
},

17u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8))(inner))?
},

18u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8 | bits.5.clone() << 5u8 | bits.6.clone() << 6u8))(inner))?
},

_ => {
0u8
}
}))())?;
deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths { code, extra }
};
accum.push(elem);
if ((|y: &Vec<deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths>| PResult::Ok((((try_fold_map_curried(y.iter().cloned(), None, |tuple_var: (Option<u8>, deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths)| PResult::Ok(match tuple_var {
(last_symbol, cl_code_extra) => {
match (cl_code_extra.code.clone()) as u8 {
16u8 => {
(last_symbol.clone(), dup32((cl_code_extra.extra.clone() + 3u8) as u32, match last_symbol {
Some(x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(1219931083643546337u64));
}
}))
},

17u8 => {
(Some(0u8), dup32((cl_code_extra.extra.clone() + 3u8) as u32, 0u8))
},

18u8 => {
(Some(0u8), dup32((cl_code_extra.extra.clone() + 11u8) as u32, 0u8))
},

v => {
(Some(v.clone()), [v.clone()].to_vec())
}
}
}
})))?.len()) as u32) >= ((hlit + hdist) as u32) + 258u32))(&accum))? {
break
}
}
accum
}))())?;
let literal_length_distance_alphabet_code_lengths_value = ((|| PResult::Ok((try_fold_map_curried(literal_length_distance_alphabet_code_lengths.iter().cloned(), None, |tuple_var: (Option<u8>, deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths)| PResult::Ok(match tuple_var {
(last_symbol, cl_code_extra) => {
match (cl_code_extra.code.clone()) as u8 {
16u8 => {
(last_symbol.clone(), dup32((cl_code_extra.extra.clone() + 3u8) as u32, match last_symbol {
Some(x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(1219931083643546337u64));
}
}))
},

17u8 => {
(Some(0u8), dup32((cl_code_extra.extra.clone() + 3u8) as u32, 0u8))
},

18u8 => {
(Some(0u8), dup32((cl_code_extra.extra.clone() + 11u8) as u32, 0u8))
},

v => {
(Some(v.clone()), [v.clone()].to_vec())
}
}
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
257u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(3u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

258u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(4u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

259u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(5u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

260u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(6u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

261u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(7u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

262u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(8u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

263u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(9u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

264u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(10u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

265u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.0.clone()))(inner))?
}))())?;
let length = ((|| PResult::Ok(11u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

266u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.0.clone()))(inner))?
}))())?;
let length = ((|| PResult::Ok(13u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

267u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.0.clone()))(inner))?
}))())?;
let length = ((|| PResult::Ok(15u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

268u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok(bits.0.clone()))(inner))?
}))())?;
let length = ((|| PResult::Ok(17u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

269u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(19u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

270u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(23u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

271u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(27u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

272u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(31u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

273u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(35u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

274u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(43u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

275u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(51u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

276u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(59u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

277u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(67u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

278u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(83u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

279u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(99u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

280u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(115u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

281u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(131u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

282u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(163u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

283u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(195u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

284u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(bits.0.clone() << 0u8 | bits.1.clone() << 1u8 | bits.2.clone() << 2u8 | bits.3.clone() << 3u8 | bits.4.clone() << 4u8))(inner))?
}))())?;
let length = ((|| PResult::Ok(227u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

285u16 => {
let inner = {
let length_extra_bits = ((|| PResult::Ok(0u8))())?;
let length = ((|| PResult::Ok(258u16 + (length_extra_bits as u16)))())?;
let distance_code = ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
let distance_record = ((|| PResult::Ok((Decoder_deflate_distance_record(_input, distance_code.clone()))?))())?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
((|val: deflate_dynamic_huffman_codes_values| PResult::Ok(Some(val)))(inner))?
},

286u16..=287u16 => {
None
},

_ => {
None
}
}))())?;
deflate_dynamic_huffman_codes { code, extra }
};
if ((|x: &deflate_dynamic_huffman_codes| PResult::Ok(((x.code.clone()) as u16) == 256u16))(&elem))? {
accum.push(elem);
break
} else {
accum.push(elem);
}
}
accum
}))())?;
let codes_values = ((|| PResult::Ok((try_flat_map_vec(codes.iter().cloned(), |x: deflate_dynamic_huffman_codes| PResult::Ok(match x.code.clone() {
256u16 => {
[].to_vec()
},

257u16..=285u16 => {
match x.extra.clone() {
Some(ref rec) => {
[deflate_main_codes__dupX1::reference(deflate_main_codes_reference { length: rec.length.clone(), distance: rec.distance_record.distance.clone() })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(4350808036978594792u64));
}
}
},

286u16..=287u16 => {
[].to_vec()
},

_ => {
[deflate_main_codes__dupX1::literal((x.code.clone()) as u8)].to_vec()
}
})))?))())?;
PResult::Ok(deflate_dynamic_huffman { hlit, hdist, hclen, code_length_alphabet_code_lengths, literal_length_distance_alphabet_code_lengths, literal_length_distance_alphabet_code_lengths_value, literal_length_alphabet_code_lengths_value, distance_alphabet_code_lengths_value, codes, codes_values })
}

fn Decoder_deflate_distance_record<'input>(_input: &mut Parser<'input>, distance_code: u16) -> Result<deflate_distance_record, ParseError> {
PResult::Ok(match distance_code as u8 {
0u8 => {
(Decoder133(_input, 0u8, 1u16))?
},

1u8 => {
(Decoder133(_input, 0u8, 2u16))?
},

2u8 => {
(Decoder133(_input, 0u8, 3u16))?
},

3u8 => {
(Decoder133(_input, 0u8, 4u16))?
},

4u8 => {
(Decoder133(_input, 1u8, 5u16))?
},

5u8 => {
(Decoder133(_input, 1u8, 7u16))?
},

6u8 => {
(Decoder133(_input, 2u8, 9u16))?
},

7u8 => {
(Decoder133(_input, 2u8, 13u16))?
},

8u8 => {
(Decoder133(_input, 3u8, 17u16))?
},

9u8 => {
(Decoder133(_input, 3u8, 25u16))?
},

10u8 => {
(Decoder133(_input, 4u8, 33u16))?
},

11u8 => {
(Decoder133(_input, 4u8, 49u16))?
},

12u8 => {
(Decoder133(_input, 5u8, 65u16))?
},

13u8 => {
(Decoder133(_input, 5u8, 97u16))?
},

14u8 => {
(Decoder133(_input, 6u8, 129u16))?
},

15u8 => {
(Decoder133(_input, 6u8, 193u16))?
},

16u8 => {
(Decoder133(_input, 7u8, 257u16))?
},

17u8 => {
(Decoder133(_input, 7u8, 385u16))?
},

18u8 => {
(Decoder133(_input, 8u8, 513u16))?
},

19u8 => {
(Decoder133(_input, 8u8, 769u16))?
},

20u8 => {
(Decoder133(_input, 9u8, 1025u16))?
},

21u8 => {
(Decoder133(_input, 9u8, 1537u16))?
},

22u8 => {
(Decoder133(_input, 10u8, 2049u16))?
},

23u8 => {
(Decoder133(_input, 10u8, 3073u16))?
},

24u8 => {
(Decoder133(_input, 11u8, 4097u16))?
},

25u8 => {
(Decoder133(_input, 11u8, 6145u16))?
},

26u8 => {
(Decoder133(_input, 12u8, 8193u16))?
},

27u8 => {
(Decoder133(_input, 12u8, 12289u16))?
},

28u8 => {
(Decoder133(_input, 13u8, 16385u16))?
},

29u8 => {
(Decoder133(_input, 13u8, 24577u16))?
},

30u8..=31u8 => {
return Err(ParseError::FailToken);
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

fn Decoder133<'input>(_input: &mut Parser<'input>, extra_bits: u8, start: u16) -> Result<deflate_distance_record, ParseError> {
let distance_extra_bits = ((|| PResult::Ok(match extra_bits {
0u8 => {
0u16
},

1u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0,)
};
((|bits: (u8,)| PResult::Ok((bits.0.clone()) as u16))(inner))?
},

2u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1)
};
((|bits: (u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16))(inner))?
},

3u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2)
};
((|bits: (u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16))(inner))?
},

4u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3)
};
((|bits: (u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16))(inner))?
},

5u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4)
};
((|bits: (u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16))(inner))?
},

6u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5)
};
((|bits: (u8, u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16 | ((bits.5.clone()) as u16) << 5u16))(inner))?
},

7u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16 | ((bits.5.clone()) as u16) << 5u16 | ((bits.6.clone()) as u16) << 6u16))(inner))?
},

8u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16 | ((bits.5.clone()) as u16) << 5u16 | ((bits.6.clone()) as u16) << 6u16 | ((bits.7.clone()) as u16) << 7u16))(inner))?
},

9u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16 | ((bits.5.clone()) as u16) << 5u16 | ((bits.6.clone()) as u16) << 6u16 | ((bits.7.clone()) as u16) << 7u16 | ((bits.8.clone()) as u16) << 8u16))(inner))?
},

10u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16 | ((bits.5.clone()) as u16) << 5u16 | ((bits.6.clone()) as u16) << 6u16 | ((bits.7.clone()) as u16) << 7u16 | ((bits.8.clone()) as u16) << 8u16 | ((bits.9.clone()) as u16) << 9u16))(inner))?
},

11u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field10 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16 | ((bits.5.clone()) as u16) << 5u16 | ((bits.6.clone()) as u16) << 6u16 | ((bits.7.clone()) as u16) << 7u16 | ((bits.8.clone()) as u16) << 8u16 | ((bits.9.clone()) as u16) << 9u16 | ((bits.10.clone()) as u16) << 10u16))(inner))?
},

12u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field10 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field11 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16 | ((bits.5.clone()) as u16) << 5u16 | ((bits.6.clone()) as u16) << 6u16 | ((bits.7.clone()) as u16) << 7u16 | ((bits.8.clone()) as u16) << 8u16 | ((bits.9.clone()) as u16) << 9u16 | ((bits.10.clone()) as u16) << 10u16 | ((bits.11.clone()) as u16) << 11u16))(inner))?
},

13u8 => {
let inner = {
let field0 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field4 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field5 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field6 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field7 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field8 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field9 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field10 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field11 = ((|| PResult::Ok((Decoder128(_input))?))())?;
let field12 = ((|| PResult::Ok((Decoder128(_input))?))())?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12)
};
((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(((bits.0.clone()) as u16) << 0u16 | ((bits.1.clone()) as u16) << 1u16 | ((bits.2.clone()) as u16) << 2u16 | ((bits.3.clone()) as u16) << 3u16 | ((bits.4.clone()) as u16) << 4u16 | ((bits.5.clone()) as u16) << 5u16 | ((bits.6.clone()) as u16) << 6u16 | ((bits.7.clone()) as u16) << 7u16 | ((bits.8.clone()) as u16) << 8u16 | ((bits.9.clone()) as u16) << 9u16 | ((bits.10.clone()) as u16) << 10u16 | ((bits.11.clone()) as u16) << 11u16 | ((bits.12.clone()) as u16) << 12u16))(inner))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let distance = ((|| PResult::Ok(start + distance_extra_bits))())?;
PResult::Ok(deflate_distance_record { distance_extra_bits, distance })
}

fn Decoder134<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
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

fn Decoder135<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
}

fn Decoder136<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if (ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(3624518403832297354u64));
})
}

fn Decoder137<'input>(_input: &mut Parser<'input>) -> Result<Vec<png_plte>, ParseError> {
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
let r = ((|| PResult::Ok((Decoder25(_input))?))())?;
let g = ((|| PResult::Ok((Decoder25(_input))?))())?;
let b = ((|| PResult::Ok((Decoder25(_input))?))())?;
png_plte { r, g, b }
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder_png_trns<'input>(_input: &mut Parser<'input>, ihdr: png_ihdr) -> Result<png_trns, ParseError> {
PResult::Ok(match ihdr.data.color_type.clone() {
0u8 => {
let inner = {
let greyscale = ((|| PResult::Ok((Decoder24(_input))?))())?;
png_bkgd_color_type_0 { greyscale }
};
png_trns::color_type_0(inner)
},

2u8 => {
let inner = {
let red = ((|| PResult::Ok((Decoder24(_input))?))())?;
let green = ((|| PResult::Ok((Decoder24(_input))?))())?;
let blue = ((|| PResult::Ok((Decoder24(_input))?))())?;
png_bkgd_color_type_2 { red, green, blue }
};
png_trns::color_type_2(inner)
},

3u8 => {
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
let palette_index = ((|| PResult::Ok((Decoder25(_input))?))())?;
png_bkgd_color_type_3 { palette_index }
};
accum.push(next_elem);
} else {
break
}
}
accum
};
png_trns::color_type_3(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

fn Decoder_png_chrm<'input>(_input: &mut Parser<'input>) -> Result<png_chrm, ParseError> {
let whitepoint_x = ((|| PResult::Ok((Decoder21(_input))?))())?;
let whitepoint_y = ((|| PResult::Ok((Decoder21(_input))?))())?;
let red_x = ((|| PResult::Ok((Decoder21(_input))?))())?;
let red_y = ((|| PResult::Ok((Decoder21(_input))?))())?;
let green_x = ((|| PResult::Ok((Decoder21(_input))?))())?;
let green_y = ((|| PResult::Ok((Decoder21(_input))?))())?;
let blue_x = ((|| PResult::Ok((Decoder21(_input))?))())?;
let blue_y = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(png_chrm { whitepoint_x, whitepoint_y, red_x, red_y, green_x, green_y, blue_x, blue_y })
}

fn Decoder_png_gama<'input>(_input: &mut Parser<'input>) -> Result<png_gama, ParseError> {
let gamma = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(png_gama { gamma })
}

fn Decoder_png_iccp<'input>(_input: &mut Parser<'input>) -> Result<png_iccp, ParseError> {
let profile_name = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder163(_input))?))())?;
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
((|tuple_var: (Vec<u8>, u8)| PResult::Ok(match tuple_var {
(x, __null) => {
x
}
}))(inner))?
}))())?;
let compression_method = ((|| PResult::Ok({
let inner = (Decoder25(_input))?;
if ((|x: u8| PResult::Ok(x == 0u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let compressed_profile = ((|| PResult::Ok((Decoder164(_input))?))())?;
PResult::Ok(png_iccp { profile_name, compression_method, compressed_profile })
}

fn Decoder_png_sbit<'input>(_input: &mut Parser<'input>, ihdr: png_ihdr) -> Result<png_sbit, ParseError> {
PResult::Ok(match ihdr.data.color_type.clone() {
0u8 => {
let inner = {
let sig_greyscale_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
png_sbit_color_type_0 { sig_greyscale_bits }
};
png_sbit::color_type_0(inner)
},

2u8 => {
let inner = {
let sig_red_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
let sig_green_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
let sig_blue_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
png_sbit_color_type_2 { sig_red_bits, sig_green_bits, sig_blue_bits }
};
png_sbit::color_type_2(inner)
},

3u8 => {
let inner = {
let sig_red_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
let sig_green_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
let sig_blue_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
png_sbit_color_type_2 { sig_red_bits, sig_green_bits, sig_blue_bits }
};
png_sbit::color_type_3(inner)
},

4u8 => {
let inner = {
let sig_greyscale_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
let sig_alpha_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
png_sbit_color_type_4 { sig_greyscale_bits, sig_alpha_bits }
};
png_sbit::color_type_4(inner)
},

6u8 => {
let inner = {
let sig_red_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
let sig_green_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
let sig_blue_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
let sig_alpha_bits = ((|| PResult::Ok((Decoder25(_input))?))())?;
png_sbit_color_type_6 { sig_red_bits, sig_green_bits, sig_blue_bits, sig_alpha_bits }
};
png_sbit::color_type_6(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

fn Decoder_png_srgb<'input>(_input: &mut Parser<'input>) -> Result<png_srgb, ParseError> {
let rendering_intent = ((|| PResult::Ok({
let inner = (Decoder25(_input))?;
if ((|x: u8| PResult::Ok(x <= 3u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
PResult::Ok(png_srgb { rendering_intent })
}

fn Decoder_png_itxt<'input>(_input: &mut Parser<'input>) -> Result<png_itxt, ParseError> {
let keyword = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder158(_input))?))())?;
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
((|tuple_var: (Vec<u8>, u8)| PResult::Ok(match tuple_var {
(x, __null) => {
x
}
}))(inner))?
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
let language_tag = ((|| PResult::Ok((Decoder_base_asciiz_string(_input))?))())?;
let translated_keyword = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder160(_input))?))())?;
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
((|tuple_var: (Vec<char>, u8)| PResult::Ok(match tuple_var {
(x, __null) => {
x
}
}))(inner))?
}))())?;
let text = ((|| PResult::Ok(match compression_flag == 1u8 {
true => {
_input.start_alt();
{
let mut f_tmp = || PResult::Ok({
let inner = {
let inner = {
let zlib = (Decoder161(_input))?;
let mut tmp = Parser::new(zlib.data.inflate.as_slice());
let reparser = &mut tmp;
(Decoder162(reparser))?
};
png_itxt_text_compressed::valid(inner)
};
png_itxt_text::compressed(inner)
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
let inner = {
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
png_itxt_text_compressed::invalid(inner)
};
png_itxt_text::compressed(inner)
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
},

false => {
let inner = (Decoder155(_input))?;
png_itxt_text::uncompressed(inner)
}
}))())?;
PResult::Ok(png_itxt { keyword, compression_flag, compression_method, language_tag, translated_keyword, text })
}

fn Decoder_png_text<'input>(_input: &mut Parser<'input>) -> Result<png_text, ParseError> {
let keyword = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder157(_input))?))())?;
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
((|tuple_var: (Vec<u8>, u8)| PResult::Ok(match tuple_var {
(x, __null) => {
x
}
}))(inner))?
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
let next_elem = (Decoder112(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(png_text { keyword, text })
}

fn Decoder_png_ztxt<'input>(_input: &mut Parser<'input>) -> Result<png_ztxt, ParseError> {
let keyword = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder153(_input))?))())?;
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
((|tuple_var: (Vec<u8>, u8)| PResult::Ok(match tuple_var {
(x, __null) => {
x
}
}))(inner))?
}))())?;
let compression_method = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
let compressed_text = ((|| PResult::Ok({
let zlib = (Decoder154(_input))?;
let mut tmp = Parser::new(zlib.data.inflate.as_slice());
let reparser = &mut tmp;
(Decoder155(reparser))?
}))())?;
PResult::Ok(png_ztxt { keyword, compression_method, compressed_text })
}

fn Decoder_png_bkgd<'input>(_input: &mut Parser<'input>, ihdr: png_ihdr) -> Result<png_bkgd, ParseError> {
PResult::Ok(match ihdr.data.color_type.clone() {
0u8 => {
let inner = {
let greyscale = ((|| PResult::Ok((Decoder24(_input))?))())?;
png_bkgd_color_type_0 { greyscale }
};
png_bkgd::color_type_0(inner)
},

4u8 => {
let inner = {
let greyscale = ((|| PResult::Ok((Decoder24(_input))?))())?;
png_bkgd_color_type_0 { greyscale }
};
png_bkgd::color_type_4(inner)
},

2u8 => {
let inner = {
let red = ((|| PResult::Ok((Decoder24(_input))?))())?;
let green = ((|| PResult::Ok((Decoder24(_input))?))())?;
let blue = ((|| PResult::Ok((Decoder24(_input))?))())?;
png_bkgd_color_type_2 { red, green, blue }
};
png_bkgd::color_type_2(inner)
},

6u8 => {
let inner = {
let red = ((|| PResult::Ok((Decoder24(_input))?))())?;
let green = ((|| PResult::Ok((Decoder24(_input))?))())?;
let blue = ((|| PResult::Ok((Decoder24(_input))?))())?;
png_bkgd_color_type_2 { red, green, blue }
};
png_bkgd::color_type_6(inner)
},

3u8 => {
let inner = {
let palette_index = ((|| PResult::Ok((Decoder25(_input))?))())?;
png_bkgd_color_type_3 { palette_index }
};
png_bkgd::color_type_3(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

fn Decoder_png_hist<'input>(_input: &mut Parser<'input>) -> Result<png_hist, ParseError> {
let histogram = ((|| PResult::Ok({
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(png_hist { histogram })
}

fn Decoder_png_phys<'input>(_input: &mut Parser<'input>) -> Result<png_phys, ParseError> {
let pixels_per_unit_x = ((|| PResult::Ok((Decoder21(_input))?))())?;
let pixels_per_unit_y = ((|| PResult::Ok((Decoder21(_input))?))())?;
let unit_specifier = ((|| PResult::Ok((Decoder25(_input))?))())?;
PResult::Ok(png_phys { pixels_per_unit_x, pixels_per_unit_y, unit_specifier })
}

fn Decoder_png_splt<'input>(_input: &mut Parser<'input>) -> Result<png_splt, ParseError> {
let palette_name = ((|| PResult::Ok({
let inner = {
let field0 = ((|| PResult::Ok((Decoder152(_input))?))())?;
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
((|tuple_var: (Vec<u8>, u8)| PResult::Ok(match tuple_var {
(x, __null) => {
x
}
}))(inner))?
}))())?;
let sample_depth = ((|| PResult::Ok({
let inner = (Decoder25(_input))?;
if ((|x: u8| PResult::Ok((x == 8u8) || (x == 16u8)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let pallette = ((|| PResult::Ok(match sample_depth {
8u8 => {
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
let red = ((|| PResult::Ok((Decoder25(_input))?))())?;
let green = ((|| PResult::Ok((Decoder25(_input))?))())?;
let blue = ((|| PResult::Ok((Decoder25(_input))?))())?;
let alpha = ((|| PResult::Ok((Decoder25(_input))?))())?;
let frequency = ((|| PResult::Ok((Decoder24(_input))?))())?;
png_splt_pallette_sample_depth_u8 { red, green, blue, alpha, frequency }
};
accum.push(next_elem);
} else {
break
}
}
accum
};
png_splt_pallette::sample_depth_u8(inner)
},

16u8 => {
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
let red = ((|| PResult::Ok((Decoder24(_input))?))())?;
let green = ((|| PResult::Ok((Decoder24(_input))?))())?;
let blue = ((|| PResult::Ok((Decoder24(_input))?))())?;
let alpha = ((|| PResult::Ok((Decoder24(_input))?))())?;
let frequency = ((|| PResult::Ok((Decoder24(_input))?))())?;
png_splt_pallette_sample_depth_u16 { red, green, blue, alpha, frequency }
};
accum.push(next_elem);
} else {
break
}
}
accum
};
png_splt_pallette::sample_depth_u16(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
PResult::Ok(png_splt { palette_name, sample_depth, pallette })
}

fn Decoder_png_time<'input>(_input: &mut Parser<'input>) -> Result<png_time, ParseError> {
let year = ((|| PResult::Ok((Decoder24(_input))?))())?;
let month = ((|| PResult::Ok((Decoder25(_input))?))())?;
let day = ((|| PResult::Ok((Decoder25(_input))?))())?;
let hour = ((|| PResult::Ok((Decoder25(_input))?))())?;
let minute = ((|| PResult::Ok((Decoder25(_input))?))())?;
let second = ((|| PResult::Ok((Decoder25(_input))?))())?;
PResult::Ok(png_time { year, month, day, hour, minute, second })
}

fn Decoder152<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
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

fn Decoder153<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
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

fn Decoder154<'input>(_input: &mut Parser<'input>) -> Result<zlib_main, ParseError> {
let compression_method_flags = ((|| PResult::Ok({
let inner = {
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(zlib_main_compression_method_flags { compression_info: packedbits >> 4u8 & 15u8, compression_method: packedbits >> 0u8 & 15u8 }))(inner))?
};
if ((|method_info: zlib_main_compression_method_flags| PResult::Ok(method_info.compression_method.clone() == 8u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let flags = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(zlib_main_flags { flevel: packedbits >> 6u8 & 3u8, fdict: packedbits >> 5u8 & 1u8, fcheck: packedbits >> 0u8 & 31u8 }))(inner))?
}))())?;
let dict_id = ((|| PResult::Ok(if flags.fdict.clone() != 0u8 {
Some((Decoder21(_input))?)
} else {
None
}))())?;
let data = ((|| PResult::Ok({
_input.enter_bits_mode()?;
let ret = ((|| PResult::Ok((Decoder_deflate_main(_input))?))())?;
let _bits_read = _input.escape_bits_mode()?;
ret
}))())?;
let adler32 = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

fn Decoder155<'input>(_input: &mut Parser<'input>) -> Result<Vec<char>, ParseError> {
PResult::Ok((Decoder156(_input))?)
}

fn Decoder156<'input>(_input: &mut Parser<'input>) -> Result<Vec<char>, ParseError> {
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

tmp if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(tmp)) => {
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
return Err(ParseError::ExcludedBranch(975831965879443532u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder17(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
}

fn Decoder157<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
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

fn Decoder158<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
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

fn Decoder_base_asciiz_string<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder160<'input>(_input: &mut Parser<'input>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(tmp)) => {
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

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(11732108077980426261u64));
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
PResult::Ok(accum)
}

fn Decoder161<'input>(_input: &mut Parser<'input>) -> Result<zlib_main, ParseError> {
let compression_method_flags = ((|| PResult::Ok({
let inner = {
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(zlib_main_compression_method_flags { compression_info: packedbits >> 4u8 & 15u8, compression_method: packedbits >> 0u8 & 15u8 }))(inner))?
};
if ((|method_info: zlib_main_compression_method_flags| PResult::Ok(method_info.compression_method.clone() == 8u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let flags = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(zlib_main_flags { flevel: packedbits >> 6u8 & 3u8, fdict: packedbits >> 5u8 & 1u8, fcheck: packedbits >> 0u8 & 31u8 }))(inner))?
}))())?;
let dict_id = ((|| PResult::Ok(if flags.fdict.clone() != 0u8 {
Some((Decoder21(_input))?)
} else {
None
}))())?;
let data = ((|| PResult::Ok({
_input.enter_bits_mode()?;
let ret = ((|| PResult::Ok((Decoder_deflate_main(_input))?))())?;
let _bits_read = _input.escape_bits_mode()?;
ret
}))())?;
let adler32 = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

fn Decoder162<'input>(_input: &mut Parser<'input>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = match b {
tmp if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(tmp)) => {
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
return Err(ParseError::ExcludedBranch(15557476372391663512u64));
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
PResult::Ok(accum)
}

fn Decoder163<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
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

fn Decoder164<'input>(_input: &mut Parser<'input>) -> Result<zlib_main, ParseError> {
let compression_method_flags = ((|| PResult::Ok({
let inner = {
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(zlib_main_compression_method_flags { compression_info: packedbits >> 4u8 & 15u8, compression_method: packedbits >> 0u8 & 15u8 }))(inner))?
};
if ((|method_info: zlib_main_compression_method_flags| PResult::Ok(method_info.compression_method.clone() == 8u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let flags = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(zlib_main_flags { flevel: packedbits >> 6u8 & 3u8, fdict: packedbits >> 5u8 & 1u8, fcheck: packedbits >> 0u8 & 31u8 }))(inner))?
}))())?;
let dict_id = ((|| PResult::Ok(if flags.fdict.clone() != 0u8 {
Some((Decoder21(_input))?)
} else {
None
}))())?;
let data = ((|| PResult::Ok({
_input.enter_bits_mode()?;
let ret = ((|| PResult::Ok((Decoder_deflate_main(_input))?))())?;
let _bits_read = _input.escape_bits_mode()?;
ret
}))())?;
let adler32 = ((|| PResult::Ok((Decoder21(_input))?))())?;
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

fn Decoder165<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
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

fn Decoder_png_ihdr_data<'input>(_input: &mut Parser<'input>) -> Result<png_ihdr_data, ParseError> {
let width = ((|| PResult::Ok((Decoder21(_input))?))())?;
let height = ((|| PResult::Ok((Decoder21(_input))?))())?;
let bit_depth = ((|| PResult::Ok((Decoder25(_input))?))())?;
let color_type = ((|| PResult::Ok((Decoder25(_input))?))())?;
let compression_method = ((|| PResult::Ok((Decoder25(_input))?))())?;
let filter_method = ((|| PResult::Ok((Decoder25(_input))?))())?;
let interlace_method = ((|| PResult::Ok((Decoder25(_input))?))())?;
PResult::Ok(png_ihdr_data { width, height, bit_depth, color_type, compression_method, filter_method, interlace_method })
}

fn Decoder_mpeg4_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(102u8, 116u8, 121u8, 112u8) => {
let inner = {
let major_brand = ((|| PResult::Ok((Decoder168(_input))?))())?;
let minor_version = ((|| PResult::Ok((Decoder21(_input))?))())?;
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
let next_elem = (Decoder168(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
mpeg4_atom_data_ftyp { major_brand, minor_version, compatible_brands }
};
mpeg4_atom_data::ftyp(inner)
},

(102u8, 114u8, 101u8, 101u8) => {
mpeg4_atom_data::free
},

(109u8, 100u8, 97u8, 116u8) => {
mpeg4_atom_data::mdat
},

(109u8, 101u8, 116u8, 97u8) => {
let field0 = ((|| PResult::Ok((Decoder21(_input))?))())?;
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
let next_elem = (Decoder_mpeg4_meta_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
mpeg4_atom_data::meta(field0, field1)
},

(109u8, 111u8, 111u8, 118u8) => {
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
let next_elem = (Decoder_mpeg4_moov_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_atom_data::moov(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_atom { size_field, r#type, size, data })
}

fn Decoder168<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = ((|| PResult::Ok((Decoder112(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder112(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder112(_input))?))())?;
let field3 = ((|| PResult::Ok((Decoder112(_input))?))())?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder_mpeg4_meta_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_meta_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(100u8, 105u8, 110u8, 102u8) => {
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
let next_elem = (Decoder_mpeg4_dinf_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_meta_atom_data::dinf(inner)
},

(104u8, 100u8, 108u8, 114u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let predefined = ((|| PResult::Ok((Decoder21(_input))?))())?;
let handler_type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let reserved = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder21(_input))?))())?;
(field0, field1, field2)
}))())?;
let name = ((|| PResult::Ok((Decoder175(_input))?))())?;
mpeg4_meta_atom_data_hdlr { version, flags, predefined, handler_type, reserved, name }
};
mpeg4_meta_atom_data::hdlr(inner)
},

(112u8, 105u8, 116u8, 109u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let item_ID = ((|| PResult::Ok(match version == 0u8 {
true => {
let inner = (Decoder24(_input))?;
mpeg4_meta_atom_data_pitm_item_ID::yes(inner)
},

false => {
let inner = (Decoder21(_input))?;
mpeg4_meta_atom_data_pitm_item_ID::no(inner)
}
}))())?;
mpeg4_meta_atom_data_pitm { version, flags, item_ID }
};
mpeg4_meta_atom_data::pitm(inner)
},

(105u8, 105u8, 110u8, 102u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok(match version == 0u8 {
true => {
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(x as u32))(inner))?
},

false => {
(Decoder21(_input))?
}
}))())?;
let item_info_entry = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder_mpeg4_iinf_atom(_input))?);
}
accum
}))())?;
mpeg4_meta_atom_data_iinf { version, flags, entry_count, item_info_entry }
};
mpeg4_meta_atom_data::iinf(inner)
},

(105u8, 114u8, 101u8, 102u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let single_item_reference = ((|| PResult::Ok(match version {
0u8 => {
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
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let from_item_ID = ((|| PResult::Ok((Decoder24(_input))?))())?;
let reference_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let to_item_ID = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..reference_count {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
mpeg4_meta_atom_data_iref_single_item_reference_small_data { from_item_ID, reference_count, to_item_ID }
}))())?;
_input.end_slice()?;
ret
}))())?;
mpeg4_meta_atom_data_iref_single_item_reference_small { size_field, r#type, size, data }
};
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_meta_atom_data_iref_single_item_reference::small(inner)
},

1u8 => {
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
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok({
let from_item_ID = ((|| PResult::Ok((Decoder21(_input))?))())?;
let reference_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let to_item_ID = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..reference_count {
accum.push((Decoder21(_input))?);
}
accum
}))())?;
mpeg4_meta_atom_data_iref_single_item_reference_large_data { from_item_ID, reference_count, to_item_ID }
}))())?;
_input.end_slice()?;
ret
}))())?;
mpeg4_meta_atom_data_iref_single_item_reference_large { size_field, r#type, size, data }
};
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_meta_atom_data_iref_single_item_reference::large(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
mpeg4_meta_atom_data_iref { version, flags, single_item_reference }
};
mpeg4_meta_atom_data::iref(inner)
},

(105u8, 108u8, 111u8, 99u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let offset_size_length_size = ((|| PResult::Ok((Decoder25(_input))?))())?;
let base_offset_size_index_size = ((|| PResult::Ok((Decoder25(_input))?))())?;
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
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(x as u32))(inner))?
},

false => {
(Decoder21(_input))?
}
}))())?;
let items = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..item_count {
accum.push({
let item_ID = ((|| PResult::Ok(match version < 2u8 {
true => {
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(x as u32))(inner))?
},

false => {
(Decoder21(_input))?
}
}))())?;
let construction_method = ((|| PResult::Ok(if version > 0u8 {
Some((Decoder24(_input))?)
} else {
None
}))())?;
let data_reference_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
let base_offset = ((|| PResult::Ok(match base_offset_size {
0u8 => {
0u64
},

4u8 => {
let inner = (Decoder21(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8u8 => {
(Decoder61(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let extent_count = ((|| PResult::Ok((Decoder24(_input))?))())?;
let extents = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..extent_count {
accum.push({
let extent_index = ((|| PResult::Ok(match index_size {
0u8 => {
0u64
},

4u8 => {
let inner = (Decoder21(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8u8 => {
(Decoder61(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let extent_offset = ((|| PResult::Ok(match offset_size {
0u8 => {
0u64
},

4u8 => {
let inner = (Decoder21(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8u8 => {
(Decoder61(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let extent_length = ((|| PResult::Ok(match length_size {
0u8 => {
0u64
},

4u8 => {
let inner = (Decoder21(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8u8 => {
(Decoder61(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
mpeg4_meta_atom_data_iloc_items_extents { extent_index, extent_offset, extent_length }
});
}
accum
}))())?;
mpeg4_meta_atom_data_iloc_items { item_ID, construction_method, data_reference_index, base_offset, extent_count, extents }
});
}
accum
}))())?;
mpeg4_meta_atom_data_iloc { version, flags, offset_size_length_size, base_offset_size_index_size, offset_size, length_size, base_offset_size, index_size, item_count, items }
};
mpeg4_meta_atom_data::iloc(inner)
},

(105u8, 108u8, 115u8, 116u8) => {
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
let next_elem = (Decoder_mpeg4_ilst_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_meta_atom_data::ilst(inner)
},

(105u8, 100u8, 97u8, 116u8) => {
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_meta_atom_data::idat(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_meta_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_meta_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_moov_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_moov_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(109u8, 118u8, 104u8, 100u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let fields = ((|| PResult::Ok(match version {
0u8 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder21(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder21(_input))?))())?;
let timescale = ((|| PResult::Ok((Decoder21(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder21(_input))?))())?;
mpeg4_moov_atom_data_mvhd_fields_version0 { creation_time, modification_time, timescale, duration }
};
mpeg4_moov_atom_data_mvhd_fields::version0(inner)
},

1u8 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder61(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder61(_input))?))())?;
let timescale = ((|| PResult::Ok((Decoder21(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder61(_input))?))())?;
mpeg4_moov_atom_data_mvhd_fields_version1 { creation_time, modification_time, timescale, duration }
};
mpeg4_moov_atom_data_mvhd_fields::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let rate = ((|| PResult::Ok((Decoder21(_input))?))())?;
let volume = ((|| PResult::Ok((Decoder24(_input))?))())?;
let reserved1 = ((|| PResult::Ok((Decoder24(_input))?))())?;
let reserved2 = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder21(_input))?))())?;
(field0, field1)
}))())?;
let matrix = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..9u8 {
accum.push((Decoder21(_input))?);
}
accum
}))())?;
let pre_defined = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..6u8 {
accum.push((Decoder21(_input))?);
}
accum
}))())?;
let next_track_ID = ((|| PResult::Ok((Decoder21(_input))?))())?;
mpeg4_moov_atom_data_mvhd { version, flags, fields, rate, volume, reserved1, reserved2, matrix, pre_defined, next_track_ID }
};
mpeg4_moov_atom_data::mvhd(inner)
},

(116u8, 114u8, 97u8, 107u8) => {
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
let next_elem = (Decoder_mpeg4_trak_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_moov_atom_data::trak(inner)
},

(117u8, 100u8, 116u8, 97u8) => {
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
let next_elem = (Decoder_mpeg4_udta_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_moov_atom_data::udta(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_moov_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_moov_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_trak_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_trak_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(116u8, 107u8, 104u8, 100u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let fields = ((|| PResult::Ok(match version {
0u8 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder21(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder21(_input))?))())?;
let track_ID = ((|| PResult::Ok((Decoder21(_input))?))())?;
let reserved = ((|| PResult::Ok((Decoder21(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder21(_input))?))())?;
mpeg4_trak_atom_data_tkhd_fields_version0 { creation_time, modification_time, track_ID, reserved, duration }
};
mpeg4_trak_atom_data_tkhd_fields::version0(inner)
},

1u8 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder61(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder61(_input))?))())?;
let track_ID = ((|| PResult::Ok((Decoder21(_input))?))())?;
let reserved = ((|| PResult::Ok((Decoder21(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder61(_input))?))())?;
mpeg4_trak_atom_data_tkhd_fields_version1 { creation_time, modification_time, track_ID, reserved, duration }
};
mpeg4_trak_atom_data_tkhd_fields::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let reserved2 = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder21(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder21(_input))?))())?;
(field0, field1)
}))())?;
let layer = ((|| PResult::Ok((Decoder24(_input))?))())?;
let alternate_group = ((|| PResult::Ok((Decoder24(_input))?))())?;
let volume = ((|| PResult::Ok((Decoder24(_input))?))())?;
let reserved1 = ((|| PResult::Ok((Decoder24(_input))?))())?;
let matrix = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..9u8 {
accum.push((Decoder21(_input))?);
}
accum
}))())?;
let width = ((|| PResult::Ok((Decoder21(_input))?))())?;
let height = ((|| PResult::Ok((Decoder21(_input))?))())?;
mpeg4_trak_atom_data_tkhd { version, flags, fields, reserved2, layer, alternate_group, volume, reserved1, matrix, width, height }
};
mpeg4_trak_atom_data::tkhd(inner)
},

(101u8, 100u8, 116u8, 115u8) => {
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
let next_elem = (Decoder_mpeg4_edts_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_trak_atom_data::edts(inner)
},

(109u8, 100u8, 105u8, 97u8) => {
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
let next_elem = (Decoder_mpeg4_mdia_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_trak_atom_data::mdia(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_trak_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_trak_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_udta_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_udta_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(109u8, 101u8, 116u8, 97u8) => {
let field0 = ((|| PResult::Ok((Decoder21(_input))?))())?;
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
let next_elem = (Decoder_mpeg4_meta_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
mpeg4_udta_atom_data::meta(field0, field1)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_udta_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_udta_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_edts_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_edts_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(101u8, 108u8, 115u8, 116u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let number_of_entries = ((|| PResult::Ok((Decoder21(_input))?))())?;
let edit_list_table = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..number_of_entries {
accum.push({
let track_duration = ((|| PResult::Ok((Decoder21(_input))?))())?;
let media_time = ((|| PResult::Ok((Decoder21(_input))?))())?;
let media_rate = ((|| PResult::Ok((Decoder21(_input))?))())?;
mpeg4_edts_atom_data_elst_edit_list_table { track_duration, media_time, media_rate }
});
}
accum
}))())?;
mpeg4_edts_atom_data_elst { version, flags, number_of_entries, edit_list_table }
};
mpeg4_edts_atom_data::elst(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_edts_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_edts_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_mdia_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_mdia_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(104u8, 100u8, 108u8, 114u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let component_type = ((|| PResult::Ok((Decoder21(_input))?))())?;
let component_subtype = ((|| PResult::Ok((Decoder168(_input))?))())?;
let component_manufacturer = ((|| PResult::Ok((Decoder21(_input))?))())?;
let component_flags = ((|| PResult::Ok((Decoder21(_input))?))())?;
let component_flags_mask = ((|| PResult::Ok((Decoder21(_input))?))())?;
let component_name = ((|| PResult::Ok((Decoder175(_input))?))())?;
mpeg4_mdia_atom_data_hdlr { version, flags, component_type, component_subtype, component_manufacturer, component_flags, component_flags_mask, component_name }
};
mpeg4_mdia_atom_data::hdlr(inner)
},

(109u8, 100u8, 104u8, 100u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let fields = ((|| PResult::Ok(match version {
0u8 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder21(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder21(_input))?))())?;
let timescale = ((|| PResult::Ok((Decoder21(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder21(_input))?))())?;
mpeg4_moov_atom_data_mvhd_fields_version0 { creation_time, modification_time, timescale, duration }
};
mpeg4_moov_atom_data_mvhd_fields::version0(inner)
},

1u8 => {
let inner = {
let creation_time = ((|| PResult::Ok((Decoder61(_input))?))())?;
let modification_time = ((|| PResult::Ok((Decoder61(_input))?))())?;
let timescale = ((|| PResult::Ok((Decoder21(_input))?))())?;
let duration = ((|| PResult::Ok((Decoder61(_input))?))())?;
mpeg4_moov_atom_data_mvhd_fields_version1 { creation_time, modification_time, timescale, duration }
};
mpeg4_moov_atom_data_mvhd_fields::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let language = ((|| PResult::Ok((Decoder24(_input))?))())?;
let pre_defined = ((|| PResult::Ok((Decoder24(_input))?))())?;
mpeg4_mdia_atom_data_mdhd { version, flags, fields, language, pre_defined }
};
mpeg4_mdia_atom_data::mdhd(inner)
},

(109u8, 105u8, 110u8, 102u8) => {
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
let next_elem = (Decoder_mpeg4_minf_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_mdia_atom_data::minf(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_mdia_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_mdia_atom { size_field, r#type, size, data })
}

fn Decoder175<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_mpeg4_minf_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_minf_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(118u8, 109u8, 104u8, 100u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let graphicsmode = ((|| PResult::Ok((Decoder24(_input))?))())?;
let opcolor = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..3u8 {
accum.push((Decoder24(_input))?);
}
accum
}))())?;
mpeg4_minf_atom_data_vmhd { version, flags, graphicsmode, opcolor }
};
mpeg4_minf_atom_data::vmhd(inner)
},

(115u8, 109u8, 104u8, 100u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let balance = ((|| PResult::Ok((Decoder24(_input))?))())?;
let reserved = ((|| PResult::Ok((Decoder24(_input))?))())?;
mpeg4_minf_atom_data_smhd { version, flags, balance, reserved }
};
mpeg4_minf_atom_data::smhd(inner)
},

(100u8, 105u8, 110u8, 102u8) => {
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
let next_elem = (Decoder_mpeg4_dinf_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_minf_atom_data::dinf(inner)
},

(115u8, 116u8, 98u8, 108u8) => {
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
let next_elem = (Decoder_mpeg4_stbl_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_minf_atom_data::stbl(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_minf_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_minf_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_dinf_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_dinf_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(100u8, 114u8, 101u8, 102u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let number_of_entries = ((|| PResult::Ok((Decoder21(_input))?))())?;
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
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
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
let next_elem = (Decoder25(_input))?;
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
mpeg4_stbl_atom_data_stsd_sample_entries { size_field, r#type, size, data }
};
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
mpeg4_dinf_atom_data_dref { version, flags, number_of_entries, data }
};
mpeg4_dinf_atom_data::dref(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_dinf_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_dinf_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_stbl_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_stbl_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(115u8, 116u8, 115u8, 100u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let sample_entries = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
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
let next_elem = (Decoder25(_input))?;
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
mpeg4_stbl_atom_data_stsd_sample_entries { size_field, r#type, size, data }
});
}
accum
}))())?;
mpeg4_stbl_atom_data_stsd { version, flags, entry_count, sample_entries }
};
mpeg4_stbl_atom_data::stsd(inner)
},

(115u8, 116u8, 116u8, 115u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let sample_entries = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let sample_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let sample_delta = ((|| PResult::Ok((Decoder21(_input))?))())?;
mpeg4_stbl_atom_data_stts_sample_entries { sample_count, sample_delta }
});
}
accum
}))())?;
mpeg4_stbl_atom_data_stts { version, flags, entry_count, sample_entries }
};
mpeg4_stbl_atom_data::stts(inner)
},

(99u8, 116u8, 116u8, 115u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let sample_entries = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let sample_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let sample_offset = ((|| PResult::Ok((Decoder21(_input))?))())?;
mpeg4_stbl_atom_data_ctts_sample_entries { sample_count, sample_offset }
});
}
accum
}))())?;
mpeg4_stbl_atom_data_ctts { version, flags, entry_count, sample_entries }
};
mpeg4_stbl_atom_data::ctts(inner)
},

(115u8, 116u8, 115u8, 115u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let sample_number = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder21(_input))?);
}
accum
}))())?;
mpeg4_stbl_atom_data_stss { version, flags, entry_count, sample_number }
};
mpeg4_stbl_atom_data::stss(inner)
},

(115u8, 116u8, 115u8, 99u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let chunk_entries = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let first_chunk = ((|| PResult::Ok((Decoder21(_input))?))())?;
let samples_per_chunk = ((|| PResult::Ok((Decoder21(_input))?))())?;
let sample_description_index = ((|| PResult::Ok((Decoder21(_input))?))())?;
mpeg4_stbl_atom_data_stsc_chunk_entries { first_chunk, samples_per_chunk, sample_description_index }
});
}
accum
}))())?;
mpeg4_stbl_atom_data_stsc { version, flags, entry_count, chunk_entries }
};
mpeg4_stbl_atom_data::stsc(inner)
},

(115u8, 116u8, 115u8, 122u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let sample_size = ((|| PResult::Ok((Decoder21(_input))?))())?;
let sample_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let entry_size = ((|| PResult::Ok(if sample_size == 0u32 {
let mut accum = Vec::new();
for _ in 0..sample_count {
accum.push((Decoder21(_input))?);
}
Some(accum)
} else {
None
}))())?;
mpeg4_stbl_atom_data_stsz { version, flags, sample_size, sample_count, entry_size }
};
mpeg4_stbl_atom_data::stsz(inner)
},

(115u8, 116u8, 99u8, 111u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let chunk_offset = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder21(_input))?);
}
accum
}))())?;
mpeg4_stbl_atom_data_stco { version, flags, entry_count, chunk_offset }
};
mpeg4_stbl_atom_data::stco(inner)
},

(99u8, 111u8, 54u8, 52u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let entry_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let chunk_offset = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder61(_input))?);
}
accum
}))())?;
mpeg4_stbl_atom_data_co64 { version, flags, entry_count, chunk_offset }
};
mpeg4_stbl_atom_data::co64(inner)
},

(115u8, 103u8, 112u8, 100u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let grouping_type = ((|| PResult::Ok((Decoder21(_input))?))())?;
let default_length = ((|| PResult::Ok((Decoder21(_input))?))())?;
let entry_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let sample_groups = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let description_length = ((|| PResult::Ok(match default_length == 0u32 {
true => {
(Decoder21(_input))?
},

false => {
default_length.clone()
}
}))())?;
let sample_group_entry = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..description_length {
accum.push((Decoder25(_input))?);
}
accum
}))())?;
mpeg4_stbl_atom_data_sgpd_sample_groups { description_length, sample_group_entry }
});
}
accum
}))())?;
mpeg4_stbl_atom_data_sgpd { version, flags, grouping_type, default_length, entry_count, sample_groups }
};
mpeg4_stbl_atom_data::sgpd(inner)
},

(115u8, 98u8, 103u8, 112u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let grouping_type = ((|| PResult::Ok((Decoder21(_input))?))())?;
let grouping_type_parameter = ((|| PResult::Ok(if version == 1u8 {
Some((Decoder21(_input))?)
} else {
None
}))())?;
let entry_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let sample_groups = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let sample_count = ((|| PResult::Ok((Decoder21(_input))?))())?;
let group_description_index = ((|| PResult::Ok((Decoder21(_input))?))())?;
mpeg4_stbl_atom_data_sbgp_sample_groups { sample_count, group_description_index }
});
}
accum
}))())?;
mpeg4_stbl_atom_data_sbgp { version, flags, grouping_type, grouping_type_parameter, entry_count, sample_groups }
};
mpeg4_stbl_atom_data::sbgp(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_stbl_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_stbl_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_iinf_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_iinf_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(105u8, 110u8, 102u8, 101u8) => {
let inner = {
let version = ((|| PResult::Ok((Decoder25(_input))?))())?;
let flags = ((|| PResult::Ok({
let field0 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field1 = ((|| PResult::Ok((Decoder25(_input))?))())?;
let field2 = ((|| PResult::Ok((Decoder25(_input))?))())?;
(field0, field1, field2)
}))())?;
let fields = ((|| PResult::Ok(match version < 2u8 {
true => {
let inner = {
let item_ID = ((|| PResult::Ok((Decoder24(_input))?))())?;
let item_protection_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
let item_name = ((|| PResult::Ok((Decoder182(_input))?))())?;
let content_type = ((|| PResult::Ok((Decoder183(_input))?))())?;
let content_encoding = ((|| PResult::Ok((Decoder184(_input))?))())?;
mpeg4_iinf_atom_data_infe_fields_yes { item_ID, item_protection_index, item_name, content_type, content_encoding }
};
mpeg4_iinf_atom_data_infe_fields::yes(inner)
},

false => {
let inner = {
let item_ID = ((|| PResult::Ok(match version == 2u8 {
true => {
let inner = (Decoder24(_input))?;
((|x: u16| PResult::Ok(x as u32))(inner))?
},

false => {
(Decoder21(_input))?
}
}))())?;
let item_protection_index = ((|| PResult::Ok((Decoder24(_input))?))())?;
let item_type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let item_name = ((|| PResult::Ok((Decoder185(_input))?))())?;
let extra_fields = ((|| PResult::Ok(match item_type {
(109u8, 105u8, 109u8, 101u8) => {
let inner = {
let content_type = ((|| PResult::Ok((Decoder186(_input))?))())?;
mpeg4_iinf_atom_data_infe_fields_no_extra_fields_mime { content_type }
};
mpeg4_iinf_atom_data_infe_fields_no_extra_fields::mime(inner)
},

(117u8, 114u8, 105u8, 32u8) => {
let inner = {
let item_uri_type = ((|| PResult::Ok((Decoder186(_input))?))())?;
mpeg4_iinf_atom_data_infe_fields_no_extra_fields_uri { item_uri_type }
};
mpeg4_iinf_atom_data_infe_fields_no_extra_fields::uri(inner)
},

_ => {
mpeg4_iinf_atom_data_infe_fields_no_extra_fields::unknown
}
}))())?;
mpeg4_iinf_atom_data_infe_fields_no { item_ID, item_protection_index, item_type, item_name, extra_fields }
};
mpeg4_iinf_atom_data_infe_fields::no(inner)
}
}))())?;
mpeg4_iinf_atom_data_infe { version, flags, fields }
};
mpeg4_iinf_atom_data::infe(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_iinf_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_iinf_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_ilst_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_ilst_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(169u8, 116u8, 111u8, 111u8) => {
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
let next_elem = (Decoder_mpeg4_tool_atom(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_ilst_atom_data::tool(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_ilst_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_ilst_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_tool_atom<'input>(_input: &mut Parser<'input>) -> Result<mpeg4_tool_atom, ParseError> {
let size_field = ((|| PResult::Ok((Decoder21(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder168(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0u32 => {
0u64
},

1u32 => {
let inner = (Decoder61(_input))?;
((|x: u64| PResult::Ok(try_sub!(x, 16u64)))(inner))?
},

_ => {
(try_sub!(size_field, 8u32)) as u64
}
}))())?;
let data = ((|| PResult::Ok({
let sz = size as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok(match r#type {
(100u8, 97u8, 116u8, 97u8) => {
let inner = {
let type_indicator = ((|| PResult::Ok((Decoder21(_input))?))())?;
let locale_indicator = ((|| PResult::Ok((Decoder21(_input))?))())?;
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
let next_elem = (Decoder112(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
mpeg4_tool_atom_data_data { type_indicator, locale_indicator, value }
};
mpeg4_tool_atom_data::data(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_tool_atom_data::unknown(inner)
}
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(mpeg4_tool_atom { size_field, r#type, size, data })
}

fn Decoder182<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder183<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder184<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder185<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder186<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_jpeg_eoi<'input>(_input: &mut Parser<'input>) -> Result<jpeg_eoi, ParseError> {
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
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder_jpeg_frame<'input>(_input: &mut Parser<'input>) -> Result<jpeg_frame, ParseError> {
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
let inner = (Decoder_jpeg_app0(_input))?;
jpeg_frame_initial_segment::app0(inner)
},

1 => {
let inner = (Decoder_jpeg_app1(_input))?;
jpeg_frame_initial_segment::app1(inner)
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
let next_elem = (Decoder_jpeg_table_or_misc(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let header = ((|| PResult::Ok((Decoder_jpeg_frame_header(_input))?))())?;
let scan = ((|| PResult::Ok((Decoder_jpeg_scan(_input))?))())?;
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
let inner = (Decoder_jpeg_dnl(_input))?;
jpeg_frame_dnl::some(inner)
},

1 => {
jpeg_frame_dnl::none
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
let next_elem = (Decoder196(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(jpeg_frame { initial_segment, segments, header, scan, dnl, scans })
}

fn Decoder189<'input>(_input: &mut Parser<'input>) -> Result<jpeg_eoi, ParseError> {
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
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder_jpeg_app0<'input>(_input: &mut Parser<'input>) -> Result<jpeg_app0, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_app0_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_app0 { marker, length, data })
}

fn Decoder_jpeg_app1<'input>(_input: &mut Parser<'input>) -> Result<jpeg_app1, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_app1_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_app1 { marker, length, data })
}

fn Decoder_jpeg_table_or_misc<'input>(_input: &mut Parser<'input>) -> Result<jpeg_table_or_misc, ParseError> {
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
let inner = (Decoder_jpeg_dqt(_input))?;
jpeg_table_or_misc::dqt(inner)
},

1 => {
let inner = (Decoder_jpeg_dht(_input))?;
jpeg_table_or_misc::dht(inner)
},

2 => {
let inner = (Decoder_jpeg_dac(_input))?;
jpeg_table_or_misc::dac(inner)
},

3 => {
let inner = (Decoder_jpeg_dri(_input))?;
jpeg_table_or_misc::dri(inner)
},

4 => {
let inner = (Decoder_jpeg_app0(_input))?;
jpeg_table_or_misc::app0(inner)
},

5 => {
let inner = (Decoder_jpeg_app1(_input))?;
jpeg_table_or_misc::app1(inner)
},

6 => {
let inner = (Decoder_jpeg_com(_input))?;
jpeg_table_or_misc::app2(inner)
},

7 => {
let inner = (Decoder232(_input))?;
jpeg_table_or_misc::app3(inner)
},

8 => {
let inner = (Decoder233(_input))?;
jpeg_table_or_misc::app4(inner)
},

9 => {
let inner = (Decoder234(_input))?;
jpeg_table_or_misc::app5(inner)
},

10 => {
let inner = (Decoder235(_input))?;
jpeg_table_or_misc::app6(inner)
},

11 => {
let inner = (Decoder236(_input))?;
jpeg_table_or_misc::app7(inner)
},

12 => {
let inner = (Decoder237(_input))?;
jpeg_table_or_misc::app8(inner)
},

13 => {
let inner = (Decoder238(_input))?;
jpeg_table_or_misc::app9(inner)
},

14 => {
let inner = (Decoder239(_input))?;
jpeg_table_or_misc::app10(inner)
},

15 => {
let inner = (Decoder240(_input))?;
jpeg_table_or_misc::app11(inner)
},

16 => {
let inner = (Decoder241(_input))?;
jpeg_table_or_misc::app12(inner)
},

17 => {
let inner = (Decoder242(_input))?;
jpeg_table_or_misc::app13(inner)
},

18 => {
let inner = (Decoder243(_input))?;
jpeg_table_or_misc::app14(inner)
},

19 => {
let inner = (Decoder244(_input))?;
jpeg_table_or_misc::app15(inner)
},

20 => {
let inner = (Decoder245(_input))?;
jpeg_table_or_misc::com(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(5858366816005674364u64));
}
})
}

fn Decoder_jpeg_frame_header<'input>(_input: &mut Parser<'input>) -> Result<jpeg_frame_header, ParseError> {
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
let inner = (Decoder_jpeg_sof15(_input))?;
jpeg_frame_header::sof0(inner)
},

1 => {
let inner = (Decoder213(_input))?;
jpeg_frame_header::sof1(inner)
},

2 => {
let inner = (Decoder214(_input))?;
jpeg_frame_header::sof2(inner)
},

3 => {
let inner = (Decoder215(_input))?;
jpeg_frame_header::sof3(inner)
},

4 => {
let inner = (Decoder216(_input))?;
jpeg_frame_header::sof5(inner)
},

5 => {
let inner = (Decoder217(_input))?;
jpeg_frame_header::sof6(inner)
},

6 => {
let inner = (Decoder218(_input))?;
jpeg_frame_header::sof7(inner)
},

7 => {
let inner = (Decoder219(_input))?;
jpeg_frame_header::sof9(inner)
},

8 => {
let inner = (Decoder220(_input))?;
jpeg_frame_header::sof10(inner)
},

9 => {
let inner = (Decoder221(_input))?;
jpeg_frame_header::sof11(inner)
},

10 => {
let inner = (Decoder222(_input))?;
jpeg_frame_header::sof13(inner)
},

11 => {
let inner = (Decoder223(_input))?;
jpeg_frame_header::sof14(inner)
},

12 => {
let inner = (Decoder224(_input))?;
jpeg_frame_header::sof15(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(13734934310846663046u64));
}
})
}

fn Decoder_jpeg_scan<'input>(_input: &mut Parser<'input>) -> Result<jpeg_scan, ParseError> {
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
let next_elem = (Decoder_jpeg_table_or_misc(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let sos = ((|| PResult::Ok((Decoder_jpeg_sos(_input))?))())?;
let data = ((|| PResult::Ok((Decoder211(_input))?))())?;
PResult::Ok(jpeg_scan { segments, sos, data })
}

fn Decoder_jpeg_dnl<'input>(_input: &mut Parser<'input>) -> Result<jpeg_dnl, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_dnl_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_dnl { marker, length, data })
}

fn Decoder196<'input>(_input: &mut Parser<'input>) -> Result<jpeg_scan, ParseError> {
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
let next_elem = (Decoder_jpeg_table_or_misc(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let sos = ((|| PResult::Ok((Decoder_jpeg_sos(_input))?))())?;
let data = ((|| PResult::Ok((Decoder_jpeg_scan_data(_input))?))())?;
PResult::Ok(jpeg_scan { segments, sos, data })
}

fn Decoder_jpeg_sos<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sos, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sos_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sos { marker, length, data })
}

fn Decoder_jpeg_scan_data<'input>(_input: &mut Parser<'input>) -> Result<jpeg_scan_data, ParseError> {
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
let inner = (Decoder199(_input))?;
jpeg_scan_data_scan_data::mcu(inner)
},

1 => {
let inner = (Decoder200(_input))?;
jpeg_scan_data_scan_data::rst0(inner)
},

2 => {
let inner = (Decoder201(_input))?;
jpeg_scan_data_scan_data::rst1(inner)
},

3 => {
let inner = (Decoder202(_input))?;
jpeg_scan_data_scan_data::rst2(inner)
},

4 => {
let inner = (Decoder203(_input))?;
jpeg_scan_data_scan_data::rst3(inner)
},

5 => {
let inner = (Decoder204(_input))?;
jpeg_scan_data_scan_data::rst4(inner)
},

6 => {
let inner = (Decoder205(_input))?;
jpeg_scan_data_scan_data::rst5(inner)
},

7 => {
let inner = (Decoder206(_input))?;
jpeg_scan_data_scan_data::rst6(inner)
},

8 => {
let inner = (Decoder207(_input))?;
jpeg_scan_data_scan_data::rst7(inner)
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
let scan_data_stream = ((|| PResult::Ok((try_flat_map_vec(scan_data.iter().cloned(), |x: jpeg_scan_data_scan_data| PResult::Ok(match x {
jpeg_scan_data_scan_data::mcu(v) => {
[v.clone()].to_vec()
},

jpeg_scan_data_scan_data::rst0(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst1(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst2(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst3(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst4(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst5(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst6(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst7(..) => {
[].to_vec()
}
})))?))())?;
PResult::Ok(jpeg_scan_data { scan_data, scan_data_stream })
}

fn Decoder199<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
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

fn Decoder200<'input>(_input: &mut Parser<'input>) -> Result<jpeg_eoi, ParseError> {
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
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder201<'input>(_input: &mut Parser<'input>) -> Result<jpeg_eoi, ParseError> {
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
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder202<'input>(_input: &mut Parser<'input>) -> Result<jpeg_eoi, ParseError> {
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
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder203<'input>(_input: &mut Parser<'input>) -> Result<jpeg_eoi, ParseError> {
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
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder204<'input>(_input: &mut Parser<'input>) -> Result<jpeg_eoi, ParseError> {
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
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder205<'input>(_input: &mut Parser<'input>) -> Result<jpeg_eoi, ParseError> {
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
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder206<'input>(_input: &mut Parser<'input>) -> Result<jpeg_eoi, ParseError> {
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
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder207<'input>(_input: &mut Parser<'input>) -> Result<jpeg_eoi, ParseError> {
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
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder_jpeg_sos_data<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sos_data, ParseError> {
let num_image_components = ((|| PResult::Ok({
let inner = (Decoder25(_input))?;
if ((|x: u8| PResult::Ok((x >= 1u8) && (x <= 4u8)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let image_components = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_image_components {
accum.push((Decoder_jpeg_sos_image_component(_input))?);
}
accum
}))())?;
let start_spectral_selection = ((|| PResult::Ok({
let inner = (Decoder25(_input))?;
if ((|x: u8| PResult::Ok(x <= 63u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let end_spectral_selection = ((|| PResult::Ok({
let inner = (Decoder25(_input))?;
if ((|x: u8| PResult::Ok(x <= 63u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let approximation_bit_position = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(jpeg_sos_data_approximation_bit_position { high: packedbits >> 4u8 & 15u8, low: packedbits >> 0u8 & 15u8 }))(inner))?
}))())?;
PResult::Ok(jpeg_sos_data { num_image_components, image_components, start_spectral_selection, end_spectral_selection, approximation_bit_position })
}

fn Decoder_jpeg_sos_image_component<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sos_image_component, ParseError> {
let component_selector = ((|| PResult::Ok((Decoder25(_input))?))())?;
let entropy_coding_table_ids = ((|| PResult::Ok({
let inner = {
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(jpeg_sos_image_component_entropy_coding_table_ids { dc_entropy_coding_table_id: packedbits >> 4u8 & 15u8, ac_entropy_coding_table_id: packedbits >> 0u8 & 15u8 }))(inner))?
};
if ((|entropy_coding_table_ids: jpeg_sos_image_component_entropy_coding_table_ids| PResult::Ok((entropy_coding_table_ids.dc_entropy_coding_table_id.clone() <= 3u8) && (entropy_coding_table_ids.ac_entropy_coding_table_id.clone() <= 3u8)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
PResult::Ok(jpeg_sos_image_component { component_selector, entropy_coding_table_ids })
}

fn Decoder_jpeg_dnl_data<'input>(_input: &mut Parser<'input>) -> Result<jpeg_dnl_data, ParseError> {
let num_lines = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x != 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
PResult::Ok(jpeg_dnl_data { num_lines })
}

fn Decoder211<'input>(_input: &mut Parser<'input>) -> Result<jpeg_scan_data, ParseError> {
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
let inner = (Decoder199(_input))?;
jpeg_scan_data_scan_data::mcu(inner)
},

1 => {
let inner = (Decoder200(_input))?;
jpeg_scan_data_scan_data::rst0(inner)
},

2 => {
let inner = (Decoder201(_input))?;
jpeg_scan_data_scan_data::rst1(inner)
},

3 => {
let inner = (Decoder202(_input))?;
jpeg_scan_data_scan_data::rst2(inner)
},

4 => {
let inner = (Decoder203(_input))?;
jpeg_scan_data_scan_data::rst3(inner)
},

5 => {
let inner = (Decoder204(_input))?;
jpeg_scan_data_scan_data::rst4(inner)
},

6 => {
let inner = (Decoder205(_input))?;
jpeg_scan_data_scan_data::rst5(inner)
},

7 => {
let inner = (Decoder206(_input))?;
jpeg_scan_data_scan_data::rst6(inner)
},

8 => {
let inner = (Decoder207(_input))?;
jpeg_scan_data_scan_data::rst7(inner)
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
let scan_data_stream = ((|| PResult::Ok((try_flat_map_vec(scan_data.iter().cloned(), |x: jpeg_scan_data_scan_data| PResult::Ok(match x {
jpeg_scan_data_scan_data::mcu(v) => {
[v.clone()].to_vec()
},

jpeg_scan_data_scan_data::rst0(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst1(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst2(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst3(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst4(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst5(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst6(..) => {
[].to_vec()
},

jpeg_scan_data_scan_data::rst7(..) => {
[].to_vec()
}
})))?))())?;
PResult::Ok(jpeg_scan_data { scan_data, scan_data_stream })
}

fn Decoder_jpeg_sof15<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder213<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder214<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder215<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder216<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder217<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder218<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder219<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder220<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder221<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder222<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder223<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder224<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof15, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_sof_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder_jpeg_sof_data<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof_data, ParseError> {
let sample_precision = ((|| PResult::Ok({
let inner = (Decoder25(_input))?;
if ((|x: u8| PResult::Ok((x >= 2u8) && (x <= 16u8)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let num_lines = ((|| PResult::Ok((Decoder24(_input))?))())?;
let num_samples_per_line = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x != 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let num_image_components = ((|| PResult::Ok({
let inner = (Decoder25(_input))?;
if ((|x: u8| PResult::Ok(x != 0u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let image_components = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..num_image_components {
accum.push((Decoder_jpeg_sof_image_component(_input))?);
}
accum
}))())?;
PResult::Ok(jpeg_sof_data { sample_precision, num_lines, num_samples_per_line, num_image_components, image_components })
}

fn Decoder_jpeg_sof_image_component<'input>(_input: &mut Parser<'input>) -> Result<jpeg_sof_image_component, ParseError> {
let id = ((|| PResult::Ok((Decoder25(_input))?))())?;
let sampling_factor = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(jpeg_sof_image_component_sampling_factor { horizontal: packedbits >> 4u8 & 15u8, vertical: packedbits >> 0u8 & 15u8 }))(inner))?
}))())?;
let quantization_table_id = ((|| PResult::Ok({
let inner = (Decoder25(_input))?;
if ((|x: u8| PResult::Ok(x <= 3u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
PResult::Ok(jpeg_sof_image_component { id, sampling_factor, quantization_table_id })
}

fn Decoder_jpeg_dqt<'input>(_input: &mut Parser<'input>) -> Result<jpeg_dqt, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder_jpeg_dqt_data(_input))?;
accum.push(next_elem);
}
}
accum
}))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_dqt { marker, length, data })
}

fn Decoder_jpeg_dht<'input>(_input: &mut Parser<'input>) -> Result<jpeg_dht, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_dht_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_dht { marker, length, data })
}

fn Decoder_jpeg_dac<'input>(_input: &mut Parser<'input>) -> Result<jpeg_dac, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_dac_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_dac { marker, length, data })
}

fn Decoder_jpeg_dri<'input>(_input: &mut Parser<'input>) -> Result<jpeg_dri, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
_input.start_slice(sz)?;
let ret = ((|| PResult::Ok((Decoder_jpeg_dri_data(_input))?))())?;
_input.end_slice()?;
ret
}))())?;
PResult::Ok(jpeg_dri { marker, length, data })
}

fn Decoder_jpeg_com<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder232<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder233<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder234<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder235<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder236<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder237<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder238<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder239<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder240<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder241<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder242<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder243<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder244<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder245<'input>(_input: &mut Parser<'input>) -> Result<jpeg_com, ParseError> {
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
jpeg_eoi { ff, marker }
}))())?;
let length = ((|| PResult::Ok((Decoder24(_input))?))())?;
let data = ((|| PResult::Ok({
let sz = (try_sub!(length, 2u16)) as usize<>;
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
let next_elem = (Decoder25(_input))?;
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
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder_jpeg_dri_data<'input>(_input: &mut Parser<'input>) -> Result<jpeg_dri_data, ParseError> {
let restart_interval = ((|| PResult::Ok((Decoder24(_input))?))())?;
PResult::Ok(jpeg_dri_data { restart_interval })
}

fn Decoder_jpeg_dac_data<'input>(_input: &mut Parser<'input>) -> Result<jpeg_dac_data, ParseError> {
let class_table_id = ((|| PResult::Ok({
let inner = {
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(jpeg_dac_data_class_table_id { class: packedbits >> 4u8 & 15u8, table_id: packedbits >> 0u8 & 15u8 }))(inner))?
};
if ((|class_table_id: jpeg_dac_data_class_table_id| PResult::Ok((class_table_id.class.clone() < 2u8) && (class_table_id.table_id.clone() < 4u8)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let value = ((|| PResult::Ok((Decoder25(_input))?))())?;
PResult::Ok(jpeg_dac_data { class_table_id, value })
}

fn Decoder_jpeg_dht_data<'input>(_input: &mut Parser<'input>) -> Result<jpeg_dht_data, ParseError> {
let class_table_id = ((|| PResult::Ok({
let inner = {
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(jpeg_dac_data_class_table_id { class: packedbits >> 4u8 & 15u8, table_id: packedbits >> 0u8 & 15u8 }))(inner))?
};
if ((|class_table_id: jpeg_dac_data_class_table_id| PResult::Ok((class_table_id.class.clone() < 2u8) && (class_table_id.table_id.clone() < 4u8)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let num_codes = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..16u8 {
accum.push((Decoder25(_input))?);
}
accum
}))())?;
let values = ((|| PResult::Ok({
let mut accum = Vec::new();
for n in num_codes.clone() {
accum.push({
let mut accum = Vec::new();
for _ in 0..n {
accum.push((Decoder25(_input))?);
}
accum
});
}
accum
}))())?;
PResult::Ok(jpeg_dht_data { class_table_id, num_codes, values })
}

fn Decoder_jpeg_dqt_data<'input>(_input: &mut Parser<'input>) -> Result<jpeg_dqt_data, ParseError> {
let precision_table_id = ((|| PResult::Ok({
let inner = {
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(jpeg_dqt_data_precision_table_id { precision: packedbits >> 4u8 & 15u8, table_id: packedbits >> 0u8 & 15u8 }))(inner))?
};
if ((|precision_table_id: jpeg_dqt_data_precision_table_id| PResult::Ok((precision_table_id.precision.clone() <= 1u8) && (precision_table_id.table_id.clone() <= 3u8)))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let elements = ((|| PResult::Ok(match precision_table_id.precision.clone() {
0u8 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..64u32 {
accum.push((Decoder25(_input))?);
}
accum
};
jpeg_dqt_data_elements::Bytes(inner)
},

1u8 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..64u32 {
accum.push((Decoder24(_input))?);
}
accum
};
jpeg_dqt_data_elements::Shorts(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
PResult::Ok(jpeg_dqt_data { precision_table_id, elements })
}

fn Decoder_jpeg_app1_data<'input>(_input: &mut Parser<'input>) -> Result<jpeg_app1_data, ParseError> {
let identifier = ((|| PResult::Ok((Decoder251(_input))?))())?;
let data = ((|| PResult::Ok(match identifier.string.as_slice() {
[69u8, 120u8, 105u8, 102u8] => {
let inner = (Decoder_jpeg_app1_exif(_input))?;
jpeg_app1_data_data::exif(inner)
},

[104u8, 116u8, 116u8, 112u8, 58u8, 47u8, 47u8, 110u8, 115u8, 46u8, 97u8, 100u8, 111u8, 98u8, 101u8, 46u8, 99u8, 111u8, 109u8, 47u8, 120u8, 97u8, 112u8, 47u8, 49u8, 46u8, 48u8, 47u8] => {
let inner = (Decoder_jpeg_app1_xmp(_input))?;
jpeg_app1_data_data::xmp(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
jpeg_app1_data_data::other(inner)
}
}))())?;
PResult::Ok(jpeg_app1_data { identifier, data })
}

fn Decoder251<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_jpeg_app1_exif<'input>(_input: &mut Parser<'input>) -> Result<jpeg_app1_exif, ParseError> {
let padding = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
}
}))())?;
let exif = ((|| PResult::Ok((Decoder_tiff_main(_input))?))())?;
PResult::Ok(jpeg_app1_exif { padding, exif })
}

fn Decoder_jpeg_app1_xmp<'input>(_input: &mut Parser<'input>) -> Result<jpeg_app1_xmp, ParseError> {
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
PResult::Ok(jpeg_app1_xmp { xmp })
}

fn Decoder_jpeg_app0_data<'input>(_input: &mut Parser<'input>) -> Result<jpeg_app0_data, ParseError> {
let identifier = ((|| PResult::Ok((Decoder255(_input))?))())?;
let data = ((|| PResult::Ok(match identifier.string.as_slice() {
[74u8, 70u8, 73u8, 70u8] => {
let inner = (Decoder_jpeg_app0_jfif(_input))?;
jpeg_app0_data_data::jfif(inner)
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
let next_elem = (Decoder25(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
jpeg_app0_data_data::other(inner)
}
}))())?;
PResult::Ok(jpeg_app0_data { identifier, data })
}

fn Decoder255<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_jpeg_app0_jfif<'input>(_input: &mut Parser<'input>) -> Result<jpeg_app0_jfif, ParseError> {
let version_major = ((|| PResult::Ok((Decoder25(_input))?))())?;
let version_minor = ((|| PResult::Ok((Decoder25(_input))?))())?;
let density_units = ((|| PResult::Ok({
let inner = (Decoder25(_input))?;
if ((|x: u8| PResult::Ok(x <= 2u8))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let density_x = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x != 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let density_y = ((|| PResult::Ok({
let inner = (Decoder24(_input))?;
if ((|x: u16| PResult::Ok(x != 0u16))(inner.clone()))? {
inner
} else {
return Err(ParseError::FalsifiedWhere);
}
}))())?;
let thumbnail_width = ((|| PResult::Ok((Decoder25(_input))?))())?;
let thumbnail_height = ((|| PResult::Ok((Decoder25(_input))?))())?;
let thumbnail_pixels = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..thumbnail_height {
accum.push({
let mut accum = Vec::new();
for _ in 0..thumbnail_width {
accum.push((Decoder_png_plte(_input))?);
}
accum
});
}
accum
}))())?;
PResult::Ok(jpeg_app0_jfif { version_major, version_minor, density_units, density_x, density_y, thumbnail_width, thumbnail_height, thumbnail_pixels })
}

fn Decoder_png_plte<'input>(_input: &mut Parser<'input>) -> Result<png_plte, ParseError> {
let r = ((|| PResult::Ok((Decoder25(_input))?))())?;
let g = ((|| PResult::Ok((Decoder25(_input))?))())?;
let b = ((|| PResult::Ok((Decoder25(_input))?))())?;
PResult::Ok(png_plte { r, g, b })
}

fn Decoder_gzip_header<'input>(_input: &mut Parser<'input>) -> Result<gzip_header, ParseError> {
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
let method = ((|| PResult::Ok((Decoder25(_input))?))())?;
let file_flags = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|flagbits: u8| PResult::Ok(gzip_header_file_flags { fcomment: flagbits >> 4u8 & 1u8 != 0u8, fname: flagbits >> 3u8 & 1u8 != 0u8, fextra: flagbits >> 2u8 & 1u8 != 0u8, fhcrc: flagbits >> 1u8 & 1u8 != 0u8, ftext: flagbits >> 0u8 & 1u8 != 0u8 }))(inner))?
}))())?;
let timestamp = ((|| PResult::Ok((Decoder89(_input))?))())?;
let compression_flags = ((|| PResult::Ok((Decoder25(_input))?))())?;
let os_id = ((|| PResult::Ok((Decoder25(_input))?))())?;
PResult::Ok(gzip_header { magic, method, file_flags, timestamp, compression_flags, os_id })
}

fn Decoder_gzip_fextra<'input>(_input: &mut Parser<'input>) -> Result<gzip_fextra, ParseError> {
let xlen = ((|| PResult::Ok((Decoder101(_input))?))())?;
let subfields = ((|| PResult::Ok({
let sz = xlen as usize<>;
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
let next_elem = (Decoder_gzip_fextra_subfield(_input))?;
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
PResult::Ok(gzip_fextra { xlen, subfields })
}

fn Decoder260<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
PResult::Ok((Decoder265(_input))?)
}

fn Decoder_gzip_fcomment<'input>(_input: &mut Parser<'input>) -> Result<gzip_fcomment, ParseError> {
let comment = ((|| PResult::Ok((Decoder264(_input))?))())?;
PResult::Ok(gzip_fcomment { comment })
}

fn Decoder_gzip_fhcrc<'input>(_input: &mut Parser<'input>) -> Result<gzip_fhcrc, ParseError> {
let crc = ((|| PResult::Ok((Decoder101(_input))?))())?;
PResult::Ok(gzip_fhcrc { crc })
}

fn Decoder_gzip_footer<'input>(_input: &mut Parser<'input>) -> Result<gzip_footer, ParseError> {
let crc = ((|| PResult::Ok((Decoder89(_input))?))())?;
let length = ((|| PResult::Ok((Decoder89(_input))?))())?;
PResult::Ok(gzip_footer { crc, length })
}

fn Decoder264<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder265<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_gzip_fextra_subfield<'input>(_input: &mut Parser<'input>) -> Result<gzip_fextra_subfield, ParseError> {
let si1 = ((|| PResult::Ok((Decoder112(_input))?))())?;
let si2 = ((|| PResult::Ok((Decoder112(_input))?))())?;
let len = ((|| PResult::Ok((Decoder101(_input))?))())?;
let data = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..len {
accum.push((Decoder25(_input))?);
}
accum
}))())?;
PResult::Ok(gzip_fextra_subfield { si1, si2, len, data })
}

fn Decoder267<'input>(_input: &mut Parser<'input>) -> Result<Vec<gzip_main>, ParseError> {
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
let header = ((|| PResult::Ok((Decoder_gzip_header(_input))?))())?;
let fextra = ((|| PResult::Ok(if header.file_flags.fextra.clone() {
Some((Decoder_gzip_fextra(_input))?)
} else {
None
}))())?;
let fname = ((|| PResult::Ok(if header.file_flags.fname.clone() {
Some((Decoder268(_input))?)
} else {
None
}))())?;
let fcomment = ((|| PResult::Ok(if header.file_flags.fcomment.clone() {
Some((Decoder269(_input))?)
} else {
None
}))())?;
let fhcrc = ((|| PResult::Ok(if header.file_flags.fhcrc.clone() {
Some((Decoder_gzip_fhcrc(_input))?)
} else {
None
}))())?;
let data = ((|| PResult::Ok({
_input.enter_bits_mode()?;
let ret = ((|| PResult::Ok((Decoder_deflate_main(_input))?))())?;
let _bits_read = _input.escape_bits_mode()?;
ret
}))())?;
let footer = ((|| PResult::Ok((Decoder_gzip_footer(_input))?))())?;
gzip_main { header, fextra, fname, fcomment, fhcrc, data, footer }
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder268<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
PResult::Ok((Decoder271(_input))?)
}

fn Decoder269<'input>(_input: &mut Parser<'input>) -> Result<gzip_fcomment, ParseError> {
let comment = ((|| PResult::Ok((Decoder270(_input))?))())?;
PResult::Ok(gzip_fcomment { comment })
}

fn Decoder270<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder271<'input>(_input: &mut Parser<'input>) -> Result<base_asciiz_string, ParseError> {
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
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_gif_header<'input>(_input: &mut Parser<'input>) -> Result<gif_header, ParseError> {
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
accum.push((Decoder112(_input))?);
}
accum
}))())?;
PResult::Ok(gif_header { signature, version })
}

fn Decoder_gif_logical_screen<'input>(_input: &mut Parser<'input>) -> Result<gif_logical_screen, ParseError> {
let descriptor = ((|| PResult::Ok((Decoder_gif_logical_screen_descriptor(_input))?))())?;
let global_color_table = ((|| PResult::Ok(if descriptor.flags.table_flag.clone() != 0u8 {
let mut accum = Vec::new();
for _ in 0..2u16 << ((descriptor.flags.table_size.clone()) as u16) {
accum.push((Decoder287(_input))?);
}
Some(accum)
} else {
None
}))())?;
PResult::Ok(gif_logical_screen { descriptor, global_color_table })
}

fn Decoder_gif_block<'input>(_input: &mut Parser<'input>) -> Result<gif_block, ParseError> {
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
let inner = (Decoder_gif_graphic_block(_input))?;
gif_block::graphic_block(inner)
},

1 => {
let inner = (Decoder_gif_special_purpose_block(_input))?;
gif_block::special_purpose_block(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(14056621334159770744u64));
}
})
}

fn Decoder_gif_trailer<'input>(_input: &mut Parser<'input>) -> Result<gif_trailer, ParseError> {
let separator = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 59 {
b
} else {
return Err(ParseError::ExcludedBranch(15783818897979407630u64));
}
}))())?;
PResult::Ok(gif_trailer { separator })
}

fn Decoder_gif_graphic_block<'input>(_input: &mut Parser<'input>) -> Result<gif_graphic_block, ParseError> {
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
let inner = (Decoder_gif_graphic_control_extension(_input))?;
gif_graphic_block_graphic_control_extension::some(inner)
},

1 => {
gif_graphic_block_graphic_control_extension::none
},

_ => {
return Err(ParseError::ExcludedBranch(15496895076277599409u64));
}
}
}))())?;
let graphic_rendering_block = ((|| PResult::Ok((Decoder_gif_graphic_rendering_block(_input))?))())?;
PResult::Ok(gif_graphic_block { graphic_control_extension, graphic_rendering_block })
}

fn Decoder_gif_special_purpose_block<'input>(_input: &mut Parser<'input>) -> Result<gif_special_purpose_block, ParseError> {
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
let inner = (Decoder_gif_application_extension(_input))?;
gif_special_purpose_block::application_extension(inner)
},

1 => {
let inner = (Decoder_gif_comment_extension(_input))?;
gif_special_purpose_block::comment_extension(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(7565262198115782210u64));
}
})
}

fn Decoder_gif_application_extension<'input>(_input: &mut Parser<'input>) -> Result<gif_application_extension, ParseError> {
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
accum.push((Decoder25(_input))?);
}
accum
}))())?;
let authentication_code = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..3u8 {
accum.push((Decoder25(_input))?);
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
let next_elem = (Decoder_gif_subblock(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let terminator = ((|| PResult::Ok((Decoder281(_input))?))())?;
PResult::Ok(gif_application_extension { separator, label, block_size, identifier, authentication_code, application_data, terminator })
}

fn Decoder_gif_comment_extension<'input>(_input: &mut Parser<'input>) -> Result<gif_comment_extension, ParseError> {
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
let next_elem = (Decoder_gif_subblock(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let terminator = ((|| PResult::Ok((Decoder281(_input))?))())?;
PResult::Ok(gif_comment_extension { separator, label, comment_data, terminator })
}

fn Decoder_gif_subblock<'input>(_input: &mut Parser<'input>) -> Result<gif_subblock, ParseError> {
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
accum.push((Decoder25(_input))?);
}
accum
}))())?;
PResult::Ok(gif_subblock { len_bytes, data })
}

fn Decoder281<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10396965092922267801u64));
})
}

fn Decoder_gif_graphic_control_extension<'input>(_input: &mut Parser<'input>) -> Result<gif_graphic_control_extension, ParseError> {
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
let flags = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(gif_graphic_control_extension_flags { reserved: packedbits >> 5u8 & 7u8, disposal_method: packedbits >> 2u8 & 7u8, user_input_flag: packedbits >> 1u8 & 1u8, transparent_color_flag: packedbits >> 0u8 & 1u8 }))(inner))?
}))())?;
let delay_time = ((|| PResult::Ok((Decoder101(_input))?))())?;
let transparent_color_index = ((|| PResult::Ok((Decoder25(_input))?))())?;
let terminator = ((|| PResult::Ok((Decoder281(_input))?))())?;
PResult::Ok(gif_graphic_control_extension { separator, label, block_size, flags, delay_time, transparent_color_index, terminator })
}

fn Decoder_gif_graphic_rendering_block<'input>(_input: &mut Parser<'input>) -> Result<gif_graphic_rendering_block, ParseError> {
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
let inner = (Decoder_gif_table_based_image(_input))?;
gif_graphic_rendering_block::table_based_image(inner)
},

1 => {
let inner = (Decoder_gif_plain_text_extension(_input))?;
gif_graphic_rendering_block::plain_text_extension(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(14120387546690436687u64));
}
})
}

fn Decoder_gif_table_based_image<'input>(_input: &mut Parser<'input>) -> Result<gif_table_based_image, ParseError> {
let descriptor = ((|| PResult::Ok((Decoder_gif_image_descriptor(_input))?))())?;
let local_color_table = ((|| PResult::Ok(if descriptor.flags.table_flag.clone() != 0u8 {
let mut accum = Vec::new();
for _ in 0..2u16 << ((descriptor.flags.table_size.clone()) as u16) {
accum.push((Decoder287(_input))?);
}
Some(accum)
} else {
None
}))())?;
let data = ((|| PResult::Ok((Decoder_gif_table_based_image_data(_input))?))())?;
PResult::Ok(gif_table_based_image { descriptor, local_color_table, data })
}

fn Decoder_gif_plain_text_extension<'input>(_input: &mut Parser<'input>) -> Result<gif_plain_text_extension, ParseError> {
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
let text_grid_left_position = ((|| PResult::Ok((Decoder101(_input))?))())?;
let text_grid_top_position = ((|| PResult::Ok((Decoder101(_input))?))())?;
let text_grid_width = ((|| PResult::Ok((Decoder101(_input))?))())?;
let text_grid_height = ((|| PResult::Ok((Decoder101(_input))?))())?;
let character_cell_width = ((|| PResult::Ok((Decoder25(_input))?))())?;
let character_cell_height = ((|| PResult::Ok((Decoder25(_input))?))())?;
let text_foreground_color_index = ((|| PResult::Ok((Decoder25(_input))?))())?;
let text_background_color_index = ((|| PResult::Ok((Decoder25(_input))?))())?;
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
let next_elem = (Decoder_gif_subblock(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let terminator = ((|| PResult::Ok((Decoder281(_input))?))())?;
PResult::Ok(gif_plain_text_extension { separator, label, block_size, text_grid_left_position, text_grid_top_position, text_grid_width, text_grid_height, character_cell_width, character_cell_height, text_foreground_color_index, text_background_color_index, plain_text_data, terminator })
}

fn Decoder_gif_image_descriptor<'input>(_input: &mut Parser<'input>) -> Result<gif_image_descriptor, ParseError> {
let separator = ((|| PResult::Ok({
let b = _input.read_byte()?;
if b == 44 {
b
} else {
return Err(ParseError::ExcludedBranch(957865226307229178u64));
}
}))())?;
let image_left_position = ((|| PResult::Ok((Decoder101(_input))?))())?;
let image_top_position = ((|| PResult::Ok((Decoder101(_input))?))())?;
let image_width = ((|| PResult::Ok((Decoder101(_input))?))())?;
let image_height = ((|| PResult::Ok((Decoder101(_input))?))())?;
let flags = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(gif_image_descriptor_flags { table_flag: packedbits >> 7u8 & 1u8, interlace_flag: packedbits >> 6u8 & 1u8, sort_flag: packedbits >> 5u8 & 1u8, reserved: packedbits >> 3u8 & 3u8, table_size: packedbits >> 0u8 & 7u8 }))(inner))?
}))())?;
PResult::Ok(gif_image_descriptor { separator, image_left_position, image_top_position, image_width, image_height, flags })
}

fn Decoder287<'input>(_input: &mut Parser<'input>) -> Result<png_plte, ParseError> {
let r = ((|| PResult::Ok((Decoder25(_input))?))())?;
let g = ((|| PResult::Ok((Decoder25(_input))?))())?;
let b = ((|| PResult::Ok((Decoder25(_input))?))())?;
PResult::Ok(png_plte { r, g, b })
}

fn Decoder_gif_table_based_image_data<'input>(_input: &mut Parser<'input>) -> Result<gif_table_based_image_data, ParseError> {
let lzw_min_code_size = ((|| PResult::Ok((Decoder25(_input))?))())?;
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
let next_elem = (Decoder_gif_subblock(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
}))())?;
let terminator = ((|| PResult::Ok((Decoder281(_input))?))())?;
PResult::Ok(gif_table_based_image_data { lzw_min_code_size, image_data, terminator })
}

fn Decoder_gif_logical_screen_descriptor<'input>(_input: &mut Parser<'input>) -> Result<gif_logical_screen_descriptor, ParseError> {
let screen_width = ((|| PResult::Ok((Decoder101(_input))?))())?;
let screen_height = ((|| PResult::Ok((Decoder101(_input))?))())?;
let flags = ((|| PResult::Ok({
let inner = {
let b = _input.read_byte()?;
b
};
((|packedbits: u8| PResult::Ok(gif_logical_screen_descriptor_flags { table_flag: packedbits >> 7u8 & 1u8, color_resolution: packedbits >> 4u8 & 7u8, sort_flag: packedbits >> 3u8 & 1u8, table_size: packedbits >> 0u8 & 7u8 }))(inner))?
}))())?;
let bg_color_index = ((|| PResult::Ok((Decoder25(_input))?))())?;
let pixel_aspect_ratio = ((|| PResult::Ok((Decoder25(_input))?))())?;
PResult::Ok(gif_logical_screen_descriptor { screen_width, screen_height, flags, bg_color_index, pixel_aspect_ratio })
}

fn Decoder290<'input>(_input: &mut Parser<'input>) -> Result<u32, ParseError> {
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
PResult::Ok(((|tuple_var: (Vec<u8>, u8)| PResult::Ok(match tuple_var {
(s, _z) => {
(s.len()) as u32
}
}))(inner))?)
}

