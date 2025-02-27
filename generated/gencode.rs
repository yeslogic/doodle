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
pub struct elf_header_ident {
magic: (u8, u8, u8, u8),
class: u8,
data: u8,
version: u8,
os_abi: u8,
abi_version: u8,
__pad: Vec<u8>
}

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub enum elf_types_elf_addr { Addr32(u32), Addr64(u64) }

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub enum elf_types_elf_off { Off32(u32), Off64(u64) }

/// expected size: 112
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

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub enum elf_types_elf_full { Full32(u32), Full64(u64) }

/// expected size: 120
#[derive(Debug, Copy, Clone)]
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

/// expected size: 112
#[derive(Debug, Copy, Clone)]
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

/// expected size: 192
#[derive(Debug, Clone)]
pub struct elf_main {
header: elf_header,
__eoh: u64,
program_headers: Option<Vec<elf_phdr_table>>,
section_headers: Option<Vec<elf_shdr_table>>,
sections: Option<Vec<Option<Vec<u8>>>>,
__skip: ()
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct gif_header {
signature: (u8, u8, u8),
version: Vec<u8>
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct gif_logical_screen_descriptor_flags {
table_flag: u8,
color_resolution: u8,
sort_flag: u8,
table_size: u8
}

/// expected size: 10
#[derive(Debug, Copy, Clone)]
pub struct gif_logical_screen_descriptor {
screen_width: u16,
screen_height: u16,
flags: gif_logical_screen_descriptor_flags,
bg_color_index: u8,
pixel_aspect_ratio: u8
}

/// expected size: 3
#[derive(Debug, Copy, Clone)]
pub struct png_plte {
r: u8,
g: u8,
b: u8
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct gif_logical_screen {
descriptor: gif_logical_screen_descriptor,
global_color_table: Option<Vec<png_plte>>
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct gif_graphic_control_extension_flags {
reserved: u8,
disposal_method: u8,
user_input_flag: u8,
transparent_color_flag: u8
}

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct gif_graphic_control_extension {
separator: u8,
label: u8,
block_size: u8,
flags: gif_graphic_control_extension_flags,
delay_time: u16,
transparent_color_index: u8,
terminator: u8
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct gif_subblock {
len_bytes: u8,
data: Vec<u8>
}

/// expected size: 40
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

/// expected size: 5
#[derive(Debug, Copy, Clone)]
pub struct gif_image_descriptor_flags {
table_flag: u8,
interlace_flag: u8,
sort_flag: u8,
reserved: u8,
table_size: u8
}

/// expected size: 14
#[derive(Debug, Copy, Clone)]
pub struct gif_image_descriptor {
separator: u8,
image_left_position: u16,
image_top_position: u16,
image_width: u16,
image_height: u16,
flags: gif_image_descriptor_flags
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct gif_table_based_image_data {
lzw_min_code_size: u8,
image_data: Vec<gif_subblock>,
terminator: u8
}

/// expected size: 72
#[derive(Debug, Clone)]
pub struct gif_table_based_image {
descriptor: gif_image_descriptor,
local_color_table: Option<Vec<png_plte>>,
data: gif_table_based_image_data
}

/// expected size: 80
#[derive(Debug, Clone)]
pub enum gif_graphic_rendering_block { plain_text_extension(gif_plain_text_extension), table_based_image(gif_table_based_image) }

/// expected size: 96
#[derive(Debug, Clone)]
pub struct gif_graphic_block {
graphic_control_extension: Option<gif_graphic_control_extension>,
graphic_rendering_block: gif_graphic_rendering_block
}

/// expected size: 80
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

/// expected size: 32
#[derive(Debug, Clone)]
pub struct gif_comment_extension {
separator: u8,
label: u8,
comment_data: Vec<gif_subblock>,
terminator: u8
}

/// expected size: 88
#[derive(Debug, Clone)]
pub enum gif_special_purpose_block { application_extension(gif_application_extension), comment_extension(gif_comment_extension) }

/// expected size: 104
#[derive(Debug, Clone)]
pub enum gif_block { graphic_block(gif_graphic_block), special_purpose_block(gif_special_purpose_block) }

/// expected size: 1
#[derive(Debug, Copy, Clone)]
pub struct gif_trailer {
separator: u8
}

/// expected size: 104
#[derive(Debug, Clone)]
pub struct gif_main {
header: gif_header,
logical_screen: gif_logical_screen,
blocks: Vec<gif_block>,
trailer: gif_trailer
}

/// expected size: 5
#[derive(Debug, Copy, Clone)]
pub struct gzip_header_file_flags {
fcomment: bool,
fname: bool,
fextra: bool,
fhcrc: bool,
ftext: bool
}

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub struct gzip_header {
magic: (u8, u8),
method: u8,
file_flags: gzip_header_file_flags,
timestamp: u32,
compression_flags: u8,
os_id: u8
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct gzip_fextra_subfield {
si1: u8,
si2: u8,
len: u16,
data: Vec<u8>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct gzip_fextra {
xlen: u16,
subfields: Vec<gzip_fextra_subfield>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct base_asciiz_string {
string: Vec<u8>,
null: u8
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct gzip_fcomment {
comment: base_asciiz_string
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct gzip_fhcrc {
crc: u16
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths {
code: u16,
extra: u8
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct deflate_distance_record {
distance_extra_bits: u16,
distance: u16
}

/// expected size: 10
#[derive(Debug, Copy, Clone)]
pub struct deflate_dynamic_huffman_codes_values {
length_extra_bits: u8,
length: u16,
distance_code: u16,
distance_record: deflate_distance_record
}

/// expected size: 14
#[derive(Debug, Copy, Clone)]
pub struct deflate_dynamic_huffman_codes {
code: u16,
extra: Option<deflate_dynamic_huffman_codes_values>
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct deflate_main_codes_reference {
length: u16,
distance: u16
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub enum deflate_main_codes { literal(u8), reference(deflate_main_codes_reference) }

/// expected size: 176
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
codes_values: Vec<deflate_main_codes>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct deflate_fixed_huffman_codes_values {
length_extra_bits: u8,
length: u16,
distance_code: u8,
distance_record: deflate_distance_record
}

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct deflate_fixed_huffman_codes {
code: u16,
extra: Option<deflate_fixed_huffman_codes_values>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct deflate_fixed_huffman {
codes: Vec<deflate_fixed_huffman_codes>,
codes_values: Vec<deflate_main_codes>
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct deflate_uncompressed {
align: (),
len: u16,
nlen: u16,
bytes: Vec<u8>,
codes_values: Vec<deflate_main_codes>
}

/// expected size: 184
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [DirectHeap, Noop, Noop] }, Layout { size: 56, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum deflate_main_codes__dupX1 { dynamic_huffman(deflate_dynamic_huffman), fixed_huffman(deflate_fixed_huffman), uncompressed(deflate_uncompressed) }

/// expected size: 192
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [DirectHeap, Noop, Noop] })] }, Layout { size: 64, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct deflate_block {
r#final: u8,
r#type: u8,
data: deflate_main_codes__dupX1
}

/// expected size: 72
#[derive(Debug, Clone)]
pub struct deflate_main {
blocks: Vec<deflate_block>,
codes: Vec<deflate_main_codes>,
inflate: Vec<u8>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct gzip_footer {
crc: u32,
length: u32
}

/// expected size: 200
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

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct jpeg_eoi {
ff: u8,
marker: u8
}

/// expected size: 40
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

/// expected size: 48
#[derive(Debug, Clone)]
pub enum jpeg_app0_data_data { jfif(jpeg_app0_jfif), other(Vec<u8>) }

/// expected size: 80
#[derive(Debug, Clone)]
pub struct jpeg_app0_data {
identifier: base_asciiz_string,
data: jpeg_app0_data_data
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct jpeg_app0 {
marker: jpeg_eoi,
length: u16,
data: jpeg_app0_data
}

/// expected size: 3
#[derive(Debug, Copy, Clone)]
pub enum tiff_main_byte_order { be(u8, u8), le(u8, u8) }

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct tiff_main_ifd_fields {
tag: u16,
r#type: u16,
length: u32,
offset_or_data: u32
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct tiff_main_ifd {
num_fields: u16,
fields: Vec<tiff_main_ifd_fields>,
next_ifd_offset: u32,
next_ifd: Vec<u8>
}

/// expected size: 72
#[derive(Debug, Clone)]
pub struct tiff_main {
start_of_header: u32,
byte_order: tiff_main_byte_order,
magic: u16,
offset: u32,
ifd: tiff_main_ifd
}

/// expected size: 80
#[derive(Debug, Clone)]
pub struct jpeg_app1_exif {
padding: u8,
exif: tiff_main
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct jpeg_app1_xmp {
xmp: Vec<u8>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub enum jpeg_app1_data_data { exif(jpeg_app1_exif), other(Vec<u8>), xmp(jpeg_app1_xmp) }

/// expected size: 120
#[derive(Debug, Clone)]
pub struct jpeg_app1_data {
identifier: base_asciiz_string,
data: jpeg_app1_data_data
}

/// expected size: 128
#[derive(Debug, Clone)]
pub struct jpeg_app1 {
marker: jpeg_eoi,
length: u16,
data: jpeg_app1_data
}

/// expected size: 136
#[derive(Debug, Clone)]
pub enum jpeg_frame_initial_segment { app0(jpeg_app0), app1(jpeg_app1) }

/// expected size: 32
#[derive(Debug, Clone)]
pub struct jpeg_com {
marker: jpeg_eoi,
length: u16,
data: Vec<u8>
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dac_data_class_table_id {
class: u8,
table_id: u8
}

/// expected size: 3
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dac_data {
class_table_id: jpeg_dac_data_class_table_id,
value: u8
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dac {
marker: jpeg_eoi,
length: u16,
data: jpeg_dac_data
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct jpeg_dht_data {
class_table_id: jpeg_dac_data_class_table_id,
num_codes: Vec<u8>,
values: Vec<Vec<u8>>
}

/// expected size: 64
#[derive(Debug, Clone)]
pub struct jpeg_dht {
marker: jpeg_eoi,
length: u16,
data: jpeg_dht_data
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dqt_data_precision_table_id {
precision: u8,
table_id: u8
}

/// expected size: 32
#[derive(Debug, Clone)]
pub enum jpeg_dqt_data_elements { Bytes(Vec<u8>), Shorts(Vec<u16>) }

/// expected size: 40
#[derive(Debug, Clone)]
pub struct jpeg_dqt_data {
precision_table_id: jpeg_dqt_data_precision_table_id,
elements: jpeg_dqt_data_elements
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct jpeg_dqt {
marker: jpeg_eoi,
length: u16,
data: Vec<jpeg_dqt_data>
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dri_data {
restart_interval: u16
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dri {
marker: jpeg_eoi,
length: u16,
data: jpeg_dri_data
}

/// expected size: 136
#[derive(Debug, Clone)]
pub enum jpeg_table_or_misc { app0(jpeg_app0), app1(jpeg_app1), app10(jpeg_com), app11(jpeg_com), app12(jpeg_com), app13(jpeg_com), app14(jpeg_com), app15(jpeg_com), app2(jpeg_com), app3(jpeg_com), app4(jpeg_com), app5(jpeg_com), app6(jpeg_com), app7(jpeg_com), app8(jpeg_com), app9(jpeg_com), com(jpeg_com), dac(jpeg_dac), dht(jpeg_dht), dqt(jpeg_dqt), dri(jpeg_dri) }

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct jpeg_sof_image_component_sampling_factor {
horizontal: u8,
vertical: u8
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct jpeg_sof_image_component {
id: u8,
sampling_factor: jpeg_sof_image_component_sampling_factor,
quantization_table_id: u8
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct jpeg_sof_data {
sample_precision: u8,
num_lines: u16,
num_samples_per_line: u16,
num_image_components: u8,
image_components: Vec<jpeg_sof_image_component>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct jpeg_sof15 {
marker: jpeg_eoi,
length: u16,
data: jpeg_sof_data
}

/// expected size: 48
#[derive(Debug, Clone)]
pub enum jpeg_frame_header { sof0(jpeg_sof15), sof1(jpeg_sof15), sof10(jpeg_sof15), sof11(jpeg_sof15), sof13(jpeg_sof15), sof14(jpeg_sof15), sof15(jpeg_sof15), sof2(jpeg_sof15), sof3(jpeg_sof15), sof5(jpeg_sof15), sof6(jpeg_sof15), sof7(jpeg_sof15), sof9(jpeg_sof15) }

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct jpeg_sos_image_component_entropy_coding_table_ids {
dc_entropy_coding_table_id: u8,
ac_entropy_coding_table_id: u8
}

/// expected size: 3
#[derive(Debug, Copy, Clone)]
pub struct jpeg_sos_image_component {
component_selector: u8,
entropy_coding_table_ids: jpeg_sos_image_component_entropy_coding_table_ids
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct jpeg_sos_data_approximation_bit_position {
high: u8,
low: u8
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct jpeg_sos_data {
num_image_components: u8,
image_components: Vec<jpeg_sos_image_component>,
start_spectral_selection: u8,
end_spectral_selection: u8,
approximation_bit_position: jpeg_sos_data_approximation_bit_position
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct jpeg_sos {
marker: jpeg_eoi,
length: u16,
data: jpeg_sos_data
}

/// expected size: 3
#[derive(Debug, Copy, Clone)]
pub enum jpeg_scan_data_scan_data { mcu(u8), rst0(jpeg_eoi), rst1(jpeg_eoi), rst2(jpeg_eoi), rst3(jpeg_eoi), rst4(jpeg_eoi), rst5(jpeg_eoi), rst6(jpeg_eoi), rst7(jpeg_eoi) }

/// expected size: 48
#[derive(Debug, Clone)]
pub struct jpeg_scan_data {
scan_data: Vec<jpeg_scan_data_scan_data>,
scan_data_stream: Vec<u8>
}

/// expected size: 112
#[derive(Debug, Clone)]
pub struct jpeg_scan {
segments: Vec<jpeg_table_or_misc>,
sos: jpeg_sos,
data: jpeg_scan_data
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dnl_data {
num_lines: u16
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dnl {
marker: jpeg_eoi,
length: u16,
data: jpeg_dnl_data
}

/// expected size: 352
#[derive(Debug, Clone)]
pub struct jpeg_frame {
initial_segment: jpeg_frame_initial_segment,
segments: Vec<jpeg_table_or_misc>,
header: jpeg_frame_header,
scan: jpeg_scan,
dnl: Option<jpeg_dnl>,
scans: Vec<jpeg_scan>
}

/// expected size: 360
#[derive(Debug, Clone)]
pub struct jpeg_main {
soi: jpeg_eoi,
frame: jpeg_frame,
eoi: jpeg_eoi
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_atom_data_ftyp {
major_brand: (u8, u8, u8, u8),
minor_version: u32,
compatible_brands: Vec<(u8, u8, u8, u8)>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stsd_sample_entries {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: Vec<u8>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_dinf_atom_data_dref {
version: u8,
flags: (u8, u8, u8),
number_of_entries: u32,
data: Vec<mpeg4_stbl_atom_data_stsd_sample_entries>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum mpeg4_dinf_atom_data { dref(mpeg4_dinf_atom_data_dref), unknown(Vec<u8>) }

/// expected size: 56
#[derive(Debug, Clone)]
pub struct mpeg4_dinf_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_dinf_atom_data
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_hdlr {
version: u8,
flags: (u8, u8, u8),
predefined: u32,
handler_type: (u8, u8, u8, u8),
reserved: (u32, u32, u32),
name: base_asciiz_string
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe_fields_no_extra_fields_mime {
content_type: base_asciiz_string
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe_fields_no_extra_fields_uri {
item_uri_type: base_asciiz_string
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum mpeg4_iinf_atom_data_infe_fields_no_extra_fields { mime(mpeg4_iinf_atom_data_infe_fields_no_extra_fields_mime), unknown, uri(mpeg4_iinf_atom_data_infe_fields_no_extra_fields_uri) }

/// expected size: 88
#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe_fields_no {
item_ID: u32,
item_protection_index: u16,
item_type: (u8, u8, u8, u8),
item_name: base_asciiz_string,
extra_fields: mpeg4_iinf_atom_data_infe_fields_no_extra_fields
}

/// expected size: 104
#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe_fields_yes {
item_ID: u16,
item_protection_index: u16,
item_name: base_asciiz_string,
content_type: base_asciiz_string,
content_encoding: base_asciiz_string
}

/// expected size: 112
#[derive(Debug, Clone)]
pub enum mpeg4_iinf_atom_data_infe_fields { no(mpeg4_iinf_atom_data_infe_fields_no), yes(mpeg4_iinf_atom_data_infe_fields_yes) }

/// expected size: 120
#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe {
version: u8,
flags: (u8, u8, u8),
fields: mpeg4_iinf_atom_data_infe_fields
}

/// expected size: 128
#[derive(Debug, Clone)]
pub enum mpeg4_iinf_atom_data { infe(mpeg4_iinf_atom_data_infe), unknown(Vec<u8>) }

/// expected size: 144
#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_iinf_atom_data
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iinf {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
item_info_entry: Vec<mpeg4_iinf_atom>
}

/// expected size: 24
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_meta_atom_data_iloc_items_extents {
extent_index: u64,
extent_offset: u64,
extent_length: u64
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iloc_items {
item_ID: u32,
construction_method: Option<u16>,
data_reference_index: u16,
base_offset: u64,
extent_count: u16,
extents: Vec<mpeg4_meta_atom_data_iloc_items_extents>
}

/// expected size: 40
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

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_tool_atom_data_data {
type_indicator: u32,
locale_indicator: u32,
value: Vec<u8>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum mpeg4_tool_atom_data { data(mpeg4_tool_atom_data_data), unknown(Vec<u8>) }

/// expected size: 56
#[derive(Debug, Clone)]
pub struct mpeg4_tool_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_tool_atom_data
}

/// expected size: 32
#[derive(Debug, Clone)]
pub enum mpeg4_ilst_atom_data { tool(Vec<mpeg4_tool_atom>), unknown(Vec<u8>) }

/// expected size: 48
#[derive(Debug, Clone)]
pub struct mpeg4_ilst_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_ilst_atom_data
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iref_single_item_reference_large_data {
from_item_ID: u32,
reference_count: u16,
to_item_ID: Vec<u32>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iref_single_item_reference_large {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_meta_atom_data_iref_single_item_reference_large_data
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iref_single_item_reference_small_data {
from_item_ID: u16,
reference_count: u16,
to_item_ID: Vec<u16>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iref_single_item_reference_small {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_meta_atom_data_iref_single_item_reference_small_data
}

/// expected size: 32
#[derive(Debug, Clone)]
pub enum mpeg4_meta_atom_data_iref_single_item_reference { large(Vec<mpeg4_meta_atom_data_iref_single_item_reference_large>), small(Vec<mpeg4_meta_atom_data_iref_single_item_reference_small>) }

/// expected size: 40
#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom_data_iref {
version: u8,
flags: (u8, u8, u8),
single_item_reference: mpeg4_meta_atom_data_iref_single_item_reference
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub enum mpeg4_meta_atom_data_pitm_item_ID { no(u32), yes(u16) }

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_meta_atom_data_pitm {
version: u8,
flags: (u8, u8, u8),
item_ID: mpeg4_meta_atom_data_pitm_item_ID
}

/// expected size: 64
#[derive(Debug, Clone)]
pub enum mpeg4_meta_atom_data { dinf(Vec<mpeg4_dinf_atom>), hdlr(mpeg4_meta_atom_data_hdlr), idat(Vec<u8>), iinf(mpeg4_meta_atom_data_iinf), iloc(mpeg4_meta_atom_data_iloc), ilst(Vec<mpeg4_ilst_atom>), iref(mpeg4_meta_atom_data_iref), pitm(mpeg4_meta_atom_data_pitm), unknown(Vec<u8>) }

/// expected size: 80
#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_meta_atom_data
}

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_moov_atom_data_mvhd_fields_version0 {
creation_time: u32,
modification_time: u32,
timescale: u32,
duration: u32
}

/// expected size: 32
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_moov_atom_data_mvhd_fields_version1 {
creation_time: u64,
modification_time: u64,
timescale: u32,
duration: u64
}

/// expected size: 40
#[derive(Debug, Copy, Clone)]
pub enum mpeg4_moov_atom_data_mvhd_fields { version0(mpeg4_moov_atom_data_mvhd_fields_version0), version1(mpeg4_moov_atom_data_mvhd_fields_version1) }

/// expected size: 112
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

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_edts_atom_data_elst_edit_list_table {
track_duration: u32,
media_time: u32,
media_rate: u32
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_edts_atom_data_elst {
version: u8,
flags: (u8, u8, u8),
number_of_entries: u32,
edit_list_table: Vec<mpeg4_edts_atom_data_elst_edit_list_table>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum mpeg4_edts_atom_data { elst(mpeg4_edts_atom_data_elst), unknown(Vec<u8>) }

/// expected size: 56
#[derive(Debug, Clone)]
pub struct mpeg4_edts_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_edts_atom_data
}

/// expected size: 56
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

/// expected size: 48
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_mdia_atom_data_mdhd {
version: u8,
flags: (u8, u8, u8),
fields: mpeg4_moov_atom_data_mvhd_fields,
language: u16,
pre_defined: u16
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_minf_atom_data_smhd {
version: u8,
flags: (u8, u8, u8),
balance: u16,
reserved: u16
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_co64 {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_offset: Vec<u64>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_stbl_atom_data_ctts_sample_entries {
sample_count: u32,
sample_offset: u32
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_ctts {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<mpeg4_stbl_atom_data_ctts_sample_entries>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_stbl_atom_data_sbgp_sample_groups {
sample_count: u32,
group_description_index: u32
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_sbgp {
version: u8,
flags: (u8, u8, u8),
grouping_type: u32,
grouping_type_parameter: Option<u32>,
entry_count: u32,
sample_groups: Vec<mpeg4_stbl_atom_data_sbgp_sample_groups>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_sgpd_sample_groups {
description_length: u32,
sample_group_entry: Vec<u8>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_sgpd {
version: u8,
flags: (u8, u8, u8),
grouping_type: u32,
default_length: u32,
entry_count: u32,
sample_groups: Vec<mpeg4_stbl_atom_data_sgpd_sample_groups>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stco {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_offset: Vec<u32>
}

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_stbl_atom_data_stsc_chunk_entries {
first_chunk: u32,
samples_per_chunk: u32,
sample_description_index: u32
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stsc {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_entries: Vec<mpeg4_stbl_atom_data_stsc_chunk_entries>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stsd {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<mpeg4_stbl_atom_data_stsd_sample_entries>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stss {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_number: Vec<u32>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stsz {
version: u8,
flags: (u8, u8, u8),
sample_size: u32,
sample_count: u32,
entry_size: Option<Vec<u32>>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_stbl_atom_data_stts_sample_entries {
sample_count: u32,
sample_delta: u32
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom_data_stts {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<mpeg4_stbl_atom_data_stts_sample_entries>
}

/// expected size: 56
#[derive(Debug, Clone)]
pub enum mpeg4_stbl_atom_data { co64(mpeg4_stbl_atom_data_co64), ctts(mpeg4_stbl_atom_data_ctts), sbgp(mpeg4_stbl_atom_data_sbgp), sgpd(mpeg4_stbl_atom_data_sgpd), stco(mpeg4_stbl_atom_data_stco), stsc(mpeg4_stbl_atom_data_stsc), stsd(mpeg4_stbl_atom_data_stsd), stss(mpeg4_stbl_atom_data_stss), stsz(mpeg4_stbl_atom_data_stsz), stts(mpeg4_stbl_atom_data_stts), unknown(Vec<u8>) }

/// expected size: 72
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_stbl_atom_data
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct mpeg4_minf_atom_data_vmhd {
version: u8,
flags: (u8, u8, u8),
graphicsmode: u16,
opcolor: Vec<u16>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum mpeg4_minf_atom_data { dinf(Vec<mpeg4_dinf_atom>), smhd(mpeg4_minf_atom_data_smhd), stbl(Vec<mpeg4_stbl_atom>), unknown(Vec<u8>), vmhd(mpeg4_minf_atom_data_vmhd) }

/// expected size: 56
#[derive(Debug, Clone)]
pub struct mpeg4_minf_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_minf_atom_data
}

/// expected size: 64
#[derive(Debug, Clone)]
pub enum mpeg4_mdia_atom_data { hdlr(mpeg4_mdia_atom_data_hdlr), mdhd(mpeg4_mdia_atom_data_mdhd), minf(Vec<mpeg4_minf_atom>), unknown(Vec<u8>) }

/// expected size: 80
#[derive(Debug, Clone)]
pub struct mpeg4_mdia_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_mdia_atom_data
}

/// expected size: 20
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_trak_atom_data_tkhd_fields_version0 {
creation_time: u32,
modification_time: u32,
track_ID: u32,
reserved: u32,
duration: u32
}

/// expected size: 32
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_trak_atom_data_tkhd_fields_version1 {
creation_time: u64,
modification_time: u64,
track_ID: u32,
reserved: u32,
duration: u64
}

/// expected size: 40
#[derive(Debug, Copy, Clone)]
pub enum mpeg4_trak_atom_data_tkhd_fields { version0(mpeg4_trak_atom_data_tkhd_fields_version0), version1(mpeg4_trak_atom_data_tkhd_fields_version1) }

/// expected size: 96
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

/// expected size: 104
#[derive(Debug, Clone)]
pub enum mpeg4_trak_atom_data { edts(Vec<mpeg4_edts_atom>), mdia(Vec<mpeg4_mdia_atom>), tkhd(mpeg4_trak_atom_data_tkhd), unknown(Vec<u8>) }

/// expected size: 120
#[derive(Debug, Clone)]
pub struct mpeg4_trak_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_trak_atom_data
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum mpeg4_udta_atom_data { meta(u32, Vec<mpeg4_meta_atom>), unknown(Vec<u8>) }

/// expected size: 56
#[derive(Debug, Clone)]
pub struct mpeg4_udta_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_udta_atom_data
}

/// expected size: 120
#[derive(Debug, Clone)]
pub enum mpeg4_moov_atom_data { mvhd(mpeg4_moov_atom_data_mvhd), trak(Vec<mpeg4_trak_atom>), udta(Vec<mpeg4_udta_atom>), unknown(Vec<u8>) }

/// expected size: 136
#[derive(Debug, Clone)]
pub struct mpeg4_moov_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_moov_atom_data
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum mpeg4_atom_data { free, ftyp(mpeg4_atom_data_ftyp), mdat, meta(u32, Vec<mpeg4_meta_atom>), moov(Vec<mpeg4_moov_atom>), unknown(Vec<u8>) }

/// expected size: 56
#[derive(Debug, Clone)]
pub struct mpeg4_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_atom_data
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct mpeg4_main {
atoms: Vec<mpeg4_atom>
}

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub struct opentype_table_record {
table_id: u32,
checksum: u32,
offset: u32,
length: u32
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format0 {
format: u16,
length: u16,
language: u16,
glyph_id_array: Vec<u8>
}

/// expected size: 48
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

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct opentype_types_sequential_map_record {
start_char_code: u32,
end_char_code: u32,
start_glyph_id: u32
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format13 {
format: u16,
__reserved: u16,
length: u32,
language: u32,
num_groups: u32,
groups: Vec<opentype_types_sequential_map_record>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_variation_selector_default_uvs_offset_link_ranges {
start_unicode_value: u32,
additional_count: u8
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_variation_selector_default_uvs_offset_link {
num_unicode_value_ranges: u32,
ranges: Vec<opentype_variation_selector_default_uvs_offset_link_ranges>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_variation_selector_default_uvs_offset {
offset: u32,
link: Option<opentype_variation_selector_default_uvs_offset_link>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_variation_selector_non_default_uvs_offset_link_uvs_mappings {
unicode_value: u32,
glyph_id: u16
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_variation_selector_non_default_uvs_offset_link {
num_uvs_mappings: u32,
uvs_mappings: Vec<opentype_variation_selector_non_default_uvs_offset_link_uvs_mappings>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_variation_selector_non_default_uvs_offset {
offset: u32,
link: Option<opentype_variation_selector_non_default_uvs_offset_link>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_variation_selector {
var_selector: u32,
default_uvs_offset: opentype_variation_selector_default_uvs_offset,
non_default_uvs_offset: opentype_variation_selector_non_default_uvs_offset
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format14 {
format: u16,
length: u32,
num_var_selector_records: u32,
var_selector: Vec<opentype_variation_selector>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_cmap_subtable_format2_sub_headers {
first_code: u16,
entry_count: u16,
id_delta: u16,
id_range_offset: u16
}

/// expected size: 80
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format2 {
format: u16,
length: u16,
language: u16,
sub_header_keys: Vec<u16>,
sub_headers: Vec<opentype_cmap_subtable_format2_sub_headers>,
glyph_array: Vec<u16>
}

/// expected size: 136
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

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format6 {
format: u16,
length: u16,
language: u16,
first_code: u16,
entry_count: u16,
glyph_id_array: Vec<u16>
}

/// expected size: 64
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

/// expected size: 144
#[derive(Debug, Clone)]
pub enum opentype_cmap_subtable_data { Format0(opentype_cmap_subtable_format0), Format10(opentype_cmap_subtable_format10), Format12(opentype_cmap_subtable_format13), Format13(opentype_cmap_subtable_format13), Format14(opentype_cmap_subtable_format14), Format2(opentype_cmap_subtable_format2), Format4(opentype_cmap_subtable_format4), Format6(opentype_cmap_subtable_format6), Format8(opentype_cmap_subtable_format8) }

/// expected size: 152
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable {
table_start: u32,
format: u16,
data: opentype_cmap_subtable_data
}

/// expected size: 160
#[derive(Debug, Clone)]
pub struct opentype_encoding_record_subtable_offset {
offset: u32,
link: Option<opentype_cmap_subtable>
}

/// expected size: 168
#[derive(Debug, Clone)]
pub struct opentype_encoding_record {
platform: u16,
encoding: u16,
subtable_offset: opentype_encoding_record_subtable_offset
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_cmap_table {
table_start: u32,
version: u16,
num_tables: u16,
encoding_records: Vec<opentype_encoding_record>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub enum opentype_var_user_tuple_coordinates { Fixed32(u32) }

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_head_table_glyph_extents {
x_min: u16,
y_min: u16,
x_max: u16,
y_max: u16
}

/// expected size: 7
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

/// expected size: 64
#[derive(Debug, Copy, Clone)]
pub struct opentype_head_table {
major_version: u16,
minor_version: u16,
font_revision: opentype_var_user_tuple_coordinates,
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

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct opentype_hhea_table_caret_slope {
rise: u16,
run: u16
}

/// expected size: 36
#[derive(Debug, Copy, Clone)]
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
number_of_long_metrics: u16
}

/// expected size: 26
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

/// expected size: 28
#[derive(Debug, Copy, Clone)]
pub enum opentype_maxp_table_data { MaxpPostScript, MaxpUnknown(u32), MaxpV1(opentype_maxp_table_version1) }

/// expected size: 36
#[derive(Debug, Copy, Clone)]
pub struct opentype_maxp_table {
version: u32,
num_glyphs: u16,
data: opentype_maxp_table_data
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct opentype_hmtx_table_long_metrics {
advance_width: u16,
left_side_bearing: u16
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_hmtx_table {
long_metrics: Vec<opentype_hmtx_table_long_metrics>,
left_side_bearings: Vec<u16>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_name_table_name_records_offset {
offset: u16,
link: Option<Vec<u8>>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_name_table_name_records {
platform: u16,
encoding: u16,
language: u16,
name_id: u16,
length: u16,
offset: opentype_name_table_name_records_offset
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_name_table_name_version_1_lang_tag_records {
length: u16,
offset: opentype_name_table_name_records_offset
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_name_table_name_version_1 {
lang_tag_count: u16,
lang_tag_records: Vec<opentype_name_table_name_version_1_lang_tag_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum opentype_name_table_data { NameVersion0, NameVersion1(opentype_name_table_name_version_1), NameVersionUnknown(u16) }

/// expected size: 80
#[derive(Debug, Clone)]
pub struct opentype_name_table {
table_start: u32,
version: u16,
name_count: u16,
storage_offset: u16,
name_records: Vec<opentype_name_table_name_records>,
data: opentype_name_table_data
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct opentype_os2_table_data_extra_fields_v1_extra_fields_v2_extra_fields_v5 {
us_lower_optical_point_size: u16,
us_upper_optical_point_size: u16
}

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub struct opentype_os2_table_data_extra_fields_v1_extra_fields_v2 {
sx_height: u16,
s_cap_height: u16,
us_default_char: u16,
us_break_char: u16,
us_max_context: u16,
extra_fields_v5: Option<opentype_os2_table_data_extra_fields_v1_extra_fields_v2_extra_fields_v5>
}

/// expected size: 24
#[derive(Debug, Copy, Clone)]
pub struct opentype_os2_table_data_extra_fields_v1 {
ul_code_page_range_1: u32,
ul_code_page_range_2: u32,
extra_fields_v2: Option<opentype_os2_table_data_extra_fields_v1_extra_fields_v2>
}

/// expected size: 36
#[derive(Debug, Copy, Clone)]
pub struct opentype_os2_table_data {
s_typo_ascender: u16,
s_typo_descender: u16,
s_typo_line_gap: u16,
us_win_ascent: u16,
us_win_descent: u16,
extra_fields_v1: Option<opentype_os2_table_data_extra_fields_v1>
}

/// expected size: 120
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

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_post_table_names_Version2 {
num_glyphs: u16,
glyph_name_index: Vec<u16>,
string_data: u32
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_post_table_names_Version2Dot5 {
num_glyphs: u16,
offset: Vec<u8>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum opentype_post_table_names { Version1, Version2(opentype_post_table_names_Version2), Version2Dot5(opentype_post_table_names_Version2Dot5), Version3, VersionUnknown(u32) }

/// expected size: 80
#[derive(Debug, Clone)]
pub struct opentype_post_table {
version: u32,
italic_angle: opentype_var_user_tuple_coordinates,
underline_position: u16,
underline_thickness: u16,
is_fixed_pitch: u32,
min_mem_type42: u32,
max_mem_type42: u32,
min_mem_type1: u32,
max_mem_type1: u32,
names: opentype_post_table_names
}

/// expected size: 32
#[derive(Debug, Clone)]
pub enum opentype_gvar_table_glyph_variation_data_offsets { Offsets16(Vec<u16>), Offsets32(Vec<u32>) }

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_loca_table {
offsets: opentype_gvar_table_glyph_variation_data_offsets
}

/// expected size: 12
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
we_have_a_scale: bool,
round_xy_to_grid: bool,
args_are_xy_values: bool,
arg_1_and_2_are_words: bool
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub enum opentype_glyf_composite_raw_argument1 { Int16(u16), Int8(u8), Uint16(u16), Uint8(u8) }

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub enum opentype_var_tuple_record_coordinates { F2Dot14(u16) }

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_composite_raw_scale_XY {
x_scale: opentype_var_tuple_record_coordinates,
y_scale: opentype_var_tuple_record_coordinates
}

/// expected size: 18
#[derive(Debug, Copy, Clone)]
pub enum opentype_glyf_composite_raw_scale { Matrix((opentype_var_tuple_record_coordinates, opentype_var_tuple_record_coordinates), (opentype_var_tuple_record_coordinates, opentype_var_tuple_record_coordinates)), Scale(opentype_var_tuple_record_coordinates), XY(opentype_glyf_composite_raw_scale_XY) }

/// expected size: 42
#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_composite_raw {
flags: opentype_glyf_composite_raw_flags,
glyph_index: u16,
argument1: opentype_glyf_composite_raw_argument1,
argument2: opentype_glyf_composite_raw_argument1,
scale: Option<opentype_glyf_composite_raw_scale>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_glyf_composite {
glyphs: Vec<opentype_glyf_composite_raw>,
instructions: Vec<u8>
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_simple_flags {
on_curve_point: bool,
x_short_vector: bool,
y_short_vector: bool,
x_is_same_or_positive_x_short_vector: bool,
y_is_same_or_positive_y_short_vector: bool,
overlap_simple: bool
}

/// expected size: 128
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

/// expected size: 136
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [Noop, Noop, DirectHeap] }, Layout { size: 48, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum opentype_glyf_description { Composite(opentype_glyf_composite), HeaderOnly, Simple(opentype_glyf_simple) }

/// expected size: 152
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, Noop, Noop, Noop, Noop, InDef(InEnum { variants: [Noop, Noop, DirectHeap] })] }, Layout { size: 64, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct opentype_glyf_table_Glyph {
number_of_contours: u16,
x_min: u16,
y_min: u16,
x_max: u16,
y_max: u16,
description: opentype_glyf_description
}

/// expected size: 160
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (NonLocal, Layout { size: 72, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum opentype_glyf_table { EmptyGlyph, Glyph(opentype_glyf_table_Glyph) }

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version0 {
dogray: bool,
gridfit: bool
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version1 {
symmetric_smoothing: bool,
symmetric_gridfit: bool,
dogray: bool,
gridfit: bool
}

/// expected size: 5
#[derive(Debug, Copy, Clone)]
pub enum opentype_gasp_table_gasp_ranges_range_gasp_behavior { Version0(opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version0), Version1(opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version1) }

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_gasp_table_gasp_ranges {
range_max_ppem: u16,
range_gasp_behavior: opentype_gasp_table_gasp_ranges_range_gasp_behavior
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_gasp_table {
version: u16,
num_ranges: u16,
gasp_ranges: Vec<opentype_gasp_table_gasp_ranges>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_axis_table_base_tag_list_offset_link {
base_tag_count: u16,
baseline_tags: Vec<u32>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_axis_table_base_tag_list_offset {
offset: u16,
link: Option<opentype_layout_axis_table_base_tag_list_offset_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_common_device_or_variation_index_table_DeviceTable {
start_size: u16,
end_size: u16,
delta_format: u16,
delta_values: Vec<u16>
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_device_or_variation_index_table_OtherTable {
field0: u16,
field1: u16,
delta_format: u16
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_device_or_variation_index_table_VariationIndexTable {
delta_set_outer_index: u16,
delta_set_inner_index: u16,
delta_format: (u8, u8)
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum opentype_common_device_or_variation_index_table { DeviceTable(opentype_common_device_or_variation_index_table_DeviceTable), OtherTable(opentype_common_device_or_variation_index_table_OtherTable), VariationIndexTable(opentype_common_device_or_variation_index_table_VariationIndexTable) }

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_common_value_record_x_advance_device {
offset: u16,
link: Option<opentype_common_device_or_variation_index_table>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_layout_base_coord_hint_DeviceHint {
device_offset: opentype_common_value_record_x_advance_device
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_base_coord_hint_GlyphHint {
reference_glyph: u16,
base_coord_point: u16
}

/// expected size: 56
#[derive(Debug, Clone)]
pub enum opentype_layout_base_coord_hint { DeviceHint(opentype_layout_base_coord_hint_DeviceHint), GlyphHint(opentype_layout_base_coord_hint_GlyphHint), NoHint }

/// expected size: 64
#[derive(Debug, Clone)]
pub struct opentype_layout_base_coord {
table_start: u32,
format: u16,
coordinate: u16,
hint: opentype_layout_base_coord_hint
}

/// expected size: 72
#[derive(Debug, Clone)]
pub struct opentype_layout_min_max_min_coord_offset {
offset: u16,
link: Option<opentype_layout_base_coord>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_base_values {
table_start: u32,
default_baseline_index: u16,
base_coord_count: u16,
base_coord_offsets: Vec<opentype_layout_min_max_min_coord_offset>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_base_script_base_values_offset {
offset: u16,
link: Option<opentype_layout_base_values>
}

/// expected size: 152
#[derive(Debug, Clone)]
pub struct opentype_layout_min_max_feat_min_max_records {
feature_tag: u32,
min_coord_offset: opentype_layout_min_max_min_coord_offset,
max_coord_offset: opentype_layout_min_max_min_coord_offset
}

/// expected size: 176
#[derive(Debug, Clone)]
pub struct opentype_layout_min_max {
table_start: u32,
min_coord_offset: opentype_layout_min_max_min_coord_offset,
max_coord_offset: opentype_layout_min_max_min_coord_offset,
feat_min_max_count: u16,
feat_min_max_records: Vec<opentype_layout_min_max_feat_min_max_records>
}

/// expected size: 184
#[derive(Debug, Clone)]
pub struct opentype_layout_base_script_default_min_max_offset {
offset: u16,
link: Option<opentype_layout_min_max>
}

/// expected size: 192
#[derive(Debug, Clone)]
pub struct opentype_layout_base_script_base_lang_sys_records {
base_lang_sys_tag: u32,
min_max_offset: opentype_layout_base_script_default_min_max_offset
}

/// expected size: 256
#[derive(Debug, Clone)]
pub struct opentype_layout_base_script {
table_start: u32,
base_values_offset: opentype_layout_base_script_base_values_offset,
default_min_max_offset: opentype_layout_base_script_default_min_max_offset,
base_lang_sys_count: u16,
base_lang_sys_records: Vec<opentype_layout_base_script_base_lang_sys_records>
}

/// expected size: 264
#[derive(Debug, Clone)]
pub struct opentype_layout_axis_table_base_script_list_offset_link_base_script_records_base_script_offset {
offset: u16,
link: Option<opentype_layout_base_script>
}

/// expected size: 272
#[derive(Debug, Clone)]
pub struct opentype_layout_axis_table_base_script_list_offset_link_base_script_records {
base_script_tag: u32,
base_script_offset: opentype_layout_axis_table_base_script_list_offset_link_base_script_records_base_script_offset
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_axis_table_base_script_list_offset_link {
table_start: u32,
base_script_count: u16,
base_script_records: Vec<opentype_layout_axis_table_base_script_list_offset_link_base_script_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_axis_table_base_script_list_offset {
offset: u16,
link: Option<opentype_layout_axis_table_base_script_list_offset_link>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_layout_axis_table {
table_start: u32,
base_tag_list_offset: opentype_layout_axis_table_base_tag_list_offset,
base_script_list_offset: opentype_layout_axis_table_base_script_list_offset
}

/// expected size: 96
#[derive(Debug, Clone)]
pub struct opentype_base_table_vert_axis_offset {
offset: u16,
link: Option<opentype_layout_axis_table>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_base_table_item_var_store_offset {
offset: u32,
link: Option<()>
}

/// expected size: 208
#[derive(Debug, Clone)]
pub struct opentype_base_table {
table_start: u32,
major_version: u16,
minor_version: u16,
horiz_axis_offset: opentype_base_table_vert_axis_offset,
vert_axis_offset: opentype_base_table_vert_axis_offset,
item_var_store_offset: Option<opentype_base_table_item_var_store_offset>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_class_def_data_Format1 {
start_glyph_id: u16,
glyph_count: u16,
class_value_array: Vec<u16>
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct opentype_class_def_data_Format2_class_range_records {
start_glyph_id: u16,
end_glyph_id: u16,
class: u16
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_class_def_data_Format2 {
class_range_count: u16,
class_range_records: Vec<opentype_class_def_data_Format2_class_range_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum opentype_class_def_data { Format1(opentype_class_def_data_Format1), Format2(opentype_class_def_data_Format2) }

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_class_def {
class_format: u16,
data: opentype_class_def_data
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_glyph_class_def {
offset: u16,
link: Option<opentype_class_def>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_coverage_table_data_Format1 {
glyph_count: u16,
glyph_array: Vec<u16>
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct opentype_coverage_table_data_Format2_range_records {
start_glyph_id: u16,
end_glyph_id: u16,
start_coverage_index: u16
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_coverage_table_data_Format2 {
range_count: u16,
range_records: Vec<opentype_coverage_table_data_Format2_range_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum opentype_coverage_table_data { Format1(opentype_coverage_table_data_Format1), Format2(opentype_coverage_table_data_Format2) }

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_coverage_table {
coverage_format: u16,
data: opentype_coverage_table_data
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_layout_reverse_chain_single_subst_coverage {
offset: u16,
link: Option<opentype_coverage_table>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_attach_list_link_attach_point_offsets_link {
point_count: u16,
point_indices: Vec<u16>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_attach_list_link_attach_point_offsets {
offset: u16,
link: Option<opentype_gdef_table_attach_list_link_attach_point_offsets_link>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_attach_list_link {
table_start: u32,
coverage: opentype_layout_reverse_chain_single_subst_coverage,
glyph_count: u16,
attach_point_offsets: Vec<opentype_gdef_table_attach_list_link_attach_point_offsets>
}

/// expected size: 96
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_attach_list {
offset: u16,
link: Option<opentype_gdef_table_attach_list_link>
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format1 {
coordinate: u16
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format2 {
caret_value_point_index: u16
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format3 {
coordinate: u16,
table: opentype_common_value_record_x_advance_device
}

/// expected size: 64
#[derive(Debug, Clone)]
pub enum opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data { Format1(opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format1), Format2(opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format2), Format3(opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format3) }

/// expected size: 72
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link {
table_start: u32,
caret_value_format: u16,
data: opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data
}

/// expected size: 80
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values {
offset: u16,
link: Option<opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link {
table_start: u32,
caret_count: u16,
caret_values: Vec<opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets {
offset: u16,
link: Option<opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list_link {
table_start: u32,
coverage: opentype_layout_reverse_chain_single_subst_coverage,
lig_glyph_count: u16,
lig_glyph_offsets: Vec<opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets>
}

/// expected size: 96
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_lig_caret_list {
offset: u16,
link: Option<opentype_gdef_table_lig_caret_list_link>
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_mark_glyph_set_coverage {
offset: u32,
link: Option<opentype_coverage_table>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_mark_glyph_set {
table_start: u32,
format: u16,
mark_glyph_set_count: u16,
coverage: Vec<opentype_mark_glyph_set_coverage>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_data_Version1_2_mark_glyph_sets_def {
offset: u16,
link: Option<opentype_mark_glyph_set>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_data_Version1_2 {
mark_glyph_sets_def: opentype_gdef_table_data_Version1_2_mark_glyph_sets_def
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_gdef_table_data_Version1_3 {
mark_glyph_sets_def: opentype_gdef_table_data_Version1_2_mark_glyph_sets_def,
item_var_store: opentype_base_table_item_var_store_offset
}

/// expected size: 56
#[derive(Debug, Clone)]
pub enum opentype_gdef_table_data { Version1_0, Version1_2(opentype_gdef_table_data_Version1_2), Version1_3(opentype_gdef_table_data_Version1_3) }

/// expected size: 368
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

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_common_langsys {
lookup_order_offset: u16,
required_feature_index: u16,
feature_index_count: u16,
feature_indices: Vec<u16>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_common_script_table_default_lang_sys {
offset: u16,
link: Option<opentype_common_langsys>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_common_script_table_lang_sys_records {
lang_sys_tag: u32,
lang_sys: opentype_common_script_table_default_lang_sys
}

/// expected size: 72
#[derive(Debug, Clone)]
pub struct opentype_common_script_table {
table_start: u32,
default_lang_sys: opentype_common_script_table_default_lang_sys,
lang_sys_count: u16,
lang_sys_records: Vec<opentype_common_script_table_lang_sys_records>
}

/// expected size: 80
#[derive(Debug, Clone)]
pub struct opentype_common_script_list_script_records_script {
offset: u16,
link: Option<opentype_common_script_table>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_common_script_list_script_records {
script_tag: u32,
script: opentype_common_script_list_script_records_script
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_common_script_list {
table_start: u32,
script_count: u16,
script_records: Vec<opentype_common_script_list_script_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_gsub_table_script_list {
offset: u16,
link: Option<opentype_common_script_list>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_common_feature_table {
table_start: u32,
feature_params: u16,
lookup_index_count: u16,
lookup_list_indices: Vec<u16>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_common_feature_list_feature_records_feature {
offset: u16,
link: Option<opentype_common_feature_table>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_common_feature_list_feature_records {
feature_tag: u32,
feature: opentype_common_feature_list_feature_records_feature
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_common_feature_list {
table_start: u32,
feature_count: u16,
feature_records: Vec<opentype_common_feature_list_feature_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_gsub_table_feature_list {
offset: u16,
link: Option<opentype_common_feature_list>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_gsub_table_lookup_list_link_lookups_link_lookup_flag {
mark_attachment_class_filter: u16,
use_mark_filtering_set: bool,
ignore_marks: bool,
ignore_ligatures: bool,
ignore_base_glyphs: bool,
right_to_left: bool
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_sequence_lookup {
sequence_index: u16,
lookup_list_index: u16
}

/// expected size: 104
#[derive(Debug, Clone)]
pub struct opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link_chained_seq_rules_link {
backtrack_glyph_count: u16,
backtrack_sequence: Vec<u16>,
input_glyph_count: u16,
input_sequence: Vec<u16>,
lookahead_glyph_count: u16,
lookahead_sequence: Vec<u16>,
seq_lookup_count: u16,
seq_lookup_records: Vec<opentype_common_sequence_lookup>
}

/// expected size: 112
#[derive(Debug, Clone)]
pub struct opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link_chained_seq_rules {
offset: u16,
link: Option<opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link_chained_seq_rules_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link {
table_start: u32,
chained_seq_rule_count: u16,
chained_seq_rules: Vec<opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link_chained_seq_rules>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets {
offset: u16,
link: Option<opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_common_chained_sequence_context_subst_Format1 {
coverage: opentype_layout_reverse_chain_single_subst_coverage,
chained_seq_rule_set_count: u16,
chained_seq_rule_sets: Vec<opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets>
}

/// expected size: 256
#[derive(Debug, Clone)]
pub struct opentype_common_chained_sequence_context_subst_Format2 {
coverage: opentype_layout_reverse_chain_single_subst_coverage,
backtrack_class_def: opentype_gdef_table_glyph_class_def,
input_class_def: opentype_gdef_table_glyph_class_def,
lookahead_class_def: opentype_gdef_table_glyph_class_def,
chained_class_seq_rule_set_count: u16,
chained_class_seq_rule_sets: Vec<opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets>
}

/// expected size: 104
#[derive(Debug, Clone)]
pub struct opentype_common_chained_sequence_context_subst_Format3 {
backtrack_glyph_count: u16,
backtrack_coverages: Vec<opentype_layout_reverse_chain_single_subst_coverage>,
input_glyph_count: u16,
input_coverages: Vec<opentype_layout_reverse_chain_single_subst_coverage>,
lookahead_glyph_count: u16,
lookahead_coverages: Vec<opentype_layout_reverse_chain_single_subst_coverage>,
seq_lookup_count: u16,
seq_lookup_records: Vec<opentype_common_sequence_lookup>
}

/// expected size: 264
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [Noop, DirectHeap, Noop] }, Layout { size: 104, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum opentype_common_chained_sequence_context_subst { Format1(opentype_common_chained_sequence_context_subst_Format1), Format2(opentype_common_chained_sequence_context_subst_Format2), Format3(opentype_common_chained_sequence_context_subst_Format3) }

/// expected size: 272
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [Noop, DirectHeap, Noop] })] }, Layout { size: 112, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct opentype_common_chained_sequence_context {
table_start: u32,
format: u16,
subst: opentype_common_chained_sequence_context_subst
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_anchor_table_table_Format1 {
x_coordinate: u16,
y_coordinate: u16
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_anchor_table_table_Format2 {
x_coordinate: u16,
y_coordinate: u16,
anchor_point: u16
}

/// expected size: 104
#[derive(Debug, Clone)]
pub struct opentype_common_anchor_table_table_Format3 {
x_coordinate: u16,
y_coordinate: u16,
x_device_offset: opentype_common_value_record_x_advance_device,
y_device_offset: opentype_common_value_record_x_advance_device
}

/// expected size: 112
#[derive(Debug, Clone)]
pub enum opentype_common_anchor_table_table { Format1(opentype_common_anchor_table_table_Format1), Format2(opentype_common_anchor_table_table_Format2), Format3(opentype_common_anchor_table_table_Format3) }

/// expected size: 120
#[derive(Debug, Clone)]
pub struct opentype_common_anchor_table {
table_start: u32,
anchor_format: u16,
table: opentype_common_anchor_table_table
}

/// expected size: 128
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_array_mark_records_mark_anchor_offset {
offset: u16,
link: Option<opentype_common_anchor_table>
}

/// expected size: 256
#[derive(Debug, Clone)]
pub struct opentype_layout_cursive_pos_entry_exit_records {
entry_anchor: opentype_layout_mark_array_mark_records_mark_anchor_offset,
exit_anchor: opentype_layout_mark_array_mark_records_mark_anchor_offset
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_layout_cursive_pos {
table_start: u32,
pos_format: u16,
coverage: opentype_layout_reverse_chain_single_subst_coverage,
entry_exit_count: u16,
entry_exit_records: Vec<opentype_layout_cursive_pos_entry_exit_records>
}

/// expected size: 136
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_array_mark_records {
mark_class: u16,
mark_anchor_offset: opentype_layout_mark_array_mark_records_mark_anchor_offset
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_array {
table_start: u32,
mark_count: u16,
mark_records: Vec<opentype_layout_mark_array_mark_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_mark_pos_mark1_array_offset {
offset: u16,
link: Option<opentype_layout_mark_array>
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_base_pos_base_array_offset_link_base_records {
base_anchor_offsets: Vec<opentype_layout_mark_array_mark_records_mark_anchor_offset>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_base_pos_base_array_offset_link {
table_start: u32,
base_count: u16,
base_records: Vec<opentype_layout_mark_base_pos_base_array_offset_link_base_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_base_pos_base_array_offset {
offset: u16,
link: Option<opentype_layout_mark_base_pos_base_array_offset_link>
}

/// expected size: 200
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_base_pos {
table_start: u32,
format: u16,
mark_coverage_offset: opentype_layout_reverse_chain_single_subst_coverage,
base_coverage_offset: opentype_layout_reverse_chain_single_subst_coverage,
mark_class_count: u16,
mark_array_offset: opentype_layout_mark_mark_pos_mark1_array_offset,
base_array_offset: opentype_layout_mark_base_pos_base_array_offset
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets_link_component_records {
ligature_anchor_offsets: Vec<opentype_layout_mark_array_mark_records_mark_anchor_offset>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets_link {
table_start: u32,
component_count: u16,
component_records: Vec<opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets_link_component_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets {
offset: u16,
link: Option<opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_lig_pos_ligature_array_offset_link {
table_start: u32,
ligature_count: u16,
ligature_attach_offsets: Vec<opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_lig_pos_ligature_array_offset {
offset: u16,
link: Option<opentype_layout_mark_lig_pos_ligature_array_offset_link>
}

/// expected size: 200
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_lig_pos {
table_start: u32,
format: u16,
mark_coverage_offset: opentype_layout_reverse_chain_single_subst_coverage,
ligature_coverage_offset: opentype_layout_reverse_chain_single_subst_coverage,
mark_class_count: u16,
mark_array_offset: opentype_layout_mark_mark_pos_mark1_array_offset,
ligature_array_offset: opentype_layout_mark_lig_pos_ligature_array_offset
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_mark_pos_mark2_array_offset_link_mark2_records {
mark2_anchor_offsets: Vec<opentype_layout_mark_array_mark_records_mark_anchor_offset>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_mark_pos_mark2_array_offset_link {
table_start: u32,
mark2_count: u16,
mark2_records: Vec<opentype_layout_mark_mark_pos_mark2_array_offset_link_mark2_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_mark_pos_mark2_array_offset {
offset: u16,
link: Option<opentype_layout_mark_mark_pos_mark2_array_offset_link>
}

/// expected size: 200
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_mark_pos {
table_start: u32,
format: u16,
mark1_coverage_offset: opentype_layout_reverse_chain_single_subst_coverage,
mark2_coverage_offset: opentype_layout_reverse_chain_single_subst_coverage,
mark_class_count: u16,
mark1_array_offset: opentype_layout_mark_mark_pos_mark1_array_offset,
mark2_array_offset: opentype_layout_mark_mark_pos_mark2_array_offset
}

/// expected size: 8
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

/// expected size: 208
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

/// expected size: 424
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_subtable_Format1_pair_sets_link_pair_value_records {
second_glyph: u16,
value_record1: Option<opentype_common_value_record>,
value_record2: Option<opentype_common_value_record>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_subtable_Format1_pair_sets_link {
table_start: u32,
pair_value_count: u16,
pair_value_records: Vec<opentype_layout_pair_pos_subtable_Format1_pair_sets_link_pair_value_records>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_subtable_Format1_pair_sets {
offset: u16,
link: Option<opentype_layout_pair_pos_subtable_Format1_pair_sets_link>
}

/// expected size: 104
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_subtable_Format1 {
coverage: opentype_layout_reverse_chain_single_subst_coverage,
value_format1: opentype_common_value_format_flags,
value_format2: opentype_common_value_format_flags,
pair_set_count: u16,
pair_sets: Vec<opentype_layout_pair_pos_subtable_Format1_pair_sets>
}

/// expected size: 416
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_subtable_Format2_class1_records_class2_records {
value_record1: Option<opentype_common_value_record>,
value_record2: Option<opentype_common_value_record>
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_subtable_Format2_class1_records {
class2_records: Vec<opentype_layout_pair_pos_subtable_Format2_class1_records_class2_records>
}

/// expected size: 216
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_subtable_Format2 {
coverage: opentype_layout_reverse_chain_single_subst_coverage,
value_format1: opentype_common_value_format_flags,
value_format2: opentype_common_value_format_flags,
class_def1: opentype_gdef_table_glyph_class_def,
class_def2: opentype_gdef_table_glyph_class_def,
class1_count: u16,
class2_count: u16,
class1_records: Vec<opentype_layout_pair_pos_subtable_Format2_class1_records>
}

/// expected size: 224
#[derive(Debug, Clone)]
pub enum opentype_layout_pair_pos_subtable { Format1(opentype_layout_pair_pos_subtable_Format1), Format2(opentype_layout_pair_pos_subtable_Format2) }

/// expected size: 232
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos {
table_start: u32,
pos_format: u16,
subtable: opentype_layout_pair_pos_subtable
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_common_sequence_context_subst_Format1_seq_rule_sets_link_rules_link {
glyph_count: u16,
seq_lookup_count: u16,
input_sequence: Vec<u16>,
seq_lookup_records: Vec<opentype_common_sequence_lookup>
}

/// expected size: 64
#[derive(Debug, Clone)]
pub struct opentype_common_sequence_context_subst_Format1_seq_rule_sets_link_rules {
offset: u16,
link: Option<opentype_common_sequence_context_subst_Format1_seq_rule_sets_link_rules_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_common_sequence_context_subst_Format1_seq_rule_sets_link {
table_start: u32,
rule_count: u16,
rules: Vec<opentype_common_sequence_context_subst_Format1_seq_rule_sets_link_rules>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_common_sequence_context_subst_Format1_seq_rule_sets {
offset: u16,
link: Option<opentype_common_sequence_context_subst_Format1_seq_rule_sets_link>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_common_sequence_context_subst_Format1 {
coverage: opentype_layout_reverse_chain_single_subst_coverage,
seq_rule_set_count: u16,
seq_rule_sets: Vec<opentype_common_sequence_context_subst_Format1_seq_rule_sets>
}

/// expected size: 144
#[derive(Debug, Clone)]
pub struct opentype_common_sequence_context_subst_Format2 {
coverage: opentype_layout_reverse_chain_single_subst_coverage,
class_def: opentype_gdef_table_glyph_class_def,
class_seq_rule_set_count: u16,
class_seq_rule_sets: Vec<opentype_common_sequence_context_subst_Format1_seq_rule_sets>
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_common_sequence_context_subst_Format3 {
glyph_count: u16,
seq_lookup_count: u16,
coverage_tables: Vec<opentype_layout_reverse_chain_single_subst_coverage>,
seq_lookup_records: Vec<opentype_common_sequence_lookup>
}

/// expected size: 152
#[derive(Debug, Clone)]
pub enum opentype_common_sequence_context_subst { Format1(opentype_common_sequence_context_subst_Format1), Format2(opentype_common_sequence_context_subst_Format2), Format3(opentype_common_sequence_context_subst_Format3) }

/// expected size: 160
#[derive(Debug, Clone)]
pub struct opentype_common_sequence_context {
table_start: u32,
format: u16,
subst: opentype_common_sequence_context_subst
}

/// expected size: 272
#[derive(Debug, Clone)]
pub struct opentype_layout_single_pos_subtable_Format1 {
coverage_offset: opentype_layout_reverse_chain_single_subst_coverage,
value_format: opentype_common_value_format_flags,
value_record: opentype_common_value_record
}

/// expected size: 96
#[derive(Debug, Clone)]
pub struct opentype_layout_single_pos_subtable_Format2 {
coverage_offset: opentype_layout_reverse_chain_single_subst_coverage,
value_format: opentype_common_value_format_flags,
value_count: u16,
value_records: Vec<opentype_common_value_record>
}

/// expected size: 280
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [DirectHeap, Noop] }, Layout { size: 96, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum opentype_layout_single_pos_subtable { Format1(opentype_layout_single_pos_subtable_Format1), Format2(opentype_layout_single_pos_subtable_Format2) }

/// expected size: 288
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [DirectHeap, Noop] })] }, Layout { size: 104, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct opentype_layout_single_pos {
table_start: u32,
pos_format: u16,
subtable: opentype_layout_single_pos_subtable
}

/// expected size: 296
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [Noop, DirectHeap, Noop] })] })] }, Noop, Noop, Noop, Noop, DirectHeap, Noop, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [DirectHeap, Noop] })] })] }] }, Layout { size: 200, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum opentype_layout_ground_pos { ChainedSequenceContext(opentype_common_chained_sequence_context), CursivePos(opentype_layout_cursive_pos), MarkBasePos(opentype_layout_mark_base_pos), MarkLigPos(opentype_layout_mark_lig_pos), MarkMarkPos(opentype_layout_mark_mark_pos), PairPos(opentype_layout_pair_pos), SequenceContext(opentype_common_sequence_context), SinglePos(opentype_layout_single_pos) }

/// expected size: 304
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, InOption(InDef(InEnum { variants: [InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [Noop, DirectHeap, Noop] })] })] }, Noop, Noop, Noop, Noop, DirectHeap, Noop, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [DirectHeap, Noop] })] })] }] }))] }, Layout { size: 24, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct opentype_layout_pos_extension_extension_offset {
offset: u32,
link: Option<opentype_layout_ground_pos>
}

/// expected size: 312
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, Noop, Noop, InDef(InRecord { fields: [Noop, InOption(InDef(InEnum { variants: [InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [Noop, DirectHeap, Noop] })] })] }, Noop, Noop, Noop, Noop, DirectHeap, Noop, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [DirectHeap, Noop] })] })] }] }))] })] }, Layout { size: 32, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct opentype_layout_pos_extension {
table_start: u32,
format: u16,
extension_lookup_type: u16,
extension_offset: opentype_layout_pos_extension_extension_offset
}

/// expected size: 320
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [DirectHeap, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, Noop, InDef(InRecord { fields: [Noop, InOption(InDef(InEnum { variants: [InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [Noop, DirectHeap, Noop] })] })] }, Noop, Noop, Noop, Noop, DirectHeap, Noop, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [DirectHeap, Noop] })] })] }] }))] })] })] }] }, Layout { size: 32, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum opentype_gpos_table_lookup_list_link_lookups_link_subtables_link { GroundPos(opentype_layout_ground_pos), PosExtension(opentype_layout_pos_extension) }

/// expected size: 328
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, InOption(InDef(InEnum { variants: [DirectHeap, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, Noop, InDef(InRecord { fields: [Noop, InOption(InDef(InEnum { variants: [InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [Noop, DirectHeap, Noop] })] })] }, Noop, Noop, Noop, Noop, DirectHeap, Noop, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [DirectHeap, Noop] })] })] }] }))] })] })] }] }))] }, Layout { size: 24, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link_lookups_link_subtables {
offset: u16,
link: Option<opentype_gpos_table_lookup_list_link_lookups_link_subtables_link>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link_lookups_link {
table_start: u32,
lookup_type: u16,
lookup_flag: opentype_gsub_table_lookup_list_link_lookups_link_lookup_flag,
sub_table_count: u16,
subtables: Vec<opentype_gpos_table_lookup_list_link_lookups_link_subtables>,
mark_filtering_set: Option<u16>
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link_lookups {
offset: u16,
link: Option<opentype_gpos_table_lookup_list_link_lookups_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list_link {
table_start: u32,
lookup_count: u16,
lookups: Vec<opentype_gpos_table_lookup_list_link_lookups>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_gpos_table_lookup_list {
offset: u16,
link: Option<opentype_gpos_table_lookup_list_link>
}

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_feature_variations_feature_variation_records_condition_set_offset_link_condition_offsets_link {
format: u16,
axis_index: u16,
filter_range_min_value: opentype_var_tuple_record_coordinates,
filter_range_max_value: opentype_var_tuple_record_coordinates
}

/// expected size: 20
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_feature_variations_feature_variation_records_condition_set_offset_link_condition_offsets {
offset: u32,
link: Option<opentype_layout_feature_variations_feature_variation_records_condition_set_offset_link_condition_offsets_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_variations_feature_variation_records_condition_set_offset_link {
table_start: u32,
condition_count: u16,
condition_offsets: Vec<opentype_layout_feature_variations_feature_variation_records_condition_set_offset_link_condition_offsets>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_variations_feature_variation_records_condition_set_offset {
offset: u32,
link: Option<opentype_layout_feature_variations_feature_variation_records_condition_set_offset_link>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset_link_substitutions_alternate_feature_offset {
offset: u32,
link: Option<opentype_common_feature_table>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset_link_substitutions {
feature_index: u16,
alternate_feature_offset: opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset_link_substitutions_alternate_feature_offset
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset_link {
table_start: u32,
major_version: u16,
minor_version: u16,
substitution_count: u16,
substitutions: Vec<opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset_link_substitutions>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset {
offset: u32,
link: Option<opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset_link>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_variations_feature_variation_records {
condition_set_offset: opentype_layout_feature_variations_feature_variation_records_condition_set_offset,
feature_table_substitution_offset: opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_variations {
table_start: u32,
major_version: u16,
minor_version: u16,
feature_variation_record_count: u32,
feature_variation_records: Vec<opentype_layout_feature_variations_feature_variation_records>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_gsub_table_feature_variations_offset {
offset: u32,
link: Option<opentype_layout_feature_variations>
}

/// expected size: 176
#[derive(Debug, Clone)]
pub struct opentype_gpos_table {
table_start: u32,
major_version: u16,
minor_version: u16,
script_list: opentype_gsub_table_script_list,
feature_list: opentype_gsub_table_feature_list,
lookup_list: opentype_gpos_table_lookup_list,
feature_variations_offset: Option<opentype_gsub_table_feature_variations_offset>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_alternate_subst_alternate_sets_link {
glyph_count: u16,
alternate_glyph_ids: Vec<u16>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_alternate_subst_alternate_sets {
offset: u16,
link: Option<opentype_layout_alternate_subst_alternate_sets_link>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_layout_alternate_subst {
table_start: u32,
subst_format: u16,
coverage: opentype_layout_reverse_chain_single_subst_coverage,
alternate_set_count: u16,
alternate_sets: Vec<opentype_layout_alternate_subst_alternate_sets>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_ligature_subst_ligature_sets_link_ligatures_link {
ligature_glyph: u16,
component_count: u16,
component_glyph_ids: Vec<u16>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_ligature_subst_ligature_sets_link_ligatures {
offset: u16,
link: Option<opentype_layout_ligature_subst_ligature_sets_link_ligatures_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_ligature_subst_ligature_sets_link {
table_start: u32,
ligature_count: u16,
ligatures: Vec<opentype_layout_ligature_subst_ligature_sets_link_ligatures>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_ligature_subst_ligature_sets {
offset: u16,
link: Option<opentype_layout_ligature_subst_ligature_sets_link>
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_layout_ligature_subst {
table_start: u32,
subst_format: u16,
coverage: opentype_layout_reverse_chain_single_subst_coverage,
ligature_set_count: u16,
ligature_sets: Vec<opentype_layout_ligature_subst_ligature_sets>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_multiple_subst_subst_Format1_sequences_link {
glyph_count: u16,
substitute_glyph_ids: Vec<u16>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_layout_multiple_subst_subst_Format1_sequences {
offset: u16,
link: Option<opentype_layout_multiple_subst_subst_Format1_sequences_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_layout_multiple_subst_subst_Format1 {
sequence_count: u16,
sequences: Vec<opentype_layout_multiple_subst_subst_Format1_sequences>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub enum opentype_layout_multiple_subst_subst { Format1(opentype_layout_multiple_subst_subst_Format1) }

/// expected size: 104
#[derive(Debug, Clone)]
pub struct opentype_layout_multiple_subst {
table_start: u32,
subst_format: u16,
coverage: opentype_layout_reverse_chain_single_subst_coverage,
subst: opentype_layout_multiple_subst_subst
}

/// expected size: 144
#[derive(Debug, Clone)]
pub struct opentype_layout_reverse_chain_single_subst {
table_start: u32,
subst_format: u16,
coverage: opentype_layout_reverse_chain_single_subst_coverage,
backtrack_glyph_count: u16,
backtrack_coverage_tables: Vec<opentype_layout_reverse_chain_single_subst_coverage>,
lookahead_glyph_count: u16,
lookahead_coverage_tables: Vec<opentype_layout_reverse_chain_single_subst_coverage>,
glyph_count: u16,
substitute_glyph_ids: Vec<u16>
}

/// expected size: 64
#[derive(Debug, Clone)]
pub struct opentype_layout_single_subst_subst_Format1 {
coverage: opentype_layout_reverse_chain_single_subst_coverage,
delta_glyph_id: u16
}

/// expected size: 88
#[derive(Debug, Clone)]
pub struct opentype_layout_single_subst_subst_Format2 {
coverage: opentype_layout_reverse_chain_single_subst_coverage,
glyph_count: u16,
substitute_glyph_ids: Vec<u16>
}

/// expected size: 96
#[derive(Debug, Clone)]
pub enum opentype_layout_single_subst_subst { Format1(opentype_layout_single_subst_subst_Format1), Format2(opentype_layout_single_subst_subst_Format2) }

/// expected size: 104
#[derive(Debug, Clone)]
pub struct opentype_layout_single_subst {
table_start: u32,
subst_format: u16,
subst: opentype_layout_single_subst_subst
}

/// expected size: 280
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (NonLocal, Layout { size: 168, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum opentype_layout_ground_subst { AlternateSubst(opentype_layout_alternate_subst), ChainedSequenceContext(opentype_common_chained_sequence_context), LigatureSubst(opentype_layout_ligature_subst), MultipleSubst(opentype_layout_multiple_subst), ReverseChainSingleSubst(opentype_layout_reverse_chain_single_subst), SequenceContext(opentype_common_sequence_context), SingleSubst(opentype_layout_single_subst) }

/// expected size: 288
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, InOption(NonLocal)] }, Layout { size: 24, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct opentype_layout_subst_extension_extension_offset {
offset: u32,
link: Option<opentype_layout_ground_subst>
}

/// expected size: 296
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, Noop, Noop, InDef(InRecord { fields: [Noop, InOption(NonLocal)] })] }, Layout { size: 32, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct opentype_layout_subst_extension {
table_start: u32,
format: u16,
extension_lookup_type: u16,
extension_offset: opentype_layout_subst_extension_extension_offset
}

/// expected size: 304
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [DirectHeap, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, Noop, InDef(InRecord { fields: [Noop, InOption(NonLocal)] })] })] }] }, Layout { size: 32, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum opentype_gsub_table_lookup_list_link_lookups_link_subtables_link { GroundSubst(opentype_layout_ground_subst), SubstExtension(opentype_layout_subst_extension) }

/// expected size: 312
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, InOption(InDef(InEnum { variants: [DirectHeap, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, Noop, InDef(InRecord { fields: [Noop, InOption(NonLocal)] })] })] }] }))] }, Layout { size: 24, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct opentype_gsub_table_lookup_list_link_lookups_link_subtables {
offset: u16,
link: Option<opentype_gsub_table_lookup_list_link_lookups_link_subtables_link>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_gsub_table_lookup_list_link_lookups_link {
table_start: u32,
lookup_type: u16,
lookup_flag: opentype_gsub_table_lookup_list_link_lookups_link_lookup_flag,
sub_table_count: u16,
subtables: Vec<opentype_gsub_table_lookup_list_link_lookups_link_subtables>,
mark_filtering_set: Option<u16>
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_gsub_table_lookup_list_link_lookups {
offset: u16,
link: Option<opentype_gsub_table_lookup_list_link_lookups_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_gsub_table_lookup_list_link {
table_start: u32,
lookup_count: u16,
lookups: Vec<opentype_gsub_table_lookup_list_link_lookups>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_gsub_table_lookup_list {
offset: u16,
link: Option<opentype_gsub_table_lookup_list_link>
}

/// expected size: 176
#[derive(Debug, Clone)]
pub struct opentype_gsub_table {
table_start: u32,
major_version: u16,
minor_version: u16,
script_list: opentype_gsub_table_script_list,
feature_list: opentype_gsub_table_feature_list,
lookup_list: opentype_gsub_table_lookup_list,
feature_variations_offset: Option<opentype_gsub_table_feature_variations_offset>
}

/// expected size: 1
#[derive(Debug, Copy, Clone)]
pub struct opentype_var_variation_axis_record_flags {
hidden_axis: bool
}

/// expected size: 32
#[derive(Debug, Copy, Clone)]
pub struct opentype_var_variation_axis_record {
axis_tag: u32,
min_value: opentype_var_user_tuple_coordinates,
default_value: opentype_var_user_tuple_coordinates,
max_value: opentype_var_user_tuple_coordinates,
flags: opentype_var_variation_axis_record_flags,
axis_name_id: u16
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct opentype_var_user_tuple {
coordinates: Vec<opentype_var_user_tuple_coordinates>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_fvar_table_instances {
subfamily_nameid: u16,
flags: u16,
coordinates: opentype_var_user_tuple,
postscript_nameid: Option<u16>
}

/// expected size: 72
#[derive(Debug, Clone)]
pub struct opentype_fvar_table {
table_start: u32,
major_version: u16,
minor_version: u16,
__offset_axes: u16,
__reserved: u16,
axis_count: u16,
axis_size: u16,
instance_count: u16,
instance_size: u16,
__axes_length: u16,
axes: Vec<opentype_var_variation_axis_record>,
instances: Vec<opentype_fvar_table_instances>
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct opentype_var_tuple_record {
coordinates: Vec<opentype_var_tuple_record_coordinates>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_gvar_table_shared_tuples_offset {
offset: u32,
link: Option<Vec<opentype_var_tuple_record>>
}

/// expected size: 1
#[derive(Debug, Copy, Clone)]
pub struct opentype_gvar_table_flags {
is_long_offset: bool
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct opentype_var_glyph_variation_data_table_tuple_variation_count {
shared_point_numbers: bool,
tuple_count: u16
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct opentype_var_glyph_variation_data_table_tuple_variation_headers_tuple_index {
embedded_peak_tuple: bool,
intermediate_region: bool,
private_point_numbers: bool,
tuple_index: u16
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_var_glyph_variation_data_table_tuple_variation_headers_intermediate_tuples {
start_tuple: opentype_var_tuple_record,
end_tuple: opentype_var_tuple_record
}

/// expected size: 80
#[derive(Debug, Clone)]
pub struct opentype_var_glyph_variation_data_table_tuple_variation_headers {
variation_data_size: u16,
tuple_index: opentype_var_glyph_variation_data_table_tuple_variation_headers_tuple_index,
peak_tuple: Option<opentype_var_tuple_record>,
intermediate_tuples: Option<opentype_var_glyph_variation_data_table_tuple_variation_headers_intermediate_tuples>
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_control {
points_are_words: bool,
point_run_count: u8
}

/// expected size: 32
#[derive(Debug, Clone)]
pub enum opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_points { Points16(Vec<u16>), Points8(Vec<u8>) }

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes {
control: opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_control,
points: opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_points
}

/// expected size: 3
#[derive(Debug, Copy, Clone)]
pub struct opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas_control {
deltas_are_zero: bool,
deltas_are_words: bool,
delta_run_count: u8
}

/// expected size: 32
#[derive(Debug, Clone)]
pub enum opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas { Delta0(u8), Delta16(Vec<u16>), Delta8(Vec<u8>) }

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas {
control: opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas_control,
deltas: opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas
}

/// expected size: 64
#[derive(Debug, Clone)]
pub struct opentype_var_glyph_variation_data_table_data_per_tuple_variation_data {
private_point_numbers: Option<(u16, Vec<opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes>)>,
x_and_y_coordinate_deltas: (u16, Vec<opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas>)
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_var_glyph_variation_data_table_data {
shared_point_numbers: Option<(u16, Vec<opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes>)>,
per_tuple_variation_data: Vec<opentype_var_glyph_variation_data_table_data_per_tuple_variation_data>
}

/// expected size: 96
#[derive(Debug, Clone)]
pub struct opentype_var_glyph_variation_data_table {
table_start: u32,
tuple_variation_count: opentype_var_glyph_variation_data_table_tuple_variation_count,
__data_offset: u16,
tuple_variation_headers: Vec<opentype_var_glyph_variation_data_table_tuple_variation_headers>,
data: opentype_var_glyph_variation_data_table_data
}

/// expected size: 112
#[derive(Debug, Clone)]
pub struct opentype_gvar_table {
gvar_table_start: u32,
major_version: u16,
minor_version: u16,
axis_count: u16,
shared_tuple_count: u16,
shared_tuples_offset: opentype_gvar_table_shared_tuples_offset,
glyph_count: u16,
flags: opentype_gvar_table_flags,
glyph_variation_data_array_offset: u32,
glyph_variation_data_offsets: opentype_gvar_table_glyph_variation_data_offsets,
glyph_variation_data_array: Vec<Option<opentype_var_glyph_variation_data_table>>
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct opentype_kern_table_subtables_coverage {
format: u16,
r#override: bool,
cross_stream: bool,
minimum: bool,
horizontal: bool
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct opentype_kern_table_subtables_data_Format0_kern_pairs {
left: u16,
right: u16,
value: u16
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_kern_table_subtables_data_Format0 {
n_pairs: u16,
search_range: u16,
entry_selector: u16,
range_shift: u16,
kern_pairs: Vec<opentype_kern_table_subtables_data_Format0_kern_pairs>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_kern_table_subtables_data_Format2_left_class_offset_link {
first_glyph: u16,
n_glyphs: u16,
class_values: Vec<u16>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_kern_table_subtables_data_Format2_left_class_offset {
offset: u16,
link: Option<opentype_kern_table_subtables_data_Format2_left_class_offset_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_kern_table_subtables_data_Format2_kerning_array_offset {
offset: u16,
link: Option<Vec<Vec<u16>>>
}

/// expected size: 120
#[derive(Debug, Clone)]
pub struct opentype_kern_table_subtables_data_Format2 {
table_start: u32,
row_width: u16,
left_class_offset: opentype_kern_table_subtables_data_Format2_left_class_offset,
right_class_offset: opentype_kern_table_subtables_data_Format2_left_class_offset,
kerning_array_offset: opentype_kern_table_subtables_data_Format2_kerning_array_offset
}

/// expected size: 128
#[derive(Debug, Clone)]
pub enum opentype_kern_table_subtables_data { Format0(opentype_kern_table_subtables_data_Format0), Format2(opentype_kern_table_subtables_data_Format2) }

/// expected size: 144
#[derive(Debug, Clone)]
pub struct opentype_kern_table_subtables {
version: u16,
length: u16,
coverage: opentype_kern_table_subtables_coverage,
data: opentype_kern_table_subtables_data
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_kern_table {
version: u16,
n_tables: u16,
subtables: Vec<opentype_kern_table_subtables>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_design_axes_offset_link_design_axes {
axis_tag: u32,
axis_name_id: u16,
axis_ordering: u16
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct opentype_stat_table_design_axes_offset_link {
design_axes: Vec<opentype_stat_table_design_axes_offset_link_design_axes>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_stat_table_design_axes_offset {
offset: u32,
link: Option<opentype_stat_table_design_axes_offset_link>
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags {
elidable_axis_value_name: bool,
older_sibling_font_attribute: bool
}

/// expected size: 16
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1 {
axis_index: u16,
flags: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags,
value_name_id: u16,
value: opentype_var_user_tuple_coordinates
}

/// expected size: 32
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format2 {
axis_index: u16,
flags: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags,
value_name_id: u16,
nominal_value: opentype_var_user_tuple_coordinates,
range_min_value: opentype_var_user_tuple_coordinates,
range_max_value: opentype_var_user_tuple_coordinates
}

/// expected size: 24
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format3 {
axis_index: u16,
flags: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags,
value_name_id: u16,
value: opentype_var_user_tuple_coordinates,
linked_value: opentype_var_user_tuple_coordinates
}

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format4_axis_values {
axis_index: u16,
value: opentype_var_user_tuple_coordinates
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
pub enum opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data { Format1(opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1), Format2(opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format2), Format3(opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format3), Format4(opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format4) }

/// expected size: 48
#[derive(Debug, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link {
format: u16,
data: opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets {
offset: u16,
link: Option<opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets_link {
table_start: u32,
axis_value_offsets: Vec<opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_stat_table_offset_to_axis_value_offsets {
offset: u32,
link: Option<opentype_stat_table_offset_to_axis_value_offsets_link>
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
elided_fallback_name_id: u16
}

/// expected size: 1976
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
base: Option<opentype_base_table>,
gdef: Option<opentype_gdef_table>,
gpos: Option<opentype_gpos_table>,
gsub: Option<opentype_gsub_table>,
fvar: Option<opentype_fvar_table>,
gvar: Option<opentype_gvar_table>,
kern: Option<opentype_kern_table>,
stat: Option<opentype_stat_table>,
vhea: Option<opentype_hhea_table>,
vmtx: Option<opentype_hmtx_table>,
__skip: ()
}

/// expected size: 2016
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

/// expected size: 2024
#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version1_table_directories {
offset: u32,
link: Option<opentype_table_directory>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version1 {
num_fonts: u32,
table_directories: Vec<opentype_ttc_header_header_Version1_table_directories>
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version2 {
num_fonts: u32,
table_directories: Vec<opentype_ttc_header_header_Version1_table_directories>,
dsig_tag: u32,
dsig_length: u32,
dsig_offset: u32
}

/// expected size: 48
#[derive(Debug, Clone)]
pub enum opentype_ttc_header_header { UnknownVersion(u16), Version1(opentype_ttc_header_header_Version1), Version2(opentype_ttc_header_header_Version2) }

/// expected size: 56
#[derive(Debug, Clone)]
pub struct opentype_ttc_header {
ttc_tag: u32,
major_version: u16,
minor_version: u16,
header: opentype_ttc_header_header,
__skip: ()
}

/// expected size: 2024
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [Noop, DirectHeap] }, Layout { size: 56, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum opentype_main_directory { TTCHeader(opentype_ttc_header), TableDirectory(opentype_table_directory) }

/// expected size: 2032
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [Noop, DirectHeap] })] }, Layout { size: 64, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct opentype_main {
file_start: u32,
magic: u32,
directory: opentype_main_directory
}

/// expected size: 16
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

/// expected size: 28
#[derive(Debug, Copy, Clone)]
pub struct png_ihdr {
length: u32,
tag: (u8, u8, u8, u8),
data: png_ihdr_data,
crc: u32
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct png_bkgd_color_type_0 {
greyscale: u16
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct png_bkgd_color_type_2 {
red: u16,
green: u16,
blue: u16
}

/// expected size: 1
#[derive(Debug, Copy, Clone)]
pub struct png_bkgd_color_type_3 {
palette_index: u8
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub enum png_bkgd { color_type_0(png_bkgd_color_type_0), color_type_2(png_bkgd_color_type_2), color_type_3(png_bkgd_color_type_3), color_type_4(png_bkgd_color_type_0), color_type_6(png_bkgd_color_type_2) }

/// expected size: 32
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

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct png_gama {
gamma: u32
}

/// expected size: 24
#[derive(Debug, Clone)]
pub struct png_hist {
histogram: Vec<u16>
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct zlib_main_compression_method_flags {
compression_info: u8,
compression_method: u8
}

/// expected size: 3
#[derive(Debug, Copy, Clone)]
pub struct zlib_main_flags {
flevel: u8,
fdict: bool,
fcheck: u8
}

/// expected size: 96
#[derive(Debug, Clone)]
pub struct zlib_main {
compression_method_flags: zlib_main_compression_method_flags,
flags: zlib_main_flags,
dict_id: Option<u32>,
data: deflate_main,
adler32: u32
}

/// expected size: 128
#[derive(Debug, Clone)]
pub struct png_iccp {
profile_name: Vec<u8>,
compression_method: u8,
compressed_profile: zlib_main
}

/// expected size: 32
#[derive(Debug, Clone)]
pub enum png_itxt_text_compressed { invalid(Vec<u8>), valid(Vec<char>) }

/// expected size: 40
#[derive(Debug, Clone)]
pub enum png_itxt_text { compressed(png_itxt_text_compressed), uncompressed(Vec<char>) }

/// expected size: 128
#[derive(Debug, Clone)]
pub struct png_itxt {
keyword: Vec<u8>,
compression_flag: u8,
compression_method: u8,
language_tag: base_asciiz_string,
translated_keyword: Vec<char>,
text: png_itxt_text
}

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct png_phys {
pixels_per_unit_x: u32,
pixels_per_unit_y: u32,
unit_specifier: u8
}

/// expected size: 1
#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_0 {
sig_greyscale_bits: u8
}

/// expected size: 3
#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_2 {
sig_red_bits: u8,
sig_green_bits: u8,
sig_blue_bits: u8
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_4 {
sig_greyscale_bits: u8,
sig_alpha_bits: u8
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_6 {
sig_red_bits: u8,
sig_green_bits: u8,
sig_blue_bits: u8,
sig_alpha_bits: u8
}

/// expected size: 5
#[derive(Debug, Copy, Clone)]
pub enum png_sbit { color_type_0(png_sbit_color_type_0), color_type_2(png_sbit_color_type_2), color_type_3(png_sbit_color_type_2), color_type_4(png_sbit_color_type_4), color_type_6(png_sbit_color_type_6) }

/// expected size: 10
#[derive(Debug, Copy, Clone)]
pub struct png_splt_pallette_sample_depth_u16 {
red: u16,
green: u16,
blue: u16,
alpha: u16,
frequency: u16
}

/// expected size: 6
#[derive(Debug, Copy, Clone)]
pub struct png_splt_pallette_sample_depth_u8 {
red: u8,
green: u8,
blue: u8,
alpha: u8,
frequency: u16
}

/// expected size: 32
#[derive(Debug, Clone)]
pub enum png_splt_pallette { sample_depth_u16(Vec<png_splt_pallette_sample_depth_u16>), sample_depth_u8(Vec<png_splt_pallette_sample_depth_u8>) }

/// expected size: 64
#[derive(Debug, Clone)]
pub struct png_splt {
palette_name: Vec<u8>,
sample_depth: u8,
pallette: png_splt_pallette
}

/// expected size: 1
#[derive(Debug, Copy, Clone)]
pub struct png_srgb {
rendering_intent: u8
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct png_text {
keyword: Vec<u8>,
text: Vec<u8>
}

/// expected size: 8
#[derive(Debug, Copy, Clone)]
pub struct png_time {
year: u16,
month: u8,
day: u8,
hour: u8,
minute: u8,
second: u8
}

/// expected size: 32
#[derive(Debug, Clone)]
pub enum png_trns { color_type_0(png_bkgd_color_type_0), color_type_2(png_bkgd_color_type_2), color_type_3(Vec<png_bkgd_color_type_3>) }

/// expected size: 56
#[derive(Debug, Clone)]
pub struct png_ztxt {
keyword: Vec<u8>,
compression_method: u8,
compressed_text: Vec<char>
}

/// expected size: 136
#[derive(Debug, Clone)]
pub enum png_chunk_data { PLTE(Vec<png_plte>), bKGD(png_bkgd), cHRM(png_chrm), gAMA(png_gama), hIST(png_hist), iCCP(png_iccp), iTXt(png_itxt), pHYs(png_phys), sBIT(png_sbit), sPLT(png_splt), sRGB(png_srgb), tEXt(png_text), tIME(png_time), tRNS(png_trns), unknown(Vec<u8>), zTXt(png_ztxt) }

/// expected size: 168
#[derive(Debug, Clone)]
pub struct png_chunk {
length: u32,
tag: Vec<u8>,
data: png_chunk_data,
crc: u32
}

/// expected size: 12
#[derive(Debug, Copy, Clone)]
pub struct png_iend {
length: u32,
tag: (u8, u8, u8, u8),
data: (),
crc: u32
}

/// expected size: 192
#[derive(Debug, Clone)]
pub struct png_main {
signature: (u8, u8, u8, u8, u8, u8, u8, u8),
ihdr: png_ihdr,
chunks: Vec<png_chunk>,
idat: zlib_main,
more_chunks: Vec<png_chunk>,
iend: png_iend
}

/// expected size: 40
#[derive(Debug, Clone)]
pub struct riff_chunk {
tag: (u8, u8, u8, u8),
length: u32,
data: Vec<u8>,
pad: Option<u8>
}

/// expected size: 32
#[derive(Debug, Clone)]
pub struct riff_subchunks {
tag: (u8, u8, u8, u8),
chunks: Vec<riff_chunk>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct riff_main {
tag: (u8, u8, u8, u8),
length: u32,
data: riff_subchunks,
pad: Option<u8>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct tar_ascii_string_opt0 {
string: Vec<u8>,
__padding: Vec<u8>
}

/// expected size: 56
#[derive(Debug, Clone)]
pub struct tar_header_uid {
string: Vec<u8>,
__nul_or_wsp: u8,
__padding: Vec<u8>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct tar_ascii_string {
string: Vec<u8>,
padding: Vec<u8>
}

/// expected size: 672
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

/// expected size: 696
#[derive(Debug, Clone)]
pub struct tar_header_with_data {
header: tar_header,
file: Vec<u8>,
__padding: ()
}

/// expected size: 72
#[derive(Debug, Clone)]
pub struct tar_main {
contents: Vec<tar_header_with_data>,
__padding: Vec<u8>,
__trailing: Vec<u8>
}

/// expected size: 48
#[derive(Debug, Clone)]
pub struct waldo_main {
r#where: u64,
noise: Vec<u8>,
sep: u8,
here: u64,
waldo: (u8, u8, u8, u8, u8),
__rem: ()
}

/// expected size: 2040
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [DirectHeap, Noop, Noop, DirectHeap, Noop, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [Noop, DirectHeap] })] })] }, Noop, DirectHeap, Noop, Noop, Noop, Noop, Noop, Noop] }, Layout { size: 104, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub enum main_data { elf(elf_main), gif(gif_main), gzip(Vec<gzip_main>), jpeg(jpeg_main), mpeg4(mpeg4_main), opentype(opentype_main), peano(Vec<u32>), png(png_main), riff(riff_main), tar(tar_main), text(Vec<char>), tgz(Vec<tar_main>), tiff(tiff_main), waldo(waldo_main) }

/// expected size: 16
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

/// expected size: 40
#[derive(Debug, Clone)]
pub struct png_idat {
length: u32,
tag: (u8, u8, u8, u8),
data: Vec<u8>,
crc: u32
}

/// expected size: 2
#[derive(Debug, Copy, Clone)]
pub struct opentype_cmap_subtable_format14_raw_raw {
format: u16
}

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct opentype_cmap_subtable_format13_raw_raw {
format: u16,
__reserved: u16
}

/// expected size: 7
#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_simple_flags_raw {
repeats: u8,
field_set: opentype_glyf_simple_flags
}

/// expected size: 7
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

/// expected size: 4
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_device_or_variation_index_table_raw_raw {
__skipped0: u16,
__skipped1: u16
}

/// expected size: 2040
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [InDef(InEnum { variants: [DirectHeap, Noop, Noop, DirectHeap, Noop, InTuple { pos: [InDef(InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [Noop, DirectHeap] })] })] }, Noop, DirectHeap, Noop, Noop, Noop, Noop, Noop, Noop] }), Noop] }, Layout { size: 104, align: 8 (1 << 3) })
#[derive(Debug, Clone)]
pub struct main {
data: main_data,
end: ()
}

fn Decoder_main<>(_input: &mut Parser<'_>) -> Result<main, ParseError> {
Decoder1(_input)
}

fn Decoder1<>(_input: &mut Parser<'_>) -> Result<main, ParseError> {
let data = ((|| {
_input.start_alt();
let res = (|| {
let inner = (Decoder_waldo_main(_input))?;
PResult::Ok(main_data::waldo(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder3(_input))?;
PResult::Ok(main_data::peano(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder_gif_main(_input))?;
PResult::Ok(main_data::gif(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder5(_input))?;
PResult::Ok(main_data::tgz(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder6(_input))?;
PResult::Ok(main_data::gzip(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder_jpeg_main(_input))?;
PResult::Ok(main_data::jpeg(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder_mpeg4_main(_input))?;
PResult::Ok(main_data::mpeg4(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder_png_main(_input))?;
PResult::Ok(main_data::png(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder_riff_main(_input))?;
PResult::Ok(main_data::riff(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder_tiff_main(_input))?;
PResult::Ok(main_data::tiff(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder_tar_main(_input))?;
PResult::Ok(main_data::tar(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder_elf_main(_input))?;
PResult::Ok(main_data::elf(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder_opentype_main(_input))?;
PResult::Ok(main_data::opentype(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(true)?;
}
};
let res = (|| {
let inner = (Decoder15(_input))?;
PResult::Ok(main_data::text(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
return Err(_e);
}
};
})())?;
let end = _input.finish()?;
PResult::Ok(main { data, end })
}

fn Decoder_waldo_main<>(_input: &mut Parser<'_>) -> Result<waldo_main, ParseError> {
let r#where = (Decoder99(_input))?;
let noise = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
255u8 => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(11599300513837427027u64));
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
return Err(ParseError::ExcludedBranch(7620281735474506525u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let sep = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(950046280632689001u64));
})
})())?;
let here = _input.get_offset_u64();
let waldo = ((|| {
let tgt_offset = here + (try_sub!(r#where, here, 13646096770106105413u64));
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 87 {
b
} else {
return Err(ParseError::ExcludedBranch(5625702265340316943u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 97 {
b
} else {
return Err(ParseError::ExcludedBranch(4260205764162136487u64));
})
})())?;
let field2 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 108 {
b
} else {
return Err(ParseError::ExcludedBranch(5712308626808297759u64));
})
})())?;
let field3 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 100 {
b
} else {
return Err(ParseError::ExcludedBranch(8638089167112501923u64));
})
})())?;
let field4 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 111 {
b
} else {
return Err(ParseError::ExcludedBranch(1453530207670075215u64));
})
})())?;
PResult::Ok((field0, field1, field2, field3, field4))
})())?;
_input.close_peek_context()?;
PResult::Ok(ret)
})())?;
let __rem = _input.skip_remainder();
PResult::Ok(waldo_main { r#where, noise, sep, here, waldo, __rem })
}

fn Decoder3<>(_input: &mut Parser<'_>) -> Result<Vec<u32>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
90u8 => {
1
},

83u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(15915510438164744429u64));
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
let next_elem = (Decoder328(_input))?;
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder_gif_main<>(_input: &mut Parser<'_>) -> Result<gif_main, ParseError> {
let header = (Decoder_gif_header(_input))?;
let logical_screen = (Decoder_gif_logical_screen(_input))?;
let blocks = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(1542992798780655146u64));
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
PResult::Ok(accum)
})())?;
let trailer = (Decoder_gif_trailer(_input))?;
PResult::Ok(gif_main { header, logical_screen, blocks, trailer })
}

fn Decoder5<>(_input: &mut Parser<'_>) -> Result<Vec<tar_main>, ParseError> {
let gzip_raw = (Decoder305(_input))?;
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

fn Decoder6<>(_input: &mut Parser<'_>) -> Result<Vec<gzip_main>, ParseError> {
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
let header = (Decoder_gzip_header(_input))?;
let fextra = ((|| {
let ret = if header.file_flags.fextra.clone() {
Some((Decoder_gzip_fextra(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let fname = ((|| {
let ret = if header.file_flags.fname.clone() {
Some((Decoder298(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let fcomment = ((|| {
let ret = if header.file_flags.fcomment.clone() {
Some((Decoder_gzip_fcomment(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let fhcrc = ((|| {
let ret = if header.file_flags.fhcrc.clone() {
Some((Decoder_gzip_fhcrc(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let data = ((|| {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
PResult::Ok(ret)
})())?;
let footer = (Decoder_gzip_footer(_input))?;
gzip_main { header, fextra, fname, fcomment, fhcrc, data, footer }
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder_jpeg_main<>(_input: &mut Parser<'_>) -> Result<jpeg_main, ParseError> {
let soi = (Decoder_jpeg_eoi(_input))?;
let frame = (Decoder_jpeg_frame(_input))?;
let eoi = (Decoder227(_input))?;
PResult::Ok(jpeg_main { soi, frame, eoi })
}

fn Decoder_mpeg4_main<>(_input: &mut Parser<'_>) -> Result<mpeg4_main, ParseError> {
let atoms = ((|| {
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
PResult::Ok(accum)
})())?;
PResult::Ok(mpeg4_main { atoms })
}

fn Decoder_png_main<>(_input: &mut Parser<'_>) -> Result<png_main, ParseError> {
let signature = (Decoder156(_input))?;
let ihdr = (Decoder_png_ihdr(_input))?;
let chunks = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
{
let ret = match _input.read_byte()? {
73u8 => {
let b = _input.read_byte()?;
if b == 68 {
1
} else {
return Err(ParseError::ExcludedBranch(7028560493922100069u64));
}
},

byte if ((ByteSet::from_bits([18446744069414594048, 18446744073709551103, 0, 0])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(2649783168072194737u64));
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
PResult::Ok(accum)
})())?;
let idat = ((|| {
let idat = {
let xs = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
{
let ret = match _input.read_byte()? {
73u8 => {
match _input.read_byte()? {
69u8 => {
0
},

68u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(8253205784254894771u64));
}
}
},

byte if ((ByteSet::from_bits([18446744069414594048, 18446744073709551103, 0, 0])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(1225514472166157741u64));
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
((|xs: Vec<png_idat>| try_flat_map_vec(xs.iter().cloned(), |x: png_idat| PResult::Ok(x.data.clone())))(xs))?
};
let mut tmp = Parser::new(idat.as_slice());
let reparser = &mut tmp;
Decoder_zlib_main(reparser)
})())?;
let more_chunks = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
_input.read_byte()?;
{
let ret = match _input.read_byte()? {
73u8 => {
let b = _input.read_byte()?;
if b == 69 {
1
} else {
return Err(ParseError::ExcludedBranch(1224415506115142500u64));
}
},

byte if ((ByteSet::from_bits([18446744069414594048, 18446744073709551103, 0, 0])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(16859485491091215361u64));
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
PResult::Ok(accum)
})())?;
let iend = (Decoder_png_iend(_input))?;
PResult::Ok(png_main { signature, ihdr, chunks, idat, more_chunks, iend })
}

fn Decoder_riff_main<>(_input: &mut Parser<'_>) -> Result<riff_main, ParseError> {
let tag = ((|| {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 82 {
b
} else {
return Err(ParseError::ExcludedBranch(14898840355839773829u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(9453951600195794313u64));
})
})())?;
let field2 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(10036157788440812915u64));
})
})())?;
let field3 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(6349531732377484771u64));
})
})())?;
PResult::Ok((field0, field1, field2, field3))
})())?;
let length = (Decoder127(_input))?;
let data = ((|| {
let sz = length as usize;
_input.start_slice(sz)?;
let ret = (Decoder_riff_subchunks(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let pad = ((|| {
let ret = if length % 2u32 == 1u32 {
let b = _input.read_byte()?;
Some(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(13785646910930464515u64));
})
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(riff_main { tag, length, data, pad })
}

fn Decoder_tiff_main<>(_input: &mut Parser<'_>) -> Result<tiff_main, ParseError> {
let start_of_header = {
let x = _input.get_offset_u64();
x as u32
};
let byte_order = ((|| {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
73u8 => {
0
},

77u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(15116592996336247086u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(5323644471994966730u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(13278122992382147879u64));
})
})())?;
tiff_main_byte_order::le(field0, field1)
},

1 => {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 77 {
b
} else {
return Err(ParseError::ExcludedBranch(18159646757349796721u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 77 {
b
} else {
return Err(ParseError::ExcludedBranch(7124606020426797957u64));
})
})())?;
tiff_main_byte_order::be(field0, field1)
},

_ => {
return Err(ParseError::ExcludedBranch(10346499338674982396u64));
}
};
PResult::Ok(ret)
})())?;
let magic = ((|| {
let ret = match byte_order {
tiff_main_byte_order::le(..) => {
(Decoder139(_input))?
},

tiff_main_byte_order::be(..) => {
(Decoder23(_input))?
}
};
PResult::Ok(ret)
})())?;
let offset = ((|| {
let ret = match byte_order {
tiff_main_byte_order::le(..) => {
(Decoder127(_input))?
},

tiff_main_byte_order::be(..) => {
(Decoder20(_input))?
}
};
PResult::Ok(ret)
})())?;
let ifd = ((|| {
let tgt_offset = start_of_header + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let ret = match byte_order {
tiff_main_byte_order::le(..) => {
let num_fields = (Decoder139(_input))?;
let fields = ((|| {
let mut accum = Vec::new();
for _ in 0..num_fields {
accum.push({
let tag = (Decoder139(_input))?;
let r#type = (Decoder139(_input))?;
let length = (Decoder127(_input))?;
let offset_or_data = (Decoder127(_input))?;
tiff_main_ifd_fields { tag, r#type, length, offset_or_data }
});
}
PResult::Ok(accum)
})())?;
let next_ifd_offset = (Decoder127(_input))?;
let next_ifd = ((|| {
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
PResult::Ok(accum)
})())?;
tiff_main_ifd { num_fields, fields, next_ifd_offset, next_ifd }
},

tiff_main_byte_order::be(..) => {
let num_fields = (Decoder23(_input))?;
let fields = ((|| {
let mut accum = Vec::new();
for _ in 0..num_fields {
accum.push({
let tag = (Decoder23(_input))?;
let r#type = (Decoder23(_input))?;
let length = (Decoder20(_input))?;
let offset_or_data = (Decoder20(_input))?;
tiff_main_ifd_fields { tag, r#type, length, offset_or_data }
});
}
PResult::Ok(accum)
})())?;
let next_ifd_offset = (Decoder20(_input))?;
let next_ifd = ((|| {
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
PResult::Ok(accum)
})())?;
tiff_main_ifd { num_fields, fields, next_ifd_offset, next_ifd }
}
};
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
PResult::Ok(ret)
})())?;
PResult::Ok(tiff_main { start_of_header, byte_order, magic, offset, ifd })
}

fn Decoder_tar_main<>(_input: &mut Parser<'_>) -> Result<tar_main, ParseError> {
let contents = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if (byte != 0) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(10951432197815892834u64));
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
PResult::Ok(accum)
})())?;
let __padding = ((|| {
let mut accum = Vec::new();
for _ in 0..1024u32 {
accum.push({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(14864597187136898256u64));
}
});
}
PResult::Ok(accum)
})())?;
let __trailing = ((|| {
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
return Err(ParseError::ExcludedBranch(15238960955167157760u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_main { contents, __padding, __trailing })
}

fn Decoder_elf_main<>(_input: &mut Parser<'_>) -> Result<elf_main, ParseError> {
let header = (Decoder_elf_header(_input))?;
let __eoh = _input.get_offset_u64();
let program_headers = ((|| {
let ret = if match header.phoff.clone() {
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
let tgt_offset = 0u64 + match header.phoff.clone() {
elf_types_elf_off::Off32(x32) => {
x32 as u64
},

elf_types_elf_off::Off64(x64) => {
x64
}
};
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder114(_input, header.ident.data.clone() == 2u8, header.ident.class.clone(), header.phnum.clone()))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
PResult::Ok(ret)
})())?;
let section_headers = ((|| {
let ret = if match header.shoff.clone() {
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
let tgt_offset = 0u64 + match header.shoff.clone() {
elf_types_elf_off::Off32(x32) => {
x32 as u64
},

elf_types_elf_off::Off64(x64) => {
x64
}
};
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder115(_input, header.ident.data.clone() == 2u8, header.ident.class.clone(), header.shnum.clone()))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
PResult::Ok(ret)
})())?;
let sections = ((|| {
let ret = match section_headers {
Some(ref shdrs) => {
let val = {
let mut accum = Vec::new();
for shdr in shdrs.clone() {
accum.push({
let ret = if (shdr.r#type.clone() != 8u32) && (shdr.r#type.clone() != 0u32) {
let tgt_offset = 0u64 + match shdr.offset.clone() {
elf_types_elf_off::Off32(x32) => {
x32 as u64
},

elf_types_elf_off::Off64(x64) => {
x64
}
};
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder116(_input, shdr.r#type.clone(), match shdr.size.clone() {
elf_types_elf_full::Full32(x32) => {
x32 as u64
},

elf_types_elf_full::Full64(x64) => {
x64.clone()
}
}))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
});
}
accum
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let __skip = _input.skip_remainder();
PResult::Ok(elf_main { header, __eoh, program_headers, section_headers, sections, __skip })
}

fn Decoder_opentype_main<>(_input: &mut Parser<'_>) -> Result<opentype_main, ParseError> {
let file_start = {
let x = _input.get_offset_u64();
x as u32
};
let magic = ((|| {
_input.open_peek_context();
let ret = (Decoder20(_input))?;
_input.close_peek_context()?;
PResult::Ok(ret)
})())?;
let directory = ((|| {
let ret = match magic {
65536u32 => {
let inner = (Decoder_opentype_table_directory(_input, file_start.clone()))?;
opentype_main_directory::TableDirectory(inner)
},

1330926671u32 => {
let inner = (Decoder_opentype_table_directory(_input, file_start.clone()))?;
opentype_main_directory::TableDirectory(inner)
},

1953784678u32 => {
let inner = (Decoder_opentype_ttc_header(_input, file_start.clone()))?;
opentype_main_directory::TTCHeader(inner)
},

1953658213u32 => {
let inner = (Decoder_opentype_table_directory(_input, file_start.clone()))?;
opentype_main_directory::TableDirectory(inner)
},

_ => {
return Err(ParseError::FailToken(17527274083228188873u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_main { file_start, magic, directory })
}

fn Decoder15<>(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
Decoder16(_input)
}

fn Decoder16<>(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(byte)) => {
0
},

224u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(byte)) => {
0
},

237u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(byte)) => {
0
},

240u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(byte)) => {
0
},

244u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(17855530393917176367u64));
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

fn Decoder17<>(_input: &mut Parser<'_>) -> Result<char, ParseError> {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(byte)) => {
1
},

byte if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(byte)) => {
1
},

224u8 => {
1
},

byte if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(byte)) => {
1
},

237u8 => {
1
},

byte if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(byte)) => {
1
},

240u8 => {
1
},

byte if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(byte)) => {
1
},

244u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(17406968167054271466u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let _ = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(11054356281452530428u64));
}
};
(char::from_u32(0u32)).unwrap()
},

1 => {
(Decoder18(_input))?
},

_ => {
return Err(ParseError::ExcludedBranch(11100042044514704042u64));
}
};
PResult::Ok(ret)
}

fn Decoder18<>(_input: &mut Parser<'_>) -> Result<char, ParseError> {
let codepoint = {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(byte)) => {
1
},

224u8 => {
2
},

byte if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(byte)) => {
2
},

237u8 => {
2
},

byte if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(byte)) => {
2
},

240u8 => {
3
},

byte if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(byte)) => {
3
},

244u8 => {
3
},

_ => {
return Err(ParseError::ExcludedBranch(12728843535195535635u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let byte = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(5409189036752851054u64));
}
};
byte as u32
},

1 => {
let tuple_var = {
let field0 = ((|| {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 0, 4294967292])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(4726315105662630465u64));
}
};
PResult::Ok(raw & 31u8)
})())?;
let field1 = (Decoder19(_input))?;
(field0, field1)
};
match tuple_var {
(x1, x0) => {
(x1 as u32) << 6u32 | (x0 as u32)
}
}
},

2 => {
let tuple_var = {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
224u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(byte)) => {
1
},

237u8 => {
2
},

byte if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(byte)) => {
3
},

_ => {
return Err(ParseError::ExcludedBranch(15631554783732883240u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let field0 = ((|| {
let raw = {
let b = _input.read_byte()?;
if b == 224 {
b
} else {
return Err(ParseError::ExcludedBranch(10036638040555853769u64));
}
};
PResult::Ok(raw & 15u8)
})())?;
let field1 = ((|| {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 18446744069414584320, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(13230337088401352826u64));
}
};
PResult::Ok(raw & 63u8)
})())?;
let field2 = (Decoder19(_input))?;
(field0, field1, field2)
},

1 => {
let field0 = ((|| {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(11460567998186064482u64));
}
};
PResult::Ok(raw & 15u8)
})())?;
let field1 = (Decoder19(_input))?;
let field2 = (Decoder19(_input))?;
(field0, field1, field2)
},

2 => {
let field0 = ((|| {
let raw = {
let b = _input.read_byte()?;
if b == 237 {
b
} else {
return Err(ParseError::ExcludedBranch(6223008304848233301u64));
}
};
PResult::Ok(raw & 15u8)
})())?;
let field1 = ((|| {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 4294967295, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(14550754927305275517u64));
}
};
PResult::Ok(raw & 63u8)
})())?;
let field2 = (Decoder19(_input))?;
(field0, field1, field2)
},

3 => {
let field0 = ((|| {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(10197098993763395417u64));
}
};
PResult::Ok(raw & 15u8)
})())?;
let field1 = (Decoder19(_input))?;
let field2 = (Decoder19(_input))?;
(field0, field1, field2)
},

_ => {
return Err(ParseError::ExcludedBranch(2391834656526534993u64));
}
};
ret
};
match tuple_var {
(x2, x1, x0) => {
(x2 as u32) << 12u32 | (x1 as u32) << 6u32 | (x0 as u32)
}
}
},

3 => {
let tuple_var = {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
240u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(byte)) => {
1
},

244u8 => {
2
},

_ => {
return Err(ParseError::ExcludedBranch(658824046370133753u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let field0 = ((|| {
let raw = {
let b = _input.read_byte()?;
if b == 240 {
b
} else {
return Err(ParseError::ExcludedBranch(9422510723961972169u64));
}
};
PResult::Ok(raw & 7u8)
})())?;
let field1 = ((|| {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 18446744073709486080, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(10940017698627680568u64));
}
};
PResult::Ok(raw & 63u8)
})())?;
let field2 = (Decoder19(_input))?;
let field3 = (Decoder19(_input))?;
(field0, field1, field2, field3)
},

1 => {
let field0 = ((|| {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(179268011689651936u64));
}
};
PResult::Ok(raw & 7u8)
})())?;
let field1 = (Decoder19(_input))?;
let field2 = (Decoder19(_input))?;
let field3 = (Decoder19(_input))?;
(field0, field1, field2, field3)
},

2 => {
let field0 = ((|| {
let raw = {
let b = _input.read_byte()?;
if b == 244 {
b
} else {
return Err(ParseError::ExcludedBranch(9665974566873665536u64));
}
};
PResult::Ok(raw & 7u8)
})())?;
let field1 = ((|| {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 65535, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(374064178837027275u64));
}
};
PResult::Ok(raw & 63u8)
})())?;
let field2 = (Decoder19(_input))?;
let field3 = (Decoder19(_input))?;
(field0, field1, field2, field3)
},

_ => {
return Err(ParseError::ExcludedBranch(3725673472712527969u64));
}
};
ret
};
match tuple_var {
(x3, x2, x1, x0) => {
(x3 as u32) << 18u32 | (x2 as u32) << 12u32 | (x1 as u32) << 6u32 | (x0 as u32)
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15741082764016749161u64));
}
};
ret
};
PResult::Ok((char::from_u32(codepoint)).unwrap())
}

fn Decoder19<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 18446744073709551615, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9967703502401950260u64));
}
};
PResult::Ok(raw & 63u8)
}

fn Decoder20<>(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
let x = {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
let field3 = (Decoder24(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(u32be(x))
}

fn Decoder_opentype_table_directory<>(_input: &mut Parser<'_>, font_start: u32) -> Result<opentype_table_directory, ParseError> {
let sfnt_version = ((|| {
let inner = (Decoder20(_input))?;
let is_valid = {
let version = inner;
match version {
65536u32 => {
true
},

1330926671u32 => {
true
},

1953658213u32 => {
true
},

_ => {
false
}
}
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8376883036401934317u64));
};
PResult::Ok(ret)
})())?;
let num_tables = (Decoder23(_input))?;
let search_range = (Decoder23(_input))?;
let entry_selector = (Decoder23(_input))?;
let range_shift = (Decoder23(_input))?;
let table_records = ((|| {
let mut accum = Vec::new();
for _ in 0..num_tables {
accum.push((Decoder_opentype_table_record(_input))?);
}
PResult::Ok(accum)
})())?;
let table_links = (Decoder_opentype_table_directory_table_links(_input, font_start.clone(), &table_records))?;
PResult::Ok(opentype_table_directory { sfnt_version, num_tables, search_range, entry_selector, range_shift, table_records, table_links })
}

fn Decoder_opentype_ttc_header<>(_input: &mut Parser<'_>, start: u32) -> Result<opentype_ttc_header, ParseError> {
let ttc_tag = ((|| {
let inner = (Decoder20(_input))?;
let is_valid = {
let tag = inner;
tag == 1953784678u32
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9069368457806005425u64));
};
PResult::Ok(ret)
})())?;
let major_version = (Decoder23(_input))?;
let minor_version = (Decoder23(_input))?;
let header = ((|| {
let ret = match major_version {
1u16 => {
let inner = {
let num_fonts = (Decoder20(_input))?;
let table_directories = ((|| {
let mut accum = Vec::new();
for _ in 0..num_fonts {
accum.push({
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder_opentype_table_directory(_input, start.clone()))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_ttc_header_header_Version1_table_directories { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_ttc_header_header_Version1 { num_fonts, table_directories }
};
opentype_ttc_header_header::Version1(inner)
},

2u16 => {
let inner = {
let num_fonts = (Decoder20(_input))?;
let table_directories = ((|| {
let mut accum = Vec::new();
for _ in 0..num_fonts {
accum.push({
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder_opentype_table_directory(_input, start.clone()))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_ttc_header_header_Version1_table_directories { offset, link }
});
}
PResult::Ok(accum)
})())?;
let dsig_tag = (Decoder20(_input))?;
let dsig_length = (Decoder20(_input))?;
let dsig_offset = (Decoder20(_input))?;
opentype_ttc_header_header_Version2 { num_fonts, table_directories, dsig_tag, dsig_length, dsig_offset }
};
opentype_ttc_header_header::Version2(inner)
},

unknown => {
let inner = unknown.clone();
opentype_ttc_header_header::UnknownVersion(inner)
}
};
PResult::Ok(ret)
})())?;
let __skip = _input.skip_remainder();
PResult::Ok(opentype_ttc_header { ttc_tag, major_version, minor_version, header, __skip })
}

fn Decoder23<>(_input: &mut Parser<'_>) -> Result<u16, ParseError> {
let x = {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
(field0, field1)
};
PResult::Ok(u16be(x))
}

fn Decoder24<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
_input.read_byte()
}

fn Decoder_opentype_table_record<>(_input: &mut Parser<'_>) -> Result<opentype_table_record, ParseError> {
let table_id = (Decoder46(_input))?;
let checksum = (Decoder20(_input))?;
let offset = (Decoder20(_input))?;
let length = (Decoder20(_input))?;
PResult::Ok(opentype_table_record { table_id, checksum, offset, length })
}

fn Decoder_opentype_table_directory_table_links<>(_input: &mut Parser<'_>, start: u32, tables: &Vec<opentype_table_record>) -> Result<opentype_table_directory_table_links, ParseError> {
let cmap = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1668112752u32, tables)).cloned() {
Some(ref matching_table) => {
let tgt_offset = start + matching_table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (matching_table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_cmap_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let head = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1751474532u32, tables)).cloned() {
Some(ref matching_table) => {
let tgt_offset = start + matching_table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (matching_table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_head_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let hhea = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1751672161u32, tables)).cloned() {
Some(ref matching_table) => {
let tgt_offset = start + matching_table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (matching_table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_hhea_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let maxp = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1835104368u32, tables)).cloned() {
Some(ref matching_table) => {
let tgt_offset = start + matching_table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (matching_table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_maxp_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let hmtx = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1752003704u32, tables)).cloned() {
Some(ref matching_table) => {
let tgt_offset = start + matching_table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (matching_table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_hmtx_table(_input, hhea.number_of_long_metrics.clone(), maxp.num_glyphs.clone()))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let name = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1851878757u32, tables)).cloned() {
Some(ref matching_table) => {
let tgt_offset = start + matching_table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (matching_table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_name_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let os2 = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1330851634u32, tables)).cloned() {
Some(ref matching_table) => {
let tgt_offset = start + matching_table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_len = matching_table.length.clone();
let sz = table_len as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_os2_table(_input, table_len.clone()))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let post = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1886352244u32, tables)).cloned() {
Some(ref matching_table) => {
let tgt_offset = start + matching_table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (matching_table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_post_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let cvt = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1668707360u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
let next_elem = (Decoder23(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let fpgm = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1718642541u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let loca = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1819239265u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_loca_table(_input, maxp.num_glyphs.clone(), head.index_to_loc_format.clone()))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let glyf = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1735162214u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder36(_input, match loca {
Some(ref x) => {
x.offsets.clone()
},

None => {
opentype_gvar_table_glyph_variation_data_offsets::Offsets32([].to_vec())
}
}))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let prep = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1886545264u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let gasp = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1734439792u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_gasp_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let base = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1111577413u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_base_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let gdef = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1195656518u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_gdef_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let gpos = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1196445523u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_gpos_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let gsub = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1196643650u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_gsub_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let fvar = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1719034226u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_fvar_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let gvar = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1735811442u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_gvar_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let kern = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1801810542u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_kern_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let stat = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1398030676u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_stat_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let vhea = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1986553185u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_hhea_table(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let vmtx = ((|| {
let ret = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id.clone(), 1986884728u32, tables)).cloned() {
Some(ref table) => {
let val = {
let tgt_offset = start + table.offset.clone();
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (table.length.clone()) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_hmtx_table(_input, match vhea {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(3179861450314844647u64));
}
}.number_of_long_metrics, maxp.num_glyphs.clone()))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
Some(val)
},

None => {
None
}
};
PResult::Ok(ret)
})())?;
let __skip = _input.skip_remainder();
PResult::Ok(opentype_table_directory_table_links { cmap, head, hhea, maxp, hmtx, name, os2, post, cvt, fpgm, loca, glyf, prep, gasp, base, gdef, gpos, gsub, fvar, gvar, kern, stat, vhea, vmtx, __skip })
}

fn Decoder_opentype_cmap_table<>(_input: &mut Parser<'_>) -> Result<opentype_cmap_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let version = (Decoder23(_input))?;
let num_tables = (Decoder23(_input))?;
let encoding_records = ((|| {
let mut accum = Vec::new();
for _ in 0..num_tables {
accum.push((Decoder_opentype_encoding_record(_input, table_start.clone()))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_cmap_table { table_start, version, num_tables, encoding_records })
}

fn Decoder_opentype_head_table<>(_input: &mut Parser<'_>) -> Result<opentype_head_table, ParseError> {
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(15080388466336998873u64));
};
PResult::Ok(ret)
})())?;
let minor_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6070260202873699214u64));
};
PResult::Ok(ret)
})())?;
let font_revision = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
let checksum_adjustment = (Decoder20(_input))?;
let magic_number = ((|| {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 95 {
b
} else {
return Err(ParseError::ExcludedBranch(8986322043713516692u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 15 {
b
} else {
return Err(ParseError::ExcludedBranch(11786939113783016634u64));
})
})())?;
let field2 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 60 {
b
} else {
return Err(ParseError::ExcludedBranch(3852079030227774582u64));
})
})())?;
let field3 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 245 {
b
} else {
return Err(ParseError::ExcludedBranch(5176232487486782188u64));
})
})())?;
PResult::Ok((field0, field1, field2, field3))
})())?;
let flags = (Decoder23(_input))?;
let units_per_em = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
(x >= 16u16) && (x <= 16384u16)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9220862562374507822u64));
};
PResult::Ok(ret)
})())?;
let created = (Decoder98(_input))?;
let modified = (Decoder98(_input))?;
let glyph_extents = ((|| {
let x_min = (Decoder23(_input))?;
let y_min = (Decoder23(_input))?;
let x_max = (Decoder23(_input))?;
let y_max = (Decoder23(_input))?;
PResult::Ok(opentype_head_table_glyph_extents { x_min, y_min, x_max, y_max })
})())?;
let mac_style = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_head_table_mac_style { extended: packed_bits >> 6u16 & 1u16 > 0u16, condensed: packed_bits >> 5u16 & 1u16 > 0u16, shadow: packed_bits >> 4u16 & 1u16 > 0u16, outline: packed_bits >> 3u16 & 1u16 > 0u16, underline: packed_bits >> 2u16 & 1u16 > 0u16, italic: packed_bits >> 1u16 & 1u16 > 0u16, bold: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let lowest_rec_ppem = (Decoder23(_input))?;
let font_direction_hint = (Decoder23(_input))?;
let index_to_loc_format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x <= 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8772793160380380086u64));
};
PResult::Ok(ret)
})())?;
let glyph_data_format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2605623462625042002u64));
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_head_table { major_version, minor_version, font_revision, checksum_adjustment, magic_number, flags, units_per_em, created, modified, glyph_extents, mac_style, lowest_rec_ppem, font_direction_hint, index_to_loc_format, glyph_data_format })
}

fn Decoder_opentype_hhea_table<>(_input: &mut Parser<'_>) -> Result<opentype_hhea_table, ParseError> {
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(18164850183020044607u64));
};
PResult::Ok(ret)
})())?;
let minor_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
match x {
0u16 => {
true
},

4096u16 => {
true
},

_ => {
false
}
}
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10688770705819276010u64));
};
PResult::Ok(ret)
})())?;
let ascent = (Decoder23(_input))?;
let descent = (Decoder23(_input))?;
let line_gap = (Decoder23(_input))?;
let advance_width_max = (Decoder23(_input))?;
let min_left_side_bearing = (Decoder23(_input))?;
let min_right_side_bearing = (Decoder23(_input))?;
let x_max_extent = (Decoder23(_input))?;
let caret_slope = ((|| {
let rise = (Decoder23(_input))?;
let run = (Decoder23(_input))?;
PResult::Ok(opentype_hhea_table_caret_slope { rise, run })
})())?;
let caret_offset = (Decoder23(_input))?;
let __reservedX4 = ((|| {
let field0 = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10416240583538343445u64));
};
PResult::Ok(ret)
})())?;
let field1 = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1457499133218925748u64));
};
PResult::Ok(ret)
})())?;
let field2 = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14215639860155940137u64));
};
PResult::Ok(ret)
})())?;
let field3 = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(5584166819955891466u64));
};
PResult::Ok(ret)
})())?;
PResult::Ok((field0, field1, field2, field3))
})())?;
let metric_data_format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11133239979815295357u64));
};
PResult::Ok(ret)
})())?;
let number_of_long_metrics = (Decoder23(_input))?;
PResult::Ok(opentype_hhea_table { major_version, minor_version, ascent, descent, line_gap, advance_width_max, min_left_side_bearing, min_right_side_bearing, x_max_extent, caret_slope, caret_offset, __reservedX4, metric_data_format, number_of_long_metrics })
}

fn Decoder_opentype_maxp_table<>(_input: &mut Parser<'_>) -> Result<opentype_maxp_table, ParseError> {
let version = (Decoder20(_input))?;
let num_glyphs = (Decoder23(_input))?;
let data = ((|| {
let ret = match version {
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
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_maxp_table { version, num_glyphs, data })
}

fn Decoder_opentype_hmtx_table<>(_input: &mut Parser<'_>, num_long_metrics: u16, num_glyphs: u16) -> Result<opentype_hmtx_table, ParseError> {
let long_metrics = ((|| {
let mut accum = Vec::new();
for _ in 0..num_long_metrics {
accum.push({
let advance_width = (Decoder23(_input))?;
let left_side_bearing = (Decoder23(_input))?;
opentype_hmtx_table_long_metrics { advance_width, left_side_bearing }
});
}
PResult::Ok(accum)
})())?;
let left_side_bearings = ((|| {
let mut accum = Vec::new();
for _ in 0..try_sub!(num_glyphs, num_long_metrics, 2206609067086327257u64) {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_hmtx_table { long_metrics, left_side_bearings })
}

fn Decoder_opentype_name_table<>(_input: &mut Parser<'_>) -> Result<opentype_name_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let version = (Decoder23(_input))?;
let name_count = (Decoder23(_input))?;
let storage_offset = (Decoder23(_input))?;
let name_records = ((|| {
let mut accum = Vec::new();
for _ in 0..name_count {
accum.push({
let platform = (Decoder23(_input))?;
let encoding = (Decoder23(_input))?;
let language = (Decoder23(_input))?;
let name_id = (Decoder23(_input))?;
let length = (Decoder23(_input))?;
let offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (storage_offset as u32) + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (storage_offset as u32) + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let mut accum = Vec::new();
for _ in 0..length {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_name_table_name_records_offset { offset, link })
})())?;
opentype_name_table_name_records { platform, encoding, language, name_id, length, offset }
});
}
PResult::Ok(accum)
})())?;
let data = ((|| {
let ret = match version {
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
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_name_table { table_start, version, name_count, storage_offset, name_records, data })
}

fn Decoder_opentype_os2_table<>(_input: &mut Parser<'_>, table_length: u32) -> Result<opentype_os2_table, ParseError> {
let version = (Decoder23(_input))?;
let x_avg_char_width = (Decoder23(_input))?;
let us_weight_class = (Decoder23(_input))?;
let us_width_class = (Decoder23(_input))?;
let fs_type = (Decoder23(_input))?;
let y_subscript_x_size = (Decoder23(_input))?;
let y_subscript_y_size = (Decoder23(_input))?;
let y_subscript_x_offset = (Decoder23(_input))?;
let y_subscript_y_offset = (Decoder23(_input))?;
let y_superscript_x_size = (Decoder23(_input))?;
let y_superscript_y_size = (Decoder23(_input))?;
let y_superscript_x_offset = (Decoder23(_input))?;
let y_superscript_y_offset = (Decoder23(_input))?;
let y_strikeout_size = (Decoder23(_input))?;
let y_strikeout_position = (Decoder23(_input))?;
let s_family_class = (Decoder23(_input))?;
let panose = ((|| {
let mut accum = Vec::new();
for _ in 0..10u8 {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
let ul_unicode_range1 = (Decoder20(_input))?;
let ul_unicode_range2 = (Decoder20(_input))?;
let ul_unicode_range3 = (Decoder20(_input))?;
let ul_unicode_range4 = (Decoder20(_input))?;
let ach_vend_id = (Decoder46(_input))?;
let fs_selection = (Decoder23(_input))?;
let us_first_char_index = (Decoder23(_input))?;
let us_last_char_index = (Decoder23(_input))?;
let data = ((|| {
let ret = if (version > 0u16) || (table_length >= 78u32) {
let s_typo_ascender = (Decoder23(_input))?;
let s_typo_descender = (Decoder23(_input))?;
let s_typo_line_gap = (Decoder23(_input))?;
let us_win_ascent = (Decoder23(_input))?;
let us_win_descent = (Decoder23(_input))?;
let extra_fields_v1 = ((|| {
let ret = if match version {
1u16.. => {
true
},

_ => {
false
}
} {
let ul_code_page_range_1 = (Decoder20(_input))?;
let ul_code_page_range_2 = (Decoder20(_input))?;
let extra_fields_v2 = ((|| {
let ret = if match version {
2u16.. => {
true
},

_ => {
false
}
} {
let sx_height = (Decoder23(_input))?;
let s_cap_height = (Decoder23(_input))?;
let us_default_char = (Decoder23(_input))?;
let us_break_char = (Decoder23(_input))?;
let us_max_context = (Decoder23(_input))?;
let extra_fields_v5 = ((|| {
let ret = if match version {
5u16.. => {
true
},

_ => {
false
}
} {
let us_lower_optical_point_size = (Decoder23(_input))?;
let us_upper_optical_point_size = (Decoder23(_input))?;
Some(opentype_os2_table_data_extra_fields_v1_extra_fields_v2_extra_fields_v5 { us_lower_optical_point_size, us_upper_optical_point_size })
} else {
None
};
PResult::Ok(ret)
})())?;
Some(opentype_os2_table_data_extra_fields_v1_extra_fields_v2 { sx_height, s_cap_height, us_default_char, us_break_char, us_max_context, extra_fields_v5 })
} else {
None
};
PResult::Ok(ret)
})())?;
Some(opentype_os2_table_data_extra_fields_v1 { ul_code_page_range_1, ul_code_page_range_2, extra_fields_v2 })
} else {
None
};
PResult::Ok(ret)
})())?;
Some(opentype_os2_table_data { s_typo_ascender, s_typo_descender, s_typo_line_gap, us_win_ascent, us_win_descent, extra_fields_v1 })
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_os2_table { version, x_avg_char_width, us_weight_class, us_width_class, fs_type, y_subscript_x_size, y_subscript_y_size, y_subscript_x_offset, y_subscript_y_offset, y_superscript_x_size, y_superscript_y_size, y_superscript_x_offset, y_superscript_y_offset, y_strikeout_size, y_strikeout_position, s_family_class, panose, ul_unicode_range1, ul_unicode_range2, ul_unicode_range3, ul_unicode_range4, ach_vend_id, fs_selection, us_first_char_index, us_last_char_index, data })
}

fn Decoder_opentype_post_table<>(_input: &mut Parser<'_>) -> Result<opentype_post_table, ParseError> {
let version = (Decoder20(_input))?;
let italic_angle = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
let underline_position = (Decoder23(_input))?;
let underline_thickness = (Decoder23(_input))?;
let is_fixed_pitch = (Decoder20(_input))?;
let min_mem_type42 = (Decoder20(_input))?;
let max_mem_type42 = (Decoder20(_input))?;
let min_mem_type1 = (Decoder20(_input))?;
let max_mem_type1 = (Decoder20(_input))?;
let names = ((|| {
let ret = match version {
65536u32 => {
opentype_post_table_names::Version1
},

131072u32 => {
let inner = {
let num_glyphs = (Decoder23(_input))?;
let glyph_name_index = ((|| {
let mut accum = Vec::new();
for _ in 0..num_glyphs {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let string_data = {
let x = _input.get_offset_u64();
x as u32
};
opentype_post_table_names_Version2 { num_glyphs, glyph_name_index, string_data }
};
opentype_post_table_names::Version2(inner)
},

151552u32 => {
let inner = {
let num_glyphs = (Decoder23(_input))?;
let offset = ((|| {
let mut accum = Vec::new();
for _ in 0..num_glyphs {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
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
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_post_table { version, italic_angle, underline_position, underline_thickness, is_fixed_pitch, min_mem_type42, max_mem_type42, min_mem_type1, max_mem_type1, names })
}

fn Decoder_opentype_loca_table<>(_input: &mut Parser<'_>, num_glyphs: u16, index_to_loc_format: u16) -> Result<opentype_loca_table, ParseError> {
let offsets = ((|| {
let ret = match index_to_loc_format {
0u16 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..succ(num_glyphs) {
accum.push((Decoder23(_input))?);
}
accum
};
opentype_gvar_table_glyph_variation_data_offsets::Offsets16(inner)
},

1u16 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..succ(num_glyphs) {
accum.push((Decoder20(_input))?);
}
accum
};
opentype_gvar_table_glyph_variation_data_offsets::Offsets32(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_loca_table { offsets })
}

fn Decoder36<>(_input: &mut Parser<'_>, offsets: opentype_gvar_table_glyph_variation_data_offsets) -> Result<Vec<opentype_glyf_table>, ParseError> {
let start_offset = {
let x = _input.get_offset_u64();
x as u32
};
let ret = match offsets {
opentype_gvar_table_glyph_variation_data_offsets::Offsets16(ref half16s) => {
let len = pred((half16s.len()) as u32);
let mut accum = Vec::new();
for ix in 0u32..len {
accum.push({
let ret = match (((half16s[ix as usize].clone()) as u32) * 2u32, ((half16s[(succ(ix)) as usize].clone()) as u32) * 2u32) {
(this_offs, next_offs) => {
let ret = match next_offs > this_offs {
true => {
let tgt_offset = start_offset + this_offs;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let inner = {
let number_of_contours = (Decoder23(_input))?;
let x_min = (Decoder23(_input))?;
let y_min = (Decoder23(_input))?;
let x_max = (Decoder23(_input))?;
let y_max = (Decoder23(_input))?;
let description = (Decoder_opentype_glyf_description(_input, number_of_contours.clone()))?;
opentype_glyf_table_Glyph { number_of_contours, x_min, y_min, x_max, y_max, description }
};
PResult::Ok(opentype_glyf_table::Glyph(inner))
})())?;
_input.close_peek_context()?;
ret
},

false => {
opentype_glyf_table::EmptyGlyph
}
};
ret
}
};
ret
});
}
accum
},

opentype_gvar_table_glyph_variation_data_offsets::Offsets32(ref off32s) => {
let len = pred((off32s.len()) as u32);
let mut accum = Vec::new();
for ix in 0u32..len {
accum.push({
let ret = match (off32s[ix as usize].clone(), off32s[(succ(ix)) as usize].clone()) {
(this_offs, next_offs) => {
let ret = match next_offs > this_offs {
true => {
let tgt_offset = start_offset + this_offs;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let inner = {
let number_of_contours = (Decoder23(_input))?;
let x_min = (Decoder23(_input))?;
let y_min = (Decoder23(_input))?;
let x_max = (Decoder23(_input))?;
let y_max = (Decoder23(_input))?;
let description = (Decoder_opentype_glyf_description(_input, number_of_contours.clone()))?;
opentype_glyf_table_Glyph { number_of_contours, x_min, y_min, x_max, y_max, description }
};
PResult::Ok(opentype_glyf_table::Glyph(inner))
})())?;
_input.close_peek_context()?;
ret
},

false => {
opentype_glyf_table::EmptyGlyph
}
};
ret
}
};
ret
});
}
accum
}
};
PResult::Ok(ret)
}

fn Decoder_opentype_gasp_table<>(_input: &mut Parser<'_>) -> Result<opentype_gasp_table, ParseError> {
let version = (Decoder23(_input))?;
let num_ranges = (Decoder23(_input))?;
let gasp_ranges = ((|| {
let mut accum = Vec::new();
for _ in 0..num_ranges {
accum.push({
let range_max_ppem = (Decoder23(_input))?;
let range_gasp_behavior = ((|| {
let ret = match version {
0u16 => {
let inner = {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version0 { dogray: packed_bits >> 1u16 & 1u16 > 0u16, gridfit: packed_bits >> 0u16 & 1u16 > 0u16 }
};
opentype_gasp_table_gasp_ranges_range_gasp_behavior::Version0(inner)
},

1u16 => {
let inner = {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version1 { symmetric_smoothing: packed_bits >> 3u16 & 1u16 > 0u16, symmetric_gridfit: packed_bits >> 2u16 & 1u16 > 0u16, dogray: packed_bits >> 1u16 & 1u16 > 0u16, gridfit: packed_bits >> 0u16 & 1u16 > 0u16 }
};
opentype_gasp_table_gasp_ranges_range_gasp_behavior::Version1(inner)
},

_ => {
return Err(ParseError::FailToken(1275286460638129217u64));
}
};
PResult::Ok(ret)
})())?;
opentype_gasp_table_gasp_ranges { range_max_ppem, range_gasp_behavior }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_gasp_table { version, num_ranges, gasp_ranges })
}

fn Decoder_opentype_base_table<>(_input: &mut Parser<'_>) -> Result<opentype_base_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(386759067598651566u64));
};
PResult::Ok(ret)
})())?;
let minor_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x <= 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13527164188224560282u64));
};
PResult::Ok(ret)
})())?;
let horiz_axis_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_axis_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_base_table_vert_axis_offset { offset, link })
})())?;
let vert_axis_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_axis_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_base_table_vert_axis_offset { offset, link })
})())?;
let item_var_store_offset = ((|| {
let ret = if minor_version > 0u16 {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder86(_input))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_base_table_item_var_store_offset { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_base_table { table_start, major_version, minor_version, horiz_axis_offset, vert_axis_offset, item_var_store_offset })
}

fn Decoder_opentype_gdef_table<>(_input: &mut Parser<'_>) -> Result<opentype_gdef_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(240888096670347429u64));
};
PResult::Ok(ret)
})())?;
let minor_version = (Decoder23(_input))?;
let glyph_class_def = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_class_def(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_glyph_class_def { offset, link })
})())?;
let attach_list = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let glyph_count = (Decoder23(_input))?;
let attach_point_offsets = ((|| {
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let point_count = (Decoder23(_input))?;
let point_indices = ((|| {
let mut accum = Vec::new();
for _ in 0..point_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_gdef_table_attach_list_link_attach_point_offsets_link { point_count, point_indices })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_gdef_table_attach_list_link_attach_point_offsets { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_gdef_table_attach_list_link { table_start, coverage, glyph_count, attach_point_offsets })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_attach_list { offset, link })
})())?;
let lig_caret_list = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let lig_glyph_count = (Decoder23(_input))?;
let lig_glyph_offsets = ((|| {
let mut accum = Vec::new();
for _ in 0..lig_glyph_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let caret_count = (Decoder23(_input))?;
let caret_values = ((|| {
let mut accum = Vec::new();
for _ in 0..caret_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let caret_value_format = (Decoder23(_input))?;
let data = ((|| {
let ret = match caret_value_format {
1u16 => {
let inner = {
let coordinate = (Decoder23(_input))?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format1 { coordinate }
};
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data::Format1(inner)
},

2u16 => {
let inner = {
let caret_value_point_index = (Decoder23(_input))?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format2 { caret_value_point_index }
};
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data::Format2(inner)
},

3u16 => {
let inner = {
let coordinate = (Decoder23(_input))?;
let table = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_value_record_x_advance_device { offset, link })
})())?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format3 { coordinate, table }
};
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data::Format3(inner)
},

_ => {
return Err(ParseError::FailToken(11936787736236307191u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link { table_start, caret_value_format, data })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link { table_start, caret_count, caret_values })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_gdef_table_lig_caret_list_link { table_start, coverage, lig_glyph_count, lig_glyph_offsets })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_lig_caret_list { offset, link })
})())?;
let mark_attach_class_def = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_class_def(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_glyph_class_def { offset, link })
})())?;
let data = ((|| {
let ret = match minor_version {
0u16 => {
opentype_gdef_table_data::Version1_0
},

1u16 => {
return Err(ParseError::FailToken(5215619712890029856u64));
},

2u16 => {
let inner = {
let mark_glyph_sets_def = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_mark_glyph_set(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_data_Version1_2_mark_glyph_sets_def { offset, link })
})())?;
opentype_gdef_table_data_Version1_2 { mark_glyph_sets_def }
};
opentype_gdef_table_data::Version1_2(inner)
},

3u16 => {
let inner = {
let mark_glyph_sets_def = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_mark_glyph_set(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_data_Version1_2_mark_glyph_sets_def { offset, link })
})())?;
let item_var_store = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder86(_input))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_base_table_item_var_store_offset { offset, link })
})())?;
opentype_gdef_table_data_Version1_3 { mark_glyph_sets_def, item_var_store }
};
opentype_gdef_table_data::Version1_3(inner)
},

_ => {
let inner = {
let mark_glyph_sets_def = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_mark_glyph_set(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_data_Version1_2_mark_glyph_sets_def { offset, link })
})())?;
let item_var_store = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder86(_input))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_base_table_item_var_store_offset { offset, link })
})())?;
opentype_gdef_table_data_Version1_3 { mark_glyph_sets_def, item_var_store }
};
opentype_gdef_table_data::Version1_3(inner)
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table { table_start, major_version, minor_version, glyph_class_def, attach_list, lig_caret_list, mark_attach_class_def, data })
}

fn Decoder_opentype_gpos_table<>(_input: &mut Parser<'_>) -> Result<opentype_gpos_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3167775832820164678u64));
};
PResult::Ok(ret)
})())?;
let minor_version = (Decoder23(_input))?;
let script_list = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_script_list(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gsub_table_script_list { offset, link })
})())?;
let feature_list = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_feature_list(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gsub_table_feature_list { offset, link })
})())?;
let lookup_list = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let lookup_count = (Decoder23(_input))?;
let lookups = ((|| {
let mut accum = Vec::new();
for _ in 0..lookup_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let lookup_type = (Decoder23(_input))?;
let lookup_flag = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_gsub_table_lookup_list_link_lookups_link_lookup_flag { mark_attachment_class_filter: packed_bits >> 8u16 & 255u16, use_mark_filtering_set: packed_bits >> 4u16 & 1u16 > 0u16, ignore_marks: packed_bits >> 3u16 & 1u16 > 0u16, ignore_ligatures: packed_bits >> 2u16 & 1u16 > 0u16, ignore_base_glyphs: packed_bits >> 1u16 & 1u16 > 0u16, right_to_left: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let sub_table_count = (Decoder23(_input))?;
let subtables = ((|| {
let mut accum = Vec::new();
for _ in 0..sub_table_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let ret = match lookup_type {
9u16 => {
let inner = (Decoder_opentype_layout_pos_extension(_input))?;
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::PosExtension(inner)
},

_ => {
let inner = (Decoder_opentype_layout_ground_pos(_input, lookup_type.clone()))?;
opentype_gpos_table_lookup_list_link_lookups_link_subtables_link::GroundPos(inner)
}
};
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_gpos_table_lookup_list_link_lookups_link_subtables { offset, link }
});
}
PResult::Ok(accum)
})())?;
let mark_filtering_set = ((|| {
let ret = match lookup_flag.use_mark_filtering_set.clone() {
true => {
let val = (Decoder23(_input))?;
Some(val)
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gpos_table_lookup_list_link_lookups_link { table_start, lookup_type, lookup_flag, sub_table_count, subtables, mark_filtering_set })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_gpos_table_lookup_list_link_lookups { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_gpos_table_lookup_list_link { table_start, lookup_count, lookups })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gpos_table_lookup_list { offset, link })
})())?;
let feature_variations_offset = ((|| {
let ret = if minor_version > 0u16 {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder_opentype_layout_feature_variations(_input))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_gsub_table_feature_variations_offset { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gpos_table { table_start, major_version, minor_version, script_list, feature_list, lookup_list, feature_variations_offset })
}

fn Decoder_opentype_gsub_table<>(_input: &mut Parser<'_>) -> Result<opentype_gsub_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7215050775822222282u64));
};
PResult::Ok(ret)
})())?;
let minor_version = (Decoder23(_input))?;
let script_list = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_script_list(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gsub_table_script_list { offset, link })
})())?;
let feature_list = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_feature_list(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gsub_table_feature_list { offset, link })
})())?;
let lookup_list = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let lookup_count = (Decoder23(_input))?;
let lookups = ((|| {
let mut accum = Vec::new();
for _ in 0..lookup_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let lookup_type = (Decoder23(_input))?;
let lookup_flag = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_gsub_table_lookup_list_link_lookups_link_lookup_flag { mark_attachment_class_filter: packed_bits >> 8u16 & 255u16, use_mark_filtering_set: packed_bits >> 4u16 & 1u16 > 0u16, ignore_marks: packed_bits >> 3u16 & 1u16 > 0u16, ignore_ligatures: packed_bits >> 2u16 & 1u16 > 0u16, ignore_base_glyphs: packed_bits >> 1u16 & 1u16 > 0u16, right_to_left: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let sub_table_count = (Decoder23(_input))?;
let subtables = ((|| {
let mut accum = Vec::new();
for _ in 0..sub_table_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let ret = match lookup_type {
7u16 => {
let inner = (Decoder_opentype_layout_subst_extension(_input))?;
opentype_gsub_table_lookup_list_link_lookups_link_subtables_link::SubstExtension(inner)
},

_ => {
let inner = (Decoder_opentype_layout_ground_subst(_input, lookup_type.clone()))?;
opentype_gsub_table_lookup_list_link_lookups_link_subtables_link::GroundSubst(inner)
}
};
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_gsub_table_lookup_list_link_lookups_link_subtables { offset, link }
});
}
PResult::Ok(accum)
})())?;
let mark_filtering_set = ((|| {
let ret = match lookup_flag.use_mark_filtering_set.clone() {
true => {
let val = (Decoder23(_input))?;
Some(val)
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gsub_table_lookup_list_link_lookups_link { table_start, lookup_type, lookup_flag, sub_table_count, subtables, mark_filtering_set })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_gsub_table_lookup_list_link_lookups { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_gsub_table_lookup_list_link { table_start, lookup_count, lookups })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gsub_table_lookup_list { offset, link })
})())?;
let feature_variations_offset = ((|| {
let ret = if minor_version > 0u16 {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder_opentype_layout_feature_variations(_input))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_gsub_table_feature_variations_offset { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gsub_table { table_start, major_version, minor_version, script_list, feature_list, lookup_list, feature_variations_offset })
}

fn Decoder_opentype_fvar_table<>(_input: &mut Parser<'_>) -> Result<opentype_fvar_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3743786174148899814u64));
};
PResult::Ok(ret)
})())?;
let minor_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12652804269632162478u64));
};
PResult::Ok(ret)
})())?;
let __offset_axes = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let raw = inner;
raw > 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(18134882366868794706u64));
};
PResult::Ok(ret)
})())?;
let __reserved = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 2u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7155653122005708978u64));
};
PResult::Ok(ret)
})())?;
let axis_count = (Decoder23(_input))?;
let axis_size = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 20u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17920584887603040596u64));
};
PResult::Ok(ret)
})())?;
let instance_count = (Decoder23(_input))?;
let instance_size = (Decoder23(_input))?;
let __axes_length = axis_count * axis_size;
let axes = ((|| {
let _ = {
let inner = {
let x = _input.get_offset_u64();
x as u32
};
let is_valid = {
let __here = inner;
table_start + (__offset_axes as u32) >= __here
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(5673845796627816005u64));
};
ret
};
let tgt_offset = 0u32 + table_start + (__offset_axes as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = __axes_length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let mut accum = Vec::new();
for _ in 0..axis_count {
accum.push({
let sz = axis_size as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_var_variation_axis_record(_input))?;
_input.end_slice()?;
ret
});
}
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
PResult::Ok(ret)
})())?;
let instances = ((|| {
let _ = {
let inner = {
let x = _input.get_offset_u64();
x as u32
};
let is_valid = {
let __here = inner;
table_start + ((__offset_axes + __axes_length) as u32) >= __here
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14591018267292443527u64));
};
ret
};
let tgt_offset = 0u32 + table_start + ((__offset_axes + __axes_length) as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let mut accum = Vec::new();
for _ in 0..instance_count {
accum.push({
let sz = instance_size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let subfamily_nameid = (Decoder23(_input))?;
let flags = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4762692522317026931u64));
};
PResult::Ok(ret)
})())?;
let coordinates = (Decoder_opentype_var_user_tuple(_input, axis_count.clone()))?;
let postscript_nameid = ((|| {
let ret = if instance_size % 4u16 == 2u16 {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_fvar_table_instances { subfamily_nameid, flags, coordinates, postscript_nameid })
})())?;
_input.end_slice()?;
ret
});
}
PResult::Ok(accum)
})())?;
_input.close_peek_context()?;
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_fvar_table { table_start, major_version, minor_version, __offset_axes, __reserved, axis_count, axis_size, instance_count, instance_size, __axes_length, axes, instances })
}

fn Decoder_opentype_gvar_table<>(_input: &mut Parser<'_>) -> Result<opentype_gvar_table, ParseError> {
let gvar_table_start = {
let x = _input.get_offset_u64();
x as u32
};
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9630069758457681762u64));
};
PResult::Ok(ret)
})())?;
let minor_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(908377722732597655u64));
};
PResult::Ok(ret)
})())?;
let axis_count = (Decoder23(_input))?;
let shared_tuple_count = (Decoder23(_input))?;
let shared_tuples_offset = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = gvar_table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = {
let mut accum = Vec::new();
for _ in 0..shared_tuple_count {
accum.push((Decoder_opentype_var_tuple_record(_input, axis_count.clone()))?);
}
accum
};
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gvar_table_shared_tuples_offset { offset, link })
})())?;
let glyph_count = (Decoder23(_input))?;
let flags = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_gvar_table_flags { is_long_offset: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let glyph_variation_data_array_offset = (Decoder20(_input))?;
let glyph_variation_data_offsets = ((|| {
let ret = match flags.is_long_offset.clone() {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..succ(glyph_count) {
accum.push((Decoder20(_input))?);
}
accum
};
opentype_gvar_table_glyph_variation_data_offsets::Offsets32(inner)
},

false => {
let inner = {
let mut accum = Vec::new();
for _ in 0..succ(glyph_count) {
accum.push((Decoder23(_input))?);
}
accum
};
opentype_gvar_table_glyph_variation_data_offsets::Offsets16(inner)
}
};
PResult::Ok(ret)
})())?;
let glyph_variation_data_array = ((|| {
let tgt_offset = gvar_table_start + glyph_variation_data_array_offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let array_start = {
let x = _input.get_offset_u64();
x as u32
};
let ret = match glyph_variation_data_offsets {
opentype_gvar_table_glyph_variation_data_offsets::Offsets16(ref half16s) => {
let len = pred((half16s.len()) as u32);
let mut accum = Vec::new();
for ix in 0u32..len {
accum.push({
let ret = match (((half16s[ix as usize].clone()) as u32) * 2u32, ((half16s[(succ(ix)) as usize].clone()) as u32) * 2u32) {
(this_offs, next_offs) => {
let ret = if next_offs > this_offs {
let tgt_offset = array_start + this_offs;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (try_sub!(next_offs, this_offs, 11876854719037224982u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_var_glyph_variation_data_table(_input, axis_count.clone()))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
}
};
ret
});
}
accum
},

opentype_gvar_table_glyph_variation_data_offsets::Offsets32(ref off32s) => {
let len = pred((off32s.len()) as u32);
let mut accum = Vec::new();
for ix in 0u32..len {
accum.push({
let ret = match (off32s[ix as usize].clone(), off32s[(succ(ix)) as usize].clone()) {
(this_offs, next_offs) => {
let ret = if next_offs > this_offs {
let tgt_offset = array_start + this_offs;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let sz = (try_sub!(next_offs, this_offs, 18270091135093349626u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_opentype_var_glyph_variation_data_table(_input, axis_count.clone()))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
}
};
ret
});
}
accum
}
};
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gvar_table { gvar_table_start, major_version, minor_version, axis_count, shared_tuple_count, shared_tuples_offset, glyph_count, flags, glyph_variation_data_array_offset, glyph_variation_data_offsets, glyph_variation_data_array })
}

fn Decoder_opentype_kern_table<>(_input: &mut Parser<'_>) -> Result<opentype_kern_table, ParseError> {
let version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3203034260088513018u64));
};
PResult::Ok(ret)
})())?;
let n_tables = (Decoder23(_input))?;
let subtables = ((|| {
let mut accum = Vec::new();
for _ in 0..n_tables {
accum.push({
let version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14677505873656710393u64));
};
PResult::Ok(ret)
})())?;
let length = (Decoder23(_input))?;
let coverage = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_kern_table_subtables_coverage { format: packed_bits >> 8u16 & 255u16, r#override: packed_bits >> 3u16 & 1u16 > 0u16, cross_stream: packed_bits >> 2u16 & 1u16 > 0u16, minimum: packed_bits >> 1u16 & 1u16 > 0u16, horizontal: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let data = ((|| {
let ret = match coverage.format.clone() {
0u16 => {
let inner = {
let n_pairs = (Decoder23(_input))?;
let search_range = (Decoder23(_input))?;
let entry_selector = (Decoder23(_input))?;
let range_shift = (Decoder23(_input))?;
let kern_pairs = ((|| {
let mut accum = Vec::new();
for _ in 0..n_pairs {
accum.push({
let left = (Decoder23(_input))?;
let right = (Decoder23(_input))?;
let value = (Decoder23(_input))?;
opentype_kern_table_subtables_data_Format0_kern_pairs { left, right, value }
});
}
PResult::Ok(accum)
})())?;
opentype_kern_table_subtables_data_Format0 { n_pairs, search_range, entry_selector, range_shift, kern_pairs }
};
opentype_kern_table_subtables_data::Format0(inner)
},

2u16 => {
let inner = {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let row_width = (Decoder23(_input))?;
let left_class_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let first_glyph = (Decoder23(_input))?;
let n_glyphs = (Decoder23(_input))?;
let class_values = ((|| {
let mut accum = Vec::new();
for _ in 0..n_glyphs {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_kern_table_subtables_data_Format2_left_class_offset_link { first_glyph, n_glyphs, class_values })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_kern_table_subtables_data_Format2_left_class_offset { offset, link })
})())?;
let right_class_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let first_glyph = (Decoder23(_input))?;
let n_glyphs = (Decoder23(_input))?;
let class_values = ((|| {
let mut accum = Vec::new();
for _ in 0..n_glyphs {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_kern_table_subtables_data_Format2_left_class_offset_link { first_glyph, n_glyphs, class_values })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_kern_table_subtables_data_Format2_left_class_offset { offset, link })
})())?;
let kerning_array_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let mut accum = Vec::new();
for _ in 0..match left_class_offset.link.clone() {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(6185506036438099345u64));
}
}.n_glyphs {
accum.push({
let mut accum = Vec::new();
for _ in 0..match right_class_offset.link.clone() {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(15794382300316794652u64));
}
}.n_glyphs {
accum.push((Decoder23(_input))?);
}
accum
});
}
PResult::Ok(accum)
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_kern_table_subtables_data_Format2_kerning_array_offset { offset, link })
})())?;
opentype_kern_table_subtables_data_Format2 { table_start, row_width, left_class_offset, right_class_offset, kerning_array_offset }
};
opentype_kern_table_subtables_data::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(10102114574336663273u64));
}
};
PResult::Ok(ret)
})())?;
opentype_kern_table_subtables { version, length, coverage, data }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_kern_table { version, n_tables, subtables })
}

fn Decoder_opentype_stat_table<>(_input: &mut Parser<'_>) -> Result<opentype_stat_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4386762582485017400u64));
};
PResult::Ok(ret)
})())?;
let minor_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
match x {
1u16 => {
true
},

2u16 => {
true
},

_ => {
false
}
}
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8893850231119365992u64));
};
PResult::Ok(ret)
})())?;
let design_axis_size = (Decoder23(_input))?;
let design_axis_count = (Decoder23(_input))?;
let design_axes_offset = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = {
let design_axes = ((|| {
let mut accum = Vec::new();
for _ in 0..design_axis_count {
accum.push({
let axis_tag = (Decoder46(_input))?;
let axis_name_id = (Decoder23(_input))?;
let axis_ordering = (Decoder23(_input))?;
opentype_stat_table_design_axes_offset_link_design_axes { axis_tag, axis_name_id, axis_ordering }
});
}
PResult::Ok(accum)
})())?;
opentype_stat_table_design_axes_offset_link { design_axes }
};
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_stat_table_design_axes_offset { offset, link })
})())?;
let axis_value_count = (Decoder23(_input))?;
let offset_to_axis_value_offsets = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let axis_value_offsets = ((|| {
let mut accum = Vec::new();
for _ in 0..axis_value_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
match x {
1u16 => {
true
},

2u16 => {
true
},

3u16 => {
true
},

4u16 => {
true
},

_ => {
false
}
}
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7659860344311718435u64));
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let ret = match format {
1u16 => {
let inner = {
let axis_index = (Decoder23(_input))?;
let flags = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let value_name_id = (Decoder23(_input))?;
let value = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1 { axis_index, flags, value_name_id, value }
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data::Format1(inner)
},

2u16 => {
let inner = {
let axis_index = (Decoder23(_input))?;
let flags = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let value_name_id = (Decoder23(_input))?;
let nominal_value = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
let range_min_value = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
let range_max_value = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format2 { axis_index, flags, value_name_id, nominal_value, range_min_value, range_max_value }
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data::Format2(inner)
},

3u16 => {
let inner = {
let axis_index = (Decoder23(_input))?;
let flags = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let value_name_id = (Decoder23(_input))?;
let value = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
let linked_value = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format3 { axis_index, flags, value_name_id, value, linked_value }
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data::Format3(inner)
},

4u16 => {
let inner = {
let axis_count = (Decoder23(_input))?;
let flags = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format1_flags { elidable_axis_value_name: packed_bits >> 1u16 & 1u16 > 0u16, older_sibling_font_attribute: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let value_name_id = (Decoder23(_input))?;
let axis_values = ((|| {
let mut accum = Vec::new();
for _ in 0..axis_count {
accum.push({
let axis_index = (Decoder23(_input))?;
let value = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format4_axis_values { axis_index, value }
});
}
PResult::Ok(accum)
})())?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data_Format4 { axis_count, flags, value_name_id, axis_values }
};
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link_data::Format4(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets_link { format, data })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_stat_table_offset_to_axis_value_offsets_link_axis_value_offsets { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_stat_table_offset_to_axis_value_offsets_link { table_start, axis_value_offsets }
};
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_stat_table_offset_to_axis_value_offsets { offset, link })
})())?;
let elided_fallback_name_id = (Decoder23(_input))?;
PResult::Ok(opentype_stat_table { table_start, major_version, minor_version, design_axis_size, design_axis_count, design_axes_offset, axis_value_count, offset_to_axis_value_offsets, elided_fallback_name_id })
}

fn Decoder46<>(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
Decoder20(_input)
}

fn Decoder_opentype_var_tuple_record<>(_input: &mut Parser<'_>, axis_count: u16) -> Result<opentype_var_tuple_record, ParseError> {
let coordinates = ((|| {
let mut accum = Vec::new();
for _ in 0..axis_count {
accum.push({
let x = (Decoder23(_input))?;
opentype_var_tuple_record_coordinates::F2Dot14(x)
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_var_tuple_record { coordinates })
}

fn Decoder_opentype_var_glyph_variation_data_table<>(_input: &mut Parser<'_>, axis_count: u16) -> Result<opentype_var_glyph_variation_data_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let tuple_variation_count = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_var_glyph_variation_data_table_tuple_variation_count { shared_point_numbers: packed_bits >> 15u16 & 1u16 > 0u16, tuple_count: packed_bits >> 0u16 & 4095u16 })
})())?;
let __data_offset = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x != 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11052099086134529863u64));
};
PResult::Ok(ret)
})())?;
let tuple_variation_headers = ((|| {
let mut accum = Vec::new();
for _ in 0..tuple_variation_count.tuple_count.clone() {
accum.push({
let variation_data_size = (Decoder23(_input))?;
let tuple_index = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_var_glyph_variation_data_table_tuple_variation_headers_tuple_index { embedded_peak_tuple: packed_bits >> 15u16 & 1u16 > 0u16, intermediate_region: packed_bits >> 14u16 & 1u16 > 0u16, private_point_numbers: packed_bits >> 13u16 & 1u16 > 0u16, tuple_index: packed_bits >> 0u16 & 4095u16 })
})())?;
let peak_tuple = ((|| {
let ret = if tuple_index.embedded_peak_tuple.clone() {
Some((Decoder_opentype_var_tuple_record(_input, axis_count.clone()))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let intermediate_tuples = ((|| {
let ret = if tuple_index.intermediate_region.clone() {
let start_tuple = (Decoder_opentype_var_tuple_record(_input, axis_count.clone()))?;
let end_tuple = (Decoder_opentype_var_tuple_record(_input, axis_count.clone()))?;
Some(opentype_var_glyph_variation_data_table_tuple_variation_headers_intermediate_tuples { start_tuple, end_tuple })
} else {
None
};
PResult::Ok(ret)
})())?;
opentype_var_glyph_variation_data_table_tuple_variation_headers { variation_data_size, tuple_index, peak_tuple, intermediate_tuples }
});
}
PResult::Ok(accum)
})())?;
let data = ((|| {
let _ = {
let inner = {
let x = _input.get_offset_u64();
x as u32
};
let is_valid = {
let __here = inner;
table_start + (__data_offset as u32) >= __here
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1079884235207081886u64));
};
ret
};
let tgt_offset = 0u32 + table_start + (__data_offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let shared_point_numbers = ((|| {
let ret = if tuple_variation_count.shared_point_numbers.clone() {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(byte)) => {
1
},

byte if ((ByteSet::from_bits([0, 0, 18446744073709551615, 18446744073709551615])).contains(byte)) => {
2
},

_ => {
return Err(ParseError::ExcludedBranch(14931240509007516758u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let _ = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(980800817911480223u64));
}
};
(0u16, [].to_vec())
},

1 => {
let point_count = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9042484249406774160u64));
}
};
let mut seq: Vec<opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes> = Vec::new();
let mut acc = 0u16;
loop {
if {
let totlen = acc.clone();
let _seq = &seq;
totlen >= (point_count as u16)
} {
break
}
let elem = {
let control = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_control { points_are_words: packed_bits >> 7u8 & 1u8 > 0u8, point_run_count: packed_bits >> 0u8 & 127u8 })
})())?;
let points = ((|| {
let run_length = succ(control.point_run_count.clone());
let ret = match control.points_are_words.clone() {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
accum.push((Decoder23(_input))?);
}
accum
};
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_points::Points16(inner)
},

false => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
accum.push((Decoder24(_input))?);
}
accum
};
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_points::Points8(inner)
}
};
PResult::Ok(ret)
})())?;
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes { control, points }
};
seq.push(elem.clone());
acc = {
let acc = acc;
let run = elem;
acc + (succ((run.control.point_run_count.clone()) as u16))
};
}
(acc, seq)
},

2 => {
let hi = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 18446744073709551615, 18446744073709551615])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(7801539417877429212u64));
}
};
let lo = (Decoder24(_input))?;
let mut seq: Vec<opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes> = Vec::new();
let mut acc = 0u16;
loop {
if {
let totlen = acc.clone();
let _seq = &seq;
totlen >= ((hi as u16) & 127u16) << 8u16 | (lo as u16)
} {
break
}
let elem = {
let control = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_control { points_are_words: packed_bits >> 7u8 & 1u8 > 0u8, point_run_count: packed_bits >> 0u8 & 127u8 })
})())?;
let points = ((|| {
let run_length = succ(control.point_run_count.clone());
let ret = match control.points_are_words.clone() {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
accum.push((Decoder23(_input))?);
}
accum
};
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_points::Points16(inner)
},

false => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
accum.push((Decoder24(_input))?);
}
accum
};
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_points::Points8(inner)
}
};
PResult::Ok(ret)
})())?;
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes { control, points }
};
seq.push(elem.clone());
acc = {
let acc = acc;
let run = elem;
acc + (succ((run.control.point_run_count.clone()) as u16))
};
}
(acc, seq)
},

_ => {
return Err(ParseError::ExcludedBranch(11328034188734904930u64));
}
};
Some(ret)
} else {
None
};
PResult::Ok(ret)
})())?;
let per_tuple_variation_data = ((|| {
let mut accum = Vec::new();
for header in tuple_variation_headers.clone() {
accum.push({
let sz = (header.variation_data_size.clone()) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let private_point_numbers = ((|| {
let ret = if header.tuple_index.private_point_numbers.clone() {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(byte)) => {
1
},

byte if ((ByteSet::from_bits([0, 0, 18446744073709551615, 18446744073709551615])).contains(byte)) => {
2
},

_ => {
return Err(ParseError::ExcludedBranch(10078755145706786000u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let _ = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(1338347005175300217u64));
}
};
(0u16, [].to_vec())
},

1 => {
let point_count = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(15432825464810477099u64));
}
};
let mut seq: Vec<opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes> = Vec::new();
let mut acc = 0u16;
loop {
if {
let totlen = acc.clone();
let _seq = &seq;
totlen >= (point_count as u16)
} {
break
}
let elem = {
let control = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_control { points_are_words: packed_bits >> 7u8 & 1u8 > 0u8, point_run_count: packed_bits >> 0u8 & 127u8 })
})())?;
let points = ((|| {
let run_length = succ(control.point_run_count.clone());
let ret = match control.points_are_words.clone() {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
accum.push((Decoder23(_input))?);
}
accum
};
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_points::Points16(inner)
},

false => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
accum.push((Decoder24(_input))?);
}
accum
};
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_points::Points8(inner)
}
};
PResult::Ok(ret)
})())?;
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes { control, points }
};
seq.push(elem.clone());
acc = {
let acc = acc;
let run = elem;
acc + (succ((run.control.point_run_count.clone()) as u16))
};
}
(acc, seq)
},

2 => {
let hi = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0, 0, 18446744073709551615, 18446744073709551615])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(8987822076696059625u64));
}
};
let lo = (Decoder24(_input))?;
let mut seq: Vec<opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes> = Vec::new();
let mut acc = 0u16;
loop {
if {
let totlen = acc.clone();
let _seq = &seq;
totlen >= ((hi as u16) & 127u16) << 8u16 | (lo as u16)
} {
break
}
let elem = {
let control = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_control { points_are_words: packed_bits >> 7u8 & 1u8 > 0u8, point_run_count: packed_bits >> 0u8 & 127u8 })
})())?;
let points = ((|| {
let run_length = succ(control.point_run_count.clone());
let ret = match control.points_are_words.clone() {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
accum.push((Decoder23(_input))?);
}
accum
};
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_points::Points16(inner)
},

false => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
accum.push((Decoder24(_input))?);
}
accum
};
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes_points::Points8(inner)
}
};
PResult::Ok(ret)
})())?;
opentype_var_glyph_variation_data_table_data_shared_point_numbers_yes { control, points }
};
seq.push(elem.clone());
acc = {
let acc = acc;
let run = elem;
acc + (succ((run.control.point_run_count.clone()) as u16))
};
}
(acc, seq)
},

_ => {
return Err(ParseError::ExcludedBranch(1977899765720151190u64));
}
};
Some(ret)
} else {
None
};
PResult::Ok(ret)
})())?;
let x_and_y_coordinate_deltas = ((|| {
let point_count = match private_point_numbers {
Some(ref x) => {
x.clone()
},

None => {
match shared_point_numbers {
Some(ref x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(18147521187885925800u64));
}
}
}
}.0;
let mut seq: Vec<opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas> = Vec::new();
let mut acc = 0u16;
loop {
if {
let totlen = acc.clone();
let _seq = &seq;
totlen >= point_count * 2u16
} {
break
}
let elem = {
let control = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas_control { deltas_are_zero: packed_bits >> 7u8 & 1u8 > 0u8, deltas_are_words: packed_bits >> 6u8 & 1u8 > 0u8, delta_run_count: packed_bits >> 0u8 & 63u8 })
})())?;
let deltas = ((|| {
let run_length = succ(control.delta_run_count.clone());
let ret = match control.deltas_are_zero.clone() {
true => {
let inner = run_length.clone();
opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas::Delta0(inner)
},

false => {
let ret = match control.deltas_are_words.clone() {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
accum.push((Decoder23(_input))?);
}
accum
};
opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas::Delta16(inner)
},

false => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
accum.push((Decoder24(_input))?);
}
accum
};
opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas::Delta8(inner)
}
};
ret
}
};
PResult::Ok(ret)
})())?;
opentype_var_glyph_variation_data_table_data_per_tuple_variation_data_x_and_y_coordinate_deltas { control, deltas }
};
seq.push(elem.clone());
acc = {
let acc = acc;
let run = elem;
acc + (succ((run.control.delta_run_count.clone()) as u16))
};
}
PResult::Ok((acc, seq))
})())?;
PResult::Ok(opentype_var_glyph_variation_data_table_data_per_tuple_variation_data { private_point_numbers, x_and_y_coordinate_deltas })
})())?;
_input.end_slice()?;
ret
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_var_glyph_variation_data_table_data { shared_point_numbers, per_tuple_variation_data })
})())?;
_input.close_peek_context()?;
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_var_glyph_variation_data_table { table_start, tuple_variation_count, __data_offset, tuple_variation_headers, data })
}

fn Decoder_opentype_var_variation_axis_record<>(_input: &mut Parser<'_>) -> Result<opentype_var_variation_axis_record, ParseError> {
let axis_tag = (Decoder46(_input))?;
let min_value = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
let default_value = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
let max_value = ((|| {
let x = (Decoder20(_input))?;
PResult::Ok(opentype_var_user_tuple_coordinates::Fixed32(x))
})())?;
let flags = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_var_variation_axis_record_flags { hidden_axis: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let axis_name_id = (Decoder23(_input))?;
PResult::Ok(opentype_var_variation_axis_record { axis_tag, min_value, default_value, max_value, flags, axis_name_id })
}

fn Decoder_opentype_var_user_tuple<>(_input: &mut Parser<'_>, axis_count: u16) -> Result<opentype_var_user_tuple, ParseError> {
let coordinates = ((|| {
let mut accum = Vec::new();
for _ in 0..axis_count {
accum.push({
let x = (Decoder20(_input))?;
opentype_var_user_tuple_coordinates::Fixed32(x)
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_var_user_tuple { coordinates })
}

fn Decoder_opentype_common_script_list<>(_input: &mut Parser<'_>) -> Result<opentype_common_script_list, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let script_count = (Decoder23(_input))?;
let script_records = ((|| {
let mut accum = Vec::new();
for _ in 0..script_count {
accum.push({
let script_tag = (Decoder46(_input))?;
let script = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_script_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_script_list_script_records_script { offset, link })
})())?;
opentype_common_script_list_script_records { script_tag, script }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_script_list { table_start, script_count, script_records })
}

fn Decoder_opentype_common_feature_list<>(_input: &mut Parser<'_>) -> Result<opentype_common_feature_list, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let feature_count = (Decoder23(_input))?;
let feature_records = ((|| {
let mut accum = Vec::new();
for _ in 0..feature_count {
accum.push({
let feature_tag = (Decoder46(_input))?;
let feature = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_feature_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_feature_list_feature_records_feature { offset, link })
})())?;
opentype_common_feature_list_feature_records { feature_tag, feature }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_feature_list { table_start, feature_count, feature_records })
}

fn Decoder_opentype_layout_subst_extension<>(_input: &mut Parser<'_>) -> Result<opentype_layout_subst_extension, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let format = inner;
format == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7343323033370781545u64));
};
PResult::Ok(ret)
})())?;
let extension_lookup_type = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
match x {
1u16..=6u16 => {
true
},

8 => {
true
},

_ => {
false
}
}
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12890902517277365935u64));
};
PResult::Ok(ret)
})())?;
let extension_offset = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder_opentype_layout_ground_subst(_input, extension_lookup_type.clone()))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_subst_extension_extension_offset { offset, link })
})())?;
PResult::Ok(opentype_layout_subst_extension { table_start, format, extension_lookup_type, extension_offset })
}

fn Decoder_opentype_layout_ground_subst<>(_input: &mut Parser<'_>, lookup_type: u16) -> Result<opentype_layout_ground_subst, ParseError> {
let ret = match lookup_type {
1u16 => {
let inner = (Decoder_opentype_layout_single_subst(_input))?;
opentype_layout_ground_subst::SingleSubst(inner)
},

2u16 => {
let inner = (Decoder_opentype_layout_multiple_subst(_input))?;
opentype_layout_ground_subst::MultipleSubst(inner)
},

3u16 => {
let inner = (Decoder_opentype_layout_alternate_subst(_input))?;
opentype_layout_ground_subst::AlternateSubst(inner)
},

4u16 => {
let inner = (Decoder_opentype_layout_ligature_subst(_input))?;
opentype_layout_ground_subst::LigatureSubst(inner)
},

5u16 => {
let inner = (Decoder_opentype_common_sequence_context(_input))?;
opentype_layout_ground_subst::SequenceContext(inner)
},

6u16 => {
let inner = (Decoder_opentype_common_chained_sequence_context(_input))?;
opentype_layout_ground_subst::ChainedSequenceContext(inner)
},

8u16 => {
let inner = (Decoder_opentype_layout_reverse_chain_single_subst(_input))?;
opentype_layout_ground_subst::ReverseChainSingleSubst(inner)
},

7u16 => {
return Err(ParseError::FailToken(13049534979177835905u64));
},

_ => {
return Err(ParseError::FailToken(9011855507994367971u64));
}
};
PResult::Ok(ret)
}

fn Decoder_opentype_layout_feature_variations<>(_input: &mut Parser<'_>) -> Result<opentype_layout_feature_variations, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14796083725261108356u64));
};
PResult::Ok(ret)
})())?;
let minor_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2879885114680241844u64));
};
PResult::Ok(ret)
})())?;
let feature_variation_record_count = (Decoder20(_input))?;
let feature_variation_records = ((|| {
let mut accum = Vec::new();
for _ in 0..feature_variation_record_count {
accum.push({
let condition_set_offset = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let condition_count = (Decoder23(_input))?;
let condition_offsets = ((|| {
let mut accum = Vec::new();
for _ in 0..condition_count {
accum.push({
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let format = inner;
format == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14009314771729697611u64));
};
PResult::Ok(ret)
})())?;
let axis_index = (Decoder23(_input))?;
let filter_range_min_value = ((|| {
let x = (Decoder23(_input))?;
PResult::Ok(opentype_var_tuple_record_coordinates::F2Dot14(x))
})())?;
let filter_range_max_value = ((|| {
let x = (Decoder23(_input))?;
PResult::Ok(opentype_var_tuple_record_coordinates::F2Dot14(x))
})())?;
opentype_layout_feature_variations_feature_variation_records_condition_set_offset_link_condition_offsets_link { format, axis_index, filter_range_min_value, filter_range_max_value }
};
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_feature_variations_feature_variation_records_condition_set_offset_link_condition_offsets { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_layout_feature_variations_feature_variation_records_condition_set_offset_link { table_start, condition_count, condition_offsets }
};
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_feature_variations_feature_variation_records_condition_set_offset { offset, link })
})())?;
let feature_table_substitution_offset = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let major_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10973085168168570837u64));
};
PResult::Ok(ret)
})())?;
let minor_version = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10603707580403307601u64));
};
PResult::Ok(ret)
})())?;
let substitution_count = (Decoder23(_input))?;
let substitutions = ((|| {
let mut accum = Vec::new();
for _ in 0..substitution_count {
accum.push({
let feature_index = (Decoder23(_input))?;
let alternate_feature_offset = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder_opentype_common_feature_table(_input))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset_link_substitutions_alternate_feature_offset { offset, link })
})())?;
opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset_link_substitutions { feature_index, alternate_feature_offset }
});
}
PResult::Ok(accum)
})())?;
opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset_link { table_start, major_version, minor_version, substitution_count, substitutions }
};
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_feature_variations_feature_variation_records_feature_table_substitution_offset { offset, link })
})())?;
opentype_layout_feature_variations_feature_variation_records { condition_set_offset, feature_table_substitution_offset }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_feature_variations { table_start, major_version, minor_version, feature_variation_record_count, feature_variation_records })
}

fn Decoder_opentype_common_feature_table<>(_input: &mut Parser<'_>) -> Result<opentype_common_feature_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let feature_params = (Decoder23(_input))?;
let lookup_index_count = (Decoder23(_input))?;
let lookup_list_indices = ((|| {
let mut accum = Vec::new();
for _ in 0..lookup_index_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_feature_table { table_start, feature_params, lookup_index_count, lookup_list_indices })
}

fn Decoder_opentype_layout_single_subst<>(_input: &mut Parser<'_>) -> Result<opentype_layout_single_subst, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let subst_format = (Decoder23(_input))?;
let subst = ((|| {
let ret = match subst_format {
1u16 => {
let inner = {
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let delta_glyph_id = (Decoder23(_input))?;
opentype_layout_single_subst_subst_Format1 { coverage, delta_glyph_id }
};
opentype_layout_single_subst_subst::Format1(inner)
},

2u16 => {
let inner = {
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let glyph_count = (Decoder23(_input))?;
let substitute_glyph_ids = ((|| {
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
opentype_layout_single_subst_subst_Format2 { coverage, glyph_count, substitute_glyph_ids }
};
opentype_layout_single_subst_subst::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(18065118697073160549u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_single_subst { table_start, subst_format, subst })
}

fn Decoder_opentype_layout_multiple_subst<>(_input: &mut Parser<'_>) -> Result<opentype_layout_multiple_subst, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let subst_format = (Decoder23(_input))?;
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let subst = ((|| {
let ret = match subst_format {
1u16 => {
let inner = {
let sequence_count = (Decoder23(_input))?;
let sequences = ((|| {
let mut accum = Vec::new();
for _ in 0..sequence_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let glyph_count = (Decoder23(_input))?;
let substitute_glyph_ids = ((|| {
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_multiple_subst_subst_Format1_sequences_link { glyph_count, substitute_glyph_ids })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_multiple_subst_subst_Format1_sequences { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_layout_multiple_subst_subst_Format1 { sequence_count, sequences }
};
opentype_layout_multiple_subst_subst::Format1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_multiple_subst { table_start, subst_format, coverage, subst })
}

fn Decoder_opentype_layout_alternate_subst<>(_input: &mut Parser<'_>) -> Result<opentype_layout_alternate_subst, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let subst_format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let subst_format = inner;
subst_format == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10686389193617118447u64));
};
PResult::Ok(ret)
})())?;
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let alternate_set_count = (Decoder23(_input))?;
let alternate_sets = ((|| {
let mut accum = Vec::new();
for _ in 0..alternate_set_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let glyph_count = (Decoder23(_input))?;
let alternate_glyph_ids = ((|| {
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_alternate_subst_alternate_sets_link { glyph_count, alternate_glyph_ids })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_alternate_subst_alternate_sets { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_alternate_subst { table_start, subst_format, coverage, alternate_set_count, alternate_sets })
}

fn Decoder_opentype_layout_ligature_subst<>(_input: &mut Parser<'_>) -> Result<opentype_layout_ligature_subst, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let subst_format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let subst_format = inner;
subst_format == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(16128388243093908143u64));
};
PResult::Ok(ret)
})())?;
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let ligature_set_count = (Decoder23(_input))?;
let ligature_sets = ((|| {
let mut accum = Vec::new();
for _ in 0..ligature_set_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let ligature_count = (Decoder23(_input))?;
let ligatures = ((|| {
let mut accum = Vec::new();
for _ in 0..ligature_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let ligature_glyph = (Decoder23(_input))?;
let component_count = (Decoder23(_input))?;
let component_glyph_ids = ((|| {
let mut accum = Vec::new();
for _ in 0..pred(component_count) {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_ligature_subst_ligature_sets_link_ligatures_link { ligature_glyph, component_count, component_glyph_ids })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_ligature_subst_ligature_sets_link_ligatures { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_ligature_subst_ligature_sets_link { table_start, ligature_count, ligatures })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_ligature_subst_ligature_sets { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_ligature_subst { table_start, subst_format, coverage, ligature_set_count, ligature_sets })
}

fn Decoder_opentype_common_sequence_context<>(_input: &mut Parser<'_>) -> Result<opentype_common_sequence_context, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let format = (Decoder23(_input))?;
let subst = ((|| {
let ret = match format {
1u16 => {
let inner = {
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let seq_rule_set_count = (Decoder23(_input))?;
let seq_rule_sets = ((|| {
let mut accum = Vec::new();
for _ in 0..seq_rule_set_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let rule_count = (Decoder23(_input))?;
let rules = ((|| {
let mut accum = Vec::new();
for _ in 0..rule_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let glyph_count = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x != 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2818918064991511645u64));
};
PResult::Ok(ret)
})())?;
let seq_lookup_count = (Decoder23(_input))?;
let input_sequence = ((|| {
let mut accum = Vec::new();
for _ in 0..pred(glyph_count) {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let seq_lookup_records = ((|| {
let mut accum = Vec::new();
for _ in 0..seq_lookup_count {
accum.push((Decoder_opentype_common_sequence_lookup(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_sequence_context_subst_Format1_seq_rule_sets_link_rules_link { glyph_count, seq_lookup_count, input_sequence, seq_lookup_records })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_common_sequence_context_subst_Format1_seq_rule_sets_link_rules { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_sequence_context_subst_Format1_seq_rule_sets_link { table_start, rule_count, rules })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_common_sequence_context_subst_Format1_seq_rule_sets { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_common_sequence_context_subst_Format1 { coverage, seq_rule_set_count, seq_rule_sets }
};
opentype_common_sequence_context_subst::Format1(inner)
},

2u16 => {
let inner = {
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let class_def = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_class_def(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_glyph_class_def { offset, link })
})())?;
let class_seq_rule_set_count = (Decoder23(_input))?;
let class_seq_rule_sets = ((|| {
let mut accum = Vec::new();
for _ in 0..class_seq_rule_set_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let rule_count = (Decoder23(_input))?;
let rules = ((|| {
let mut accum = Vec::new();
for _ in 0..rule_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let glyph_count = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x != 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14082539304789607227u64));
};
PResult::Ok(ret)
})())?;
let seq_lookup_count = (Decoder23(_input))?;
let input_sequence = ((|| {
let mut accum = Vec::new();
for _ in 0..pred(glyph_count) {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let seq_lookup_records = ((|| {
let mut accum = Vec::new();
for _ in 0..seq_lookup_count {
accum.push((Decoder_opentype_common_sequence_lookup(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_sequence_context_subst_Format1_seq_rule_sets_link_rules_link { glyph_count, seq_lookup_count, input_sequence, seq_lookup_records })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_common_sequence_context_subst_Format1_seq_rule_sets_link_rules { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_sequence_context_subst_Format1_seq_rule_sets_link { table_start, rule_count, rules })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_common_sequence_context_subst_Format1_seq_rule_sets { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_common_sequence_context_subst_Format2 { coverage, class_def, class_seq_rule_set_count, class_seq_rule_sets }
};
opentype_common_sequence_context_subst::Format2(inner)
},

3u16 => {
let inner = {
let glyph_count = (Decoder23(_input))?;
let seq_lookup_count = (Decoder23(_input))?;
let coverage_tables = ((|| {
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_reverse_chain_single_subst_coverage { offset, link }
});
}
PResult::Ok(accum)
})())?;
let seq_lookup_records = ((|| {
let mut accum = Vec::new();
for _ in 0..seq_lookup_count {
accum.push((Decoder_opentype_common_sequence_lookup(_input))?);
}
PResult::Ok(accum)
})())?;
opentype_common_sequence_context_subst_Format3 { glyph_count, seq_lookup_count, coverage_tables, seq_lookup_records }
};
opentype_common_sequence_context_subst::Format3(inner)
},

_ => {
return Err(ParseError::FailToken(11072034178440885507u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_sequence_context { table_start, format, subst })
}

fn Decoder_opentype_common_chained_sequence_context<>(_input: &mut Parser<'_>) -> Result<opentype_common_chained_sequence_context, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let format = (Decoder23(_input))?;
let subst = ((|| {
let ret = match format {
1u16 => {
let inner = {
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let chained_seq_rule_set_count = (Decoder23(_input))?;
let chained_seq_rule_sets = ((|| {
let mut accum = Vec::new();
for _ in 0..chained_seq_rule_set_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let chained_seq_rule_count = (Decoder23(_input))?;
let chained_seq_rules = ((|| {
let mut accum = Vec::new();
for _ in 0..chained_seq_rule_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let backtrack_glyph_count = (Decoder23(_input))?;
let backtrack_sequence = ((|| {
let mut accum = Vec::new();
for _ in 0..backtrack_glyph_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let input_glyph_count = (Decoder23(_input))?;
let input_sequence = ((|| {
let mut accum = Vec::new();
for _ in 0..pred(input_glyph_count) {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let lookahead_glyph_count = (Decoder23(_input))?;
let lookahead_sequence = ((|| {
let mut accum = Vec::new();
for _ in 0..lookahead_glyph_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let seq_lookup_count = (Decoder23(_input))?;
let seq_lookup_records = ((|| {
let mut accum = Vec::new();
for _ in 0..seq_lookup_count {
accum.push((Decoder_opentype_common_sequence_lookup(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link_chained_seq_rules_link { backtrack_glyph_count, backtrack_sequence, input_glyph_count, input_sequence, lookahead_glyph_count, lookahead_sequence, seq_lookup_count, seq_lookup_records })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link_chained_seq_rules { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link { table_start, chained_seq_rule_count, chained_seq_rules })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_common_chained_sequence_context_subst_Format1 { coverage, chained_seq_rule_set_count, chained_seq_rule_sets }
};
opentype_common_chained_sequence_context_subst::Format1(inner)
},

2u16 => {
let inner = {
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let backtrack_class_def = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_class_def(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_glyph_class_def { offset, link })
})())?;
let input_class_def = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_class_def(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_glyph_class_def { offset, link })
})())?;
let lookahead_class_def = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_class_def(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_glyph_class_def { offset, link })
})())?;
let chained_class_seq_rule_set_count = (Decoder23(_input))?;
let chained_class_seq_rule_sets = ((|| {
let mut accum = Vec::new();
for _ in 0..chained_class_seq_rule_set_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let chained_seq_rule_count = (Decoder23(_input))?;
let chained_seq_rules = ((|| {
let mut accum = Vec::new();
for _ in 0..chained_seq_rule_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let backtrack_glyph_count = (Decoder23(_input))?;
let backtrack_sequence = ((|| {
let mut accum = Vec::new();
for _ in 0..backtrack_glyph_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let input_glyph_count = (Decoder23(_input))?;
let input_sequence = ((|| {
let mut accum = Vec::new();
for _ in 0..pred(input_glyph_count) {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let lookahead_glyph_count = (Decoder23(_input))?;
let lookahead_sequence = ((|| {
let mut accum = Vec::new();
for _ in 0..lookahead_glyph_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let seq_lookup_count = (Decoder23(_input))?;
let seq_lookup_records = ((|| {
let mut accum = Vec::new();
for _ in 0..seq_lookup_count {
accum.push((Decoder_opentype_common_sequence_lookup(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link_chained_seq_rules_link { backtrack_glyph_count, backtrack_sequence, input_glyph_count, input_sequence, lookahead_glyph_count, lookahead_sequence, seq_lookup_count, seq_lookup_records })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link_chained_seq_rules { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link { table_start, chained_seq_rule_count, chained_seq_rules })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_common_chained_sequence_context_subst_Format2 { coverage, backtrack_class_def, input_class_def, lookahead_class_def, chained_class_seq_rule_set_count, chained_class_seq_rule_sets }
};
opentype_common_chained_sequence_context_subst::Format2(inner)
},

3u16 => {
let inner = {
let backtrack_glyph_count = (Decoder23(_input))?;
let backtrack_coverages = ((|| {
let mut accum = Vec::new();
for _ in 0..backtrack_glyph_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_reverse_chain_single_subst_coverage { offset, link }
});
}
PResult::Ok(accum)
})())?;
let input_glyph_count = (Decoder23(_input))?;
let input_coverages = ((|| {
let mut accum = Vec::new();
for _ in 0..input_glyph_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_reverse_chain_single_subst_coverage { offset, link }
});
}
PResult::Ok(accum)
})())?;
let lookahead_glyph_count = (Decoder23(_input))?;
let lookahead_coverages = ((|| {
let mut accum = Vec::new();
for _ in 0..lookahead_glyph_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_reverse_chain_single_subst_coverage { offset, link }
});
}
PResult::Ok(accum)
})())?;
let seq_lookup_count = (Decoder23(_input))?;
let seq_lookup_records = ((|| {
let mut accum = Vec::new();
for _ in 0..seq_lookup_count {
accum.push((Decoder_opentype_common_sequence_lookup(_input))?);
}
PResult::Ok(accum)
})())?;
opentype_common_chained_sequence_context_subst_Format3 { backtrack_glyph_count, backtrack_coverages, input_glyph_count, input_coverages, lookahead_glyph_count, lookahead_coverages, seq_lookup_count, seq_lookup_records }
};
opentype_common_chained_sequence_context_subst::Format3(inner)
},

_ => {
return Err(ParseError::FailToken(4608405370414018463u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_chained_sequence_context { table_start, format, subst })
}

fn Decoder_opentype_layout_reverse_chain_single_subst<>(_input: &mut Parser<'_>) -> Result<opentype_layout_reverse_chain_single_subst, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let subst_format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let subst_format = inner;
subst_format == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4418518334087228745u64));
};
PResult::Ok(ret)
})())?;
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let backtrack_glyph_count = (Decoder23(_input))?;
let backtrack_coverage_tables = ((|| {
let mut accum = Vec::new();
for _ in 0..backtrack_glyph_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_reverse_chain_single_subst_coverage { offset, link }
});
}
PResult::Ok(accum)
})())?;
let lookahead_glyph_count = (Decoder23(_input))?;
let lookahead_coverage_tables = ((|| {
let mut accum = Vec::new();
for _ in 0..lookahead_glyph_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_reverse_chain_single_subst_coverage { offset, link }
});
}
PResult::Ok(accum)
})())?;
let glyph_count = (Decoder23(_input))?;
let substitute_glyph_ids = ((|| {
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst { table_start, subst_format, coverage, backtrack_glyph_count, backtrack_coverage_tables, lookahead_glyph_count, lookahead_coverage_tables, glyph_count, substitute_glyph_ids })
}

fn Decoder_opentype_coverage_table<>(_input: &mut Parser<'_>) -> Result<opentype_coverage_table, ParseError> {
let coverage_format = (Decoder23(_input))?;
let data = ((|| {
let ret = match coverage_format {
1u16 => {
let inner = {
let glyph_count = (Decoder23(_input))?;
let glyph_array = ((|| {
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
opentype_coverage_table_data_Format1 { glyph_count, glyph_array }
};
opentype_coverage_table_data::Format1(inner)
},

2u16 => {
let inner = {
let range_count = (Decoder23(_input))?;
let range_records = ((|| {
let mut accum = Vec::new();
for _ in 0..range_count {
accum.push({
let start_glyph_id = (Decoder23(_input))?;
let end_glyph_id = (Decoder23(_input))?;
let start_coverage_index = (Decoder23(_input))?;
opentype_coverage_table_data_Format2_range_records { start_glyph_id, end_glyph_id, start_coverage_index }
});
}
PResult::Ok(accum)
})())?;
opentype_coverage_table_data_Format2 { range_count, range_records }
};
opentype_coverage_table_data::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(7086880279337729577u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_coverage_table { coverage_format, data })
}

fn Decoder_opentype_common_sequence_lookup<>(_input: &mut Parser<'_>) -> Result<opentype_common_sequence_lookup, ParseError> {
let sequence_index = (Decoder23(_input))?;
let lookup_list_index = (Decoder23(_input))?;
PResult::Ok(opentype_common_sequence_lookup { sequence_index, lookup_list_index })
}

fn Decoder_opentype_class_def<>(_input: &mut Parser<'_>) -> Result<opentype_class_def, ParseError> {
let class_format = (Decoder23(_input))?;
let data = ((|| {
let ret = match class_format {
1u16 => {
let inner = {
let start_glyph_id = (Decoder23(_input))?;
let glyph_count = (Decoder23(_input))?;
let class_value_array = ((|| {
let mut accum = Vec::new();
for _ in 0..glyph_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
opentype_class_def_data_Format1 { start_glyph_id, glyph_count, class_value_array }
};
opentype_class_def_data::Format1(inner)
},

2u16 => {
let inner = {
let class_range_count = (Decoder23(_input))?;
let class_range_records = ((|| {
let mut accum = Vec::new();
for _ in 0..class_range_count {
accum.push({
let start_glyph_id = (Decoder23(_input))?;
let end_glyph_id = (Decoder23(_input))?;
let class = (Decoder23(_input))?;
opentype_class_def_data_Format2_class_range_records { start_glyph_id, end_glyph_id, class }
});
}
PResult::Ok(accum)
})())?;
opentype_class_def_data_Format2 { class_range_count, class_range_records }
};
opentype_class_def_data::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(7511456693437940214u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_class_def { class_format, data })
}

fn Decoder_opentype_common_script_table<>(_input: &mut Parser<'_>) -> Result<opentype_common_script_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let default_lang_sys = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_langsys(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_script_table_default_lang_sys { offset, link })
})())?;
let lang_sys_count = (Decoder23(_input))?;
let lang_sys_records = ((|| {
let mut accum = Vec::new();
for _ in 0..lang_sys_count {
accum.push({
let lang_sys_tag = (Decoder46(_input))?;
let lang_sys = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_langsys(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_script_table_default_lang_sys { offset, link })
})())?;
opentype_common_script_table_lang_sys_records { lang_sys_tag, lang_sys }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_script_table { table_start, default_lang_sys, lang_sys_count, lang_sys_records })
}

fn Decoder_opentype_common_langsys<>(_input: &mut Parser<'_>) -> Result<opentype_common_langsys, ParseError> {
let lookup_order_offset = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(973408085875818710u64));
};
PResult::Ok(ret)
})())?;
let required_feature_index = (Decoder23(_input))?;
let feature_index_count = (Decoder23(_input))?;
let feature_indices = ((|| {
let mut accum = Vec::new();
for _ in 0..feature_index_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_common_langsys { lookup_order_offset, required_feature_index, feature_index_count, feature_indices })
}

fn Decoder_opentype_layout_pos_extension<>(_input: &mut Parser<'_>) -> Result<opentype_layout_pos_extension, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let format = inner;
format == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(15557503981608772456u64));
};
PResult::Ok(ret)
})())?;
let extension_lookup_type = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
match x {
1u16..=8u16 => {
true
},

_ => {
false
}
}
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2154669163482751322u64));
};
PResult::Ok(ret)
})())?;
let extension_offset = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder_opentype_layout_ground_pos(_input, extension_lookup_type.clone()))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_pos_extension_extension_offset { offset, link })
})())?;
PResult::Ok(opentype_layout_pos_extension { table_start, format, extension_lookup_type, extension_offset })
}

fn Decoder_opentype_layout_ground_pos<>(_input: &mut Parser<'_>, lookup_type: u16) -> Result<opentype_layout_ground_pos, ParseError> {
let ret = match lookup_type {
1u16 => {
let inner = (Decoder_opentype_layout_single_pos(_input))?;
opentype_layout_ground_pos::SinglePos(inner)
},

2u16 => {
let inner = (Decoder_opentype_layout_pair_pos(_input))?;
opentype_layout_ground_pos::PairPos(inner)
},

3u16 => {
let inner = (Decoder_opentype_layout_cursive_pos(_input))?;
opentype_layout_ground_pos::CursivePos(inner)
},

4u16 => {
let inner = (Decoder_opentype_layout_mark_base_pos(_input))?;
opentype_layout_ground_pos::MarkBasePos(inner)
},

5u16 => {
let inner = (Decoder_opentype_layout_mark_lig_pos(_input))?;
opentype_layout_ground_pos::MarkLigPos(inner)
},

6u16 => {
let inner = (Decoder_opentype_layout_mark_mark_pos(_input))?;
opentype_layout_ground_pos::MarkMarkPos(inner)
},

7u16 => {
let inner = (Decoder_opentype_common_sequence_context(_input))?;
opentype_layout_ground_pos::SequenceContext(inner)
},

8u16 => {
let inner = (Decoder_opentype_common_chained_sequence_context(_input))?;
opentype_layout_ground_pos::ChainedSequenceContext(inner)
},

9u16 => {
return Err(ParseError::FailToken(10263667190582992611u64));
},

_ => {
return Err(ParseError::FailToken(5482396765248532989u64));
}
};
PResult::Ok(ret)
}

fn Decoder_opentype_layout_single_pos<>(_input: &mut Parser<'_>) -> Result<opentype_layout_single_pos, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let pos_format = (Decoder23(_input))?;
let subtable = ((|| {
let ret = match pos_format {
1u16 => {
let inner = {
let coverage_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let value_format = (Decoder_opentype_common_value_format_flags(_input))?;
let value_record = (Decoder82(_input, table_start.clone(), value_format.clone()))?;
opentype_layout_single_pos_subtable_Format1 { coverage_offset, value_format, value_record }
};
opentype_layout_single_pos_subtable::Format1(inner)
},

2u16 => {
let inner = {
let coverage_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let value_format = (Decoder_opentype_common_value_format_flags(_input))?;
let value_count = (Decoder23(_input))?;
let value_records = ((|| {
let mut accum = Vec::new();
for _ in 0..value_count {
accum.push((Decoder82(_input, table_start.clone(), value_format.clone()))?);
}
PResult::Ok(accum)
})())?;
opentype_layout_single_pos_subtable_Format2 { coverage_offset, value_format, value_count, value_records }
};
opentype_layout_single_pos_subtable::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(12275201028130973875u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_single_pos { table_start, pos_format, subtable })
}

fn Decoder_opentype_layout_pair_pos<>(_input: &mut Parser<'_>) -> Result<opentype_layout_pair_pos, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let pos_format = (Decoder23(_input))?;
let subtable = ((|| {
let ret = match pos_format {
1u16 => {
let inner = {
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let value_format1 = (Decoder_opentype_common_value_format_flags(_input))?;
let value_format2 = (Decoder_opentype_common_value_format_flags(_input))?;
let pair_set_count = (Decoder23(_input))?;
let pair_sets = ((|| {
let mut accum = Vec::new();
for _ in 0..pair_set_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let pair_value_count = (Decoder23(_input))?;
let pair_value_records = ((|| {
let mut accum = Vec::new();
for _ in 0..pair_value_count {
accum.push({
let second_glyph = (Decoder23(_input))?;
let value_record1 = ((|| {
let ret = if value_format1.x_placement.clone() || value_format1.y_placement.clone() || value_format1.x_advance.clone() || value_format1.y_advance.clone() || value_format1.x_placement_device.clone() || value_format1.y_placement_device.clone() || value_format1.x_advance_device.clone() || value_format1.y_advance_device.clone() {
Some((Decoder_opentype_common_value_record(_input, table_start.clone(), value_format1.clone()))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let value_record2 = ((|| {
let ret = if value_format2.x_placement.clone() || value_format2.y_placement.clone() || value_format2.x_advance.clone() || value_format2.y_advance.clone() || value_format2.x_placement_device.clone() || value_format2.y_placement_device.clone() || value_format2.x_advance_device.clone() || value_format2.y_advance_device.clone() {
Some((Decoder82(_input, table_start.clone(), value_format2.clone()))?)
} else {
None
};
PResult::Ok(ret)
})())?;
opentype_layout_pair_pos_subtable_Format1_pair_sets_link_pair_value_records { second_glyph, value_record1, value_record2 }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_pair_pos_subtable_Format1_pair_sets_link { table_start, pair_value_count, pair_value_records })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_pair_pos_subtable_Format1_pair_sets { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_layout_pair_pos_subtable_Format1 { coverage, value_format1, value_format2, pair_set_count, pair_sets }
};
opentype_layout_pair_pos_subtable::Format1(inner)
},

2u16 => {
let inner = {
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let value_format1 = (Decoder_opentype_common_value_format_flags(_input))?;
let value_format2 = (Decoder_opentype_common_value_format_flags(_input))?;
let class_def1 = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_class_def(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_glyph_class_def { offset, link })
})())?;
let class_def2 = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_class_def(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_gdef_table_glyph_class_def { offset, link })
})())?;
let class1_count = (Decoder23(_input))?;
let class2_count = (Decoder23(_input))?;
let class1_records = ((|| {
let mut accum = Vec::new();
for _ in 0..class1_count {
accum.push({
let class2_records = ((|| {
let mut accum = Vec::new();
for _ in 0..class2_count {
accum.push({
let value_record1 = ((|| {
let ret = if value_format1.x_placement.clone() || value_format1.y_placement.clone() || value_format1.x_advance.clone() || value_format1.y_advance.clone() || value_format1.x_placement_device.clone() || value_format1.y_placement_device.clone() || value_format1.x_advance_device.clone() || value_format1.y_advance_device.clone() {
Some((Decoder83(_input, table_start.clone(), value_format1.clone()))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let value_record2 = ((|| {
let ret = if value_format2.x_placement.clone() || value_format2.y_placement.clone() || value_format2.x_advance.clone() || value_format2.y_advance.clone() || value_format2.x_placement_device.clone() || value_format2.y_placement_device.clone() || value_format2.x_advance_device.clone() || value_format2.y_advance_device.clone() {
Some((Decoder84(_input, table_start.clone(), value_format2.clone()))?)
} else {
None
};
PResult::Ok(ret)
})())?;
opentype_layout_pair_pos_subtable_Format2_class1_records_class2_records { value_record1, value_record2 }
});
}
PResult::Ok(accum)
})())?;
opentype_layout_pair_pos_subtable_Format2_class1_records { class2_records }
});
}
PResult::Ok(accum)
})())?;
opentype_layout_pair_pos_subtable_Format2 { coverage, value_format1, value_format2, class_def1, class_def2, class1_count, class2_count, class1_records }
};
opentype_layout_pair_pos_subtable::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(16097120758067046920u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_pair_pos { table_start, pos_format, subtable })
}

fn Decoder_opentype_layout_cursive_pos<>(_input: &mut Parser<'_>) -> Result<opentype_layout_cursive_pos, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let pos_format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let pos_format = inner;
pos_format == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9331632426086095927u64));
};
PResult::Ok(ret)
})())?;
let coverage = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let entry_exit_count = (Decoder23(_input))?;
let entry_exit_records = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_exit_count {
accum.push({
let entry_anchor = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_anchor_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_mark_array_mark_records_mark_anchor_offset { offset, link })
})())?;
let exit_anchor = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_anchor_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_mark_array_mark_records_mark_anchor_offset { offset, link })
})())?;
opentype_layout_cursive_pos_entry_exit_records { entry_anchor, exit_anchor }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_cursive_pos { table_start, pos_format, coverage, entry_exit_count, entry_exit_records })
}

fn Decoder_opentype_layout_mark_base_pos<>(_input: &mut Parser<'_>) -> Result<opentype_layout_mark_base_pos, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let format = inner;
format == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14959848987246965519u64));
};
PResult::Ok(ret)
})())?;
let mark_coverage_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let base_coverage_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let mark_class_count = (Decoder23(_input))?;
let mark_array_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_mark_array(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_mark_mark_pos_mark1_array_offset { offset, link })
})())?;
let base_array_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let base_count = (Decoder23(_input))?;
let base_records = ((|| {
let mut accum = Vec::new();
for _ in 0..base_count {
accum.push({
let base_anchor_offsets = ((|| {
let mut accum = Vec::new();
for _ in 0..mark_class_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_anchor_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_mark_array_mark_records_mark_anchor_offset { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_layout_mark_base_pos_base_array_offset_link_base_records { base_anchor_offsets }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_mark_base_pos_base_array_offset_link { table_start, base_count, base_records })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_mark_base_pos_base_array_offset { offset, link })
})())?;
PResult::Ok(opentype_layout_mark_base_pos { table_start, format, mark_coverage_offset, base_coverage_offset, mark_class_count, mark_array_offset, base_array_offset })
}

fn Decoder_opentype_layout_mark_lig_pos<>(_input: &mut Parser<'_>) -> Result<opentype_layout_mark_lig_pos, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let format = inner;
format == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9092905213558799443u64));
};
PResult::Ok(ret)
})())?;
let mark_coverage_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let ligature_coverage_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let mark_class_count = (Decoder23(_input))?;
let mark_array_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_mark_array(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_mark_mark_pos_mark1_array_offset { offset, link })
})())?;
let ligature_array_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let ligature_count = (Decoder23(_input))?;
let ligature_attach_offsets = ((|| {
let mut accum = Vec::new();
for _ in 0..ligature_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let component_count = (Decoder23(_input))?;
let component_records = ((|| {
let mut accum = Vec::new();
for _ in 0..component_count {
accum.push({
let ligature_anchor_offsets = ((|| {
let mut accum = Vec::new();
for _ in 0..mark_class_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_anchor_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_mark_array_mark_records_mark_anchor_offset { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets_link_component_records { ligature_anchor_offsets }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets_link { table_start, component_count, component_records })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_mark_lig_pos_ligature_array_offset_link { table_start, ligature_count, ligature_attach_offsets })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_mark_lig_pos_ligature_array_offset { offset, link })
})())?;
PResult::Ok(opentype_layout_mark_lig_pos { table_start, format, mark_coverage_offset, ligature_coverage_offset, mark_class_count, mark_array_offset, ligature_array_offset })
}

fn Decoder_opentype_layout_mark_mark_pos<>(_input: &mut Parser<'_>) -> Result<opentype_layout_mark_mark_pos, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let format = inner;
format == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17544092807091201u64));
};
PResult::Ok(ret)
})())?;
let mark1_coverage_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let mark2_coverage_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_coverage_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_reverse_chain_single_subst_coverage { offset, link })
})())?;
let mark_class_count = (Decoder23(_input))?;
let mark1_array_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_mark_array(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_mark_mark_pos_mark1_array_offset { offset, link })
})())?;
let mark2_array_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let mark2_count = (Decoder23(_input))?;
let mark2_records = ((|| {
let mut accum = Vec::new();
for _ in 0..mark2_count {
accum.push({
let mark2_anchor_offsets = ((|| {
let mut accum = Vec::new();
for _ in 0..mark_class_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_anchor_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_mark_array_mark_records_mark_anchor_offset { offset, link }
});
}
PResult::Ok(accum)
})())?;
opentype_layout_mark_mark_pos_mark2_array_offset_link_mark2_records { mark2_anchor_offsets }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_mark_mark_pos_mark2_array_offset_link { table_start, mark2_count, mark2_records })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_mark_mark_pos_mark2_array_offset { offset, link })
})())?;
PResult::Ok(opentype_layout_mark_mark_pos { table_start, format, mark1_coverage_offset, mark2_coverage_offset, mark_class_count, mark1_array_offset, mark2_array_offset })
}

fn Decoder_opentype_layout_mark_array<>(_input: &mut Parser<'_>) -> Result<opentype_layout_mark_array, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let mark_count = (Decoder23(_input))?;
let mark_records = ((|| {
let mut accum = Vec::new();
for _ in 0..mark_count {
accum.push({
let mark_class = (Decoder23(_input))?;
let mark_anchor_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_anchor_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_mark_array_mark_records_mark_anchor_offset { offset, link })
})())?;
opentype_layout_mark_array_mark_records { mark_class, mark_anchor_offset }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_mark_array { table_start, mark_count, mark_records })
}

fn Decoder_opentype_common_anchor_table<>(_input: &mut Parser<'_>) -> Result<opentype_common_anchor_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let anchor_format = (Decoder23(_input))?;
let table = ((|| {
let ret = match anchor_format {
1u16 => {
let inner = {
let x_coordinate = (Decoder23(_input))?;
let y_coordinate = (Decoder23(_input))?;
opentype_common_anchor_table_table_Format1 { x_coordinate, y_coordinate }
};
opentype_common_anchor_table_table::Format1(inner)
},

2u16 => {
let inner = {
let x_coordinate = (Decoder23(_input))?;
let y_coordinate = (Decoder23(_input))?;
let anchor_point = (Decoder23(_input))?;
opentype_common_anchor_table_table_Format2 { x_coordinate, y_coordinate, anchor_point }
};
opentype_common_anchor_table_table::Format2(inner)
},

3u16 => {
let inner = {
let x_coordinate = (Decoder23(_input))?;
let y_coordinate = (Decoder23(_input))?;
let x_device_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_value_record_x_advance_device { offset, link })
})())?;
let y_device_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_value_record_x_advance_device { offset, link })
})())?;
opentype_common_anchor_table_table_Format3 { x_coordinate, y_coordinate, x_device_offset, y_device_offset }
};
opentype_common_anchor_table_table::Format3(inner)
},

_ => {
return Err(ParseError::FailToken(10502127387712395480u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_anchor_table { table_start, anchor_format, table })
}

fn Decoder_opentype_common_device_or_variation_index_table<>(_input: &mut Parser<'_>) -> Result<opentype_common_device_or_variation_index_table, ParseError> {
let delta_format = {
_input.open_peek_context();
let ret = ((|| {
let _ = {
let __skipped0 = (Decoder23(_input))?;
let __skipped1 = (Decoder23(_input))?;
opentype_common_device_or_variation_index_table_raw_raw { __skipped0, __skipped1 }
};
Decoder23(_input)
})())?;
_input.close_peek_context()?;
ret
};
let ret = match delta_format {
1u16..=3u16 => {
let inner = {
let start_size = (Decoder23(_input))?;
let end_size = (Decoder23(_input))?;
let delta_format = (Decoder23(_input))?;
let delta_values = ((|| {
let mut accum = Vec::new();
for _ in 0..match delta_format {
1u16 => {
match ((succ(try_sub!(end_size, start_size, 7364705619221056123u64))) / 8u16) * 8u16 < (succ(try_sub!(end_size, start_size, 2404222719611925354u64))) {
true => {
succ((succ(try_sub!(end_size, start_size, 13091357170910775568u64))) / 8u16)
},

false => {
(succ(try_sub!(end_size, start_size, 17170585774888887431u64))) / 8u16
}
}
},

2u16 => {
match ((succ(try_sub!(end_size, start_size, 1548601315919054830u64))) / 4u16) * 4u16 < (succ(try_sub!(end_size, start_size, 9339905250237811640u64))) {
true => {
succ((succ(try_sub!(end_size, start_size, 6039633234730737119u64))) / 4u16)
},

false => {
(succ(try_sub!(end_size, start_size, 1654541969082323602u64))) / 4u16
}
}
},

3u16 => {
match ((succ(try_sub!(end_size, start_size, 1272171534487716374u64))) / 2u16) * 2u16 < (succ(try_sub!(end_size, start_size, 7000022209644146403u64))) {
true => {
succ((succ(try_sub!(end_size, start_size, 8451221290566481190u64))) / 2u16)
},

false => {
(succ(try_sub!(end_size, start_size, 7283163102885684771u64))) / 2u16
}
}
},

_ => {
0u16
}
} {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
opentype_common_device_or_variation_index_table_DeviceTable { start_size, end_size, delta_format, delta_values }
};
opentype_common_device_or_variation_index_table::DeviceTable(inner)
},

32768u16 => {
let inner = {
let delta_set_outer_index = (Decoder23(_input))?;
let delta_set_inner_index = (Decoder23(_input))?;
let delta_format = ((|| {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 128 {
b
} else {
return Err(ParseError::ExcludedBranch(14454034443522724586u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(5322124757500927073u64));
})
})())?;
PResult::Ok((field0, field1))
})())?;
opentype_common_device_or_variation_index_table_VariationIndexTable { delta_set_outer_index, delta_set_inner_index, delta_format }
};
opentype_common_device_or_variation_index_table::VariationIndexTable(inner)
},

other => {
let inner = {
let field0 = (Decoder23(_input))?;
let field1 = (Decoder23(_input))?;
let delta_format = other.clone();
opentype_common_device_or_variation_index_table_OtherTable { field0, field1, delta_format }
};
opentype_common_device_or_variation_index_table::OtherTable(inner)
}
};
PResult::Ok(ret)
}

fn Decoder_opentype_common_value_format_flags<>(_input: &mut Parser<'_>) -> Result<opentype_common_value_format_flags, ParseError> {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_common_value_format_flags { y_advance_device: packed_bits >> 7u16 & 1u16 > 0u16, x_advance_device: packed_bits >> 6u16 & 1u16 > 0u16, y_placement_device: packed_bits >> 5u16 & 1u16 > 0u16, x_placement_device: packed_bits >> 4u16 & 1u16 > 0u16, y_advance: packed_bits >> 3u16 & 1u16 > 0u16, x_advance: packed_bits >> 2u16 & 1u16 > 0u16, y_placement: packed_bits >> 1u16 & 1u16 > 0u16, x_placement: packed_bits >> 0u16 & 1u16 > 0u16 })
}

fn Decoder_opentype_common_value_record<>(_input: &mut Parser<'_>, table_start: u32, flags: opentype_common_value_format_flags) -> Result<opentype_common_value_record, ParseError> {
let x_placement = ((|| {
let ret = if flags.x_placement.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let y_placement = ((|| {
let ret = if flags.y_placement.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let x_advance = ((|| {
let ret = if flags.x_advance.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let y_advance = ((|| {
let ret = if flags.y_advance.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let x_placement_device = ((|| {
let ret = if flags.x_placement_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let y_placement_device = ((|| {
let ret = if flags.y_placement_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let x_advance_device = ((|| {
let ret = if flags.x_advance_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let y_advance_device = ((|| {
let ret = if flags.y_advance_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

fn Decoder82<>(_input: &mut Parser<'_>, table_start: u32, flags: opentype_common_value_format_flags) -> Result<opentype_common_value_record, ParseError> {
let x_placement = ((|| {
let ret = if flags.x_placement.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let y_placement = ((|| {
let ret = if flags.y_placement.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let x_advance = ((|| {
let ret = if flags.x_advance.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let y_advance = ((|| {
let ret = if flags.y_advance.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let x_placement_device = ((|| {
let ret = if flags.x_placement_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let y_placement_device = ((|| {
let ret = if flags.y_placement_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let x_advance_device = ((|| {
let ret = if flags.x_advance_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let y_advance_device = ((|| {
let ret = if flags.y_advance_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

fn Decoder83<>(_input: &mut Parser<'_>, table_start: u32, flags: opentype_common_value_format_flags) -> Result<opentype_common_value_record, ParseError> {
let x_placement = ((|| {
let ret = if flags.x_placement.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let y_placement = ((|| {
let ret = if flags.y_placement.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let x_advance = ((|| {
let ret = if flags.x_advance.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let y_advance = ((|| {
let ret = if flags.y_advance.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let x_placement_device = ((|| {
let ret = if flags.x_placement_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let y_placement_device = ((|| {
let ret = if flags.y_placement_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let x_advance_device = ((|| {
let ret = if flags.x_advance_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let y_advance_device = ((|| {
let ret = if flags.y_advance_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

fn Decoder84<>(_input: &mut Parser<'_>, table_start: u32, flags: opentype_common_value_format_flags) -> Result<opentype_common_value_record, ParseError> {
let x_placement = ((|| {
let ret = if flags.x_placement.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let y_placement = ((|| {
let ret = if flags.y_placement.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let x_advance = ((|| {
let ret = if flags.x_advance.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let y_advance = ((|| {
let ret = if flags.y_advance.clone() {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let x_placement_device = ((|| {
let ret = if flags.x_placement_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let y_placement_device = ((|| {
let ret = if flags.y_placement_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let x_advance_device = ((|| {
let ret = if flags.x_advance_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
let y_advance_device = ((|| {
let ret = if flags.y_advance_device.clone() {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
Some(opentype_common_value_record_x_advance_device { offset, link })
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

fn Decoder_opentype_mark_glyph_set<>(_input: &mut Parser<'_>) -> Result<opentype_mark_glyph_set, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 1u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17869550927478639832u64));
};
PResult::Ok(ret)
})())?;
let mark_glyph_set_count = (Decoder23(_input))?;
let coverage = ((|| {
let mut accum = Vec::new();
for _ in 0..mark_glyph_set_count {
accum.push({
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder_opentype_coverage_table(_input))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_mark_glyph_set_coverage { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_mark_glyph_set { table_start, format, mark_glyph_set_count, coverage })
}

fn Decoder86<>(_input: &mut Parser<'_>) -> Result<(), ParseError> {
PResult::Ok(())
}

fn Decoder_opentype_layout_axis_table<>(_input: &mut Parser<'_>) -> Result<opentype_layout_axis_table, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let base_tag_list_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let base_tag_count = (Decoder23(_input))?;
let baseline_tags = ((|| {
let mut accum = Vec::new();
for _ in 0..base_tag_count {
accum.push((Decoder46(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_axis_table_base_tag_list_offset_link { base_tag_count, baseline_tags })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_axis_table_base_tag_list_offset { offset, link })
})())?;
let base_script_list_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let base_script_count = (Decoder23(_input))?;
let base_script_records = ((|| {
let mut accum = Vec::new();
for _ in 0..base_script_count {
accum.push({
let base_script_tag = (Decoder46(_input))?;
let base_script_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_base_script(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_axis_table_base_script_list_offset_link_base_script_records_base_script_offset { offset, link })
})())?;
opentype_layout_axis_table_base_script_list_offset_link_base_script_records { base_script_tag, base_script_offset }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_axis_table_base_script_list_offset_link { table_start, base_script_count, base_script_records })
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_axis_table_base_script_list_offset { offset, link })
})())?;
PResult::Ok(opentype_layout_axis_table { table_start, base_tag_list_offset, base_script_list_offset })
}

fn Decoder_opentype_layout_base_script<>(_input: &mut Parser<'_>) -> Result<opentype_layout_base_script, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let base_values_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_base_values(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_base_script_base_values_offset { offset, link })
})())?;
let default_min_max_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_min_max(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_base_script_default_min_max_offset { offset, link })
})())?;
let base_lang_sys_count = (Decoder23(_input))?;
let base_lang_sys_records = ((|| {
let mut accum = Vec::new();
for _ in 0..base_lang_sys_count {
accum.push({
let base_lang_sys_tag = (Decoder46(_input))?;
let min_max_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_min_max(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_base_script_default_min_max_offset { offset, link })
})())?;
opentype_layout_base_script_base_lang_sys_records { base_lang_sys_tag, min_max_offset }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_base_script { table_start, base_values_offset, default_min_max_offset, base_lang_sys_count, base_lang_sys_records })
}

fn Decoder_opentype_layout_base_values<>(_input: &mut Parser<'_>) -> Result<opentype_layout_base_values, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let default_baseline_index = (Decoder23(_input))?;
let base_coord_count = (Decoder23(_input))?;
let base_coord_offsets = ((|| {
let mut accum = Vec::new();
for _ in 0..base_coord_count {
accum.push({
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_base_coord(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
opentype_layout_min_max_min_coord_offset { offset, link }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_base_values { table_start, default_baseline_index, base_coord_count, base_coord_offsets })
}

fn Decoder_opentype_layout_min_max<>(_input: &mut Parser<'_>) -> Result<opentype_layout_min_max, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let min_coord_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_base_coord(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_min_max_min_coord_offset { offset, link })
})())?;
let max_coord_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_base_coord(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_min_max_min_coord_offset { offset, link })
})())?;
let feat_min_max_count = (Decoder23(_input))?;
let feat_min_max_records = ((|| {
let mut accum = Vec::new();
for _ in 0..feat_min_max_count {
accum.push({
let feature_tag = (Decoder46(_input))?;
let min_coord_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_base_coord(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_min_max_min_coord_offset { offset, link })
})())?;
let max_coord_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_layout_base_coord(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_min_max_min_coord_offset { offset, link })
})())?;
opentype_layout_min_max_feat_min_max_records { feature_tag, min_coord_offset, max_coord_offset }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_layout_min_max { table_start, min_coord_offset, max_coord_offset, feat_min_max_count, feat_min_max_records })
}

fn Decoder_opentype_layout_base_coord<>(_input: &mut Parser<'_>) -> Result<opentype_layout_base_coord, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let format = (Decoder23(_input))?;
let coordinate = (Decoder23(_input))?;
let hint = ((|| {
let ret = match format {
1u16 => {
opentype_layout_base_coord_hint::NoHint
},

2u16 => {
let inner = {
let reference_glyph = (Decoder23(_input))?;
let base_coord_point = (Decoder23(_input))?;
opentype_layout_base_coord_hint_GlyphHint { reference_glyph, base_coord_point }
};
opentype_layout_base_coord_hint::GlyphHint(inner)
},

3u16 => {
let inner = {
let device_offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if table_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + table_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_opentype_common_device_or_variation_index_table(_input))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_common_value_record_x_advance_device { offset, link })
})())?;
opentype_layout_base_coord_hint_DeviceHint { device_offset }
};
opentype_layout_base_coord_hint::DeviceHint(inner)
},

_ => {
return Err(ParseError::FailToken(13431462572241034712u64));
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_layout_base_coord { table_start, format, coordinate, hint })
}

fn Decoder_opentype_glyf_description<>(_input: &mut Parser<'_>, n_contours: u16) -> Result<opentype_glyf_description, ParseError> {
let ret = match n_contours {
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
};
PResult::Ok(ret)
}

fn Decoder_opentype_glyf_simple<>(_input: &mut Parser<'_>, n_contours: u16) -> Result<opentype_glyf_simple, ParseError> {
let end_points_of_contour = ((|| {
let mut accum = Vec::new();
for _ in 0..n_contours {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let instruction_length = (Decoder23(_input))?;
let instructions = ((|| {
let mut accum = Vec::new();
for _ in 0..instruction_length {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
let number_of_coordinates = succ(end_points_of_contour[(pred((end_points_of_contour.len()) as u32)) as usize].clone());
let flags = ((|| {
let arr_flags = {
let tuple_var = {
let mut seq: Vec<opentype_glyf_simple_flags_raw> = Vec::new();
let mut acc = 0u16;
loop {
if {
let totlen = acc.clone();
let _seq = &seq;
totlen >= number_of_coordinates
} {
break
}
let elem = {
let flags = (Decoder_opentype_glyph_description_simple_flags_raw(_input))?;
let repeats = ((|| {
let ret = match flags.repeat_flag.clone() {
true => {
(Decoder24(_input))?
},

false => {
0u8
}
};
PResult::Ok(ret)
})())?;
let field_set = opentype_glyf_simple_flags { on_curve_point: flags.on_curve_point.clone(), x_short_vector: flags.x_short_vector.clone(), y_short_vector: flags.y_short_vector.clone(), x_is_same_or_positive_x_short_vector: flags.x_is_same_or_positive_x_short_vector.clone(), y_is_same_or_positive_y_short_vector: flags.y_is_same_or_positive_y_short_vector.clone(), overlap_simple: flags.overlap_simple.clone() };
opentype_glyf_simple_flags_raw { repeats, field_set }
};
seq.push(elem.clone());
acc = {
let acc = acc;
let flags = elem;
acc + (succ((flags.repeats.clone()) as u16))
};
}
(acc, seq)
};
match tuple_var {
(_len, flags) => {
flags
}
}
};
(|arr_flags: Vec<opentype_glyf_simple_flags_raw>| try_flat_map_vec(arr_flags.iter().cloned(), |packed: opentype_glyf_simple_flags_raw| PResult::Ok(dup32(((packed.repeats.clone()) as u32) + 1u32, packed.field_set.clone()))))(arr_flags)
})())?;
let x_coordinates = ((|| {
let mut accum = Vec::new();
for flag_vals in flags.clone() {
accum.push({
let ret = match flag_vals.x_short_vector.clone() {
true => {
let abs = (Decoder24(_input))?;
match flag_vals.x_is_same_or_positive_x_short_vector.clone() {
true => {
abs as u16
},

false => {
match abs {
0u8 => {
0u16
},

n => {
try_sub!(65535u16, pred(n as u16), 13516986665125759073u64)
}
}
}
}
},

false => {
let ret = match flag_vals.x_is_same_or_positive_x_short_vector.clone() {
true => {
0u16
},

false => {
(Decoder23(_input))?
}
};
ret
}
};
ret
});
}
PResult::Ok(accum)
})())?;
let y_coordinates = ((|| {
let mut accum = Vec::new();
for flag_vals in flags.clone() {
accum.push({
let ret = match flag_vals.y_short_vector.clone() {
true => {
let abs = (Decoder24(_input))?;
match flag_vals.y_is_same_or_positive_y_short_vector.clone() {
true => {
abs as u16
},

false => {
match abs {
0u8 => {
0u16
},

n => {
try_sub!(65535u16, pred(n as u16), 5733880678136728614u64)
}
}
}
}
},

false => {
let ret = match flag_vals.y_is_same_or_positive_y_short_vector.clone() {
true => {
0u16
},

false => {
(Decoder23(_input))?
}
};
ret
}
};
ret
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_glyf_simple { end_points_of_contour, instruction_length, instructions, number_of_coordinates, flags, x_coordinates, y_coordinates })
}

fn Decoder_opentype_glyf_composite<>(_input: &mut Parser<'_>) -> Result<opentype_glyf_composite, ParseError> {
let acc_glyphs = {
let mut seq: Vec<opentype_glyf_composite_raw> = Vec::new();
let mut acc = false;
loop {
if {
let _has_instructions = acc.clone();
let seq = &seq;
match match ((seq.len()) as u32) != 0u32 {
true => {
Some(seq[(pred((seq.len()) as u32)) as usize].clone())
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
} {
break
}
let elem = {
let flags = ((|| {
let packed_bits = {
let x = {
let field0 = _input.read_byte()?;
let field1 = _input.read_byte()?;
(field0, field1)
};
u16be(x)
};
PResult::Ok(opentype_glyf_composite_raw_flags { unscaled_component_offset: packed_bits >> 12u16 & 1u16 > 0u16, scaled_component_offset: packed_bits >> 11u16 & 1u16 > 0u16, overlap_compound: packed_bits >> 10u16 & 1u16 > 0u16, use_my_metrics: packed_bits >> 9u16 & 1u16 > 0u16, we_have_instructions: packed_bits >> 8u16 & 1u16 > 0u16, we_have_a_two_by_two: packed_bits >> 7u16 & 1u16 > 0u16, we_have_an_x_and_y_scale: packed_bits >> 6u16 & 1u16 > 0u16, more_components: packed_bits >> 5u16 & 1u16 > 0u16, we_have_a_scale: packed_bits >> 3u16 & 1u16 > 0u16, round_xy_to_grid: packed_bits >> 2u16 & 1u16 > 0u16, args_are_xy_values: packed_bits >> 1u16 & 1u16 > 0u16, arg_1_and_2_are_words: packed_bits >> 0u16 & 1u16 > 0u16 })
})())?;
let glyph_index = (Decoder23(_input))?;
let argument1 = ((|| {
let ret = match flags.arg_1_and_2_are_words.clone() {
true => {
let ret = match flags.args_are_xy_values.clone() {
true => {
let inner = (Decoder23(_input))?;
opentype_glyf_composite_raw_argument1::Int16(inner)
},

false => {
let inner = (Decoder23(_input))?;
opentype_glyf_composite_raw_argument1::Uint16(inner)
}
};
ret
},

false => {
let ret = match flags.args_are_xy_values.clone() {
true => {
let inner = (Decoder24(_input))?;
opentype_glyf_composite_raw_argument1::Int8(inner)
},

false => {
let inner = (Decoder24(_input))?;
opentype_glyf_composite_raw_argument1::Uint8(inner)
}
};
ret
}
};
PResult::Ok(ret)
})())?;
let argument2 = ((|| {
let ret = match flags.arg_1_and_2_are_words.clone() {
true => {
let ret = match flags.args_are_xy_values.clone() {
true => {
let inner = (Decoder23(_input))?;
opentype_glyf_composite_raw_argument1::Int16(inner)
},

false => {
let inner = (Decoder23(_input))?;
opentype_glyf_composite_raw_argument1::Uint16(inner)
}
};
ret
},

false => {
let ret = match flags.args_are_xy_values.clone() {
true => {
let inner = (Decoder24(_input))?;
opentype_glyf_composite_raw_argument1::Int8(inner)
},

false => {
let inner = (Decoder24(_input))?;
opentype_glyf_composite_raw_argument1::Uint8(inner)
}
};
ret
}
};
PResult::Ok(ret)
})())?;
let scale = ((|| {
let ret = match flags.we_have_a_scale.clone() {
true => {
let val = {
let inner = {
let x = (Decoder23(_input))?;
opentype_var_tuple_record_coordinates::F2Dot14(x)
};
opentype_glyf_composite_raw_scale::Scale(inner)
};
Some(val)
},

false => {
let ret = match flags.we_have_an_x_and_y_scale.clone() {
true => {
let val = {
let inner = {
let x_scale = ((|| {
let x = (Decoder23(_input))?;
PResult::Ok(opentype_var_tuple_record_coordinates::F2Dot14(x))
})())?;
let y_scale = ((|| {
let x = (Decoder23(_input))?;
PResult::Ok(opentype_var_tuple_record_coordinates::F2Dot14(x))
})())?;
opentype_glyf_composite_raw_scale_XY { x_scale, y_scale }
};
opentype_glyf_composite_raw_scale::XY(inner)
};
Some(val)
},

false => {
let ret = match flags.we_have_a_two_by_two.clone() {
true => {
let val = {
let field0 = ((|| {
let field0 = ((|| {
let x = (Decoder23(_input))?;
PResult::Ok(opentype_var_tuple_record_coordinates::F2Dot14(x))
})())?;
let field1 = ((|| {
let x = (Decoder23(_input))?;
PResult::Ok(opentype_var_tuple_record_coordinates::F2Dot14(x))
})())?;
PResult::Ok((field0, field1))
})())?;
let field1 = ((|| {
let field0 = ((|| {
let x = (Decoder23(_input))?;
PResult::Ok(opentype_var_tuple_record_coordinates::F2Dot14(x))
})())?;
let field1 = ((|| {
let x = (Decoder23(_input))?;
PResult::Ok(opentype_var_tuple_record_coordinates::F2Dot14(x))
})())?;
PResult::Ok((field0, field1))
})())?;
opentype_glyf_composite_raw_scale::Matrix(field0, field1)
};
Some(val)
},

false => {
None
}
};
ret
}
};
ret
}
};
PResult::Ok(ret)
})())?;
opentype_glyf_composite_raw { flags, glyph_index, argument1, argument2, scale }
};
seq.push(elem.clone());
acc = {
let acc = acc;
let glyph = elem;
acc || glyph.flags.we_have_instructions.clone()
};
}
(acc, seq)
};
let glyphs = acc_glyphs.1.clone();
let instructions = ((|| {
let ret = match acc_glyphs.0 {
true => {
let instructions_length = (Decoder23(_input))?;
let mut accum = Vec::new();
for _ in 0..instructions_length {
accum.push((Decoder24(_input))?);
}
accum
},

false => {
[].to_vec()
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_glyf_composite { glyphs, instructions })
}

fn Decoder_opentype_glyph_description_simple_flags_raw<>(_input: &mut Parser<'_>) -> Result<opentype_glyph_description_simple_flags_raw, ParseError> {
let packed_bits = _input.read_byte()?;
PResult::Ok(opentype_glyph_description_simple_flags_raw { overlap_simple: packed_bits >> 6u8 & 1u8 > 0u8, y_is_same_or_positive_y_short_vector: packed_bits >> 5u8 & 1u8 > 0u8, x_is_same_or_positive_x_short_vector: packed_bits >> 4u8 & 1u8 > 0u8, repeat_flag: packed_bits >> 3u8 & 1u8 > 0u8, y_short_vector: packed_bits >> 2u8 & 1u8 > 0u8, x_short_vector: packed_bits >> 1u8 & 1u8 > 0u8, on_curve_point: packed_bits >> 0u8 & 1u8 > 0u8 })
}

fn Decoder_opentype_name_table_name_version_1<>(_input: &mut Parser<'_>, storage_start: u32) -> Result<opentype_name_table_name_version_1, ParseError> {
let lang_tag_count = (Decoder23(_input))?;
let lang_tag_records = ((|| {
let mut accum = Vec::new();
for _ in 0..lang_tag_count {
accum.push({
let length = (Decoder23(_input))?;
let offset = ((|| {
let offset = (Decoder23(_input))?;
let link = ((|| {
let ret = match offset > 0u16 {
true => {
let __here = {
let x = _input.get_offset_u64();
x as u32
};
let ret = if storage_start + (offset as u32) >= __here {
let tgt_offset = 0u32 + storage_start + (offset as u32);
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let mut accum = Vec::new();
for _ in 0..length {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_name_table_name_records_offset { offset, link })
})())?;
opentype_name_table_name_version_1_lang_tag_records { length, offset }
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_name_table_name_version_1 { lang_tag_count, lang_tag_records })
}

fn Decoder_opentype_maxp_table_version1<>(_input: &mut Parser<'_>) -> Result<opentype_maxp_table_version1, ParseError> {
let max_points = (Decoder23(_input))?;
let max_contours = (Decoder23(_input))?;
let max_composite_points = (Decoder23(_input))?;
let max_composite_contours = (Decoder23(_input))?;
let max_zones = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
(x >= 1u16) && (x <= 2u16)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8997881400116719018u64));
};
PResult::Ok(ret)
})())?;
let max_twilight_points = (Decoder23(_input))?;
let max_storage = (Decoder23(_input))?;
let max_function_defs = (Decoder23(_input))?;
let max_instruction_defs = (Decoder23(_input))?;
let max_stack_elements = (Decoder23(_input))?;
let max_size_of_instructions = (Decoder23(_input))?;
let max_component_elements = (Decoder23(_input))?;
let max_component_depth = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x <= 16u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13614619987783239962u64));
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_maxp_table_version1 { max_points, max_contours, max_composite_points, max_composite_contours, max_zones, max_twilight_points, max_storage, max_function_defs, max_instruction_defs, max_stack_elements, max_size_of_instructions, max_component_elements, max_component_depth })
}

fn Decoder98<>(_input: &mut Parser<'_>) -> Result<u64, ParseError> {
Decoder99(_input)
}

fn Decoder99<>(_input: &mut Parser<'_>) -> Result<u64, ParseError> {
let x = {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
let field3 = (Decoder24(_input))?;
let field4 = (Decoder24(_input))?;
let field5 = (Decoder24(_input))?;
let field6 = (Decoder24(_input))?;
let field7 = (Decoder24(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
PResult::Ok(u64be(x))
}

fn Decoder_opentype_encoding_record<>(_input: &mut Parser<'_>, start: u32) -> Result<opentype_encoding_record, ParseError> {
let platform = (Decoder23(_input))?;
let encoding = (Decoder23(_input))?;
let subtable_offset = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = (Decoder_opentype_cmap_subtable(_input, platform.clone()))?;
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_encoding_record_subtable_offset { offset, link })
})())?;
PResult::Ok(opentype_encoding_record { platform, encoding, subtable_offset })
}

fn Decoder_opentype_cmap_subtable<>(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable, ParseError> {
let table_start = {
let x = _input.get_offset_u64();
x as u32
};
let format = ((|| {
_input.open_peek_context();
let ret = (Decoder23(_input))?;
_input.close_peek_context()?;
PResult::Ok(ret)
})())?;
let data = ((|| {
let ret = match format {
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
let inner = (Decoder109(_input, _platform.clone()))?;
opentype_cmap_subtable_data::Format13(inner)
},

14u16 => {
let inner = (Decoder_opentype_cmap_subtable_format14(_input, table_start.clone()))?;
opentype_cmap_subtable_data::Format14(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_cmap_subtable { table_start, format, data })
}

fn Decoder_opentype_cmap_subtable_format0<>(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format0, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
let _ = {
let format = (Decoder23(_input))?;
opentype_cmap_subtable_format14_raw_raw { format }
};
Decoder23(_input)
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = (Decoder23(_input))?;
let length = (Decoder23(_input))?;
let language = (Decoder23(_input))?;
let glyph_id_array = ((|| {
let mut accum = Vec::new();
for _ in 0..256u16 {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_cmap_subtable_format0 { format, length, language, glyph_id_array })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format2<>(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format2, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
let _ = {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 2u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6915530142412472120u64));
};
PResult::Ok(ret)
})())?;
opentype_cmap_subtable_format14_raw_raw { format }
};
let inner = (Decoder23(_input))?;
let is_valid = {
let l = inner;
(l >= 518u16) && (l % 2u16 == 0u16)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6949960292533894002u64));
};
PResult::Ok(ret)
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 2u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1347174710810305478u64));
};
PResult::Ok(ret)
})())?;
let length = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let l = inner;
(l >= 518u16) && (l % 2u16 == 0u16)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8958899994948144829u64));
};
PResult::Ok(ret)
})())?;
let language = (Decoder23(_input))?;
let sub_header_keys = ((|| {
let mut accum = Vec::new();
for _ in 0..256u16 {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let sub_headers = ((|| {
let mut accum = Vec::new();
for _ in 0..succ(match (try_fold_left_curried(sub_header_keys.iter().cloned(), None, |tuple_var: (Option<u16>, u16)| PResult::Ok(match tuple_var {
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
return Err(ParseError::ExcludedBranch(5100077783044507986u64));
}
}) {
accum.push({
let first_code = (Decoder23(_input))?;
let entry_count = (Decoder23(_input))?;
let id_delta = (Decoder23(_input))?;
let id_range_offset = (Decoder23(_input))?;
opentype_cmap_subtable_format2_sub_headers { first_code, entry_count, id_delta, id_range_offset }
});
}
PResult::Ok(accum)
})())?;
let glyph_array = ((|| {
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
let next_elem = (Decoder23(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_cmap_subtable_format2 { format, length, language, sub_header_keys, sub_headers, glyph_array })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format4<>(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format4, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
let _ = {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 4u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(15803403730818557393u64));
};
PResult::Ok(ret)
})())?;
opentype_cmap_subtable_format14_raw_raw { format }
};
Decoder23(_input)
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 4u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8390724546948265409u64));
};
PResult::Ok(ret)
})())?;
let length = (Decoder23(_input))?;
let language = (Decoder23(_input))?;
let seg_count = ((|| {
let seg_count_x2 = (Decoder23(_input))?;
PResult::Ok(seg_count_x2 / 2u16)
})())?;
let search_range = (Decoder23(_input))?;
let entry_selector = (Decoder23(_input))?;
let range_shift = (Decoder23(_input))?;
let end_code = ((|| {
let mut accum = Vec::new();
for _ in 0..seg_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let __reserved_pad = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6347242493551283856u64));
};
PResult::Ok(ret)
})())?;
let start_code = ((|| {
let mut accum = Vec::new();
for _ in 0..seg_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let id_delta = ((|| {
let mut accum = Vec::new();
for _ in 0..seg_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let id_range_offset = ((|| {
let mut accum = Vec::new();
for _ in 0..seg_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
let glyph_array = ((|| {
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
let next_elem = (Decoder23(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_cmap_subtable_format4 { format, length, language, seg_count, search_range, entry_selector, range_shift, end_code, __reserved_pad, start_code, id_delta, id_range_offset, glyph_array })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format6<>(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format6, ParseError> {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 6u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4251627061094365437u64));
};
PResult::Ok(ret)
})())?;
let length = (Decoder23(_input))?;
let language = (Decoder23(_input))?;
let first_code = (Decoder23(_input))?;
let entry_count = (Decoder23(_input))?;
let glyph_id_array = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_cmap_subtable_format6 { format, length, language, first_code, entry_count, glyph_id_array })
}

fn Decoder_opentype_cmap_subtable_format8<>(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format8, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
let _ = {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 8u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11915580511665106140u64));
};
PResult::Ok(ret)
})())?;
let __reserved = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17324980155911269375u64));
};
PResult::Ok(ret)
})())?;
opentype_cmap_subtable_format13_raw_raw { format, __reserved }
};
Decoder20(_input)
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 8u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17670535809278048255u64));
};
PResult::Ok(ret)
})())?;
let __reserved = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2444204717155307095u64));
};
PResult::Ok(ret)
})())?;
let length = (Decoder20(_input))?;
let language = (Decoder20(_input))?;
let is32 = ((|| {
let mut accum = Vec::new();
for _ in 0..8192u16 {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
let num_groups = (Decoder20(_input))?;
let groups = ((|| {
let mut accum = Vec::new();
for _ in 0..num_groups {
accum.push((Decoder_opentype_types_sequential_map_record(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_cmap_subtable_format8 { format, __reserved, length, language, is32, num_groups, groups })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format10<>(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format10, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
let _ = {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 10u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1278184758971178969u64));
};
PResult::Ok(ret)
})())?;
let __reserved = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4480225125687487743u64));
};
PResult::Ok(ret)
})())?;
opentype_cmap_subtable_format13_raw_raw { format, __reserved }
};
Decoder20(_input)
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 10u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12879845237981630531u64));
};
PResult::Ok(ret)
})())?;
let __reserved = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3426398976290336157u64));
};
PResult::Ok(ret)
})())?;
let length = (Decoder20(_input))?;
let language = (Decoder20(_input))?;
let start_char_code = (Decoder20(_input))?;
let num_chars = (Decoder20(_input))?;
let glyph_id_array = ((|| {
let mut accum = Vec::new();
for _ in 0..num_chars {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_cmap_subtable_format10 { format, __reserved, length, language, start_char_code, num_chars, glyph_id_array })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format13<>(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format13, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
let _ = {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 12u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11250208753083412758u64));
};
PResult::Ok(ret)
})())?;
let __reserved = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17349123374714965876u64));
};
PResult::Ok(ret)
})())?;
opentype_cmap_subtable_format13_raw_raw { format, __reserved }
};
Decoder20(_input)
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 12u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2153064741293804702u64));
};
PResult::Ok(ret)
})())?;
let __reserved = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1588651938759015246u64));
};
PResult::Ok(ret)
})())?;
let length = (Decoder20(_input))?;
let language = (Decoder20(_input))?;
let num_groups = (Decoder20(_input))?;
let groups = ((|| {
let mut accum = Vec::new();
for _ in 0..num_groups {
accum.push((Decoder_opentype_types_sequential_map_record(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_cmap_subtable_format13 { format, __reserved, length, language, num_groups, groups })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder109<>(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format13, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
let _ = {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 13u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3249387167439447765u64));
};
PResult::Ok(ret)
})())?;
let __reserved = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7744051144774795087u64));
};
PResult::Ok(ret)
})())?;
opentype_cmap_subtable_format13_raw_raw { format, __reserved }
};
Decoder20(_input)
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 13u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8700288293163706751u64));
};
PResult::Ok(ret)
})())?;
let __reserved = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(16771529512960957239u64));
};
PResult::Ok(ret)
})())?;
let length = (Decoder20(_input))?;
let language = (Decoder20(_input))?;
let num_groups = (Decoder20(_input))?;
let groups = ((|| {
let mut accum = Vec::new();
for _ in 0..num_groups {
accum.push((Decoder_opentype_types_sequential_map_record(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_cmap_subtable_format13 { format, __reserved, length, language, num_groups, groups })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_cmap_subtable_format14<>(_input: &mut Parser<'_>, table_start: u32) -> Result<opentype_cmap_subtable_format14, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
let _ = {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 14u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13846498452079501214u64));
};
PResult::Ok(ret)
})())?;
opentype_cmap_subtable_format14_raw_raw { format }
};
Decoder20(_input)
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x == 14u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9798710097031164942u64));
};
PResult::Ok(ret)
})())?;
let length = (Decoder20(_input))?;
let num_var_selector_records = (Decoder20(_input))?;
let var_selector = ((|| {
let mut accum = Vec::new();
for _ in 0..num_var_selector_records {
accum.push((Decoder_opentype_variation_selector(_input, table_start.clone()))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(opentype_cmap_subtable_format14 { format, length, num_var_selector_records, var_selector })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_opentype_variation_selector<>(_input: &mut Parser<'_>, table_start: u32) -> Result<opentype_variation_selector, ParseError> {
let var_selector = ((|| {
let x = {
let field0 = 0u8;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
let field3 = (Decoder24(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(u32be(x))
})())?;
let default_uvs_offset = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = {
let num_unicode_value_ranges = (Decoder20(_input))?;
let ranges = ((|| {
let mut accum = Vec::new();
for _ in 0..num_unicode_value_ranges {
accum.push({
let start_unicode_value = ((|| {
let x = {
let field0 = 0u8;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
let field3 = (Decoder24(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(u32be(x))
})())?;
let additional_count = (Decoder24(_input))?;
opentype_variation_selector_default_uvs_offset_link_ranges { start_unicode_value, additional_count }
});
}
PResult::Ok(accum)
})())?;
opentype_variation_selector_default_uvs_offset_link { num_unicode_value_ranges, ranges }
};
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_variation_selector_default_uvs_offset { offset, link })
})())?;
let non_default_uvs_offset = ((|| {
let offset = (Decoder20(_input))?;
let link = ((|| {
let ret = match offset > 0u32 {
true => {
let tgt_offset = table_start + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = ((|| {
let val = {
let num_uvs_mappings = (Decoder20(_input))?;
let uvs_mappings = ((|| {
let mut accum = Vec::new();
for _ in 0..num_uvs_mappings {
accum.push({
let unicode_value = ((|| {
let x = {
let field0 = 0u8;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
let field3 = (Decoder24(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(u32be(x))
})())?;
let glyph_id = (Decoder23(_input))?;
opentype_variation_selector_non_default_uvs_offset_link_uvs_mappings { unicode_value, glyph_id }
});
}
PResult::Ok(accum)
})())?;
opentype_variation_selector_non_default_uvs_offset_link { num_uvs_mappings, uvs_mappings }
};
PResult::Ok(Some(val))
})())?;
_input.close_peek_context()?;
ret
},

false => {
None
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(opentype_variation_selector_non_default_uvs_offset { offset, link })
})())?;
PResult::Ok(opentype_variation_selector { var_selector, default_uvs_offset, non_default_uvs_offset })
}

fn Decoder_opentype_types_sequential_map_record<>(_input: &mut Parser<'_>) -> Result<opentype_types_sequential_map_record, ParseError> {
let start_char_code = (Decoder20(_input))?;
let end_char_code = (Decoder20(_input))?;
let start_glyph_id = (Decoder20(_input))?;
PResult::Ok(opentype_types_sequential_map_record { start_char_code, end_char_code, start_glyph_id })
}

fn Decoder_elf_header<>(_input: &mut Parser<'_>) -> Result<elf_header, ParseError> {
let ident = ((|| {
let sz = 16u32 as usize;
_input.start_slice(sz)?;
let ret = (Decoder_elf_header_ident(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let r#type = (Decoder135(_input, ident.data.clone() == 2u8))?;
let machine = (Decoder136(_input, ident.data.clone() == 2u8))?;
let version = (Decoder137(_input, ident.data.clone() == 2u8))?;
let entry = (Decoder_elf_types_elf_addr(_input, ident.data.clone() == 2u8, ident.class.clone()))?;
let phoff = (Decoder_elf_types_elf_off(_input, ident.data.clone() == 2u8, ident.class.clone()))?;
let shoff = (Decoder_elf_types_elf_off(_input, ident.data.clone() == 2u8, ident.class.clone()))?;
let flags = (Decoder118(_input, ident.data.clone() == 2u8))?;
let ehsize = (Decoder138(_input, ident.data.clone() == 2u8))?;
let phentsize = (Decoder138(_input, ident.data.clone() == 2u8))?;
let phnum = (Decoder138(_input, ident.data.clone() == 2u8))?;
let shentsize = (Decoder138(_input, ident.data.clone() == 2u8))?;
let shnum = (Decoder138(_input, ident.data.clone() == 2u8))?;
let shstrndx = (Decoder138(_input, ident.data.clone() == 2u8))?;
PResult::Ok(elf_header { ident, r#type, machine, version, entry, phoff, shoff, flags, ehsize, phentsize, phnum, shentsize, shnum, shstrndx })
}

fn Decoder114<>(_input: &mut Parser<'_>, is_be: bool, class: u8, phnum: u16) -> Result<Vec<elf_phdr_table>, ParseError> {
let mut accum = Vec::new();
for _ in 0..phnum {
accum.push((Decoder_elf_phdr_table(_input, is_be.clone(), class.clone()))?);
}
PResult::Ok(accum)
}

fn Decoder115<>(_input: &mut Parser<'_>, is_be: bool, class: u8, shnum: u16) -> Result<Vec<elf_shdr_table>, ParseError> {
let mut accum = Vec::new();
for _ in 0..shnum {
accum.push((Decoder_elf_shdr_table(_input, is_be.clone(), class.clone()))?);
}
PResult::Ok(accum)
}

fn Decoder116<>(_input: &mut Parser<'_>, r#type: u32, size: u64) -> Result<Vec<u8>, ParseError> {
let ret = match r#type {
_ => {
let mut accum = Vec::new();
for _ in 0..size {
accum.push((Decoder24(_input))?);
}
accum
}
};
PResult::Ok(ret)
}

fn Decoder_elf_shdr_table<>(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<elf_shdr_table, ParseError> {
let name = (Decoder118(_input, is_be.clone()))?;
let r#type = (Decoder119(_input, is_be.clone()))?;
let flags = (Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?;
let addr = (Decoder_elf_types_elf_addr(_input, is_be.clone(), class.clone()))?;
let offset = (Decoder_elf_types_elf_off(_input, is_be.clone(), class.clone()))?;
let size = (Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?;
let link = (Decoder118(_input, is_be.clone()))?;
let info = (Decoder123(_input, is_be.clone()))?;
let addralign = (Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?;
let entsize = (Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?;
PResult::Ok(elf_shdr_table { name, r#type, flags, addr, offset, size, link, info, addralign, entsize })
}

fn Decoder118<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
let _ = _input.skip_align(4)?;
let ret = match is_be {
true => {
(Decoder20(_input))?
},

false => {
(Decoder127(_input))?
}
};
PResult::Ok(ret)
}

fn Decoder119<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
let inner = (Decoder118(_input, is_be.clone()))?;
let is_valid = {
let sh_type = inner;
match sh_type {
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
}
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9819345728844658158u64));
};
PResult::Ok(ret)
}

fn Decoder_elf_types_elf_full<>(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<elf_types_elf_full, ParseError> {
let ret = match class {
1u8 => {
let inner = (Decoder118(_input, is_be.clone()))?;
elf_types_elf_full::Full32(inner)
},

2u8 => {
let inner = (Decoder130(_input, is_be.clone()))?;
elf_types_elf_full::Full64(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
}

fn Decoder_elf_types_elf_addr<>(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<elf_types_elf_addr, ParseError> {
let ret = match class {
1u8 => {
let inner = (Decoder128(_input, is_be.clone()))?;
elf_types_elf_addr::Addr32(inner)
},

2u8 => {
let inner = (Decoder129(_input, is_be.clone()))?;
elf_types_elf_addr::Addr64(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
}

fn Decoder_elf_types_elf_off<>(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<elf_types_elf_off, ParseError> {
let ret = match class {
1u8 => {
let inner = (Decoder124(_input, is_be.clone()))?;
elf_types_elf_off::Off32(inner)
},

2u8 => {
let inner = (Decoder125(_input, is_be.clone()))?;
elf_types_elf_off::Off64(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
}

fn Decoder123<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
Decoder118(_input, is_be.clone())
}

fn Decoder124<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
let _ = _input.skip_align(4)?;
let ret = match is_be {
true => {
(Decoder20(_input))?
},

false => {
(Decoder127(_input))?
}
};
PResult::Ok(ret)
}

fn Decoder125<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u64, ParseError> {
let _ = _input.skip_align(8)?;
let ret = match is_be {
true => {
(Decoder99(_input))?
},

false => {
(Decoder126(_input))?
}
};
PResult::Ok(ret)
}

fn Decoder126<>(_input: &mut Parser<'_>) -> Result<u64, ParseError> {
let x = {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
let field3 = (Decoder24(_input))?;
let field4 = (Decoder24(_input))?;
let field5 = (Decoder24(_input))?;
let field6 = (Decoder24(_input))?;
let field7 = (Decoder24(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
PResult::Ok(u64le(x))
}

fn Decoder127<>(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
let x = {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
let field3 = (Decoder24(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(u32le(x))
}

fn Decoder128<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
let _ = _input.skip_align(4)?;
let ret = match is_be {
true => {
(Decoder20(_input))?
},

false => {
(Decoder127(_input))?
}
};
PResult::Ok(ret)
}

fn Decoder129<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u64, ParseError> {
let _ = _input.skip_align(8)?;
let ret = match is_be {
true => {
(Decoder99(_input))?
},

false => {
(Decoder126(_input))?
}
};
PResult::Ok(ret)
}

fn Decoder130<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u64, ParseError> {
let _ = _input.skip_align(8)?;
let ret = match is_be {
true => {
(Decoder99(_input))?
},

false => {
(Decoder126(_input))?
}
};
PResult::Ok(ret)
}

fn Decoder_elf_phdr_table<>(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<elf_phdr_table, ParseError> {
let r#type = (Decoder118(_input, is_be.clone()))?;
let flags64 = (Decoder132(_input, is_be.clone(), class.clone()))?;
let offset = (Decoder_elf_types_elf_off(_input, is_be.clone(), class.clone()))?;
let vaddr = (Decoder_elf_types_elf_addr(_input, is_be.clone(), class.clone()))?;
let paddr = (Decoder_elf_types_elf_addr(_input, is_be.clone(), class.clone()))?;
let filesz = (Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?;
let memsz = (Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?;
let flags32 = (Decoder133(_input, is_be.clone(), class.clone()))?;
let align = (Decoder_elf_types_elf_full(_input, is_be.clone(), class.clone()))?;
PResult::Ok(elf_phdr_table { r#type, flags64, offset, vaddr, paddr, filesz, memsz, flags32, align })
}

fn Decoder132<>(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<Option<u32>, ParseError> {
let ret = if class == 2u8 {
Some((Decoder118(_input, is_be.clone()))?)
} else {
None
};
PResult::Ok(ret)
}

fn Decoder133<>(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<Option<u32>, ParseError> {
let ret = if class == 1u8 {
Some((Decoder118(_input, is_be.clone()))?)
} else {
None
};
PResult::Ok(ret)
}

fn Decoder_elf_header_ident<>(_input: &mut Parser<'_>) -> Result<elf_header_ident, ParseError> {
let magic = ((|| {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 127 {
b
} else {
return Err(ParseError::ExcludedBranch(18279137173405083757u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 69 {
b
} else {
return Err(ParseError::ExcludedBranch(14954891776835932150u64));
})
})())?;
let field2 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 76 {
b
} else {
return Err(ParseError::ExcludedBranch(11046436797737227751u64));
})
})())?;
let field3 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(14984809111992638634u64));
})
})())?;
PResult::Ok((field0, field1, field2, field3))
})())?;
let class = (Decoder140(_input))?;
let data = (Decoder141(_input))?;
let version = (Decoder142(_input))?;
let os_abi = (Decoder143(_input))?;
let abi_version = (Decoder144(_input))?;
let __pad = ((|| {
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
return Err(ParseError::ExcludedBranch(9342187932533045817u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(elf_header_ident { magic, class, data, version, os_abi, abi_version, __pad })
}

fn Decoder135<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u16, ParseError> {
let inner = (Decoder138(_input, is_be.clone()))?;
let is_valid = {
let r#type = inner;
match r#type {
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
}
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13404710972790825894u64));
};
PResult::Ok(ret)
}

fn Decoder136<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u16, ParseError> {
Decoder138(_input, is_be.clone())
}

fn Decoder137<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
let inner = (Decoder118(_input, is_be.clone()))?;
let is_valid = {
let x = inner;
x <= 1u32
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2688427941405105545u64));
};
PResult::Ok(ret)
}

fn Decoder138<>(_input: &mut Parser<'_>, is_be: bool) -> Result<u16, ParseError> {
let _ = _input.skip_align(2)?;
let ret = match is_be {
true => {
(Decoder23(_input))?
},

false => {
(Decoder139(_input))?
}
};
PResult::Ok(ret)
}

fn Decoder139<>(_input: &mut Parser<'_>) -> Result<u16, ParseError> {
let x = {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
(field0, field1)
};
PResult::Ok(u16le(x))
}

fn Decoder140<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
x <= 2u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6279463968646665849u64));
};
PResult::Ok(ret)
}

fn Decoder141<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
x <= 2u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10875553067752207222u64));
};
PResult::Ok(ret)
}

fn Decoder142<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
x <= 1u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11323981950571132721u64));
};
PResult::Ok(ret)
}

fn Decoder143<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
Decoder24(_input)
}

fn Decoder144<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
Decoder24(_input)
}

fn Decoder_tar_header_with_data<>(_input: &mut Parser<'_>) -> Result<tar_header_with_data, ParseError> {
let header = (Decoder_tar_header(_input))?;
let file = ((|| {
let mut accum = Vec::new();
for _ in 0..header.size.clone() {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
let __padding = _input.skip_align(512)?;
PResult::Ok(tar_header_with_data { header, file, __padding })
}

fn Decoder_tar_header<>(_input: &mut Parser<'_>) -> Result<tar_header, ParseError> {
let sz = 512u32 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let name = ((|| {
let sz = 100u16 as usize;
_input.start_slice(sz)?;
let ret = (Decoder_tar_ascii_string_opt0(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let mode = ((|| {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(1179945139148562335u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder148(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let __nul_or_wsp = (Decoder149(_input))?;
let __padding = ((|| {
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
return Err(ParseError::ExcludedBranch(3270685119814653163u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_header_uid { string, __nul_or_wsp, __padding })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let uid = ((|| {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(6821845925776570829u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder148(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let __nul_or_wsp = (Decoder149(_input))?;
let __padding = ((|| {
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
return Err(ParseError::ExcludedBranch(15898809900392744567u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_header_uid { string, __nul_or_wsp, __padding })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let gid = ((|| {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(5653230390980289841u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder148(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let __nul_or_wsp = (Decoder149(_input))?;
let __padding = ((|| {
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
return Err(ParseError::ExcludedBranch(9179996462972575343u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_header_uid { string, __nul_or_wsp, __padding })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let size = ((|| {
let rec = {
let oA = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 6495907546257147840u64))
})())?;
let o9 = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 10310785543736156275u64))
})())?;
let o8 = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 16529910322175208638u64))
})())?;
let o7 = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 8766708729375264031u64))
})())?;
let o6 = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 163858356033350300u64))
})())?;
let o5 = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 4770836931378141069u64))
})())?;
let o4 = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 3595277668730903043u64))
})())?;
let o3 = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 12530712830475607577u64))
})())?;
let o2 = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 4795509455376621436u64))
})())?;
let o1 = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 5174369311102857850u64))
})())?;
let o0 = ((|| {
let bit = (Decoder148(_input))?;
PResult::Ok(try_sub!(bit as u8, 48u8, 4063460887563813446u64))
})())?;
let __nil = (Decoder149(_input))?;
let value = (((0u8 as u32) << 3u32 | (oA as u32)) << 6u32 | (o9 as u32) << 3u32 | (o8 as u32)) << 24u32 | (((o7 as u32) << 3u32 | (o6 as u32)) << 6u32 | (o5 as u32) << 3u32 | (o4 as u32)) << 12u32 | ((o3 as u32) << 3u32 | (o2 as u32)) << 6u32 | (o1 as u32) << 3u32 | (o0 as u32);
tar_header_size_raw { oA, o9, o8, o7, o6, o5, o4, o3, o2, o1, o0, __nil, value }
};
PResult::Ok(rec.value.clone())
})())?;
let mtime = ((|| {
let sz = 12u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(10243418979491025991u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder148(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let __nul_or_wsp = (Decoder149(_input))?;
let __padding = ((|| {
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
return Err(ParseError::ExcludedBranch(6726475111737435495u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_header_uid { string, __nul_or_wsp, __padding })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let chksum = ((|| {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(3702191162545267776u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder148(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let __nul_or_wsp = (Decoder149(_input))?;
let __padding = ((|| {
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
return Err(ParseError::ExcludedBranch(14926982082392674388u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_header_uid { string, __nul_or_wsp, __padding })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let typeflag = (Decoder150(_input))?;
let linkname = ((|| {
let sz = 100u16 as usize;
_input.start_slice(sz)?;
let ret = (Decoder151(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let magic = ((|| {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 117 {
b
} else {
return Err(ParseError::ExcludedBranch(13281230340934385869u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 115 {
b
} else {
return Err(ParseError::ExcludedBranch(2229770659268432585u64));
})
})())?;
let field2 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 116 {
b
} else {
return Err(ParseError::ExcludedBranch(8862619478422395719u64));
})
})())?;
let field3 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 97 {
b
} else {
return Err(ParseError::ExcludedBranch(7200474585457206375u64));
})
})())?;
let field4 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 114 {
b
} else {
return Err(ParseError::ExcludedBranch(13266006752343456203u64));
})
})())?;
let field5 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(13264741506377240721u64));
})
})())?;
PResult::Ok((field0, field1, field2, field3, field4, field5))
})())?;
let version = ((|| {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 48 {
b
} else {
return Err(ParseError::ExcludedBranch(16152968816646114000u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 48 {
b
} else {
return Err(ParseError::ExcludedBranch(12815986247090051214u64));
})
})())?;
PResult::Ok((field0, field1))
})())?;
let uname = ((|| {
let sz = 32u16 as usize;
_input.start_slice(sz)?;
let ret = (Decoder_tar_ascii_string(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let gname = ((|| {
let sz = 32u16 as usize;
_input.start_slice(sz)?;
let ret = (Decoder_tar_ascii_string(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let devmajor = ((|| {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(2508979988921372290u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder148(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let __nul_or_wsp = (Decoder149(_input))?;
let __padding = ((|| {
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
return Err(ParseError::ExcludedBranch(10057441536650509049u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_header_uid { string, __nul_or_wsp, __padding })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let devminor = ((|| {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(11815677057767015929u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder148(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let __nul_or_wsp = (Decoder149(_input))?;
let __padding = ((|| {
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
return Err(ParseError::ExcludedBranch(829032137919921844u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_header_uid { string, __nul_or_wsp, __padding })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let prefix = ((|| {
let sz = 155u16 as usize;
_input.start_slice(sz)?;
let ret = (Decoder151(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let pad = ((|| {
let mut accum = Vec::new();
for _ in 0..12u32 {
accum.push({
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(5170050512307443704u64));
}
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_header { name, mode, uid, gid, size, mtime, chksum, typeflag, linkname, magic, version, uname, gname, devmajor, devminor, prefix, pad })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

fn Decoder_tar_ascii_string_opt0<>(_input: &mut Parser<'_>) -> Result<tar_ascii_string_opt0, ParseError> {
let string = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if (byte != 0) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(11309019127259385425u64));
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
return Err(ParseError::ExcludedBranch(18440348483137307888u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
})())?;
let __padding = ((|| {
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
return Err(ParseError::ExcludedBranch(5159371628350638829u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_ascii_string_opt0 { string, __padding })
}

fn Decoder148<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if (ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(7762269623995317946u64));
})
}

fn Decoder149<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if (ByteSet::from_bits([4294967297, 0, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(16134612799304961491u64));
})
}

fn Decoder150<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
_input.read_byte()
}

fn Decoder151<>(_input: &mut Parser<'_>) -> Result<tar_ascii_string_opt0, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(16190528142421852545u64));
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
return Err(ParseError::ExcludedBranch(5308477118997970057u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let __padding = ((|| {
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
return Err(ParseError::ExcludedBranch(173922233423713068u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_ascii_string_opt0 { string, __padding })
}

fn Decoder_tar_ascii_string<>(_input: &mut Parser<'_>) -> Result<tar_ascii_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(515819609734101411u64));
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
return Err(ParseError::ExcludedBranch(12540117573097456360u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let padding = ((|| {
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
return Err(ParseError::ExcludedBranch(9277543013594125416u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(tar_ascii_string { string, padding })
}

fn Decoder_riff_subchunks<>(_input: &mut Parser<'_>) -> Result<riff_subchunks, ParseError> {
let tag = (Decoder154(_input))?;
let chunks = ((|| {
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
PResult::Ok(accum)
})())?;
PResult::Ok(riff_subchunks { tag, chunks })
}

fn Decoder154<>(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = (Decoder150(_input))?;
let field1 = (Decoder150(_input))?;
let field2 = (Decoder150(_input))?;
let field3 = (Decoder150(_input))?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder_riff_chunk<>(_input: &mut Parser<'_>) -> Result<riff_chunk, ParseError> {
let tag = (Decoder154(_input))?;
let length = (Decoder127(_input))?;
let data = ((|| {
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let pad = ((|| {
let ret = if length % 2u32 == 1u32 {
let b = _input.read_byte()?;
Some(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(5955168674639093440u64));
})
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(riff_chunk { tag, length, data, pad })
}

fn Decoder156<>(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8, u8, u8, u8, u8), ParseError> {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 137 {
b
} else {
return Err(ParseError::ExcludedBranch(4471438437047399494u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 80 {
b
} else {
return Err(ParseError::ExcludedBranch(13319523888327217639u64));
})
})())?;
let field2 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 78 {
b
} else {
return Err(ParseError::ExcludedBranch(824589811577025210u64));
})
})())?;
let field3 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 71 {
b
} else {
return Err(ParseError::ExcludedBranch(4649034608147552416u64));
})
})())?;
let field4 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 13 {
b
} else {
return Err(ParseError::ExcludedBranch(16096650375442290768u64));
})
})())?;
let field5 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 10 {
b
} else {
return Err(ParseError::ExcludedBranch(14339975513692068616u64));
})
})())?;
let field6 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 26 {
b
} else {
return Err(ParseError::ExcludedBranch(16299205781335471965u64));
})
})())?;
let field7 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 10 {
b
} else {
return Err(ParseError::ExcludedBranch(1479153625485860551u64));
})
})())?;
PResult::Ok((field0, field1, field2, field3, field4, field5, field6, field7))
}

fn Decoder_png_ihdr<>(_input: &mut Parser<'_>) -> Result<png_ihdr, ParseError> {
let length = ((|| {
let inner = (Decoder20(_input))?;
let is_valid = {
let length = inner;
length <= 2147483647u32
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12668500753644823654u64));
};
PResult::Ok(ret)
})())?;
let tag = (Decoder203(_input))?;
let data = ((|| {
let sz = length as usize;
_input.start_slice(sz)?;
let ret = (Decoder_png_ihdr_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let crc = (Decoder20(_input))?;
PResult::Ok(png_ihdr { length, tag, data, crc })
}

fn Decoder_png_chunk<>(_input: &mut Parser<'_>, ihdr: png_ihdr) -> Result<png_chunk, ParseError> {
let length = ((|| {
let inner = (Decoder20(_input))?;
let is_valid = {
let length = inner;
length <= 2147483647u32
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8094248233631264621u64));
};
PResult::Ok(ret)
})())?;
let tag = ((|| {
let _ = {
_input.open_peek_not_context();
let res = (|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(1844274570107701975u64));
})
})();
if res.is_err() {
_input.close_peek_not_context()?;
} else {
return Err(ParseError::NegatedSuccess);
}
()
};
let mut accum = Vec::new();
for _ in 0..4u32 {
accum.push((Decoder174(_input))?);
}
PResult::Ok(accum)
})())?;
let data = ((|| {
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match tag.as_slice() {
[80u8, 76u8, 84u8, 69u8] => {
let inner = (Decoder175(_input))?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
png_chunk_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let crc = (Decoder20(_input))?;
PResult::Ok(png_chunk { length, tag, data, crc })
}

fn Decoder_png_idat<>(_input: &mut Parser<'_>) -> Result<png_idat, ParseError> {
let length = ((|| {
let inner = (Decoder20(_input))?;
let is_valid = {
let length = inner;
length <= 2147483647u32
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4839194687019048322u64));
};
PResult::Ok(ret)
})())?;
let tag = (Decoder172(_input))?;
let data = ((|| {
let sz = length as usize;
_input.start_slice(sz)?;
let ret = (Decoder173(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let crc = (Decoder20(_input))?;
PResult::Ok(png_idat { length, tag, data, crc })
}

fn Decoder_zlib_main<>(_input: &mut Parser<'_>) -> Result<zlib_main, ParseError> {
let compression_method_flags = ((|| {
let inner = {
let packed_bits = _input.read_byte()?;
zlib_main_compression_method_flags { compression_info: packed_bits >> 4u8 & 15u8, compression_method: packed_bits >> 0u8 & 15u8 }
};
let is_valid = {
let method_info = inner;
method_info.compression_method.clone() == 8u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7230273548678969972u64));
};
PResult::Ok(ret)
})())?;
let flags = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(zlib_main_flags { flevel: packed_bits >> 6u8 & 3u8, fdict: packed_bits >> 5u8 & 1u8 > 0u8, fcheck: packed_bits >> 0u8 & 31u8 })
})())?;
let dict_id = ((|| {
let ret = if flags.fdict.clone() {
Some((Decoder20(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let data = ((|| {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
PResult::Ok(ret)
})())?;
let adler32 = (Decoder20(_input))?;
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

fn Decoder_png_iend<>(_input: &mut Parser<'_>) -> Result<png_iend, ParseError> {
let length = ((|| {
let inner = (Decoder20(_input))?;
let is_valid = {
let length = inner;
length <= 2147483647u32
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14903563845775542749u64));
};
PResult::Ok(ret)
})())?;
let tag = (Decoder162(_input))?;
let data = ((|| {
let sz = length as usize;
_input.start_slice(sz)?;
let ret = (Decoder163(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
let crc = (Decoder20(_input))?;
PResult::Ok(png_iend { length, tag, data, crc })
}

fn Decoder162<>(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(1969670610881234889u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 69 {
b
} else {
return Err(ParseError::ExcludedBranch(9038350950373664822u64));
})
})())?;
let field2 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 78 {
b
} else {
return Err(ParseError::ExcludedBranch(7281717462557989541u64));
})
})())?;
let field3 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(15510952803379905659u64));
})
})())?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder163<>(_input: &mut Parser<'_>) -> Result<(), ParseError> {
PResult::Ok(())
}

fn Decoder_deflate_main<>(_input: &mut Parser<'_>) -> Result<deflate_main, ParseError> {
let blocks = ((|| {
let mut accum = Vec::new();
loop {
let elem = (Decoder_deflate_block(_input))?;
if {
let x = &elem;
x.r#final.clone() == 1u8
} {
accum.push(elem);
break
} else {
accum.push(elem);
}
}
PResult::Ok(accum)
})())?;
let codes = (try_flat_map_vec(blocks.iter().cloned(), |x: deflate_block| PResult::Ok(match x.data.clone() {
deflate_main_codes__dupX1::uncompressed(ref y) => {
y.codes_values.clone()
},

deflate_main_codes__dupX1::fixed_huffman(ref y) => {
y.codes_values.clone()
},

deflate_main_codes__dupX1::dynamic_huffman(ref y) => {
y.codes_values.clone()
}
})))?;
let inflate = (try_flat_map_append_vec(codes.iter().cloned(), |tuple_var: (&Vec<u8>, deflate_main_codes)| PResult::Ok(match tuple_var {
(buffer, symbol) => {
match symbol {
deflate_main_codes::literal(b) => {
[b].to_vec()
},

deflate_main_codes::reference(r) => {
{
let ix = (try_sub!((buffer.len()) as u32, (r.distance.clone()) as u32, 16200207902741715318u64)) as usize;
(slice_ext(&buffer, ix..ix + (((r.length.clone()) as u32) as usize))).to_vec()
}
}
}
}
})))?;
PResult::Ok(deflate_main { blocks, codes, inflate })
}

fn Decoder_deflate_block<>(_input: &mut Parser<'_>) -> Result<deflate_block, ParseError> {
let r#final = (Decoder166(_input))?;
let r#type = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8)
})())?;
let data = ((|| {
let ret = match r#type {
0u8 => {
let inner = (Decoder_deflate_uncompressed(_input))?;
deflate_main_codes__dupX1::uncompressed(inner)
},

1u8 => {
let inner = (Decoder_deflate_fixed_huffman(_input))?;
deflate_main_codes__dupX1::fixed_huffman(inner)
},

2u8 => {
let inner = (Decoder_deflate_dynamic_huffman(_input))?;
deflate_main_codes__dupX1::dynamic_huffman(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(deflate_block { r#final, r#type, data })
}

fn Decoder166<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
_input.read_byte()
}

fn Decoder_deflate_uncompressed<>(_input: &mut Parser<'_>) -> Result<deflate_uncompressed, ParseError> {
let align = _input.skip_align(8)?;
let len = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
let field7 = (Decoder166(_input))?;
let field8 = (Decoder166(_input))?;
let field9 = (Decoder166(_input))?;
let field10 = (Decoder166(_input))?;
let field11 = (Decoder166(_input))?;
let field12 = (Decoder166(_input))?;
let field13 = (Decoder166(_input))?;
let field14 = (Decoder166(_input))?;
let field15 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15)
};
PResult::Ok((bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16 | (bits.10 as u16) << 10u16 | (bits.11 as u16) << 11u16 | (bits.12 as u16) << 12u16 | (bits.13 as u16) << 13u16 | (bits.14 as u16) << 14u16 | (bits.15 as u16) << 15u16)
})())?;
let nlen = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
let field7 = (Decoder166(_input))?;
let field8 = (Decoder166(_input))?;
let field9 = (Decoder166(_input))?;
let field10 = (Decoder166(_input))?;
let field11 = (Decoder166(_input))?;
let field12 = (Decoder166(_input))?;
let field13 = (Decoder166(_input))?;
let field14 = (Decoder166(_input))?;
let field15 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15)
};
PResult::Ok((bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16 | (bits.10 as u16) << 10u16 | (bits.11 as u16) << 11u16 | (bits.12 as u16) << 12u16 | (bits.13 as u16) << 13u16 | (bits.14 as u16) << 14u16 | (bits.15 as u16) << 15u16)
})())?;
let bytes = ((|| {
let mut accum = Vec::new();
for _ in 0..len {
accum.push({
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
let field7 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8 | bits.5 << 5u8 | bits.6 << 6u8 | bits.7 << 7u8
});
}
PResult::Ok(accum)
})())?;
let codes_values = (try_flat_map_vec(bytes.iter().cloned(), |x: u8| PResult::Ok([deflate_main_codes::literal(x)].to_vec())))?;
PResult::Ok(deflate_uncompressed { align, len, nlen, bytes, codes_values })
}

fn Decoder_deflate_fixed_huffman<>(_input: &mut Parser<'_>) -> Result<deflate_fixed_huffman, ParseError> {
let codes = ((|| {
let format = parse_huffman([8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8].to_vec(), None);
let mut accum = Vec::new();
loop {
let elem = {
let code = (format(_input))?;
let extra = ((|| {
let ret = match code {
257u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 3u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

258u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 4u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

259u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 5u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

260u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 6u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

261u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 7u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

262u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 8u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

263u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 9u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

264u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 10u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

265u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
(field0,)
};
PResult::Ok(bits.0)
})())?;
let length = 11u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

266u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
(field0,)
};
PResult::Ok(bits.0)
})())?;
let length = 13u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

267u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
(field0,)
};
PResult::Ok(bits.0)
})())?;
let length = 15u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

268u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
(field0,)
};
PResult::Ok(bits.0)
})())?;
let length = 17u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

269u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8)
})())?;
let length = 19u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

270u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8)
})())?;
let length = 23u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

271u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8)
})())?;
let length = 27u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

272u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8)
})())?;
let length = 31u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

273u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8)
})())?;
let length = 35u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

274u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8)
})())?;
let length = 43u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

275u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8)
})())?;
let length = 51u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

276u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8)
})())?;
let length = 59u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

277u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8)
})())?;
let length = 67u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

278u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8)
})())?;
let length = 83u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

279u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8)
})())?;
let length = 99u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

280u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8)
})())?;
let length = 115u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

281u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8)
})())?;
let length = 131u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

282u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8)
})())?;
let length = 163u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

283u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8)
})())?;
let length = 195u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

284u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8)
})())?;
let length = 227u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

285u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 258u16 + (length_extra_bits as u16);
let distance_code = ((|| {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
accum.push((Decoder166(_input))?);
}
accum
};
PResult::Ok(bits[0u32 as usize].clone() << 4u8 | bits[1u32 as usize].clone() << 3u8 | bits[2u32 as usize].clone() << 2u8 | bits[3u32 as usize].clone() << 1u8 | bits[4u32 as usize].clone())
})())?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code as u16))?;
deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

286u16..=287u16 => {
None
},

_ => {
None
}
};
PResult::Ok(ret)
})())?;
deflate_fixed_huffman_codes { code, extra }
};
if {
let x = &elem;
((x.code.clone()) as u16) == 256u16
} {
accum.push(elem);
break
} else {
accum.push(elem);
}
}
PResult::Ok(accum)
})())?;
let codes_values = (try_flat_map_vec(codes.iter().cloned(), |x: deflate_fixed_huffman_codes| PResult::Ok(match x.code.clone() {
256u16 => {
[].to_vec()
},

257u16..=285u16 => {
match x.extra.clone() {
Some(ref rec) => {
[deflate_main_codes::reference(deflate_main_codes_reference { length: rec.length.clone(), distance: rec.distance_record.distance.clone() })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(5576343694315527798u64));
}
}
},

286u16..=287u16 => {
[].to_vec()
},

_ => {
[deflate_main_codes::literal((x.code.clone()) as u8)].to_vec()
}
})))?;
PResult::Ok(deflate_fixed_huffman { codes, codes_values })
}

fn Decoder_deflate_dynamic_huffman<>(_input: &mut Parser<'_>) -> Result<deflate_dynamic_huffman, ParseError> {
let hlit = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8)
})())?;
let hdist = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8)
})())?;
let hclen = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8)
})())?;
let code_length_alphabet_code_lengths = ((|| {
let mut accum = Vec::new();
for _ in 0..hclen + 4u8 {
accum.push({
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8
});
}
PResult::Ok(accum)
})())?;
let literal_length_distance_alphabet_code_lengths = ((|| {
let code_length_alphabet_format = parse_huffman(code_length_alphabet_code_lengths.clone(), Some([16u8, 17u8, 18u8, 0u8, 8u8, 7u8, 9u8, 6u8, 10u8, 5u8, 11u8, 4u8, 12u8, 3u8, 13u8, 2u8, 14u8, 1u8, 15u8].to_vec()));
let mut accum = Vec::new();
loop {
let elem = {
let code = (code_length_alphabet_format(_input))?;
let extra = ((|| {
let ret = match code as u8 {
16u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
bits.0 << 0u8 | bits.1 << 1u8
},

17u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8
},

18u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6)
};
bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8 | bits.5 << 5u8 | bits.6 << 6u8
},

_ => {
0u8
}
};
PResult::Ok(ret)
})())?;
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
return Err(ParseError::ExcludedBranch(6209434968043366837u64));
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
PResult::Ok(accum)
})())?;
let literal_length_distance_alphabet_code_lengths_value = (try_fold_map_curried(literal_length_distance_alphabet_code_lengths.iter().cloned(), None, |tuple_var: (Option<u8>, deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths)| PResult::Ok(match tuple_var {
(last_symbol, cl_code_extra) => {
match (cl_code_extra.code.clone()) as u8 {
16u8 => {
(last_symbol.clone(), dup32((cl_code_extra.extra.clone() + 3u8) as u32, match last_symbol {
Some(x) => {
x.clone()
},

_ => {
return Err(ParseError::ExcludedBranch(4672672775256824980u64));
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
})))?;
let literal_length_alphabet_code_lengths_value = {
let ix = 0u32 as usize;
Vec::from(&literal_length_distance_alphabet_code_lengths_value[ix..(ix + (((hlit as u32) + 257u32) as usize))])
};
let distance_alphabet_code_lengths_value = {
let ix = ((hlit as u32) + 257u32) as usize;
Vec::from(&literal_length_distance_alphabet_code_lengths_value[ix..(ix + (((hdist as u32) + 1u32) as usize))])
};
let codes = ((|| {
let distance_alphabet_format = parse_huffman(distance_alphabet_code_lengths_value.clone(), None);
let literal_length_alphabet_format = parse_huffman(literal_length_alphabet_code_lengths_value.clone(), None);
let mut accum = Vec::new();
loop {
let elem = {
let code = (literal_length_alphabet_format(_input))?;
let extra = ((|| {
let ret = match code {
257u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 3u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

258u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 4u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

259u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 5u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

260u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 6u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

261u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 7u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

262u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 8u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

263u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 9u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

264u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 10u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

265u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
(field0,)
};
PResult::Ok(bits.0)
})())?;
let length = 11u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

266u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
(field0,)
};
PResult::Ok(bits.0)
})())?;
let length = 13u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

267u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
(field0,)
};
PResult::Ok(bits.0)
})())?;
let length = 15u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

268u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
(field0,)
};
PResult::Ok(bits.0)
})())?;
let length = 17u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

269u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8)
})())?;
let length = 19u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

270u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8)
})())?;
let length = 23u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

271u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8)
})())?;
let length = 27u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

272u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8)
})())?;
let length = 31u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

273u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8)
})())?;
let length = 35u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

274u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8)
})())?;
let length = 43u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

275u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8)
})())?;
let length = 51u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

276u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8)
})())?;
let length = 59u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

277u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8)
})())?;
let length = 67u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

278u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8)
})())?;
let length = 83u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

279u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8)
})())?;
let length = 99u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

280u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
(field0, field1, field2, field3)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8)
})())?;
let length = 115u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

281u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8)
})())?;
let length = 131u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

282u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8)
})())?;
let length = 163u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

283u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8)
})())?;
let length = 195u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

284u16 => {
let val = {
let length_extra_bits = ((|| {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
PResult::Ok(bits.0 << 0u8 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8)
})())?;
let length = 227u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

285u16 => {
let val = {
let length_extra_bits = 0u8;
let length = 258u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record(_input, distance_code.clone()))?;
deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record }
};
Some(val)
},

286u16..=287u16 => {
None
},

_ => {
None
}
};
PResult::Ok(ret)
})())?;
deflate_dynamic_huffman_codes { code, extra }
};
if {
let x = &elem;
((x.code.clone()) as u16) == 256u16
} {
accum.push(elem);
break
} else {
accum.push(elem);
}
}
PResult::Ok(accum)
})())?;
let codes_values = (try_flat_map_vec(codes.iter().cloned(), |x: deflate_dynamic_huffman_codes| PResult::Ok(match x.code.clone() {
256u16 => {
[].to_vec()
},

257u16..=285u16 => {
match x.extra.clone() {
Some(ref rec) => {
[deflate_main_codes::reference(deflate_main_codes_reference { length: rec.length.clone(), distance: rec.distance_record.distance.clone() })].to_vec()
},

_ => {
return Err(ParseError::ExcludedBranch(98102193810481173u64));
}
}
},

286u16..=287u16 => {
[].to_vec()
},

_ => {
[deflate_main_codes::literal((x.code.clone()) as u8)].to_vec()
}
})))?;
PResult::Ok(deflate_dynamic_huffman { hlit, hdist, hclen, code_length_alphabet_code_lengths, literal_length_distance_alphabet_code_lengths, literal_length_distance_alphabet_code_lengths_value, literal_length_alphabet_code_lengths_value, distance_alphabet_code_lengths_value, codes, codes_values })
}

fn Decoder_deflate_distance_record<>(_input: &mut Parser<'_>, distance_code: u16) -> Result<deflate_distance_record, ParseError> {
let ret = match distance_code as u8 {
0u8 => {
(Decoder171(_input, 0u8, 1u16))?
},

1u8 => {
(Decoder171(_input, 0u8, 2u16))?
},

2u8 => {
(Decoder171(_input, 0u8, 3u16))?
},

3u8 => {
(Decoder171(_input, 0u8, 4u16))?
},

4u8 => {
(Decoder171(_input, 1u8, 5u16))?
},

5u8 => {
(Decoder171(_input, 1u8, 7u16))?
},

6u8 => {
(Decoder171(_input, 2u8, 9u16))?
},

7u8 => {
(Decoder171(_input, 2u8, 13u16))?
},

8u8 => {
(Decoder171(_input, 3u8, 17u16))?
},

9u8 => {
(Decoder171(_input, 3u8, 25u16))?
},

10u8 => {
(Decoder171(_input, 4u8, 33u16))?
},

11u8 => {
(Decoder171(_input, 4u8, 49u16))?
},

12u8 => {
(Decoder171(_input, 5u8, 65u16))?
},

13u8 => {
(Decoder171(_input, 5u8, 97u16))?
},

14u8 => {
(Decoder171(_input, 6u8, 129u16))?
},

15u8 => {
(Decoder171(_input, 6u8, 193u16))?
},

16u8 => {
(Decoder171(_input, 7u8, 257u16))?
},

17u8 => {
(Decoder171(_input, 7u8, 385u16))?
},

18u8 => {
(Decoder171(_input, 8u8, 513u16))?
},

19u8 => {
(Decoder171(_input, 8u8, 769u16))?
},

20u8 => {
(Decoder171(_input, 9u8, 1025u16))?
},

21u8 => {
(Decoder171(_input, 9u8, 1537u16))?
},

22u8 => {
(Decoder171(_input, 10u8, 2049u16))?
},

23u8 => {
(Decoder171(_input, 10u8, 3073u16))?
},

24u8 => {
(Decoder171(_input, 11u8, 4097u16))?
},

25u8 => {
(Decoder171(_input, 11u8, 6145u16))?
},

26u8 => {
(Decoder171(_input, 12u8, 8193u16))?
},

27u8 => {
(Decoder171(_input, 12u8, 12289u16))?
},

28u8 => {
(Decoder171(_input, 13u8, 16385u16))?
},

29u8 => {
(Decoder171(_input, 13u8, 24577u16))?
},

30u8..=31u8 => {
return Err(ParseError::FailToken(12217686503432178884u64));
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
}

fn Decoder171<>(_input: &mut Parser<'_>, extra_bits: u8, start: u16) -> Result<deflate_distance_record, ParseError> {
let distance_extra_bits = ((|| {
let ret = match extra_bits {
0u8 => {
0u16
},

1u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
(field0,)
};
bits.0 as u16
},

2u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
(field0, field1)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16
},

3u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
(field0, field1, field2)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16
},

4u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
(field0, field1, field2, field3)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16
},

5u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16
},

6u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16
},

7u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16
},

8u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
let field7 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16
},

9u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
let field7 = (Decoder166(_input))?;
let field8 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16
},

10u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
let field7 = (Decoder166(_input))?;
let field8 = (Decoder166(_input))?;
let field9 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16
},

11u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
let field7 = (Decoder166(_input))?;
let field8 = (Decoder166(_input))?;
let field9 = (Decoder166(_input))?;
let field10 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16 | (bits.10 as u16) << 10u16
},

12u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
let field7 = (Decoder166(_input))?;
let field8 = (Decoder166(_input))?;
let field9 = (Decoder166(_input))?;
let field10 = (Decoder166(_input))?;
let field11 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16 | (bits.10 as u16) << 10u16 | (bits.11 as u16) << 11u16
},

13u8 => {
let bits = {
let field0 = (Decoder166(_input))?;
let field1 = (Decoder166(_input))?;
let field2 = (Decoder166(_input))?;
let field3 = (Decoder166(_input))?;
let field4 = (Decoder166(_input))?;
let field5 = (Decoder166(_input))?;
let field6 = (Decoder166(_input))?;
let field7 = (Decoder166(_input))?;
let field8 = (Decoder166(_input))?;
let field9 = (Decoder166(_input))?;
let field10 = (Decoder166(_input))?;
let field11 = (Decoder166(_input))?;
let field12 = (Decoder166(_input))?;
(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12)
};
(bits.0 as u16) << 0u16 | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16 | (bits.10 as u16) << 10u16 | (bits.11 as u16) << 11u16 | (bits.12 as u16) << 12u16
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let distance = start + distance_extra_bits;
PResult::Ok(deflate_distance_record { distance_extra_bits, distance })
}

fn Decoder172<>(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(8399572043096922156u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(7832192330748800109u64));
})
})())?;
let field2 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 65 {
b
} else {
return Err(ParseError::ExcludedBranch(9815657591077818003u64));
})
})())?;
let field3 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 84 {
b
} else {
return Err(ParseError::ExcludedBranch(2197379665604321609u64));
})
})())?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder173<>(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
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
PResult::Ok(accum)
}

fn Decoder174<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if (ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(16624020278885696461u64));
})
}

fn Decoder175<>(_input: &mut Parser<'_>) -> Result<Vec<png_plte>, ParseError> {
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
let r = (Decoder24(_input))?;
let g = (Decoder24(_input))?;
let b = (Decoder24(_input))?;
png_plte { r, g, b }
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder_png_trns<>(_input: &mut Parser<'_>, ihdr: png_ihdr) -> Result<png_trns, ParseError> {
let ret = match ihdr.data.color_type.clone() {
0u8 => {
let inner = {
let greyscale = (Decoder23(_input))?;
png_bkgd_color_type_0 { greyscale }
};
png_trns::color_type_0(inner)
},

2u8 => {
let inner = {
let red = (Decoder23(_input))?;
let green = (Decoder23(_input))?;
let blue = (Decoder23(_input))?;
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
let palette_index = (Decoder24(_input))?;
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
};
PResult::Ok(ret)
}

fn Decoder_png_chrm<>(_input: &mut Parser<'_>) -> Result<png_chrm, ParseError> {
let whitepoint_x = (Decoder20(_input))?;
let whitepoint_y = (Decoder20(_input))?;
let red_x = (Decoder20(_input))?;
let red_y = (Decoder20(_input))?;
let green_x = (Decoder20(_input))?;
let green_y = (Decoder20(_input))?;
let blue_x = (Decoder20(_input))?;
let blue_y = (Decoder20(_input))?;
PResult::Ok(png_chrm { whitepoint_x, whitepoint_y, red_x, red_y, green_x, green_y, blue_x, blue_y })
}

fn Decoder_png_gama<>(_input: &mut Parser<'_>) -> Result<png_gama, ParseError> {
let gamma = (Decoder20(_input))?;
PResult::Ok(png_gama { gamma })
}

fn Decoder_png_iccp<>(_input: &mut Parser<'_>) -> Result<png_iccp, ParseError> {
let profile_name = ((|| {
let tuple_var = {
let field0 = (Decoder201(_input))?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(14485842416732585139u64));
})
})())?;
(field0, field1)
};
PResult::Ok(match tuple_var {
(x, __null) => {
x
}
})
})())?;
let compression_method = ((|| {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
x == 0u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8179432974518885725u64));
};
PResult::Ok(ret)
})())?;
let compressed_profile = (Decoder202(_input))?;
PResult::Ok(png_iccp { profile_name, compression_method, compressed_profile })
}

fn Decoder_png_sbit<>(_input: &mut Parser<'_>, ihdr: png_ihdr) -> Result<png_sbit, ParseError> {
let ret = match ihdr.data.color_type.clone() {
0u8 => {
let inner = {
let sig_greyscale_bits = (Decoder24(_input))?;
png_sbit_color_type_0 { sig_greyscale_bits }
};
png_sbit::color_type_0(inner)
},

2u8 => {
let inner = {
let sig_red_bits = (Decoder24(_input))?;
let sig_green_bits = (Decoder24(_input))?;
let sig_blue_bits = (Decoder24(_input))?;
png_sbit_color_type_2 { sig_red_bits, sig_green_bits, sig_blue_bits }
};
png_sbit::color_type_2(inner)
},

3u8 => {
let inner = {
let sig_red_bits = (Decoder24(_input))?;
let sig_green_bits = (Decoder24(_input))?;
let sig_blue_bits = (Decoder24(_input))?;
png_sbit_color_type_2 { sig_red_bits, sig_green_bits, sig_blue_bits }
};
png_sbit::color_type_3(inner)
},

4u8 => {
let inner = {
let sig_greyscale_bits = (Decoder24(_input))?;
let sig_alpha_bits = (Decoder24(_input))?;
png_sbit_color_type_4 { sig_greyscale_bits, sig_alpha_bits }
};
png_sbit::color_type_4(inner)
},

6u8 => {
let inner = {
let sig_red_bits = (Decoder24(_input))?;
let sig_green_bits = (Decoder24(_input))?;
let sig_blue_bits = (Decoder24(_input))?;
let sig_alpha_bits = (Decoder24(_input))?;
png_sbit_color_type_6 { sig_red_bits, sig_green_bits, sig_blue_bits, sig_alpha_bits }
};
png_sbit::color_type_6(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
}

fn Decoder_png_srgb<>(_input: &mut Parser<'_>) -> Result<png_srgb, ParseError> {
let rendering_intent = ((|| {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
x <= 3u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(5152282179373241998u64));
};
PResult::Ok(ret)
})())?;
PResult::Ok(png_srgb { rendering_intent })
}

fn Decoder_png_itxt<>(_input: &mut Parser<'_>) -> Result<png_itxt, ParseError> {
let keyword = ((|| {
let tuple_var = {
let field0 = (Decoder196(_input))?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(13780055874544357936u64));
})
})())?;
(field0, field1)
};
PResult::Ok(match tuple_var {
(x, __null) => {
x
}
})
})())?;
let compression_flag = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if (ByteSet::from_bits([3, 0, 0, 0])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(16909208071962620789u64));
})
})())?;
let compression_method = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(7933266403838225878u64));
})
})())?;
let language_tag = (Decoder_base_asciiz_string(_input))?;
let translated_keyword = ((|| {
let tuple_var = {
let field0 = (Decoder198(_input))?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(12100308281236296642u64));
})
})())?;
(field0, field1)
};
PResult::Ok(match tuple_var {
(x, __null) => {
x
}
})
})())?;
let text = ((|| {
let ret = match compression_flag == 1u8 {
true => {
_input.start_alt();
let res = (|| {
let inner = {
let inner = {
let zlib = (Decoder199(_input))?;
let mut tmp = Parser::new(zlib.data.inflate.as_slice());
let reparser = &mut tmp;
(Decoder200(reparser))?
};
png_itxt_text_compressed::valid(inner)
};
PResult::Ok(png_itxt_text::compressed(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
_input.next_alt(true)?;
}
};
let res = (|| {
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
png_itxt_text_compressed::invalid(inner)
};
PResult::Ok(png_itxt_text::compressed(inner))
})();
match res {
Ok(inner) => {
return PResult::Ok(inner);
},

Err(_e) => {
return Err(_e);
}
};
},

false => {
let inner = (Decoder193(_input))?;
png_itxt_text::uncompressed(inner)
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(png_itxt { keyword, compression_flag, compression_method, language_tag, translated_keyword, text })
}

fn Decoder_png_text<>(_input: &mut Parser<'_>) -> Result<png_text, ParseError> {
let keyword = ((|| {
let tuple_var = {
let field0 = (Decoder195(_input))?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(9041056097467752267u64));
})
})())?;
(field0, field1)
};
PResult::Ok(match tuple_var {
(x, __null) => {
x
}
})
})())?;
let text = ((|| {
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
let next_elem = (Decoder150(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(png_text { keyword, text })
}

fn Decoder_png_ztxt<>(_input: &mut Parser<'_>) -> Result<png_ztxt, ParseError> {
let keyword = ((|| {
let tuple_var = {
let field0 = (Decoder191(_input))?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(5025197102194587315u64));
})
})())?;
(field0, field1)
};
PResult::Ok(match tuple_var {
(x, __null) => {
x
}
})
})())?;
let compression_method = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(3810055094392728880u64));
})
})())?;
let compressed_text = ((|| {
let zlib = (Decoder192(_input))?;
let mut tmp = Parser::new(zlib.data.inflate.as_slice());
let reparser = &mut tmp;
Decoder193(reparser)
})())?;
PResult::Ok(png_ztxt { keyword, compression_method, compressed_text })
}

fn Decoder_png_bkgd<>(_input: &mut Parser<'_>, ihdr: png_ihdr) -> Result<png_bkgd, ParseError> {
let ret = match ihdr.data.color_type.clone() {
0u8 => {
let inner = {
let greyscale = (Decoder23(_input))?;
png_bkgd_color_type_0 { greyscale }
};
png_bkgd::color_type_0(inner)
},

4u8 => {
let inner = {
let greyscale = (Decoder23(_input))?;
png_bkgd_color_type_0 { greyscale }
};
png_bkgd::color_type_4(inner)
},

2u8 => {
let inner = {
let red = (Decoder23(_input))?;
let green = (Decoder23(_input))?;
let blue = (Decoder23(_input))?;
png_bkgd_color_type_2 { red, green, blue }
};
png_bkgd::color_type_2(inner)
},

6u8 => {
let inner = {
let red = (Decoder23(_input))?;
let green = (Decoder23(_input))?;
let blue = (Decoder23(_input))?;
png_bkgd_color_type_2 { red, green, blue }
};
png_bkgd::color_type_6(inner)
},

3u8 => {
let inner = {
let palette_index = (Decoder24(_input))?;
png_bkgd_color_type_3 { palette_index }
};
png_bkgd::color_type_3(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
}

fn Decoder_png_hist<>(_input: &mut Parser<'_>) -> Result<png_hist, ParseError> {
let histogram = ((|| {
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
let next_elem = (Decoder23(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(png_hist { histogram })
}

fn Decoder_png_phys<>(_input: &mut Parser<'_>) -> Result<png_phys, ParseError> {
let pixels_per_unit_x = (Decoder20(_input))?;
let pixels_per_unit_y = (Decoder20(_input))?;
let unit_specifier = (Decoder24(_input))?;
PResult::Ok(png_phys { pixels_per_unit_x, pixels_per_unit_y, unit_specifier })
}

fn Decoder_png_splt<>(_input: &mut Parser<'_>) -> Result<png_splt, ParseError> {
let palette_name = ((|| {
let tuple_var = {
let field0 = (Decoder190(_input))?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(15252450768049745444u64));
})
})())?;
(field0, field1)
};
PResult::Ok(match tuple_var {
(x, __null) => {
x
}
})
})())?;
let sample_depth = ((|| {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
(x == 8u8) || (x == 16u8)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(5599331855309773603u64));
};
PResult::Ok(ret)
})())?;
let pallette = ((|| {
let ret = match sample_depth {
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
let red = (Decoder24(_input))?;
let green = (Decoder24(_input))?;
let blue = (Decoder24(_input))?;
let alpha = (Decoder24(_input))?;
let frequency = (Decoder23(_input))?;
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
let red = (Decoder23(_input))?;
let green = (Decoder23(_input))?;
let blue = (Decoder23(_input))?;
let alpha = (Decoder23(_input))?;
let frequency = (Decoder23(_input))?;
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
};
PResult::Ok(ret)
})())?;
PResult::Ok(png_splt { palette_name, sample_depth, pallette })
}

fn Decoder_png_time<>(_input: &mut Parser<'_>) -> Result<png_time, ParseError> {
let year = (Decoder23(_input))?;
let month = (Decoder24(_input))?;
let day = (Decoder24(_input))?;
let hour = (Decoder24(_input))?;
let minute = (Decoder24(_input))?;
let second = (Decoder24(_input))?;
PResult::Ok(png_time { year, month, day, hour, minute, second })
}

fn Decoder190<>(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
1
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
2
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
3
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
4
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
5
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
6
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
7
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
8
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
9
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
10
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
11
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
12
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
13
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
14
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
15
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
16
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
17
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
18
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
19
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
20
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
21
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
22
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
23
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
24
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
25
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
26
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
27
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
28
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
29
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
30
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
31
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
32
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
33
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
34
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
35
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
36
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
37
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
38
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
39
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
40
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
41
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
42
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
43
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
44
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
45
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
46
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
47
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
48
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
49
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
50
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
51
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
52
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
53
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
54
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
55
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
56
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
57
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
58
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
59
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
60
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
61
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
62
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
63
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
64
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
65
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
66
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
67
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
68
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
69
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
70
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
71
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
72
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
73
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
74
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
75
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
76
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
77
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
78
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
79
},

_ => {
return Err(ParseError::ExcludedBranch(2988545765690796708u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6215067399528787845u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17176374570344757031u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3566920116549027235u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7979287392867129207u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3653195934333285574u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16671136947067655757u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10721249873135158334u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8898504689444561451u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(441240706992005484u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3950014938140253048u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10046433636842398056u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11079395374415646537u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(757122060916971772u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5986772336072340665u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8812292064350598352u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10645729856418057640u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2908689796368760670u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4316175446384649956u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13311790038092155306u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4598583460226006268u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(275550262640764009u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11490274700962832028u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15680765559661576738u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1587806253186841834u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11297314001547702431u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18399269270080151498u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(30874382969105279u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9791114990321288281u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1595897747104696027u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2481175643332430741u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15702070659753069395u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4338497647520366709u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14600508952542130472u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(460669108121189046u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8971553008180040990u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1310624491311340594u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11225936372640404826u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1029952099207838423u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17327099206515189757u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14652068248613900169u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3089242474000390105u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8785324329127396734u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14646838598150249928u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14949659785259585833u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(562280208679883345u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1426091679331900812u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5775567136742802567u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12616585043782016404u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13260893460097040029u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5931637197703965434u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(144820728017547457u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17376845638706524656u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9452754313802575046u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15362228896620571409u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17715157964684782708u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9465826900165497155u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14571733789425208869u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6611121695530188940u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9660375186237087060u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11684704871632490773u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17081364943144677526u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9710347097769530785u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2466404913032252300u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4149374297771033461u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5869833854865239916u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13372240079418200167u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10504805981668764726u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5250529337043320049u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4159150678276994707u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5633181162991720115u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13699185545200670755u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1705816027536538342u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5442671660922928935u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15425278341212694869u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17519877619184542224u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2881491179107816928u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3382972670024593436u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3782015444980282771u64));
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
return Err(ParseError::ExcludedBranch(16437491640759399344u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder191<>(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
1
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
2
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
3
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
4
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
5
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
6
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
7
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
8
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
9
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
10
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
11
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
12
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
13
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
14
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
15
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
16
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
17
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
18
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
19
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
20
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
21
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
22
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
23
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
24
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
25
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
26
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
27
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
28
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
29
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
30
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
31
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
32
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
33
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
34
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
35
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
36
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
37
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
38
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
39
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
40
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
41
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
42
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
43
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
44
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
45
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
46
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
47
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
48
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
49
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
50
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
51
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
52
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
53
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
54
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
55
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
56
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
57
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
58
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
59
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
60
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
61
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
62
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
63
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
64
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
65
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
66
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
67
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
68
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
69
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
70
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
71
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
72
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
73
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
74
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
75
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
76
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
77
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
78
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
79
},

_ => {
return Err(ParseError::ExcludedBranch(14908034280634314212u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3857251694269754536u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5041932778480497827u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4514882359072253410u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13286112843953025473u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15541745679144160988u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16366772103869684910u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16354405091856567045u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15047091743774256727u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(89425857491903975u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4693381172248071257u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10755314521203959634u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6506136565977297327u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17617969929120925997u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9726369214549228587u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7323683635844484191u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2785003688991605442u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17143373953369837893u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4319635423750959827u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3022274272397071746u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14869310993580240597u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7391010474856587818u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7025852844623144626u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13513276483415770047u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1292030895245088137u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8527564631842216417u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10545338643554355463u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3529321240709143928u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9481712747994656857u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11863749907612277673u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8408007422644693463u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2137824516769298342u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17504519908837839248u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5377488665469248769u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10179224889113195865u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2526895115167988738u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7617559532652678498u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4051432836859471288u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14741532803131558705u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10276185770177327434u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16586067667342674799u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2497564037439806374u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8400230153663738380u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3348696109325747597u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3775958429611079664u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15402873290357121767u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7699494290217942468u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15920470819585350103u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2212229856396822503u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13487040053473546512u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7142800526810245879u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15228292911834498078u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14228769055716402588u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2096340590066926443u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6262990113977164585u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16057321717383774211u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5291563035461819971u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14334546370445091615u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18123524617814052121u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3966750415843992376u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3512844604428973098u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14732383712267459599u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13357689525249598011u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16153196263894316997u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11930340298772268405u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8442068294613883251u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6850311165997710199u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2310756498479603065u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5816243344835903905u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16915030153566842158u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5877293982949196624u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2440323998191641952u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3265375406401843811u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11458871772722170518u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12156808917241975156u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15301408058960503169u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4255813677013328811u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7761201277159812979u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15409834313606096443u64));
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
return Err(ParseError::ExcludedBranch(1977196682923428575u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder192<>(_input: &mut Parser<'_>) -> Result<zlib_main, ParseError> {
let compression_method_flags = ((|| {
let inner = {
let packed_bits = _input.read_byte()?;
zlib_main_compression_method_flags { compression_info: packed_bits >> 4u8 & 15u8, compression_method: packed_bits >> 0u8 & 15u8 }
};
let is_valid = {
let method_info = inner;
method_info.compression_method.clone() == 8u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7572218778908935167u64));
};
PResult::Ok(ret)
})())?;
let flags = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(zlib_main_flags { flevel: packed_bits >> 6u8 & 3u8, fdict: packed_bits >> 5u8 & 1u8 > 0u8, fcheck: packed_bits >> 0u8 & 31u8 })
})())?;
let dict_id = ((|| {
let ret = if flags.fdict.clone() {
Some((Decoder20(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let data = ((|| {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
PResult::Ok(ret)
})())?;
let adler32 = (Decoder20(_input))?;
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

fn Decoder193<>(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
Decoder194(_input)
}

fn Decoder194<>(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(byte)) => {
0
},

224u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(byte)) => {
0
},

237u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(byte)) => {
0
},

240u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(byte)) => {
0
},

244u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(12520942526725743695u64));
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

fn Decoder195<>(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
1
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
2
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
3
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
4
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
5
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
6
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
7
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
8
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
9
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
10
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
11
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
12
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
13
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
14
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
15
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
16
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
17
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
18
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
19
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
20
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
21
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
22
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
23
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
24
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
25
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
26
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
27
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
28
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
29
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
30
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
31
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
32
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
33
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
34
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
35
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
36
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
37
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
38
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
39
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
40
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
41
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
42
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
43
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
44
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
45
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
46
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
47
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
48
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
49
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
50
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
51
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
52
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
53
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
54
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
55
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
56
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
57
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
58
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
59
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
60
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
61
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
62
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
63
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
64
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
65
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
66
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
67
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
68
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
69
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
70
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
71
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
72
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
73
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
74
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
75
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
76
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
77
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
78
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
79
},

_ => {
return Err(ParseError::ExcludedBranch(8609603324479018835u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8973115486793444912u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2872401692234135189u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3556125603266802472u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16173438081116718978u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3532284049956758497u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1214015012285792661u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18287039256282529512u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6040541316788286267u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6926344996361682930u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1335666833550050804u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12949211704949756849u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4317621489266312319u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(868434092978367357u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(496554245154466536u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8244423856248085319u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14587337454519331439u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17469688165057057832u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17175946860967673146u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4411296728399804345u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13980028702310773940u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4312000683241916062u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3334424754000117797u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(527013363415661133u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4717292700030555590u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(653172816195928449u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(581976537271693173u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10727170711560529872u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17095884813960222885u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(974442582679666596u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4054889295781446756u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14775695284177607002u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6914153644311469309u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11188141958235814565u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15180040648844909945u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18312372211249290289u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6298802191889347847u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11017185023542530489u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15395924218065814216u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14060832500024730160u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3344470983013526080u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10251322642913008199u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10455173543833790502u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1789963736344690987u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15079752956117840734u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7980438082889912554u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6840264564700035644u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2232617280930402416u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10367781439645737926u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6016311883578211466u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14984769167642974435u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11733915986279978987u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12796096677294114859u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4290849149416714172u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2598935692633451852u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8938360652744148620u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3379987508464424555u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11173052675913205247u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1780493074893517013u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5013701440573848944u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15508559755986719975u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14782609537615230269u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17984802637964449391u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4337320827937013231u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9730514595131843432u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11851311358763079517u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4195586632355556000u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3595083506735372973u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7160308966995483862u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10995965655495216468u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(329678914966083835u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6740511944232229269u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16842149192073737461u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2734069901426030304u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8313738771561634204u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5782291498890001703u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10699246393255895644u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16725113430369695556u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10581544802310072854u64));
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
return Err(ParseError::ExcludedBranch(5905249956584638130u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder196<>(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
1
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
2
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
3
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
4
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
5
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
6
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
7
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
8
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
9
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
10
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
11
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
12
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
13
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
14
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
15
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
16
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
17
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
18
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
19
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
20
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
21
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
22
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
23
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
24
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
25
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
26
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
27
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
28
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
29
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
30
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
31
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
32
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
33
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
34
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
35
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
36
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
37
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
38
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
39
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
40
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
41
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
42
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
43
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
44
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
45
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
46
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
47
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
48
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
49
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
50
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
51
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
52
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
53
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
54
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
55
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
56
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
57
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
58
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
59
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
60
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
61
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
62
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
63
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
64
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
65
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
66
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
67
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
68
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
69
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
70
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
71
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
72
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
73
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
74
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
75
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
76
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
77
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
78
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
79
},

_ => {
return Err(ParseError::ExcludedBranch(9788088961065186348u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13475201244121026739u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10514680212862927148u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1404288588910574651u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9722140062746187322u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14672290401516300248u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1570732024996461745u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3970524133912495743u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10022572355386706896u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7175325427369635964u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10941190781838111798u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8533828000951561331u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7620563226324297760u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3141147280724933447u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5155365750119375266u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6019636780841875619u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9286045119570450224u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12351156548319118754u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16002368984748306955u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2518353173875372183u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1057280760989719477u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4360542291358407189u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11728165168471348185u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10495009576779598530u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7128967683436470476u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3036714361878569528u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17797183748027644744u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4766620025042958433u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3832366084378679427u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12230053902523648559u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6309917041149428265u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14708340496221370872u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8149951888585961823u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9665308631980535174u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17114848294724579474u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6471433373632717886u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11488186652992817125u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16428598146864409139u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11934620222090497034u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7597388605883543966u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1942606498606899849u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9288520242296510642u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12753818979092594971u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9662462399070735352u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8414850703985147087u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14428558544277089677u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17505260505493155057u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4244096627290639074u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16278372111013231116u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4797145928961714098u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11396441833338683368u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18000244536401340529u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17499296848696657524u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17871409524597489995u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18319329181163975393u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9728697411637566730u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9691647707184281179u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7972156773864461481u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11074874806330017418u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12662813720965160010u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3243138897542356980u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15920917552321559809u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(835240278964504247u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7150174851274887183u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11306355108769900243u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9864490923722431849u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1995664400520545068u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14904826071154862403u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9819888267966403254u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9848248130448711783u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13072404838946064765u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(608962121954693839u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4003079682566975393u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4315160623525052581u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1862457564087794359u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4725257428709613290u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11823569341425521789u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14474361861190022573u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8440332211549640930u64));
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
return Err(ParseError::ExcludedBranch(2930236810929305906u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder_base_asciiz_string<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(12507789284617859731u64));
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
return Err(ParseError::ExcludedBranch(13045651918551022996u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(4636203914394120912u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder198<>(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(byte)) => {
0
},

224u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(byte)) => {
0
},

237u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(byte)) => {
0
},

240u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(byte)) => {
0
},

244u8 => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(12770885481695079655u64));
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

fn Decoder199<>(_input: &mut Parser<'_>) -> Result<zlib_main, ParseError> {
let compression_method_flags = ((|| {
let inner = {
let packed_bits = _input.read_byte()?;
zlib_main_compression_method_flags { compression_info: packed_bits >> 4u8 & 15u8, compression_method: packed_bits >> 0u8 & 15u8 }
};
let is_valid = {
let method_info = inner;
method_info.compression_method.clone() == 8u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(5713736954960046494u64));
};
PResult::Ok(ret)
})())?;
let flags = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(zlib_main_flags { flevel: packed_bits >> 6u8 & 3u8, fdict: packed_bits >> 5u8 & 1u8 > 0u8, fcheck: packed_bits >> 0u8 & 31u8 })
})())?;
let dict_id = ((|| {
let ret = if flags.fdict.clone() {
Some((Decoder20(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let data = ((|| {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
PResult::Ok(ret)
})())?;
let adler32 = (Decoder20(_input))?;
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

fn Decoder200<>(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([18446744073709551614, 18446744073709551615, 0, 0])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(byte)) => {
0
},

224u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(byte)) => {
0
},

237u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(byte)) => {
0
},

240u8 => {
0
},

byte if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(byte)) => {
0
},

244u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(5286459060741837465u64));
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

fn Decoder201<>(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
1
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
2
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
3
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
4
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
5
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
6
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
7
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
8
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
9
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
10
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
11
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
12
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
13
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
14
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
15
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
16
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
17
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
18
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
19
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
20
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
21
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
22
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
23
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
24
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
25
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
26
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
27
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
28
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
29
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
30
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
31
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
32
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
33
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
34
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
35
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
36
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
37
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
38
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
39
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
40
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
41
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
42
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
43
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
44
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
45
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
46
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
47
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
48
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
49
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
50
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
51
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
52
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
53
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
54
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
55
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
56
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
57
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
58
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
59
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
60
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
61
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
62
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
63
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
64
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
65
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
66
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
67
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
68
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
69
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
70
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
71
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
72
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
73
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
74
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
75
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
76
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
77
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
78
},

byte if ((ByteSet::from_bits([18446744069414584320, 9223372036854775807, 18446744065119617024, 18446744073709551615])).contains(byte)) => {
79
},

_ => {
return Err(ParseError::ExcludedBranch(8347358143797316202u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9289217078927880343u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7619374930571283120u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5502272810018950495u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9422091606538556682u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1986031727359985319u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2609726213773413101u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4688564412461098550u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4942686269660057715u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9251320185524837870u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16934907723271638747u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12709578918531114377u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4227980562164448700u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6852038687184722387u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(18441633166538026093u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(401788761305210720u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5252251088830583126u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5396814765406749711u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12786091976415550822u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15224635086995724042u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12911570055982528377u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17578303967306014514u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7236895128762201498u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10396295304006156759u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14603388329461445014u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15092052115449480204u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5644646321622228807u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12876869868341544086u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6757475294452472580u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5316076842439816359u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13241229060561103069u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8896324603808149952u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8480214967847684808u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4089331332020821320u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7660172056721883940u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8118664158941399609u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11259402406166723834u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2562653558681619679u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9066549901844041194u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17788403991563985894u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14014946361797352706u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17148278567107839372u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10059971034460660304u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11566301548302897286u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4726882489015010540u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16731067715008063623u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10050470520268747158u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7028862034699171644u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6768457941542394460u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3336832804021932765u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14287222125204946454u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14784334396256787566u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17975519714191498979u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15547793456796641064u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2708015872450907711u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17628659998315744600u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7574719322668410505u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6886530205512451724u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4782096577080450146u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14577481400621526493u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14148913971033431093u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17190949120592669315u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8699787504309122591u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(78099073500381561u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5364461768558038471u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7135191863324865081u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5733684513012333041u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3269600573864009399u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16604650314446872341u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14275129881911283147u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8473763373531540844u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9532564966458988001u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1456571545446476568u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14908888453225293887u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4384065895993710795u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14063858022942822585u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7083101893872508858u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15979415359593570628u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17922878356929717082u64));
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
return Err(ParseError::ExcludedBranch(10857757036842967573u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder202<>(_input: &mut Parser<'_>) -> Result<zlib_main, ParseError> {
let compression_method_flags = ((|| {
let inner = {
let packed_bits = _input.read_byte()?;
zlib_main_compression_method_flags { compression_info: packed_bits >> 4u8 & 15u8, compression_method: packed_bits >> 0u8 & 15u8 }
};
let is_valid = {
let method_info = inner;
method_info.compression_method.clone() == 8u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(15321253101048235163u64));
};
PResult::Ok(ret)
})())?;
let flags = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(zlib_main_flags { flevel: packed_bits >> 6u8 & 3u8, fdict: packed_bits >> 5u8 & 1u8 > 0u8, fcheck: packed_bits >> 0u8 & 31u8 })
})())?;
let dict_id = ((|| {
let ret = if flags.fdict.clone() {
Some((Decoder20(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let data = ((|| {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
PResult::Ok(ret)
})())?;
let adler32 = (Decoder20(_input))?;
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

fn Decoder203<>(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(14285291594842403582u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 72 {
b
} else {
return Err(ParseError::ExcludedBranch(7161350271661739096u64));
})
})())?;
let field2 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(14445026714989735755u64));
})
})())?;
let field3 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 82 {
b
} else {
return Err(ParseError::ExcludedBranch(7573348369521592997u64));
})
})())?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder_png_ihdr_data<>(_input: &mut Parser<'_>) -> Result<png_ihdr_data, ParseError> {
let width = (Decoder20(_input))?;
let height = (Decoder20(_input))?;
let bit_depth = (Decoder24(_input))?;
let color_type = (Decoder24(_input))?;
let compression_method = (Decoder24(_input))?;
let filter_method = (Decoder24(_input))?;
let interlace_method = (Decoder24(_input))?;
PResult::Ok(png_ihdr_data { width, height, bit_depth, color_type, compression_method, filter_method, interlace_method })
}

fn Decoder_mpeg4_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 15885886181646629118u64)
},

_ => {
(try_sub!(size_field, 8u32, 734991270787736827u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(102u8, 116u8, 121u8, 112u8) => {
let inner = {
let major_brand = (Decoder206(_input))?;
let minor_version = (Decoder20(_input))?;
let compatible_brands = ((|| {
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
let next_elem = (Decoder206(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
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
let field0 = (Decoder20(_input))?;
let field1 = ((|| {
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
PResult::Ok(accum)
})())?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_atom { size_field, r#type, size, data })
}

fn Decoder206<>(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8), ParseError> {
let field0 = (Decoder150(_input))?;
let field1 = (Decoder150(_input))?;
let field2 = (Decoder150(_input))?;
let field3 = (Decoder150(_input))?;
PResult::Ok((field0, field1, field2, field3))
}

fn Decoder_mpeg4_meta_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_meta_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 14349391981174483355u64)
},

_ => {
(try_sub!(size_field, 8u32, 18167425999583150549u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
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
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let predefined = (Decoder20(_input))?;
let handler_type = (Decoder206(_input))?;
let reserved = ((|| {
let field0 = (Decoder20(_input))?;
let field1 = (Decoder20(_input))?;
let field2 = (Decoder20(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let name = (Decoder213(_input))?;
mpeg4_meta_atom_data_hdlr { version, flags, predefined, handler_type, reserved, name }
};
mpeg4_meta_atom_data::hdlr(inner)
},

(112u8, 105u8, 116u8, 109u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let item_ID = ((|| {
let ret = match version == 0u8 {
true => {
let inner = (Decoder23(_input))?;
mpeg4_meta_atom_data_pitm_item_ID::yes(inner)
},

false => {
let inner = (Decoder20(_input))?;
mpeg4_meta_atom_data_pitm_item_ID::no(inner)
}
};
PResult::Ok(ret)
})())?;
mpeg4_meta_atom_data_pitm { version, flags, item_ID }
};
mpeg4_meta_atom_data::pitm(inner)
},

(105u8, 105u8, 110u8, 102u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let entry_count = ((|| {
let ret = match version == 0u8 {
true => {
let x = (Decoder23(_input))?;
x as u32
},

false => {
(Decoder20(_input))?
}
};
PResult::Ok(ret)
})())?;
let item_info_entry = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder_mpeg4_iinf_atom(_input))?);
}
PResult::Ok(accum)
})())?;
mpeg4_meta_atom_data_iinf { version, flags, entry_count, item_info_entry }
};
mpeg4_meta_atom_data::iinf(inner)
},

(105u8, 114u8, 101u8, 102u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let single_item_reference = ((|| {
let ret = match version {
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
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 13819997351842221509u64)
},

_ => {
(try_sub!(size_field, 8u32, 1644793874183523166u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let from_item_ID = (Decoder23(_input))?;
let reference_count = (Decoder23(_input))?;
let to_item_ID = ((|| {
let mut accum = Vec::new();
for _ in 0..reference_count {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(mpeg4_meta_atom_data_iref_single_item_reference_small_data { from_item_ID, reference_count, to_item_ID })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
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
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 14114750189009003637u64)
},

_ => {
(try_sub!(size_field, 8u32, 16396082708135795071u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let from_item_ID = (Decoder20(_input))?;
let reference_count = (Decoder23(_input))?;
let to_item_ID = ((|| {
let mut accum = Vec::new();
for _ in 0..reference_count {
accum.push((Decoder20(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(mpeg4_meta_atom_data_iref_single_item_reference_large_data { from_item_ID, reference_count, to_item_ID })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
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
};
PResult::Ok(ret)
})())?;
mpeg4_meta_atom_data_iref { version, flags, single_item_reference }
};
mpeg4_meta_atom_data::iref(inner)
},

(105u8, 108u8, 111u8, 99u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let offset_size_length_size = (Decoder24(_input))?;
let base_offset_size_index_size = (Decoder24(_input))?;
let offset_size = offset_size_length_size >> 4u8;
let length_size = offset_size_length_size & 7u8;
let base_offset_size = base_offset_size_index_size >> 4u8;
let index_size = {
let ret = match version > 0u8 {
true => {
base_offset_size_index_size & 7u8
},

false => {
0u8
}
};
ret
};
let item_count = ((|| {
let ret = match version < 2u8 {
true => {
let x = (Decoder23(_input))?;
x as u32
},

false => {
(Decoder20(_input))?
}
};
PResult::Ok(ret)
})())?;
let items = ((|| {
let mut accum = Vec::new();
for _ in 0..item_count {
accum.push({
let item_ID = ((|| {
let ret = match version < 2u8 {
true => {
let x = (Decoder23(_input))?;
x as u32
},

false => {
(Decoder20(_input))?
}
};
PResult::Ok(ret)
})())?;
let construction_method = ((|| {
let ret = if version > 0u8 {
Some((Decoder23(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let data_reference_index = (Decoder23(_input))?;
let base_offset = ((|| {
let ret = match base_offset_size {
0u8 => {
0u64
},

4u8 => {
let x = (Decoder20(_input))?;
x as u64
},

8u8 => {
(Decoder99(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let extent_count = (Decoder23(_input))?;
let extents = ((|| {
let mut accum = Vec::new();
for _ in 0..extent_count {
accum.push({
let extent_index = ((|| {
let ret = match index_size {
0u8 => {
0u64
},

4u8 => {
let x = (Decoder20(_input))?;
x as u64
},

8u8 => {
(Decoder99(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let extent_offset = ((|| {
let ret = match offset_size {
0u8 => {
0u64
},

4u8 => {
let x = (Decoder20(_input))?;
x as u64
},

8u8 => {
(Decoder99(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let extent_length = ((|| {
let ret = match length_size {
0u8 => {
0u64
},

4u8 => {
let x = (Decoder20(_input))?;
x as u64
},

8u8 => {
(Decoder99(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
mpeg4_meta_atom_data_iloc_items_extents { extent_index, extent_offset, extent_length }
});
}
PResult::Ok(accum)
})())?;
mpeg4_meta_atom_data_iloc_items { item_ID, construction_method, data_reference_index, base_offset, extent_count, extents }
});
}
PResult::Ok(accum)
})())?;
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
let next_elem = (Decoder24(_input))?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_meta_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_meta_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_moov_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_moov_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 9620443865397033050u64)
},

_ => {
(try_sub!(size_field, 8u32, 5546123200965512193u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(109u8, 118u8, 104u8, 100u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let fields = ((|| {
let ret = match version {
0u8 => {
let inner = {
let creation_time = (Decoder20(_input))?;
let modification_time = (Decoder20(_input))?;
let timescale = (Decoder20(_input))?;
let duration = (Decoder20(_input))?;
mpeg4_moov_atom_data_mvhd_fields_version0 { creation_time, modification_time, timescale, duration }
};
mpeg4_moov_atom_data_mvhd_fields::version0(inner)
},

1u8 => {
let inner = {
let creation_time = (Decoder99(_input))?;
let modification_time = (Decoder99(_input))?;
let timescale = (Decoder20(_input))?;
let duration = (Decoder99(_input))?;
mpeg4_moov_atom_data_mvhd_fields_version1 { creation_time, modification_time, timescale, duration }
};
mpeg4_moov_atom_data_mvhd_fields::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let rate = (Decoder20(_input))?;
let volume = (Decoder23(_input))?;
let reserved1 = (Decoder23(_input))?;
let reserved2 = ((|| {
let field0 = (Decoder20(_input))?;
let field1 = (Decoder20(_input))?;
PResult::Ok((field0, field1))
})())?;
let matrix = ((|| {
let mut accum = Vec::new();
for _ in 0..9u8 {
accum.push((Decoder20(_input))?);
}
PResult::Ok(accum)
})())?;
let pre_defined = ((|| {
let mut accum = Vec::new();
for _ in 0..6u8 {
accum.push((Decoder20(_input))?);
}
PResult::Ok(accum)
})())?;
let next_track_ID = (Decoder20(_input))?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_moov_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_moov_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_trak_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_trak_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 3933597635930613605u64)
},

_ => {
(try_sub!(size_field, 8u32, 9403121491749669432u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(116u8, 107u8, 104u8, 100u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let fields = ((|| {
let ret = match version {
0u8 => {
let inner = {
let creation_time = (Decoder20(_input))?;
let modification_time = (Decoder20(_input))?;
let track_ID = (Decoder20(_input))?;
let reserved = (Decoder20(_input))?;
let duration = (Decoder20(_input))?;
mpeg4_trak_atom_data_tkhd_fields_version0 { creation_time, modification_time, track_ID, reserved, duration }
};
mpeg4_trak_atom_data_tkhd_fields::version0(inner)
},

1u8 => {
let inner = {
let creation_time = (Decoder99(_input))?;
let modification_time = (Decoder99(_input))?;
let track_ID = (Decoder20(_input))?;
let reserved = (Decoder20(_input))?;
let duration = (Decoder99(_input))?;
mpeg4_trak_atom_data_tkhd_fields_version1 { creation_time, modification_time, track_ID, reserved, duration }
};
mpeg4_trak_atom_data_tkhd_fields::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let reserved2 = ((|| {
let field0 = (Decoder20(_input))?;
let field1 = (Decoder20(_input))?;
PResult::Ok((field0, field1))
})())?;
let layer = (Decoder23(_input))?;
let alternate_group = (Decoder23(_input))?;
let volume = (Decoder23(_input))?;
let reserved1 = (Decoder23(_input))?;
let matrix = ((|| {
let mut accum = Vec::new();
for _ in 0..9u8 {
accum.push((Decoder20(_input))?);
}
PResult::Ok(accum)
})())?;
let width = (Decoder20(_input))?;
let height = (Decoder20(_input))?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_trak_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_trak_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_udta_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_udta_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 1105171745447487568u64)
},

_ => {
(try_sub!(size_field, 8u32, 7981520858864097140u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(109u8, 101u8, 116u8, 97u8) => {
let field0 = (Decoder20(_input))?;
let field1 = ((|| {
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
PResult::Ok(accum)
})())?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_udta_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_udta_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_edts_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_edts_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 6351919260209655786u64)
},

_ => {
(try_sub!(size_field, 8u32, 17948395312093823900u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(101u8, 108u8, 115u8, 116u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let number_of_entries = (Decoder20(_input))?;
let edit_list_table = ((|| {
let mut accum = Vec::new();
for _ in 0..number_of_entries {
accum.push({
let track_duration = (Decoder20(_input))?;
let media_time = (Decoder20(_input))?;
let media_rate = (Decoder20(_input))?;
mpeg4_edts_atom_data_elst_edit_list_table { track_duration, media_time, media_rate }
});
}
PResult::Ok(accum)
})())?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_edts_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_edts_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_mdia_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_mdia_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 17332407069754301555u64)
},

_ => {
(try_sub!(size_field, 8u32, 7370180348639650351u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(104u8, 100u8, 108u8, 114u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let component_type = (Decoder20(_input))?;
let component_subtype = (Decoder206(_input))?;
let component_manufacturer = (Decoder20(_input))?;
let component_flags = (Decoder20(_input))?;
let component_flags_mask = (Decoder20(_input))?;
let component_name = (Decoder213(_input))?;
mpeg4_mdia_atom_data_hdlr { version, flags, component_type, component_subtype, component_manufacturer, component_flags, component_flags_mask, component_name }
};
mpeg4_mdia_atom_data::hdlr(inner)
},

(109u8, 100u8, 104u8, 100u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let fields = ((|| {
let ret = match version {
0u8 => {
let inner = {
let creation_time = (Decoder20(_input))?;
let modification_time = (Decoder20(_input))?;
let timescale = (Decoder20(_input))?;
let duration = (Decoder20(_input))?;
mpeg4_moov_atom_data_mvhd_fields_version0 { creation_time, modification_time, timescale, duration }
};
mpeg4_moov_atom_data_mvhd_fields::version0(inner)
},

1u8 => {
let inner = {
let creation_time = (Decoder99(_input))?;
let modification_time = (Decoder99(_input))?;
let timescale = (Decoder20(_input))?;
let duration = (Decoder99(_input))?;
mpeg4_moov_atom_data_mvhd_fields_version1 { creation_time, modification_time, timescale, duration }
};
mpeg4_moov_atom_data_mvhd_fields::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
let language = (Decoder23(_input))?;
let pre_defined = (Decoder23(_input))?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_mdia_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_mdia_atom { size_field, r#type, size, data })
}

fn Decoder213<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(7347749870002275487u64));
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
return Err(ParseError::ExcludedBranch(14279133095670415871u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10748847049349746078u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_mpeg4_minf_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_minf_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 17741760072369420240u64)
},

_ => {
(try_sub!(size_field, 8u32, 9851243859021733611u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(118u8, 109u8, 104u8, 100u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let graphicsmode = (Decoder23(_input))?;
let opcolor = ((|| {
let mut accum = Vec::new();
for _ in 0..3u8 {
accum.push((Decoder23(_input))?);
}
PResult::Ok(accum)
})())?;
mpeg4_minf_atom_data_vmhd { version, flags, graphicsmode, opcolor }
};
mpeg4_minf_atom_data::vmhd(inner)
},

(115u8, 109u8, 104u8, 100u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let balance = (Decoder23(_input))?;
let reserved = (Decoder23(_input))?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_minf_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_minf_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_dinf_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_dinf_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 6072942808717419822u64)
},

_ => {
(try_sub!(size_field, 8u32, 5357406925723651718u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(100u8, 114u8, 101u8, 102u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let number_of_entries = (Decoder20(_input))?;
let data = ((|| {
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
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 7977419944699061624u64)
},

_ => {
(try_sub!(size_field, 8u32, 12318721104400761032u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
mpeg4_stbl_atom_data_stsd_sample_entries { size_field, r#type, size, data }
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_dinf_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_dinf_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_stbl_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_stbl_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 11678443062630698028u64)
},

_ => {
(try_sub!(size_field, 8u32, 3320665455366264189u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(115u8, 116u8, 115u8, 100u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let entry_count = (Decoder20(_input))?;
let sample_entries = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 1398536204687975789u64)
},

_ => {
(try_sub!(size_field, 8u32, 3673300442962989464u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
mpeg4_stbl_atom_data_stsd_sample_entries { size_field, r#type, size, data }
});
}
PResult::Ok(accum)
})())?;
mpeg4_stbl_atom_data_stsd { version, flags, entry_count, sample_entries }
};
mpeg4_stbl_atom_data::stsd(inner)
},

(115u8, 116u8, 116u8, 115u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let entry_count = (Decoder20(_input))?;
let sample_entries = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let sample_count = (Decoder20(_input))?;
let sample_delta = (Decoder20(_input))?;
mpeg4_stbl_atom_data_stts_sample_entries { sample_count, sample_delta }
});
}
PResult::Ok(accum)
})())?;
mpeg4_stbl_atom_data_stts { version, flags, entry_count, sample_entries }
};
mpeg4_stbl_atom_data::stts(inner)
},

(99u8, 116u8, 116u8, 115u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let entry_count = (Decoder20(_input))?;
let sample_entries = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let sample_count = (Decoder20(_input))?;
let sample_offset = (Decoder20(_input))?;
mpeg4_stbl_atom_data_ctts_sample_entries { sample_count, sample_offset }
});
}
PResult::Ok(accum)
})())?;
mpeg4_stbl_atom_data_ctts { version, flags, entry_count, sample_entries }
};
mpeg4_stbl_atom_data::ctts(inner)
},

(115u8, 116u8, 115u8, 115u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let entry_count = (Decoder20(_input))?;
let sample_number = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder20(_input))?);
}
PResult::Ok(accum)
})())?;
mpeg4_stbl_atom_data_stss { version, flags, entry_count, sample_number }
};
mpeg4_stbl_atom_data::stss(inner)
},

(115u8, 116u8, 115u8, 99u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let entry_count = (Decoder20(_input))?;
let chunk_entries = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let first_chunk = (Decoder20(_input))?;
let samples_per_chunk = (Decoder20(_input))?;
let sample_description_index = (Decoder20(_input))?;
mpeg4_stbl_atom_data_stsc_chunk_entries { first_chunk, samples_per_chunk, sample_description_index }
});
}
PResult::Ok(accum)
})())?;
mpeg4_stbl_atom_data_stsc { version, flags, entry_count, chunk_entries }
};
mpeg4_stbl_atom_data::stsc(inner)
},

(115u8, 116u8, 115u8, 122u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let sample_size = (Decoder20(_input))?;
let sample_count = (Decoder20(_input))?;
let entry_size = ((|| {
let ret = if sample_size == 0u32 {
let mut accum = Vec::new();
for _ in 0..sample_count {
accum.push((Decoder20(_input))?);
}
Some(accum)
} else {
None
};
PResult::Ok(ret)
})())?;
mpeg4_stbl_atom_data_stsz { version, flags, sample_size, sample_count, entry_size }
};
mpeg4_stbl_atom_data::stsz(inner)
},

(115u8, 116u8, 99u8, 111u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let entry_count = (Decoder20(_input))?;
let chunk_offset = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder20(_input))?);
}
PResult::Ok(accum)
})())?;
mpeg4_stbl_atom_data_stco { version, flags, entry_count, chunk_offset }
};
mpeg4_stbl_atom_data::stco(inner)
},

(99u8, 111u8, 54u8, 52u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let entry_count = (Decoder20(_input))?;
let chunk_offset = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push((Decoder99(_input))?);
}
PResult::Ok(accum)
})())?;
mpeg4_stbl_atom_data_co64 { version, flags, entry_count, chunk_offset }
};
mpeg4_stbl_atom_data::co64(inner)
},

(115u8, 103u8, 112u8, 100u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let grouping_type = (Decoder20(_input))?;
let default_length = (Decoder20(_input))?;
let entry_count = (Decoder20(_input))?;
let sample_groups = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let description_length = ((|| {
let ret = match default_length == 0u32 {
true => {
(Decoder20(_input))?
},

false => {
default_length.clone()
}
};
PResult::Ok(ret)
})())?;
let sample_group_entry = ((|| {
let mut accum = Vec::new();
for _ in 0..description_length {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
mpeg4_stbl_atom_data_sgpd_sample_groups { description_length, sample_group_entry }
});
}
PResult::Ok(accum)
})())?;
mpeg4_stbl_atom_data_sgpd { version, flags, grouping_type, default_length, entry_count, sample_groups }
};
mpeg4_stbl_atom_data::sgpd(inner)
},

(115u8, 98u8, 103u8, 112u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let grouping_type = (Decoder20(_input))?;
let grouping_type_parameter = ((|| {
let ret = if version == 1u8 {
Some((Decoder20(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let entry_count = (Decoder20(_input))?;
let sample_groups = ((|| {
let mut accum = Vec::new();
for _ in 0..entry_count {
accum.push({
let sample_count = (Decoder20(_input))?;
let group_description_index = (Decoder20(_input))?;
mpeg4_stbl_atom_data_sbgp_sample_groups { sample_count, group_description_index }
});
}
PResult::Ok(accum)
})())?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_stbl_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_stbl_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_iinf_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_iinf_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 4152914559762097168u64)
},

_ => {
(try_sub!(size_field, 8u32, 8970999014112821604u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(105u8, 110u8, 102u8, 101u8) => {
let inner = {
let version = (Decoder24(_input))?;
let flags = ((|| {
let field0 = (Decoder24(_input))?;
let field1 = (Decoder24(_input))?;
let field2 = (Decoder24(_input))?;
PResult::Ok((field0, field1, field2))
})())?;
let fields = ((|| {
let ret = match version < 2u8 {
true => {
let inner = {
let item_ID = (Decoder23(_input))?;
let item_protection_index = (Decoder23(_input))?;
let item_name = (Decoder220(_input))?;
let content_type = (Decoder221(_input))?;
let content_encoding = (Decoder222(_input))?;
mpeg4_iinf_atom_data_infe_fields_yes { item_ID, item_protection_index, item_name, content_type, content_encoding }
};
mpeg4_iinf_atom_data_infe_fields::yes(inner)
},

false => {
let inner = {
let item_ID = ((|| {
let ret = match version == 2u8 {
true => {
let x = (Decoder23(_input))?;
x as u32
},

false => {
(Decoder20(_input))?
}
};
PResult::Ok(ret)
})())?;
let item_protection_index = (Decoder23(_input))?;
let item_type = (Decoder206(_input))?;
let item_name = (Decoder223(_input))?;
let extra_fields = ((|| {
let ret = match item_type {
(109u8, 105u8, 109u8, 101u8) => {
let inner = {
let content_type = (Decoder224(_input))?;
mpeg4_iinf_atom_data_infe_fields_no_extra_fields_mime { content_type }
};
mpeg4_iinf_atom_data_infe_fields_no_extra_fields::mime(inner)
},

(117u8, 114u8, 105u8, 32u8) => {
let inner = {
let item_uri_type = (Decoder224(_input))?;
mpeg4_iinf_atom_data_infe_fields_no_extra_fields_uri { item_uri_type }
};
mpeg4_iinf_atom_data_infe_fields_no_extra_fields::uri(inner)
},

_ => {
mpeg4_iinf_atom_data_infe_fields_no_extra_fields::unknown
}
};
PResult::Ok(ret)
})())?;
mpeg4_iinf_atom_data_infe_fields_no { item_ID, item_protection_index, item_type, item_name, extra_fields }
};
mpeg4_iinf_atom_data_infe_fields::no(inner)
}
};
PResult::Ok(ret)
})())?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_iinf_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_iinf_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_ilst_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_ilst_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 5095757730543354711u64)
},

_ => {
(try_sub!(size_field, 8u32, 17089130856162883194u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_ilst_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_ilst_atom { size_field, r#type, size, data })
}

fn Decoder_mpeg4_tool_atom<>(_input: &mut Parser<'_>) -> Result<mpeg4_tool_atom, ParseError> {
let size_field = (Decoder20(_input))?;
let r#type = (Decoder206(_input))?;
let size = ((|| {
let ret = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = (Decoder99(_input))?;
try_sub!(x, 16u64, 6515957116553005671u64)
},

_ => {
(try_sub!(size_field, 8u32, 13537165373980795457u64)) as u64
}
};
PResult::Ok(ret)
})())?;
let data = ((|| {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let ret = match r#type {
(100u8, 97u8, 116u8, 97u8) => {
let inner = {
let type_indicator = (Decoder20(_input))?;
let locale_indicator = (Decoder20(_input))?;
let value = ((|| {
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
let next_elem = (Decoder150(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
mpeg4_tool_atom_data::unknown(inner)
}
};
PResult::Ok(ret)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(mpeg4_tool_atom { size_field, r#type, size, data })
}

fn Decoder220<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(11789784461021426583u64));
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
return Err(ParseError::ExcludedBranch(2017059186547121525u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(16022216421479351095u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder221<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(2750471357152633230u64));
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
return Err(ParseError::ExcludedBranch(15337804701822118436u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(8306706226429158303u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder222<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(7258731811542513498u64));
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
return Err(ParseError::ExcludedBranch(8033982984919675631u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(1283209893442238385u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder223<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(7474037925185307628u64));
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
return Err(ParseError::ExcludedBranch(1920187793319100008u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(4475730102931494177u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder224<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(6739127967943113267u64));
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
return Err(ParseError::ExcludedBranch(17780439059155340308u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10170756778737993654u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_jpeg_eoi<>(_input: &mut Parser<'_>) -> Result<jpeg_eoi, ParseError> {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(14306133355400503306u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 216 {
b
} else {
return Err(ParseError::ExcludedBranch(8902666087419502325u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder_jpeg_frame<>(_input: &mut Parser<'_>) -> Result<jpeg_frame, ParseError> {
let initial_segment = ((|| {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
match _input.read_byte()? {
224u8 => {
0
},

225u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(10248009767256971850u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(7807103255128873628u64));
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let inner = (Decoder_jpeg_app0(_input))?;
jpeg_frame_initial_segment::app0(inner)
},

1 => {
let inner = (Decoder_jpeg_app1(_input))?;
jpeg_frame_initial_segment::app1(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(7840596951976883698u64));
}
};
PResult::Ok(ret)
})())?;
let segments = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(8004105446758774533u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(12569316739694558801u64));
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
PResult::Ok(accum)
})())?;
let header = (Decoder_jpeg_frame_header(_input))?;
let scan = (Decoder_jpeg_scan(_input))?;
let dnl = ((|| {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(2967911718584065013u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(7227788188777836434u64));
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let val = (Decoder_jpeg_dnl(_input))?;
Some(val)
},

1 => {
None
},

_ => {
return Err(ParseError::ExcludedBranch(2859130192484418172u64));
}
};
PResult::Ok(ret)
})())?;
let scans = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(12550558264664848853u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(16954835414833850385u64));
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder234(_input))?;
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
PResult::Ok(jpeg_frame { initial_segment, segments, header, scan, dnl, scans })
}

fn Decoder227<>(_input: &mut Parser<'_>) -> Result<jpeg_eoi, ParseError> {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(1891774877762105457u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 217 {
b
} else {
return Err(ParseError::ExcludedBranch(13705211812356460160u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder_jpeg_app0<>(_input: &mut Parser<'_>) -> Result<jpeg_app0, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(5515497093089591991u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 224 {
b
} else {
return Err(ParseError::ExcludedBranch(1328880024623199753u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 11669649807369914251u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_app0_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_app0 { marker, length, data })
}

fn Decoder_jpeg_app1<>(_input: &mut Parser<'_>) -> Result<jpeg_app1, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(6882184431082022206u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 225 {
b
} else {
return Err(ParseError::ExcludedBranch(3998072683184925592u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 8880661182590738257u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_app1_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_app1 { marker, length, data })
}

fn Decoder_jpeg_table_or_misc<>(_input: &mut Parser<'_>) -> Result<jpeg_table_or_misc, ParseError> {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(29850628954056690u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(7279615132236188739u64));
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
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
let inner = (Decoder270(_input))?;
jpeg_table_or_misc::app3(inner)
},

8 => {
let inner = (Decoder271(_input))?;
jpeg_table_or_misc::app4(inner)
},

9 => {
let inner = (Decoder272(_input))?;
jpeg_table_or_misc::app5(inner)
},

10 => {
let inner = (Decoder273(_input))?;
jpeg_table_or_misc::app6(inner)
},

11 => {
let inner = (Decoder274(_input))?;
jpeg_table_or_misc::app7(inner)
},

12 => {
let inner = (Decoder275(_input))?;
jpeg_table_or_misc::app8(inner)
},

13 => {
let inner = (Decoder276(_input))?;
jpeg_table_or_misc::app9(inner)
},

14 => {
let inner = (Decoder277(_input))?;
jpeg_table_or_misc::app10(inner)
},

15 => {
let inner = (Decoder278(_input))?;
jpeg_table_or_misc::app11(inner)
},

16 => {
let inner = (Decoder279(_input))?;
jpeg_table_or_misc::app12(inner)
},

17 => {
let inner = (Decoder280(_input))?;
jpeg_table_or_misc::app13(inner)
},

18 => {
let inner = (Decoder281(_input))?;
jpeg_table_or_misc::app14(inner)
},

19 => {
let inner = (Decoder282(_input))?;
jpeg_table_or_misc::app15(inner)
},

20 => {
let inner = (Decoder283(_input))?;
jpeg_table_or_misc::com(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(17636172564439370608u64));
}
};
PResult::Ok(ret)
}

fn Decoder_jpeg_frame_header<>(_input: &mut Parser<'_>) -> Result<jpeg_frame_header, ParseError> {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(13863787293436782080u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(16714498072262546943u64));
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let inner = (Decoder_jpeg_sof15(_input))?;
jpeg_frame_header::sof0(inner)
},

1 => {
let inner = (Decoder251(_input))?;
jpeg_frame_header::sof1(inner)
},

2 => {
let inner = (Decoder252(_input))?;
jpeg_frame_header::sof2(inner)
},

3 => {
let inner = (Decoder253(_input))?;
jpeg_frame_header::sof3(inner)
},

4 => {
let inner = (Decoder254(_input))?;
jpeg_frame_header::sof5(inner)
},

5 => {
let inner = (Decoder255(_input))?;
jpeg_frame_header::sof6(inner)
},

6 => {
let inner = (Decoder256(_input))?;
jpeg_frame_header::sof7(inner)
},

7 => {
let inner = (Decoder257(_input))?;
jpeg_frame_header::sof9(inner)
},

8 => {
let inner = (Decoder258(_input))?;
jpeg_frame_header::sof10(inner)
},

9 => {
let inner = (Decoder259(_input))?;
jpeg_frame_header::sof11(inner)
},

10 => {
let inner = (Decoder260(_input))?;
jpeg_frame_header::sof13(inner)
},

11 => {
let inner = (Decoder261(_input))?;
jpeg_frame_header::sof14(inner)
},

12 => {
let inner = (Decoder262(_input))?;
jpeg_frame_header::sof15(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(2157707350523277837u64));
}
};
PResult::Ok(ret)
}

fn Decoder_jpeg_scan<>(_input: &mut Parser<'_>) -> Result<jpeg_scan, ParseError> {
let segments = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(15134222038433106385u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(14950271805613481359u64));
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
PResult::Ok(accum)
})())?;
let sos = (Decoder_jpeg_sos(_input))?;
let data = (Decoder249(_input))?;
PResult::Ok(jpeg_scan { segments, sos, data })
}

fn Decoder_jpeg_dnl<>(_input: &mut Parser<'_>) -> Result<jpeg_dnl, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(5499566165243611472u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 220 {
b
} else {
return Err(ParseError::ExcludedBranch(11265176092564100083u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 691490157317212239u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_dnl_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_dnl { marker, length, data })
}

fn Decoder234<>(_input: &mut Parser<'_>) -> Result<jpeg_scan, ParseError> {
let segments = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 255 {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(14916894554939814670u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(10473830801714814973u64));
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
PResult::Ok(accum)
})())?;
let sos = (Decoder_jpeg_sos(_input))?;
let data = (Decoder_jpeg_scan_data(_input))?;
PResult::Ok(jpeg_scan { segments, sos, data })
}

fn Decoder_jpeg_sos<>(_input: &mut Parser<'_>) -> Result<jpeg_sos, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(5334325531610156978u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 218 {
b
} else {
return Err(ParseError::ExcludedBranch(16975008930446149745u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 10719628102612994677u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sos_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sos { marker, length, data })
}

fn Decoder_jpeg_scan_data<>(_input: &mut Parser<'_>) -> Result<jpeg_scan_data, ParseError> {
let scan_data = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if (byte != 255) => {
0
},

255u8 => {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(18313399323903636110u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15786118691017431738u64));
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
{
let ret = match _input.read_byte()? {
byte if (byte != 255) => {
0
},

255u8 => {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(760820951392925727u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14363790737598139216u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let inner = (Decoder237(_input))?;
jpeg_scan_data_scan_data::mcu(inner)
},

1 => {
let inner = (Decoder238(_input))?;
jpeg_scan_data_scan_data::rst0(inner)
},

2 => {
let inner = (Decoder239(_input))?;
jpeg_scan_data_scan_data::rst1(inner)
},

3 => {
let inner = (Decoder240(_input))?;
jpeg_scan_data_scan_data::rst2(inner)
},

4 => {
let inner = (Decoder241(_input))?;
jpeg_scan_data_scan_data::rst3(inner)
},

5 => {
let inner = (Decoder242(_input))?;
jpeg_scan_data_scan_data::rst4(inner)
},

6 => {
let inner = (Decoder243(_input))?;
jpeg_scan_data_scan_data::rst5(inner)
},

7 => {
let inner = (Decoder244(_input))?;
jpeg_scan_data_scan_data::rst6(inner)
},

8 => {
let inner = (Decoder245(_input))?;
jpeg_scan_data_scan_data::rst7(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(4600414761378562541u64));
}
};
ret
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let scan_data_stream = (try_flat_map_vec(scan_data.iter().cloned(), |x: jpeg_scan_data_scan_data| PResult::Ok(match x {
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
})))?;
PResult::Ok(jpeg_scan_data { scan_data, scan_data_stream })
}

fn Decoder237<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if (byte != 255) => {
0
},

255u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(17863486658382945784u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let b = _input.read_byte()?;
if b != 255 {
b
} else {
return Err(ParseError::ExcludedBranch(16165934354425559621u64));
}
},

1 => {
let _ = {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(16399036514137665776u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(7931358881575056193u64));
})
})())?;
(field0, field1)
};
255u8
},

_ => {
return Err(ParseError::ExcludedBranch(11515797873012483658u64));
}
};
PResult::Ok(ret)
}

fn Decoder238<>(_input: &mut Parser<'_>) -> Result<jpeg_eoi, ParseError> {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(8584109755265226714u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 208 {
b
} else {
return Err(ParseError::ExcludedBranch(8076978189295213982u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder239<>(_input: &mut Parser<'_>) -> Result<jpeg_eoi, ParseError> {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(11570281271401624317u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 209 {
b
} else {
return Err(ParseError::ExcludedBranch(14687724984806605719u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder240<>(_input: &mut Parser<'_>) -> Result<jpeg_eoi, ParseError> {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(1378805635639824117u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 210 {
b
} else {
return Err(ParseError::ExcludedBranch(8385173961957899741u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder241<>(_input: &mut Parser<'_>) -> Result<jpeg_eoi, ParseError> {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(8407356061009412694u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 211 {
b
} else {
return Err(ParseError::ExcludedBranch(6881565717664829242u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder242<>(_input: &mut Parser<'_>) -> Result<jpeg_eoi, ParseError> {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(13752480002470540422u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 212 {
b
} else {
return Err(ParseError::ExcludedBranch(17107648091243309207u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder243<>(_input: &mut Parser<'_>) -> Result<jpeg_eoi, ParseError> {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(5534609128357633386u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 213 {
b
} else {
return Err(ParseError::ExcludedBranch(14539762430836305896u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder244<>(_input: &mut Parser<'_>) -> Result<jpeg_eoi, ParseError> {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(16625761205375889740u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 214 {
b
} else {
return Err(ParseError::ExcludedBranch(2662265345698212949u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder245<>(_input: &mut Parser<'_>) -> Result<jpeg_eoi, ParseError> {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(3344648651879382526u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 215 {
b
} else {
return Err(ParseError::ExcludedBranch(8599210436172030522u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
}

fn Decoder_jpeg_sos_data<>(_input: &mut Parser<'_>) -> Result<jpeg_sos_data, ParseError> {
let num_image_components = ((|| {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
(x >= 1u8) && (x <= 4u8)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3484767027554133518u64));
};
PResult::Ok(ret)
})())?;
let image_components = ((|| {
let mut accum = Vec::new();
for _ in 0..num_image_components {
accum.push((Decoder_jpeg_sos_image_component(_input))?);
}
PResult::Ok(accum)
})())?;
let start_spectral_selection = ((|| {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
x <= 63u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(15403934492100194569u64));
};
PResult::Ok(ret)
})())?;
let end_spectral_selection = ((|| {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
x <= 63u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12041148194529633639u64));
};
PResult::Ok(ret)
})())?;
let approximation_bit_position = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(jpeg_sos_data_approximation_bit_position { high: packed_bits >> 4u8 & 15u8, low: packed_bits >> 0u8 & 15u8 })
})())?;
PResult::Ok(jpeg_sos_data { num_image_components, image_components, start_spectral_selection, end_spectral_selection, approximation_bit_position })
}

fn Decoder_jpeg_sos_image_component<>(_input: &mut Parser<'_>) -> Result<jpeg_sos_image_component, ParseError> {
let component_selector = (Decoder24(_input))?;
let entropy_coding_table_ids = ((|| {
let inner = {
let packed_bits = _input.read_byte()?;
jpeg_sos_image_component_entropy_coding_table_ids { dc_entropy_coding_table_id: packed_bits >> 4u8 & 15u8, ac_entropy_coding_table_id: packed_bits >> 0u8 & 15u8 }
};
let is_valid = {
let entropy_coding_table_ids = inner;
(entropy_coding_table_ids.dc_entropy_coding_table_id.clone() <= 3u8) && (entropy_coding_table_ids.ac_entropy_coding_table_id.clone() <= 3u8)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2288772415159374970u64));
};
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sos_image_component { component_selector, entropy_coding_table_ids })
}

fn Decoder_jpeg_dnl_data<>(_input: &mut Parser<'_>) -> Result<jpeg_dnl_data, ParseError> {
let num_lines = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x != 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17888323854924040413u64));
};
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_dnl_data { num_lines })
}

fn Decoder249<>(_input: &mut Parser<'_>) -> Result<jpeg_scan_data, ParseError> {
let scan_data = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if (byte != 255) => {
0
},

255u8 => {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(13675295148592556047u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4569970360394099475u64));
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
{
let ret = match _input.read_byte()? {
byte if (byte != 255) => {
0
},

255u8 => {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(6279087434444973374u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11074951631636946051u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let inner = (Decoder237(_input))?;
jpeg_scan_data_scan_data::mcu(inner)
},

1 => {
let inner = (Decoder238(_input))?;
jpeg_scan_data_scan_data::rst0(inner)
},

2 => {
let inner = (Decoder239(_input))?;
jpeg_scan_data_scan_data::rst1(inner)
},

3 => {
let inner = (Decoder240(_input))?;
jpeg_scan_data_scan_data::rst2(inner)
},

4 => {
let inner = (Decoder241(_input))?;
jpeg_scan_data_scan_data::rst3(inner)
},

5 => {
let inner = (Decoder242(_input))?;
jpeg_scan_data_scan_data::rst4(inner)
},

6 => {
let inner = (Decoder243(_input))?;
jpeg_scan_data_scan_data::rst5(inner)
},

7 => {
let inner = (Decoder244(_input))?;
jpeg_scan_data_scan_data::rst6(inner)
},

8 => {
let inner = (Decoder245(_input))?;
jpeg_scan_data_scan_data::rst7(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(15601622509425384091u64));
}
};
ret
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let scan_data_stream = (try_flat_map_vec(scan_data.iter().cloned(), |x: jpeg_scan_data_scan_data| PResult::Ok(match x {
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
})))?;
PResult::Ok(jpeg_scan_data { scan_data, scan_data_stream })
}

fn Decoder_jpeg_sof15<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(9110520999974091875u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 192 {
b
} else {
return Err(ParseError::ExcludedBranch(15293691521783146694u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 13685962128001446815u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder251<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(15433822888775103886u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 193 {
b
} else {
return Err(ParseError::ExcludedBranch(8403192837054512577u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 7538966935051243003u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder252<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(17073037115051226650u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 194 {
b
} else {
return Err(ParseError::ExcludedBranch(3975307768385535064u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 4867798537713738914u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder253<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10599514554463239458u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 195 {
b
} else {
return Err(ParseError::ExcludedBranch(16112061863928357291u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 11266387855511437693u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder254<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(12017601628070515145u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 197 {
b
} else {
return Err(ParseError::ExcludedBranch(1872233699568519226u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 11452033436843896773u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder255<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(10708294527730390829u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 198 {
b
} else {
return Err(ParseError::ExcludedBranch(7432469293302627017u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 8138544351856664662u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder256<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(13181260675040079306u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 199 {
b
} else {
return Err(ParseError::ExcludedBranch(9159119361499271180u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 12696272221194189133u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder257<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(7795160901559545235u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 201 {
b
} else {
return Err(ParseError::ExcludedBranch(3490919313637905107u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 3995820927126919547u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder258<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(9331389203258424019u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 202 {
b
} else {
return Err(ParseError::ExcludedBranch(16679512278832019969u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 9149418055219508197u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder259<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(15311158871930328757u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 203 {
b
} else {
return Err(ParseError::ExcludedBranch(9892894478446917378u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 8000269442706245049u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder260<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(3585635225240718191u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 205 {
b
} else {
return Err(ParseError::ExcludedBranch(5208404121666294786u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 4100106362216887809u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder261<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(237665900562449517u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 206 {
b
} else {
return Err(ParseError::ExcludedBranch(13751590285972774894u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 3198904588321530108u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder262<>(_input: &mut Parser<'_>) -> Result<jpeg_sof15, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(12552648416444111338u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 207 {
b
} else {
return Err(ParseError::ExcludedBranch(9201081899504003615u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 8674930063339641954u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof15 { marker, length, data })
}

fn Decoder_jpeg_sof_data<>(_input: &mut Parser<'_>) -> Result<jpeg_sof_data, ParseError> {
let sample_precision = ((|| {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
(x >= 2u8) && (x <= 16u8)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10776065777346510440u64));
};
PResult::Ok(ret)
})())?;
let num_lines = (Decoder23(_input))?;
let num_samples_per_line = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x != 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1821331332215525359u64));
};
PResult::Ok(ret)
})())?;
let num_image_components = ((|| {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
x != 0u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1550574349011231204u64));
};
PResult::Ok(ret)
})())?;
let image_components = ((|| {
let mut accum = Vec::new();
for _ in 0..num_image_components {
accum.push((Decoder_jpeg_sof_image_component(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(jpeg_sof_data { sample_precision, num_lines, num_samples_per_line, num_image_components, image_components })
}

fn Decoder_jpeg_sof_image_component<>(_input: &mut Parser<'_>) -> Result<jpeg_sof_image_component, ParseError> {
let id = (Decoder24(_input))?;
let sampling_factor = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(jpeg_sof_image_component_sampling_factor { horizontal: packed_bits >> 4u8 & 15u8, vertical: packed_bits >> 0u8 & 15u8 })
})())?;
let quantization_table_id = ((|| {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
x <= 3u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6867774794241173436u64));
};
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_sof_image_component { id, sampling_factor, quantization_table_id })
}

fn Decoder_jpeg_dqt<>(_input: &mut Parser<'_>) -> Result<jpeg_dqt, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(3475686103639625566u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 219 {
b
} else {
return Err(ParseError::ExcludedBranch(4130856500275801127u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 15244023661753025012u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_dqt { marker, length, data })
}

fn Decoder_jpeg_dht<>(_input: &mut Parser<'_>) -> Result<jpeg_dht, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(11582380281701370059u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 196 {
b
} else {
return Err(ParseError::ExcludedBranch(7228157205966134869u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 11029695522295027332u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_dht_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_dht { marker, length, data })
}

fn Decoder_jpeg_dac<>(_input: &mut Parser<'_>) -> Result<jpeg_dac, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(2184161105566707760u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 204 {
b
} else {
return Err(ParseError::ExcludedBranch(9924059786910440358u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 1560031033762626303u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_dac_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_dac { marker, length, data })
}

fn Decoder_jpeg_dri<>(_input: &mut Parser<'_>) -> Result<jpeg_dri, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(7274029685341305701u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 221 {
b
} else {
return Err(ParseError::ExcludedBranch(16051783775494465147u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 2452372056966650770u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_dri_data(_input))?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_dri { marker, length, data })
}

fn Decoder_jpeg_com<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(13744164271564421708u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 226 {
b
} else {
return Err(ParseError::ExcludedBranch(5892114170581446733u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 7866350329714952610u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder270<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(11821813774070801620u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 227 {
b
} else {
return Err(ParseError::ExcludedBranch(14520503729026832983u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 12954594173805448799u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder271<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(5309491469191307378u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 228 {
b
} else {
return Err(ParseError::ExcludedBranch(17983075411320920965u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 9573183374517388194u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder272<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(7023661717588102849u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 229 {
b
} else {
return Err(ParseError::ExcludedBranch(3448575031819686448u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 15327783809571612236u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder273<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(9960855096836829935u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 230 {
b
} else {
return Err(ParseError::ExcludedBranch(218475477370319322u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 9684775926499943714u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder274<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(18357658168615546095u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 231 {
b
} else {
return Err(ParseError::ExcludedBranch(10650412753233146525u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 5912167672739605892u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder275<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(15859964085544252343u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 232 {
b
} else {
return Err(ParseError::ExcludedBranch(653325817133119558u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 1998097826508262195u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder276<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(3349032559334020401u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 233 {
b
} else {
return Err(ParseError::ExcludedBranch(14115009527471272688u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 16370266426490485062u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder277<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(8350850950759220429u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 234 {
b
} else {
return Err(ParseError::ExcludedBranch(15412400192383838763u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 14923902544344582218u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder278<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(16334217566159141080u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 235 {
b
} else {
return Err(ParseError::ExcludedBranch(15032955882314050195u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 9033025935232855564u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder279<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(12522857579864693834u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 236 {
b
} else {
return Err(ParseError::ExcludedBranch(15286713778088114821u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 6569230515699692699u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder280<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(17055268834995250246u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 237 {
b
} else {
return Err(ParseError::ExcludedBranch(15014773733126201031u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 15682767706885925172u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder281<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(9895655502210650925u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 238 {
b
} else {
return Err(ParseError::ExcludedBranch(3344835778759068560u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 2946338368865429585u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder282<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(2014773054382805425u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 239 {
b
} else {
return Err(ParseError::ExcludedBranch(3011460078285478248u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 466150863659326234u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder283<>(_input: &mut Parser<'_>) -> Result<jpeg_com, ParseError> {
let marker = ((|| {
let ff = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(5117297982688264891u64));
})
})())?;
let marker = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 254 {
b
} else {
return Err(ParseError::ExcludedBranch(4614223265245060097u64));
})
})())?;
PResult::Ok(jpeg_eoi { ff, marker })
})())?;
let length = (Decoder23(_input))?;
let data = ((|| {
let sz = (try_sub!(length, 2u16, 11988854374464943326u64)) as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_com { marker, length, data })
}

fn Decoder_jpeg_dri_data<>(_input: &mut Parser<'_>) -> Result<jpeg_dri_data, ParseError> {
let restart_interval = (Decoder23(_input))?;
PResult::Ok(jpeg_dri_data { restart_interval })
}

fn Decoder_jpeg_dac_data<>(_input: &mut Parser<'_>) -> Result<jpeg_dac_data, ParseError> {
let class_table_id = ((|| {
let inner = {
let packed_bits = _input.read_byte()?;
jpeg_dac_data_class_table_id { class: packed_bits >> 4u8 & 15u8, table_id: packed_bits >> 0u8 & 15u8 }
};
let is_valid = {
let class_table_id = inner;
(class_table_id.class.clone() < 2u8) && (class_table_id.table_id.clone() < 4u8)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2858990937242709991u64));
};
PResult::Ok(ret)
})())?;
let value = (Decoder24(_input))?;
PResult::Ok(jpeg_dac_data { class_table_id, value })
}

fn Decoder_jpeg_dht_data<>(_input: &mut Parser<'_>) -> Result<jpeg_dht_data, ParseError> {
let class_table_id = ((|| {
let inner = {
let packed_bits = _input.read_byte()?;
jpeg_dac_data_class_table_id { class: packed_bits >> 4u8 & 15u8, table_id: packed_bits >> 0u8 & 15u8 }
};
let is_valid = {
let class_table_id = inner;
(class_table_id.class.clone() < 2u8) && (class_table_id.table_id.clone() < 4u8)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13162270726566423196u64));
};
PResult::Ok(ret)
})())?;
let num_codes = ((|| {
let mut accum = Vec::new();
for _ in 0..16u8 {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
let values = ((|| {
let mut accum = Vec::new();
for n in num_codes.clone() {
accum.push({
let mut accum = Vec::new();
for _ in 0..n {
accum.push((Decoder24(_input))?);
}
accum
});
}
PResult::Ok(accum)
})())?;
PResult::Ok(jpeg_dht_data { class_table_id, num_codes, values })
}

fn Decoder_jpeg_dqt_data<>(_input: &mut Parser<'_>) -> Result<jpeg_dqt_data, ParseError> {
let precision_table_id = ((|| {
let inner = {
let packed_bits = _input.read_byte()?;
jpeg_dqt_data_precision_table_id { precision: packed_bits >> 4u8 & 15u8, table_id: packed_bits >> 0u8 & 15u8 }
};
let is_valid = {
let precision_table_id = inner;
(precision_table_id.precision.clone() <= 1u8) && (precision_table_id.table_id.clone() <= 3u8)
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6766897041260485978u64));
};
PResult::Ok(ret)
})())?;
let elements = ((|| {
let ret = match precision_table_id.precision.clone() {
0u8 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..64u32 {
accum.push((Decoder24(_input))?);
}
accum
};
jpeg_dqt_data_elements::Bytes(inner)
},

1u8 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..64u32 {
accum.push((Decoder23(_input))?);
}
accum
};
jpeg_dqt_data_elements::Shorts(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_dqt_data { precision_table_id, elements })
}

fn Decoder_jpeg_app1_data<>(_input: &mut Parser<'_>) -> Result<jpeg_app1_data, ParseError> {
let identifier = (Decoder289(_input))?;
let data = ((|| {
let ret = match identifier.string.as_slice() {
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
jpeg_app1_data_data::other(inner)
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_app1_data { identifier, data })
}

fn Decoder289<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(4005260763079064488u64));
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
return Err(ParseError::ExcludedBranch(7359082011512182682u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(9895427541506148364u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_jpeg_app1_exif<>(_input: &mut Parser<'_>) -> Result<jpeg_app1_exif, ParseError> {
let padding = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(15241527188218394569u64));
})
})())?;
let exif = (Decoder_tiff_main(_input))?;
PResult::Ok(jpeg_app1_exif { padding, exif })
}

fn Decoder_jpeg_app1_xmp<>(_input: &mut Parser<'_>) -> Result<jpeg_app1_xmp, ParseError> {
let xmp = ((|| {
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
PResult::Ok(accum)
})())?;
PResult::Ok(jpeg_app1_xmp { xmp })
}

fn Decoder_jpeg_app0_data<>(_input: &mut Parser<'_>) -> Result<jpeg_app0_data, ParseError> {
let identifier = (Decoder293(_input))?;
let data = ((|| {
let ret = match identifier.string.as_slice() {
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
let next_elem = (Decoder24(_input))?;
accum.push(next_elem);
} else {
break
}
}
accum
};
jpeg_app0_data_data::other(inner)
}
};
PResult::Ok(ret)
})())?;
PResult::Ok(jpeg_app0_data { identifier, data })
}

fn Decoder293<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(663652071640520941u64));
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
return Err(ParseError::ExcludedBranch(5019692195244899787u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(16835260701216065402u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_jpeg_app0_jfif<>(_input: &mut Parser<'_>) -> Result<jpeg_app0_jfif, ParseError> {
let version_major = (Decoder24(_input))?;
let version_minor = (Decoder24(_input))?;
let density_units = ((|| {
let inner = (Decoder24(_input))?;
let is_valid = {
let x = inner;
x <= 2u8
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4248622096514297129u64));
};
PResult::Ok(ret)
})())?;
let density_x = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x != 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(18304605036866855350u64));
};
PResult::Ok(ret)
})())?;
let density_y = ((|| {
let inner = (Decoder23(_input))?;
let is_valid = {
let x = inner;
x != 0u16
};
let ret = if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(49400955721755355u64));
};
PResult::Ok(ret)
})())?;
let thumbnail_width = (Decoder24(_input))?;
let thumbnail_height = (Decoder24(_input))?;
let thumbnail_pixels = ((|| {
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
PResult::Ok(accum)
})())?;
PResult::Ok(jpeg_app0_jfif { version_major, version_minor, density_units, density_x, density_y, thumbnail_width, thumbnail_height, thumbnail_pixels })
}

fn Decoder_png_plte<>(_input: &mut Parser<'_>) -> Result<png_plte, ParseError> {
let r = (Decoder24(_input))?;
let g = (Decoder24(_input))?;
let b = (Decoder24(_input))?;
PResult::Ok(png_plte { r, g, b })
}

fn Decoder_gzip_header<>(_input: &mut Parser<'_>) -> Result<gzip_header, ParseError> {
let magic = ((|| {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 31 {
b
} else {
return Err(ParseError::ExcludedBranch(8882217996184815919u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 139 {
b
} else {
return Err(ParseError::ExcludedBranch(11885930557202460461u64));
})
})())?;
PResult::Ok((field0, field1))
})())?;
let method = (Decoder24(_input))?;
let file_flags = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(gzip_header_file_flags { fcomment: packed_bits >> 4u8 & 1u8 > 0u8, fname: packed_bits >> 3u8 & 1u8 > 0u8, fextra: packed_bits >> 2u8 & 1u8 > 0u8, fhcrc: packed_bits >> 1u8 & 1u8 > 0u8, ftext: packed_bits >> 0u8 & 1u8 > 0u8 })
})())?;
let timestamp = (Decoder127(_input))?;
let compression_flags = (Decoder24(_input))?;
let os_id = (Decoder24(_input))?;
PResult::Ok(gzip_header { magic, method, file_flags, timestamp, compression_flags, os_id })
}

fn Decoder_gzip_fextra<>(_input: &mut Parser<'_>) -> Result<gzip_fextra, ParseError> {
let xlen = (Decoder139(_input))?;
let subfields = ((|| {
let sz = xlen as usize;
_input.start_slice(sz)?;
let ret = ((|| {
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
PResult::Ok(accum)
})())?;
_input.end_slice()?;
PResult::Ok(ret)
})())?;
PResult::Ok(gzip_fextra { xlen, subfields })
}

fn Decoder298<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
Decoder303(_input)
}

fn Decoder_gzip_fcomment<>(_input: &mut Parser<'_>) -> Result<gzip_fcomment, ParseError> {
let comment = (Decoder302(_input))?;
PResult::Ok(gzip_fcomment { comment })
}

fn Decoder_gzip_fhcrc<>(_input: &mut Parser<'_>) -> Result<gzip_fhcrc, ParseError> {
let crc = (Decoder139(_input))?;
PResult::Ok(gzip_fhcrc { crc })
}

fn Decoder_gzip_footer<>(_input: &mut Parser<'_>) -> Result<gzip_footer, ParseError> {
let crc = (Decoder127(_input))?;
let length = (Decoder127(_input))?;
PResult::Ok(gzip_footer { crc, length })
}

fn Decoder302<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(15065685669539080124u64));
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
return Err(ParseError::ExcludedBranch(9841369023026740320u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(4896351207164742422u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder303<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(8497774971318424699u64));
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
return Err(ParseError::ExcludedBranch(12845528861092334564u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(13677998342346693652u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_gzip_fextra_subfield<>(_input: &mut Parser<'_>) -> Result<gzip_fextra_subfield, ParseError> {
let si1 = (Decoder150(_input))?;
let si2 = (Decoder150(_input))?;
let len = (Decoder139(_input))?;
let data = ((|| {
let mut accum = Vec::new();
for _ in 0..len {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(gzip_fextra_subfield { si1, si2, len, data })
}

fn Decoder305<>(_input: &mut Parser<'_>) -> Result<Vec<gzip_main>, ParseError> {
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
let header = (Decoder_gzip_header(_input))?;
let fextra = ((|| {
let ret = if header.file_flags.fextra.clone() {
Some((Decoder_gzip_fextra(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let fname = ((|| {
let ret = if header.file_flags.fname.clone() {
Some((Decoder306(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let fcomment = ((|| {
let ret = if header.file_flags.fcomment.clone() {
Some((Decoder307(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let fhcrc = ((|| {
let ret = if header.file_flags.fhcrc.clone() {
Some((Decoder_gzip_fhcrc(_input))?)
} else {
None
};
PResult::Ok(ret)
})())?;
let data = ((|| {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
PResult::Ok(ret)
})())?;
let footer = (Decoder_gzip_footer(_input))?;
gzip_main { header, fextra, fname, fcomment, fhcrc, data, footer }
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
}

fn Decoder306<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
Decoder309(_input)
}

fn Decoder307<>(_input: &mut Parser<'_>) -> Result<gzip_fcomment, ParseError> {
let comment = (Decoder308(_input))?;
PResult::Ok(gzip_fcomment { comment })
}

fn Decoder308<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(2668737607901180946u64));
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
return Err(ParseError::ExcludedBranch(18442161777584514946u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(12828788577937869717u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder309<>(_input: &mut Parser<'_>) -> Result<base_asciiz_string, ParseError> {
let string = ((|| {
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
return Err(ParseError::ExcludedBranch(12955185663715491460u64));
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
return Err(ParseError::ExcludedBranch(4528919599938425798u64));
}
};
accum.push(next_elem);
} else {
break
}
}
PResult::Ok(accum)
})())?;
let null = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(6221053009072016381u64));
})
})())?;
PResult::Ok(base_asciiz_string { string, null })
}

fn Decoder_gif_header<>(_input: &mut Parser<'_>) -> Result<gif_header, ParseError> {
let signature = ((|| {
let field0 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 71 {
b
} else {
return Err(ParseError::ExcludedBranch(4825757476091239776u64));
})
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(13877876706306354357u64));
})
})())?;
let field2 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(12525311251009778949u64));
})
})())?;
PResult::Ok((field0, field1, field2))
})())?;
let version = ((|| {
let mut accum = Vec::new();
for _ in 0..3u8 {
accum.push((Decoder150(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(gif_header { signature, version })
}

fn Decoder_gif_logical_screen<>(_input: &mut Parser<'_>) -> Result<gif_logical_screen, ParseError> {
let descriptor = (Decoder_gif_logical_screen_descriptor(_input))?;
let global_color_table = ((|| {
let ret = if descriptor.flags.table_flag.clone() > 0u8 {
let mut accum = Vec::new();
for _ in 0..2u16 << ((descriptor.flags.table_size.clone()) as u16) {
accum.push((Decoder325(_input))?);
}
Some(accum)
} else {
None
};
PResult::Ok(ret)
})())?;
PResult::Ok(gif_logical_screen { descriptor, global_color_table })
}

fn Decoder_gif_block<>(_input: &mut Parser<'_>) -> Result<gif_block, ParseError> {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
33u8 => {
match _input.read_byte()? {
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
return Err(ParseError::ExcludedBranch(14962551735998681614u64));
}
}
},

44u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(2007599397777734421u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let inner = (Decoder_gif_graphic_block(_input))?;
gif_block::graphic_block(inner)
},

1 => {
let inner = (Decoder_gif_special_purpose_block(_input))?;
gif_block::special_purpose_block(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(18069850277258932991u64));
}
};
PResult::Ok(ret)
}

fn Decoder_gif_trailer<>(_input: &mut Parser<'_>) -> Result<gif_trailer, ParseError> {
let separator = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 59 {
b
} else {
return Err(ParseError::ExcludedBranch(180881308211696508u64));
})
})())?;
PResult::Ok(gif_trailer { separator })
}

fn Decoder_gif_graphic_block<>(_input: &mut Parser<'_>) -> Result<gif_graphic_block, ParseError> {
let graphic_control_extension = ((|| {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
33u8 => {
match _input.read_byte()? {
249u8 => {
0
},

1u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(17753230141940491005u64));
}
}
},

44u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(6110942357212830202u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let val = (Decoder_gif_graphic_control_extension(_input))?;
Some(val)
},

1 => {
None
},

_ => {
return Err(ParseError::ExcludedBranch(6596410053543851077u64));
}
};
PResult::Ok(ret)
})())?;
let graphic_rendering_block = (Decoder_gif_graphic_rendering_block(_input))?;
PResult::Ok(gif_graphic_block { graphic_control_extension, graphic_rendering_block })
}

fn Decoder_gif_special_purpose_block<>(_input: &mut Parser<'_>) -> Result<gif_special_purpose_block, ParseError> {
let tree_index = {
_input.open_peek_context();
let b = _input.read_byte()?;
{
let ret = if b == 33 {
match _input.read_byte()? {
255u8 => {
0
},

254u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(16339654162669176472u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(1886358831178290550u64));
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let inner = (Decoder_gif_application_extension(_input))?;
gif_special_purpose_block::application_extension(inner)
},

1 => {
let inner = (Decoder_gif_comment_extension(_input))?;
gif_special_purpose_block::comment_extension(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(17004441093219507547u64));
}
};
PResult::Ok(ret)
}

fn Decoder_gif_application_extension<>(_input: &mut Parser<'_>) -> Result<gif_application_extension, ParseError> {
let separator = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(3168197157646945762u64));
})
})())?;
let label = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(360138127928998237u64));
})
})())?;
let block_size = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 11 {
b
} else {
return Err(ParseError::ExcludedBranch(9092411637461100444u64));
})
})())?;
let identifier = ((|| {
let mut accum = Vec::new();
for _ in 0..8u8 {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
let authentication_code = ((|| {
let mut accum = Vec::new();
for _ in 0..3u8 {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
let application_data = ((|| {
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
return Err(ParseError::ExcludedBranch(15117497265985508077u64));
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
PResult::Ok(accum)
})())?;
let terminator = (Decoder319(_input))?;
PResult::Ok(gif_application_extension { separator, label, block_size, identifier, authentication_code, application_data, terminator })
}

fn Decoder_gif_comment_extension<>(_input: &mut Parser<'_>) -> Result<gif_comment_extension, ParseError> {
let separator = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(11247369915737850258u64));
})
})())?;
let label = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 254 {
b
} else {
return Err(ParseError::ExcludedBranch(7546547171508918509u64));
})
})())?;
let comment_data = ((|| {
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
return Err(ParseError::ExcludedBranch(14304129837244038526u64));
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
PResult::Ok(accum)
})())?;
let terminator = (Decoder319(_input))?;
PResult::Ok(gif_comment_extension { separator, label, comment_data, terminator })
}

fn Decoder_gif_subblock<>(_input: &mut Parser<'_>) -> Result<gif_subblock, ParseError> {
let len_bytes = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b != 0 {
b
} else {
return Err(ParseError::ExcludedBranch(15347994581840044382u64));
})
})())?;
let data = ((|| {
let mut accum = Vec::new();
for _ in 0..len_bytes {
accum.push((Decoder24(_input))?);
}
PResult::Ok(accum)
})())?;
PResult::Ok(gif_subblock { len_bytes, data })
}

fn Decoder319<>(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(18237415135895269790u64));
})
}

fn Decoder_gif_graphic_control_extension<>(_input: &mut Parser<'_>) -> Result<gif_graphic_control_extension, ParseError> {
let separator = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(4706601668945989307u64));
})
})())?;
let label = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 249 {
b
} else {
return Err(ParseError::ExcludedBranch(1678201523848816632u64));
})
})())?;
let block_size = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 4 {
b
} else {
return Err(ParseError::ExcludedBranch(13246474195614162055u64));
})
})())?;
let flags = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(gif_graphic_control_extension_flags { reserved: packed_bits >> 5u8 & 7u8, disposal_method: packed_bits >> 2u8 & 7u8, user_input_flag: packed_bits >> 1u8 & 1u8, transparent_color_flag: packed_bits >> 0u8 & 1u8 })
})())?;
let delay_time = (Decoder139(_input))?;
let transparent_color_index = (Decoder24(_input))?;
let terminator = (Decoder319(_input))?;
PResult::Ok(gif_graphic_control_extension { separator, label, block_size, flags, delay_time, transparent_color_index, terminator })
}

fn Decoder_gif_graphic_rendering_block<>(_input: &mut Parser<'_>) -> Result<gif_graphic_rendering_block, ParseError> {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
44u8 => {
0
},

33u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(8414108255387456730u64));
}
};
_input.close_peek_context()?;
ret
}
};
let ret = match tree_index {
0 => {
let inner = (Decoder_gif_table_based_image(_input))?;
gif_graphic_rendering_block::table_based_image(inner)
},

1 => {
let inner = (Decoder_gif_plain_text_extension(_input))?;
gif_graphic_rendering_block::plain_text_extension(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(1432698095084823598u64));
}
};
PResult::Ok(ret)
}

fn Decoder_gif_table_based_image<>(_input: &mut Parser<'_>) -> Result<gif_table_based_image, ParseError> {
let descriptor = (Decoder_gif_image_descriptor(_input))?;
let local_color_table = ((|| {
let ret = if descriptor.flags.table_flag.clone() > 0u8 {
let mut accum = Vec::new();
for _ in 0..2u16 << ((descriptor.flags.table_size.clone()) as u16) {
accum.push((Decoder325(_input))?);
}
Some(accum)
} else {
None
};
PResult::Ok(ret)
})())?;
let data = (Decoder_gif_table_based_image_data(_input))?;
PResult::Ok(gif_table_based_image { descriptor, local_color_table, data })
}

fn Decoder_gif_plain_text_extension<>(_input: &mut Parser<'_>) -> Result<gif_plain_text_extension, ParseError> {
let separator = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(9372987348964131232u64));
})
})())?;
let label = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 1 {
b
} else {
return Err(ParseError::ExcludedBranch(8473414866110322269u64));
})
})())?;
let block_size = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 12 {
b
} else {
return Err(ParseError::ExcludedBranch(671785503440284610u64));
})
})())?;
let text_grid_left_position = (Decoder139(_input))?;
let text_grid_top_position = (Decoder139(_input))?;
let text_grid_width = (Decoder139(_input))?;
let text_grid_height = (Decoder139(_input))?;
let character_cell_width = (Decoder24(_input))?;
let character_cell_height = (Decoder24(_input))?;
let text_foreground_color_index = (Decoder24(_input))?;
let text_background_color_index = (Decoder24(_input))?;
let plain_text_data = ((|| {
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
return Err(ParseError::ExcludedBranch(2651182070283403637u64));
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
PResult::Ok(accum)
})())?;
let terminator = (Decoder319(_input))?;
PResult::Ok(gif_plain_text_extension { separator, label, block_size, text_grid_left_position, text_grid_top_position, text_grid_width, text_grid_height, character_cell_width, character_cell_height, text_foreground_color_index, text_background_color_index, plain_text_data, terminator })
}

fn Decoder_gif_image_descriptor<>(_input: &mut Parser<'_>) -> Result<gif_image_descriptor, ParseError> {
let separator = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 44 {
b
} else {
return Err(ParseError::ExcludedBranch(8284996377131534070u64));
})
})())?;
let image_left_position = (Decoder139(_input))?;
let image_top_position = (Decoder139(_input))?;
let image_width = (Decoder139(_input))?;
let image_height = (Decoder139(_input))?;
let flags = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(gif_image_descriptor_flags { table_flag: packed_bits >> 7u8 & 1u8, interlace_flag: packed_bits >> 6u8 & 1u8, sort_flag: packed_bits >> 5u8 & 1u8, reserved: packed_bits >> 3u8 & 3u8, table_size: packed_bits >> 0u8 & 7u8 })
})())?;
PResult::Ok(gif_image_descriptor { separator, image_left_position, image_top_position, image_width, image_height, flags })
}

fn Decoder325<>(_input: &mut Parser<'_>) -> Result<png_plte, ParseError> {
let r = (Decoder24(_input))?;
let g = (Decoder24(_input))?;
let b = (Decoder24(_input))?;
PResult::Ok(png_plte { r, g, b })
}

fn Decoder_gif_table_based_image_data<>(_input: &mut Parser<'_>) -> Result<gif_table_based_image_data, ParseError> {
let lzw_min_code_size = (Decoder24(_input))?;
let image_data = ((|| {
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
return Err(ParseError::ExcludedBranch(16948595091237008543u64));
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
PResult::Ok(accum)
})())?;
let terminator = (Decoder319(_input))?;
PResult::Ok(gif_table_based_image_data { lzw_min_code_size, image_data, terminator })
}

fn Decoder_gif_logical_screen_descriptor<>(_input: &mut Parser<'_>) -> Result<gif_logical_screen_descriptor, ParseError> {
let screen_width = (Decoder139(_input))?;
let screen_height = (Decoder139(_input))?;
let flags = ((|| {
let packed_bits = _input.read_byte()?;
PResult::Ok(gif_logical_screen_descriptor_flags { table_flag: packed_bits >> 7u8 & 1u8, color_resolution: packed_bits >> 4u8 & 7u8, sort_flag: packed_bits >> 3u8 & 1u8, table_size: packed_bits >> 0u8 & 7u8 })
})())?;
let bg_color_index = (Decoder24(_input))?;
let pixel_aspect_ratio = (Decoder24(_input))?;
PResult::Ok(gif_logical_screen_descriptor { screen_width, screen_height, flags, bg_color_index, pixel_aspect_ratio })
}

fn Decoder328<>(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
let tuple_var = {
let field0 = ((|| {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
90u8 => {
0
},

83u8 => {
match _input.read_byte()? {
90u8 => {
1
},

83u8 => {
match _input.read_byte()? {
90u8 => {
2
},

83u8 => {
match _input.read_byte()? {
90u8 => {
3
},

83u8 => {
match _input.read_byte()? {
90u8 => {
4
},

83u8 => {
match _input.read_byte()? {
90u8 => {
5
},

83u8 => {
match _input.read_byte()? {
90u8 => {
6
},

83u8 => {
match _input.read_byte()? {
90u8 => {
7
},

83u8 => {
match _input.read_byte()? {
90u8 => {
8
},

83u8 => {
9
},

_ => {
return Err(ParseError::ExcludedBranch(7834223795690054720u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(787921071240225899u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11557974043504662535u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14238499412440345954u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14703397633096852322u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(268478239438800266u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6301432137385173939u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17762152702085771435u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6803443871185192093u64));
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
return Err(ParseError::ExcludedBranch(108256050843388088u64));
}
};
accum.push(next_elem);
}
}
PResult::Ok(accum)
})())?;
let field1 = ((|| {
let b = _input.read_byte()?;
PResult::Ok(if b == 90 {
b
} else {
return Err(ParseError::ExcludedBranch(204361505388532862u64));
})
})())?;
(field0, field1)
};
PResult::Ok(match tuple_var {
(s, _z) => {
(s.len()) as u32
}
})
}

