use doodle::prelude::*;

struct Type86 {
length: u32,
tag: (u8, u8, u8, u8),
data: Type62,
crc: u32
}

enum Type23 { dynamic_huffman {
hlit: u8,
hdist: u8,
hclen: u8,
code_length_alphabet_code_lengths: Vec<u8>,
literal_length_distance_alphabet_code_lengths: Vec<Type15>,
literal_length_distance_alphabet_code_lengths_value: Vec<u8>,
literal_length_alphabet_code_lengths_value: Vec<u8>,
distance_alphabet_code_lengths_value: Vec<u8>,
codes: Vec<Type18>,
codes_values: Vec<Type19>
}, fixed_huffman {
codes: Vec<Type21>,
codes_values: Vec<Type19>
}, uncompressed {
align: (),
len: u16,
nlen: u16,
bytes: Vec<u8>,
codes_values: Vec<Type22>
} }

struct Type92 {
marker: Type28,
length: u16,
data: Type40
}

struct Type84 {
length: u32,
tag: (u8, u8, u8, u8),
data: Vec<Type2>,
crc: u32
}

enum Type30 { other(Vec<u8>), jfif {
version_major: u8,
version_minor: u8,
density_units: u8,
density_x: u16,
density_y: u16,
thumbnail_width: u8,
thumbnail_height: u8,
thumbnail_pixels: Vec<Vec<Type2>>
} }

struct Type99 {
align: (),
len: u16,
nlen: u16,
bytes: Vec<u8>,
codes_values: Vec<Type22>
}

struct Type39 {
precision_table_id: u8,
elements: Vec<u8>
}

struct Type100 {
codes: Vec<Type21>,
codes_values: Vec<Type19>
}

struct Type21 {
code: u16,
extra: Type20
}

struct Type7 {
len_bytes: u8,
data: Vec<u8>
}

enum Type43 { dqt {
marker: Type28,
length: u16,
data: Type39
}, dht {
marker: Type28,
length: u16,
data: Type40
}, dac {
marker: Type28,
length: u16,
data: Type41
}, dri {
marker: Type28,
length: u16,
data: Type42
}, app0 {
marker: Type28,
length: u16,
data: Type31
}, app1 {
marker: Type28,
length: u16,
data: Type37
}, app2 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app3 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app4 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app5 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app6 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app7 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app8 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app9 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app10 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app11 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app12 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app13 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app14 {
marker: Type28,
length: u16,
data: Vec<u8>
}, app15 {
marker: Type28,
length: u16,
data: Vec<u8>
}, com {
marker: Type28,
length: u16,
data: Vec<u8>
} }

struct Type28 {
ff: u8,
marker: u8
}

struct Type57 {
length: u32,
tag: (u8, u8, u8, u8),
data: Type56,
crc: u32
}

struct Type77 {
header: Type0,
logical_screen: Type4,
blocks: Vec<Type11>,
trailer: Type12
}

struct Type59 {
pixels_per_unit_x: u32,
pixels_per_unit_y: u32,
unit_specifier: u8
}

struct Type67 {
tag: (u8, u8, u8, u8),
length: u32,
data: Vec<u8>,
pad: Type66
}

struct Type88 {
marker: Type28,
length: u16,
data: Type37
}

struct Type35 {
byte_order: Type32,
magic: u16,
offset: u32,
ifd: Type34
}

enum Type11 { graphic_block {
graphic_control_extension: Type5,
graphic_rendering_block: Type9
}, special_purpose_block(Type10) }

struct Type26 {
crc: u32,
length: u32
}

struct Type87 {
marker: Type28,
length: u16,
data: Type31
}

struct Type93 {
marker: Type28,
length: u16,
data: Type41
}

struct Type107 {
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
plain_text_data: Vec<Type7>,
terminator: u8
}

enum Type5 { some {
separator: u8,
label: u8,
block_size: u8,
flags: u8,
delay_time: u16,
transparent_color_index: u8,
terminator: u8
}, none() }

struct Type89 {
marker: Type28,
length: u16,
data: Type53
}

enum Type10 { application_extension {
separator: u8,
label: u8,
block_size: u8,
identifier: Vec<u8>,
authentication_code: Vec<u8>,
application_data: Vec<Type7>,
terminator: u8
}, comment_extension {
separator: u8,
label: u8,
comment_data: Vec<Type7>,
terminator: u8
} }

struct Type25 {
blocks: Vec<Type24>,
codes: Vec<Type19>,
inflate: Vec<u8>
}

struct Type33 {
tag: u16,
r#type: u16,
length: u32,
offset_or_data: u32
}

struct Type41 {
class_table_id: u8,
value: u8
}

struct Type2 {
r: u8,
g: u8,
b: u8
}

struct Type45 {
sample_precision: u8,
num_lines: u16,
num_samples_per_line: u16,
num_image_components: u8,
image_components: Vec<Type44>
}

struct Type96 {
padding: u8,
exif: Type35
}

struct Type98 {
version_major: u8,
version_minor: u8,
density_units: u8,
density_x: u16,
density_y: u16,
thumbnail_width: u8,
thumbnail_height: u8,
thumbnail_pixels: Vec<Vec<Type2>>
}

enum Type20 { none(), some {
length_extra_bits: u8,
length: u16,
distance_code: u8,
distance_record: Type16
} }

