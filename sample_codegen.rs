
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

fn Decoder0<'input>(input: &mut ParseMonad<'input>) -> Result<Type196, ParseError> {
    (PResult::Ok((Decoder1(input))?))
}

fn Decoder1<'input>(input: &mut ParseMonad<'input>) -> Result<Type196, ParseError> {
    let data = ((|| {
        PResult::Ok({
            (input.start_alt());
            {
                let mut f_tmp = (|| {
                    PResult::Ok({
                        let inner = (Decoder2(input))?;
                        (Type193::gif(inner))
                    })
                });
                match (f_tmp()) {
                    Ok(inner) => {
                        return (PResult::Ok(inner));
                    }

                    Err(_e) => {
                        (input.next_alt(false));
                    }
                }
            };
            {
                let mut f_tmp = (|| {
                    PResult::Ok({
                        let inner = (Decoder3(input))?;
                        (Type193::gzip(inner))
                    })
                });
                match (f_tmp()) {
                    Ok(inner) => {
                        return (PResult::Ok(inner));
                    }

                    Err(_e) => {
                        (input.next_alt(false));
                    }
                }
            };
            {
                let mut f_tmp = (|| {
                    PResult::Ok({
                        let inner = (Decoder4(input))?;
                        (Type193::jpeg(inner))
                    })
                });
                match (f_tmp()) {
                    Ok(inner) => {
                        return (PResult::Ok(inner));
                    }

                    Err(_e) => {
                        (input.next_alt(false));
                    }
                }
            };
            {
                let mut f_tmp = (|| {
                    PResult::Ok({
                        let inner = (Decoder5(input))?;
                        (Type193::mpeg4(inner))
                    })
                });
                match (f_tmp()) {
                    Ok(inner) => {
                        return (PResult::Ok(inner));
                    }

                    Err(_e) => {
                        (input.next_alt(false));
                    }
                }
            };
            {
                let mut f_tmp = (|| {
                    PResult::Ok({
                        let inner = (Decoder6(input))?;
                        (Type193::png(inner))
                    })
                });
                match (f_tmp()) {
                    Ok(inner) => {
                        return (PResult::Ok(inner));
                    }

                    Err(_e) => {
                        (input.next_alt(false));
                    }
                }
            };
            {
                let mut f_tmp = (|| {
                    PResult::Ok({
                        let inner = (Decoder7(input))?;
                        (Type193::riff(inner))
                    })
                });
                match (f_tmp()) {
                    Ok(inner) => {
                        return (PResult::Ok(inner));
                    }

                    Err(_e) => {
                        (input.next_alt(false));
                    }
                }
            };
            {
                let mut f_tmp = (|| {
                    PResult::Ok({
                        let inner = (Decoder8(input))?;
                        (Type193::tar(inner))
                    })
                });
                match (f_tmp()) {
                    Ok(inner) => {
                        return (PResult::Ok(inner));
                    }

                    Err(_e) => {
                        (input.next_alt(true));
                    }
                }
            };
            {
                let mut f_tmp = (|| {
                    PResult::Ok({
                        let inner = (Decoder9(input))?;
                        (Type193::text(inner))
                    })
                });
                match (f_tmp()) {
                    Ok(inner) => {
                        return (PResult::Ok(inner));
                    }

                    Err(_e) => {
                        return (Err(_e));
                    }
                }
            };
            (panic!(r#"last branch should return something unconditionally"#))
        })
    })())?;
    let end = ((|| {
        PResult::Ok({
            if (input.remaining() == 0) {
                ()
            } else {
                return (Err(ParseError::IncompleteParse));
            }
        })
    })())?;
    (PResult::Ok(Type196 { data, end }))
}

fn Decoder2<'input>(input: &mut ParseMonad<'input>) -> Result<Type19, ParseError> {
    let header = ((|| PResult::Ok({ (Decoder152(input))? }))())?;
    let logical_screen = ((|| PResult::Ok({ (Decoder153(input))? }))())?;
    let blocks = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            33 => 0,

                            44 => 0,

                            59 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder154(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let trailer = ((|| PResult::Ok({ (Decoder155(input))? }))())?;
    (PResult::Ok(Type19 {
        header,
        logical_screen,
        blocks,
        trailer,
    }))
}

fn Decoder3<'input>(input: &mut ParseMonad<'input>) -> Result<Vec<Type40>, ParseError> {
    let mut accum = (Vec::new());
    while (input.remaining() > 0) {
        let matching_ix = {
            (input.open_peek_context());
            let b = (input.read_byte())?;
            {
                let ret = if (b == 31) { 1 } else { 0 };
                (input.close_peek_context())?;
                ret
            }
        };
        if (matching_ix == 0) {
            break;
        } else {
            let next_elem = {
                let header = ((|| PResult::Ok({ (Decoder140(input))? }))())?;
                let fname = ((|| {
                    PResult::Ok({
                        match (header.file_flags & 8 != 0) {
                            true => {
                                let inner = (Decoder141(input))?;
                                (Type22::yes(inner))
                            }

                            false => {
                                let _ = ();
                                Type22::no
                            }
                        }
                    })
                })())?;
                let data =
                    ((|| PResult::Ok({ (unimplemented!(r#"translate @ Decoder::Bits"#)) }))())?;
                let footer = ((|| PResult::Ok({ (Decoder143(input))? }))())?;
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
    (PResult::Ok(accum))
}

fn Decoder4<'input>(input: &mut ParseMonad<'input>) -> Result<Type80, ParseError> {
    let soi = ((|| PResult::Ok({ (Decoder67(input))? }))())?;
    let frame = ((|| PResult::Ok({ (Decoder68(input))? }))())?;
    let eoi = ((|| PResult::Ok({ (Decoder69(input))? }))())?;
    (PResult::Ok(Type80 { soi, frame, eoi }))
}

fn Decoder5<'input>(input: &mut ParseMonad<'input>) -> Result<Type163, ParseError> {
    let atoms = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    {
                        let ret = 0;
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder46(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    (PResult::Ok(Type163 { atoms }))
}

fn Decoder6<'input>(input: &mut ParseMonad<'input>) -> Result<Type181, ParseError> {
    let signature = ((|| PResult::Ok({ (Decoder28(input))? }))())?;
    let ihdr = ((|| PResult::Ok({ (Decoder29(input))? }))())?;
    let chunks = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            98 => 0,

                            112 => 0,

                            80 => 0,

                            116 => 0,

                            73 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder30(input, ihdr))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let idat = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            73 => {
                                let b = (input.read_byte())?;
                                match b {
                                    69 => 0,

                                    68 => 1,

                                    _ => {
                                        return (Err(ParseError::ExcludedBranch));
                                    }
                                }
                            }

                            98 => 0,

                            112 => 0,

                            80 => 0,

                            116 => 0,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    break;
                } else {
                    let next_elem = (Decoder31(input))?;
                    (accum.push(next_elem));
                }
            }
            accum
        })
    })())?;
    let more_chunks = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            98 => 0,

                            112 => 0,

                            80 => 0,

                            116 => 0,

                            73 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder30(input, ihdr))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let iend = ((|| PResult::Ok({ (Decoder32(input))? }))())?;
    (PResult::Ok(Type181 {
        signature,
        ihdr,
        chunks,
        idat,
        more_chunks,
        iend,
    }))
}

fn Decoder7<'input>(input: &mut ParseMonad<'input>) -> Result<Type185, ParseError> {
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 82) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 73) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 70) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 70) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder24(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder25(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    let pad = ((|| {
        PResult::Ok({
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
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (Type182::no(inner))
                }
            }
        })
    })())?;
    (PResult::Ok(Type185 {
        tag,
        length,
        data,
        pad,
    }))
}

fn Decoder8<'input>(input: &mut ParseMonad<'input>) -> Result<Type191, ParseError> {
    let contents = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            0 => 0,

                            tmp if (tmp != 0) => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    break;
                } else {
                    let next_elem = (Decoder15(input))?;
                    (accum.push(next_elem));
                }
            }
            accum
        })
    })())?;
    let __padding = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..1024 {
                (accum.push({
                    let b = (input.read_byte())?;
                    if (b == 0) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                }));
            }
            accum
        })
    })())?;
    let __trailing = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = if (b == 0) { 0 } else { 1 };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b == 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    (PResult::Ok(Type191 {
        contents,
        __padding,
        __trailing,
    }))
}

fn Decoder9<'input>(input: &mut ParseMonad<'input>) -> Result<Type192, ParseError> {
    (input.start_alt());
    {
        let mut f_tmp = (|| {
            PResult::Ok({
                let inner = (Decoder10(input))?;
                (Type192::ascii(inner))
            })
        });
        match (f_tmp()) {
            Ok(inner) => {
                return (PResult::Ok(inner));
            }

            Err(_e) => {
                (input.next_alt(true));
            }
        }
    };
    {
        let mut f_tmp = (|| {
            PResult::Ok({
                let inner = (Decoder11(input))?;
                (Type192::utf8(inner))
            })
        });
        match (f_tmp()) {
            Ok(inner) => {
                return (PResult::Ok(inner));
            }

            Err(_e) => {
                return (Err(_e));
            }
        }
    };
    (PResult::Ok((panic!(r#"last branch should return something unconditionally"#))))
}

fn Decoder10<'input>(input: &mut ParseMonad<'input>) -> Result<Vec<u8>, ParseError> {
    let mut accum = (Vec::new());
    while (input.remaining() > 0) {
        let matching_ix = {
            (input.open_peek_context());
            let b = (input.read_byte())?;
            {
                let ret =
                    if ((ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0]))
                        .contains(b))
                    {
                        1
                    } else {
                        0
                    };
                (input.close_peek_context())?;
                ret
            }
        };
        if (matching_ix == 0) {
            break;
        } else {
            let next_elem = (Decoder14(input))?;
            (accum.push(next_elem));
        }
    }
    (PResult::Ok(accum))
}

fn Decoder11<'input>(input: &mut ParseMonad<'input>) -> Result<Vec<char>, ParseError> {
    let mut accum = (Vec::new());
    while (input.remaining() > 0) {
        let matching_ix = {
            (input.open_peek_context());
            let b = (input.read_byte())?;
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

                    224 => 0,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => 0,

                    237 => 0,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => 0,

                    240 => 0,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => 0,

                    244 => 0,

                    _ => {
                        return (Err(ParseError::ExcludedBranch));
                    }
                };
                (input.close_peek_context())?;
                ret
            }
        };
        if (matching_ix == 0) {
            let next_elem = (Decoder12(input))?;
            (accum.push(next_elem));
        } else {
            break;
        }
    }
    (PResult::Ok(accum))
}

fn Decoder12<'input>(input: &mut ParseMonad<'input>) -> Result<char, ParseError> {
    let inner = {
        let tree_index = {
            (input.open_peek_context());
            let b = (input.read_byte())?;
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

                    224 => 2,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => 2,

                    237 => 2,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => 2,

                    240 => 3,

                    tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => 3,

                    244 => 3,

                    _ => {
                        return (Err(ParseError::ExcludedBranch));
                    }
                };
                (input.close_peek_context())?;
                ret
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
                        return (Err(ParseError::ExcludedBranch));
                    }
                };
                ((|byte: u8| PResult::Ok((byte as u32)))(inner))?
            }

            1 => {
                let inner = {
                    let field0 = ((|| {
                        PResult::Ok({
                            let inner = {
                                let b = (input.read_byte())?;
                                if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(b)) {
                                    b
                                } else {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            };
                            ((|raw: u8| PResult::Ok((raw & 31)))(inner))?
                        })
                    })())?;
                    let field1 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                    (field0, field1)
                };
                ((|bytes: (u8, u8)| {
                    PResult::Ok(match bytes {
                        (x1, x0) => ((x1 as u32) << 6 | (x0 as u32)),
                    })
                })(inner))?
            }

            2 => {
                let inner = {
                    let tree_index = {
                        (input.open_peek_context());
                        let b = (input.read_byte())?;
                        {
                            let ret = match b {
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

                                _ => {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            };
                            (input.close_peek_context())?;
                            ret
                        }
                    };
                    match tree_index {
                        0 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if (b == 224) {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 15)))(inner))?
                                })
                            })())?;
                            let field1 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if ((ByteSet::from_bits([0, 0, 18446744069414584320, 0]))
                                            .contains(b))
                                        {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 63)))(inner))?
                                })
                            })())?;
                            let field2 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            (field0, field1, field2)
                        }

                        1 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if ((ByteSet::from_bits([0, 0, 0, 35175782154240]))
                                            .contains(b))
                                        {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 15)))(inner))?
                                })
                            })())?;
                            let field1 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            let field2 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            (field0, field1, field2)
                        }

                        2 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if (b == 237) {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 15)))(inner))?
                                })
                            })())?;
                            let field1 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if ((ByteSet::from_bits([0, 0, 4294967295, 0])).contains(b))
                                        {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 63)))(inner))?
                                })
                            })())?;
                            let field2 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            (field0, field1, field2)
                        }

                        3 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if ((ByteSet::from_bits([0, 0, 0, 211106232532992]))
                                            .contains(b))
                                        {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 15)))(inner))?
                                })
                            })())?;
                            let field1 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            let field2 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            (field0, field1, field2)
                        }

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    }
                };
                ((|bytes: (u8, u8, u8)| {
                    PResult::Ok(match bytes {
                        (x2, x1, x0) => ((x2 as u32) << 12 | (x1 as u32) << 6 | (x0 as u32)),
                    })
                })(inner))?
            }

            3 => {
                let inner = {
                    let tree_index = {
                        (input.open_peek_context());
                        let b = (input.read_byte())?;
                        {
                            let ret = match b {
                                240 => 0,

                                tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184]))
                                    .contains(tmp)) =>
                                {
                                    1
                                }

                                244 => 2,

                                _ => {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            };
                            (input.close_peek_context())?;
                            ret
                        }
                    };
                    match tree_index {
                        0 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if (b == 240) {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 7)))(inner))?
                                })
                            })())?;
                            let field1 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if ((ByteSet::from_bits([0, 0, 18446744073709486080, 0]))
                                            .contains(b))
                                        {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 63)))(inner))?
                                })
                            })())?;
                            let field2 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            let field3 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            (field0, field1, field2, field3)
                        }

                        1 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if ((ByteSet::from_bits([0, 0, 0, 3940649673949184]))
                                            .contains(b))
                                        {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 7)))(inner))?
                                })
                            })())?;
                            let field1 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            let field2 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            let field3 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            (field0, field1, field2, field3)
                        }

                        2 => {
                            let field0 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if (b == 244) {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 7)))(inner))?
                                })
                            })())?;
                            let field1 = ((|| {
                                PResult::Ok({
                                    let inner = {
                                        let b = (input.read_byte())?;
                                        if ((ByteSet::from_bits([0, 0, 65535, 0])).contains(b)) {
                                            b
                                        } else {
                                            return (Err(ParseError::ExcludedBranch));
                                        }
                                    };
                                    ((|raw: u8| PResult::Ok((raw & 63)))(inner))?
                                })
                            })())?;
                            let field2 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            let field3 = ((|| PResult::Ok({ (Decoder13(input))? }))())?;
                            (field0, field1, field2, field3)
                        }

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    }
                };
                ((|bytes: (u8, u8, u8, u8)| {
                    PResult::Ok(match bytes {
                        (x3, x2, x1, x0) => {
                            ((x3 as u32) << 18 | (x2 as u32) << 12 | (x1 as u32) << 6 | (x0 as u32))
                        }
                    })
                })(inner))?
            }

            _ => {
                return (Err(ParseError::ExcludedBranch));
            }
        }
    };
    (PResult::Ok(((|codepoint: u32| PResult::Ok(((char::from_u32(codepoint)).unwrap())))(inner))?))
}

fn Decoder13<'input>(input: &mut ParseMonad<'input>) -> Result<u8, ParseError> {
    let inner = {
        let b = (input.read_byte())?;
        if ((ByteSet::from_bits([0, 0, 18446744073709551615, 0])).contains(b)) {
            b
        } else {
            return (Err(ParseError::ExcludedBranch));
        }
    };
    (PResult::Ok(((|raw: u8| PResult::Ok((raw & 63)))(inner))?))
}

fn Decoder14<'input>(input: &mut ParseMonad<'input>) -> Result<u8, ParseError> {
    let b = (input.read_byte())?;
    (PResult::Ok(
        if ((ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0])).contains(b)) {
            b
        } else {
            return (Err(ParseError::ExcludedBranch));
        },
    ))
}

fn Decoder15<'input>(input: &mut ParseMonad<'input>) -> Result<Type190, ParseError> {
    let header = ((|| PResult::Ok({ (Decoder16(input))? }))())?;
    let file = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..header.size {
                (accum.push((Decoder17(input))?));
            }
            accum
        })
    })())?;
    let __padding = ((|| {
        PResult::Ok({
            (input.skip_align(512))?;
            ()
        })
    })())?;
    (PResult::Ok(Type190 {
        header,
        file,
        __padding,
    }))
}

