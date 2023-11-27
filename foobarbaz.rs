enum Type62 { color_type_(Vec<Type61>), color_type_ {
red: u16,
green: u16,
blue: u16
}, color_type_ {
greyscale: u16
} }

struct Type77 {
header: Type0,
logical_screen: Type4,
blocks: Vec<Type11>,
trailer: Type12
}

struct Type106 {
descriptor: Type6,
local_color_table: Type3,
data: Type8
}

enum Type32 { le(u8, u8), be(u8, u8) }

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

struct Type41 {
class_table_id: u8,
value: u8
}

enum Type17 { none(), some {
length_extra_bits: u8,
length: u16,
distance_code: u16,
distance_record: Type16
} }

struct Type60 {
year: u16,
month: u8,
day: u8,
hour: u8,
minute: u8,
second: u8
}

struct Type1 {
screen_width: u16,
screen_height: u16,
flags: u8,
bg_color_index: u8,
pixel_aspect_ratio: u8
}

struct Type65 {
length: u32,
tag: (u8, u8, u8, u8),
data: (),
crc: u32
}

struct Type56 {
width: u32,
height: u32,
bit_depth: u8,
color_type: u8,
compression_method: u8,
filter_method: u8,
interlace_method: u8
}

struct Type68 {
tag: (u8, u8, u8, u8),
chunks: Vec<Type67>
}

struct Type7 {
len_bytes: u8,
data: Vec<u8>
}

struct Type49 {
marker: Type28,
length: u16,
data: Type48
}

struct Type12 {
separator: u8
}

struct Type53 {
num_lines: u16
}

struct Type47 {
component_selector: u8,
entropy_coding_table_ids: u8
}

struct Type29 {
string: Vec<u8>,
null: u8
}

enum Type22 { literal(u8) }

enum Type54 { some {
marker: Type28,
length: u16,
data: Type53
}, none() }

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

struct Type76 {
data: Type75,
end: ()
}

struct Type4 {
descriptor: Type1,
global_color_table: Type3
}

