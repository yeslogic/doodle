
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
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
    pixels_per_unit_x: u32,
    pixels_per_unit_y: u32,
    unit_specifier: u8,
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
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

#[derive(Debug, Clone)]
struct Type175 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type174,
    crc: u32,
}

#[derive(Debug, Clone)]
enum Type176 {
    color_type_0(Type167),
    color_type_2(Type168),
    color_type_3(Vec<Type169>),
}

#[derive(Debug, Clone)]
struct Type177 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type176,
    crc: u32,
}

#[derive(Debug, Clone)]
enum Type178 {
    PLTE(Type166),
    bKGD(Type171),
    pHYs(Type173),
    tIME(Type175),
    tRNS(Type177),
}

#[derive(Debug, Clone)]
struct Type179 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Vec<u8>,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type180 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: (),
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type181 {
    signature: (u8, u8, u8, u8, u8, u8, u8, u8),
    ihdr: Type165,
    chunks: Vec<Type178>,
    idat: Vec<Type179>,
    more_chunks: Vec<Type178>,
    iend: Type180,
}

#[derive(Debug, Clone)]
enum Type182 {
    no(u8),
    yes,
}

#[derive(Debug, Clone)]
struct Type183 {
    tag: (u8, u8, u8, u8),
    length: u32,
    data: Vec<u8>,
    pad: Type182,
}

#[derive(Debug, Clone)]
struct Type184 {
    tag: (u8, u8, u8, u8),
    chunks: Vec<Type183>,
}

#[derive(Debug, Clone)]
struct Type185 {
    tag: (u8, u8, u8, u8),
    length: u32,
    data: Type184,
    pad: Type182,
}

#[derive(Debug, Clone)]
struct Type186 {
    string: Vec<u8>,
    __padding: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type187 {
    string: Vec<u8>,
    __nul_or_wsp: u8,
    __padding: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type188 {
    string: Vec<u8>,
    padding: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type189 {
    name: Type186,
    mode: Type187,
    uid: Type187,
    gid: Type187,
    size: u32,
    mtime: Type187,
    chksum: Type187,
    typeflag: u8,
    linkname: Type186,
    magic: (u8, u8, u8, u8, u8, u8),
    version: (u8, u8),
    uname: Type188,
    gname: Type188,
    devmajor: Type187,
    devminor: Type187,
    prefix: Type186,
    pad: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type190 {
    header: Type189,
    file: Vec<u8>,
    __padding: (),
}

#[derive(Debug, Clone)]
struct Type191 {
    contents: Vec<Type190>,
    __padding: Vec<u8>,
    __trailing: Vec<u8>,
}

#[derive(Debug, Clone)]
enum Type192 {
    ascii(Vec<u8>),
    utf8(Vec<char>),
}

#[derive(Debug, Clone)]
enum Type193 {
    gif(Type19),
    gzip(Vec<Type40>),
    jpeg(Type80),
    mpeg4(Type163),
    png(Type181),
    riff(Type185),
    tar(Type191),
    text(Type192),
}

#[derive(Debug, Clone)]
enum Type194 {
    none,
    some(u8),
}

#[derive(Debug, Clone)]
struct Type195 {
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
struct Type196 {
    data: Type193,
    end: (),
}

fn Decoder0<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type196> {
    (Some((Decoder1(scope, input))?))
}

fn Decoder1<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type196> {
    let data = { (unimplemented!(r#"ParallelLogic::Alts.to_ast(..)"#)) };
    let end = {
        if ((input.read_byte()).is_none()) {
            ()
        } else {
            return None;
        }
    };
    (Some(Type196 { data, end }))
}

fn Decoder2<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type19> {
    let header = { (Decoder151(scope, input))? };
    let logical_screen = { (Decoder152(scope, input))? };
    let blocks = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    33 => 0,

                    44 => 0,

                    59 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder153(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let trailer = { (Decoder154(scope, input))? };
    (Some(Type19 {
        header,
        logical_screen,
        blocks,
        trailer,
    }))
}

fn Decoder3<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<Type40>> {
    let mut accum = (Vec::new());
    while true {
        let matching_ix = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            if (b == 31) {
                1
            } else {
                0
            }
        };
        if (matching_ix == 0) {
            break;
        } else {
            let next_elem = {
                let header = { (Decoder140(scope, input))? };
                let fname = {
                    match (header.file_flags & 8 != 0) {
                        true => {
                            let inner = (Decoder141(scope, input))?;
                            (Type22::yes(inner))
                        }

                        false => {
                            let _ = ();
                            Type22::no
                        }

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                };
                let data = { (unimplemented!(r#"translate @ Decoder::Bits"#)) };
                let footer = { (Decoder143(scope, input))? };
                Type40 {
                    header,
                    fname,
                    data,
                    footer,
                }
            };
            (accum.push(next_elem));
        }
    }
    (Some(accum))
}

fn Decoder4<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type80> {
    let soi = { (Decoder67(scope, input))? };
    let frame = { (Decoder68(scope, input))? };
    let eoi = { (Decoder69(scope, input))? };
    (Some(Type80 { soi, frame, eoi }))
}

fn Decoder5<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type163> {
    let atoms = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                0
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder46(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type163 { atoms }))
}

fn Decoder6<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type181> {
    let signature = { (Decoder28(scope, input))? };
    let ihdr = { (Decoder29(scope, input))? };
    let chunks = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    98 => 0,

                    112 => 0,

                    80 => 0,

                    116 => 0,

                    73 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder30(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let idat = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    73 => {
                        let b = (lookahead.read_byte())?;
                        match b {
                            69 => 0,

                            68 => 1,

                            _other => {
                                (unreachable!(r#"unexpected: {:?}"#, _other));
                            }
                        }
                    }

                    98 => 0,

                    112 => 0,

                    80 => 0,

                    116 => 0,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                break;
            } else {
                let next_elem = (Decoder31(scope, input))?;
                (accum.push(next_elem));
            }
        }
        accum
    };
    let more_chunks = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    98 => 0,

                    112 => 0,

                    80 => 0,

                    116 => 0,

                    73 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder30(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let iend = { (Decoder32(scope, input))? };
    (Some(Type181 {
        signature,
        ihdr,
        chunks,
        idat,
        more_chunks,
        iend,
    }))
}

fn Decoder7<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type185> {
    let tag = {
        let field0 = {
            let b = (input.read_byte())?;
            if (b == 82) {
                b
            } else {
                return None;
            }
        };
        let field1 = {
            let b = (input.read_byte())?;
            if (b == 73) {
                b
            } else {
                return None;
            }
        };
        let field2 = {
            let b = (input.read_byte())?;
            if (b == 70) {
                b
            } else {
                return None;
            }
        };
        let field3 = {
            let b = (input.read_byte())?;
            if (b == 70) {
                b
            } else {
                return None;
            }
        };
        (field0, field1, field2, field3)
    };
    let length = { (Decoder24(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let pad = {
        match (length % 2 == 0) {
            true => {
                let _ = ();
                Type182::yes
            }

            false => {
                let inner = {
                    let b = (input.read_byte())?;
                    if (b == 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (Type182::no(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(Type185 {
        tag,
        length,
        data,
        pad,
    }))
}

fn Decoder8<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type191> {
    let contents = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    0 => 0,

                    tmp if (tmp != 0) => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                break;
            } else {
                let next_elem = (Decoder15(scope, input))?;
                (accum.push(next_elem));
            }
        }
        accum
    };
    let __padding = {
        let mut accum = (Vec::new());
        for _ in 0..1024 {
            (accum.push({
                let b = (input.read_byte())?;
                if (b == 0) {
                    b
                } else {
                    return None;
                }
            }));
        }
        accum
    };
    let __trailing = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                if (b == 0) {
                    0
                } else {
                    1
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b == 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type191 {
        contents,
        __padding,
        __trailing,
    }))
}

fn Decoder9<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type192> {
    (Some((unimplemented!(r#"ParallelLogic::Alts.to_ast(..)"#))))
}

fn Decoder10<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<u8>> {
    let mut accum = (Vec::new());
    while true {
        let matching_ix = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            if ((ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0]))
                .contains(b))
            {
                1
            } else {
                0
            }
        };
        if (matching_ix == 0) {
            break;
        } else {
            let next_elem = (Decoder14(scope, input))?;
            (accum.push(next_elem));
        }
    }
    (Some(accum))
}

fn Decoder11<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<char>> {
    let mut accum = (Vec::new());
    while true {
        let matching_ix = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            match b {
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

                224 => 0,

                tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => 0,

                237 => 0,

                tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => 0,

                240 => 0,

                tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => 0,

                244 => 0,

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }
        };
        if (matching_ix == 0) {
            let next_elem = (Decoder12(scope, input))?;
            (accum.push(next_elem));
        } else {
            break;
        }
    }
    (Some(accum))
}

fn Decoder12<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<char> {
    let inner = {
        let tree_index = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            match b {
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

                224 => 2,

                tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => 2,

                237 => 2,

                tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => 2,

                240 => 3,

                tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => 3,

                244 => 3,

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }
        };
        match tree_index {
            0 => {
                let inner = {
                    let b = (input.read_byte())?;
                    if ((ByteSet::from_bits([18446744073709551615, 18446744073709551615, 0, 0]))
                        .contains(b))
                    {
                        b
                    } else {
                        return None;
                    }
                };
                ((|byte: u8| byte as u32)(inner))
            }

            1 => {
                let inner = {
                    let field0 = {
                        let inner = {
                            let b = (input.read_byte())?;
                            if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(b)) {
                                b
                            } else {
                                return None;
                            }
                        };
                        ((|raw: u8| raw & 31)(inner))
                    };
                    let field1 = { (Decoder13(scope, input))? };
                    (field0, field1)
                };
                ((|bytes: (u8, u8)| match bytes {
                    (x1, x0) => ((x1 as u32) << 6 | (x0 as u32)),

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                })(inner))
            }

            2 => {
                let inner = {
                    let tree_index = {
                        let lookahead = &mut (input.clone());
                        let b = (lookahead.read_byte())?;
                        match b {
                            224 => 0,

                            tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240]))
                                .contains(tmp)) =>
                            {
                                1
                            }

                            237 => 2,

                            tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992]))
                                .contains(tmp)) =>
                            {
                                3
                            }

                            _other => {
                                (unreachable!(r#"unexpected: {:?}"#, _other));
                            }
                        }
                    };
                    match tree_index {
                        0 => {
                            let field0 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if (b == 224) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 15)(inner))
                            };
                            let field1 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if ((ByteSet::from_bits([0, 0, 18446744069414584320, 0]))
                                        .contains(b))
                                    {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 63)(inner))
                            };
                            let field2 = { (Decoder13(scope, input))? };
                            (field0, field1, field2)
                        }

                        1 => {
                            let field0 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(b))
                                    {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 15)(inner))
                            };
                            let field1 = { (Decoder13(scope, input))? };
                            let field2 = { (Decoder13(scope, input))? };
                            (field0, field1, field2)
                        }

                        2 => {
                            let field0 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if (b == 237) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 15)(inner))
                            };
                            let field1 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if ((ByteSet::from_bits([0, 0, 4294967295, 0])).contains(b)) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 63)(inner))
                            };
                            let field2 = { (Decoder13(scope, input))? };
                            (field0, field1, field2)
                        }