fn Decoder16<'input>(input: &mut ParseMonad<'input>) -> Result<Type189, ParseError> {
    let sz = (512 as usize);
    (input.start_slice(sz))?;
    let mut ret =
        ((|| {
            PResult::Ok({
                let name = ((|| {
                    PResult::Ok({
                        let sz = (100 as usize);
                        (input.start_slice(sz))?;
                        let mut ret = ((|| PResult::Ok({ (Decoder18(input))? }))())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let mode = ((|| {
                    PResult::Ok({
                        let sz = (8 as usize);
                        (input.start_slice(sz))?;
                        let mut ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = (Vec::new());
                                                while (input.remaining() > 0) {
                                                    let matching_ix =
                                                        {
                                                            (input.open_peek_context());
                                                            let b = (input.read_byte())?;
                                                            {
                                                                let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return (Err(ParseError::ExcludedBranch));
}
};
                                                                (input.close_peek_context())?;
                                                                ret
                                                            }
                                                        };
                                                    if (matching_ix == 0) {
                                                        let next_elem = (Decoder19(input))?;
                                                        (accum.push(next_elem));
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp =
                                        ((|| PResult::Ok({ (Decoder20(input))? }))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = (Vec::new());
                                            while (input.remaining() > 0) {
                                                let matching_ix = {
                                                    (input.open_peek_context());
                                                    let b = (input.read_byte())?;
                                                    {
                                                        let ret = if (b == 0) { 0 } else { 1 };
                                                        (input.close_peek_context())?;
                                                        ret
                                                    }
                                                };
                                                if (matching_ix == 0) {
                                                    let next_elem = {
                                                        let b = (input.read_byte())?;
                                                        if (b == 0) {
                                                            b
                                                        } else {
                                                            return (Err(
                                                                ParseError::ExcludedBranch,
                                                            ));
                                                        }
                                                    };
                                                    (accum.push(next_elem));
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type187 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let uid = ((|| {
                    PResult::Ok({
                        let sz = (8 as usize);
                        (input.start_slice(sz))?;
                        let mut ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = (Vec::new());
                                                while (input.remaining() > 0) {
                                                    let matching_ix =
                                                        {
                                                            (input.open_peek_context());
                                                            let b = (input.read_byte())?;
                                                            {
                                                                let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return (Err(ParseError::ExcludedBranch));
}
};
                                                                (input.close_peek_context())?;
                                                                ret
                                                            }
                                                        };
                                                    if (matching_ix == 0) {
                                                        let next_elem = (Decoder19(input))?;
                                                        (accum.push(next_elem));
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp =
                                        ((|| PResult::Ok({ (Decoder20(input))? }))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = (Vec::new());
                                            while (input.remaining() > 0) {
                                                let matching_ix = {
                                                    (input.open_peek_context());
                                                    let b = (input.read_byte())?;
                                                    {
                                                        let ret = if (b == 0) { 0 } else { 1 };
                                                        (input.close_peek_context())?;
                                                        ret
                                                    }
                                                };
                                                if (matching_ix == 0) {
                                                    let next_elem = {
                                                        let b = (input.read_byte())?;
                                                        if (b == 0) {
                                                            b
                                                        } else {
                                                            return (Err(
                                                                ParseError::ExcludedBranch,
                                                            ));
                                                        }
                                                    };
                                                    (accum.push(next_elem));
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type187 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let gid = ((|| {
                    PResult::Ok({
                        let sz = (8 as usize);
                        (input.start_slice(sz))?;
                        let mut ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = (Vec::new());
                                                while (input.remaining() > 0) {
                                                    let matching_ix =
                                                        {
                                                            (input.open_peek_context());
                                                            let b = (input.read_byte())?;
                                                            {
                                                                let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return (Err(ParseError::ExcludedBranch));
}
};
                                                                (input.close_peek_context())?;
                                                                ret
                                                            }
                                                        };
                                                    if (matching_ix == 0) {
                                                        let next_elem = (Decoder19(input))?;
                                                        (accum.push(next_elem));
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp =
                                        ((|| PResult::Ok({ (Decoder20(input))? }))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = (Vec::new());
                                            while (input.remaining() > 0) {
                                                let matching_ix = {
                                                    (input.open_peek_context());
                                                    let b = (input.read_byte())?;
                                                    {
                                                        let ret = if (b == 0) { 0 } else { 1 };
                                                        (input.close_peek_context())?;
                                                        ret
                                                    }
                                                };
                                                if (matching_ix == 0) {
                                                    let next_elem = {
                                                        let b = (input.read_byte())?;
                                                        if (b == 0) {
                                                            b
                                                        } else {
                                                            return (Err(
                                                                ParseError::ExcludedBranch,
                                                            ));
                                                        }
                                                    };
                                                    (accum.push(next_elem));
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type187 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let size = ((|| {
                    PResult::Ok({
                        let inner = {
                            let oA = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let o9 = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let o8 = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let o7 = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let o6 = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let o5 = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let o4 = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let o3 = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let o2 = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let o1 = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let o0 = ((|| {
                                PResult::Ok({
                                    let inner = (Decoder19(input))?;
                                    ((|bit: u8| PResult::Ok(((bit as u8) - 48)))(inner))?
                                })
                            })())?;
                            let __nil = ((|| PResult::Ok({ (Decoder20(input))? }))())?;
                            let value = ((|| {
                                PResult::Ok({
                                    ((((0 as u32) << 3 | (oA as u32)) << 6
                                        | (o9 as u32) << 3
                                        | (o8 as u32))
                                        << 24
                                        | (((o7 as u32) << 3 | (o6 as u32)) << 6
                                            | (o5 as u32) << 3
                                            | (o4 as u32))
                                            << 12
                                        | ((o3 as u32) << 3 | (o2 as u32)) << 6
                                        | (o1 as u32) << 3
                                        | (o0 as u32))
                                })
                            })())?;
                            Type195 {
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
                        ((|rec: Type195| PResult::Ok(rec.value))(inner))?
                    })
                })())?;
                let mtime = ((|| {
                    PResult::Ok({
                        let sz = (12 as usize);
                        (input.start_slice(sz))?;
                        let mut ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = (Vec::new());
                                                while (input.remaining() > 0) {
                                                    let matching_ix =
                                                        {
                                                            (input.open_peek_context());
                                                            let b = (input.read_byte())?;
                                                            {
                                                                let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return (Err(ParseError::ExcludedBranch));
}
};
                                                                (input.close_peek_context())?;
                                                                ret
                                                            }
                                                        };
                                                    if (matching_ix == 0) {
                                                        let next_elem = (Decoder19(input))?;
                                                        (accum.push(next_elem));
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp =
                                        ((|| PResult::Ok({ (Decoder20(input))? }))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = (Vec::new());
                                            while (input.remaining() > 0) {
                                                let matching_ix = {
                                                    (input.open_peek_context());
                                                    let b = (input.read_byte())?;
                                                    {
                                                        let ret = if (b == 0) { 0 } else { 1 };
                                                        (input.close_peek_context())?;
                                                        ret
                                                    }
                                                };
                                                if (matching_ix == 0) {
                                                    let next_elem = {
                                                        let b = (input.read_byte())?;
                                                        if (b == 0) {
                                                            b
                                                        } else {
                                                            return (Err(
                                                                ParseError::ExcludedBranch,
                                                            ));
                                                        }
                                                    };
                                                    (accum.push(next_elem));
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type187 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let chksum = ((|| {
                    PResult::Ok({
                        let sz = (8 as usize);
                        (input.start_slice(sz))?;
                        let mut ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = (Vec::new());
                                                while (input.remaining() > 0) {
                                                    let matching_ix =
                                                        {
                                                            (input.open_peek_context());
                                                            let b = (input.read_byte())?;
                                                            {
                                                                let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return (Err(ParseError::ExcludedBranch));
}
};
                                                                (input.close_peek_context())?;
                                                                ret
                                                            }
                                                        };
                                                    if (matching_ix == 0) {
                                                        let next_elem = (Decoder19(input))?;
                                                        (accum.push(next_elem));
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp =
                                        ((|| PResult::Ok({ (Decoder20(input))? }))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = (Vec::new());
                                            while (input.remaining() > 0) {
                                                let matching_ix = {
                                                    (input.open_peek_context());
                                                    let b = (input.read_byte())?;
                                                    {
                                                        let ret = if (b == 0) { 0 } else { 1 };
                                                        (input.close_peek_context())?;
                                                        ret
                                                    }
                                                };
                                                if (matching_ix == 0) {
                                                    let next_elem = {
                                                        let b = (input.read_byte())?;
                                                        if (b == 0) {
                                                            b
                                                        } else {
                                                            return (Err(
                                                                ParseError::ExcludedBranch,
                                                            ));
                                                        }
                                                    };
                                                    (accum.push(next_elem));
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type187 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let typeflag = ((|| PResult::Ok({ (Decoder21(input))? }))())?;
                let linkname = ((|| {
                    PResult::Ok({
                        let sz = (100 as usize);
                        (input.start_slice(sz))?;
                        let mut ret = ((|| PResult::Ok({ (Decoder22(input))? }))())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let magic = ((|| {
                    PResult::Ok({
                        let field0 = ((|| {
                            PResult::Ok({
                                let b = (input.read_byte())?;
                                if (b == 117) {
                                    b
                                } else {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            })
                        })())?;
                        let field1 = ((|| {
                            PResult::Ok({
                                let b = (input.read_byte())?;
                                if (b == 115) {
                                    b
                                } else {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            })
                        })())?;
                        let field2 = ((|| {
                            PResult::Ok({
                                let b = (input.read_byte())?;
                                if (b == 116) {
                                    b
                                } else {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            })
                        })())?;
                        let field3 = ((|| {
                            PResult::Ok({
                                let b = (input.read_byte())?;
                                if (b == 97) {
                                    b
                                } else {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            })
                        })())?;
                        let field4 = ((|| {
                            PResult::Ok({
                                let b = (input.read_byte())?;
                                if (b == 114) {
                                    b
                                } else {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            })
                        })())?;
                        let field5 = ((|| {
                            PResult::Ok({
                                let b = (input.read_byte())?;
                                if (b == 0) {
                                    b
                                } else {
                                    return (Err(ParseError::ExcludedBranch));
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
                                let b = (input.read_byte())?;
                                if (b == 48) {
                                    b
                                } else {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            })
                        })())?;
                        let field1 = ((|| {
                            PResult::Ok({
                                let b = (input.read_byte())?;
                                if (b == 48) {
                                    b
                                } else {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            })
                        })())?;
                        (field0, field1)
                    })
                })())?;
                let uname = ((|| {
                    PResult::Ok({
                        let sz = (32 as usize);
                        (input.start_slice(sz))?;
                        let mut ret = ((|| PResult::Ok({ (Decoder23(input))? }))())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let gname = ((|| {
                    PResult::Ok({
                        let sz = (32 as usize);
                        (input.start_slice(sz))?;
                        let mut ret = ((|| PResult::Ok({ (Decoder23(input))? }))())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let devmajor = ((|| {
                    PResult::Ok({
                        let sz = (8 as usize);
                        (input.start_slice(sz))?;
                        let mut ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = (Vec::new());
                                                while (input.remaining() > 0) {
                                                    let matching_ix =
                                                        {
                                                            (input.open_peek_context());
                                                            let b = (input.read_byte())?;
                                                            {
                                                                let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return (Err(ParseError::ExcludedBranch));
}
};
                                                                (input.close_peek_context())?;
                                                                ret
                                                            }
                                                        };
                                                    if (matching_ix == 0) {
                                                        let next_elem = (Decoder19(input))?;
                                                        (accum.push(next_elem));
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp =
                                        ((|| PResult::Ok({ (Decoder20(input))? }))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = (Vec::new());
                                            while (input.remaining() > 0) {
                                                let matching_ix = {
                                                    (input.open_peek_context());
                                                    let b = (input.read_byte())?;
                                                    {
                                                        let ret = if (b == 0) { 0 } else { 1 };
                                                        (input.close_peek_context())?;
                                                        ret
                                                    }
                                                };
                                                if (matching_ix == 0) {
                                                    let next_elem = {
                                                        let b = (input.read_byte())?;
                                                        if (b == 0) {
                                                            b
                                                        } else {
                                                            return (Err(
                                                                ParseError::ExcludedBranch,
                                                            ));
                                                        }
                                                    };
                                                    (accum.push(next_elem));
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type187 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let devminor = ((|| {
                    PResult::Ok({
                        let sz = (8 as usize);
                        (input.start_slice(sz))?;
                        let mut ret =
                            ((|| {
                                PResult::Ok({
                                    let string =
                                        ((|| {
                                            PResult::Ok({
                                                let mut accum = (Vec::new());
                                                while (input.remaining() > 0) {
                                                    let matching_ix =
                                                        {
                                                            (input.open_peek_context());
                                                            let b = (input.read_byte())?;
                                                            {
                                                                let ret = match b {
tmp if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(tmp)) => {
0
},

tmp if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(tmp)) => {
1
},

_ => {
return (Err(ParseError::ExcludedBranch));
}
};
                                                                (input.close_peek_context())?;
                                                                ret
                                                            }
                                                        };
                                                    if (matching_ix == 0) {
                                                        let next_elem = (Decoder19(input))?;
                                                        (accum.push(next_elem));
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                accum
                                            })
                                        })())?;
                                    let __nul_or_wsp =
                                        ((|| PResult::Ok({ (Decoder20(input))? }))())?;
                                    let __padding = ((|| {
                                        PResult::Ok({
                                            let mut accum = (Vec::new());
                                            while (input.remaining() > 0) {
                                                let matching_ix = {
                                                    (input.open_peek_context());
                                                    let b = (input.read_byte())?;
                                                    {
                                                        let ret = if (b == 0) { 0 } else { 1 };
                                                        (input.close_peek_context())?;
                                                        ret
                                                    }
                                                };
                                                if (matching_ix == 0) {
                                                    let next_elem = {
                                                        let b = (input.read_byte())?;
                                                        if (b == 0) {
                                                            b
                                                        } else {
                                                            return (Err(
                                                                ParseError::ExcludedBranch,
                                                            ));
                                                        }
                                                    };
                                                    (accum.push(next_elem));
                                                } else {
                                                    break;
                                                }
                                            }
                                            accum
                                        })
                                    })())?;
                                    Type187 {
                                        string,
                                        __nul_or_wsp,
                                        __padding,
                                    }
                                })
                            })())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let prefix = ((|| {
                    PResult::Ok({
                        let sz = (155 as usize);
                        (input.start_slice(sz))?;
                        let mut ret = ((|| PResult::Ok({ (Decoder22(input))? }))())?;
                        (input.end_slice())?;
                        ret
                    })
                })())?;
                let pad = ((|| {
                    PResult::Ok({
                        let mut accum = (Vec::new());
                        for _ in 0..12 {
                            (accum.push({
                                let b = (input.read_byte())?;
                                if (b == 0) {
                                    b
                                } else {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            }));
                        }
                        accum
                    })
                })())?;
                Type189 {
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
    (input.end_slice())?;
    (PResult::Ok(ret))
}

fn Decoder17<'input>(input: &mut ParseMonad<'input>) -> Result<u8, ParseError> {
    let b = (input.read_byte())?;
    (PResult::Ok(b))
}

fn Decoder18<'input>(input: &mut ParseMonad<'input>) -> Result<Type186, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            0 => 0,

                            tmp if (tmp != 0) => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
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
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                }
            }
            accum
        })
    })())?;
    let __padding = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = if (b == 0) { 0 } else { 1 };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b == 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    (PResult::Ok(Type186 { string, __padding }))
}

fn Decoder19<'input>(input: &mut ParseMonad<'input>) -> Result<u8, ParseError> {
    let b = (input.read_byte())?;
    (PResult::Ok(
        if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(b)) {
            b
        } else {
            return (Err(ParseError::ExcludedBranch));
        },
    ))
}

fn Decoder20<'input>(input: &mut ParseMonad<'input>) -> Result<u8, ParseError> {
    let b = (input.read_byte())?;
    (PResult::Ok(
        if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(b)) {
            b
        } else {
            return (Err(ParseError::ExcludedBranch));
        },
    ))
}

fn Decoder21<'input>(input: &mut ParseMonad<'input>) -> Result<u8, ParseError> {
    let b = (input.read_byte())?;
    (PResult::Ok(b))
}

fn Decoder22<'input>(input: &mut ParseMonad<'input>) -> Result<Type186, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let __padding = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = if (b == 0) { 0 } else { 1 };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b == 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    (PResult::Ok(Type186 { string, __padding }))
}

fn Decoder23<'input>(input: &mut ParseMonad<'input>) -> Result<Type188, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let padding = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = if (b == 0) { 1 } else { 0 };
                        (input.close_peek_context())?;
                        ret
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
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                }
            }
            accum
        })
    })())?;
    (PResult::Ok(Type188 { string, padding }))
}

fn Decoder24<'input>(input: &mut ParseMonad<'input>) -> Result<u32, ParseError> {
    let inner = {
        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field3 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        (field0, field1, field2, field3)
    };
    (PResult::Ok(((|x: (u8, u8, u8, u8)| PResult::Ok((u32le(x))))(inner))?))
}

fn Decoder25<'input>(input: &mut ParseMonad<'input>) -> Result<Type184, ParseError> {
    let tag = ((|| PResult::Ok({ (Decoder26(input))? }))())?;
    let chunks = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    {
                        let ret = 0;
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder27(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    (PResult::Ok(Type184 { tag, chunks }))
}

fn Decoder26<'input>(input: &mut ParseMonad<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
    let field0 = ((|| PResult::Ok({ (Decoder21(input))? }))())?;
    let field1 = ((|| PResult::Ok({ (Decoder21(input))? }))())?;
    let field2 = ((|| PResult::Ok({ (Decoder21(input))? }))())?;
    let field3 = ((|| PResult::Ok({ (Decoder21(input))? }))())?;
    (PResult::Ok((field0, field1, field2, field3)))
}

fn Decoder27<'input>(input: &mut ParseMonad<'input>) -> Result<Type183, ParseError> {
    let tag = ((|| PResult::Ok({ (Decoder26(input))? }))())?;
    let length = ((|| PResult::Ok({ (Decoder24(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    let pad = ((|| {
        PResult::Ok({
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
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (Type182::no(inner))
                }
            }
        })
    })())?;
    (PResult::Ok(Type183 {
        tag,
        length,
        data,
        pad,
    }))
}

fn Decoder28<'input>(
    input: &mut ParseMonad<'input>,
) -> Result<(u8, u8, u8, u8, u8, u8, u8, u8), ParseError> {
    let field0 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 137) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field1 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 80) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field2 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 78) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field3 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 71) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field4 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 13) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field5 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 10) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field6 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 26) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field7 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 10) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok((
        field0, field1, field2, field3, field4, field5, field6, field7,
    )))
}

fn Decoder29<'input>(input: &mut ParseMonad<'input>) -> Result<Type165, ParseError> {
    let length = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let tag = ((|| PResult::Ok({ (Decoder44(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder45(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    (PResult::Ok(Type165 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder30<'input>(input: &mut ParseMonad<'input>, ihdr: Type165) -> Result<Type178, ParseError> {
    let tree_index = {
        (input.open_peek_context());
        let b = (input.read_byte())?;
        {
            let ret = match b {
                98 => 0,

                112 => 1,

                80 => 2,

                116 => {
                    let b = (input.read_byte())?;
                    match b {
                        73 => 3,

                        82 => 4,

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    }
                }

                _ => {
                    return (Err(ParseError::ExcludedBranch));
                }
            };
            (input.close_peek_context())?;
            ret
        }
    };
    (PResult::Ok(match tree_index {
        0 => {
            let inner = (Decoder38(input, ihdr))?;
            (Type178::bKGD(inner))
        }

        1 => {
            let inner = (Decoder39(input))?;
            (Type178::pHYs(inner))
        }

        2 => {
            let inner = (Decoder40(input))?;
            (Type178::PLTE(inner))
        }

        3 => {
            let inner = (Decoder41(input))?;
            (Type178::tIME(inner))
        }

        4 => {
            let inner = (Decoder42(input, ihdr))?;
            (Type178::tRNS(inner))
        }

        _ => {
            return (Err(ParseError::ExcludedBranch));
        }
    }))
}

fn Decoder31<'input>(input: &mut ParseMonad<'input>) -> Result<Type179, ParseError> {
    let length = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let tag = ((|| PResult::Ok({ (Decoder36(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder37(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    (PResult::Ok(Type179 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder32<'input>(input: &mut ParseMonad<'input>) -> Result<Type180, ParseError> {
    let length = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let tag = ((|| PResult::Ok({ (Decoder34(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder35(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    (PResult::Ok(Type180 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder33<'input>(input: &mut ParseMonad<'input>) -> Result<u32, ParseError> {
    let inner = {
        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field3 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        (field0, field1, field2, field3)
    };
    (PResult::Ok(((|x: (u8, u8, u8, u8)| PResult::Ok((u32be(x))))(inner))?))
}

fn Decoder34<'input>(input: &mut ParseMonad<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
    let field0 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 73) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field1 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 69) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field2 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 78) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field3 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 68) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok((field0, field1, field2, field3)))
}

fn Decoder35<'input>(input: &mut ParseMonad<'input>) -> Result<(), ParseError> {
    (PResult::Ok(()))
}

fn Decoder36<'input>(input: &mut ParseMonad<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
    let field0 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 73) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field1 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 68) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field2 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 65) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field3 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 84) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok((field0, field1, field2, field3)))
}

fn Decoder37<'input>(input: &mut ParseMonad<'input>) -> Result<Vec<u8>, ParseError> {
    let mut accum = (Vec::new());
    while (input.remaining() > 0) {
        let matching_ix = {
            (input.open_peek_context());
            {
                let ret = 0;
                (input.close_peek_context())?;
                ret
            }
        };
        if (matching_ix == 0) {
            let next_elem = (Decoder17(input))?;
            (accum.push(next_elem));
        } else {
            break;
        }
    }
    (PResult::Ok(accum))
}

fn Decoder38<'input>(input: &mut ParseMonad<'input>, ihdr: Type165) -> Result<Type171, ParseError> {
    let length = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 98) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 75) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 71) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 68) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match ihdr.data.color_type {
                        0 => {
                            let inner = {
                                let greyscale = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                Type167 { greyscale }
                            };
                            (Type170::color_type_0(inner))
                        }

                        4 => {
                            let inner = {
                                let greyscale = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                Type167 { greyscale }
                            };
                            (Type170::color_type_4(inner))
                        }

                        2 => {
                            let inner = {
                                let red = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let green = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let blue = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                Type168 { red, green, blue }
                            };
                            (Type170::color_type_2(inner))
                        }

                        6 => {
                            let inner = {
                                let red = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let green = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let blue = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                Type168 { red, green, blue }
                            };
                            (Type170::color_type_6(inner))
                        }

                        3 => {
                            let inner = {
                                let palette_index = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                Type169 { palette_index }
                            };
                            (Type170::color_type_3(inner))
                        }

                        _other => {
                            (unreachable!(
                                r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                            ));
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    (PResult::Ok(Type171 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder39<'input>(input: &mut ParseMonad<'input>) -> Result<Type173, ParseError> {
    let length = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 112) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 72) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 89) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 115) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let pixels_per_unit_x = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                    let pixels_per_unit_y = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                    let unit_specifier = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                    Type172 {
                        pixels_per_unit_x,
                        pixels_per_unit_y,
                        unit_specifier,
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    (PResult::Ok(Type173 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder40<'input>(input: &mut ParseMonad<'input>) -> Result<Type166, ParseError> {
    let length = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 80) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 76) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 84) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 69) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 1;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            break;
                        } else {
                            let next_elem = {
                                let r = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let g = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let b = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                Type2 { r, g, b }
                            };
                            (accum.push(next_elem));
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    (PResult::Ok(Type166 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder41<'input>(input: &mut ParseMonad<'input>) -> Result<Type175, ParseError> {
    let length = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 116) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 73) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 77) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 69) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let year = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                    let month = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                    let day = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                    let hour = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                    let minute = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                    let second = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                    Type174 {
                        year,
                        month,
                        day,
                        hour,
                        minute,
                        second,
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    (PResult::Ok(Type175 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder42<'input>(input: &mut ParseMonad<'input>, ihdr: Type165) -> Result<Type177, ParseError> {
    let length = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let tag = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 116) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 82) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 78) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field3 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 83) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            (field0, field1, field2, field3)
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (length as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match ihdr.data.color_type {
                        0 => {
                            let inner = {
                                let greyscale = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                Type167 { greyscale }
                            };
                            (Type176::color_type_0(inner))
                        }

                        2 => {
                            let inner = {
                                let red = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let green = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let blue = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                Type168 { red, green, blue }
                            };
                            (Type176::color_type_2(inner))
                        }

                        3 => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = {
                                            let palette_index =
                                                ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                            Type169 { palette_index }
                                        };
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type176::color_type_3(inner))
                        }

                        _other => {
                            (unreachable!(
                                r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                            ));
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    let crc = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    (PResult::Ok(Type177 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder43<'input>(input: &mut ParseMonad<'input>) -> Result<u16, ParseError> {
    let inner = {
        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        (field0, field1)
    };
    (PResult::Ok(((|x: (u8, u8)| PResult::Ok((u16be(x))))(inner))?))
}

fn Decoder44<'input>(input: &mut ParseMonad<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
    let field0 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 73) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field1 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 72) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field2 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 68) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let field3 = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 82) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok((field0, field1, field2, field3)))
}

fn Decoder45<'input>(input: &mut ParseMonad<'input>) -> Result<Type164, ParseError> {
    let width = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let height = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let bit_depth = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let color_type = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let compression_method = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let filter_method = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let interlace_method = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    (PResult::Ok(Type164 {
        width,
        height,
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
    }))
}

fn Decoder46<'input>(input: &mut ParseMonad<'input>) -> Result<Type162, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (102, 116, 121, 112) => {
                            let inner = {
                                let major_brand = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
                                let minor_version = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let compatible_brands = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        while (input.remaining() > 0) {
                                            let matching_ix = {
                                                (input.open_peek_context());
                                                {
                                                    let ret = 0;
                                                    (input.close_peek_context())?;
                                                    ret
                                                }
                                            };
                                            if (matching_ix == 0) {
                                                let next_elem = (Decoder47(input))?;
                                                (accum.push(next_elem));
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
                            (Type161::ftyp(inner))
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
                            let field0 = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                            let field1 = ((|| {
                                PResult::Ok({
                                    let mut accum = (Vec::new());
                                    while (input.remaining() > 0) {
                                        let matching_ix = {
                                            (input.open_peek_context());
                                            {
                                                let ret = 0;
                                                (input.close_peek_context())?;
                                                ret
                                            }
                                        };
                                        if (matching_ix == 0) {
                                            let next_elem = (Decoder49(input))?;
                                            (accum.push(next_elem));
                                        } else {
                                            break;
                                        }
                                    }
                                    accum
                                })
                            })())?;
                            (Type161::meta(field0, field1))
                        }

                        (109, 111, 111, 118) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder50(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type161::moov(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type161::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type162 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder47<'input>(input: &mut ParseMonad<'input>) -> Result<(u8, u8, u8, u8), ParseError> {
    let field0 = ((|| PResult::Ok({ (Decoder21(input))? }))())?;
    let field1 = ((|| PResult::Ok({ (Decoder21(input))? }))())?;
    let field2 = ((|| PResult::Ok({ (Decoder21(input))? }))())?;
    let field3 = ((|| PResult::Ok({ (Decoder21(input))? }))())?;
    (PResult::Ok((field0, field1, field2, field3)))
}

fn Decoder48<'input>(input: &mut ParseMonad<'input>) -> Result<u64, ParseError> {
    let inner = {
        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field3 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field4 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field5 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field6 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field7 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        (
            field0, field1, field2, field3, field4, field5, field6, field7,
        )
    };
    (PResult::Ok(((|x: (u8, u8, u8, u8, u8, u8, u8, u8)| PResult::Ok((u64be(x))))(inner))?))
}

fn Decoder49<'input>(input: &mut ParseMonad<'input>) -> Result<Type115, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (100, 105, 110, 102) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder57(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type114::dinf(inner))
                        }

                        (104, 100, 108, 114) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let predefined = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let handler_type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
                                let reserved = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let name = ((|| PResult::Ok({ (Decoder55(input))? }))())?;
                                Type86 {
                                    version,
                                    flags,
                                    predefined,
                                    handler_type,
                                    reserved,
                                    name,
                                }
                            };
                            (Type114::hdlr(inner))
                        }

                        (112, 105, 116, 109) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let item_ID = ((|| {
                                    PResult::Ok({
                                        match (version == 0) {
                                            true => {
                                                let inner = (Decoder43(input))?;
                                                (Type112::yes(inner))
                                            }

                                            false => {
                                                let inner = (Decoder33(input))?;
                                                (Type112::no(inner))
                                            }
                                        }
                                    })
                                })())?;
                                Type113 {
                                    version,
                                    flags,
                                    item_ID,
                                }
                            };
                            (Type114::pitm(inner))
                        }

                        (105, 105, 110, 102) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let entry_count = ((|| {
                                    PResult::Ok({
                                        match (version == 0) {
                                            true => {
                                                let inner = (Decoder43(input))?;
                                                ((|x: u16| PResult::Ok((x as u32)))(inner))?
                                            }

                                            false => (Decoder33(input))?,
                                        }
                                    })
                                })())?;
                                let item_info_entry = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..entry_count {
                                            (accum.push((Decoder59(input))?));
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
                            (Type114::iinf(inner))
                        }

                        (105, 114, 101, 102) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let single_item_reference = ((|| {
                                    PResult::Ok({
                                        match version {
                                            0 => {
                                                let inner = {
                                                    let mut accum = (Vec::new());
                                                    while (input.remaining() > 0) {
                                                        let matching_ix = {
                                                            (input.open_peek_context());
                                                            {
                                                                let ret = 0;
                                                                (input.close_peek_context())?;
                                                                ret
                                                            }
                                                        };
                                                        if (matching_ix == 0) {
                                                            let next_elem = {
                                                                let size_field = ((|| {
                                                                    PResult::Ok({
                                                                        (Decoder33(input))?
                                                                    })
                                                                })(
                                                                ))?;
                                                                let r#type = ((|| {
                                                                    PResult::Ok({
                                                                        (Decoder47(input))?
                                                                    })
                                                                })(
                                                                ))?;
                                                                let size = ((|| {
                                                                    PResult::Ok({
                                                                        match size_field {
                                                                            0 => 0,

                                                                            1 => {
                                                                                let inner =
                                                                                    (Decoder48(
                                                                                        input,
                                                                                    ))?;
                                                                                ((|x: u64| {
                                                                                    PResult::Ok(
                                                                                        (x - 16),
                                                                                    )
                                                                                })(
                                                                                    inner
                                                                                ))?
                                                                            }

                                                                            _ => {
                                                                                ((size_field - 8)
                                                                                    as u64)
                                                                            }
                                                                        }
                                                                    })
                                                                })(
                                                                ))?;
                                                                let data = ((|| {
                                                                    PResult::Ok({
                                                                        let sz = (size as usize);
                                                                        (input.start_slice(sz))?;
                                                                        let mut ret = ((|| {
                                                                            PResult::Ok({
                                                                                let from_item_ID =
                                                                                    ((|| {
                                                                                        PResult::Ok(
                                                                                            {
                                                                                                (Decoder43(input))?
                                                                                            },
                                                                                        )
                                                                                    })(
                                                                                    ))?;
                                                                                let reference_count =
                                                                                    ((|| {
                                                                                        PResult::Ok(
                                                                                            {
                                                                                                (Decoder43(input))?
                                                                                            },
                                                                                        )
                                                                                    })(
                                                                                    ))?;
                                                                                let to_item_ID =
                                                                                    ((|| {
                                                                                        PResult::Ok(
                                                                                            {
                                                                                                let mut accum = (Vec::new());
                                                                                                for _ in 0..reference_count {
(accum.push((Decoder43(input))?));
}
                                                                                                accum
                                                                                            },
                                                                                        )
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
                                                                        (input.end_slice())?;
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
                                                            (accum.push(next_elem));
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    accum
                                                };
                                                (Type110::small(inner))
                                            }

                                            1 => {
                                                let inner = {
                                                    let mut accum = (Vec::new());
                                                    while (input.remaining() > 0) {
                                                        let matching_ix = {
                                                            (input.open_peek_context());
                                                            {
                                                                let ret = 0;
                                                                (input.close_peek_context())?;
                                                                ret
                                                            }
                                                        };
                                                        if (matching_ix == 0) {
                                                            let next_elem = {
                                                                let size_field = ((|| {
                                                                    PResult::Ok({
                                                                        (Decoder33(input))?
                                                                    })
                                                                })(
                                                                ))?;
                                                                let r#type = ((|| {
                                                                    PResult::Ok({
                                                                        (Decoder47(input))?
                                                                    })
                                                                })(
                                                                ))?;
                                                                let size = ((|| {
                                                                    PResult::Ok({
                                                                        match size_field {
                                                                            0 => 0,

                                                                            1 => {
                                                                                let inner =
                                                                                    (Decoder48(
                                                                                        input,
                                                                                    ))?;
                                                                                ((|x: u64| {
                                                                                    PResult::Ok(
                                                                                        (x - 16),
                                                                                    )
                                                                                })(
                                                                                    inner
                                                                                ))?
                                                                            }

                                                                            _ => {
                                                                                ((size_field - 8)
                                                                                    as u64)
                                                                            }
                                                                        }
                                                                    })
                                                                })(
                                                                ))?;
                                                                let data = ((|| {
                                                                    PResult::Ok({
                                                                        let sz = (size as usize);
                                                                        (input.start_slice(sz))?;
                                                                        let mut ret = ((|| {
                                                                            PResult::Ok({
                                                                                let from_item_ID =
                                                                                    ((|| {
                                                                                        PResult::Ok(
                                                                                            {
                                                                                                (Decoder33(input))?
                                                                                            },
                                                                                        )
                                                                                    })(
                                                                                    ))?;
                                                                                let reference_count =
                                                                                    ((|| {
                                                                                        PResult::Ok(
                                                                                            {
                                                                                                (Decoder43(input))?
                                                                                            },
                                                                                        )
                                                                                    })(
                                                                                    ))?;
                                                                                let to_item_ID =
                                                                                    ((|| {
                                                                                        PResult::Ok(
                                                                                            {
                                                                                                let mut accum = (Vec::new());
                                                                                                for _ in 0..reference_count {
(accum.push((Decoder33(input))?));
}
                                                                                                accum
                                                                                            },
                                                                                        )
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
                                                                        (input.end_slice())?;
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
                                                            (accum.push(next_elem));
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    accum
                                                };
                                                (Type110::large(inner))
                                            }

                                            _other => {
                                                (unreachable!(
                                                    r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                                                ));
                                            }
                                        }
                                    })
                                })())?;
                                Type111 {
                                    version,
                                    flags,
                                    single_item_reference,
                                }
                            };
                            (Type114::iref(inner))
                        }

                        (105, 108, 111, 99) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let offset_size_length_size =
                                    ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let base_offset_size_index_size =
                                    ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let offset_size =
                                    ((|| PResult::Ok({ (offset_size_length_size >> 4) }))())?;
                                let length_size =
                                    ((|| PResult::Ok({ (offset_size_length_size & 7) }))())?;
                                let base_offset_size =
                                    ((|| PResult::Ok({ (base_offset_size_index_size >> 4) }))())?;
                                let index_size = ((|| {
                                    PResult::Ok({
                                        match (version > 0) {
                                            true => (base_offset_size_index_size & 7),

                                            false => 0,
                                        }
                                    })
                                })())?;
                                let item_count = ((|| {
                                    PResult::Ok({
                                        match (version < 2) {
                                            true => {
                                                let inner = (Decoder43(input))?;
                                                ((|x: u16| PResult::Ok((x as u32)))(inner))?
                                            }

                                            false => (Decoder33(input))?,
                                        }
                                    })
                                })())?;
                                let items = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..item_count {
                                            (accum.push({
let item_ID = ((|| PResult::Ok({
match (version < 2) {
true => {
let inner = (Decoder43(input))?;
(((|x: u16| PResult::Ok((x as u32))))(inner))?
},

false => {
(Decoder33(input))?
}
}
}))())?;
let construction_method = ((|| PResult::Ok({
match (version > 0) {
true => {
let inner = (Decoder43(input))?;
(Type97::yes(inner))
},

false => {
let _ = ();
Type97::no
}
}
}))())?;
let data_reference_index = ((|| PResult::Ok({
(Decoder43(input))?
}))())?;
let base_offset = ((|| PResult::Ok({
match base_offset_size {
0 => {
0
},

4 => {
let inner = (Decoder33(input))?;
(((|x: u32| PResult::Ok((x as u64))))(inner))?
},

8 => {
(Decoder48(input))?
},

_other => {
(unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#));
}
}
}))())?;
let extent_count = ((|| PResult::Ok({
(Decoder43(input))?
}))())?;
let extents = ((|| PResult::Ok({
let mut accum = (Vec::new());
for _ in 0..extent_count {
(accum.push({
let extent_index = ((|| PResult::Ok({
match index_size {
0 => {
0
},

4 => {
let inner = (Decoder33(input))?;
(((|x: u32| PResult::Ok((x as u64))))(inner))?
},

8 => {
(Decoder48(input))?
},

_other => {
(unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#));
}
}
}))())?;
let extent_offset = ((|| PResult::Ok({
match offset_size {
0 => {
0
},

4 => {
let inner = (Decoder33(input))?;
(((|x: u32| PResult::Ok((x as u64))))(inner))?
},

8 => {
(Decoder48(input))?
},

_other => {
(unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#));
}
}
}))())?;
let extent_length = ((|| PResult::Ok({
match length_size {
0 => {
0
},

4 => {
let inner = (Decoder33(input))?;
(((|x: u32| PResult::Ok((x as u64))))(inner))?
},

8 => {
(Decoder48(input))?
},

_other => {
(unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#));
}
}
}))())?;
Type98 { extent_index, extent_offset, extent_length }
}));
}
accum
}))())?;
Type99 { item_ID, construction_method, data_reference_index, base_offset, extent_count, extents }
}));
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
                            (Type114::iloc(inner))
                        }

                        (105, 108, 115, 116) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder60(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type114::ilst(inner))
                        }

                        (105, 100, 97, 116) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type114::idat(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type114::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type115 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder50<'input>(input: &mut ParseMonad<'input>) -> Result<Type160, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (109, 118, 104, 100) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let fields = ((|| {
                                    PResult::Ok({
                                        match version {
                                            0 => {
                                                let inner = {
                                                    let creation_time = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let modification_time = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let timescale = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let duration = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    Type116 {
                                                        creation_time,
                                                        modification_time,
                                                        timescale,
                                                        duration,
                                                    }
                                                };
                                                (Type118::version0(inner))
                                            }

                                            1 => {
                                                let inner = {
                                                    let creation_time = ((|| {
                                                        PResult::Ok({ (Decoder48(input))? })
                                                    })(
                                                    ))?;
                                                    let modification_time = ((|| {
                                                        PResult::Ok({ (Decoder48(input))? })
                                                    })(
                                                    ))?;
                                                    let timescale = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let duration = ((|| {
                                                        PResult::Ok({ (Decoder48(input))? })
                                                    })(
                                                    ))?;
                                                    Type117 {
                                                        creation_time,
                                                        modification_time,
                                                        timescale,
                                                        duration,
                                                    }
                                                };
                                                (Type118::version1(inner))
                                            }

                                            _other => {
                                                (unreachable!(
                                                    r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                                                ));
                                            }
                                        }
                                    })
                                })())?;
                                let rate = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let volume = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let reserved1 = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let reserved2 = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                        (field0, field1)
                                    })
                                })())?;
                                let matrix = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..9 {
                                            (accum.push((Decoder33(input))?));
                                        }
                                        accum
                                    })
                                })())?;
                                let pre_defined = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..6 {
                                            (accum.push((Decoder33(input))?));
                                        }
                                        accum
                                    })
                                })())?;
                                let next_track_ID = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
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
                            (Type159::mvhd(inner))
                        }

                        (116, 114, 97, 107) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder51(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type159::trak(inner))
                        }

                        (117, 100, 116, 97) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder52(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type159::udta(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type159::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type160 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder51<'input>(input: &mut ParseMonad<'input>) -> Result<Type156, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (116, 107, 104, 100) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let fields = ((|| {
                                    PResult::Ok({
                                        match version {
                                            0 => {
                                                let inner = {
                                                    let creation_time = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let modification_time = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let track_ID = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let reserved = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let duration = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    Type151 {
                                                        creation_time,
                                                        modification_time,
                                                        track_ID,
                                                        reserved,
                                                        duration,
                                                    }
                                                };
                                                (Type153::version0(inner))
                                            }

                                            1 => {
                                                let inner = {
                                                    let creation_time = ((|| {
                                                        PResult::Ok({ (Decoder48(input))? })
                                                    })(
                                                    ))?;
                                                    let modification_time = ((|| {
                                                        PResult::Ok({ (Decoder48(input))? })
                                                    })(
                                                    ))?;
                                                    let track_ID = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let reserved = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let duration = ((|| {
                                                        PResult::Ok({ (Decoder48(input))? })
                                                    })(
                                                    ))?;
                                                    Type152 {
                                                        creation_time,
                                                        modification_time,
                                                        track_ID,
                                                        reserved,
                                                        duration,
                                                    }
                                                };
                                                (Type153::version1(inner))
                                            }

                                            _other => {
                                                (unreachable!(
                                                    r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                                                ));
                                            }
                                        }
                                    })
                                })())?;
                                let reserved2 = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                        (field0, field1)
                                    })
                                })())?;
                                let layer = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let alternate_group =
                                    ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let volume = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let reserved1 = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let matrix = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..9 {
                                            (accum.push((Decoder33(input))?));
                                        }
                                        accum
                                    })
                                })())?;
                                let width = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let height = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
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
                            (Type155::tkhd(inner))
                        }

                        (101, 100, 116, 115) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder53(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type155::edts(inner))
                        }

                        (109, 100, 105, 97) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder54(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type155::mdia(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type155::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type156 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder52<'input>(input: &mut ParseMonad<'input>) -> Result<Type158, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (109, 101, 116, 97) => {
                            let field0 = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                            let field1 = ((|| {
                                PResult::Ok({
                                    let mut accum = (Vec::new());
                                    while (input.remaining() > 0) {
                                        let matching_ix = {
                                            (input.open_peek_context());
                                            {
                                                let ret = 0;
                                                (input.close_peek_context())?;
                                                ret
                                            }
                                        };
                                        if (matching_ix == 0) {
                                            let next_elem = (Decoder49(input))?;
                                            (accum.push(next_elem));
                                        } else {
                                            break;
                                        }
                                    }
                                    accum
                                })
                            })())?;
                            (Type157::meta(field0, field1))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type157::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type158 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder53<'input>(input: &mut ParseMonad<'input>) -> Result<Type123, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (101, 108, 115, 116) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let number_of_entries =
                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let edit_list_table = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..number_of_entries {
                                            (accum.push({
                                                let track_duration =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                let media_time =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                let media_rate =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                Type120 {
                                                    track_duration,
                                                    media_time,
                                                    media_rate,
                                                }
                                            }));
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
                            (Type122::elst(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type122::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type123 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder54<'input>(input: &mut ParseMonad<'input>) -> Result<Type150, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (104, 100, 108, 114) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let component_type = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let component_subtype =
                                    ((|| PResult::Ok({ (Decoder47(input))? }))())?;
                                let component_manufacturer =
                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let component_flags =
                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let component_flags_mask =
                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let component_name = ((|| PResult::Ok({ (Decoder55(input))? }))())?;
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
                            (Type149::hdlr(inner))
                        }

                        (109, 100, 104, 100) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let fields = ((|| {
                                    PResult::Ok({
                                        match version {
                                            0 => {
                                                let inner = {
                                                    let creation_time = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let modification_time = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let timescale = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let duration = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    Type116 {
                                                        creation_time,
                                                        modification_time,
                                                        timescale,
                                                        duration,
                                                    }
                                                };
                                                (Type118::version0(inner))
                                            }

                                            1 => {
                                                let inner = {
                                                    let creation_time = ((|| {
                                                        PResult::Ok({ (Decoder48(input))? })
                                                    })(
                                                    ))?;
                                                    let modification_time = ((|| {
                                                        PResult::Ok({ (Decoder48(input))? })
                                                    })(
                                                    ))?;
                                                    let timescale = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let duration = ((|| {
                                                        PResult::Ok({ (Decoder48(input))? })
                                                    })(
                                                    ))?;
                                                    Type117 {
                                                        creation_time,
                                                        modification_time,
                                                        timescale,
                                                        duration,
                                                    }
                                                };
                                                (Type118::version1(inner))
                                            }

                                            _other => {
                                                (unreachable!(
                                                    r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                                                ));
                                            }
                                        }
                                    })
                                })())?;
                                let language = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let pre_defined = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                Type125 {
                                    version,
                                    flags,
                                    fields,
                                    language,
                                    pre_defined,
                                }
                            };
                            (Type149::mdhd(inner))
                        }

                        (109, 105, 110, 102) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder56(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type149::minf(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type149::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type150 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder55<'input>(input: &mut ParseMonad<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type21 { string, null }))
}

fn Decoder56<'input>(input: &mut ParseMonad<'input>) -> Result<Type148, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (118, 109, 104, 100) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let graphicsmode = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let opcolor = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..3 {
                                            (accum.push((Decoder43(input))?));
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
                            (Type147::vmhd(inner))
                        }

                        (115, 109, 104, 100) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let balance = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                let reserved = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
                                Type126 {
                                    version,
                                    flags,
                                    balance,
                                    reserved,
                                }
                            };
                            (Type147::smhd(inner))
                        }

                        (100, 105, 110, 102) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder57(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type147::dinf(inner))
                        }

                        (115, 116, 98, 108) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder58(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type147::stbl(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type147::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type148 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder57<'input>(input: &mut ParseMonad<'input>) -> Result<Type85, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (100, 114, 101, 102) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let number_of_entries =
                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let data = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        while (input.remaining() > 0) {
                                            let matching_ix = {
                                                (input.open_peek_context());
                                                {
                                                    let ret = 0;
                                                    (input.close_peek_context())?;
                                                    ret
                                                }
                                            };
                                            if (matching_ix == 0) {
                                                let next_elem = {
                                                    let size_field = ((|| {
                                                        PResult::Ok({ (Decoder33(input))? })
                                                    })(
                                                    ))?;
                                                    let r#type = ((|| {
                                                        PResult::Ok({ (Decoder47(input))? })
                                                    })(
                                                    ))?;
                                                    let size = ((|| {
                                                        PResult::Ok({
                                                            match size_field {
                                                                0 => 0,

                                                                1 => {
                                                                    let inner = (Decoder48(input))?;
                                                                    ((|x: u64| {
                                                                        PResult::Ok((x - 16))
                                                                    })(
                                                                        inner
                                                                    ))?
                                                                }

                                                                _ => ((size_field - 8) as u64),
                                                            }
                                                        })
                                                    })(
                                                    ))?;
                                                    let data = ((|| {
                                                        PResult::Ok({
                                                            let sz = (size as usize);
                                                            (input.start_slice(sz))?;
                                                            let mut ret = ((|| {
                                                                PResult::Ok({
                                                                    let mut accum = (Vec::new());
                                                                    while (input.remaining() > 0) {
                                                                        let matching_ix = {
                                                                            (input
                                                                                .open_peek_context(
                                                                                ));
                                                                            {
                                                                                let ret = 0;
                                                                                (input.close_peek_context())?;
                                                                                ret
                                                                            }
                                                                        };
                                                                        if (matching_ix == 0) {
                                                                            let next_elem =
                                                                                (Decoder17(input))?;
                                                                            (accum.push(next_elem));
                                                                        } else {
                                                                            break;
                                                                        }
                                                                    }
                                                                    accum
                                                                })
                                                            })(
                                                            ))?;
                                                            (input.end_slice())?;
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
                                                (accum.push(next_elem));
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
                            (Type84::dref(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type84::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type85 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder58<'input>(input: &mut ParseMonad<'input>) -> Result<Type145, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (115, 116, 115, 100) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let entry_count = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let sample_entries =
                                    ((|| {
                                        PResult::Ok({
                                            let mut accum = (Vec::new());
                                            for _ in 0..entry_count {
                                                (accum.push({
let size_field = ((|| PResult::Ok({
(Decoder33(input))?
}))())?;
let r#type = ((|| PResult::Ok({
(Decoder47(input))?
}))())?;
let size = ((|| PResult::Ok({
match size_field {
0 => {
0
},

1 => {
let inner = (Decoder48(input))?;
(((|x: u64| PResult::Ok((x - 16))))(inner))?
},

_ => {
((size_field - 8) as u64)
}
}
}))())?;
let data = ((|| PResult::Ok({
let sz = (size as usize<>);
(input.start_slice(sz))?;
let mut ret = ((|| PResult::Ok({
let mut accum = (Vec::new());
while (input.remaining() > 0) {
let matching_ix = {
(input.open_peek_context());
{
let ret = 0;
(input.close_peek_context())?;
ret
}
};
if (matching_ix == 0) {
let next_elem = (Decoder17(input))?;
(accum.push(next_elem));
} else {
break
}
}
accum
}))())?;
(input.end_slice())?;
ret
}))())?;
Type82 { size_field, r#type, size, data }
}));
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
                            (Type144::stsd(inner))
                        }

                        (115, 116, 116, 115) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let entry_count = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let sample_entries = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..entry_count {
                                            (accum.push({
                                                let sample_count =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                let sample_delta =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                Type142 {
                                                    sample_count,
                                                    sample_delta,
                                                }
                                            }));
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
                            (Type144::stts(inner))
                        }

                        (99, 116, 116, 115) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let entry_count = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let sample_entries = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..entry_count {
                                            (accum.push({
                                                let sample_count =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                let sample_offset =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                Type128 {
                                                    sample_count,
                                                    sample_offset,
                                                }
                                            }));
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
                            (Type144::ctts(inner))
                        }

                        (115, 116, 115, 115) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let entry_count = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let sample_number = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..entry_count {
                                            (accum.push((Decoder33(input))?));
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
                            (Type144::stss(inner))
                        }

                        (115, 116, 115, 99) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let entry_count = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let chunk_entries = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..entry_count {
                                            (accum.push({
                                                let first_chunk =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                let samples_per_chunk =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                let sample_description_index =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                Type136 {
                                                    first_chunk,
                                                    samples_per_chunk,
                                                    sample_description_index,
                                                }
                                            }));
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
                            (Type144::stsc(inner))
                        }

                        (115, 116, 115, 122) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let sample_size = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let sample_count = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let entry_size = ((|| {
                                    PResult::Ok({
                                        match (sample_size == 0) {
                                            true => {
                                                let inner = {
                                                    let mut accum = (Vec::new());
                                                    for _ in 0..sample_count {
                                                        (accum.push((Decoder33(input))?));
                                                    }
                                                    accum
                                                };
                                                (Type140::yes(inner))
                                            }

                                            false => {
                                                let _ = ();
                                                Type140::no
                                            }
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
                            (Type144::stsz(inner))
                        }

                        (115, 116, 99, 111) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let entry_count = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let chunk_offset = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..entry_count {
                                            (accum.push((Decoder33(input))?));
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
                            (Type144::stco(inner))
                        }

                        (99, 111, 54, 52) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let entry_count = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let chunk_offset = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..entry_count {
                                            (accum.push((Decoder48(input))?));
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
                            (Type144::co64(inner))
                        }

                        (115, 103, 112, 100) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let grouping_type = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let default_length = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let entry_count = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let sample_groups = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..entry_count {
                                            (accum.push({
                                                let description_length = ((|| {
                                                    PResult::Ok({
                                                        match (default_length == 0) {
                                                            true => (Decoder33(input))?,

                                                            false => default_length,
                                                        }
                                                    })
                                                })(
                                                ))?;
                                                let sample_group_entry = ((|| {
                                                    PResult::Ok({
                                                        let mut accum = (Vec::new());
                                                        for _ in 0..description_length {
                                                            (accum.push((Decoder17(input))?));
                                                        }
                                                        accum
                                                    })
                                                })(
                                                ))?;
                                                Type133 {
                                                    description_length,
                                                    sample_group_entry,
                                                }
                                            }));
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
                            (Type144::sgpd(inner))
                        }

                        (115, 98, 103, 112) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let grouping_type = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let grouping_type_parameter = ((|| {
                                    PResult::Ok({
                                        match (version == 1) {
                                            true => {
                                                let inner = (Decoder33(input))?;
                                                (Type130::yes(inner))
                                            }

                                            false => {
                                                let _ = ();
                                                Type130::no
                                            }
                                        }
                                    })
                                })(
                                ))?;
                                let entry_count = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let sample_groups = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        for _ in 0..entry_count {
                                            (accum.push({
                                                let sample_count =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                let group_description_index =
                                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                                Type131 {
                                                    sample_count,
                                                    group_description_index,
                                                }
                                            }));
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
                            (Type144::sbgp(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type144::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type145 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder59<'input>(input: &mut ParseMonad<'input>) -> Result<Type95, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (105, 110, 102, 101) => {
                            let inner = {
                                let version = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                let flags = ((|| {
                                    PResult::Ok({
                                        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        let field2 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
                                        (field0, field1, field2)
                                    })
                                })())?;
                                let fields = ((|| {
                                    PResult::Ok({
                                        match (version < 2) {
                                            true => {
                                                let inner = {
                                                    let item_ID = ((|| {
                                                        PResult::Ok({ (Decoder43(input))? })
                                                    })(
                                                    ))?;
                                                    let item_protection_index = ((|| {
                                                        PResult::Ok({ (Decoder43(input))? })
                                                    })(
                                                    ))?;
                                                    let item_name = ((|| {
                                                        PResult::Ok({ (Decoder62(input))? })
                                                    })(
                                                    ))?;
                                                    let content_type = ((|| {
                                                        PResult::Ok({ (Decoder63(input))? })
                                                    })(
                                                    ))?;
                                                    let content_encoding = ((|| {
                                                        PResult::Ok({ (Decoder64(input))? })
                                                    })(
                                                    ))?;
                                                    Type91 {
                                                        item_ID,
                                                        item_protection_index,
                                                        item_name,
                                                        content_type,
                                                        content_encoding,
                                                    }
                                                };
                                                (Type92::yes(inner))
                                            }

                                            false => {
                                                let inner = {
                                                    let item_ID = ((|| {
                                                        PResult::Ok({
                                                            match (version == 2) {
                                                                true => {
                                                                    let inner = (Decoder43(input))?;
                                                                    ((|x: u16| {
                                                                        PResult::Ok((x as u32))
                                                                    })(
                                                                        inner
                                                                    ))?
                                                                }

                                                                false => (Decoder33(input))?,
                                                            }
                                                        })
                                                    })(
                                                    ))?;
                                                    let item_protection_index = ((|| {
                                                        PResult::Ok({ (Decoder43(input))? })
                                                    })(
                                                    ))?;
                                                    let item_type = ((|| {
                                                        PResult::Ok({ (Decoder47(input))? })
                                                    })(
                                                    ))?;
                                                    let item_name = ((|| {
                                                        PResult::Ok({ (Decoder65(input))? })
                                                    })(
                                                    ))?;
                                                    let extra_fields = ((|| {
                                                        PResult::Ok({
                                                            match item_type {
                                                                (109, 105, 109, 101) => {
                                                                    let inner = {
                                                                        let content_type =
                                                                            ((|| {
                                                                                PResult::Ok({
                                                                                    (Decoder66(
                                                                                        input,
                                                                                    ))?
                                                                                })
                                                                            })(
                                                                            ))?;
                                                                        Type87 { content_type }
                                                                    };
                                                                    (Type89::mime(inner))
                                                                }

                                                                (117, 114, 105, 32) => {
                                                                    let inner = {
                                                                        let item_uri_type =
                                                                            ((|| {
                                                                                PResult::Ok({
                                                                                    (Decoder66(
                                                                                        input,
                                                                                    ))?
                                                                                })
                                                                            })(
                                                                            ))?;
                                                                        Type88 { item_uri_type }
                                                                    };
                                                                    (Type89::uri(inner))
                                                                }

                                                                _ => {
                                                                    let _ = ();
                                                                    Type89::unknown
                                                                }
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
                                                (Type92::no(inner))
                                            }
                                        }
                                    })
                                })())?;
                                Type93 {
                                    version,
                                    flags,
                                    fields,
                                }
                            };
                            (Type94::infe(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type94::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type95 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder60<'input>(input: &mut ParseMonad<'input>) -> Result<Type105, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (169, 116, 111, 111) => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder61(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type104::tool(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type104::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type105 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder61<'input>(input: &mut ParseMonad<'input>) -> Result<Type103, ParseError> {
    let size_field = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
    let r#type = ((|| PResult::Ok({ (Decoder47(input))? }))())?;
    let size = ((|| {
        PResult::Ok({
            match size_field {
                0 => 0,

                1 => {
                    let inner = (Decoder48(input))?;
                    ((|x: u64| PResult::Ok((x - 16)))(inner))?
                }

                _ => ((size_field - 8) as u64),
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let sz = (size as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    match r#type {
                        (100, 97, 116, 97) => {
                            let inner = {
                                let type_indicator = ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let locale_indicator =
                                    ((|| PResult::Ok({ (Decoder33(input))? }))())?;
                                let value = ((|| {
                                    PResult::Ok({
                                        let mut accum = (Vec::new());
                                        while (input.remaining() > 0) {
                                            let matching_ix = {
                                                (input.open_peek_context());
                                                {
                                                    let ret = 0;
                                                    (input.close_peek_context())?;
                                                    ret
                                                }
                                            };
                                            if (matching_ix == 0) {
                                                let next_elem = (Decoder21(input))?;
                                                (accum.push(next_elem));
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
                            (Type102::data(inner))
                        }

                        _ => {
                            let inner = {
                                let mut accum = (Vec::new());
                                while (input.remaining() > 0) {
                                    let matching_ix = {
                                        (input.open_peek_context());
                                        {
                                            let ret = 0;
                                            (input.close_peek_context())?;
                                            ret
                                        }
                                    };
                                    if (matching_ix == 0) {
                                        let next_elem = (Decoder17(input))?;
                                        (accum.push(next_elem));
                                    } else {
                                        break;
                                    }
                                }
                                accum
                            };
                            (Type102::unknown(inner))
                        }
                    }
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type103 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder62<'input>(input: &mut ParseMonad<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type21 { string, null }))
}

fn Decoder63<'input>(input: &mut ParseMonad<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type21 { string, null }))
}

fn Decoder64<'input>(input: &mut ParseMonad<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type21 { string, null }))
}

fn Decoder65<'input>(input: &mut ParseMonad<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type21 { string, null }))
}

fn Decoder66<'input>(input: &mut ParseMonad<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type21 { string, null }))
}

fn Decoder67<'input>(input: &mut ParseMonad<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 216) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type41 { ff, marker }))
}

fn Decoder68<'input>(input: &mut ParseMonad<'input>) -> Result<Type79, ParseError> {
    let initial_segment = ((|| {
        PResult::Ok({
            let tree_index = {
                (input.open_peek_context());
                let b = (input.read_byte())?;
                {
                    let ret = if (b == 255) {
                        let b = (input.read_byte())?;
                        match b {
                            224 => 0,

                            225 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        }
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    };
                    (input.close_peek_context())?;
                    ret
                }
            };
            match tree_index {
                0 => {
                    let inner = (Decoder70(input))?;
                    (Type55::app0(inner))
                }

                1 => {
                    let inner = (Decoder71(input))?;
                    (Type55::app1(inner))
                }

                _ => {
                    return (Err(ParseError::ExcludedBranch));
                }
            }
        })
    })())?;
    let segments = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = if (b == 255) {
                            let b = (input.read_byte())?;
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

                                _ => {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            }
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder72(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let header = ((|| PResult::Ok({ (Decoder73(input))? }))())?;
    let scan = ((|| PResult::Ok({ (Decoder74(input))? }))())?;
    let dnl = ((|| {
        PResult::Ok({
            let tree_index = {
                (input.open_peek_context());
                let b = (input.read_byte())?;
                {
                    let ret = if (b == 255) {
                        let b = (input.read_byte())?;
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

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        }
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    };
                    (input.close_peek_context())?;
                    ret
                }
            };
            match tree_index {
                0 => {
                    let inner = (Decoder75(input))?;
                    (Type78::some(inner))
                }

                1 => {
                    let _ = ();
                    Type78::none
                }

                _ => {
                    return (Err(ParseError::ExcludedBranch));
                }
            }
        })
    })())?;
    let scans = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = if (b == 255) {
                            let b = (input.read_byte())?;
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

                                _ => {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            }
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder76(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    (PResult::Ok(Type79 {
        initial_segment,
        segments,
        header,
        scan,
        dnl,
        scans,
    }))
}

fn Decoder69<'input>(input: &mut ParseMonad<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 217) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type41 { ff, marker }))
}

fn Decoder70<'input>(input: &mut ParseMonad<'input>) -> Result<Type45, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 224) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder136(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type45 {
        marker,
        length,
        data,
    }))
}

fn Decoder71<'input>(input: &mut ParseMonad<'input>) -> Result<Type54, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 225) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder130(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type54 {
        marker,
        length,
        data,
    }))
}

fn Decoder72<'input>(input: &mut ParseMonad<'input>) -> Result<Type65, ParseError> {
    let tree_index = {
        (input.open_peek_context());
        let b = (input.read_byte())?;
        {
            let ret = if (b == 255) {
                let b = (input.read_byte())?;
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

                    _ => {
                        return (Err(ParseError::ExcludedBranch));
                    }
                }
            } else {
                return (Err(ParseError::ExcludedBranch));
            };
            (input.close_peek_context())?;
            ret
        }
    };
    (PResult::Ok(match tree_index {
        0 => {
            let inner = (Decoder107(input))?;
            (Type65::dqt(inner))
        }

        1 => {
            let inner = (Decoder108(input))?;
            (Type65::dht(inner))
        }

        2 => {
            let inner = (Decoder109(input))?;
            (Type65::dac(inner))
        }

        3 => {
            let inner = (Decoder110(input))?;
            (Type65::dri(inner))
        }

        4 => {
            let inner = (Decoder70(input))?;
            (Type65::app0(inner))
        }

        5 => {
            let inner = (Decoder71(input))?;
            (Type65::app1(inner))
        }

        6 => {
            let inner = (Decoder111(input))?;
            (Type65::app2(inner))
        }

        7 => {
            let inner = (Decoder112(input))?;
            (Type65::app3(inner))
        }

        8 => {
            let inner = (Decoder113(input))?;
            (Type65::app4(inner))
        }

        9 => {
            let inner = (Decoder114(input))?;
            (Type65::app5(inner))
        }

        10 => {
            let inner = (Decoder115(input))?;
            (Type65::app6(inner))
        }

        11 => {
            let inner = (Decoder116(input))?;
            (Type65::app7(inner))
        }

        12 => {
            let inner = (Decoder117(input))?;
            (Type65::app8(inner))
        }

        13 => {
            let inner = (Decoder118(input))?;
            (Type65::app9(inner))
        }

        14 => {
            let inner = (Decoder119(input))?;
            (Type65::app10(inner))
        }

        15 => {
            let inner = (Decoder120(input))?;
            (Type65::app11(inner))
        }

        16 => {
            let inner = (Decoder121(input))?;
            (Type65::app12(inner))
        }

        17 => {
            let inner = (Decoder122(input))?;
            (Type65::app13(inner))
        }

        18 => {
            let inner = (Decoder123(input))?;
            (Type65::app14(inner))
        }

        19 => {
            let inner = (Decoder124(input))?;
            (Type65::app15(inner))
        }

        20 => {
            let inner = (Decoder125(input))?;
            (Type65::com(inner))
        }

        _ => {
            return (Err(ParseError::ExcludedBranch));
        }
    }))
}

fn Decoder73<'input>(input: &mut ParseMonad<'input>) -> Result<Type69, ParseError> {
    let tree_index = {
        (input.open_peek_context());
        let b = (input.read_byte())?;
        {
            let ret = if (b == 255) {
                let b = (input.read_byte())?;
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

                    _ => {
                        return (Err(ParseError::ExcludedBranch));
                    }
                }
            } else {
                return (Err(ParseError::ExcludedBranch));
            };
            (input.close_peek_context())?;
            ret
        }
    };
    (PResult::Ok(match tree_index {
        0 => {
            let inner = (Decoder92(input))?;
            (Type69::sof0(inner))
        }

        1 => {
            let inner = (Decoder93(input))?;
            (Type69::sof1(inner))
        }

        2 => {
            let inner = (Decoder94(input))?;
            (Type69::sof2(inner))
        }

        3 => {
            let inner = (Decoder95(input))?;
            (Type69::sof3(inner))
        }

        4 => {
            let inner = (Decoder96(input))?;
            (Type69::sof5(inner))
        }

        5 => {
            let inner = (Decoder97(input))?;
            (Type69::sof6(inner))
        }

        6 => {
            let inner = (Decoder98(input))?;
            (Type69::sof7(inner))
        }

        7 => {
            let inner = (Decoder99(input))?;
            (Type69::sof9(inner))
        }

        8 => {
            let inner = (Decoder100(input))?;
            (Type69::sof10(inner))
        }

        9 => {
            let inner = (Decoder101(input))?;
            (Type69::sof11(inner))
        }

        10 => {
            let inner = (Decoder102(input))?;
            (Type69::sof13(inner))
        }

        11 => {
            let inner = (Decoder103(input))?;
            (Type69::sof14(inner))
        }

        12 => {
            let inner = (Decoder104(input))?;
            (Type69::sof15(inner))
        }

        _ => {
            return (Err(ParseError::ExcludedBranch));
        }
    }))
}

fn Decoder74<'input>(input: &mut ParseMonad<'input>) -> Result<Type75, ParseError> {
    let segments = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = if (b == 255) {
                            let b = (input.read_byte())?;
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

                                _ => {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            }
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder72(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let sos = ((|| PResult::Ok({ (Decoder77(input))? }))())?;
    let data = ((|| PResult::Ok({ (Decoder91(input))? }))())?;
    (PResult::Ok(Type75 {
        segments,
        sos,
        data,
    }))
}

fn Decoder75<'input>(input: &mut ParseMonad<'input>) -> Result<Type77, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 220) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder90(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type77 {
        marker,
        length,
        data,
    }))
}

fn Decoder76<'input>(input: &mut ParseMonad<'input>) -> Result<Type75, ParseError> {
    let segments = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = if (b == 255) {
                            let b = (input.read_byte())?;
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

                                _ => {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            }
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder72(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let sos = ((|| PResult::Ok({ (Decoder77(input))? }))())?;
    let data = ((|| PResult::Ok({ (Decoder78(input))? }))())?;
    (PResult::Ok(Type75 {
        segments,
        sos,
        data,
    }))
}

fn Decoder77<'input>(input: &mut ParseMonad<'input>) -> Result<Type72, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 218) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder88(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type72 {
        marker,
        length,
        data,
    }))
}

fn Decoder78<'input>(input: &mut ParseMonad<'input>) -> Result<Type74, ParseError> {
    let scan_data = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 255) => 0,

                            255 => {
                                let b = (input.read_byte())?;
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

                                    _ => {
                                        return (Err(ParseError::ExcludedBranch));
                                    }
                                }
                            }

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let tree_index = {
                            (input.open_peek_context());
                            let b = (input.read_byte())?;
                            {
                                let ret = match b {
                                    tmp if (tmp != 255) => 0,

                                    255 => {
                                        let b = (input.read_byte())?;
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

                                            _ => {
                                                return (Err(ParseError::ExcludedBranch));
                                            }
                                        }
                                    }

                                    _ => {
                                        return (Err(ParseError::ExcludedBranch));
                                    }
                                };
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        match tree_index {
                            0 => {
                                let inner = (Decoder79(input))?;
                                (Type73::mcu(inner))
                            }

                            1 => {
                                let inner = (Decoder80(input))?;
                                (Type73::rst0(inner))
                            }

                            2 => {
                                let inner = (Decoder81(input))?;
                                (Type73::rst1(inner))
                            }

                            3 => {
                                let inner = (Decoder82(input))?;
                                (Type73::rst2(inner))
                            }

                            4 => {
                                let inner = (Decoder83(input))?;
                                (Type73::rst3(inner))
                            }

                            5 => {
                                let inner = (Decoder84(input))?;
                                (Type73::rst4(inner))
                            }

                            6 => {
                                let inner = (Decoder85(input))?;
                                (Type73::rst5(inner))
                            }

                            7 => {
                                let inner = (Decoder86(input))?;
                                (Type73::rst6(inner))
                            }

                            8 => {
                                let inner = (Decoder87(input))?;
                                (Type73::rst7(inner))
                            }

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let scan_data_stream = ((|| {
        PResult::Ok({
            (try_flat_map_vec(
                ((scan_data.iter()).cloned()),
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
                }),
            ))
        })
    })())?;
    (PResult::Ok(Type74 {
        scan_data,
        scan_data_stream,
    }))
}

fn Decoder79<'input>(input: &mut ParseMonad<'input>) -> Result<u8, ParseError> {
    let tree_index = {
        (input.open_peek_context());
        let b = (input.read_byte())?;
        {
            let ret = match b {
                tmp if (tmp != 255) => 0,

                255 => 1,

                _ => {
                    return (Err(ParseError::ExcludedBranch));
                }
            };
            (input.close_peek_context())?;
            ret
        }
    };
    (PResult::Ok(match tree_index {
        0 => {
            let b = (input.read_byte())?;
            if (b != 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        }

        1 => {
            let inner = {
                let field0 = ((|| {
                    PResult::Ok({
                        let b = (input.read_byte())?;
                        if (b == 255) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    })
                })())?;
                let field1 = ((|| {
                    PResult::Ok({
                        let b = (input.read_byte())?;
                        if (b == 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    })
                })())?;
                (field0, field1)
            };
            ((|_: (u8, u8)| PResult::Ok(255))(inner))?
        }

        _ => {
            return (Err(ParseError::ExcludedBranch));
        }
    }))
}

fn Decoder80<'input>(input: &mut ParseMonad<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 208) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type41 { ff, marker }))
}

fn Decoder81<'input>(input: &mut ParseMonad<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 209) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type41 { ff, marker }))
}

fn Decoder82<'input>(input: &mut ParseMonad<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 210) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type41 { ff, marker }))
}

fn Decoder83<'input>(input: &mut ParseMonad<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 211) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type41 { ff, marker }))
}

fn Decoder84<'input>(input: &mut ParseMonad<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 212) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type41 { ff, marker }))
}

fn Decoder85<'input>(input: &mut ParseMonad<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 213) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type41 { ff, marker }))
}

fn Decoder86<'input>(input: &mut ParseMonad<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 214) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type41 { ff, marker }))
}

fn Decoder87<'input>(input: &mut ParseMonad<'input>) -> Result<Type41, ParseError> {
    let ff = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let marker = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 215) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type41 { ff, marker }))
}

fn Decoder88<'input>(input: &mut ParseMonad<'input>) -> Result<Type71, ParseError> {
    let num_image_components = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let image_components = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..num_image_components {
                (accum.push((Decoder89(input))?));
            }
            accum
        })
    })())?;
    let start_spectral_selection = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let end_spectral_selection = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let approximation_bit_position = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    (PResult::Ok(Type71 {
        num_image_components,
        image_components,
        start_spectral_selection,
        end_spectral_selection,
        approximation_bit_position,
    }))
}

fn Decoder89<'input>(input: &mut ParseMonad<'input>) -> Result<Type70, ParseError> {
    let component_selector = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let entropy_coding_table_ids = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    (PResult::Ok(Type70 {
        component_selector,
        entropy_coding_table_ids,
    }))
}

fn Decoder90<'input>(input: &mut ParseMonad<'input>) -> Result<Type76, ParseError> {
    let num_lines = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    (PResult::Ok(Type76 { num_lines }))
}

fn Decoder91<'input>(input: &mut ParseMonad<'input>) -> Result<Type74, ParseError> {
    let scan_data = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 255) => 0,

                            255 => {
                                let b = (input.read_byte())?;
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

                                    _ => {
                                        return (Err(ParseError::ExcludedBranch));
                                    }
                                }
                            }

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let tree_index = {
                            (input.open_peek_context());
                            let b = (input.read_byte())?;
                            {
                                let ret = match b {
                                    tmp if (tmp != 255) => 0,

                                    255 => {
                                        let b = (input.read_byte())?;
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

                                            _ => {
                                                return (Err(ParseError::ExcludedBranch));
                                            }
                                        }
                                    }

                                    _ => {
                                        return (Err(ParseError::ExcludedBranch));
                                    }
                                };
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        match tree_index {
                            0 => {
                                let inner = (Decoder79(input))?;
                                (Type73::mcu(inner))
                            }

                            1 => {
                                let inner = (Decoder80(input))?;
                                (Type73::rst0(inner))
                            }

                            2 => {
                                let inner = (Decoder81(input))?;
                                (Type73::rst1(inner))
                            }

                            3 => {
                                let inner = (Decoder82(input))?;
                                (Type73::rst2(inner))
                            }

                            4 => {
                                let inner = (Decoder83(input))?;
                                (Type73::rst3(inner))
                            }

                            5 => {
                                let inner = (Decoder84(input))?;
                                (Type73::rst4(inner))
                            }

                            6 => {
                                let inner = (Decoder85(input))?;
                                (Type73::rst5(inner))
                            }

                            7 => {
                                let inner = (Decoder86(input))?;
                                (Type73::rst6(inner))
                            }

                            8 => {
                                let inner = (Decoder87(input))?;
                                (Type73::rst7(inner))
                            }

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let scan_data_stream = ((|| {
        PResult::Ok({
            (try_flat_map_vec(
                ((scan_data.iter()).cloned()),
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
                }),
            ))
        })
    })())?;
    (PResult::Ok(Type74 {
        scan_data,
        scan_data_stream,
    }))
}

fn Decoder92<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 192) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder93<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 193) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder94<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 194) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder95<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 195) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder96<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 197) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder97<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 198) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder98<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 199) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder99<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 201) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder100<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 202) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder101<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 203) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder102<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 205) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder103<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 206) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder104<'input>(input: &mut ParseMonad<'input>) -> Result<Type68, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 207) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder105(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type68 {
        marker,
        length,
        data,
    }))
}

fn Decoder105<'input>(input: &mut ParseMonad<'input>) -> Result<Type67, ParseError> {
    let sample_precision = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let num_lines = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let num_samples_per_line = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let num_image_components = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let image_components = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..num_image_components {
                (accum.push((Decoder106(input))?));
            }
            accum
        })
    })())?;
    (PResult::Ok(Type67 {
        sample_precision,
        num_lines,
        num_samples_per_line,
        num_image_components,
        image_components,
    }))
}

fn Decoder106<'input>(input: &mut ParseMonad<'input>) -> Result<Type66, ParseError> {
    let id = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let sampling_factor = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let quantization_table_id = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    (PResult::Ok(Type66 {
        id,
        sampling_factor,
        quantization_table_id,
    }))
}

fn Decoder107<'input>(input: &mut ParseMonad<'input>) -> Result<Type62, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 219) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder129(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type62 {
        marker,
        length,
        data,
    }))
}

fn Decoder108<'input>(input: &mut ParseMonad<'input>) -> Result<Type60, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 196) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder128(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type60 {
        marker,
        length,
        data,
    }))
}

fn Decoder109<'input>(input: &mut ParseMonad<'input>) -> Result<Type58, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 204) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder127(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type58 {
        marker,
        length,
        data,
    }))
}

fn Decoder110<'input>(input: &mut ParseMonad<'input>) -> Result<Type64, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 221) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| PResult::Ok({ (Decoder126(input))? }))())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type64 {
        marker,
        length,
        data,
    }))
}