enum Type36 { other(Vec<u8>), xmp {
xmp: Vec<u8>
}, exif {
padding: u8,
exif: Type35
} }

struct Type72 {
name: Type69,
mode: Type70,
uid: Type70,
gid: Type70,
size: u32,
mtime: Type70,
chksum: Type70,
typeflag: u8,
linkname: Type69,
magic: (u8, u8, u8, u8, u8, u8),
version: (u8, u8),
uname: Type71,
gname: Type71,
devmajor: Type70,
devminor: Type70,
prefix: Type69,
pad: Vec<u8>
}

struct Type82 {
length: u32,
tag: (u8, u8, u8, u8),
data: Type58,
crc: u32
}

struct Type42 {
restart_interval: u16
}

enum Type19 { literal(u8), reference {
length: u16,
distance: u16
} }

struct Type44 {
id: u8,
sampling_factor: u8,
quantization_table_id: u8
}

struct Type60 {
year: u16,
month: u8,
day: u8,
hour: u8,
minute: u8,
second: u8
}

struct Type6 {
separator: u8,
image_left_position: u16,
image_top_position: u16,
image_width: u16,
image_height: u16,
flags: u8
}

enum Type54 { some {
marker: Type28,
length: u16,
data: Type53
}, none() }

struct Type31 {
identifier: Type29,
data: Type30
}

struct Type103 {
separator: u8,
label: u8,
block_size: u8,
identifier: Vec<u8>,
authentication_code: Vec<u8>,
application_data: Vec<Type7>,
terminator: u8
}

struct Type0 {
signature: (u8, u8, u8),
version: Vec<u8>
}

struct Type55 {
initial_segment: Type38,
segments: Vec<Type43>,
header: Type46,
scan: Type52,
dnl: Type54,
scans: Vec<Type52>
}

enum Type38 { app0 {
marker: Type28,
length: u16,
data: Type31
}, app1 {
marker: Type28,
length: u16,
data: Type37
} }

struct Type51 {
scan_data: Vec<Type50>,
scan_data_stream: Vec<u8>
}

enum Type58 { color_type_3 {
palette_index: u8
}, color_type_6 {
red: u16,
green: u16,
blue: u16
}, color_type_2 {
red: u16,
green: u16,
blue: u16
}, color_type_4 {
greyscale: u16
}, color_type_0 {
greyscale: u16
} }

struct Type13 {
magic: (u8, u8),
method: u8,
file_flags: u8,
timestamp: u32,
compression_flags: u8,
os_id: u8
}

struct Type49 {
marker: Type28,
length: u16,
data: Type48
}

enum Type66 { no(u8), yes() }

struct Type68 {
tag: (u8, u8, u8, u8),
chunks: Vec<Type67>
}

struct Type81 {
contents: Vec<Type73>,
__padding: Vec<u8>,
__trailing: Vec<u8>
}

struct Type101 {
hlit: u8,
hdist: u8,
hclen: u8,
code_length_alphabet_code_lengths: Vec<u8>,
literal_length_distance_alphabet_code_lengths: Vec<Type15>,
literal_length_distance_alphabet_code_lengths_value: Vec<u8>,
literal_length_alphabet_code_lengths_value: Vec<u8>,
distance_alphabet_code_lengths_value: Vec<u8>,
codes: Vec<Type18>,
codes_values: Vec<Type19>
}

struct Type105 {
separator: u8,
label: u8,
block_size: u8,
flags: u8,
delay_time: u16,
transparent_color_index: u8,
terminator: u8
}

struct Type12 {
separator: u8
}

struct Type106 {
descriptor: Type6,
local_color_table: Type3,
data: Type8
}

enum Type9 { table_based_image {
descriptor: Type6,
local_color_table: Type3,
data: Type8
}, plain_text_extension {
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
plain_text_data: Vec<Type7>,
terminator: u8
} }

struct Type56 {
width: u32,
height: u32,
bit_depth: u8,
color_type: u8,
compression_method: u8,
filter_method: u8,
interlace_method: u8
}

enum Type32 { le(u8, u8), be(u8, u8) }

struct Type8 {
lzw_min_code_size: u8,
image_data: Vec<Type7>,
terminator: u8
}

struct Type24 {
r#final: u8,
r#type: u8,
data: Type23
}

enum Type50 { mcu(u8), rst0 {
ff: u8,
marker: u8
}, rst1 {
ff: u8,
marker: u8
}, rst2 {
ff: u8,
marker: u8
}, rst3 {
ff: u8,
marker: u8
}, rst4 {
ff: u8,
marker: u8
}, rst5 {
ff: u8,
marker: u8
}, rst6 {
ff: u8,
marker: u8
}, rst7 {
ff: u8,
marker: u8
} }

struct Type94 {
marker: Type28,
length: u16,
data: Type42
}

struct Type83 {
length: u32,
tag: (u8, u8, u8, u8),
data: Type59,
crc: u32
}

struct Type53 {
num_lines: u16
}

struct Type79 {
signature: (u8, u8, u8, u8, u8, u8, u8, u8),
ihdr: Type57,
chunks: Vec<Type63>,
idat: Vec<Type64>,
more_chunks: Vec<Type63>,
iend: Type65
}

struct Type85 {
length: u32,
tag: (u8, u8, u8, u8),
data: Type60,
crc: u32
}

