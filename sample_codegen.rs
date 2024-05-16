#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use doodle::prelude::*;

#[derive(Debug, Clone)]
struct Type0 {
    signature: (u8, u8, u8),
    version: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type1 {
    screen_width: u16,
    screen_height: u16,
    flags: u8,
    bg_color_index: u8,
    pixel_aspect_ratio: u8,
}

#[derive(Debug, Clone)]
struct Type2 {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone)]
enum Type3 {
    no,
    yes(Vec<Type2>),
}

#[derive(Debug, Clone)]
struct Type4 {
    descriptor: Type1,
    global_color_table: Type3,
}

#[derive(Debug, Clone)]
struct Type5 {
    separator: u8,
    label: u8,
    block_size: u8,
    flags: u8,
    delay_time: u16,
    transparent_color_index: u8,
    terminator: u8,
}

#[derive(Debug, Clone)]
enum Type6 {
    none,
    some(Type5),
}

#[derive(Debug, Clone)]
struct Type7 {
    len_bytes: u8,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type8 {
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
    terminator: u8,
}

#[derive(Debug, Clone)]
struct Type9 {
    separator: u8,
    image_left_position: u16,
    image_top_position: u16,
    image_width: u16,
    image_height: u16,
    flags: u8,
}

#[derive(Debug, Clone)]
struct Type10 {
    lzw_min_code_size: u8,
    image_data: Vec<Type7>,
    terminator: u8,
}

#[derive(Debug, Clone)]
struct Type11 {
    descriptor: Type9,
    local_color_table: Type3,
    data: Type10,
}

#[derive(Debug, Clone)]
enum Type12 {
    plain_text_extension(Type8),
    table_based_image(Type11),
}

#[derive(Debug, Clone)]
struct Type13 {
    graphic_control_extension: Type6,
    graphic_rendering_block: Type12,
}

#[derive(Debug, Clone)]
struct Type14 {
    separator: u8,
    label: u8,
    block_size: u8,
    identifier: Vec<u8>,
    authentication_code: Vec<u8>,
    application_data: Vec<Type7>,
    terminator: u8,
}

#[derive(Debug, Clone)]
struct Type15 {
    separator: u8,
    label: u8,
    comment_data: Vec<Type7>,
    terminator: u8,
}

#[derive(Debug, Clone)]
enum Type16 {
    application_extension(Type14),
    comment_extension(Type15),
}

#[derive(Debug, Clone)]
enum Type17 {
    graphic_block(Type13),
    special_purpose_block(Type16),
}

#[derive(Debug, Clone)]
struct Type18 {
    separator: u8,
}

#[derive(Debug, Clone)]
struct Type19 {
    header: Type0,
    logical_screen: Type4,
    blocks: Vec<Type17>,
    trailer: Type18,
}

#[derive(Debug, Clone)]
struct Type20 {
    magic: (u8, u8),
    method: u8,
    file_flags: u8,
    timestamp: u32,
    compression_flags: u8,
    os_id: u8,
}

#[derive(Debug, Clone)]
struct Type21 {
    string: Vec<u8>,
    null: u8,
}

#[derive(Debug, Clone)]
enum Type22 {
    no,
    yes(Type21),
}

#[derive(Debug, Clone)]
struct Type23 {
    code: u16,
    extra: u8,
}

#[derive(Debug, Clone)]
struct Type24 {
    distance_extra_bits: u16,
    distance: u16,
}

#[derive(Debug, Clone)]
struct Type25 {
    length_extra_bits: u8,
    length: u16,
    distance_code: u16,
    distance_record: Type24,
}

#[derive(Debug, Clone)]
enum Type26 {
    none,
    some(Type25),
}

#[derive(Debug, Clone)]
struct Type27 {
    code: u16,
    extra: Type26,
}

#[derive(Debug, Clone)]
struct Type28 {
    length: u16,
    distance: u16,
}

#[derive(Debug, Clone)]
enum Type29 {
    literal(u8),
    reference(Type28),
}

#[derive(Debug, Clone)]
struct Type30 {
    hlit: u8,
    hdist: u8,
    hclen: u8,
    code_length_alphabet_code_lengths: Vec<u8>,
    literal_length_distance_alphabet_code_lengths: Vec<Type23>,
    literal_length_distance_alphabet_code_lengths_value: Vec<u8>,
    literal_length_alphabet_code_lengths_value: Vec<u8>,
    distance_alphabet_code_lengths_value: Vec<u8>,
    codes: Vec<Type27>,
    codes_values: Vec<Type29>,
}

#[derive(Debug, Clone)]
struct Type31 {
    length_extra_bits: u8,
    length: u16,
    distance_code: u8,
    distance_record: Type24,
}

#[derive(Debug, Clone)]
enum Type32 {
    none,
    some(Type31),
}

#[derive(Debug, Clone)]
struct Type33 {
    code: u16,
    extra: Type32,
}

#[derive(Debug, Clone)]
struct Type34 {
    codes: Vec<Type33>,
    codes_values: Vec<Type29>,
}

#[derive(Debug, Clone)]
struct Type35 {
    align: (),
    len: u16,
    nlen: u16,
    bytes: Vec<u8>,
    codes_values: Vec<Type29>,
}

#[derive(Debug, Clone)]
enum Type36 {
    dynamic_huffman(Type30),
    fixed_huffman(Type34),
    uncompressed(Type35),
}

#[derive(Debug, Clone)]
struct Type37 {
    r#final: u8,
    r#type: u8,
    data: Type36,
}

#[derive(Debug, Clone)]
struct Type38 {
    blocks: Vec<Type37>,
    codes: Vec<Type29>,
    inflate: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type39 {
    crc: u32,
    length: u32,
}

#[derive(Debug, Clone)]
struct Type40 {
    header: Type20,
    fname: Type22,
    data: Type38,
    footer: Type39,
}

#[derive(Debug, Clone)]
struct Type41 {
    ff: u8,
    marker: u8,
}

#[derive(Debug, Clone)]
struct Type42 {
    version_major: u8,
    version_minor: u8,
    density_units: u8,
    density_x: u16,
    density_y: u16,
    thumbnail_width: u8,
    thumbnail_height: u8,
    thumbnail_pixels: Vec<Vec<Type2>>,
}

#[derive(Debug, Clone)]
enum Type43 {
    jfif(Type42),
    other(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type44 {
    identifier: Type21,
    data: Type43,
}

#[derive(Debug, Clone)]
struct Type45 {
    marker: Type41,
    length: u16,
    data: Type44,
}

#[derive(Debug, Clone)]
enum Type46 {
    be(u8, u8),
    le(u8, u8),
}

#[derive(Debug, Clone)]
struct Type47 {
    tag: u16,
    r#type: u16,
    length: u32,
    offset_or_data: u32,
}

#[derive(Debug, Clone)]
struct Type48 {
    num_fields: u16,
    fields: Vec<Type47>,
    next_ifd_offset: u32,
    next_ifd: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type49 {
    byte_order: Type46,
    magic: u16,
    offset: u32,
    ifd: Type48,
}

#[derive(Debug, Clone)]
struct Type50 {
    padding: u8,
    exif: Type49,
}

#[derive(Debug, Clone)]
struct Type51 {
    xmp: Vec<u8>,
}

#[derive(Debug, Clone)]
enum Type52 {
    exif(Type50),
    other(Vec<u8>),
    xmp(Type51),
}

#[derive(Debug, Clone)]
struct Type53 {
    identifier: Type21,
    data: Type52,
}

#[derive(Debug, Clone)]
struct Type54 {
    marker: Type41,
    length: u16,
    data: Type53,
}

#[derive(Debug, Clone)]
enum Type55 {
    app0(Type45),
    app1(Type54),
}

#[derive(Debug, Clone)]
struct Type56 {
    marker: Type41,
    length: u16,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type57 {
    class_table_id: u8,
    value: u8,
}

#[derive(Debug, Clone)]
struct Type58 {
    marker: Type41,
    length: u16,
    data: Type57,
}

#[derive(Debug, Clone)]
struct Type59 {
    class_table_id: u8,
    num_codes: Vec<u8>,
    values: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type60 {
    marker: Type41,
    length: u16,
    data: Type59,
}

#[derive(Debug, Clone)]
struct Type61 {
    precision_table_id: u8,
    elements: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type62 {
    marker: Type41,
    length: u16,
    data: Type61,
}

#[derive(Debug, Clone)]
struct Type63 {
    restart_interval: u16,
}

#[derive(Debug, Clone)]
struct Type64 {
    marker: Type41,
    length: u16,
    data: Type63,
}

#[derive(Debug, Clone)]
enum Type65 {
    app0(Type45),
    app1(Type54),
    app10(Type56),
    app11(Type56),
    app12(Type56),
    app13(Type56),
    app14(Type56),
    app15(Type56),
    app2(Type56),
    app3(Type56),
    app4(Type56),
    app5(Type56),
    app6(Type56),
    app7(Type56),
    app8(Type56),
    app9(Type56),
    com(Type56),
    dac(Type58),
    dht(Type60),
    dqt(Type62),
    dri(Type64),
}

#[derive(Debug, Clone)]
struct Type66 {
    id: u8,
    sampling_factor: u8,
    quantization_table_id: u8,
}

#[derive(Debug, Clone)]
struct Type67 {
    sample_precision: u8,
    num_lines: u16,
    num_samples_per_line: u16,
    num_image_components: u8,
    image_components: Vec<Type66>,
}

#[derive(Debug, Clone)]
struct Type68 {
    marker: Type41,
    length: u16,
    data: Type67,
}

#[derive(Debug, Clone)]
enum Type69 {
    sof0(Type68),
    sof1(Type68),
    sof10(Type68),
    sof11(Type68),
    sof13(Type68),
    sof14(Type68),
    sof15(Type68),
    sof2(Type68),
    sof3(Type68),
    sof5(Type68),
    sof6(Type68),
    sof7(Type68),
    sof9(Type68),
}

#[derive(Debug, Clone)]
struct Type70 {
    component_selector: u8,
    entropy_coding_table_ids: u8,
}

#[derive(Debug, Clone)]
struct Type71 {
    num_image_components: u8,
    image_components: Vec<Type70>,
    start_spectral_selection: u8,
    end_spectral_selection: u8,
    approximation_bit_position: u8,
}

#[derive(Debug, Clone)]
struct Type72 {
    marker: Type41,
    length: u16,
    data: Type71,
}

#[derive(Debug, Clone)]
enum Type73 {
    mcu(u8),
    rst0(Type41),
    rst1(Type41),
    rst2(Type41),
    rst3(Type41),
    rst4(Type41),
    rst5(Type41),
    rst6(Type41),
    rst7(Type41),
}

#[derive(Debug, Clone)]
struct Type74 {
    scan_data: Vec<Type73>,
    scan_data_stream: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type75 {
    segments: Vec<Type65>,
    sos: Type72,
    data: Type74,
}

#[derive(Debug, Clone)]
struct Type76 {
    num_lines: u16,
}

#[derive(Debug, Clone)]
struct Type77 {
    marker: Type41,
    length: u16,
    data: Type76,
}

#[derive(Debug, Clone)]
enum Type78 {
    none,
    some(Type77),
}

#[derive(Debug, Clone)]
struct Type79 {
    initial_segment: Type55,
    segments: Vec<Type65>,
    header: Type69,
    scan: Type75,
    dnl: Type78,
    scans: Vec<Type75>,
}

#[derive(Debug, Clone)]
struct Type80 {
    soi: Type41,
    frame: Type79,
    eoi: Type41,
}

#[derive(Debug, Clone)]
struct Type81 {
    major_brand: (u8, u8, u8, u8),
    minor_version: u32,
    compatible_brands: Vec<(u8, u8, u8, u8)>,
}

#[derive(Debug, Clone)]
struct Type82 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type83 {
    version: u8,
    flags: (u8, u8, u8),
    number_of_entries: u32,
    data: Vec<Type82>,
}

#[derive(Debug, Clone)]
enum Type84 {
    dref(Type83),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type85 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type84,
}

#[derive(Debug, Clone)]
struct Type86 {
    version: u8,
    flags: (u8, u8, u8),
    predefined: u32,
    handler_type: (u8, u8, u8, u8),
    reserved: (u32, u32, u32),
    name: Type21,
}

#[derive(Debug, Clone)]
struct Type87 {
    content_type: Type21,
}

#[derive(Debug, Clone)]
struct Type88 {
    item_uri_type: Type21,
}

#[derive(Debug, Clone)]
enum Type89 {
    mime(Type87),
    unknown,
    uri(Type88),
}

#[derive(Debug, Clone)]
struct Type90 {
    item_ID: u32,
    item_protection_index: u16,
    item_type: (u8, u8, u8, u8),
    item_name: Type21,
    extra_fields: Type89,
}

#[derive(Debug, Clone)]
struct Type91 {
    item_ID: u16,
    item_protection_index: u16,
    item_name: Type21,
    content_type: Type21,
    content_encoding: Type21,
}

#[derive(Debug, Clone)]
enum Type92 {
    no(Type90),
    yes(Type91),
}

#[derive(Debug, Clone)]
struct Type93 {
    version: u8,
    flags: (u8, u8, u8),
    fields: Type92,
}

#[derive(Debug, Clone)]
enum Type94 {
    infe(Type93),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type95 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type94,
}

#[derive(Debug, Clone)]
struct Type96 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    item_info_entry: Vec<Type95>,
}

#[derive(Debug, Clone)]
enum Type97 {
    no,
    yes(u16),
}

#[derive(Debug, Clone)]
struct Type98 {
    extent_index: u64,
    extent_offset: u64,
    extent_length: u64,
}

#[derive(Debug, Clone)]
struct Type99 {
    item_ID: u32,
    construction_method: Type97,
    data_reference_index: u16,
    base_offset: u64,
    extent_count: u16,
    extents: Vec<Type98>,
}

#[derive(Debug, Clone)]
struct Type100 {
    version: u8,
    flags: (u8, u8, u8),
    offset_size_length_size: u8,
    base_offset_size_index_size: u8,
    offset_size: u8,
    length_size: u8,
    base_offset_size: u8,
    index_size: u8,
    item_count: u32,
    items: Vec<Type99>,
}

#[derive(Debug, Clone)]
struct Type101 {
    type_indicator: u32,
    locale_indicator: u32,
    value: Vec<u8>,
}

#[derive(Debug, Clone)]
enum Type102 {
    data(Type101),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type103 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type102,
}

#[derive(Debug, Clone)]
enum Type104 {
    tool(Vec<Type103>),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type105 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type104,
}

#[derive(Debug, Clone)]
struct Type106 {
    from_item_ID: u32,
    reference_count: u16,
    to_item_ID: Vec<u32>,
}

#[derive(Debug, Clone)]
struct Type107 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type106,
}

#[derive(Debug, Clone)]
struct Type108 {
    from_item_ID: u16,
    reference_count: u16,
    to_item_ID: Vec<u16>,
}

#[derive(Debug, Clone)]
struct Type109 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type108,
}

#[derive(Debug, Clone)]
enum Type110 {
    large(Vec<Type107>),
    small(Vec<Type109>),
}

#[derive(Debug, Clone)]
struct Type111 {
    version: u8,
    flags: (u8, u8, u8),
    single_item_reference: Type110,
}

#[derive(Debug, Clone)]
enum Type112 {
    no(u32),
    yes(u16),
}

#[derive(Debug, Clone)]
struct Type113 {
    version: u8,
    flags: (u8, u8, u8),
    item_ID: Type112,
}

#[derive(Debug, Clone)]
enum Type114 {
    dinf(Vec<Type85>),
    hdlr(Type86),
    idat(Vec<u8>),
    iinf(Type96),
    iloc(Type100),
    ilst(Vec<Type105>),
    iref(Type111),
    pitm(Type113),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type115 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type114,
}

#[derive(Debug, Clone)]
struct Type116 {
    creation_time: u32,
    modification_time: u32,
    timescale: u32,
    duration: u32,
}

#[derive(Debug, Clone)]
struct Type117 {
    creation_time: u64,
    modification_time: u64,
    timescale: u32,
    duration: u64,
}

#[derive(Debug, Clone)]
enum Type118 {
    version0(Type116),
    version1(Type117),
}

#[derive(Debug, Clone)]
struct Type119 {
    version: u8,
    flags: (u8, u8, u8),
    fields: Type118,
    rate: u32,
    volume: u16,
    reserved1: u16,
    reserved2: (u32, u32),
    matrix: Vec<u32>,
    pre_defined: Vec<u32>,
    next_track_ID: u32,
}

#[derive(Debug, Clone)]
struct Type120 {
    track_duration: u32,
    media_time: u32,
    media_rate: u32,
}

#[derive(Debug, Clone)]
struct Type121 {
    version: u8,
    flags: (u8, u8, u8),
    number_of_entries: u32,
    edit_list_table: Vec<Type120>,
}

#[derive(Debug, Clone)]
enum Type122 {
    elst(Type121),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type123 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type122,
}

#[derive(Debug, Clone)]
struct Type124 {
    version: u8,
    flags: (u8, u8, u8),
    component_type: u32,
    component_subtype: (u8, u8, u8, u8),
    component_manufacturer: u32,
    component_flags: u32,
    component_flags_mask: u32,
    component_name: Type21,
}

#[derive(Debug, Clone)]
struct Type125 {
    version: u8,
    flags: (u8, u8, u8),
    fields: Type118,
    language: u16,
    pre_defined: u16,
}

#[derive(Debug, Clone)]
struct Type126 {
    version: u8,
    flags: (u8, u8, u8),
    balance: u16,
    reserved: u16,
}

#[derive(Debug, Clone)]
struct Type127 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    chunk_offset: Vec<u64>,
}

#[derive(Debug, Clone)]
struct Type128 {
    sample_count: u32,
    sample_offset: u32,
}

#[derive(Debug, Clone)]
struct Type129 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    sample_entries: Vec<Type128>,
}

#[derive(Debug, Clone)]
enum Type130 {
    no,
    yes(u32),
}

#[derive(Debug, Clone)]
struct Type131 {
    sample_count: u32,
    group_description_index: u32,
}

#[derive(Debug, Clone)]
struct Type132 {
    version: u8,
    flags: (u8, u8, u8),
    grouping_type: u32,
    grouping_type_parameter: Type130,
    entry_count: u32,
    sample_groups: Vec<Type131>,
}

#[derive(Debug, Clone)]
struct Type133 {
    description_length: u32,
    sample_group_entry: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type134 {
    version: u8,
    flags: (u8, u8, u8),
    grouping_type: u32,
    default_length: u32,
    entry_count: u32,
    sample_groups: Vec<Type133>,
}

#[derive(Debug, Clone)]
struct Type135 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    chunk_offset: Vec<u32>,
}

#[derive(Debug, Clone)]
struct Type136 {
    first_chunk: u32,
    samples_per_chunk: u32,
    sample_description_index: u32,
}

#[derive(Debug, Clone)]
struct Type137 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    chunk_entries: Vec<Type136>,
}

#[derive(Debug, Clone)]
struct Type138 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    sample_entries: Vec<Type82>,
}

#[derive(Debug, Clone)]
struct Type139 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    sample_number: Vec<u32>,
}

#[derive(Debug, Clone)]
enum Type140 {
    no,
    yes(Vec<u32>),
}

#[derive(Debug, Clone)]
struct Type141 {
    version: u8,
    flags: (u8, u8, u8),
    sample_size: u32,
    sample_count: u32,
    entry_size: Type140,
}

#[derive(Debug, Clone)]
struct Type142 {
    sample_count: u32,
    sample_delta: u32,
}

#[derive(Debug, Clone)]
struct Type143 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    sample_entries: Vec<Type142>,
}

#[derive(Debug, Clone)]
enum Type144 {
    co64(Type127),
    ctts(Type129),
    sbgp(Type132),
    sgpd(Type134),
    stco(Type135),
    stsc(Type137),
    stsd(Type138),
    stss(Type139),
    stsz(Type141),
    stts(Type143),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type145 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type144,
}

#[derive(Debug, Clone)]
struct Type146 {
    version: u8,
    flags: (u8, u8, u8),
    graphicsmode: u16,
    opcolor: Vec<u16>,
}

#[derive(Debug, Clone)]
enum Type147 {
    dinf(Vec<Type85>),
    smhd(Type126),
    stbl(Vec<Type145>),
    unknown(Vec<u8>),
    vmhd(Type146),
}

#[derive(Debug, Clone)]
struct Type148 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type147,
}

#[derive(Debug, Clone)]
enum Type149 {
    hdlr(Type124),
    mdhd(Type125),
    minf(Vec<Type148>),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type150 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type149,
}

#[derive(Debug, Clone)]
struct Type151 {
    creation_time: u32,
    modification_time: u32,
    track_ID: u32,
    reserved: u32,
    duration: u32,
}

#[derive(Debug, Clone)]
struct Type152 {
    creation_time: u64,
    modification_time: u64,
    track_ID: u32,
    reserved: u32,
    duration: u64,
}

#[derive(Debug, Clone)]
enum Type153 {
    version0(Type151),
    version1(Type152),
}

#[derive(Debug, Clone)]
struct Type154 {
    version: u8,
    flags: (u8, u8, u8),
    fields: Type153,
    reserved2: (u32, u32),
    layer: u16,
    alternate_group: u16,
    volume: u16,
    reserved1: u16,
    matrix: Vec<u32>,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone)]
enum Type155 {
    edts(Vec<Type123>),
    mdia(Vec<Type150>),
    tkhd(Type154),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type156 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type155,
}

#[derive(Debug, Clone)]
enum Type157 {
    meta(u32, Vec<Type115>),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type158 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type157,
}

#[derive(Debug, Clone)]
enum Type159 {
    mvhd(Type119),
    trak(Vec<Type156>),
    udta(Vec<Type158>),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type160 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type159,
}

#[derive(Debug, Clone)]
enum Type161 {
    free,
    ftyp(Type81),
    mdat,
    meta(u32, Vec<Type115>),
    moov(Vec<Type160>),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type162 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type161,
}

#[derive(Debug, Clone)]
struct Type163 {
    atoms: Vec<Type162>,
}