                        3 => {
                            let field0 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if ((ByteSet::from_bits([0, 0, 0, 211106232532992]))
                                        .contains(b))
                                    {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 15)(inner))
                            };
                            let field1 = { (Decoder13(scope, input))? };
                            let field2 = { (Decoder13(scope, input))? };
                            (field0, field1, field2)
                        }

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                };
                ((|bytes: (u8, u8, u8)| match bytes {
                    (x2, x1, x0) => ((x2 as u32) << 12 | (x1 as u32) << 6 | (x0 as u32)),

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                })(inner))
            }

            3 => {
                let inner = {
                    let tree_index = {
                        let lookahead = &mut (input.clone());
                        let b = (lookahead.read_byte())?;
                        match b {
                            240 => 0,

                            tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184]))
                                .contains(tmp)) =>
                            {
                                1
                            }

                            244 => 2,

                            _other => {
                                (unreachable!(r#"unexpected: {:?}"#, _other));
                            }
                        }
                    };
                    match tree_index {
                        0 => {
                            let field0 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if (b == 240) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 7)(inner))
                            };
                            let field1 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if ((ByteSet::from_bits([0, 0, 18446744073709486080, 0]))
                                        .contains(b))
                                    {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 63)(inner))
                            };
                            let field2 = { (Decoder13(scope, input))? };
                            let field3 = { (Decoder13(scope, input))? };
                            (field0, field1, field2, field3)
                        }

                        1 => {
                            let field0 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if ((ByteSet::from_bits([0, 0, 0, 3940649673949184]))
                                        .contains(b))
                                    {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 7)(inner))
                            };
                            let field1 = { (Decoder13(scope, input))? };
                            let field2 = { (Decoder13(scope, input))? };
                            let field3 = { (Decoder13(scope, input))? };
                            (field0, field1, field2, field3)
                        }

                        2 => {
                            let field0 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if (b == 244) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 7)(inner))
                            };
                            let field1 = {
                                let inner = {
                                    let b = (input.read_byte())?;
                                    if ((ByteSet::from_bits([0, 0, 65535, 0])).contains(b)) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                ((|raw: u8| raw & 63)(inner))
                            };
                            let field2 = { (Decoder13(scope, input))? };
                            let field3 = { (Decoder13(scope, input))? };
                            (field0, field1, field2, field3)
                        }

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                };
                ((|bytes: (u8, u8, u8, u8)| match bytes {
                    (x3, x2, x1, x0) => {
                        ((x3 as u32) << 18 | (x2 as u32) << 12 | (x1 as u32) << 6 | (x0 as u32))
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                })(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(((|codepoint: u32| (char::from_u32(codepoint)).unwrap())(inner))))
}

fn Decoder13<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let inner = {
        let b = (input.read_byte())?;
        if ((ByteSet::from_bits([0, 0, 18446744073709551615, 0])).contains(b)) {
            b
        } else {
            return None;
        }
    };
    (Some(((|raw: u8| raw & 63)(inner))))
}

fn Decoder14<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(
        if ((ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0])).contains(b)) {
            b
        } else {
            return None;
        },
    ))
}

fn Decoder15<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type190> {
    let header = { (Decoder16(scope, input))? };
    let file = {
        let mut accum = (Vec::new());
        for _ in 0..header.size {
            (accum.push((Decoder17(scope, input))?));
        }
        accum
    };
    let __padding = {
        while (input.offset() % 512 != 0) {
            let _ = (input.read_byte())?;
        }
        ()
    };
    (Some(Type190 {
        header,
        file,
        __padding,
    }))
}

fn Decoder16<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type189> {
    (Some((unimplemented!(r#"translate @ Decoder::Slice"#))))
}

fn Decoder17<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(b))
}

fn Decoder18<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type186> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    0 => 0,

                    tmp if (tmp != 0) => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                break;
            } else {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            }
        }
        accum
    };
    let __padding = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                if (b == 0) {
                    0
                } else {
                    1
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b == 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type186 { string, __padding }))
}

fn Decoder19<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(
        if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(b)) {
            b
        } else {
            return None;
        },
    ))
}

fn Decoder20<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(
        if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(b)) {
            b
        } else {
            return None;
        },
    ))
}

fn Decoder21<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(b))
}

fn Decoder22<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type186> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let __padding = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                if (b == 0) {
                    0
                } else {
                    1
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b == 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type186 { string, __padding }))
}

fn Decoder23<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type188> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let padding = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                if (b == 0) {
                    1
                } else {
                    0
                }
            };
            if (matching_ix == 0) {
                break;
            } else {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b == 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            }
        }
        accum
    };
    (Some(Type188 { string, padding }))
}

fn Decoder24<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u32> {
    let inner = {
        let field0 = { (Decoder17(scope, input))? };
        let field1 = { (Decoder17(scope, input))? };
        let field2 = { (Decoder17(scope, input))? };
        let field3 = { (Decoder17(scope, input))? };
        (field0, field1, field2, field3)
    };
    (Some(((|x: (u8, u8, u8, u8)| u32le(x))(inner))))
}

fn Decoder25<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type184> {
    let tag = { (Decoder26(scope, input))? };
    let chunks = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                0
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder27(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type184 { tag, chunks }))
}

fn Decoder26<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
    let field0 = { (Decoder21(scope, input))? };
    let field1 = { (Decoder21(scope, input))? };
    let field2 = { (Decoder21(scope, input))? };
    let field3 = { (Decoder21(scope, input))? };
    (Some((field0, field1, field2, field3)))
}