fn Decoder111<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 226) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder112<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 227) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder113<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 228) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder114<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 229) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder115<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 230) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder116<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 231) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder117<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 232) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder118<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 233) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder119<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 234) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder120<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 235) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder121<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 236) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder122<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 237) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder123<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 238) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder124<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 239) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder125<'input>(input: &mut ParseMonad<'input>) -> Result<Type56, ParseError> {
    let marker = ((|| {
        PResult::Ok({
            let ff = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 255) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let marker = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 254) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            Type41 { ff, marker }
        })
    })())?;
    let length = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            let sz = ((length - 2) as usize);
            (input.start_slice(sz))?;
            let mut ret = ((|| {
                PResult::Ok({
                    let mut accum = (Vec::new());
                    while (input.remaining() > 0) {
                        let matching_ix = {
                            (input.open_peek_context());
                            {
                                let ret = 0;
                                (input.close_peek_context())?;
                                ret
                            }
                        };
                        if (matching_ix == 0) {
                            let next_elem = (Decoder17(input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                })
            })())?;
            (input.end_slice())?;
            ret
        })
    })())?;
    (PResult::Ok(Type56 {
        marker,
        length,
        data,
    }))
}

fn Decoder126<'input>(input: &mut ParseMonad<'input>) -> Result<Type63, ParseError> {
    let restart_interval = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    (PResult::Ok(Type63 { restart_interval }))
}