#[derive(Debug, Clone)]
struct Type164 {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

#[derive(Debug, Clone)]
struct Type165 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type164,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type166 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Vec<Type2>,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type167 {
    greyscale: u16,
}

#[derive(Debug, Clone)]
struct Type168 {
    red: u16,
    green: u16,
    blue: u16,
}

#[derive(Debug, Clone)]
struct Type169 {
    palette_index: u8,
}

#[derive(Debug, Clone)]
enum Type170 {
    color_type_0(Type167),
    color_type_2(Type168),
    color_type_3(Type169),
    color_type_4(Type167),
    color_type_6(Type168),
}

#[derive(Debug, Clone)]
struct Type171 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type170,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type172 {
    whitepoint_x: u32,
    whitepoint_y: u32,
    red_x: u32,
    red_y: u32,
    green_x: u32,
    green_y: u32,
    blue_x: u32,
    blue_y: u32,
}

#[derive(Debug, Clone)]
struct Type173 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type172,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type174 {
    gamma: u32,
}

#[derive(Debug, Clone)]
struct Type175 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type174,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type176 {
    pixels_per_unit_x: u32,
    pixels_per_unit_y: u32,
    unit_specifier: u8,
}

#[derive(Debug, Clone)]
struct Type177 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type176,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type178 {
    keyword: Type21,
    text: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type179 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type178,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type180 {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

#[derive(Debug, Clone)]
struct Type181 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type180,
    crc: u32,
}

#[derive(Debug, Clone)]
enum Type182 {
    color_type_0(Type167),
    color_type_2(Type168),
    color_type_3(Vec<Type169>),
}

#[derive(Debug, Clone)]
struct Type183 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type182,
    crc: u32,
}

#[derive(Debug, Clone)]
enum Type184 {
    PLTE(Type166),
    bKGD(Type171),
    cHRM(Type173),
    gAMA(Type175),
    pHYs(Type177),
    tEXt(Type179),
    tIME(Type181),
    tRNS(Type183),
}

#[derive(Debug, Clone)]
struct Type185 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Vec<u8>,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type186 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: (),
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type187 {
    signature: (u8, u8, u8, u8, u8, u8, u8, u8),
    ihdr: Type165,
    chunks: Vec<Type184>,
    idat: Vec<Type185>,
    more_chunks: Vec<Type184>,
    iend: Type186,
}

#[derive(Debug, Clone)]
enum Type188 {
    no(u8),
    yes,
}

#[derive(Debug, Clone)]
struct Type189 {
    tag: (u8, u8, u8, u8),
    length: u32,
    data: Vec<u8>,
    pad: Type188,
}

#[derive(Debug, Clone)]
struct Type190 {
    tag: (u8, u8, u8, u8),
    chunks: Vec<Type189>,
}

#[derive(Debug, Clone)]
struct Type191 {
    tag: (u8, u8, u8, u8),
    length: u32,
    data: Type190,
    pad: Type188,
}

#[derive(Debug, Clone)]
struct Type192 {
    string: Vec<u8>,
    __padding: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type193 {
    string: Vec<u8>,
    __nul_or_wsp: u8,
    __padding: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type194 {
    string: Vec<u8>,
    padding: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type195 {
    name: Type192,
    mode: Type193,
    uid: Type193,
    gid: Type193,
    size: u32,
    mtime: Type193,
    chksum: Type193,
    typeflag: u8,
    linkname: Type192,
    magic: (u8, u8, u8, u8, u8, u8),
    version: (u8, u8),
    uname: Type194,
    gname: Type194,
    devmajor: Type193,
    devminor: Type193,
    prefix: Type192,
    pad: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type196 {
    header: Type195,
    file: Vec<u8>,
    __padding: (),
}

#[derive(Debug, Clone)]
struct Type197 {
    contents: Vec<Type196>,
    __padding: Vec<u8>,
    __trailing: Vec<u8>,
}

#[derive(Debug, Clone)]
enum Type198 {
    gif(Type19),
    gzip(Vec<Type40>),
    jpeg(Type80),
    mpeg4(Type163),
    png(Type187),
    riff(Type191),
    tar(Type197),
    text(Vec<char>),
}

#[derive(Debug, Clone)]
enum Type199 {
    none,
    some(u8),
}

#[derive(Debug, Clone)]
struct Type200 {
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
    value: u32,
}

#[derive(Debug, Clone)]
struct Type201 {
    data: Type198,
    end: (),
}

fn Decoder0<'input>(_input: &mut Parser<'input>) -> Result<Type201, ParseError> {
    PResult::Ok((Decoder1(_input))?)
}

fn Decoder1<'input>(_input: &mut Parser<'input>) -> Result<Type201, ParseError> {
    let data = ((|| {
        _input.start_alt();
        {
            let mut f_tmp = || {
                PResult::Ok({
                    let inner = (Decoder2(_input))?;
                    Type198::gif(inner)
                })
            };
            match f_tmp() {
                Ok(inner) => {
                    return PResult::Ok(inner);
                }

                Err(_e) => {
                    _input.next_alt(false)?;
                }
            }
        };
        {
            let mut f_tmp = || {
                PResult::Ok({
                    let inner = (Decoder3(_input))?;
                    Type198::gzip(inner)
                })
            };
            match f_tmp() {
                Ok(inner) => {
                    return PResult::Ok(inner);
                }

                Err(_e) => {
                    _input.next_alt(false)?;
                }
            }
        };
        {
            let mut f_tmp = || {
                PResult::Ok({
                    let inner = (Decoder4(_input))?;
                    Type198::jpeg(inner)
                })
            };
            match f_tmp() {
                Ok(inner) => {
                    return PResult::Ok(inner);
                }

                Err(_e) => {
                    _input.next_alt(false)?;
                }
            }
        };
        {
            let mut f_tmp = || {
                PResult::Ok({
                    let inner = (Decoder5(_input))?;
                    Type198::mpeg4(inner)
                })
            };
            match f_tmp() {
                Ok(inner) => {
                    return PResult::Ok(inner);
                }

                Err(_e) => {
                    _input.next_alt(false)?;
                }
            }
        };
        {
            let mut f_tmp = || {
                PResult::Ok({
                    let inner = (Decoder6(_input))?;
                    Type198::png(inner)
                })
            };
            match f_tmp() {
                Ok(inner) => {
                    return PResult::Ok(inner);
                }

                Err(_e) => {
                    _input.next_alt(false)?;
                }
            }
        };
        {
            let mut f_tmp = || {
                PResult::Ok({
                    let inner = (Decoder7(_input))?;
                    Type198::riff(inner)
                })
            };
            match f_tmp() {
                Ok(inner) => {
                    return PResult::Ok(inner);
                }

                Err(_e) => {
                    _input.next_alt(false)?;
                }
            }
        };
        {
            let mut f_tmp = || {
                PResult::Ok({
                    let inner = (Decoder8(_input))?;
                    Type198::tar(inner)
                })
            };
            match f_tmp() {
                Ok(inner) => {
                    return PResult::Ok(inner);
                }

                Err(_e) => {
                    _input.next_alt(true)?;
                }
            }
        };
        {
            let mut f_tmp = || {
                PResult::Ok({
                    let inner = (Decoder9(_input))?;
                    Type198::text(inner)
                })
            };
            match f_tmp() {
                Ok(inner) => {
                    return PResult::Ok(inner);
                }

                Err(_e) => {
                    return Err(_e);
                }
            }
        };
    })())?;
    let end = ((|| PResult::Ok(_input.finish()?))())?;
    PResult::Ok(Type201 { data, end })
}

fn Decoder2<'input>(_input: &mut Parser<'input>) -> Result<Type19, ParseError> {
    let header = ((|| PResult::Ok((Decoder154(_input))?))())?;
    let logical_screen = ((|| PResult::Ok((Decoder155(_input))?))())?;
    let blocks = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            33u8 => 0,

                            44u8 => 0,

                            59u8 => 1,

                            _ => {
                                return Err(ParseError::ExcludedBranch(18325384555431379809u64));
                            }
                        };
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder156(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let trailer = ((|| PResult::Ok((Decoder157(_input))?))())?;
    PResult::Ok(Type19 {
        header,
        logical_screen,
        blocks,
        trailer,
    })
}

fn Decoder3<'input>(_input: &mut Parser<'input>) -> Result<Vec<Type40>, ParseError> {
    let mut accum = Vec::new();
    while _input.remaining() > 0 {
        let matching_ix = {
            _input.open_peek_context();
            let b = _input.read_byte()?;
            {
                let ret = if b == 31 { 1 } else { 0 };
                _input.close_peek_context()?;
                ret
            }
        };
        if matching_ix == 0 {
            if accum.is_empty() {
                return Err(ParseError::InsufficientRepeats);
            } else {
                break;
            }
        } else {
            let next_elem = {
                let header = ((|| PResult::Ok((Decoder142(_input))?))())?;
                let fname = ((|| {
                    PResult::Ok(match header.clone().file_flags & 8u8 != 0u8 {
                        true => {
                            let inner = (Decoder143(_input))?;
                            Type22::yes(inner)
                        }

                        false => {
                            let _ = ();
                            Type22::no
                        }
                    })
                })())?;
                let data = ((|| {
                    PResult::Ok({
                        _input.enter_bits_mode()?;
                        let ret = ((|| PResult::Ok((Decoder144(_input))?))())?;
                        let _bits_read = _input.escape_bits_mode()?;
                        ret
                    })
                })())?;
                let footer = ((|| PResult::Ok((Decoder145(_input))?))())?;
                Type40 {
                    header,
                    fname,
                    data,
                    footer,
                }
            };
            accum.push(next_elem);
        }
    }
    PResult::Ok(accum)
}

fn Decoder4<'input>(_input: &mut Parser<'input>) -> Result<Type80, ParseError> {
    let soi = ((|| PResult::Ok((Decoder69(_input))?))())?;
    let frame = ((|| PResult::Ok((Decoder70(_input))?))())?;
    let eoi = ((|| PResult::Ok((Decoder71(_input))?))())?;
    PResult::Ok(Type80 { soi, frame, eoi })
}

fn Decoder5<'input>(_input: &mut Parser<'input>) -> Result<Type163, ParseError> {
    let atoms = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    _input.read_byte()?;
                    {
                        let ret = 0;
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder48(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    PResult::Ok(Type163 { atoms })
}

fn Decoder6<'input>(_input: &mut Parser<'input>) -> Result<Type187, ParseError> {
    let signature = ((|| PResult::Ok((Decoder26(_input))?))())?;
    let ihdr = ((|| PResult::Ok((Decoder27(_input))?))())?;
    let chunks = ((|| {
        PResult::Ok({
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
                            98u8 => 0,

                            99u8 => 0,

                            103u8 => 0,

                            112u8 => 0,

                            80u8 => 0,

                            116u8 => 0,

                            73u8 => 1,

                            _ => {
                                return Err(ParseError::ExcludedBranch(5125128728024664750u64));
                            }
                        };
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder28(_input, ihdr.clone()))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let idat = ((|| {
        PResult::Ok({
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
                                    69u8 => 0,

                                    68u8 => 1,

                                    _ => {
                                        return Err(ParseError::ExcludedBranch(
                                            8806068124070768035u64,
                                        ));
                                    }
                                }
                            }

                            98u8 => 0,

                            99u8 => 0,

                            103u8 => 0,

                            112u8 => 0,

                            80u8 => 0,

                            116u8 => 0,

                            _ => {
                                return Err(ParseError::ExcludedBranch(2367749018041928916u64));
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
                        break;
                    }
                } else {
                    let next_elem = (Decoder29(_input))?;
                    accum.push(next_elem);
                }
            }
            accum
        })
    })())?;
    let more_chunks = ((|| {
        PResult::Ok({
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
                            98u8 => 0,

                            99u8 => 0,

                            103u8 => 0,

                            112u8 => 0,

                            80u8 => 0,

                            116u8 => 0,

                            73u8 => 1,

                            _ => {
                                return Err(ParseError::ExcludedBranch(5125128728024664750u64));
                            }
                        };
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder28(_input, ihdr.clone()))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let iend = ((|| PResult::Ok((Decoder30(_input))?))())?;
    PResult::Ok(Type187 {
        signature,
        ihdr,
        chunks,
        idat,
        more_chunks,
        iend,
    })
}

fn Decoder7<'input>(_input: &mut Parser<'input>) -> Result<Type191, ParseError> {
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 82 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(4610689655322527862u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 73 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(17197161005512507961u64));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 70 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(14049552398800766371u64));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 70 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(14049552398800766371u64));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder22(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder23(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let pad = ((|| {
        PResult::Ok(match length % 2u32 == 0u32 {
            true => {
                let _ = ();
                Type188::yes
            }

            false => {
                let inner = {
                    let b = _input.read_byte()?;
                    if b == 0 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10396965092922267801u64));
                    }
                };
                Type188::no(inner)
            }
        })
    })())?;
    PResult::Ok(Type191 {
        tag,
        length,
        data,
        pad,
    })
}

fn Decoder8<'input>(_input: &mut Parser<'input>) -> Result<Type197, ParseError> {
    let contents = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            0u8 => 0,

                            tmp if (tmp != 0) => 1,

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
                        break;
                    }
                } else {
                    let next_elem = (Decoder13(_input))?;
                    accum.push(next_elem);
                }
            }
            accum
        })
    })())?;
    let __padding = ((|| {
        PResult::Ok({
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
        })
    })())?;
    let __trailing = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = if b == 0 { 0 } else { 1 };
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
                    break;
                }
            }
            accum
        })
    })())?;
    PResult::Ok(Type197 {
        contents,
        __padding,
        __trailing,
    })
}

fn Decoder9<'input>(_input: &mut Parser<'input>) -> Result<Vec<char>, ParseError> {
    PResult::Ok((Decoder10(_input))?)
}

fn Decoder10<'input>(_input: &mut Parser<'input>) -> Result<Vec<char>, ParseError> {
    let mut accum = Vec::new();
    while _input.remaining() > 0 {
        let matching_ix = {
            _input.open_peek_context();
            let b = _input.read_byte()?;
            {
                let ret = match b {
                    tmp if ((ByteSet::from_bits([
                        18446744073709551615,
                        18446744073709551615,
                        0,
                        0,
                    ]))
                    .contains(tmp)) =>
                    {
                        0
                    }

                    tmp if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(tmp)) => 0,

                    224u8 => 0,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => 0,

                    237u8 => 0,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => 0,

                    240u8 => 0,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => 0,

                    244u8 => 0,

                    _ => {
                        return Err(ParseError::ExcludedBranch(8259468294382519899u64));
                    }
                };
                _input.close_peek_context()?;
                ret
            }
        };
        if matching_ix == 0 {
            let next_elem = (Decoder11(_input))?;
            accum.push(next_elem);
        } else {
            break;
        }
    }
    PResult::Ok(accum)
}

fn Decoder11<'input>(_input: &mut Parser<'input>) -> Result<char, ParseError> {
    let inner = {
        let tree_index = {
            _input.open_peek_context();
            let b = _input.read_byte()?;
            {
                let ret = match b {
                    tmp if ((ByteSet::from_bits([
                        18446744073709551615,
                        18446744073709551615,
                        0,
                        0,
                    ]))
                    .contains(tmp)) =>
                    {
                        0
                    }

                    tmp if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(tmp)) => 1,

                    224u8 => 2,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => 2,

                    237u8 => 2,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => 2,

                    240u8 => 3,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => 3,

                    244u8 => 3,

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
                    if (ByteSet::from_bits([18446744073709551615, 18446744073709551615, 0, 0]))
                        .contains(b)
                    {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(11920168927252633217u64));
                    }
                };
                ((|byte: u8| PResult::Ok(byte as u32))(inner))?
            }

            1 => {
                let inner = {
                    let field0 = ((|| {
                        PResult::Ok({
                            let inner = {
                                let b = _input.read_byte()?;
                                if (ByteSet::from_bits([0, 0, 0, 4294967292])).contains(b) {
                                    b
                                } else {
                                    return Err(ParseError::ExcludedBranch(
                                        17624589492623733874u64,
                                    ));
                                }
                            };
                            ((|raw: u8| PResult::Ok(raw & 31u8))(inner))?
                        })
                    })())?;
                    let field1 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                    (field0, field1)
                };
                ((|bytes: (u8, u8)| {
                    PResult::Ok(match bytes {
                        (x1, x0) => (x1 as u32) << 6u32 | (x0 as u32),
                    })
                })(inner))?
            }

            2 => {
                let inner = {
                    let tree_index = {
                        _input.open_peek_context();
                        let b = _input.read_byte()?;
                        {
                            let ret = match b {
                                224u8 => 0,

                                tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240]))
                                    .contains(tmp)) =>
                                {
                                    1
                                }

                                237u8 => 2,

                                tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992]))
                                    .contains(tmp)) =>
                                {
                                    3
                                }

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
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if b == 224 {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                5346911683359312959u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 15u8))(inner))?
                                })
                            })())?;
                            let field1 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if (ByteSet::from_bits([0, 0, 18446744069414584320, 0]))
                                            .contains(b)
                                        {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                10020684034467804360u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 63u8))(inner))?
                                })
                            })())?;
                            let field2 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            (field0, field1, field2)
                        }

                        1 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if (ByteSet::from_bits([0, 0, 0, 35175782154240]))
                                            .contains(b)
                                        {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                15018012796466655710u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 15u8))(inner))?
                                })
                            })())?;
                            let field1 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            let field2 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            (field0, field1, field2)
                        }

                        2 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if b == 237 {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                4000866269867594892u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 15u8))(inner))?
                                })
                            })())?;
                            let field1 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if (ByteSet::from_bits([0, 0, 4294967295, 0])).contains(b) {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                11663367663089555181u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 63u8))(inner))?
                                })
                            })())?;
                            let field2 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            (field0, field1, field2)
                        }

                        3 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if (ByteSet::from_bits([0, 0, 0, 211106232532992]))
                                            .contains(b)
                                        {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                6041500870840229679u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 15u8))(inner))?
                                })
                            })())?;
                            let field1 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            let field2 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            (field0, field1, field2)
                        }

                        _ => {
                            return Err(ParseError::ExcludedBranch(16680599075360251934u64));
                        }
                    }
                };
                ((|bytes: (u8, u8, u8)| {
                    PResult::Ok(match bytes {
                        (x2, x1, x0) => (x2 as u32) << 12u32 | (x1 as u32) << 6u32 | (x0 as u32),
                    })
                })(inner))?
            }

            3 => {
                let inner = {
                    let tree_index = {
                        _input.open_peek_context();
                        let b = _input.read_byte()?;
                        {
                            let ret = match b {
                                240u8 => 0,

                                tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184]))
                                    .contains(tmp)) =>
                                {
                                    1
                                }

                                244u8 => 2,

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
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if b == 240 {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                4436478097112104593u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 7u8))(inner))?
                                })
                            })())?;
                            let field1 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if (ByteSet::from_bits([0, 0, 18446744073709486080, 0]))
                                            .contains(b)
                                        {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                2326106400913054182u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 63u8))(inner))?
                                })
                            })())?;
                            let field2 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            let field3 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            (field0, field1, field2, field3)
                        }

                        1 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if (ByteSet::from_bits([0, 0, 0, 3940649673949184]))
                                            .contains(b)
                                        {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                2405483008932899239u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 7u8))(inner))?
                                })
                            })())?;
                            let field1 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            let field2 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            let field3 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            (field0, field1, field2, field3)
                        }

                        2 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if b == 244 {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                7074153516412524481u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 7u8))(inner))?
                                })
                            })())?;
                            let field1 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = _input.read_byte()?;
                                        if (ByteSet::from_bits([0, 0, 65535, 0])).contains(b) {
                                            b
                                        } else {
                                            return Err(ParseError::ExcludedBranch(
                                                7043438521252360401u64,
                                            ));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok(raw & 63u8))(inner))?
                                })
                            })())?;
                            let field2 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            let field3 = ((|| PResult::Ok((Decoder12(_input))?))())?;
                            (field0, field1, field2, field3)
                        }

                        _ => {
                            return Err(ParseError::ExcludedBranch(10897670729404727847u64));
                        }
                    }
                };
                ((|bytes: (u8, u8, u8, u8)| {
                    PResult::Ok(match bytes {
                        (x3, x2, x1, x0) => {
                            (x3 as u32) << 18u32
                                | (x2 as u32) << 12u32
                                | (x1 as u32) << 6u32
                                | (x0 as u32)
                        }
                    })
                })(inner))?
            }

            _ => {
                return Err(ParseError::ExcludedBranch(7414759281301542086u64));
            }
        }
    };
    PResult::Ok(((|codepoint: u32| PResult::Ok((char::from_u32(codepoint)).unwrap()))(inner))?)
}

fn Decoder12<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
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

fn Decoder13<'input>(_input: &mut Parser<'input>) -> Result<Type196, ParseError> {
    let header = ((|| PResult::Ok((Decoder14(_input))?))())?;
    let file = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..header.clone().size {
                accum.push((Decoder15(_input))?);
            }
            accum
        })
    })())?;
    let __padding = ((|| PResult::Ok(_input.skip_align(512)?))())?;
    PResult::Ok(Type196 {
        header,
        file,
        __padding,
    })
}

