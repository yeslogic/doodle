#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt::skip)]

mod codegen_tests;
pub mod api_helper;

use doodle::prelude::*;
use doodle::try_sub;

impl CommonObject for elf_header_ident {
type Args<'x> = ();

type Output<'x> = elf_header_ident;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_elf_header_ident(p)
}
}

/// expected size: 5
/// trait-ready: unique decoder function (d#212)
#[derive(Debug, Copy, Clone)]
pub struct elf_header_ident {
class: u8,
data: u8,
version: u8,
os_abi: u8,
abi_version: u8
}

impl CommonObject for elf_types_elf_addr {
type Args<'x> = (bool, u8);

type Output<'x> = elf_types_elf_addr;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let is_be = args.0;
let class = args.1;
Decoder_elf_types_elf_addr(p, is_be, class)
}
}

/// expected size: 16
/// trait-ready: unique decoder function (d#201)
#[derive(Debug, Copy, Clone)]
pub enum elf_types_elf_addr { Addr32(u32), Addr64(u64) }

impl CommonObject for elf_types_elf_off {
type Args<'x> = (bool, u8);

type Output<'x> = elf_types_elf_off;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let is_be = args.0;
let class = args.1;
Decoder_elf_types_elf_off(p, is_be, class)
}
}

/// expected size: 16
/// trait-ready: unique decoder function (d#202)
#[derive(Debug, Copy, Clone)]
pub enum elf_types_elf_off { Off32(u32), Off64(u64) }

impl CommonObject for elf_header {
type Args<'x> = ();

type Output<'x> = elf_header;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_elf_header(p)
}
}

/// expected size: 80
/// trait-ready: unique decoder function (d#193)
#[derive(Debug, Copy, Clone)]
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

impl CommonObject for elf_types_elf_full {
type Args<'x> = (bool, u8);

type Output<'x> = elf_types_elf_full;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let is_be = args.0;
let class = args.1;
Decoder_elf_types_elf_full(p, is_be, class)
}
}

/// expected size: 16
/// trait-ready: unique decoder function (d#200)
#[derive(Debug, Copy, Clone)]
pub enum elf_types_elf_full { Full32(u32), Full64(u64) }

impl CommonObject for elf_phdr {
type Args<'x> = (bool, u8);

type Output<'x> = elf_phdr;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let is_be = args.0;
let class = args.1;
Decoder_elf_phdr(p, is_be, class)
}
}

/// expected size: 120
/// trait-ready: unique decoder function (d#209)
#[derive(Debug, Copy, Clone)]
pub struct elf_phdr {
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

impl CommonObject for elf_shdr {
type Args<'x> = (bool, u8);

type Output<'x> = elf_shdr;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let is_be = args.0;
let class = args.1;
Decoder_elf_shdr(p, is_be, class)
}
}

/// expected size: 112
/// trait-ready: unique decoder function (d#197)
#[derive(Debug, Copy, Clone)]
pub struct elf_shdr {
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

impl CommonObject for elf_main {
type Args<'x> = ();

type Output<'x> = elf_main;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_elf_main(p)
}
}

/// expected size: 152
/// trait-ready: unique decoder function (d#13)
#[derive(Debug, Clone)]
pub struct elf_main {
header: elf_header,
program_headers: Option<Vec<elf_phdr>>,
section_headers: Option<Vec<elf_shdr>>,
sections: Option<Vec<Option<Vec<u8>>>>
}

impl CommonObject for gif_header {
type Args<'x> = ();

type Output<'x> = gif_header;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_header(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#429)
#[derive(Debug, Clone)]
pub struct gif_header {
signature: (u8, u8, u8),
version: Vec<u8>
}

impl CommonObject for gif_logical_screen_descriptor_flags {
type Args<'x> = ();

type Output<'x> = gif_logical_screen_descriptor_flags;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_logical_screen_descriptor_flags(p)
}
}

/// expected size: 4
/// trait-ready: unique decoder function (d#449)
#[derive(Debug, Copy, Clone)]
pub struct gif_logical_screen_descriptor_flags {
table_flag: bool,
color_resolution: u8,
sort_flag: bool,
table_size: u8
}

impl CommonObject for gif_logical_screen_descriptor {
type Args<'x> = ();

type Output<'x> = gif_logical_screen_descriptor;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_logical_screen_descriptor(p)
}
}

/// expected size: 10
/// trait-ready: unique decoder function (d#448)
#[derive(Debug, Copy, Clone)]
pub struct gif_logical_screen_descriptor {
screen_width: u16,
screen_height: u16,
flags: gif_logical_screen_descriptor_flags,
bg_color_index: u8,
pixel_aspect_ratio: u8
}

/// expected size: 3
/// trait-unready: multiple (2) decoders exist (d#{409, 444})
#[derive(Debug, Copy, Clone)]
pub struct gif_color_table_entry {
r: u8,
g: u8,
b: u8
}

impl CommonObject for gif_logical_screen {
type Args<'x> = ();

type Output<'x> = gif_logical_screen;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_logical_screen(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#430)
#[derive(Debug, Clone)]
pub struct gif_logical_screen {
descriptor: gif_logical_screen_descriptor,
global_color_table: Option<Vec<gif_color_table_entry>>
}

impl CommonObject for gif_graphic_control_extension_flags {
type Args<'x> = ();

type Output<'x> = gif_graphic_control_extension_flags;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_graphic_control_extension_flags(p)
}
}

/// expected size: 3
/// trait-ready: unique decoder function (d#447)
#[derive(Debug, Copy, Clone)]
pub struct gif_graphic_control_extension_flags {
disposal_method: u8,
user_input_flag: bool,
transparent_color_flag: bool
}

impl CommonObject for gif_graphic_control_extension {
type Args<'x> = ();

type Output<'x> = gif_graphic_control_extension;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_graphic_control_extension(p)
}
}

/// expected size: 10
/// trait-ready: unique decoder function (d#439)
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

impl CommonObject for gif_subblock {
type Args<'x> = ();

type Output<'x> = gif_subblock;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_subblock(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#437)
#[derive(Debug, Clone)]
pub struct gif_subblock {
len_bytes: u8,
data: Vec<u8>
}

impl CommonObject for gif_plain_text_extension {
type Args<'x> = ();

type Output<'x> = gif_plain_text_extension;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_plain_text_extension(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#442)
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

impl CommonObject for gif_image_descriptor_flags {
type Args<'x> = ();

type Output<'x> = gif_image_descriptor_flags;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_image_descriptor_flags(p)
}
}

/// expected size: 4
/// trait-ready: unique decoder function (d#446)
#[derive(Debug, Copy, Clone)]
pub struct gif_image_descriptor_flags {
table_flag: bool,
interlace_flag: bool,
sort_flag: bool,
table_size: u8
}

impl CommonObject for gif_image_descriptor {
type Args<'x> = ();

type Output<'x> = gif_image_descriptor;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_image_descriptor(p)
}
}

/// expected size: 14
/// trait-ready: unique decoder function (d#443)
#[derive(Debug, Copy, Clone)]
pub struct gif_image_descriptor {
separator: u8,
image_left_position: u16,
image_top_position: u16,
image_width: u16,
image_height: u16,
flags: gif_image_descriptor_flags
}

impl CommonObject for gif_table_based_image_data {
type Args<'x> = ();

type Output<'x> = gif_table_based_image_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_table_based_image_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#445)
#[derive(Debug, Clone)]
pub struct gif_table_based_image_data {
lzw_min_code_size: u8,
image_data: Vec<gif_subblock>,
terminator: u8
}

impl CommonObject for gif_table_based_image {
type Args<'x> = ();

type Output<'x> = gif_table_based_image;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_table_based_image(p)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#441)
#[derive(Debug, Clone)]
pub struct gif_table_based_image {
descriptor: gif_image_descriptor,
local_color_table: Option<Vec<gif_color_table_entry>>,
data: gif_table_based_image_data
}

impl CommonObject for gif_graphic_rendering_block {
type Args<'x> = ();

type Output<'x> = gif_graphic_rendering_block;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_graphic_rendering_block(p)
}
}

/// expected size: 80
/// trait-ready: unique decoder function (d#440)
#[derive(Debug, Clone)]
pub enum gif_graphic_rendering_block { plain_text_extension(gif_plain_text_extension), table_based_image(gif_table_based_image) }

impl CommonObject for gif_graphic_block {
type Args<'x> = ();

type Output<'x> = gif_graphic_block;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_graphic_block(p)
}
}

/// expected size: 96
/// trait-ready: unique decoder function (d#433)
#[derive(Debug, Clone)]
pub struct gif_graphic_block {
graphic_control_extension: Option<gif_graphic_control_extension>,
graphic_rendering_block: gif_graphic_rendering_block
}

impl CommonObject for gif_application_extension {
type Args<'x> = ();

type Output<'x> = gif_application_extension;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_application_extension(p)
}
}

/// expected size: 80
/// trait-ready: unique decoder function (d#435)
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

impl CommonObject for gif_comment_extension {
type Args<'x> = ();

type Output<'x> = gif_comment_extension;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_comment_extension(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#436)
#[derive(Debug, Clone)]
pub struct gif_comment_extension {
separator: u8,
label: u8,
comment_data: Vec<gif_subblock>,
terminator: u8
}

impl CommonObject for gif_special_purpose_block {
type Args<'x> = ();

type Output<'x> = gif_special_purpose_block;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_special_purpose_block(p)
}
}

/// expected size: 88
/// trait-ready: unique decoder function (d#434)
#[derive(Debug, Clone)]
pub enum gif_special_purpose_block { application_extension(gif_application_extension), comment_extension(gif_comment_extension) }

impl CommonObject for gif_block {
type Args<'x> = ();

type Output<'x> = gif_block;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_block(p)
}
}

/// expected size: 104
/// trait-ready: unique decoder function (d#431)
#[derive(Debug, Clone)]
pub enum gif_block { graphic_block(gif_graphic_block), special_purpose_block(gif_special_purpose_block) }

impl CommonObject for gif_trailer {
type Args<'x> = ();

type Output<'x> = gif_trailer;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_trailer(p)
}
}

/// expected size: 1
/// trait-ready: unique decoder function (d#432)
#[derive(Debug, Copy, Clone)]
pub struct gif_trailer {
separator: u8
}

impl CommonObject for gif_main {
type Args<'x> = ();

type Output<'x> = gif_main;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gif_main(p)
}
}

/// expected size: 104
/// trait-ready: unique decoder function (d#4)
#[derive(Debug, Clone)]
pub struct gif_main {
header: gif_header,
logical_screen: gif_logical_screen,
blocks: Vec<gif_block>,
trailer: gif_trailer
}

impl CommonObject for gzip_header_file_flags {
type Args<'x> = ();

type Output<'x> = gzip_header_file_flags;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gzip_header_file_flags(p)
}
}

/// expected size: 5
/// trait-ready: unique decoder function (d#424)
#[derive(Debug, Copy, Clone)]
pub struct gzip_header_file_flags {
fcomment: bool,
fname: bool,
fextra: bool,
fhcrc: bool,
ftext: bool
}

impl CommonObject for gzip_header {
type Args<'x> = ();

type Output<'x> = gzip_header;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gzip_header(p)
}
}

/// expected size: 16
/// trait-ready: unique decoder function (d#417)
#[derive(Debug, Copy, Clone)]
pub struct gzip_header {
magic: (u8, u8),
method: u8,
file_flags: gzip_header_file_flags,
timestamp: u32,
compression_flags: u8,
os_id: u8
}

impl CommonObject for gzip_fextra_subfield {
type Args<'x> = ();

type Output<'x> = gzip_fextra_subfield;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gzip_fextra_subfield(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#423)
#[derive(Debug, Clone)]
pub struct gzip_fextra_subfield {
si1: u8,
si2: u8,
len: u16,
data: Vec<u8>
}

impl CommonObject for gzip_fextra {
type Args<'x> = ();

type Output<'x> = gzip_fextra;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gzip_fextra(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#418)
#[derive(Debug, Clone)]
pub struct gzip_fextra {
xlen: u16,
subfields: Vec<gzip_fextra_subfield>
}

/// expected size: 24
/// trait-unready: multiple (2) decoders exist (d#{420, 428})
#[derive(Debug, Clone)]
pub struct gzip_fcomment {
comment: Vec<u8>
}

impl CommonObject for gzip_fhcrc {
type Args<'x> = ();

type Output<'x> = gzip_fhcrc;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gzip_fhcrc(p)
}
}

/// expected size: 2
/// trait-ready: unique decoder function (d#421)
#[derive(Debug, Copy, Clone)]
pub struct gzip_fhcrc {
crc: u16
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths {
code: u16,
extra: u8
}

/// expected size: 4
/// trait-unready: multiple (2) decoders exist (d#{247, 248})
#[derive(Debug, Copy, Clone)]
pub struct deflate_distance_record0 {
distance_extra_bits: u16,
distance: u16
}

/// expected size: 10
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct deflate_dynamic_huffman_codes_values {
length_extra_bits: u8,
length: u16,
distance_code: u16,
distance_record: deflate_distance_record0
}

/// expected size: 14
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct deflate_dynamic_huffman_codes {
code: u16,
extra: Option<deflate_dynamic_huffman_codes_values>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct deflate_fixed_huffman_codes_values_reference {
length: u16,
distance: u16
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum deflate_uncompressed_codes_values { literal(u8), reference(deflate_fixed_huffman_codes_values_reference) }

impl CommonObject for deflate_dynamic_huffman {
type Args<'x> = ();

type Output<'x> = deflate_dynamic_huffman;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_deflate_dynamic_huffman(p)
}
}

/// expected size: 176
/// trait-ready: unique decoder function (d#245)
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
codes_values: Vec<deflate_uncompressed_codes_values>
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct deflate_fixed_huffman_codes_values {
length_extra_bits: u8,
length: u16,
distance_code: u8,
distance_record: deflate_distance_record0
}

/// expected size: 12
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct deflate_fixed_huffman_codes {
code: u16,
extra: Option<deflate_fixed_huffman_codes_values>
}

impl CommonObject for deflate_fixed_huffman {
type Args<'x> = ();

type Output<'x> = deflate_fixed_huffman;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_deflate_fixed_huffman(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#244)
#[derive(Debug, Clone)]
pub struct deflate_fixed_huffman {
codes: Vec<deflate_fixed_huffman_codes>,
codes_values: Vec<deflate_uncompressed_codes_values>
}

impl CommonObject for deflate_uncompressed {
type Args<'x> = ();

type Output<'x> = deflate_uncompressed;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_deflate_uncompressed(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#243)
#[derive(Debug, Clone)]
pub struct deflate_uncompressed {
len: u16,
nlen: u16,
bytes: Vec<u8>,
codes_values: Vec<deflate_uncompressed_codes_values>
}

/// expected size: 184
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [DirectHeap, Noop, Noop] }, Layout { size: 56, align: 8 (1 << 3) })
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum deflate_block_data { dynamic_huffman(deflate_dynamic_huffman), fixed_huffman(deflate_fixed_huffman), uncompressed(deflate_uncompressed) }

impl CommonObject for deflate_block {
type Args<'x> = ();

type Output<'x> = deflate_block;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_deflate_block(p)
}
}

/// expected size: 192
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, Noop, InDef(InEnum { variants: [DirectHeap, Noop, Noop] })] }, Layout { size: 64, align: 8 (1 << 3) })
/// trait-ready: unique decoder function (d#242)
#[derive(Debug, Clone)]
pub struct deflate_block {
r#final: u8,
r#type: u8,
data: deflate_block_data
}

impl CommonObject for deflate_main {
type Args<'x> = ();

type Output<'x> = deflate_main;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_deflate_main(p)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#241)
#[derive(Debug, Clone)]
pub struct deflate_main {
blocks: Vec<deflate_block>,
codes: Vec<deflate_uncompressed_codes_values>,
inflate: Vec<u8>
}

impl CommonObject for gzip_footer {
type Args<'x> = ();

type Output<'x> = gzip_footer;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_gzip_footer(p)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#422)
#[derive(Debug, Copy, Clone)]
pub struct gzip_footer {
crc: u32,
length: u32
}

/// expected size: 184
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct gzip_main {
header: gzip_header,
fextra: Option<gzip_fextra>,
fname: Option<Vec<u8>>,
fcomment: Option<gzip_fcomment>,
fhcrc: Option<gzip_fhcrc>,
data: deflate_main,
footer: gzip_footer
}

/// expected size: 1
/// trait-unready: multiple (10) decoders exist (d#{339, 341, 352, 353, 354, 355, 356, 357, 358, 359})
#[derive(Debug, Copy, Clone)]
pub struct jpeg_soi {
marker: u8
}

impl CommonObject for jpeg_app0_jfif {
type Args<'x> = ();

type Output<'x> = jpeg_app0_jfif;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_app0_jfif(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#408)
#[derive(Debug, Clone)]
pub struct jpeg_app0_jfif {
version_major: u8,
version_minor: u8,
density_units: u8,
density_x: u16,
density_y: u16,
thumbnail_width: u8,
thumbnail_height: u8,
thumbnail_pixels: Vec<Vec<gif_color_table_entry>>
}

impl CommonObject for jpeg_app0_data_data {
type Args<'x> = &'x [u8];

type Output<'x> = jpeg_app0_data_data;

fn parse<'input>(p: &mut Parser<'input>, identifier: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_app0_data_data(p, identifier)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#407)
#[derive(Debug, Clone)]
pub enum jpeg_app0_data_data { jfif(jpeg_app0_jfif), other(Vec<u8>) }

impl CommonObject for jpeg_app0_data {
type Args<'x> = ();

type Output<'x> = jpeg_app0_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_app0_data(p)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#406)
#[derive(Debug, Clone)]
pub struct jpeg_app0_data {
identifier: Vec<u8>,
data: jpeg_app0_data_data
}

impl CommonObject for jpeg_app0 {
type Args<'x> = ();

type Output<'x> = jpeg_app0;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_app0(p)
}
}

/// expected size: 80
/// trait-ready: unique decoder function (d#385)
#[derive(Debug, Clone)]
pub struct jpeg_app0 {
marker: jpeg_soi,
length: u16,
data: jpeg_app0_data
}

impl CommonObject for tiff_byte_order {
type Args<'x> = ();

type Output<'x> = tiff_byte_order;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_tiff_byte_order(p)
}
}

/// expected size: 3
/// trait-ready: unique decoder function (d#228)
#[derive(Debug, Copy, Clone)]
pub enum tiff_byte_order { be(u8, u8), le(u8, u8) }

/// expected size: 12
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct tiff_ifd_le_fields {
tag: u16,
r#type: u16,
length: u32,
offset_or_data: u32
}

/// expected size: 56
/// trait-unready: multiple (3) decoders exist (d#{229, 230, 231})
#[derive(Debug, Clone)]
pub struct tiff_ifd_le {
num_fields: u16,
fields: Vec<tiff_ifd_le_fields>,
next_ifd_offset: u32,
next_ifd: Vec<u8>
}

impl CommonObject for tiff_main {
type Args<'x> = ();

type Output<'x> = tiff_main;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_tiff_main(p)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#11)
#[derive(Debug, Clone)]
pub struct tiff_main {
start_of_header: u32,
byte_order: tiff_byte_order,
magic: u16,
offset: u32,
ifd: tiff_ifd_le
}

impl CommonObject for jpeg_app1_exif {
type Args<'x> = ();

type Output<'x> = jpeg_app1_exif;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_app1_exif(p)
}
}

/// expected size: 80
/// trait-ready: unique decoder function (d#404)
#[derive(Debug, Clone)]
pub struct jpeg_app1_exif {
padding: u8,
exif: tiff_main
}

impl CommonObject for jpeg_app1_xmp {
type Args<'x> = ();

type Output<'x> = jpeg_app1_xmp;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_app1_xmp(p)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#405)
#[derive(Debug, Clone)]
pub struct jpeg_app1_xmp {
xmp: Vec<u8>
}

impl CommonObject for jpeg_app1_data_data {
type Args<'x> = &'x [u8];

type Output<'x> = jpeg_app1_data_data;

fn parse<'input>(p: &mut Parser<'input>, identifier: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_app1_data_data(p, identifier)
}
}

/// expected size: 88
/// trait-ready: unique decoder function (d#403)
#[derive(Debug, Clone)]
pub enum jpeg_app1_data_data { exif(jpeg_app1_exif), other(Vec<u8>), xmp(jpeg_app1_xmp) }

impl CommonObject for jpeg_app1_data {
type Args<'x> = ();

type Output<'x> = jpeg_app1_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_app1_data(p)
}
}

/// expected size: 112
/// trait-ready: unique decoder function (d#402)
#[derive(Debug, Clone)]
pub struct jpeg_app1_data {
identifier: Vec<u8>,
data: jpeg_app1_data_data
}

impl CommonObject for jpeg_app1 {
type Args<'x> = ();

type Output<'x> = jpeg_app1;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_app1(p)
}
}

/// expected size: 120
/// trait-ready: unique decoder function (d#386)
#[derive(Debug, Clone)]
pub struct jpeg_app1 {
marker: jpeg_soi,
length: u16,
data: jpeg_app1_data
}

impl CommonObject for jpeg_frame_initial_segment {
type Args<'x> = ();

type Output<'x> = jpeg_frame_initial_segment;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_frame_initial_segment(p)
}
}

/// expected size: 128
/// trait-ready: unique decoder function (d#342)
#[derive(Debug, Clone)]
pub enum jpeg_frame_initial_segment { app0(jpeg_app0), app1(jpeg_app1) }

/// expected size: 32
/// trait-unready: multiple (16) decoders exist (d#{387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 457})
#[derive(Debug, Clone)]
pub struct jpeg_app2 {
marker: jpeg_soi,
length: u16,
data: Vec<u8>
}

impl CommonObject for jpeg_class_table_id {
type Args<'x> = ();

type Output<'x> = jpeg_class_table_id;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_class_table_id(p)
}
}

/// expected size: 2
/// trait-ready: unique decoder function (d#412)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_class_table_id {
class: u8,
table_id: u8
}

impl CommonObject for jpeg_dac_data {
type Args<'x> = ();

type Output<'x> = jpeg_dac_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dac_data(p)
}
}

/// expected size: 3
/// trait-ready: unique decoder function (d#411)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dac_data {
class_table_id: jpeg_class_table_id,
value: u8
}

impl CommonObject for jpeg_dac {
type Args<'x> = ();

type Output<'x> = jpeg_dac;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dac(p)
}
}

/// expected size: 6
/// trait-ready: unique decoder function (d#383)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dac {
marker: jpeg_soi,
length: u16,
data: jpeg_dac_data
}

impl CommonObject for jpeg_dht_data {
type Args<'x> = ();

type Output<'x> = jpeg_dht_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dht_data(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#413)
#[derive(Debug, Clone)]
pub struct jpeg_dht_data {
class_table_id: jpeg_class_table_id,
num_codes: Vec<u8>,
values: Vec<Vec<u8>>
}

impl CommonObject for jpeg_dht {
type Args<'x> = ();

type Output<'x> = jpeg_dht;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dht(p)
}
}

/// expected size: 64
/// trait-ready: unique decoder function (d#382)
#[derive(Debug, Clone)]
pub struct jpeg_dht {
marker: jpeg_soi,
length: u16,
data: jpeg_dht_data
}

impl CommonObject for jpeg_precision_table_id {
type Args<'x> = ();

type Output<'x> = jpeg_precision_table_id;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_precision_table_id(p)
}
}

/// expected size: 2
/// trait-ready: unique decoder function (d#415)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_precision_table_id {
precision: u8,
table_id: u8
}

impl CommonObject for jpeg_dqt_data_elements {
type Args<'x> = u8;

type Output<'x> = jpeg_dqt_data_elements;

fn parse<'input>(p: &mut Parser<'input>, precision: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dqt_data_elements(p, precision)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#416)
#[derive(Debug, Clone)]
pub enum jpeg_dqt_data_elements { Bytes(Vec<u8>), Shorts(Vec<u16>) }

impl CommonObject for jpeg_dqt_data {
type Args<'x> = ();

type Output<'x> = jpeg_dqt_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dqt_data(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#414)
#[derive(Debug, Clone)]
pub struct jpeg_dqt_data {
precision_table_id: jpeg_precision_table_id,
elements: jpeg_dqt_data_elements
}

impl CommonObject for jpeg_dqt {
type Args<'x> = ();

type Output<'x> = jpeg_dqt;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dqt(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#381)
#[derive(Debug, Clone)]
pub struct jpeg_dqt {
marker: jpeg_soi,
length: u16,
data: Vec<jpeg_dqt_data>
}

impl CommonObject for jpeg_dri_data {
type Args<'x> = ();

type Output<'x> = jpeg_dri_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dri_data(p)
}
}

/// expected size: 2
/// trait-ready: unique decoder function (d#410)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dri_data {
restart_interval: u16
}

impl CommonObject for jpeg_dri {
type Args<'x> = ();

type Output<'x> = jpeg_dri;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dri(p)
}
}

/// expected size: 6
/// trait-ready: unique decoder function (d#384)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dri {
marker: jpeg_soi,
length: u16,
data: jpeg_dri_data
}

impl CommonObject for jpeg_table_or_misc {
type Args<'x> = ();

type Output<'x> = jpeg_table_or_misc;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_table_or_misc(p)
}
}

/// expected size: 128
/// trait-ready: unique decoder function (d#343)
#[derive(Debug, Clone)]
pub enum jpeg_table_or_misc { app0(jpeg_app0), app1(jpeg_app1), app10(jpeg_app2), app11(jpeg_app2), app12(jpeg_app2), app13(jpeg_app2), app14(jpeg_app2), app15(jpeg_app2), app2(jpeg_app2), app3(jpeg_app2), app4(jpeg_app2), app5(jpeg_app2), app6(jpeg_app2), app7(jpeg_app2), app8(jpeg_app2), app9(jpeg_app2), com(jpeg_app2), dac(jpeg_dac), dht(jpeg_dht), dqt(jpeg_dqt), dri(jpeg_dri) }

/// expected size: 2
/// trait-unready: multiple (2) decoders exist (d#{451, 453})
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dhp_image_component_sampling_factor {
horizontal: u8,
vertical: u8
}

/// expected size: 4
/// trait-unready: multiple (3) decoders exist (d#{380, 452, 455})
#[derive(Debug, Copy, Clone)]
pub struct jpeg_sof_image_component {
id: u8,
sampling_factor: jpeg_dhp_image_component_sampling_factor,
quantization_table_id: u8
}

/// expected size: 32
/// trait-unready: multiple (3) decoders exist (d#{379, 454, 459})
#[derive(Debug, Clone)]
pub struct jpeg_sof_data {
sample_precision: u8,
num_lines: u16,
num_samples_per_line: u16,
num_image_components: u8,
image_components: Vec<jpeg_sof_image_component>
}

/// expected size: 40
/// trait-unready: multiple (14) decoders exist (d#{366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 458})
#[derive(Debug, Clone)]
pub struct jpeg_sof0 {
marker: jpeg_soi,
length: u16,
data: jpeg_sof_data
}

impl CommonObject for jpeg_frame_header {
type Args<'x> = ();

type Output<'x> = jpeg_frame_header;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_frame_header(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#344)
#[derive(Debug, Clone)]
pub enum jpeg_frame_header { sof0(jpeg_sof0), sof1(jpeg_sof0), sof10(jpeg_sof0), sof11(jpeg_sof0), sof13(jpeg_sof0), sof14(jpeg_sof0), sof15(jpeg_sof0), sof2(jpeg_sof0), sof3(jpeg_sof0), sof5(jpeg_sof0), sof6(jpeg_sof0), sof7(jpeg_sof0), sof9(jpeg_sof0) }

impl CommonObject for jpeg_sos_image_component_entropy_coding_table_ids {
type Args<'x> = ();

type Output<'x> = jpeg_sos_image_component_entropy_coding_table_ids;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_sos_image_component_entropy_coding_table_ids(p)
}
}

/// expected size: 2
/// trait-ready: unique decoder function (d#363)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_sos_image_component_entropy_coding_table_ids {
dc_entropy_coding_table_id: u8,
ac_entropy_coding_table_id: u8
}

impl CommonObject for jpeg_sos_image_component {
type Args<'x> = ();

type Output<'x> = jpeg_sos_image_component;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_sos_image_component(p)
}
}

/// expected size: 3
/// trait-ready: unique decoder function (d#361)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_sos_image_component {
component_selector: u8,
entropy_coding_table_ids: jpeg_sos_image_component_entropy_coding_table_ids
}

impl CommonObject for jpeg_sos_data_approximation_bit_position {
type Args<'x> = ();

type Output<'x> = jpeg_sos_data_approximation_bit_position;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_sos_data_approximation_bit_position(p)
}
}

/// expected size: 2
/// trait-ready: unique decoder function (d#362)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_sos_data_approximation_bit_position {
high: u8,
low: u8
}

impl CommonObject for jpeg_sos_data {
type Args<'x> = ();

type Output<'x> = jpeg_sos_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_sos_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#360)
#[derive(Debug, Clone)]
pub struct jpeg_sos_data {
num_image_components: u8,
image_components: Vec<jpeg_sos_image_component>,
start_spectral_selection: u8,
end_spectral_selection: u8,
approximation_bit_position: jpeg_sos_data_approximation_bit_position
}

impl CommonObject for jpeg_sos {
type Args<'x> = ();

type Output<'x> = jpeg_sos;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_sos(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#348)
#[derive(Debug, Clone)]
pub struct jpeg_sos {
marker: jpeg_soi,
length: u16,
data: jpeg_sos_data
}

impl CommonObject for jpeg_scan_data_entropy_coded_segment {
type Args<'x> = ();

type Output<'x> = jpeg_scan_data_entropy_coded_segment;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_scan_data_entropy_coded_segment(p)
}
}

/// expected size: 2
/// trait-ready: unique decoder function (d#350)
#[derive(Debug, Copy, Clone)]
pub enum jpeg_scan_data_entropy_coded_segment { mcu(u8), rst0(jpeg_soi), rst1(jpeg_soi), rst2(jpeg_soi), rst3(jpeg_soi), rst4(jpeg_soi), rst5(jpeg_soi), rst6(jpeg_soi), rst7(jpeg_soi) }

/// expected size: 48
/// trait-unready: multiple (2) decoders exist (d#{349, 365})
#[derive(Debug, Clone)]
pub struct jpeg_scan_data {
scan_data: Vec<jpeg_scan_data_entropy_coded_segment>,
scan_data_stream: Vec<u8>
}

/// expected size: 112
/// trait-unready: multiple (2) decoders exist (d#{345, 347})
#[derive(Debug, Clone)]
pub struct jpeg_scan {
segments: Vec<jpeg_table_or_misc>,
sos: jpeg_sos,
data: jpeg_scan_data
}

impl CommonObject for jpeg_dnl_data {
type Args<'x> = ();

type Output<'x> = jpeg_dnl_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dnl_data(p)
}
}

/// expected size: 2
/// trait-ready: unique decoder function (d#364)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dnl_data {
num_lines: u16
}

impl CommonObject for jpeg_dnl {
type Args<'x> = ();

type Output<'x> = jpeg_dnl;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_dnl(p)
}
}

/// expected size: 6
/// trait-ready: unique decoder function (d#346)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_dnl {
marker: jpeg_soi,
length: u16,
data: jpeg_dnl_data
}

impl CommonObject for jpeg_frame {
type Args<'x> = ();

type Output<'x> = jpeg_frame;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_frame(p)
}
}

/// expected size: 344
/// trait-ready: unique decoder function (d#340)
#[derive(Debug, Clone)]
pub struct jpeg_frame {
initial_segment: jpeg_frame_initial_segment,
segments: Vec<jpeg_table_or_misc>,
header: jpeg_frame_header,
scan: jpeg_scan,
dnl: Option<jpeg_dnl>,
scans: Vec<jpeg_scan>
}

impl CommonObject for jpeg_main {
type Args<'x> = ();

type Output<'x> = jpeg_main;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_main(p)
}
}

/// expected size: 352
/// trait-ready: unique decoder function (d#7)
#[derive(Debug, Clone)]
pub struct jpeg_main {
soi: jpeg_soi,
frame: jpeg_frame,
eoi: jpeg_soi
}

impl CommonObject for mpeg4_ftyp_data {
type Args<'x> = ();

type Output<'x> = mpeg4_ftyp_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_ftyp_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#285)
#[derive(Debug, Clone)]
pub struct mpeg4_ftyp_data {
major_brand: (u8, u8, u8, u8),
minor_version: u32,
compatible_brands: Vec<(u8, u8, u8, u8)>
}

/// expected size: 40
/// trait-unready: multiple (2) decoders exist (d#{314, 316})
#[derive(Debug, Clone)]
pub struct mpeg4_dref_data_data {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: Vec<u8>
}

impl CommonObject for mpeg4_dref_data {
type Args<'x> = ();

type Output<'x> = mpeg4_dref_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_dref_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#315)
#[derive(Debug, Clone)]
pub struct mpeg4_dref_data {
version: u8,
flags: (u8, u8, u8),
number_of_entries: u32,
data: Vec<mpeg4_dref_data_data>
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_dinf_atom_data { dref(mpeg4_dref_data), unknown(Vec<u8>) }

impl CommonObject for mpeg4_dinf_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_dinf_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_dinf_atom(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#300)
#[derive(Debug, Clone)]
pub struct mpeg4_dinf_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_dinf_atom_data
}

impl CommonObject for mpeg4_meta_hdlr_data {
type Args<'x> = ();

type Output<'x> = mpeg4_meta_hdlr_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_meta_hdlr_data(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#320)
#[derive(Debug, Clone)]
pub struct mpeg4_meta_hdlr_data {
version: u8,
flags: (u8, u8, u8),
predefined: u32,
handler_type: (u8, u8, u8, u8),
reserved: (u32, u32, u32),
name: Vec<u8>
}

impl CommonObject for mpeg4_infe_data_fields_version_lt2 {
type Args<'x> = ();

type Output<'x> = mpeg4_infe_data_fields_version_lt2;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_infe_data_fields_version_lt2(p)
}
}

/// expected size: 80
/// trait-ready: unique decoder function (d#334)
#[derive(Debug, Clone)]
pub struct mpeg4_infe_data_fields_version_lt2 {
item_ID: u16,
item_protection_index: u16,
item_name: Vec<u8>,
content_type: Vec<u8>,
content_encoding: Vec<u8>
}

impl CommonObject for mpeg4_infe_atom_data_extra_fields_mime {
type Args<'x> = ();

type Output<'x> = mpeg4_infe_atom_data_extra_fields_mime;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_infe_atom_data_extra_fields_mime(p)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#337)
#[derive(Debug, Clone)]
pub struct mpeg4_infe_atom_data_extra_fields_mime {
content_type: Vec<u8>
}

impl CommonObject for mpeg4_infe_atom_data_extra_fields_uri {
type Args<'x> = ();

type Output<'x> = mpeg4_infe_atom_data_extra_fields_uri;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_infe_atom_data_extra_fields_uri(p)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#338)
#[derive(Debug, Clone)]
pub struct mpeg4_infe_atom_data_extra_fields_uri {
item_uri_type: Vec<u8>
}

impl CommonObject for mpeg4_infe_atom_data_extra_fields {
type Args<'x> = (u8, u8, u8, u8);

type Output<'x> = mpeg4_infe_atom_data_extra_fields;

fn parse<'input>(p: &mut Parser<'input>, item_type: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_infe_atom_data_extra_fields(p, item_type)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#336)
#[derive(Debug, Clone)]
pub enum mpeg4_infe_atom_data_extra_fields { mime(mpeg4_infe_atom_data_extra_fields_mime), unknown, uri(mpeg4_infe_atom_data_extra_fields_uri) }

impl CommonObject for mpeg4_infe_data_fields_version_gte2 {
type Args<'x> = u8;

type Output<'x> = mpeg4_infe_data_fields_version_gte2;

fn parse<'input>(p: &mut Parser<'input>, version: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_infe_data_fields_version_gte2(p, version)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#335)
#[derive(Debug, Clone)]
pub struct mpeg4_infe_data_fields_version_gte2 {
item_ID: u32,
item_protection_index: u16,
item_type: (u8, u8, u8, u8),
item_name: Vec<u8>,
extra_fields: mpeg4_infe_atom_data_extra_fields
}

impl CommonObject for mpeg4_infe_data_fields {
type Args<'x> = u8;

type Output<'x> = mpeg4_infe_data_fields;

fn parse<'input>(p: &mut Parser<'input>, version: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_infe_data_fields(p, version)
}
}

/// expected size: 88
/// trait-ready: unique decoder function (d#333)
#[derive(Debug, Clone)]
pub enum mpeg4_infe_data_fields { Version1(mpeg4_infe_data_fields_version_lt2), Version2(mpeg4_infe_data_fields_version_gte2) }

impl CommonObject for mpeg4_iinf_atom_data_infe {
type Args<'x> = ();

type Output<'x> = mpeg4_iinf_atom_data_infe;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_iinf_atom_data_infe(p)
}
}

/// expected size: 96
/// trait-ready: unique decoder function (d#332)
#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom_data_infe {
version: u8,
flags: (u8, u8, u8),
fields: mpeg4_infe_data_fields
}

/// expected size: 104
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_iinf_atom_data { infe(mpeg4_iinf_atom_data_infe), unknown(Vec<u8>) }

impl CommonObject for mpeg4_iinf_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_iinf_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_iinf_atom(p)
}
}

/// expected size: 120
/// trait-ready: unique decoder function (d#331)
#[derive(Debug, Clone)]
pub struct mpeg4_iinf_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_iinf_atom_data
}

impl CommonObject for mpeg4_iinf {
type Args<'x> = ();

type Output<'x> = mpeg4_iinf;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_iinf(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#322)
#[derive(Debug, Clone)]
pub struct mpeg4_iinf {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
item_info_entry: Vec<mpeg4_iinf_atom>
}

impl CommonObject for mpeg4_iloc_extent {
type Args<'x> = (u8, u8, u8);

type Output<'x> = mpeg4_iloc_extent;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let offset_size = args.0;
let length_size = args.1;
let index_size = args.2;
Decoder_mpeg4_iloc_extent(p, offset_size, length_size, index_size)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#328)
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_iloc_extent {
extent_index: u64,
extent_offset: u64,
extent_length: u64
}

/// expected size: 48
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct mpeg4_iloc_atom_data_items {
item_ID: u32,
construction_method: Option<u16>,
data_reference_index: u16,
base_offset: u64,
extent_count: u16,
extents: Vec<mpeg4_iloc_extent>
}

impl CommonObject for mpeg4_iloc_atom_data {
type Args<'x> = ();

type Output<'x> = mpeg4_iloc_atom_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_iloc_atom_data(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#324)
#[derive(Debug, Clone)]
pub struct mpeg4_iloc_atom_data {
version: u8,
flags: (u8, u8, u8),
offset_size: u8,
length_size: u8,
base_offset_size: u8,
index_size: u8,
item_count: u32,
items: Vec<mpeg4_iloc_atom_data_items>
}

impl CommonObject for mpeg4_tool_atom_data_data {
type Args<'x> = ();

type Output<'x> = mpeg4_tool_atom_data_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_tool_atom_data_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#327)
#[derive(Debug, Clone)]
pub struct mpeg4_tool_atom_data_data {
type_indicator: u32,
locale_indicator: u32,
value: Vec<u8>
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_tool_atom_data { data(mpeg4_tool_atom_data_data), unknown(Vec<u8>) }

impl CommonObject for mpeg4_tool_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_tool_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_tool_atom(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#326)
#[derive(Debug, Clone)]
pub struct mpeg4_tool_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_tool_atom_data
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_ilst_atom_data { tool(Vec<mpeg4_tool_atom>), unknown(Vec<u8>) }

impl CommonObject for mpeg4_ilst_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_ilst_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_ilst_atom(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#325)
#[derive(Debug, Clone)]
pub struct mpeg4_ilst_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_ilst_atom_data
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct mpeg4_iref_data_single_item_reference_large_data {
from_item_ID: u32,
reference_count: u16,
to_item_ID: Vec<u32>
}

impl CommonObject for mpeg4_iref_data_single_item_reference_large {
type Args<'x> = ();

type Output<'x> = mpeg4_iref_data_single_item_reference_large;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_iref_data_single_item_reference_large(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#330)
#[derive(Debug, Clone)]
pub struct mpeg4_iref_data_single_item_reference_large {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_iref_data_single_item_reference_large_data
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct mpeg4_iref_data_single_item_reference_small_data {
from_item_ID: u16,
reference_count: u16,
to_item_ID: Vec<u16>
}

impl CommonObject for mpeg4_iref_data_single_item_reference_small {
type Args<'x> = ();

type Output<'x> = mpeg4_iref_data_single_item_reference_small;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_iref_data_single_item_reference_small(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#329)
#[derive(Debug, Clone)]
pub struct mpeg4_iref_data_single_item_reference_small {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_iref_data_single_item_reference_small_data
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_iref_data_single_item_reference { large(Vec<mpeg4_iref_data_single_item_reference_large>), small(Vec<mpeg4_iref_data_single_item_reference_small>) }

impl CommonObject for mpeg4_iref_data {
type Args<'x> = ();

type Output<'x> = mpeg4_iref_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_iref_data(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#323)
#[derive(Debug, Clone)]
pub struct mpeg4_iref_data {
version: u8,
flags: (u8, u8, u8),
single_item_reference: mpeg4_iref_data_single_item_reference
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum mpeg4_pitm_atom_data_item_ID { Id16(u16), Id32(u32) }

impl CommonObject for mpeg4_pitm_atom_data {
type Args<'x> = ();

type Output<'x> = mpeg4_pitm_atom_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_pitm_atom_data(p)
}
}

/// expected size: 12
/// trait-ready: unique decoder function (d#321)
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_pitm_atom_data {
version: u8,
flags: (u8, u8, u8),
item_ID: mpeg4_pitm_atom_data_item_ID
}

impl CommonObject for mpeg4_meta_atom_data {
type Args<'x> = (u8, u8, u8, u8);

type Output<'x> = mpeg4_meta_atom_data;

fn parse<'input>(p: &mut Parser<'input>, r#type: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_meta_atom_data(p, r#type)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#319)
#[derive(Debug, Clone)]
pub enum mpeg4_meta_atom_data { dinf(Vec<mpeg4_dinf_atom>), hdlr(mpeg4_meta_hdlr_data), idat(Vec<u8>), iinf(mpeg4_iinf), iloc(mpeg4_iloc_atom_data), ilst(Vec<mpeg4_ilst_atom>), iref(mpeg4_iref_data), pitm(mpeg4_pitm_atom_data), unknown(Vec<u8>) }

impl CommonObject for mpeg4_meta_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_meta_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_meta_atom(p)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#286)
#[derive(Debug, Clone)]
pub struct mpeg4_meta_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_meta_atom_data
}

/// expected size: 16
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_mvhd_data_fields_version0 {
creation_time: u32,
modification_time: u32,
timescale: u32,
duration: u32
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_mvhd_data_fields_version1 {
creation_time: u64,
modification_time: u64,
timescale: u32,
duration: u64
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum mpeg4_mvhd_data_fields { version0(mpeg4_mvhd_data_fields_version0), version1(mpeg4_mvhd_data_fields_version1) }

impl CommonObject for mpeg4_mvhd_data {
type Args<'x> = ();

type Output<'x> = mpeg4_mvhd_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_mvhd_data(p)
}
}

/// expected size: 112
/// trait-ready: unique decoder function (d#288)
#[derive(Debug, Clone)]
pub struct mpeg4_mvhd_data {
version: u8,
flags: (u8, u8, u8),
fields: mpeg4_mvhd_data_fields,
rate: u32,
volume: u16,
reserved1: u16,
reserved2: (u32, u32),
matrix: Vec<u32>,
pre_defined: Vec<u32>,
next_track_ID: u32
}

impl CommonObject for mpeg4_elst_data_entry {
type Args<'x> = ();

type Output<'x> = mpeg4_elst_data_entry;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_elst_data_entry(p)
}
}

/// expected size: 12
/// trait-ready: unique decoder function (d#318)
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_elst_data_entry {
track_duration: u32,
media_time: u32,
media_rate: u32
}

impl CommonObject for mpeg4_elst_data {
type Args<'x> = ();

type Output<'x> = mpeg4_elst_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_elst_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#317)
#[derive(Debug, Clone)]
pub struct mpeg4_elst_data {
version: u8,
flags: (u8, u8, u8),
number_of_entries: u32,
edit_list_table: Vec<mpeg4_elst_data_entry>
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_edts_atom_data { elst(mpeg4_elst_data), unknown(Vec<u8>) }

impl CommonObject for mpeg4_edts_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_edts_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_edts_atom(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#292)
#[derive(Debug, Clone)]
pub struct mpeg4_edts_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_edts_atom_data
}

impl CommonObject for mpeg4_mdia_hdlr_data {
type Args<'x> = ();

type Output<'x> = mpeg4_mdia_hdlr_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_mdia_hdlr_data(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#295)
#[derive(Debug, Clone)]
pub struct mpeg4_mdia_hdlr_data {
version: u8,
flags: (u8, u8, u8),
component_type: u32,
component_subtype: (u8, u8, u8, u8),
component_manufacturer: u32,
component_flags: u32,
component_flags_mask: u32,
component_name: Vec<u8>
}

impl CommonObject for mpeg4_mdhd_data {
type Args<'x> = ();

type Output<'x> = mpeg4_mdhd_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_mdhd_data(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#296)
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_mdhd_data {
version: u8,
flags: (u8, u8, u8),
fields: mpeg4_mvhd_data_fields,
language: u16,
pre_defined: u16
}

impl CommonObject for mpeg4_smhd_data {
type Args<'x> = ();

type Output<'x> = mpeg4_smhd_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_smhd_data(p)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#299)
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_smhd_data {
version: u8,
flags: (u8, u8, u8),
balance: u16,
reserved: u16
}

impl CommonObject for mpeg4_co64_data {
type Args<'x> = ();

type Output<'x> = mpeg4_co64_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_co64_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#309)
#[derive(Debug, Clone)]
pub struct mpeg4_co64_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_offset: Vec<u64>
}

impl CommonObject for mpeg4_ctts_sample_entry {
type Args<'x> = ();

type Output<'x> = mpeg4_ctts_sample_entry;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_ctts_sample_entry(p)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#312)
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_ctts_sample_entry {
sample_count: u32,
sample_offset: u32
}

impl CommonObject for mpeg4_ctts_data {
type Args<'x> = ();

type Output<'x> = mpeg4_ctts_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_ctts_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#304)
#[derive(Debug, Clone)]
pub struct mpeg4_ctts_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<mpeg4_ctts_sample_entry>
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_sbgp_data_sample_groups {
sample_count: u32,
group_description_index: u32
}

impl CommonObject for mpeg4_sbgp_data {
type Args<'x> = ();

type Output<'x> = mpeg4_sbgp_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_sbgp_data(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#311)
#[derive(Debug, Clone)]
pub struct mpeg4_sbgp_data {
version: u8,
flags: (u8, u8, u8),
grouping_type: u32,
grouping_type_parameter: Option<u32>,
entry_count: u32,
sample_groups: Vec<mpeg4_sbgp_data_sample_groups>
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct mpeg4_sgpd_data_sample_groups {
description_length: u32,
sample_group_entry: Vec<u8>
}

impl CommonObject for mpeg4_sgpd_data {
type Args<'x> = ();

type Output<'x> = mpeg4_sgpd_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_sgpd_data(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#310)
#[derive(Debug, Clone)]
pub struct mpeg4_sgpd_data {
version: u8,
flags: (u8, u8, u8),
grouping_type: u32,
default_length: u32,
entry_count: u32,
sample_groups: Vec<mpeg4_sgpd_data_sample_groups>
}

impl CommonObject for mpeg4_stco_data {
type Args<'x> = ();

type Output<'x> = mpeg4_stco_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_stco_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#308)
#[derive(Debug, Clone)]
pub struct mpeg4_stco_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_offset: Vec<u32>
}

/// expected size: 12
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_stsc_data_chunk_entries {
first_chunk: u32,
samples_per_chunk: u32,
sample_description_index: u32
}

impl CommonObject for mpeg4_stsc_data {
type Args<'x> = ();

type Output<'x> = mpeg4_stsc_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_stsc_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#306)
#[derive(Debug, Clone)]
pub struct mpeg4_stsc_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
chunk_entries: Vec<mpeg4_stsc_data_chunk_entries>
}

impl CommonObject for mpeg4_stsd_data {
type Args<'x> = ();

type Output<'x> = mpeg4_stsd_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_stsd_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#302)
#[derive(Debug, Clone)]
pub struct mpeg4_stsd_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<mpeg4_dref_data_data>
}

impl CommonObject for mpeg4_stss_data {
type Args<'x> = ();

type Output<'x> = mpeg4_stss_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_stss_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#305)
#[derive(Debug, Clone)]
pub struct mpeg4_stss_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_number: Vec<u32>
}

impl CommonObject for mpeg4_stsz_data {
type Args<'x> = ();

type Output<'x> = mpeg4_stsz_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_stsz_data(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#307)
#[derive(Debug, Clone)]
pub struct mpeg4_stsz_data {
version: u8,
flags: (u8, u8, u8),
sample_size: u32,
sample_count: u32,
entry_size: Option<Vec<u32>>
}

impl CommonObject for mpeg4_stts_sample_entry {
type Args<'x> = ();

type Output<'x> = mpeg4_stts_sample_entry;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_stts_sample_entry(p)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#313)
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_stts_sample_entry {
sample_count: u32,
sample_delta: u32
}

impl CommonObject for mpeg4_stts_data {
type Args<'x> = ();

type Output<'x> = mpeg4_stts_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_stts_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#303)
#[derive(Debug, Clone)]
pub struct mpeg4_stts_data {
version: u8,
flags: (u8, u8, u8),
entry_count: u32,
sample_entries: Vec<mpeg4_stts_sample_entry>
}

/// expected size: 56
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_stbl_atom_data { co64(mpeg4_co64_data), ctts(mpeg4_ctts_data), sbgp(mpeg4_sbgp_data), sgpd(mpeg4_sgpd_data), stco(mpeg4_stco_data), stsc(mpeg4_stsc_data), stsd(mpeg4_stsd_data), stss(mpeg4_stss_data), stsz(mpeg4_stsz_data), stts(mpeg4_stts_data), unknown(Vec<u8>) }

impl CommonObject for mpeg4_stbl_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_stbl_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_stbl_atom(p)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#301)
#[derive(Debug, Clone)]
pub struct mpeg4_stbl_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_stbl_atom_data
}

impl CommonObject for mpeg4_vmhd_data {
type Args<'x> = ();

type Output<'x> = mpeg4_vmhd_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_vmhd_data(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#298)
#[derive(Debug, Clone)]
pub struct mpeg4_vmhd_data {
version: u8,
flags: (u8, u8, u8),
graphicsmode: u16,
opcolor: Vec<u16>
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_minf_atom_data { dinf(Vec<mpeg4_dinf_atom>), smhd(mpeg4_smhd_data), stbl(Vec<mpeg4_stbl_atom>), unknown(Vec<u8>), vmhd(mpeg4_vmhd_data) }

impl CommonObject for mpeg4_minf_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_minf_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_minf_atom(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#297)
#[derive(Debug, Clone)]
pub struct mpeg4_minf_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_minf_atom_data
}

impl CommonObject for mpeg4_mdia_atom_data {
type Args<'x> = (u8, u8, u8, u8);

type Output<'x> = mpeg4_mdia_atom_data;

fn parse<'input>(p: &mut Parser<'input>, r#type: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_mdia_atom_data(p, r#type)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#294)
#[derive(Debug, Clone)]
pub enum mpeg4_mdia_atom_data { hdlr(mpeg4_mdia_hdlr_data), mdhd(mpeg4_mdhd_data), minf(Vec<mpeg4_minf_atom>), unknown(Vec<u8>) }

impl CommonObject for mpeg4_mdia_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_mdia_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_mdia_atom(p)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#293)
#[derive(Debug, Clone)]
pub struct mpeg4_mdia_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_mdia_atom_data
}

/// expected size: 20
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_tkhd_data_fields_version0 {
creation_time: u32,
modification_time: u32,
track_ID: u32,
reserved: u32,
duration: u32
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct mpeg4_tkhd_data_fields_version1 {
creation_time: u64,
modification_time: u64,
track_ID: u32,
reserved: u32,
duration: u64
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum mpeg4_tkhd_data_fields { version0(mpeg4_tkhd_data_fields_version0), version1(mpeg4_tkhd_data_fields_version1) }

impl CommonObject for mpeg4_tkhd_data {
type Args<'x> = ();

type Output<'x> = mpeg4_tkhd_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_tkhd_data(p)
}
}

/// expected size: 96
/// trait-ready: unique decoder function (d#291)
#[derive(Debug, Clone)]
pub struct mpeg4_tkhd_data {
version: u8,
flags: (u8, u8, u8),
fields: mpeg4_tkhd_data_fields,
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
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_trak_atom_data { edts(Vec<mpeg4_edts_atom>), mdia(Vec<mpeg4_mdia_atom>), tkhd(mpeg4_tkhd_data), unknown(Vec<u8>) }

impl CommonObject for mpeg4_trak_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_trak_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_trak_atom(p)
}
}

/// expected size: 120
/// trait-ready: unique decoder function (d#289)
#[derive(Debug, Clone)]
pub struct mpeg4_trak_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_trak_atom_data
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_udta_atom_data { meta(u32, Vec<mpeg4_meta_atom>), unknown(Vec<u8>) }

impl CommonObject for mpeg4_udta_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_udta_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_udta_atom(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#290)
#[derive(Debug, Clone)]
pub struct mpeg4_udta_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_udta_atom_data
}

/// expected size: 120
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_moov_atom_data { mvhd(mpeg4_mvhd_data), trak(Vec<mpeg4_trak_atom>), udta(Vec<mpeg4_udta_atom>), unknown(Vec<u8>) }

impl CommonObject for mpeg4_moov_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_moov_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_moov_atom(p)
}
}

/// expected size: 136
/// trait-ready: unique decoder function (d#287)
#[derive(Debug, Clone)]
pub struct mpeg4_moov_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_moov_atom_data
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum mpeg4_atom_data { free, ftyp(mpeg4_ftyp_data), mdat, meta(u32, Vec<mpeg4_meta_atom>), moov(Vec<mpeg4_moov_atom>), unknown(Vec<u8>) }

impl CommonObject for mpeg4_atom {
type Args<'x> = ();

type Output<'x> = mpeg4_atom;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_atom(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#283)
#[derive(Debug, Clone)]
pub struct mpeg4_atom {
size_field: u32,
r#type: (u8, u8, u8, u8),
size: u64,
data: mpeg4_atom_data
}

impl CommonObject for mpeg4_main {
type Args<'x> = ();

type Output<'x> = mpeg4_main;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_mpeg4_main(p)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#8)
#[derive(Debug, Clone)]
pub struct mpeg4_main {
atoms: Vec<mpeg4_atom>
}

impl CommonObject for opentype_table_record {
type Args<'x> = ();

type Output<'x> = opentype_table_record;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_table_record(p)
}
}

/// expected size: 16
/// trait-ready: unique decoder function (d#27)
#[derive(Debug, Copy, Clone)]
pub struct opentype_table_record {
table_id: u32,
checksum: u32,
offset: u32,
length: u32
}

impl CommonObject for opentype_cmap_subtable_format0 {
type Args<'x> = u16;

type Output<'x> = opentype_cmap_subtable_format0;

fn parse<'input>(p: &mut Parser<'input>, _platform: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_cmap_subtable_format0(p, _platform)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#182)
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format0 {
format: u16,
length: u16,
language: u16,
glyph_id_array: Vec<u8>
}

impl CommonObject for opentype_cmap_subtable_format10 {
type Args<'x> = u16;

type Output<'x> = opentype_cmap_subtable_format10;

fn parse<'input>(p: &mut Parser<'input>, _platform: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_cmap_subtable_format10(p, _platform)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#187)
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format10 {
format: u16,
length: u32,
language: u32,
start_char_code: u32,
num_chars: u32,
glyph_id_array: Vec<u16>
}

impl CommonObject for opentype_types_sequential_map_record {
type Args<'x> = ();

type Output<'x> = opentype_types_sequential_map_record;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_types_sequential_map_record(p)
}
}

/// expected size: 12
/// trait-ready: unique decoder function (d#192)
#[derive(Debug, Copy, Clone)]
pub struct opentype_types_sequential_map_record {
start_char_code: u32,
end_char_code: u32,
start_glyph_id: u32
}

/// expected size: 40
/// trait-unready: multiple (2) decoders exist (d#{188, 189})
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format12 {
format: u16,
length: u32,
language: u32,
num_groups: u32,
groups: Vec<opentype_types_sequential_map_record>
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct main_data_opentype_directory_TTCHeader_header_Version1_table_directories_data_table_links_cmap_encoding_records_subtable__data_data_Format14_var_selector_default_uvs_offset__data_ranges {
start_unicode_value: u32,
additional_count: u8
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_variation_selector_default_uvs_offset__data {
num_unicode_value_ranges: u32,
ranges: Vec<main_data_opentype_directory_TTCHeader_header_Version1_table_directories_data_table_links_cmap_encoding_records_subtable__data_data_Format14_var_selector_default_uvs_offset__data_ranges>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_variation_selector_default_uvs_offset {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_variation_selector_default_uvs_offset__data>>
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct main_data_opentype_directory_TTCHeader_header_Version1_table_directories_data_table_links_cmap_encoding_records_subtable__data_data_Format14_var_selector_non_default_uvs_offset__data_uvs_mappings {
unicode_value: u32,
glyph_id: u16
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_variation_selector_non_default_uvs_offset__data {
num_uvs_mappings: u32,
uvs_mappings: Vec<main_data_opentype_directory_TTCHeader_header_Version1_table_directories_data_table_links_cmap_encoding_records_subtable__data_data_Format14_var_selector_non_default_uvs_offset__data_uvs_mappings>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_variation_selector_non_default_uvs_offset {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_variation_selector_non_default_uvs_offset__data>>
}

impl CommonObject for opentype_variation_selector {
type Args<'x> = View<'x>;

type Output<'x> = opentype_variation_selector;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_variation_selector(p, table_view)
}
}

/// expected size: 12
/// trait-ready: unique decoder function (d#191)
#[derive(Debug, Copy, Clone)]
pub struct opentype_variation_selector {
var_selector: u32,
default_uvs_offset: opentype_variation_selector_default_uvs_offset,
non_default_uvs_offset: opentype_variation_selector_non_default_uvs_offset
}

impl CommonObject for opentype_cmap_subtable_format14 {
type Args<'x> = View<'x>;

type Output<'x> = opentype_cmap_subtable_format14;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_cmap_subtable_format14(p, table_view)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#190)
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format14 {
format: u16,
length: u32,
num_var_selector_records: u32,
var_selector: Vec<opentype_variation_selector>
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_cmap_subtable_format2_sub_headers {
first_code: u16,
entry_count: u16,
id_delta: u16,
id_range_offset: u16
}

impl CommonObject for opentype_cmap_subtable_format2 {
type Args<'x> = u16;

type Output<'x> = opentype_cmap_subtable_format2;

fn parse<'input>(p: &mut Parser<'input>, _platform: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_cmap_subtable_format2(p, _platform)
}
}

/// expected size: 80
/// trait-ready: unique decoder function (d#183)
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format2 {
format: u16,
length: u16,
language: u16,
sub_header_keys: Vec<u16>,
sub_headers: Vec<opentype_cmap_subtable_format2_sub_headers>,
glyph_array: Vec<u16>
}

impl CommonObject for opentype_cmap_subtable_format4 {
type Args<'x> = u16;

type Output<'x> = opentype_cmap_subtable_format4;

fn parse<'input>(p: &mut Parser<'input>, _platform: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_cmap_subtable_format4(p, _platform)
}
}

/// expected size: 136
/// trait-ready: unique decoder function (d#184)
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
start_code: Vec<u16>,
id_delta: Vec<u16>,
id_range_offset: Vec<u16>,
glyph_array: Vec<u16>
}

impl CommonObject for opentype_cmap_subtable_format6 {
type Args<'x> = u16;

type Output<'x> = opentype_cmap_subtable_format6;

fn parse<'input>(p: &mut Parser<'input>, _platform: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_cmap_subtable_format6(p, _platform)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#185)
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format6 {
format: u16,
length: u16,
language: u16,
first_code: u16,
entry_count: u16,
glyph_id_array: Vec<u16>
}

impl CommonObject for opentype_cmap_subtable_format8 {
type Args<'x> = u16;

type Output<'x> = opentype_cmap_subtable_format8;

fn parse<'input>(p: &mut Parser<'input>, _platform: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_cmap_subtable_format8(p, _platform)
}
}

/// expected size: 64
/// trait-ready: unique decoder function (d#186)
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable_format8 {
format: u16,
length: u32,
language: u32,
is32: Vec<u8>,
num_groups: u32,
groups: Vec<opentype_types_sequential_map_record>
}

/// expected size: 144
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_cmap_subtable_data { Format0(opentype_cmap_subtable_format0), Format10(opentype_cmap_subtable_format10), Format12(opentype_cmap_subtable_format12), Format13(opentype_cmap_subtable_format12), Format14(opentype_cmap_subtable_format14), Format2(opentype_cmap_subtable_format2), Format4(opentype_cmap_subtable_format4), Format6(opentype_cmap_subtable_format6), Format8(opentype_cmap_subtable_format8) }

impl<'a> CommonObject for opentype_cmap_subtable<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_cmap_subtable<'x>;

fn parse<'input>(p: &mut Parser<'input>, _platform: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_cmap_subtable(p, _platform)
}
}

/// expected size: 176
/// trait-ready: unique decoder function (d#181)
#[derive(Debug, Clone)]
pub struct opentype_cmap_subtable<'input> {
table_scope: View<'input>,
format: u16,
data: opentype_cmap_subtable_data
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_encoding_record_subtable<'input> {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_cmap_subtable<'input>>>
}

impl<'a> CommonObject for opentype_encoding_record<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_encoding_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_encoding_record(p, table_view)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#180)
#[derive(Debug, Copy, Clone)]
pub struct opentype_encoding_record<'input> {
platform: u16,
encoding: u16,
subtable: opentype_encoding_record_subtable<'input>
}

impl<'a> CommonObject for opentype_cmap_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_cmap_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_cmap_table(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#29)
#[derive(Debug, Clone)]
pub struct opentype_cmap_table<'input> {
table_scope: View<'input>,
version: u16,
num_tables: u16,
encoding_records: Vec<opentype_encoding_record<'input>>
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_head_table_font_revision { Fixed32(u32) }

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_head_table_glyph_extents {
x_min: u16,
y_min: u16,
x_max: u16,
y_max: u16
}

/// expected size: 7
/// trait-orphaned: no decoder functions provided
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

impl CommonObject for opentype_head_table {
type Args<'x> = ();

type Output<'x> = opentype_head_table;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_head_table(p)
}
}

/// expected size: 64
/// trait-ready: unique decoder function (d#30)
#[derive(Debug, Copy, Clone)]
pub struct opentype_head_table {
major_version: u16,
minor_version: u16,
font_revision: opentype_head_table_font_revision,
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
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_hhea_table_caret_slope {
rise: u16,
run: u16
}

/// expected size: 28
/// trait-unready: multiple (2) decoders exist (d#{31, 48})
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
metric_data_format: u16,
number_of_long_metrics: u16
}

impl CommonObject for opentype_maxp_version1 {
type Args<'x> = ();

type Output<'x> = opentype_maxp_version1;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_maxp_version1(p)
}
}

/// expected size: 26
/// trait-ready: unique decoder function (d#178)
#[derive(Debug, Copy, Clone)]
pub struct opentype_maxp_version1 {
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
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_maxp_table_data { MaxpPostScript, MaxpUnknown(u32), MaxpV1(opentype_maxp_version1) }

impl CommonObject for opentype_maxp_table {
type Args<'x> = ();

type Output<'x> = opentype_maxp_table;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_maxp_table(p)
}
}

/// expected size: 36
/// trait-ready: unique decoder function (d#32)
#[derive(Debug, Copy, Clone)]
pub struct opentype_maxp_table {
version: u32,
num_glyphs: u16,
data: opentype_maxp_table_data
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_hmtx_table_long_metrics {
advance_width: u16,
left_side_bearing: u16
}

/// expected size: 48
/// trait-unready: multiple (2) decoders exist (d#{33, 49})
#[derive(Debug, Clone)]
pub struct opentype_hmtx_table {
long_metrics: Vec<opentype_hmtx_table_long_metrics>,
left_side_bearings: Vec<u16>
}

/// expected size: 16
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_name_name_record_string<'input> {
offset: u16,
data: &'input [u8]
}

impl<'a> CommonObject for opentype_name_name_record<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_name_name_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, storage_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_name_name_record(p, storage_view)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#175)
#[derive(Debug, Copy, Clone)]
pub struct opentype_name_name_record<'input> {
platform: u16,
encoding: u16,
language: u16,
name_id: u16,
length: u16,
string: opentype_name_name_record_string<'input>
}

impl<'a> CommonObject for opentype_name_lang_tag_record<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_name_lang_tag_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, storage_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_name_lang_tag_record(p, storage_view)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#177)
#[derive(Debug, Copy, Clone)]
pub struct opentype_name_lang_tag_record<'input> {
length: u16,
lang_tag: opentype_name_name_record_string<'input>
}

impl<'a> CommonObject for opentype_name_name_version_1<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_name_name_version_1<'x>;

fn parse<'input>(p: &mut Parser<'input>, storage_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_name_name_version_1(p, storage_view)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#176)
#[derive(Debug, Clone)]
pub struct opentype_name_name_version_1<'input> {
lang_tag_count: u16,
lang_tag_records: Vec<opentype_name_lang_tag_record<'input>>
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_name_table_data<'input> { NameVersion0, NameVersion1(opentype_name_name_version_1<'input>), NameVersionUnknown(u16) }

impl<'a> CommonObject for opentype_name_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_name_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_name_table(p)
}
}

/// expected size: 96
/// trait-ready: unique decoder function (d#34)
#[derive(Debug, Clone)]
pub struct opentype_name_table<'input> {
table_scope: View<'input>,
version: u16,
name_count: u16,
storage_offset: u16,
name_records: Vec<opentype_name_name_record<'input>>,
data: opentype_name_table_data<'input>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_os2_table_data_extra_fields_v1_extra_fields_v2_extra_fields_v5 {
us_lower_optical_point_size: u16,
us_upper_optical_point_size: u16
}

/// expected size: 16
/// trait-orphaned: no decoder functions provided
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
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_os2_table_data_extra_fields_v1 {
ul_code_page_range_1: u32,
ul_code_page_range_2: u32,
extra_fields_v2: Option<opentype_os2_table_data_extra_fields_v1_extra_fields_v2>
}

/// expected size: 36
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_os2_table_data {
s_typo_ascender: u16,
s_typo_descender: u16,
s_typo_line_gap: u16,
us_win_ascent: u16,
us_win_descent: u16,
extra_fields_v1: Option<opentype_os2_table_data_extra_fields_v1>
}

impl CommonObject for opentype_os2_table {
type Args<'x> = u32;

type Output<'x> = opentype_os2_table;

fn parse<'input>(p: &mut Parser<'input>, table_length: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_os2_table(p, table_length)
}
}

/// expected size: 120
/// trait-ready: unique decoder function (d#35)
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

impl<'a> CommonObject for opentype_post_pascal_string<'a> {
type Args<'x> = ();

type Output<'x> = opentype_post_pascal_string<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_post_pascal_string(p)
}
}

/// expected size: 16
/// trait-ready: unique decoder function (d#174)
#[derive(Debug, Copy, Clone)]
pub struct opentype_post_pascal_string<'input> {
length: u8,
string: &'input [u8]
}

impl<'a> CommonObject for opentype_post_version2<'a> {
type Args<'x> = ();

type Output<'x> = opentype_post_version2<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_post_version2(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#173)
#[derive(Debug, Clone)]
pub struct opentype_post_version2<'input> {
num_glyphs: u16,
glyph_name_index: Vec<u16>,
string_data: Vec<opentype_post_pascal_string<'input>>
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_post_table_names_Version2Dot5 {
num_glyphs: u16,
offset: Vec<u8>
}

/// expected size: 64
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_post_table_names<'input> { Version1, Version2(opentype_post_version2<'input>), Version2Dot5(opentype_post_table_names_Version2Dot5), Version3, VersionUnknown(u32) }

impl<'a> CommonObject for opentype_post_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_post_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_post_table(p)
}
}

/// expected size: 104
/// trait-ready: unique decoder function (d#36)
#[derive(Debug, Clone)]
pub struct opentype_post_table<'input> {
version: u32,
italic_angle: opentype_head_table_font_revision,
underline_position: u16,
underline_thickness: u16,
is_fixed_pitch: u32,
min_mem_type42: u32,
max_mem_type42: u32,
min_mem_type1: u32,
max_mem_type1: u32,
names: opentype_post_table_names<'input>
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_loca_table_offsets { Offsets16(Vec<u16>), Offsets32(Vec<u32>) }

impl CommonObject for opentype_loca_table {
type Args<'x> = (u16, u16);

type Output<'x> = opentype_loca_table;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let num_glyphs = args.0;
let index_to_loc_format = args.1;
Decoder_opentype_loca_table(p, num_glyphs, index_to_loc_format)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#37)
#[derive(Debug, Clone)]
pub struct opentype_loca_table {
offsets: opentype_loca_table_offsets
}

/// expected size: 12
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_composite_acc_glyphs_flags {
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
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_glyf_composite_acc_glyphs_argument1 { Int16(u16), Int8(u8), Uint16(u16), Uint8(u8) }

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_gvar_tuple_record_coordinates { F2Dot14(u16) }

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_composite_acc_glyphs_scale_XY {
x_scale: opentype_gvar_tuple_record_coordinates,
y_scale: opentype_gvar_tuple_record_coordinates
}

/// expected size: 18
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_glyf_composite_acc_glyphs_scale { Matrix((opentype_gvar_tuple_record_coordinates, opentype_gvar_tuple_record_coordinates), (opentype_gvar_tuple_record_coordinates, opentype_gvar_tuple_record_coordinates)), Scale(opentype_gvar_tuple_record_coordinates), XY(opentype_glyf_composite_acc_glyphs_scale_XY) }

/// expected size: 42
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_composite_acc_glyphs {
flags: opentype_glyf_composite_acc_glyphs_flags,
glyph_index: u16,
argument1: opentype_glyf_composite_acc_glyphs_argument1,
argument2: opentype_glyf_composite_acc_glyphs_argument1,
scale: Option<opentype_glyf_composite_acc_glyphs_scale>
}

impl CommonObject for opentype_glyf_composite {
type Args<'x> = ();

type Output<'x> = opentype_glyf_composite;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_glyf_composite(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#171)
#[derive(Debug, Clone)]
pub struct opentype_glyf_composite {
glyphs: Vec<opentype_glyf_composite_acc_glyphs>,
instructions: Vec<u8>
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_simple_flags {
on_curve_point: bool,
x_short_vector: bool,
y_short_vector: bool,
x_is_same_or_positive_x_short_vector: bool,
y_is_same_or_positive_y_short_vector: bool,
overlap_simple: bool
}

impl CommonObject for opentype_glyf_simple {
type Args<'x> = u16;

type Output<'x> = opentype_glyf_simple;

fn parse<'input>(p: &mut Parser<'input>, n_contours: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_glyf_simple(p, n_contours)
}
}

/// expected size: 128
/// trait-ready: unique decoder function (d#170)
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

impl CommonObject for opentype_glyf_description {
type Args<'x> = u16;

type Output<'x> = opentype_glyf_description;

fn parse<'input>(p: &mut Parser<'input>, n_contours: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_glyf_description(p, n_contours)
}
}

/// expected size: 136
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [Noop, Noop, DirectHeap] }, Layout { size: 48, align: 8 (1 << 3) })
/// trait-ready: unique decoder function (d#169)
#[derive(Debug, Clone)]
pub enum opentype_glyf_description { Composite(opentype_glyf_composite), HeaderOnly, Simple(opentype_glyf_simple) }

impl CommonObject for opentype_glyf_entry {
type Args<'x> = ();

type Output<'x> = opentype_glyf_entry;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_glyf_entry(p)
}
}

/// expected size: 152
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, Noop, Noop, Noop, Noop, InDef(InEnum { variants: [Noop, Noop, DirectHeap] })] }, Layout { size: 64, align: 8 (1 << 3) })
/// trait-ready: unique decoder function (d#168)
#[derive(Debug, Clone)]
pub struct opentype_glyf_entry {
number_of_contours: u16,
x_min: u16,
y_min: u16,
x_max: u16,
y_max: u16,
description: opentype_glyf_description
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_table_glyphs_Glyph {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_glyf_entry>>
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_glyf_table_glyphs { EmptyGlyph, Glyph(opentype_glyf_table_glyphs_Glyph) }

impl<'a> CommonObject for opentype_glyf_table<'a> {
type Args<'x> = opentype_loca_table_offsets;

type Output<'x> = opentype_glyf_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, offsets: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_glyf_table(p, offsets)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#38)
#[derive(Debug, Clone)]
pub struct opentype_glyf_table<'input> {
table_scope: View<'input>,
glyphs: Vec<opentype_glyf_table_glyphs>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version0 {
dogray: bool,
gridfit: bool
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version1 {
symmetric_smoothing: bool,
symmetric_gridfit: bool,
dogray: bool,
gridfit: bool
}

/// expected size: 5
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_gasp_table_gasp_ranges_range_gasp_behavior { Version0(opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version0), Version1(opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version1) }

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gasp_table_gasp_ranges {
range_max_ppem: u16,
range_gasp_behavior: opentype_gasp_table_gasp_ranges_range_gasp_behavior
}

impl CommonObject for opentype_gasp_table {
type Args<'x> = ();

type Output<'x> = opentype_gasp_table;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gasp_table(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#39)
#[derive(Debug, Clone)]
pub struct opentype_gasp_table {
version: u16,
num_ranges: u16,
gasp_ranges: Vec<opentype_gasp_table_gasp_ranges>
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_layout_axis_table_base_tag_list_offset__data {
base_tag_count: u16,
baseline_tags: Vec<u32>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_axis_table_base_tag_list_offset {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_axis_table_base_tag_list_offset__data>>
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_common_device_or_variation_index_table_DeviceTable {
start_size: u16,
end_size: u16,
delta_format: u16,
delta_values: Vec<u16>
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_device_or_variation_index_table_OtherTable {
field0: u16,
field1: u16,
delta_format: u16
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_device_or_variation_index_table_VariationIndexTable {
delta_set_outer_index: u16,
delta_set_inner_index: u16,
delta_format: (u8, u8)
}

impl CommonObject for opentype_common_device_or_variation_index_table {
type Args<'x> = ();

type Output<'x> = opentype_common_device_or_variation_index_table;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_common_device_or_variation_index_table(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#128)
#[derive(Debug, Clone)]
pub enum opentype_common_device_or_variation_index_table { DeviceTable(opentype_common_device_or_variation_index_table_DeviceTable), OtherTable(opentype_common_device_or_variation_index_table_OtherTable), VariationIndexTable(opentype_common_device_or_variation_index_table_VariationIndexTable) }

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_caret_value_data_format3_table {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_common_device_or_variation_index_table>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_base_coord_data_DeviceData {
device: opentype_gdef_caret_value_data_format3_table
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_base_coord_data_GlyphData {
reference_glyph: u16,
base_coord_point: u16
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_layout_base_coord_data { DeviceData(opentype_layout_base_coord_data_DeviceData), GlyphData(opentype_layout_base_coord_data_GlyphData), NoData }

impl<'a> CommonObject for opentype_layout_base_coord<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_base_coord<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_base_coord(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#166)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_base_coord<'input> {
table_scope: View<'input>,
format: u16,
coordinate: u16,
data: opentype_layout_base_coord_data
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_base_values_base_coord_offsets<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_base_coord<'input>>>
}

impl<'a> CommonObject for opentype_layout_base_values<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_base_values<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_base_values(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#163)
#[derive(Debug, Clone)]
pub struct opentype_layout_base_values<'input> {
table_scope: View<'input>,
default_baseline_index: u16,
base_coord_count: u16,
base_coord_offsets: Vec<opentype_layout_base_values_base_coord_offsets<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_base_script_base_values_offset<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_base_values<'input>>>
}

impl<'a> CommonObject for opentype_layout_feat_min_max<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_feat_min_max<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_feat_min_max(p, table_view)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#167)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_feat_min_max<'input> {
feature_tag: u32,
min_coord_offset: opentype_layout_base_values_base_coord_offsets<'input>,
max_coord_offset: opentype_layout_base_values_base_coord_offsets<'input>
}

impl<'a> CommonObject for opentype_layout_min_max<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_min_max<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_min_max(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#164)
#[derive(Debug, Clone)]
pub struct opentype_layout_min_max<'input> {
table_scope: View<'input>,
min_coord_offset: opentype_layout_base_values_base_coord_offsets<'input>,
max_coord_offset: opentype_layout_base_values_base_coord_offsets<'input>,
feat_min_max_count: u16,
feat_min_max_records: Vec<opentype_layout_feat_min_max<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_base_script_default_min_max_offset<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_min_max<'input>>>
}

impl<'a> CommonObject for opentype_base_base_langsys<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_base_base_langsys<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_base_base_langsys(p, table_view)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#165)
#[derive(Debug, Copy, Clone)]
pub struct opentype_base_base_langsys<'input> {
base_lang_sys_tag: u32,
min_max: opentype_layout_base_script_default_min_max_offset<'input>
}

impl<'a> CommonObject for opentype_layout_base_script<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_base_script<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_base_script(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#162)
#[derive(Debug, Clone)]
pub struct opentype_layout_base_script<'input> {
table_scope: View<'input>,
base_values_offset: opentype_layout_base_script_base_values_offset<'input>,
default_min_max_offset: opentype_layout_base_script_default_min_max_offset<'input>,
base_lang_sys_count: u16,
base_lang_sys_records: Vec<opentype_base_base_langsys<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_base_base_script_record_base_script<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_base_script<'input>>>
}

impl<'a> CommonObject for opentype_base_base_script_record<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_base_base_script_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_base_base_script_record(p, table_view)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#161)
#[derive(Debug, Copy, Clone)]
pub struct opentype_base_base_script_record<'input> {
base_script_tag: u32,
base_script: opentype_base_base_script_record_base_script<'input>
}

/// expected size: 56
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_layout_axis_table_base_script_list_offset__data<'input> {
table_scope: View<'input>,
base_script_count: u16,
base_script_records: Vec<opentype_base_base_script_record<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_axis_table_base_script_list_offset<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_axis_table_base_script_list_offset__data<'input>>>
}

impl<'a> CommonObject for opentype_layout_axis_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_axis_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_axis_table(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#160)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_axis_table<'input> {
table_scope: View<'input>,
base_tag_list_offset: opentype_layout_axis_table_base_tag_list_offset,
base_script_list_offset: opentype_layout_axis_table_base_script_list_offset<'input>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_base_table_horiz_axis_offset<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_axis_table<'input>>>
}

/// expected size: 12
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_variation_region_list_variation_regions_region_axes {
start_coord: opentype_gvar_tuple_record_coordinates,
peak_coord: opentype_gvar_tuple_record_coordinates,
end_coord: opentype_gvar_tuple_record_coordinates
}

/// expected size: 24
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_common_variation_region_list_variation_regions {
region_axes: Vec<opentype_common_variation_region_list_variation_regions_region_axes>
}

impl CommonObject for opentype_common_variation_region_list {
type Args<'x> = ();

type Output<'x> = opentype_common_variation_region_list;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_common_variation_region_list(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#154)
#[derive(Debug, Clone)]
pub struct opentype_common_variation_region_list {
axis_count: u16,
region_count: u16,
variation_regions: Vec<opentype_common_variation_region_list_variation_regions>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_item_variation_store_variation_region_list {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_common_variation_region_list>>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_item_variation_data_word_delta_count {
long_words: bool,
word_count: u16
}

/// expected size: 48
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_common_item_variation_data_delta_sets_Delta16Sets {
delta_data_full_word: Vec<u16>,
delta_data_half_word: Vec<u8>
}

/// expected size: 48
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_common_item_variation_data_delta_sets_Delta32Sets {
delta_data_full_word: Vec<u32>,
delta_data_half_word: Vec<u16>
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_common_item_variation_data_delta_sets { Delta16Sets(Vec<opentype_common_item_variation_data_delta_sets_Delta16Sets>), Delta32Sets(Vec<opentype_common_item_variation_data_delta_sets_Delta32Sets>) }

impl CommonObject for opentype_common_item_variation_data {
type Args<'x> = ();

type Output<'x> = opentype_common_item_variation_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_common_item_variation_data(p)
}
}

/// expected size: 64
/// trait-ready: unique decoder function (d#155)
#[derive(Debug, Clone)]
pub struct opentype_common_item_variation_data {
item_count: u16,
word_delta_count: opentype_common_item_variation_data_word_delta_count,
region_index_count: u16,
region_indices: Vec<u16>,
delta_sets: opentype_common_item_variation_data_delta_sets
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_item_variation_store_item_variation_data_list {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_common_item_variation_data>>
}

impl<'a> CommonObject for opentype_common_item_variation_store<'a> {
type Args<'x> = ();

type Output<'x> = opentype_common_item_variation_store<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_common_item_variation_store(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#153)
#[derive(Debug, Clone)]
pub struct opentype_common_item_variation_store<'input> {
table_scope: View<'input>,
format: u16,
variation_region_list: opentype_common_item_variation_store_variation_region_list,
item_variation_data_count: u16,
item_variation_data_list: Vec<opentype_common_item_variation_store_item_variation_data_list>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_base_table_item_var_store_offset<'input> {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_common_item_variation_store<'input>>>
}

impl<'a> CommonObject for opentype_base_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_base_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_base_table(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#40)
#[derive(Debug, Copy, Clone)]
pub struct opentype_base_table<'input> {
table_scope: View<'input>,
major_version: u16,
minor_version: u16,
horiz_axis_offset: opentype_base_table_horiz_axis_offset<'input>,
vert_axis_offset: opentype_base_table_horiz_axis_offset<'input>,
item_var_store_offset: Option<opentype_base_table_item_var_store_offset<'input>>
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_class_def_data_Format1 {
start_glyph_id: u16,
glyph_count: u16,
class_value_array: Vec<u16>
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_class_def_data_Format2_class_range_records {
start_glyph_id: u16,
end_glyph_id: u16,
class: u16
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_class_def_data_Format2 {
class_range_count: u16,
class_range_records: Vec<opentype_class_def_data_Format2_class_range_records>
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_class_def_data { Format1(opentype_class_def_data_Format1), Format2(opentype_class_def_data_Format2) }

impl CommonObject for opentype_class_def {
type Args<'x> = ();

type Output<'x> = opentype_class_def;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_class_def(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#93)
#[derive(Debug, Clone)]
pub struct opentype_class_def {
class_format: u16,
data: opentype_class_def_data
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_glyph_class_def {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_class_def>>
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_coverage_table_data_Format1 {
glyph_count: u16,
glyph_array: Vec<u16>
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_coverage_table_data_Format2_range_records {
start_glyph_id: u16,
end_glyph_id: u16,
start_coverage_index: u16
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_coverage_table_data_Format2 {
range_count: u16,
range_records: Vec<opentype_coverage_table_data_Format2_range_records>
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_coverage_table_data { Format1(opentype_coverage_table_data_Format1), Format2(opentype_coverage_table_data_Format2) }

impl CommonObject for opentype_coverage_table {
type Args<'x> = ();

type Output<'x> = opentype_coverage_table;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_coverage_table(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#88)
#[derive(Debug, Clone)]
pub struct opentype_coverage_table {
coverage_format: u16,
data: opentype_coverage_table_data
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_attach_list_coverage {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_coverage_table>>
}

impl CommonObject for opentype_gdef_attach_point {
type Args<'x> = ();

type Output<'x> = opentype_gdef_attach_point;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gdef_attach_point(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#159)
#[derive(Debug, Clone)]
pub struct opentype_gdef_attach_point {
point_count: u16,
point_indices: Vec<u16>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_attach_list_attach_point_offsets {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gdef_attach_point>>
}

impl<'a> CommonObject for opentype_gdef_attach_list<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gdef_attach_list<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gdef_attach_list(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#150)
#[derive(Debug, Clone)]
pub struct opentype_gdef_attach_list<'input> {
list_scope: View<'input>,
coverage: opentype_gdef_attach_list_coverage,
glyph_count: u16,
attach_point_offsets: Vec<opentype_gdef_attach_list_attach_point_offsets>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_attach_list<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gdef_attach_list<'input>>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_caret_value_data_Format1 {
coordinate: u16
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_caret_value_data_Format2 {
caret_value_point_index: u16
}

impl<'a> CommonObject for opentype_gdef_caret_value_data_format3<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_gdef_caret_value_data_format3<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gdef_caret_value_data_format3(p, table_view)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#158)
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_caret_value_data_format3<'input> {
table_scope: View<'input>,
coordinate: u16,
table: opentype_gdef_caret_value_data_format3_table
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_gdef_caret_value_data<'input> { Format1(opentype_gdef_caret_value_data_Format1), Format2(opentype_gdef_caret_value_data_Format2), Format3(opentype_gdef_caret_value_data_format3<'input>) }

impl<'a> CommonObject for opentype_gdef_caret_value<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gdef_caret_value<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gdef_caret_value(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#157)
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_caret_value<'input> {
format: u16,
data: opentype_gdef_caret_value_data<'input>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_lig_glyph_caret_values<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gdef_caret_value<'input>>>
}

impl<'a> CommonObject for opentype_gdef_lig_glyph<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gdef_lig_glyph<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gdef_lig_glyph(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#156)
#[derive(Debug, Clone)]
pub struct opentype_gdef_lig_glyph<'input> {
table_scope: View<'input>,
caret_count: u16,
caret_values: Vec<opentype_gdef_lig_glyph_caret_values<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_lig_caret_list_lig_glyph_offsets<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gdef_lig_glyph<'input>>>
}

impl<'a> CommonObject for opentype_gdef_lig_caret_list<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gdef_lig_caret_list<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gdef_lig_caret_list(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#151)
#[derive(Debug, Clone)]
pub struct opentype_gdef_lig_caret_list<'input> {
list_scope: View<'input>,
coverage: opentype_gdef_attach_list_coverage,
lig_glyph_count: u16,
lig_glyph_offsets: Vec<opentype_gdef_lig_caret_list_lig_glyph_offsets<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_lig_caret_list<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gdef_lig_caret_list<'input>>>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_mark_glyph_set_coverage {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_coverage_table>>
}

impl<'a> CommonObject for opentype_gdef_mark_glyph_set<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gdef_mark_glyph_set<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gdef_mark_glyph_set(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#152)
#[derive(Debug, Clone)]
pub struct opentype_gdef_mark_glyph_set<'input> {
table_scope: View<'input>,
format: u16,
mark_glyph_set_count: u16,
coverage: Vec<opentype_gdef_mark_glyph_set_coverage>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_data_Version1_2_mark_glyph_sets_def<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gdef_mark_glyph_set<'input>>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_data_Version1_2<'input> {
mark_glyph_sets_def: opentype_gdef_table_data_Version1_2_mark_glyph_sets_def<'input>
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table_data_Version1_3<'input> {
mark_glyph_sets_def: opentype_gdef_table_data_Version1_2_mark_glyph_sets_def<'input>,
item_var_store: opentype_base_table_item_var_store_offset<'input>
}

/// expected size: 12
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_gdef_table_data<'input> { Version1_0, Version1_2(opentype_gdef_table_data_Version1_2<'input>), Version1_3(opentype_gdef_table_data_Version1_3<'input>) }

impl<'a> CommonObject for opentype_gdef_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gdef_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gdef_table(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#41)
#[derive(Debug, Copy, Clone)]
pub struct opentype_gdef_table<'input> {
table_scope: View<'input>,
major_version: u16,
minor_version: u16,
glyph_class_def: opentype_gdef_table_glyph_class_def,
attach_list: opentype_gdef_table_attach_list<'input>,
lig_caret_list: opentype_gdef_table_lig_caret_list<'input>,
mark_attach_class_def: opentype_gdef_table_glyph_class_def,
data: opentype_gdef_table_data<'input>
}

impl CommonObject for opentype_layout_langsys {
type Args<'x> = ();

type Output<'x> = opentype_layout_langsys;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_langsys(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#110)
#[derive(Debug, Clone)]
pub struct opentype_layout_langsys {
lookup_order_offset: u16,
required_feature_index: u16,
feature_index_count: u16,
feature_indices: Vec<u16>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_script_table_default_lang_sys {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_langsys>>
}

impl CommonObject for opentype_layout_lang_sys_record {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_lang_sys_record;

fn parse<'input>(p: &mut Parser<'input>, script_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_lang_sys_record(p, script_view)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#111)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_lang_sys_record {
lang_sys_tag: u32,
lang_sys: opentype_layout_script_table_default_lang_sys
}

impl<'a> CommonObject for opentype_layout_script_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_script_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_script_table(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#109)
#[derive(Debug, Clone)]
pub struct opentype_layout_script_table<'input> {
script_scope: View<'input>,
default_lang_sys: opentype_layout_script_table_default_lang_sys,
lang_sys_count: u16,
lang_sys_records: Vec<opentype_layout_lang_sys_record>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_script_record_script<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_script_table<'input>>>
}

impl<'a> CommonObject for opentype_layout_script_record<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_script_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_script_record(p, table_view)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#108)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_script_record<'input> {
script_tag: u32,
script: opentype_layout_script_record_script<'input>
}

impl<'a> CommonObject for opentype_layout_script_list<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_script_list<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_script_list(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#69)
#[derive(Debug, Clone)]
pub struct opentype_layout_script_list<'input> {
table_scope: View<'input>,
script_count: u16,
script_records: Vec<opentype_layout_script_record<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gpos_table_script_list<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_script_list<'input>>>
}

impl<'a> CommonObject for opentype_layout_feature_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_feature_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_feature_table(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#76)
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_table<'input> {
table_scope: View<'input>,
feature_params: u16,
lookup_index_count: u16,
lookup_list_indices: Vec<u16>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_feature_record_feature<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_feature_table<'input>>>
}

impl<'a> CommonObject for opentype_layout_feature_record<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_feature_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, list_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_feature_record(p, list_view)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#107)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_feature_record<'input> {
feature_tag: u32,
feature: opentype_layout_feature_record_feature<'input>
}

impl<'a> CommonObject for opentype_layout_feature_list<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_feature_list<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_feature_list(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#70)
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_list<'input> {
list_scope: View<'input>,
feature_count: u16,
feature_records: Vec<opentype_layout_feature_record<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gpos_table_feature_list<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_feature_list<'input>>>
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gpos_lookup_table_lookup_flag {
mark_attachment_class_filter: u16,
use_mark_filtering_set: bool,
ignore_marks: bool,
ignore_ligatures: bool,
ignore_base_glyphs: bool,
right_to_left: bool
}

impl CommonObject for opentype_layout_sequence_lookup {
type Args<'x> = ();

type Output<'x> = opentype_layout_sequence_lookup;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_sequence_lookup(p)
}
}

/// expected size: 4
/// trait-ready: unique decoder function (d#92)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_sequence_lookup {
sequence_index: u16,
lookup_list_index: u16
}

impl CommonObject for opentype_layout_chained_sequence_rule {
type Args<'x> = ();

type Output<'x> = opentype_layout_chained_sequence_rule;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_chained_sequence_rule(p)
}
}

/// expected size: 104
/// trait-ready: unique decoder function (d#95)
#[derive(Debug, Clone)]
pub struct opentype_layout_chained_sequence_rule {
backtrack_glyph_count: u16,
backtrack_sequence: Vec<u16>,
input_glyph_count: u16,
input_sequence: Vec<u16>,
lookahead_glyph_count: u16,
lookahead_sequence: Vec<u16>,
seq_lookup_count: u16,
seq_lookup_records: Vec<opentype_layout_sequence_lookup>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_chained_sequence_rule_set_chained_seq_rules {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_chained_sequence_rule>>
}

impl<'a> CommonObject for opentype_layout_chained_sequence_rule_set<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_chained_sequence_rule_set<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_chained_sequence_rule_set(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#94)
#[derive(Debug, Clone)]
pub struct opentype_layout_chained_sequence_rule_set<'input> {
table_scope: View<'input>,
chained_seq_rule_count: u16,
chained_seq_rules: Vec<opentype_layout_chained_sequence_rule_set_chained_seq_rules>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_chained_sequence_context_format1_chained_seq_rule_sets<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_chained_sequence_rule_set<'input>>>
}

impl<'a> CommonObject for opentype_layout_chained_sequence_context_format1<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_chained_sequence_context_format1<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_chained_sequence_context_format1(p, table_view)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#89)
#[derive(Debug, Clone)]
pub struct opentype_layout_chained_sequence_context_format1<'input> {
coverage: opentype_gdef_attach_list_coverage,
chained_seq_rule_set_count: u16,
chained_seq_rule_sets: Vec<opentype_layout_chained_sequence_context_format1_chained_seq_rule_sets<'input>>
}

impl<'a> CommonObject for opentype_layout_chained_sequence_context_format2<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_chained_sequence_context_format2<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_chained_sequence_context_format2(p, table_view)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#90)
#[derive(Debug, Clone)]
pub struct opentype_layout_chained_sequence_context_format2<'input> {
coverage: opentype_gdef_attach_list_coverage,
backtrack_class_def: opentype_gdef_table_glyph_class_def,
input_class_def: opentype_gdef_table_glyph_class_def,
lookahead_class_def: opentype_gdef_table_glyph_class_def,
chained_class_seq_rule_set_count: u16,
chained_class_seq_rule_sets: Vec<opentype_layout_chained_sequence_context_format1_chained_seq_rule_sets<'input>>
}

impl CommonObject for opentype_layout_chained_sequence_context_format3 {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_chained_sequence_context_format3;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_chained_sequence_context_format3(p, table_view)
}
}

/// expected size: 104
/// trait-ready: unique decoder function (d#91)
#[derive(Debug, Clone)]
pub struct opentype_layout_chained_sequence_context_format3 {
backtrack_glyph_count: u16,
backtrack_coverages: Vec<opentype_gdef_attach_list_coverage>,
input_glyph_count: u16,
input_coverages: Vec<opentype_gdef_attach_list_coverage>,
lookahead_glyph_count: u16,
lookahead_coverages: Vec<opentype_gdef_attach_list_coverage>,
seq_lookup_count: u16,
seq_lookup_records: Vec<opentype_layout_sequence_lookup>
}

/// expected size: 112
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_layout_chained_sequence_context_subst<'input> { Format1(opentype_layout_chained_sequence_context_format1<'input>), Format2(opentype_layout_chained_sequence_context_format2<'input>), Format3(opentype_layout_chained_sequence_context_format3) }

impl<'a> CommonObject for opentype_layout_chained_sequence_context<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_chained_sequence_context<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_chained_sequence_context(p)
}
}

/// expected size: 144
/// trait-ready: unique decoder function (d#86)
#[derive(Debug, Clone)]
pub struct opentype_layout_chained_sequence_context<'input> {
table_scope: View<'input>,
format: u16,
subst: opentype_layout_chained_sequence_context_subst<'input>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_anchor_table_table_Format1 {
x_coordinate: u16,
y_coordinate: u16
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_anchor_table_table_Format2 {
x_coordinate: u16,
y_coordinate: u16,
anchor_point: u16
}

impl<'a> CommonObject for opentype_layout_anchor_table_format3<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_anchor_table_format3<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_anchor_table_format3(p, table_view)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#127)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_anchor_table_format3<'input> {
table_scope: View<'input>,
x_coordinate: u16,
y_coordinate: u16,
x_device: opentype_gdef_caret_value_data_format3_table,
y_device: opentype_gdef_caret_value_data_format3_table
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub enum opentype_layout_anchor_table_table<'input> { Format1(opentype_layout_anchor_table_table_Format1), Format2(opentype_layout_anchor_table_table_Format2), Format3(opentype_layout_anchor_table_format3<'input>) }

impl<'a> CommonObject for opentype_layout_anchor_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_anchor_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_anchor_table(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#126)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_anchor_table<'input> {
anchor_format: u16,
table: opentype_layout_anchor_table_table<'input>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_entry_exit_record_entry_anchor<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_anchor_table<'input>>>
}

impl<'a> CommonObject for opentype_layout_entry_exit_record<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_entry_exit_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_entry_exit_record(p, table_view)
}
}

/// expected size: 4
/// trait-ready: unique decoder function (d#135)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_entry_exit_record<'input> {
entry_anchor: opentype_layout_entry_exit_record_entry_anchor<'input>,
exit_anchor: opentype_layout_entry_exit_record_entry_anchor<'input>
}

impl<'a> CommonObject for opentype_layout_cursive_pos<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_cursive_pos<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_cursive_pos(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#119)
#[derive(Debug, Clone)]
pub struct opentype_layout_cursive_pos<'input> {
pos_format: u16,
table_scope: View<'input>,
coverage: opentype_gdef_attach_list_coverage,
entry_exit_count: u16,
entry_exit_records: Vec<opentype_layout_entry_exit_record<'input>>
}

impl<'a> CommonObject for opentype_layout_mark_record<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_mark_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, array_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_mark_record(p, array_view)
}
}

/// expected size: 4
/// trait-ready: unique decoder function (d#129)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_mark_record<'input> {
mark_class: u16,
mark_anchor: opentype_layout_entry_exit_record_entry_anchor<'input>
}

impl<'a> CommonObject for opentype_layout_mark_array<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_mark_array<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_mark_array(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#123)
#[derive(Debug, Clone)]
pub struct opentype_layout_mark_array<'input> {
array_scope: View<'input>,
mark_count: u16,
mark_records: Vec<opentype_layout_mark_record<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_mark_base_pos_mark_array<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_mark_array<'input>>>
}

impl<'a> CommonObject for opentype_layout_base_array_base_record<'a> {
type Args<'x> = (u16, View<'x>);

type Output<'x> = opentype_layout_base_array_base_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let mark_class_count = args.0;
let _array_view = args.1;
Decoder_opentype_layout_base_array_base_record(p, mark_class_count, _array_view)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#134)
#[derive(Debug, Clone)]
pub struct opentype_layout_base_array_base_record<'input> {
base_anchor_offsets: Vec<u16>,
_base_anchors: std::marker::PhantomData<Vec<Option<opentype_layout_anchor_table<'input>>>>
}

impl<'a> CommonObject for opentype_layout_base_array<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_layout_base_array<'x>;

fn parse<'input>(p: &mut Parser<'input>, mark_class_count: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_base_array(p, mark_class_count)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#133)
#[derive(Debug, Clone)]
pub struct opentype_layout_base_array<'input> {
array_scope: View<'input>,
base_count: u16,
base_records: Vec<opentype_layout_base_array_base_record<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_mark_base_pos_base_array<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_base_array<'input>>>
}

impl<'a> CommonObject for opentype_layout_mark_base_pos<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_mark_base_pos<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_mark_base_pos(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#120)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_mark_base_pos<'input> {
format: u16,
table_scope: View<'input>,
mark_coverage: opentype_gdef_attach_list_coverage,
base_coverage: opentype_gdef_attach_list_coverage,
mark_class_count: u16,
mark_array: opentype_layout_mark_base_pos_mark_array<'input>,
base_array: opentype_layout_mark_base_pos_base_array<'input>
}

impl<'a> CommonObject for opentype_layout_ligature_attach_component_record<'a> {
type Args<'x> = (u16, View<'x>);

type Output<'x> = opentype_layout_ligature_attach_component_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let mark_class_count = args.0;
let table_view = args.1;
Decoder_opentype_layout_ligature_attach_component_record(p, mark_class_count, table_view)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#132)
#[derive(Debug, Clone)]
pub struct opentype_layout_ligature_attach_component_record<'input> {
record_scope: View<'input>,
ligature_anchor_offsets: Vec<u16>,
_ligature_anchors: std::marker::PhantomData<Vec<Option<opentype_layout_anchor_table<'input>>>>
}

impl<'a> CommonObject for opentype_layout_ligature_attach<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_layout_ligature_attach<'x>;

fn parse<'input>(p: &mut Parser<'input>, mark_class_count: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_ligature_attach(p, mark_class_count)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#131)
#[derive(Debug, Clone)]
pub struct opentype_layout_ligature_attach<'input> {
component_count: u16,
component_records: Vec<opentype_layout_ligature_attach_component_record<'input>>
}

impl<'a> CommonObject for opentype_layout_ligature_array<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_layout_ligature_array<'x>;

fn parse<'input>(p: &mut Parser<'input>, mark_class_count: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_ligature_array(p, mark_class_count)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#130)
#[derive(Debug, Clone)]
pub struct opentype_layout_ligature_array<'input> {
array_scope: View<'input>,
mark_class_count: u16,
ligature_count: u16,
ligature_attach_offsets: Vec<u16>,
_ligature_attaches: std::marker::PhantomData<Vec<Option<opentype_layout_ligature_attach<'input>>>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_mark_lig_pos_ligature_array<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_ligature_array<'input>>>
}

impl<'a> CommonObject for opentype_layout_mark_lig_pos<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_mark_lig_pos<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_mark_lig_pos(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#121)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_mark_lig_pos<'input> {
table_scope: View<'input>,
format: u16,
mark_coverage: opentype_gdef_attach_list_coverage,
ligature_coverage: opentype_gdef_attach_list_coverage,
mark_class_count: u16,
mark_array: opentype_layout_mark_base_pos_mark_array<'input>,
ligature_array: opentype_layout_mark_lig_pos_ligature_array<'input>
}

impl<'a> CommonObject for opentype_layout_mark2_array_mark2_record<'a> {
type Args<'x> = (u16, View<'x>);

type Output<'x> = opentype_layout_mark2_array_mark2_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let mark_class_count = args.0;
let _array_view = args.1;
Decoder_opentype_layout_mark2_array_mark2_record(p, mark_class_count, _array_view)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#125)
#[derive(Debug, Clone)]
pub struct opentype_layout_mark2_array_mark2_record<'input> {
mark2_anchor_offsets: Vec<u16>,
_mark2_anchors: std::marker::PhantomData<Vec<Option<opentype_layout_anchor_table<'input>>>>
}

impl<'a> CommonObject for opentype_layout_mark2_array<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_layout_mark2_array<'x>;

fn parse<'input>(p: &mut Parser<'input>, mark_class_count: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_mark2_array(p, mark_class_count)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#124)
#[derive(Debug, Clone)]
pub struct opentype_layout_mark2_array<'input> {
array_scope: View<'input>,
mark2_count: u16,
mark2_records: Vec<opentype_layout_mark2_array_mark2_record<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_mark_mark_pos_mark2_array<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_mark2_array<'input>>>
}

impl<'a> CommonObject for opentype_layout_mark_mark_pos<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_mark_mark_pos<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_mark_mark_pos(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#122)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_mark_mark_pos<'input> {
format: u16,
table_scope: View<'input>,
mark1_coverage: opentype_gdef_attach_list_coverage,
mark2_coverage: opentype_gdef_attach_list_coverage,
mark_class_count: u16,
mark1_array: opentype_layout_mark_base_pos_mark_array<'input>,
mark2_array: opentype_layout_mark_mark_pos_mark2_array<'input>
}

impl CommonObject for opentype_layout_value_format_flags {
type Args<'x> = ();

type Output<'x> = opentype_layout_value_format_flags;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_value_format_flags(p)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#138)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_value_format_flags {
y_advance_device: bool,
x_advance_device: bool,
y_placement_device: bool,
x_placement_device: bool,
y_advance: bool,
x_advance: bool,
y_placement: bool,
x_placement: bool
}

/// expected size: 32
/// trait-unready: multiple (6) decoders exist (d#{140, 141, 144, 145, 148, 149})
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_value_record {
x_placement: Option<u16>,
y_placement: Option<u16>,
x_advance: Option<u16>,
y_advance: Option<u16>,
x_placement_device: Option<opentype_gdef_caret_value_data_format3_table>,
y_placement_device: Option<opentype_gdef_caret_value_data_format3_table>,
x_advance_device: Option<opentype_gdef_caret_value_data_format3_table>,
y_advance_device: Option<opentype_gdef_caret_value_data_format3_table>
}

impl CommonObject for opentype_layout_pair_pos_pair_value_record {
type Args<'x> = (opentype_layout_value_format_flags, opentype_layout_value_format_flags, View<'x>);

type Output<'x> = opentype_layout_pair_pos_pair_value_record;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let value_format1 = args.0;
let value_format2 = args.1;
let set_view = args.2;
Decoder_opentype_layout_pair_pos_pair_value_record(p, value_format1, value_format2, set_view)
}
}

/// expected size: 66
/// trait-ready: unique decoder function (d#143)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_pair_pos_pair_value_record {
second_glyph: u16,
value_record1: Option<opentype_layout_value_record>,
value_record2: Option<opentype_layout_value_record>
}

impl<'a> CommonObject for opentype_layout_pair_pos_pair_set<'a> {
type Args<'x> = (opentype_layout_value_format_flags, opentype_layout_value_format_flags);

type Output<'x> = opentype_layout_pair_pos_pair_set<'x>;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let value_format1 = args.0;
let value_format2 = args.1;
Decoder_opentype_layout_pair_pos_pair_set(p, value_format1, value_format2)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#142)
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_pair_set<'input> {
set_scope: View<'input>,
pair_value_count: u16,
pair_value_records: Vec<opentype_layout_pair_pos_pair_value_record>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_pair_pos_format1_pair_sets<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_pair_pos_pair_set<'input>>>
}

impl<'a> CommonObject for opentype_layout_pair_pos_format1<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_pair_pos_format1<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_pair_pos_format1(p, table_view)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#136)
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_format1<'input> {
table_scope: View<'input>,
coverage: opentype_gdef_attach_list_coverage,
value_format1: opentype_layout_value_format_flags,
value_format2: opentype_layout_value_format_flags,
pair_set_count: u16,
pair_sets: Vec<opentype_layout_pair_pos_format1_pair_sets<'input>>
}

impl CommonObject for opentype_layout_pair_pos_class2_record {
type Args<'x> = (opentype_layout_value_format_flags, opentype_layout_value_format_flags, View<'x>);

type Output<'x> = opentype_layout_pair_pos_class2_record;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let value_format1 = args.0;
let value_format2 = args.1;
let table_view = args.2;
Decoder_opentype_layout_pair_pos_class2_record(p, value_format1, value_format2, table_view)
}
}

/// expected size: 64
/// trait-ready: unique decoder function (d#139)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_pair_pos_class2_record {
value_record1: Option<opentype_layout_value_record>,
value_record2: Option<opentype_layout_value_record>
}

/// expected size: 24
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_format2_class1_records {
class2_records: Vec<opentype_layout_pair_pos_class2_record>
}

impl<'a> CommonObject for opentype_layout_pair_pos_format2<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_pair_pos_format2<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_pair_pos_format2(p, table_view)
}
}

/// expected size: 80
/// trait-ready: unique decoder function (d#137)
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos_format2<'input> {
table_scope: View<'input>,
coverage: opentype_gdef_attach_list_coverage,
value_format1: opentype_layout_value_format_flags,
value_format2: opentype_layout_value_format_flags,
class_def1: opentype_gdef_table_glyph_class_def,
class_def2: opentype_gdef_table_glyph_class_def,
class1_count: u16,
class2_count: u16,
class1_records: Vec<opentype_layout_pair_pos_format2_class1_records>
}

/// expected size: 88
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_layout_pair_pos_subtable<'input> { Format1(opentype_layout_pair_pos_format1<'input>), Format2(opentype_layout_pair_pos_format2<'input>) }

impl<'a> CommonObject for opentype_layout_pair_pos<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_pair_pos<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_pair_pos(p)
}
}

/// expected size: 96
/// trait-ready: unique decoder function (d#118)
#[derive(Debug, Clone)]
pub struct opentype_layout_pair_pos<'input> {
pos_format: u16,
subtable: opentype_layout_pair_pos_subtable<'input>
}

impl CommonObject for opentype_layout_sequence_context_rule {
type Args<'x> = ();

type Output<'x> = opentype_layout_sequence_context_rule;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_sequence_context_rule(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#100)
#[derive(Debug, Clone)]
pub struct opentype_layout_sequence_context_rule {
glyph_count: u16,
seq_lookup_count: u16,
input_sequence: Vec<u16>,
seq_lookup_records: Vec<opentype_layout_sequence_lookup>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_sequence_context_rule_set_rules {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_sequence_context_rule>>
}

impl<'a> CommonObject for opentype_layout_sequence_context_rule_set<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_sequence_context_rule_set<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_sequence_context_rule_set(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#99)
#[derive(Debug, Clone)]
pub struct opentype_layout_sequence_context_rule_set<'input> {
table_scope: View<'input>,
rule_count: u16,
rules: Vec<opentype_layout_sequence_context_rule_set_rules>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_sequence_context_format1_seq_rule_sets<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_layout_sequence_context_rule_set<'input>>>
}

impl<'a> CommonObject for opentype_layout_sequence_context_format1<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_sequence_context_format1<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_sequence_context_format1(p, table_view)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#96)
#[derive(Debug, Clone)]
pub struct opentype_layout_sequence_context_format1<'input> {
coverage: opentype_gdef_attach_list_coverage,
seq_rule_set_count: u16,
seq_rule_sets: Vec<opentype_layout_sequence_context_format1_seq_rule_sets<'input>>
}

impl<'a> CommonObject for opentype_layout_sequence_context_format2<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_sequence_context_format2<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_sequence_context_format2(p, table_view)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#97)
#[derive(Debug, Clone)]
pub struct opentype_layout_sequence_context_format2<'input> {
coverage: opentype_gdef_attach_list_coverage,
class_def: opentype_gdef_table_glyph_class_def,
class_seq_rule_set_count: u16,
class_seq_rule_sets: Vec<opentype_layout_sequence_context_format1_seq_rule_sets<'input>>
}

impl CommonObject for opentype_layout_sequence_context_format3 {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_sequence_context_format3;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_sequence_context_format3(p, table_view)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#98)
#[derive(Debug, Clone)]
pub struct opentype_layout_sequence_context_format3 {
glyph_count: u16,
seq_lookup_count: u16,
coverage_tables: Vec<opentype_gdef_attach_list_coverage>,
seq_lookup_records: Vec<opentype_layout_sequence_lookup>
}

/// expected size: 64
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_layout_sequence_context_subst<'input> { Format1(opentype_layout_sequence_context_format1<'input>), Format2(opentype_layout_sequence_context_format2<'input>), Format3(opentype_layout_sequence_context_format3) }

impl<'a> CommonObject for opentype_layout_sequence_context<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_sequence_context<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_sequence_context(p)
}
}

/// expected size: 96
/// trait-ready: unique decoder function (d#85)
#[derive(Debug, Clone)]
pub struct opentype_layout_sequence_context<'input> {
table_scope: View<'input>,
format: u16,
subst: opentype_layout_sequence_context_subst<'input>
}

impl<'a> CommonObject for opentype_layout_single_pos_format1<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_single_pos_format1<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_single_pos_format1(p, table_view)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#146)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_single_pos_format1<'input> {
table_scope: View<'input>,
coverage: opentype_gdef_attach_list_coverage,
value_format: opentype_layout_value_format_flags,
value_record: opentype_layout_value_record
}

impl<'a> CommonObject for opentype_layout_single_pos_format2<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_single_pos_format2<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_single_pos_format2(p, table_view)
}
}

/// expected size: 64
/// trait-ready: unique decoder function (d#147)
#[derive(Debug, Clone)]
pub struct opentype_layout_single_pos_format2<'input> {
table_scope: View<'input>,
coverage: opentype_gdef_attach_list_coverage,
value_format: opentype_layout_value_format_flags,
value_count: u16,
value_records: Vec<opentype_layout_value_record>
}

/// expected size: 80
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_layout_single_pos_subtable<'input> { Format1(opentype_layout_single_pos_format1<'input>), Format2(opentype_layout_single_pos_format2<'input>) }

impl<'a> CommonObject for opentype_layout_single_pos<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_single_pos<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_single_pos(p)
}
}

/// expected size: 88
/// trait-ready: unique decoder function (d#117)
#[derive(Debug, Clone)]
pub struct opentype_layout_single_pos<'input> {
pos_format: u16,
subtable: opentype_layout_single_pos_subtable<'input>
}

impl<'a> CommonObject for opentype_layout_ground_pos<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_layout_ground_pos<'x>;

fn parse<'input>(p: &mut Parser<'input>, lookup_type: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_ground_pos(p, lookup_type)
}
}

/// expected size: 152
/// trait-ready: unique decoder function (d#116)
#[derive(Debug, Clone)]
pub enum opentype_layout_ground_pos<'input> { ChainedSequenceContext(opentype_layout_chained_sequence_context<'input>), CursivePos(opentype_layout_cursive_pos<'input>), MarkBasePos(opentype_layout_mark_base_pos<'input>), MarkLigPos(opentype_layout_mark_lig_pos<'input>), MarkMarkPos(opentype_layout_mark_mark_pos<'input>), PairPos(opentype_layout_pair_pos<'input>), SequenceContext(opentype_layout_sequence_context<'input>), SinglePos(opentype_layout_single_pos<'input>) }

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_pos_extension_extension_offset<'input> {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_layout_ground_pos<'input>>>
}

impl<'a> CommonObject for opentype_layout_pos_extension<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_pos_extension<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_pos_extension(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#115)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_pos_extension<'input> {
format: u16,
table_scope: View<'input>,
extension_lookup_type: u16,
extension_offset: opentype_layout_pos_extension_extension_offset<'input>
}

impl<'a> CommonObject for opentype_gpos_lookup_subtable<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_gpos_lookup_subtable<'x>;

fn parse<'input>(p: &mut Parser<'input>, lookup_type: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gpos_lookup_subtable(p, lookup_type)
}
}

/// expected size: 160
/// trait-ready: unique decoder function (d#114)
#[derive(Debug, Clone)]
pub enum opentype_gpos_lookup_subtable<'input> { GroundPos(opentype_layout_ground_pos<'input>), PosExtension(opentype_layout_pos_extension<'input>) }

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gpos_lookup_table_subtables<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gpos_lookup_subtable<'input>>>
}

impl<'a> CommonObject for opentype_gpos_lookup_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gpos_lookup_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gpos_lookup_table(p)
}
}

/// expected size: 64
/// trait-ready: unique decoder function (d#113)
#[derive(Debug, Clone)]
pub struct opentype_gpos_lookup_table<'input> {
table_scope: View<'input>,
lookup_type: u16,
lookup_flag: opentype_gpos_lookup_table_lookup_flag,
sub_table_count: u16,
subtables: Vec<opentype_gpos_lookup_table_subtables<'input>>,
mark_filtering_set: Option<u16>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gpos_lookup_list_lookups<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gpos_lookup_table<'input>>>
}

impl<'a> CommonObject for opentype_gpos_lookup_list<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gpos_lookup_list<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gpos_lookup_list(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#112)
#[derive(Debug, Clone)]
pub struct opentype_gpos_lookup_list<'input> {
list_scope: View<'input>,
lookup_count: u16,
lookups: Vec<opentype_gpos_lookup_list_lookups<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gpos_table_lookup_list<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gpos_lookup_list<'input>>>
}

/// expected size: 12
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct main_data_opentype_directory_TTCHeader_header_Version1_table_directories_data_table_links_gpos_feature_variations_offset__data_feature_variation_records_condition_set__data__conditions {
format: u16,
axis_index: u16,
filter_range_min_value: opentype_gvar_tuple_record_coordinates,
filter_range_max_value: opentype_gvar_tuple_record_coordinates
}

/// expected size: 56
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_variation_record_condition_set__data<'input> {
set_scope: View<'input>,
condition_count: u16,
condition_offsets: Vec<u32>,
_conditions: std::marker::PhantomData<Vec<Option<main_data_opentype_directory_TTCHeader_header_Version1_table_directories_data_table_links_gpos_feature_variations_offset__data_feature_variation_records_condition_set__data__conditions>>>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_feature_variation_record_condition_set<'input> {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_layout_feature_variation_record_condition_set__data<'input>>>
}

impl<'a> CommonObject for opentype_layout_feature_table_substitution_record<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_feature_table_substitution_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, _table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_feature_table_substitution_record(p, _table_view)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#75)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_feature_table_substitution_record<'input> {
feature_index: u16,
alternate_feature_offset: u32,
alternate_feature: std::marker::PhantomData<Option<opentype_layout_feature_table<'input>>>
}

impl<'a> CommonObject for opentype_layout_feature_table_substitution<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_feature_table_substitution<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_feature_table_substitution(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#74)
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_table_substitution<'input> {
table_scope: View<'input>,
major_version: u16,
minor_version: u16,
substitution_count: u16,
substitutions: Vec<opentype_layout_feature_table_substitution_record<'input>>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_feature_variation_record_feature_table_substitution<'input> {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_layout_feature_table_substitution<'input>>>
}

impl<'a> CommonObject for opentype_layout_feature_variation_record<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_feature_variation_record<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_feature_variation_record(p, table_view)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#73)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_feature_variation_record<'input> {
condition_set: opentype_layout_feature_variation_record_condition_set<'input>,
feature_table_substitution: opentype_layout_feature_variation_record_feature_table_substitution<'input>
}

impl<'a> CommonObject for opentype_layout_feature_variations<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_feature_variations<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_feature_variations(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#72)
#[derive(Debug, Clone)]
pub struct opentype_layout_feature_variations<'input> {
table_scope: View<'input>,
major_version: u16,
minor_version: u16,
feature_variation_record_count: u32,
feature_variation_records: Vec<opentype_layout_feature_variation_record<'input>>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gpos_table_feature_variations_offset<'input> {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_layout_feature_variations<'input>>>
}

impl<'a> CommonObject for opentype_gpos_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gpos_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gpos_table(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#42)
#[derive(Debug, Copy, Clone)]
pub struct opentype_gpos_table<'input> {
table_scope: View<'input>,
major_version: u16,
minor_version: u16,
script_list: opentype_gpos_table_script_list<'input>,
feature_list: opentype_gpos_table_feature_list<'input>,
lookup_list: opentype_gpos_table_lookup_list<'input>,
feature_variations_offset: Option<opentype_gpos_table_feature_variations_offset<'input>>
}

impl CommonObject for opentype_gsub_alternate_subst_alternate_set {
type Args<'x> = ();

type Output<'x> = opentype_gsub_alternate_subst_alternate_set;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gsub_alternate_subst_alternate_set(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#103)
#[derive(Debug, Clone)]
pub struct opentype_gsub_alternate_subst_alternate_set {
glyph_count: u16,
alternate_glyph_ids: Vec<u16>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gsub_alternate_subst_alternate_sets {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gsub_alternate_subst_alternate_set>>
}

impl<'a> CommonObject for opentype_gsub_alternate_subst<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gsub_alternate_subst<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gsub_alternate_subst(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#83)
#[derive(Debug, Clone)]
pub struct opentype_gsub_alternate_subst<'input> {
table_scope: View<'input>,
subst_format: u16,
coverage: opentype_gdef_attach_list_coverage,
alternate_set_count: u16,
alternate_sets: Vec<opentype_gsub_alternate_subst_alternate_sets>
}

impl CommonObject for opentype_gsub_ligature_subst_ligature_table {
type Args<'x> = ();

type Output<'x> = opentype_gsub_ligature_subst_ligature_table;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gsub_ligature_subst_ligature_table(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#102)
#[derive(Debug, Clone)]
pub struct opentype_gsub_ligature_subst_ligature_table {
ligature_glyph: u16,
component_count: u16,
component_glyph_ids: Vec<u16>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gsub_ligature_subst_ligature_set_ligatures {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gsub_ligature_subst_ligature_table>>
}

impl<'a> CommonObject for opentype_gsub_ligature_subst_ligature_set<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gsub_ligature_subst_ligature_set<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gsub_ligature_subst_ligature_set(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#101)
#[derive(Debug, Clone)]
pub struct opentype_gsub_ligature_subst_ligature_set<'input> {
set_scope: View<'input>,
ligature_count: u16,
ligatures: Vec<opentype_gsub_ligature_subst_ligature_set_ligatures>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_ligature_subst_ligature_sets<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gsub_ligature_subst_ligature_set<'input>>>
}

impl<'a> CommonObject for opentype_layout_ligature_subst<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_ligature_subst<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_ligature_subst(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#84)
#[derive(Debug, Clone)]
pub struct opentype_layout_ligature_subst<'input> {
table_scope: View<'input>,
subst_format: u16,
coverage: opentype_gdef_attach_list_coverage,
ligature_set_count: u16,
ligature_sets: Vec<opentype_layout_ligature_subst_ligature_sets<'input>>
}

impl CommonObject for opentype_layout_multiple_subst_sequence_table {
type Args<'x> = ();

type Output<'x> = opentype_layout_multiple_subst_sequence_table;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_multiple_subst_sequence_table(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#104)
#[derive(Debug, Clone)]
pub struct opentype_layout_multiple_subst_sequence_table {
glyph_count: u16,
substitute_glyph_ids: Vec<u16>
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_layout_multiple_subst_subst_Format1 {
sequence_count: u16,
sequence_offsets: Vec<u16>,
_sequences: std::marker::PhantomData<Vec<Option<opentype_layout_multiple_subst_sequence_table>>>
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_layout_multiple_subst_subst { Format1(opentype_layout_multiple_subst_subst_Format1) }

impl<'a> CommonObject for opentype_layout_multiple_subst<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_multiple_subst<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_multiple_subst(p)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#82)
#[derive(Debug, Clone)]
pub struct opentype_layout_multiple_subst<'input> {
table_scope: View<'input>,
subst_format: u16,
coverage: opentype_gdef_attach_list_coverage,
subst: opentype_layout_multiple_subst_subst
}

impl<'a> CommonObject for opentype_layout_reverse_chain_single_subst<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_reverse_chain_single_subst<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_reverse_chain_single_subst(p)
}
}

/// expected size: 112
/// trait-ready: unique decoder function (d#87)
#[derive(Debug, Clone)]
pub struct opentype_layout_reverse_chain_single_subst<'input> {
subst_format: u16,
table_scope: View<'input>,
coverage: opentype_gdef_attach_list_coverage,
backtrack_glyph_count: u16,
backtrack_coverage_tables: Vec<opentype_gdef_attach_list_coverage>,
lookahead_glyph_count: u16,
lookahead_coverage_tables: Vec<opentype_gdef_attach_list_coverage>,
glyph_count: u16,
substitute_glyph_ids: Vec<u16>
}

impl<'a> CommonObject for opentype_layout_single_subst_format1<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_single_subst_format1<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_single_subst_format1(p, table_view)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#105)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_single_subst_format1<'input> {
table_scope: View<'input>,
coverage: opentype_gdef_attach_list_coverage,
delta_glyph_id: u16
}

impl<'a> CommonObject for opentype_layout_single_subst_format2<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_layout_single_subst_format2<'x>;

fn parse<'input>(p: &mut Parser<'input>, table_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_single_subst_format2(p, table_view)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#106)
#[derive(Debug, Clone)]
pub struct opentype_layout_single_subst_format2<'input> {
table_scope: View<'input>,
coverage: opentype_gdef_attach_list_coverage,
glyph_count: u16,
substitute_glyph_ids: Vec<u16>
}

/// expected size: 64
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_layout_single_subst_subst<'input> { Format1(opentype_layout_single_subst_format1<'input>), Format2(opentype_layout_single_subst_format2<'input>) }

impl<'a> CommonObject for opentype_layout_single_subst<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_single_subst<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_single_subst(p)
}
}

/// expected size: 72
/// trait-ready: unique decoder function (d#81)
#[derive(Debug, Clone)]
pub struct opentype_layout_single_subst<'input> {
subst_format: u16,
subst: opentype_layout_single_subst_subst<'input>
}

impl<'a> CommonObject for opentype_layout_ground_subst<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_layout_ground_subst<'x>;

fn parse<'input>(p: &mut Parser<'input>, lookup_type: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_ground_subst(p, lookup_type)
}
}

/// expected size: 152
/// trait-ready: unique decoder function (d#80)
#[derive(Debug, Clone)]
pub enum opentype_layout_ground_subst<'input> { AlternateSubst(opentype_gsub_alternate_subst<'input>), ChainedSequenceContext(opentype_layout_chained_sequence_context<'input>), LigatureSubst(opentype_layout_ligature_subst<'input>), MultipleSubst(opentype_layout_multiple_subst<'input>), ReverseChainSingleSubst(opentype_layout_reverse_chain_single_subst<'input>), SequenceContext(opentype_layout_sequence_context<'input>), SingleSubst(opentype_layout_single_subst<'input>) }

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_subst_extension_extension_offset<'input> {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_layout_ground_subst<'input>>>
}

impl<'a> CommonObject for opentype_layout_subst_extension<'a> {
type Args<'x> = ();

type Output<'x> = opentype_layout_subst_extension<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_layout_subst_extension(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#79)
#[derive(Debug, Copy, Clone)]
pub struct opentype_layout_subst_extension<'input> {
format: u16,
table_scope: View<'input>,
extension_lookup_type: u16,
extension_offset: opentype_layout_subst_extension_extension_offset<'input>
}

impl<'a> CommonObject for opentype_gsub_lookup_subtable<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_gsub_lookup_subtable<'x>;

fn parse<'input>(p: &mut Parser<'input>, lookup_type: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gsub_lookup_subtable(p, lookup_type)
}
}

/// expected size: 160
/// trait-ready: unique decoder function (d#78)
#[derive(Debug, Clone)]
pub enum opentype_gsub_lookup_subtable<'input> { GroundSubst(opentype_layout_ground_subst<'input>), SubstExtension(opentype_layout_subst_extension<'input>) }

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gsub_lookup_table_subtables<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gsub_lookup_subtable<'input>>>
}

impl<'a> CommonObject for opentype_gsub_lookup_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gsub_lookup_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gsub_lookup_table(p)
}
}

/// expected size: 64
/// trait-ready: unique decoder function (d#77)
#[derive(Debug, Clone)]
pub struct opentype_gsub_lookup_table<'input> {
table_scope: View<'input>,
lookup_type: u16,
lookup_flag: opentype_gpos_lookup_table_lookup_flag,
sub_table_count: u16,
subtables: Vec<opentype_gsub_lookup_table_subtables<'input>>,
mark_filtering_set: Option<u16>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gsub_lookup_list_lookups<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gsub_lookup_table<'input>>>
}

impl<'a> CommonObject for opentype_gsub_lookup_list<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gsub_lookup_list<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gsub_lookup_list(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#71)
#[derive(Debug, Clone)]
pub struct opentype_gsub_lookup_list<'input> {
list_scope: View<'input>,
lookup_count: u16,
lookups: Vec<opentype_gsub_lookup_list_lookups<'input>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gsub_table_lookup_list<'input> {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_gsub_lookup_list<'input>>>
}

impl<'a> CommonObject for opentype_gsub_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gsub_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gsub_table(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#43)
#[derive(Debug, Copy, Clone)]
pub struct opentype_gsub_table<'input> {
table_scope: View<'input>,
major_version: u16,
minor_version: u16,
script_list: opentype_gpos_table_script_list<'input>,
feature_list: opentype_gpos_table_feature_list<'input>,
lookup_list: opentype_gsub_table_lookup_list<'input>,
feature_variations_offset: Option<opentype_gpos_table_feature_variations_offset<'input>>
}

/// expected size: 1
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_fvar_variation_axis_record_flags {
hidden_axis: bool
}

impl CommonObject for opentype_fvar_variation_axis_record {
type Args<'x> = ();

type Output<'x> = opentype_fvar_variation_axis_record;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_fvar_variation_axis_record(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#66)
#[derive(Debug, Copy, Clone)]
pub struct opentype_fvar_variation_axis_record {
axis_tag: u32,
min_value: opentype_head_table_font_revision,
default_value: opentype_head_table_font_revision,
max_value: opentype_head_table_font_revision,
flags: opentype_fvar_variation_axis_record_flags,
axis_name_id: u16
}

impl CommonObject for opentype_fvar_user_tuple {
type Args<'x> = u16;

type Output<'x> = opentype_fvar_user_tuple;

fn parse<'input>(p: &mut Parser<'input>, axis_count: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_fvar_user_tuple(p, axis_count)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#68)
#[derive(Debug, Clone)]
pub struct opentype_fvar_user_tuple {
coordinates: Vec<opentype_head_table_font_revision>
}

impl CommonObject for opentype_fvar_instance_record {
type Args<'x> = (u16, u16);

type Output<'x> = opentype_fvar_instance_record;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let axis_count = args.0;
let instance_size = args.1;
Decoder_opentype_fvar_instance_record(p, axis_count, instance_size)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#67)
#[derive(Debug, Clone)]
pub struct opentype_fvar_instance_record {
subfamily_nameid: u16,
flags: u16,
coordinates: opentype_fvar_user_tuple,
postscript_nameid: Option<u16>
}

impl<'a> CommonObject for opentype_fvar_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_fvar_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_fvar_table(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#44)
#[derive(Debug, Copy, Clone)]
pub struct opentype_fvar_table<'input> {
table_scope: View<'input>,
major_version: u16,
minor_version: u16,
offset_axes: u16,
axis_count: u16,
axis_size: u16,
instance_count: u16,
instance_size: u16,
_axes: std::marker::PhantomData<Vec<opentype_fvar_variation_axis_record>>,
offset_instances: u16,
_instances: std::marker::PhantomData<Vec<opentype_fvar_instance_record>>
}

impl CommonObject for opentype_gvar_tuple_record {
type Args<'x> = u16;

type Output<'x> = opentype_gvar_tuple_record;

fn parse<'input>(p: &mut Parser<'input>, axis_count: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gvar_tuple_record(p, axis_count)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#59)
#[derive(Debug, Clone)]
pub struct opentype_gvar_tuple_record {
coordinates: Vec<opentype_gvar_tuple_record_coordinates>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gvar_table_shared_tuples {
offset: u32,
_data: std::marker::PhantomData<Option<Vec<opentype_gvar_tuple_record>>>
}

/// expected size: 1
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gvar_table_flags {
is_long_offset: bool
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gvar_glyph_variation_data_tuple_variation_count {
shared_point_numbers: bool,
tuple_count: u16
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gvar_tuple_variation_header_tuple_index {
embedded_peak_tuple: bool,
intermediate_region: bool,
private_point_numbers: bool,
tuple_index: u16
}

/// expected size: 48
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_gvar_tuple_variation_header_intermediate_tuples {
start_tuple: opentype_gvar_tuple_record,
end_tuple: opentype_gvar_tuple_record
}

impl CommonObject for opentype_gvar_tuple_variation_header {
type Args<'x> = u16;

type Output<'x> = opentype_gvar_tuple_variation_header;

fn parse<'input>(p: &mut Parser<'input>, axis_count: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gvar_tuple_variation_header(p, axis_count)
}
}

/// expected size: 80
/// trait-ready: unique decoder function (d#61)
#[derive(Debug, Clone)]
pub struct opentype_gvar_tuple_variation_header {
variation_data_size: u16,
tuple_index: opentype_gvar_tuple_variation_header_tuple_index,
peak_tuple: Option<opentype_gvar_tuple_record>,
intermediate_tuples: Option<opentype_gvar_tuple_variation_header_intermediate_tuples>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_var_packed_point_numbers_run_control {
points_are_words: bool,
point_run_count: u8
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_var_packed_point_numbers_run_points { Points16(Vec<u16>), Points8(Vec<u8>) }

impl CommonObject for opentype_var_packed_point_numbers_runs {
type Args<'x> = ();

type Output<'x> = opentype_var_packed_point_numbers_runs;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_var_packed_point_numbers_runs(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#65)
#[derive(Debug, Clone)]
pub struct opentype_var_packed_point_numbers_runs {
control: opentype_var_packed_point_numbers_run_control,
points: opentype_var_packed_point_numbers_run_points
}

/// expected size: 3
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas_control {
deltas_are_zero: bool,
deltas_are_words: bool,
delta_run_count: u8
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas { Delta0(u8), Delta16(Vec<u16>), Delta8(Vec<u8>) }

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas {
control: opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas_control,
deltas: opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas
}

/// expected size: 64
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_gvar_serialized_data_per_tuple_variation_data {
private_point_numbers: Option<(u16, Vec<opentype_var_packed_point_numbers_runs>)>,
x_and_y_coordinate_deltas: (u16, Vec<opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas>)
}

impl CommonObject for opentype_gvar_serialized_data {
type Args<'x> = (bool, &'x [opentype_gvar_tuple_variation_header]);

type Output<'x> = opentype_gvar_serialized_data;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let shared_point_numbers = args.0;
let tuple_var_headers = args.1;
Decoder_opentype_gvar_serialized_data(p, shared_point_numbers, tuple_var_headers)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#62)
#[derive(Debug, Clone)]
pub struct opentype_gvar_serialized_data {
shared_point_numbers: Option<(u16, Vec<opentype_var_packed_point_numbers_runs>)>,
per_tuple_variation_data: Vec<opentype_gvar_serialized_data_per_tuple_variation_data>
}

impl<'a> CommonObject for opentype_gvar_glyph_variation_data<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_gvar_glyph_variation_data<'x>;

fn parse<'input>(p: &mut Parser<'input>, axis_count: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gvar_glyph_variation_data(p, axis_count)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#60)
#[derive(Debug, Clone)]
pub struct opentype_gvar_glyph_variation_data<'input> {
data_scope: View<'input>,
tuple_variation_count: opentype_gvar_glyph_variation_data_tuple_variation_count,
data_offset: u16,
tuple_variation_headers: Vec<opentype_gvar_tuple_variation_header>,
_data: std::marker::PhantomData<opentype_gvar_serialized_data>
}

impl<'a> CommonObject for opentype_gvar_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_gvar_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_gvar_table(p)
}
}

/// expected size: 80
/// trait-ready: unique decoder function (d#45)
#[derive(Debug, Clone)]
pub struct opentype_gvar_table<'input> {
table_scope: View<'input>,
major_version: u16,
minor_version: u16,
axis_count: u16,
shared_tuple_count: u16,
shared_tuples: opentype_gvar_table_shared_tuples,
glyph_count: u16,
flags: opentype_gvar_table_flags,
glyph_variation_data_array_offset: u32,
glyph_variation_data_offsets: opentype_loca_table_offsets,
_glyph_variation_data_array: std::marker::PhantomData<Vec<Option<opentype_gvar_glyph_variation_data<'input>>>>
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_kern_kern_subtable_coverage {
format: u16,
r#override: bool,
cross_stream: bool,
minimum: bool,
horizontal: bool
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_kern_subtable_format0_kern_pairs {
left: u16,
right: u16,
value: u16
}

impl CommonObject for opentype_kern_subtable_format0 {
type Args<'x> = ();

type Output<'x> = opentype_kern_subtable_format0;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_kern_subtable_format0(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#55)
#[derive(Debug, Clone)]
pub struct opentype_kern_subtable_format0 {
n_pairs: u16,
search_range: u16,
entry_selector: u16,
range_shift: u16,
kern_pairs: Vec<opentype_kern_subtable_format0_kern_pairs>
}

impl CommonObject for opentype_kern_class_table {
type Args<'x> = ();

type Output<'x> = opentype_kern_class_table;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_kern_class_table(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#57)
#[derive(Debug, Clone)]
pub struct opentype_kern_class_table {
first_glyph: u16,
n_glyphs: u16,
class_values: Vec<u16>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_kern_subtable_format2_left_class_offset {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_kern_class_table>>
}

impl CommonObject for opentype_kern_kerning_array {
type Args<'x> = (u16, u16);

type Output<'x> = opentype_kern_kerning_array;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let left_glyph_count = args.0;
let right_glyph_count = args.1;
Decoder_opentype_kern_kerning_array(p, left_glyph_count, right_glyph_count)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#58)
#[derive(Debug, Clone)]
pub struct opentype_kern_kerning_array {
left_glyph_count: u16,
right_glyph_count: u16,
kerning_values: Vec<Vec<u16>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_kern_subtable_format2_kerning_array_offset {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_kern_kerning_array>>
}

impl<'a> CommonObject for opentype_kern_subtable_format2<'a> {
type Args<'x> = ();

type Output<'x> = opentype_kern_subtable_format2<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_kern_subtable_format2(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#56)
#[derive(Debug, Copy, Clone)]
pub struct opentype_kern_subtable_format2<'input> {
table_scope: View<'input>,
row_width: u16,
left_class_offset: opentype_kern_subtable_format2_left_class_offset,
right_class_offset: opentype_kern_subtable_format2_left_class_offset,
kerning_array_offset: opentype_kern_subtable_format2_kerning_array_offset
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_kern_kern_subtable_data<'input> { Format0(opentype_kern_subtable_format0), Format2(opentype_kern_subtable_format2<'input>) }

impl<'a> CommonObject for opentype_kern_kern_subtable<'a> {
type Args<'x> = ();

type Output<'x> = opentype_kern_kern_subtable<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_kern_kern_subtable(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#54)
#[derive(Debug, Clone)]
pub struct opentype_kern_kern_subtable<'input> {
version: u16,
length: u16,
coverage: opentype_kern_kern_subtable_coverage,
data: opentype_kern_kern_subtable_data<'input>
}

impl<'a> CommonObject for opentype_kern_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_kern_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_kern_table(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#46)
#[derive(Debug, Clone)]
pub struct opentype_kern_table<'input> {
version: u16,
n_tables: u16,
subtables: Vec<opentype_kern_kern_subtable<'input>>
}

/// expected size: 8
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_design_axes_array_design_axes {
axis_tag: u32,
axis_name_id: u16,
axis_ordering: u16
}

impl CommonObject for opentype_stat_design_axes_array {
type Args<'x> = u16;

type Output<'x> = opentype_stat_design_axes_array;

fn parse<'input>(p: &mut Parser<'input>, design_axis_count: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_stat_design_axes_array(p, design_axis_count)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#50)
#[derive(Debug, Clone)]
pub struct opentype_stat_design_axes_array {
design_axes: Vec<opentype_stat_design_axes_array_design_axes>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_design_axes {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_stat_design_axes_array>>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_table_data_Format1_flags {
elidable_axis_value_name: bool,
older_sibling_font_attribute: bool
}

/// expected size: 16
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_table_data_Format1 {
axis_index: u16,
flags: opentype_stat_axis_value_table_data_Format1_flags,
value_name_id: u16,
value: opentype_head_table_font_revision
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_table_data_Format2 {
axis_index: u16,
flags: opentype_stat_axis_value_table_data_Format1_flags,
value_name_id: u16,
nominal_value: opentype_head_table_font_revision,
range_min_value: opentype_head_table_font_revision,
range_max_value: opentype_head_table_font_revision
}

/// expected size: 24
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_table_data_Format3 {
axis_index: u16,
flags: opentype_stat_axis_value_table_data_Format1_flags,
value_name_id: u16,
value: opentype_head_table_font_revision,
linked_value: opentype_head_table_font_revision
}

/// expected size: 12
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_table_data_Format4_axis_values {
axis_index: u16,
value: opentype_head_table_font_revision
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_stat_axis_value_table_data_Format4 {
axis_count: u16,
flags: opentype_stat_axis_value_table_data_Format1_flags,
value_name_id: u16,
axis_values: Vec<opentype_stat_axis_value_table_data_Format4_axis_values>
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_stat_axis_value_table_data { Format1(opentype_stat_axis_value_table_data_Format1), Format2(opentype_stat_axis_value_table_data_Format2), Format3(opentype_stat_axis_value_table_data_Format3), Format4(opentype_stat_axis_value_table_data_Format4) }

impl CommonObject for opentype_stat_axis_value_table {
type Args<'x> = ();

type Output<'x> = opentype_stat_axis_value_table;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_stat_axis_value_table(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#52)
#[derive(Debug, Clone)]
pub struct opentype_stat_axis_value_table {
format: u16,
data: opentype_stat_axis_value_table_data
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_axis_value_array_axis_values {
offset: u16,
_data: std::marker::PhantomData<Option<opentype_stat_axis_value_table>>
}

impl<'a> CommonObject for opentype_stat_axis_value_array<'a> {
type Args<'x> = u16;

type Output<'x> = opentype_stat_axis_value_array<'x>;

fn parse<'input>(p: &mut Parser<'input>, axis_value_count: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_stat_axis_value_array(p, axis_value_count)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#51)
#[derive(Debug, Clone)]
pub struct opentype_stat_axis_value_array<'input> {
array_scope: View<'input>,
axis_values: Vec<opentype_stat_axis_value_array_axis_values>
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table_axis_value_offsets<'input> {
offset: u32,
_data: std::marker::PhantomData<Option<opentype_stat_axis_value_array<'input>>>
}

impl<'a> CommonObject for opentype_stat_table<'a> {
type Args<'x> = ();

type Output<'x> = opentype_stat_table<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_stat_table(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#47)
#[derive(Debug, Copy, Clone)]
pub struct opentype_stat_table<'input> {
table_scope: View<'input>,
major_version: u16,
minor_version: u16,
design_axis_size: u16,
design_axis_count: u16,
design_axes: opentype_stat_table_design_axes,
axis_value_count: u16,
axis_value_offsets: opentype_stat_table_axis_value_offsets<'input>,
elided_fallback_name_id: u16
}

impl<'a> CommonObject for opentype_table_directory_table_links<'a> {
type Args<'x> = (&'x [opentype_table_record], View<'x>);

type Output<'x> = opentype_table_directory_table_links<'x>;

fn parse<'input>(p: &mut Parser<'input>, args: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
let tables = args.0;
let font_view = args.1;
Decoder_opentype_table_directory_table_links(p, tables, font_view)
}
}

/// expected size: 1200
/// trait-ready: unique decoder function (d#28)
#[derive(Debug, Clone)]
pub struct opentype_table_directory_table_links<'input> {
cmap: opentype_cmap_table<'input>,
head: opentype_head_table,
hhea: opentype_hhea_table,
maxp: opentype_maxp_table,
hmtx: opentype_hmtx_table,
name: opentype_name_table<'input>,
os2: opentype_os2_table,
post: opentype_post_table<'input>,
cvt: Option<Vec<u16>>,
fpgm: Option<Vec<u8>>,
loca: Option<opentype_loca_table>,
glyf: Option<opentype_glyf_table<'input>>,
prep: Option<Vec<u8>>,
gasp: Option<opentype_gasp_table>,
base: Option<opentype_base_table<'input>>,
gdef: Option<opentype_gdef_table<'input>>,
gpos: Option<opentype_gpos_table<'input>>,
gsub: Option<opentype_gsub_table<'input>>,
fvar: Option<opentype_fvar_table<'input>>,
gvar: Option<opentype_gvar_table<'input>>,
kern: Option<opentype_kern_table<'input>>,
stat: Option<opentype_stat_table<'input>>,
vhea: Option<opentype_hhea_table>,
vmtx: Option<opentype_hmtx_table>
}

impl<'a> CommonObject for opentype_table_directory<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_table_directory<'x>;

fn parse<'input>(p: &mut Parser<'input>, font_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_table_directory(p, font_view)
}
}

/// expected size: 1240
/// trait-ready: unique decoder function (d#25)
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

/// expected size: 1248
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version1_table_directories<'input> {
offset: u32,
data: Option<opentype_table_directory<'input>>
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version1<'input> {
num_fonts: u32,
table_directories: Vec<opentype_ttc_header_header_Version1_table_directories<'input>>
}

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub struct opentype_ttc_header_header_Version2<'input> {
num_fonts: u32,
table_directories: Vec<opentype_ttc_header_header_Version1_table_directories<'input>>,
dsig_tag: u32,
dsig_length: u32,
dsig_offset: u32
}

/// expected size: 48
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_ttc_header_header<'input> { UnknownVersion(u16), Version1(opentype_ttc_header_header_Version1<'input>), Version2(opentype_ttc_header_header_Version2<'input>) }

impl<'a> CommonObject for opentype_ttc_header<'a> {
type Args<'x> = View<'x>;

type Output<'x> = opentype_ttc_header<'x>;

fn parse<'input>(p: &mut Parser<'input>, font_view: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_ttc_header(p, font_view)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#26)
#[derive(Debug, Clone)]
pub struct opentype_ttc_header<'input> {
ttc_tag: u32,
major_version: u16,
minor_version: u16,
header: opentype_ttc_header_header<'input>
}

/// expected size: 1248
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [Noop, DirectHeap] }, Layout { size: 56, align: 8 (1 << 3) })
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum opentype_main_directory<'input> { TTCHeader(opentype_ttc_header<'input>), TableDirectory(opentype_table_directory<'input>) }

impl<'a> CommonObject for opentype_main<'a> {
type Args<'x> = ();

type Output<'x> = opentype_main<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_main(p)
}
}

/// expected size: 1256
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [Noop, InDef(InEnum { variants: [Noop, DirectHeap] })] }, Layout { size: 64, align: 8 (1 << 3) })
/// trait-ready: unique decoder function (d#14)
#[derive(Debug, Clone)]
pub struct opentype_main<'input> {
magic: u32,
directory: opentype_main_directory<'input>
}

impl CommonObject for png_ihdr_data {
type Args<'x> = ();

type Output<'x> = png_ihdr_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_ihdr_data(p)
}
}

/// expected size: 16
/// trait-ready: unique decoder function (d#282)
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

impl CommonObject for png_ihdr {
type Args<'x> = ();

type Output<'x> = png_ihdr;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_ihdr(p)
}
}

/// expected size: 28
/// trait-ready: unique decoder function (d#235)
#[derive(Debug, Copy, Clone)]
pub struct png_ihdr {
length: u32,
tag: (u8, u8, u8, u8),
data: png_ihdr_data,
crc: u32
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct png_trns_color_type_0 {
greyscale: u16
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct png_trns_color_type_2 {
red: u16,
green: u16,
blue: u16
}

/// expected size: 1
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct png_trns_color_type_3 {
palette_index: u8
}

impl CommonObject for png_bkgd {
type Args<'x> = png_ihdr;

type Output<'x> = png_bkgd;

fn parse<'input>(p: &mut Parser<'input>, ihdr: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_bkgd(p, ihdr)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#262)
#[derive(Debug, Copy, Clone)]
pub enum png_bkgd { color_type_0(png_trns_color_type_0), color_type_2(png_trns_color_type_2), color_type_3(png_trns_color_type_3), color_type_4(png_trns_color_type_0), color_type_6(png_trns_color_type_2) }

impl CommonObject for png_chrm {
type Args<'x> = ();

type Output<'x> = png_chrm;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_chrm(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#254)
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

impl CommonObject for png_gama {
type Args<'x> = ();

type Output<'x> = png_gama;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_gama(p)
}
}

/// expected size: 4
/// trait-ready: unique decoder function (d#255)
#[derive(Debug, Copy, Clone)]
pub struct png_gama {
gamma: u32
}

impl CommonObject for png_hist {
type Args<'x> = ();

type Output<'x> = png_hist;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_hist(p)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#263)
#[derive(Debug, Clone)]
pub struct png_hist {
histogram: Vec<u16>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct zlib_main_compression_method_flags {
compression_info: u8,
compression_method: u8
}

/// expected size: 3
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct zlib_main_flags {
flevel: u8,
fdict: bool,
fcheck: u8
}

/// expected size: 96
/// trait-unready: multiple (4) decoders exist (d#{238, 269, 275, 280})
#[derive(Debug, Clone)]
pub struct zlib_main {
compression_method_flags: zlib_main_compression_method_flags,
flags: zlib_main_flags,
dict_id: Option<u32>,
data: deflate_main,
adler32: u32
}

impl CommonObject for png_iccp {
type Args<'x> = ();

type Output<'x> = png_iccp;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_iccp(p)
}
}

/// expected size: 128
/// trait-ready: unique decoder function (d#256)
#[derive(Debug, Clone)]
pub struct png_iccp {
profile_name: Vec<u8>,
compression_method: u8,
compressed_profile: zlib_main
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum png_itxt_text_compressed { invalid(Vec<u8>), valid(Vec<char>) }

/// expected size: 40
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum png_itxt_text { compressed(png_itxt_text_compressed), uncompressed(Vec<char>) }

impl CommonObject for png_itxt {
type Args<'x> = ();

type Output<'x> = png_itxt;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_itxt(p)
}
}

/// expected size: 120
/// trait-ready: unique decoder function (d#259)
#[derive(Debug, Clone)]
pub struct png_itxt {
keyword: Vec<u8>,
compression_flag: u8,
compression_method: u8,
language_tag: Vec<u8>,
translated_keyword: Vec<char>,
text: png_itxt_text
}

impl CommonObject for png_phys {
type Args<'x> = ();

type Output<'x> = png_phys;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_phys(p)
}
}

/// expected size: 12
/// trait-ready: unique decoder function (d#264)
#[derive(Debug, Copy, Clone)]
pub struct png_phys {
pixels_per_unit_x: u32,
pixels_per_unit_y: u32,
unit_specifier: u8
}

/// expected size: 1
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_0 {
sig_greyscale_bits: u8
}

/// expected size: 3
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_2 {
sig_red_bits: u8,
sig_green_bits: u8,
sig_blue_bits: u8
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_4 {
sig_greyscale_bits: u8,
sig_alpha_bits: u8
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct png_sbit_color_type_6 {
sig_red_bits: u8,
sig_green_bits: u8,
sig_blue_bits: u8,
sig_alpha_bits: u8
}

impl CommonObject for png_sbit {
type Args<'x> = png_ihdr;

type Output<'x> = png_sbit;

fn parse<'input>(p: &mut Parser<'input>, ihdr: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_sbit(p, ihdr)
}
}

/// expected size: 5
/// trait-ready: unique decoder function (d#257)
#[derive(Debug, Copy, Clone)]
pub enum png_sbit { color_type_0(png_sbit_color_type_0), color_type_2(png_sbit_color_type_2), color_type_3(png_sbit_color_type_2), color_type_4(png_sbit_color_type_4), color_type_6(png_sbit_color_type_6) }

/// expected size: 10
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct png_splt_palette_sample_depth_u16 {
red: u16,
green: u16,
blue: u16,
alpha: u16,
frequency: u16
}

/// expected size: 6
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct png_splt_palette_sample_depth_u8 {
red: u8,
green: u8,
blue: u8,
alpha: u8,
frequency: u16
}

/// expected size: 32
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum png_splt_palette { sample_depth_u16(Vec<png_splt_palette_sample_depth_u16>), sample_depth_u8(Vec<png_splt_palette_sample_depth_u8>) }

impl CommonObject for png_splt {
type Args<'x> = ();

type Output<'x> = png_splt;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_splt(p)
}
}

/// expected size: 64
/// trait-ready: unique decoder function (d#265)
#[derive(Debug, Clone)]
pub struct png_splt {
palette_name: Vec<u8>,
sample_depth: u8,
palette: png_splt_palette
}

impl CommonObject for png_srgb {
type Args<'x> = ();

type Output<'x> = png_srgb;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_srgb(p)
}
}

/// expected size: 1
/// trait-ready: unique decoder function (d#258)
#[derive(Debug, Copy, Clone)]
pub struct png_srgb {
rendering_intent: u8
}

impl CommonObject for png_text {
type Args<'x> = ();

type Output<'x> = png_text;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_text(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#260)
#[derive(Debug, Clone)]
pub struct png_text {
keyword: Vec<u8>,
text: Vec<u8>
}

impl CommonObject for png_time {
type Args<'x> = ();

type Output<'x> = png_time;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_time(p)
}
}

/// expected size: 8
/// trait-ready: unique decoder function (d#266)
#[derive(Debug, Copy, Clone)]
pub struct png_time {
year: u16,
month: u8,
day: u8,
hour: u8,
minute: u8,
second: u8
}

impl CommonObject for png_trns {
type Args<'x> = png_ihdr;

type Output<'x> = png_trns;

fn parse<'input>(p: &mut Parser<'input>, ihdr: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_trns(p, ihdr)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#253)
#[derive(Debug, Clone)]
pub enum png_trns { color_type_0(png_trns_color_type_0), color_type_2(png_trns_color_type_2), color_type_3(Vec<png_trns_color_type_3>) }

impl CommonObject for png_ztxt {
type Args<'x> = ();

type Output<'x> = png_ztxt;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_ztxt(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#261)
#[derive(Debug, Clone)]
pub struct png_ztxt {
keyword: Vec<u8>,
compression_method: u8,
compressed_text: Vec<char>
}

/// expected size: 136
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum png_chunk_data { PLTE(Vec<gif_color_table_entry>), bKGD(png_bkgd), cHRM(png_chrm), gAMA(png_gama), hIST(png_hist), iCCP(png_iccp), iTXt(png_itxt), pHYs(png_phys), sBIT(png_sbit), sPLT(png_splt), sRGB(png_srgb), tEXt(png_text), tIME(png_time), tRNS(png_trns), unknown(Vec<u8>), zTXt(png_ztxt) }

impl CommonObject for png_chunk {
type Args<'x> = png_ihdr;

type Output<'x> = png_chunk;

fn parse<'input>(p: &mut Parser<'input>, ihdr: Self::Args<'input>) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_chunk(p, ihdr)
}
}

/// expected size: 168
/// trait-ready: unique decoder function (d#236)
#[derive(Debug, Clone)]
pub struct png_chunk {
length: u32,
tag: Vec<u8>,
data: png_chunk_data,
crc: u32
}

impl CommonObject for png_iend {
type Args<'x> = ();

type Output<'x> = png_iend;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_iend(p)
}
}

/// expected size: 12
/// trait-ready: unique decoder function (d#239)
#[derive(Debug, Copy, Clone)]
pub struct png_iend {
length: u32,
tag: (u8, u8, u8, u8),
crc: u32
}

impl CommonObject for png_main {
type Args<'x> = ();

type Output<'x> = png_main;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_main(p)
}
}

/// expected size: 208
/// trait-ready: unique decoder function (d#9)
#[derive(Debug, Clone)]
pub struct png_main {
signature: Vec<u8>,
ihdr: png_ihdr,
chunks: Vec<png_chunk>,
idat: zlib_main,
more_chunks: Vec<png_chunk>,
iend: png_iend
}

impl CommonObject for riff_chunk {
type Args<'x> = ();

type Output<'x> = riff_chunk;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_riff_chunk(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#234)
#[derive(Debug, Clone)]
pub struct riff_chunk {
tag: (u8, u8, u8, u8),
length: u32,
data: Vec<u8>,
pad: Option<u8>
}

impl CommonObject for riff_subchunks {
type Args<'x> = ();

type Output<'x> = riff_subchunks;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_riff_subchunks(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#232)
#[derive(Debug, Clone)]
pub struct riff_subchunks {
tag: (u8, u8, u8, u8),
chunks: Vec<riff_chunk>
}

impl CommonObject for riff_main {
type Args<'x> = ();

type Output<'x> = riff_main;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_riff_main(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#10)
#[derive(Debug, Clone)]
pub struct riff_main {
tag: (u8, u8, u8, u8),
length: u32,
data: riff_subchunks,
pad: Option<u8>
}

impl CommonObject for rle_new_style {
type Args<'x> = ();

type Output<'x> = rle_new_style;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_rle_new_style(p)
}
}

/// expected size: 24
/// trait-ready: unique decoder function (d#22)
#[derive(Debug, Clone)]
pub struct rle_new_style {
data: Vec<u8>
}

impl CommonObject for rle_old_style_run {
type Args<'x> = ();

type Output<'x> = rle_old_style_run;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_rle_old_style_run(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#24)
#[derive(Debug, Clone)]
pub struct rle_old_style_run {
len: u8,
char: u8,
buf: Vec<u8>
}

impl CommonObject for rle_old_style {
type Args<'x> = ();

type Output<'x> = rle_old_style;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_rle_old_style(p)
}
}

/// expected size: 48
/// trait-ready: unique decoder function (d#21)
#[derive(Debug, Clone)]
pub struct rle_old_style {
runs: Vec<rle_old_style_run>,
data: Vec<u8>
}

impl CommonObject for rle_main {
type Args<'x> = ();

type Output<'x> = rle_main;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_rle_main(p)
}
}

/// expected size: 56
/// trait-ready: unique decoder function (d#15)
#[derive(Debug, Clone)]
pub enum rle_main { new_style(rle_new_style), old_style(rle_old_style) }

/// expected size: 24
/// trait-unready: multiple (3) decoders exist (d#{224, 226, 227})
#[derive(Debug, Clone)]
pub struct tar_ascii_string_opt0_nonempty {
string: Vec<u8>
}

impl CommonObject for tar_header {
type Args<'x> = ();

type Output<'x> = tar_header;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_tar_header(p)
}
}

/// expected size: 328
/// trait-ready: unique decoder function (d#223)
#[derive(Debug, Clone)]
pub struct tar_header {
name: tar_ascii_string_opt0_nonempty,
mode: tar_ascii_string_opt0_nonempty,
uid: tar_ascii_string_opt0_nonempty,
gid: tar_ascii_string_opt0_nonempty,
size: u32,
mtime: tar_ascii_string_opt0_nonempty,
chksum: tar_ascii_string_opt0_nonempty,
typeflag: u8,
linkname: tar_ascii_string_opt0_nonempty,
magic: (u8, u8, u8, u8, u8, u8),
version: (u8, u8),
uname: tar_ascii_string_opt0_nonempty,
gname: tar_ascii_string_opt0_nonempty,
devmajor: tar_ascii_string_opt0_nonempty,
devminor: tar_ascii_string_opt0_nonempty,
prefix: tar_ascii_string_opt0_nonempty,
pad: Vec<u8>
}

impl CommonObject for tar_header_with_data {
type Args<'x> = ();

type Output<'x> = tar_header_with_data;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_tar_header_with_data(p)
}
}

/// expected size: 352
/// trait-ready: unique decoder function (d#222)
#[derive(Debug, Clone)]
pub struct tar_header_with_data {
header: tar_header,
file: Vec<u8>
}

/// expected size: 24
/// trait-unready: multiple (2) decoders exist (d#{12, 426})
#[derive(Debug, Clone)]
pub struct tar_main {
contents: Vec<tar_header_with_data>
}

impl<'a> CommonObject for waldo_main<'a> {
type Args<'x> = ();

type Output<'x> = waldo_main<'x>;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_waldo_main(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#2)
#[derive(Debug, Clone)]
pub struct waldo_main<'input> {
r#where: u64,
noise: Vec<u8>,
waldo: &'input [u8]
}

/// expected size: 1264
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InEnum { variants: [DirectHeap, Noop, Noop, DirectHeap, Noop, InTuple { pos: [InDef(InRecord { fields: [Noop, InDef(InEnum { variants: [Noop, DirectHeap] })] })] }, Noop, DirectHeap, Noop, Noop, Noop, Noop, Noop, Noop, Noop] }, Layout { size: 104, align: 8 (1 << 3) })
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Clone)]
pub enum main_data<'input> { elf(elf_main), gif(gif_main), gzip(Vec<gzip_main>), jpeg(jpeg_main), mpeg4(mpeg4_main), opentype(opentype_main<'input>), peano(Vec<u32>), png(png_main), riff(riff_main), rle(rle_main), tar(tar_main), text(Vec<char>), tgz(Vec<tar_main>), tiff(tiff_main), waldo(waldo_main<'input>) }

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct tar_header_size_lhs {
value: u32
}

impl CommonObject for png_idat {
type Args<'x> = ();

type Output<'x> = png_idat;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_png_idat(p)
}
}

/// expected size: 40
/// trait-ready: unique decoder function (d#237)
#[derive(Debug, Clone)]
pub struct png_idat {
length: u32,
tag: (u8, u8, u8, u8),
data: Vec<u8>,
crc: u32
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_cmap_subtable_format0_length_lhs {
format: u16
}

/// expected size: 4
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_cmap_subtable_format8_length_lhs {
format: u16,
__reserved: u16
}

/// expected size: 7
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_glyf_simple_flags_lhs {
repeats: u8,
field_set: opentype_glyf_simple_flags
}

impl CommonObject for opentype_glyph_description_simple_flags_raw {
type Args<'x> = ();

type Output<'x> = opentype_glyph_description_simple_flags_raw;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_opentype_glyph_description_simple_flags_raw(p)
}
}

/// expected size: 7
/// trait-ready: unique decoder function (d#172)
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
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct opentype_common_device_or_variation_index_table_delta_format_lhs {
__skipped0: u16,
__skipped1: u16
}

impl CommonObject for rle_new_style_run {
type Args<'x> = ();

type Output<'x> = rle_new_style_run;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_rle_new_style_run(p)
}
}

/// expected size: 32
/// trait-ready: unique decoder function (d#23)
#[derive(Debug, Clone)]
pub struct rle_new_style_run {
_len: u8,
_char: u8,
buf: Vec<u8>
}

/// expected size: 1264
/// heap outcome (HeapStrategy { absolute_cutoff: None, variant_cutoff: Some(128) }): (InRecord { fields: [InDef(InEnum { variants: [DirectHeap, Noop, Noop, DirectHeap, Noop, InTuple { pos: [InDef(InRecord { fields: [Noop, InDef(InEnum { variants: [Noop, DirectHeap] })] })] }, Noop, DirectHeap, Noop, Noop, Noop, Noop, Noop, Noop, Noop] })] }, Layout { size: 104, align: 8 (1 << 3) })
/// trait-unready: multiple (2) decoders exist (d#{0, 1})
#[derive(Debug, Clone)]
pub struct main<'input> {
data: main_data<'input>
}

/// expected size: 2
/// trait-orphaned: no decoder functions provided
#[derive(Debug, Copy, Clone)]
pub struct jpeg_exp_data_expand_horizontal_vertical {
expand_horizontal: u8,
expand_vertical: u8
}

/// expected size: 2
/// trait-unready: multiple (2) decoders exist (d#{456, 461})
#[derive(Debug, Copy, Clone)]
pub struct jpeg_exp_data {
expand_horizontal_vertical: jpeg_exp_data_expand_horizontal_vertical
}

impl CommonObject for jpeg_exp {
type Args<'x> = ();

type Output<'x> = jpeg_exp;

fn parse<'input>(p: &mut Parser<'input>, _: ()) -> Result<Self::Output<'input>, ParseError> {
Decoder_jpeg_exp(p)
}
}

/// expected size: 6
/// trait-ready: unique decoder function (d#460)
#[derive(Debug, Copy, Clone)]
pub struct jpeg_exp {
marker: jpeg_soi,
length: u16,
data: jpeg_exp_data
}

/// d#0
fn Decoder_main<'input>(_input: &mut Parser<'input>) -> Result<main<'input>, ParseError> {
Decoder1(_input)
}

/// d#1
fn Decoder1<'input>(_input: &mut Parser<'input>) -> Result<main<'input>, ParseError> {
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
_input.next_alt(false)?;
}
};
let res = (|| {
let inner = (Decoder_rle_main(_input))?;
PResult::Ok(main_data::rle(inner))
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
let inner = (Decoder16(_input))?;
PResult::Ok(main_data::text(inner))
})();
match res {
Ok(inner) => {
PResult::Ok(inner)
},

Err(_e) => {
Err(_e)
}
}
})())?;
_input.finish()?;
PResult::Ok(main { data })
}

/// d#2
fn Decoder_waldo_main<'input>(_input: &mut Parser<'input>) -> Result<waldo_main<'input>, ParseError> {
let r#where = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
let noise = {
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
return Err(ParseError::ExcludedBranch(5625702265340316943u64));
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
return Err(ParseError::ExcludedBranch(4260205764162136487u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(5712308626808297759u64));
}
};
let _here = _input.get_offset_u64();
let waldo = {
let scope = _input.view();
scope.offset((try_sub!(r#where, _here, 13646096770106105413u64)) as usize)?.read_len(5u8 as usize)
};
_input.skip_remainder();
PResult::Ok(waldo_main { r#where, noise, waldo })
}

/// d#3
fn Decoder3(_input: &mut Parser<'_>) -> Result<Vec<u32>, ParseError> {
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
return Err(ParseError::ExcludedBranch(8638089167112501923u64));
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
let next_elem = (Decoder450(_input))?;
accum.push(next_elem)
}
};
PResult::Ok(accum)
}

/// d#4
fn Decoder_gif_main(_input: &mut Parser<'_>) -> Result<gif_main, ParseError> {
let header = (Decoder_gif_header(_input))?;
let logical_screen = (Decoder_gif_logical_screen(_input))?;
let blocks = {
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
return Err(ParseError::ExcludedBranch(1453530207670075215u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_gif_block(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let trailer = (Decoder_gif_trailer(_input))?;
PResult::Ok(gif_main { header, logical_screen, blocks, trailer })
}

/// d#5
fn Decoder5(_input: &mut Parser<'_>) -> Result<Vec<tar_main>, ParseError> {
let gzip_raw = (Decoder425(_input))?;
let mut accum = Vec::new();
for item in gzip_raw.clone() {
let next_elem = {
let mut buf_parser = Parser::new(slice_all(&item.data.inflate));
let buf_input = &mut buf_parser;
(Decoder426(buf_input))?
};
accum.push(next_elem)
};
PResult::Ok(accum)
}

/// d#6
fn Decoder6(_input: &mut Parser<'_>) -> Result<Vec<gzip_main>, ParseError> {
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
let fextra = if header.file_flags.fextra {
Some((Decoder_gzip_fextra(_input))?)
} else {
None
};
let fname = if header.file_flags.fname {
Some((Decoder419(_input))?)
} else {
None
};
let fcomment = if header.file_flags.fcomment {
Some((Decoder_gzip_fcomment(_input))?)
} else {
None
};
let fhcrc = if header.file_flags.fhcrc {
Some((Decoder_gzip_fhcrc(_input))?)
} else {
None
};
let data = {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
ret
};
let footer = (Decoder_gzip_footer(_input))?;
gzip_main { header, fextra, fname, fcomment, fhcrc, data, footer }
};
accum.push(next_elem)
}
};
PResult::Ok(accum)
}

/// d#7
fn Decoder_jpeg_main(_input: &mut Parser<'_>) -> Result<jpeg_main, ParseError> {
let soi = (Decoder_jpeg_soi(_input))?;
let frame = (Decoder_jpeg_frame(_input))?;
let eoi = (Decoder341(_input))?;
PResult::Ok(jpeg_main { soi, frame, eoi })
}

/// d#8
fn Decoder_mpeg4_main(_input: &mut Parser<'_>) -> Result<mpeg4_main, ParseError> {
let atoms = {
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
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(mpeg4_main { atoms })
}

/// d#9
fn Decoder_png_main(_input: &mut Parser<'_>) -> Result<png_main, ParseError> {
let signature = {
let ix0 = {
let b = _input.read_byte()?;
if b == 137 {
b
} else {
return Err(ParseError::ExcludedBranch(15915510438164744429u64));
}
};
let ix1 = {
let b = _input.read_byte()?;
if b == 80 {
b
} else {
return Err(ParseError::ExcludedBranch(1542992798780655146u64));
}
};
let ix2 = {
let b = _input.read_byte()?;
if b == 78 {
b
} else {
return Err(ParseError::ExcludedBranch(7028560493922100069u64));
}
};
let ix3 = {
let b = _input.read_byte()?;
if b == 71 {
b
} else {
return Err(ParseError::ExcludedBranch(2649783168072194737u64));
}
};
let ix4 = {
let b = _input.read_byte()?;
if b == 13 {
b
} else {
return Err(ParseError::ExcludedBranch(8253205784254894771u64));
}
};
let ix5 = {
let b = _input.read_byte()?;
if b == 10 {
b
} else {
return Err(ParseError::ExcludedBranch(1225514472166157741u64));
}
};
let ix6 = {
let b = _input.read_byte()?;
if b == 26 {
b
} else {
return Err(ParseError::ExcludedBranch(1224415506115142500u64));
}
};
let ix7 = {
let b = _input.read_byte()?;
if b == 10 {
b
} else {
return Err(ParseError::ExcludedBranch(16859485491091215361u64));
}
};
vec![ix0, ix1, ix2, ix3, ix4, ix5, ix6, ix7]
};
let ihdr = (Decoder_png_ihdr(_input))?;
let chunks = {
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
68u8 => {
match _input.read_byte()? {
65u8 => {
match _input.read_byte()? {
84u8 => {
1
},

byte if ((ByteSet::from_bits([0u64, 576460743846658046u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(14898840355839773829u64));
}
}
},

byte if ((ByteSet::from_bits([0u64, 576460743847706620u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(9453951600195794313u64));
}
}
},

69u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 576460743847706574u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(10036157788440812915u64));
}
}
},

byte if ((ByteSet::from_bits([0u64, 576460743847706110u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(6349531732377484771u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_png_chunk(_input, ihdr))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let idat = {
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
match _input.read_byte()? {
65u8 => {
match _input.read_byte()? {
84u8 => {
1
},

byte if ((ByteSet::from_bits([0u64, 576460743846658046u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(13785646910930464515u64));
}
}
},

byte if ((ByteSet::from_bits([0u64, 576460743847706620u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(5323644471994966730u64));
}
}
},

byte if ((ByteSet::from_bits([0u64, 576460743847706574u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(13278122992382147879u64));
}
}
},

byte if ((ByteSet::from_bits([0u64, 576460743847706110u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(18159646757349796721u64));
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
accum.push(next_elem)
}
};
accum
};
(try_flat_map_vec(xs.iter().cloned(), |x: png_idat| PResult::Ok(x.data.clone())))?
};
let mut buf_parser = Parser::new(slice_all(&idat));
let buf_input = &mut buf_parser;
(Decoder_zlib_main(buf_input))?
};
let more_chunks = {
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
68u8 => {
0
},

69u8 => {
match _input.read_byte()? {
78u8 => {
match _input.read_byte()? {
68u8 => {
_input.read_byte()?;
1
},

byte if ((ByteSet::from_bits([0u64, 576460743847706606u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(7124606020426797957u64));
}
}
},

byte if ((ByteSet::from_bits([0u64, 576460743847690238u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(15116592996336247086u64));
}
}
},

byte if ((ByteSet::from_bits([0u64, 576460743847706574u64, 0u64, 0u64])).contains(byte)) => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(10346499338674982396u64));
}
}
},

byte if ((ByteSet::from_bits([0u64, 576460743847706110u64, 0u64, 0u64])).contains(byte)) => {
0
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
let next_elem = (Decoder_png_chunk(_input, ihdr))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let iend = (Decoder_png_iend(_input))?;
PResult::Ok(png_main { signature, ihdr, chunks, idat, more_chunks, iend })
}

/// d#10
fn Decoder_riff_main(_input: &mut Parser<'_>) -> Result<riff_main, ParseError> {
let tag = {
let arg0 = {
let b = _input.read_byte()?;
if b == 82 {
b
} else {
return Err(ParseError::ExcludedBranch(14864597187136898256u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(15238960955167157760u64));
}
};
let arg2 = {
let b = _input.read_byte()?;
if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(17527274083228188873u64));
}
};
let arg3 = {
let b = _input.read_byte()?;
if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(17855530393917176367u64));
}
};
(arg0, arg1, arg2, arg3)
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
};
let data = {
let sz = length as usize;
_input.start_slice(sz)?;
let ret = (Decoder_riff_subchunks(_input))?;
_input.end_slice()?;
ret
};
let pad = if length % 2u32 == 1u32 {
let b = _input.read_byte()?;
Some(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(11054356281452530428u64));
})
} else {
None
};
PResult::Ok(riff_main { tag, length, data, pad })
}

/// d#11
fn Decoder_tiff_main(_input: &mut Parser<'_>) -> Result<tiff_main, ParseError> {
let start_of_header = {
let x = _input.get_offset_u64();
x as u32
};
let byte_order = (Decoder_tiff_byte_order(_input))?;
let magic = match byte_order {
tiff_byte_order::le(..) => {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
},

tiff_byte_order::be(..) => {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
}
};
let offset = match byte_order {
tiff_byte_order::le(..) => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
},

tiff_byte_order::be(..) => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
}
};
let ifd = {
let tgt_offset = start_of_header + offset;
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder_tiff_ifd_le(_input, byte_order))?;
_input.close_peek_context()?;
ret
};
PResult::Ok(tiff_main { start_of_header, byte_order, magic, offset, ifd })
}

/// d#12
fn Decoder_tar_main(_input: &mut Parser<'_>) -> Result<tar_main, ParseError> {
let contents = {
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
return Err(ParseError::ExcludedBranch(17406968167054271466u64));
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
accum.push(next_elem)
}
};
accum
};
{
let mut accum = Vec::new();
for _ in 0..1024u32 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(11100042044514704042u64));
}
};
accum.push(next_elem)
};
accum
};
{
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
return Err(ParseError::ExcludedBranch(5409189036752851054u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_main { contents })
}

/// d#13
fn Decoder_elf_main(_input: &mut Parser<'_>) -> Result<elf_main, ParseError> {
let header = (Decoder_elf_header(_input))?;
_input.get_offset_u64();
let program_headers = if !matches!(header.phoff, elf_types_elf_off::Off32(0u32) | elf_types_elf_off::Off64(0u64)) {
let tgt_offset = match header.phoff {
elf_types_elf_off::Off32(x32) => {
x32 as u64
},

elf_types_elf_off::Off64(x64) => {
x64
}
};
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder194(_input, header.ident.data == 2u8, header.ident.class, header.phnum))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
let section_headers = if !matches!(header.shoff, elf_types_elf_off::Off32(0u32) | elf_types_elf_off::Off64(0u64)) {
let tgt_offset = match header.shoff {
elf_types_elf_off::Off32(x32) => {
x32 as u64
},

elf_types_elf_off::Off64(x64) => {
x64
}
};
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder195(_input, header.ident.data == 2u8, header.ident.class, header.shnum))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
let sections = match section_headers {
Some(ref shdrs) => {
let mut accum = Vec::new();
for shdr in shdrs.clone() {
let next_elem = if (shdr.r#type != 8u32) && (shdr.r#type != 0u32) {
let tgt_offset = match shdr.offset {
elf_types_elf_off::Off32(x32) => {
x32 as u64
},

elf_types_elf_off::Off64(x64) => {
x64
}
};
let _is_advance = _input.advance_or_seek(tgt_offset)?;
let ret = (Decoder196(_input, shdr.r#type, match shdr.size {
elf_types_elf_full::Full32(x32) => {
x32 as u64
},

elf_types_elf_full::Full64(x64) => {
x64
}
}))?;
_input.close_peek_context()?;
Some(ret)
} else {
None
};
accum.push(next_elem)
};
Some(accum)
},

None => {
None
}
};
_input.skip_remainder();
PResult::Ok(elf_main { header, program_headers, section_headers, sections })
}

/// d#14
fn Decoder_opentype_main<'input>(_input: &mut Parser<'input>) -> Result<opentype_main<'input>, ParseError> {
let font_view = _input.view();
let magic = {
_input.open_peek_context();
let ret = ((|| {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
PResult::Ok(u32be(x))
})())?;
_input.close_peek_context()?;
ret
};
let directory = match magic {
65536u32 => {
let inner = (Decoder_opentype_table_directory(_input, font_view))?;
opentype_main_directory::TableDirectory(inner)
},

1330926671u32 => {
let inner = (Decoder_opentype_table_directory(_input, font_view))?;
opentype_main_directory::TableDirectory(inner)
},

1953784678u32 => {
let inner = (Decoder_opentype_ttc_header(_input, font_view))?;
opentype_main_directory::TTCHeader(inner)
},

1953658213u32 => {
let inner = (Decoder_opentype_table_directory(_input, font_view))?;
opentype_main_directory::TableDirectory(inner)
},

_ => {
return Err(ParseError::FailToken(4726315105662630465u64));
}
};
PResult::Ok(opentype_main { magic, directory })
}

/// d#15
fn Decoder_rle_main(_input: &mut Parser<'_>) -> Result<rle_main, ParseError> {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

1u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(11460567998186064482u64));
}
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10036638040555853769u64));
}
};
let inner = (Decoder_rle_old_style(_input))?;
rle_main::old_style(inner)
},

1 => {
{
let b = _input.read_byte()?;
if b == 1 {
b
} else {
return Err(ParseError::ExcludedBranch(13230337088401352826u64));
}
};
let inner = (Decoder_rle_new_style(_input))?;
rle_main::new_style(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(6223008304848233301u64));
}
})
}

/// d#16
fn Decoder16(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
Decoder17(_input)
}

/// d#17
fn Decoder17(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744073709551614u64, 18446744073709551615u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 4294967292u64])).contains(byte)) => {
0
},

224u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 35175782154240u64])).contains(byte)) => {
0
},

237u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 211106232532992u64])).contains(byte)) => {
0
},

240u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 3940649673949184u64])).contains(byte)) => {
0
},

244u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(14550754927305275517u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder18(_input))?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
}

/// d#18
fn Decoder18(_input: &mut Parser<'_>) -> Result<char, ParseError> {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744073709551614u64, 18446744073709551615u64, 0u64, 0u64])).contains(byte)) => {
1
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 4294967292u64])).contains(byte)) => {
1
},

224u8 => {
1
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 35175782154240u64])).contains(byte)) => {
1
},

237u8 => {
1
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 211106232532992u64])).contains(byte)) => {
1
},

240u8 => {
1
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 3940649673949184u64])).contains(byte)) => {
1
},

244u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(15631554783732883240u64));
}
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let _ = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(10197098993763395417u64));
}
};
(char::from_u32(0u32)).unwrap()
},

1 => {
(Decoder19(_input))?
},

_ => {
return Err(ParseError::ExcludedBranch(2391834656526534993u64));
}
})
}

/// d#19
fn Decoder19(_input: &mut Parser<'_>) -> Result<char, ParseError> {
let codepoint = {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([18446744073709551614u64, 18446744073709551615u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 4294967292u64])).contains(byte)) => {
1
},

224u8 => {
2
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 35175782154240u64])).contains(byte)) => {
2
},

237u8 => {
2
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 211106232532992u64])).contains(byte)) => {
2
},

240u8 => {
3
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 3940649673949184u64])).contains(byte)) => {
3
},

244u8 => {
3
},

_ => {
return Err(ParseError::ExcludedBranch(3852079030227774582u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let byte = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744073709551614u64, 18446744073709551615u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9422510723961972169u64));
}
};
byte as u32
},

1 => {
let tuple_var = {
let arg0 = {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 0u64, 0u64, 4294967292u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(10940017698627680568u64));
}
};
raw & 31u8
};
let arg1 = (Decoder20(_input))?;
(arg0, arg1)
};
{
let (x1, x0) = tuple_var;
(x1 as u32) << 6u32 | (x0 as u32)
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

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 35175782154240u64])).contains(byte)) => {
1
},

237u8 => {
2
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 211106232532992u64])).contains(byte)) => {
3
},

_ => {
return Err(ParseError::ExcludedBranch(15741082764016749161u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let arg0 = {
let raw = {
let b = _input.read_byte()?;
if b == 224 {
b
} else {
return Err(ParseError::ExcludedBranch(179268011689651936u64));
}
};
raw & 15u8
};
let arg1 = {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 0u64, 18446744069414584320u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9665974566873665536u64));
}
};
raw & 63u8
};
let arg2 = (Decoder20(_input))?;
(arg0, arg1, arg2)
},

1 => {
let arg0 = {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 0u64, 0u64, 35175782154240u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(374064178837027275u64));
}
};
raw & 15u8
};
let arg1 = (Decoder20(_input))?;
let arg2 = (Decoder20(_input))?;
(arg0, arg1, arg2)
},

2 => {
let arg0 = {
let raw = {
let b = _input.read_byte()?;
if b == 237 {
b
} else {
return Err(ParseError::ExcludedBranch(658824046370133753u64));
}
};
raw & 15u8
};
let arg1 = {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 0u64, 4294967295u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(3725673472712527969u64));
}
};
raw & 63u8
};
let arg2 = (Decoder20(_input))?;
(arg0, arg1, arg2)
},

3 => {
let arg0 = {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 0u64, 0u64, 211106232532992u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(12728843535195535635u64));
}
};
raw & 15u8
};
let arg1 = (Decoder20(_input))?;
let arg2 = (Decoder20(_input))?;
(arg0, arg1, arg2)
},

_ => {
return Err(ParseError::ExcludedBranch(9967703502401950260u64));
}
}
};
{
let (x2, x1, x0) = tuple_var;
(x2 as u32) << 12u32 | (x1 as u32) << 6u32 | (x0 as u32)
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

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 3940649673949184u64])).contains(byte)) => {
1
},

244u8 => {
2
},

_ => {
return Err(ParseError::ExcludedBranch(8986322043713516692u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
let arg0 = {
let raw = {
let b = _input.read_byte()?;
if b == 240 {
b
} else {
return Err(ParseError::ExcludedBranch(8376883036401934317u64));
}
};
raw & 7u8
};
let arg1 = {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 0u64, 18446744073709486080u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9069368457806005425u64));
}
};
raw & 63u8
};
let arg2 = (Decoder20(_input))?;
let arg3 = (Decoder20(_input))?;
(arg0, arg1, arg2, arg3)
},

1 => {
let arg0 = {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 0u64, 0u64, 3940649673949184u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(3179861450314844647u64));
}
};
raw & 7u8
};
let arg1 = (Decoder20(_input))?;
let arg2 = (Decoder20(_input))?;
let arg3 = (Decoder20(_input))?;
(arg0, arg1, arg2, arg3)
},

2 => {
let arg0 = {
let raw = {
let b = _input.read_byte()?;
if b == 244 {
b
} else {
return Err(ParseError::ExcludedBranch(15080388466336998873u64));
}
};
raw & 7u8
};
let arg1 = {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 0u64, 65535u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(6070260202873699214u64));
}
};
raw & 63u8
};
let arg2 = (Decoder20(_input))?;
let arg3 = (Decoder20(_input))?;
(arg0, arg1, arg2, arg3)
},

_ => {
return Err(ParseError::ExcludedBranch(11786939113783016634u64));
}
}
};
{
let (x3, x2, x1, x0) = tuple_var;
(x3 as u32) << 18u32 | (x2 as u32) << 12u32 | (x1 as u32) << 6u32 | (x0 as u32)
}
},

_ => {
return Err(ParseError::ExcludedBranch(5176232487486782188u64));
}
}
};
PResult::Ok((char::from_u32(codepoint)).unwrap())
}

/// d#20
fn Decoder20(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let raw = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 0u64, 18446744073709551615u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9220862562374507822u64));
}
};
PResult::Ok(raw & 63u8)
}

/// d#21
fn Decoder_rle_old_style(_input: &mut Parser<'_>) -> Result<rle_old_style, ParseError> {
let runs = {
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
let next_elem = (Decoder_rle_old_style_run(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let data = (try_flat_map_vec(runs.iter().cloned(), |run: rle_old_style_run| PResult::Ok(run.buf.clone())))?;
PResult::Ok(rle_old_style { runs, data })
}

/// d#22
fn Decoder_rle_new_style(_input: &mut Parser<'_>) -> Result<rle_new_style, ParseError> {
let _runs = {
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
let next_elem = (Decoder_rle_new_style_run(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let data = (try_flat_map_vec(_runs.iter().cloned(), |run: rle_new_style_run| PResult::Ok(run.buf.clone())))?;
PResult::Ok(rle_new_style { data })
}

/// d#23
fn Decoder_rle_new_style_run(_input: &mut Parser<'_>) -> Result<rle_new_style_run, ParseError> {
let _len = _input.read_byte()?;
let _char = _input.read_byte()?;
let buf = {
let mut accum = Vec::new();
for _ in 0.._len {
let next_elem = _char;
accum.push(next_elem)
};
accum
};
PResult::Ok(rle_new_style_run { _len, _char, buf })
}

/// d#24
fn Decoder_rle_old_style_run(_input: &mut Parser<'_>) -> Result<rle_old_style_run, ParseError> {
let len = _input.read_byte()?;
let char = _input.read_byte()?;
let buf = {
let mut accum = Vec::new();
for _ in 0..len {
let next_elem = char;
accum.push(next_elem)
};
accum
};
PResult::Ok(rle_old_style_run { len, char, buf })
}

/// d#25
fn Decoder_opentype_table_directory<'input>(_input: &mut Parser<'input>, font_view: View<'input>) -> Result<opentype_table_directory<'input>, ParseError> {
let sfnt_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let is_valid = {
let version = inner;
matches!(version, 65536u32 | 1330926671u32 | 1953658213u32)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8772793160380380086u64));
}
};
let num_tables = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let search_range = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let entry_selector = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let range_shift = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let table_records = {
let mut accum = Vec::new();
for _ in 0..num_tables {
let next_elem = (Decoder_opentype_table_record(_input))?;
accum.push(next_elem)
};
accum
};
let table_links = (Decoder_opentype_table_directory_table_links(_input, &table_records, font_view))?;
PResult::Ok(opentype_table_directory { sfnt_version, num_tables, search_range, entry_selector, range_shift, table_records, table_links })
}

/// d#26
fn Decoder_opentype_ttc_header<'input>(_input: &mut Parser<'input>, font_view: View<'input>) -> Result<opentype_ttc_header<'input>, ParseError> {
let ttc_tag = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let is_valid = {
let tag = inner;
tag == 1953784678u32
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2605623462625042002u64));
}
};
let major_version = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let minor_version = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let header = match major_version {
1u16 => {
let inner = {
let num_fonts = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let table_directories = {
let mut accum = Vec::new();
for _ in 0..num_fonts {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let data = if offset != 0u32 {
let mut view_parser = Parser::from(font_view.offset(offset as usize)?);
let view_input = &mut view_parser;
Some((Decoder_opentype_table_directory(view_input, font_view))?)
} else {
None
};
opentype_ttc_header_header_Version1_table_directories { offset, data }
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
let num_fonts = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let table_directories = {
let mut accum = Vec::new();
for _ in 0..num_fonts {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let data = if offset != 0u32 {
let mut view_parser = Parser::from(font_view.offset(offset as usize)?);
let view_input = &mut view_parser;
Some((Decoder_opentype_table_directory(view_input, font_view))?)
} else {
None
};
opentype_ttc_header_header_Version1_table_directories { offset, data }
};
accum.push(next_elem)
};
accum
};
let dsig_tag = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let dsig_length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let dsig_offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
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

/// d#27
fn Decoder_opentype_table_record(_input: &mut Parser<'_>) -> Result<opentype_table_record, ParseError> {
let table_id = (Decoder53(_input))?;
let checksum = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(opentype_table_record { table_id, checksum, offset, length })
}

/// d#28
fn Decoder_opentype_table_directory_table_links<'input>(_input: &mut Parser<'input>, tables: &[opentype_table_record], font_view: View<'input>) -> Result<opentype_table_directory_table_links<'input>, ParseError> {
let cmap = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1668112752u32, tables)).copied() {
Some(ref matching_table) => {
let mut view_parser = Parser::from(font_view.offset(matching_table.offset as usize)?);
let view_input = &mut view_parser;
let sz = matching_table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_cmap_table(view_input))?;
view_input.end_slice()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let head = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1751474532u32, tables)).copied() {
Some(ref matching_table) => {
let mut view_parser = Parser::from(font_view.offset(matching_table.offset as usize)?);
let view_input = &mut view_parser;
let sz = matching_table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_head_table(view_input))?;
view_input.end_slice()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let hhea = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1751672161u32, tables)).copied() {
Some(ref matching_table) => {
let mut view_parser = Parser::from(font_view.offset(matching_table.offset as usize)?);
let view_input = &mut view_parser;
let sz = matching_table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_hhea_table(view_input))?;
view_input.end_slice()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let maxp = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1835104368u32, tables)).copied() {
Some(ref matching_table) => {
let mut view_parser = Parser::from(font_view.offset(matching_table.offset as usize)?);
let view_input = &mut view_parser;
let sz = matching_table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_maxp_table(view_input))?;
view_input.end_slice()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let hmtx = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1752003704u32, tables)).copied() {
Some(ref matching_table) => {
let mut view_parser = Parser::from(font_view.offset(matching_table.offset as usize)?);
let view_input = &mut view_parser;
let sz = matching_table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_hmtx_table(view_input, hhea.number_of_long_metrics, maxp.num_glyphs))?;
view_input.end_slice()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let name = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1851878757u32, tables)).copied() {
Some(ref matching_table) => {
let mut view_parser = Parser::from(font_view.offset(matching_table.offset as usize)?);
let view_input = &mut view_parser;
let sz = matching_table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_name_table(view_input))?;
view_input.end_slice()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let os2 = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1330851634u32, tables)).copied() {
Some(ref matching_table) => {
let mut view_parser = Parser::from(font_view.offset(matching_table.offset as usize)?);
let view_input = &mut view_parser;
let table_len = matching_table.length;
let sz = table_len as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_os2_table(view_input, table_len))?;
view_input.end_slice()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let post = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1886352244u32, tables)).copied() {
Some(ref matching_table) => {
let mut view_parser = Parser::from(font_view.offset(matching_table.offset as usize)?);
let view_input = &mut view_parser;
let sz = matching_table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_post_table(view_input))?;
view_input.end_slice()?;
ret
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let cvt = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1668707360u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = ((|| {
let mut accum = Vec::new();
while view_input.remaining() > 0 {
let matching_ix = {
view_input.open_peek_context();
view_input.read_byte()?;
{
let ret = 0;
view_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let x = (view_input.read_byte()?, view_input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let fpgm = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1718642541u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = ((|| {
let mut accum = Vec::new();
while view_input.remaining() > 0 {
let matching_ix = {
view_input.open_peek_context();
view_input.read_byte()?;
{
let ret = 0;
view_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = view_input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let loca = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1819239265u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_loca_table(view_input, maxp.num_glyphs, head.index_to_loc_format))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let glyf = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1735162214u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_glyf_table(view_input, match loca {
Some(ref x) => {
x.offsets.clone()
},

None => {
opentype_loca_table_offsets::Offsets32([].to_vec())
}
}))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let prep = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1886545264u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = ((|| {
let mut accum = Vec::new();
while view_input.remaining() > 0 {
let matching_ix = {
view_input.open_peek_context();
view_input.read_byte()?;
{
let ret = 0;
view_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = view_input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let gasp = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1734439792u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_gasp_table(view_input))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let base = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1111577413u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_base_table(view_input))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let gdef = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1195656518u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_gdef_table(view_input))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let gpos = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1196445523u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_gpos_table(view_input))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let gsub = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1196643650u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_gsub_table(view_input))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let fvar = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1719034226u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_fvar_table(view_input))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let gvar = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1735811442u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_gvar_table(view_input))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let kern = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1801810542u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_kern_table(view_input))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let stat = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1398030676u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder_opentype_stat_table(view_input))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let vhea = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1986553185u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder48(view_input))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
let vmtx = match (find_by_key_unsorted(|elem: &opentype_table_record| elem.table_id, 1986884728u32, tables)).copied() {
Some(ref table) => {
let mut view_parser = Parser::from(font_view.offset(table.offset as usize)?);
let view_input = &mut view_parser;
let sz = table.length as usize;
view_input.start_slice(sz)?;
let ret = (Decoder49(view_input, match vhea {
Some(ref x) => {
x
},

_ => {
return Err(ParseError::ExcludedBranch(18164850183020044607u64));
}
}.number_of_long_metrics, maxp.num_glyphs))?;
view_input.end_slice()?;
Some(ret)
},

None => {
None
}
};
_input.skip_remainder();
PResult::Ok(opentype_table_directory_table_links { cmap, head, hhea, maxp, hmtx, name, os2, post, cvt, fpgm, loca, glyf, prep, gasp, base, gdef, gpos, gsub, fvar, gvar, kern, stat, vhea, vmtx })
}

/// d#29
fn Decoder_opentype_cmap_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_cmap_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let version = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let num_tables = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let encoding_records = {
let mut accum = Vec::new();
for _ in 0..num_tables {
let next_elem = (Decoder_opentype_encoding_record(_input, table_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_cmap_table { table_scope, version, num_tables, encoding_records })
}

/// d#30
fn Decoder_opentype_head_table(_input: &mut Parser<'_>) -> Result<opentype_head_table, ParseError> {
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10688770705819276010u64));
}
};
let minor_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10416240583538343445u64));
}
};
let font_revision = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
let checksum_adjustment = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let magic_number = {
let arg0 = {
let b = _input.read_byte()?;
if b == 95 {
b
} else {
return Err(ParseError::ExcludedBranch(1457499133218925748u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 15 {
b
} else {
return Err(ParseError::ExcludedBranch(14215639860155940137u64));
}
};
let arg2 = {
let b = _input.read_byte()?;
if b == 60 {
b
} else {
return Err(ParseError::ExcludedBranch(5584166819955891466u64));
}
};
let arg3 = {
let b = _input.read_byte()?;
if b == 245 {
b
} else {
return Err(ParseError::ExcludedBranch(11133239979815295357u64));
}
};
(arg0, arg1, arg2, arg3)
};
let flags = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let units_per_em = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
matches!(x, 16u16..=16384u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1275286460638129217u64));
}
};
let created = (Decoder179(_input))?;
let modified = (Decoder179(_input))?;
let glyph_extents = {
let x_min = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_min = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let x_max = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_max = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_head_table_glyph_extents { x_min, y_min, x_max, y_max }
};
let mac_style = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let extended = _packed_bits >> 6u16 & 1u16 > 0u16;
let condensed = _packed_bits >> 5u16 & 1u16 > 0u16;
let shadow = _packed_bits >> 4u16 & 1u16 > 0u16;
let outline = _packed_bits >> 3u16 & 1u16 > 0u16;
let underline = _packed_bits >> 2u16 & 1u16 > 0u16;
let italic = _packed_bits >> 1u16 & 1u16 > 0u16;
let bold = _packed_bits & 1u16 > 0u16;
opentype_head_table_mac_style { extended, condensed, shadow, outline, underline, italic, bold }
};
let lowest_rec_ppem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let font_direction_hint = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let index_to_loc_format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x <= 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(386759067598651566u64));
}
};
let glyph_data_format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13527164188224560282u64));
}
};
PResult::Ok(opentype_head_table { major_version, minor_version, font_revision, checksum_adjustment, magic_number, flags, units_per_em, created, modified, glyph_extents, mac_style, lowest_rec_ppem, font_direction_hint, index_to_loc_format, glyph_data_format })
}

/// d#31
fn Decoder_opentype_hhea_table(_input: &mut Parser<'_>) -> Result<opentype_hhea_table, ParseError> {
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(240888096670347429u64));
}
};
let minor_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
matches!(x, 0u16 | 4096u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11936787736236307191u64));
}
};
let ascent = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let descent = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let line_gap = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let advance_width_max = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let min_left_side_bearing = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let min_right_side_bearing = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let x_max_extent = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let caret_slope = {
let rise = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let run = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_hhea_table_caret_slope { rise, run }
};
let caret_offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
{
let arg0 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(5215619712890029856u64));
}
};
let arg1 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3167775832820164678u64));
}
};
let arg2 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7215050775822222282u64));
}
};
let arg3 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3743786174148899814u64));
}
};
(arg0, arg1, arg2, arg3)
};
let metric_data_format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12652804269632162478u64));
}
};
let number_of_long_metrics = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
PResult::Ok(opentype_hhea_table { major_version, minor_version, ascent, descent, line_gap, advance_width_max, min_left_side_bearing, min_right_side_bearing, x_max_extent, caret_slope, caret_offset, metric_data_format, number_of_long_metrics })
}

/// d#32
fn Decoder_opentype_maxp_table(_input: &mut Parser<'_>) -> Result<opentype_maxp_table, ParseError> {
let version = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let num_glyphs = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = match version {
65536u32 => {
let inner = (Decoder_opentype_maxp_version1(_input))?;
opentype_maxp_table_data::MaxpV1(inner)
},

20480u32 => {
opentype_maxp_table_data::MaxpPostScript
},

unknown => {
let inner = unknown;
opentype_maxp_table_data::MaxpUnknown(inner)
}
};
PResult::Ok(opentype_maxp_table { version, num_glyphs, data })
}

/// d#33
fn Decoder_opentype_hmtx_table(_input: &mut Parser<'_>, num_long_metrics: u16, num_glyphs: u16) -> Result<opentype_hmtx_table, ParseError> {
let long_metrics = {
let mut accum = Vec::new();
for _ in 0..num_long_metrics {
let next_elem = {
let advance_width = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let left_side_bearing = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_hmtx_table_long_metrics { advance_width, left_side_bearing }
};
accum.push(next_elem)
};
accum
};
let left_side_bearings = {
let mut accum = Vec::new();
for _ in 0..try_sub!(num_glyphs, num_long_metrics, 2206609067086327257u64) {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_hmtx_table { long_metrics, left_side_bearings })
}

/// d#34
fn Decoder_opentype_name_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_name_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let version = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let name_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let storage_offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let name_records = {
let mut accum = Vec::new();
for _ in 0..name_count {
let next_elem = (Decoder_opentype_name_name_record(_input, table_view.offset(storage_offset as usize)?))?;
accum.push(next_elem)
};
accum
};
let data = match version {
0u16 => {
opentype_name_table_data::NameVersion0
},

1u16 => {
let inner = (Decoder_opentype_name_name_version_1(_input, table_view.offset(storage_offset as usize)?))?;
opentype_name_table_data::NameVersion1(inner)
},

unknown => {
let inner = unknown;
opentype_name_table_data::NameVersionUnknown(inner)
}
};
PResult::Ok(opentype_name_table { table_scope, version, name_count, storage_offset, name_records, data })
}

/// d#35
fn Decoder_opentype_os2_table(_input: &mut Parser<'_>, table_length: u32) -> Result<opentype_os2_table, ParseError> {
let version = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let x_avg_char_width = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let us_weight_class = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let us_width_class = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let fs_type = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_subscript_x_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_subscript_y_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_subscript_x_offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_subscript_y_offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_superscript_x_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_superscript_y_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_superscript_x_offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_superscript_y_offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_strikeout_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_strikeout_position = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let s_family_class = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let panose = {
let mut accum = Vec::new();
for _ in 0..10u8 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
let ul_unicode_range1 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let ul_unicode_range2 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let ul_unicode_range3 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let ul_unicode_range4 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let ach_vend_id = (Decoder53(_input))?;
let fs_selection = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let us_first_char_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let us_last_char_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = if (version > 0u16) || (table_length >= 78u32) {
let s_typo_ascender = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let s_typo_descender = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let s_typo_line_gap = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let us_win_ascent = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let us_win_descent = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let extra_fields_v1 = if matches!(version, 1u16..) {
let ul_code_page_range_1 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let ul_code_page_range_2 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let extra_fields_v2 = if matches!(version, 2u16..) {
let sx_height = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let s_cap_height = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let us_default_char = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let us_break_char = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let us_max_context = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let extra_fields_v5 = if matches!(version, 5u16..) {
let us_lower_optical_point_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let us_upper_optical_point_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
Some(opentype_os2_table_data_extra_fields_v1_extra_fields_v2_extra_fields_v5 { us_lower_optical_point_size, us_upper_optical_point_size })
} else {
None
};
Some(opentype_os2_table_data_extra_fields_v1_extra_fields_v2 { sx_height, s_cap_height, us_default_char, us_break_char, us_max_context, extra_fields_v5 })
} else {
None
};
Some(opentype_os2_table_data_extra_fields_v1 { ul_code_page_range_1, ul_code_page_range_2, extra_fields_v2 })
} else {
None
};
Some(opentype_os2_table_data { s_typo_ascender, s_typo_descender, s_typo_line_gap, us_win_ascent, us_win_descent, extra_fields_v1 })
} else {
None
};
PResult::Ok(opentype_os2_table { version, x_avg_char_width, us_weight_class, us_width_class, fs_type, y_subscript_x_size, y_subscript_y_size, y_subscript_x_offset, y_subscript_y_offset, y_superscript_x_size, y_superscript_y_size, y_superscript_x_offset, y_superscript_y_offset, y_strikeout_size, y_strikeout_position, s_family_class, panose, ul_unicode_range1, ul_unicode_range2, ul_unicode_range3, ul_unicode_range4, ach_vend_id, fs_selection, us_first_char_index, us_last_char_index, data })
}

/// d#36
fn Decoder_opentype_post_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_post_table<'input>, ParseError> {
let version = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let italic_angle = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
let underline_position = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let underline_thickness = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_fixed_pitch = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let min_mem_type42 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let max_mem_type42 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let min_mem_type1 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let max_mem_type1 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let names = match version {
65536u32 => {
opentype_post_table_names::Version1
},

131072u32 => {
let inner = (Decoder_opentype_post_version2(_input))?;
opentype_post_table_names::Version2(inner)
},

151552u32 => {
let inner = {
let num_glyphs = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let offset = {
let mut accum = Vec::new();
for _ in 0..num_glyphs {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
opentype_post_table_names_Version2Dot5 { num_glyphs, offset }
};
opentype_post_table_names::Version2Dot5(inner)
},

196608u32 => {
opentype_post_table_names::Version3
},

unknown => {
let inner = unknown;
opentype_post_table_names::VersionUnknown(inner)
}
};
PResult::Ok(opentype_post_table { version, italic_angle, underline_position, underline_thickness, is_fixed_pitch, min_mem_type42, max_mem_type42, min_mem_type1, max_mem_type1, names })
}

/// d#37
fn Decoder_opentype_loca_table(_input: &mut Parser<'_>, num_glyphs: u16, index_to_loc_format: u16) -> Result<opentype_loca_table, ParseError> {
let offsets = match index_to_loc_format {
0u16 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..succ(num_glyphs) {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
opentype_loca_table_offsets::Offsets16(inner)
},

1u16 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..succ(num_glyphs) {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
accum.push(next_elem)
};
accum
};
opentype_loca_table_offsets::Offsets32(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(opentype_loca_table { offsets })
}

/// d#38
fn Decoder_opentype_glyf_table<'input>(_input: &mut Parser<'input>, offsets: opentype_loca_table_offsets) -> Result<opentype_glyf_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let glyphs = match offsets {
opentype_loca_table_offsets::Offsets16(ref half16s) => {
let len = pred((half16s.len()) as u32);
let mut accum = Vec::new();
for ix in 0u32..len {
let next_elem = {
let (this_offs, next_offs) = ((half16s[ix as usize] as u32) * 2u32, (half16s[(succ(ix)) as usize] as u32) * 2u32);
match next_offs > this_offs {
true => {
let inner = {
let offset = this_offs;
let _data = std::marker::PhantomData;
opentype_glyf_table_glyphs_Glyph { offset, _data }
};
opentype_glyf_table_glyphs::Glyph(inner)
},

false => {
opentype_glyf_table_glyphs::EmptyGlyph
}
}
};
accum.push(next_elem)
};
accum
},

opentype_loca_table_offsets::Offsets32(ref off32s) => {
let len = pred((off32s.len()) as u32);
let mut accum = Vec::new();
for ix in 0u32..len {
let next_elem = {
let (this_offs, next_offs) = (off32s[ix as usize], off32s[(succ(ix)) as usize]);
match next_offs > this_offs {
true => {
let inner = {
let offset = this_offs;
let _data = std::marker::PhantomData;
opentype_glyf_table_glyphs_Glyph { offset, _data }
};
opentype_glyf_table_glyphs::Glyph(inner)
},

false => {
opentype_glyf_table_glyphs::EmptyGlyph
}
}
};
accum.push(next_elem)
};
accum
}
};
PResult::Ok(opentype_glyf_table { table_scope, glyphs })
}

/// d#39
fn Decoder_opentype_gasp_table(_input: &mut Parser<'_>) -> Result<opentype_gasp_table, ParseError> {
let version = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let num_ranges = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let gasp_ranges = {
let mut accum = Vec::new();
for _ in 0..num_ranges {
let next_elem = {
let range_max_ppem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let range_gasp_behavior = match version {
0u16 => {
let inner = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let dogray = _packed_bits >> 1u16 & 1u16 > 0u16;
let gridfit = _packed_bits & 1u16 > 0u16;
opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version0 { dogray, gridfit }
};
opentype_gasp_table_gasp_ranges_range_gasp_behavior::Version0(inner)
},

1u16 => {
let inner = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let symmetric_smoothing = _packed_bits >> 3u16 & 1u16 > 0u16;
let symmetric_gridfit = _packed_bits >> 2u16 & 1u16 > 0u16;
let dogray = _packed_bits >> 1u16 & 1u16 > 0u16;
let gridfit = _packed_bits & 1u16 > 0u16;
opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version1 { symmetric_smoothing, symmetric_gridfit, dogray, gridfit }
};
opentype_gasp_table_gasp_ranges_range_gasp_behavior::Version1(inner)
},

_ => {
return Err(ParseError::FailToken(18134882366868794706u64));
}
};
opentype_gasp_table_gasp_ranges { range_max_ppem, range_gasp_behavior }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gasp_table { version, num_ranges, gasp_ranges })
}

/// d#40
fn Decoder_opentype_base_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_base_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7155653122005708978u64));
}
};
let minor_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x <= 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17920584887603040596u64));
}
};
let horiz_axis_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_base_table_horiz_axis_offset { offset, _data }
};
let vert_axis_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_base_table_horiz_axis_offset { offset, _data }
};
let item_var_store_offset = if minor_version > 0u16 {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_base_table_item_var_store_offset { offset, _data })
} else {
None
};
PResult::Ok(opentype_base_table { table_scope, major_version, minor_version, horiz_axis_offset, vert_axis_offset, item_var_store_offset })
}

/// d#41
fn Decoder_opentype_gdef_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_gdef_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(5673845796627816005u64));
}
};
let minor_version = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let glyph_class_def = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_glyph_class_def { offset, _data }
};
let attach_list = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_attach_list { offset, _data }
};
let lig_caret_list = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_lig_caret_list { offset, _data }
};
let mark_attach_class_def = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_glyph_class_def { offset, _data }
};
let data = match minor_version {
0u16 => {
opentype_gdef_table_data::Version1_0
},

1u16 => {
return Err(ParseError::FailToken(14591018267292443527u64));
},

2u16 => {
let inner = {
let mark_glyph_sets_def = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_data_Version1_2_mark_glyph_sets_def { offset, _data }
};
opentype_gdef_table_data_Version1_2 { mark_glyph_sets_def }
};
opentype_gdef_table_data::Version1_2(inner)
},

3u16 => {
let inner = {
let mark_glyph_sets_def = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_data_Version1_2_mark_glyph_sets_def { offset, _data }
};
let item_var_store = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_base_table_item_var_store_offset { offset, _data }
};
opentype_gdef_table_data_Version1_3 { mark_glyph_sets_def, item_var_store }
};
opentype_gdef_table_data::Version1_3(inner)
},

_ => {
let inner = {
let mark_glyph_sets_def = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_data_Version1_2_mark_glyph_sets_def { offset, _data }
};
let item_var_store = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_base_table_item_var_store_offset { offset, _data }
};
opentype_gdef_table_data_Version1_3 { mark_glyph_sets_def, item_var_store }
};
opentype_gdef_table_data::Version1_3(inner)
}
};
PResult::Ok(opentype_gdef_table { table_scope, major_version, minor_version, glyph_class_def, attach_list, lig_caret_list, mark_attach_class_def, data })
}

/// d#42
fn Decoder_opentype_gpos_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_gpos_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4762692522317026931u64));
}
};
let minor_version = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let script_list = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gpos_table_script_list { offset, _data }
};
let feature_list = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gpos_table_feature_list { offset, _data }
};
let lookup_list = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gpos_table_lookup_list { offset, _data }
};
let feature_variations_offset = if minor_version > 0u16 {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gpos_table_feature_variations_offset { offset, _data })
} else {
None
};
PResult::Ok(opentype_gpos_table { table_scope, major_version, minor_version, script_list, feature_list, lookup_list, feature_variations_offset })
}

/// d#43
fn Decoder_opentype_gsub_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_gsub_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9630069758457681762u64));
}
};
let minor_version = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let script_list = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gpos_table_script_list { offset, _data }
};
let feature_list = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gpos_table_feature_list { offset, _data }
};
let lookup_list = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gsub_table_lookup_list { offset, _data }
};
let feature_variations_offset = if minor_version > 0u16 {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gpos_table_feature_variations_offset { offset, _data })
} else {
None
};
PResult::Ok(opentype_gsub_table { table_scope, major_version, minor_version, script_list, feature_list, lookup_list, feature_variations_offset })
}

/// d#44
fn Decoder_opentype_fvar_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_fvar_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(908377722732597655u64));
}
};
let minor_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3203034260088513018u64));
}
};
let offset_axes = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let raw = inner;
raw > 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14677505873656710393u64));
}
};
{
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 2u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10102114574336663273u64));
}
};
let axis_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let axis_size = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 20u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4386762582485017400u64));
}
};
let instance_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let instance_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _axes_length = axis_count * axis_size;
let _axes = std::marker::PhantomData;
let offset_instances = offset_axes + _axes_length;
let _instances = std::marker::PhantomData;
PResult::Ok(opentype_fvar_table { table_scope, major_version, minor_version, offset_axes, axis_count, axis_size, instance_count, instance_size, _axes, offset_instances, _instances })
}

/// d#45
fn Decoder_opentype_gvar_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_gvar_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8893850231119365992u64));
}
};
let minor_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7659860344311718435u64));
}
};
let axis_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let shared_tuple_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let shared_tuples = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_gvar_table_shared_tuples { offset, _data }
};
let glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let flags = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_long_offset = _packed_bits & 1u16 > 0u16;
opentype_gvar_table_flags { is_long_offset }
};
let glyph_variation_data_array_offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let glyph_variation_data_offsets = match flags.is_long_offset {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..succ(glyph_count) {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
accum.push(next_elem)
};
accum
};
opentype_loca_table_offsets::Offsets32(inner)
},

false => {
let inner = {
let mut accum = Vec::new();
for _ in 0..succ(glyph_count) {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
opentype_loca_table_offsets::Offsets16(inner)
}
};
let _glyph_variation_data_array = std::marker::PhantomData;
PResult::Ok(opentype_gvar_table { table_scope, major_version, minor_version, axis_count, shared_tuple_count, shared_tuples, glyph_count, flags, glyph_variation_data_array_offset, glyph_variation_data_offsets, _glyph_variation_data_array })
}

/// d#46
fn Decoder_opentype_kern_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_kern_table<'input>, ParseError> {
let version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11052099086134529863u64));
}
};
let n_tables = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let subtables = {
let mut accum = Vec::new();
for _ in 0..n_tables {
let next_elem = (Decoder_opentype_kern_kern_subtable(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_kern_table { version, n_tables, subtables })
}

/// d#47
fn Decoder_opentype_stat_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_stat_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1079884235207081886u64));
}
};
let minor_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
matches!(x, 1u16 | 2u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(980800817911480223u64));
}
};
let design_axis_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let design_axis_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let design_axes = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_stat_table_design_axes { offset, _data }
};
let axis_value_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let axis_value_offsets = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_stat_table_axis_value_offsets { offset, _data }
};
let elided_fallback_name_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
PResult::Ok(opentype_stat_table { table_scope, major_version, minor_version, design_axis_size, design_axis_count, design_axes, axis_value_count, axis_value_offsets, elided_fallback_name_id })
}

/// d#48
fn Decoder48(_input: &mut Parser<'_>) -> Result<opentype_hhea_table, ParseError> {
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9042484249406774160u64));
}
};
let minor_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
matches!(x, 0u16 | 4096u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7801539417877429212u64));
}
};
let ascent = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let descent = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let line_gap = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let advance_width_max = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let min_left_side_bearing = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let min_right_side_bearing = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let x_max_extent = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let caret_slope = {
let rise = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let run = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_hhea_table_caret_slope { rise, run }
};
let caret_offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
{
let arg0 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14931240509007516758u64));
}
};
let arg1 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11328034188734904930u64));
}
};
let arg2 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1338347005175300217u64));
}
};
let arg3 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(15432825464810477099u64));
}
};
(arg0, arg1, arg2, arg3)
};
let metric_data_format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8987822076696059625u64));
}
};
let number_of_long_metrics = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
PResult::Ok(opentype_hhea_table { major_version, minor_version, ascent, descent, line_gap, advance_width_max, min_left_side_bearing, min_right_side_bearing, x_max_extent, caret_slope, caret_offset, metric_data_format, number_of_long_metrics })
}

/// d#49
fn Decoder49(_input: &mut Parser<'_>, num_long_metrics: u16, num_glyphs: u16) -> Result<opentype_hmtx_table, ParseError> {
let long_metrics = {
let mut accum = Vec::new();
for _ in 0..num_long_metrics {
let next_elem = {
let advance_width = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let left_side_bearing = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_hmtx_table_long_metrics { advance_width, left_side_bearing }
};
accum.push(next_elem)
};
accum
};
let left_side_bearings = {
let mut accum = Vec::new();
for _ in 0..try_sub!(num_glyphs, num_long_metrics, 11876854719037224982u64) {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_hmtx_table { long_metrics, left_side_bearings })
}

/// d#50
fn Decoder_opentype_stat_design_axes_array(_input: &mut Parser<'_>, design_axis_count: u16) -> Result<opentype_stat_design_axes_array, ParseError> {
let design_axes = {
let mut accum = Vec::new();
for _ in 0..design_axis_count {
let next_elem = {
let axis_tag = (Decoder53(_input))?;
let axis_name_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let axis_ordering = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_stat_design_axes_array_design_axes { axis_tag, axis_name_id, axis_ordering }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_stat_design_axes_array { design_axes })
}

/// d#51
fn Decoder_opentype_stat_axis_value_array<'input>(_input: &mut Parser<'input>, axis_value_count: u16) -> Result<opentype_stat_axis_value_array<'input>, ParseError> {
let array_view = _input.view();
let array_scope = array_view;
let axis_values = {
let mut accum = Vec::new();
for _ in 0..axis_value_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_stat_axis_value_array_axis_values { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_stat_axis_value_array { array_scope, axis_values })
}

/// d#52
fn Decoder_opentype_stat_axis_value_table(_input: &mut Parser<'_>) -> Result<opentype_stat_axis_value_table, ParseError> {
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
matches!(x, 1u16..=4u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10078755145706786000u64));
}
};
let data = match format {
1u16 => {
let inner = {
let axis_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let flags = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let elidable_axis_value_name = _packed_bits >> 1u16 & 1u16 > 0u16;
let older_sibling_font_attribute = _packed_bits & 1u16 > 0u16;
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name, older_sibling_font_attribute }
};
let value_name_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let value = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
opentype_stat_axis_value_table_data_Format1 { axis_index, flags, value_name_id, value }
};
opentype_stat_axis_value_table_data::Format1(inner)
},

2u16 => {
let inner = {
let axis_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let flags = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let elidable_axis_value_name = _packed_bits >> 1u16 & 1u16 > 0u16;
let older_sibling_font_attribute = _packed_bits & 1u16 > 0u16;
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name, older_sibling_font_attribute }
};
let value_name_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let nominal_value = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
let range_min_value = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
let range_max_value = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
opentype_stat_axis_value_table_data_Format2 { axis_index, flags, value_name_id, nominal_value, range_min_value, range_max_value }
};
opentype_stat_axis_value_table_data::Format2(inner)
},

3u16 => {
let inner = {
let axis_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let flags = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let elidable_axis_value_name = _packed_bits >> 1u16 & 1u16 > 0u16;
let older_sibling_font_attribute = _packed_bits & 1u16 > 0u16;
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name, older_sibling_font_attribute }
};
let value_name_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let value = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
let linked_value = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
opentype_stat_axis_value_table_data_Format3 { axis_index, flags, value_name_id, value, linked_value }
};
opentype_stat_axis_value_table_data::Format3(inner)
},

4u16 => {
let inner = {
let axis_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let flags = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let elidable_axis_value_name = _packed_bits >> 1u16 & 1u16 > 0u16;
let older_sibling_font_attribute = _packed_bits & 1u16 > 0u16;
opentype_stat_axis_value_table_data_Format1_flags { elidable_axis_value_name, older_sibling_font_attribute }
};
let value_name_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let axis_values = {
let mut accum = Vec::new();
for _ in 0..axis_count {
let next_elem = {
let axis_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let value = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
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

/// d#53
fn Decoder53(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
PResult::Ok(u32be(x))
}

/// d#54
fn Decoder_opentype_kern_kern_subtable<'input>(_input: &mut Parser<'input>) -> Result<opentype_kern_kern_subtable<'input>, ParseError> {
let version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1977899765720151190u64));
}
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let coverage = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let format = _packed_bits >> 8u16 & 255u16;
let r#override = _packed_bits >> 3u16 & 1u16 > 0u16;
let cross_stream = _packed_bits >> 2u16 & 1u16 > 0u16;
let minimum = _packed_bits >> 1u16 & 1u16 > 0u16;
let horizontal = _packed_bits & 1u16 > 0u16;
opentype_kern_kern_subtable_coverage { format, r#override, cross_stream, minimum, horizontal }
};
let data = match coverage.format {
0u16 => {
let inner = (Decoder_opentype_kern_subtable_format0(_input))?;
opentype_kern_kern_subtable_data::Format0(inner)
},

2u16 => {
let inner = (Decoder_opentype_kern_subtable_format2(_input))?;
opentype_kern_kern_subtable_data::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(7343323033370781545u64));
}
};
PResult::Ok(opentype_kern_kern_subtable { version, length, coverage, data })
}

/// d#55
fn Decoder_opentype_kern_subtable_format0(_input: &mut Parser<'_>) -> Result<opentype_kern_subtable_format0, ParseError> {
let n_pairs = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let search_range = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let entry_selector = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let range_shift = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let kern_pairs = {
let mut accum = Vec::new();
for _ in 0..n_pairs {
let next_elem = {
let left = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let right = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let value = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_kern_subtable_format0_kern_pairs { left, right, value }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_kern_subtable_format0 { n_pairs, search_range, entry_selector, range_shift, kern_pairs })
}

/// d#56
fn Decoder_opentype_kern_subtable_format2<'input>(_input: &mut Parser<'input>) -> Result<opentype_kern_subtable_format2<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let row_width = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let left_class_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_kern_subtable_format2_left_class_offset { offset, _data }
};
let right_class_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_kern_subtable_format2_left_class_offset { offset, _data }
};
let kerning_array_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_kern_subtable_format2_kerning_array_offset { offset, _data }
};
PResult::Ok(opentype_kern_subtable_format2 { table_scope, row_width, left_class_offset, right_class_offset, kerning_array_offset })
}

/// d#57
fn Decoder_opentype_kern_class_table(_input: &mut Parser<'_>) -> Result<opentype_kern_class_table, ParseError> {
let first_glyph = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let n_glyphs = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let class_values = {
let mut accum = Vec::new();
for _ in 0..n_glyphs {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_kern_class_table { first_glyph, n_glyphs, class_values })
}

/// d#58
fn Decoder_opentype_kern_kerning_array(_input: &mut Parser<'_>, left_glyph_count: u16, right_glyph_count: u16) -> Result<opentype_kern_kerning_array, ParseError> {
let left_glyph_count = left_glyph_count;
let right_glyph_count = right_glyph_count;
let kerning_values = {
let mut accum = Vec::new();
for _ in 0..left_glyph_count {
let next_elem = {
let mut accum = Vec::new();
for _ in 0..right_glyph_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_kern_kerning_array { left_glyph_count, right_glyph_count, kerning_values })
}

/// d#59
fn Decoder_opentype_gvar_tuple_record(_input: &mut Parser<'_>, axis_count: u16) -> Result<opentype_gvar_tuple_record, ParseError> {
let coordinates = {
let mut accum = Vec::new();
for _ in 0..axis_count {
let next_elem = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gvar_tuple_record { coordinates })
}

/// d#60
fn Decoder_opentype_gvar_glyph_variation_data<'input>(_input: &mut Parser<'input>, axis_count: u16) -> Result<opentype_gvar_glyph_variation_data<'input>, ParseError> {
let data_view = _input.view();
let data_scope = data_view;
let tuple_variation_count = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let shared_point_numbers = _packed_bits >> 15u16 & 1u16 > 0u16;
let tuple_count = _packed_bits & 4095u16;
opentype_gvar_glyph_variation_data_tuple_variation_count { shared_point_numbers, tuple_count }
};
let data_offset = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x != 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12890902517277365935u64));
}
};
let tuple_variation_headers = {
let mut accum = Vec::new();
for _ in 0..tuple_variation_count.tuple_count {
let next_elem = (Decoder_opentype_gvar_tuple_variation_header(_input, axis_count))?;
accum.push(next_elem)
};
accum
};
let _data = std::marker::PhantomData;
PResult::Ok(opentype_gvar_glyph_variation_data { data_scope, tuple_variation_count, data_offset, tuple_variation_headers, _data })
}

/// d#61
fn Decoder_opentype_gvar_tuple_variation_header(_input: &mut Parser<'_>, axis_count: u16) -> Result<opentype_gvar_tuple_variation_header, ParseError> {
let variation_data_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let tuple_index = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let embedded_peak_tuple = _packed_bits >> 15u16 & 1u16 > 0u16;
let intermediate_region = _packed_bits >> 14u16 & 1u16 > 0u16;
let private_point_numbers = _packed_bits >> 13u16 & 1u16 > 0u16;
let tuple_index = _packed_bits & 4095u16;
opentype_gvar_tuple_variation_header_tuple_index { embedded_peak_tuple, intermediate_region, private_point_numbers, tuple_index }
};
let peak_tuple = if tuple_index.embedded_peak_tuple {
Some((Decoder_opentype_gvar_tuple_record(_input, axis_count))?)
} else {
None
};
let intermediate_tuples = if tuple_index.intermediate_region {
let start_tuple = (Decoder_opentype_gvar_tuple_record(_input, axis_count))?;
let end_tuple = (Decoder_opentype_gvar_tuple_record(_input, axis_count))?;
Some(opentype_gvar_tuple_variation_header_intermediate_tuples { start_tuple, end_tuple })
} else {
None
};
PResult::Ok(opentype_gvar_tuple_variation_header { variation_data_size, tuple_index, peak_tuple, intermediate_tuples })
}

/// d#62
fn Decoder_opentype_gvar_serialized_data(_input: &mut Parser<'_>, shared_point_numbers: bool, tuple_var_headers: &[opentype_gvar_tuple_variation_header]) -> Result<opentype_gvar_serialized_data, ParseError> {
let shared_point_numbers = if shared_point_numbers {
Some((Decoder63(_input))?)
} else {
None
};
let per_tuple_variation_data = {
let mut accum = Vec::new();
for header in tuple_var_headers.clone() {
let next_elem = {
let sz = header.variation_data_size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let private_point_numbers = if header.tuple_index.private_point_numbers {
Some((Decoder63(_input))?)
} else {
None
};
let x_and_y_coordinate_deltas = {
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
return Err(ParseError::ExcludedBranch(18270091135093349626u64));
}
}
}
}.0;
let mut seq: Vec<opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas> = Vec::new();
let mut acc = 0u16;
loop {
{
let tmp_cond = {
let totlen = acc;
totlen >= point_count * 2u16
};
if tmp_cond {
break
};

};
let elem = {
let control = {
let _packed_bits = _input.read_byte()?;
let deltas_are_zero = _packed_bits >> 7u8 & 1u8 > 0u8;
let deltas_are_words = _packed_bits >> 6u8 & 1u8 > 0u8;
let delta_run_count = _packed_bits & 63u8;
opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas_control { deltas_are_zero, deltas_are_words, delta_run_count }
};
let deltas = {
let run_length = succ(control.delta_run_count);
match control.deltas_are_zero {
true => {
let inner = run_length;
opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas::Delta0(inner)
},

false => {
match control.deltas_are_words {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas::Delta16(inner)
},

false => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas::Delta8(inner)
}
}
}
}
};
opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas { control, deltas }
};
acc = {
let acc = acc;
let run = elem.clone();
acc + (succ(run.control.delta_run_count as u16))
};
seq.push(elem)
};
(acc, seq)
};
PResult::Ok(opentype_gvar_serialized_data_per_tuple_variation_data { private_point_numbers, x_and_y_coordinate_deltas })
})())?;
_input.end_slice()?;
ret
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gvar_serialized_data { shared_point_numbers, per_tuple_variation_data })
}

/// d#63
fn Decoder63(_input: &mut Parser<'_>) -> Result<(u16, Vec<opentype_var_packed_point_numbers_runs>), ParseError> {
let tree_index = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744073709551614u64, 18446744073709551615u64, 0u64, 0u64])).contains(byte)) => {
1
},

byte if ((ByteSet::from_bits([0u64, 0u64, 18446744073709551615u64, 18446744073709551615u64])).contains(byte)) => {
2
},

_ => {
return Err(ParseError::ExcludedBranch(2879885114680241844u64));
}
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let _ = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(13049534979177835905u64));
}
};
(0u16, [].to_vec())
},

1 => {
let point_count = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744073709551614u64, 18446744073709551615u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9011855507994367971u64));
}
};
(Decoder64(_input, point_count as u16))?
},

2 => {
let hi = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 0u64, 18446744073709551615u64, 18446744073709551615u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(14796083725261108356u64));
}
};
let lo = _input.read_byte()?;
(Decoder64(_input, ((hi as u16) & 127u16) << 8u16 | (lo as u16)))?
},

_ => {
return Err(ParseError::ExcludedBranch(14009314771729697611u64));
}
})
}

/// d#64
fn Decoder64(_input: &mut Parser<'_>, point_count: u16) -> Result<(u16, Vec<opentype_var_packed_point_numbers_runs>), ParseError> {
let mut seq: Vec<opentype_var_packed_point_numbers_runs> = Vec::new();
let mut acc = 0u16;
loop {
{
let tmp_cond = {
let totlen = acc;
totlen >= point_count
};
if tmp_cond {
break
};

};
let elem = (Decoder_opentype_var_packed_point_numbers_runs(_input))?;
acc = {
let acc = acc;
let run = elem.clone();
acc + (succ(run.control.point_run_count as u16))
};
seq.push(elem)
};
PResult::Ok((acc, seq))
}

/// d#65
fn Decoder_opentype_var_packed_point_numbers_runs(_input: &mut Parser<'_>) -> Result<opentype_var_packed_point_numbers_runs, ParseError> {
let control = {
let _packed_bits = _input.read_byte()?;
let points_are_words = _packed_bits >> 7u8 & 1u8 > 0u8;
let point_run_count = _packed_bits & 127u8;
opentype_var_packed_point_numbers_run_control { points_are_words, point_run_count }
};
let points = {
let run_length = succ(control.point_run_count);
match control.points_are_words {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
opentype_var_packed_point_numbers_run_points::Points16(inner)
},

false => {
let inner = {
let mut accum = Vec::new();
for _ in 0..run_length {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
opentype_var_packed_point_numbers_run_points::Points8(inner)
}
}
};
PResult::Ok(opentype_var_packed_point_numbers_runs { control, points })
}

/// d#66
fn Decoder_opentype_fvar_variation_axis_record(_input: &mut Parser<'_>) -> Result<opentype_fvar_variation_axis_record, ParseError> {
let axis_tag = (Decoder53(_input))?;
let min_value = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
let default_value = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
let max_value = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
let flags = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let hidden_axis = _packed_bits & 1u16 > 0u16;
opentype_fvar_variation_axis_record_flags { hidden_axis }
};
let axis_name_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
PResult::Ok(opentype_fvar_variation_axis_record { axis_tag, min_value, default_value, max_value, flags, axis_name_id })
}

/// d#67
fn Decoder_opentype_fvar_instance_record(_input: &mut Parser<'_>, axis_count: u16, instance_size: u16) -> Result<opentype_fvar_instance_record, ParseError> {
let subfamily_nameid = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let flags = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10973085168168570837u64));
}
};
let coordinates = (Decoder_opentype_fvar_user_tuple(_input, axis_count))?;
let postscript_nameid = if instance_size % 4u16 == 2u16 {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
PResult::Ok(opentype_fvar_instance_record { subfamily_nameid, flags, coordinates, postscript_nameid })
}

/// d#68
fn Decoder_opentype_fvar_user_tuple(_input: &mut Parser<'_>, axis_count: u16) -> Result<opentype_fvar_user_tuple, ParseError> {
let coordinates = {
let mut accum = Vec::new();
for _ in 0..axis_count {
let next_elem = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
opentype_head_table_font_revision::Fixed32(inner)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_fvar_user_tuple { coordinates })
}

/// d#69
fn Decoder_opentype_layout_script_list<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_script_list<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let script_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let script_records = {
let mut accum = Vec::new();
for _ in 0..script_count {
let next_elem = (Decoder_opentype_layout_script_record(_input, table_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_script_list { table_scope, script_count, script_records })
}

/// d#70
fn Decoder_opentype_layout_feature_list<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_feature_list<'input>, ParseError> {
let list_view = _input.view();
let list_scope = list_view;
let feature_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let feature_records = {
let mut accum = Vec::new();
for _ in 0..feature_count {
let next_elem = (Decoder_opentype_layout_feature_record(_input, list_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_feature_list { list_scope, feature_count, feature_records })
}

/// d#71
fn Decoder_opentype_gsub_lookup_list<'input>(_input: &mut Parser<'input>) -> Result<opentype_gsub_lookup_list<'input>, ParseError> {
let list_view = _input.view();
let list_scope = list_view;
let lookup_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lookups = {
let mut accum = Vec::new();
for _ in 0..lookup_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gsub_lookup_list_lookups { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gsub_lookup_list { list_scope, lookup_count, lookups })
}

/// d#72
fn Decoder_opentype_layout_feature_variations<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_feature_variations<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10603707580403307601u64));
}
};
let minor_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(18065118697073160549u64));
}
};
let feature_variation_record_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let feature_variation_records = {
let mut accum = Vec::new();
for _ in 0..feature_variation_record_count {
let next_elem = (Decoder_opentype_layout_feature_variation_record(_input, table_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_feature_variations { table_scope, major_version, minor_version, feature_variation_record_count, feature_variation_records })
}

/// d#73
fn Decoder_opentype_layout_feature_variation_record<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_feature_variation_record<'input>, ParseError> {
let condition_set = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_feature_variation_record_condition_set { offset, _data }
};
let feature_table_substitution = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_feature_variation_record_feature_table_substitution { offset, _data }
};
PResult::Ok(opentype_layout_feature_variation_record { condition_set, feature_table_substitution })
}

/// d#74
fn Decoder_opentype_layout_feature_table_substitution<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_feature_table_substitution<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let major_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10686389193617118447u64));
}
};
let minor_version = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(16128388243093908143u64));
}
};
let substitution_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let substitutions = {
let mut accum = Vec::new();
for _ in 0..substitution_count {
let next_elem = (Decoder_opentype_layout_feature_table_substitution_record(_input, table_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_feature_table_substitution { table_scope, major_version, minor_version, substitution_count, substitutions })
}

/// d#75
fn Decoder_opentype_layout_feature_table_substitution_record<'input>(_input: &mut Parser<'input>, _table_view: View<'input>) -> Result<opentype_layout_feature_table_substitution_record<'input>, ParseError> {
let feature_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let alternate_feature_offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let alternate_feature = std::marker::PhantomData;
PResult::Ok(opentype_layout_feature_table_substitution_record { feature_index, alternate_feature_offset, alternate_feature })
}

/// d#76
fn Decoder_opentype_layout_feature_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_feature_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let feature_params = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lookup_index_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lookup_list_indices = {
let mut accum = Vec::new();
for _ in 0..lookup_index_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_feature_table { table_scope, feature_params, lookup_index_count, lookup_list_indices })
}

/// d#77
fn Decoder_opentype_gsub_lookup_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_gsub_lookup_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let lookup_type = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lookup_flag = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let mark_attachment_class_filter = _packed_bits >> 8u16 & 255u16;
let use_mark_filtering_set = _packed_bits >> 4u16 & 1u16 > 0u16;
let ignore_marks = _packed_bits >> 3u16 & 1u16 > 0u16;
let ignore_ligatures = _packed_bits >> 2u16 & 1u16 > 0u16;
let ignore_base_glyphs = _packed_bits >> 1u16 & 1u16 > 0u16;
let right_to_left = _packed_bits & 1u16 > 0u16;
opentype_gpos_lookup_table_lookup_flag { mark_attachment_class_filter, use_mark_filtering_set, ignore_marks, ignore_ligatures, ignore_base_glyphs, right_to_left }
};
let sub_table_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let subtables = {
let mut accum = Vec::new();
for _ in 0..sub_table_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gsub_lookup_table_subtables { offset, _data }
};
accum.push(next_elem)
};
accum
};
let mark_filtering_set = match lookup_flag.use_mark_filtering_set {
true => {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
},

false => {
None
}
};
PResult::Ok(opentype_gsub_lookup_table { table_scope, lookup_type, lookup_flag, sub_table_count, subtables, mark_filtering_set })
}

/// d#78
fn Decoder_opentype_gsub_lookup_subtable<'input>(_input: &mut Parser<'input>, lookup_type: u16) -> Result<opentype_gsub_lookup_subtable<'input>, ParseError> {
PResult::Ok(match lookup_type {
7u16 => {
let inner = (Decoder_opentype_layout_subst_extension(_input))?;
opentype_gsub_lookup_subtable::SubstExtension(inner)
},

_ => {
let inner = (Decoder_opentype_layout_ground_subst(_input, lookup_type))?;
opentype_gsub_lookup_subtable::GroundSubst(inner)
}
})
}

/// d#79
fn Decoder_opentype_layout_subst_extension<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_subst_extension<'input>, ParseError> {
let table_view = _input.view();
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let format = inner;
format == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2818918064991511645u64));
}
};
let table_scope = table_view;
let extension_lookup_type = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
matches!(x, 1u16..=6u16 | 8)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14082539304789607227u64));
}
};
let extension_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_subst_extension_extension_offset { offset, _data }
};
PResult::Ok(opentype_layout_subst_extension { format, table_scope, extension_lookup_type, extension_offset })
}

/// d#80
fn Decoder_opentype_layout_ground_subst<'input>(_input: &mut Parser<'input>, lookup_type: u16) -> Result<opentype_layout_ground_subst<'input>, ParseError> {
PResult::Ok(match lookup_type {
1u16 => {
let inner = (Decoder_opentype_layout_single_subst(_input))?;
opentype_layout_ground_subst::SingleSubst(inner)
},

2u16 => {
let inner = (Decoder_opentype_layout_multiple_subst(_input))?;
opentype_layout_ground_subst::MultipleSubst(inner)
},

3u16 => {
let inner = (Decoder_opentype_gsub_alternate_subst(_input))?;
opentype_layout_ground_subst::AlternateSubst(inner)
},

4u16 => {
let inner = (Decoder_opentype_layout_ligature_subst(_input))?;
opentype_layout_ground_subst::LigatureSubst(inner)
},

5u16 => {
let inner = (Decoder_opentype_layout_sequence_context(_input))?;
opentype_layout_ground_subst::SequenceContext(inner)
},

6u16 => {
let inner = (Decoder_opentype_layout_chained_sequence_context(_input))?;
opentype_layout_ground_subst::ChainedSequenceContext(inner)
},

8u16 => {
let inner = (Decoder_opentype_layout_reverse_chain_single_subst(_input))?;
opentype_layout_ground_subst::ReverseChainSingleSubst(inner)
},

7u16 => {
return Err(ParseError::FailToken(11072034178440885507u64));
},

_ => {
return Err(ParseError::FailToken(4608405370414018463u64));
}
})
}

/// d#81
fn Decoder_opentype_layout_single_subst<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_single_subst<'input>, ParseError> {
let table_view = _input.view();
let subst_format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let subst = match subst_format {
1u16 => {
let inner = (Decoder_opentype_layout_single_subst_format1(_input, table_view))?;
opentype_layout_single_subst_subst::Format1(inner)
},

2u16 => {
let inner = (Decoder_opentype_layout_single_subst_format2(_input, table_view))?;
opentype_layout_single_subst_subst::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(4418518334087228745u64));
}
};
PResult::Ok(opentype_layout_single_subst { subst_format, subst })
}

/// d#82
fn Decoder_opentype_layout_multiple_subst<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_multiple_subst<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let subst_format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let subst = match subst_format {
1u16 => {
let inner = {
let sequence_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let sequence_offsets = {
let mut accum = Vec::new();
for _ in 0..sequence_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let _sequences = std::marker::PhantomData;
opentype_layout_multiple_subst_subst_Format1 { sequence_count, sequence_offsets, _sequences }
};
opentype_layout_multiple_subst_subst::Format1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(opentype_layout_multiple_subst { table_scope, subst_format, coverage, subst })
}

/// d#83
fn Decoder_opentype_gsub_alternate_subst<'input>(_input: &mut Parser<'input>) -> Result<opentype_gsub_alternate_subst<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let subst_format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let subst_format = inner;
subst_format == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7086880279337729577u64));
}
};
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let alternate_set_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let alternate_sets = {
let mut accum = Vec::new();
for _ in 0..alternate_set_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gsub_alternate_subst_alternate_sets { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gsub_alternate_subst { table_scope, subst_format, coverage, alternate_set_count, alternate_sets })
}

/// d#84
fn Decoder_opentype_layout_ligature_subst<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_ligature_subst<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let subst_format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let subst_format = inner;
subst_format == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7511456693437940214u64));
}
};
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let ligature_set_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let ligature_sets = {
let mut accum = Vec::new();
for _ in 0..ligature_set_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_ligature_subst_ligature_sets { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_ligature_subst { table_scope, subst_format, coverage, ligature_set_count, ligature_sets })
}

/// d#85
fn Decoder_opentype_layout_sequence_context<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_sequence_context<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let subst = match format {
1u16 => {
let inner = (Decoder_opentype_layout_sequence_context_format1(_input, table_view))?;
opentype_layout_sequence_context_subst::Format1(inner)
},

2u16 => {
let inner = (Decoder_opentype_layout_sequence_context_format2(_input, table_view))?;
opentype_layout_sequence_context_subst::Format2(inner)
},

3u16 => {
let inner = (Decoder_opentype_layout_sequence_context_format3(_input, table_view))?;
opentype_layout_sequence_context_subst::Format3(inner)
},

_ => {
return Err(ParseError::FailToken(973408085875818710u64));
}
};
PResult::Ok(opentype_layout_sequence_context { table_scope, format, subst })
}

/// d#86
fn Decoder_opentype_layout_chained_sequence_context<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_chained_sequence_context<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let subst = match format {
1u16 => {
let inner = (Decoder_opentype_layout_chained_sequence_context_format1(_input, table_view))?;
opentype_layout_chained_sequence_context_subst::Format1(inner)
},

2u16 => {
let inner = (Decoder_opentype_layout_chained_sequence_context_format2(_input, table_view))?;
opentype_layout_chained_sequence_context_subst::Format2(inner)
},

3u16 => {
let inner = (Decoder_opentype_layout_chained_sequence_context_format3(_input, table_view))?;
opentype_layout_chained_sequence_context_subst::Format3(inner)
},

_ => {
return Err(ParseError::FailToken(15557503981608772456u64));
}
};
PResult::Ok(opentype_layout_chained_sequence_context { table_scope, format, subst })
}

/// d#87
fn Decoder_opentype_layout_reverse_chain_single_subst<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_reverse_chain_single_subst<'input>, ParseError> {
let table_view = _input.view();
let subst_format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let subst_format = inner;
subst_format == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2154669163482751322u64));
}
};
let table_scope = table_view;
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let backtrack_glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let backtrack_coverage_tables = {
let mut accum = Vec::new();
for _ in 0..backtrack_glyph_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
accum.push(next_elem)
};
accum
};
let lookahead_glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lookahead_coverage_tables = {
let mut accum = Vec::new();
for _ in 0..lookahead_glyph_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
accum.push(next_elem)
};
accum
};
let glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let substitute_glyph_ids = {
let mut accum = Vec::new();
for _ in 0..glyph_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_reverse_chain_single_subst { subst_format, table_scope, coverage, backtrack_glyph_count, backtrack_coverage_tables, lookahead_glyph_count, lookahead_coverage_tables, glyph_count, substitute_glyph_ids })
}

/// d#88
fn Decoder_opentype_coverage_table(_input: &mut Parser<'_>) -> Result<opentype_coverage_table, ParseError> {
let coverage_format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = match coverage_format {
1u16 => {
let inner = {
let glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let glyph_array = {
let mut accum = Vec::new();
for _ in 0..glyph_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
opentype_coverage_table_data_Format1 { glyph_count, glyph_array }
};
opentype_coverage_table_data::Format1(inner)
},

2u16 => {
let inner = {
let range_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let range_records = {
let mut accum = Vec::new();
for _ in 0..range_count {
let next_elem = {
let start_glyph_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let end_glyph_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let start_coverage_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_coverage_table_data_Format2_range_records { start_glyph_id, end_glyph_id, start_coverage_index }
};
accum.push(next_elem)
};
accum
};
opentype_coverage_table_data_Format2 { range_count, range_records }
};
opentype_coverage_table_data::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(10263667190582992611u64));
}
};
PResult::Ok(opentype_coverage_table { coverage_format, data })
}

/// d#89
fn Decoder_opentype_layout_chained_sequence_context_format1<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_chained_sequence_context_format1<'input>, ParseError> {
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let chained_seq_rule_set_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let chained_seq_rule_sets = {
let mut accum = Vec::new();
for _ in 0..chained_seq_rule_set_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_chained_sequence_context_format1_chained_seq_rule_sets { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_chained_sequence_context_format1 { coverage, chained_seq_rule_set_count, chained_seq_rule_sets })
}

/// d#90
fn Decoder_opentype_layout_chained_sequence_context_format2<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_chained_sequence_context_format2<'input>, ParseError> {
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let backtrack_class_def = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_glyph_class_def { offset, _data }
};
let input_class_def = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_glyph_class_def { offset, _data }
};
let lookahead_class_def = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_glyph_class_def { offset, _data }
};
let chained_class_seq_rule_set_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let chained_class_seq_rule_sets = {
let mut accum = Vec::new();
for _ in 0..chained_class_seq_rule_set_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_chained_sequence_context_format1_chained_seq_rule_sets { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_chained_sequence_context_format2 { coverage, backtrack_class_def, input_class_def, lookahead_class_def, chained_class_seq_rule_set_count, chained_class_seq_rule_sets })
}

/// d#91
fn Decoder_opentype_layout_chained_sequence_context_format3<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_chained_sequence_context_format3, ParseError> {
let backtrack_glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let backtrack_coverages = {
let mut accum = Vec::new();
for _ in 0..backtrack_glyph_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
accum.push(next_elem)
};
accum
};
let input_glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let input_coverages = {
let mut accum = Vec::new();
for _ in 0..input_glyph_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
accum.push(next_elem)
};
accum
};
let lookahead_glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lookahead_coverages = {
let mut accum = Vec::new();
for _ in 0..lookahead_glyph_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
accum.push(next_elem)
};
accum
};
let seq_lookup_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let seq_lookup_records = {
let mut accum = Vec::new();
for _ in 0..seq_lookup_count {
let next_elem = (Decoder_opentype_layout_sequence_lookup(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_chained_sequence_context_format3 { backtrack_glyph_count, backtrack_coverages, input_glyph_count, input_coverages, lookahead_glyph_count, lookahead_coverages, seq_lookup_count, seq_lookup_records })
}

/// d#92
fn Decoder_opentype_layout_sequence_lookup(_input: &mut Parser<'_>) -> Result<opentype_layout_sequence_lookup, ParseError> {
let sequence_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lookup_list_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
PResult::Ok(opentype_layout_sequence_lookup { sequence_index, lookup_list_index })
}

/// d#93
fn Decoder_opentype_class_def(_input: &mut Parser<'_>) -> Result<opentype_class_def, ParseError> {
let class_format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = match class_format {
1u16 => {
let inner = {
let start_glyph_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let class_value_array = {
let mut accum = Vec::new();
for _ in 0..glyph_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
opentype_class_def_data_Format1 { start_glyph_id, glyph_count, class_value_array }
};
opentype_class_def_data::Format1(inner)
},

2u16 => {
let inner = {
let class_range_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let class_range_records = {
let mut accum = Vec::new();
for _ in 0..class_range_count {
let next_elem = {
let start_glyph_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let end_glyph_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let class = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_class_def_data_Format2_class_range_records { start_glyph_id, end_glyph_id, class }
};
accum.push(next_elem)
};
accum
};
opentype_class_def_data_Format2 { class_range_count, class_range_records }
};
opentype_class_def_data::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(5482396765248532989u64));
}
};
PResult::Ok(opentype_class_def { class_format, data })
}

/// d#94
fn Decoder_opentype_layout_chained_sequence_rule_set<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_chained_sequence_rule_set<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let chained_seq_rule_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let chained_seq_rules = {
let mut accum = Vec::new();
for _ in 0..chained_seq_rule_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_chained_sequence_rule_set_chained_seq_rules { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_chained_sequence_rule_set { table_scope, chained_seq_rule_count, chained_seq_rules })
}

/// d#95
fn Decoder_opentype_layout_chained_sequence_rule(_input: &mut Parser<'_>) -> Result<opentype_layout_chained_sequence_rule, ParseError> {
let backtrack_glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let backtrack_sequence = {
let mut accum = Vec::new();
for _ in 0..backtrack_glyph_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let input_glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let input_sequence = {
let mut accum = Vec::new();
for _ in 0..pred(input_glyph_count) {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let lookahead_glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lookahead_sequence = {
let mut accum = Vec::new();
for _ in 0..lookahead_glyph_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let seq_lookup_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let seq_lookup_records = {
let mut accum = Vec::new();
for _ in 0..seq_lookup_count {
let next_elem = (Decoder_opentype_layout_sequence_lookup(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_chained_sequence_rule { backtrack_glyph_count, backtrack_sequence, input_glyph_count, input_sequence, lookahead_glyph_count, lookahead_sequence, seq_lookup_count, seq_lookup_records })
}

/// d#96
fn Decoder_opentype_layout_sequence_context_format1<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_sequence_context_format1<'input>, ParseError> {
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let seq_rule_set_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let seq_rule_sets = {
let mut accum = Vec::new();
for _ in 0..seq_rule_set_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_sequence_context_format1_seq_rule_sets { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_sequence_context_format1 { coverage, seq_rule_set_count, seq_rule_sets })
}

/// d#97
fn Decoder_opentype_layout_sequence_context_format2<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_sequence_context_format2<'input>, ParseError> {
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let class_def = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_glyph_class_def { offset, _data }
};
let class_seq_rule_set_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let class_seq_rule_sets = {
let mut accum = Vec::new();
for _ in 0..class_seq_rule_set_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_sequence_context_format1_seq_rule_sets { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_sequence_context_format2 { coverage, class_def, class_seq_rule_set_count, class_seq_rule_sets })
}

/// d#98
fn Decoder_opentype_layout_sequence_context_format3<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_sequence_context_format3, ParseError> {
let glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let seq_lookup_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let coverage_tables = {
let mut accum = Vec::new();
for _ in 0..glyph_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
accum.push(next_elem)
};
accum
};
let seq_lookup_records = {
let mut accum = Vec::new();
for _ in 0..seq_lookup_count {
let next_elem = (Decoder_opentype_layout_sequence_lookup(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_sequence_context_format3 { glyph_count, seq_lookup_count, coverage_tables, seq_lookup_records })
}

/// d#99
fn Decoder_opentype_layout_sequence_context_rule_set<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_sequence_context_rule_set<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let rule_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let rules = {
let mut accum = Vec::new();
for _ in 0..rule_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_sequence_context_rule_set_rules { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_sequence_context_rule_set { table_scope, rule_count, rules })
}

/// d#100
fn Decoder_opentype_layout_sequence_context_rule(_input: &mut Parser<'_>) -> Result<opentype_layout_sequence_context_rule, ParseError> {
let glyph_count = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x != 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12275201028130973875u64));
}
};
let seq_lookup_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let input_sequence = {
let mut accum = Vec::new();
for _ in 0..pred(glyph_count) {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let seq_lookup_records = {
let mut accum = Vec::new();
for _ in 0..seq_lookup_count {
let next_elem = (Decoder_opentype_layout_sequence_lookup(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_sequence_context_rule { glyph_count, seq_lookup_count, input_sequence, seq_lookup_records })
}

/// d#101
fn Decoder_opentype_gsub_ligature_subst_ligature_set<'input>(_input: &mut Parser<'input>) -> Result<opentype_gsub_ligature_subst_ligature_set<'input>, ParseError> {
let set_view = _input.view();
let set_scope = set_view;
let ligature_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let ligatures = {
let mut accum = Vec::new();
for _ in 0..ligature_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gsub_ligature_subst_ligature_set_ligatures { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gsub_ligature_subst_ligature_set { set_scope, ligature_count, ligatures })
}

/// d#102
fn Decoder_opentype_gsub_ligature_subst_ligature_table(_input: &mut Parser<'_>) -> Result<opentype_gsub_ligature_subst_ligature_table, ParseError> {
let ligature_glyph = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let component_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let component_glyph_ids = {
let mut accum = Vec::new();
for _ in 0..pred(component_count) {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gsub_ligature_subst_ligature_table { ligature_glyph, component_count, component_glyph_ids })
}

/// d#103
fn Decoder_opentype_gsub_alternate_subst_alternate_set(_input: &mut Parser<'_>) -> Result<opentype_gsub_alternate_subst_alternate_set, ParseError> {
let glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let alternate_glyph_ids = {
let mut accum = Vec::new();
for _ in 0..glyph_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gsub_alternate_subst_alternate_set { glyph_count, alternate_glyph_ids })
}

/// d#104
fn Decoder_opentype_layout_multiple_subst_sequence_table(_input: &mut Parser<'_>) -> Result<opentype_layout_multiple_subst_sequence_table, ParseError> {
let glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let substitute_glyph_ids = {
let mut accum = Vec::new();
for _ in 0..glyph_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_multiple_subst_sequence_table { glyph_count, substitute_glyph_ids })
}

/// d#105
fn Decoder_opentype_layout_single_subst_format1<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_single_subst_format1<'input>, ParseError> {
let table_scope = table_view;
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let delta_glyph_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
PResult::Ok(opentype_layout_single_subst_format1 { table_scope, coverage, delta_glyph_id })
}

/// d#106
fn Decoder_opentype_layout_single_subst_format2<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_single_subst_format2<'input>, ParseError> {
let table_scope = table_view;
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let substitute_glyph_ids = {
let mut accum = Vec::new();
for _ in 0..glyph_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_single_subst_format2 { table_scope, coverage, glyph_count, substitute_glyph_ids })
}

/// d#107
fn Decoder_opentype_layout_feature_record<'input>(_input: &mut Parser<'input>, list_view: View<'input>) -> Result<opentype_layout_feature_record<'input>, ParseError> {
let feature_tag = (Decoder53(_input))?;
let feature = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_feature_record_feature { offset, _data }
};
PResult::Ok(opentype_layout_feature_record { feature_tag, feature })
}

/// d#108
fn Decoder_opentype_layout_script_record<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_script_record<'input>, ParseError> {
let script_tag = (Decoder53(_input))?;
let script = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_script_record_script { offset, _data }
};
PResult::Ok(opentype_layout_script_record { script_tag, script })
}

/// d#109
fn Decoder_opentype_layout_script_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_script_table<'input>, ParseError> {
let script_view = _input.view();
let script_scope = script_view;
let default_lang_sys = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_script_table_default_lang_sys { offset, _data }
};
let lang_sys_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lang_sys_records = {
let mut accum = Vec::new();
for _ in 0..lang_sys_count {
let next_elem = (Decoder_opentype_layout_lang_sys_record(_input, script_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_script_table { script_scope, default_lang_sys, lang_sys_count, lang_sys_records })
}

/// d#110
fn Decoder_opentype_layout_langsys(_input: &mut Parser<'_>) -> Result<opentype_layout_langsys, ParseError> {
let lookup_order_offset = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(16097120758067046920u64));
}
};
let required_feature_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let feature_index_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let feature_indices = {
let mut accum = Vec::new();
for _ in 0..feature_index_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_langsys { lookup_order_offset, required_feature_index, feature_index_count, feature_indices })
}

/// d#111
fn Decoder_opentype_layout_lang_sys_record<'input>(_input: &mut Parser<'input>, script_view: View<'input>) -> Result<opentype_layout_lang_sys_record, ParseError> {
let lang_sys_tag = (Decoder53(_input))?;
let lang_sys = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_script_table_default_lang_sys { offset, _data }
};
PResult::Ok(opentype_layout_lang_sys_record { lang_sys_tag, lang_sys })
}

/// d#112
fn Decoder_opentype_gpos_lookup_list<'input>(_input: &mut Parser<'input>) -> Result<opentype_gpos_lookup_list<'input>, ParseError> {
let list_view = _input.view();
let list_scope = list_view;
let lookup_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lookups = {
let mut accum = Vec::new();
for _ in 0..lookup_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gpos_lookup_list_lookups { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gpos_lookup_list { list_scope, lookup_count, lookups })
}

/// d#113
fn Decoder_opentype_gpos_lookup_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_gpos_lookup_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let lookup_type = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lookup_flag = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let mark_attachment_class_filter = _packed_bits >> 8u16 & 255u16;
let use_mark_filtering_set = _packed_bits >> 4u16 & 1u16 > 0u16;
let ignore_marks = _packed_bits >> 3u16 & 1u16 > 0u16;
let ignore_ligatures = _packed_bits >> 2u16 & 1u16 > 0u16;
let ignore_base_glyphs = _packed_bits >> 1u16 & 1u16 > 0u16;
let right_to_left = _packed_bits & 1u16 > 0u16;
opentype_gpos_lookup_table_lookup_flag { mark_attachment_class_filter, use_mark_filtering_set, ignore_marks, ignore_ligatures, ignore_base_glyphs, right_to_left }
};
let sub_table_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let subtables = {
let mut accum = Vec::new();
for _ in 0..sub_table_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gpos_lookup_table_subtables { offset, _data }
};
accum.push(next_elem)
};
accum
};
let mark_filtering_set = match lookup_flag.use_mark_filtering_set {
true => {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
},

false => {
None
}
};
PResult::Ok(opentype_gpos_lookup_table { table_scope, lookup_type, lookup_flag, sub_table_count, subtables, mark_filtering_set })
}

/// d#114
fn Decoder_opentype_gpos_lookup_subtable<'input>(_input: &mut Parser<'input>, lookup_type: u16) -> Result<opentype_gpos_lookup_subtable<'input>, ParseError> {
PResult::Ok(match lookup_type {
9u16 => {
let inner = (Decoder_opentype_layout_pos_extension(_input))?;
opentype_gpos_lookup_subtable::PosExtension(inner)
},

_ => {
let inner = (Decoder_opentype_layout_ground_pos(_input, lookup_type))?;
opentype_gpos_lookup_subtable::GroundPos(inner)
}
})
}

/// d#115
fn Decoder_opentype_layout_pos_extension<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_pos_extension<'input>, ParseError> {
let table_view = _input.view();
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let format = inner;
format == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9331632426086095927u64));
}
};
let table_scope = table_view;
let extension_lookup_type = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
matches!(x, 1u16..=8u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14959848987246965519u64));
}
};
let extension_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_pos_extension_extension_offset { offset, _data }
};
PResult::Ok(opentype_layout_pos_extension { format, table_scope, extension_lookup_type, extension_offset })
}

/// d#116
fn Decoder_opentype_layout_ground_pos<'input>(_input: &mut Parser<'input>, lookup_type: u16) -> Result<opentype_layout_ground_pos<'input>, ParseError> {
PResult::Ok(match lookup_type {
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
let inner = (Decoder_opentype_layout_sequence_context(_input))?;
opentype_layout_ground_pos::SequenceContext(inner)
},

8u16 => {
let inner = (Decoder_opentype_layout_chained_sequence_context(_input))?;
opentype_layout_ground_pos::ChainedSequenceContext(inner)
},

9u16 => {
return Err(ParseError::FailToken(9092905213558799443u64));
},

_ => {
return Err(ParseError::FailToken(17544092807091201u64));
}
})
}

/// d#117
fn Decoder_opentype_layout_single_pos<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_single_pos<'input>, ParseError> {
let table_view = _input.view();
let pos_format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let subtable = match pos_format {
1u16 => {
let inner = (Decoder_opentype_layout_single_pos_format1(_input, table_view))?;
opentype_layout_single_pos_subtable::Format1(inner)
},

2u16 => {
let inner = (Decoder_opentype_layout_single_pos_format2(_input, table_view))?;
opentype_layout_single_pos_subtable::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(10502127387712395480u64));
}
};
PResult::Ok(opentype_layout_single_pos { pos_format, subtable })
}

/// d#118
fn Decoder_opentype_layout_pair_pos<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_pair_pos<'input>, ParseError> {
let table_view = _input.view();
let pos_format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let subtable = match pos_format {
1u16 => {
let inner = (Decoder_opentype_layout_pair_pos_format1(_input, table_view))?;
opentype_layout_pair_pos_subtable::Format1(inner)
},

2u16 => {
let inner = (Decoder_opentype_layout_pair_pos_format2(_input, table_view))?;
opentype_layout_pair_pos_subtable::Format2(inner)
},

_ => {
return Err(ParseError::FailToken(14454034443522724586u64));
}
};
PResult::Ok(opentype_layout_pair_pos { pos_format, subtable })
}

/// d#119
fn Decoder_opentype_layout_cursive_pos<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_cursive_pos<'input>, ParseError> {
let table_view = _input.view();
let pos_format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let pos_format = inner;
pos_format == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(5322124757500927073u64));
}
};
let table_scope = table_view;
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let entry_exit_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let entry_exit_records = {
let mut accum = Vec::new();
for _ in 0..entry_exit_count {
let next_elem = (Decoder_opentype_layout_entry_exit_record(_input, table_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_cursive_pos { pos_format, table_scope, coverage, entry_exit_count, entry_exit_records })
}

/// d#120
fn Decoder_opentype_layout_mark_base_pos<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_mark_base_pos<'input>, ParseError> {
let table_view = _input.view();
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let format = inner;
format == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17869550927478639832u64));
}
};
let table_scope = table_view;
let mark_coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let base_coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let mark_class_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let mark_array = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_mark_base_pos_mark_array { offset, _data }
};
let base_array = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_mark_base_pos_base_array { offset, _data }
};
PResult::Ok(opentype_layout_mark_base_pos { format, table_scope, mark_coverage, base_coverage, mark_class_count, mark_array, base_array })
}

/// d#121
fn Decoder_opentype_layout_mark_lig_pos<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_mark_lig_pos<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let format = inner;
format == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13431462572241034712u64));
}
};
let mark_coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let ligature_coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let mark_class_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let mark_array = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_mark_base_pos_mark_array { offset, _data }
};
let ligature_array = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_mark_lig_pos_ligature_array { offset, _data }
};
PResult::Ok(opentype_layout_mark_lig_pos { table_scope, format, mark_coverage, ligature_coverage, mark_class_count, mark_array, ligature_array })
}

/// d#122
fn Decoder_opentype_layout_mark_mark_pos<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_mark_mark_pos<'input>, ParseError> {
let table_view = _input.view();
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let format = inner;
format == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3433937857563719729u64));
}
};
let table_scope = table_view;
let mark1_coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let mark2_coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let mark_class_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let mark1_array = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_mark_base_pos_mark_array { offset, _data }
};
let mark2_array = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_mark_mark_pos_mark2_array { offset, _data }
};
PResult::Ok(opentype_layout_mark_mark_pos { format, table_scope, mark1_coverage, mark2_coverage, mark_class_count, mark1_array, mark2_array })
}

/// d#123
fn Decoder_opentype_layout_mark_array<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_mark_array<'input>, ParseError> {
let array_view = _input.view();
let array_scope = array_view;
let mark_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let mark_records = {
let mut accum = Vec::new();
for _ in 0..mark_count {
let next_elem = (Decoder_opentype_layout_mark_record(_input, array_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_mark_array { array_scope, mark_count, mark_records })
}

/// d#124
fn Decoder_opentype_layout_mark2_array<'input>(_input: &mut Parser<'input>, mark_class_count: u16) -> Result<opentype_layout_mark2_array<'input>, ParseError> {
let array_view = _input.view();
let array_scope = array_view;
let mark2_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let mark2_records = {
let mut accum = Vec::new();
for _ in 0..mark2_count {
let next_elem = (Decoder_opentype_layout_mark2_array_mark2_record(_input, mark_class_count, array_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_mark2_array { array_scope, mark2_count, mark2_records })
}

/// d#125
fn Decoder_opentype_layout_mark2_array_mark2_record<'input>(_input: &mut Parser<'input>, mark_class_count: u16, _array_view: View<'input>) -> Result<opentype_layout_mark2_array_mark2_record<'input>, ParseError> {
let mark2_anchor_offsets = {
let mut accum = Vec::new();
for _ in 0..mark_class_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let _mark2_anchors = std::marker::PhantomData;
PResult::Ok(opentype_layout_mark2_array_mark2_record { mark2_anchor_offsets, _mark2_anchors })
}

/// d#126
fn Decoder_opentype_layout_anchor_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_anchor_table<'input>, ParseError> {
let table_view = _input.view();
let anchor_format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let table = match anchor_format {
1u16 => {
let inner = {
let x_coordinate = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_coordinate = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_layout_anchor_table_table_Format1 { x_coordinate, y_coordinate }
};
opentype_layout_anchor_table_table::Format1(inner)
},

2u16 => {
let inner = {
let x_coordinate = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_coordinate = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let anchor_point = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_layout_anchor_table_table_Format2 { x_coordinate, y_coordinate, anchor_point }
};
opentype_layout_anchor_table_table::Format2(inner)
},

3u16 => {
let inner = (Decoder_opentype_layout_anchor_table_format3(_input, table_view))?;
opentype_layout_anchor_table_table::Format3(inner)
},

_ => {
return Err(ParseError::FailToken(13516986665125759073u64));
}
};
PResult::Ok(opentype_layout_anchor_table { anchor_format, table })
}

/// d#127
fn Decoder_opentype_layout_anchor_table_format3<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_anchor_table_format3<'input>, ParseError> {
let table_scope = table_view;
let x_coordinate = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_coordinate = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let x_device = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_caret_value_data_format3_table { offset, _data }
};
let y_device = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_caret_value_data_format3_table { offset, _data }
};
PResult::Ok(opentype_layout_anchor_table_format3 { table_scope, x_coordinate, y_coordinate, x_device, y_device })
}

/// d#128
fn Decoder_opentype_common_device_or_variation_index_table(_input: &mut Parser<'_>) -> Result<opentype_common_device_or_variation_index_table, ParseError> {
let delta_format = {
_input.open_peek_context();
let ret = ((|| {
{
let __skipped0 = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let __skipped1 = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_common_device_or_variation_index_table_delta_format_lhs { __skipped0, __skipped1 }
};
let x = (_input.read_byte()?, _input.read_byte()?);
PResult::Ok(u16be(x))
})())?;
_input.close_peek_context()?;
ret
};
PResult::Ok(match delta_format {
1u16..=3u16 => {
let inner = {
let start_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let end_size = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let delta_format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let delta_values = {
let mut accum = Vec::new();
for _ in 0..match delta_format {
1u16 => {
match ((succ(try_sub!(end_size, start_size, 6185506036438099345u64))) / 8u16) * 8u16 < (succ(try_sub!(end_size, start_size, 15794382300316794652u64))) {
true => {
succ((succ(try_sub!(end_size, start_size, 18147521187885925800u64))) / 8u16)
},

false => {
(succ(try_sub!(end_size, start_size, 7364705619221056123u64))) / 8u16
}
}
},

2u16 => {
match ((succ(try_sub!(end_size, start_size, 2404222719611925354u64))) / 4u16) * 4u16 < (succ(try_sub!(end_size, start_size, 13091357170910775568u64))) {
true => {
succ((succ(try_sub!(end_size, start_size, 17170585774888887431u64))) / 4u16)
},

false => {
(succ(try_sub!(end_size, start_size, 1548601315919054830u64))) / 4u16
}
}
},

3u16 => {
match ((succ(try_sub!(end_size, start_size, 9339905250237811640u64))) / 2u16) * 2u16 < (succ(try_sub!(end_size, start_size, 6039633234730737119u64))) {
true => {
succ((succ(try_sub!(end_size, start_size, 1654541969082323602u64))) / 2u16)
},

false => {
(succ(try_sub!(end_size, start_size, 1272171534487716374u64))) / 2u16
}
}
},

_ => {
0u16
}
} {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
opentype_common_device_or_variation_index_table_DeviceTable { start_size, end_size, delta_format, delta_values }
};
opentype_common_device_or_variation_index_table::DeviceTable(inner)
},

32768u16 => {
let inner = {
let delta_set_outer_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let delta_set_inner_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let delta_format = {
let arg0 = {
let b = _input.read_byte()?;
if b == 128 {
b
} else {
return Err(ParseError::ExcludedBranch(14751251992141172493u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(5733880678136728614u64));
}
};
(arg0, arg1)
};
opentype_common_device_or_variation_index_table_VariationIndexTable { delta_set_outer_index, delta_set_inner_index, delta_format }
};
opentype_common_device_or_variation_index_table::VariationIndexTable(inner)
},

other => {
let inner = {
let field0 = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let field1 = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let delta_format = other;
opentype_common_device_or_variation_index_table_OtherTable { field0, field1, delta_format }
};
opentype_common_device_or_variation_index_table::OtherTable(inner)
}
})
}

/// d#129
fn Decoder_opentype_layout_mark_record<'input>(_input: &mut Parser<'input>, array_view: View<'input>) -> Result<opentype_layout_mark_record<'input>, ParseError> {
let mark_class = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let mark_anchor = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_entry_exit_record_entry_anchor { offset, _data }
};
PResult::Ok(opentype_layout_mark_record { mark_class, mark_anchor })
}

/// d#130
fn Decoder_opentype_layout_ligature_array<'input>(_input: &mut Parser<'input>, mark_class_count: u16) -> Result<opentype_layout_ligature_array<'input>, ParseError> {
let array_view = _input.view();
let array_scope = array_view;
let mark_class_count = mark_class_count;
let ligature_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let ligature_attach_offsets = {
let mut accum = Vec::new();
for _ in 0..ligature_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let _ligature_attaches = std::marker::PhantomData;
PResult::Ok(opentype_layout_ligature_array { array_scope, mark_class_count, ligature_count, ligature_attach_offsets, _ligature_attaches })
}

/// d#131
fn Decoder_opentype_layout_ligature_attach<'input>(_input: &mut Parser<'input>, mark_class_count: u16) -> Result<opentype_layout_ligature_attach<'input>, ParseError> {
let table_view = _input.view();
let component_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let component_records = {
let mut accum = Vec::new();
for _ in 0..component_count {
let next_elem = (Decoder_opentype_layout_ligature_attach_component_record(_input, mark_class_count, table_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_ligature_attach { component_count, component_records })
}

/// d#132
fn Decoder_opentype_layout_ligature_attach_component_record<'input>(_input: &mut Parser<'input>, mark_class_count: u16, table_view: View<'input>) -> Result<opentype_layout_ligature_attach_component_record<'input>, ParseError> {
let record_scope = table_view;
let ligature_anchor_offsets = {
let mut accum = Vec::new();
for _ in 0..mark_class_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let _ligature_anchors = std::marker::PhantomData;
PResult::Ok(opentype_layout_ligature_attach_component_record { record_scope, ligature_anchor_offsets, _ligature_anchors })
}

/// d#133
fn Decoder_opentype_layout_base_array<'input>(_input: &mut Parser<'input>, mark_class_count: u16) -> Result<opentype_layout_base_array<'input>, ParseError> {
let array_view = _input.view();
let array_scope = array_view;
let base_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let base_records = {
let mut accum = Vec::new();
for _ in 0..base_count {
let next_elem = (Decoder_opentype_layout_base_array_base_record(_input, mark_class_count, array_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_base_array { array_scope, base_count, base_records })
}

/// d#134
fn Decoder_opentype_layout_base_array_base_record<'input>(_input: &mut Parser<'input>, mark_class_count: u16, _array_view: View<'input>) -> Result<opentype_layout_base_array_base_record<'input>, ParseError> {
let base_anchor_offsets = {
let mut accum = Vec::new();
for _ in 0..mark_class_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let _base_anchors = std::marker::PhantomData;
PResult::Ok(opentype_layout_base_array_base_record { base_anchor_offsets, _base_anchors })
}

/// d#135
fn Decoder_opentype_layout_entry_exit_record<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_entry_exit_record<'input>, ParseError> {
let entry_anchor = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_entry_exit_record_entry_anchor { offset, _data }
};
let exit_anchor = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_entry_exit_record_entry_anchor { offset, _data }
};
PResult::Ok(opentype_layout_entry_exit_record { entry_anchor, exit_anchor })
}

/// d#136
fn Decoder_opentype_layout_pair_pos_format1<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_pair_pos_format1<'input>, ParseError> {
let table_scope = table_view;
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let value_format1 = (Decoder_opentype_layout_value_format_flags(_input))?;
let value_format2 = (Decoder_opentype_layout_value_format_flags(_input))?;
let pair_set_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let pair_sets = {
let mut accum = Vec::new();
for _ in 0..pair_set_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_pair_pos_format1_pair_sets { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_pair_pos_format1 { table_scope, coverage, value_format1, value_format2, pair_set_count, pair_sets })
}

/// d#137
fn Decoder_opentype_layout_pair_pos_format2<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_pair_pos_format2<'input>, ParseError> {
let table_scope = table_view;
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let value_format1 = (Decoder_opentype_layout_value_format_flags(_input))?;
let value_format2 = (Decoder_opentype_layout_value_format_flags(_input))?;
let class_def1 = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_glyph_class_def { offset, _data }
};
let class_def2 = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_table_glyph_class_def { offset, _data }
};
let class1_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let class2_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let class1_records = {
let mut accum = Vec::new();
for _ in 0..class1_count {
let next_elem = {
let class2_records = {
let mut accum = Vec::new();
for _ in 0..class2_count {
let next_elem = (Decoder_opentype_layout_pair_pos_class2_record(_input, value_format1, value_format2, table_view))?;
accum.push(next_elem)
};
accum
};
opentype_layout_pair_pos_format2_class1_records { class2_records }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_pair_pos_format2 { table_scope, coverage, value_format1, value_format2, class_def1, class_def2, class1_count, class2_count, class1_records })
}

/// d#138
fn Decoder_opentype_layout_value_format_flags(_input: &mut Parser<'_>) -> Result<opentype_layout_value_format_flags, ParseError> {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_advance_device = _packed_bits >> 7u16 & 1u16 > 0u16;
let x_advance_device = _packed_bits >> 6u16 & 1u16 > 0u16;
let y_placement_device = _packed_bits >> 5u16 & 1u16 > 0u16;
let x_placement_device = _packed_bits >> 4u16 & 1u16 > 0u16;
let y_advance = _packed_bits >> 3u16 & 1u16 > 0u16;
let x_advance = _packed_bits >> 2u16 & 1u16 > 0u16;
let y_placement = _packed_bits >> 1u16 & 1u16 > 0u16;
let x_placement = _packed_bits & 1u16 > 0u16;
PResult::Ok(opentype_layout_value_format_flags { y_advance_device, x_advance_device, y_placement_device, x_placement_device, y_advance, x_advance, y_placement, x_placement })
}

/// d#139
fn Decoder_opentype_layout_pair_pos_class2_record<'input>(_input: &mut Parser<'input>, value_format1: opentype_layout_value_format_flags, value_format2: opentype_layout_value_format_flags, table_view: View<'input>) -> Result<opentype_layout_pair_pos_class2_record, ParseError> {
let value_record1 = if value_format1.x_placement || value_format1.y_placement || value_format1.x_advance || value_format1.y_advance || value_format1.x_placement_device || value_format1.y_placement_device || value_format1.x_advance_device || value_format1.y_advance_device {
Some((Decoder_opentype_layout_value_record(_input, value_format1, table_view))?)
} else {
None
};
let value_record2 = if value_format2.x_placement || value_format2.y_placement || value_format2.x_advance || value_format2.y_advance || value_format2.x_placement_device || value_format2.y_placement_device || value_format2.x_advance_device || value_format2.y_advance_device {
Some((Decoder141(_input, value_format2, table_view))?)
} else {
None
};
PResult::Ok(opentype_layout_pair_pos_class2_record { value_record1, value_record2 })
}

/// d#140
fn Decoder_opentype_layout_value_record<'input>(_input: &mut Parser<'input>, flags: opentype_layout_value_format_flags, table_view: View<'input>) -> Result<opentype_layout_value_record, ParseError> {
let x_placement = if flags.x_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_placement = if flags.y_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_advance = if flags.x_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_advance = if flags.y_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_placement_device = if flags.x_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_placement_device = if flags.y_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let x_advance_device = if flags.x_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_advance_device = if flags.y_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
PResult::Ok(opentype_layout_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

/// d#141
fn Decoder141<'input>(_input: &mut Parser<'input>, flags: opentype_layout_value_format_flags, table_view: View<'input>) -> Result<opentype_layout_value_record, ParseError> {
let x_placement = if flags.x_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_placement = if flags.y_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_advance = if flags.x_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_advance = if flags.y_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_placement_device = if flags.x_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_placement_device = if flags.y_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let x_advance_device = if flags.x_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_advance_device = if flags.y_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
PResult::Ok(opentype_layout_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

/// d#142
fn Decoder_opentype_layout_pair_pos_pair_set<'input>(_input: &mut Parser<'input>, value_format1: opentype_layout_value_format_flags, value_format2: opentype_layout_value_format_flags) -> Result<opentype_layout_pair_pos_pair_set<'input>, ParseError> {
let set_view = _input.view();
let set_scope = set_view;
let pair_value_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let pair_value_records = {
let mut accum = Vec::new();
for _ in 0..pair_value_count {
let next_elem = (Decoder_opentype_layout_pair_pos_pair_value_record(_input, value_format1, value_format2, set_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_pair_pos_pair_set { set_scope, pair_value_count, pair_value_records })
}

/// d#143
fn Decoder_opentype_layout_pair_pos_pair_value_record<'input>(_input: &mut Parser<'input>, value_format1: opentype_layout_value_format_flags, value_format2: opentype_layout_value_format_flags, set_view: View<'input>) -> Result<opentype_layout_pair_pos_pair_value_record, ParseError> {
let second_glyph = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let value_record1 = if value_format1.x_placement || value_format1.y_placement || value_format1.x_advance || value_format1.y_advance || value_format1.x_placement_device || value_format1.y_placement_device || value_format1.x_advance_device || value_format1.y_advance_device {
Some((Decoder144(_input, value_format1, set_view))?)
} else {
None
};
let value_record2 = if value_format2.x_placement || value_format2.y_placement || value_format2.x_advance || value_format2.y_advance || value_format2.x_placement_device || value_format2.y_placement_device || value_format2.x_advance_device || value_format2.y_advance_device {
Some((Decoder145(_input, value_format2, set_view))?)
} else {
None
};
PResult::Ok(opentype_layout_pair_pos_pair_value_record { second_glyph, value_record1, value_record2 })
}

/// d#144
fn Decoder144<'input>(_input: &mut Parser<'input>, flags: opentype_layout_value_format_flags, table_view: View<'input>) -> Result<opentype_layout_value_record, ParseError> {
let x_placement = if flags.x_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_placement = if flags.y_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_advance = if flags.x_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_advance = if flags.y_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_placement_device = if flags.x_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_placement_device = if flags.y_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let x_advance_device = if flags.x_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_advance_device = if flags.y_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
PResult::Ok(opentype_layout_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

/// d#145
fn Decoder145<'input>(_input: &mut Parser<'input>, flags: opentype_layout_value_format_flags, table_view: View<'input>) -> Result<opentype_layout_value_record, ParseError> {
let x_placement = if flags.x_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_placement = if flags.y_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_advance = if flags.x_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_advance = if flags.y_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_placement_device = if flags.x_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_placement_device = if flags.y_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let x_advance_device = if flags.x_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_advance_device = if flags.y_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
PResult::Ok(opentype_layout_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

/// d#146
fn Decoder_opentype_layout_single_pos_format1<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_single_pos_format1<'input>, ParseError> {
let table_scope = table_view;
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let value_format = (Decoder_opentype_layout_value_format_flags(_input))?;
let value_record = (Decoder149(_input, value_format, table_view))?;
PResult::Ok(opentype_layout_single_pos_format1 { table_scope, coverage, value_format, value_record })
}

/// d#147
fn Decoder_opentype_layout_single_pos_format2<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_single_pos_format2<'input>, ParseError> {
let table_scope = table_view;
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let value_format = (Decoder_opentype_layout_value_format_flags(_input))?;
let value_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let value_records = {
let mut accum = Vec::new();
for _ in 0..value_count {
let next_elem = (Decoder148(_input, value_format, table_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_single_pos_format2 { table_scope, coverage, value_format, value_count, value_records })
}

/// d#148
fn Decoder148<'input>(_input: &mut Parser<'input>, flags: opentype_layout_value_format_flags, table_view: View<'input>) -> Result<opentype_layout_value_record, ParseError> {
let x_placement = if flags.x_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_placement = if flags.y_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_advance = if flags.x_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_advance = if flags.y_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_placement_device = if flags.x_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_placement_device = if flags.y_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let x_advance_device = if flags.x_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_advance_device = if flags.y_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
PResult::Ok(opentype_layout_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

/// d#149
fn Decoder149<'input>(_input: &mut Parser<'input>, flags: opentype_layout_value_format_flags, table_view: View<'input>) -> Result<opentype_layout_value_record, ParseError> {
let x_placement = if flags.x_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_placement = if flags.y_placement {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_advance = if flags.x_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let y_advance = if flags.y_advance {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let x_placement_device = if flags.x_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_placement_device = if flags.y_placement_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let x_advance_device = if flags.x_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
let y_advance_device = if flags.y_advance_device {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
Some(opentype_gdef_caret_value_data_format3_table { offset, _data })
} else {
None
};
PResult::Ok(opentype_layout_value_record { x_placement, y_placement, x_advance, y_advance, x_placement_device, y_placement_device, x_advance_device, y_advance_device })
}

/// d#150
fn Decoder_opentype_gdef_attach_list<'input>(_input: &mut Parser<'input>) -> Result<opentype_gdef_attach_list<'input>, ParseError> {
let list_view = _input.view();
let list_scope = list_view;
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let attach_point_offsets = {
let mut accum = Vec::new();
for _ in 0..glyph_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_attach_point_offsets { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gdef_attach_list { list_scope, coverage, glyph_count, attach_point_offsets })
}

/// d#151
fn Decoder_opentype_gdef_lig_caret_list<'input>(_input: &mut Parser<'input>) -> Result<opentype_gdef_lig_caret_list<'input>, ParseError> {
let list_view = _input.view();
let list_scope = list_view;
let coverage = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_attach_list_coverage { offset, _data }
};
let lig_glyph_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lig_glyph_offsets = {
let mut accum = Vec::new();
for _ in 0..lig_glyph_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_lig_caret_list_lig_glyph_offsets { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gdef_lig_caret_list { list_scope, coverage, lig_glyph_count, lig_glyph_offsets })
}

/// d#152
fn Decoder_opentype_gdef_mark_glyph_set<'input>(_input: &mut Parser<'input>) -> Result<opentype_gdef_mark_glyph_set<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8997881400116719018u64));
}
};
let mark_glyph_set_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let coverage = {
let mut accum = Vec::new();
for _ in 0..mark_glyph_set_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_mark_glyph_set_coverage { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gdef_mark_glyph_set { table_scope, format, mark_glyph_set_count, coverage })
}

/// d#153
fn Decoder_opentype_common_item_variation_store<'input>(_input: &mut Parser<'input>) -> Result<opentype_common_item_variation_store<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 1u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13614619987783239962u64));
}
};
let variation_region_list = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_common_item_variation_store_variation_region_list { offset, _data }
};
let item_variation_data_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let item_variation_data_list = {
let mut accum = Vec::new();
for _ in 0..item_variation_data_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_common_item_variation_store_item_variation_data_list { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_common_item_variation_store { table_scope, format, variation_region_list, item_variation_data_count, item_variation_data_list })
}

/// d#154
fn Decoder_opentype_common_variation_region_list(_input: &mut Parser<'_>) -> Result<opentype_common_variation_region_list, ParseError> {
let axis_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let region_count = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
matches!(x, 0u16..=32767u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6915530142412472120u64));
}
};
let variation_regions = {
let mut accum = Vec::new();
for _ in 0..region_count {
let next_elem = {
let region_axes = {
let mut accum = Vec::new();
for _ in 0..axis_count {
let next_elem = {
let start_coord = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
let peak_coord = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
let end_coord = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
opentype_common_variation_region_list_variation_regions_region_axes { start_coord, peak_coord, end_coord }
};
accum.push(next_elem)
};
accum
};
opentype_common_variation_region_list_variation_regions { region_axes }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_common_variation_region_list { axis_count, region_count, variation_regions })
}

/// d#155
fn Decoder_opentype_common_item_variation_data(_input: &mut Parser<'_>) -> Result<opentype_common_item_variation_data, ParseError> {
let item_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let word_delta_count = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let long_words = _packed_bits >> 15u16 & 1u16 > 0u16;
let word_count = _packed_bits & 32767u16;
opentype_common_item_variation_data_word_delta_count { long_words, word_count }
};
let region_index_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let region_indices = {
let mut accum = Vec::new();
for _ in 0..region_index_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let delta_sets = match word_delta_count.long_words {
true => {
let inner = {
let mut accum = Vec::new();
for _ in 0..item_count {
let next_elem = {
let delta_data_full_word = {
let mut accum = Vec::new();
for _ in 0..word_delta_count.word_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
accum.push(next_elem)
};
accum
};
let delta_data_half_word = {
let mut accum = Vec::new();
for _ in 0..try_sub!(region_index_count, word_delta_count.word_count, 7000022209644146403u64) {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
opentype_common_item_variation_data_delta_sets_Delta32Sets { delta_data_full_word, delta_data_half_word }
};
accum.push(next_elem)
};
accum
};
opentype_common_item_variation_data_delta_sets::Delta32Sets(inner)
},

false => {
let inner = {
let mut accum = Vec::new();
for _ in 0..item_count {
let next_elem = {
let delta_data_full_word = {
let mut accum = Vec::new();
for _ in 0..word_delta_count.word_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let delta_data_half_word = {
let mut accum = Vec::new();
for _ in 0..try_sub!(region_index_count, word_delta_count.word_count, 8451221290566481190u64) {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
opentype_common_item_variation_data_delta_sets_Delta16Sets { delta_data_full_word, delta_data_half_word }
};
accum.push(next_elem)
};
accum
};
opentype_common_item_variation_data_delta_sets::Delta16Sets(inner)
}
};
PResult::Ok(opentype_common_item_variation_data { item_count, word_delta_count, region_index_count, region_indices, delta_sets })
}

/// d#156
fn Decoder_opentype_gdef_lig_glyph<'input>(_input: &mut Parser<'input>) -> Result<opentype_gdef_lig_glyph<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let caret_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let caret_values = {
let mut accum = Vec::new();
for _ in 0..caret_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_lig_glyph_caret_values { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gdef_lig_glyph { table_scope, caret_count, caret_values })
}

/// d#157
fn Decoder_opentype_gdef_caret_value<'input>(_input: &mut Parser<'input>) -> Result<opentype_gdef_caret_value<'input>, ParseError> {
let table_view = _input.view();
let format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = match format {
1u16 => {
let inner = {
let coordinate = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gdef_caret_value_data_Format1 { coordinate }
};
opentype_gdef_caret_value_data::Format1(inner)
},

2u16 => {
let inner = {
let caret_value_point_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gdef_caret_value_data_Format2 { caret_value_point_index }
};
opentype_gdef_caret_value_data::Format2(inner)
},

3u16 => {
let inner = (Decoder_opentype_gdef_caret_value_data_format3(_input, table_view))?;
opentype_gdef_caret_value_data::Format3(inner)
},

_ => {
return Err(ParseError::FailToken(6949960292533894002u64));
}
};
PResult::Ok(opentype_gdef_caret_value { format, data })
}

/// d#158
fn Decoder_opentype_gdef_caret_value_data_format3<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_gdef_caret_value_data_format3<'input>, ParseError> {
let table_scope = table_view;
let coordinate = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let table = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_caret_value_data_format3_table { offset, _data }
};
PResult::Ok(opentype_gdef_caret_value_data_format3 { table_scope, coordinate, table })
}

/// d#159
fn Decoder_opentype_gdef_attach_point(_input: &mut Parser<'_>) -> Result<opentype_gdef_attach_point, ParseError> {
let point_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let point_indices = {
let mut accum = Vec::new();
for _ in 0..point_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_gdef_attach_point { point_count, point_indices })
}

/// d#160
fn Decoder_opentype_layout_axis_table<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_axis_table<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let base_tag_list_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_axis_table_base_tag_list_offset { offset, _data }
};
let base_script_list_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_axis_table_base_script_list_offset { offset, _data }
};
PResult::Ok(opentype_layout_axis_table { table_scope, base_tag_list_offset, base_script_list_offset })
}

/// d#161
fn Decoder_opentype_base_base_script_record<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_base_base_script_record<'input>, ParseError> {
let base_script_tag = (Decoder53(_input))?;
let base_script = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_base_base_script_record_base_script { offset, _data }
};
PResult::Ok(opentype_base_base_script_record { base_script_tag, base_script })
}

/// d#162
fn Decoder_opentype_layout_base_script<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_base_script<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let base_values_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_base_script_base_values_offset { offset, _data }
};
let default_min_max_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_base_script_default_min_max_offset { offset, _data }
};
let base_lang_sys_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let base_lang_sys_records = {
let mut accum = Vec::new();
for _ in 0..base_lang_sys_count {
let next_elem = (Decoder_opentype_base_base_langsys(_input, table_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_base_script { table_scope, base_values_offset, default_min_max_offset, base_lang_sys_count, base_lang_sys_records })
}

/// d#163
fn Decoder_opentype_layout_base_values<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_base_values<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let default_baseline_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let base_coord_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let base_coord_offsets = {
let mut accum = Vec::new();
for _ in 0..base_coord_count {
let next_elem = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_base_values_base_coord_offsets { offset, _data }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_base_values { table_scope, default_baseline_index, base_coord_count, base_coord_offsets })
}

/// d#164
fn Decoder_opentype_layout_min_max<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_min_max<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let min_coord_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_base_values_base_coord_offsets { offset, _data }
};
let max_coord_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_base_values_base_coord_offsets { offset, _data }
};
let feat_min_max_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let feat_min_max_records = {
let mut accum = Vec::new();
for _ in 0..feat_min_max_count {
let next_elem = (Decoder_opentype_layout_feat_min_max(_input, table_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_layout_min_max { table_scope, min_coord_offset, max_coord_offset, feat_min_max_count, feat_min_max_records })
}

/// d#165
fn Decoder_opentype_base_base_langsys<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_base_base_langsys<'input>, ParseError> {
let base_lang_sys_tag = (Decoder53(_input))?;
let min_max = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_base_script_default_min_max_offset { offset, _data }
};
PResult::Ok(opentype_base_base_langsys { base_lang_sys_tag, min_max })
}

/// d#166
fn Decoder_opentype_layout_base_coord<'input>(_input: &mut Parser<'input>) -> Result<opentype_layout_base_coord<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let coordinate = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = match format {
1u16 => {
opentype_layout_base_coord_data::NoData
},

2u16 => {
let inner = {
let reference_glyph = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let base_coord_point = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_layout_base_coord_data_GlyphData { reference_glyph, base_coord_point }
};
opentype_layout_base_coord_data::GlyphData(inner)
},

3u16 => {
let inner = {
let device = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_gdef_caret_value_data_format3_table { offset, _data }
};
opentype_layout_base_coord_data_DeviceData { device }
};
opentype_layout_base_coord_data::DeviceData(inner)
},

_ => {
return Err(ParseError::FailToken(1347174710810305478u64));
}
};
PResult::Ok(opentype_layout_base_coord { table_scope, format, coordinate, data })
}

/// d#167
fn Decoder_opentype_layout_feat_min_max<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_layout_feat_min_max<'input>, ParseError> {
let feature_tag = (Decoder53(_input))?;
let min_coord_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_base_values_base_coord_offsets { offset, _data }
};
let max_coord_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let _data = std::marker::PhantomData;
opentype_layout_base_values_base_coord_offsets { offset, _data }
};
PResult::Ok(opentype_layout_feat_min_max { feature_tag, min_coord_offset, max_coord_offset })
}

/// d#168
fn Decoder_opentype_glyf_entry(_input: &mut Parser<'_>) -> Result<opentype_glyf_entry, ParseError> {
let number_of_contours = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let x_min = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_min = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let x_max = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let y_max = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let description = (Decoder_opentype_glyf_description(_input, number_of_contours))?;
PResult::Ok(opentype_glyf_entry { number_of_contours, x_min, y_min, x_max, y_max, description })
}

/// d#169
fn Decoder_opentype_glyf_description(_input: &mut Parser<'_>, n_contours: u16) -> Result<opentype_glyf_description, ParseError> {
PResult::Ok(match n_contours {
0u16 => {
opentype_glyf_description::HeaderOnly
},

1u16..=32767u16 => {
let inner = (Decoder_opentype_glyf_simple(_input, n_contours))?;
opentype_glyf_description::Simple(inner)
},

_ => {
let inner = (Decoder_opentype_glyf_composite(_input))?;
opentype_glyf_description::Composite(inner)
}
})
}

/// d#170
fn Decoder_opentype_glyf_simple(_input: &mut Parser<'_>, n_contours: u16) -> Result<opentype_glyf_simple, ParseError> {
let end_points_of_contour = {
let mut accum = Vec::new();
for _ in 0..n_contours {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let instruction_length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let instructions = {
let mut accum = Vec::new();
for _ in 0..instruction_length {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
let number_of_coordinates = succ(end_points_of_contour[(pred((end_points_of_contour.len()) as u32)) as usize]);
let flags = {
let arr_flags = {
let tuple_var = {
let mut seq: Vec<opentype_glyf_simple_flags_lhs> = Vec::new();
let mut acc = 0u16;
loop {
{
let tmp_cond = {
let totlen = acc;
totlen >= number_of_coordinates
};
if tmp_cond {
break
};

};
let elem = {
let flags = (Decoder_opentype_glyph_description_simple_flags_raw(_input))?;
let repeats = match flags.repeat_flag {
true => {
_input.read_byte()?
},

false => {
0u8
}
};
let field_set = opentype_glyf_simple_flags { on_curve_point: flags.on_curve_point, x_short_vector: flags.x_short_vector, y_short_vector: flags.y_short_vector, x_is_same_or_positive_x_short_vector: flags.x_is_same_or_positive_x_short_vector, y_is_same_or_positive_y_short_vector: flags.y_is_same_or_positive_y_short_vector, overlap_simple: flags.overlap_simple };
opentype_glyf_simple_flags_lhs { repeats, field_set }
};
acc = {
let acc = acc;
let flags = elem;
acc + (succ(flags.repeats as u16))
};
seq.push(elem)
};
(acc, seq)
};
{
let (_len, flags) = tuple_var;
flags
}
};
(try_flat_map_vec(arr_flags.iter().cloned(), |packed: opentype_glyf_simple_flags_lhs| PResult::Ok(dup32((packed.repeats as u32) + 1u32, packed.field_set))))?
};
let x_coordinates = {
let mut accum = Vec::new();
for flag_vals in flags.clone() {
let next_elem = match flag_vals.x_short_vector {
true => {
let abs = _input.read_byte()?;
match flag_vals.x_is_same_or_positive_x_short_vector {
true => {
abs as u16
},

false => {
match abs {
0u8 => {
0u16
},

n => {
try_sub!(65535u16, pred(n as u16), 15803403730818557393u64)
}
}
}
}
},

false => {
match flag_vals.x_is_same_or_positive_x_short_vector {
true => {
0u16
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
}
}
}
};
accum.push(next_elem)
};
accum
};
let y_coordinates = {
let mut accum = Vec::new();
for flag_vals in flags.clone() {
let next_elem = match flag_vals.y_short_vector {
true => {
let abs = _input.read_byte()?;
match flag_vals.y_is_same_or_positive_y_short_vector {
true => {
abs as u16
},

false => {
match abs {
0u8 => {
0u16
},

n => {
try_sub!(65535u16, pred(n as u16), 6347242493551283856u64)
}
}
}
}
},

false => {
match flag_vals.y_is_same_or_positive_y_short_vector {
true => {
0u16
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
}
}
}
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_glyf_simple { end_points_of_contour, instruction_length, instructions, number_of_coordinates, flags, x_coordinates, y_coordinates })
}

/// d#171
fn Decoder_opentype_glyf_composite(_input: &mut Parser<'_>) -> Result<opentype_glyf_composite, ParseError> {
let acc_glyphs = {
let mut seq: Vec<opentype_glyf_composite_acc_glyphs> = Vec::new();
let mut acc = false;
loop {
{
let tmp_cond = {
let seq = &seq;
match match ((seq.len()) as u32) != 0u32 {
true => {
Some(seq[(pred((seq.len()) as u32)) as usize])
},

false => {
None
}
} {
Some(ref x) => {
!x.flags.more_components
},

None => {
false
}
}
};
if tmp_cond {
break
};

};
let elem = {
let flags = {
let _packed_bits = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let unscaled_component_offset = _packed_bits >> 12u16 & 1u16 > 0u16;
let scaled_component_offset = _packed_bits >> 11u16 & 1u16 > 0u16;
let overlap_compound = _packed_bits >> 10u16 & 1u16 > 0u16;
let use_my_metrics = _packed_bits >> 9u16 & 1u16 > 0u16;
let we_have_instructions = _packed_bits >> 8u16 & 1u16 > 0u16;
let we_have_a_two_by_two = _packed_bits >> 7u16 & 1u16 > 0u16;
let we_have_an_x_and_y_scale = _packed_bits >> 6u16 & 1u16 > 0u16;
let more_components = _packed_bits >> 5u16 & 1u16 > 0u16;
let we_have_a_scale = _packed_bits >> 3u16 & 1u16 > 0u16;
let round_xy_to_grid = _packed_bits >> 2u16 & 1u16 > 0u16;
let args_are_xy_values = _packed_bits >> 1u16 & 1u16 > 0u16;
let arg_1_and_2_are_words = _packed_bits & 1u16 > 0u16;
opentype_glyf_composite_acc_glyphs_flags { unscaled_component_offset, scaled_component_offset, overlap_compound, use_my_metrics, we_have_instructions, we_have_a_two_by_two, we_have_an_x_and_y_scale, more_components, we_have_a_scale, round_xy_to_grid, args_are_xy_values, arg_1_and_2_are_words }
};
let glyph_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let argument1 = match flags.arg_1_and_2_are_words {
true => {
match flags.args_are_xy_values {
true => {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_glyf_composite_acc_glyphs_argument1::Int16(inner)
},

false => {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_glyf_composite_acc_glyphs_argument1::Uint16(inner)
}
}
},

false => {
match flags.args_are_xy_values {
true => {
let inner = _input.read_byte()?;
opentype_glyf_composite_acc_glyphs_argument1::Int8(inner)
},

false => {
let inner = _input.read_byte()?;
opentype_glyf_composite_acc_glyphs_argument1::Uint8(inner)
}
}
}
};
let argument2 = match flags.arg_1_and_2_are_words {
true => {
match flags.args_are_xy_values {
true => {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_glyf_composite_acc_glyphs_argument1::Int16(inner)
},

false => {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_glyf_composite_acc_glyphs_argument1::Uint16(inner)
}
}
},

false => {
match flags.args_are_xy_values {
true => {
let inner = _input.read_byte()?;
opentype_glyf_composite_acc_glyphs_argument1::Int8(inner)
},

false => {
let inner = _input.read_byte()?;
opentype_glyf_composite_acc_glyphs_argument1::Uint8(inner)
}
}
}
};
let scale = match flags.we_have_a_scale {
true => {
let inner = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
Some(opentype_glyf_composite_acc_glyphs_scale::Scale(inner))
},

false => {
match flags.we_have_an_x_and_y_scale {
true => {
let inner = {
let x_scale = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
let y_scale = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
opentype_glyf_composite_acc_glyphs_scale_XY { x_scale, y_scale }
};
Some(opentype_glyf_composite_acc_glyphs_scale::XY(inner))
},

false => {
match flags.we_have_a_two_by_two {
true => {
let arg0 = {
let arg0 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
let arg1 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
(arg0, arg1)
};
let arg1 = {
let arg0 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
let arg1 = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_gvar_tuple_record_coordinates::F2Dot14(inner)
};
(arg0, arg1)
};
Some(opentype_glyf_composite_acc_glyphs_scale::Matrix(arg0, arg1))
},

false => {
None
}
}
}
}
}
};
opentype_glyf_composite_acc_glyphs { flags, glyph_index, argument1, argument2, scale }
};
acc = {
let acc = acc;
let glyph = elem;
acc || glyph.flags.we_have_instructions
};
seq.push(elem)
};
(acc, seq)
};
let glyphs = acc_glyphs.1.clone();
let instructions = match acc_glyphs.0 {
true => {
let instructions_length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let mut accum = Vec::new();
for _ in 0..instructions_length {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
},

false => {
[].to_vec()
}
};
PResult::Ok(opentype_glyf_composite { glyphs, instructions })
}

/// d#172
fn Decoder_opentype_glyph_description_simple_flags_raw(_input: &mut Parser<'_>) -> Result<opentype_glyph_description_simple_flags_raw, ParseError> {
let _packed_bits = _input.read_byte()?;
let overlap_simple = _packed_bits >> 6u8 & 1u8 > 0u8;
let y_is_same_or_positive_y_short_vector = _packed_bits >> 5u8 & 1u8 > 0u8;
let x_is_same_or_positive_x_short_vector = _packed_bits >> 4u8 & 1u8 > 0u8;
let repeat_flag = _packed_bits >> 3u8 & 1u8 > 0u8;
let y_short_vector = _packed_bits >> 2u8 & 1u8 > 0u8;
let x_short_vector = _packed_bits >> 1u8 & 1u8 > 0u8;
let on_curve_point = _packed_bits & 1u8 > 0u8;
PResult::Ok(opentype_glyph_description_simple_flags_raw { overlap_simple, y_is_same_or_positive_y_short_vector, x_is_same_or_positive_x_short_vector, repeat_flag, y_short_vector, x_short_vector, on_curve_point })
}

/// d#173
fn Decoder_opentype_post_version2<'input>(_input: &mut Parser<'input>) -> Result<opentype_post_version2<'input>, ParseError> {
let num_glyphs = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let glyph_name_index = {
let mut accum = Vec::new();
for _ in 0..num_glyphs {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let string_data = {
let mut accum = Vec::new();
for _ in 0..match (succ((try_fold_left_curried(glyph_name_index.iter().cloned(), 0u16, |tuple_var: (u16, u16)| PResult::Ok({
let (acc, x) = tuple_var;
match acc >= x {
true => {
acc
},

false => {
x
}
}
})))?)) <= 258u16 {
true => {
0u16
},

false => {
try_sub!(succ((try_fold_left_curried(glyph_name_index.iter().cloned(), 0u16, |tuple_var: (u16, u16)| PResult::Ok({
let (acc, x) = tuple_var;
match acc >= x {
true => {
acc
},

false => {
x
}
}
})))?), 258u16, 7283163102885684771u64)
}
} {
let next_elem = (Decoder_opentype_post_pascal_string(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_post_version2 { num_glyphs, glyph_name_index, string_data })
}

/// d#174
fn Decoder_opentype_post_pascal_string<'input>(_input: &mut Parser<'input>) -> Result<opentype_post_pascal_string<'input>, ParseError> {
let length = _input.read_byte()?;
let string = {
let pascal_string_data = _input.view();
pascal_string_data.read_len(length as usize)
};
PResult::Ok(opentype_post_pascal_string { length, string })
}

/// d#175
fn Decoder_opentype_name_name_record<'input>(_input: &mut Parser<'input>, storage_view: View<'input>) -> Result<opentype_name_name_record<'input>, ParseError> {
let platform = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let encoding = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let language = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let name_id = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let string = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = storage_view.offset(offset as usize)?.read_len(length as usize);
opentype_name_name_record_string { offset, data }
};
PResult::Ok(opentype_name_name_record { platform, encoding, language, name_id, length, string })
}

/// d#176
fn Decoder_opentype_name_name_version_1<'input>(_input: &mut Parser<'input>, storage_view: View<'input>) -> Result<opentype_name_name_version_1<'input>, ParseError> {
let lang_tag_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lang_tag_records = {
let mut accum = Vec::new();
for _ in 0..lang_tag_count {
let next_elem = (Decoder_opentype_name_lang_tag_record(_input, storage_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_name_name_version_1 { lang_tag_count, lang_tag_records })
}

/// d#177
fn Decoder_opentype_name_lang_tag_record<'input>(_input: &mut Parser<'input>, storage_view: View<'input>) -> Result<opentype_name_lang_tag_record<'input>, ParseError> {
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let lang_tag = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = storage_view.offset(offset as usize)?.read_len(length as usize);
opentype_name_name_record_string { offset, data }
};
PResult::Ok(opentype_name_lang_tag_record { length, lang_tag })
}

/// d#178
fn Decoder_opentype_maxp_version1(_input: &mut Parser<'_>) -> Result<opentype_maxp_version1, ParseError> {
let max_points = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_contours = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_composite_points = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_composite_contours = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_zones = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
matches!(x, 1u16..=2u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4251627061094365437u64));
}
};
let max_twilight_points = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_storage = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_function_defs = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_instruction_defs = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_stack_elements = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_size_of_instructions = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_component_elements = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let max_component_depth = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x <= 16u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11915580511665106140u64));
}
};
PResult::Ok(opentype_maxp_version1 { max_points, max_contours, max_composite_points, max_composite_contours, max_zones, max_twilight_points, max_storage, max_function_defs, max_instruction_defs, max_stack_elements, max_size_of_instructions, max_component_elements, max_component_depth })
}

/// d#179
fn Decoder179(_input: &mut Parser<'_>) -> Result<u64, ParseError> {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
PResult::Ok(u64be(x))
}

/// d#180
fn Decoder_opentype_encoding_record<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_encoding_record<'input>, ParseError> {
let platform = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let encoding = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let subtable = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_encoding_record_subtable { offset, _data }
};
PResult::Ok(opentype_encoding_record { platform, encoding, subtable })
}

/// d#181
fn Decoder_opentype_cmap_subtable<'input>(_input: &mut Parser<'input>, _platform: u16) -> Result<opentype_cmap_subtable<'input>, ParseError> {
let table_view = _input.view();
let table_scope = table_view;
let format = {
_input.open_peek_context();
let ret = ((|| {
let x = (_input.read_byte()?, _input.read_byte()?);
PResult::Ok(u16be(x))
})())?;
_input.close_peek_context()?;
ret
};
let data = match format {
0u16 => {
let inner = (Decoder_opentype_cmap_subtable_format0(_input, _platform))?;
opentype_cmap_subtable_data::Format0(inner)
},

2u16 => {
let inner = (Decoder_opentype_cmap_subtable_format2(_input, _platform))?;
opentype_cmap_subtable_data::Format2(inner)
},

4u16 => {
let inner = (Decoder_opentype_cmap_subtable_format4(_input, _platform))?;
opentype_cmap_subtable_data::Format4(inner)
},

6u16 => {
let inner = (Decoder_opentype_cmap_subtable_format6(_input, _platform))?;
opentype_cmap_subtable_data::Format6(inner)
},

8u16 => {
let inner = (Decoder_opentype_cmap_subtable_format8(_input, _platform))?;
opentype_cmap_subtable_data::Format8(inner)
},

10u16 => {
let inner = (Decoder_opentype_cmap_subtable_format10(_input, _platform))?;
opentype_cmap_subtable_data::Format10(inner)
},

12u16 => {
let inner = (Decoder_opentype_cmap_subtable_format12(_input, _platform))?;
opentype_cmap_subtable_data::Format12(inner)
},

13u16 => {
let inner = (Decoder189(_input, _platform))?;
opentype_cmap_subtable_data::Format13(inner)
},

14u16 => {
let inner = (Decoder_opentype_cmap_subtable_format14(_input, table_view))?;
opentype_cmap_subtable_data::Format14(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(opentype_cmap_subtable { table_scope, format, data })
}

/// d#182
fn Decoder_opentype_cmap_subtable_format0(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format0, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
{
let format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_cmap_subtable_format0_length_lhs { format }
};
let x = (_input.read_byte()?, _input.read_byte()?);
PResult::Ok(u16be(x))
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let language = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let glyph_id_array = {
let mut accum = Vec::new();
for _ in 0..256u16 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_cmap_subtable_format0 { format, length, language, glyph_id_array })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

/// d#183
fn Decoder_opentype_cmap_subtable_format2(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format2, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
{
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 2u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17324980155911269375u64));
}
};
opentype_cmap_subtable_format0_length_lhs { format }
};
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let l = inner;
(l >= 518u16) && (l % 2u16 == 0u16)
};
PResult::Ok(if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17670535809278048255u64));
})
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 2u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2444204717155307095u64));
}
};
let length = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let l = inner;
(l >= 518u16) && (l % 2u16 == 0u16)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1278184758971178969u64));
}
};
let language = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let sub_header_keys = {
let mut accum = Vec::new();
for _ in 0..256u16 {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let sub_headers = {
let mut accum = Vec::new();
for _ in 0..succ(match (try_fold_left_curried(sub_header_keys.iter().cloned(), None, |tuple_var: (Option<u16>, u16)| PResult::Ok({
let (acc, y) = tuple_var;
match acc {
Some(x) => {
Some(match x >= y / 8u16 {
true => {
x
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
})))? {
Some(x) => {
x
},

_ => {
return Err(ParseError::ExcludedBranch(5100077783044507986u64));
}
}) {
let next_elem = {
let first_code = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let id_delta = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let id_range_offset = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
opentype_cmap_subtable_format2_sub_headers { first_code, entry_count, id_delta, id_range_offset }
};
accum.push(next_elem)
};
accum
};
let glyph_array = {
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
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(opentype_cmap_subtable_format2 { format, length, language, sub_header_keys, sub_headers, glyph_array })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

/// d#184
fn Decoder_opentype_cmap_subtable_format4(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format4, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
{
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 4u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4480225125687487743u64));
}
};
opentype_cmap_subtable_format0_length_lhs { format }
};
let x = (_input.read_byte()?, _input.read_byte()?);
PResult::Ok(u16be(x))
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 4u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12879845237981630531u64));
}
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let language = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let seg_count = {
let seg_count_x2 = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
seg_count_x2 / 2u16
};
let search_range = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let entry_selector = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let range_shift = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let end_code = {
let mut accum = Vec::new();
for _ in 0..seg_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
{
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3426398976290336157u64));
}
};
let start_code = {
let mut accum = Vec::new();
for _ in 0..seg_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let id_delta = {
let mut accum = Vec::new();
for _ in 0..seg_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let id_range_offset = {
let mut accum = Vec::new();
for _ in 0..seg_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
let glyph_array = {
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
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(opentype_cmap_subtable_format4 { format, length, language, seg_count, search_range, entry_selector, range_shift, end_code, start_code, id_delta, id_range_offset, glyph_array })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

/// d#185
fn Decoder_opentype_cmap_subtable_format6(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format6, ParseError> {
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 6u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11250208753083412758u64));
}
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let language = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let first_code = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let glyph_id_array = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_cmap_subtable_format6 { format, length, language, first_code, entry_count, glyph_id_array })
}

/// d#186
fn Decoder_opentype_cmap_subtable_format8(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format8, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
{
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 8u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(17349123374714965876u64));
}
};
let __reserved = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2153064741293804702u64));
}
};
opentype_cmap_subtable_format8_length_lhs { format, __reserved }
};
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
PResult::Ok(u32be(x))
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 8u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1588651938759015246u64));
}
};
{
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3249387167439447765u64));
}
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let language = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let is32 = {
let mut accum = Vec::new();
for _ in 0..8192u16 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
let num_groups = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let groups = {
let mut accum = Vec::new();
for _ in 0..num_groups {
let next_elem = (Decoder_opentype_types_sequential_map_record(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_cmap_subtable_format8 { format, length, language, is32, num_groups, groups })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

/// d#187
fn Decoder_opentype_cmap_subtable_format10(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format10, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
{
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 10u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7744051144774795087u64));
}
};
let __reserved = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8700288293163706751u64));
}
};
opentype_cmap_subtable_format8_length_lhs { format, __reserved }
};
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
PResult::Ok(u32be(x))
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 10u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(16771529512960957239u64));
}
};
{
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13846498452079501214u64));
}
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let language = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let start_char_code = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let num_chars = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let glyph_id_array = {
let mut accum = Vec::new();
for _ in 0..num_chars {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_cmap_subtable_format10 { format, length, language, start_char_code, num_chars, glyph_id_array })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

/// d#188
fn Decoder_opentype_cmap_subtable_format12(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format12, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
{
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 12u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9798710097031164942u64));
}
};
let __reserved = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9819345728844658158u64));
}
};
opentype_cmap_subtable_format8_length_lhs { format, __reserved }
};
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
PResult::Ok(u32be(x))
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 12u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(18279137173405083757u64));
}
};
{
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14954891776835932150u64));
}
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let language = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let num_groups = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let groups = {
let mut accum = Vec::new();
for _ in 0..num_groups {
let next_elem = (Decoder_opentype_types_sequential_map_record(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_cmap_subtable_format12 { format, length, language, num_groups, groups })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

/// d#189
fn Decoder189(_input: &mut Parser<'_>, _platform: u16) -> Result<opentype_cmap_subtable_format12, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
{
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 13u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11046436797737227751u64));
}
};
let __reserved = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(14984809111992638634u64));
}
};
opentype_cmap_subtable_format8_length_lhs { format, __reserved }
};
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
PResult::Ok(u32be(x))
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 13u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9342187932533045817u64));
}
};
{
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13404710972790825894u64));
}
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let language = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let num_groups = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let groups = {
let mut accum = Vec::new();
for _ in 0..num_groups {
let next_elem = (Decoder_opentype_types_sequential_map_record(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_cmap_subtable_format12 { format, length, language, num_groups, groups })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

/// d#190
fn Decoder_opentype_cmap_subtable_format14<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_cmap_subtable_format14, ParseError> {
let length = {
_input.open_peek_context();
let ret = ((|| {
{
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 14u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(2688427941405105545u64));
}
};
opentype_cmap_subtable_format0_length_lhs { format }
};
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
PResult::Ok(u32be(x))
})())?;
_input.close_peek_context()?;
ret
};
let sz = length as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let format = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x == 14u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6279463968646665849u64));
}
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let num_var_selector_records = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let var_selector = {
let mut accum = Vec::new();
for _ in 0..num_var_selector_records {
let next_elem = (Decoder_opentype_variation_selector(_input, table_view))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(opentype_cmap_subtable_format14 { format, length, num_var_selector_records, var_selector })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

/// d#191
fn Decoder_opentype_variation_selector<'input>(_input: &mut Parser<'input>, table_view: View<'input>) -> Result<opentype_variation_selector, ParseError> {
let var_selector = {
let x = (0u8, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let default_uvs_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_variation_selector_default_uvs_offset { offset, _data }
};
let non_default_uvs_offset = {
let offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let _data = std::marker::PhantomData;
opentype_variation_selector_non_default_uvs_offset { offset, _data }
};
PResult::Ok(opentype_variation_selector { var_selector, default_uvs_offset, non_default_uvs_offset })
}

/// d#192
fn Decoder_opentype_types_sequential_map_record(_input: &mut Parser<'_>) -> Result<opentype_types_sequential_map_record, ParseError> {
let start_char_code = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let end_char_code = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let start_glyph_id = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(opentype_types_sequential_map_record { start_char_code, end_char_code, start_glyph_id })
}

/// d#193
fn Decoder_elf_header(_input: &mut Parser<'_>) -> Result<elf_header, ParseError> {
let ident = {
let sz = 16u32 as usize;
_input.start_slice(sz)?;
let ret = (Decoder_elf_header_ident(_input))?;
_input.end_slice()?;
ret
};
let r#type = (Decoder213(_input, ident.data == 2u8))?;
let machine = (Decoder214(_input, ident.data == 2u8))?;
let version = (Decoder215(_input, ident.data == 2u8))?;
let entry = (Decoder_elf_types_elf_addr(_input, ident.data == 2u8, ident.class))?;
let phoff = (Decoder_elf_types_elf_off(_input, ident.data == 2u8, ident.class))?;
let shoff = (Decoder_elf_types_elf_off(_input, ident.data == 2u8, ident.class))?;
let flags = (Decoder198(_input, ident.data == 2u8))?;
let ehsize = (Decoder216(_input, ident.data == 2u8))?;
let phentsize = (Decoder216(_input, ident.data == 2u8))?;
let phnum = (Decoder216(_input, ident.data == 2u8))?;
let shentsize = (Decoder216(_input, ident.data == 2u8))?;
let shnum = (Decoder216(_input, ident.data == 2u8))?;
let shstrndx = (Decoder216(_input, ident.data == 2u8))?;
PResult::Ok(elf_header { ident, r#type, machine, version, entry, phoff, shoff, flags, ehsize, phentsize, phnum, shentsize, shnum, shstrndx })
}

/// d#194
fn Decoder194(_input: &mut Parser<'_>, is_be: bool, class: u8, phnum: u16) -> Result<Vec<elf_phdr>, ParseError> {
let mut accum = Vec::new();
for _ in 0..phnum {
let next_elem = (Decoder_elf_phdr(_input, is_be, class))?;
accum.push(next_elem)
};
PResult::Ok(accum)
}

/// d#195
fn Decoder195(_input: &mut Parser<'_>, is_be: bool, class: u8, shnum: u16) -> Result<Vec<elf_shdr>, ParseError> {
let mut accum = Vec::new();
for _ in 0..shnum {
let next_elem = (Decoder_elf_shdr(_input, is_be, class))?;
accum.push(next_elem)
};
PResult::Ok(accum)
}

/// d#196
fn Decoder196(_input: &mut Parser<'_>, r#type: u32, size: u64) -> Result<Vec<u8>, ParseError> {
PResult::Ok({
let _ = r#type;
let mut accum = Vec::new();
for _ in 0..size {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
})
}

/// d#197
fn Decoder_elf_shdr(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<elf_shdr, ParseError> {
let name = (Decoder198(_input, is_be))?;
let r#type = (Decoder199(_input, is_be))?;
let flags = (Decoder_elf_types_elf_full(_input, is_be, class))?;
let addr = (Decoder_elf_types_elf_addr(_input, is_be, class))?;
let offset = (Decoder_elf_types_elf_off(_input, is_be, class))?;
let size = (Decoder_elf_types_elf_full(_input, is_be, class))?;
let link = (Decoder198(_input, is_be))?;
let info = (Decoder203(_input, is_be))?;
let addralign = (Decoder_elf_types_elf_full(_input, is_be, class))?;
let entsize = (Decoder_elf_types_elf_full(_input, is_be, class))?;
PResult::Ok(elf_shdr { name, r#type, flags, addr, offset, size, link, info, addralign, entsize })
}

/// d#198
fn Decoder198(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
_input.skip_align(4)?;
PResult::Ok(match is_be {
true => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
}
})
}

/// d#199
fn Decoder199(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
let inner = (Decoder198(_input, is_be))?;
let is_valid = {
let sh_type = inner;
matches!(sh_type, 0u32..=11u32 | 14u32..=18u32 | 1610612736u32..=4294967295u32)
};
PResult::Ok(if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10875553067752207222u64));
})
}

/// d#200
fn Decoder_elf_types_elf_full(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<elf_types_elf_full, ParseError> {
PResult::Ok(match class {
1u8 => {
let inner = (Decoder198(_input, is_be))?;
elf_types_elf_full::Full32(inner)
},

2u8 => {
let inner = (Decoder208(_input, is_be))?;
elf_types_elf_full::Full64(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

/// d#201
fn Decoder_elf_types_elf_addr(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<elf_types_elf_addr, ParseError> {
PResult::Ok(match class {
1u8 => {
let inner = (Decoder206(_input, is_be))?;
elf_types_elf_addr::Addr32(inner)
},

2u8 => {
let inner = (Decoder207(_input, is_be))?;
elf_types_elf_addr::Addr64(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

/// d#202
fn Decoder_elf_types_elf_off(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<elf_types_elf_off, ParseError> {
PResult::Ok(match class {
1u8 => {
let inner = (Decoder204(_input, is_be))?;
elf_types_elf_off::Off32(inner)
},

2u8 => {
let inner = (Decoder205(_input, is_be))?;
elf_types_elf_off::Off64(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

/// d#203
fn Decoder203(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
Decoder198(_input, is_be)
}

/// d#204
fn Decoder204(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
_input.skip_align(4)?;
PResult::Ok(match is_be {
true => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
}
})
}

/// d#205
fn Decoder205(_input: &mut Parser<'_>, is_be: bool) -> Result<u64, ParseError> {
_input.skip_align(8)?;
PResult::Ok(match is_be {
true => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64le(x)
}
})
}

/// d#206
fn Decoder206(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
_input.skip_align(4)?;
PResult::Ok(match is_be {
true => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
}
})
}

/// d#207
fn Decoder207(_input: &mut Parser<'_>, is_be: bool) -> Result<u64, ParseError> {
_input.skip_align(8)?;
PResult::Ok(match is_be {
true => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64le(x)
}
})
}

/// d#208
fn Decoder208(_input: &mut Parser<'_>, is_be: bool) -> Result<u64, ParseError> {
_input.skip_align(8)?;
PResult::Ok(match is_be {
true => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64le(x)
}
})
}

/// d#209
fn Decoder_elf_phdr(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<elf_phdr, ParseError> {
let r#type = (Decoder198(_input, is_be))?;
let flags64 = (Decoder210(_input, is_be, class))?;
let offset = (Decoder_elf_types_elf_off(_input, is_be, class))?;
let vaddr = (Decoder_elf_types_elf_addr(_input, is_be, class))?;
let paddr = (Decoder_elf_types_elf_addr(_input, is_be, class))?;
let filesz = (Decoder_elf_types_elf_full(_input, is_be, class))?;
let memsz = (Decoder_elf_types_elf_full(_input, is_be, class))?;
let flags32 = (Decoder211(_input, is_be, class))?;
let align = (Decoder_elf_types_elf_full(_input, is_be, class))?;
PResult::Ok(elf_phdr { r#type, flags64, offset, vaddr, paddr, filesz, memsz, flags32, align })
}

/// d#210
fn Decoder210(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<Option<u32>, ParseError> {
PResult::Ok(if class == 2u8 {
Some((Decoder198(_input, is_be))?)
} else {
None
})
}

/// d#211
fn Decoder211(_input: &mut Parser<'_>, is_be: bool, class: u8) -> Result<Option<u32>, ParseError> {
PResult::Ok(if class == 1u8 {
Some((Decoder198(_input, is_be))?)
} else {
None
})
}

/// d#212
fn Decoder_elf_header_ident(_input: &mut Parser<'_>) -> Result<elf_header_ident, ParseError> {
{
let arg0 = {
let b = _input.read_byte()?;
if b == 127 {
b
} else {
return Err(ParseError::ExcludedBranch(11323981950571132721u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 69 {
b
} else {
return Err(ParseError::ExcludedBranch(1179945139148562335u64));
}
};
let arg2 = {
let b = _input.read_byte()?;
if b == 76 {
b
} else {
return Err(ParseError::ExcludedBranch(3270685119814653163u64));
}
};
let arg3 = {
let b = _input.read_byte()?;
if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(6821845925776570829u64));
}
};
(arg0, arg1, arg2, arg3)
};
let class = (Decoder217(_input))?;
let data = (Decoder218(_input))?;
let version = (Decoder219(_input))?;
let os_abi = (Decoder220(_input))?;
let abi_version = (Decoder221(_input))?;
{
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
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(elf_header_ident { class, data, version, os_abi, abi_version })
}

/// d#213
fn Decoder213(_input: &mut Parser<'_>, is_be: bool) -> Result<u16, ParseError> {
let inner = (Decoder216(_input, is_be))?;
let is_valid = {
let r#type = inner;
matches!(r#type, 0u16..=4u16 | 65024u16..=65279u16 | 65280u16..=65535u16)
};
PResult::Ok(if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(5653230390980289841u64));
})
}

/// d#214
fn Decoder214(_input: &mut Parser<'_>, is_be: bool) -> Result<u16, ParseError> {
Decoder216(_input, is_be)
}

/// d#215
fn Decoder215(_input: &mut Parser<'_>, is_be: bool) -> Result<u32, ParseError> {
let inner = (Decoder198(_input, is_be))?;
let is_valid = {
let x = inner;
x <= 1u32
};
PResult::Ok(if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9179996462972575343u64));
})
}

/// d#216
fn Decoder216(_input: &mut Parser<'_>, is_be: bool) -> Result<u16, ParseError> {
_input.skip_align(2)?;
PResult::Ok(match is_be {
true => {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
}
})
}

/// d#217
fn Decoder217(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x <= 2u8
};
PResult::Ok(if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3675496117133668659u64));
})
}

/// d#218
fn Decoder218(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x <= 2u8
};
PResult::Ok(if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(6495907546257147840u64));
})
}

/// d#219
fn Decoder219(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x <= 1u8
};
PResult::Ok(if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(8327471529801851430u64));
})
}

/// d#220
fn Decoder220(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
_input.read_byte()
}

/// d#221
fn Decoder221(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
_input.read_byte()
}

/// d#222
fn Decoder_tar_header_with_data(_input: &mut Parser<'_>) -> Result<tar_header_with_data, ParseError> {
let header = (Decoder_tar_header(_input))?;
let file = {
let mut accum = Vec::new();
for _ in 0..header.size {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
_input.skip_align(512)?;
PResult::Ok(tar_header_with_data { header, file })
}

/// d#223
fn Decoder_tar_header(_input: &mut Parser<'_>) -> Result<tar_header, ParseError> {
let sz = 512u32 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let name = {
let sz = 100u16 as usize;
_input.start_slice(sz)?;
let ret = (Decoder_tar_ascii_string_opt0_nonempty(_input))?;
_input.end_slice()?;
ret
};
let mode = {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297u64, 0u64, 0u64, 0u64])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(10310785543736156275u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(16594239496794104967u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
(Decoder225(_input))?;
{
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
return Err(ParseError::ExcludedBranch(16529910322175208638u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_ascii_string_opt0_nonempty { string })
})())?;
_input.end_slice()?;
ret
};
let uid = {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297u64, 0u64, 0u64, 0u64])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(12187643960709778443u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(8766708729375264031u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
(Decoder225(_input))?;
{
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
return Err(ParseError::ExcludedBranch(1369437808023015077u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_ascii_string_opt0_nonempty { string })
})())?;
_input.end_slice()?;
ret
};
let gid = {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297u64, 0u64, 0u64, 0u64])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(163858356033350300u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(888161872995526095u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
(Decoder225(_input))?;
{
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
return Err(ParseError::ExcludedBranch(4770836931378141069u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_ascii_string_opt0_nonempty { string })
})())?;
_input.end_slice()?;
ret
};
let size = {
let rec = {
let _oA = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9976720501248819272u64));
}
};
try_sub!(bit as u8, 48u8, 5446531490235636452u64)
};
let _o9 = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(12530712830475607577u64));
}
};
try_sub!(bit as u8, 48u8, 4795509455376621436u64)
};
let _o8 = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(3923207427992258326u64));
}
};
try_sub!(bit as u8, 48u8, 15995337135637623051u64)
};
let _o7 = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(4063460887563813446u64));
}
};
try_sub!(bit as u8, 48u8, 6726475111737435495u64)
};
let _o6 = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(3702191162545267776u64));
}
};
try_sub!(bit as u8, 48u8, 13281230340934385869u64)
};
let _o5 = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(2229770659268432585u64));
}
};
try_sub!(bit as u8, 48u8, 7200474585457206375u64)
};
let _o4 = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(13266006752343456203u64));
}
};
try_sub!(bit as u8, 48u8, 16152968816646114000u64)
};
let _o3 = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(12815986247090051214u64));
}
};
try_sub!(bit as u8, 48u8, 10057441536650509049u64)
};
let _o2 = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(11815677057767015929u64));
}
};
try_sub!(bit as u8, 48u8, 5170050512307443704u64)
};
let _o1 = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(18440348483137307888u64));
}
};
try_sub!(bit as u8, 48u8, 5159371628350638829u64)
};
let _o0 = {
let bit = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(7762269623995317946u64));
}
};
try_sub!(bit as u8, 48u8, 5308477118997970057u64)
};
(Decoder225(_input))?;
let value = (((0u8 as u32) << 3u32 | (_oA as u32)) << 6u32 | (_o9 as u32) << 3u32 | (_o8 as u32)) << 24u32 | (((_o7 as u32) << 3u32 | (_o6 as u32)) << 6u32 | (_o5 as u32) << 3u32 | (_o4 as u32)) << 12u32 | ((_o3 as u32) << 3u32 | (_o2 as u32)) << 6u32 | (_o1 as u32) << 3u32 | (_o0 as u32);
tar_header_size_lhs { value }
};
rec.value
};
let mtime = {
let sz = 12u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297u64, 0u64, 0u64, 0u64])).contains(byte)) => {
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
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(173922233423713068u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
(Decoder225(_input))?;
{
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
return Err(ParseError::ExcludedBranch(12540117573097456360u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_ascii_string_opt0_nonempty { string })
})())?;
_input.end_slice()?;
ret
};
let chksum = {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297u64, 0u64, 0u64, 0u64])).contains(byte)) => {
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
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(9277543013594125416u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
(Decoder225(_input))?;
{
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
return Err(ParseError::ExcludedBranch(5955168674639093440u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_ascii_string_opt0_nonempty { string })
})())?;
_input.end_slice()?;
ret
};
let typeflag = _input.read_byte()?;
let linkname = {
let sz = 100u16 as usize;
_input.start_slice(sz)?;
let ret = (Decoder226(_input))?;
_input.end_slice()?;
ret
};
let magic = {
let arg0 = {
let b = _input.read_byte()?;
if b == 117 {
b
} else {
return Err(ParseError::ExcludedBranch(4471438437047399494u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 115 {
b
} else {
return Err(ParseError::ExcludedBranch(13319523888327217639u64));
}
};
let arg2 = {
let b = _input.read_byte()?;
if b == 116 {
b
} else {
return Err(ParseError::ExcludedBranch(824589811577025210u64));
}
};
let arg3 = {
let b = _input.read_byte()?;
if b == 97 {
b
} else {
return Err(ParseError::ExcludedBranch(4649034608147552416u64));
}
};
let arg4 = {
let b = _input.read_byte()?;
if b == 114 {
b
} else {
return Err(ParseError::ExcludedBranch(16096650375442290768u64));
}
};
let arg5 = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(14339975513692068616u64));
}
};
(arg0, arg1, arg2, arg3, arg4, arg5)
};
let version = {
let arg0 = {
let b = _input.read_byte()?;
if b == 48 {
b
} else {
return Err(ParseError::ExcludedBranch(16299205781335471965u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 48 {
b
} else {
return Err(ParseError::ExcludedBranch(1479153625485860551u64));
}
};
(arg0, arg1)
};
let uname = {
let sz = 32u16 as usize;
_input.start_slice(sz)?;
let ret = (Decoder227(_input))?;
_input.end_slice()?;
ret
};
let gname = {
let sz = 32u16 as usize;
_input.start_slice(sz)?;
let ret = (Decoder227(_input))?;
_input.end_slice()?;
ret
};
let devmajor = {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297u64, 0u64, 0u64, 0u64])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(12668500753644823654u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(8094248233631264621u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
(Decoder225(_input))?;
{
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
return Err(ParseError::ExcludedBranch(1844274570107701975u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_ascii_string_opt0_nonempty { string })
})())?;
_input.end_slice()?;
ret
};
let devminor = {
let sz = 8u16 as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let string = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([4294967297u64, 0u64, 0u64, 0u64])).contains(byte)) => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(4839194687019048322u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([71776119061217280u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(7230273548678969972u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
(Decoder225(_input))?;
{
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
return Err(ParseError::ExcludedBranch(14903563845775542749u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_ascii_string_opt0_nonempty { string })
})())?;
_input.end_slice()?;
ret
};
let prefix = {
let sz = 155u16 as usize;
_input.start_slice(sz)?;
let ret = (Decoder226(_input))?;
_input.end_slice()?;
ret
};
let pad = {
let mut accum = Vec::new();
for _ in 0..12u32 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(1969670610881234889u64));
}
};
accum.push(next_elem)
};
accum
};
PResult::Ok(tar_header { name, mode, uid, gid, size, mtime, chksum, typeflag, linkname, magic, version, uname, gname, devmajor, devminor, prefix, pad })
})())?;
_input.end_slice()?;
PResult::Ok(ret)
}

/// d#224
fn Decoder_tar_ascii_string_opt0_nonempty(_input: &mut Parser<'_>) -> Result<tar_ascii_string_opt0_nonempty, ParseError> {
let string = {
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
return Err(ParseError::ExcludedBranch(9038350950373664822u64));
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
return Err(ParseError::ExcludedBranch(7281717462557989541u64));
}
};
accum.push(next_elem)
}
};
accum
};
{
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
return Err(ParseError::ExcludedBranch(15510952803379905659u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_ascii_string_opt0_nonempty { string })
}

/// d#225
fn Decoder225(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if (ByteSet::from_bits([4294967297u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(14681668243282477517u64));
})
}

/// d#226
fn Decoder226(_input: &mut Parser<'_>) -> Result<tar_ascii_string_opt0_nonempty, ParseError> {
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
return Err(ParseError::ExcludedBranch(6209434968043366837u64));
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
return Err(ParseError::ExcludedBranch(16474038368490899078u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
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
return Err(ParseError::ExcludedBranch(12217686503432178884u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_ascii_string_opt0_nonempty { string })
}

/// d#227
fn Decoder227(_input: &mut Parser<'_>) -> Result<tar_ascii_string_opt0_nonempty, ParseError> {
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
return Err(ParseError::ExcludedBranch(8399572043096922156u64));
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
return Err(ParseError::ExcludedBranch(7832192330748800109u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
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
return Err(ParseError::ExcludedBranch(9815657591077818003u64));
}
};
accum.push(next_elem)
}
};
accum
};
PResult::Ok(tar_ascii_string_opt0_nonempty { string })
}

/// d#228
fn Decoder_tiff_byte_order(_input: &mut Parser<'_>) -> Result<tiff_byte_order, ParseError> {
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
return Err(ParseError::ExcludedBranch(5152282179373241998u64));
}
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let arg0 = {
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(2197379665604321609u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(16624020278885696461u64));
}
};
tiff_byte_order::le(arg0, arg1)
},

1 => {
let arg0 = {
let b = _input.read_byte()?;
if b == 77 {
b
} else {
return Err(ParseError::ExcludedBranch(14485842416732585139u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 77 {
b
} else {
return Err(ParseError::ExcludedBranch(8179432974518885725u64));
}
};
tiff_byte_order::be(arg0, arg1)
},

_ => {
return Err(ParseError::ExcludedBranch(13780055874544357936u64));
}
})
}

/// d#229
fn Decoder_tiff_ifd_le(_input: &mut Parser<'_>, byte_order: tiff_byte_order) -> Result<tiff_ifd_le, ParseError> {
PResult::Ok(match byte_order {
tiff_byte_order::le(..) => {
(Decoder230(_input))?
},

tiff_byte_order::be(..) => {
(Decoder231(_input))?
}
})
}

/// d#230
fn Decoder230(_input: &mut Parser<'_>) -> Result<tiff_ifd_le, ParseError> {
let num_fields = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let fields = {
let mut accum = Vec::new();
for _ in 0..num_fields {
let next_elem = {
let tag = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let r#type = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
};
let offset_or_data = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
};
tiff_ifd_le_fields { tag, r#type, length, offset_or_data }
};
accum.push(next_elem)
};
accum
};
let next_ifd_offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
};
let next_ifd = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tiff_ifd_le { num_fields, fields, next_ifd_offset, next_ifd })
}

/// d#231
fn Decoder231(_input: &mut Parser<'_>) -> Result<tiff_ifd_le, ParseError> {
let num_fields = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let fields = {
let mut accum = Vec::new();
for _ in 0..num_fields {
let next_elem = {
let tag = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let r#type = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let offset_or_data = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
tiff_ifd_le_fields { tag, r#type, length, offset_or_data }
};
accum.push(next_elem)
};
accum
};
let next_ifd_offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let next_ifd = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tiff_ifd_le { num_fields, fields, next_ifd_offset, next_ifd })
}

/// d#232
fn Decoder_riff_subchunks(_input: &mut Parser<'_>) -> Result<riff_subchunks, ParseError> {
let tag = (Decoder233(_input))?;
let chunks = {
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
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(riff_subchunks { tag, chunks })
}

/// d#233
fn Decoder233(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8), ParseError> {
PResult::Ok((_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?))
}

/// d#234
fn Decoder_riff_chunk(_input: &mut Parser<'_>) -> Result<riff_chunk, ParseError> {
let tag = (Decoder233(_input))?;
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
let pad = if length % 2u32 == 1u32 {
let b = _input.read_byte()?;
Some(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(16909208071962620789u64));
})
} else {
None
};
PResult::Ok(riff_chunk { tag, length, data, pad })
}

/// d#235
fn Decoder_png_ihdr(_input: &mut Parser<'_>) -> Result<png_ihdr, ParseError> {
let length = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let is_valid = {
let length = inner;
length <= 2147483647u32
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7933266403838225878u64));
}
};
let tag = (Decoder281(_input))?;
let data = {
let sz = length as usize;
_input.start_slice(sz)?;
let ret = (Decoder_png_ihdr_data(_input))?;
_input.end_slice()?;
ret
};
let crc = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(png_ihdr { length, tag, data, crc })
}

/// d#236
fn Decoder_png_chunk(_input: &mut Parser<'_>, ihdr: png_ihdr) -> Result<png_chunk, ParseError> {
let length = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let is_valid = {
let length = inner;
length <= 2147483647u32
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12100308281236296642u64));
}
};
let tag = (Decoder251(_input))?;
let data = {
let sz = length as usize;
_input.start_slice(sz)?;
let ret = match slice_all(&tag) {
[80u8, 76u8, 84u8, 69u8] => {
let inner = (Decoder252(_input))?;
png_chunk_data::PLTE(inner)
},

[116u8, 82u8, 78u8, 83u8] => {
let inner = (Decoder_png_trns(_input, ihdr))?;
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
let inner = (Decoder_png_sbit(_input, ihdr))?;
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
let inner = (Decoder_png_bkgd(_input, ihdr))?;
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
png_chunk_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
let crc = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(png_chunk { length, tag, data, crc })
}

/// d#237
fn Decoder_png_idat(_input: &mut Parser<'_>) -> Result<png_idat, ParseError> {
let length = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let is_valid = {
let length = inner;
length <= 2147483647u32
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9041056097467752267u64));
}
};
let tag = (Decoder249(_input))?;
let data = {
let sz = length as usize;
_input.start_slice(sz)?;
let ret = (Decoder250(_input))?;
_input.end_slice()?;
ret
};
let crc = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(png_idat { length, tag, data, crc })
}

/// d#238
fn Decoder_zlib_main(_input: &mut Parser<'_>) -> Result<zlib_main, ParseError> {
let compression_method_flags = {
let inner = {
let _packed_bits = _input.read_byte()?;
let compression_info = _packed_bits >> 4u8 & 15u8;
let compression_method = _packed_bits & 15u8;
zlib_main_compression_method_flags { compression_info, compression_method }
};
let is_valid = {
let method_info = inner;
method_info.compression_method == 8u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(5025197102194587315u64));
}
};
let flags = {
let _packed_bits = _input.read_byte()?;
let flevel = _packed_bits >> 6u8 & 3u8;
let fdict = _packed_bits >> 5u8 & 1u8 > 0u8;
let fcheck = _packed_bits & 31u8;
zlib_main_flags { flevel, fdict, fcheck }
};
let dict_id = if flags.fdict {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
Some(u32be(x))
} else {
None
};
let data = {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
ret
};
let adler32 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

/// d#239
fn Decoder_png_iend(_input: &mut Parser<'_>) -> Result<png_iend, ParseError> {
let length = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let is_valid = {
let length = inner;
length == 0u32
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(3810055094392728880u64));
}
};
let tag = (Decoder240(_input))?;
let crc = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(png_iend { length, tag, crc })
}

/// d#240
fn Decoder240(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8), ParseError> {
let arg0 = {
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(15252450768049745444u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 69 {
b
} else {
return Err(ParseError::ExcludedBranch(5599331855309773603u64));
}
};
let arg2 = {
let b = _input.read_byte()?;
if b == 78 {
b
} else {
return Err(ParseError::ExcludedBranch(16437491640759399344u64));
}
};
let arg3 = {
let b = _input.read_byte()?;
if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(2988545765690796708u64));
}
};
PResult::Ok((arg0, arg1, arg2, arg3))
}

/// d#241
fn Decoder_deflate_main(_input: &mut Parser<'_>) -> Result<deflate_main, ParseError> {
let blocks = {
let mut accum = Vec::new();
loop {
let next_elem = (Decoder_deflate_block(_input))?;
{
let tmp_cond = {
let x = &next_elem;
x.r#final == 1u8
};
if tmp_cond {
accum.push(next_elem);
break
} else {
accum.push(next_elem)
};

}
};
accum
};
let codes = (try_flat_map_vec(blocks.iter().cloned(), |x: deflate_block| PResult::Ok(match x.data {
deflate_block_data::uncompressed(ref y) => {
y.codes_values.clone()
},

deflate_block_data::fixed_huffman(ref y) => {
y.codes_values.clone()
},

deflate_block_data::dynamic_huffman(ref y) => {
y.codes_values.clone()
}
})))?;
let inflate = (try_flat_map_append_vec(codes.iter().cloned(), |tuple_var: (&[u8], deflate_uncompressed_codes_values)| PResult::Ok({
let (buffer, symbol) = tuple_var;
match symbol {
deflate_uncompressed_codes_values::literal(b) => {
[b].to_vec()
},

deflate_uncompressed_codes_values::reference(r) => {
{
let ix = (try_sub!((buffer.len()) as u32, r.distance as u32, 16200207902741715318u64)) as usize;
(slice_ext(buffer, ix..ix + ((r.length as u32) as usize))).to_vec()
}
}
}
})))?;
PResult::Ok(deflate_main { blocks, codes, inflate })
}

/// d#242
fn Decoder_deflate_block(_input: &mut Parser<'_>) -> Result<deflate_block, ParseError> {
let r#final = _input.read_byte()?;
let r#type = {
let bits = (_input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8
};
let data = match r#type {
0u8 => {
let inner = (Decoder_deflate_uncompressed(_input))?;
deflate_block_data::uncompressed(inner)
},

1u8 => {
let inner = (Decoder_deflate_fixed_huffman(_input))?;
deflate_block_data::fixed_huffman(inner)
},

2u8 => {
let inner = (Decoder_deflate_dynamic_huffman(_input))?;
deflate_block_data::dynamic_huffman(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(deflate_block { r#final, r#type, data })
}

/// d#243
fn Decoder_deflate_uncompressed(_input: &mut Parser<'_>) -> Result<deflate_uncompressed, ParseError> {
_input.skip_align(8)?;
let len = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16 | (bits.10 as u16) << 10u16 | (bits.11 as u16) << 11u16 | (bits.12 as u16) << 12u16 | (bits.13 as u16) << 13u16 | (bits.14 as u16) << 14u16 | (bits.15 as u16) << 15u16
};
let nlen = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16 | (bits.10 as u16) << 10u16 | (bits.11 as u16) << 11u16 | (bits.12 as u16) << 12u16 | (bits.13 as u16) << 13u16 | (bits.14 as u16) << 14u16 | (bits.15 as u16) << 15u16
};
let bytes = {
let mut accum = Vec::new();
for _ in 0..len {
let next_elem = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8 | bits.5 << 5u8 | bits.6 << 6u8 | bits.7 << 7u8
};
accum.push(next_elem)
};
accum
};
let codes_values = (try_flat_map_vec(bytes.iter().cloned(), |x: u8| PResult::Ok([deflate_uncompressed_codes_values::literal(x)].to_vec())))?;
PResult::Ok(deflate_uncompressed { len, nlen, bytes, codes_values })
}

/// d#244
fn Decoder_deflate_fixed_huffman(_input: &mut Parser<'_>) -> Result<deflate_fixed_huffman, ParseError> {
let codes = {
let format = parse_huffman([8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8].to_vec(), None);
let mut accum = Vec::new();
loop {
let next_elem = {
let code = (format(_input))?;
let extra = match code {
257u16 => {
let length_extra_bits = 0u8;
let length = 3u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

258u16 => {
let length_extra_bits = 0u8;
let length = 4u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

259u16 => {
let length_extra_bits = 0u8;
let length = 5u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

260u16 => {
let length_extra_bits = 0u8;
let length = 6u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

261u16 => {
let length_extra_bits = 0u8;
let length = 7u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

262u16 => {
let length_extra_bits = 0u8;
let length = 8u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

263u16 => {
let length_extra_bits = 0u8;
let length = 9u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

264u16 => {
let length_extra_bits = 0u8;
let length = 10u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

265u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?,);
bits.0
};
let length = 11u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

266u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?,);
bits.0
};
let length = 13u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

267u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?,);
bits.0
};
let length = 15u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

268u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?,);
bits.0
};
let length = 17u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

269u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8
};
let length = 19u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

270u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8
};
let length = 23u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

271u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8
};
let length = 27u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

272u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8
};
let length = 31u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

273u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8
};
let length = 35u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

274u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8
};
let length = 43u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

275u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8
};
let length = 51u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

276u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8
};
let length = 59u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

277u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8
};
let length = 67u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

278u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8
};
let length = 83u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

279u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8
};
let length = 99u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

280u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8
};
let length = 115u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

281u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8
};
let length = 131u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

282u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8
};
let length = 163u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

283u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8
};
let length = 195u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

284u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8
};
let length = 227u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

285u16 => {
let length_extra_bits = 0u8;
let length = 258u16 + (length_extra_bits as u16);
let distance_code = {
let bits = {
let mut accum = Vec::new();
for _ in 0..5u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
bits[0u32 as usize] << 4u8 | bits[1u32 as usize] << 3u8 | bits[2u32 as usize] << 2u8 | bits[3u32 as usize] << 1u8 | bits[4u32 as usize]
};
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code as u16))?;
Some(deflate_fixed_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

286u16..=287u16 => {
None
},

_ => {
None
}
};
deflate_fixed_huffman_codes { code, extra }
};
{
let tmp_cond = {
let x = &next_elem;
(x.code as u16) == 256u16
};
if tmp_cond {
accum.push(next_elem);
break
} else {
accum.push(next_elem)
};

}
};
accum
};
let codes_values = (try_flat_map_vec(codes.iter().cloned(), |x: deflate_fixed_huffman_codes| PResult::Ok(match x.code {
256u16 => {
[].to_vec()
},

257u16..=285u16 => {
match x.extra {
Some(ref rec) => {
[deflate_uncompressed_codes_values::reference(deflate_fixed_huffman_codes_values_reference { length: rec.length, distance: rec.distance_record.distance })].to_vec()
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
[deflate_uncompressed_codes_values::literal(x.code as u8)].to_vec()
}
})))?;
PResult::Ok(deflate_fixed_huffman { codes, codes_values })
}

/// d#245
fn Decoder_deflate_dynamic_huffman(_input: &mut Parser<'_>) -> Result<deflate_dynamic_huffman, ParseError> {
let hlit = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8
};
let hdist = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8
};
let hclen = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8
};
let code_length_alphabet_code_lengths = {
let mut accum = Vec::new();
for _ in 0..hclen + 4u8 {
let next_elem = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8
};
accum.push(next_elem)
};
accum
};
let literal_length_distance_alphabet_code_lengths = (Decoder246(_input, hlit, hdist, &code_length_alphabet_code_lengths))?;
let literal_length_distance_alphabet_code_lengths_value = (try_fold_map_curried(literal_length_distance_alphabet_code_lengths.iter().cloned(), None, |tuple_var: (Option<u8>, deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths)| PResult::Ok({
let (last_symbol, cl_code_extra) = tuple_var;
match cl_code_extra.code as u8 {
16u8 => {
(last_symbol, dup32((cl_code_extra.extra + 3u8) as u32, match last_symbol {
Some(x) => {
x
},

_ => {
return Err(ParseError::ExcludedBranch(4672672775256824980u64));
}
}))
},

17u8 => {
(Some(0u8), dup32((cl_code_extra.extra + 3u8) as u32, 0u8))
},

18u8 => {
(Some(0u8), dup32((cl_code_extra.extra + 11u8) as u32, 0u8))
},

v => {
(Some(v), [v].to_vec())
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
let codes = {
let distance_alphabet_format = parse_huffman(distance_alphabet_code_lengths_value.clone(), None);
let literal_length_alphabet_format = parse_huffman(literal_length_alphabet_code_lengths_value.clone(), None);
let mut accum = Vec::new();
loop {
let next_elem = {
let code = (literal_length_alphabet_format(_input))?;
let extra = match code {
257u16 => {
let length_extra_bits = 0u8;
let length = 3u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

258u16 => {
let length_extra_bits = 0u8;
let length = 4u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

259u16 => {
let length_extra_bits = 0u8;
let length = 5u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

260u16 => {
let length_extra_bits = 0u8;
let length = 6u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

261u16 => {
let length_extra_bits = 0u8;
let length = 7u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

262u16 => {
let length_extra_bits = 0u8;
let length = 8u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

263u16 => {
let length_extra_bits = 0u8;
let length = 9u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

264u16 => {
let length_extra_bits = 0u8;
let length = 10u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

265u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?,);
bits.0
};
let length = 11u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

266u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?,);
bits.0
};
let length = 13u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

267u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?,);
bits.0
};
let length = 15u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

268u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?,);
bits.0
};
let length = 17u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

269u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8
};
let length = 19u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

270u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8
};
let length = 23u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

271u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8
};
let length = 27u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

272u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8
};
let length = 31u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

273u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8
};
let length = 35u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

274u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8
};
let length = 43u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

275u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8
};
let length = 51u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

276u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8
};
let length = 59u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

277u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8
};
let length = 67u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

278u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8
};
let length = 83u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

279u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8
};
let length = 99u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

280u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8
};
let length = 115u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

281u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8
};
let length = 131u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

282u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8
};
let length = 163u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

283u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8
};
let length = 195u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

284u16 => {
let length_extra_bits = {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8
};
let length = 227u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

285u16 => {
let length_extra_bits = 0u8;
let length = 258u16 + (length_extra_bits as u16);
let distance_code = (distance_alphabet_format(_input))?;
let distance_record = (Decoder_deflate_distance_record0(_input, distance_code))?;
Some(deflate_dynamic_huffman_codes_values { length_extra_bits, length, distance_code, distance_record })
},

286u16..=287u16 => {
None
},

_ => {
None
}
};
deflate_dynamic_huffman_codes { code, extra }
};
{
let tmp_cond = {
let x = &next_elem;
(x.code as u16) == 256u16
};
if tmp_cond {
accum.push(next_elem);
break
} else {
accum.push(next_elem)
};

}
};
accum
};
let codes_values = (try_flat_map_vec(codes.iter().cloned(), |x: deflate_dynamic_huffman_codes| PResult::Ok(match x.code {
256u16 => {
[].to_vec()
},

257u16..=285u16 => {
match x.extra {
Some(ref rec) => {
[deflate_uncompressed_codes_values::reference(deflate_fixed_huffman_codes_values_reference { length: rec.length, distance: rec.distance_record.distance })].to_vec()
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
[deflate_uncompressed_codes_values::literal(x.code as u8)].to_vec()
}
})))?;
PResult::Ok(deflate_dynamic_huffman { hlit, hdist, hclen, code_length_alphabet_code_lengths, literal_length_distance_alphabet_code_lengths, literal_length_distance_alphabet_code_lengths_value, literal_length_alphabet_code_lengths_value, distance_alphabet_code_lengths_value, codes, codes_values })
}

/// d#246
fn Decoder246(_input: &mut Parser<'_>, hlit: u8, hdist: u8, code_length_alphabet_code_lengths: &[u8]) -> Result<Vec<deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths>, ParseError> {
let code_length_alphabet_format = parse_huffman(code_length_alphabet_code_lengths.clone(), Some([16u8, 17u8, 18u8, 0u8, 8u8, 7u8, 9u8, 6u8, 10u8, 5u8, 11u8, 4u8, 12u8, 3u8, 13u8, 2u8, 14u8, 1u8, 15u8].to_vec()));
let mut accum = Vec::new();
loop {
let next_elem = {
let code = (code_length_alphabet_format(_input))?;
let extra = match code as u8 {
16u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8
},

17u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8
},

18u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
bits.0 | bits.1 << 1u8 | bits.2 << 2u8 | bits.3 << 3u8 | bits.4 << 4u8 | bits.5 << 5u8 | bits.6 << 6u8
},

_ => {
0u8
}
};
deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths { code, extra }
};
accum.push(next_elem);
{
let tmp_cond = {
let y = &accum;
(((try_fold_map_curried(y.iter().cloned(), None, |tuple_var: (Option<u8>, deflate_dynamic_huffman_literal_length_distance_alphabet_code_lengths)| PResult::Ok({
let (last_symbol, cl_code_extra) = tuple_var;
match cl_code_extra.code as u8 {
16u8 => {
(last_symbol, dup32((cl_code_extra.extra + 3u8) as u32, match last_symbol {
Some(x) => {
x
},

_ => {
return Err(ParseError::ExcludedBranch(17176374570344757031u64));
}
}))
},

17u8 => {
(Some(0u8), dup32((cl_code_extra.extra + 3u8) as u32, 0u8))
},

18u8 => {
(Some(0u8), dup32((cl_code_extra.extra + 11u8) as u32, 0u8))
},

v => {
(Some(v), [v].to_vec())
}
}
})))?.len()) as u32) >= ((hlit + hdist) as u32) + 258u32
};
if tmp_cond {
break
};

}
};
PResult::Ok(accum)
}

/// d#247
fn Decoder_deflate_distance_record0(_input: &mut Parser<'_>, distance_code: u16) -> Result<deflate_distance_record0, ParseError> {
PResult::Ok(match distance_code as u8 {
0u8 => {
(Decoder248(_input, 0u8, 1u16))?
},

1u8 => {
(Decoder248(_input, 0u8, 2u16))?
},

2u8 => {
(Decoder248(_input, 0u8, 3u16))?
},

3u8 => {
(Decoder248(_input, 0u8, 4u16))?
},

4u8 => {
(Decoder248(_input, 1u8, 5u16))?
},

5u8 => {
(Decoder248(_input, 1u8, 7u16))?
},

6u8 => {
(Decoder248(_input, 2u8, 9u16))?
},

7u8 => {
(Decoder248(_input, 2u8, 13u16))?
},

8u8 => {
(Decoder248(_input, 3u8, 17u16))?
},

9u8 => {
(Decoder248(_input, 3u8, 25u16))?
},

10u8 => {
(Decoder248(_input, 4u8, 33u16))?
},

11u8 => {
(Decoder248(_input, 4u8, 49u16))?
},

12u8 => {
(Decoder248(_input, 5u8, 65u16))?
},

13u8 => {
(Decoder248(_input, 5u8, 97u16))?
},

14u8 => {
(Decoder248(_input, 6u8, 129u16))?
},

15u8 => {
(Decoder248(_input, 6u8, 193u16))?
},

16u8 => {
(Decoder248(_input, 7u8, 257u16))?
},

17u8 => {
(Decoder248(_input, 7u8, 385u16))?
},

18u8 => {
(Decoder248(_input, 8u8, 513u16))?
},

19u8 => {
(Decoder248(_input, 8u8, 769u16))?
},

20u8 => {
(Decoder248(_input, 9u8, 1025u16))?
},

21u8 => {
(Decoder248(_input, 9u8, 1537u16))?
},

22u8 => {
(Decoder248(_input, 10u8, 2049u16))?
},

23u8 => {
(Decoder248(_input, 10u8, 3073u16))?
},

24u8 => {
(Decoder248(_input, 11u8, 4097u16))?
},

25u8 => {
(Decoder248(_input, 11u8, 6145u16))?
},

26u8 => {
(Decoder248(_input, 12u8, 8193u16))?
},

27u8 => {
(Decoder248(_input, 12u8, 12289u16))?
},

28u8 => {
(Decoder248(_input, 13u8, 16385u16))?
},

29u8 => {
(Decoder248(_input, 13u8, 24577u16))?
},

30u8..=31u8 => {
return Err(ParseError::FailToken(3566920116549027235u64));
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

/// d#248
fn Decoder248(_input: &mut Parser<'_>, extra_bits: u8, start: u16) -> Result<deflate_distance_record0, ParseError> {
let distance_extra_bits = match extra_bits {
0u8 => {
0u16
},

1u8 => {
let bits = (_input.read_byte()?,);
bits.0 as u16
},

2u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16
},

3u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16
},

4u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16
},

5u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16
},

6u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16
},

7u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16
},

8u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16
},

9u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16
},

10u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16
},

11u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16 | (bits.10 as u16) << 10u16
},

12u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16 | (bits.10 as u16) << 10u16 | (bits.11 as u16) << 11u16
},

13u8 => {
let bits = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
(bits.0 as u16) | (bits.1 as u16) << 1u16 | (bits.2 as u16) << 2u16 | (bits.3 as u16) << 3u16 | (bits.4 as u16) << 4u16 | (bits.5 as u16) << 5u16 | (bits.6 as u16) << 6u16 | (bits.7 as u16) << 7u16 | (bits.8 as u16) << 8u16 | (bits.9 as u16) << 9u16 | (bits.10 as u16) << 10u16 | (bits.11 as u16) << 11u16 | (bits.12 as u16) << 12u16
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let distance = start + distance_extra_bits;
PResult::Ok(deflate_distance_record0 { distance_extra_bits, distance })
}

/// d#249
fn Decoder249(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8), ParseError> {
let arg0 = {
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(7979287392867129207u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(3653195934333285574u64));
}
};
let arg2 = {
let b = _input.read_byte()?;
if b == 65 {
b
} else {
return Err(ParseError::ExcludedBranch(16671136947067655757u64));
}
};
let arg3 = {
let b = _input.read_byte()?;
if b == 84 {
b
} else {
return Err(ParseError::ExcludedBranch(10721249873135158334u64));
}
};
PResult::Ok((arg0, arg1, arg2, arg3))
}

/// d#250
fn Decoder250(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
}

/// d#251
fn Decoder251(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
{
_input.open_peek_not_context();
let res = (|| (|| {
_input.start_alt();
let res = (|| {
{
let arg0 = {
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(8898504689444561451u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(441240706992005484u64));
}
};
let arg2 = {
let b = _input.read_byte()?;
if b == 65 {
b
} else {
return Err(ParseError::ExcludedBranch(3950014938140253048u64));
}
};
let arg3 = {
let b = _input.read_byte()?;
if b == 84 {
b
} else {
return Err(ParseError::ExcludedBranch(10046433636842398056u64));
}
};
(arg0, arg1, arg2, arg3)
};
PResult::Ok(())
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
{
let arg0 = {
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(11079395374415646537u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 69 {
b
} else {
return Err(ParseError::ExcludedBranch(757122060916971772u64));
}
};
let arg2 = {
let b = _input.read_byte()?;
if b == 78 {
b
} else {
return Err(ParseError::ExcludedBranch(5986772336072340665u64));
}
};
let arg3 = {
let b = _input.read_byte()?;
if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(8812292064350598352u64));
}
};
(arg0, arg1, arg2, arg3)
};
PResult::Ok(())
})();
match res {
Ok(inner) => {
PResult::Ok(inner)
},

Err(_e) => {
Err(_e)
}
}
})())();
if res.is_err() {
_input.close_peek_not_context()?
} else {
return Err(ParseError::NegatedSuccess);
}
};
let mut accum = Vec::new();
for _ in 0..4u32 {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([0u64, 576460743847706622u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(10645729856418057640u64));
}
};
accum.push(next_elem)
};
PResult::Ok(accum)
}

/// d#252
fn Decoder252(_input: &mut Parser<'_>) -> Result<Vec<gif_color_table_entry>, ParseError> {
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
let r = _input.read_byte()?;
let g = _input.read_byte()?;
let b = _input.read_byte()?;
gif_color_table_entry { r, g, b }
};
accum.push(next_elem)
}
};
PResult::Ok(accum)
}

/// d#253
fn Decoder_png_trns(_input: &mut Parser<'_>, ihdr: png_ihdr) -> Result<png_trns, ParseError> {
PResult::Ok(match ihdr.data.color_type {
0u8 => {
let inner = {
let greyscale = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
png_trns_color_type_0 { greyscale }
};
png_trns::color_type_0(inner)
},

2u8 => {
let inner = {
let red = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let green = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let blue = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
png_trns_color_type_2 { red, green, blue }
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
let palette_index = _input.read_byte()?;
png_trns_color_type_3 { palette_index }
};
accum.push(next_elem)
} else {
break
}
};
accum
};
png_trns::color_type_3(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

/// d#254
fn Decoder_png_chrm(_input: &mut Parser<'_>) -> Result<png_chrm, ParseError> {
let whitepoint_x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let whitepoint_y = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let red_x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let red_y = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let green_x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let green_y = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let blue_x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let blue_y = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(png_chrm { whitepoint_x, whitepoint_y, red_x, red_y, green_x, green_y, blue_x, blue_y })
}

/// d#255
fn Decoder_png_gama(_input: &mut Parser<'_>) -> Result<png_gama, ParseError> {
let gamma = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(png_gama { gamma })
}

/// d#256
fn Decoder_png_iccp(_input: &mut Parser<'_>) -> Result<png_iccp, ParseError> {
let profile_name = {
let val = (Decoder279(_input))?;
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(2908689796368760670u64));
}
};
val.clone()
};
let compression_method = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(4316175446384649956u64));
}
};
let compressed_profile = (Decoder280(_input))?;
PResult::Ok(png_iccp { profile_name, compression_method, compressed_profile })
}

/// d#257
fn Decoder_png_sbit(_input: &mut Parser<'_>, ihdr: png_ihdr) -> Result<png_sbit, ParseError> {
PResult::Ok(match ihdr.data.color_type {
0u8 => {
let inner = {
let sig_greyscale_bits = _input.read_byte()?;
png_sbit_color_type_0 { sig_greyscale_bits }
};
png_sbit::color_type_0(inner)
},

2u8 => {
let inner = {
let sig_red_bits = _input.read_byte()?;
let sig_green_bits = _input.read_byte()?;
let sig_blue_bits = _input.read_byte()?;
png_sbit_color_type_2 { sig_red_bits, sig_green_bits, sig_blue_bits }
};
png_sbit::color_type_2(inner)
},

3u8 => {
let inner = {
let sig_red_bits = _input.read_byte()?;
let sig_green_bits = _input.read_byte()?;
let sig_blue_bits = _input.read_byte()?;
png_sbit_color_type_2 { sig_red_bits, sig_green_bits, sig_blue_bits }
};
png_sbit::color_type_3(inner)
},

4u8 => {
let inner = {
let sig_greyscale_bits = _input.read_byte()?;
let sig_alpha_bits = _input.read_byte()?;
png_sbit_color_type_4 { sig_greyscale_bits, sig_alpha_bits }
};
png_sbit::color_type_4(inner)
},

6u8 => {
let inner = {
let sig_red_bits = _input.read_byte()?;
let sig_green_bits = _input.read_byte()?;
let sig_blue_bits = _input.read_byte()?;
let sig_alpha_bits = _input.read_byte()?;
png_sbit_color_type_6 { sig_red_bits, sig_green_bits, sig_blue_bits, sig_alpha_bits }
};
png_sbit::color_type_6(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

/// d#258
fn Decoder_png_srgb(_input: &mut Parser<'_>) -> Result<png_srgb, ParseError> {
let rendering_intent = {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x <= 3u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13311790038092155306u64));
}
};
PResult::Ok(png_srgb { rendering_intent })
}

/// d#259
fn Decoder_png_itxt(_input: &mut Parser<'_>) -> Result<png_itxt, ParseError> {
let keyword = {
let val = (Decoder273(_input))?;
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(4598583460226006268u64));
}
};
val.clone()
};
let compression_flag = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([3u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(275550262640764009u64));
}
};
let compression_method = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(11490274700962832028u64));
}
};
let language_tag = {
let chars = {
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
return Err(ParseError::ExcludedBranch(15680765559661576738u64));
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
return Err(ParseError::ExcludedBranch(1587806253186841834u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(11297314001547702431u64));
}
};
chars.clone()
};
let translated_keyword = {
let val = (Decoder274(_input))?;
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(18399269270080151498u64));
}
};
val.clone()
};
let text = match compression_flag == 1u8 {
true => {
((|| {
_input.start_alt();
let res = (|| {
let inner = {
let inner = {
let zlib = (Decoder275(_input))?;
let mut buf_parser = Parser::new(slice_all(&zlib.data.inflate));
let buf_input = &mut buf_parser;
(Decoder276(buf_input))?
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
png_itxt_text_compressed::invalid(inner)
};
PResult::Ok(png_itxt_text::compressed(inner))
})();
match res {
Ok(inner) => {
PResult::Ok(inner)
},

Err(_e) => {
Err(_e)
}
}
})())?
},

false => {
let inner = (Decoder277(_input))?;
png_itxt_text::uncompressed(inner)
}
};
PResult::Ok(png_itxt { keyword, compression_flag, compression_method, language_tag, translated_keyword, text })
}

/// d#260
fn Decoder_png_text(_input: &mut Parser<'_>) -> Result<png_text, ParseError> {
let keyword = {
let val = (Decoder272(_input))?;
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(30874382969105279u64));
}
};
val.clone()
};
let text = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(png_text { keyword, text })
}

/// d#261
fn Decoder_png_ztxt(_input: &mut Parser<'_>) -> Result<png_ztxt, ParseError> {
let keyword = {
let val = (Decoder268(_input))?;
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(9791114990321288281u64));
}
};
val.clone()
};
let compression_method = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(1595897747104696027u64));
}
};
let compressed_text = {
let zlib = (Decoder269(_input))?;
let mut buf_parser = Parser::new(slice_all(&zlib.data.inflate));
let buf_input = &mut buf_parser;
(Decoder270(buf_input))?
};
PResult::Ok(png_ztxt { keyword, compression_method, compressed_text })
}

/// d#262
fn Decoder_png_bkgd(_input: &mut Parser<'_>, ihdr: png_ihdr) -> Result<png_bkgd, ParseError> {
PResult::Ok(match ihdr.data.color_type {
0u8 => {
let inner = {
let greyscale = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
png_trns_color_type_0 { greyscale }
};
png_bkgd::color_type_0(inner)
},

4u8 => {
let inner = {
let greyscale = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
png_trns_color_type_0 { greyscale }
};
png_bkgd::color_type_4(inner)
},

2u8 => {
let inner = {
let red = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let green = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let blue = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
png_trns_color_type_2 { red, green, blue }
};
png_bkgd::color_type_2(inner)
},

6u8 => {
let inner = {
let red = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let green = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let blue = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
png_trns_color_type_2 { red, green, blue }
};
png_bkgd::color_type_6(inner)
},

3u8 => {
let inner = {
let palette_index = _input.read_byte()?;
png_trns_color_type_3 { palette_index }
};
png_bkgd::color_type_3(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

/// d#263
fn Decoder_png_hist(_input: &mut Parser<'_>) -> Result<png_hist, ParseError> {
let histogram = {
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
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(png_hist { histogram })
}

/// d#264
fn Decoder_png_phys(_input: &mut Parser<'_>) -> Result<png_phys, ParseError> {
let pixels_per_unit_x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let pixels_per_unit_y = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let unit_specifier = _input.read_byte()?;
PResult::Ok(png_phys { pixels_per_unit_x, pixels_per_unit_y, unit_specifier })
}

/// d#265
fn Decoder_png_splt(_input: &mut Parser<'_>) -> Result<png_splt, ParseError> {
let palette_name = {
let val = (Decoder267(_input))?;
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(2481175643332430741u64));
}
};
val.clone()
};
let sample_depth = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([65792u64, 0u64, 0u64, 0u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(15702070659753069395u64));
}
};
let palette = match sample_depth {
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
let red = _input.read_byte()?;
let green = _input.read_byte()?;
let blue = _input.read_byte()?;
let alpha = _input.read_byte()?;
let frequency = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
png_splt_palette_sample_depth_u8 { red, green, blue, alpha, frequency }
};
accum.push(next_elem)
} else {
break
}
};
accum
};
png_splt_palette::sample_depth_u8(inner)
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
let red = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let green = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let blue = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let alpha = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let frequency = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
png_splt_palette_sample_depth_u16 { red, green, blue, alpha, frequency }
};
accum.push(next_elem)
} else {
break
}
};
accum
};
png_splt_palette::sample_depth_u16(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(png_splt { palette_name, sample_depth, palette })
}

/// d#266
fn Decoder_png_time(_input: &mut Parser<'_>) -> Result<png_time, ParseError> {
let year = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let month = _input.read_byte()?;
let day = _input.read_byte()?;
let hour = _input.read_byte()?;
let minute = _input.read_byte()?;
let second = _input.read_byte()?;
PResult::Ok(png_time { year, month, day, hour, minute, second })
}

/// d#267
fn Decoder267(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let reps_left = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
1
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
2
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
3
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
4
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
5
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
6
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
7
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
8
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
9
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
10
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
11
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
12
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
13
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
14
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
15
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
16
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
17
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
18
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
19
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
20
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
21
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
22
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
23
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
24
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
25
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
26
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
27
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
28
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
29
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
30
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
31
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
32
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
33
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
34
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
35
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
36
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
37
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
38
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
39
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
40
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
41
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
42
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
43
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
44
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
45
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
46
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
47
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
48
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
49
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
50
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
51
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
52
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
53
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
54
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
55
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
56
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
57
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
58
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
59
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
60
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
61
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
62
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
63
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
64
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
65
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
66
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
67
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
68
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
69
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
70
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
71
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
72
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
73
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
74
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
75
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
76
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
77
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
78
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
79
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
}
},

_ => {
return Err(ParseError::ExcludedBranch(1977196682923428575u64));
}
}
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
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(reps_left == 0, accum.len(), 1u32 as usize, 79u32 as usize))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(2137824516769298342u64));
}
};
accum.push(next_elem)
}
};
PResult::Ok(accum)
}

/// d#268
fn Decoder268(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let reps_left = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
1
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
2
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
3
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
4
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
5
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
6
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
7
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
8
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
9
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
10
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
11
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
12
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
13
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
14
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
15
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
16
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
17
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
18
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
19
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
20
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
21
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
22
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
23
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
24
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
25
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
26
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
27
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
28
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
29
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
30
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
31
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
32
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
33
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
34
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
35
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
36
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
37
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
38
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
39
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
40
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
41
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
42
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
43
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
44
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
45
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
46
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
47
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
48
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
49
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
50
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
51
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
52
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
53
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
54
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
55
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
56
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
57
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
58
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
59
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
60
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
61
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
62
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
63
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
64
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
65
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
66
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
67
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
68
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
69
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
70
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
71
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
72
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
73
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
74
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
75
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
76
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
77
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
78
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
79
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
}
},

_ => {
return Err(ParseError::ExcludedBranch(7572218778908935167u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12520942526725743695u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5905249956584638130u64));
}
}
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
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(reps_left == 0, accum.len(), 1u32 as usize, 79u32 as usize))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(974442582679666596u64));
}
};
accum.push(next_elem)
}
};
PResult::Ok(accum)
}

/// d#269
fn Decoder269(_input: &mut Parser<'_>) -> Result<zlib_main, ParseError> {
let compression_method_flags = {
let inner = {
let _packed_bits = _input.read_byte()?;
let compression_info = _packed_bits >> 4u8 & 15u8;
let compression_method = _packed_bits & 15u8;
zlib_main_compression_method_flags { compression_info, compression_method }
};
let is_valid = {
let method_info = inner;
method_info.compression_method == 8u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(4054889295781446756u64));
}
};
let flags = {
let _packed_bits = _input.read_byte()?;
let flevel = _packed_bits >> 6u8 & 3u8;
let fdict = _packed_bits >> 5u8 & 1u8 > 0u8;
let fcheck = _packed_bits & 31u8;
zlib_main_flags { flevel, fdict, fcheck }
};
let dict_id = if flags.fdict {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
Some(u32be(x))
} else {
None
};
let data = {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
ret
};
let adler32 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

/// d#270
fn Decoder270(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
Decoder271(_input)
}

/// d#271
fn Decoder271(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744073709551614u64, 18446744073709551615u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 4294967292u64])).contains(byte)) => {
0
},

224u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 35175782154240u64])).contains(byte)) => {
0
},

237u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 211106232532992u64])).contains(byte)) => {
0
},

240u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 3940649673949184u64])).contains(byte)) => {
0
},

244u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(14775695284177607002u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder18(_input))?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
}

/// d#272
fn Decoder272(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let reps_left = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
1
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
2
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
3
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
4
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
5
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
6
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
7
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
8
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
9
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
10
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
11
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
12
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
13
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
14
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
15
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
16
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
17
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
18
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
19
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
20
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
21
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
22
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
23
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
24
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
25
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
26
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
27
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
28
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
29
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
30
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
31
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
32
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
33
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
34
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
35
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
36
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
37
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
38
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
39
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
40
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
41
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
42
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
43
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
44
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
45
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
46
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
47
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
48
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
49
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
50
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
51
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
52
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
53
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
54
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
55
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
56
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
57
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
58
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
59
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
60
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
61
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
62
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
63
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
64
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
65
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
66
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
67
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
68
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
69
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
70
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
71
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
72
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
73
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
74
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
75
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
76
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
77
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
78
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
79
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
}
},

_ => {
return Err(ParseError::ExcludedBranch(2930236810929305906u64));
}
}
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
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(reps_left == 0, accum.len(), 1u32 as usize, 79u32 as usize))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(14708340496221370872u64));
}
};
accum.push(next_elem)
}
};
PResult::Ok(accum)
}

/// d#273
fn Decoder273(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let reps_left = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
1
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
2
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
3
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
4
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
5
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
6
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
7
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
8
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
9
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
10
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
11
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
12
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
13
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
14
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
15
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
16
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
17
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
18
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
19
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
20
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
21
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
22
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
23
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
24
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
25
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
26
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
27
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
28
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
29
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
30
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
31
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
32
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
33
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
34
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
35
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
36
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
37
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
38
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
39
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
40
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
41
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
42
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
43
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
44
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
45
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
46
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
47
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
48
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
49
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
50
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
51
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
52
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
53
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
54
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
55
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
56
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
57
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
58
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
59
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
60
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
61
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
62
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
63
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
64
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
65
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
66
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
67
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
68
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
69
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
70
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
71
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
72
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
73
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
74
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
75
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
76
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
77
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
78
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
79
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
}
},

_ => {
return Err(ParseError::ExcludedBranch(13045651918551022996u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12507789284617859731u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4636203914394120912u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12770885481695079655u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5713736954960046494u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5286459060741837465u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10857757036842967573u64));
}
}
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
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(reps_left == 0, accum.len(), 1u32 as usize, 79u32 as usize))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(15092052115449480204u64));
}
};
accum.push(next_elem)
}
};
PResult::Ok(accum)
}

/// d#274
fn Decoder274(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([18446744073709551614u64, 18446744073709551615u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 4294967292u64])).contains(byte)) => {
0
},

224u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 35175782154240u64])).contains(byte)) => {
0
},

237u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 211106232532992u64])).contains(byte)) => {
0
},

240u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 3940649673949184u64])).contains(byte)) => {
0
},

244u8 => {
0
},

0u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(5644646321622228807u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder19(_input))?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
}

/// d#275
fn Decoder275(_input: &mut Parser<'_>) -> Result<zlib_main, ParseError> {
let compression_method_flags = {
let inner = {
let _packed_bits = _input.read_byte()?;
let compression_info = _packed_bits >> 4u8 & 15u8;
let compression_method = _packed_bits & 15u8;
zlib_main_compression_method_flags { compression_info, compression_method }
};
let is_valid = {
let method_info = inner;
method_info.compression_method == 8u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12876869868341544086u64));
}
};
let flags = {
let _packed_bits = _input.read_byte()?;
let flevel = _packed_bits >> 6u8 & 3u8;
let fdict = _packed_bits >> 5u8 & 1u8 > 0u8;
let fcheck = _packed_bits & 31u8;
zlib_main_flags { flevel, fdict, fcheck }
};
let dict_id = if flags.fdict {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
Some(u32be(x))
} else {
None
};
let data = {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
ret
};
let adler32 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

/// d#276
fn Decoder276(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
byte if ((ByteSet::from_bits([18446744073709551614u64, 18446744073709551615u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 4294967292u64])).contains(byte)) => {
0
},

224u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 35175782154240u64])).contains(byte)) => {
0
},

237u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 211106232532992u64])).contains(byte)) => {
0
},

240u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 3940649673949184u64])).contains(byte)) => {
0
},

244u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(6757475294452472580u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder19(_input))?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
}

/// d#277
fn Decoder277(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
Decoder278(_input)
}

/// d#278
fn Decoder278(_input: &mut Parser<'_>) -> Result<Vec<char>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let matching_ix = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744073709551614u64, 18446744073709551615u64, 0u64, 0u64])).contains(byte)) => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 4294967292u64])).contains(byte)) => {
0
},

224u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 35175782154240u64])).contains(byte)) => {
0
},

237u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 211106232532992u64])).contains(byte)) => {
0
},

240u8 => {
0
},

byte if ((ByteSet::from_bits([0u64, 0u64, 0u64, 3940649673949184u64])).contains(byte)) => {
0
},

244u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(5316076842439816359u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder18(_input))?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
}

/// d#279
fn Decoder279(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let reps_left = {
_input.open_peek_context();
{
let ret = match _input.read_byte()? {
0u8 => {
0
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
1
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
2
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
3
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
4
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
5
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
6
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
7
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
8
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
9
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
10
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
11
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
12
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
13
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
14
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
15
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
16
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
17
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
18
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
19
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
20
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
21
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
22
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
23
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
24
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
25
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
26
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
27
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
28
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
29
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
30
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
31
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
32
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
33
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
34
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
35
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
36
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
37
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
38
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
39
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
40
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
41
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
42
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
43
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
44
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
45
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
46
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
47
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
48
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
49
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
50
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
51
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
52
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
53
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
54
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
55
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
56
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
57
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
58
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
59
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
60
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
61
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
62
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
63
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
64
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
65
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
66
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
67
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
68
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
69
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
70
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
71
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
72
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
73
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
74
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
75
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
76
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
77
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
match _input.read_byte()? {
0u8 => {
78
},

byte if ((ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(byte)) => {
79
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
}
},

_ => {
return Err(ParseError::ExcludedBranch(15321253101048235163u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14285291594842403582u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7161350271661739096u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14445026714989735755u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7573348369521592997u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(2912602219218223367u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(15885886181646629118u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13594649006224468849u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14349391981174483355u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10927780046062734427u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13819997351842221509u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(11959902760514814588u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14114750189009003637u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12268471799124361536u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9620443865397033050u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3999778194527899420u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(3933597635930613605u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(5638313771627483501u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(1105171745447487568u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(646997207007428137u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6351919260209655786u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(552953513858366451u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17332407069754301555u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14279133095670415871u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(7347749870002275487u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(10748847049349746078u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14220958138979104104u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(17741760072369420240u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(12223407337737822059u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6072942808717419822u64));
}
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(reps_left == 0, accum.len(), 1u32 as usize, 79u32 as usize))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if (ByteSet::from_bits([18446744069414584320u64, 9223372036854775807u64, 18446744065119617024u64, 18446744073709551615u64])).contains(b) {
b
} else {
return Err(ParseError::ExcludedBranch(15560056883377919848u64));
}
};
accum.push(next_elem)
}
};
PResult::Ok(accum)
}

/// d#280
fn Decoder280(_input: &mut Parser<'_>) -> Result<zlib_main, ParseError> {
let compression_method_flags = {
let inner = {
let _packed_bits = _input.read_byte()?;
let compression_info = _packed_bits >> 4u8 & 15u8;
let compression_method = _packed_bits & 15u8;
zlib_main_compression_method_flags { compression_info, compression_method }
};
let is_valid = {
let method_info = inner;
method_info.compression_method == 8u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7977419944699061624u64));
}
};
let flags = {
let _packed_bits = _input.read_byte()?;
let flevel = _packed_bits >> 6u8 & 3u8;
let fdict = _packed_bits >> 5u8 & 1u8 > 0u8;
let fcheck = _packed_bits & 31u8;
zlib_main_flags { flevel, fdict, fcheck }
};
let dict_id = if flags.fdict {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
Some(u32be(x))
} else {
None
};
let data = {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
ret
};
let adler32 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(zlib_main { compression_method_flags, flags, dict_id, data, adler32 })
}

/// d#281
fn Decoder281(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8), ParseError> {
let arg0 = {
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(8323252642575612937u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 72 {
b
} else {
return Err(ParseError::ExcludedBranch(11678443062630698028u64));
}
};
let arg2 = {
let b = _input.read_byte()?;
if b == 68 {
b
} else {
return Err(ParseError::ExcludedBranch(14832405617500840744u64));
}
};
let arg3 = {
let b = _input.read_byte()?;
if b == 82 {
b
} else {
return Err(ParseError::ExcludedBranch(1398536204687975789u64));
}
};
PResult::Ok((arg0, arg1, arg2, arg3))
}

/// d#282
fn Decoder_png_ihdr_data(_input: &mut Parser<'_>) -> Result<png_ihdr_data, ParseError> {
let width = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let height = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let bit_depth = _input.read_byte()?;
let color_type = _input.read_byte()?;
let compression_method = _input.read_byte()?;
let filter_method = _input.read_byte()?;
let interlace_method = _input.read_byte()?;
PResult::Ok(png_ihdr_data { width, height, bit_depth, color_type, compression_method, filter_method, interlace_method })
}

/// d#283
fn Decoder_mpeg4_atom(_input: &mut Parser<'_>) -> Result<mpeg4_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 4152914559762097168u64)
},

_ => {
(try_sub!(size_field, 8u32, 734991270787736827u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = match r#type {
(102u8, 116u8, 121u8, 112u8) => {
let inner = (Decoder_mpeg4_ftyp_data(_input))?;
mpeg4_atom_data::ftyp(inner)
},

(102u8, 114u8, 101u8, 101u8) => {
mpeg4_atom_data::free
},

(109u8, 100u8, 97u8, 116u8) => {
mpeg4_atom_data::mdat
},

(109u8, 101u8, 116u8, 97u8) => {
let arg0 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let arg1 = {
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
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_atom_data::meta(arg0, arg1)
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
accum.push(next_elem)
} else {
break
}
};
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_atom { size_field, r#type, size, data })
}

/// d#284
fn Decoder284(_input: &mut Parser<'_>) -> Result<(u8, u8, u8, u8), ParseError> {
PResult::Ok((_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?))
}

/// d#285
fn Decoder_mpeg4_ftyp_data(_input: &mut Parser<'_>) -> Result<mpeg4_ftyp_data, ParseError> {
let major_brand = (Decoder284(_input))?;
let minor_version = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let compatible_brands = {
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
let next_elem = (Decoder284(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(mpeg4_ftyp_data { major_brand, minor_version, compatible_brands })
}

/// d#286
fn Decoder_mpeg4_meta_atom(_input: &mut Parser<'_>) -> Result<mpeg4_meta_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 5095757730543354711u64)
},

_ => {
(try_sub!(size_field, 8u32, 18167425999583150549u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = (Decoder_mpeg4_meta_atom_data(_input, r#type))?;
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_meta_atom { size_field, r#type, size, data })
}

/// d#287
fn Decoder_mpeg4_moov_atom(_input: &mut Parser<'_>) -> Result<mpeg4_moov_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 6515957116553005671u64)
},

_ => {
(try_sub!(size_field, 8u32, 1644793874183523166u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = match r#type {
(109u8, 118u8, 104u8, 100u8) => {
let inner = (Decoder_mpeg4_mvhd_data(_input))?;
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
accum.push(next_elem)
} else {
break
}
};
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
accum.push(next_elem)
} else {
break
}
};
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_moov_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_moov_atom { size_field, r#type, size, data })
}

/// d#288
fn Decoder_mpeg4_mvhd_data(_input: &mut Parser<'_>) -> Result<mpeg4_mvhd_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let fields = match version {
0u8 => {
let inner = {
let creation_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let modification_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let timescale = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let duration = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
mpeg4_mvhd_data_fields_version0 { creation_time, modification_time, timescale, duration }
};
mpeg4_mvhd_data_fields::version0(inner)
},

1u8 => {
let inner = {
let creation_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
let modification_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
let timescale = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let duration = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
mpeg4_mvhd_data_fields_version1 { creation_time, modification_time, timescale, duration }
};
mpeg4_mvhd_data_fields::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let rate = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let volume = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let reserved1 = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let reserved2 = {
let arg0 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let arg1 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
(arg0, arg1)
};
let matrix = {
let mut accum = Vec::new();
for _ in 0..9u8 {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
accum.push(next_elem)
};
accum
};
let pre_defined = {
let mut accum = Vec::new();
for _ in 0..6u8 {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
accum.push(next_elem)
};
accum
};
let next_track_ID = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(mpeg4_mvhd_data { version, flags, fields, rate, volume, reserved1, reserved2, matrix, pre_defined, next_track_ID })
}

/// d#289
fn Decoder_mpeg4_trak_atom(_input: &mut Parser<'_>) -> Result<mpeg4_trak_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 11789784461021426583u64)
},

_ => {
(try_sub!(size_field, 8u32, 16396082708135795071u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = match r#type {
(116u8, 107u8, 104u8, 100u8) => {
let inner = (Decoder_mpeg4_tkhd_data(_input))?;
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
accum.push(next_elem)
} else {
break
}
};
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
accum.push(next_elem)
} else {
break
}
};
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_trak_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_trak_atom { size_field, r#type, size, data })
}

/// d#290
fn Decoder_mpeg4_udta_atom(_input: &mut Parser<'_>) -> Result<mpeg4_udta_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 15337804701822118436u64)
},

_ => {
(try_sub!(size_field, 8u32, 5546123200965512193u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = match r#type {
(109u8, 101u8, 116u8, 97u8) => {
let arg0 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let arg1 = {
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
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_udta_atom_data::meta(arg0, arg1)
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_udta_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_udta_atom { size_field, r#type, size, data })
}

/// d#291
fn Decoder_mpeg4_tkhd_data(_input: &mut Parser<'_>) -> Result<mpeg4_tkhd_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let fields = match version {
0u8 => {
let inner = {
let creation_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let modification_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let track_ID = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let reserved = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let duration = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
mpeg4_tkhd_data_fields_version0 { creation_time, modification_time, track_ID, reserved, duration }
};
mpeg4_tkhd_data_fields::version0(inner)
},

1u8 => {
let inner = {
let creation_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
let modification_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
let track_ID = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let reserved = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let duration = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
mpeg4_tkhd_data_fields_version1 { creation_time, modification_time, track_ID, reserved, duration }
};
mpeg4_tkhd_data_fields::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let reserved2 = {
let arg0 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let arg1 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
(arg0, arg1)
};
let layer = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let alternate_group = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let volume = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let reserved1 = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let matrix = {
let mut accum = Vec::new();
for _ in 0..9u8 {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
accum.push(next_elem)
};
accum
};
let width = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let height = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(mpeg4_tkhd_data { version, flags, fields, reserved2, layer, alternate_group, volume, reserved1, matrix, width, height })
}

/// d#292
fn Decoder_mpeg4_edts_atom(_input: &mut Parser<'_>) -> Result<mpeg4_edts_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 8306706226429158303u64)
},

_ => {
(try_sub!(size_field, 8u32, 9403121491749669432u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = match r#type {
(101u8, 108u8, 115u8, 116u8) => {
let inner = (Decoder_mpeg4_elst_data(_input))?;
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_edts_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_edts_atom { size_field, r#type, size, data })
}

/// d#293
fn Decoder_mpeg4_mdia_atom(_input: &mut Parser<'_>) -> Result<mpeg4_mdia_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 7258731811542513498u64)
},

_ => {
(try_sub!(size_field, 8u32, 7981520858864097140u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = (Decoder_mpeg4_mdia_atom_data(_input, r#type))?;
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_mdia_atom { size_field, r#type, size, data })
}

/// d#294
fn Decoder_mpeg4_mdia_atom_data(_input: &mut Parser<'_>, r#type: (u8, u8, u8, u8)) -> Result<mpeg4_mdia_atom_data, ParseError> {
PResult::Ok(match r#type {
(104u8, 100u8, 108u8, 114u8) => {
let inner = (Decoder_mpeg4_mdia_hdlr_data(_input))?;
mpeg4_mdia_atom_data::hdlr(inner)
},

(109u8, 100u8, 104u8, 100u8) => {
let inner = (Decoder_mpeg4_mdhd_data(_input))?;
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
accum.push(next_elem)
} else {
break
}
};
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_mdia_atom_data::unknown(inner)
}
})
}

/// d#295
fn Decoder_mpeg4_mdia_hdlr_data(_input: &mut Parser<'_>) -> Result<mpeg4_mdia_hdlr_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let component_type = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let component_subtype = (Decoder284(_input))?;
let component_manufacturer = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let component_flags = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let component_flags_mask = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let component_name = {
let chars = {
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
return Err(ParseError::ExcludedBranch(1283209893442238385u64));
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
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(7474037925185307628u64));
}
};
chars.clone()
};
PResult::Ok(mpeg4_mdia_hdlr_data { version, flags, component_type, component_subtype, component_manufacturer, component_flags, component_flags_mask, component_name })
}

/// d#296
fn Decoder_mpeg4_mdhd_data(_input: &mut Parser<'_>) -> Result<mpeg4_mdhd_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let fields = match version {
0u8 => {
let inner = {
let creation_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let modification_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let timescale = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let duration = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
mpeg4_mvhd_data_fields_version0 { creation_time, modification_time, timescale, duration }
};
mpeg4_mvhd_data_fields::version0(inner)
},

1u8 => {
let inner = {
let creation_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
let modification_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
let timescale = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let duration = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
mpeg4_mvhd_data_fields_version1 { creation_time, modification_time, timescale, duration }
};
mpeg4_mvhd_data_fields::version1(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let language = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let pre_defined = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
PResult::Ok(mpeg4_mdhd_data { version, flags, fields, language, pre_defined })
}

/// d#297
fn Decoder_mpeg4_minf_atom(_input: &mut Parser<'_>) -> Result<mpeg4_minf_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 17780439059155340308u64)
},

_ => {
(try_sub!(size_field, 8u32, 17948395312093823900u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = match r#type {
(118u8, 109u8, 104u8, 100u8) => {
let inner = (Decoder_mpeg4_vmhd_data(_input))?;
mpeg4_minf_atom_data::vmhd(inner)
},

(115u8, 109u8, 104u8, 100u8) => {
let inner = (Decoder_mpeg4_smhd_data(_input))?;
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
accum.push(next_elem)
} else {
break
}
};
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
accum.push(next_elem)
} else {
break
}
};
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_minf_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_minf_atom { size_field, r#type, size, data })
}

/// d#298
fn Decoder_mpeg4_vmhd_data(_input: &mut Parser<'_>) -> Result<mpeg4_vmhd_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let graphicsmode = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let opcolor = {
let mut accum = Vec::new();
for _ in 0..3u8 {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_vmhd_data { version, flags, graphicsmode, opcolor })
}

/// d#299
fn Decoder_mpeg4_smhd_data(_input: &mut Parser<'_>) -> Result<mpeg4_smhd_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let balance = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let reserved = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
PResult::Ok(mpeg4_smhd_data { version, flags, balance, reserved })
}

/// d#300
fn Decoder_mpeg4_dinf_atom(_input: &mut Parser<'_>) -> Result<mpeg4_dinf_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 10170756778737993654u64)
},

_ => {
(try_sub!(size_field, 8u32, 7370180348639650351u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = match r#type {
(100u8, 114u8, 101u8, 102u8) => {
let inner = (Decoder_mpeg4_dref_data(_input))?;
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_dinf_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_dinf_atom { size_field, r#type, size, data })
}

/// d#301
fn Decoder_mpeg4_stbl_atom(_input: &mut Parser<'_>) -> Result<mpeg4_stbl_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 8902666087419502325u64)
},

_ => {
(try_sub!(size_field, 8u32, 9851243859021733611u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = match r#type {
(115u8, 116u8, 115u8, 100u8) => {
let inner = (Decoder_mpeg4_stsd_data(_input))?;
mpeg4_stbl_atom_data::stsd(inner)
},

(115u8, 116u8, 116u8, 115u8) => {
let inner = (Decoder_mpeg4_stts_data(_input))?;
mpeg4_stbl_atom_data::stts(inner)
},

(99u8, 116u8, 116u8, 115u8) => {
let inner = (Decoder_mpeg4_ctts_data(_input))?;
mpeg4_stbl_atom_data::ctts(inner)
},

(115u8, 116u8, 115u8, 115u8) => {
let inner = (Decoder_mpeg4_stss_data(_input))?;
mpeg4_stbl_atom_data::stss(inner)
},

(115u8, 116u8, 115u8, 99u8) => {
let inner = (Decoder_mpeg4_stsc_data(_input))?;
mpeg4_stbl_atom_data::stsc(inner)
},

(115u8, 116u8, 115u8, 122u8) => {
let inner = (Decoder_mpeg4_stsz_data(_input))?;
mpeg4_stbl_atom_data::stsz(inner)
},

(115u8, 116u8, 99u8, 111u8) => {
let inner = (Decoder_mpeg4_stco_data(_input))?;
mpeg4_stbl_atom_data::stco(inner)
},

(99u8, 111u8, 54u8, 52u8) => {
let inner = (Decoder_mpeg4_co64_data(_input))?;
mpeg4_stbl_atom_data::co64(inner)
},

(115u8, 103u8, 112u8, 100u8) => {
let inner = (Decoder_mpeg4_sgpd_data(_input))?;
mpeg4_stbl_atom_data::sgpd(inner)
},

(115u8, 98u8, 103u8, 112u8) => {
let inner = (Decoder_mpeg4_sbgp_data(_input))?;
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_stbl_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_stbl_atom { size_field, r#type, size, data })
}

/// d#302
fn Decoder_mpeg4_stsd_data(_input: &mut Parser<'_>) -> Result<mpeg4_stsd_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let sample_entries = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = (Decoder_mpeg4_dref_data_data(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_stsd_data { version, flags, entry_count, sample_entries })
}

/// d#303
fn Decoder_mpeg4_stts_data(_input: &mut Parser<'_>) -> Result<mpeg4_stts_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let sample_entries = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = (Decoder_mpeg4_stts_sample_entry(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_stts_data { version, flags, entry_count, sample_entries })
}

/// d#304
fn Decoder_mpeg4_ctts_data(_input: &mut Parser<'_>) -> Result<mpeg4_ctts_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let sample_entries = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = (Decoder_mpeg4_ctts_sample_entry(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_ctts_data { version, flags, entry_count, sample_entries })
}

/// d#305
fn Decoder_mpeg4_stss_data(_input: &mut Parser<'_>) -> Result<mpeg4_stss_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let sample_number = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_stss_data { version, flags, entry_count, sample_number })
}

/// d#306
fn Decoder_mpeg4_stsc_data(_input: &mut Parser<'_>) -> Result<mpeg4_stsc_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let chunk_entries = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = {
let first_chunk = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let samples_per_chunk = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let sample_description_index = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
mpeg4_stsc_data_chunk_entries { first_chunk, samples_per_chunk, sample_description_index }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_stsc_data { version, flags, entry_count, chunk_entries })
}

/// d#307
fn Decoder_mpeg4_stsz_data(_input: &mut Parser<'_>) -> Result<mpeg4_stsz_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let sample_size = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let sample_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let entry_size = if sample_size == 0u32 {
let mut accum = Vec::new();
for _ in 0..sample_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
accum.push(next_elem)
};
Some(accum)
} else {
None
};
PResult::Ok(mpeg4_stsz_data { version, flags, sample_size, sample_count, entry_size })
}

/// d#308
fn Decoder_mpeg4_stco_data(_input: &mut Parser<'_>) -> Result<mpeg4_stco_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let chunk_offset = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_stco_data { version, flags, entry_count, chunk_offset })
}

/// d#309
fn Decoder_mpeg4_co64_data(_input: &mut Parser<'_>) -> Result<mpeg4_co64_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let chunk_offset = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_co64_data { version, flags, entry_count, chunk_offset })
}

/// d#310
fn Decoder_mpeg4_sgpd_data(_input: &mut Parser<'_>) -> Result<mpeg4_sgpd_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let grouping_type = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let default_length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let sample_groups = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = {
let description_length = match default_length == 0u32 {
true => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
},

false => {
default_length
}
};
let sample_group_entry = {
let mut accum = Vec::new();
for _ in 0..description_length {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
mpeg4_sgpd_data_sample_groups { description_length, sample_group_entry }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_sgpd_data { version, flags, grouping_type, default_length, entry_count, sample_groups })
}

/// d#311
fn Decoder_mpeg4_sbgp_data(_input: &mut Parser<'_>) -> Result<mpeg4_sbgp_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let grouping_type = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let grouping_type_parameter = if version == 1u8 {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
Some(u32be(x))
} else {
None
};
let entry_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let sample_groups = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = {
let sample_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let group_description_index = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
mpeg4_sbgp_data_sample_groups { sample_count, group_description_index }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_sbgp_data { version, flags, grouping_type, grouping_type_parameter, entry_count, sample_groups })
}

/// d#312
fn Decoder_mpeg4_ctts_sample_entry(_input: &mut Parser<'_>) -> Result<mpeg4_ctts_sample_entry, ParseError> {
let sample_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let sample_offset = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(mpeg4_ctts_sample_entry { sample_count, sample_offset })
}

/// d#313
fn Decoder_mpeg4_stts_sample_entry(_input: &mut Parser<'_>) -> Result<mpeg4_stts_sample_entry, ParseError> {
let sample_count = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let sample_delta = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(mpeg4_stts_sample_entry { sample_count, sample_delta })
}

/// d#314
fn Decoder_mpeg4_dref_data_data(_input: &mut Parser<'_>) -> Result<mpeg4_dref_data_data, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 7807103255128873628u64)
},

_ => {
(try_sub!(size_field, 8u32, 5357406925723651718u64)) as u64
}
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_dref_data_data { size_field, r#type, size, data })
}

/// d#315
fn Decoder_mpeg4_dref_data(_input: &mut Parser<'_>) -> Result<mpeg4_dref_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let number_of_entries = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let data = {
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
let next_elem = (Decoder316(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(mpeg4_dref_data { version, flags, number_of_entries, data })
}

/// d#316
fn Decoder316(_input: &mut Parser<'_>) -> Result<mpeg4_dref_data_data, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 8004105446758774533u64)
},

_ => {
(try_sub!(size_field, 8u32, 12318721104400761032u64)) as u64
}
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_dref_data_data { size_field, r#type, size, data })
}

/// d#317
fn Decoder_mpeg4_elst_data(_input: &mut Parser<'_>) -> Result<mpeg4_elst_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let number_of_entries = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let edit_list_table = {
let mut accum = Vec::new();
for _ in 0..number_of_entries {
let next_elem = (Decoder_mpeg4_elst_data_entry(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_elst_data { version, flags, number_of_entries, edit_list_table })
}

/// d#318
fn Decoder_mpeg4_elst_data_entry(_input: &mut Parser<'_>) -> Result<mpeg4_elst_data_entry, ParseError> {
let track_duration = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let media_time = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let media_rate = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
PResult::Ok(mpeg4_elst_data_entry { track_duration, media_time, media_rate })
}

/// d#319
fn Decoder_mpeg4_meta_atom_data(_input: &mut Parser<'_>, r#type: (u8, u8, u8, u8)) -> Result<mpeg4_meta_atom_data, ParseError> {
PResult::Ok(match r#type {
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
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_meta_atom_data::dinf(inner)
},

(104u8, 100u8, 108u8, 114u8) => {
let inner = (Decoder_mpeg4_meta_hdlr_data(_input))?;
mpeg4_meta_atom_data::hdlr(inner)
},

(112u8, 105u8, 116u8, 109u8) => {
let inner = (Decoder_mpeg4_pitm_atom_data(_input))?;
mpeg4_meta_atom_data::pitm(inner)
},

(105u8, 105u8, 110u8, 102u8) => {
let inner = (Decoder_mpeg4_iinf(_input))?;
mpeg4_meta_atom_data::iinf(inner)
},

(105u8, 114u8, 101u8, 102u8) => {
let inner = (Decoder_mpeg4_iref_data(_input))?;
mpeg4_meta_atom_data::iref(inner)
},

(105u8, 108u8, 111u8, 99u8) => {
let inner = (Decoder_mpeg4_iloc_atom_data(_input))?;
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
accum.push(next_elem)
} else {
break
}
};
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_meta_atom_data::unknown(inner)
}
})
}

/// d#320
fn Decoder_mpeg4_meta_hdlr_data(_input: &mut Parser<'_>) -> Result<mpeg4_meta_hdlr_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let predefined = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let handler_type = (Decoder284(_input))?;
let reserved = {
let arg0 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let arg1 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let arg2 = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
(arg0, arg1, arg2)
};
let name = {
let chars = {
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
return Err(ParseError::ExcludedBranch(12569316739694558801u64));
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
return Err(ParseError::ExcludedBranch(2967911718584065013u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(7227788188777836434u64));
}
};
chars.clone()
};
PResult::Ok(mpeg4_meta_hdlr_data { version, flags, predefined, handler_type, reserved, name })
}

/// d#321
fn Decoder_mpeg4_pitm_atom_data(_input: &mut Parser<'_>) -> Result<mpeg4_pitm_atom_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let item_ID = match version == 0u8 {
true => {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
mpeg4_pitm_atom_data_item_ID::Id16(inner)
},

false => {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
mpeg4_pitm_atom_data_item_ID::Id32(inner)
}
};
PResult::Ok(mpeg4_pitm_atom_data { version, flags, item_ID })
}

/// d#322
fn Decoder_mpeg4_iinf(_input: &mut Parser<'_>) -> Result<mpeg4_iinf, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let entry_count = match version == 0u8 {
true => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
x as u32
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
}
};
let item_info_entry = {
let mut accum = Vec::new();
for _ in 0..entry_count {
let next_elem = (Decoder_mpeg4_iinf_atom(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_iinf { version, flags, entry_count, item_info_entry })
}

/// d#323
fn Decoder_mpeg4_iref_data(_input: &mut Parser<'_>) -> Result<mpeg4_iref_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let single_item_reference = match version {
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
let next_elem = (Decoder_mpeg4_iref_data_single_item_reference_small(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_iref_data_single_item_reference::small(inner)
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
let next_elem = (Decoder_mpeg4_iref_data_single_item_reference_large(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_iref_data_single_item_reference::large(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(mpeg4_iref_data { version, flags, single_item_reference })
}

/// d#324
fn Decoder_mpeg4_iloc_atom_data(_input: &mut Parser<'_>) -> Result<mpeg4_iloc_atom_data, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let _packed_bits = _input.read_byte()?;
let offset_size = _packed_bits >> 4u8 & 15u8;
let length_size = _packed_bits & 15u8;
let _packed_bits = _input.read_byte()?;
let base_offset_size = _packed_bits >> 4u8 & 15u8;
let index_size = match version > 0u8 {
true => {
_packed_bits & 15u8
},

false => {
0u8
}
};
let item_count = match version < 2u8 {
true => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
x as u32
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
}
};
let items = {
let mut accum = Vec::new();
for _ in 0..item_count {
let next_elem = {
let item_ID = match version < 2u8 {
true => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
x as u32
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
}
};
let construction_method = if version > 0u8 {
let x = (_input.read_byte()?, _input.read_byte()?);
Some(u16be(x))
} else {
None
};
let data_reference_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let base_offset = match base_offset_size {
0u8 => {
0u64
},

4u8 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
x as u64
},

8u8 => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let extent_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let extents = {
let mut accum = Vec::new();
for _ in 0..extent_count {
let next_elem = (Decoder_mpeg4_iloc_extent(_input, offset_size, length_size, index_size))?;
accum.push(next_elem)
};
accum
};
mpeg4_iloc_atom_data_items { item_ID, construction_method, data_reference_index, base_offset, extent_count, extents }
};
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_iloc_atom_data { version, flags, offset_size, length_size, base_offset_size, index_size, item_count, items })
}

/// d#325
fn Decoder_mpeg4_ilst_atom(_input: &mut Parser<'_>) -> Result<mpeg4_ilst_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 12550558264664848853u64)
},

_ => {
(try_sub!(size_field, 8u32, 3320665455366264189u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
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
accum.push(next_elem)
} else {
break
}
};
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_ilst_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_ilst_atom { size_field, r#type, size, data })
}

/// d#326
fn Decoder_mpeg4_tool_atom(_input: &mut Parser<'_>) -> Result<mpeg4_tool_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 1891774877762105457u64)
},

_ => {
(try_sub!(size_field, 8u32, 3673300442962989464u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = match r#type {
(100u8, 97u8, 116u8, 97u8) => {
let inner = (Decoder_mpeg4_tool_atom_data_data(_input))?;
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_tool_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_tool_atom { size_field, r#type, size, data })
}

/// d#327
fn Decoder_mpeg4_tool_atom_data_data(_input: &mut Parser<'_>) -> Result<mpeg4_tool_atom_data_data, ParseError> {
let type_indicator = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let locale_indicator = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let value = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(mpeg4_tool_atom_data_data { type_indicator, locale_indicator, value })
}

/// d#328
fn Decoder_mpeg4_iloc_extent(_input: &mut Parser<'_>, offset_size: u8, length_size: u8, index_size: u8) -> Result<mpeg4_iloc_extent, ParseError> {
let extent_index = match index_size {
0u8 => {
0u64
},

4u8 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
x as u64
},

8u8 => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let extent_offset = match offset_size {
0u8 => {
0u64
},

4u8 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
x as u64
},

8u8 => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
let extent_length = match length_size {
0u8 => {
0u64
},

4u8 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
x as u64
},

8u8 => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
};
PResult::Ok(mpeg4_iloc_extent { extent_index, extent_offset, extent_length })
}

/// d#329
fn Decoder_mpeg4_iref_data_single_item_reference_small(_input: &mut Parser<'_>) -> Result<mpeg4_iref_data_single_item_reference_small, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 5515497093089591991u64)
},

_ => {
(try_sub!(size_field, 8u32, 8970999014112821604u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let from_item_ID = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let reference_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let to_item_ID = {
let mut accum = Vec::new();
for _ in 0..reference_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_iref_data_single_item_reference_small_data { from_item_ID, reference_count, to_item_ID })
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_iref_data_single_item_reference_small { size_field, r#type, size, data })
}

/// d#330
fn Decoder_mpeg4_iref_data_single_item_reference_large(_input: &mut Parser<'_>) -> Result<mpeg4_iref_data_single_item_reference_large, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 6882184431082022206u64)
},

_ => {
(try_sub!(size_field, 8u32, 17089130856162883194u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = ((|| {
let from_item_ID = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let reference_count = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let to_item_ID = {
let mut accum = Vec::new();
for _ in 0..reference_count {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
accum.push(next_elem)
};
accum
};
PResult::Ok(mpeg4_iref_data_single_item_reference_large_data { from_item_ID, reference_count, to_item_ID })
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_iref_data_single_item_reference_large { size_field, r#type, size, data })
}

/// d#331
fn Decoder_mpeg4_iinf_atom(_input: &mut Parser<'_>) -> Result<mpeg4_iinf_atom, ParseError> {
let size_field = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
};
let r#type = (Decoder284(_input))?;
let size = match size_field {
0u32 => {
0u64
},

1u32 => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u64be(x)
};
try_sub!(x, 16u64, 29850628954056690u64)
},

_ => {
(try_sub!(size_field, 8u32, 13537165373980795457u64)) as u64
}
};
let data = {
let sz = size as usize;
_input.start_slice(sz)?;
let ret = match r#type {
(105u8, 110u8, 102u8, 101u8) => {
let inner = (Decoder_mpeg4_iinf_atom_data_infe(_input))?;
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
mpeg4_iinf_atom_data::unknown(inner)
}
};
_input.end_slice()?;
ret
};
PResult::Ok(mpeg4_iinf_atom { size_field, r#type, size, data })
}

/// d#332
fn Decoder_mpeg4_iinf_atom_data_infe(_input: &mut Parser<'_>) -> Result<mpeg4_iinf_atom_data_infe, ParseError> {
let version = _input.read_byte()?;
let flags = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
let fields = (Decoder_mpeg4_infe_data_fields(_input, version))?;
PResult::Ok(mpeg4_iinf_atom_data_infe { version, flags, fields })
}

/// d#333
fn Decoder_mpeg4_infe_data_fields(_input: &mut Parser<'_>, version: u8) -> Result<mpeg4_infe_data_fields, ParseError> {
PResult::Ok(match version < 2u8 {
true => {
let inner = (Decoder_mpeg4_infe_data_fields_version_lt2(_input))?;
mpeg4_infe_data_fields::Version1(inner)
},

false => {
let inner = (Decoder_mpeg4_infe_data_fields_version_gte2(_input, version))?;
mpeg4_infe_data_fields::Version2(inner)
}
})
}

/// d#334
fn Decoder_mpeg4_infe_data_fields_version_lt2(_input: &mut Parser<'_>) -> Result<mpeg4_infe_data_fields_version_lt2, ParseError> {
let item_ID = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let item_protection_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let item_name = {
let chars = {
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
return Err(ParseError::ExcludedBranch(7279615132236188739u64));
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
return Err(ParseError::ExcludedBranch(17636172564439370608u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(13863787293436782080u64));
}
};
chars.clone()
};
let content_type = {
let chars = {
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
return Err(ParseError::ExcludedBranch(16714498072262546943u64));
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
return Err(ParseError::ExcludedBranch(2157707350523277837u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(15134222038433106385u64));
}
};
chars.clone()
};
let content_encoding = {
let chars = {
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
return Err(ParseError::ExcludedBranch(14950271805613481359u64));
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
return Err(ParseError::ExcludedBranch(5499566165243611472u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(11265176092564100083u64));
}
};
chars.clone()
};
PResult::Ok(mpeg4_infe_data_fields_version_lt2 { item_ID, item_protection_index, item_name, content_type, content_encoding })
}

/// d#335
fn Decoder_mpeg4_infe_data_fields_version_gte2(_input: &mut Parser<'_>, version: u8) -> Result<mpeg4_infe_data_fields_version_gte2, ParseError> {
let item_ID = match version == 2u8 {
true => {
let x = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
x as u32
},

false => {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32be(x)
}
};
let item_protection_index = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let item_type = (Decoder284(_input))?;
let item_name = {
let chars = {
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
return Err(ParseError::ExcludedBranch(14916894554939814670u64));
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
return Err(ParseError::ExcludedBranch(10473830801714814973u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(5334325531610156978u64));
}
};
chars.clone()
};
let extra_fields = (Decoder_mpeg4_infe_atom_data_extra_fields(_input, item_type))?;
PResult::Ok(mpeg4_infe_data_fields_version_gte2 { item_ID, item_protection_index, item_type, item_name, extra_fields })
}

/// d#336
fn Decoder_mpeg4_infe_atom_data_extra_fields(_input: &mut Parser<'_>, item_type: (u8, u8, u8, u8)) -> Result<mpeg4_infe_atom_data_extra_fields, ParseError> {
PResult::Ok(match item_type {
(109u8, 105u8, 109u8, 101u8) => {
let inner = (Decoder_mpeg4_infe_atom_data_extra_fields_mime(_input))?;
mpeg4_infe_atom_data_extra_fields::mime(inner)
},

(117u8, 114u8, 105u8, 32u8) => {
let inner = (Decoder_mpeg4_infe_atom_data_extra_fields_uri(_input))?;
mpeg4_infe_atom_data_extra_fields::uri(inner)
},

_ => {
mpeg4_infe_atom_data_extra_fields::unknown
}
})
}

/// d#337
fn Decoder_mpeg4_infe_atom_data_extra_fields_mime(_input: &mut Parser<'_>) -> Result<mpeg4_infe_atom_data_extra_fields_mime, ParseError> {
let content_type = {
let chars = {
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
return Err(ParseError::ExcludedBranch(16975008930446149745u64));
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
return Err(ParseError::ExcludedBranch(760820951392925727u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(14363790737598139216u64));
}
};
chars.clone()
};
PResult::Ok(mpeg4_infe_atom_data_extra_fields_mime { content_type })
}

/// d#338
fn Decoder_mpeg4_infe_atom_data_extra_fields_uri(_input: &mut Parser<'_>) -> Result<mpeg4_infe_atom_data_extra_fields_uri, ParseError> {
let item_uri_type = {
let chars = {
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
return Err(ParseError::ExcludedBranch(4600414761378562541u64));
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
return Err(ParseError::ExcludedBranch(18313399323903636110u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(15786118691017431738u64));
}
};
chars.clone()
};
PResult::Ok(mpeg4_infe_atom_data_extra_fields_uri { item_uri_type })
}

/// d#339
fn Decoder_jpeg_soi(_input: &mut Parser<'_>) -> Result<jpeg_soi, ParseError> {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(16165934354425559621u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 216 {
b
} else {
return Err(ParseError::ExcludedBranch(16399036514137665776u64));
}
};
PResult::Ok(jpeg_soi { marker })
}

/// d#340
fn Decoder_jpeg_frame(_input: &mut Parser<'_>) -> Result<jpeg_frame, ParseError> {
let initial_segment = (Decoder_jpeg_frame_initial_segment(_input))?;
let segments = {
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
return Err(ParseError::ExcludedBranch(7931358881575056193u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(17863486658382945784u64));
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_jpeg_table_or_misc(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let header = (Decoder_jpeg_frame_header(_input))?;
let scan = (Decoder_jpeg_scan(_input))?;
let dnl = {
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
return Err(ParseError::ExcludedBranch(11515797873012483658u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(8584109755265226714u64));
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
Some((Decoder_jpeg_dnl(_input))?)
},

1 => {
None
},

_ => {
return Err(ParseError::ExcludedBranch(8076978189295213982u64));
}
}
};
let scans = {
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
return Err(ParseError::ExcludedBranch(11570281271401624317u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(14687724984806605719u64));
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder347(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(jpeg_frame { initial_segment, segments, header, scan, dnl, scans })
}

/// d#341
fn Decoder341(_input: &mut Parser<'_>) -> Result<jpeg_soi, ParseError> {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(1378805635639824117u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 217 {
b
} else {
return Err(ParseError::ExcludedBranch(8385173961957899741u64));
}
};
PResult::Ok(jpeg_soi { marker })
}

/// d#342
fn Decoder_jpeg_frame_initial_segment(_input: &mut Parser<'_>) -> Result<jpeg_frame_initial_segment, ParseError> {
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
return Err(ParseError::ExcludedBranch(8407356061009412694u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(6881565717664829242u64));
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = (Decoder_jpeg_app0(_input))?;
jpeg_frame_initial_segment::app0(inner)
},

1 => {
let inner = (Decoder_jpeg_app1(_input))?;
jpeg_frame_initial_segment::app1(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(13752480002470540422u64));
}
})
}

/// d#343
fn Decoder_jpeg_table_or_misc(_input: &mut Parser<'_>) -> Result<jpeg_table_or_misc, ParseError> {
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
return Err(ParseError::ExcludedBranch(17107648091243309207u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(5534609128357633386u64));
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
let inner = (Decoder_jpeg_app2(_input))?;
jpeg_table_or_misc::app2(inner)
},

7 => {
let inner = (Decoder388(_input))?;
jpeg_table_or_misc::app3(inner)
},

8 => {
let inner = (Decoder389(_input))?;
jpeg_table_or_misc::app4(inner)
},

9 => {
let inner = (Decoder390(_input))?;
jpeg_table_or_misc::app5(inner)
},

10 => {
let inner = (Decoder391(_input))?;
jpeg_table_or_misc::app6(inner)
},

11 => {
let inner = (Decoder392(_input))?;
jpeg_table_or_misc::app7(inner)
},

12 => {
let inner = (Decoder393(_input))?;
jpeg_table_or_misc::app8(inner)
},

13 => {
let inner = (Decoder394(_input))?;
jpeg_table_or_misc::app9(inner)
},

14 => {
let inner = (Decoder395(_input))?;
jpeg_table_or_misc::app10(inner)
},

15 => {
let inner = (Decoder396(_input))?;
jpeg_table_or_misc::app11(inner)
},

16 => {
let inner = (Decoder397(_input))?;
jpeg_table_or_misc::app12(inner)
},

17 => {
let inner = (Decoder398(_input))?;
jpeg_table_or_misc::app13(inner)
},

18 => {
let inner = (Decoder399(_input))?;
jpeg_table_or_misc::app14(inner)
},

19 => {
let inner = (Decoder400(_input))?;
jpeg_table_or_misc::app15(inner)
},

20 => {
let inner = (Decoder401(_input))?;
jpeg_table_or_misc::com(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(14539762430836305896u64));
}
})
}

/// d#344
fn Decoder_jpeg_frame_header(_input: &mut Parser<'_>) -> Result<jpeg_frame_header, ParseError> {
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
return Err(ParseError::ExcludedBranch(16625761205375889740u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(2662265345698212949u64));
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = (Decoder_jpeg_sof0(_input))?;
jpeg_frame_header::sof0(inner)
},

1 => {
let inner = (Decoder367(_input))?;
jpeg_frame_header::sof1(inner)
},

2 => {
let inner = (Decoder368(_input))?;
jpeg_frame_header::sof2(inner)
},

3 => {
let inner = (Decoder369(_input))?;
jpeg_frame_header::sof3(inner)
},

4 => {
let inner = (Decoder370(_input))?;
jpeg_frame_header::sof5(inner)
},

5 => {
let inner = (Decoder371(_input))?;
jpeg_frame_header::sof6(inner)
},

6 => {
let inner = (Decoder372(_input))?;
jpeg_frame_header::sof7(inner)
},

7 => {
let inner = (Decoder373(_input))?;
jpeg_frame_header::sof9(inner)
},

8 => {
let inner = (Decoder374(_input))?;
jpeg_frame_header::sof10(inner)
},

9 => {
let inner = (Decoder375(_input))?;
jpeg_frame_header::sof11(inner)
},

10 => {
let inner = (Decoder376(_input))?;
jpeg_frame_header::sof13(inner)
},

11 => {
let inner = (Decoder377(_input))?;
jpeg_frame_header::sof14(inner)
},

12 => {
let inner = (Decoder378(_input))?;
jpeg_frame_header::sof15(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(3344648651879382526u64));
}
})
}

/// d#345
fn Decoder_jpeg_scan(_input: &mut Parser<'_>) -> Result<jpeg_scan, ParseError> {
let segments = {
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
return Err(ParseError::ExcludedBranch(8599210436172030522u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(3484767027554133518u64));
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_jpeg_table_or_misc(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let sos = (Decoder_jpeg_sos(_input))?;
let data = (Decoder365(_input))?;
PResult::Ok(jpeg_scan { segments, sos, data })
}

/// d#346
fn Decoder_jpeg_dnl(_input: &mut Parser<'_>) -> Result<jpeg_dnl, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(15403934492100194569u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 220 {
b
} else {
return Err(ParseError::ExcludedBranch(12041148194529633639u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 11669649807369914251u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_dnl_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_dnl { marker, length, data })
}

/// d#347
fn Decoder347(_input: &mut Parser<'_>) -> Result<jpeg_scan, ParseError> {
let segments = {
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
return Err(ParseError::ExcludedBranch(2288772415159374970u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(17888323854924040413u64));
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_jpeg_table_or_misc(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let sos = (Decoder_jpeg_sos(_input))?;
let data = (Decoder_jpeg_scan_data(_input))?;
PResult::Ok(jpeg_scan { segments, sos, data })
}

/// d#348
fn Decoder_jpeg_sos(_input: &mut Parser<'_>) -> Result<jpeg_sos, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(6279087434444973374u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 218 {
b
} else {
return Err(ParseError::ExcludedBranch(11074951631636946051u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 8880661182590738257u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sos_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sos { marker, length, data })
}

/// d#349
fn Decoder_jpeg_scan_data(_input: &mut Parser<'_>) -> Result<jpeg_scan_data, ParseError> {
let scan_data = {
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
return Err(ParseError::ExcludedBranch(15601622509425384091u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(13675295148592556047u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_jpeg_scan_data_entropy_coded_segment(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let scan_data_stream = (try_flat_map_vec(scan_data.iter().cloned(), |x: jpeg_scan_data_entropy_coded_segment| PResult::Ok(match x {
jpeg_scan_data_entropy_coded_segment::mcu(v) => {
[v].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst0(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst1(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst2(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst3(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst4(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst5(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst6(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst7(..) => {
[].to_vec()
}
})))?;
PResult::Ok(jpeg_scan_data { scan_data, scan_data_stream })
}

/// d#350
fn Decoder_jpeg_scan_data_entropy_coded_segment(_input: &mut Parser<'_>) -> Result<jpeg_scan_data_entropy_coded_segment, ParseError> {
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
return Err(ParseError::ExcludedBranch(4569970360394099475u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9110520999974091875u64));
}
};
_input.close_peek_context()?;
ret
}
};
PResult::Ok(match tree_index {
0 => {
let inner = (Decoder351(_input))?;
jpeg_scan_data_entropy_coded_segment::mcu(inner)
},

1 => {
let inner = (Decoder352(_input))?;
jpeg_scan_data_entropy_coded_segment::rst0(inner)
},

2 => {
let inner = (Decoder353(_input))?;
jpeg_scan_data_entropy_coded_segment::rst1(inner)
},

3 => {
let inner = (Decoder354(_input))?;
jpeg_scan_data_entropy_coded_segment::rst2(inner)
},

4 => {
let inner = (Decoder355(_input))?;
jpeg_scan_data_entropy_coded_segment::rst3(inner)
},

5 => {
let inner = (Decoder356(_input))?;
jpeg_scan_data_entropy_coded_segment::rst4(inner)
},

6 => {
let inner = (Decoder357(_input))?;
jpeg_scan_data_entropy_coded_segment::rst5(inner)
},

7 => {
let inner = (Decoder358(_input))?;
jpeg_scan_data_entropy_coded_segment::rst6(inner)
},

8 => {
let inner = (Decoder359(_input))?;
jpeg_scan_data_entropy_coded_segment::rst7(inner)
},

_ => {
return Err(ParseError::ExcludedBranch(15293691521783146694u64));
}
})
}

/// d#351
fn Decoder351(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
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
return Err(ParseError::ExcludedBranch(3975307768385535064u64));
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
return Err(ParseError::ExcludedBranch(15433822888775103886u64));
}
},

1 => {
let _ = {
let arg0 = {
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(8403192837054512577u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(17073037115051226650u64));
}
};
(arg0, arg1)
};
255u8
},

_ => {
return Err(ParseError::ExcludedBranch(10599514554463239458u64));
}
})
}

/// d#352
fn Decoder352(_input: &mut Parser<'_>) -> Result<jpeg_soi, ParseError> {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(16112061863928357291u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 208 {
b
} else {
return Err(ParseError::ExcludedBranch(12017601628070515145u64));
}
};
PResult::Ok(jpeg_soi { marker })
}

/// d#353
fn Decoder353(_input: &mut Parser<'_>) -> Result<jpeg_soi, ParseError> {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(1872233699568519226u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 209 {
b
} else {
return Err(ParseError::ExcludedBranch(10708294527730390829u64));
}
};
PResult::Ok(jpeg_soi { marker })
}

/// d#354
fn Decoder354(_input: &mut Parser<'_>) -> Result<jpeg_soi, ParseError> {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(7432469293302627017u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 210 {
b
} else {
return Err(ParseError::ExcludedBranch(13181260675040079306u64));
}
};
PResult::Ok(jpeg_soi { marker })
}

/// d#355
fn Decoder355(_input: &mut Parser<'_>) -> Result<jpeg_soi, ParseError> {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(9159119361499271180u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 211 {
b
} else {
return Err(ParseError::ExcludedBranch(7795160901559545235u64));
}
};
PResult::Ok(jpeg_soi { marker })
}

/// d#356
fn Decoder356(_input: &mut Parser<'_>) -> Result<jpeg_soi, ParseError> {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(3490919313637905107u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 212 {
b
} else {
return Err(ParseError::ExcludedBranch(9331389203258424019u64));
}
};
PResult::Ok(jpeg_soi { marker })
}

/// d#357
fn Decoder357(_input: &mut Parser<'_>) -> Result<jpeg_soi, ParseError> {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(16679512278832019969u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 213 {
b
} else {
return Err(ParseError::ExcludedBranch(15311158871930328757u64));
}
};
PResult::Ok(jpeg_soi { marker })
}

/// d#358
fn Decoder358(_input: &mut Parser<'_>) -> Result<jpeg_soi, ParseError> {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(9892894478446917378u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 214 {
b
} else {
return Err(ParseError::ExcludedBranch(3585635225240718191u64));
}
};
PResult::Ok(jpeg_soi { marker })
}

/// d#359
fn Decoder359(_input: &mut Parser<'_>) -> Result<jpeg_soi, ParseError> {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(5208404121666294786u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 215 {
b
} else {
return Err(ParseError::ExcludedBranch(237665900562449517u64));
}
};
PResult::Ok(jpeg_soi { marker })
}

/// d#360
fn Decoder_jpeg_sos_data(_input: &mut Parser<'_>) -> Result<jpeg_sos_data, ParseError> {
let num_image_components = {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
matches!(x, 1u8..=4u8)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13751590285972774894u64));
}
};
let image_components = {
let mut accum = Vec::new();
for _ in 0..num_image_components {
let next_elem = (Decoder_jpeg_sos_image_component(_input))?;
accum.push(next_elem)
};
accum
};
let start_spectral_selection = {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x <= 63u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12552648416444111338u64));
}
};
let end_spectral_selection = {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x <= 63u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9201081899504003615u64));
}
};
let approximation_bit_position = (Decoder_jpeg_sos_data_approximation_bit_position(_input))?;
PResult::Ok(jpeg_sos_data { num_image_components, image_components, start_spectral_selection, end_spectral_selection, approximation_bit_position })
}

/// d#361
fn Decoder_jpeg_sos_image_component(_input: &mut Parser<'_>) -> Result<jpeg_sos_image_component, ParseError> {
let component_selector = _input.read_byte()?;
let entropy_coding_table_ids = (Decoder_jpeg_sos_image_component_entropy_coding_table_ids(_input))?;
PResult::Ok(jpeg_sos_image_component { component_selector, entropy_coding_table_ids })
}

/// d#362
fn Decoder_jpeg_sos_data_approximation_bit_position(_input: &mut Parser<'_>) -> Result<jpeg_sos_data_approximation_bit_position, ParseError> {
let _packed_bits = _input.read_byte()?;
let high = _packed_bits >> 4u8 & 15u8;
let low = _packed_bits & 15u8;
PResult::Ok(jpeg_sos_data_approximation_bit_position { high, low })
}

/// d#363
fn Decoder_jpeg_sos_image_component_entropy_coding_table_ids(_input: &mut Parser<'_>) -> Result<jpeg_sos_image_component_entropy_coding_table_ids, ParseError> {
let inner = {
let _packed_bits = _input.read_byte()?;
let dc_entropy_coding_table_id = _packed_bits >> 4u8 & 15u8;
let ac_entropy_coding_table_id = _packed_bits & 15u8;
jpeg_sos_image_component_entropy_coding_table_ids { dc_entropy_coding_table_id, ac_entropy_coding_table_id }
};
let is_valid = {
let entropy_coding_table_ids = inner;
(entropy_coding_table_ids.dc_entropy_coding_table_id <= 3u8) && (entropy_coding_table_ids.ac_entropy_coding_table_id <= 3u8)
};
PResult::Ok(if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10776065777346510440u64));
})
}

/// d#364
fn Decoder_jpeg_dnl_data(_input: &mut Parser<'_>) -> Result<jpeg_dnl_data, ParseError> {
let num_lines = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x != 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(1821331332215525359u64));
}
};
PResult::Ok(jpeg_dnl_data { num_lines })
}

/// d#365
fn Decoder365(_input: &mut Parser<'_>) -> Result<jpeg_scan_data, ParseError> {
let scan_data = {
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
return Err(ParseError::ExcludedBranch(1550574349011231204u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6867774794241173436u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_jpeg_scan_data_entropy_coded_segment(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let scan_data_stream = (try_flat_map_vec(scan_data.iter().cloned(), |x: jpeg_scan_data_entropy_coded_segment| PResult::Ok(match x {
jpeg_scan_data_entropy_coded_segment::mcu(v) => {
[v].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst0(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst1(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst2(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst3(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst4(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst5(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst6(..) => {
[].to_vec()
},

jpeg_scan_data_entropy_coded_segment::rst7(..) => {
[].to_vec()
}
})))?;
PResult::Ok(jpeg_scan_data { scan_data, scan_data_stream })
}

/// d#366
fn Decoder_jpeg_sof0(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(3475686103639625566u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 192 {
b
} else {
return Err(ParseError::ExcludedBranch(4130856500275801127u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 691490157317212239u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#367
fn Decoder367(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(11582380281701370059u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 193 {
b
} else {
return Err(ParseError::ExcludedBranch(7228157205966134869u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 10719628102612994677u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#368
fn Decoder368(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(2184161105566707760u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 194 {
b
} else {
return Err(ParseError::ExcludedBranch(9924059786910440358u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 13685962128001446815u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#369
fn Decoder369(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(7274029685341305701u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 195 {
b
} else {
return Err(ParseError::ExcludedBranch(16051783775494465147u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 7538966935051243003u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#370
fn Decoder370(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(13744164271564421708u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 197 {
b
} else {
return Err(ParseError::ExcludedBranch(5892114170581446733u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 4867798537713738914u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#371
fn Decoder371(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(11821813774070801620u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 198 {
b
} else {
return Err(ParseError::ExcludedBranch(14520503729026832983u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 11266387855511437693u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#372
fn Decoder372(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(5309491469191307378u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 199 {
b
} else {
return Err(ParseError::ExcludedBranch(17983075411320920965u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 11452033436843896773u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#373
fn Decoder373(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(7023661717588102849u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 201 {
b
} else {
return Err(ParseError::ExcludedBranch(3448575031819686448u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 8138544351856664662u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#374
fn Decoder374(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(9960855096836829935u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 202 {
b
} else {
return Err(ParseError::ExcludedBranch(218475477370319322u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 12696272221194189133u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#375
fn Decoder375(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(18357658168615546095u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 203 {
b
} else {
return Err(ParseError::ExcludedBranch(10650412753233146525u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 3995820927126919547u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#376
fn Decoder376(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(15859964085544252343u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 205 {
b
} else {
return Err(ParseError::ExcludedBranch(653325817133119558u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 9149418055219508197u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#377
fn Decoder377(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(3349032559334020401u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 206 {
b
} else {
return Err(ParseError::ExcludedBranch(14115009527471272688u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 8000269442706245049u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#378
fn Decoder378(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(8350850950759220429u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 207 {
b
} else {
return Err(ParseError::ExcludedBranch(15412400192383838763u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 4100106362216887809u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_sof_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#379
fn Decoder_jpeg_sof_data(_input: &mut Parser<'_>) -> Result<jpeg_sof_data, ParseError> {
let sample_precision = {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
matches!(x, 2u8..=16u8)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(16334217566159141080u64));
}
};
let num_lines = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let num_samples_per_line = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x != 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(15032955882314050195u64));
}
};
let num_image_components = {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x != 0u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(12522857579864693834u64));
}
};
let image_components = {
let mut accum = Vec::new();
for _ in 0..num_image_components {
let next_elem = (Decoder_jpeg_sof_image_component(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(jpeg_sof_data { sample_precision, num_lines, num_samples_per_line, num_image_components, image_components })
}

/// d#380
fn Decoder_jpeg_sof_image_component(_input: &mut Parser<'_>) -> Result<jpeg_sof_image_component, ParseError> {
let id = _input.read_byte()?;
let sampling_factor = {
let _packed_bits = _input.read_byte()?;
let horizontal = _packed_bits >> 4u8 & 15u8;
let vertical = _packed_bits & 15u8;
jpeg_dhp_image_component_sampling_factor { horizontal, vertical }
};
let quantization_table_id = {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x <= 3u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(15286713778088114821u64));
}
};
PResult::Ok(jpeg_sof_image_component { id, sampling_factor, quantization_table_id })
}

/// d#381
fn Decoder_jpeg_dqt(_input: &mut Parser<'_>) -> Result<jpeg_dqt, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(17055268834995250246u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 219 {
b
} else {
return Err(ParseError::ExcludedBranch(15014773733126201031u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 3198904588321530108u64)) as usize;
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
accum.push(next_elem)
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_dqt { marker, length, data })
}

/// d#382
fn Decoder_jpeg_dht(_input: &mut Parser<'_>) -> Result<jpeg_dht, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(9895655502210650925u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 196 {
b
} else {
return Err(ParseError::ExcludedBranch(3344835778759068560u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 8674930063339641954u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_dht_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_dht { marker, length, data })
}

/// d#383
fn Decoder_jpeg_dac(_input: &mut Parser<'_>) -> Result<jpeg_dac, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(2014773054382805425u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 204 {
b
} else {
return Err(ParseError::ExcludedBranch(3011460078285478248u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 15244023661753025012u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_dac_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_dac { marker, length, data })
}

/// d#384
fn Decoder_jpeg_dri(_input: &mut Parser<'_>) -> Result<jpeg_dri, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(5117297982688264891u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 221 {
b
} else {
return Err(ParseError::ExcludedBranch(4614223265245060097u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 11029695522295027332u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_dri_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_dri { marker, length, data })
}

/// d#385
fn Decoder_jpeg_app0(_input: &mut Parser<'_>) -> Result<jpeg_app0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(2858990937242709991u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 224 {
b
} else {
return Err(ParseError::ExcludedBranch(13162270726566423196u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 1560031033762626303u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_app0_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app0 { marker, length, data })
}

/// d#386
fn Decoder_jpeg_app1(_input: &mut Parser<'_>) -> Result<jpeg_app1, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(6766897041260485978u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 225 {
b
} else {
return Err(ParseError::ExcludedBranch(7359082011512182682u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 2452372056966650770u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder_jpeg_app1_data(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app1 { marker, length, data })
}

/// d#387
fn Decoder_jpeg_app2(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(4005260763079064488u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 226 {
b
} else {
return Err(ParseError::ExcludedBranch(9895427541506148364u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#388
fn Decoder388(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(15241527188218394569u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 227 {
b
} else {
return Err(ParseError::ExcludedBranch(5019692195244899787u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#389
fn Decoder389(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(663652071640520941u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 228 {
b
} else {
return Err(ParseError::ExcludedBranch(16835260701216065402u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#390
fn Decoder390(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(4248622096514297129u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 229 {
b
} else {
return Err(ParseError::ExcludedBranch(18304605036866855350u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#391
fn Decoder391(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(49400955721755355u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 230 {
b
} else {
return Err(ParseError::ExcludedBranch(8882217996184815919u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#392
fn Decoder392(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(11885930557202460461u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 231 {
b
} else {
return Err(ParseError::ExcludedBranch(9841369023026740320u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#393
fn Decoder393(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(15065685669539080124u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 232 {
b
} else {
return Err(ParseError::ExcludedBranch(4896351207164742422u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#394
fn Decoder394(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(12845528861092334564u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 233 {
b
} else {
return Err(ParseError::ExcludedBranch(8497774971318424699u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#395
fn Decoder395(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(13677998342346693652u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 234 {
b
} else {
return Err(ParseError::ExcludedBranch(18442161777584514946u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#396
fn Decoder396(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(2668737607901180946u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 235 {
b
} else {
return Err(ParseError::ExcludedBranch(12828788577937869717u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#397
fn Decoder397(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(4528919599938425798u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 236 {
b
} else {
return Err(ParseError::ExcludedBranch(12955185663715491460u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#398
fn Decoder398(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(6221053009072016381u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 237 {
b
} else {
return Err(ParseError::ExcludedBranch(4825757476091239776u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#399
fn Decoder399(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(13877876706306354357u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 238 {
b
} else {
return Err(ParseError::ExcludedBranch(12525311251009778949u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#400
fn Decoder400(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(14962551735998681614u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 239 {
b
} else {
return Err(ParseError::ExcludedBranch(2007599397777734421u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#401
fn Decoder401(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(18069850277258932991u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 254 {
b
} else {
return Err(ParseError::ExcludedBranch(180881308211696508u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#402
fn Decoder_jpeg_app1_data(_input: &mut Parser<'_>) -> Result<jpeg_app1_data, ParseError> {
let identifier = {
let chars = {
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
return Err(ParseError::ExcludedBranch(17753230141940491005u64));
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
return Err(ParseError::ExcludedBranch(6110942357212830202u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(6596410053543851077u64));
}
};
chars.clone()
};
let data = (Decoder_jpeg_app1_data_data(_input, &identifier))?;
PResult::Ok(jpeg_app1_data { identifier, data })
}

/// d#403
fn Decoder_jpeg_app1_data_data(_input: &mut Parser<'_>, identifier: &[u8]) -> Result<jpeg_app1_data_data, ParseError> {
PResult::Ok(match slice_all(&identifier) {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
jpeg_app1_data_data::other(inner)
}
})
}

/// d#404
fn Decoder_jpeg_app1_exif(_input: &mut Parser<'_>) -> Result<jpeg_app1_exif, ParseError> {
let padding = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(16339654162669176472u64));
}
};
let exif = (Decoder_tiff_main(_input))?;
PResult::Ok(jpeg_app1_exif { padding, exif })
}

/// d#405
fn Decoder_jpeg_app1_xmp(_input: &mut Parser<'_>) -> Result<jpeg_app1_xmp, ParseError> {
let xmp = {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(jpeg_app1_xmp { xmp })
}

/// d#406
fn Decoder_jpeg_app0_data(_input: &mut Parser<'_>) -> Result<jpeg_app0_data, ParseError> {
let identifier = {
let chars = {
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
return Err(ParseError::ExcludedBranch(1886358831178290550u64));
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
return Err(ParseError::ExcludedBranch(17004441093219507547u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(3168197157646945762u64));
}
};
chars.clone()
};
let data = (Decoder_jpeg_app0_data_data(_input, &identifier))?;
PResult::Ok(jpeg_app0_data { identifier, data })
}

/// d#407
fn Decoder_jpeg_app0_data_data(_input: &mut Parser<'_>, identifier: &[u8]) -> Result<jpeg_app0_data_data, ParseError> {
PResult::Ok(match slice_all(&identifier) {
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
accum
};
jpeg_app0_data_data::other(inner)
}
})
}

/// d#408
fn Decoder_jpeg_app0_jfif(_input: &mut Parser<'_>) -> Result<jpeg_app0_jfif, ParseError> {
let version_major = _input.read_byte()?;
let version_minor = _input.read_byte()?;
let density_units = {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x <= 2u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(360138127928998237u64));
}
};
let density_x = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x != 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9092411637461100444u64));
}
};
let density_y = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x != 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(15117497265985508077u64));
}
};
let thumbnail_width = _input.read_byte()?;
let thumbnail_height = _input.read_byte()?;
let thumbnail_pixels = {
let mut accum = Vec::new();
for _ in 0..thumbnail_height {
let next_elem = {
let mut accum = Vec::new();
for _ in 0..thumbnail_width {
let next_elem = (Decoder_gif_color_table_entry(_input))?;
accum.push(next_elem)
};
accum
};
accum.push(next_elem)
};
accum
};
PResult::Ok(jpeg_app0_jfif { version_major, version_minor, density_units, density_x, density_y, thumbnail_width, thumbnail_height, thumbnail_pixels })
}

/// d#409
fn Decoder_gif_color_table_entry(_input: &mut Parser<'_>) -> Result<gif_color_table_entry, ParseError> {
let r = _input.read_byte()?;
let g = _input.read_byte()?;
let b = _input.read_byte()?;
PResult::Ok(gif_color_table_entry { r, g, b })
}

/// d#410
fn Decoder_jpeg_dri_data(_input: &mut Parser<'_>) -> Result<jpeg_dri_data, ParseError> {
let restart_interval = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
PResult::Ok(jpeg_dri_data { restart_interval })
}

/// d#411
fn Decoder_jpeg_dac_data(_input: &mut Parser<'_>) -> Result<jpeg_dac_data, ParseError> {
let class_table_id = (Decoder_jpeg_class_table_id(_input))?;
let value = _input.read_byte()?;
PResult::Ok(jpeg_dac_data { class_table_id, value })
}

/// d#412
fn Decoder_jpeg_class_table_id(_input: &mut Parser<'_>) -> Result<jpeg_class_table_id, ParseError> {
let inner = {
let _packed_bits = _input.read_byte()?;
let class = _packed_bits >> 4u8 & 15u8;
let table_id = _packed_bits & 15u8;
jpeg_class_table_id { class, table_id }
};
let is_valid = {
let class_table_id = inner;
(class_table_id.class < 2u8) && (class_table_id.table_id < 4u8)
};
PResult::Ok(if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(11247369915737850258u64));
})
}

/// d#413
fn Decoder_jpeg_dht_data(_input: &mut Parser<'_>) -> Result<jpeg_dht_data, ParseError> {
let class_table_id = (Decoder_jpeg_class_table_id(_input))?;
let num_codes = {
let mut accum = Vec::new();
for _ in 0..16u8 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
let values = {
let mut accum = Vec::new();
for n in num_codes.clone() {
let next_elem = {
let mut accum = Vec::new();
for _ in 0..n {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
accum.push(next_elem)
};
accum
};
PResult::Ok(jpeg_dht_data { class_table_id, num_codes, values })
}

/// d#414
fn Decoder_jpeg_dqt_data(_input: &mut Parser<'_>) -> Result<jpeg_dqt_data, ParseError> {
let precision_table_id = (Decoder_jpeg_precision_table_id(_input))?;
let elements = (Decoder_jpeg_dqt_data_elements(_input, precision_table_id.precision))?;
PResult::Ok(jpeg_dqt_data { precision_table_id, elements })
}

/// d#415
fn Decoder_jpeg_precision_table_id(_input: &mut Parser<'_>) -> Result<jpeg_precision_table_id, ParseError> {
let inner = {
let _packed_bits = _input.read_byte()?;
let precision = _packed_bits >> 4u8 & 15u8;
let table_id = _packed_bits & 15u8;
jpeg_precision_table_id { precision, table_id }
};
let is_valid = {
let precision_table_id = inner;
(precision_table_id.precision < 2u8) && (precision_table_id.table_id < 4u8)
};
PResult::Ok(if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(7546547171508918509u64));
})
}

/// d#416
fn Decoder_jpeg_dqt_data_elements(_input: &mut Parser<'_>, precision: u8) -> Result<jpeg_dqt_data_elements, ParseError> {
PResult::Ok(match precision {
0u8 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..64u32 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
jpeg_dqt_data_elements::Bytes(inner)
},

1u8 => {
let inner = {
let mut accum = Vec::new();
for _ in 0..64u32 {
let next_elem = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
accum.push(next_elem)
};
accum
};
jpeg_dqt_data_elements::Shorts(inner)
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
})
}

/// d#417
fn Decoder_gzip_header(_input: &mut Parser<'_>) -> Result<gzip_header, ParseError> {
let magic = {
let arg0 = {
let b = _input.read_byte()?;
if b == 31 {
b
} else {
return Err(ParseError::ExcludedBranch(14304129837244038526u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 139 {
b
} else {
return Err(ParseError::ExcludedBranch(15347994581840044382u64));
}
};
(arg0, arg1)
};
let method = _input.read_byte()?;
let file_flags = (Decoder_gzip_header_file_flags(_input))?;
let timestamp = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
};
let compression_flags = _input.read_byte()?;
let os_id = _input.read_byte()?;
PResult::Ok(gzip_header { magic, method, file_flags, timestamp, compression_flags, os_id })
}

/// d#418
fn Decoder_gzip_fextra(_input: &mut Parser<'_>) -> Result<gzip_fextra, ParseError> {
let xlen = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let subfields = {
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
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(gzip_fextra { xlen, subfields })
}

/// d#419
fn Decoder419(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let chars = {
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
return Err(ParseError::ExcludedBranch(18237415135895269790u64));
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
return Err(ParseError::ExcludedBranch(4706601668945989307u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(1678201523848816632u64));
}
};
PResult::Ok(chars.clone())
}

/// d#420
fn Decoder_gzip_fcomment(_input: &mut Parser<'_>) -> Result<gzip_fcomment, ParseError> {
let comment = {
let chars = {
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
return Err(ParseError::ExcludedBranch(13246474195614162055u64));
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
return Err(ParseError::ExcludedBranch(8414108255387456730u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(1432698095084823598u64));
}
};
chars.clone()
};
PResult::Ok(gzip_fcomment { comment })
}

/// d#421
fn Decoder_gzip_fhcrc(_input: &mut Parser<'_>) -> Result<gzip_fhcrc, ParseError> {
let crc = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
PResult::Ok(gzip_fhcrc { crc })
}

/// d#422
fn Decoder_gzip_footer(_input: &mut Parser<'_>) -> Result<gzip_footer, ParseError> {
let crc = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?, _input.read_byte()?, _input.read_byte()?);
u32le(x)
};
PResult::Ok(gzip_footer { crc, length })
}

/// d#423
fn Decoder_gzip_fextra_subfield(_input: &mut Parser<'_>) -> Result<gzip_fextra_subfield, ParseError> {
let si1 = _input.read_byte()?;
let si2 = _input.read_byte()?;
let len = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let data = {
let mut accum = Vec::new();
for _ in 0..len {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
PResult::Ok(gzip_fextra_subfield { si1, si2, len, data })
}

/// d#424
fn Decoder_gzip_header_file_flags(_input: &mut Parser<'_>) -> Result<gzip_header_file_flags, ParseError> {
let _packed_bits = _input.read_byte()?;
let fcomment = _packed_bits >> 4u8 & 1u8 > 0u8;
let fname = _packed_bits >> 3u8 & 1u8 > 0u8;
let fextra = _packed_bits >> 2u8 & 1u8 > 0u8;
let fhcrc = _packed_bits >> 1u8 & 1u8 > 0u8;
let ftext = _packed_bits & 1u8 > 0u8;
PResult::Ok(gzip_header_file_flags { fcomment, fname, fextra, fhcrc, ftext })
}

/// d#425
fn Decoder425(_input: &mut Parser<'_>) -> Result<Vec<gzip_main>, ParseError> {
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
let fextra = if header.file_flags.fextra {
Some((Decoder_gzip_fextra(_input))?)
} else {
None
};
let fname = if header.file_flags.fname {
Some((Decoder427(_input))?)
} else {
None
};
let fcomment = if header.file_flags.fcomment {
Some((Decoder428(_input))?)
} else {
None
};
let fhcrc = if header.file_flags.fhcrc {
Some((Decoder_gzip_fhcrc(_input))?)
} else {
None
};
let data = {
_input.enter_bits_mode()?;
let ret = (Decoder_deflate_main(_input))?;
let _bits_read = _input.escape_bits_mode()?;
ret
};
let footer = (Decoder_gzip_footer(_input))?;
gzip_main { header, fextra, fname, fcomment, fhcrc, data, footer }
};
accum.push(next_elem)
}
};
PResult::Ok(accum)
}

/// d#426
fn Decoder426(_input: &mut Parser<'_>) -> Result<tar_main, ParseError> {
let contents = {
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
return Err(ParseError::ExcludedBranch(9372987348964131232u64));
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
accum.push(next_elem)
}
};
accum
};
{
let mut accum = Vec::new();
for _ in 0..1024u32 {
let next_elem = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(8473414866110322269u64));
}
};
accum.push(next_elem)
};
accum
};
{
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
return Err(ParseError::ExcludedBranch(671785503440284610u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
PResult::Ok(tar_main { contents })
}

/// d#427
fn Decoder427(_input: &mut Parser<'_>) -> Result<Vec<u8>, ParseError> {
let chars = {
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
let next_elem = {
let b = _input.read_byte()?;
if b != 0 {
b
} else {
return Err(ParseError::ExcludedBranch(8284996377131534070u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(16948595091237008543u64));
}
};
PResult::Ok(chars.clone())
}

/// d#428
fn Decoder428(_input: &mut Parser<'_>) -> Result<gzip_fcomment, ParseError> {
let comment = {
let chars = {
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
return Err(ParseError::ExcludedBranch(108256050843388088u64));
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
return Err(ParseError::ExcludedBranch(7834223795690054720u64));
}
};
accum.push(next_elem)
} else {
break
}
};
accum
};
{
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(787921071240225899u64));
}
};
chars.clone()
};
PResult::Ok(gzip_fcomment { comment })
}

/// d#429
fn Decoder_gif_header(_input: &mut Parser<'_>) -> Result<gif_header, ParseError> {
let signature = {
let arg0 = {
let b = _input.read_byte()?;
if b == 71 {
b
} else {
return Err(ParseError::ExcludedBranch(11557974043504662535u64));
}
};
let arg1 = {
let b = _input.read_byte()?;
if b == 73 {
b
} else {
return Err(ParseError::ExcludedBranch(14238499412440345954u64));
}
};
let arg2 = {
let b = _input.read_byte()?;
if b == 70 {
b
} else {
return Err(ParseError::ExcludedBranch(14703397633096852322u64));
}
};
(arg0, arg1, arg2)
};
let version = vec![_input.read_byte()?, _input.read_byte()?, _input.read_byte()?];
PResult::Ok(gif_header { signature, version })
}

/// d#430
fn Decoder_gif_logical_screen(_input: &mut Parser<'_>) -> Result<gif_logical_screen, ParseError> {
let descriptor = (Decoder_gif_logical_screen_descriptor(_input))?;
let global_color_table = if descriptor.flags.table_flag {
let mut accum = Vec::new();
for _ in 0..2u16 << (descriptor.flags.table_size as u16) {
let next_elem = (Decoder444(_input))?;
accum.push(next_elem)
};
Some(accum)
} else {
None
};
PResult::Ok(gif_logical_screen { descriptor, global_color_table })
}

/// d#431
fn Decoder_gif_block(_input: &mut Parser<'_>) -> Result<gif_block, ParseError> {
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
return Err(ParseError::ExcludedBranch(268478239438800266u64));
}
}
},

44u8 => {
0
},

_ => {
return Err(ParseError::ExcludedBranch(6301432137385173939u64));
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
return Err(ParseError::ExcludedBranch(17762152702085771435u64));
}
})
}

/// d#432
fn Decoder_gif_trailer(_input: &mut Parser<'_>) -> Result<gif_trailer, ParseError> {
let separator = {
let b = _input.read_byte()?;
if b == 59 {
b
} else {
return Err(ParseError::ExcludedBranch(6803443871185192093u64));
}
};
PResult::Ok(gif_trailer { separator })
}

/// d#433
fn Decoder_gif_graphic_block(_input: &mut Parser<'_>) -> Result<gif_graphic_block, ParseError> {
let graphic_control_extension = {
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
return Err(ParseError::ExcludedBranch(204361505388532862u64));
}
}
},

44u8 => {
1
},

_ => {
return Err(ParseError::ExcludedBranch(11161970641928094938u64));
}
};
_input.close_peek_context()?;
ret
}
};
match tree_index {
0 => {
Some((Decoder_gif_graphic_control_extension(_input))?)
},

1 => {
None
},

_ => {
return Err(ParseError::ExcludedBranch(8787536656121914522u64));
}
}
};
let graphic_rendering_block = (Decoder_gif_graphic_rendering_block(_input))?;
PResult::Ok(gif_graphic_block { graphic_control_extension, graphic_rendering_block })
}

/// d#434
fn Decoder_gif_special_purpose_block(_input: &mut Parser<'_>) -> Result<gif_special_purpose_block, ParseError> {
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
return Err(ParseError::ExcludedBranch(18210277358428599455u64));
}
}
} else {
return Err(ParseError::ExcludedBranch(9453259036882642818u64));
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
return Err(ParseError::ExcludedBranch(16811847696882257499u64));
}
})
}

/// d#435
fn Decoder_gif_application_extension(_input: &mut Parser<'_>) -> Result<gif_application_extension, ParseError> {
let separator = {
let b = _input.read_byte()?;
if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(11078254580983048780u64));
}
};
let label = {
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(1845850007550452160u64));
}
};
let block_size = {
let b = _input.read_byte()?;
if b == 11 {
b
} else {
return Err(ParseError::ExcludedBranch(7779176190297216638u64));
}
};
let identifier = {
let mut accum = Vec::new();
for _ in 0..8u8 {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
let authentication_code = vec![_input.read_byte()?, _input.read_byte()?, _input.read_byte()?];
let application_data = {
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
return Err(ParseError::ExcludedBranch(1016564408906296566u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_gif_subblock(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let terminator = (Decoder438(_input))?;
PResult::Ok(gif_application_extension { separator, label, block_size, identifier, authentication_code, application_data, terminator })
}

/// d#436
fn Decoder_gif_comment_extension(_input: &mut Parser<'_>) -> Result<gif_comment_extension, ParseError> {
let separator = {
let b = _input.read_byte()?;
if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(1591903561633999639u64));
}
};
let label = {
let b = _input.read_byte()?;
if b == 254 {
b
} else {
return Err(ParseError::ExcludedBranch(9590821821210520154u64));
}
};
let comment_data = {
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
return Err(ParseError::ExcludedBranch(6175893723851407495u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_gif_subblock(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let terminator = (Decoder438(_input))?;
PResult::Ok(gif_comment_extension { separator, label, comment_data, terminator })
}

/// d#437
fn Decoder_gif_subblock(_input: &mut Parser<'_>) -> Result<gif_subblock, ParseError> {
let len_bytes = {
let b = _input.read_byte()?;
if b != 0 {
b
} else {
return Err(ParseError::ExcludedBranch(4491050975676636472u64));
}
};
let data = {
let mut accum = Vec::new();
for _ in 0..len_bytes {
let next_elem = _input.read_byte()?;
accum.push(next_elem)
};
accum
};
PResult::Ok(gif_subblock { len_bytes, data })
}

/// d#438
fn Decoder438(_input: &mut Parser<'_>) -> Result<u8, ParseError> {
let b = _input.read_byte()?;
PResult::Ok(if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(9933460716242958610u64));
})
}

/// d#439
fn Decoder_gif_graphic_control_extension(_input: &mut Parser<'_>) -> Result<gif_graphic_control_extension, ParseError> {
let separator = {
let b = _input.read_byte()?;
if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(10865781264025109219u64));
}
};
let label = {
let b = _input.read_byte()?;
if b == 249 {
b
} else {
return Err(ParseError::ExcludedBranch(1125515999835788388u64));
}
};
let block_size = {
let b = _input.read_byte()?;
if b == 4 {
b
} else {
return Err(ParseError::ExcludedBranch(6928743980636918648u64));
}
};
let flags = (Decoder_gif_graphic_control_extension_flags(_input))?;
let delay_time = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let transparent_color_index = _input.read_byte()?;
let terminator = (Decoder438(_input))?;
PResult::Ok(gif_graphic_control_extension { separator, label, block_size, flags, delay_time, transparent_color_index, terminator })
}

/// d#440
fn Decoder_gif_graphic_rendering_block(_input: &mut Parser<'_>) -> Result<gif_graphic_rendering_block, ParseError> {
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
return Err(ParseError::ExcludedBranch(10349067556055585673u64));
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
return Err(ParseError::ExcludedBranch(9276145871181842621u64));
}
})
}

/// d#441
fn Decoder_gif_table_based_image(_input: &mut Parser<'_>) -> Result<gif_table_based_image, ParseError> {
let descriptor = (Decoder_gif_image_descriptor(_input))?;
let local_color_table = if descriptor.flags.table_flag {
let mut accum = Vec::new();
for _ in 0..2u16 << (descriptor.flags.table_size as u16) {
let next_elem = (Decoder444(_input))?;
accum.push(next_elem)
};
Some(accum)
} else {
None
};
let data = (Decoder_gif_table_based_image_data(_input))?;
PResult::Ok(gif_table_based_image { descriptor, local_color_table, data })
}

/// d#442
fn Decoder_gif_plain_text_extension(_input: &mut Parser<'_>) -> Result<gif_plain_text_extension, ParseError> {
let separator = {
let b = _input.read_byte()?;
if b == 33 {
b
} else {
return Err(ParseError::ExcludedBranch(16224083238566163922u64));
}
};
let label = {
let b = _input.read_byte()?;
if b == 1 {
b
} else {
return Err(ParseError::ExcludedBranch(9651999844283402729u64));
}
};
let block_size = {
let b = _input.read_byte()?;
if b == 12 {
b
} else {
return Err(ParseError::ExcludedBranch(4150962867603307131u64));
}
};
let text_grid_left_position = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let text_grid_top_position = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let text_grid_width = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let text_grid_height = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let character_cell_width = _input.read_byte()?;
let character_cell_height = _input.read_byte()?;
let text_foreground_color_index = _input.read_byte()?;
let text_background_color_index = _input.read_byte()?;
let plain_text_data = {
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
return Err(ParseError::ExcludedBranch(1180075112413234847u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_gif_subblock(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let terminator = (Decoder438(_input))?;
PResult::Ok(gif_plain_text_extension { separator, label, block_size, text_grid_left_position, text_grid_top_position, text_grid_width, text_grid_height, character_cell_width, character_cell_height, text_foreground_color_index, text_background_color_index, plain_text_data, terminator })
}

/// d#443
fn Decoder_gif_image_descriptor(_input: &mut Parser<'_>) -> Result<gif_image_descriptor, ParseError> {
let separator = {
let b = _input.read_byte()?;
if b == 44 {
b
} else {
return Err(ParseError::ExcludedBranch(4914981965961925407u64));
}
};
let image_left_position = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let image_top_position = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let image_width = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let image_height = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let flags = (Decoder_gif_image_descriptor_flags(_input))?;
PResult::Ok(gif_image_descriptor { separator, image_left_position, image_top_position, image_width, image_height, flags })
}

/// d#444
fn Decoder444(_input: &mut Parser<'_>) -> Result<gif_color_table_entry, ParseError> {
let r = _input.read_byte()?;
let g = _input.read_byte()?;
let b = _input.read_byte()?;
PResult::Ok(gif_color_table_entry { r, g, b })
}

/// d#445
fn Decoder_gif_table_based_image_data(_input: &mut Parser<'_>) -> Result<gif_table_based_image_data, ParseError> {
let lzw_min_code_size = _input.read_byte()?;
let image_data = {
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
return Err(ParseError::ExcludedBranch(4792114144900142999u64));
}
};
_input.close_peek_context()?;
ret
}
};
if matching_ix == 0 {
let next_elem = (Decoder_gif_subblock(_input))?;
accum.push(next_elem)
} else {
break
}
};
accum
};
let terminator = (Decoder438(_input))?;
PResult::Ok(gif_table_based_image_data { lzw_min_code_size, image_data, terminator })
}

/// d#446
fn Decoder_gif_image_descriptor_flags(_input: &mut Parser<'_>) -> Result<gif_image_descriptor_flags, ParseError> {
let _packed_bits = _input.read_byte()?;
let table_flag = _packed_bits >> 7u8 & 1u8 > 0u8;
let interlace_flag = _packed_bits >> 6u8 & 1u8 > 0u8;
let sort_flag = _packed_bits >> 5u8 & 1u8 > 0u8;
let table_size = _packed_bits & 7u8;
PResult::Ok(gif_image_descriptor_flags { table_flag, interlace_flag, sort_flag, table_size })
}

/// d#447
fn Decoder_gif_graphic_control_extension_flags(_input: &mut Parser<'_>) -> Result<gif_graphic_control_extension_flags, ParseError> {
let _packed_bits = _input.read_byte()?;
let disposal_method = _packed_bits >> 2u8 & 7u8;
let user_input_flag = _packed_bits >> 1u8 & 1u8 > 0u8;
let transparent_color_flag = _packed_bits & 1u8 > 0u8;
PResult::Ok(gif_graphic_control_extension_flags { disposal_method, user_input_flag, transparent_color_flag })
}

/// d#448
fn Decoder_gif_logical_screen_descriptor(_input: &mut Parser<'_>) -> Result<gif_logical_screen_descriptor, ParseError> {
let screen_width = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let screen_height = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16le(x)
};
let flags = (Decoder_gif_logical_screen_descriptor_flags(_input))?;
let bg_color_index = _input.read_byte()?;
let pixel_aspect_ratio = _input.read_byte()?;
PResult::Ok(gif_logical_screen_descriptor { screen_width, screen_height, flags, bg_color_index, pixel_aspect_ratio })
}

/// d#449
fn Decoder_gif_logical_screen_descriptor_flags(_input: &mut Parser<'_>) -> Result<gif_logical_screen_descriptor_flags, ParseError> {
let _packed_bits = _input.read_byte()?;
let table_flag = _packed_bits >> 7u8 & 1u8 > 0u8;
let color_resolution = _packed_bits >> 4u8 & 7u8;
let sort_flag = _packed_bits >> 3u8 & 1u8 > 0u8;
let table_size = _packed_bits & 7u8;
PResult::Ok(gif_logical_screen_descriptor_flags { table_flag, color_resolution, sort_flag, table_size })
}

/// d#450
fn Decoder450(_input: &mut Parser<'_>) -> Result<u32, ParseError> {
let tuple_var = {
let arg0 = {
let mut accum = Vec::new();
while _input.remaining() > 0 {
let reps_left = {
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
return Err(ParseError::ExcludedBranch(6091354260726402337u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(16173518488310098141u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(6485872802951288360u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14222995392916087968u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(9204716142356529030u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(14055113049862078139u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(8236384974725516720u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4851501206534868925u64));
}
}
},

_ => {
return Err(ParseError::ExcludedBranch(4255480819017852249u64));
}
};
_input.close_peek_context()?;
ret
}
};
if (repeat_between_finished(reps_left == 0, accum.len(), 0u16 as usize, 9u16 as usize))? {
break
} else {
let next_elem = {
let b = _input.read_byte()?;
if b == 83 {
b
} else {
return Err(ParseError::ExcludedBranch(17733863216727871551u64));
}
};
accum.push(next_elem)
}
};
accum
};
let arg1 = {
let b = _input.read_byte()?;
if b == 90 {
b
} else {
return Err(ParseError::ExcludedBranch(389354767835086292u64));
}
};
(arg0, arg1)
};
PResult::Ok({
let (s, _z) = tuple_var;
(s.len()) as u32
})
}

/// d#451
fn Decoder_jpeg_dhp_image_component_sampling_factor(_input: &mut Parser<'_>) -> Result<jpeg_dhp_image_component_sampling_factor, ParseError> {
let _packed_bits = _input.read_byte()?;
let horizontal = _packed_bits >> 4u8 & 15u8;
let vertical = _packed_bits & 15u8;
PResult::Ok(jpeg_dhp_image_component_sampling_factor { horizontal, vertical })
}

/// d#452
fn Decoder452(_input: &mut Parser<'_>) -> Result<jpeg_sof_image_component, ParseError> {
let id = _input.read_byte()?;
let sampling_factor = (Decoder453(_input))?;
let quantization_table_id = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(3216028881355025849u64));
}
};
PResult::Ok(jpeg_sof_image_component { id, sampling_factor, quantization_table_id })
}

/// d#453
fn Decoder453(_input: &mut Parser<'_>) -> Result<jpeg_dhp_image_component_sampling_factor, ParseError> {
let _packed_bits = _input.read_byte()?;
let horizontal = _packed_bits >> 4u8 & 15u8;
let vertical = _packed_bits & 15u8;
PResult::Ok(jpeg_dhp_image_component_sampling_factor { horizontal, vertical })
}

/// d#454
fn Decoder454(_input: &mut Parser<'_>) -> Result<jpeg_sof_data, ParseError> {
let sample_precision = _input.read_byte()?;
let num_lines = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let num_samples_per_line = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x != 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(479680595494617916u64));
}
};
let num_image_components = {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x != 0u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(10531068763070667405u64));
}
};
let image_components = {
let mut accum = Vec::new();
for _ in 0..num_image_components {
let next_elem = (Decoder455(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(jpeg_sof_data { sample_precision, num_lines, num_samples_per_line, num_image_components, image_components })
}

/// d#455
fn Decoder455(_input: &mut Parser<'_>) -> Result<jpeg_sof_image_component, ParseError> {
let id = _input.read_byte()?;
let sampling_factor = (Decoder453(_input))?;
let quantization_table_id = {
let b = _input.read_byte()?;
if b == 0 {
b
} else {
return Err(ParseError::ExcludedBranch(12431089125438936538u64));
}
};
PResult::Ok(jpeg_sof_image_component { id, sampling_factor, quantization_table_id })
}

/// d#456
fn Decoder_jpeg_exp_data(_input: &mut Parser<'_>) -> Result<jpeg_exp_data, ParseError> {
let expand_horizontal_vertical = {
let inner = {
let _packed_bits = _input.read_byte()?;
let expand_horizontal = _packed_bits >> 4u8 & 15u8;
let expand_vertical = _packed_bits & 15u8;
jpeg_exp_data_expand_horizontal_vertical { expand_horizontal, expand_vertical }
};
let is_valid = {
let x = inner;
(x.expand_horizontal <= 1u8) && (x.expand_vertical <= 1u8)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(9066809807580136020u64));
}
};
PResult::Ok(jpeg_exp_data { expand_horizontal_vertical })
}

/// d#457
fn Decoder457(_input: &mut Parser<'_>) -> Result<jpeg_app2, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(12347909352078849049u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 200 {
b
} else {
return Err(ParseError::ExcludedBranch(10900015085754267678u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 7620281735474506525u64)) as usize;
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
let next_elem = _input.read_byte()?;
accum.push(next_elem)
} else {
break
}
};
PResult::Ok(accum)
})())?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_app2 { marker, length, data })
}

/// d#458
fn Decoder458(_input: &mut Parser<'_>) -> Result<jpeg_sof0, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(5999000939515818447u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 222 {
b
} else {
return Err(ParseError::ExcludedBranch(10508718825232435214u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 11599300513837427027u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder459(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_sof0 { marker, length, data })
}

/// d#459
fn Decoder459(_input: &mut Parser<'_>) -> Result<jpeg_sof_data, ParseError> {
let sample_precision = _input.read_byte()?;
let num_lines = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let num_samples_per_line = {
let inner = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let is_valid = {
let x = inner;
x != 0u16
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13460389694602013078u64));
}
};
let num_image_components = {
let inner = _input.read_byte()?;
let is_valid = {
let x = inner;
x != 0u8
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(16535687493193441589u64));
}
};
let image_components = {
let mut accum = Vec::new();
for _ in 0..num_image_components {
let next_elem = (Decoder455(_input))?;
accum.push(next_elem)
};
accum
};
PResult::Ok(jpeg_sof_data { sample_precision, num_lines, num_samples_per_line, num_image_components, image_components })
}

/// d#460
fn Decoder_jpeg_exp(_input: &mut Parser<'_>) -> Result<jpeg_exp, ParseError> {
let marker = {
{
let b = _input.read_byte()?;
if b == 255 {
b
} else {
return Err(ParseError::ExcludedBranch(16678380679225161433u64));
}
};
let marker = {
let b = _input.read_byte()?;
if b == 223 {
b
} else {
return Err(ParseError::ExcludedBranch(11416999240256111317u64));
}
};
jpeg_soi { marker }
};
let length = {
let x = (_input.read_byte()?, _input.read_byte()?);
u16be(x)
};
let data = {
let sz = (try_sub!(length, 2u16, 950046280632689001u64)) as usize;
_input.start_slice(sz)?;
let ret = (Decoder461(_input))?;
_input.end_slice()?;
ret
};
PResult::Ok(jpeg_exp { marker, length, data })
}

/// d#461
fn Decoder461(_input: &mut Parser<'_>) -> Result<jpeg_exp_data, ParseError> {
let expand_horizontal_vertical = {
let inner = {
let _packed_bits = _input.read_byte()?;
let expand_horizontal = _packed_bits >> 4u8 & 15u8;
let expand_vertical = _packed_bits & 15u8;
jpeg_exp_data_expand_horizontal_vertical { expand_horizontal, expand_vertical }
};
let is_valid = {
let x = inner;
(x.expand_horizontal <= 1u8) && (x.expand_vertical <= 1u8)
};
if is_valid {
inner
} else {
return Err(ParseError::FalsifiedWhere(13319962755558045565u64));
}
};
PResult::Ok(jpeg_exp_data { expand_horizontal_vertical })
}

