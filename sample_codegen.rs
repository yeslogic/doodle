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
    ff: u8,
    marker: u8,
}

#[derive(Debug, Clone)]
struct Type21 {
    string: Vec<u8>,
    null: u8,
}

#[derive(Debug, Clone)]
struct Type22 {
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
enum Type23 {
    jfif(Type22),
    other(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type24 {
    identifier: Type21,
    data: Type23,
}

#[derive(Debug, Clone)]
struct Type25 {
    marker: Type20,
    length: u16,
    data: Type24,
}

#[derive(Debug, Clone)]
enum Type26 {
    be(u8, u8),
    le(u8, u8),
}

#[derive(Debug, Clone)]
struct Type27 {
    tag: u16,
    r#type: u16,
    length: u32,
    offset_or_data: u32,
}

#[derive(Debug, Clone)]
struct Type28 {
    num_fields: u16,
    fields: Vec<Type27>,
    next_ifd_offset: u32,
    next_ifd: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type29 {
    byte_order: Type26,
    magic: u16,
    offset: u32,
    ifd: Type28,
}

#[derive(Debug, Clone)]
struct Type30 {
    padding: u8,
    exif: Type29,
}

#[derive(Debug, Clone)]
struct Type31 {
    xmp: Vec<u8>,
}

#[derive(Debug, Clone)]
enum Type32 {
    exif(Type30),
    other(Vec<u8>),
    xmp(Type31),
}

#[derive(Debug, Clone)]
struct Type33 {
    identifier: Type21,
    data: Type32,
}

#[derive(Debug, Clone)]
struct Type34 {
    marker: Type20,
    length: u16,
    data: Type33,
}

#[derive(Debug, Clone)]
enum Type35 {
    app0(Type25),
    app1(Type34),
}

#[derive(Debug, Clone)]
struct Type36 {
    marker: Type20,
    length: u16,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type37 {
    class_table_id: u8,
    value: u8,
}

#[derive(Debug, Clone)]
struct Type38 {
    marker: Type20,
    length: u16,
    data: Type37,
}

#[derive(Debug, Clone)]
struct Type39 {
    class_table_id: u8,
    num_codes: Vec<u8>,
    values: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type40 {
    marker: Type20,
    length: u16,
    data: Type39,
}

#[derive(Debug, Clone)]
struct Type41 {
    precision_table_id: u8,
    elements: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type42 {
    marker: Type20,
    length: u16,
    data: Type41,
}

#[derive(Debug, Clone)]
struct Type43 {
    restart_interval: u16,
}

#[derive(Debug, Clone)]
struct Type44 {
    marker: Type20,
    length: u16,
    data: Type43,
}

#[derive(Debug, Clone)]
enum Type45 {
    app0(Type25),
    app1(Type34),
    app10(Type36),
    app11(Type36),
    app12(Type36),
    app13(Type36),
    app14(Type36),
    app15(Type36),
    app2(Type36),
    app3(Type36),
    app4(Type36),
    app5(Type36),
    app6(Type36),
    app7(Type36),
    app8(Type36),
    app9(Type36),
    com(Type36),
    dac(Type38),
    dht(Type40),
    dqt(Type42),
    dri(Type44),
}

#[derive(Debug, Clone)]
struct Type46 {
    id: u8,
    sampling_factor: u8,
    quantization_table_id: u8,
}

#[derive(Debug, Clone)]
struct Type47 {
    sample_precision: u8,
    num_lines: u16,
    num_samples_per_line: u16,
    num_image_components: u8,
    image_components: Vec<Type46>,
}

#[derive(Debug, Clone)]
struct Type48 {
    marker: Type20,
    length: u16,
    data: Type47,
}

#[derive(Debug, Clone)]
enum Type49 {
    sof0(Type48),
    sof1(Type48),
    sof10(Type48),
    sof11(Type48),
    sof13(Type48),
    sof14(Type48),
    sof15(Type48),
    sof2(Type48),
    sof3(Type48),
    sof5(Type48),
    sof6(Type48),
    sof7(Type48),
    sof9(Type48),
}

#[derive(Debug, Clone)]
struct Type50 {
    component_selector: u8,
    entropy_coding_table_ids: u8,
}

#[derive(Debug, Clone)]
struct Type51 {
    num_image_components: u8,
    image_components: Vec<Type50>,
    start_spectral_selection: u8,
    end_spectral_selection: u8,
    approximation_bit_position: u8,
}

#[derive(Debug, Clone)]
struct Type52 {
    marker: Type20,
    length: u16,
    data: Type51,
}

#[derive(Debug, Clone)]
enum Type53 {
    mcu(u8),
    rst0(Type20),
    rst1(Type20),
    rst2(Type20),
    rst3(Type20),
    rst4(Type20),
    rst5(Type20),
    rst6(Type20),
    rst7(Type20),
}

#[derive(Debug, Clone)]
struct Type54 {
    scan_data: Vec<Type53>,
    scan_data_stream: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type55 {
    segments: Vec<Type45>,
    sos: Type52,
    data: Type54,
}

#[derive(Debug, Clone)]
struct Type56 {
    num_lines: u16,
}

#[derive(Debug, Clone)]
struct Type57 {
    marker: Type20,
    length: u16,
    data: Type56,
}

#[derive(Debug, Clone)]
enum Type58 {
    none,
    some(Type57),
}

#[derive(Debug, Clone)]
struct Type59 {
    initial_segment: Type35,
    segments: Vec<Type45>,
    header: Type49,
    scan: Type55,
    dnl: Type58,
    scans: Vec<Type55>,
}

#[derive(Debug, Clone)]
struct Type60 {
    soi: Type20,
    frame: Type59,
    eoi: Type20,
}

#[derive(Debug, Clone)]
struct Type61 {
    major_brand: (u8, u8, u8, u8),
    minor_version: u32,
    compatible_brands: Vec<(u8, u8, u8, u8)>,
}

#[derive(Debug, Clone)]
struct Type62 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type63 {
    version: u8,
    flags: (u8, u8, u8),
    number_of_entries: u32,
    data: Vec<Type62>,
}

#[derive(Debug, Clone)]
enum Type64 {
    dref(Type63),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type65 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type64,
}

#[derive(Debug, Clone)]
struct Type66 {
    version: u8,
    flags: (u8, u8, u8),
    predefined: u32,
    handler_type: (u8, u8, u8, u8),
    reserved: (u32, u32, u32),
    name: Type21,
}

#[derive(Debug, Clone)]
struct Type67 {
    content_type: Type21,
}

#[derive(Debug, Clone)]
struct Type68 {
    item_uri_type: Type21,
}

#[derive(Debug, Clone)]
enum Type69 {
    mime(Type67),
    unknown,
    uri(Type68),
}

#[derive(Debug, Clone)]
struct Type70 {
    item_ID: u32,
    item_protection_index: u16,
    item_type: (u8, u8, u8, u8),
    item_name: Type21,
    extra_fields: Type69,
}

#[derive(Debug, Clone)]
struct Type71 {
    item_ID: u16,
    item_protection_index: u16,
    item_name: Type21,
    content_type: Type21,
    content_encoding: Type21,
}

#[derive(Debug, Clone)]
enum Type72 {
    no(Type70),
    yes(Type71),
}

#[derive(Debug, Clone)]
struct Type73 {
    version: u8,
    flags: (u8, u8, u8),
    fields: Type72,
}

#[derive(Debug, Clone)]
enum Type74 {
    infe(Type73),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type75 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type74,
}

#[derive(Debug, Clone)]
struct Type76 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    item_info_entry: Vec<Type75>,
}

#[derive(Debug, Clone)]
enum Type77 {
    no,
    yes(u16),
}

#[derive(Debug, Clone)]
struct Type78 {
    extent_index: u64,
    extent_offset: u64,
    extent_length: u64,
}

#[derive(Debug, Clone)]
struct Type79 {
    item_ID: u32,
    construction_method: Type77,
    data_reference_index: u16,
    base_offset: u64,
    extent_count: u16,
    extents: Vec<Type78>,
}

#[derive(Debug, Clone)]
struct Type80 {
    version: u8,
    flags: (u8, u8, u8),
    offset_size_length_size: u8,
    base_offset_size_index_size: u8,
    offset_size: u8,
    length_size: u8,
    base_offset_size: u8,
    index_size: u8,
    item_count: u32,
    items: Vec<Type79>,
}

#[derive(Debug, Clone)]
struct Type81 {
    type_indicator: u32,
    locale_indicator: u32,
    value: Vec<u8>,
}

#[derive(Debug, Clone)]
enum Type82 {
    data(Type81),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type83 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type82,
}

#[derive(Debug, Clone)]
enum Type84 {
    tool(Vec<Type83>),
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
    from_item_ID: u32,
    reference_count: u16,
    to_item_ID: Vec<u32>,
}

#[derive(Debug, Clone)]
struct Type87 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type86,
}

#[derive(Debug, Clone)]
struct Type88 {
    from_item_ID: u16,
    reference_count: u16,
    to_item_ID: Vec<u16>,
}

#[derive(Debug, Clone)]
struct Type89 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type88,
}

#[derive(Debug, Clone)]
enum Type90 {
    large(Vec<Type87>),
    small(Vec<Type89>),
}

#[derive(Debug, Clone)]
struct Type91 {
    version: u8,
    flags: (u8, u8, u8),
    single_item_reference: Type90,
}

#[derive(Debug, Clone)]
enum Type92 {
    no(u32),
    yes(u16),
}

#[derive(Debug, Clone)]
struct Type93 {
    version: u8,
    flags: (u8, u8, u8),
    item_ID: Type92,
}

#[derive(Debug, Clone)]
enum Type94 {
    dinf(Vec<Type65>),
    hdlr(Type66),
    idat(Vec<u8>),
    iinf(Type76),
    iloc(Type80),
    ilst(Vec<Type85>),
    iref(Type91),
    pitm(Type93),
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
    creation_time: u32,
    modification_time: u32,
    timescale: u32,
    duration: u32,
}

#[derive(Debug, Clone)]
struct Type97 {
    creation_time: u64,
    modification_time: u64,
    timescale: u32,
    duration: u64,
}

#[derive(Debug, Clone)]
enum Type98 {
    version0(Type96),
    version1(Type97),
}

#[derive(Debug, Clone)]
struct Type99 {
    version: u8,
    flags: (u8, u8, u8),
    fields: Type98,
    rate: u32,
    volume: u16,
    reserved1: u16,
    reserved2: (u32, u32),
    matrix: Vec<u32>,
    pre_defined: Vec<u32>,
    next_track_ID: u32,
}

#[derive(Debug, Clone)]
struct Type100 {
    track_duration: u32,
    media_time: u32,
    media_rate: u32,
}

#[derive(Debug, Clone)]
struct Type101 {
    version: u8,
    flags: (u8, u8, u8),
    number_of_entries: u32,
    edit_list_table: Vec<Type100>,
}

#[derive(Debug, Clone)]
enum Type102 {
    elst(Type101),
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
struct Type104 {
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
struct Type105 {
    version: u8,
    flags: (u8, u8, u8),
    fields: Type98,
    language: u16,
    pre_defined: u16,
}

#[derive(Debug, Clone)]
struct Type106 {
    version: u8,
    flags: (u8, u8, u8),
    balance: u16,
    reserved: u16,
}

#[derive(Debug, Clone)]
struct Type107 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    chunk_offset: Vec<u64>,
}

#[derive(Debug, Clone)]
struct Type108 {
    sample_count: u32,
    sample_offset: u32,
}

#[derive(Debug, Clone)]
struct Type109 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    sample_entries: Vec<Type108>,
}

#[derive(Debug, Clone)]
enum Type110 {
    no,
    yes(u32),
}

#[derive(Debug, Clone)]
struct Type111 {
    sample_count: u32,
    group_description_index: u32,
}

#[derive(Debug, Clone)]
struct Type112 {
    version: u8,
    flags: (u8, u8, u8),
    grouping_type: u32,
    grouping_type_parameter: Type110,
    entry_count: u32,
    sample_groups: Vec<Type111>,
}

#[derive(Debug, Clone)]
struct Type113 {
    description_length: u32,
    sample_group_entry: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type114 {
    version: u8,
    flags: (u8, u8, u8),
    grouping_type: u32,
    default_length: u32,
    entry_count: u32,
    sample_groups: Vec<Type113>,
}

#[derive(Debug, Clone)]
struct Type115 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    chunk_offset: Vec<u32>,
}

#[derive(Debug, Clone)]
struct Type116 {
    first_chunk: u32,
    samples_per_chunk: u32,
    sample_description_index: u32,
}

#[derive(Debug, Clone)]
struct Type117 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    chunk_entries: Vec<Type116>,
}

#[derive(Debug, Clone)]
struct Type118 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    sample_entries: Vec<Type62>,
}

#[derive(Debug, Clone)]
struct Type119 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    sample_number: Vec<u32>,
}

#[derive(Debug, Clone)]
enum Type120 {
    no,
    yes(Vec<u32>),
}

#[derive(Debug, Clone)]
struct Type121 {
    version: u8,
    flags: (u8, u8, u8),
    sample_size: u32,
    sample_count: u32,
    entry_size: Type120,
}

#[derive(Debug, Clone)]
struct Type122 {
    sample_count: u32,
    sample_delta: u32,
}

#[derive(Debug, Clone)]
struct Type123 {
    version: u8,
    flags: (u8, u8, u8),
    entry_count: u32,
    sample_entries: Vec<Type122>,
}

#[derive(Debug, Clone)]
enum Type124 {
    co64(Type107),
    ctts(Type109),
    sbgp(Type112),
    sgpd(Type114),
    stco(Type115),
    stsc(Type117),
    stsd(Type118),
    stss(Type119),
    stsz(Type121),
    stts(Type123),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type125 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type124,
}

#[derive(Debug, Clone)]
struct Type126 {
    version: u8,
    flags: (u8, u8, u8),
    graphicsmode: u16,
    opcolor: Vec<u16>,
}

#[derive(Debug, Clone)]
enum Type127 {
    dinf(Vec<Type65>),
    smhd(Type106),
    stbl(Vec<Type125>),
    unknown(Vec<u8>),
    vmhd(Type126),
}

#[derive(Debug, Clone)]
struct Type128 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type127,
}

#[derive(Debug, Clone)]
enum Type129 {
    hdlr(Type104),
    mdhd(Type105),
    minf(Vec<Type128>),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type130 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type129,
}

#[derive(Debug, Clone)]
struct Type131 {
    creation_time: u32,
    modification_time: u32,
    track_ID: u32,
    reserved: u32,
    duration: u32,
}

#[derive(Debug, Clone)]
struct Type132 {
    creation_time: u64,
    modification_time: u64,
    track_ID: u32,
    reserved: u32,
    duration: u64,
}

#[derive(Debug, Clone)]
enum Type133 {
    version0(Type131),
    version1(Type132),
}

#[derive(Debug, Clone)]
struct Type134 {
    version: u8,
    flags: (u8, u8, u8),
    fields: Type133,
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
enum Type135 {
    edts(Vec<Type103>),
    mdia(Vec<Type130>),
    tkhd(Type134),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type136 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type135,
}

#[derive(Debug, Clone)]
enum Type137 {
    meta(u32, Vec<Type95>),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type138 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type137,
}

#[derive(Debug, Clone)]
enum Type139 {
    mvhd(Type99),
    trak(Vec<Type136>),
    udta(Vec<Type138>),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type140 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type139,
}

#[derive(Debug, Clone)]
enum Type141 {
    free,
    ftyp(Type61),
    mdat,
    meta(u32, Vec<Type95>),
    moov(Vec<Type140>),
    unknown(Vec<u8>),
}

#[derive(Debug, Clone)]
struct Type142 {
    size_field: u32,
    r#type: (u8, u8, u8, u8),
    size: u64,
    data: Type141,
}

#[derive(Debug, Clone)]
struct Type143 {
    atoms: Vec<Type142>,
}

#[derive(Debug, Clone)]
struct Type144 {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

#[derive(Debug, Clone)]
struct Type145 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type144,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type146 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Vec<Type2>,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type147 {
    greyscale: u16,
}

#[derive(Debug, Clone)]
struct Type148 {
    red: u16,
    green: u16,
    blue: u16,
}

#[derive(Debug, Clone)]
struct Type149 {
    palette_index: u8,
}

#[derive(Debug, Clone)]
enum Type150 {
    color_type_0(Type147),
    color_type_2(Type148),
    color_type_3(Type149),
    color_type_4(Type147),
    color_type_6(Type148),
}

#[derive(Debug, Clone)]
struct Type151 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type150,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type152 {
    pixels_per_unit_x: u32,
    pixels_per_unit_y: u32,
    unit_specifier: u8,
}

#[derive(Debug, Clone)]
struct Type153 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type152,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type154 {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

#[derive(Debug, Clone)]
struct Type155 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type154,
    crc: u32,
}

#[derive(Debug, Clone)]
enum Type156 {
    color_type_0(Type147),
    color_type_2(Type148),
    color_type_3(Vec<Type149>),
}

#[derive(Debug, Clone)]
struct Type157 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type156,
    crc: u32,
}

#[derive(Debug, Clone)]
enum Type158 {
    PLTE(Type146),
    bKGD(Type151),
    pHYs(Type153),
    tIME(Type155),
    tRNS(Type157),
}

#[derive(Debug, Clone)]
struct Type159 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Vec<u8>,
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type160 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: (),
    crc: u32,
}

#[derive(Debug, Clone)]
struct Type161 {
    signature: (u8, u8, u8, u8, u8, u8, u8, u8),
    ihdr: Type145,
    chunks: Vec<Type158>,
    idat: Vec<Type159>,
    more_chunks: Vec<Type158>,
    iend: Type160,
}

#[derive(Debug, Clone)]
enum Type162 {
    no(u8),
    yes,
}

#[derive(Debug, Clone)]
struct Type163 {
    tag: (u8, u8, u8, u8),
    length: u32,
    data: Vec<u8>,
    pad: Type162,
}

#[derive(Debug, Clone)]
struct Type164 {
    tag: (u8, u8, u8, u8),
    chunks: Vec<Type163>,
}

#[derive(Debug, Clone)]
struct Type165 {
    tag: (u8, u8, u8, u8),
    length: u32,
    data: Type164,
    pad: Type162,
}

#[derive(Debug, Clone)]
struct Type166 {
    string: Vec<u8>,
    __padding: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type167 {
    string: Vec<u8>,
    __nul_or_wsp: u8,
    __padding: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type168 {
    string: Vec<u8>,
    padding: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type169 {
    name: Type166,
    mode: Type167,
    uid: Type167,
    gid: Type167,
    size: u32,
    mtime: Type167,
    chksum: Type167,
    typeflag: u8,
    linkname: Type166,
    magic: (u8, u8, u8, u8, u8, u8),
    version: (u8, u8),
    uname: Type168,
    gname: Type168,
    devmajor: Type167,
    devminor: Type167,
    prefix: Type166,
    pad: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Type170 {
    header: Type169,
    file: Vec<u8>,
    __padding: (),
}

#[derive(Debug, Clone)]
struct Type171 {
    contents: Vec<Type170>,
    __padding: Vec<u8>,
    __trailing: Vec<u8>,
}

#[derive(Debug, Clone)]
enum Type172 {
    ascii(Vec<u8>),
    utf8(Vec<char>),
}

#[derive(Debug, Clone)]
enum Type173 {
    gif(Type19),
    jpeg(Type60),
    mpeg4(Type143),
    png(Type161),
    riff(Type165),
    tar(Type171),
    text(Type172),
}

#[derive(Debug, Clone)]
struct Type174 {
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
struct Type175 {
    data: Type173,
    end: (),
}

fn Decoder0<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type175> {
    (Some((Decoder1(scope, input))?))
}

fn Decoder1<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type175> {
    let data = { (unimplemented!(r#"ParallelLogic::Alts.to_ast(..)"#)) };
    let end = {
        if ((input.read_byte()).is_none()) {
            ()
        } else {
            return None;
        }
    };
    (Some(Type175 { data, end }))
}

fn Decoder2<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type19> {
    let header = { (Decoder139(scope, input))? };
    let logical_screen = { (Decoder140(scope, input))? };
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
                let next_elem = (Decoder141(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let trailer = { (Decoder142(scope, input))? };
    (Some(Type19 {
        header,
        logical_screen,
        blocks,
        trailer,
    }))
}

fn Decoder3<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type60> {
    let soi = { (Decoder66(scope, input))? };
    let frame = { (Decoder67(scope, input))? };
    let eoi = { (Decoder68(scope, input))? };
    (Some(Type60 { soi, frame, eoi }))
}

fn Decoder4<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type143> {
    let atoms = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                0
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder45(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type143 { atoms }))
}

fn Decoder5<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type161> {
    let signature = { (Decoder27(scope, input))? };
    let ihdr = { (Decoder28(scope, input))? };
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
                let next_elem = (Decoder29(scope, input))?;
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
                let next_elem = (Decoder30(scope, input))?;
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
                let next_elem = (Decoder29(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let iend = { (Decoder31(scope, input))? };
    (Some(Type161 {
        signature,
        ihdr,
        chunks,
        idat,
        more_chunks,
        iend,
    }))
}

fn Decoder6<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type165> {
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
    let length = { (Decoder23(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let pad = {
        match (length % 2 == 0) {
            true => {
                let _ = ();
                Type162::yes
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
                (Type162::no(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(Type165 {
        tag,
        length,
        data,
        pad,
    }))
}

fn Decoder7<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type171> {
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
                let next_elem = (Decoder14(scope, input))?;
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
    (Some(Type171 {
        contents,
        __padding,
        __trailing,
    }))
}

fn Decoder8<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type172> {
    (Some((unimplemented!(r#"ParallelLogic::Alts.to_ast(..)"#))))
}

fn Decoder9<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<u8>> {
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
            let next_elem = (Decoder13(scope, input))?;
            (accum.push(next_elem));
        }
    }
    (Some(accum))
}

fn Decoder10<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<char>> {
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
            let next_elem = (Decoder11(scope, input))?;
            (accum.push(next_elem));
        } else {
            break;
        }
    }
    (Some(accum))
}

fn Decoder11<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<char> {
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
                ((|byte| byte as u32)(inner))
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
                        ((|raw| raw & 31)(inner))
                    };
                    let field1 = { (Decoder12(scope, input))? };
                    (field0, field1)
                };
                ((|bytes| match bytes {
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
                                ((|raw| raw & 15)(inner))
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
                                ((|raw| raw & 63)(inner))
                            };
                            let field2 = { (Decoder12(scope, input))? };
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
                                ((|raw| raw & 15)(inner))
                            };
                            let field1 = { (Decoder12(scope, input))? };
                            let field2 = { (Decoder12(scope, input))? };
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
                                ((|raw| raw & 15)(inner))
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
                                ((|raw| raw & 63)(inner))
                            };
                            let field2 = { (Decoder12(scope, input))? };
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
                                ((|raw| raw & 15)(inner))
                            };
                            let field1 = { (Decoder12(scope, input))? };
                            let field2 = { (Decoder12(scope, input))? };
                            (field0, field1, field2)
                        }

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                };
                ((|bytes| match bytes {
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
                                ((|raw| raw & 7)(inner))
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
                                ((|raw| raw & 63)(inner))
                            };
                            let field2 = { (Decoder12(scope, input))? };
                            let field3 = { (Decoder12(scope, input))? };
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
                                ((|raw| raw & 7)(inner))
                            };
                            let field1 = { (Decoder12(scope, input))? };
                            let field2 = { (Decoder12(scope, input))? };
                            let field3 = { (Decoder12(scope, input))? };
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
                                ((|raw| raw & 7)(inner))
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
                                ((|raw| raw & 63)(inner))
                            };
                            let field2 = { (Decoder12(scope, input))? };
                            let field3 = { (Decoder12(scope, input))? };
                            (field0, field1, field2, field3)
                        }

                        _other => {
                            (unreachable!(r#"unexpected: {:?}"#, _other));
                        }
                    }
                };
                ((|bytes| match bytes {
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
    (Some(((|codepoint| (char::from_u32(codepoint)).unwrap())(inner))))
}

fn Decoder12<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let inner = {
        let b = (input.read_byte())?;
        if ((ByteSet::from_bits([0, 0, 18446744073709551615, 0])).contains(b)) {
            b
        } else {
            return None;
        }
    };
    (Some(((|raw| raw & 63)(inner))))
}

fn Decoder13<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(
        if ((ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0])).contains(b)) {
            b
        } else {
            return None;
        },
    ))
}

fn Decoder14<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type170> {
    let header = { (Decoder15(scope, input))? };
    let file = {
        let mut accum = (Vec::new());
        for _ in 0..header.size {
            (accum.push((Decoder16(scope, input))?));
        }
        accum
    };
    let __padding = {
        while (input.offset() % 512 != 0) {
            let _ = (input.read_byte())?;
        }
        ()
    };
    (Some(Type170 {
        header,
        file,
        __padding,
    }))
}

fn Decoder15<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type169> {
    (Some((unimplemented!(r#"translate @ Decoder::Slice"#))))
}

fn Decoder16<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(b))
}

fn Decoder17<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type166> {
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
    (Some(Type166 { string, __padding }))
}

fn Decoder18<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(
        if ((ByteSet::from_bits([71776119061217280, 0, 0, 0])).contains(b)) {
            b
        } else {
            return None;
        },
    ))
}

fn Decoder19<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(
        if ((ByteSet::from_bits([4294967297, 0, 0, 0])).contains(b)) {
            b
        } else {
            return None;
        },
    ))
}

fn Decoder20<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(b))
}

fn Decoder21<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type166> {
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
    (Some(Type166 { string, __padding }))
}

fn Decoder22<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type168> {
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
    (Some(Type168 { string, padding }))
}

fn Decoder23<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u32> {
    let inner = {
        let field0 = { (Decoder16(scope, input))? };
        let field1 = { (Decoder16(scope, input))? };
        let field2 = { (Decoder16(scope, input))? };
        let field3 = { (Decoder16(scope, input))? };
        (field0, field1, field2, field3)
    };
    (Some(((|x| u32le(x))(inner))))
}

fn Decoder24<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type164> {
    let tag = { (Decoder25(scope, input))? };
    let chunks = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                0
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder26(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type164 { tag, chunks }))
}

fn Decoder25<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
    let field0 = { (Decoder20(scope, input))? };
    let field1 = { (Decoder20(scope, input))? };
    let field2 = { (Decoder20(scope, input))? };
    let field3 = { (Decoder20(scope, input))? };
    (Some((field0, field1, field2, field3)))
}

fn Decoder26<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type163> {
    let tag = { (Decoder25(scope, input))? };
    let length = { (Decoder23(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let pad = {
        match (length % 2 == 0) {
            true => {
                let _ = ();
                Type162::yes
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
                (Type162::no(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(Type163 {
        tag,
        length,
        data,
        pad,
    }))
}

fn Decoder27<'input>(
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

fn Decoder28<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type145> {
    let length = { (Decoder32(scope, input))? };
    let tag = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder32(scope, input))? };
    (Some(Type145 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder29<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type158> {
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
            let inner = (Decoder37(scope, input))?;
            (Type158::bKGD(inner))
        }

        1 => {
            let inner = (Decoder38(scope, input))?;
            (Type158::pHYs(inner))
        }

        2 => {
            let inner = (Decoder39(scope, input))?;
            (Type158::PLTE(inner))
        }

        3 => {
            let inner = (Decoder40(scope, input))?;
            (Type158::tIME(inner))
        }

        4 => {
            let inner = (Decoder41(scope, input))?;
            (Type158::tRNS(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder30<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type159> {
    let length = { (Decoder32(scope, input))? };
    let tag = { (Decoder35(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder32(scope, input))? };
    (Some(Type159 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder31<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type160> {
    let length = { (Decoder32(scope, input))? };
    let tag = { (Decoder33(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder32(scope, input))? };
    (Some(Type160 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder32<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u32> {
    let inner = {
        let field0 = { (Decoder16(scope, input))? };
        let field1 = { (Decoder16(scope, input))? };
        let field2 = { (Decoder16(scope, input))? };
        let field3 = { (Decoder16(scope, input))? };
        (field0, field1, field2, field3)
    };
    (Some(((|x| u32be(x))(inner))))
}

fn Decoder33<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
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

fn Decoder34<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<()> {
    (Some(()))
}

fn Decoder35<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
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

fn Decoder36<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<u8>> {
    let mut accum = (Vec::new());
    while true {
        let matching_ix = {
            let lookahead = &mut (input.clone());
            0
        };
        if (matching_ix == 0) {
            let next_elem = (Decoder16(scope, input))?;
            (accum.push(next_elem));
        } else {
            break;
        }
    }
    (Some(accum))
}

fn Decoder37<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type151> {
    let length = { (Decoder32(scope, input))? };
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
    let crc = { (Decoder32(scope, input))? };
    (Some(Type151 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder38<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type153> {
    let length = { (Decoder32(scope, input))? };
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
    let crc = { (Decoder32(scope, input))? };
    (Some(Type153 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder39<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type146> {
    let length = { (Decoder32(scope, input))? };
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
    let crc = { (Decoder32(scope, input))? };
    (Some(Type146 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder40<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type155> {
    let length = { (Decoder32(scope, input))? };
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
    let crc = { (Decoder32(scope, input))? };
    (Some(Type155 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder41<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type157> {
    let length = { (Decoder32(scope, input))? };
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
    let crc = { (Decoder32(scope, input))? };
    (Some(Type157 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder42<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u16> {
    let inner = {
        let field0 = { (Decoder16(scope, input))? };
        let field1 = { (Decoder16(scope, input))? };
        (field0, field1)
    };
    (Some(((|x| u16be(x))(inner))))
}

fn Decoder43<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
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

fn Decoder44<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type144> {
    let width = { (Decoder32(scope, input))? };
    let height = { (Decoder32(scope, input))? };
    let bit_depth = { (Decoder16(scope, input))? };
    let color_type = { (Decoder16(scope, input))? };
    let compression_method = { (Decoder16(scope, input))? };
    let filter_method = { (Decoder16(scope, input))? };
    let interlace_method = { (Decoder16(scope, input))? };
    (Some(Type144 {
        width,
        height,
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
    }))
}

fn Decoder45<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type142> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type142 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder46<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
    let field0 = { (Decoder20(scope, input))? };
    let field1 = { (Decoder20(scope, input))? };
    let field2 = { (Decoder20(scope, input))? };
    let field3 = { (Decoder20(scope, input))? };
    (Some((field0, field1, field2, field3)))
}

fn Decoder47<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u64> {
    let inner = {
        let field0 = { (Decoder16(scope, input))? };
        let field1 = { (Decoder16(scope, input))? };
        let field2 = { (Decoder16(scope, input))? };
        let field3 = { (Decoder16(scope, input))? };
        let field4 = { (Decoder16(scope, input))? };
        let field5 = { (Decoder16(scope, input))? };
        let field6 = { (Decoder16(scope, input))? };
        let field7 = { (Decoder16(scope, input))? };
        (
            field0, field1, field2, field3, field4, field5, field6, field7,
        )
    };
    (Some(((|x| u64be(x))(inner))))
}

fn Decoder48<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
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

fn Decoder49<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type140> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type140 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder50<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type136> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type136 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder51<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type138> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type138 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder52<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type103> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
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

fn Decoder53<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type130> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type130 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder54<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
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

fn Decoder55<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type128> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type128 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder56<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder57<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type125> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type125 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder58<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type75> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type75 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder59<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type85> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
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

fn Decoder60<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type83> {
    let size_field = { (Decoder32(scope, input))? };
    let r#type = { (Decoder46(scope, input))? };
    let size = {
        match size_field {
            0 => 0,

            1 => {
                let inner = (Decoder47(scope, input))?;
                ((|x| x - 16)(inner))
            }

            _ => ((size_field - 8) as u64),

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type83 {
        size_field,
        r#type,
        size,
        data,
    }))
}

fn Decoder61<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
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

fn Decoder66<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    (Some(Type20 { ff, marker }))
}

fn Decoder67<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type59> {
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
                let inner = (Decoder69(scope, input))?;
                (Type35::app0(inner))
            }

            1 => {
                let inner = (Decoder70(scope, input))?;
                (Type35::app1(inner))
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
                let next_elem = (Decoder71(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let header = { (Decoder72(scope, input))? };
    let scan = { (Decoder73(scope, input))? };
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
                let inner = (Decoder74(scope, input))?;
                (Type58::some(inner))
            }

            1 => {
                let _ = ();
                Type58::none
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
                let next_elem = (Decoder75(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type59 {
        initial_segment,
        segments,
        header,
        scan,
        dnl,
        scans,
    }))
}

fn Decoder68<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    (Some(Type20 { ff, marker }))
}

fn Decoder69<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type25> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type25 {
        marker,
        length,
        data,
    }))
}

fn Decoder70<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type34> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type34 {
        marker,
        length,
        data,
    }))
}

fn Decoder71<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type45> {
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
            let inner = (Decoder106(scope, input))?;
            (Type45::dqt(inner))
        }

        1 => {
            let inner = (Decoder107(scope, input))?;
            (Type45::dht(inner))
        }

        2 => {
            let inner = (Decoder108(scope, input))?;
            (Type45::dac(inner))
        }

        3 => {
            let inner = (Decoder109(scope, input))?;
            (Type45::dri(inner))
        }

        4 => {
            let inner = (Decoder69(scope, input))?;
            (Type45::app0(inner))
        }

        5 => {
            let inner = (Decoder70(scope, input))?;
            (Type45::app1(inner))
        }

        6 => {
            let inner = (Decoder110(scope, input))?;
            (Type45::app2(inner))
        }

        7 => {
            let inner = (Decoder111(scope, input))?;
            (Type45::app3(inner))
        }

        8 => {
            let inner = (Decoder112(scope, input))?;
            (Type45::app4(inner))
        }

        9 => {
            let inner = (Decoder113(scope, input))?;
            (Type45::app5(inner))
        }

        10 => {
            let inner = (Decoder114(scope, input))?;
            (Type45::app6(inner))
        }

        11 => {
            let inner = (Decoder115(scope, input))?;
            (Type45::app7(inner))
        }

        12 => {
            let inner = (Decoder116(scope, input))?;
            (Type45::app8(inner))
        }

        13 => {
            let inner = (Decoder117(scope, input))?;
            (Type45::app9(inner))
        }

        14 => {
            let inner = (Decoder118(scope, input))?;
            (Type45::app10(inner))
        }

        15 => {
            let inner = (Decoder119(scope, input))?;
            (Type45::app11(inner))
        }

        16 => {
            let inner = (Decoder120(scope, input))?;
            (Type45::app12(inner))
        }

        17 => {
            let inner = (Decoder121(scope, input))?;
            (Type45::app13(inner))
        }

        18 => {
            let inner = (Decoder122(scope, input))?;
            (Type45::app14(inner))
        }

        19 => {
            let inner = (Decoder123(scope, input))?;
            (Type45::app15(inner))
        }

        20 => {
            let inner = (Decoder124(scope, input))?;
            (Type45::com(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder72<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type49> {
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
            let inner = (Decoder91(scope, input))?;
            (Type49::sof0(inner))
        }

        1 => {
            let inner = (Decoder92(scope, input))?;
            (Type49::sof1(inner))
        }

        2 => {
            let inner = (Decoder93(scope, input))?;
            (Type49::sof2(inner))
        }

        3 => {
            let inner = (Decoder94(scope, input))?;
            (Type49::sof3(inner))
        }

        4 => {
            let inner = (Decoder95(scope, input))?;
            (Type49::sof5(inner))
        }

        5 => {
            let inner = (Decoder96(scope, input))?;
            (Type49::sof6(inner))
        }

        6 => {
            let inner = (Decoder97(scope, input))?;
            (Type49::sof7(inner))
        }

        7 => {
            let inner = (Decoder98(scope, input))?;
            (Type49::sof9(inner))
        }

        8 => {
            let inner = (Decoder99(scope, input))?;
            (Type49::sof10(inner))
        }

        9 => {
            let inner = (Decoder100(scope, input))?;
            (Type49::sof11(inner))
        }

        10 => {
            let inner = (Decoder101(scope, input))?;
            (Type49::sof13(inner))
        }

        11 => {
            let inner = (Decoder102(scope, input))?;
            (Type49::sof14(inner))
        }

        12 => {
            let inner = (Decoder103(scope, input))?;
            (Type49::sof15(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder73<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type55> {
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
                let next_elem = (Decoder71(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let sos = { (Decoder76(scope, input))? };
    let data = { (Decoder90(scope, input))? };
    (Some(Type55 {
        segments,
        sos,
        data,
    }))
}

fn Decoder74<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type57> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type57 {
        marker,
        length,
        data,
    }))
}

fn Decoder75<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type55> {
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
                let next_elem = (Decoder71(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let sos = { (Decoder76(scope, input))? };
    let data = { (Decoder77(scope, input))? };
    (Some(Type55 {
        segments,
        sos,
        data,
    }))
}

fn Decoder76<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type52> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type52 {
        marker,
        length,
        data,
    }))
}

fn Decoder77<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type54> {
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
                            let inner = (Decoder78(scope, input))?;
                            (Type53::mcu(inner))
                        }

                        1 => {
                            let inner = (Decoder79(scope, input))?;
                            (Type53::rst0(inner))
                        }

                        2 => {
                            let inner = (Decoder80(scope, input))?;
                            (Type53::rst1(inner))
                        }

                        3 => {
                            let inner = (Decoder81(scope, input))?;
                            (Type53::rst2(inner))
                        }

                        4 => {
                            let inner = (Decoder82(scope, input))?;
                            (Type53::rst3(inner))
                        }

                        5 => {
                            let inner = (Decoder83(scope, input))?;
                            (Type53::rst4(inner))
                        }

                        6 => {
                            let inner = (Decoder84(scope, input))?;
                            (Type53::rst5(inner))
                        }

                        7 => {
                            let inner = (Decoder85(scope, input))?;
                            (Type53::rst6(inner))
                        }

                        8 => {
                            let inner = (Decoder86(scope, input))?;
                            (Type53::rst7(inner))
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
        ((((scan_data.clone()).into_iter()).flat_map(
            (|x| match x {
                Type53::mcu(v) => ([v].to_vec()),

                Type53::rst0(..) => ([].to_vec()),

                Type53::rst1(..) => ([].to_vec()),

                Type53::rst2(..) => ([].to_vec()),

                Type53::rst3(..) => ([].to_vec()),

                Type53::rst4(..) => ([].to_vec()),

                Type53::rst5(..) => ([].to_vec()),

                Type53::rst6(..) => ([].to_vec()),

                Type53::rst7(..) => ([].to_vec()),

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }),
        ))
        .collect())
    };
    (Some(Type54 {
        scan_data,
        scan_data_stream,
    }))
}

fn Decoder78<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
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
            ((|_| 255)(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder79<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    (Some(Type20 { ff, marker }))
}

fn Decoder80<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    (Some(Type20 { ff, marker }))
}

fn Decoder81<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    (Some(Type20 { ff, marker }))
}

fn Decoder82<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    (Some(Type20 { ff, marker }))
}

fn Decoder83<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    (Some(Type20 { ff, marker }))
}

fn Decoder84<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    (Some(Type20 { ff, marker }))
}

fn Decoder85<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    (Some(Type20 { ff, marker }))
}

fn Decoder86<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    (Some(Type20 { ff, marker }))
}

fn Decoder87<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type51> {
    let num_image_components = { (Decoder16(scope, input))? };
    let image_components = {
        let mut accum = (Vec::new());
        for _ in 0..num_image_components {
            (accum.push((Decoder88(scope, input))?));
        }
        accum
    };
    let start_spectral_selection = { (Decoder16(scope, input))? };
    let end_spectral_selection = { (Decoder16(scope, input))? };
    let approximation_bit_position = { (Decoder16(scope, input))? };
    (Some(Type51 {
        num_image_components,
        image_components,
        start_spectral_selection,
        end_spectral_selection,
        approximation_bit_position,
    }))
}

fn Decoder88<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type50> {
    let component_selector = { (Decoder16(scope, input))? };
    let entropy_coding_table_ids = { (Decoder16(scope, input))? };
    (Some(Type50 {
        component_selector,
        entropy_coding_table_ids,
    }))
}

fn Decoder89<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type56> {
    let num_lines = { (Decoder42(scope, input))? };
    (Some(Type56 { num_lines }))
}

fn Decoder90<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type54> {
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
                            let inner = (Decoder78(scope, input))?;
                            (Type53::mcu(inner))
                        }

                        1 => {
                            let inner = (Decoder79(scope, input))?;
                            (Type53::rst0(inner))
                        }

                        2 => {
                            let inner = (Decoder80(scope, input))?;
                            (Type53::rst1(inner))
                        }

                        3 => {
                            let inner = (Decoder81(scope, input))?;
                            (Type53::rst2(inner))
                        }

                        4 => {
                            let inner = (Decoder82(scope, input))?;
                            (Type53::rst3(inner))
                        }

                        5 => {
                            let inner = (Decoder83(scope, input))?;
                            (Type53::rst4(inner))
                        }

                        6 => {
                            let inner = (Decoder84(scope, input))?;
                            (Type53::rst5(inner))
                        }

                        7 => {
                            let inner = (Decoder85(scope, input))?;
                            (Type53::rst6(inner))
                        }

                        8 => {
                            let inner = (Decoder86(scope, input))?;
                            (Type53::rst7(inner))
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
        ((((scan_data.clone()).into_iter()).flat_map(
            (|x| match x {
                Type53::mcu(v) => ([v].to_vec()),

                Type53::rst0(..) => ([].to_vec()),

                Type53::rst1(..) => ([].to_vec()),

                Type53::rst2(..) => ([].to_vec()),

                Type53::rst3(..) => ([].to_vec()),

                Type53::rst4(..) => ([].to_vec()),

                Type53::rst5(..) => ([].to_vec()),

                Type53::rst6(..) => ([].to_vec()),

                Type53::rst7(..) => ([].to_vec()),

                _other => {
                    (unreachable!(r#"unexpected: {:?}"#, _other));
                }
            }),
        ))
        .collect())
    };
    (Some(Type54 {
        scan_data,
        scan_data_stream,
    }))
}

fn Decoder91<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder92<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder93<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder94<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder95<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder96<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder97<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder98<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder99<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder100<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder101<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder102<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder103<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type48> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type48 {
        marker,
        length,
        data,
    }))
}

fn Decoder104<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type47> {
    let sample_precision = { (Decoder16(scope, input))? };
    let num_lines = { (Decoder42(scope, input))? };
    let num_samples_per_line = { (Decoder42(scope, input))? };
    let num_image_components = { (Decoder16(scope, input))? };
    let image_components = {
        let mut accum = (Vec::new());
        for _ in 0..num_image_components {
            (accum.push((Decoder105(scope, input))?));
        }
        accum
    };
    (Some(Type47 {
        sample_precision,
        num_lines,
        num_samples_per_line,
        num_image_components,
        image_components,
    }))
}

fn Decoder105<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type46> {
    let id = { (Decoder16(scope, input))? };
    let sampling_factor = { (Decoder16(scope, input))? };
    let quantization_table_id = { (Decoder16(scope, input))? };
    (Some(Type46 {
        id,
        sampling_factor,
        quantization_table_id,
    }))
}

fn Decoder106<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type42 {
        marker,
        length,
        data,
    }))
}

fn Decoder107<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type40> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type40 {
        marker,
        length,
        data,
    }))
}

fn Decoder108<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type38> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type38 {
        marker,
        length,
        data,
    }))
}

fn Decoder109<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type44> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type44 {
        marker,
        length,
        data,
    }))
}

fn Decoder110<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder111<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder112<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder113<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder114<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder115<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder116<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder117<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder118<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder119<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder120<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder121<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder122<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder123<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder124<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
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
        Type20 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type36 {
        marker,
        length,
        data,
    }))
}

fn Decoder125<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type43> {
    let restart_interval = { (Decoder42(scope, input))? };
    (Some(Type43 { restart_interval }))
}

fn Decoder126<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type37> {
    let class_table_id = { (Decoder16(scope, input))? };
    let value = { (Decoder16(scope, input))? };
    (Some(Type37 {
        class_table_id,
        value,
    }))
}

fn Decoder127<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type39> {
    let class_table_id = { (Decoder16(scope, input))? };
    let num_codes = {
        let mut accum = (Vec::new());
        for _ in 0..16 {
            (accum.push((Decoder16(scope, input))?));
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
                let next_elem = (Decoder16(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type39 {
        class_table_id,
        num_codes,
        values,
    }))
}

fn Decoder128<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type41> {
    let precision_table_id = { (Decoder16(scope, input))? };
    let elements = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                0
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder16(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type41 {
        precision_table_id,
        elements,
    }))
}

fn Decoder129<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type33> {
    /* Sequential(AccumRecord { constructor: Simple("Type33"), fields: [("identifier", Simple(Invoke(130, []))), ("data", Other(ExprMatch(FunctionCall(FieldAccess(FieldAccess(Entity(Local("identifier")), ByName("string")), ByName("as_slice")), []), [(Pattern(ArrayLiteral([PrimLiteral(Numeric(69)), PrimLiteral(Numeric(120)), PrimLiteral(Numeric(105)), PrimLiteral(Numeric(102))])), Derived(VariantOf(Compound("Type32", "exif"), Simple(Invoke(131, []))))), (Pattern(ArrayLiteral([PrimLiteral(Numeric(104)), PrimLiteral(Numeric(116)), PrimLiteral(Numeric(116)), PrimLiteral(Numeric(112)), PrimLiteral(Numeric(58)), PrimLiteral(Numeric(47)), PrimLiteral(Numeric(47)), PrimLiteral(Numeric(110)), PrimLiteral(Numeric(115)), PrimLiteral(Numeric(46)), PrimLiteral(Numeric(97)), PrimLiteral(Numeric(100)), PrimLiteral(Numeric(111)), PrimLiteral(Numeric(98)), PrimLiteral(Numeric(101)), PrimLiteral(Numeric(46)), PrimLiteral(Numeric(99)), PrimLiteral(Numeric(111)), PrimLiteral(Numeric(109)), PrimLiteral(Numeric(47)), PrimLiteral(Numeric(120)), PrimLiteral(Numeric(97)), PrimLiteral(Numeric(112)), PrimLiteral(Numeric(47)), PrimLiteral(Numeric(49)), PrimLiteral(Numeric(46)), PrimLiteral(Numeric(48)), PrimLiteral(Numeric(47))])), Derived(VariantOf(Compound("Type32", "xmp"), Simple(Invoke(132, []))))), (Pattern(CatchAll(None)), Derived(VariantOf(Compound("Type32", "other"), Repeat(ContinueOnMatch(MatchTree { accept: Some(1), branches: [(!{}, MatchTree { accept: Some(0), branches: [] })] }, Simple(Invoke(16, [])))))))])))] }) */
    let identifier = { (Decoder130(scope, input))? };
    let data = {
        match (identifier.string.as_slice()) {
            [69, 120, 105, 102] => {
                let inner = (Decoder131(scope, input))?;
                (Type32::exif(inner))
            }

            [104, 116, 116, 112, 58, 47, 47, 110, 115, 46, 97, 100, 111, 98, 101, 46, 99, 111, 109, 47, 120, 97, 112, 47, 49, 46, 48, 47] =>
            {
                let inner = (Decoder132(scope, input))?;
                (Type32::xmp(inner))
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
                            let next_elem = (Decoder16(scope, input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                };
                (Type32::other(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(Type33 { identifier, data }))
}

fn Decoder130<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
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

fn Decoder131<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type30> {
    let padding = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    let exif = { (Decoder133(scope, input))? };
    (Some(Type30 { padding, exif }))
}

fn Decoder132<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type31> {
    let xmp = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                0
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder16(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type31 { xmp }))
}

fn Decoder133<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type29> {
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
                (Type26::le(field0, field1))
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
                (Type26::be(field0, field1))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let magic = {
        match byte_order {
            Type26::le(..) => (Decoder134(scope, input))?,

            Type26::be(..) => (Decoder42(scope, input))?,

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let offset = {
        match byte_order {
            Type26::le(..) => (Decoder23(scope, input))?,

            Type26::be(..) => (Decoder32(scope, input))?,

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    let ifd = { (unimplemented!(r#"translate @ Decoder::WithRelativeOffset"#)) };
    (Some(Type29 {
        byte_order,
        magic,
        offset,
        ifd,
    }))
}

fn Decoder134<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u16> {
    let inner = {
        let field0 = { (Decoder16(scope, input))? };
        let field1 = { (Decoder16(scope, input))? };
        (field0, field1)
    };
    (Some(((|x| u16le(x))(inner))))
}

fn Decoder135<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type24> {
    let identifier = { (Decoder136(scope, input))? };
    let data = {
        match (identifier.string.as_slice()) {
            [74, 70, 73, 70] => {
                let inner = (Decoder137(scope, input))?;
                (Type23::jfif(inner))
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
                            let next_elem = (Decoder16(scope, input))?;
                            (accum.push(next_elem));
                        } else {
                            break;
                        }
                    }
                    accum
                };
                (Type23::other(inner))
            }

            _other => {
                (unreachable!(r#"unexpected: {:?}"#, _other));
            }
        }
    };
    (Some(Type24 { identifier, data }))
}

fn Decoder136<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
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

fn Decoder137<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type22> {
    let version_major = { (Decoder16(scope, input))? };
    let version_minor = { (Decoder16(scope, input))? };
    let density_units = { (Decoder16(scope, input))? };
    let density_x = { (Decoder42(scope, input))? };
    let density_y = { (Decoder42(scope, input))? };
    let thumbnail_width = { (Decoder16(scope, input))? };
    let thumbnail_height = { (Decoder16(scope, input))? };
    let thumbnail_pixels = {
        let mut accum = (Vec::new());
        for _ in 0..thumbnail_height {
            (accum.push({
                let mut accum = (Vec::new());
                for _ in 0..thumbnail_width {
                    (accum.push((Decoder138(scope, input))?));
                }
                accum
            }));
        }
        accum
    };
    (Some(Type22 {
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

fn Decoder138<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type2> {
    let r = { (Decoder16(scope, input))? };
    let g = { (Decoder16(scope, input))? };
    let b = { (Decoder16(scope, input))? };
    (Some(Type2 { r, g, b }))
}

fn Decoder139<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type0> {
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
            (accum.push((Decoder20(scope, input))?));
        }
        accum
    };
    (Some(Type0 { signature, version }))
}

fn Decoder140<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type4> {
    let descriptor = { (Decoder156(scope, input))? };
    let global_color_table = {
        match (descriptor.flags & 128 != 0) {
            true => {
                let inner = {
                    let mut accum = (Vec::new());
                    for _ in 0..(2 << (descriptor.flags & 7)) {
                        (accum.push((Decoder154(scope, input))?));
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

fn Decoder141<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type17> {
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
            let inner = (Decoder143(scope, input))?;
            (Type17::graphic_block(inner))
        }

        1 => {
            let inner = (Decoder144(scope, input))?;
            (Type17::special_purpose_block(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder142<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type18> {
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

fn Decoder143<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type13> {
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
                let inner = (Decoder149(scope, input))?;
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
    let graphic_rendering_block = { (Decoder150(scope, input))? };
    (Some(Type13 {
        graphic_control_extension,
        graphic_rendering_block,
    }))
}

fn Decoder144<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type16> {
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
            let inner = (Decoder145(scope, input))?;
            (Type16::application_extension(inner))
        }

        1 => {
            let inner = (Decoder146(scope, input))?;
            (Type16::comment_extension(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder145<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type14> {
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
            (accum.push((Decoder16(scope, input))?));
        }
        accum
    };
    let authentication_code = {
        let mut accum = (Vec::new());
        for _ in 0..3 {
            (accum.push((Decoder16(scope, input))?));
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
                let next_elem = (Decoder147(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder148(scope, input))? };
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

fn Decoder146<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type15> {
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
                let next_elem = (Decoder147(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder148(scope, input))? };
    (Some(Type15 {
        separator,
        label,
        comment_data,
        terminator,
    }))
}

fn Decoder147<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type7> {
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
            (accum.push((Decoder16(scope, input))?));
        }
        accum
    };
    (Some(Type7 { len_bytes, data }))
}

fn Decoder148<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(if (b == 0) {
        b
    } else {
        return None;
    }))
}

fn Decoder149<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type5> {
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
    let flags = { (Decoder16(scope, input))? };
    let delay_time = { (Decoder134(scope, input))? };
    let transparent_color_index = { (Decoder16(scope, input))? };
    let terminator = { (Decoder148(scope, input))? };
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

fn Decoder150<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type12> {
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
            let inner = (Decoder151(scope, input))?;
            (Type12::table_based_image(inner))
        }

        1 => {
            let inner = (Decoder152(scope, input))?;
            (Type12::plain_text_extension(inner))
        }

        _other => {
            (unreachable!(r#"unexpected: {:?}"#, _other));
        }
    }))
}

fn Decoder151<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type11> {
    let descriptor = { (Decoder153(scope, input))? };
    let local_color_table = {
        match (descriptor.flags & 128 != 0) {
            true => {
                let inner = {
                    let mut accum = (Vec::new());
                    for _ in 0..(2 << (descriptor.flags & 7)) {
                        (accum.push((Decoder154(scope, input))?));
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
    let data = { (Decoder155(scope, input))? };
    (Some(Type11 {
        descriptor,
        local_color_table,
        data,
    }))
}

fn Decoder152<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type8> {
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
    let text_grid_left_position = { (Decoder134(scope, input))? };
    let text_grid_top_position = { (Decoder134(scope, input))? };
    let text_grid_width = { (Decoder134(scope, input))? };
    let text_grid_height = { (Decoder134(scope, input))? };
    let character_cell_width = { (Decoder16(scope, input))? };
    let character_cell_height = { (Decoder16(scope, input))? };
    let text_foreground_color_index = { (Decoder16(scope, input))? };
    let text_background_color_index = { (Decoder16(scope, input))? };
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
                let next_elem = (Decoder147(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder148(scope, input))? };
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

fn Decoder153<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type9> {
    let separator = {
        let b = (input.read_byte())?;
        if (b == 44) {
            b
        } else {
            return None;
        }
    };
    let image_left_position = { (Decoder134(scope, input))? };
    let image_top_position = { (Decoder134(scope, input))? };
    let image_width = { (Decoder134(scope, input))? };
    let image_height = { (Decoder134(scope, input))? };
    let flags = { (Decoder16(scope, input))? };
    (Some(Type9 {
        separator,
        image_left_position,
        image_top_position,
        image_width,
        image_height,
        flags,
    }))
}

fn Decoder154<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type2> {
    let r = { (Decoder16(scope, input))? };
    let g = { (Decoder16(scope, input))? };
    let b = { (Decoder16(scope, input))? };
    (Some(Type2 { r, g, b }))
}

fn Decoder155<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type10> {
    let lzw_min_code_size = { (Decoder16(scope, input))? };
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
                let next_elem = (Decoder147(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder148(scope, input))? };
    (Some(Type10 {
        lzw_min_code_size,
        image_data,
        terminator,
    }))
}

fn Decoder156<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type1> {
    let screen_width = { (Decoder134(scope, input))? };
    let screen_height = { (Decoder134(scope, input))? };
    let flags = { (Decoder16(scope, input))? };
    let bg_color_index = { (Decoder16(scope, input))? };
    let pixel_aspect_ratio = { (Decoder16(scope, input))? };
    (Some(Type1 {
        screen_width,
        screen_height,
        flags,
        bg_color_index,
        pixel_aspect_ratio,
    }))
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