fn Decoder14<'input>(_input: &mut Parser<'input>) -> Result<Type195, ParseError> {
    let sz = 512u32 as usize;
    _input.start_slice(sz)?;
    let ret = ((|| {
        PResult::Ok({
            let name = ((|| {
                PResult::Ok({
                    let sz = 100u16 as usize;
                    _input.start_slice(sz)?;
                    let ret = ((|| PResult::Ok((Decoder16(_input))?))())?;
                    _input.end_slice()?;
                    ret
                })
            })())?;
            let mode =
                ((|| {
                    PResult::Ok({
                        let sz = 8u16 as usize;
                        _input.start_slice(sz)?;
                        let ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = Vec::new();
                                                while _input.remaining() > 0 {
                                                    let matching_ix =
                                                        {
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
                                                        let next_elem = (Decoder17(_input))?;
                                                        accum.push(next_elem);
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp = ((|| PResult::Ok((Decoder18(_input))?))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = Vec::new();
                                            while _input.remaining() > 0 {
                                                let matching_ix = {
                                                    _input.open_peek_context();
                                                    let b = _input.read_byte()?;
                                                    {
                                                        let ret = if b == 0 { 0 } else { 1 };
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
                                                            return Err(
                                                                ParseError::ExcludedBranch(
                                                                    10396965092922267801u64,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    accum.push(next_elem);
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type193 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        _input.end_slice()?;
                        ret
                    })
                })())?;
            let uid =
                ((|| {
                    PResult::Ok({
                        let sz = 8u16 as usize;
                        _input.start_slice(sz)?;
                        let ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = Vec::new();
                                                while _input.remaining() > 0 {
                                                    let matching_ix =
                                                        {
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
                                                        let next_elem = (Decoder17(_input))?;
                                                        accum.push(next_elem);
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp = ((|| PResult::Ok((Decoder18(_input))?))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = Vec::new();
                                            while _input.remaining() > 0 {
                                                let matching_ix = {
                                                    _input.open_peek_context();
                                                    let b = _input.read_byte()?;
                                                    {
                                                        let ret = if b == 0 { 0 } else { 1 };
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
                                                            return Err(
                                                                ParseError::ExcludedBranch(
                                                                    10396965092922267801u64,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    accum.push(next_elem);
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type193 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        _input.end_slice()?;
                        ret
                    })
                })())?;
            let gid =
                ((|| {
                    PResult::Ok({
                        let sz = 8u16 as usize;
                        _input.start_slice(sz)?;
                        let ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = Vec::new();
                                                while _input.remaining() > 0 {
                                                    let matching_ix =
                                                        {
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
                                                        let next_elem = (Decoder17(_input))?;
                                                        accum.push(next_elem);
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp = ((|| PResult::Ok((Decoder18(_input))?))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = Vec::new();
                                            while _input.remaining() > 0 {
                                                let matching_ix = {
                                                    _input.open_peek_context();
                                                    let b = _input.read_byte()?;
                                                    {
                                                        let ret = if b == 0 { 0 } else { 1 };
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
                                                            return Err(
                                                                ParseError::ExcludedBranch(
                                                                    10396965092922267801u64,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    accum.push(next_elem);
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type193 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        _input.end_slice()?;
                        ret
                    })
                })())?;
            let size = ((|| {
                PResult::Ok({
                    let inner = {
                        let oA = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let o9 = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let o8 = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let o7 = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let o6 = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let o5 = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let o4 = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let o3 = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let o2 = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let o1 = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let o0 = ((|| {
                            PResult::Ok({
                                let inner = (Decoder17(_input))?;
                                ((|bit: u8| PResult::Ok((bit as u8) - 48u8))(inner))?
                            })
                        })())?;
                        let __nil = ((|| PResult::Ok((Decoder18(_input))?))())?;
                        let value = ((|| {
                            PResult::Ok(
                                (((0u8 as u32) << 3u32 | (oA as u32)) << 6u32
                                    | (o9 as u32) << 3u32
                                    | (o8 as u32))
                                    << 24u32
                                    | (((o7 as u32) << 3u32 | (o6 as u32)) << 6u32
                                        | (o5 as u32) << 3u32
                                        | (o4 as u32))
                                        << 12u32
                                    | ((o3 as u32) << 3u32 | (o2 as u32)) << 6u32
                                    | (o1 as u32) << 3u32
                                    | (o0 as u32),
                            )
                        })())?;
                        Type200 {
                            oA,
                            o9,
                            o8,
                            o7,
                            o6,
                            o5,
                            o4,
                            o3,
                            o2,
                            o1,
                            o0,
                            __nil,
                            value,
                        }
                    };
                    ((|rec: Type200| PResult::Ok(rec.clone().value))(inner))?
                })
            })())?;
            let mtime =
                ((|| {
                    PResult::Ok({
                        let sz = 12u16 as usize;
                        _input.start_slice(sz)?;
                        let ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = Vec::new();
                                                while _input.remaining() > 0 {
                                                    let matching_ix =
                                                        {
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
                                                        let next_elem = (Decoder17(_input))?;
                                                        accum.push(next_elem);
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp = ((|| PResult::Ok((Decoder18(_input))?))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = Vec::new();
                                            while _input.remaining() > 0 {
                                                let matching_ix = {
                                                    _input.open_peek_context();
                                                    let b = _input.read_byte()?;
                                                    {
                                                        let ret = if b == 0 { 0 } else { 1 };
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
                                                            return Err(
                                                                ParseError::ExcludedBranch(
                                                                    10396965092922267801u64,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    accum.push(next_elem);
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type193 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        _input.end_slice()?;
                        ret
                    })
                })())?;
            let chksum =
                ((|| {
                    PResult::Ok({
                        let sz = 8u16 as usize;
                        _input.start_slice(sz)?;
                        let ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = Vec::new();
                                                while _input.remaining() > 0 {
                                                    let matching_ix =
                                                        {
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
                                                        let next_elem = (Decoder17(_input))?;
                                                        accum.push(next_elem);
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp = ((|| PResult::Ok((Decoder18(_input))?))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = Vec::new();
                                            while _input.remaining() > 0 {
                                                let matching_ix = {
                                                    _input.open_peek_context();
                                                    let b = _input.read_byte()?;
                                                    {
                                                        let ret = if b == 0 { 0 } else { 1 };
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
                                                            return Err(
                                                                ParseError::ExcludedBranch(
                                                                    10396965092922267801u64,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    accum.push(next_elem);
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type193 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        _input.end_slice()?;
                        ret
                    })
                })())?;
            let typeflag = ((|| PResult::Ok((Decoder19(_input))?))())?;
            let linkname = ((|| {
                PResult::Ok({
                    let sz = 100u16 as usize;
                    _input.start_slice(sz)?;
                    let ret = ((|| PResult::Ok((Decoder20(_input))?))())?;
                    _input.end_slice()?;
                    ret
                })
            })())?;
            let magic = ((|| {
                PResult::Ok({
                    let field0 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 117 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(2907868308058195485u64));
                            }
                        })
                    })())?;
                    let field1 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 115 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(17994192348199484624u64));
                            }
                        })
                    })())?;
                    let field2 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 116 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(1704008783145591213u64));
                            }
                        })
                    })())?;
                    let field3 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 97 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(5987471249031546739u64));
                            }
                        })
                    })())?;
                    let field4 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 114 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(3985419300396206930u64));
                            }
                        })
                    })())?;
                    let field5 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 0 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
                            }
                        })
                    })())?;
                    (field0, field1, field2, field3, field4, field5)
                })
            })())?;
            let version = ((|| {
                PResult::Ok({
                    let field0 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 48 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(10502325800520584810u64));
                            }
                        })
                    })())?;
                    let field1 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 48 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(10502325800520584810u64));
                            }
                        })
                    })())?;
                    (field0, field1)
                })
            })())?;
            let uname = ((|| {
                PResult::Ok({
                    let sz = 32u16 as usize;
                    _input.start_slice(sz)?;
                    let ret = ((|| PResult::Ok((Decoder21(_input))?))())?;
                    _input.end_slice()?;
                    ret
                })
            })())?;
            let gname = ((|| {
                PResult::Ok({
                    let sz = 32u16 as usize;
                    _input.start_slice(sz)?;
                    let ret = ((|| PResult::Ok((Decoder21(_input))?))())?;
                    _input.end_slice()?;
                    ret
                })
            })())?;
            let devmajor =
                ((|| {
                    PResult::Ok({
                        let sz = 8u16 as usize;
                        _input.start_slice(sz)?;
                        let ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = Vec::new();
                                                while _input.remaining() > 0 {
                                                    let matching_ix =
                                                        {
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
                                                        let next_elem = (Decoder17(_input))?;
                                                        accum.push(next_elem);
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp = ((|| PResult::Ok((Decoder18(_input))?))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = Vec::new();
                                            while _input.remaining() > 0 {
                                                let matching_ix = {
                                                    _input.open_peek_context();
                                                    let b = _input.read_byte()?;
                                                    {
                                                        let ret = if b == 0 { 0 } else { 1 };
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
                                                            return Err(
                                                                ParseError::ExcludedBranch(
                                                                    10396965092922267801u64,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    accum.push(next_elem);
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type193 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        _input.end_slice()?;
                        ret
                    })
                })())?;
            let devminor =
                ((|| {
                    PResult::Ok({
                        let sz = 8u16 as usize;
                        _input.start_slice(sz)?;
                        let ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = Vec::new();
                                                while _input.remaining() > 0 {
                                                    let matching_ix =
                                                        {
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
                                                        let next_elem = (Decoder17(_input))?;
                                                        accum.push(next_elem);
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp = ((|| PResult::Ok((Decoder18(_input))?))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = Vec::new();
                                            while _input.remaining() > 0 {
                                                let matching_ix = {
                                                    _input.open_peek_context();
                                                    let b = _input.read_byte()?;
                                                    {
                                                        let ret = if b == 0 { 0 } else { 1 };
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
                                                            return Err(
                                                                ParseError::ExcludedBranch(
                                                                    10396965092922267801u64,
                                                                ),
                                                            );
                                                        }
                                                    };
                                                    accum.push(next_elem);
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type193 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        _input.end_slice()?;
                        ret
                    })
                })())?;
            let prefix = ((|| {
                PResult::Ok({
                    let sz = 155u16 as usize;
                    _input.start_slice(sz)?;
                    let ret = ((|| PResult::Ok((Decoder20(_input))?))())?;
                    _input.end_slice()?;
                    ret
                })
            })())?;
            let pad = ((|| {
                PResult::Ok({
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
                })
            })())?;
            Type195 {
                name,
                mode,
                uid,
                gid,
                size,
                mtime,
                chksum,
                typeflag,
                linkname,
                magic,
                version,
                uname,
                gname,
                devmajor,
                devminor,
                prefix,
                pad,
            }
        })
    })())?;
    _input.end_slice()?;
    PResult::Ok(ret)
}

fn Decoder15<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
    let b = _input.read_byte()?;
    PResult::Ok(b)
}

fn Decoder16<'input>(_input: &mut Parser<'input>) -> Result<Type192, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            0u8 => 0,

                            tmp if (tmp != 0) => 1,

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
                        break;
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
        })
    })())?;
    let __padding = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = if b == 0 { 0 } else { 1 };
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
                    break;
                }
            }
            accum
        })
    })())?;
    PResult::Ok(Type192 { string, __padding })
}

fn Decoder17<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
    let b = _input.read_byte()?;
    PResult::Ok(
        if (ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(b) {
            b
        } else {
            return Err(ParseError::ExcludedBranch(16196330650984947656u64));
        },
    )
}

fn Decoder18<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
    let b = _input.read_byte()?;
    PResult::Ok(if (ByteSet::from_bits([4294967297, 0, 0, 0])).contains(b) {
        b
    } else {
        return Err(ParseError::ExcludedBranch(9824667705306069359u64));
    })
}

fn Decoder19<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
    let b = _input.read_byte()?;
    PResult::Ok(b)
}

fn Decoder20<'input>(_input: &mut Parser<'input>) -> Result<Type192, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let __padding = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = if b == 0 { 0 } else { 1 };
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
                    break;
                }
            }
            accum
        })
    })())?;
    PResult::Ok(Type192 { string, __padding })
}

fn Decoder21<'input>(_input: &mut Parser<'input>) -> Result<Type194, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let padding = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = if b == 0 { 1 } else { 0 };
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    if accum.is_empty() {
                        return Err(ParseError::InsufficientRepeats);
                    } else {
                        break;
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
        })
    })())?;
    PResult::Ok(Type194 { string, padding })
}

fn Decoder22<'input>(_input: &mut Parser<'input>) -> Result<u32, ParseError> {
    let inner = {
        let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field3 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        (field0, field1, field2, field3)
    };
    PResult::Ok(((|x: (u8, u8, u8, u8)| PResult::Ok(u32le(x)))(inner))?)
}

fn Decoder23<'input>(_input: &mut Parser<'input>) -> Result<Type190, ParseError> {
    let tag = ((|| PResult::Ok((Decoder24(_input))?))())?;
    let chunks = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
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
                    break;
                }
            }
            accum
        })
    })())?;
    PResult::Ok(Type190 { tag, chunks })
}

fn Decoder24<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
    let field0 = ((|| PResult::Ok((Decoder19(_input))?))())?;
    let field1 = ((|| PResult::Ok((Decoder19(_input))?))())?;
    let field2 = ((|| PResult::Ok((Decoder19(_input))?))())?;
    let field3 = ((|| PResult::Ok((Decoder19(_input))?))())?;
    PResult::Ok((field0, field1, field2, field3))
}

fn Decoder25<'input>(_input: &mut Parser<'input>) -> Result<Type189, ParseError> {
    let tag = ((|| PResult::Ok((Decoder24(_input))?))())?;
    let length = ((|| PResult::Ok((Decoder22(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let pad = ((|| {
        PResult::Ok(match length % 2u32 == 0u32 {
            true => {
                let _ = ();
                Type188::yes
            }

            false => {
                let inner = {
                    let b = _input.read_byte()?;
                    if b == 0 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10396965092922267801u64));
                    }
                };
                Type188::no(inner)
            }
        })
    })())?;
    PResult::Ok(Type189 {
        tag,
        length,
        data,
        pad,
    })
}

fn Decoder26<'input>(
    _input: &mut Parser<'input>,
) -> Result<(u8, u8, u8, u8, u8, u8, u8, u8), ParseError> {
    let field0 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 137 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10008271234946209065u64));
            }
        })
    })())?;
    let field1 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 80 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(11521109187063420822u64));
            }
        })
    })())?;
    let field2 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 78 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(8604468179520937907u64));
            }
        })
    })())?;
    let field3 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 71 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(690880023569680479u64));
            }
        })
    })())?;
    let field4 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 13 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10755821400739488603u64));
            }
        })
    })())?;
    let field5 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 10 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(4202505692043699682u64));
            }
        })
    })())?;
    let field6 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 26 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(349275303258878611u64));
            }
        })
    })())?;
    let field7 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 10 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(4202505692043699682u64));
            }
        })
    })())?;
    PResult::Ok((
        field0, field1, field2, field3, field4, field5, field6, field7,
    ))
}

fn Decoder27<'input>(_input: &mut Parser<'input>) -> Result<Type165, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| PResult::Ok((Decoder46(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder47(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type165 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder28<'input>(_input: &mut Parser<'input>, ihdr: Type165) -> Result<Type184, ParseError> {
    let tree_index = {
        _input.open_peek_context();
        _input.read_byte()?;
        _input.read_byte()?;
        _input.read_byte()?;
        _input.read_byte()?;
        let b = _input.read_byte()?;
        {
            let ret = match b {
                98u8 => 0,

                99u8 => 1,

                103u8 => 2,

                112u8 => 3,

                80u8 => 4,

                116u8 => {
                    let b = _input.read_byte()?;
                    match b {
                        69u8 => 5,

                        73u8 => 6,

                        82u8 => 7,

                        _ => {
                            return Err(ParseError::ExcludedBranch(10357236553041691094u64));
                        }
                    }
                }

                _ => {
                    return Err(ParseError::ExcludedBranch(1665541366391616829u64));
                }
            };
            _input.close_peek_context()?;
            ret
        }
    };
    PResult::Ok(match tree_index {
        0 => {
            let inner = (Decoder36(_input, ihdr.clone()))?;
            Type184::bKGD(inner)
        }

        1 => {
            let inner = (Decoder37(_input))?;
            Type184::cHRM(inner)
        }

        2 => {
            let inner = (Decoder38(_input))?;
            Type184::gAMA(inner)
        }

        3 => {
            let inner = (Decoder39(_input))?;
            Type184::pHYs(inner)
        }

        4 => {
            let inner = (Decoder40(_input))?;
            Type184::PLTE(inner)
        }

        5 => {
            let inner = (Decoder41(_input))?;
            Type184::tEXt(inner)
        }

        6 => {
            let inner = (Decoder42(_input))?;
            Type184::tIME(inner)
        }

        7 => {
            let inner = (Decoder43(_input, ihdr.clone()))?;
            Type184::tRNS(inner)
        }

        _ => {
            return Err(ParseError::ExcludedBranch(2929130490246398173u64));
        }
    })
}

fn Decoder29<'input>(_input: &mut Parser<'input>) -> Result<Type185, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| PResult::Ok((Decoder34(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder35(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type185 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder30<'input>(_input: &mut Parser<'input>) -> Result<Type186, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| PResult::Ok((Decoder32(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder33(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type186 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder31<'input>(_input: &mut Parser<'input>) -> Result<u32, ParseError> {
    let inner = {
        let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field3 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        (field0, field1, field2, field3)
    };
    PResult::Ok(((|x: (u8, u8, u8, u8)| PResult::Ok(u32be(x)))(inner))?)
}

fn Decoder32<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
    let field0 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 73 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(17197161005512507961u64));
            }
        })
    })())?;
    let field1 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 69 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(4321719390811047443u64));
            }
        })
    })())?;
    let field2 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 78 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(8604468179520937907u64));
            }
        })
    })())?;
    let field3 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 68 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(11087183532096489351u64));
            }
        })
    })())?;
    PResult::Ok((field0, field1, field2, field3))
}

fn Decoder33<'input>(_input: &mut Parser<'input>) -> Result<(), ParseError> {
    PResult::Ok(())
}

fn Decoder34<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
    let field0 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 73 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(17197161005512507961u64));
            }
        })
    })())?;
    let field1 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 68 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(11087183532096489351u64));
            }
        })
    })())?;
    let field2 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 65 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(5168475411614401238u64));
            }
        })
    })())?;
    let field3 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 84 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(145148447135656575u64));
            }
        })
    })())?;
    PResult::Ok((field0, field1, field2, field3))
}

fn Decoder35<'input>(_input: &mut Parser<'input>) -> Result<Vec<u8>, ParseError> {
    let mut accum = Vec::new();
    while _input.remaining() > 0 {
        let matching_ix = {
            _input.open_peek_context();
            _input.read_byte()?;
            {
                let ret = 0;
                _input.close_peek_context()?;
                ret
            }
        };
        if matching_ix == 0 {
            let next_elem = (Decoder15(_input))?;
            accum.push(next_elem);
        } else {
            break;
        }
    }
    PResult::Ok(accum)
}

fn Decoder36<'input>(_input: &mut Parser<'input>, ihdr: Type165) -> Result<Type171, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 98 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(17793564444344969327u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 75 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(6039736239144529496u64));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 71 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(690880023569680479u64));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 68 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(11087183532096489351u64));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match ihdr.clone().data.color_type {
                    0 => {
                        let inner = {
                            let greyscale = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            Type167 { greyscale }
                        };
                        Type170::color_type_0(inner)
                    }

                    4 => {
                        let inner = {
                            let greyscale = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            Type167 { greyscale }
                        };
                        Type170::color_type_4(inner)
                    }

                    2 => {
                        let inner = {
                            let red = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let green = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let blue = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            Type168 { red, green, blue }
                        };
                        Type170::color_type_2(inner)
                    }

                    6 => {
                        let inner = {
                            let red = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let green = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let blue = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            Type168 { red, green, blue }
                        };
                        Type170::color_type_6(inner)
                    }

                    3 => {
                        let inner = {
                            let palette_index = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            Type169 { palette_index }
                        };
                        Type170::color_type_3(inner)
                    }

                    _other => {
                        unreachable!(
                            r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                        );
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type171 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder37<'input>(_input: &mut Parser<'input>) -> Result<Type173, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 99 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(11313607538540189010u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 72 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(13017675598322041426u64));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 82 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(4610689655322527862u64));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 77 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(1661485880725065159u64));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let whitepoint_x = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    let whitepoint_y = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    let red_x = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    let red_y = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    let green_x = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    let green_y = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    let blue_x = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    let blue_y = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    Type172 {
                        whitepoint_x,
                        whitepoint_y,
                        red_x,
                        red_y,
                        green_x,
                        green_y,
                        blue_x,
                        blue_y,
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type173 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder38<'input>(_input: &mut Parser<'input>) -> Result<Type175, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 103 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(1468953881245131984u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 65 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(5168475411614401238u64));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 77 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(1661485880725065159u64));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 65 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(5168475411614401238u64));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let gamma = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    Type174 { gamma }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type175 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder39<'input>(_input: &mut Parser<'input>) -> Result<Type177, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 112 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(11580992248901122101u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 72 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(13017675598322041426u64));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 89 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(8653514599897871365u64));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 115 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(17994192348199484624u64));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let pixels_per_unit_x = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    let pixels_per_unit_y = ((|| PResult::Ok((Decoder31(_input))?))())?;
                    let unit_specifier = ((|| PResult::Ok((Decoder15(_input))?))())?;
                    Type176 {
                        pixels_per_unit_x,
                        pixels_per_unit_y,
                        unit_specifier,
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type177 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder40<'input>(_input: &mut Parser<'input>) -> Result<Type166, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 80 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(11521109187063420822u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 76 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(7343583089148506132u64));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 84 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(145148447135656575u64));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 69 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(4321719390811047443u64));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
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
                                break;
                            }
                        } else {
                            let next_elem = {
                                let r = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                let g = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                let b = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                Type2 { r, g, b }
                            };
                            accum.push(next_elem);
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type166 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder41<'input>(_input: &mut Parser<'input>) -> Result<Type179, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 116 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(1704008783145591213u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 69 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(4321719390811047443u64));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 88 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(17869373923605816126u64));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 116 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(1704008783145591213u64));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let keyword = ((|| PResult::Ok((Decoder45(_input))?))())?;
                    let text = ((|| {
                        PResult::Ok({
                            let mut accum = Vec::new();
                            while _input.remaining() > 0 {
                                let matching_ix = {
                                    _input.open_peek_context();
                                    _input.read_byte()?;
                                    {
                                        let ret = 0;
                                        _input.close_peek_context()?;
                                        ret
                                    }
                                };
                                if matching_ix == 0 {
                                    let next_elem = (Decoder19(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        })
                    })())?;
                    Type178 { keyword, text }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type179 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder42<'input>(_input: &mut Parser<'input>) -> Result<Type181, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 116 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(1704008783145591213u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 73 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(17197161005512507961u64));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 77 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(1661485880725065159u64));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 69 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(4321719390811047443u64));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let year = ((|| PResult::Ok((Decoder44(_input))?))())?;
                    let month = ((|| PResult::Ok((Decoder15(_input))?))())?;
                    let day = ((|| PResult::Ok((Decoder15(_input))?))())?;
                    let hour = ((|| PResult::Ok((Decoder15(_input))?))())?;
                    let minute = ((|| PResult::Ok((Decoder15(_input))?))())?;
                    let second = ((|| PResult::Ok((Decoder15(_input))?))())?;
                    Type180 {
                        year,
                        month,
                        day,
                        hour,
                        minute,
                        second,
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type181 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder43<'input>(_input: &mut Parser<'input>, ihdr: Type165) -> Result<Type183, ParseError> {
    let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 116 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(1704008783145591213u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 82 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(4610689655322527862u64));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 78 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(8604468179520937907u64));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 83 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(16554645260058031671u64));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = length as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match ihdr.clone().data.color_type {
                    0 => {
                        let inner = {
                            let greyscale = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            Type167 { greyscale }
                        };
                        Type182::color_type_0(inner)
                    }

                    2 => {
                        let inner = {
                            let red = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let green = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let blue = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            Type168 { red, green, blue }
                        };
                        Type182::color_type_2(inner)
                    }

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
                                        let palette_index =
                                            ((|| PResult::Ok((Decoder15(_input))?))())?;
                                        Type169 { palette_index }
                                    };
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type182::color_type_3(inner)
                    }

                    _other => {
                        unreachable!(
                            r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                        );
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok((Decoder31(_input))?))())?;
    PResult::Ok(Type183 {
        length,
        tag,
        data,
        crc,
    })
}