fn Decoder127<'input>(input: &mut ParseMonad<'input>) -> Result<Type57, ParseError> {
    let class_table_id = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let value = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    (PResult::Ok(Type57 {
        class_table_id,
        value,
    }))
}

fn Decoder128<'input>(input: &mut ParseMonad<'input>) -> Result<Type59, ParseError> {
    let class_table_id = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let num_codes = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..16 {
                (accum.push((Decoder17(input))?));
            }
            accum
        })
    })())?;
    let values = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    {
                        let ret = 0;
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder17(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    (PResult::Ok(Type59 {
        class_table_id,
        num_codes,
        values,
    }))
}

fn Decoder129<'input>(input: &mut ParseMonad<'input>) -> Result<Type61, ParseError> {
    let precision_table_id = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let elements = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    {
                        let ret = 0;
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder17(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    (PResult::Ok(Type61 {
        precision_table_id,
        elements,
    }))
}

fn Decoder130<'input>(input: &mut ParseMonad<'input>) -> Result<Type53, ParseError> {
    let identifier = ((|| PResult::Ok({ (Decoder131(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            match (identifier.string.as_slice()) {
                [69, 120, 105, 102] => {
                    let inner = (Decoder132(input))?;
                    (Type52::exif(inner))
                }

                [104, 116, 116, 112, 58, 47, 47, 110, 115, 46, 97, 100, 111, 98, 101, 46, 99, 111, 109, 47, 120, 97, 112, 47, 49, 46, 48, 47] =>
                {
                    let inner = (Decoder133(input))?;
                    (Type52::xmp(inner))
                }

                _ => {
                    let inner = {
                        let mut accum = (Vec::new());
                        while (input.remaining() > 0) {
                            let matching_ix = {
                                (input.open_peek_context());
                                {
                                    let ret = 0;
                                    (input.close_peek_context())?;
                                    ret
                                }
                            };
                            if (matching_ix == 0) {
                                let next_elem = (Decoder17(input))?;
                                (accum.push(next_elem));
                            } else {
                                break;
                            }
                        }
                        accum
                    };
                    (Type52::other(inner))
                }
            }
        })
    })())?;
    (PResult::Ok(Type53 { identifier, data }))
}

fn Decoder131<'input>(input: &mut ParseMonad<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type21 { string, null }))
}

fn Decoder132<'input>(input: &mut ParseMonad<'input>) -> Result<Type50, ParseError> {
    let padding = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let exif = ((|| PResult::Ok({ (Decoder134(input))? }))())?;
    (PResult::Ok(Type50 { padding, exif }))
}