fn Decoder27<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type183> {
    let tag = { (Decoder26(scope, input))? };
    let length = { (Decoder24(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let pad = {
        match (length % 2 == 0) {
            true => {
                let _ = ();
                Type182::yes
            }

            false => {
                let inner = {
                    let b = (input.read_byte())?;
                    if (b == 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (Type182::no(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(Type183 {
        tag,
        length,
        data,
        pad,
    }))
}

fn Decoder28<'input>(
    scope: &mut Scope,
    input: &mut ParseCtxt<'input>,
) -> Option<(u8, u8, u8, u8, u8, u8, u8, u8)> {
    let field0 = {
        let b = (input.read_byte())?;
        if (b == 137) {
            b
        } else {
            return None;
        }
    };
    let field1 = {
        let b = (input.read_byte())?;
        if (b == 80) {
            b
        } else {
            return None;
        }
    };
    let field2 = {
        let b = (input.read_byte())?;
        if (b == 78) {
            b
        } else {
            return None;
        }
    };
    let field3 = {
        let b = (input.read_byte())?;
        if (b == 71) {
            b
        } else {
            return None;
        }
    };
    let field4 = {
        let b = (input.read_byte())?;
        if (b == 13) {
            b
        } else {
            return None;
        }
    };
    let field5 = {
        let b = (input.read_byte())?;
        if (b == 10) {
            b
        } else {
            return None;
        }
    };
    let field6 = {
        let b = (input.read_byte())?;
        if (b == 26) {
            b
        } else {
            return None;
        }
    };
    let field7 = {
        let b = (input.read_byte())?;
        if (b == 10) {
            b
        } else {
            return None;
        }
    };
    (Some((
        field0, field1, field2, field3, field4, field5, field6, field7,
    )))
}

fn Decoder29<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type165> {
    let length = { (Decoder33(scope, input))? };
    let tag = { (Decoder44(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder33(scope, input))? };
    (Some(Type165 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder30<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type178> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        match b {
            98 => 0,

            112 => 1,

            80 => 2,

            116 => {
                let b = (lookahead.read_byte())?;
                match b {
                    73 => 3,

                    82 => 4,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder38(scope, input))?;
            (Type178::bKGD(inner))
        }

        1 => {
            let inner = (Decoder39(scope, input))?;
            (Type178::pHYs(inner))
        }

        2 => {
            let inner = (Decoder40(scope, input))?;
            (Type178::PLTE(inner))
        }

        3 => {
            let inner = (Decoder41(scope, input))?;
            (Type178::tIME(inner))
        }

        4 => {
            let inner = (Decoder42(scope, input))?;
            (Type178::tRNS(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder31<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type179> {
    let length = { (Decoder33(scope, input))? };
    let tag = { (Decoder36(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder33(scope, input))? };
    (Some(Type179 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder32<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type180> {
    let length = { (Decoder33(scope, input))? };
    let tag = { (Decoder34(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder33(scope, input))? };
    (Some(Type180 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder33<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u32> {
    let inner = {
        let field0 = { (Decoder17(scope, input))? };
        let field1 = { (Decoder17(scope, input))? };
        let field2 = { (Decoder17(scope, input))? };
        let field3 = { (Decoder17(scope, input))? };
        (field0, field1, field2, field3)
    };
    (Some(((|x: (u8, u8, u8, u8)| u32be(x))(inner))))
}

fn Decoder34<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
    let field0 = {
        let b = (input.read_byte())?;
        if (b == 73) {
            b
        } else {
            return None;
        }
    };
    let field1 = {
        let b = (input.read_byte())?;
        if (b == 69) {
            b
        } else {
            return None;
        }
    };
    let field2 = {
        let b = (input.read_byte())?;
        if (b == 78) {
            b
        } else {
            return None;
        }
    };
    let field3 = {
        let b = (input.read_byte())?;
        if (b == 68) {
            b
        } else {
            return None;
        }
    };
    (Some((field0, field1, field2, field3)))
}

fn Decoder35<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<()> {
    (Some(()))
}

fn Decoder36<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
    let field0 = {
        let b = (input.read_byte())?;
        if (b == 73) {
            b
        } else {
            return None;
        }
    };
    let field1 = {
        let b = (input.read_byte())?;
        if (b == 68) {
            b
        } else {
            return None;
        }
    };
    let field2 = {
        let b = (input.read_byte())?;
        if (b == 65) {
            b
        } else {
            return None;
        }
    };
    let field3 = {
        let b = (input.read_byte())?;
        if (b == 84) {
            b
        } else {
            return None;
        }
    };
    (Some((field0, field1, field2, field3)))
}

fn Decoder37<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<u8>> {
    let mut accum = (Vec::new());
    while true {
        let matching_ix = {
            let lookahead = &mut (input.clone());
            0
        };
        if (matching_ix == 0) {
            let next_elem = (Decoder17(scope, input))?;
            (accum.push(next_elem));
        } else {
            break;
        }
    }
    (Some(accum))
}

fn Decoder38<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type171> {
    let length = { (Decoder33(scope, input))? };
    let tag = {
        let field0 = {
            let b = (input.read_byte())?;
            if (b == 98) {
                b
            } else {
                return None;
            }
        };
        let field1 = {
            let b = (input.read_byte())?;
            if (b == 75) {
                b
            } else {
                return None;
            }
        };
        let field2 = {
            let b = (input.read_byte())?;
            if (b == 71) {
                b
            } else {
                return None;
            }
        };
        let field3 = {
            let b = (input.read_byte())?;
            if (b == 68) {
                b
            } else {
                return None;
            }
        };
        (field0, field1, field2, field3)
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder33(scope, input))? };
    (Some(Type171 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder39<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type173> {
    let length = { (Decoder33(scope, input))? };
    let tag = {
        let field0 = {
            let b = (input.read_byte())?;
            if (b == 112) {
                b
            } else {
                return None;
            }
        };
        let field1 = {
            let b = (input.read_byte())?;
            if (b == 72) {
                b
            } else {
                return None;
            }
        };
        let field2 = {
            let b = (input.read_byte())?;
            if (b == 89) {
                b
            } else {
                return None;
            }
        };
        let field3 = {
            let b = (input.read_byte())?;
            if (b == 115) {
                b
            } else {
                return None;
            }
        };
        (field0, field1, field2, field3)
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder33(scope, input))? };
    (Some(Type173 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder40<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type166> {
    let length = { (Decoder33(scope, input))? };
    let tag = {
        let field0 = {
            let b = (input.read_byte())?;
            if (b == 80) {
                b
            } else {
                return None;
            }
        };
        let field1 = {
            let b = (input.read_byte())?;
            if (b == 76) {
                b
            } else {
                return None;
            }
        };
        let field2 = {
            let b = (input.read_byte())?;
            if (b == 84) {
                b
            } else {
                return None;
            }
        };
        let field3 = {
            let b = (input.read_byte())?;
            if (b == 69) {
                b
            } else {
                return None;
            }
        };
        (field0, field1, field2, field3)
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder33(scope, input))? };
    (Some(Type166 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder41<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type175> {
    let length = { (Decoder33(scope, input))? };
    let tag = {
        let field0 = {
            let b = (input.read_byte())?;
            if (b == 116) {
                b
            } else {
                return None;
            }
        };
        let field1 = {
            let b = (input.read_byte())?;
            if (b == 73) {
                b
            } else {
                return None;
            }
        };
        let field2 = {
            let b = (input.read_byte())?;
            if (b == 77) {
                b
            } else {
                return None;
            }
        };
        let field3 = {
            let b = (input.read_byte())?;
            if (b == 69) {
                b
            } else {
                return None;
            }
        };
        (field0, field1, field2, field3)
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder33(scope, input))? };
    (Some(Type175 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder42<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type177> {
    let length = { (Decoder33(scope, input))? };
    let tag = {
        let field0 = {
            let b = (input.read_byte())?;
            if (b == 116) {
                b
            } else {
                return None;
            }
        };
        let field1 = {
            let b = (input.read_byte())?;
            if (b == 82) {
                b
            } else {
                return None;
            }
        };
        let field2 = {
            let b = (input.read_byte())?;
            if (b == 78) {
                b
            } else {
                return None;
            }
        };
        let field3 = {
            let b = (input.read_byte())?;
            if (b == 83) {
                b
            } else {
                return None;
            }
        };
        (field0, field1, field2, field3)
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder33(scope, input))? };
    (Some(Type177 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder43<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u16> {
    let inner = {
        let field0 = { (Decoder17(scope, input))? };
        let field1 = { (Decoder17(scope, input))? };
        (field0, field1)
    };
    (Some(((|x: (u8, u8)| u16be(x))(inner))))
}

fn Decoder44<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
    let field0 = {
        let b = (input.read_byte())?;
        if (b == 73) {
            b
        } else {
            return None;
        }
    };
    let field1 = {
        let b = (input.read_byte())?;
        if (b == 72) {
            b
        } else {
            return None;
        }
    };
    let field2 = {
        let b = (input.read_byte())?;
        if (b == 68) {
            b
        } else {
            return None;
        }
    };
    let field3 = {
        let b = (input.read_byte())?;
        if (b == 82) {
            b
        } else {
            return None;
        }
    };
    (Some((field0, field1, field2, field3)))
}

fn Decoder45<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type164> {
    let width = { (Decoder33(scope, input))? };
    let height = { (Decoder33(scope, input))? };
    let bit_depth = { (Decoder17(scope, input))? };
    let color_type = { (Decoder17(scope, input))? };
    let compression_method = { (Decoder17(scope, input))? };
    let filter_method = { (Decoder17(scope, input))? };
    let interlace_method = { (Decoder17(scope, input))? };
    (Some(Type164 {
        width,
        height,
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
    }))
}

fn Decoder46<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type162> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type162 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder47<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
    let field0 = { (Decoder21(scope, input))? };
    let field1 = { (Decoder21(scope, input))? };
    let field2 = { (Decoder21(scope, input))? };
    let field3 = { (Decoder21(scope, input))? };
    (Some((field0, field1, field2, field3)))
}

fn Decoder48<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u64> {
    let inner = {
        let field0 = { (Decoder17(scope, input))? };
        let field1 = { (Decoder17(scope, input))? };
        let field2 = { (Decoder17(scope, input))? };
        let field3 = { (Decoder17(scope, input))? };
        let field4 = { (Decoder17(scope, input))? };
        let field5 = { (Decoder17(scope, input))? };
        let field6 = { (Decoder17(scope, input))? };
        let field7 = { (Decoder17(scope, input))? };
        (
            field0, field1, field2, field3, field4, field5, field6, field7,
        )
    };
    (Some(((|x: (u8, u8, u8, u8, u8, u8, u8, u8)| u64be(x))(inner))))
}

fn Decoder49<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type115> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type115 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder50<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type160> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type160 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder51<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type156> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type156 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder52<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type158> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type158 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder53<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type123> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type123 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder54<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type150> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type150 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder55<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let null = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    (Some(Type21 { string, null }))
}

fn Decoder56<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type148> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type148 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder57<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type85> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type85 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder58<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type145> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type145 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder59<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type95 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder60<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type105> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type105 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder61<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type103> {
    let size_field = { (Decoder33(scope, input))? };
    let r#type = { (Decoder47(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder48(scope, input))?;
                ((|x: u64| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type103 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder62<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let null = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    (Some(Type21 { string, null }))
}

fn Decoder63<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let null = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    (Some(Type21 { string, null }))
}

fn Decoder64<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let null = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    (Some(Type21 { string, null }))
}

fn Decoder65<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let null = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    (Some(Type21 { string, null }))
}

fn Decoder66<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let null = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    (Some(Type21 { string, null }))
}

fn Decoder67<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let ff = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let marker = {
        let b = (input.read_byte())?;
        if (b == 216) {
            b
        } else {
            return None;
        }
    };
    (Some(Type41 { ff, marker }))
}

fn Decoder68<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type79> {
    let initial_segment = {
        let tree_index = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            if (b == 255) {
                let b = (lookahead.read_byte())?;
                match b {
                    224 => 0,

                    225 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            } else {
                return None;
            }
        };
        match tree_index {
            0 => {
                let inner = (Decoder70(scope, input))?;
                (Type55::app0(inner))
            }

            1 => {
                let inner = (Decoder71(scope, input))?;
                (Type55::app1(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let segments = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                if (b == 255) {
                    let b = (lookahead.read_byte())?;
                    match b {
                        219 => 0,

                        196 => 0,

                        204 => 0,

                        221 => 0,

                        224 => 0,

                        225 => 0,

                        226 => 0,

                        227 => 0,

                        228 => 0,

                        229 => 0,

                        230 => 0,

                        231 => 0,

                        232 => 0,

                        233 => 0,

                        234 => 0,

                        235 => 0,

                        236 => 0,

                        237 => 0,

                        238 => 0,

                        239 => 0,

                        254 => 0,

                        192 => 1,

                        193 => 1,

                        194 => 1,

                        195 => 1,

                        197 => 1,

                        198 => 1,

                        199 => 1,

                        201 => 1,

                        202 => 1,

                        203 => 1,

                        205 => 1,

                        206 => 1,

                        207 => 1,

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                } else {
                    return None;
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder72(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let header = { (Decoder73(scope, input))? };
    let scan = { (Decoder74(scope, input))? };
    let dnl = {
        let tree_index = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            if (b == 255) {
                let b = (lookahead.read_byte())?;
                match b {
                    220 => 0,

                    217 => 1,

                    218 => 1,

                    219 => 1,

                    196 => 1,

                    204 => 1,

                    221 => 1,

                    224 => 1,

                    225 => 1,

                    226 => 1,

                    227 => 1,

                    228 => 1,

                    229 => 1,

                    230 => 1,

                    231 => 1,

                    232 => 1,

                    233 => 1,

                    234 => 1,

                    235 => 1,

                    236 => 1,

                    237 => 1,

                    238 => 1,

                    239 => 1,

                    254 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            } else {
                return None;
            }
        };
        match tree_index {
            0 => {
                let inner = (Decoder75(scope, input))?;
                (Type78::some(inner))
            }

            1 => {
                let _ = ();
                Type78::none
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let scans = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                if (b == 255) {
                    let b = (lookahead.read_byte())?;
                    match b {
                        218 => 0,

                        219 => 0,

                        196 => 0,

                        204 => 0,

                        221 => 0,

                        224 => 0,

                        225 => 0,

                        226 => 0,

                        227 => 0,

                        228 => 0,

                        229 => 0,

                        230 => 0,

                        231 => 0,

                        232 => 0,

                        233 => 0,

                        234 => 0,

                        235 => 0,

                        236 => 0,

                        237 => 0,

                        238 => 0,

                        239 => 0,

                        254 => 0,

                        217 => 1,

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                } else {
                    return None;
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder76(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type79 {
        initial_segment,
        segments,
        header,
        scan,
        dnl,
        scans,
    }))
}

fn Decoder69<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let ff = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let marker = {
        let b = (input.read_byte())?;
        if (b == 217) {
            b
        } else {
            return None;
        }
    };
    (Some(Type41 { ff, marker }))
}

fn Decoder70<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type45> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 224) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type45 {
        marker,
        length,
        data,
    }))
}

fn Decoder71<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type54> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 225) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type54 {
        marker,
        length,
        data,
    }))
}

fn Decoder72<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        if (b == 255) {
            let b = (lookahead.read_byte())?;
            match b {
                219 => 0,

                196 => 1,

                204 => 2,

                221 => 3,

                224 => 4,

                225 => 5,

                226 => 6,

                227 => 7,

                228 => 8,

                229 => 9,

                230 => 10,

                231 => 11,

                232 => 12,

                233 => 13,

                234 => 14,

                235 => 15,

                236 => 16,

                237 => 17,

                238 => 18,

                239 => 19,

                254 => 20,

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }
        } else {
            return None;
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder107(scope, input))?;
            (Type65::dqt(inner))
        }

        1 => {
            let inner = (Decoder108(scope, input))?;
            (Type65::dht(inner))
        }

        2 => {
            let inner = (Decoder109(scope, input))?;
            (Type65::dac(inner))
        }

        3 => {
            let inner = (Decoder110(scope, input))?;
            (Type65::dri(inner))
        }

        4 => {
            let inner = (Decoder70(scope, input))?;
            (Type65::app0(inner))
        }

        5 => {
            let inner = (Decoder71(scope, input))?;
            (Type65::app1(inner))
        }

        6 => {
            let inner = (Decoder111(scope, input))?;
            (Type65::app2(inner))
        }

        7 => {
            let inner = (Decoder112(scope, input))?;
            (Type65::app3(inner))
        }

        8 => {
            let inner = (Decoder113(scope, input))?;
            (Type65::app4(inner))
        }

        9 => {
            let inner = (Decoder114(scope, input))?;
            (Type65::app5(inner))
        }

        10 => {
            let inner = (Decoder115(scope, input))?;
            (Type65::app6(inner))
        }

        11 => {
            let inner = (Decoder116(scope, input))?;
            (Type65::app7(inner))
        }

        12 => {
            let inner = (Decoder117(scope, input))?;
            (Type65::app8(inner))
        }

        13 => {
            let inner = (Decoder118(scope, input))?;
            (Type65::app9(inner))
        }

        14 => {
            let inner = (Decoder119(scope, input))?;
            (Type65::app10(inner))
        }

        15 => {
            let inner = (Decoder120(scope, input))?;
            (Type65::app11(inner))
        }

        16 => {
            let inner = (Decoder121(scope, input))?;
            (Type65::app12(inner))
        }

        17 => {
            let inner = (Decoder122(scope, input))?;
            (Type65::app13(inner))
        }

        18 => {
            let inner = (Decoder123(scope, input))?;
            (Type65::app14(inner))
        }

        19 => {
            let inner = (Decoder124(scope, input))?;
            (Type65::app15(inner))
        }

        20 => {
            let inner = (Decoder125(scope, input))?;
            (Type65::com(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder73<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        if (b == 255) {
            let b = (lookahead.read_byte())?;
            match b {
                192 => 0,

                193 => 1,

                194 => 2,

                195 => 3,

                197 => 4,

                198 => 5,

                199 => 6,

                201 => 7,

                202 => 8,

                203 => 9,

                205 => 10,

                206 => 11,

                207 => 12,

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }
        } else {
            return None;
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder92(scope, input))?;
            (Type69::sof0(inner))
        }

        1 => {
            let inner = (Decoder93(scope, input))?;
            (Type69::sof1(inner))
        }

        2 => {
            let inner = (Decoder94(scope, input))?;
            (Type69::sof2(inner))
        }

        3 => {
            let inner = (Decoder95(scope, input))?;
            (Type69::sof3(inner))
        }

        4 => {
            let inner = (Decoder96(scope, input))?;
            (Type69::sof5(inner))
        }

        5 => {
            let inner = (Decoder97(scope, input))?;
            (Type69::sof6(inner))
        }

        6 => {
            let inner = (Decoder98(scope, input))?;
            (Type69::sof7(inner))
        }

        7 => {
            let inner = (Decoder99(scope, input))?;
            (Type69::sof9(inner))
        }

        8 => {
            let inner = (Decoder100(scope, input))?;
            (Type69::sof10(inner))
        }

        9 => {
            let inner = (Decoder101(scope, input))?;
            (Type69::sof11(inner))
        }

        10 => {
            let inner = (Decoder102(scope, input))?;
            (Type69::sof13(inner))
        }

        11 => {
            let inner = (Decoder103(scope, input))?;
            (Type69::sof14(inner))
        }

        12 => {
            let inner = (Decoder104(scope, input))?;
            (Type69::sof15(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder74<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type75> {
    let segments = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                if (b == 255) {
                    let b = (lookahead.read_byte())?;
                    match b {
                        219 => 0,

                        196 => 0,

                        204 => 0,

                        221 => 0,

                        224 => 0,

                        225 => 0,

                        226 => 0,

                        227 => 0,

                        228 => 0,

                        229 => 0,

                        230 => 0,

                        231 => 0,

                        232 => 0,

                        233 => 0,

                        234 => 0,

                        235 => 0,

                        236 => 0,

                        237 => 0,

                        238 => 0,

                        239 => 0,

                        254 => 0,

                        218 => 1,

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                } else {
                    return None;
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder72(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let sos = { (Decoder77(scope, input))? };
    let data = { (Decoder91(scope, input))? };
    (Some(Type75 {
        segments,
        sos,
        data,
    }))
}

fn Decoder75<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type77> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 220) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type77 {
        marker,
        length,
        data,
    }))
}

fn Decoder76<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type75> {
    let segments = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                if (b == 255) {
                    let b = (lookahead.read_byte())?;
                    match b {
                        219 => 0,

                        196 => 0,

                        204 => 0,

                        221 => 0,

                        224 => 0,

                        225 => 0,

                        226 => 0,

                        227 => 0,

                        228 => 0,

                        229 => 0,

                        230 => 0,

                        231 => 0,

                        232 => 0,

                        233 => 0,

                        234 => 0,

                        235 => 0,

                        236 => 0,

                        237 => 0,

                        238 => 0,

                        239 => 0,

                        254 => 0,

                        218 => 1,

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                } else {
                    return None;
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder72(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let sos = { (Decoder77(scope, input))? };
    let data = { (Decoder78(scope, input))? };
    (Some(Type75 {
        segments,
        sos,
        data,
    }))
}

fn Decoder77<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type72> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 218) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type72 {
        marker,
        length,
        data,
    }))
}

fn Decoder78<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type74> {
    let scan_data = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 255) => 0,

                    255 => {
                        let b = (lookahead.read_byte())?;
                        match b {
                            0 => 0,

                            208 => 0,

                            209 => 0,

                            210 => 0,

                            211 => 0,

                            212 => 0,

                            213 => 0,

                            214 => 0,

                            215 => 0,

                            217 => 1,

                            218 => 1,

                            219 => 1,

                            196 => 1,

                            204 => 1,

                            221 => 1,

                            224 => 1,

                            225 => 1,

                            226 => 1,

                            227 => 1,

                            228 => 1,

                            229 => 1,

                            230 => 1,

                            231 => 1,

                            232 => 1,

                            233 => 1,

                            234 => 1,

                            235 => 1,

                            236 => 1,

                            237 => 1,

                            238 => 1,

                            239 => 1,

                            254 => 1,

                            _other => {
                                (unreachable!(r#"unexpected: {:?}"#, _other));
                            }
                        }
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let tree_index = {
                        let lookahead = &mut (input.clone());
                        let b = (lookahead.read_byte())?;
                        match b {
                            tmp if (tmp != 255) => 0,

                            255 => {
                                let b = (lookahead.read_byte())?;
                                match b {
                                    0 => 0,

                                    208 => 1,

                                    209 => 2,

                                    210 => 3,

                                    211 => 4,

                                    212 => 5,

                                    213 => 6,

                                    214 => 7,

                                    215 => 8,

                                    _other => {
                                        (unreachable!(r#"unexpected: {:?}"#, _other));
                                    }
                                }
                            }

                            _other => {
                                (unreachable!(r#"unexpected: {:?}"#, _other));
                            }
                        }
                    };
                    match tree_index {
                        0 => {
                            let inner = (Decoder79(scope, input))?;
                            (Type73::mcu(inner))
                        }

                        1 => {
                            let inner = (Decoder80(scope, input))?;
                            (Type73::rst0(inner))
                        }

                        2 => {
                            let inner = (Decoder81(scope, input))?;
                            (Type73::rst1(inner))
                        }

                        3 => {
                            let inner = (Decoder82(scope, input))?;
                            (Type73::rst2(inner))
                        }

                        4 => {
                            let inner = (Decoder83(scope, input))?;
                            (Type73::rst3(inner))
                        }

                        5 => {
                            let inner = (Decoder84(scope, input))?;
                            (Type73::rst4(inner))
                        }

                        6 => {
                            let inner = (Decoder85(scope, input))?;
                            (Type73::rst5(inner))
                        }

                        7 => {
                            let inner = (Decoder86(scope, input))?;
                            (Type73::rst6(inner))
                        }

                        8 => {
                            let inner = (Decoder87(scope, input))?;
                            (Type73::rst7(inner))
                        }

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let scan_data_stream = {
        ((((scan_data.iter()).cloned()).flat_map(
            (|x: Type73| match x {
                Type73::mcu(v) => ([v].to_vec()),

                Type73::rst0(..) => ([].to_vec()),

                Type73::rst1(..) => ([].to_vec()),

                Type73::rst2(..) => ([].to_vec()),

                Type73::rst3(..) => ([].to_vec()),

                Type73::rst4(..) => ([].to_vec()),

                Type73::rst5(..) => ([].to_vec()),

                Type73::rst6(..) => ([].to_vec()),

                Type73::rst7(..) => ([].to_vec()),

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }),
        ))
        .collect())
    };
    (Some(Type74 {
        scan_data,
        scan_data_stream,
    }))
}

fn Decoder79<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        match b {
            tmp if (tmp != 255) => 0,

            255 => 1,

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(match tree_index {
        0 => {
            let b = (input.read_byte())?;
            if (b != 255) {
                b
            } else {
                return None;
            }
        }

        1 => {
            let inner = {
                let field0 = {
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return None;
                    }
                };
                let field1 = {
                    let b = (input.read_byte())?;
                    if (b == 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (field0, field1)
            };
            ((|_: (u8, u8)| 255)(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder80<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let ff = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let marker = {
        let b = (input.read_byte())?;
        if (b == 208) {
            b
        } else {
            return None;
        }
    };
    (Some(Type41 { ff, marker }))
}

fn Decoder81<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let ff = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let marker = {
        let b = (input.read_byte())?;
        if (b == 209) {
            b
        } else {
            return None;
        }
    };
    (Some(Type41 { ff, marker }))
}

fn Decoder82<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let ff = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let marker = {
        let b = (input.read_byte())?;
        if (b == 210) {
            b
        } else {
            return None;
        }
    };
    (Some(Type41 { ff, marker }))
}

fn Decoder83<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let ff = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let marker = {
        let b = (input.read_byte())?;
        if (b == 211) {
            b
        } else {
            return None;
        }
    };
    (Some(Type41 { ff, marker }))
}

fn Decoder84<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let ff = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let marker = {
        let b = (input.read_byte())?;
        if (b == 212) {
            b
        } else {
            return None;
        }
    };
    (Some(Type41 { ff, marker }))
}

fn Decoder85<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let ff = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let marker = {
        let b = (input.read_byte())?;
        if (b == 213) {
            b
        } else {
            return None;
        }
    };
    (Some(Type41 { ff, marker }))
}

fn Decoder86<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let ff = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let marker = {
        let b = (input.read_byte())?;
        if (b == 214) {
            b
        } else {
            return None;
        }
    };
    (Some(Type41 { ff, marker }))
}

fn Decoder87<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let ff = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let marker = {
        let b = (input.read_byte())?;
        if (b == 215) {
            b
        } else {
            return None;
        }
    };
    (Some(Type41 { ff, marker }))
}

fn Decoder88<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type71> {
    let num_image_components = { (Decoder17(scope, input))? };
    let image_components = {
        let mut accum = (Vec::new());
        for _ in 0..num_image_components {
            (accum.push((Decoder89(scope, input))?));
        }
        accum
    };
    let start_spectral_selection = { (Decoder17(scope, input))? };
    let end_spectral_selection = { (Decoder17(scope, input))? };
    let approximation_bit_position = { (Decoder17(scope, input))? };
    (Some(Type71 {
        num_image_components,
        image_components,
        start_spectral_selection,
        end_spectral_selection,
        approximation_bit_position,
    }))
}

fn Decoder89<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type70> {
    let component_selector = { (Decoder17(scope, input))? };
    let entropy_coding_table_ids = { (Decoder17(scope, input))? };
    (Some(Type70 {
        component_selector,
        entropy_coding_table_ids,
    }))
}

fn Decoder90<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type76> {
    let num_lines = { (Decoder43(scope, input))? };
    (Some(Type76 { num_lines }))
}

fn Decoder91<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type74> {
    let scan_data = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 255) => 0,

                    255 => {
                        let b = (lookahead.read_byte())?;
                        match b {
                            0 => 0,

                            208 => 0,

                            209 => 0,

                            210 => 0,

                            211 => 0,

                            212 => 0,

                            213 => 0,

                            214 => 0,

                            215 => 0,

                            220 => 1,

                            217 => 1,

                            218 => 1,

                            219 => 1,

                            196 => 1,

                            204 => 1,

                            221 => 1,

                            224 => 1,

                            225 => 1,

                            226 => 1,

                            227 => 1,

                            228 => 1,

                            229 => 1,

                            230 => 1,

                            231 => 1,

                            232 => 1,

                            233 => 1,

                            234 => 1,

                            235 => 1,

                            236 => 1,

                            237 => 1,

                            238 => 1,

                            239 => 1,

                            254 => 1,

                            _other => {
                                (unreachable!(r#"unexpected: {:?}"#, _other));
                            }
                        }
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let tree_index = {
                        let lookahead = &mut (input.clone());
                        let b = (lookahead.read_byte())?;
                        match b {
                            tmp if (tmp != 255) => 0,

                            255 => {
                                let b = (lookahead.read_byte())?;
                                match b {
                                    0 => 0,

                                    208 => 1,

                                    209 => 2,

                                    210 => 3,

                                    211 => 4,

                                    212 => 5,

                                    213 => 6,

                                    214 => 7,

                                    215 => 8,

                                    _other => {
                                        (unreachable!(r#"unexpected: {:?}"#, _other));
                                    }
                                }
                            }

                            _other => {
                                (unreachable!(r#"unexpected: {:?}"#, _other));
                            }
                        }
                    };
                    match tree_index {
                        0 => {
                            let inner = (Decoder79(scope, input))?;
                            (Type73::mcu(inner))
                        }

                        1 => {
                            let inner = (Decoder80(scope, input))?;
                            (Type73::rst0(inner))
                        }

                        2 => {
                            let inner = (Decoder81(scope, input))?;
                            (Type73::rst1(inner))
                        }

                        3 => {
                            let inner = (Decoder82(scope, input))?;
                            (Type73::rst2(inner))
                        }

                        4 => {
                            let inner = (Decoder83(scope, input))?;
                            (Type73::rst3(inner))
                        }

                        5 => {
                            let inner = (Decoder84(scope, input))?;
                            (Type73::rst4(inner))
                        }

                        6 => {
                            let inner = (Decoder85(scope, input))?;
                            (Type73::rst5(inner))
                        }

                        7 => {
                            let inner = (Decoder86(scope, input))?;
                            (Type73::rst6(inner))
                        }

                        8 => {
                            let inner = (Decoder87(scope, input))?;
                            (Type73::rst7(inner))
                        }

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let scan_data_stream = {
        ((((scan_data.iter()).cloned()).flat_map(
            (|x: Type73| match x {
                Type73::mcu(v) => ([v].to_vec()),

                Type73::rst0(..) => ([].to_vec()),

                Type73::rst1(..) => ([].to_vec()),

                Type73::rst2(..) => ([].to_vec()),

                Type73::rst3(..) => ([].to_vec()),

                Type73::rst4(..) => ([].to_vec()),

                Type73::rst5(..) => ([].to_vec()),

                Type73::rst6(..) => ([].to_vec()),

                Type73::rst7(..) => ([].to_vec()),

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }),
        ))
        .collect())
    };
    (Some(Type74 {
        scan_data,
        scan_data_stream,
    }))
}

fn Decoder92<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 192) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder93<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 193) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder94<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 194) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder95<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 195) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder96<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 197) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder97<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 198) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder98<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 199) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder99<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 201) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder100<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 202) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder101<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 203) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder102<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 205) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder103<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 206) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder104<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 207) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder105<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type67> {
    let sample_precision = { (Decoder17(scope, input))? };
    let num_lines = { (Decoder43(scope, input))? };
    let num_samples_per_line = { (Decoder43(scope, input))? };
    let num_image_components = { (Decoder17(scope, input))? };
    let image_components = {
        let mut accum = (Vec::new());
        for _ in 0..num_image_components {
            (accum.push((Decoder106(scope, input))?));
        }
        accum
    };
    (Some(Type67 {
        sample_precision,
        num_lines,
        num_samples_per_line,
        num_image_components,
        image_components,
    }))
}

fn Decoder106<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type66> {
    let id = { (Decoder17(scope, input))? };
    let sampling_factor = { (Decoder17(scope, input))? };
    let quantization_table_id = { (Decoder17(scope, input))? };
    (Some(Type66 {
        id,
        sampling_factor,
        quantization_table_id,
    }))
}

fn Decoder107<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type62> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 219) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type62 {
        marker,
        length,
        data,
    }))
}

fn Decoder108<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type60> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 196) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type60 {
        marker,
        length,
        data,
    }))
}

fn Decoder109<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type58> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 204) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type58 {
        marker,
        length,
        data,
    }))
}

fn Decoder110<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type64> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 221) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type64 {
        marker,
        length,
        data,
    }))
}

fn Decoder111<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 226) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder112<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 227) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder113<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 228) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder114<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 229) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder115<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 230) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder116<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 231) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder117<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 232) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder118<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 233) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder119<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 234) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder120<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 235) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder121<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 236) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder122<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 237) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder123<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 238) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder124<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 239) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder125<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let marker = {
        let ff = {
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return None;
            }
        };
        let marker = {
            let b = (input.read_byte())?;
            if (b == 254) {
                b
            } else {
                return None;
            }
        };
        Type41 { ff, marker }
    };
    let length = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder126<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type63> {
    let restart_interval = { (Decoder43(scope, input))? };
    (Some(Type63 { restart_interval }))
}