fn Decoder44<'input>(_input: &mut Parser<'input>) -> Result<u16, ParseError> {
    let inner = {
        let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        (field0, field1)
    };
    PResult::Ok(((|x: (u8, u8)| PResult::Ok(u16be(x)))(inner))?)
}

fn Decoder45<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    PResult::Ok(Type21 { string, null })
}

fn Decoder46<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
    let field0 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 73 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(17197161005512507961u64));
            }
        })
    })())?;
    let field1 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 72 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(13017675598322041426u64));
            }
        })
    })())?;
    let field2 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 68 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(11087183532096489351u64));
            }
        })
    })())?;
    let field3 = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 82 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(4610689655322527862u64));
            }
        })
    })())?;
    PResult::Ok((field0, field1, field2, field3))
}

fn Decoder47<'input>(_input: &mut Parser<'input>) -> Result<Type164, ParseError> {
    let width = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let height = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let bit_depth = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let color_type = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let compression_method = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let filter_method = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let interlace_method = ((|| PResult::Ok((Decoder15(_input))?))())?;
    PResult::Ok(Type164 {
        width,
        height,
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
    })
}

fn Decoder48<'input>(_input: &mut Parser<'input>) -> Result<Type162, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (102, 116, 121, 112) => {
                        let inner = {
                            let major_brand = ((|| PResult::Ok((Decoder49(_input))?))())?;
                            let minor_version = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let compatible_brands = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    while _input.remaining() > 0 {
                                        let matching_ix = {
                                            _input.open_peek_context();
                                            _input.read_byte()?;
                                            {
                                                let ret = 0;
                                                _input.close_peek_context()?;
                                                ret
                                            }
                                        };
                                        if matching_ix == 0 {
                                            let next_elem = (Decoder49(_input))?;
                                            accum.push(next_elem);
                                        } else {
                                            break;
                                        }
                                    }
                                    accum
                                })
                            })())?;
                            Type81 {
                                major_brand,
                                minor_version,
                                compatible_brands,
                            }
                        };
                        Type161::ftyp(inner)
                    }

                    (102, 114, 101, 101) => {
                        let _ = ();
                        Type161::free
                    }

                    (109, 100, 97, 116) => {
                        let _ = ();
                        Type161::mdat
                    }

                    (109, 101, 116, 97) => {
                        let field0 = ((|| PResult::Ok((Decoder31(_input))?))())?;
                        let field1 = ((|| {
                            PResult::Ok({
                                let mut accum = Vec::new();
                                while _input.remaining() > 0 {
                                    let matching_ix = {
                                        _input.open_peek_context();
                                        _input.read_byte()?;
                                        {
                                            let ret = 0;
                                            _input.close_peek_context()?;
                                            ret
                                        }
                                    };
                                    if matching_ix == 0 {
                                        let next_elem = (Decoder51(_input))?;
                                        accum.push(next_elem);
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            })
                        })())?;
                        Type161::meta(field0, field1)
                    }

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
                                    let next_elem = (Decoder52(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type161::moov(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type161::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type162 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder49<'input>(_input: &mut Parser<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
    let field0 = ((|| PResult::Ok((Decoder19(_input))?))())?;
    let field1 = ((|| PResult::Ok((Decoder19(_input))?))())?;
    let field2 = ((|| PResult::Ok((Decoder19(_input))?))())?;
    let field3 = ((|| PResult::Ok((Decoder19(_input))?))())?;
    PResult::Ok((field0, field1, field2, field3))
}

fn Decoder50<'input>(_input: &mut Parser<'input>) -> Result<u64, ParseError> {
    let inner = {
        let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field3 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field4 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field5 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field6 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field7 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        (
            field0, field1, field2, field3, field4, field5, field6, field7,
        )
    };
    PResult::Ok(((|x: (u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok(u64be(x)))(inner))?)
}

fn Decoder51<'input>(_input: &mut Parser<'input>) -> Result<Type115, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
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
                                    let next_elem = (Decoder59(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type114::dinf(inner)
                    }

                    (104, 100, 108, 114) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let predefined = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let handler_type = ((|| PResult::Ok((Decoder49(_input))?))())?;
                            let reserved = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder31(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder31(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder31(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let name = ((|| PResult::Ok((Decoder57(_input))?))())?;
                            Type86 {
                                version,
                                flags,
                                predefined,
                                handler_type,
                                reserved,
                                name,
                            }
                        };
                        Type114::hdlr(inner)
                    }

                    (112, 105, 116, 109) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let item_ID = ((|| {
                                PResult::Ok(match version == 0u8 {
                                    true => {
                                        let inner = (Decoder44(_input))?;
                                        Type112::yes(inner)
                                    }

                                    false => {
                                        let inner = (Decoder31(_input))?;
                                        Type112::no(inner)
                                    }
                                })
                            })())?;
                            Type113 {
                                version,
                                flags,
                                item_ID,
                            }
                        };
                        Type114::pitm(inner)
                    }

                    (105, 105, 110, 102) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let entry_count = ((|| {
                                PResult::Ok(match version == 0u8 {
                                    true => {
                                        let inner = (Decoder44(_input))?;
                                        ((|x: u16| PResult::Ok(x as u32))(inner))?
                                    }

                                    false => (Decoder31(_input))?,
                                })
                            })())?;
                            let item_info_entry = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..entry_count {
                                        accum.push((Decoder61(_input))?);
                                    }
                                    accum
                                })
                            })())?;
                            Type96 {
                                version,
                                flags,
                                entry_count,
                                item_info_entry,
                            }
                        };
                        Type114::iinf(inner)
                    }

                    (105, 114, 101, 102) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let single_item_reference = ((|| {
                                PResult::Ok(match version {
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
                                                        let size_field = ((|| {
                                                            PResult::Ok((Decoder31(_input))?)
                                                        })(
                                                        ))?;
                                                        let r#type = ((|| {
                                                            PResult::Ok((Decoder49(_input))?)
                                                        })(
                                                        ))?;
                                                        let size = ((|| {
                                                            PResult::Ok(match size_field {
                                                                0 => 0u64,

                                                                1 => {
                                                                    let inner =
                                                                        (Decoder50(_input))?;
                                                                    ((|x: u64| {
                                                                        PResult::Ok(x - 16u64)
                                                                    })(
                                                                        inner
                                                                    ))?
                                                                }

                                                                _ => (size_field - 8u32) as u64,
                                                            })
                                                        })(
                                                        ))?;
                                                        let data = ((|| {
                                                            PResult::Ok({
                                                                let sz = size as usize;
                                                                _input.start_slice(sz)?;
                                                                let ret = ((|| {
                                                                    PResult::Ok({
                                                                        let from_item_ID =
                                                                            ((|| {
                                                                                PResult::Ok(
                                                                                    (Decoder44(
                                                                                        _input,
                                                                                    ))?,
                                                                                )
                                                                            })(
                                                                            ))?;
                                                                        let reference_count =
                                                                            ((|| {
                                                                                PResult::Ok(
                                                                                    (Decoder44(
                                                                                        _input,
                                                                                    ))?,
                                                                                )
                                                                            })(
                                                                            ))?;
                                                                        let to_item_ID =
                                                                            ((|| {
                                                                                PResult::Ok({
                                                                                    let mut accum =
                                                                                        Vec::new();
                                                                                    for _ in 0..reference_count {
accum.push((Decoder44(_input))?);
}
                                                                                    accum
                                                                                })
                                                                            })(
                                                                            ))?;
                                                                        Type108 {
                                                                            from_item_ID,
                                                                            reference_count,
                                                                            to_item_ID,
                                                                        }
                                                                    })
                                                                })(
                                                                ))?;
                                                                _input.end_slice()?;
                                                                ret
                                                            })
                                                        })(
                                                        ))?;
                                                        Type109 {
                                                            size_field,
                                                            r#type,
                                                            size,
                                                            data,
                                                        }
                                                    };
                                                    accum.push(next_elem);
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        };
                                        Type110::small(inner)
                                    }

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
                                                        let size_field = ((|| {
                                                            PResult::Ok((Decoder31(_input))?)
                                                        })(
                                                        ))?;
                                                        let r#type = ((|| {
                                                            PResult::Ok((Decoder49(_input))?)
                                                        })(
                                                        ))?;
                                                        let size = ((|| {
                                                            PResult::Ok(match size_field {
                                                                0 => 0u64,

                                                                1 => {
                                                                    let inner =
                                                                        (Decoder50(_input))?;
                                                                    ((|x: u64| {
                                                                        PResult::Ok(x - 16u64)
                                                                    })(
                                                                        inner
                                                                    ))?
                                                                }

                                                                _ => (size_field - 8u32) as u64,
                                                            })
                                                        })(
                                                        ))?;
                                                        let data = ((|| {
                                                            PResult::Ok({
                                                                let sz = size as usize;
                                                                _input.start_slice(sz)?;
                                                                let ret = ((|| {
                                                                    PResult::Ok({
                                                                        let from_item_ID =
                                                                            ((|| {
                                                                                PResult::Ok(
                                                                                    (Decoder31(
                                                                                        _input,
                                                                                    ))?,
                                                                                )
                                                                            })(
                                                                            ))?;
                                                                        let reference_count =
                                                                            ((|| {
                                                                                PResult::Ok(
                                                                                    (Decoder44(
                                                                                        _input,
                                                                                    ))?,
                                                                                )
                                                                            })(
                                                                            ))?;
                                                                        let to_item_ID =
                                                                            ((|| {
                                                                                PResult::Ok({
                                                                                    let mut accum =
                                                                                        Vec::new();
                                                                                    for _ in 0..reference_count {
accum.push((Decoder31(_input))?);
}
                                                                                    accum
                                                                                })
                                                                            })(
                                                                            ))?;
                                                                        Type106 {
                                                                            from_item_ID,
                                                                            reference_count,
                                                                            to_item_ID,
                                                                        }
                                                                    })
                                                                })(
                                                                ))?;
                                                                _input.end_slice()?;
                                                                ret
                                                            })
                                                        })(
                                                        ))?;
                                                        Type107 {
                                                            size_field,
                                                            r#type,
                                                            size,
                                                            data,
                                                        }
                                                    };
                                                    accum.push(next_elem);
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        };
                                        Type110::large(inner)
                                    }

                                    _other => {
                                        unreachable!(
                                            r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                                        );
                                    }
                                })
                            })())?;
                            Type111 {
                                version,
                                flags,
                                single_item_reference,
                            }
                        };
                        Type114::iref(inner)
                    }

                    (105, 108, 111, 99) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let offset_size_length_size =
                                ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let base_offset_size_index_size =
                                ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let offset_size = ((|| PResult::Ok(offset_size_length_size >> 4u8))())?;
                            let length_size = ((|| PResult::Ok(offset_size_length_size & 7u8))())?;
                            let base_offset_size =
                                ((|| PResult::Ok(base_offset_size_index_size >> 4u8))())?;
                            let index_size = ((|| {
                                PResult::Ok(match version > 0u8 {
                                    true => base_offset_size_index_size & 7u8,

                                    false => 0u8,
                                })
                            })())?;
                            let item_count = ((|| {
                                PResult::Ok(match version < 2u8 {
                                    true => {
                                        let inner = (Decoder44(_input))?;
                                        ((|x: u16| PResult::Ok(x as u32))(inner))?
                                    }

                                    false => (Decoder31(_input))?,
                                })
                            })())?;
                            let items = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..item_count {
                                        accum.push({
let item_ID = ((|| PResult::Ok(match version < 2u8 {
true => {
let inner = (Decoder44(_input))?;
((|x: u16| PResult::Ok(x as u32))(inner))?
},

false => {
(Decoder31(_input))?
}
}))())?;
let construction_method = ((|| PResult::Ok(match version > 0u8 {
true => {
let inner = (Decoder44(_input))?;
Type97::yes(inner)
},

false => {
let _ = ();
Type97::no
}
}))())?;
let data_reference_index = ((|| PResult::Ok((Decoder44(_input))?))())?;
let base_offset = ((|| PResult::Ok(match base_offset_size {
0 => {
0u64
},

4 => {
let inner = (Decoder31(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8 => {
(Decoder50(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
let extent_count = ((|| PResult::Ok((Decoder44(_input))?))())?;
let extents = ((|| PResult::Ok({
let mut accum = Vec::new();
for _ in 0..extent_count {
accum.push({
let extent_index = ((|| PResult::Ok(match index_size {
0 => {
0u64
},

4 => {
let inner = (Decoder31(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8 => {
(Decoder50(_input))?
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
let inner = (Decoder31(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8 => {
(Decoder50(_input))?
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
let inner = (Decoder31(_input))?;
((|x: u32| PResult::Ok(x as u64))(inner))?
},

8 => {
(Decoder50(_input))?
},

_other => {
unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
}
}))())?;
Type98 { extent_index, extent_offset, extent_length }
});
}
accum
}))())?;
Type99 { item_ID, construction_method, data_reference_index, base_offset, extent_count, extents }
});
                                    }
                                    accum
                                })
                            })())?;
                            Type100 {
                                version,
                                flags,
                                offset_size_length_size,
                                base_offset_size_index_size,
                                offset_size,
                                length_size,
                                base_offset_size,
                                index_size,
                                item_count,
                                items,
                            }
                        };
                        Type114::iloc(inner)
                    }

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
                                    let next_elem = (Decoder62(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type114::ilst(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type114::idat(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type114::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type115 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder52<'input>(_input: &mut Parser<'input>) -> Result<Type160, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (109, 118, 104, 100) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let fields = ((|| {
                                PResult::Ok(match version {
                                    0 => {
                                        let inner = {
                                            let creation_time =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let modification_time =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let timescale =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let duration =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            Type116 {
                                                creation_time,
                                                modification_time,
                                                timescale,
                                                duration,
                                            }
                                        };
                                        Type118::version0(inner)
                                    }

                                    1 => {
                                        let inner = {
                                            let creation_time =
                                                ((|| PResult::Ok((Decoder50(_input))?))())?;
                                            let modification_time =
                                                ((|| PResult::Ok((Decoder50(_input))?))())?;
                                            let timescale =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let duration =
                                                ((|| PResult::Ok((Decoder50(_input))?))())?;
                                            Type117 {
                                                creation_time,
                                                modification_time,
                                                timescale,
                                                duration,
                                            }
                                        };
                                        Type118::version1(inner)
                                    }

                                    _other => {
                                        unreachable!(
                                            r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                                        );
                                    }
                                })
                            })())?;
                            let rate = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let volume = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let reserved1 = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let reserved2 = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder31(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder31(_input))?))())?;
                                    (field0, field1)
                                })
                            })())?;
                            let matrix = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..9u8 {
                                        accum.push((Decoder31(_input))?);
                                    }
                                    accum
                                })
                            })())?;
                            let pre_defined = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..6u8 {
                                        accum.push((Decoder31(_input))?);
                                    }
                                    accum
                                })
                            })())?;
                            let next_track_ID = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            Type119 {
                                version,
                                flags,
                                fields,
                                rate,
                                volume,
                                reserved1,
                                reserved2,
                                matrix,
                                pre_defined,
                                next_track_ID,
                            }
                        };
                        Type159::mvhd(inner)
                    }

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
                                    let next_elem = (Decoder53(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type159::trak(inner)
                    }

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
                                    let next_elem = (Decoder54(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type159::udta(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type159::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type160 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder53<'input>(_input: &mut Parser<'input>) -> Result<Type156, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (116, 107, 104, 100) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let fields = ((|| {
                                PResult::Ok(match version {
                                    0 => {
                                        let inner = {
                                            let creation_time =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let modification_time =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let track_ID =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let reserved =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let duration =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            Type151 {
                                                creation_time,
                                                modification_time,
                                                track_ID,
                                                reserved,
                                                duration,
                                            }
                                        };
                                        Type153::version0(inner)
                                    }

                                    1 => {
                                        let inner = {
                                            let creation_time =
                                                ((|| PResult::Ok((Decoder50(_input))?))())?;
                                            let modification_time =
                                                ((|| PResult::Ok((Decoder50(_input))?))())?;
                                            let track_ID =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let reserved =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let duration =
                                                ((|| PResult::Ok((Decoder50(_input))?))())?;
                                            Type152 {
                                                creation_time,
                                                modification_time,
                                                track_ID,
                                                reserved,
                                                duration,
                                            }
                                        };
                                        Type153::version1(inner)
                                    }

                                    _other => {
                                        unreachable!(
                                            r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                                        );
                                    }
                                })
                            })())?;
                            let reserved2 = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder31(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder31(_input))?))())?;
                                    (field0, field1)
                                })
                            })())?;
                            let layer = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let alternate_group = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let volume = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let reserved1 = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let matrix = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..9u8 {
                                        accum.push((Decoder31(_input))?);
                                    }
                                    accum
                                })
                            })())?;
                            let width = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let height = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            Type154 {
                                version,
                                flags,
                                fields,
                                reserved2,
                                layer,
                                alternate_group,
                                volume,
                                reserved1,
                                matrix,
                                width,
                                height,
                            }
                        };
                        Type155::tkhd(inner)
                    }

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
                                    let next_elem = (Decoder55(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type155::edts(inner)
                    }

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
                                    let next_elem = (Decoder56(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type155::mdia(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type155::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type156 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder54<'input>(_input: &mut Parser<'input>) -> Result<Type158, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (109, 101, 116, 97) => {
                        let field0 = ((|| PResult::Ok((Decoder31(_input))?))())?;
                        let field1 = ((|| {
                            PResult::Ok({
                                let mut accum = Vec::new();
                                while _input.remaining() > 0 {
                                    let matching_ix = {
                                        _input.open_peek_context();
                                        _input.read_byte()?;
                                        {
                                            let ret = 0;
                                            _input.close_peek_context()?;
                                            ret
                                        }
                                    };
                                    if matching_ix == 0 {
                                        let next_elem = (Decoder51(_input))?;
                                        accum.push(next_elem);
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            })
                        })())?;
                        Type157::meta(field0, field1)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type157::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type158 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder55<'input>(_input: &mut Parser<'input>) -> Result<Type123, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (101, 108, 115, 116) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let number_of_entries = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let edit_list_table = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..number_of_entries {
                                        accum.push({
                                            let track_duration =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let media_time =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let media_rate =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            Type120 {
                                                track_duration,
                                                media_time,
                                                media_rate,
                                            }
                                        });
                                    }
                                    accum
                                })
                            })())?;
                            Type121 {
                                version,
                                flags,
                                number_of_entries,
                                edit_list_table,
                            }
                        };
                        Type122::elst(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type122::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type123 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder56<'input>(_input: &mut Parser<'input>) -> Result<Type150, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (104, 100, 108, 114) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let component_type = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let component_subtype = ((|| PResult::Ok((Decoder49(_input))?))())?;
                            let component_manufacturer =
                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let component_flags = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let component_flags_mask = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let component_name = ((|| PResult::Ok((Decoder57(_input))?))())?;
                            Type124 {
                                version,
                                flags,
                                component_type,
                                component_subtype,
                                component_manufacturer,
                                component_flags,
                                component_flags_mask,
                                component_name,
                            }
                        };
                        Type149::hdlr(inner)
                    }

                    (109, 100, 104, 100) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let fields = ((|| {
                                PResult::Ok(match version {
                                    0 => {
                                        let inner = {
                                            let creation_time =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let modification_time =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let timescale =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let duration =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            Type116 {
                                                creation_time,
                                                modification_time,
                                                timescale,
                                                duration,
                                            }
                                        };
                                        Type118::version0(inner)
                                    }

                                    1 => {
                                        let inner = {
                                            let creation_time =
                                                ((|| PResult::Ok((Decoder50(_input))?))())?;
                                            let modification_time =
                                                ((|| PResult::Ok((Decoder50(_input))?))())?;
                                            let timescale =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let duration =
                                                ((|| PResult::Ok((Decoder50(_input))?))())?;
                                            Type117 {
                                                creation_time,
                                                modification_time,
                                                timescale,
                                                duration,
                                            }
                                        };
                                        Type118::version1(inner)
                                    }

                                    _other => {
                                        unreachable!(
                                            r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                                        );
                                    }
                                })
                            })())?;
                            let language = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let pre_defined = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            Type125 {
                                version,
                                flags,
                                fields,
                                language,
                                pre_defined,
                            }
                        };
                        Type149::mdhd(inner)
                    }

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
                                    let next_elem = (Decoder58(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type149::minf(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type149::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type150 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder57<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    PResult::Ok(Type21 { string, null })
}

fn Decoder58<'input>(_input: &mut Parser<'input>) -> Result<Type148, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (118, 109, 104, 100) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let graphicsmode = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let opcolor = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..3u8 {
                                        accum.push((Decoder44(_input))?);
                                    }
                                    accum
                                })
                            })())?;
                            Type146 {
                                version,
                                flags,
                                graphicsmode,
                                opcolor,
                            }
                        };
                        Type147::vmhd(inner)
                    }

                    (115, 109, 104, 100) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let balance = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            let reserved = ((|| PResult::Ok((Decoder44(_input))?))())?;
                            Type126 {
                                version,
                                flags,
                                balance,
                                reserved,
                            }
                        };
                        Type147::smhd(inner)
                    }

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
                                    let next_elem = (Decoder59(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type147::dinf(inner)
                    }

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
                                    let next_elem = (Decoder60(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type147::stbl(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type147::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type148 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder59<'input>(_input: &mut Parser<'input>) -> Result<Type85, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (100, 114, 101, 102) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let number_of_entries = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let data = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    while _input.remaining() > 0 {
                                        let matching_ix = {
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
                                                let size_field =
                                                    ((|| PResult::Ok((Decoder31(_input))?))())?;
                                                let r#type =
                                                    ((|| PResult::Ok((Decoder49(_input))?))())?;
                                                let size = ((|| {
                                                    PResult::Ok(match size_field {
                                                        0 => 0u64,

                                                        1 => {
                                                            let inner = (Decoder50(_input))?;
                                                            ((|x: u64| PResult::Ok(x - 16u64))(
                                                                inner,
                                                            ))?
                                                        }

                                                        _ => (size_field - 8u32) as u64,
                                                    })
                                                })(
                                                ))?;
                                                let data = ((|| {
                                                    PResult::Ok({
                                                        let sz = size as usize;
                                                        _input.start_slice(sz)?;
                                                        let ret = ((|| {
                                                            PResult::Ok({
                                                                let mut accum = Vec::new();
                                                                while _input.remaining() > 0 {
                                                                    let matching_ix = {
                                                                        _input.open_peek_context();
                                                                        _input.read_byte()?;
                                                                        {
                                                                            let ret = 0;
                                                                            _input
                                                                                .close_peek_context(
                                                                                )?;
                                                                            ret
                                                                        }
                                                                    };
                                                                    if matching_ix == 0 {
                                                                        let next_elem =
                                                                            (Decoder15(_input))?;
                                                                        accum.push(next_elem);
                                                                    } else {
                                                                        break;
                                                                    }
                                                                }
                                                                accum
                                                            })
                                                        })(
                                                        ))?;
                                                        _input.end_slice()?;
                                                        ret
                                                    })
                                                })(
                                                ))?;
                                                Type82 {
                                                    size_field,
                                                    r#type,
                                                    size,
                                                    data,
                                                }
                                            };
                                            accum.push(next_elem);
                                        } else {
                                            break;
                                        }
                                    }
                                    accum
                                })
                            })())?;
                            Type83 {
                                version,
                                flags,
                                number_of_entries,
                                data,
                            }
                        };
                        Type84::dref(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type84::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type85 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder60<'input>(_input: &mut Parser<'input>) -> Result<Type145, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (115, 116, 115, 100) => {
                        let inner =
                            {
                                let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                        let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                        let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let entry_count = ((|| PResult::Ok((Decoder31(_input))?))())?;
                                let sample_entries =
                                    ((|| {
                                        PResult::Ok({
                                            let mut accum = Vec::new();
                                            for _ in 0..entry_count {
                                                accum.push({
let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
let size = ((|| PResult::Ok(match size_field {
0 => {
0u64
},

1 => {
let inner = (Decoder50(_input))?;
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
let next_elem = (Decoder15(_input))?;
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
Type82 { size_field, r#type, size, data }
});
                                            }
                                            accum
                                        })
                                    })())?;
                                Type138 {
                                    version,
                                    flags,
                                    entry_count,
                                    sample_entries,
                                }
                            };
                        Type144::stsd(inner)
                    }

                    (115, 116, 116, 115) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let entry_count = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let sample_entries = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..entry_count {
                                        accum.push({
                                            let sample_count =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let sample_delta =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            Type142 {
                                                sample_count,
                                                sample_delta,
                                            }
                                        });
                                    }
                                    accum
                                })
                            })())?;
                            Type143 {
                                version,
                                flags,
                                entry_count,
                                sample_entries,
                            }
                        };
                        Type144::stts(inner)
                    }

                    (99, 116, 116, 115) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let entry_count = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let sample_entries = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..entry_count {
                                        accum.push({
                                            let sample_count =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let sample_offset =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            Type128 {
                                                sample_count,
                                                sample_offset,
                                            }
                                        });
                                    }
                                    accum
                                })
                            })())?;
                            Type129 {
                                version,
                                flags,
                                entry_count,
                                sample_entries,
                            }
                        };
                        Type144::ctts(inner)
                    }

                    (115, 116, 115, 115) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let entry_count = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let sample_number = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..entry_count {
                                        accum.push((Decoder31(_input))?);
                                    }
                                    accum
                                })
                            })())?;
                            Type139 {
                                version,
                                flags,
                                entry_count,
                                sample_number,
                            }
                        };
                        Type144::stss(inner)
                    }

                    (115, 116, 115, 99) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let entry_count = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let chunk_entries = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..entry_count {
                                        accum.push({
                                            let first_chunk =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let samples_per_chunk =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let sample_description_index =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            Type136 {
                                                first_chunk,
                                                samples_per_chunk,
                                                sample_description_index,
                                            }
                                        });
                                    }
                                    accum
                                })
                            })())?;
                            Type137 {
                                version,
                                flags,
                                entry_count,
                                chunk_entries,
                            }
                        };
                        Type144::stsc(inner)
                    }

                    (115, 116, 115, 122) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let sample_size = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let sample_count = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let entry_size = ((|| {
                                PResult::Ok(match sample_size == 0u32 {
                                    true => {
                                        let inner = {
                                            let mut accum = Vec::new();
                                            for _ in 0..sample_count {
                                                accum.push((Decoder31(_input))?);
                                            }
                                            accum
                                        };
                                        Type140::yes(inner)
                                    }

                                    false => {
                                        let _ = ();
                                        Type140::no
                                    }
                                })
                            })())?;
                            Type141 {
                                version,
                                flags,
                                sample_size,
                                sample_count,
                                entry_size,
                            }
                        };
                        Type144::stsz(inner)
                    }

                    (115, 116, 99, 111) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let entry_count = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let chunk_offset = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..entry_count {
                                        accum.push((Decoder31(_input))?);
                                    }
                                    accum
                                })
                            })())?;
                            Type135 {
                                version,
                                flags,
                                entry_count,
                                chunk_offset,
                            }
                        };
                        Type144::stco(inner)
                    }

                    (99, 111, 54, 52) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let entry_count = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let chunk_offset = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..entry_count {
                                        accum.push((Decoder50(_input))?);
                                    }
                                    accum
                                })
                            })())?;
                            Type127 {
                                version,
                                flags,
                                entry_count,
                                chunk_offset,
                            }
                        };
                        Type144::co64(inner)
                    }

                    (115, 103, 112, 100) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let grouping_type = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let default_length = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let entry_count = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let sample_groups = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..entry_count {
                                        accum.push({
                                            let description_length = ((|| {
                                                PResult::Ok(match default_length == 0u32 {
                                                    true => (Decoder31(_input))?,

                                                    false => default_length.clone(),
                                                })
                                            })(
                                            ))?;
                                            let sample_group_entry = ((|| {
                                                PResult::Ok({
                                                    let mut accum = Vec::new();
                                                    for _ in 0..description_length {
                                                        accum.push((Decoder15(_input))?);
                                                    }
                                                    accum
                                                })
                                            })(
                                            ))?;
                                            Type133 {
                                                description_length,
                                                sample_group_entry,
                                            }
                                        });
                                    }
                                    accum
                                })
                            })())?;
                            Type134 {
                                version,
                                flags,
                                grouping_type,
                                default_length,
                                entry_count,
                                sample_groups,
                            }
                        };
                        Type144::sgpd(inner)
                    }

                    (115, 98, 103, 112) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let grouping_type = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let grouping_type_parameter = ((|| {
                                PResult::Ok(match version == 1u8 {
                                    true => {
                                        let inner = (Decoder31(_input))?;
                                        Type130::yes(inner)
                                    }

                                    false => {
                                        let _ = ();
                                        Type130::no
                                    }
                                })
                            })())?;
                            let entry_count = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let sample_groups = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    for _ in 0..entry_count {
                                        accum.push({
                                            let sample_count =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            let group_description_index =
                                                ((|| PResult::Ok((Decoder31(_input))?))())?;
                                            Type131 {
                                                sample_count,
                                                group_description_index,
                                            }
                                        });
                                    }
                                    accum
                                })
                            })())?;
                            Type132 {
                                version,
                                flags,
                                grouping_type,
                                grouping_type_parameter,
                                entry_count,
                                sample_groups,
                            }
                        };
                        Type144::sbgp(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type144::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type145 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder61<'input>(_input: &mut Parser<'input>) -> Result<Type95, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (105, 110, 102, 101) => {
                        let inner = {
                            let version = ((|| PResult::Ok((Decoder15(_input))?))())?;
                            let flags = ((|| {
                                PResult::Ok({
                                    let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder15(_input))?))())?;
                                    (field0, field1, field2)
                                })
                            })())?;
                            let fields = ((|| {
                                PResult::Ok(match version < 2u8 {
                                    true => {
                                        let inner = {
                                            let item_ID =
                                                ((|| PResult::Ok((Decoder44(_input))?))())?;
                                            let item_protection_index =
                                                ((|| PResult::Ok((Decoder44(_input))?))())?;
                                            let item_name =
                                                ((|| PResult::Ok((Decoder64(_input))?))())?;
                                            let content_type =
                                                ((|| PResult::Ok((Decoder65(_input))?))())?;
                                            let content_encoding =
                                                ((|| PResult::Ok((Decoder66(_input))?))())?;
                                            Type91 {
                                                item_ID,
                                                item_protection_index,
                                                item_name,
                                                content_type,
                                                content_encoding,
                                            }
                                        };
                                        Type92::yes(inner)
                                    }

                                    false => {
                                        let inner = {
                                            let item_ID = ((|| {
                                                PResult::Ok(match version == 2u8 {
                                                    true => {
                                                        let inner = (Decoder44(_input))?;
                                                        ((|x: u16| PResult::Ok(x as u32))(inner))?
                                                    }

                                                    false => (Decoder31(_input))?,
                                                })
                                            })(
                                            ))?;
                                            let item_protection_index =
                                                ((|| PResult::Ok((Decoder44(_input))?))())?;
                                            let item_type =
                                                ((|| PResult::Ok((Decoder49(_input))?))())?;
                                            let item_name =
                                                ((|| PResult::Ok((Decoder67(_input))?))())?;
                                            let extra_fields = ((|| {
                                                PResult::Ok(match item_type {
                                                    (109, 105, 109, 101) => {
                                                        let inner = {
                                                            let content_type = ((|| {
                                                                PResult::Ok((Decoder68(_input))?)
                                                            })(
                                                            ))?;
                                                            Type87 { content_type }
                                                        };
                                                        Type89::mime(inner)
                                                    }

                                                    (117, 114, 105, 32) => {
                                                        let inner = {
                                                            let item_uri_type = ((|| {
                                                                PResult::Ok((Decoder68(_input))?)
                                                            })(
                                                            ))?;
                                                            Type88 { item_uri_type }
                                                        };
                                                        Type89::uri(inner)
                                                    }

                                                    _ => {
                                                        let _ = ();
                                                        Type89::unknown
                                                    }
                                                })
                                            })(
                                            ))?;
                                            Type90 {
                                                item_ID,
                                                item_protection_index,
                                                item_type,
                                                item_name,
                                                extra_fields,
                                            }
                                        };
                                        Type92::no(inner)
                                    }
                                })
                            })())?;
                            Type93 {
                                version,
                                flags,
                                fields,
                            }
                        };
                        Type94::infe(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type94::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type95 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder62<'input>(_input: &mut Parser<'input>) -> Result<Type105, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
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
                                    let next_elem = (Decoder63(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type104::tool(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type104::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type105 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder63<'input>(_input: &mut Parser<'input>) -> Result<Type103, ParseError> {
    let size_field = ((|| PResult::Ok((Decoder31(_input))?))())?;
    let r#type = ((|| PResult::Ok((Decoder49(_input))?))())?;
    let size = ((|| {
        PResult::Ok(match size_field {
            0 => 0u64,

            1 => {
                let inner = (Decoder50(_input))?;
                ((|x: u64| PResult::Ok(x - 16u64))(inner))?
            }

            _ => (size_field - 8u32) as u64,
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = size as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok(match r#type {
                    (100, 97, 116, 97) => {
                        let inner = {
                            let type_indicator = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let locale_indicator = ((|| PResult::Ok((Decoder31(_input))?))())?;
                            let value = ((|| {
                                PResult::Ok({
                                    let mut accum = Vec::new();
                                    while _input.remaining() > 0 {
                                        let matching_ix = {
                                            _input.open_peek_context();
                                            _input.read_byte()?;
                                            {
                                                let ret = 0;
                                                _input.close_peek_context()?;
                                                ret
                                            }
                                        };
                                        if matching_ix == 0 {
                                            let next_elem = (Decoder19(_input))?;
                                            accum.push(next_elem);
                                        } else {
                                            break;
                                        }
                                    }
                                    accum
                                })
                            })())?;
                            Type101 {
                                type_indicator,
                                locale_indicator,
                                value,
                            }
                        };
                        Type102::data(inner)
                    }

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
                                    let next_elem = (Decoder15(_input))?;
                                    accum.push(next_elem);
                                } else {
                                    break;
                                }
                            }
                            accum
                        };
                        Type102::unknown(inner)
                    }
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type103 {
        size_field,
        r#type,
        size,
        data,
    })
}

fn Decoder64<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    PResult::Ok(Type21 { string, null })
}

fn Decoder65<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    PResult::Ok(Type21 { string, null })
}

fn Decoder66<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    PResult::Ok(Type21 { string, null })
}