fn Decoder133<'input>(input: &mut ParseMonad<'input>) -> Result<Type51, ParseError> {
    let xmp = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    {
                        let ret = 0;
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder17(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    (PResult::Ok(Type51 { xmp }))
}

fn Decoder134<'input>(input: &mut ParseMonad<'input>) -> Result<Type49, ParseError> {
    let byte_order = ((|| {
        PResult::Ok({
            let tree_index = {
                (input.open_peek_context());
                let b = (input.read_byte())?;
                {
                    let ret = match b {
                        73 => 0,

                        77 => 1,

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (input.close_peek_context())?;
                    ret
                }
            };
            match tree_index {
                0 => {
                    let field0 = ((|| {
                        PResult::Ok({
                            let b = (input.read_byte())?;
                            if (b == 73) {
                                b
                            } else {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        })
                    })())?;
                    let field1 = ((|| {
                        PResult::Ok({
                            let b = (input.read_byte())?;
                            if (b == 73) {
                                b
                            } else {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        })
                    })())?;
                    (Type46::le(field0, field1))
                }

                1 => {
                    let field0 = ((|| {
                        PResult::Ok({
                            let b = (input.read_byte())?;
                            if (b == 77) {
                                b
                            } else {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        })
                    })())?;
                    let field1 = ((|| {
                        PResult::Ok({
                            let b = (input.read_byte())?;
                            if (b == 77) {
                                b
                            } else {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        })
                    })())?;
                    (Type46::be(field0, field1))
                }

                _ => {
                    return (Err(ParseError::ExcludedBranch));
                }
            }
        })
    })())?;
    let magic = ((|| {
        PResult::Ok({
            match byte_order {
                Type46::le(..) => (Decoder135(input))?,

                Type46::be(..) => (Decoder43(input))?,
            }
        })
    })())?;
    let offset = ((|| {
        PResult::Ok({
            match byte_order {
                Type46::le(..) => (Decoder24(input))?,

                Type46::be(..) => (Decoder33(input))?,
            }
        })
    })())?;
    let ifd =
        ((|| PResult::Ok({ (unimplemented!(r#"translate @ Decoder::WithRelativeOffset"#)) }))())?;
    (PResult::Ok(Type49 {
        byte_order,
        magic,
        offset,
        ifd,
    }))
}

fn Decoder135<'input>(input: &mut ParseMonad<'input>) -> Result<u16, ParseError> {
    let inner = {
        let field0 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        let field1 = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
        (field0, field1)
    };
    (PResult::Ok(((|x: (u8, u8)| PResult::Ok((u16le(x))))(inner))?))
}

fn Decoder136<'input>(input: &mut ParseMonad<'input>) -> Result<Type44, ParseError> {
    let identifier = ((|| PResult::Ok({ (Decoder137(input))? }))())?;
    let data = ((|| {
        PResult::Ok({
            match (identifier.string.as_slice()) {
                [74, 70, 73, 70] => {
                    let inner = (Decoder138(input))?;
                    (Type43::jfif(inner))
                }

                _ => {
                    let inner = {
                        let mut accum = (Vec::new());
                        while (input.remaining() > 0) {
                            let matching_ix = {
                                (input.open_peek_context());
                                {
                                    let ret = 0;
                                    (input.close_peek_context())?;
                                    ret
                                }
                            };
                            if (matching_ix == 0) {
                                let next_elem = (Decoder17(input))?;
                                (accum.push(next_elem));
                            } else {
                                break;
                            }
                        }
                        accum
                    };
                    (Type43::other(inner))
                }
            }
        })
    })())?;
    (PResult::Ok(Type44 { identifier, data }))
}

fn Decoder137<'input>(input: &mut ParseMonad<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type21 { string, null }))
}

fn Decoder138<'input>(input: &mut ParseMonad<'input>) -> Result<Type42, ParseError> {
    let version_major = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let version_minor = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let density_units = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let density_x = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let density_y = ((|| PResult::Ok({ (Decoder43(input))? }))())?;
    let thumbnail_width = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let thumbnail_height = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let thumbnail_pixels = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..thumbnail_height {
                (accum.push({
                    let mut accum = (Vec::new());
                    for _ in 0..thumbnail_width {
                        (accum.push((Decoder139(input))?));
                    }
                    accum
                }));
            }
            accum
        })
    })())?;
    (PResult::Ok(Type42 {
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

fn Decoder139<'input>(input: &mut ParseMonad<'input>) -> Result<Type2, ParseError> {
    let r = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let g = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let b = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    (PResult::Ok(Type2 { r, g, b }))
}

fn Decoder140<'input>(input: &mut ParseMonad<'input>) -> Result<Type20, ParseError> {
    let magic = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 31) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 139) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            (field0, field1)
        })
    })())?;
    let method = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let file_flags = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let timestamp = ((|| PResult::Ok({ (Decoder24(input))? }))())?;
    let compression_flags = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let os_id = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    (PResult::Ok(Type20 {
        magic,
        method,
        file_flags,
        timestamp,
        compression_flags,
        os_id,
    }))
}