struct Type4 {
descriptor: Type1,
global_color_table: Type3
}

struct Type15 {
code: u16,
extra: u8
}

enum Type17 { none(), some {
length_extra_bits: u8,
length: u16,
distance_code: u16,
distance_record: Type16
} }

enum Type22 { literal(u8) }

struct Type29 {
string: Vec<u8>,
null: u8
}

enum Type3 { no(), yes(Vec<Type2>) }

struct Type37 {
identifier: Type29,
data: Type36
}

struct Type73 {
header: Type72,
file: Vec<u8>,
__padding: ()
}

struct Type102 {
graphic_control_extension: Type5,
graphic_rendering_block: Type9
}

struct Type69 {
string: Vec<u8>,
__padding: Vec<u8>
}

struct Type104 {
separator: u8,
label: u8,
comment_data: Vec<Type7>,
terminator: u8
}

struct Type48 {
num_image_components: u8,
image_components: Vec<Type47>,
start_spectral_selection: u8,
end_spectral_selection: u8,
approximation_bit_position: u8
}

enum Type62 { color_type_3(Vec<Type61>), color_type_2 {
red: u16,
green: u16,
blue: u16
}, color_type_0 {
greyscale: u16
} }

struct Type52 {
segments: Vec<Type43>,
sos: Type49,
data: Type51
}

enum Type75 { gif {
header: Type0,
logical_screen: Type4,
blocks: Vec<Type11>,
trailer: Type12
}, gzip(Vec<Type27>), jpeg {
soi: Type28,
frame: Type55,
eoi: Type28
}, png {
signature: (u8, u8, u8, u8, u8, u8, u8, u8),
ihdr: Type57,
chunks: Vec<Type63>,
idat: Vec<Type64>,
more_chunks: Vec<Type63>,
iend: Type65
}, riff {
tag: (u8, u8, u8, u8),
length: u32,
data: Type68,
pad: Type66
}, tar {
contents: Vec<Type73>,
__padding: Vec<u8>,
__trailing: Vec<u8>
}, text(Type74) }

struct Type91 {
marker: Type28,
length: u16,
data: Type39
}

struct Type70 {
string: Vec<u8>,
__nul_or_wsp: u8,
__padding: Vec<u8>
}

struct Type61 {
palette_index: u8
}

enum Type46 { sof0 {
marker: Type28,
length: u16,
data: Type45
}, sof1 {
marker: Type28,
length: u16,
data: Type45
}, sof2 {
marker: Type28,
length: u16,
data: Type45
}, sof3 {
marker: Type28,
length: u16,
data: Type45
}, sof5 {
marker: Type28,
length: u16,
data: Type45
}, sof6 {
marker: Type28,
length: u16,
data: Type45
}, sof7 {
marker: Type28,
length: u16,
data: Type45
}, sof9 {
marker: Type28,
length: u16,
data: Type45
}, sof10 {
marker: Type28,
length: u16,
data: Type45
}, sof11 {
marker: Type28,
length: u16,
data: Type45
}, sof13 {
marker: Type28,
length: u16,
data: Type45
}, sof14 {
marker: Type28,
length: u16,
data: Type45
}, sof15 {
marker: Type28,
length: u16,
data: Type45
} }

struct Type71 {
string: Vec<u8>,
padding: Vec<u8>
}

struct Type16 {
distance_extra_bits: u16,
distance: u16
}

struct Type40 {
class_table_id: u8,
num_codes: Vec<u8>,
values: Vec<u8>
}

struct Type47 {
component_selector: u8,
entropy_coding_table_ids: u8
}

struct Type65 {
length: u32,
tag: (u8, u8, u8, u8),
data: (),
crc: u32
}

struct Type80 {
tag: (u8, u8, u8, u8),
length: u32,
data: Type68,
pad: Type66
}

struct Type90 {
marker: Type28,
length: u16,
data: Type45
}

struct Type27 {
header: Type13,
fname: Type14,
data: Type25,
footer: Type26
}

enum Type74 { ascii(Vec<u8>), utf8(Vec<char>) }

struct Type95 {
marker: Type28,
length: u16,
data: Vec<u8>
}

struct Type97 {
xmp: Vec<u8>
}

struct Type1 {
screen_width: u16,
screen_height: u16,
flags: u8,
bg_color_index: u8,
pixel_aspect_ratio: u8
}

enum Type14 { no(), yes {
string: Vec<u8>,
null: u8
} }

struct Type76 {
data: Type75,
end: ()
}

struct Type64 {
length: u32,
tag: (u8, u8, u8, u8),
data: Vec<u8>,
crc: u32
}

struct Type78 {
soi: Type28,
frame: Type55,
eoi: Type28
}

enum Type63 { bKGD {
length: u32,
tag: (u8, u8, u8, u8),
data: Type58,
crc: u32
}, pHYs {
length: u32,
tag: (u8, u8, u8, u8),
data: Type59,
crc: u32
}, PLTE {
length: u32,
tag: (u8, u8, u8, u8),
data: Vec<Type2>,
crc: u32
}, tIME {
length: u32,
tag: (u8, u8, u8, u8),
data: Type60,
crc: u32
}, tRNS {
length: u32,
tag: (u8, u8, u8, u8),
data: Type62,
crc: u32
} }

struct Type18 {
code: u16,
extra: Type17
}