fn Decoder67<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    PResult::Ok(Type21 { string, null })
}

fn Decoder68<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    PResult::Ok(Type21 { string, null })
}

fn Decoder69<'input>(_input: &mut Parser<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 216 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(5637435011420551755u64));
            }
        })
    })())?;
    PResult::Ok(Type41 { ff, marker })
}

fn Decoder70<'input>(_input: &mut Parser<'input>) -> Result<Type79, ParseError> {
    let initial_segment = ((|| {
        PResult::Ok({
            let tree_index = {
                _input.open_peek_context();
                let b = _input.read_byte()?;
                {
                    let ret = if b == 255 {
                        let b = _input.read_byte()?;
                        match b {
                            224u8 => 0,

                            225u8 => 1,

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
                    let inner = (Decoder72(_input))?;
                    Type55::app0(inner)
                }

                1 => {
                    let inner = (Decoder73(_input))?;
                    Type55::app1(inner)
                }

                _ => {
                    return Err(ParseError::ExcludedBranch(3642042507085222192u64));
                }
            }
        })
    })())?;
    let segments = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = if b == 255 {
                            let b = _input.read_byte()?;
                            match b {
                                219u8 => 0,

                                196u8 => 0,

                                204u8 => 0,

                                221u8 => 0,

                                224u8 => 0,

                                225u8 => 0,

                                226u8 => 0,

                                227u8 => 0,

                                228u8 => 0,

                                229u8 => 0,

                                230u8 => 0,

                                231u8 => 0,

                                232u8 => 0,

                                233u8 => 0,

                                234u8 => 0,

                                235u8 => 0,

                                236u8 => 0,

                                237u8 => 0,

                                238u8 => 0,

                                239u8 => 0,

                                254u8 => 0,

                                192u8 => 1,

                                193u8 => 1,

                                194u8 => 1,

                                195u8 => 1,

                                197u8 => 1,

                                198u8 => 1,

                                199u8 => 1,

                                201u8 => 1,

                                202u8 => 1,

                                203u8 => 1,

                                205u8 => 1,

                                206u8 => 1,

                                207u8 => 1,

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
                    let next_elem = (Decoder74(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let header = ((|| PResult::Ok((Decoder75(_input))?))())?;
    let scan = ((|| PResult::Ok((Decoder76(_input))?))())?;
    let dnl = ((|| {
        PResult::Ok({
            let tree_index = {
                _input.open_peek_context();
                let b = _input.read_byte()?;
                {
                    let ret = if b == 255 {
                        let b = _input.read_byte()?;
                        match b {
                            220u8 => 0,

                            217u8 => 1,

                            218u8 => 1,

                            219u8 => 1,

                            196u8 => 1,

                            204u8 => 1,

                            221u8 => 1,

                            224u8 => 1,

                            225u8 => 1,

                            226u8 => 1,

                            227u8 => 1,

                            228u8 => 1,

                            229u8 => 1,

                            230u8 => 1,

                            231u8 => 1,

                            232u8 => 1,

                            233u8 => 1,

                            234u8 => 1,

                            235u8 => 1,

                            236u8 => 1,

                            237u8 => 1,

                            238u8 => 1,

                            239u8 => 1,

                            254u8 => 1,

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
                    let inner = (Decoder77(_input))?;
                    Type78::some(inner)
                }

                1 => {
                    let _ = ();
                    Type78::none
                }

                _ => {
                    return Err(ParseError::ExcludedBranch(11678103101816798445u64));
                }
            }
        })
    })())?;
    let scans = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = if b == 255 {
                            let b = _input.read_byte()?;
                            match b {
                                218u8 => 0,

                                219u8 => 0,

                                196u8 => 0,

                                204u8 => 0,

                                221u8 => 0,

                                224u8 => 0,

                                225u8 => 0,

                                226u8 => 0,

                                227u8 => 0,

                                228u8 => 0,

                                229u8 => 0,

                                230u8 => 0,

                                231u8 => 0,

                                232u8 => 0,

                                233u8 => 0,

                                234u8 => 0,

                                235u8 => 0,

                                236u8 => 0,

                                237u8 => 0,

                                238u8 => 0,

                                239u8 => 0,

                                254u8 => 0,

                                217u8 => 1,

                                _ => {
                                    return Err(ParseError::ExcludedBranch(
                                        18361368374853160051u64,
                                    ));
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
                    let next_elem = (Decoder78(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    PResult::Ok(Type79 {
        initial_segment,
        segments,
        header,
        scan,
        dnl,
        scans,
    })
}

fn Decoder71<'input>(_input: &mut Parser<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 217 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(16574347298383600551u64));
            }
        })
    })())?;
    PResult::Ok(Type41 { ff, marker })
}

fn Decoder72<'input>(_input: &mut Parser<'input>) -> Result<Type45, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 224 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(5346911683359312959u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder138(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type45 {
        marker,
        length,
        data,
    })
}

fn Decoder73<'input>(_input: &mut Parser<'input>) -> Result<Type54, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 225 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(301524255299452508u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder132(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type54 {
        marker,
        length,
        data,
    })
}

fn Decoder74<'input>(_input: &mut Parser<'input>) -> Result<Type65, ParseError> {
    let tree_index = {
        _input.open_peek_context();
        let b = _input.read_byte()?;
        {
            let ret = if b == 255 {
                let b = _input.read_byte()?;
                match b {
                    219u8 => 0,

                    196u8 => 1,

                    204u8 => 2,

                    221u8 => 3,

                    224u8 => 4,

                    225u8 => 5,

                    226u8 => 6,

                    227u8 => 7,

                    228u8 => 8,

                    229u8 => 9,

                    230u8 => 10,

                    231u8 => 11,

                    232u8 => 12,

                    233u8 => 13,

                    234u8 => 14,

                    235u8 => 15,

                    236u8 => 16,

                    237u8 => 17,

                    238u8 => 18,

                    239u8 => 19,

                    254u8 => 20,

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
            let inner = (Decoder109(_input))?;
            Type65::dqt(inner)
        }

        1 => {
            let inner = (Decoder110(_input))?;
            Type65::dht(inner)
        }

        2 => {
            let inner = (Decoder111(_input))?;
            Type65::dac(inner)
        }

        3 => {
            let inner = (Decoder112(_input))?;
            Type65::dri(inner)
        }

        4 => {
            let inner = (Decoder72(_input))?;
            Type65::app0(inner)
        }

        5 => {
            let inner = (Decoder73(_input))?;
            Type65::app1(inner)
        }

        6 => {
            let inner = (Decoder113(_input))?;
            Type65::app2(inner)
        }

        7 => {
            let inner = (Decoder114(_input))?;
            Type65::app3(inner)
        }

        8 => {
            let inner = (Decoder115(_input))?;
            Type65::app4(inner)
        }

        9 => {
            let inner = (Decoder116(_input))?;
            Type65::app5(inner)
        }

        10 => {
            let inner = (Decoder117(_input))?;
            Type65::app6(inner)
        }

        11 => {
            let inner = (Decoder118(_input))?;
            Type65::app7(inner)
        }

        12 => {
            let inner = (Decoder119(_input))?;
            Type65::app8(inner)
        }

        13 => {
            let inner = (Decoder120(_input))?;
            Type65::app9(inner)
        }

        14 => {
            let inner = (Decoder121(_input))?;
            Type65::app10(inner)
        }

        15 => {
            let inner = (Decoder122(_input))?;
            Type65::app11(inner)
        }

        16 => {
            let inner = (Decoder123(_input))?;
            Type65::app12(inner)
        }

        17 => {
            let inner = (Decoder124(_input))?;
            Type65::app13(inner)
        }

        18 => {
            let inner = (Decoder125(_input))?;
            Type65::app14(inner)
        }

        19 => {
            let inner = (Decoder126(_input))?;
            Type65::app15(inner)
        }

        20 => {
            let inner = (Decoder127(_input))?;
            Type65::com(inner)
        }

        _ => {
            return Err(ParseError::ExcludedBranch(5858366816005674364u64));
        }
    })
}

fn Decoder75<'input>(_input: &mut Parser<'input>) -> Result<Type69, ParseError> {
    let tree_index = {
        _input.open_peek_context();
        let b = _input.read_byte()?;
        {
            let ret = if b == 255 {
                let b = _input.read_byte()?;
                match b {
                    192u8 => 0,

                    193u8 => 1,

                    194u8 => 2,

                    195u8 => 3,

                    197u8 => 4,

                    198u8 => 5,

                    199u8 => 6,

                    201u8 => 7,

                    202u8 => 8,

                    203u8 => 9,

                    205u8 => 10,

                    206u8 => 11,

                    207u8 => 12,

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
            let inner = (Decoder94(_input))?;
            Type69::sof0(inner)
        }

        1 => {
            let inner = (Decoder95(_input))?;
            Type69::sof1(inner)
        }

        2 => {
            let inner = (Decoder96(_input))?;
            Type69::sof2(inner)
        }

        3 => {
            let inner = (Decoder97(_input))?;
            Type69::sof3(inner)
        }

        4 => {
            let inner = (Decoder98(_input))?;
            Type69::sof5(inner)
        }

        5 => {
            let inner = (Decoder99(_input))?;
            Type69::sof6(inner)
        }

        6 => {
            let inner = (Decoder100(_input))?;
            Type69::sof7(inner)
        }

        7 => {
            let inner = (Decoder101(_input))?;
            Type69::sof9(inner)
        }

        8 => {
            let inner = (Decoder102(_input))?;
            Type69::sof10(inner)
        }

        9 => {
            let inner = (Decoder103(_input))?;
            Type69::sof11(inner)
        }

        10 => {
            let inner = (Decoder104(_input))?;
            Type69::sof13(inner)
        }

        11 => {
            let inner = (Decoder105(_input))?;
            Type69::sof14(inner)
        }

        12 => {
            let inner = (Decoder106(_input))?;
            Type69::sof15(inner)
        }

        _ => {
            return Err(ParseError::ExcludedBranch(13734934310846663046u64));
        }
    })
}

fn Decoder76<'input>(_input: &mut Parser<'input>) -> Result<Type75, ParseError> {
    let segments = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = if b == 255 {
                            let b = _input.read_byte()?;
                            match b {
                                219u8 => 0,

                                196u8 => 0,

                                204u8 => 0,

                                221u8 => 0,

                                224u8 => 0,

                                225u8 => 0,

                                226u8 => 0,

                                227u8 => 0,

                                228u8 => 0,

                                229u8 => 0,

                                230u8 => 0,

                                231u8 => 0,

                                232u8 => 0,

                                233u8 => 0,

                                234u8 => 0,

                                235u8 => 0,

                                236u8 => 0,

                                237u8 => 0,

                                238u8 => 0,

                                239u8 => 0,

                                254u8 => 0,

                                218u8 => 1,

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
                    let next_elem = (Decoder74(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let sos = ((|| PResult::Ok((Decoder79(_input))?))())?;
    let data = ((|| PResult::Ok((Decoder93(_input))?))())?;
    PResult::Ok(Type75 {
        segments,
        sos,
        data,
    })
}

fn Decoder77<'input>(_input: &mut Parser<'input>) -> Result<Type77, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 220 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(2912073318189654678u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder92(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type77 {
        marker,
        length,
        data,
    })
}

fn Decoder78<'input>(_input: &mut Parser<'input>) -> Result<Type75, ParseError> {
    let segments = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = if b == 255 {
                            let b = _input.read_byte()?;
                            match b {
                                219u8 => 0,

                                196u8 => 0,

                                204u8 => 0,

                                221u8 => 0,

                                224u8 => 0,

                                225u8 => 0,

                                226u8 => 0,

                                227u8 => 0,

                                228u8 => 0,

                                229u8 => 0,

                                230u8 => 0,

                                231u8 => 0,

                                232u8 => 0,

                                233u8 => 0,

                                234u8 => 0,

                                235u8 => 0,

                                236u8 => 0,

                                237u8 => 0,

                                238u8 => 0,

                                239u8 => 0,

                                254u8 => 0,

                                218u8 => 1,

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
                    let next_elem = (Decoder74(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let sos = ((|| PResult::Ok((Decoder79(_input))?))())?;
    let data = ((|| PResult::Ok((Decoder80(_input))?))())?;
    PResult::Ok(Type75 {
        segments,
        sos,
        data,
    })
}

fn Decoder79<'input>(_input: &mut Parser<'input>) -> Result<Type72, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 218 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(5297104498937034880u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder90(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type72 {
        marker,
        length,
        data,
    })
}

fn Decoder80<'input>(_input: &mut Parser<'input>) -> Result<Type74, ParseError> {
    let scan_data = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 255) => 0,

                            255u8 => {
                                let b = _input.read_byte()?;
                                match b {
                                    0u8 => 0,

                                    208u8 => 0,

                                    209u8 => 0,

                                    210u8 => 0,

                                    211u8 => 0,

                                    212u8 => 0,

                                    213u8 => 0,

                                    214u8 => 0,

                                    215u8 => 0,

                                    217u8 => 1,

                                    218u8 => 1,

                                    219u8 => 1,

                                    196u8 => 1,

                                    204u8 => 1,

                                    221u8 => 1,

                                    224u8 => 1,

                                    225u8 => 1,

                                    226u8 => 1,

                                    227u8 => 1,

                                    228u8 => 1,

                                    229u8 => 1,

                                    230u8 => 1,

                                    231u8 => 1,

                                    232u8 => 1,

                                    233u8 => 1,

                                    234u8 => 1,

                                    235u8 => 1,

                                    236u8 => 1,

                                    237u8 => 1,

                                    238u8 => 1,

                                    239u8 => 1,

                                    254u8 => 1,

                                    _ => {
                                        return Err(ParseError::ExcludedBranch(
                                            9445433320207076674u64,
                                        ));
                                    }
                                }
                            }

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
                                    tmp if (tmp != 255) => 0,

                                    255u8 => {
                                        let b = _input.read_byte()?;
                                        match b {
                                            0u8 => 0,

                                            208u8 => 1,

                                            209u8 => 2,

                                            210u8 => 3,

                                            211u8 => 4,

                                            212u8 => 5,

                                            213u8 => 6,

                                            214u8 => 7,

                                            215u8 => 8,

                                            _ => {
                                                return Err(ParseError::ExcludedBranch(
                                                    2047945967620228231u64,
                                                ));
                                            }
                                        }
                                    }

                                    _ => {
                                        return Err(ParseError::ExcludedBranch(
                                            3966792236320797464u64,
                                        ));
                                    }
                                };
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        match tree_index {
                            0 => {
                                let inner = (Decoder81(_input))?;
                                Type73::mcu(inner)
                            }

                            1 => {
                                let inner = (Decoder82(_input))?;
                                Type73::rst0(inner)
                            }

                            2 => {
                                let inner = (Decoder83(_input))?;
                                Type73::rst1(inner)
                            }

                            3 => {
                                let inner = (Decoder84(_input))?;
                                Type73::rst2(inner)
                            }

                            4 => {
                                let inner = (Decoder85(_input))?;
                                Type73::rst3(inner)
                            }

                            5 => {
                                let inner = (Decoder86(_input))?;
                                Type73::rst4(inner)
                            }

                            6 => {
                                let inner = (Decoder87(_input))?;
                                Type73::rst5(inner)
                            }

                            7 => {
                                let inner = (Decoder88(_input))?;
                                Type73::rst6(inner)
                            }

                            8 => {
                                let inner = (Decoder89(_input))?;
                                Type73::rst7(inner)
                            }

                            _ => {
                                return Err(ParseError::ExcludedBranch(16335009692206494675u64));
                            }
                        }
                    };
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let scan_data_stream = ((|| {
        PResult::Ok(
            (try_flat_map_vec(scan_data.iter().cloned(), |x: Type73| {
                PResult::Ok(match x {
                    Type73::mcu(v) => [v.clone()].to_vec(),

                    Type73::rst0(..) => [].to_vec(),

                    Type73::rst1(..) => [].to_vec(),

                    Type73::rst2(..) => [].to_vec(),

                    Type73::rst3(..) => [].to_vec(),

                    Type73::rst4(..) => [].to_vec(),

                    Type73::rst5(..) => [].to_vec(),

                    Type73::rst6(..) => [].to_vec(),

                    Type73::rst7(..) => [].to_vec(),
                })
            }))?,
        )
    })())?;
    PResult::Ok(Type74 {
        scan_data,
        scan_data_stream,
    })
}

fn Decoder81<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
    let tree_index = {
        _input.open_peek_context();
        let b = _input.read_byte()?;
        {
            let ret = match b {
                tmp if (tmp != 255) => 0,

                255u8 => 1,

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
        }

        1 => {
            let inner = {
                let field0 = ((|| {
                    PResult::Ok({
                        let b = _input.read_byte()?;
                        if b == 255 {
                            b
                        } else {
                            return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                        }
                    })
                })())?;
                let field1 = ((|| {
                    PResult::Ok({
                        let b = _input.read_byte()?;
                        if b == 0 {
                            b
                        } else {
                            return Err(ParseError::ExcludedBranch(10396965092922267801u64));
                        }
                    })
                })())?;
                (field0, field1)
            };
            ((|_: (u8, u8)| PResult::Ok(255u8))(inner))?
        }

        _ => {
            return Err(ParseError::ExcludedBranch(4297833600800538456u64));
        }
    })
}

fn Decoder82<'input>(_input: &mut Parser<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 208 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(5421268784727520761u64));
            }
        })
    })())?;
    PResult::Ok(Type41 { ff, marker })
}