fn Decoder127<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type57> {
    let class_table_id = { (Decoder17(scope, input))? };
    let value = { (Decoder17(scope, input))? };
    (Some(Type57 {
        class_table_id,
        value,
    }))
}

fn Decoder128<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type59> {
    let class_table_id = { (Decoder17(scope, input))? };
    let num_codes = {
        let mut accum = (Vec::new());
        for _ in 0..16 {
            (accum.push((Decoder17(scope, input))?));
        }
        accum
    };
    let values = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                0
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder17(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type59 {
        class_table_id,
        num_codes,
        values,
    }))
}

fn Decoder129<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type61> {
    let precision_table_id = { (Decoder17(scope, input))? };
    let elements = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                0
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder17(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type61 {
        precision_table_id,
        elements,
    }))
}

fn Decoder130<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type53> {
    let identifier = { (Decoder131(scope, input))? };
    let data = {
        match (identifier.string.as_slice()) {
            [69, 120, 105, 102] => {
                let inner = (Decoder132(scope, input))?;
                (Type52::exif(inner))
            }

            [104, 116, 116, 112, 58, 47, 47, 110, 115, 46, 97, 100, 111, 98, 101, 46, 99, 111, 109, 47, 120, 97, 112, 47, 49, 46, 48, 47] =>
            {
                let inner = (Decoder133(scope, input))?;
                (Type52::xmp(inner))
            }

            _ => {
                let inner = {
                    let mut accum = (Vec::new());
                    while true {
                        let matching_ix = {
                            let lookahead = &mut (input.clone());
                            0
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(scope, input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                };
                (Type52::other(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(Type53 { identifier, data }))
}

fn Decoder131<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let null = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    (Some(Type21 { string, null }))
}

fn Decoder132<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type50> {
    let padding = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    let exif = { (Decoder134(scope, input))? };
    (Some(Type50 { padding, exif }))
}

fn Decoder133<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type51> {
    let xmp = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                0
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder17(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type51 { xmp }))
}

fn Decoder134<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type49> {
    let byte_order = {
        let tree_index = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            match b {
                73 => 0,

                77 => 1,

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }
        };
        match tree_index {
            0 => {
                let field0 = {
                    let b = (input.read_byte())?;
                    if (b == 73) {
                        b
                    } else {
                        return None;
                    }
                };
                let field1 = {
                    let b = (input.read_byte())?;
                    if (b == 73) {
                        b
                    } else {
                        return None;
                    }
                };
                (Type46::le(field0, field1))
            }

            1 => {
                let field0 = {
                    let b = (input.read_byte())?;
                    if (b == 77) {
                        b
                    } else {
                        return None;
                    }
                };
                let field1 = {
                    let b = (input.read_byte())?;
                    if (b == 77) {
                        b
                    } else {
                        return None;
                    }
                };
                (Type46::be(field0, field1))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let magic = {
        match byte_order {
            Type46::le(..) => (Decoder135(scope, input))?,

            Type46::be(..) => (Decoder43(scope, input))?,

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let offset = {
        match byte_order {
            Type46::le(..) => (Decoder24(scope, input))?,

            Type46::be(..) => (Decoder33(scope, input))?,

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let ifd = { (unimplemented!(r#"translate @ Decoder::WithRelativeOffset"#)) };
    (Some(Type49 {
        byte_order,
        magic,
        offset,
        ifd,
    }))
}

fn Decoder135<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u16> {
    let inner = {
        let field0 = { (Decoder17(scope, input))? };
        let field1 = { (Decoder17(scope, input))? };
        (field0, field1)
    };
    (Some(((|x: (u8, u8)| u16le(x))(inner))))
}

fn Decoder136<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type44> {
    let identifier = { (Decoder137(scope, input))? };
    let data = {
        match (identifier.string.as_slice()) {
            [74, 70, 73, 70] => {
                let inner = (Decoder138(scope, input))?;
                (Type43::jfif(inner))
            }

            _ => {
                let inner = {
                    let mut accum = (Vec::new());
                    while true {
                        let matching_ix = {
                            let lookahead = &mut (input.clone());
                            0
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(scope, input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                };
                (Type43::other(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(Type44 { identifier, data }))
}

fn Decoder137<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let null = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    (Some(Type21 { string, null }))
}

fn Decoder138<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
    let version_major = { (Decoder17(scope, input))? };
    let version_minor = { (Decoder17(scope, input))? };
    let density_units = { (Decoder17(scope, input))? };
    let density_x = { (Decoder43(scope, input))? };
    let density_y = { (Decoder43(scope, input))? };
    let thumbnail_width = { (Decoder17(scope, input))? };
    let thumbnail_height = { (Decoder17(scope, input))? };
    let thumbnail_pixels = {
        let mut accum = (Vec::new());
        for _ in 0..thumbnail_height {
            (accum.push({
                let mut accum = (Vec::new());
                for _ in 0..thumbnail_width {
                    (accum.push((Decoder139(scope, input))?));
                }
                accum
            }));
        }
        accum
    };
    (Some(Type42 {
        version_major,
        version_minor,
        density_units,
        density_x,
        density_y,
        thumbnail_width,
        thumbnail_height,
        thumbnail_pixels,
    }))
}

fn Decoder139<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type2> {
    let r = { (Decoder17(scope, input))? };
    let g = { (Decoder17(scope, input))? };
    let b = { (Decoder17(scope, input))? };
    (Some(Type2 { r, g, b }))
}

fn Decoder140<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
    let magic = {
        let field0 = {
            let b = (input.read_byte())?;
            if (b == 31) {
                b
            } else {
                return None;
            }
        };
        let field1 = {
            let b = (input.read_byte())?;
            if (b == 139) {
                b
            } else {
                return None;
            }
        };
        (field0, field1)
    };
    let method = { (Decoder17(scope, input))? };
    let file_flags = { (Decoder17(scope, input))? };
    let timestamp = { (Decoder24(scope, input))? };
    let compression_flags = { (Decoder17(scope, input))? };
    let os_id = { (Decoder17(scope, input))? };
    (Some(Type20 {
        magic,
        method,
        file_flags,
        timestamp,
        compression_flags,
        os_id,
    }))
}

fn Decoder141<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    (Some((Decoder150(scope, input))?))
}

fn Decoder142<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type38> {
    let blocks = {
        let mut accum = (Vec::new());
        while true {
            let elem = (Decoder144(scope, input))?;
            if ((|x: &Type37| x.r#final == 1)(&elem)) {
                (accum.push(elem));
                break;
            } else {
                (accum.push(elem));
            }
        }
        accum
    };
    let codes = {
        ((((blocks.iter()).cloned()).flat_map(
            (|x: Type37| match x.data {
                Type36::uncompressed(y) => y.codes_values,

                Type36::fixed_huffman(y) => y.codes_values,

                Type36::dynamic_huffman(y) => y.codes_values,

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }),
        ))
        .collect())
    };
    let inflate = { (unimplemented!(r#"embed_expr is not implemented for Expr::Inflate"#)) };
    (Some(Type38 {
        blocks,
        codes,
        inflate,
    }))
}

fn Decoder143<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type39> {
    let crc = { (Decoder24(scope, input))? };
    let length = { (Decoder24(scope, input))? };
    (Some(Type39 { crc, length }))
}

fn Decoder144<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type37> {
    let r#final = { (Decoder145(scope, input))? };
    let r#type = {
        let inner = {
            let field0 = { (Decoder145(scope, input))? };
            let field1 = { (Decoder145(scope, input))? };
            (field0, field1)
        };
        ((|bits: (u8, u8)| bits.1 << 1 | bits.0)(inner))
    };
    let data = {
        match r#type {
            0 => {
                let inner = (Decoder146(scope, input))?;
                (Type36::uncompressed(inner))
            }

            1 => {
                let inner = (Decoder147(scope, input))?;
                (Type36::fixed_huffman(inner))
            }

            2 => {
                let inner = (Decoder148(scope, input))?;
                (Type36::dynamic_huffman(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(Type37 {
        r#final,
        r#type,
        data,
    }))
}

fn Decoder145<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(b))
}

fn Decoder146<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type35> {
    let align = {
        while (input.offset() % 8 != 0) {
            let _ = (input.read_byte())?;
        }
        ()
    };
    let len = {
        let inner = {
            let field0 = { (Decoder145(scope, input))? };
            let field1 = { (Decoder145(scope, input))? };
            let field2 = { (Decoder145(scope, input))? };
            let field3 = { (Decoder145(scope, input))? };
            let field4 = { (Decoder145(scope, input))? };
            let field5 = { (Decoder145(scope, input))? };
            let field6 = { (Decoder145(scope, input))? };
            let field7 = { (Decoder145(scope, input))? };
            let field8 = { (Decoder145(scope, input))? };
            let field9 = { (Decoder145(scope, input))? };
            let field10 = { (Decoder145(scope, input))? };
            let field11 = { (Decoder145(scope, input))? };
            let field12 = { (Decoder145(scope, input))? };
            let field13 = { (Decoder145(scope, input))? };
            let field14 = { (Decoder145(scope, input))? };
            let field15 = { (Decoder145(scope, input))? };
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
            (bits.15 as u16) << 15
                | (bits.14 as u16) << 14
                | (bits.13 as u16) << 13
                | (bits.12 as u16) << 12
                | (bits.11 as u16) << 11
                | (bits.10 as u16) << 10
                | (bits.9 as u16) << 9
                | (bits.8 as u16) << 8
                | (bits.7 as u16) << 7
                | (bits.6 as u16) << 6
                | (bits.5 as u16) << 5
                | (bits.4 as u16) << 4
                | (bits.3 as u16) << 3
                | (bits.2 as u16) << 2
                | (bits.1 as u16) << 1
                | (bits.0 as u16)
        })(inner))
    };
    let nlen = {
        let inner = {
            let field0 = { (Decoder145(scope, input))? };
            let field1 = { (Decoder145(scope, input))? };
            let field2 = { (Decoder145(scope, input))? };
            let field3 = { (Decoder145(scope, input))? };
            let field4 = { (Decoder145(scope, input))? };
            let field5 = { (Decoder145(scope, input))? };
            let field6 = { (Decoder145(scope, input))? };
            let field7 = { (Decoder145(scope, input))? };
            let field8 = { (Decoder145(scope, input))? };
            let field9 = { (Decoder145(scope, input))? };
            let field10 = { (Decoder145(scope, input))? };
            let field11 = { (Decoder145(scope, input))? };
            let field12 = { (Decoder145(scope, input))? };
            let field13 = { (Decoder145(scope, input))? };
            let field14 = { (Decoder145(scope, input))? };
            let field15 = { (Decoder145(scope, input))? };
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
            (bits.15 as u16) << 15
                | (bits.14 as u16) << 14
                | (bits.13 as u16) << 13
                | (bits.12 as u16) << 12
                | (bits.11 as u16) << 11
                | (bits.10 as u16) << 10
                | (bits.9 as u16) << 9
                | (bits.8 as u16) << 8
                | (bits.7 as u16) << 7
                | (bits.6 as u16) << 6
                | (bits.5 as u16) << 5
                | (bits.4 as u16) << 4
                | (bits.3 as u16) << 3
                | (bits.2 as u16) << 2
                | (bits.1 as u16) << 1
                | (bits.0 as u16)
        })(inner))
    };
    let bytes = {
        let mut accum = (Vec::new());
        for _ in 0..len {
            (accum.push({
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| {
                    bits.7 << 7
                        | bits.6 << 6
                        | bits.5 << 5
                        | bits.4 << 4
                        | bits.3 << 3
                        | bits.2 << 2
                        | bits.1 << 1
                        | bits.0
                })(inner))
            }));
        }
        accum
    };
    let codes_values = {
        ((((bytes.iter()).cloned()).flat_map((|x: u8| [(Type29::literal(x))].to_vec()))).collect())
    };
    (Some(Type35 {
        align,
        len,
        nlen,
        bytes,
        codes_values,
    }))
}

fn Decoder147<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type34> {
    let codes = {
        let format = (unimplemented!(
            r#"no implementation for for DynamicLogic::Huffman AST-transcription"#
        ));
        let mut accum = (Vec::new());
        while true {
            let elem = {
                let code = { (format(scope, input))? };
                let extra = {
                    match code {
                        257 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (3 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        258 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (4 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        259 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (5 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        260 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (6 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        261 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (7 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        262 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (8 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        263 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (9 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        264 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (10 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        265 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        (field0)
                                    };
                                    ((|bits: (u8)| bits.0)(inner))
                                };
                                let length = { (11 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        266 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        (field0)
                                    };
                                    ((|bits: (u8)| bits.0)(inner))
                                };
                                let length = { (13 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        267 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        (field0)
                                    };
                                    ((|bits: (u8)| bits.0)(inner))
                                };
                                let length = { (15 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        268 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        (field0)
                                    };
                                    ((|bits: (u8)| bits.0)(inner))
                                };
                                let length = { (17 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        269 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        (field0, field1)
                                    };
                                    ((|bits: (u8, u8)| bits.1 << 1 | bits.0)(inner))
                                };
                                let length = { (19 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        270 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        (field0, field1)
                                    };
                                    ((|bits: (u8, u8)| bits.1 << 1 | bits.0)(inner))
                                };
                                let length = { (23 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        271 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        (field0, field1)
                                    };
                                    ((|bits: (u8, u8)| bits.1 << 1 | bits.0)(inner))
                                };
                                let length = { (27 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        272 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        (field0, field1)
                                    };
                                    ((|bits: (u8, u8)| bits.1 << 1 | bits.0)(inner))
                                };
                                let length = { (31 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        273 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2)
                                    };
                                    ((|bits: (u8, u8, u8)| bits.2 << 2 | bits.1 << 1 | bits.0)(
                                        inner,
                                    ))
                                };
                                let length = { (35 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        274 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2)
                                    };
                                    ((|bits: (u8, u8, u8)| bits.2 << 2 | bits.1 << 1 | bits.0)(
                                        inner,
                                    ))
                                };
                                let length = { (43 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        275 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2)
                                    };
                                    ((|bits: (u8, u8, u8)| bits.2 << 2 | bits.1 << 1 | bits.0)(
                                        inner,
                                    ))
                                };
                                let length = { (51 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        276 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2)
                                    };
                                    ((|bits: (u8, u8, u8)| bits.2 << 2 | bits.1 << 1 | bits.0)(
                                        inner,
                                    ))
                                };
                                let length = { (59 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        277 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3)
                                    };
                                    ((|bits: (u8, u8, u8, u8)| {
                                        bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0
                                    })(inner))
                                };
                                let length = { (67 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        278 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3)
                                    };
                                    ((|bits: (u8, u8, u8, u8)| {
                                        bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0
                                    })(inner))
                                };
                                let length = { (83 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        279 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3)
                                    };
                                    ((|bits: (u8, u8, u8, u8)| {
                                        bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0
                                    })(inner))
                                };
                                let length = { (99 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        280 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3)
                                    };
                                    ((|bits: (u8, u8, u8, u8)| {
                                        bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0
                                    })(inner))
                                };
                                let length = { (115 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        281 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let length = { (131 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        282 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let length = { (163 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        283 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let length = { (195 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        284 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let length = { (227 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        285 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (258 + (length_extra_bits as u16)) };
                                let distance_code = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type31 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type32::some(inner))
                        }

                        _ => {
                            let _ = ();
                            Type32::none
                        }

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                };
                Type33 { code, extra }
            };
            if ((|x: &Type33| (x.code as u16) == 256)(&elem)) {
                (accum.push(elem));
                break;
            } else {
                (accum.push(elem));
            }
        }
        accum
    };
    let codes_values = {
        ((((codes.iter()).cloned()).flat_map(
            (|x: Type33| match x.code {
                256 => ([].to_vec()),

                257 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                258 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                259 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                260 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                261 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                262 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                263 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                264 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                265 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                266 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                267 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                268 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                269 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                270 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                271 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                272 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                273 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                274 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                275 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                276 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                277 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                278 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                279 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                280 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                281 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                282 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                283 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                284 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                285 => match x.extra {
                    Type32::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                _ => ([(Type29::literal((x.code as u8)))].to_vec()),

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }),
        ))
        .collect())
    };
    (Some(Type34 {
        codes,
        codes_values,
    }))
}

fn Decoder148<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type30> {
    let hlit = {
        let inner = {
            let field0 = { (Decoder145(scope, input))? };
            let field1 = { (Decoder145(scope, input))? };
            let field2 = { (Decoder145(scope, input))? };
            let field3 = { (Decoder145(scope, input))? };
            let field4 = { (Decoder145(scope, input))? };
            (field0, field1, field2, field3, field4)
        };
        ((|bits: (u8, u8, u8, u8, u8)| {
            bits.4 << 4 | bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0
        })(inner))
    };
    let hdist = {
        let inner = {
            let field0 = { (Decoder145(scope, input))? };
            let field1 = { (Decoder145(scope, input))? };
            let field2 = { (Decoder145(scope, input))? };
            let field3 = { (Decoder145(scope, input))? };
            let field4 = { (Decoder145(scope, input))? };
            (field0, field1, field2, field3, field4)
        };
        ((|bits: (u8, u8, u8, u8, u8)| {
            bits.4 << 4 | bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0
        })(inner))
    };
    let hclen = {
        let inner = {
            let field0 = { (Decoder145(scope, input))? };
            let field1 = { (Decoder145(scope, input))? };
            let field2 = { (Decoder145(scope, input))? };
            let field3 = { (Decoder145(scope, input))? };
            (field0, field1, field2, field3)
        };
        ((|bits: (u8, u8, u8, u8)| bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0)(inner))
    };
    let code_length_alphabet_code_lengths = {
        let mut accum = (Vec::new());
        for _ in 0..(hclen + 4) {
            (accum.push({
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    (field0, field1, field2)
                };
                ((|bits: (u8, u8, u8)| bits.2 << 2 | bits.1 << 1 | bits.0)(inner))
            }));
        }
        accum
    };
    let literal_length_distance_alphabet_code_lengths = {
        let code_length_alphabet_format = (unimplemented!(
            r#"no implementation for for DynamicLogic::Huffman AST-transcription"#
        ));
        let mut accum = (Vec::new());
        while true {
            let elem = {
                let code = { (code_length_alphabet_format(scope, input))? };
                let extra = {
                    match (code as u8) {
                        16 => {
                            let inner = {
                                let field0 = { (Decoder145(scope, input))? };
                                let field1 = { (Decoder145(scope, input))? };
                                (field0, field1)
                            };
                            ((|bits: (u8, u8)| bits.1 << 1 | bits.0)(inner))
                        }

                        17 => {
                            let inner = {
                                let field0 = { (Decoder145(scope, input))? };
                                let field1 = { (Decoder145(scope, input))? };
                                let field2 = { (Decoder145(scope, input))? };
                                (field0, field1, field2)
                            };
                            ((|bits: (u8, u8, u8)| bits.2 << 2 | bits.1 << 1 | bits.0)(inner))
                        }

                        18 => {
                            let inner = {
                                let field0 = { (Decoder145(scope, input))? };
                                let field1 = { (Decoder145(scope, input))? };
                                let field2 = { (Decoder145(scope, input))? };
                                let field3 = { (Decoder145(scope, input))? };
                                let field4 = { (Decoder145(scope, input))? };
                                let field5 = { (Decoder145(scope, input))? };
                                let field6 = { (Decoder145(scope, input))? };
                                (field0, field1, field2, field3, field4, field5, field6)
                            };
                            ((|bits: (u8, u8, u8, u8, u8, u8, u8)| {
                                bits.6 << 6
                                    | bits.5 << 5
                                    | bits.4 << 4
                                    | bits.3 << 3
                                    | bits.2 << 2
                                    | bits.1 << 1
                                    | bits.0
                            })(inner))
                        }

                        _ => 0,

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                };
                Type23 { code, extra }
            };
            (accum.push(elem));
            if (((|y: &Vec<Type23>| {
                ((((y.iter()).cloned()).fold(
                    {
                        ();
                        Type194::none
                    },
                    (|x: (Type194, Type23)| match (x.1.code as u8) {
                        16 => (
                            x.0,
                            (dup32(
                                ((x.1.extra + 3) as u32),
                                match x.0 {
                                    Type194::some(y) => y,

                                    _other => {
                                        (unreachable!(r#"unexpected: {:?}"#, _other));
                                    }
                                },
                            )),
                        ),

                        17 => (x.0, (dup32(((x.1.extra + 3) as u32), 0))),

                        18 => (x.0, (dup32(((x.1.extra + 11) as u32), 0))),

                        v => ((Type194::some(v)), ([v].to_vec())),

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }),
                ))
                .collect())
                .len()
                    >= ((hlit + hdist) as u32) + 258
            })())(&accum))
            {
                break;
            }
        }
        accum
    };
    let literal_length_distance_alphabet_code_lengths_value = {
        ((((literal_length_distance_alphabet_code_lengths.iter()).cloned()).fold(
            {
                ();
                Type194::none
            },
            (|x: (Type194, Type23)| match (x.1.code as u8) {
                16 => (
                    x.0,
                    (dup32(
                        ((x.1.extra + 3) as u32),
                        match x.0 {
                            Type194::some(y) => y,

                            _other => {
                                (unreachable!(r#"unexpected: {:?}"#, _other));
                            }
                        },
                    )),
                ),

                17 => (x.0, (dup32(((x.1.extra + 3) as u32), 0))),

                18 => (x.0, (dup32(((x.1.extra + 11) as u32), 0))),

                v => ((Type194::some(v)), ([v].to_vec())),

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }),
        ))
        .collect())
    };
    let literal_length_alphabet_code_lengths_value = {
        {
            let ix = 0;
            literal_length_distance_alphabet_code_lengths_value[ix..(ix + (hlit as u32) + 257)]
        }
    };
    let distance_alphabet_code_lengths_value = {
        {
            let ix = ((hlit as u32) + 257);
            literal_length_distance_alphabet_code_lengths_value[ix..(ix + (hdist as u32) + 1)]
        }
    };
    let codes = {
        let distance_alphabet_format = (unimplemented!(
            r#"no implementation for for DynamicLogic::Huffman AST-transcription"#
        ));
        let literal_length_alphabet_format = (unimplemented!(
            r#"no implementation for for DynamicLogic::Huffman AST-transcription"#
        ));
        let mut accum = (Vec::new());
        while true {
            let elem = {
                let code = { (literal_length_alphabet_format(scope, input))? };
                let extra = {
                    match code {
                        257 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (3 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        258 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (4 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        259 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (5 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        260 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (6 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        261 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (7 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        262 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (8 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        263 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (9 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        264 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (10 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        265 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        (field0)
                                    };
                                    ((|bits: (u8)| bits.0)(inner))
                                };
                                let length = { (11 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        266 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        (field0)
                                    };
                                    ((|bits: (u8)| bits.0)(inner))
                                };
                                let length = { (13 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        267 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        (field0)
                                    };
                                    ((|bits: (u8)| bits.0)(inner))
                                };
                                let length = { (15 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        268 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        (field0)
                                    };
                                    ((|bits: (u8)| bits.0)(inner))
                                };
                                let length = { (17 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        269 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        (field0, field1)
                                    };
                                    ((|bits: (u8, u8)| bits.1 << 1 | bits.0)(inner))
                                };
                                let length = { (19 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        270 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        (field0, field1)
                                    };
                                    ((|bits: (u8, u8)| bits.1 << 1 | bits.0)(inner))
                                };
                                let length = { (23 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        271 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        (field0, field1)
                                    };
                                    ((|bits: (u8, u8)| bits.1 << 1 | bits.0)(inner))
                                };
                                let length = { (27 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        272 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        (field0, field1)
                                    };
                                    ((|bits: (u8, u8)| bits.1 << 1 | bits.0)(inner))
                                };
                                let length = { (31 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        273 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2)
                                    };
                                    ((|bits: (u8, u8, u8)| bits.2 << 2 | bits.1 << 1 | bits.0)(
                                        inner,
                                    ))
                                };
                                let length = { (35 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        274 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2)
                                    };
                                    ((|bits: (u8, u8, u8)| bits.2 << 2 | bits.1 << 1 | bits.0)(
                                        inner,
                                    ))
                                };
                                let length = { (43 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        275 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2)
                                    };
                                    ((|bits: (u8, u8, u8)| bits.2 << 2 | bits.1 << 1 | bits.0)(
                                        inner,
                                    ))
                                };
                                let length = { (51 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        276 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2)
                                    };
                                    ((|bits: (u8, u8, u8)| bits.2 << 2 | bits.1 << 1 | bits.0)(
                                        inner,
                                    ))
                                };
                                let length = { (59 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        277 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3)
                                    };
                                    ((|bits: (u8, u8, u8, u8)| {
                                        bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0
                                    })(inner))
                                };
                                let length = { (67 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        278 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3)
                                    };
                                    ((|bits: (u8, u8, u8, u8)| {
                                        bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0
                                    })(inner))
                                };
                                let length = { (83 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        279 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3)
                                    };
                                    ((|bits: (u8, u8, u8, u8)| {
                                        bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0
                                    })(inner))
                                };
                                let length = { (99 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        280 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3)
                                    };
                                    ((|bits: (u8, u8, u8, u8)| {
                                        bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0
                                    })(inner))
                                };
                                let length = { (115 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        281 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let length = { (131 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        282 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let length = { (163 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        283 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let length = { (195 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        284 => {
                            let inner = {
                                let length_extra_bits = {
                                    let inner = {
                                        let field0 = { (Decoder145(scope, input))? };
                                        let field1 = { (Decoder145(scope, input))? };
                                        let field2 = { (Decoder145(scope, input))? };
                                        let field3 = { (Decoder145(scope, input))? };
                                        let field4 = { (Decoder145(scope, input))? };
                                        (field0, field1, field2, field3, field4)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8)| {
                                        bits.4 << 4
                                            | bits.3 << 3
                                            | bits.2 << 2
                                            | bits.1 << 1
                                            | bits.0
                                    })(inner))
                                };
                                let length = { (227 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        285 => {
                            let inner = {
                                let length_extra_bits = { 0 };
                                let length = { (258 + (length_extra_bits as u16)) };
                                let distance_code = { (distance_alphabet_format(scope, input))? };
                                let distance_record = { (Decoder149(scope, input))? };
                                Type25 {
                                    length_extra_bits,
                                    length,
                                    distance_code,
                                    distance_record,
                                }
                            };
                            (Type26::some(inner))
                        }

                        _ => {
                            let _ = ();
                            Type26::none
                        }

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                };
                Type27 { code, extra }
            };
            if ((|x: &Type27| (x.code as u16) == 256)(&elem)) {
                (accum.push(elem));
                break;
            } else {
                (accum.push(elem));
            }
        }
        accum
    };
    let codes_values = {
        ((((codes.iter()).cloned()).flat_map(
            (|x: Type27| match x.code {
                256 => ([].to_vec()),

                257 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                258 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                259 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                260 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                261 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                262 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                263 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                264 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                265 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                266 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                267 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                268 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                269 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                270 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                271 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                272 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                273 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                274 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                275 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                276 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                277 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                278 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                279 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                280 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                281 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                282 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                283 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                284 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                285 => match x.extra {
                    Type26::some(rec) => {
                        ([(Type29::reference(Type28 {
                            length: rec.length,
                            distance: rec.distance_record.distance,
                        }))]
                        .to_vec())
                    }

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                },

                _ => ([(Type29::literal((x.code as u8)))].to_vec()),

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }),
        ))
        .collect())
    };
    (Some(Type30 {
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
    }))
}

fn Decoder149<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type24> {
    (Some(match (distance_code as u8) {
        0 => {
            let distance_extra_bits = { 0 };
            let distance = { (1 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        1 => {
            let distance_extra_bits = { 0 };
            let distance = { (2 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        2 => {
            let distance_extra_bits = { 0 };
            let distance = { (3 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        3 => {
            let distance_extra_bits = { 0 };
            let distance = { (4 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        4 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    (field0)
                };
                ((|bits: (u8)| bits.0 as u16)(inner))
            };
            let distance = { (5 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        5 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    (field0)
                };
                ((|bits: (u8)| bits.0 as u16)(inner))
            };
            let distance = { (7 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        6 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    (field0, field1)
                };
                ((|bits: (u8, u8)| (bits.1 as u16) << 1 | (bits.0 as u16))(inner))
            };
            let distance = { (9 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        7 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    (field0, field1)
                };
                ((|bits: (u8, u8)| (bits.1 as u16) << 1 | (bits.0 as u16))(inner))
            };
            let distance = { (13 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        8 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    (field0, field1, field2)
                };
                ((|bits: (u8, u8, u8)| {
                    (bits.2 as u16) << 2 | (bits.1 as u16) << 1 | (bits.0 as u16)
                })(inner))
            };
            let distance = { (17 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        9 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    (field0, field1, field2)
                };
                ((|bits: (u8, u8, u8)| {
                    (bits.2 as u16) << 2 | (bits.1 as u16) << 1 | (bits.0 as u16)
                })(inner))
            };
            let distance = { (25 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        10 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    (field0, field1, field2, field3)
                };
                ((|bits: (u8, u8, u8, u8)| {
                    (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (33 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        11 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    (field0, field1, field2, field3)
                };
                ((|bits: (u8, u8, u8, u8)| {
                    (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (49 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        12 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    (field0, field1, field2, field3, field4)
                };
                ((|bits: (u8, u8, u8, u8, u8)| {
                    (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (65 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        13 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    (field0, field1, field2, field3, field4)
                };
                ((|bits: (u8, u8, u8, u8, u8)| {
                    (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (97 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        14 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    (field0, field1, field2, field3, field4, field5)
                };
                ((|bits: (u8, u8, u8, u8, u8, u8)| {
                    (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (129 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        15 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    (field0, field1, field2, field3, field4, field5)
                };
                ((|bits: (u8, u8, u8, u8, u8, u8)| {
                    (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (193 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        16 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    (field0, field1, field2, field3, field4, field5, field6)
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (257 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        17 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    (field0, field1, field2, field3, field4, field5, field6)
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (385 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        18 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (513 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        19 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (769 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        20 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    let field8 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.8 as u16) << 8
                        | (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (1025 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        21 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    let field8 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.8 as u16) << 8
                        | (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (1537 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        22 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    let field8 = { (Decoder145(scope, input))? };
                    let field9 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.9 as u16) << 9
                        | (bits.8 as u16) << 8
                        | (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (2049 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        23 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    let field8 = { (Decoder145(scope, input))? };
                    let field9 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.9 as u16) << 9
                        | (bits.8 as u16) << 8
                        | (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (3073 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        24 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    let field8 = { (Decoder145(scope, input))? };
                    let field9 = { (Decoder145(scope, input))? };
                    let field10 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9, field10,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.10 as u16) << 10
                        | (bits.9 as u16) << 9
                        | (bits.8 as u16) << 8
                        | (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (4097 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        25 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    let field8 = { (Decoder145(scope, input))? };
                    let field9 = { (Decoder145(scope, input))? };
                    let field10 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9, field10,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.10 as u16) << 10
                        | (bits.9 as u16) << 9
                        | (bits.8 as u16) << 8
                        | (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (6145 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        26 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    let field8 = { (Decoder145(scope, input))? };
                    let field9 = { (Decoder145(scope, input))? };
                    let field10 = { (Decoder145(scope, input))? };
                    let field11 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9, field10, field11,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.11 as u16) << 11
                        | (bits.10 as u16) << 10
                        | (bits.9 as u16) << 9
                        | (bits.8 as u16) << 8
                        | (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (8193 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        27 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    let field8 = { (Decoder145(scope, input))? };
                    let field9 = { (Decoder145(scope, input))? };
                    let field10 = { (Decoder145(scope, input))? };
                    let field11 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9, field10, field11,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.11 as u16) << 11
                        | (bits.10 as u16) << 10
                        | (bits.9 as u16) << 9
                        | (bits.8 as u16) << 8
                        | (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (12289 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        28 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    let field8 = { (Decoder145(scope, input))? };
                    let field9 = { (Decoder145(scope, input))? };
                    let field10 = { (Decoder145(scope, input))? };
                    let field11 = { (Decoder145(scope, input))? };
                    let field12 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9, field10, field11, field12,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.12 as u16) << 12
                        | (bits.11 as u16) << 11
                        | (bits.10 as u16) << 10
                        | (bits.9 as u16) << 9
                        | (bits.8 as u16) << 8
                        | (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (16385 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        29 => {
            let distance_extra_bits = {
                let inner = {
                    let field0 = { (Decoder145(scope, input))? };
                    let field1 = { (Decoder145(scope, input))? };
                    let field2 = { (Decoder145(scope, input))? };
                    let field3 = { (Decoder145(scope, input))? };
                    let field4 = { (Decoder145(scope, input))? };
                    let field5 = { (Decoder145(scope, input))? };
                    let field6 = { (Decoder145(scope, input))? };
                    let field7 = { (Decoder145(scope, input))? };
                    let field8 = { (Decoder145(scope, input))? };
                    let field9 = { (Decoder145(scope, input))? };
                    let field10 = { (Decoder145(scope, input))? };
                    let field11 = { (Decoder145(scope, input))? };
                    let field12 = { (Decoder145(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        field9, field10, field11, field12,
                    )
                };
                ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                    (bits.12 as u16) << 12
                        | (bits.11 as u16) << 11
                        | (bits.10 as u16) << 10
                        | (bits.9 as u16) << 9
                        | (bits.8 as u16) << 8
                        | (bits.7 as u16) << 7
                        | (bits.6 as u16) << 6
                        | (bits.5 as u16) << 5
                        | (bits.4 as u16) << 4
                        | (bits.3 as u16) << 3
                        | (bits.2 as u16) << 2
                        | (bits.1 as u16) << 1
                        | (bits.0 as u16)
                })(inner))
            };
            let distance = { (24577 + distance_extra_bits) };
            Type24 {
                distance_extra_bits,
                distance,
            }
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder150<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let b = (input.read_byte())?;
                    if (b != 0) {
                        b
                    } else {
                        return None;
                    }
                };
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let null = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    (Some(Type21 { string, null }))
}

fn Decoder151<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type0> {
    let signature = {
        let field0 = {
            let b = (input.read_byte())?;
            if (b == 71) {
                b
            } else {
                return None;
            }
        };
        let field1 = {
            let b = (input.read_byte())?;
            if (b == 73) {
                b
            } else {
                return None;
            }
        };
        let field2 = {
            let b = (input.read_byte())?;
            if (b == 70) {
                b
            } else {
                return None;
            }
        };
        (field0, field1, field2)
    };
    let version = {
        let mut accum = (Vec::new());
        for _ in 0..3 {
            (accum.push((Decoder21(scope, input))?));
        }
        accum
    };
    (Some(Type0 { signature, version }))
}

fn Decoder152<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type4> {
    let descriptor = { (Decoder168(scope, input))? };
    let global_color_table = {
        match (descriptor.flags & 128 != 0) {
            true => {
                let inner = {
                    let mut accum = (Vec::new());
                    for _ in 0..(2 << (descriptor.flags & 7)) {
                        (accum.push((Decoder166(scope, input))?));
                    }
                    accum
                };
                (Type3::yes(inner))
            }

            false => {
                let _ = ();
                Type3::no
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(Type4 {
        descriptor,
        global_color_table,
    }))
}

fn Decoder153<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type17> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        match b {
            33 => {
                let b = (lookahead.read_byte())?;
                match b {
                    249 => 0,

                    1 => 0,

                    255 => 1,

                    254 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            }

            44 => 0,

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder155(scope, input))?;
            (Type17::graphic_block(inner))
        }

        1 => {
            let inner = (Decoder156(scope, input))?;
            (Type17::special_purpose_block(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder154<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type18> {
    let separator = {
        let b = (input.read_byte())?;
        if (b == 59) {
            b
        } else {
            return None;
        }
    };
    (Some(Type18 { separator }))
}

fn Decoder155<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type13> {
    let graphic_control_extension = {
        let tree_index = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            match b {
                33 => {
                    let b = (lookahead.read_byte())?;
                    match b {
                        249 => 0,

                        1 => 1,

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                }

                44 => 1,

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }
        };
        match tree_index {
            0 => {
                let inner = (Decoder161(scope, input))?;
                (Type6::some(inner))
            }

            1 => {
                let _ = ();
                Type6::none
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let graphic_rendering_block = { (Decoder162(scope, input))? };
    (Some(Type13 {
        graphic_control_extension,
        graphic_rendering_block,
    }))
}

fn Decoder156<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type16> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        if (b == 33) {
            let b = (lookahead.read_byte())?;
            match b {
                255 => 0,

                254 => 1,

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }
        } else {
            return None;
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder157(scope, input))?;
            (Type16::application_extension(inner))
        }

        1 => {
            let inner = (Decoder158(scope, input))?;
            (Type16::comment_extension(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder157<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type14> {
    let separator = {
        let b = (input.read_byte())?;
        if (b == 33) {
            b
        } else {
            return None;
        }
    };
    let label = {
        let b = (input.read_byte())?;
        if (b == 255) {
            b
        } else {
            return None;
        }
    };
    let block_size = {
        let b = (input.read_byte())?;
        if (b == 11) {
            b
        } else {
            return None;
        }
    };
    let identifier = {
        let mut accum = (Vec::new());
        for _ in 0..8 {
            (accum.push((Decoder17(scope, input))?));
        }
        accum
    };
    let authentication_code = {
        let mut accum = (Vec::new());
        for _ in 0..3 {
            (accum.push((Decoder17(scope, input))?));
        }
        accum
    };
    let application_data = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder159(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder160(scope, input))? };
    (Some(Type14 {
        separator,
        label,
        block_size,
        identifier,
        authentication_code,
        application_data,
        terminator,
    }))
}

fn Decoder158<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type15> {
    let separator = {
        let b = (input.read_byte())?;
        if (b == 33) {
            b
        } else {
            return None;
        }
    };
    let label = {
        let b = (input.read_byte())?;
        if (b == 254) {
            b
        } else {
            return None;
        }
    };
    let comment_data = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder159(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder160(scope, input))? };
    (Some(Type15 {
        separator,
        label,
        comment_data,
        terminator,
    }))
}

fn Decoder159<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type7> {
    let len_bytes = {
        let b = (input.read_byte())?;
        if (b != 0) {
            b
        } else {
            return None;
        }
    };
    let data = {
        let mut accum = (Vec::new());
        for _ in 0..len_bytes {
            (accum.push((Decoder17(scope, input))?));
        }
        accum
    };
    (Some(Type7 { len_bytes, data }))
}

fn Decoder160<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(if (b == 0) {
        b
    } else {
        return None;
    }))
}

fn Decoder161<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type5> {
    let separator = {
        let b = (input.read_byte())?;
        if (b == 33) {
            b
        } else {
            return None;
        }
    };
    let label = {
        let b = (input.read_byte())?;
        if (b == 249) {
            b
        } else {
            return None;
        }
    };
    let block_size = {
        let b = (input.read_byte())?;
        if (b == 4) {
            b
        } else {
            return None;
        }
    };
    let flags = { (Decoder17(scope, input))? };
    let delay_time = { (Decoder135(scope, input))? };
    let transparent_color_index = { (Decoder17(scope, input))? };
    let terminator = { (Decoder160(scope, input))? };
    (Some(Type5 {
        separator,
        label,
        block_size,
        flags,
        delay_time,
        transparent_color_index,
        terminator,
    }))
}

fn Decoder162<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type12> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        match b {
            44 => 0,

            33 => 1,

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder163(scope, input))?;
            (Type12::table_based_image(inner))
        }

        1 => {
            let inner = (Decoder164(scope, input))?;
            (Type12::plain_text_extension(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder163<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type11> {
    let descriptor = { (Decoder165(scope, input))? };
    let local_color_table = {
        match (descriptor.flags & 128 != 0) {
            true => {
                let inner = {
                    let mut accum = (Vec::new());
                    for _ in 0..(2 << (descriptor.flags & 7)) {
                        (accum.push((Decoder166(scope, input))?));
                    }
                    accum
                };
                (Type3::yes(inner))
            }

            false => {
                let _ = ();
                Type3::no
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (Decoder167(scope, input))? };
    (Some(Type11 {
        descriptor,
        local_color_table,
        data,
    }))
}

fn Decoder164<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type8> {
    let separator = {
        let b = (input.read_byte())?;
        if (b == 33) {
            b
        } else {
            return None;
        }
    };
    let label = {
        let b = (input.read_byte())?;
        if (b == 1) {
            b
        } else {
            return None;
        }
    };
    let block_size = {
        let b = (input.read_byte())?;
        if (b == 12) {
            b
        } else {
            return None;
        }
    };
    let text_grid_left_position = { (Decoder135(scope, input))? };
    let text_grid_top_position = { (Decoder135(scope, input))? };
    let text_grid_width = { (Decoder135(scope, input))? };
    let text_grid_height = { (Decoder135(scope, input))? };
    let character_cell_width = { (Decoder17(scope, input))? };
    let character_cell_height = { (Decoder17(scope, input))? };
    let text_foreground_color_index = { (Decoder17(scope, input))? };
    let text_background_color_index = { (Decoder17(scope, input))? };
    let plain_text_data = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder159(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder160(scope, input))? };
    (Some(Type8 {
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
    }))
}

fn Decoder165<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type9> {
    let separator = {
        let b = (input.read_byte())?;
        if (b == 44) {
            b
        } else {
            return None;
        }
    };
    let image_left_position = { (Decoder135(scope, input))? };
    let image_top_position = { (Decoder135(scope, input))? };
    let image_width = { (Decoder135(scope, input))? };
    let image_height = { (Decoder135(scope, input))? };
    let flags = { (Decoder17(scope, input))? };
    (Some(Type9 {
        separator,
        image_left_position,
        image_top_position,
        image_width,
        image_height,
        flags,
    }))
}

fn Decoder166<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type2> {
    let r = { (Decoder17(scope, input))? };
    let g = { (Decoder17(scope, input))? };
    let b = { (Decoder17(scope, input))? };
    (Some(Type2 { r, g, b }))
}

fn Decoder167<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type10> {
    let lzw_min_code_size = { (Decoder17(scope, input))? };
    let image_data = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,

                    _other => {
                        (unreachable!(r#"unexpected: {:?}"#, _other));
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder159(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder160(scope, input))? };
    (Some(Type10 {
        lzw_min_code_size,
        image_data,
        terminator,
    }))
}

fn Decoder168<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type1> {
    let screen_width = { (Decoder135(scope, input))? };
    let screen_height = { (Decoder135(scope, input))? };
    let flags = { (Decoder17(scope, input))? };
    let bg_color_index = { (Decoder17(scope, input))? };
    let pixel_aspect_ratio = { (Decoder17(scope, input))? };
    (Some(Type1 {
        screen_width,
        screen_height,
        flags,
        bg_color_index,
        pixel_aspect_ratio,
    }))
}

#[test]

fn test_decoder_28() {
    // PNG signature
    let input = b"\x89PNG\r\n\x1A\n";
    let mut parse_ctxt = ParseCtxt::new(input);
    let mut scope = Scope::Empty;
    let ret = Decoder28(&mut scope, &mut parse_ctxt);
    assert!(ret.is_some());
}