struct Type34 {
num_fields: u16,
fields: Vec<Type33>,
next_ifd_offset: u32,
next_ifd: Vec<u8>
}

fn Decoder0<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type76> {
return Some(Decoder1(scope, input)?);
}

fn Decoder1<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type76> {
let data = {
let tmp = r#"invoke_decoder @ Discriminant(6)"#;
unimplemented!(r#"{}"#, tmp)
};
let end = if input.read_byte().is_none() {
()
} else {
return None;
};
return Some(Type76 { data, end });
}

fn Decoder2<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type77> {
let header = Decoder128(scope, input)?;
let logical_screen = Decoder129(scope, input)?;
let blocks = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let trailer = Decoder131(scope, input)?;
return Some(Type77 { header, logical_screen, blocks, trailer });
}

fn Decoder3<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<Type27>> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(11)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder4<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type78> {
let soi = Decoder45(scope, input)?;
let frame = Decoder46(scope, input)?;
let eoi = Decoder47(scope, input)?;
return Some(Type78 { soi, frame, eoi });
}

fn Decoder5<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type79> {
let signature = Decoder27(scope, input)?;
let ihdr = Decoder28(scope, input)?;
let chunks = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let idat = {
let tmp = r#"invoke_decoder @ Discriminant(11)"#;
unimplemented!(r#"{}"#, tmp)
};
let more_chunks = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let iend = Decoder31(scope, input)?;
return Some(Type79 { signature, ihdr, chunks, idat, more_chunks, iend });
}

fn Decoder6<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type80> {
let tag = {
let tmp = r#"invoke_decoder @ Discriminant(8)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder23(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
let pad = {
let tmp = r#"invoke_decoder @ Discriminant(23)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type80 { tag, length, data, pad });
}

fn Decoder7<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type81> {
let contents = {
let tmp = r#"invoke_decoder @ Discriminant(11)"#;
unimplemented!(r#"{}"#, tmp)
};
let __padding = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
let __trailing = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type81 { contents, __padding, __trailing });
}

fn Decoder8<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type74> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(6)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder9<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<u8>> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(11)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder10<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<char>> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder11<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<char> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder12<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder13<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
return Some({
let bs = ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
});
}

fn Decoder14<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type73> {
let header = Decoder15(scope, input)?;
let file = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
let __padding = {
while input.offset() % 512 != 0 {
let _ = input.read_byte()?;
}
()
};
return Some(Type73 { header, file, __padding });
}

fn Decoder15<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type72> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder16<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
return Some({
let bs = ByteSet::from_bits([18446744073709551615, 18446744073709551615, 18446744073709551615, 18446744073709551615]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
});
}

fn Decoder17<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
let string = {
let tmp = r#"invoke_decoder @ Discriminant(11)"#;
unimplemented!(r#"{}"#, tmp)
};
let __padding = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type69 { string, __padding });
}

fn Decoder18<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
return Some({
let bs = ByteSet::from_bits([71776119061217280, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
});
}

fn Decoder19<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
return Some({
let bs = ByteSet::from_bits([4294967297, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
});
}

fn Decoder20<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
return Some({
let bs = ByteSet::from_bits([18446744073709551615, 18446744073709551615, 18446744073709551615, 18446744073709551615]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
});
}

fn Decoder21<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
let string = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let __padding = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type69 { string, __padding });
}

fn Decoder22<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type71> {
let string = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let padding = {
let tmp = r#"invoke_decoder @ Discriminant(11)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type71 { string, padding });
}

fn Decoder23<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u32> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder24<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
let tag = Decoder25(scope, input)?;
let chunks = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type68 { tag, chunks });
}

fn Decoder25<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
let field0 = Decoder20(scope, input)?;
let field1 = Decoder20(scope, input)?;
let field2 = Decoder20(scope, input)?;
let field3 = Decoder20(scope, input)?;
return Some((field0, field1, field2, field3));
}