fn Decoder83<'input>(_input: &mut Parser<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 209 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10069632627653602280u64));
            }
        })
    })())?;
    PResult::Ok(Type41 { ff, marker })
}

fn Decoder84<'input>(_input: &mut Parser<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 210 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(7941505592535629367u64));
            }
        })
    })())?;
    PResult::Ok(Type41 { ff, marker })
}

fn Decoder85<'input>(_input: &mut Parser<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 211 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(4842764822111760355u64));
            }
        })
    })())?;
    PResult::Ok(Type41 { ff, marker })
}

fn Decoder86<'input>(_input: &mut Parser<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 212 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(172561454190383201u64));
            }
        })
    })())?;
    PResult::Ok(Type41 { ff, marker })
}

fn Decoder87<'input>(_input: &mut Parser<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 213 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(12052389963453405046u64));
            }
        })
    })())?;
    PResult::Ok(Type41 { ff, marker })
}

fn Decoder88<'input>(_input: &mut Parser<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 214 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(14545630498792155294u64));
            }
        })
    })())?;
    PResult::Ok(Type41 { ff, marker })
}

fn Decoder89<'input>(_input: &mut Parser<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 215 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10573988543901039080u64));
            }
        })
    })())?;
    PResult::Ok(Type41 { ff, marker })
}

fn Decoder90<'input>(_input: &mut Parser<'input>) -> Result<Type71, ParseError> {
    let num_image_components = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let image_components = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..num_image_components {
                accum.push((Decoder91(_input))?);
            }
            accum
        })
    })())?;
    let start_spectral_selection = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let end_spectral_selection = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let approximation_bit_position = ((|| PResult::Ok((Decoder15(_input))?))())?;
    PResult::Ok(Type71 {
        num_image_components,
        image_components,
        start_spectral_selection,
        end_spectral_selection,
        approximation_bit_position,
    })
}

fn Decoder91<'input>(_input: &mut Parser<'input>) -> Result<Type70, ParseError> {
    let component_selector = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let entropy_coding_table_ids = ((|| PResult::Ok((Decoder15(_input))?))())?;
    PResult::Ok(Type70 {
        component_selector,
        entropy_coding_table_ids,
    })
}

fn Decoder92<'input>(_input: &mut Parser<'input>) -> Result<Type76, ParseError> {
    let num_lines = ((|| PResult::Ok((Decoder44(_input))?))())?;
    PResult::Ok(Type76 { num_lines })
}

fn Decoder93<'input>(_input: &mut Parser<'input>) -> Result<Type74, ParseError> {
    let scan_data = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 255) => 0,

                            255u8 => {
                                let b = _input.read_byte()?;
                                match b {
                                    0u8 => 0,

                                    208u8 => 0,

                                    209u8 => 0,

                                    210u8 => 0,

                                    211u8 => 0,

                                    212u8 => 0,

                                    213u8 => 0,

                                    214u8 => 0,

                                    215u8 => 0,

                                    220u8 => 1,

                                    217u8 => 1,

                                    218u8 => 1,

                                    219u8 => 1,

                                    196u8 => 1,

                                    204u8 => 1,

                                    221u8 => 1,

                                    224u8 => 1,

                                    225u8 => 1,

                                    226u8 => 1,

                                    227u8 => 1,

                                    228u8 => 1,

                                    229u8 => 1,

                                    230u8 => 1,

                                    231u8 => 1,

                                    232u8 => 1,

                                    233u8 => 1,

                                    234u8 => 1,

                                    235u8 => 1,

                                    236u8 => 1,

                                    237u8 => 1,

                                    238u8 => 1,

                                    239u8 => 1,

                                    254u8 => 1,

                                    _ => {
                                        return Err(ParseError::ExcludedBranch(
                                            9741508811552252074u64,
                                        ));
                                    }
                                }
                            }

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
                                    tmp if (tmp != 255) => 0,

                                    255u8 => {
                                        let b = _input.read_byte()?;
                                        match b {
                                            0u8 => 0,

                                            208u8 => 1,

                                            209u8 => 2,

                                            210u8 => 3,

                                            211u8 => 4,

                                            212u8 => 5,

                                            213u8 => 6,

                                            214u8 => 7,

                                            215u8 => 8,

                                            _ => {
                                                return Err(ParseError::ExcludedBranch(
                                                    2047945967620228231u64,
                                                ));
                                            }
                                        }
                                    }

                                    _ => {
                                        return Err(ParseError::ExcludedBranch(
                                            3966792236320797464u64,
                                        ));
                                    }
                                };
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        match tree_index {
                            0 => {
                                let inner = (Decoder81(_input))?;
                                Type73::mcu(inner)
                            }

                            1 => {
                                let inner = (Decoder82(_input))?;
                                Type73::rst0(inner)
                            }

                            2 => {
                                let inner = (Decoder83(_input))?;
                                Type73::rst1(inner)
                            }

                            3 => {
                                let inner = (Decoder84(_input))?;
                                Type73::rst2(inner)
                            }

                            4 => {
                                let inner = (Decoder85(_input))?;
                                Type73::rst3(inner)
                            }

                            5 => {
                                let inner = (Decoder86(_input))?;
                                Type73::rst4(inner)
                            }

                            6 => {
                                let inner = (Decoder87(_input))?;
                                Type73::rst5(inner)
                            }

                            7 => {
                                let inner = (Decoder88(_input))?;
                                Type73::rst6(inner)
                            }

                            8 => {
                                let inner = (Decoder89(_input))?;
                                Type73::rst7(inner)
                            }

                            _ => {
                                return Err(ParseError::ExcludedBranch(16335009692206494675u64));
                            }
                        }
                    };
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let scan_data_stream = ((|| {
        PResult::Ok(
            (try_flat_map_vec(scan_data.iter().cloned(), |x: Type73| {
                PResult::Ok(match x {
                    Type73::mcu(v) => [v.clone()].to_vec(),

                    Type73::rst0(..) => [].to_vec(),

                    Type73::rst1(..) => [].to_vec(),

                    Type73::rst2(..) => [].to_vec(),

                    Type73::rst3(..) => [].to_vec(),

                    Type73::rst4(..) => [].to_vec(),

                    Type73::rst5(..) => [].to_vec(),

                    Type73::rst6(..) => [].to_vec(),

                    Type73::rst7(..) => [].to_vec(),
                })
            }))?,
        )
    })())?;
    PResult::Ok(Type74 {
        scan_data,
        scan_data_stream,
    })
}

fn Decoder94<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 192 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(8297024098414101254u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder95<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 193 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(8756819601933520429u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder96<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 194 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(11080817261996913520u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder97<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 195 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(12909450577628061793u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder98<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 197 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(5274098556056955310u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder99<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 198 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(5472222913557901985u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder100<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 199 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(935456091642960999u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder101<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 201 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(17091795488609854789u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder102<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 202 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(14420220630934832328u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder103<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 203 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10502663948806018262u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder104<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 205 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(5170411260421882161u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder105<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 206 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(8862644040087288472u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder106<'input>(_input: &mut Parser<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 207 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(6282714738219454149u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder107(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type68 {
        marker,
        length,
        data,
    })
}

fn Decoder107<'input>(_input: &mut Parser<'input>) -> Result<Type67, ParseError> {
    let sample_precision = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let num_lines = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let num_samples_per_line = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let num_image_components = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let image_components = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..num_image_components {
                accum.push((Decoder108(_input))?);
            }
            accum
        })
    })())?;
    PResult::Ok(Type67 {
        sample_precision,
        num_lines,
        num_samples_per_line,
        num_image_components,
        image_components,
    })
}

fn Decoder108<'input>(_input: &mut Parser<'input>) -> Result<Type66, ParseError> {
    let id = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let sampling_factor = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let quantization_table_id = ((|| PResult::Ok((Decoder15(_input))?))())?;
    PResult::Ok(Type66 {
        id,
        sampling_factor,
        quantization_table_id,
    })
}

fn Decoder109<'input>(_input: &mut Parser<'input>) -> Result<Type62, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 219 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(11201713527929929098u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder131(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type62 {
        marker,
        length,
        data,
    })
}

fn Decoder110<'input>(_input: &mut Parser<'input>) -> Result<Type60, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 196 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(13231341950566326183u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder130(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type60 {
        marker,
        length,
        data,
    })
}

fn Decoder111<'input>(_input: &mut Parser<'input>) -> Result<Type58, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 204 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10217556179496943797u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder129(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type58 {
        marker,
        length,
        data,
    })
}

fn Decoder112<'input>(_input: &mut Parser<'input>) -> Result<Type64, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 221 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(8814285897505247341u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| PResult::Ok((Decoder128(_input))?))())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type64 {
        marker,
        length,
        data,
    })
}

fn Decoder113<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 226 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(12140482413237234104u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder114<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 227 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(2795443158724701367u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder115<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 228 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(3355559118720108211u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder116<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 229 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(14742198720488010940u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder117<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 230 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(6277645557415946825u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder118<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 231 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(2176159342917065583u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder119<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 232 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(6957547562921215229u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder120<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 233 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(3756953894146529854u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder121<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 234 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(12608692552323012024u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder122<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 235 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(2716996167109240019u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder123<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 236 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(6641423197242755780u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder124<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 237 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(4000866269867594892u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder125<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 238 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(7832938568744868798u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder126<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 239 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(52255437925028600u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder127<'input>(_input: &mut Parser<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 255 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(10618271977672484401u64));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 254 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(5705528789532761578u64));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length - 2u16) as usize;
            _input.start_slice(sz)?;
            let ret = ((|| {
                PResult::Ok({
                    let mut accum = Vec::new();
                    while _input.remaining() > 0 {
                        let matching_ix = {
                            _input.open_peek_context();
                            _input.read_byte()?;
                            {
                                let ret = 0;
                                _input.close_peek_context()?;
                                ret
                            }
                        };
                        if matching_ix == 0 {
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            _input.end_slice()?;
            ret
        })
    })())?;
    PResult::Ok(Type56 {
        marker,
        length,
        data,
    })
}

fn Decoder128<'input>(_input: &mut Parser<'input>) -> Result<Type63, ParseError> {
    let restart_interval = ((|| PResult::Ok((Decoder44(_input))?))())?;
    PResult::Ok(Type63 { restart_interval })
}

fn Decoder129<'input>(_input: &mut Parser<'input>) -> Result<Type57, ParseError> {
    let class_table_id = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let value = ((|| PResult::Ok((Decoder15(_input))?))())?;
    PResult::Ok(Type57 {
        class_table_id,
        value,
    })
}

fn Decoder130<'input>(_input: &mut Parser<'input>) -> Result<Type59, ParseError> {
    let class_table_id = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let num_codes = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..16u8 {
                accum.push((Decoder15(_input))?);
            }
            accum
        })
    })())?;
    let values = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    _input.read_byte()?;
                    {
                        let ret = 0;
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder15(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    PResult::Ok(Type59 {
        class_table_id,
        num_codes,
        values,
    })
}

fn Decoder131<'input>(_input: &mut Parser<'input>) -> Result<Type61, ParseError> {
    let precision_table_id = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let elements = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    _input.read_byte()?;
                    {
                        let ret = 0;
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder15(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    PResult::Ok(Type61 {
        precision_table_id,
        elements,
    })
}

fn Decoder132<'input>(_input: &mut Parser<'input>) -> Result<Type53, ParseError> {
    let identifier = ((|| PResult::Ok((Decoder133(_input))?))())?;
    let data = ((|| {
        PResult::Ok(match identifier.clone().string.as_slice() {
            [69, 120, 105, 102] => {
                let inner = (Decoder134(_input))?;
                Type52::exif(inner)
            }

            [104, 116, 116, 112, 58, 47, 47, 110, 115, 46, 97, 100, 111, 98, 101, 46, 99, 111, 109, 47, 120, 97, 112, 47, 49, 46, 48, 47] =>
            {
                let inner = (Decoder135(_input))?;
                Type52::xmp(inner)
            }

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
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                };
                Type52::other(inner)
            }
        })
    })())?;
    PResult::Ok(Type53 { identifier, data })
}

fn Decoder133<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    PResult::Ok(Type21 { string, null })
}

fn Decoder134<'input>(_input: &mut Parser<'input>) -> Result<Type50, ParseError> {
    let padding = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    let exif = ((|| PResult::Ok((Decoder136(_input))?))())?;
    PResult::Ok(Type50 { padding, exif })
}

fn Decoder135<'input>(_input: &mut Parser<'input>) -> Result<Type51, ParseError> {
    let xmp = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    _input.read_byte()?;
                    {
                        let ret = 0;
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder15(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    PResult::Ok(Type51 { xmp })
}

fn Decoder136<'input>(_input: &mut Parser<'input>) -> Result<Type49, ParseError> {
    let byte_order = ((|| {
        PResult::Ok({
            let tree_index = {
                _input.open_peek_context();
                let b = _input.read_byte()?;
                {
                    let ret = match b {
                        73u8 => 0,

                        77u8 => 1,

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
                    let field0 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 73 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(17197161005512507961u64));
                            }
                        })
                    })())?;
                    let field1 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 73 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(17197161005512507961u64));
                            }
                        })
                    })())?;
                    Type46::le(field0, field1)
                }

                1 => {
                    let field0 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 77 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(1661485880725065159u64));
                            }
                        })
                    })())?;
                    let field1 = ((|| {
                        PResult::Ok({
                            let b = _input.read_byte()?;
                            if b == 77 {
                                b
                            } else {
                                return Err(ParseError::ExcludedBranch(1661485880725065159u64));
                            }
                        })
                    })())?;
                    Type46::be(field0, field1)
                }

                _ => {
                    return Err(ParseError::ExcludedBranch(8662494850867647108u64));
                }
            }
        })
    })())?;
    let magic = ((|| {
        PResult::Ok(match byte_order {
            Type46::le(..) => (Decoder137(_input))?,

            Type46::be(..) => (Decoder44(_input))?,
        })
    })())?;
    let offset = ((|| {
        PResult::Ok(match byte_order {
            Type46::le(..) => (Decoder22(_input))?,

            Type46::be(..) => (Decoder31(_input))?,
        })
    })())?;
    let ifd = ((|| {
        PResult::Ok({
            _input.open_peek_context();
            _input.advance_by(offset - 8u32)?;
            let ret = ((|| {
                PResult::Ok(match byte_order {
                    Type46::le(..) => {
                        let num_fields = ((|| PResult::Ok((Decoder137(_input))?))())?;
                        let fields = ((|| {
                            PResult::Ok({
                                let mut accum = Vec::new();
                                for _ in 0..num_fields {
                                    accum.push({
                                        let tag = ((|| PResult::Ok((Decoder137(_input))?))())?;
                                        let r#type = ((|| PResult::Ok((Decoder137(_input))?))())?;
                                        let length = ((|| PResult::Ok((Decoder22(_input))?))())?;
                                        let offset_or_data =
                                            ((|| PResult::Ok((Decoder22(_input))?))())?;
                                        Type47 {
                                            tag,
                                            r#type,
                                            length,
                                            offset_or_data,
                                        }
                                    });
                                }
                                accum
                            })
                        })())?;
                        let next_ifd_offset = ((|| PResult::Ok((Decoder22(_input))?))())?;
                        let next_ifd = ((|| {
                            PResult::Ok({
                                let mut accum = Vec::new();
                                while _input.remaining() > 0 {
                                    let matching_ix = {
                                        _input.open_peek_context();
                                        _input.read_byte()?;
                                        {
                                            let ret = 0;
                                            _input.close_peek_context()?;
                                            ret
                                        }
                                    };
                                    if matching_ix == 0 {
                                        let next_elem = (Decoder15(_input))?;
                                        accum.push(next_elem);
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            })
                        })())?;
                        Type48 {
                            num_fields,
                            fields,
                            next_ifd_offset,
                            next_ifd,
                        }
                    }

                    Type46::be(..) => {
                        let num_fields = ((|| PResult::Ok((Decoder44(_input))?))())?;
                        let fields = ((|| {
                            PResult::Ok({
                                let mut accum = Vec::new();
                                for _ in 0..num_fields {
                                    accum.push({
                                        let tag = ((|| PResult::Ok((Decoder44(_input))?))())?;
                                        let r#type = ((|| PResult::Ok((Decoder44(_input))?))())?;
                                        let length = ((|| PResult::Ok((Decoder31(_input))?))())?;
                                        let offset_or_data =
                                            ((|| PResult::Ok((Decoder31(_input))?))())?;
                                        Type47 {
                                            tag,
                                            r#type,
                                            length,
                                            offset_or_data,
                                        }
                                    });
                                }
                                accum
                            })
                        })())?;
                        let next_ifd_offset = ((|| PResult::Ok((Decoder31(_input))?))())?;
                        let next_ifd = ((|| {
                            PResult::Ok({
                                let mut accum = Vec::new();
                                while _input.remaining() > 0 {
                                    let matching_ix = {
                                        _input.open_peek_context();
                                        _input.read_byte()?;
                                        {
                                            let ret = 0;
                                            _input.close_peek_context()?;
                                            ret
                                        }
                                    };
                                    if matching_ix == 0 {
                                        let next_elem = (Decoder15(_input))?;
                                        accum.push(next_elem);
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            })
                        })())?;
                        Type48 {
                            num_fields,
                            fields,
                            next_ifd_offset,
                            next_ifd,
                        }
                    }
                })
            })())?;
            _input.close_peek_context()?;
            ret
        })
    })())?;
    PResult::Ok(Type49 {
        byte_order,
        magic,
        offset,
        ifd,
    })
}

fn Decoder137<'input>(_input: &mut Parser<'input>) -> Result<u16, ParseError> {
    let inner = {
        let field0 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        let field1 = ((|| PResult::Ok((Decoder15(_input))?))())?;
        (field0, field1)
    };
    PResult::Ok(((|x: (u8, u8)| PResult::Ok(u16le(x)))(inner))?)
}

fn Decoder138<'input>(_input: &mut Parser<'input>) -> Result<Type44, ParseError> {
    let identifier = ((|| PResult::Ok((Decoder139(_input))?))())?;
    let data = ((|| {
        PResult::Ok(match identifier.clone().string.as_slice() {
            [74, 70, 73, 70] => {
                let inner = (Decoder140(_input))?;
                Type43::jfif(inner)
            }

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
                            let next_elem = (Decoder15(_input))?;
                            accum.push(next_elem);
                        } else {
                            break;
                        }
                    }
                    accum
                };
                Type43::other(inner)
            }
        })
    })())?;
    PResult::Ok(Type44 { identifier, data })
}