fn Decoder141<'input>(input: &mut ParseMonad<'input>) -> Result<Type21, ParseError> {
    (PResult::Ok((Decoder151(input))?))
}

fn Decoder142<'input>(input: &mut ParseMonad<'input>) -> Result<Type38, ParseError> {
    let blocks = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            loop {
                let elem = (Decoder144(input))?;
                if ((|x: &Type37| x.r#final == 1)(&elem)) {
                    (accum.push(elem));
                    break;
                } else {
                    (accum.push(elem));
                }
            }
            accum
        })
    })())?;
    let codes = ((|| {
        PResult::Ok({
            (try_flat_map_vec(
                ((blocks.iter()).cloned()),
                (|x: Type37| match x.data {
                    Type36::uncompressed(y) => y.codes_values,

                    Type36::fixed_huffman(y) => y.codes_values,

                    Type36::dynamic_huffman(y) => y.codes_values,
                }),
            ))
        })
    })())?;
    let inflate = ((|| {
        PResult::Ok({ (unimplemented!(r#"embed_expr is not implemented for Expr::Inflate"#)) })
    })())?;
    (PResult::Ok(Type38 {
        blocks,
        codes,
        inflate,
    }))
}

fn Decoder143<'input>(input: &mut ParseMonad<'input>) -> Result<Type39, ParseError> {
    let crc = ((|| PResult::Ok({ (Decoder24(input))? }))())?;
    let length = ((|| PResult::Ok({ (Decoder24(input))? }))())?;
    (PResult::Ok(Type39 { crc, length }))
}

fn Decoder144<'input>(input: &mut ParseMonad<'input>) -> Result<Type37, ParseError> {
    let r#final = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
    let r#type = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                (field0, field1)
            };
            ((|bits: (u8, u8)| PResult::Ok((bits.1 << 1 | bits.0)))(inner))?
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            match r#type {
                0 => {
                    let inner = (Decoder146(input))?;
                    (Type36::uncompressed(inner))
                }

                1 => {
                    let inner = (Decoder147(input))?;
                    (Type36::fixed_huffman(inner))
                }

                2 => {
                    let inner = (Decoder148(input))?;
                    (Type36::dynamic_huffman(inner))
                }

                _other => {
                    (unreachable!(
                        r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                    ));
                }
            }
        })
    })())?;
    (PResult::Ok(Type37 {
        r#final,
        r#type,
        data,
    }))
}