fn Decoder26<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type67> {
let tag = Decoder25(scope, input)?;
let length = Decoder23(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
let pad = {
let tmp = r#"invoke_decoder @ Discriminant(23)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type67 { tag, length, data, pad });
}

fn Decoder27<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8, u8, u8, u8, u8)> {
let field0 = {
let bs = ByteSet::from_bits([0, 0, 512, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field1 = {
let bs = ByteSet::from_bits([0, 65536, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field2 = {
let bs = ByteSet::from_bits([0, 16384, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field3 = {
let bs = ByteSet::from_bits([0, 128, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field4 = {
let bs = ByteSet::from_bits([8192, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field5 = {
let bs = ByteSet::from_bits([1024, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field6 = {
let bs = ByteSet::from_bits([67108864, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field7 = {
let bs = ByteSet::from_bits([1024, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some((field0, field1, field2, field3, field4, field5, field6, field7));
}

fn Decoder28<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type57> {
let length = Decoder32(scope, input)?;
let tag = Decoder43(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
let crc = Decoder32(scope, input)?;
return Some(Type57 { length, tag, data, crc });
}

fn Decoder29<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type63> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder30<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type64> {
let length = Decoder32(scope, input)?;
let tag = Decoder35(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
let crc = Decoder32(scope, input)?;
return Some(Type64 { length, tag, data, crc });
}

fn Decoder31<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
let length = Decoder32(scope, input)?;
let tag = Decoder33(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
let crc = Decoder32(scope, input)?;
return Some(Type65 { length, tag, data, crc });
}

fn Decoder32<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u32> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder33<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
let field0 = {
let bs = ByteSet::from_bits([0, 512, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field1 = {
let bs = ByteSet::from_bits([0, 32, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field2 = {
let bs = ByteSet::from_bits([0, 16384, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field3 = {
let bs = ByteSet::from_bits([0, 16, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some((field0, field1, field2, field3));
}

fn Decoder34<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<()> {
Some(())
}

fn Decoder35<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
let field0 = {
let bs = ByteSet::from_bits([0, 512, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field1 = {
let bs = ByteSet::from_bits([0, 16, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field2 = {
let bs = ByteSet::from_bits([0, 2, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field3 = {
let bs = ByteSet::from_bits([0, 1048576, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some((field0, field1, field2, field3));
}

fn Decoder36<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<u8>> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder37<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type82> {
let length = Decoder32(scope, input)?;
let tag = {
let tmp = r#"invoke_decoder @ Discriminant(8)"#;
unimplemented!(r#"{}"#, tmp)
};
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
let crc = Decoder32(scope, input)?;
return Some(Type82 { length, tag, data, crc });
}

fn Decoder38<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type83> {
let length = Decoder32(scope, input)?;
let tag = {
let tmp = r#"invoke_decoder @ Discriminant(8)"#;
unimplemented!(r#"{}"#, tmp)
};
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
let crc = Decoder32(scope, input)?;
return Some(Type83 { length, tag, data, crc });
}

fn Decoder39<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type84> {
let length = Decoder32(scope, input)?;
let tag = {
let tmp = r#"invoke_decoder @ Discriminant(8)"#;
unimplemented!(r#"{}"#, tmp)
};
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
let crc = Decoder32(scope, input)?;
return Some(Type84 { length, tag, data, crc });
}

fn Decoder40<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type85> {
let length = Decoder32(scope, input)?;
let tag = {
let tmp = r#"invoke_decoder @ Discriminant(8)"#;
unimplemented!(r#"{}"#, tmp)
};
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
let crc = Decoder32(scope, input)?;
return Some(Type85 { length, tag, data, crc });
}

fn Decoder41<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type86> {
let length = Decoder32(scope, input)?;
let tag = {
let tmp = r#"invoke_decoder @ Discriminant(8)"#;
unimplemented!(r#"{}"#, tmp)
};
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
let crc = Decoder32(scope, input)?;
return Some(Type86 { length, tag, data, crc });
}

fn Decoder42<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u16> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder43<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
let field0 = {
let bs = ByteSet::from_bits([0, 512, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field1 = {
let bs = ByteSet::from_bits([0, 256, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field2 = {
let bs = ByteSet::from_bits([0, 16, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let field3 = {
let bs = ByteSet::from_bits([0, 262144, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some((field0, field1, field2, field3));
}

fn Decoder44<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
let width = Decoder32(scope, input)?;
let height = Decoder32(scope, input)?;
let bit_depth = Decoder16(scope, input)?;
let color_type = Decoder16(scope, input)?;
let compression_method = Decoder16(scope, input)?;
let filter_method = Decoder16(scope, input)?;
let interlace_method = Decoder16(scope, input)?;
return Some(Type56 { width, height, bit_depth, color_type, compression_method, filter_method, interlace_method });
}

fn Decoder45<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type28> {
let ff = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let marker = {
let bs = ByteSet::from_bits([0, 0, 0, 16777216]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type28 { ff, marker });
}

fn Decoder46<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type55> {
let initial_segment = {
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
};
let segments = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let header = Decoder51(scope, input)?;
let scan = Decoder52(scope, input)?;
let dnl = {
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
};
let scans = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type55 { initial_segment, segments, header, scan, dnl, scans });
}

fn Decoder47<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type28> {
let ff = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let marker = {
let bs = ByteSet::from_bits([0, 0, 0, 33554432]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type28 { ff, marker });
}

fn Decoder48<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type87> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type87 { marker, length, data });
}

fn Decoder49<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type88> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type88 { marker, length, data });
}

fn Decoder50<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type43> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder51<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type46> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder52<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type52> {
let segments = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let sos = Decoder55(scope, input)?;
let data = Decoder69(scope, input)?;
return Some(Type52 { segments, sos, data });
}

fn Decoder53<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type89> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type89 { marker, length, data });
}

fn Decoder54<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type52> {
let segments = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let sos = Decoder55(scope, input)?;
let data = Decoder56(scope, input)?;
return Some(Type52 { segments, sos, data });
}

fn Decoder55<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type49> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type49 { marker, length, data });
}

fn Decoder56<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type51> {
let scan_data = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let scan_data_stream = {
let tmp = r#"invoke_decoder @ Discriminant(21)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type51 { scan_data, scan_data_stream });
}

fn Decoder57<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder58<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type28> {
let ff = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let marker = {
let bs = ByteSet::from_bits([0, 0, 0, 65536]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type28 { ff, marker });
}

fn Decoder59<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type28> {
let ff = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let marker = {
let bs = ByteSet::from_bits([0, 0, 0, 131072]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type28 { ff, marker });
}

fn Decoder60<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type28> {
let ff = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let marker = {
let bs = ByteSet::from_bits([0, 0, 0, 262144]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type28 { ff, marker });
}

fn Decoder61<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type28> {
let ff = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let marker = {
let bs = ByteSet::from_bits([0, 0, 0, 524288]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type28 { ff, marker });
}

fn Decoder62<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type28> {
let ff = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let marker = {
let bs = ByteSet::from_bits([0, 0, 0, 1048576]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type28 { ff, marker });
}

fn Decoder63<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type28> {
let ff = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let marker = {
let bs = ByteSet::from_bits([0, 0, 0, 2097152]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type28 { ff, marker });
}

fn Decoder64<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type28> {
let ff = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let marker = {
let bs = ByteSet::from_bits([0, 0, 0, 4194304]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type28 { ff, marker });
}

fn Decoder65<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type28> {
let ff = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let marker = {
let bs = ByteSet::from_bits([0, 0, 0, 8388608]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type28 { ff, marker });
}

fn Decoder66<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
let num_image_components = Decoder16(scope, input)?;
let image_components = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
let start_spectral_selection = Decoder16(scope, input)?;
let end_spectral_selection = Decoder16(scope, input)?;
let approximation_bit_position = Decoder16(scope, input)?;
return Some(Type48 { num_image_components, image_components, start_spectral_selection, end_spectral_selection, approximation_bit_position });
}

fn Decoder67<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type47> {
let component_selector = Decoder16(scope, input)?;
let entropy_coding_table_ids = Decoder16(scope, input)?;
return Some(Type47 { component_selector, entropy_coding_table_ids });
}

fn Decoder68<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type53> {
let num_lines = Decoder42(scope, input)?;
return Some(Type53 { num_lines });
}

fn Decoder69<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type51> {
let scan_data = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let scan_data_stream = {
let tmp = r#"invoke_decoder @ Discriminant(21)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type51 { scan_data, scan_data_stream });
}

fn Decoder70<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder71<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder72<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder73<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder74<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder75<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder76<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder77<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder78<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder79<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder80<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder81<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder82<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type90 { marker, length, data });
}

fn Decoder83<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type45> {
let sample_precision = Decoder16(scope, input)?;
let num_lines = Decoder42(scope, input)?;
let num_samples_per_line = Decoder42(scope, input)?;
let num_image_components = Decoder16(scope, input)?;
let image_components = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type45 { sample_precision, num_lines, num_samples_per_line, num_image_components, image_components });
}

fn Decoder84<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type44> {
let id = Decoder16(scope, input)?;
let sampling_factor = Decoder16(scope, input)?;
let quantization_table_id = Decoder16(scope, input)?;
return Some(Type44 { id, sampling_factor, quantization_table_id });
}

fn Decoder85<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type91> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type91 { marker, length, data });
}

fn Decoder86<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type92> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type92 { marker, length, data });
}

fn Decoder87<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type93> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type93 { marker, length, data });
}

fn Decoder88<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type94> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type94 { marker, length, data });
}

fn Decoder89<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder90<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder91<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder92<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder93<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder94<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder95<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder96<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder97<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder98<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder99<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder100<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder101<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder102<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder103<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
let marker = {
let tmp = r#"invoke_decoder @ Discriminant(9)"#;
unimplemented!(r#"{}"#, tmp)
};
let length = Decoder42(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(17)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type95 { marker, length, data });
}

fn Decoder104<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
let restart_interval = Decoder42(scope, input)?;
return Some(Type42 { restart_interval });
}

fn Decoder105<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
let class_table_id = Decoder16(scope, input)?;
let value = Decoder16(scope, input)?;
return Some(Type41 { class_table_id, value });
}

fn Decoder106<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type40> {
let class_table_id = Decoder16(scope, input)?;
let num_codes = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
let values = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type40 { class_table_id, num_codes, values });
}

fn Decoder107<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type39> {
let precision_table_id = Decoder16(scope, input)?;
let elements = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type39 { precision_table_id, elements });
}

fn Decoder108<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type37> {
let identifier = Decoder109(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(23)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type37 { identifier, data });
}

fn Decoder109<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type29> {
let string = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let null = {
let bs = ByteSet::from_bits([1, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type29 { string, null });
}

fn Decoder110<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type96> {
let padding = {
let bs = ByteSet::from_bits([1, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let exif = Decoder112(scope, input)?;
return Some(Type96 { padding, exif });
}

fn Decoder111<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type97> {
let xmp = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type97 { xmp });
}

fn Decoder112<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type35> {
let byte_order = {
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
};
let magic = {
let tmp = r#"invoke_decoder @ Discriminant(23)"#;
unimplemented!(r#"{}"#, tmp)
};
let offset = {
let tmp = r#"invoke_decoder @ Discriminant(23)"#;
unimplemented!(r#"{}"#, tmp)
};
let ifd = {
let tmp = r#"invoke_decoder @ Discriminant(19)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type35 { byte_order, magic, offset, ifd });
}

fn Decoder113<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u16> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder114<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type31> {
let identifier = Decoder115(scope, input)?;
let data = {
let tmp = r#"invoke_decoder @ Discriminant(23)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type31 { identifier, data });
}

fn Decoder115<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type29> {
let string = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let null = {
let bs = ByteSet::from_bits([1, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type29 { string, null });
}

fn Decoder116<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type98> {
let version_major = Decoder16(scope, input)?;
let version_minor = Decoder16(scope, input)?;
let density_units = Decoder16(scope, input)?;
let density_x = Decoder42(scope, input)?;
let density_y = Decoder42(scope, input)?;
let thumbnail_width = Decoder16(scope, input)?;
let thumbnail_height = Decoder16(scope, input)?;
let thumbnail_pixels = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type98 { version_major, version_minor, density_units, density_x, density_y, thumbnail_width, thumbnail_height, thumbnail_pixels });
}

fn Decoder117<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type2> {
let r = Decoder16(scope, input)?;
let g = Decoder16(scope, input)?;
let b = Decoder16(scope, input)?;
return Some(Type2 { r, g, b });
}

fn Decoder118<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type13> {
let magic = {
let tmp = r#"invoke_decoder @ Discriminant(8)"#;
unimplemented!(r#"{}"#, tmp)
};
let method = Decoder16(scope, input)?;
let file_flags = Decoder16(scope, input)?;
let timestamp = Decoder23(scope, input)?;
let compression_flags = Decoder16(scope, input)?;
let os_id = Decoder16(scope, input)?;
return Some(Type13 { magic, method, file_flags, timestamp, compression_flags, os_id });
}

fn Decoder119<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type29> {
return Some(Decoder127(scope, input)?);
}

fn Decoder120<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type25> {
let blocks = {
let tmp = r#"invoke_decoder @ Discriminant(13)"#;
unimplemented!(r#"{}"#, tmp)
};
let codes = {
let tmp = r#"invoke_decoder @ Discriminant(21)"#;
unimplemented!(r#"{}"#, tmp)
};
let inflate = {
let tmp = r#"invoke_decoder @ Discriminant(21)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type25 { blocks, codes, inflate });
}

fn Decoder121<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type26> {
let crc = Decoder23(scope, input)?;
let length = Decoder23(scope, input)?;
return Some(Type26 { crc, length });
}

fn Decoder122<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type24> {
let r#final = Decoder123(scope, input)?;
let r#type = {
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
};
let data = {
let tmp = r#"invoke_decoder @ Discriminant(23)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type24 { r#final, r#type, data });
}

fn Decoder123<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
return Some({
let bs = ByteSet::from_bits([18446744073709551615, 18446744073709551615, 18446744073709551615, 18446744073709551615]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
});
}

fn Decoder124<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type99> {
let align = {
while input.offset() % 8 != 0 {
let _ = input.read_byte()?;
}
()
};
let len = {
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
};
let nlen = {
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
};
let bytes = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
let codes_values = {
let tmp = r#"invoke_decoder @ Discriminant(21)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type99 { align, len, nlen, bytes, codes_values });
}

fn Decoder125<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type100> {
let codes = {
let tmp = r#"invoke_decoder @ Discriminant(24)"#;
unimplemented!(r#"{}"#, tmp)
};
let codes_values = {
let tmp = r#"invoke_decoder @ Discriminant(21)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type100 { codes, codes_values });
}

fn Decoder126<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type101> {
let hlit = {
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
};
let hdist = {
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
};
let hclen = {
let tmp = r#"invoke_decoder @ Discriminant(20)"#;
unimplemented!(r#"{}"#, tmp)
};
let code_length_alphabet_code_lengths = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
let literal_length_distance_alphabet_code_lengths = {
let tmp = r#"invoke_decoder @ Discriminant(24)"#;
unimplemented!(r#"{}"#, tmp)
};
let literal_length_distance_alphabet_code_lengths_value = {
let tmp = r#"invoke_decoder @ Discriminant(21)"#;
unimplemented!(r#"{}"#, tmp)
};
let literal_length_alphabet_code_lengths_value = {
let tmp = r#"invoke_decoder @ Discriminant(21)"#;
unimplemented!(r#"{}"#, tmp)
};
let distance_alphabet_code_lengths_value = {
let tmp = r#"invoke_decoder @ Discriminant(21)"#;
unimplemented!(r#"{}"#, tmp)
};
let codes = {
let tmp = r#"invoke_decoder @ Discriminant(24)"#;
unimplemented!(r#"{}"#, tmp)
};
let codes_values = {
let tmp = r#"invoke_decoder @ Discriminant(21)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type101 { hlit, hdist, hclen, code_length_alphabet_code_lengths, literal_length_distance_alphabet_code_lengths, literal_length_distance_alphabet_code_lengths_value, literal_length_alphabet_code_lengths_value, distance_alphabet_code_lengths_value, codes, codes_values });
}

fn Decoder127<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type29> {
let string = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let null = {
let bs = ByteSet::from_bits([1, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type29 { string, null });
}

fn Decoder128<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type0> {
let signature = {
let tmp = r#"invoke_decoder @ Discriminant(8)"#;
unimplemented!(r#"{}"#, tmp)
};
let version = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type0 { signature, version });
}

fn Decoder129<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type4> {
let descriptor = Decoder145(scope, input)?;
let global_color_table = {
let tmp = r#"invoke_decoder @ Discriminant(23)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type4 { descriptor, global_color_table });
}

fn Decoder130<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type11> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder131<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type12> {
let separator = {
let bs = ByteSet::from_bits([576460752303423488, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
return Some(Type12 { separator });
}

fn Decoder132<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type102> {
let graphic_control_extension = {
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
};
let graphic_rendering_block = Decoder139(scope, input)?;
return Some(Type102 { graphic_control_extension, graphic_rendering_block });
}

fn Decoder133<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type10> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder134<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type103> {
let separator = {
let bs = ByteSet::from_bits([8589934592, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let label = {
let bs = ByteSet::from_bits([0, 0, 0, 9223372036854775808]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let block_size = {
let bs = ByteSet::from_bits([2048, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let identifier = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
let authentication_code = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
let application_data = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let terminator = Decoder137(scope, input)?;
return Some(Type103 { separator, label, block_size, identifier, authentication_code, application_data, terminator });
}

fn Decoder135<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type104> {
let separator = {
let bs = ByteSet::from_bits([8589934592, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let label = {
let bs = ByteSet::from_bits([0, 0, 0, 4611686018427387904]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let comment_data = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let terminator = Decoder137(scope, input)?;
return Some(Type104 { separator, label, comment_data, terminator });
}

fn Decoder136<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type7> {
let len_bytes = {
let bs = ByteSet::from_bits([18446744073709551614, 18446744073709551615, 18446744073709551615, 18446744073709551615]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let data = {
let tmp = r#"invoke_decoder @ Discriminant(12)"#;
unimplemented!(r#"{}"#, tmp)
};
return Some(Type7 { len_bytes, data });
}

fn Decoder137<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
return Some({
let bs = ByteSet::from_bits([1, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
});
}

fn Decoder138<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type105> {
let separator = {
let bs = ByteSet::from_bits([8589934592, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let label = {
let bs = ByteSet::from_bits([0, 0, 0, 144115188075855872]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let block_size = {
let bs = ByteSet::from_bits([16, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let flags = Decoder16(scope, input)?;
let delay_time = Decoder113(scope, input)?;
let transparent_color_index = Decoder16(scope, input)?;
let terminator = Decoder137(scope, input)?;
return Some(Type105 { separator, label, block_size, flags, delay_time, transparent_color_index, terminator });
}

fn Decoder139<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type9> {
return Some({
let tmp = r#"invoke_decoder @ Discriminant(7)"#;
unimplemented!(r#"{}"#, tmp)
});
}

fn Decoder140<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type106> {
let descriptor = Decoder142(scope, input)?;
let local_color_table = {
let tmp = r#"invoke_decoder @ Discriminant(23)"#;
unimplemented!(r#"{}"#, tmp)
};
let data = Decoder144(scope, input)?;
return Some(Type106 { descriptor, local_color_table, data });
}

fn Decoder141<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type107> {
let separator = {
let bs = ByteSet::from_bits([8589934592, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let label = {
let bs = ByteSet::from_bits([2, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let block_size = {
let bs = ByteSet::from_bits([4096, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let text_grid_left_position = Decoder113(scope, input)?;
let text_grid_top_position = Decoder113(scope, input)?;
let text_grid_width = Decoder113(scope, input)?;
let text_grid_height = Decoder113(scope, input)?;
let character_cell_width = Decoder16(scope, input)?;
let character_cell_height = Decoder16(scope, input)?;
let text_foreground_color_index = Decoder16(scope, input)?;
let text_background_color_index = Decoder16(scope, input)?;
let plain_text_data = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let terminator = Decoder137(scope, input)?;
return Some(Type107 { separator, label, block_size, text_grid_left_position, text_grid_top_position, text_grid_width, text_grid_height, character_cell_width, character_cell_height, text_foreground_color_index, text_background_color_index, plain_text_data, terminator });
}

fn Decoder142<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type6> {
let separator = {
let bs = ByteSet::from_bits([17592186044416, 0, 0, 0]);
let b = input.read_byte()?;
if bs.contains(b) {
b
} else {
return None;
}
};
let image_left_position = Decoder113(scope, input)?;
let image_top_position = Decoder113(scope, input)?;
let image_width = Decoder113(scope, input)?;
let image_height = Decoder113(scope, input)?;
let flags = Decoder16(scope, input)?;
return Some(Type6 { separator, image_left_position, image_top_position, image_width, image_height, flags });
}

fn Decoder143<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type2> {
let r = Decoder16(scope, input)?;
let g = Decoder16(scope, input)?;
let b = Decoder16(scope, input)?;
return Some(Type2 { r, g, b });
}

fn Decoder144<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type8> {
let lzw_min_code_size = Decoder16(scope, input)?;
let image_data = {
let tmp = r#"invoke_decoder @ Discriminant(10)"#;
unimplemented!(r#"{}"#, tmp)
};
let terminator = Decoder137(scope, input)?;
return Some(Type8 { lzw_min_code_size, image_data, terminator });
}

fn Decoder145<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type1> {
let screen_width = Decoder113(scope, input)?;
let screen_height = Decoder113(scope, input)?;
let flags = Decoder16(scope, input)?;
let bg_color_index = Decoder16(scope, input)?;
let pixel_aspect_ratio = Decoder16(scope, input)?;
return Some(Type1 { screen_width, screen_height, flags, bg_color_index, pixel_aspect_ratio });
}


#[test]
fn test_decoder_27() {
    // PNG signature
    let input = b"\x89PNG\r\n\x1A\n";
    let mut parse_ctxt = ParseCtxt::new(input);
    let mut scope = Scope::Empty;
    let ret = Decoder27(&mut scope, &mut parse_ctxt);
    assert!(ret.is_some());
}