fn Decoder139<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    PResult::Ok(Type21 { string, null })
}

fn Decoder140<'input>(_input: &mut Parser<'input>) -> Result<Type42, ParseError> {
    let version_major = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let version_minor = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let density_units = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let density_x = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let density_y = ((|| PResult::Ok((Decoder44(_input))?))())?;
    let thumbnail_width = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let thumbnail_height = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let thumbnail_pixels = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..thumbnail_height {
                accum.push({
                    let mut accum = Vec::new();
                    for _ in 0..thumbnail_width {
                        accum.push((Decoder141(_input))?);
                    }
                    accum
                });
            }
            accum
        })
    })())?;
    PResult::Ok(Type42 {
        version_major,
        version_minor,
        density_units,
        density_x,
        density_y,
        thumbnail_width,
        thumbnail_height,
        thumbnail_pixels,
    })
}

fn Decoder141<'input>(_input: &mut Parser<'input>) -> Result<Type2, ParseError> {
    let r = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let g = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let b = ((|| PResult::Ok((Decoder15(_input))?))())?;
    PResult::Ok(Type2 { r, g, b })
}

fn Decoder142<'input>(_input: &mut Parser<'input>) -> Result<Type20, ParseError> {
    let magic = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 31 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(6728817869016996251u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 139 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(12646530475123667541u64));
                    }
                })
            })())?;
            (field0, field1)
        })
    })())?;
    let method = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let file_flags = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let timestamp = ((|| PResult::Ok((Decoder22(_input))?))())?;
    let compression_flags = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let os_id = ((|| PResult::Ok((Decoder15(_input))?))())?;
    PResult::Ok(Type20 {
        magic,
        method,
        file_flags,
        timestamp,
        compression_flags,
        os_id,
    })
}

fn Decoder143<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    PResult::Ok((Decoder153(_input))?)
}

fn Decoder144<'input>(_input: &mut Parser<'input>) -> Result<Type38, ParseError> {
    let blocks = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            loop {
                let elem = (Decoder146(_input))?;
                if ((|x: &Type37| PResult::Ok(x.clone().r#final == 1u8))(&elem))? {
                    accum.push(elem);
                    break;
                } else {
                    accum.push(elem);
                }
            }
            accum
        })
    })())?;
    let codes = ((|| {
        PResult::Ok(
            (try_flat_map_vec(blocks.iter().cloned(), |x: Type37| {
                PResult::Ok(match x.clone().data {
                    Type36::uncompressed(y) => y.clone().codes_values,

                    Type36::fixed_huffman(y) => y.clone().codes_values,

                    Type36::dynamic_huffman(y) => y.clone().codes_values,
                })
            }))?,
        )
    })())?;
    let inflate = ((|| {
        PResult::Ok({
            fn inflate(codes: &Vec<Type29>) -> Vec<u8> {
                let mut vs = Vec::new();
                for code in codes.iter() {
                    match code {
                        Type29::literal(v) => {
                            vs.push(*v);
                        }

                        Type29::reference(fields) => {
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
        })
    })())?;
    PResult::Ok(Type38 {
        blocks,
        codes,
        inflate,
    })
}

fn Decoder145<'input>(_input: &mut Parser<'input>) -> Result<Type39, ParseError> {
    let crc = ((|| PResult::Ok((Decoder22(_input))?))())?;
    let length = ((|| PResult::Ok((Decoder22(_input))?))())?;
    PResult::Ok(Type39 { crc, length })
}

fn Decoder146<'input>(_input: &mut Parser<'input>) -> Result<Type37, ParseError> {
    let r#final = ((|| PResult::Ok((Decoder147(_input))?))())?;
    let r#type = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                (field0, field1)
            };
            ((|bits: (u8, u8)| PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0))(inner))?
        })
    })())?;
    let data = ((|| {
        PResult::Ok(match r#type {
            0 => {
                let inner = (Decoder148(_input))?;
                Type36::uncompressed(inner)
            }

            1 => {
                let inner = (Decoder149(_input))?;
                Type36::fixed_huffman(inner)
            }

            2 => {
                let inner = (Decoder150(_input))?;
                Type36::dynamic_huffman(inner)
            }

            _other => {
                unreachable!(
                    r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                );
            }
        })
    })())?;
    PResult::Ok(Type37 {
        r#final,
        r#type,
        data,
    })
}

fn Decoder147<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
    let b = _input.read_byte()?;
    PResult::Ok(b)
}

fn Decoder148<'input>(_input: &mut Parser<'input>) -> Result<Type35, ParseError> {
    let align = ((|| PResult::Ok(_input.skip_align(8)?))())?;
    let len = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field7 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field8 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field9 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field10 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field11 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field12 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field13 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field14 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field15 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                (
                    field0, field1, field2, field3, field4, field5, field6, field7, field8, field9,
                    field10, field11, field12, field13, field14, field15,
                )
            };
            ((|bits: (
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
            )| {
                PResult::Ok(
                    (bits.clone().15 as u16) << 15u16
                        | (bits.clone().14 as u16) << 14u16
                        | (bits.clone().13 as u16) << 13u16
                        | (bits.clone().12 as u16) << 12u16
                        | (bits.clone().11 as u16) << 11u16
                        | (bits.clone().10 as u16) << 10u16
                        | (bits.clone().9 as u16) << 9u16
                        | (bits.clone().8 as u16) << 8u16
                        | (bits.clone().7 as u16) << 7u16
                        | (bits.clone().6 as u16) << 6u16
                        | (bits.clone().5 as u16) << 5u16
                        | (bits.clone().4 as u16) << 4u16
                        | (bits.clone().3 as u16) << 3u16
                        | (bits.clone().2 as u16) << 2u16
                        | (bits.clone().1 as u16) << 1u16
                        | (bits.clone().0 as u16),
                )
            })(inner))?
        })
    })())?;
    let nlen = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field7 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field8 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field9 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field10 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field11 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field12 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field13 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field14 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field15 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                (
                    field0, field1, field2, field3, field4, field5, field6, field7, field8, field9,
                    field10, field11, field12, field13, field14, field15,
                )
            };
            ((|bits: (
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
                u8,
            )| {
                PResult::Ok(
                    (bits.clone().15 as u16) << 15u16
                        | (bits.clone().14 as u16) << 14u16
                        | (bits.clone().13 as u16) << 13u16
                        | (bits.clone().12 as u16) << 12u16
                        | (bits.clone().11 as u16) << 11u16
                        | (bits.clone().10 as u16) << 10u16
                        | (bits.clone().9 as u16) << 9u16
                        | (bits.clone().8 as u16) << 8u16
                        | (bits.clone().7 as u16) << 7u16
                        | (bits.clone().6 as u16) << 6u16
                        | (bits.clone().5 as u16) << 5u16
                        | (bits.clone().4 as u16) << 4u16
                        | (bits.clone().3 as u16) << 3u16
                        | (bits.clone().2 as u16) << 2u16
                        | (bits.clone().1 as u16) << 1u16
                        | (bits.clone().0 as u16),
                )
            })(inner))?
        })
    })())?;
    let bytes = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..len {
                accum.push({
                    let inner = {
                        let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        let field7 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        (
                            field0, field1, field2, field3, field4, field5, field6, field7,
                        )
                    };
                    ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            bits.clone().7 << 7u8
                                | bits.clone().6 << 6u8
                                | bits.clone().5 << 5u8
                                | bits.clone().4 << 4u8
                                | bits.clone().3 << 3u8
                                | bits.clone().2 << 2u8
                                | bits.clone().1 << 1u8
                                | bits.clone().0,
                        )
                    })(inner))?
                });
            }
            accum
        })
    })())?;
    let codes_values = ((|| {
        PResult::Ok(
            (try_flat_map_vec(bytes.iter().cloned(), |x: u8| {
                PResult::Ok([Type29::literal(x)].to_vec())
            }))?,
        )
    })())?;
    PResult::Ok(Type35 {
        align,
        len,
        nlen,
        bytes,
        codes_values,
    })
}