struct Type42 {
restart_interval: u16
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

struct Type100 {
codes: Vec<Type21>,
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

struct Type48 {
num_image_components: u8,
image_components: Vec<Type47>,
start_spectral_selection: u8,
end_spectral_selection: u8,
approximation_bit_position: u8
}

struct Type35 {
byte_order: Type32,
magic: u16,
offset: u32,
ifd: Type34
}

struct Type31 {
identifier: Type29,
data: Type30
}

struct Type83 {
length: u32,
tag: (u8, u8, u8, u8),
data: Type59,
crc: u32
}

enum Type14 { no(), yes {
string: Vec<u8>,
null: u8
} }

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

struct Type8 {
lzw_min_code_size: u8,
image_data: Vec<Type7>,
terminator: u8
}

enum Type20 { none(), some {
length_extra_bits: u8,
length: u16,
distance_code: u8,
distance_record: Type16
} }

struct Type37 {
identifier: Type29,
data: Type36
}

struct Type27 {
header: Type13,
fname: Type14,
data: Type25,
footer: Type26
}

struct Type39 {
precision_table_id: u8,
elements: Vec<u8>
}

struct Type25 {
blocks: Vec<Type24>,
codes: Vec<Type19>,
inflate: Vec<u8>
}

struct Type45 {
sample_precision: u8,
num_lines: u16,
num_samples_per_line: u16,
num_image_components: u8,
image_components: Vec<Type44>
}

struct Type16 {
distance_extra_bits: u16,
distance: u16
}

struct Type21 {
code: u16,
extra: Type20
}

struct Type67 {
tag: (u8, u8, u8, u8),
length: u32,
data: Vec<u8>,
pad: Type66
}

struct Type73 {
header: Type72,
file: Vec<u8>,
__padding: ()
}

enum Type74 { ascii(Vec<u8>), utf8(Vec<char>) }

struct Type80 {
tag: (u8, u8, u8, u8),
length: u32,
data: Type68,
pad: Type66
}

struct Type44 {
id: u8,
sampling_factor: u8,
quantization_table_id: u8
}

struct Type70 {
string: Vec<u8>,
__nul_or_wsp: u8,
__padding: Vec<u8>
}

struct Type79 {
signature: (u8, u8, u8, u8, u8, u8, u8, u8),
ihdr: Type57,
chunks: Vec<Type63>,
idat: Vec<Type64>,
more_chunks: Vec<Type63>,
iend: Type65
}

struct Type81 {
contents: Vec<Type73>,
__padding: Vec<u8>,
__trailing: Vec<u8>
}

struct Type88 {
marker: Type28,
length: u16,
data: Type37
}

struct Type33 {
tag: u16,
r#type: u16,
length: u32,
offset_or_data: u32
}

struct Type89 {
marker: Type28,
length: u16,
data: Type53
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

struct Type92 {
marker: Type28,
length: u16,
data: Type40
}

struct Type93 {
marker: Type28,
length: u16,
data: Type41
}

struct Type94 {
marker: Type28,
length: u16,
data: Type42
}

struct Type51 {
scan_data: Vec<Type50>,
scan_data_stream: Vec<u8>
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

struct Type95 {
marker: Type28,
length: u16,
data: Vec<u8>
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

struct Type6 {
separator: u8,
image_left_position: u16,
image_top_position: u16,
image_width: u16,
image_height: u16,
flags: u8
}

struct Type24 {
r#final: u8,
r#type: u8,
data: Type23
}

struct Type13 {
magic: (u8, u8),
method: u8,
file_flags: u8,
timestamp: u32,
compression_flags: u8,
os_id: u8
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

struct Type57 {
length: u32,
tag: (u8, u8, u8, u8),
data: Type56,
crc: u32
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

struct Type91 {
marker: Type28,
length: u16,
data: Type39
}

struct Type99 {
align: (),
len: u16,
nlen: u16,
bytes: Vec<u8>,
codes_values: Vec<Type22>
}

struct Type84 {
length: u32,
tag: (u8, u8, u8, u8),
data: Vec<Type2>,
crc: u32
}

enum Type11 { graphic_block {
graphic_control_extension: Type5,
graphic_rendering_block: Type9
}, special_purpose_block(Type10) }

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

enum Type5 { some {
separator: u8,
label: u8,
block_size: u8,
flags: u8,
delay_time: u16,
transparent_color_index: u8,
terminator: u8
}, none() }

enum Type36 { other(Vec<u8>), xmp {
xmp: Vec<u8>
}, exif {
padding: u8,
exif: Type35
} }

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

struct Type52 {
segments: Vec<Type43>,
sos: Type49,
data: Type51
}

struct Type85 {
length: u32,
tag: (u8, u8, u8, u8),
data: Type60,
crc: u32
}

struct Type34 {
num_fields: u16,
fields: Vec<Type33>,
next_ifd_offset: u32,
next_ifd: Vec<u8>
}

struct Type0 {
signature: (u8, u8, u8),
version: Vec<u8>
}

struct Type86 {
length: u32,
tag: (u8, u8, u8, u8),
data: Type62,
crc: u32
}

struct Type18 {
code: u16,
extra: Type17
}

enum Type66 { no(u8), yes() }

struct Type90 {
marker: Type28,
length: u16,
data: Type45
}

struct Type15 {
code: u16,
extra: u8
}

struct Type78 {
soi: Type28,
frame: Type55,
eoi: Type28
}

struct Type82 {
length: u32,
tag: (u8, u8, u8, u8),
data: Type58,
crc: u32
}

struct Type104 {
separator: u8,
label: u8,
comment_data: Vec<Type7>,
terminator: u8
}

struct Type55 {
initial_segment: Type38,
segments: Vec<Type43>,
header: Type46,
scan: Type52,
dnl: Type54,
scans: Vec<Type52>
}

struct Type61 {
palette_index: u8
}

struct Type40 {
class_table_id: u8,
num_codes: Vec<u8>,
values: Vec<u8>
}

struct Type71 {
string: Vec<u8>,
padding: Vec<u8>
}

struct Type87 {
marker: Type28,
length: u16,
data: Type31
}

struct Type64 {
length: u32,
tag: (u8, u8, u8, u8),
data: Vec<u8>,
crc: u32
}

struct Type59 {
pixels_per_unit_x: u32,
pixels_per_unit_y: u32,
unit_specifier: u8
}

struct Type69 {
string: Vec<u8>,
__padding: Vec<u8>
}

struct Type97 {
xmp: Vec<u8>
}

struct Type102 {
graphic_control_extension: Type5,
graphic_rendering_block: Type9
}

struct Type28 {
ff: u8,
marker: u8
}

struct Type26 {
crc: u32,
length: u32
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

enum Type19 { literal(u8), reference {
length: u16,
distance: u16
} }

struct Type96 {
padding: u8,
exif: Type35
}

enum Type58 { color_type_ {
palette_index: u8
}, color_type_ {
red: u16,
green: u16,
blue: u16
}, color_type_ {
red: u16,
green: u16,
blue: u16
}, color_type_ {
greyscale: u16
}, color_type_ {
greyscale: u16
} }

struct Type2 {
r: u8,
g: u8,
b: u8
}

enum Type3 { no(), yes(Vec<Type2>) }

fn Decoder0<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type76> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder1<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type76> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder2<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type77> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder3<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Vec<Type27>> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder4<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type78> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder5<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type79> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder6<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type80> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder7<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type81> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder8<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type74> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder9<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Vec<u8>> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder10<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Vec<char>> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder11<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<char> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder12<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u8> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder13<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u8> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder14<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type73> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder15<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type72> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder16<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u8> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder17<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type69> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder18<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u8> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder19<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u8> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder20<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u8> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder21<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type69> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder22<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type71> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder23<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u32> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder24<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type68> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder25<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder26<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type67> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder27<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<(u8, u8, u8, u8, u8, u8, u8, u8)> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder28<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type57> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder29<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type63> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder30<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type64> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder31<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type65> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder32<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u32> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder33<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder34<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<()> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder35<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder36<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Vec<u8>> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder37<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type82> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder38<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type83> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder39<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type84> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder40<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type85> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder41<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type86> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder42<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u16> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder43<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder44<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type56> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder45<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type28> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder46<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type55> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder47<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type28> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder48<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type87> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder49<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type88> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder50<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type43> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder51<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type46> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder52<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type52> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder53<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type89> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder54<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type52> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder55<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type49> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder56<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type51> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder57<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u8> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder58<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type28> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder59<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type28> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder60<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type28> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder61<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type28> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder62<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type28> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder63<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type28> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder64<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type28> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder65<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type28> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder66<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type48> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder67<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type47> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder68<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type53> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder69<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type51> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder70<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder71<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder72<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder73<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder74<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder75<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder76<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder77<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder78<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder79<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder80<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder81<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder82<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type90> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder83<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type45> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder84<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type44> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder85<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type91> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder86<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type92> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder87<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type93> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder88<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type94> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder89<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder90<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder91<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder92<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder93<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder94<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder95<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder96<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder97<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder98<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder99<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder100<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder101<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder102<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder103<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type95> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder104<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type42> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder105<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type41> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder106<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type40> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder107<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type39> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder108<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type37> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder109<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type29> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder110<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type96> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder111<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type97> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder112<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type35> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder113<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u16> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder114<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type31> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder115<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type29> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder116<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type98> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder117<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type2> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder118<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type13> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder119<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type29> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder120<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type25> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder121<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type26> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder122<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type24> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder123<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u8> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder124<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type99> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder125<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type100> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder126<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type101> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder127<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type29> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder128<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type0> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder129<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type4> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder130<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type11> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder131<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type12> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder132<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type102> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder133<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type10> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder134<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type103> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder135<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type104> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder136<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type7> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder137<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<u8> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder138<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type105> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder139<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type9> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder140<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type106> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder141<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type107> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder142<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type6> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder143<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type2> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder144<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type8> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

fn Decoder145<'input>(scope: &mut Scope, input: ReadCtxt<'input>) -> Option<Type1> {
while input.offset % 512 != 0 {
let _ = input.read_byte();
}
return Some(());
}