fn Decoder145<'input>(input: &mut ParseMonad<'input>) -> Result<u8, ParseError> {
    let b = (input.read_byte())?;
    (PResult::Ok(b))
}

fn Decoder146<'input>(input: &mut ParseMonad<'input>) -> Result<Type35, ParseError> {
    let align = ((|| {
        PResult::Ok({
            (input.skip_align(8))?;
            ()
        })
    })())?;
    let len = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field6 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field7 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field8 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field9 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field10 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field11 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field12 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field13 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field14 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field15 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
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
                    ((bits.15 as u16) << 15
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
                        | (bits.0 as u16)),
                )
            })(inner))?
        })
    })())?;
    let nlen = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field6 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field7 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field8 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field9 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field10 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field11 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field12 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field13 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field14 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field15 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
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
                    ((bits.15 as u16) << 15
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
                        | (bits.0 as u16)),
                )
            })(inner))?
        })
    })())?;
    let bytes = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..len {
                (accum.push({
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field6 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field7 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (
                            field0, field1, field2, field3, field4, field5, field6, field7,
                        )
                    };
                    ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            (bits.7 << 7
                                | bits.6 << 6
                                | bits.5 << 5
                                | bits.4 << 4
                                | bits.3 << 3
                                | bits.2 << 2
                                | bits.1 << 1
                                | bits.0),
                        )
                    })(inner))?
                }));
            }
            accum
        })
    })())?;
    let codes_values = ((|| {
        PResult::Ok({
            (try_flat_map_vec(
                ((bytes.iter()).cloned()),
                (|x: u8| [(Type29::literal(x))].to_vec()),
            ))
        })
    })())?;
    (PResult::Ok(Type35 {
        align,
        len,
        nlen,
        bytes,
        codes_values,
    }))
}

fn Decoder147<'input>(input: &mut ParseMonad<'input>) -> Result<Type34, ParseError> {
    let codes = ((|| {
        PResult::Ok({
            let format = (unimplemented!(
                r#"no implementation for for DynamicLogic::Huffman AST-transcription"#
            ));
            let mut accum = (Vec::new());
            loop {
                let elem = {
                    let code = ((|| PResult::Ok({ (format(input))? }))())?;
                    let extra = ((|| {
                        PResult::Ok({
                            match code {
                                257 => {
                                    let inner = {
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (3 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (4 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (5 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (6 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (7 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (8 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (9 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (10 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0,)
                                                };
                                                ((|bits: (u8,)| PResult::Ok(bits.0))(inner))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (11 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0,)
                                                };
                                                ((|bits: (u8,)| PResult::Ok(bits.0))(inner))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (13 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0,)
                                                };
                                                ((|bits: (u8,)| PResult::Ok(bits.0))(inner))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (15 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0,)
                                                };
                                                ((|bits: (u8,)| PResult::Ok(bits.0))(inner))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (17 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1)
                                                };
                                                ((|bits: (u8, u8)| {
                                                    PResult::Ok((bits.1 << 1 | bits.0))
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (19 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1)
                                                };
                                                ((|bits: (u8, u8)| {
                                                    PResult::Ok((bits.1 << 1 | bits.0))
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (23 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1)
                                                };
                                                ((|bits: (u8, u8)| {
                                                    PResult::Ok((bits.1 << 1 | bits.0))
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (27 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1)
                                                };
                                                ((|bits: (u8, u8)| {
                                                    PResult::Ok((bits.1 << 1 | bits.0))
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (31 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2)
                                                };
                                                ((|bits: (u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.2 << 2 | bits.1 << 1 | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (35 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2)
                                                };
                                                ((|bits: (u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.2 << 2 | bits.1 << 1 | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (43 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2)
                                                };
                                                ((|bits: (u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.2 << 2 | bits.1 << 1 | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (51 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2)
                                                };
                                                ((|bits: (u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.2 << 2 | bits.1 << 1 | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (59 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3)
                                                };
                                                ((|bits: (u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (67 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3)
                                                };
                                                ((|bits: (u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (83 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3)
                                                };
                                                ((|bits: (u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (99 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3)
                                                };
                                                ((|bits: (u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (115 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (131 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (163 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (195 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (227 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (258 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({
                                                (Decoder149(input, (distance_code as u16)))?
                                            })
                                        })(
                                        ))?;
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
                            }
                        })
                    })())?;
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
        })
    })())?;
    let codes_values = ((|| {
        PResult::Ok({
            (try_flat_map_vec(
                ((codes.iter()).cloned()),
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    },

                    _ => ([(Type29::literal((x.code as u8)))].to_vec()),
                }),
            ))
        })
    })())?;
    (PResult::Ok(Type34 {
        codes,
        codes_values,
    }))
}

fn Decoder148<'input>(input: &mut ParseMonad<'input>) -> Result<Type30, ParseError> {
    let hlit = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                (field0, field1, field2, field3, field4)
            };
            ((|bits: (u8, u8, u8, u8, u8)| {
                PResult::Ok((bits.4 << 4 | bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0))
            })(inner))?
        })
    })())?;
    let hdist = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                (field0, field1, field2, field3, field4)
            };
            ((|bits: (u8, u8, u8, u8, u8)| {
                PResult::Ok((bits.4 << 4 | bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0))
            })(inner))?
        })
    })())?;
    let hclen = ((|| {
        PResult::Ok({
            let inner = {
                let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                (field0, field1, field2, field3)
            };
            ((|bits: (u8, u8, u8, u8)| {
                PResult::Ok((bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0))
            })(inner))?
        })
    })())?;
    let code_length_alphabet_code_lengths = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..(hclen + 4) {
                (accum.push({
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (field0, field1, field2)
                    };
                    ((|bits: (u8, u8, u8)| PResult::Ok((bits.2 << 2 | bits.1 << 1 | bits.0)))(
                        inner,
                    ))?
                }));
            }
            accum
        })
    })())?;
    let literal_length_distance_alphabet_code_lengths = ((|| {
        PResult::Ok({
            let code_length_alphabet_format = (unimplemented!(
                r#"no implementation for for DynamicLogic::Huffman AST-transcription"#
            ));
            let mut accum = (Vec::new());
            loop {
                let elem = {
                    let code = ((|| PResult::Ok({ (code_length_alphabet_format(input))? }))())?;
                    let extra = ((|| {
                        PResult::Ok({
                            match (code as u8) {
                                16 => {
                                    let inner = {
                                        let field0 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        let field1 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        (field0, field1)
                                    };
                                    ((|bits: (u8, u8)| PResult::Ok((bits.1 << 1 | bits.0)))(inner))?
                                }

                                17 => {
                                    let inner = {
                                        let field0 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        let field1 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        let field2 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        (field0, field1, field2)
                                    };
                                    ((|bits: (u8, u8, u8)| {
                                        PResult::Ok((bits.2 << 2 | bits.1 << 1 | bits.0))
                                    })(inner))?
                                }

                                18 => {
                                    let inner = {
                                        let field0 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        let field1 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        let field2 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        let field3 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        let field4 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        let field5 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        let field6 =
                                            ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                                        (field0, field1, field2, field3, field4, field5, field6)
                                    };
                                    ((|bits: (u8, u8, u8, u8, u8, u8, u8)| {
                                        PResult::Ok(
                                            (bits.6 << 6
                                                | bits.5 << 5
                                                | bits.4 << 4
                                                | bits.3 << 3
                                                | bits.2 << 2
                                                | bits.1 << 1
                                                | bits.0),
                                        )
                                    })(inner))?
                                }

                                _ => 0,
                            }
                        })
                    })())?;
                    Type23 { code, extra }
                };
                (accum.push(elem));
                if ((|y: &Vec<Type23>| {
                    ((((y.iter()).cloned()).try_fold(
                        {
                            ();
                            Type194::none
                        },
                        (|x: (Type194, Type23)| {
                            PResult::Ok(match (x.1.code as u8) {
                                16 => (
                                    x.0,
                                    (dup32(
                                        ((x.1.extra + 3) as u32),
                                        match x.0 {
                                            Type194::some(y) => y,

                                            _ => {
                                                return (Err(ParseError::ExcludedBranch));
                                            }
                                        },
                                    )),
                                ),

                                17 => (x.0, (dup32(((x.1.extra + 3) as u32), 0))),

                                18 => (x.0, (dup32(((x.1.extra + 11) as u32), 0))),

                                v => ((Type194::some(v)), ([v].to_vec())),
                            })
                        }),
                    ))
                    .collect())
                    .len()
                        >= ((hlit + hdist) as u32) + 258
                })(&accum))
                {
                    break;
                }
            }
            accum
        })
    })())?;
    let literal_length_distance_alphabet_code_lengths_value = ((|| {
        PResult::Ok({
            ((((literal_length_distance_alphabet_code_lengths.iter()).cloned()).try_fold(
                {
                    ();
                    Type194::none
                },
                (|x: (Type194, Type23)| {
                    PResult::Ok(match (x.1.code as u8) {
                        16 => (
                            x.0,
                            (dup32(
                                ((x.1.extra + 3) as u32),
                                match x.0 {
                                    Type194::some(y) => y,

                                    _ => {
                                        return (Err(ParseError::ExcludedBranch));
                                    }
                                },
                            )),
                        ),

                        17 => (x.0, (dup32(((x.1.extra + 3) as u32), 0))),

                        18 => (x.0, (dup32(((x.1.extra + 11) as u32), 0))),

                        v => ((Type194::some(v)), ([v].to_vec())),
                    })
                }),
            ))
            .collect())
        })
    })())?;
    let literal_length_alphabet_code_lengths_value = ((|| {
        PResult::Ok({
            {
                let ix = 0;
                literal_length_distance_alphabet_code_lengths_value[ix..(ix + (hlit as u32) + 257)]
            }
        })
    })())?;
    let distance_alphabet_code_lengths_value = ((|| {
        PResult::Ok({
            {
                let ix = ((hlit as u32) + 257);
                literal_length_distance_alphabet_code_lengths_value[ix..(ix + (hdist as u32) + 1)]
            }
        })
    })())?;
    let codes = ((|| {
        PResult::Ok({
            let distance_alphabet_format = (unimplemented!(
                r#"no implementation for for DynamicLogic::Huffman AST-transcription"#
            ));
            let literal_length_alphabet_format = (unimplemented!(
                r#"no implementation for for DynamicLogic::Huffman AST-transcription"#
            ));
            let mut accum = (Vec::new());
            loop {
                let elem = {
                    let code = ((|| PResult::Ok({ (literal_length_alphabet_format(input))? }))())?;
                    let extra = ((|| {
                        PResult::Ok({
                            match code {
                                257 => {
                                    let inner = {
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (3 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (4 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (5 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (6 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (7 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (8 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (9 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (10 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0,)
                                                };
                                                ((|bits: (u8,)| PResult::Ok(bits.0))(inner))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (11 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0,)
                                                };
                                                ((|bits: (u8,)| PResult::Ok(bits.0))(inner))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (13 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0,)
                                                };
                                                ((|bits: (u8,)| PResult::Ok(bits.0))(inner))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (15 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0,)
                                                };
                                                ((|bits: (u8,)| PResult::Ok(bits.0))(inner))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (17 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1)
                                                };
                                                ((|bits: (u8, u8)| {
                                                    PResult::Ok((bits.1 << 1 | bits.0))
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (19 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1)
                                                };
                                                ((|bits: (u8, u8)| {
                                                    PResult::Ok((bits.1 << 1 | bits.0))
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (23 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1)
                                                };
                                                ((|bits: (u8, u8)| {
                                                    PResult::Ok((bits.1 << 1 | bits.0))
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (27 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1)
                                                };
                                                ((|bits: (u8, u8)| {
                                                    PResult::Ok((bits.1 << 1 | bits.0))
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (31 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2)
                                                };
                                                ((|bits: (u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.2 << 2 | bits.1 << 1 | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (35 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2)
                                                };
                                                ((|bits: (u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.2 << 2 | bits.1 << 1 | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (43 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2)
                                                };
                                                ((|bits: (u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.2 << 2 | bits.1 << 1 | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (51 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2)
                                                };
                                                ((|bits: (u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.2 << 2 | bits.1 << 1 | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (59 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3)
                                                };
                                                ((|bits: (u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (67 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3)
                                                };
                                                ((|bits: (u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (83 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3)
                                                };
                                                ((|bits: (u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (99 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3)
                                                };
                                                ((|bits: (u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (115 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (131 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (163 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (195 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| {
                                            PResult::Ok({
                                                let inner = {
                                                    let field0 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field1 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field2 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field3 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    let field4 = ((|| {
                                                        PResult::Ok({ (Decoder145(input))? })
                                                    })(
                                                    ))?;
                                                    (field0, field1, field2, field3, field4)
                                                };
                                                ((|bits: (u8, u8, u8, u8, u8)| {
                                                    PResult::Ok(
                                                        (bits.4 << 4
                                                            | bits.3 << 3
                                                            | bits.2 << 2
                                                            | bits.1 << 1
                                                            | bits.0),
                                                    )
                                                })(
                                                    inner
                                                ))?
                                            })
                                        })(
                                        ))?;
                                        let length = ((|| {
                                            PResult::Ok({ (227 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                                        let length_extra_bits = ((|| PResult::Ok({ 0 }))())?;
                                        let length = ((|| {
                                            PResult::Ok({ (258 + (length_extra_bits as u16)) })
                                        })(
                                        ))?;
                                        let distance_code = ((|| {
                                            PResult::Ok({ (distance_alphabet_format(input))? })
                                        })(
                                        ))?;
                                        let distance_record = ((|| {
                                            PResult::Ok({ (Decoder149(input, distance_code))? })
                                        })(
                                        ))?;
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
                            }
                        })
                    })())?;
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
        })
    })())?;
    let codes_values = ((|| {
        PResult::Ok({
            (try_flat_map_vec(
                ((codes.iter()).cloned()),
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
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

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    },

                    _ => ([(Type29::literal((x.code as u8)))].to_vec()),
                }),
            ))
        })
    })())?;
    (PResult::Ok(Type30 {
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

fn Decoder149<'input>(
    input: &mut ParseMonad<'input>,
    distance_code: u16,
) -> Result<Type24, ParseError> {
    (PResult::Ok(match (distance_code as u8) {
        0 => (Decoder150(input, 0, 1))?,

        1 => (Decoder150(input, 0, 2))?,

        2 => (Decoder150(input, 0, 3))?,

        3 => (Decoder150(input, 0, 4))?,

        4 => (Decoder150(input, 1, 5))?,

        5 => (Decoder150(input, 1, 7))?,

        6 => (Decoder150(input, 2, 9))?,

        7 => (Decoder150(input, 2, 13))?,

        8 => (Decoder150(input, 3, 17))?,

        9 => (Decoder150(input, 3, 25))?,

        10 => (Decoder150(input, 4, 33))?,

        11 => (Decoder150(input, 4, 49))?,

        12 => (Decoder150(input, 5, 65))?,

        13 => (Decoder150(input, 5, 97))?,

        14 => (Decoder150(input, 6, 129))?,

        15 => (Decoder150(input, 6, 193))?,

        16 => (Decoder150(input, 7, 257))?,

        17 => (Decoder150(input, 7, 385))?,

        18 => (Decoder150(input, 8, 513))?,

        19 => (Decoder150(input, 8, 769))?,

        20 => (Decoder150(input, 9, 1025))?,

        21 => (Decoder150(input, 9, 1537))?,

        22 => (Decoder150(input, 10, 2049))?,

        23 => (Decoder150(input, 10, 3073))?,

        24 => (Decoder150(input, 11, 4097))?,

        25 => (Decoder150(input, 11, 6145))?,

        26 => (Decoder150(input, 12, 8193))?,

        27 => (Decoder150(input, 12, 12289))?,

        28 => (Decoder150(input, 13, 16385))?,

        29 => (Decoder150(input, 13, 24577))?,

        _other => {
            (unreachable!(r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#));
        }
    }))
}

fn Decoder150<'input>(
    input: &mut ParseMonad<'input>,
    extra_bits: u8,
    start: u16,
) -> Result<Type24, ParseError> {
    let distance_extra_bits = ((|| {
        PResult::Ok({
            match extra_bits {
                0 => 0,

                1 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (field0,)
                    };
                    ((|bits: (u8,)| PResult::Ok((bits.0 as u16)))(inner))?
                }

                2 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (field0, field1)
                    };
                    ((|bits: (u8, u8)| PResult::Ok(((bits.1 as u16) << 1 | (bits.0 as u16))))(
                        inner,
                    ))?
                }

                3 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (field0, field1, field2)
                    };
                    ((|bits: (u8, u8, u8)| {
                        PResult::Ok(((bits.2 as u16) << 2 | (bits.1 as u16) << 1 | (bits.0 as u16)))
                    })(inner))?
                }

                4 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (field0, field1, field2, field3)
                    };
                    ((|bits: (u8, u8, u8, u8)| {
                        PResult::Ok(
                            ((bits.3 as u16) << 3
                                | (bits.2 as u16) << 2
                                | (bits.1 as u16) << 1
                                | (bits.0 as u16)),
                        )
                    })(inner))?
                }

                5 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (field0, field1, field2, field3, field4)
                    };
                    ((|bits: (u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            ((bits.4 as u16) << 4
                                | (bits.3 as u16) << 3
                                | (bits.2 as u16) << 2
                                | (bits.1 as u16) << 1
                                | (bits.0 as u16)),
                        )
                    })(inner))?
                }

                6 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (field0, field1, field2, field3, field4, field5)
                    };
                    ((|bits: (u8, u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            ((bits.5 as u16) << 5
                                | (bits.4 as u16) << 4
                                | (bits.3 as u16) << 3
                                | (bits.2 as u16) << 2
                                | (bits.1 as u16) << 1
                                | (bits.0 as u16)),
                        )
                    })(inner))?
                }

                7 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field6 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (field0, field1, field2, field3, field4, field5, field6)
                    };
                    ((|bits: (u8, u8, u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            ((bits.6 as u16) << 6
                                | (bits.5 as u16) << 5
                                | (bits.4 as u16) << 4
                                | (bits.3 as u16) << 3
                                | (bits.2 as u16) << 2
                                | (bits.1 as u16) << 1
                                | (bits.0 as u16)),
                        )
                    })(inner))?
                }

                8 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field6 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field7 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (
                            field0, field1, field2, field3, field4, field5, field6, field7,
                        )
                    };
                    ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            ((bits.7 as u16) << 7
                                | (bits.6 as u16) << 6
                                | (bits.5 as u16) << 5
                                | (bits.4 as u16) << 4
                                | (bits.3 as u16) << 3
                                | (bits.2 as u16) << 2
                                | (bits.1 as u16) << 1
                                | (bits.0 as u16)),
                        )
                    })(inner))?
                }

                9 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field6 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field7 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field8 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (
                            field0, field1, field2, field3, field4, field5, field6, field7, field8,
                        )
                    };
                    ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            ((bits.8 as u16) << 8
                                | (bits.7 as u16) << 7
                                | (bits.6 as u16) << 6
                                | (bits.5 as u16) << 5
                                | (bits.4 as u16) << 4
                                | (bits.3 as u16) << 3
                                | (bits.2 as u16) << 2
                                | (bits.1 as u16) << 1
                                | (bits.0 as u16)),
                        )
                    })(inner))?
                }

                10 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field6 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field7 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field8 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field9 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (
                            field0, field1, field2, field3, field4, field5, field6, field7, field8,
                            field9,
                        )
                    };
                    ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            ((bits.9 as u16) << 9
                                | (bits.8 as u16) << 8
                                | (bits.7 as u16) << 7
                                | (bits.6 as u16) << 6
                                | (bits.5 as u16) << 5
                                | (bits.4 as u16) << 4
                                | (bits.3 as u16) << 3
                                | (bits.2 as u16) << 2
                                | (bits.1 as u16) << 1
                                | (bits.0 as u16)),
                        )
                    })(inner))?
                }

                11 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field6 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field7 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field8 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field9 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field10 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (
                            field0, field1, field2, field3, field4, field5, field6, field7, field8,
                            field9, field10,
                        )
                    };
                    ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            ((bits.10 as u16) << 10
                                | (bits.9 as u16) << 9
                                | (bits.8 as u16) << 8
                                | (bits.7 as u16) << 7
                                | (bits.6 as u16) << 6
                                | (bits.5 as u16) << 5
                                | (bits.4 as u16) << 4
                                | (bits.3 as u16) << 3
                                | (bits.2 as u16) << 2
                                | (bits.1 as u16) << 1
                                | (bits.0 as u16)),
                        )
                    })(inner))?
                }

                12 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field6 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field7 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field8 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field9 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field10 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field11 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (
                            field0, field1, field2, field3, field4, field5, field6, field7, field8,
                            field9, field10, field11,
                        )
                    };
                    ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            ((bits.11 as u16) << 11
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
                                | (bits.0 as u16)),
                        )
                    })(inner))?
                }

                13 => {
                    let inner = {
                        let field0 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field1 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field2 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field3 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field4 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field5 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field6 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field7 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field8 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field9 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field10 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field11 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        let field12 = ((|| PResult::Ok({ (Decoder145(input))? }))())?;
                        (
                            field0, field1, field2, field3, field4, field5, field6, field7, field8,
                            field9, field10, field11, field12,
                        )
                    };
                    ((|bits: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)| {
                        PResult::Ok(
                            ((bits.12 as u16) << 12
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
                                | (bits.0 as u16)),
                        )
                    })(inner))?
                }

                _other => {
                    (unreachable!(
                        r#"ExprMatch refuted: match refuted with unexpected value {_other:?}"#
                    ));
                }
            }
        })
    })())?;
    let distance = ((|| PResult::Ok({ (start + distance_extra_bits) }))())?;
    (PResult::Ok(Type24 {
        distance_extra_bits,
        distance,
    }))
}

fn Decoder151<'input>(input: &mut ParseMonad<'input>) -> Result<Type21, ParseError> {
    let string = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = {
                        let b = (input.read_byte())?;
                        if (b != 0) {
                            b
                        } else {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let null = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type21 { string, null }))
}

fn Decoder152<'input>(input: &mut ParseMonad<'input>) -> Result<Type0, ParseError> {
    let signature = ((|| {
        PResult::Ok({
            let field0 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 71) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field1 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 73) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            let field2 = ((|| {
                PResult::Ok({
                    let b = (input.read_byte())?;
                    if (b == 70) {
                        b
                    } else {
                        return (Err(ParseError::ExcludedBranch));
                    }
                })
            })())?;
            (field0, field1, field2)
        })
    })())?;
    let version = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..3 {
                (accum.push((Decoder21(input))?));
            }
            accum
        })
    })())?;
    (PResult::Ok(Type0 { signature, version }))
}