fn Decoder149<'input>(_input: &mut Parser<'input>) -> Result<Type34, ParseError> {
    let codes = ((|| {
        PResult::Ok({
            let format = parse_huffman(
                [
                    8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8,
                    8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8,
                    8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8,
                    8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8,
                    8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8,
                    8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8,
                    8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8,
                    8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8,
                    8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8,
                    9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8,
                    9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8,
                    9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8,
                    9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8,
                    9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8,
                    9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8,
                    9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8,
                    7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8,
                    7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8,
                ]
                .to_vec(),
                None,
            );
            let mut accum = Vec::new();
            loop {
                let elem = {
                    let code = ((|| PResult::Ok((format(_input))?))())?;
                    let extra = ((|| {
                        PResult::Ok(match code {
                            257 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(3u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            258 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(4u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            259 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(5u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            260 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(6u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            261 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(7u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            262 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(8u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            263 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(9u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            264 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(10u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            265 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0,)
                                            };
                                            ((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(11u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            266 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0,)
                                            };
                                            ((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(13u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            267 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0,)
                                            };
                                            ((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(15u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            268 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0,)
                                            };
                                            ((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(17u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            269 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1)
                                            };
                                            ((|bits: (u8, u8)| {
                                                PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0)
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(19u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            270 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1)
                                            };
                                            ((|bits: (u8, u8)| {
                                                PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0)
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(23u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            271 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1)
                                            };
                                            ((|bits: (u8, u8)| {
                                                PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0)
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(27u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            272 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1)
                                            };
                                            ((|bits: (u8, u8)| {
                                                PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0)
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(31u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            273 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2)
                                            };
                                            ((|bits: (u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(35u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            274 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2)
                                            };
                                            ((|bits: (u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(43u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            275 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2)
                                            };
                                            ((|bits: (u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(51u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            276 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2)
                                            };
                                            ((|bits: (u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(59u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            277 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3)
                                            };
                                            ((|bits: (u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(67u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            278 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3)
                                            };
                                            ((|bits: (u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(83u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            279 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3)
                                            };
                                            ((|bits: (u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(99u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            280 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3)
                                            };
                                            ((|bits: (u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(115u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            281 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(131u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            282 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(163u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            283 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(195u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            284 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(227u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            285 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(258u16 + (length_extra_bits as u16)))())?;
                                    let distance_code = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code as u16))?)
                                    })(
                                    ))?;
                                    Type31 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type32::some(inner)
                            }

                            _ => {
                                let _ = ();
                                Type32::none
                            }
                        })
                    })())?;
                    Type33 { code, extra }
                };
                if ((|x: &Type33| PResult::Ok((x.clone().code as u16) == 256u16))(&elem))? {
                    accum.push(elem);
                    break;
                } else {
                    accum.push(elem);
                }
            }
            accum
        })
    })())?;
    let codes_values = ((|| {
        PResult::Ok(
            (try_flat_map_vec(codes.iter().cloned(), |x: Type33| {
                PResult::Ok(match x.clone().code {
                    256 => [].to_vec(),

                    257 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    258 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    259 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    260 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    261 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    262 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    263 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    264 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    265 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    266 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    267 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    268 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    269 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    270 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    271 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    272 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    273 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    274 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    275 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    276 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    277 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    278 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    279 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    280 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    281 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    282 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    283 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    284 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    285 => match x.clone().extra {
                        Type32::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(3837024443630474590u64));
                        }
                    },

                    _ => [Type29::literal(x.clone().code as u8)].to_vec(),
                })
            }))?,
        )
    })())?;
    PResult::Ok(Type34 {
        codes,
        codes_values,
    })
}

fn Decoder150<'input>(_input: &mut Parser<'input>) -> Result<Type30, ParseError> {
    let hlit = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                (field0, field1, field2, field3, field4)
            };
            ((|bits: (u8, u8, u8, u8, u8)| {
                PResult::Ok(
                    bits.clone().4 << 4u8
                        | bits.clone().3 << 3u8
                        | bits.clone().2 << 2u8
                        | bits.clone().1 << 1u8
                        | bits.clone().0,
                )
            })(inner))?
        })
    })())?;
    let hdist = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                (field0, field1, field2, field3, field4)
            };
            ((|bits: (u8, u8, u8, u8, u8)| {
                PResult::Ok(
                    bits.clone().4 << 4u8
                        | bits.clone().3 << 3u8
                        | bits.clone().2 << 2u8
                        | bits.clone().1 << 1u8
                        | bits.clone().0,
                )
            })(inner))?
        })
    })())?;
    let hclen = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                (field0, field1, field2, field3)
            };
            ((|bits: (u8, u8, u8, u8)| {
                PResult::Ok(
                    bits.clone().3 << 3u8
                        | bits.clone().2 << 2u8
                        | bits.clone().1 << 1u8
                        | bits.clone().0,
                )
            })(inner))?
        })
    })())?;
    let code_length_alphabet_code_lengths = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..hclen + 4u8 {
                accum.push({
                    let inner = {
                        let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                        (field0, field1, field2)
                    };
                    ((|bits: (u8, u8, u8)| {
                        PResult::Ok(bits.clone().2 << 2u8 | bits.clone().1 << 1u8 | bits.clone().0)
                    })(inner))?
                });
            }
            accum
        })
    })())?;
    let literal_length_distance_alphabet_code_lengths = ((|| {
        PResult::Ok({
            let code_length_alphabet_format = parse_huffman(
                code_length_alphabet_code_lengths.clone(),
                Some(
                    [
                        16u8, 17u8, 18u8, 0u8, 8u8, 7u8, 9u8, 6u8, 10u8, 5u8, 11u8, 4u8, 12u8, 3u8,
                        13u8, 2u8, 14u8, 1u8, 15u8,
                    ]
                    .to_vec(),
                ),
            );
            let mut accum = Vec::new();
            loop {
                let elem = {
                    let code = ((|| PResult::Ok((code_length_alphabet_format(_input))?))())?;
                    let extra = ((|| {
                        PResult::Ok(match code as u8 {
                            16 => {
                                let inner = {
                                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    (field0, field1)
                                };
                                ((|bits: (u8, u8)| {
                                    PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0)
                                })(inner))?
                            }

                            17 => {
                                let inner = {
                                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    (field0, field1, field2)
                                };
                                ((|bits: (u8, u8, u8)| {
                                    PResult::Ok(
                                        bits.clone().2 << 2u8
                                            | bits.clone().1 << 1u8
                                            | bits.clone().0,
                                    )
                                })(inner))?
                            }

                            18 => {
                                let inner = {
                                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                                    (field0, field1, field2, field3, field4, field5, field6)
                                };
                                ((|bits: (u8, u8, u8, u8, u8, u8, u8)| {
                                    PResult::Ok(
                                        bits.clone().6 << 6u8
                                            | bits.clone().5 << 5u8
                                            | bits.clone().4 << 4u8
                                            | bits.clone().3 << 3u8
                                            | bits.clone().2 << 2u8
                                            | bits.clone().1 << 1u8
                                            | bits.clone().0,
                                    )
                                })(inner))?
                            }

                            _ => 0u8,
                        })
                    })())?;
                    Type23 { code, extra }
                };
                accum.push(elem);
                if ((|y: &Vec<Type23>| {
                    PResult::Ok(
                        (((try_fold_map_curried(
                            y.iter().cloned(),
                            {
                                ();
                                Type199::none
                            },
                            |x: (Type199, Type23)| {
                                PResult::Ok(match x.clone().1.code as u8 {
                                    16 => (
                                        x.clone().0,
                                        dup32(
                                            (x.clone().1.extra + 3u8) as u32,
                                            match x.clone().0 {
                                                Type199::some(y) => y.clone(),

                                                _ => {
                                                    return Err(ParseError::ExcludedBranch(
                                                        15488148895825521580u64,
                                                    ));
                                                }
                                            },
                                        ),
                                    ),

                                    17 => {
                                        (x.clone().0, dup32((x.clone().1.extra + 3u8) as u32, 0u8))
                                    }

                                    18 => {
                                        (x.clone().0, dup32((x.clone().1.extra + 11u8) as u32, 0u8))
                                    }

                                    v => (Type199::some(v), [v.clone()].to_vec()),
                                })
                            },
                        ))?
                        .len()) as u32)
                            >= ((hlit + hdist) as u32) + 258u32,
                    )
                })(&accum))?
                {
                    break;
                }
            }
            accum
        })
    })())?;
    let literal_length_distance_alphabet_code_lengths_value = ((|| {
        PResult::Ok(
            (try_fold_map_curried(
                literal_length_distance_alphabet_code_lengths
                    .iter()
                    .cloned(),
                {
                    ();
                    Type199::none
                },
                |x: (Type199, Type23)| {
                    PResult::Ok(match x.clone().1.code as u8 {
                        16 => (
                            x.clone().0,
                            dup32(
                                (x.clone().1.extra + 3u8) as u32,
                                match x.clone().0 {
                                    Type199::some(y) => y.clone(),

                                    _ => {
                                        return Err(ParseError::ExcludedBranch(
                                            15488148895825521580u64,
                                        ));
                                    }
                                },
                            ),
                        ),

                        17 => (x.clone().0, dup32((x.clone().1.extra + 3u8) as u32, 0u8)),

                        18 => (x.clone().0, dup32((x.clone().1.extra + 11u8) as u32, 0u8)),

                        v => (Type199::some(v), [v.clone()].to_vec()),
                    })
                },
            ))?,
        )
    })())?;
    let literal_length_alphabet_code_lengths_value = ((|| {
        PResult::Ok({
            let ix = 0u32 as usize;
            Vec::from(
                &literal_length_distance_alphabet_code_lengths_value
                    [ix..(ix + (((hlit as u32) + 257u32) as usize))],
            )
        })
    })())?;
    let distance_alphabet_code_lengths_value = ((|| {
        PResult::Ok({
            let ix = ((hlit as u32) + 257u32) as usize;
            Vec::from(
                &literal_length_distance_alphabet_code_lengths_value
                    [ix..(ix + (((hdist as u32) + 1u32) as usize))],
            )
        })
    })())?;
    let codes = ((|| {
        PResult::Ok({
            let distance_alphabet_format =
                parse_huffman(distance_alphabet_code_lengths_value.clone(), None);
            let literal_length_alphabet_format =
                parse_huffman(literal_length_alphabet_code_lengths_value.clone(), None);
            let mut accum = Vec::new();
            loop {
                let elem = {
                    let code = ((|| PResult::Ok((literal_length_alphabet_format(_input))?))())?;
                    let extra = ((|| {
                        PResult::Ok(match code {
                            257 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(3u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            258 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(4u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            259 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(5u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            260 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(6u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            261 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(7u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            262 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(8u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            263 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(9u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            264 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(10u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            265 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0,)
                                            };
                                            ((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(11u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            266 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0,)
                                            };
                                            ((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(13u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            267 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0,)
                                            };
                                            ((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(15u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            268 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0,)
                                            };
                                            ((|bits: (u8,)| PResult::Ok(bits.clone().0))(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(17u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            269 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1)
                                            };
                                            ((|bits: (u8, u8)| {
                                                PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0)
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(19u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            270 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1)
                                            };
                                            ((|bits: (u8, u8)| {
                                                PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0)
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(23u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            271 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1)
                                            };
                                            ((|bits: (u8, u8)| {
                                                PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0)
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(27u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            272 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1)
                                            };
                                            ((|bits: (u8, u8)| {
                                                PResult::Ok(bits.clone().1 << 1u8 | bits.clone().0)
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(31u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            273 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2)
                                            };
                                            ((|bits: (u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(35u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            274 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2)
                                            };
                                            ((|bits: (u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(43u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            275 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2)
                                            };
                                            ((|bits: (u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(51u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            276 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2)
                                            };
                                            ((|bits: (u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(59u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            277 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3)
                                            };
                                            ((|bits: (u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(67u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            278 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3)
                                            };
                                            ((|bits: (u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(83u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            279 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3)
                                            };
                                            ((|bits: (u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(99u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            280 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3)
                                            };
                                            ((|bits: (u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(115u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            281 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(131u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            282 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(163u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            283 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(195u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            284 => {
                                let inner = {
                                    let length_extra_bits = ((|| {
                                        PResult::Ok({
                                            let inner = {
                                                let field0 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field1 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field2 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field3 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                let field4 =
                                                    ((|| PResult::Ok((Decoder147(_input))?))())?;
                                                (field0, field1, field2, field3, field4)
                                            };
                                            ((|bits: (u8, u8, u8, u8, u8)| {
                                                PResult::Ok(
                                                    bits.clone().4 << 4u8
                                                        | bits.clone().3 << 3u8
                                                        | bits.clone().2 << 2u8
                                                        | bits.clone().1 << 1u8
                                                        | bits.clone().0,
                                                )
                                            })(inner))?
                                        })
                                    })(
                                    ))?;
                                    let length =
                                        ((|| PResult::Ok(227u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            285 => {
                                let inner = {
                                    let length_extra_bits = ((|| PResult::Ok(0u8))())?;
                                    let length =
                                        ((|| PResult::Ok(258u16 + (length_extra_bits as u16)))())?;
                                    let distance_code =
                                        ((|| PResult::Ok((distance_alphabet_format(_input))?))())?;
                                    let distance_record = ((|| {
                                        PResult::Ok((Decoder151(_input, distance_code.clone()))?)
                                    })(
                                    ))?;
                                    Type25 {
                                        length_extra_bits,
                                        length,
                                        distance_code,
                                        distance_record,
                                    }
                                };
                                Type26::some(inner)
                            }

                            _ => {
                                let _ = ();
                                Type26::none
                            }
                        })
                    })())?;
                    Type27 { code, extra }
                };
                if ((|x: &Type27| PResult::Ok((x.clone().code as u16) == 256u16))(&elem))? {
                    accum.push(elem);
                    break;
                } else {
                    accum.push(elem);
                }
            }
            accum
        })
    })())?;
    let codes_values = ((|| {
        PResult::Ok(
            (try_flat_map_vec(codes.iter().cloned(), |x: Type27| {
                PResult::Ok(match x.clone().code {
                    256 => [].to_vec(),

                    257 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    258 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    259 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    260 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    261 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    262 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    263 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    264 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    265 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    266 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    267 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    268 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    269 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    270 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    271 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    272 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    273 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    274 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    275 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    276 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    277 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    278 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    279 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    280 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    281 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    282 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    283 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    284 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    285 => match x.clone().extra {
                        Type26::some(rec) => [Type29::reference(Type28 {
                            length: rec.clone().length,
                            distance: rec.clone().distance_record.distance,
                        })]
                        .to_vec(),

                        _ => {
                            return Err(ParseError::ExcludedBranch(13749635282143713177u64));
                        }
                    },

                    _ => [Type29::literal(x.clone().code as u8)].to_vec(),
                })
            }))?,
        )
    })())?;
    PResult::Ok(Type30 {
        hlit,
        hdist,
        hclen,
        code_length_alphabet_code_lengths,
        literal_length_distance_alphabet_code_lengths,
        literal_length_distance_alphabet_code_lengths_value,
        literal_length_alphabet_code_lengths_value,
        distance_alphabet_code_lengths_value,
        codes,
        codes_values,
    })
}

fn Decoder151<'input>(
    _input: &mut Parser<'input>,
    distance_code: u16,
) -> Result<Type24, ParseError> {
    PResult::Ok(match distance_code as u8 {
        0 => (Decoder152(_input, 0u8, 1u16))?,

        1 => (Decoder152(_input, 0u8, 2u16))?,

        2 => (Decoder152(_input, 0u8, 3u16))?,

        3 => (Decoder152(_input, 0u8, 4u16))?,

        4 => (Decoder152(_input, 1u8, 5u16))?,

        5 => (Decoder152(_input, 1u8, 7u16))?,

        6 => (Decoder152(_input, 2u8, 9u16))?,

        7 => (Decoder152(_input, 2u8, 13u16))?,

        8 => (Decoder152(_input, 3u8, 17u16))?,

        9 => (Decoder152(_input, 3u8, 25u16))?,

        10 => (Decoder152(_input, 4u8, 33u16))?,

        11 => (Decoder152(_input, 4u8, 49u16))?,

        12 => (Decoder152(_input, 5u8, 65u16))?,

        13 => (Decoder152(_input, 5u8, 97u16))?,

        14 => (Decoder152(_input, 6u8, 129u16))?,

        15 => (Decoder152(_input, 6u8, 193u16))?,

        16 => (Decoder152(_input, 7u8, 257u16))?,

        17 => (Decoder152(_input, 7u8, 385u16))?,

        18 => (Decoder152(_input, 8u8, 513u16))?,

        19 => (Decoder152(_input, 8u8, 769u16))?,

        20 => (Decoder152(_input, 9u8, 1025u16))?,

        21 => (Decoder152(_input, 9u8, 1537u16))?,

        22 => (Decoder152(_input, 10u8, 2049u16))?,

        23 => (Decoder152(_input, 10u8, 3073u16))?,

        24 => (Decoder152(_input, 11u8, 4097u16))?,

        25 => (Decoder152(_input, 11u8, 6145u16))?,

        26 => (Decoder152(_input, 12u8, 8193u16))?,

        27 => (Decoder152(_input, 12u8, 12289u16))?,

        28 => (Decoder152(_input, 13u8, 16385u16))?,

        29 => (Decoder152(_input, 13u8, 24577u16))?,

        _other => {
            unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#);
        }
    })
}

fn Decoder152<'input>(
    _input: &mut Parser<'input>,
    extra_bits: u8,
    start: u16,
) -> Result<Type24, ParseError> {
    let distance_extra_bits = ((|| {
        PResult::Ok(match extra_bits {
            0 => 0u16,

            1 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (field0,)
                };
                ((|bits: (u8,)| PResult::Ok(bits.clone().0 as u16))(inner))?
            }

            2 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (field0, field1)
                };
                ((|bits: (u8, u8)| {
                    PResult::Ok((bits.clone().1 as u16) << 1u16 | (bits.clone().0 as u16))
                })(inner))?
            }

            3 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (field0, field1, field2)
                };
                ((|bits: (u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            4 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (field0, field1, field2, field3)
                };
                ((|bits: (u8, u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().3 as u16) << 3u16
                            | (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            5 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (field0, field1, field2, field3, field4)
                };
                ((|bits: (u8, u8, u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().4 as u16) << 4u16
                            | (bits.clone().3 as u16) << 3u16
                            | (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            6 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (field0, field1, field2, field3, field4, field5)
                };
                ((|bits: (u8, u8, u8, u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().5 as u16) << 5u16
                            | (bits.clone().4 as u16) << 4u16
                            | (bits.clone().3 as u16) << 3u16
                            | (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            7 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (field0, field1, field2, field3, field4, field5, field6)
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().6 as u16) << 6u16
                            | (bits.clone().5 as u16) << 5u16
                            | (bits.clone().4 as u16) << 4u16
                            | (bits.clone().3 as u16) << 3u16
                            | (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            8 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field7 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().7 as u16) << 7u16
                            | (bits.clone().6 as u16) << 6u16
                            | (bits.clone().5 as u16) << 5u16
                            | (bits.clone().4 as u16) << 4u16
                            | (bits.clone().3 as u16) << 3u16
                            | (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            9 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field7 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field8 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().8 as u16) << 8u16
                            | (bits.clone().7 as u16) << 7u16
                            | (bits.clone().6 as u16) << 6u16
                            | (bits.clone().5 as u16) << 5u16
                            | (bits.clone().4 as u16) << 4u16
                            | (bits.clone().3 as u16) << 3u16
                            | (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            10 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field7 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field8 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field9 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().9 as u16) << 9u16
                            | (bits.clone().8 as u16) << 8u16
                            | (bits.clone().7 as u16) << 7u16
                            | (bits.clone().6 as u16) << 6u16
                            | (bits.clone().5 as u16) << 5u16
                            | (bits.clone().4 as u16) << 4u16
                            | (bits.clone().3 as u16) << 3u16
                            | (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            11 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field7 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field8 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field9 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field10 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9, field10,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().10 as u16) << 10u16
                            | (bits.clone().9 as u16) << 9u16
                            | (bits.clone().8 as u16) << 8u16
                            | (bits.clone().7 as u16) << 7u16
                            | (bits.clone().6 as u16) << 6u16
                            | (bits.clone().5 as u16) << 5u16
                            | (bits.clone().4 as u16) << 4u16
                            | (bits.clone().3 as u16) << 3u16
                            | (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            12 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field7 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field8 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field9 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field10 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field11 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9, field10, field11,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().11 as u16) << 11u16
                            | (bits.clone().10 as u16) << 10u16
                            | (bits.clone().9 as u16) << 9u16
                            | (bits.clone().8 as u16) << 8u16
                            | (bits.clone().7 as u16) << 7u16
                            | (bits.clone().6 as u16) << 6u16
                            | (bits.clone().5 as u16) << 5u16
                            | (bits.clone().4 as u16) << 4u16
                            | (bits.clone().3 as u16) << 3u16
                            | (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            13 => {
                let inner = {
                    let field0 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field1 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field2 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field3 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field4 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field5 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field6 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field7 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field8 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field9 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field10 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field11 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    let field12 = ((|| PResult::Ok((Decoder147(_input))?))())?;
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9, field10, field11, field12,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    PResult::Ok(
                        (bits.clone().12 as u16) << 12u16
                            | (bits.clone().11 as u16) << 11u16
                            | (bits.clone().10 as u16) << 10u16
                            | (bits.clone().9 as u16) << 9u16
                            | (bits.clone().8 as u16) << 8u16
                            | (bits.clone().7 as u16) << 7u16
                            | (bits.clone().6 as u16) << 6u16
                            | (bits.clone().5 as u16) << 5u16
                            | (bits.clone().4 as u16) << 4u16
                            | (bits.clone().3 as u16) << 3u16
                            | (bits.clone().2 as u16) << 2u16
                            | (bits.clone().1 as u16) << 1u16
                            | (bits.clone().0 as u16),
                    )
                })(inner))?
            }

            _other => {
                unreachable!(
                    r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                );
            }
        })
    })())?;
    let distance = ((|| PResult::Ok(start + distance_extra_bits))())?;
    PResult::Ok(Type24 {
        distance_extra_bits,
        distance,
    })
}

fn Decoder153<'input>(_input: &mut Parser<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

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
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10396965092922267801u64));
            }
        })
    })())?;
    PResult::Ok(Type21 { string, null })
}

fn Decoder154<'input>(_input: &mut Parser<'input>) -> Result<Type0, ParseError> {
    let signature = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 71 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(690880023569680479u64));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 73 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(17197161005512507961u64));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = _input.read_byte()?;
                    if b == 70 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(14049552398800766371u64));
                    }
                })
            })())?;
            (field0, field1, field2)
        })
    })())?;
    let version = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..3u8 {
                accum.push((Decoder19(_input))?);
            }
            accum
        })
    })())?;
    PResult::Ok(Type0 { signature, version })
}

fn Decoder155<'input>(_input: &mut Parser<'input>) -> Result<Type4, ParseError> {
    let descriptor = ((|| PResult::Ok((Decoder171(_input))?))())?;
    let global_color_table = ((|| {
        PResult::Ok(match descriptor.clone().flags & 128u8 != 0u8 {
            true => {
                let inner = {
                    let mut accum = Vec::new();
                    for _ in 0..2u8 << (descriptor.clone().flags & 7u8) {
                        accum.push((Decoder169(_input))?);
                    }
                    accum
                };
                Type3::yes(inner)
            }

            false => {
                let _ = ();
                Type3::no
            }
        })
    })())?;
    PResult::Ok(Type4 {
        descriptor,
        global_color_table,
    })
}

fn Decoder156<'input>(_input: &mut Parser<'input>) -> Result<Type17, ParseError> {
    let tree_index = {
        _input.open_peek_context();
        let b = _input.read_byte()?;
        {
            let ret = match b {
                33u8 => {
                    let b = _input.read_byte()?;
                    match b {
                        249u8 => 0,

                        1u8 => 0,

                        255u8 => 1,

                        254u8 => 1,

                        _ => {
                            return Err(ParseError::ExcludedBranch(5009412587336832230u64));
                        }
                    }
                }

                44u8 => 0,

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
            let inner = (Decoder158(_input))?;
            Type17::graphic_block(inner)
        }

        1 => {
            let inner = (Decoder159(_input))?;
            Type17::special_purpose_block(inner)
        }

        _ => {
            return Err(ParseError::ExcludedBranch(14056621334159770744u64));
        }
    })
}

fn Decoder157<'input>(_input: &mut Parser<'input>) -> Result<Type18, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 59 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(15783818897979407630u64));
            }
        })
    })())?;
    PResult::Ok(Type18 { separator })
}

fn Decoder158<'input>(_input: &mut Parser<'input>) -> Result<Type13, ParseError> {
    let graphic_control_extension = ((|| {
        PResult::Ok({
            let tree_index = {
                _input.open_peek_context();
                let b = _input.read_byte()?;
                {
                    let ret = match b {
                        33u8 => {
                            let b = _input.read_byte()?;
                            match b {
                                249u8 => 0,

                                1u8 => 1,

                                _ => {
                                    return Err(ParseError::ExcludedBranch(
                                        16676828686615745155u64,
                                    ));
                                }
                            }
                        }

                        44u8 => 1,

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
                    let inner = (Decoder164(_input))?;
                    Type6::some(inner)
                }

                1 => {
                    let _ = ();
                    Type6::none
                }

                _ => {
                    return Err(ParseError::ExcludedBranch(15496895076277599409u64));
                }
            }
        })
    })())?;
    let graphic_rendering_block = ((|| PResult::Ok((Decoder165(_input))?))())?;
    PResult::Ok(Type13 {
        graphic_control_extension,
        graphic_rendering_block,
    })
}

fn Decoder159<'input>(_input: &mut Parser<'input>) -> Result<Type16, ParseError> {
    let tree_index = {
        _input.open_peek_context();
        let b = _input.read_byte()?;
        {
            let ret = if b == 33 {
                let b = _input.read_byte()?;
                match b {
                    255u8 => 0,

                    254u8 => 1,

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
            let inner = (Decoder160(_input))?;
            Type16::application_extension(inner)
        }

        1 => {
            let inner = (Decoder161(_input))?;
            Type16::comment_extension(inner)
        }

        _ => {
            return Err(ParseError::ExcludedBranch(7565262198115782210u64));
        }
    })
}

fn Decoder160<'input>(_input: &mut Parser<'input>) -> Result<Type14, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 33 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(12638618761954708471u64));
            }
        })
    })())?;
    let label = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 255 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(10618271977672484401u64));
            }
        })
    })())?;
    let block_size = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 11 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(16286797724653440122u64));
            }
        })
    })())?;
    let identifier = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..8u8 {
                accum.push((Decoder15(_input))?);
            }
            accum
        })
    })())?;
    let authentication_code = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..3u8 {
                accum.push((Decoder15(_input))?);
            }
            accum
        })
    })())?;
    let application_data = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

                            _ => {
                                return Err(ParseError::ExcludedBranch(13862338712518612949u64));
                            }
                        };
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder162(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let terminator = ((|| PResult::Ok((Decoder163(_input))?))())?;
    PResult::Ok(Type14 {
        separator,
        label,
        block_size,
        identifier,
        authentication_code,
        application_data,
        terminator,
    })
}

fn Decoder161<'input>(_input: &mut Parser<'input>) -> Result<Type15, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 33 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(12638618761954708471u64));
            }
        })
    })())?;
    let label = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 254 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(5705528789532761578u64));
            }
        })
    })())?;
    let comment_data = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

                            _ => {
                                return Err(ParseError::ExcludedBranch(13862338712518612949u64));
                            }
                        };
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder162(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let terminator = ((|| PResult::Ok((Decoder163(_input))?))())?;
    PResult::Ok(Type15 {
        separator,
        label,
        comment_data,
        terminator,
    })
}

fn Decoder162<'input>(_input: &mut Parser<'input>) -> Result<Type7, ParseError> {
    let len_bytes = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b != 0 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(8606461246239977862u64));
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            for _ in 0..len_bytes {
                accum.push((Decoder15(_input))?);
            }
            accum
        })
    })())?;
    PResult::Ok(Type7 { len_bytes, data })
}

fn Decoder163<'input>(_input: &mut Parser<'input>) -> Result<u8, ParseError> {
    let b = _input.read_byte()?;
    PResult::Ok(if b == 0 {
        b
    } else {
        return Err(ParseError::ExcludedBranch(10396965092922267801u64));
    })
}

fn Decoder164<'input>(_input: &mut Parser<'input>) -> Result<Type5, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 33 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(12638618761954708471u64));
            }
        })
    })())?;
    let label = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 249 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(2007898731924533432u64));
            }
        })
    })())?;
    let block_size = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 4 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(12797785829236001664u64));
            }
        })
    })())?;
    let flags = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let delay_time = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let transparent_color_index = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let terminator = ((|| PResult::Ok((Decoder163(_input))?))())?;
    PResult::Ok(Type5 {
        separator,
        label,
        block_size,
        flags,
        delay_time,
        transparent_color_index,
        terminator,
    })
}

fn Decoder165<'input>(_input: &mut Parser<'input>) -> Result<Type12, ParseError> {
    let tree_index = {
        _input.open_peek_context();
        let b = _input.read_byte()?;
        {
            let ret = match b {
                44u8 => 0,

                33u8 => 1,

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
            let inner = (Decoder166(_input))?;
            Type12::table_based_image(inner)
        }

        1 => {
            let inner = (Decoder167(_input))?;
            Type12::plain_text_extension(inner)
        }

        _ => {
            return Err(ParseError::ExcludedBranch(14120387546690436687u64));
        }
    })
}

fn Decoder166<'input>(_input: &mut Parser<'input>) -> Result<Type11, ParseError> {
    let descriptor = ((|| PResult::Ok((Decoder168(_input))?))())?;
    let local_color_table = ((|| {
        PResult::Ok(match descriptor.clone().flags & 128u8 != 0u8 {
            true => {
                let inner = {
                    let mut accum = Vec::new();
                    for _ in 0..2u8 << (descriptor.clone().flags & 7u8) {
                        accum.push((Decoder169(_input))?);
                    }
                    accum
                };
                Type3::yes(inner)
            }

            false => {
                let _ = ();
                Type3::no
            }
        })
    })())?;
    let data = ((|| PResult::Ok((Decoder170(_input))?))())?;
    PResult::Ok(Type11 {
        descriptor,
        local_color_table,
        data,
    })
}

fn Decoder167<'input>(_input: &mut Parser<'input>) -> Result<Type8, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 33 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(12638618761954708471u64));
            }
        })
    })())?;
    let label = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 1 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(2974505448909915409u64));
            }
        })
    })())?;
    let block_size = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 12 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(15268554964885899593u64));
            }
        })
    })())?;
    let text_grid_left_position = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let text_grid_top_position = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let text_grid_width = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let text_grid_height = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let character_cell_width = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let character_cell_height = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let text_foreground_color_index = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let text_background_color_index = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let plain_text_data = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

                            _ => {
                                return Err(ParseError::ExcludedBranch(13862338712518612949u64));
                            }
                        };
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder162(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let terminator = ((|| PResult::Ok((Decoder163(_input))?))())?;
    PResult::Ok(Type8 {
        separator,
        label,
        block_size,
        text_grid_left_position,
        text_grid_top_position,
        text_grid_width,
        text_grid_height,
        character_cell_width,
        character_cell_height,
        text_foreground_color_index,
        text_background_color_index,
        plain_text_data,
        terminator,
    })
}

fn Decoder168<'input>(_input: &mut Parser<'input>) -> Result<Type9, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = _input.read_byte()?;
            if b == 44 {
                b
            } else {
                return Err(ParseError::ExcludedBranch(957865226307229178u64));
            }
        })
    })())?;
    let image_left_position = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let image_top_position = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let image_width = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let image_height = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let flags = ((|| PResult::Ok((Decoder15(_input))?))())?;
    PResult::Ok(Type9 {
        separator,
        image_left_position,
        image_top_position,
        image_width,
        image_height,
        flags,
    })
}

fn Decoder169<'input>(_input: &mut Parser<'input>) -> Result<Type2, ParseError> {
    let r = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let g = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let b = ((|| PResult::Ok((Decoder15(_input))?))())?;
    PResult::Ok(Type2 { r, g, b })
}

fn Decoder170<'input>(_input: &mut Parser<'input>) -> Result<Type10, ParseError> {
    let lzw_min_code_size = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let image_data = ((|| {
        PResult::Ok({
            let mut accum = Vec::new();
            while _input.remaining() > 0 {
                let matching_ix = {
                    _input.open_peek_context();
                    let b = _input.read_byte()?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0u8 => 1,

                            _ => {
                                return Err(ParseError::ExcludedBranch(13862338712518612949u64));
                            }
                        };
                        _input.close_peek_context()?;
                        ret
                    }
                };
                if matching_ix == 0 {
                    let next_elem = (Decoder162(_input))?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let terminator = ((|| PResult::Ok((Decoder163(_input))?))())?;
    PResult::Ok(Type10 {
        lzw_min_code_size,
        image_data,
        terminator,
    })
}

fn Decoder171<'input>(_input: &mut Parser<'input>) -> Result<Type1, ParseError> {
    let screen_width = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let screen_height = ((|| PResult::Ok((Decoder137(_input))?))())?;
    let flags = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let bg_color_index = ((|| PResult::Ok((Decoder15(_input))?))())?;
    let pixel_aspect_ratio = ((|| PResult::Ok((Decoder15(_input))?))())?;
    PResult::Ok(Type1 {
        screen_width,
        screen_height,
        flags,
        bg_color_index,
        pixel_aspect_ratio,
    })
}

mod codegen_tests;