fn Decoder153<'input>(input: &mut ParseMonad<'input>) -> Result<Type4, ParseError> {
    let descriptor = ((|| PResult::Ok({ (Decoder169(input))? }))())?;
    let global_color_table = ((|| {
        PResult::Ok({
            match (descriptor.flags & 128 != 0) {
                true => {
                    let inner = {
                        let mut accum = (Vec::new());
                        for _ in 0..(2 << (descriptor.flags & 7)) {
                            (accum.push((Decoder167(input))?));
                        }
                        accum
                    };
                    (Type3::yes(inner))
                }

                false => {
                    let _ = ();
                    Type3::no
                }
            }
        })
    })())?;
    (PResult::Ok(Type4 {
        descriptor,
        global_color_table,
    }))
}

fn Decoder154<'input>(input: &mut ParseMonad<'input>) -> Result<Type17, ParseError> {
    let tree_index = {
        (input.open_peek_context());
        let b = (input.read_byte())?;
        {
            let ret = match b {
                33 => {
                    let b = (input.read_byte())?;
                    match b {
                        249 => 0,

                        1 => 0,

                        255 => 1,

                        254 => 1,

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    }
                }

                44 => 0,

                _ => {
                    return (Err(ParseError::ExcludedBranch));
                }
            };
            (input.close_peek_context())?;
            ret
        }
    };
    (PResult::Ok(match tree_index {
        0 => {
            let inner = (Decoder156(input))?;
            (Type17::graphic_block(inner))
        }

        1 => {
            let inner = (Decoder157(input))?;
            (Type17::special_purpose_block(inner))
        }

        _ => {
            return (Err(ParseError::ExcludedBranch));
        }
    }))
}

fn Decoder155<'input>(input: &mut ParseMonad<'input>) -> Result<Type18, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 59) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    (PResult::Ok(Type18 { separator }))
}

fn Decoder156<'input>(input: &mut ParseMonad<'input>) -> Result<Type13, ParseError> {
    let graphic_control_extension = ((|| {
        PResult::Ok({
            let tree_index = {
                (input.open_peek_context());
                let b = (input.read_byte())?;
                {
                    let ret = match b {
                        33 => {
                            let b = (input.read_byte())?;
                            match b {
                                249 => 0,

                                1 => 1,

                                _ => {
                                    return (Err(ParseError::ExcludedBranch));
                                }
                            }
                        }

                        44 => 1,

                        _ => {
                            return (Err(ParseError::ExcludedBranch));
                        }
                    };
                    (input.close_peek_context())?;
                    ret
                }
            };
            match tree_index {
                0 => {
                    let inner = (Decoder162(input))?;
                    (Type6::some(inner))
                }

                1 => {
                    let _ = ();
                    Type6::none
                }

                _ => {
                    return (Err(ParseError::ExcludedBranch));
                }
            }
        })
    })())?;
    let graphic_rendering_block = ((|| PResult::Ok({ (Decoder163(input))? }))())?;
    (PResult::Ok(Type13 {
        graphic_control_extension,
        graphic_rendering_block,
    }))
}

fn Decoder157<'input>(input: &mut ParseMonad<'input>) -> Result<Type16, ParseError> {
    let tree_index = {
        (input.open_peek_context());
        let b = (input.read_byte())?;
        {
            let ret = if (b == 33) {
                let b = (input.read_byte())?;
                match b {
                    255 => 0,

                    254 => 1,

                    _ => {
                        return (Err(ParseError::ExcludedBranch));
                    }
                }
            } else {
                return (Err(ParseError::ExcludedBranch));
            };
            (input.close_peek_context())?;
            ret
        }
    };
    (PResult::Ok(match tree_index {
        0 => {
            let inner = (Decoder158(input))?;
            (Type16::application_extension(inner))
        }

        1 => {
            let inner = (Decoder159(input))?;
            (Type16::comment_extension(inner))
        }

        _ => {
            return (Err(ParseError::ExcludedBranch));
        }
    }))
}

fn Decoder158<'input>(input: &mut ParseMonad<'input>) -> Result<Type14, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 33) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let label = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 255) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let block_size = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 11) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let identifier = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..8 {
                (accum.push((Decoder17(input))?));
            }
            accum
        })
    })())?;
    let authentication_code = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..3 {
                (accum.push((Decoder17(input))?));
            }
            accum
        })
    })())?;
    let application_data = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder160(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let terminator = ((|| PResult::Ok({ (Decoder161(input))? }))())?;
    (PResult::Ok(Type14 {
        separator,
        label,
        block_size,
        identifier,
        authentication_code,
        application_data,
        terminator,
    }))
}

fn Decoder159<'input>(input: &mut ParseMonad<'input>) -> Result<Type15, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 33) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let label = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 254) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let comment_data = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder160(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let terminator = ((|| PResult::Ok({ (Decoder161(input))? }))())?;
    (PResult::Ok(Type15 {
        separator,
        label,
        comment_data,
        terminator,
    }))
}

fn Decoder160<'input>(input: &mut ParseMonad<'input>) -> Result<Type7, ParseError> {
    let len_bytes = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b != 0) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let data = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            for _ in 0..len_bytes {
                (accum.push((Decoder17(input))?));
            }
            accum
        })
    })())?;
    (PResult::Ok(Type7 { len_bytes, data }))
}

fn Decoder161<'input>(input: &mut ParseMonad<'input>) -> Result<u8, ParseError> {
    let b = (input.read_byte())?;
    (PResult::Ok(if (b == 0) {
        b
    } else {
        return (Err(ParseError::ExcludedBranch));
    }))
}

fn Decoder162<'input>(input: &mut ParseMonad<'input>) -> Result<Type5, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 33) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let label = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 249) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let block_size = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 4) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let flags = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let delay_time = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let transparent_color_index = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let terminator = ((|| PResult::Ok({ (Decoder161(input))? }))())?;
    (PResult::Ok(Type5 {
        separator,
        label,
        block_size,
        flags,
        delay_time,
        transparent_color_index,
        terminator,
    }))
}

fn Decoder163<'input>(input: &mut ParseMonad<'input>) -> Result<Type12, ParseError> {
    let tree_index = {
        (input.open_peek_context());
        let b = (input.read_byte())?;
        {
            let ret = match b {
                44 => 0,

                33 => 1,

                _ => {
                    return (Err(ParseError::ExcludedBranch));
                }
            };
            (input.close_peek_context())?;
            ret
        }
    };
    (PResult::Ok(match tree_index {
        0 => {
            let inner = (Decoder164(input))?;
            (Type12::table_based_image(inner))
        }

        1 => {
            let inner = (Decoder165(input))?;
            (Type12::plain_text_extension(inner))
        }

        _ => {
            return (Err(ParseError::ExcludedBranch));
        }
    }))
}

fn Decoder164<'input>(input: &mut ParseMonad<'input>) -> Result<Type11, ParseError> {
    let descriptor = ((|| PResult::Ok({ (Decoder166(input))? }))())?;
    let local_color_table = ((|| {
        PResult::Ok({
            match (descriptor.flags & 128 != 0) {
                true => {
                    let inner = {
                        let mut accum = (Vec::new());
                        for _ in 0..(2 << (descriptor.flags & 7)) {
                            (accum.push((Decoder167(input))?));
                        }
                        accum
                    };
                    (Type3::yes(inner))
                }

                false => {
                    let _ = ();
                    Type3::no
                }
            }
        })
    })())?;
    let data = ((|| PResult::Ok({ (Decoder168(input))? }))())?;
    (PResult::Ok(Type11 {
        descriptor,
        local_color_table,
        data,
    }))
}

fn Decoder165<'input>(input: &mut ParseMonad<'input>) -> Result<Type8, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 33) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let label = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 1) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let block_size = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 12) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let text_grid_left_position = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let text_grid_top_position = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let text_grid_width = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let text_grid_height = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let character_cell_width = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let character_cell_height = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let text_foreground_color_index = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let text_background_color_index = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let plain_text_data = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder160(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let terminator = ((|| PResult::Ok({ (Decoder161(input))? }))())?;
    (PResult::Ok(Type8 {
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

fn Decoder166<'input>(input: &mut ParseMonad<'input>) -> Result<Type9, ParseError> {
    let separator = ((|| {
        PResult::Ok({
            let b = (input.read_byte())?;
            if (b == 44) {
                b
            } else {
                return (Err(ParseError::ExcludedBranch));
            }
        })
    })())?;
    let image_left_position = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let image_top_position = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let image_width = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let image_height = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let flags = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    (PResult::Ok(Type9 {
        separator,
        image_left_position,
        image_top_position,
        image_width,
        image_height,
        flags,
    }))
}

fn Decoder167<'input>(input: &mut ParseMonad<'input>) -> Result<Type2, ParseError> {
    let r = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let g = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let b = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    (PResult::Ok(Type2 { r, g, b }))
}

fn Decoder168<'input>(input: &mut ParseMonad<'input>) -> Result<Type10, ParseError> {
    let lzw_min_code_size = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let image_data = ((|| {
        PResult::Ok({
            let mut accum = (Vec::new());
            while (input.remaining() > 0) {
                let matching_ix = {
                    (input.open_peek_context());
                    let b = (input.read_byte())?;
                    {
                        let ret = match b {
                            tmp if (tmp != 0) => 0,

                            0 => 1,

                            _ => {
                                return (Err(ParseError::ExcludedBranch));
                            }
                        };
                        (input.close_peek_context())?;
                        ret
                    }
                };
                if (matching_ix == 0) {
                    let next_elem = (Decoder160(input))?;
                    (accum.push(next_elem));
                } else {
                    break;
                }
            }
            accum
        })
    })())?;
    let terminator = ((|| PResult::Ok({ (Decoder161(input))? }))())?;
    (PResult::Ok(Type10 {
        lzw_min_code_size,
        image_data,
        terminator,
    }))
}

fn Decoder169<'input>(input: &mut ParseMonad<'input>) -> Result<Type1, ParseError> {
    let screen_width = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let screen_height = ((|| PResult::Ok({ (Decoder135(input))? }))())?;
    let flags = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let bg_color_index = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    let pixel_aspect_ratio = ((|| PResult::Ok({ (Decoder17(input))? }))())?;
    (PResult::Ok(Type1 {
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
    let ret = Decoder28(&mut parse_ctxt);
    assert!(ret.is_some());
}
