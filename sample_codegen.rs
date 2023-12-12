use doodle::prelude::*;

struct Type0 {
    signature: (u8, u8, u8),
    version: Vec<u8>,
}

struct Type1 {
    screen_width: u16,
    screen_height: u16,
    flags: u8,
    bg_color_index: u8,
    pixel_aspect_ratio: u8,
}

struct Type2 {
    r: u8,
    g: u8,
    b: u8,
}

enum Type3 {
    no,
    yes(Vec<Type2>),
}

struct Type4 {
    descriptor: Type1,
    global_color_table: Type3,
}

struct Type5 {
    separator: u8,
    label: u8,
    block_size: u8,
    flags: u8,
    delay_time: u16,
    transparent_color_index: u8,
    terminator: u8,
}

enum Type6 {
    some(Type5),
    none,
}

struct Type7 {
    separator: u8,
    image_left_position: u16,
    image_top_position: u16,
    image_width: u16,
    image_height: u16,
    flags: u8,
}

struct Type8 {
    len_bytes: u8,
    data: Vec<u8>,
}

struct Type9 {
    lzw_min_code_size: u8,
    image_data: Vec<Type8>,
    terminator: u8,
}

struct Type10 {
    descriptor: Type7,
    local_color_table: Type3,
    data: Type9,
}

struct Type11 {
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
    plain_text_data: Vec<Type8>,
    terminator: u8,
}

enum Type12 {
    table_based_image(Type10),
    plain_text_extension(Type11),
}

struct Type13 {
    graphic_control_extension: Type6,
    graphic_rendering_block: Type12,
}

struct Type14 {
    separator: u8,
    label: u8,
    block_size: u8,
    identifier: Vec<u8>,
    authentication_code: Vec<u8>,
    application_data: Vec<Type8>,
    terminator: u8,
}

struct Type15 {
    separator: u8,
    label: u8,
    comment_data: Vec<Type8>,
    terminator: u8,
}

enum Type16 {
    application_extension(Type14),
    comment_extension(Type15),
}

enum Type17 {
    graphic_block(Type13),
    special_purpose_block(Type16),
}

struct Type18 {
    separator: u8,
}

struct Type19 {
    header: Type0,
    logical_screen: Type4,
    blocks: Vec<Type17>,
    trailer: Type18,
}

struct Type20 {
    magic: (u8, u8),
    method: u8,
    file_flags: u8,
    timestamp: u32,
    compression_flags: u8,
    os_id: u8,
}

struct Type21 {
    string: Vec<u8>,
    null: u8,
}

enum Type22 {
    no,
    yes(Type21),
}

struct Type23 {
    code: u16,
    extra: u8,
}

struct Type24 {
    distance_extra_bits: u16,
    distance: u16,
}

struct Type25 {
    length_extra_bits: u8,
    length: u16,
    distance_code: u16,
    distance_record: Type24,
}

enum Type26 {
    none,
    some(Type25),
}

struct Type27 {
    code: u16,
    extra: Type26,
}

struct Type28 {
    length: u16,
    distance: u16,
}

enum Type29 {
    literal(u8),
    reference(Type28),
}

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

struct Type31 {
    length_extra_bits: u8,
    length: u16,
    distance_code: u8,
    distance_record: Type24,
}

enum Type32 {
    none,
    some(Type31),
}

struct Type33 {
    code: u16,
    extra: Type32,
}

struct Type34 {
    codes: Vec<Type33>,
    codes_values: Vec<Type29>,
}

enum Type35 {
    literal(u8),
}

struct Type36 {
    align: (),
    len: u16,
    nlen: u16,
    bytes: Vec<u8>,
    codes_values: Vec<Type35>,
}

enum Type37 {
    dynamic_huffman(Type30),
    fixed_huffman(Type34),
    uncompressed(Type36),
}

struct Type38 {
    r#final: u8,
    r#type: u8,
    data: Type37,
}

struct Type39 {
    blocks: Vec<Type38>,
    codes: Vec<Type29>,
    inflate: Vec<u8>,
}

struct Type40 {
    crc: u32,
    length: u32,
}

struct Type41 {
    header: Type20,
    fname: Type22,
    data: Type39,
    footer: Type40,
}

struct Type42 {
    ff: u8,
    marker: u8,
}

struct Type43 {
    version_major: u8,
    version_minor: u8,
    density_units: u8,
    density_x: u16,
    density_y: u16,
    thumbnail_width: u8,
    thumbnail_height: u8,
    thumbnail_pixels: Vec<Vec<Type2>>,
}

enum Type44 {
    other(Vec<u8>),
    jfif(Type43),
}

struct Type45 {
    identifier: Type21,
    data: Type44,
}

struct Type46 {
    marker: Type42,
    length: u16,
    data: Type45,
}

struct Type47 {
    xmp: Vec<u8>,
}

enum Type48 {
    le(u8, u8),
    be(u8, u8),
}

struct Type49 {
    tag: u16,
    r#type: u16,
    length: u32,
    offset_or_data: u32,
}

struct Type50 {
    num_fields: u16,
    fields: Vec<Type49>,
    next_ifd_offset: u32,
    next_ifd: Vec<u8>,
}

struct Type51 {
    byte_order: Type48,
    magic: u16,
    offset: u32,
    ifd: Type50,
}

struct Type52 {
    padding: u8,
    exif: Type51,
}

enum Type53 {
    other(Vec<u8>),
    xmp(Type47),
    exif(Type52),
}

struct Type54 {
    identifier: Type21,
    data: Type53,
}

struct Type55 {
    marker: Type42,
    length: u16,
    data: Type54,
}

enum Type56 {
    app0(Type46),
    app1(Type55),
}

struct Type57 {
    precision_table_id: u8,
    elements: Vec<u8>,
}

struct Type58 {
    marker: Type42,
    length: u16,
    data: Type57,
}

struct Type59 {
    class_table_id: u8,
    num_codes: Vec<u8>,
    values: Vec<u8>,
}

struct Type60 {
    marker: Type42,
    length: u16,
    data: Type59,
}

struct Type61 {
    class_table_id: u8,
    value: u8,
}

struct Type62 {
    marker: Type42,
    length: u16,
    data: Type61,
}

struct Type63 {
    restart_interval: u16,
}

struct Type64 {
    marker: Type42,
    length: u16,
    data: Type63,
}

struct Type65 {
    marker: Type42,
    length: u16,
    data: Vec<u8>,
}

enum Type66 {
    dqt(Type58),
    dht(Type60),
    dac(Type62),
    dri(Type64),
    app0(Type46),
    app1(Type55),
    app2(Type65),
    app3(Type65),
    app4(Type65),
    app5(Type65),
    app6(Type65),
    app7(Type65),
    app8(Type65),
    app9(Type65),
    app10(Type65),
    app11(Type65),
    app12(Type65),
    app13(Type65),
    app14(Type65),
    app15(Type65),
    com(Type65),
}

struct Type67 {
    id: u8,
    sampling_factor: u8,
    quantization_table_id: u8,
}

struct Type68 {
    sample_precision: u8,
    num_lines: u16,
    num_samples_per_line: u16,
    num_image_components: u8,
    image_components: Vec<Type67>,
}

struct Type69 {
    marker: Type42,
    length: u16,
    data: Type68,
}

enum Type70 {
    sof0(Type69),
    sof1(Type69),
    sof2(Type69),
    sof3(Type69),
    sof5(Type69),
    sof6(Type69),
    sof7(Type69),
    sof9(Type69),
    sof10(Type69),
    sof11(Type69),
    sof13(Type69),
    sof14(Type69),
    sof15(Type69),
}

struct Type71 {
    component_selector: u8,
    entropy_coding_table_ids: u8,
}

struct Type72 {
    num_image_components: u8,
    image_components: Vec<Type71>,
    start_spectral_selection: u8,
    end_spectral_selection: u8,
    approximation_bit_position: u8,
}

struct Type73 {
    marker: Type42,
    length: u16,
    data: Type72,
}

enum Type74 {
    mcu(u8),
    rst0(Type42),
    rst1(Type42),
    rst2(Type42),
    rst3(Type42),
    rst4(Type42),
    rst5(Type42),
    rst6(Type42),
    rst7(Type42),
}

struct Type75 {
    scan_data: Vec<Type74>,
    scan_data_stream: Vec<u8>,
}

struct Type76 {
    segments: Vec<Type66>,
    sos: Type73,
    data: Type75,
}

struct Type77 {
    num_lines: u16,
}

struct Type78 {
    marker: Type42,
    length: u16,
    data: Type77,
}

enum Type79 {
    some(Type78),
    none,
}

struct Type80 {
    initial_segment: Type56,
    segments: Vec<Type66>,
    header: Type70,
    scan: Type76,
    dnl: Type79,
    scans: Vec<Type76>,
}

struct Type81 {
    soi: Type42,
    frame: Type80,
    eoi: Type42,
}

struct Type82 {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

struct Type83 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type82,
    crc: u32,
}

struct Type84 {
    palette_index: u8,
}

struct Type85 {
    red: u16,
    green: u16,
    blue: u16,
}

struct Type86 {
    greyscale: u16,
}

enum Type87 {
    color_type_3(Type84),
    color_type_6(Type85),
    color_type_2(Type85),
    color_type_4(Type86),
    color_type_0(Type86),
}

struct Type88 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type87,
    crc: u32,
}

struct Type89 {
    pixels_per_unit_x: u32,
    pixels_per_unit_y: u32,
    unit_specifier: u8,
}

struct Type90 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type89,
    crc: u32,
}

struct Type91 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Vec<Type2>,
    crc: u32,
}

struct Type92 {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

struct Type93 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type92,
    crc: u32,
}

enum Type94 {
    color_type_3(Vec<Type84>),
    color_type_2(Type85),
    color_type_0(Type86),
}

struct Type95 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Type94,
    crc: u32,
}

enum Type96 {
    bKGD(Type88),
    pHYs(Type90),
    PLTE(Type91),
    tIME(Type93),
    tRNS(Type95),
}

struct Type97 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: Vec<u8>,
    crc: u32,
}

struct Type98 {
    length: u32,
    tag: (u8, u8, u8, u8),
    data: (),
    crc: u32,
}

struct Type99 {
    signature: (u8, u8, u8, u8, u8, u8, u8, u8),
    ihdr: Type83,
    chunks: Vec<Type96>,
    idat: Vec<Type97>,
    more_chunks: Vec<Type96>,
    iend: Type98,
}

enum Type100 {
    no(u8),
    yes,
}

struct Type101 {
    tag: (u8, u8, u8, u8),
    length: u32,
    data: Vec<u8>,
    pad: Type100,
}

struct Type102 {
    tag: (u8, u8, u8, u8),
    chunks: Vec<Type101>,
}

struct Type103 {
    tag: (u8, u8, u8, u8),
    length: u32,
    data: Type102,
    pad: Type100,
}

struct Type104 {
    string: Vec<u8>,
    __padding: Vec<u8>,
}

struct Type105 {
    string: Vec<u8>,
    __nul_or_wsp: u8,
    __padding: Vec<u8>,
}

struct Type106 {
    string: Vec<u8>,
    padding: Vec<u8>,
}

struct Type107 {
    name: Type104,
    mode: Type105,
    uid: Type105,
    gid: Type105,
    size: u32,
    mtime: Type105,
    chksum: Type105,
    typeflag: u8,
    linkname: Type104,
    magic: (u8, u8, u8, u8, u8, u8),
    version: (u8, u8),
    uname: Type106,
    gname: Type106,
    devmajor: Type105,
    devminor: Type105,
    prefix: Type104,
    pad: Vec<u8>,
}

struct Type108 {
    header: Type107,
    file: Vec<u8>,
    __padding: (),
}

struct Type109 {
    contents: Vec<Type108>,
    __padding: Vec<u8>,
    __trailing: Vec<u8>,
}

enum Type110 {
    ascii(Vec<u8>),
    utf8(Vec<char>),
}

enum Type111 {
    gif(Type19),
    gzip(Vec<Type41>),
    jpeg(Type81),
    png(Type99),
    riff(Type103),
    tar(Type109),
    text(Type110),
}

struct Type112 {
    data: Type111,
    end: (),
}

fn Decoder0<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type112> {
    (Some((Decoder1(scope, input))?))
}

fn Decoder1<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type112> {
    let data = { (unimplemented!(r#"ParallelLogic::Alts.to_ast(..)"#)) };
    let end = {
        if ((input.read_byte()).is_none()) {
            ()
        } else {
            return None;
        }
    };
    (Some(Type112 { data, end }))
}

fn Decoder2<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type19> {
    let header = { (Decoder128(scope, input))? };
    let logical_screen = { (Decoder129(scope, input))? };
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
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder130(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let trailer = { (Decoder131(scope, input))? };
    (Some(Type19 {
        header,
        logical_screen,
        blocks,
        trailer,
    }))
}

fn Decoder3<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<Type41>> {
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
                let header = { (Decoder118(scope, input))? };
                let fname = {
                    match (header.file_flags & 8 != 0) {
                        true => {
                            let inner = (Decoder119(scope, input))?;
                            (Type22::yes(inner))
                        }

                        false => {
                            let inner = ();
                            (Type22::no(inner))
                        }
                    }
                };
                let data = { (unimplemented!(r#"translate @ Decoder::Bits"#)) };
                let footer = { (Decoder121(scope, input))? };
                Type41 {
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

fn Decoder4<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type81> {
    let soi = { (Decoder45(scope, input))? };
    let frame = { (Decoder46(scope, input))? };
    let eoi = { (Decoder47(scope, input))? };
    (Some(Type81 { soi, frame, eoi }))
}

fn Decoder5<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type99> {
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
                        }
                    }

                    98 => 0,

                    112 => 0,

                    80 => 0,

                    116 => 0,
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
    (Some(Type99 {
        signature,
        ihdr,
        chunks,
        idat,
        more_chunks,
        iend,
    }))
}

fn Decoder6<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type103> {
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
                let inner = ();
                (Type100::yes(inner))
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
                (Type100::no(inner))
            }
        }
    };
    (Some(Type103 {
        tag,
        length,
        data,
        pad,
    }))
}

fn Decoder7<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type109> {
    let contents = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    0 => 0,

                    tmp if (tmp != 0) => 1,
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
    (Some(Type109 {
        contents,
        __padding,
        __trailing,
    }))
}

fn Decoder8<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type110> {
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

                224 => 2,

                tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240])).contains(tmp)) => 2,

                237 => 2,

                tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992])).contains(tmp)) => 2,

                240 => 3,

                tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184])).contains(tmp)) => 3,

                244 => 3,

                tmp if ((ByteSet::from_bits([0, 0, 0, 4294967292])).contains(tmp)) => 1,
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
                    (x1, x0) => {
                        ((x1 as u32) << 6 | (x0 as u32));
                    }
                })(inner))
            }

            2 => {
                let inner = {
                    let tree_index = {
                        let lookahead = &mut (input.clone());
                        let b = (lookahead.read_byte())?;
                        match b {
                            tmp if ((ByteSet::from_bits([0, 0, 0, 211106232532992]))
                                .contains(tmp)) =>
                            {
                                3
                            }

                            237 => 2,

                            224 => 0,

                            tmp if ((ByteSet::from_bits([0, 0, 0, 35175782154240]))
                                .contains(tmp)) =>
                            {
                                1
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
                    }
                };
                ((|bytes| match bytes {
                    (x2, x1, x0) => {
                        ((x2 as u32) << 12 | (x1 as u32) << 6 | (x0 as u32));
                    }
                })(inner))
            }

            3 => {
                let inner = {
                    let tree_index = {
                        let lookahead = &mut (input.clone());
                        let b = (lookahead.read_byte())?;
                        match b {
                            244 => 2,

                            240 => 0,

                            tmp if ((ByteSet::from_bits([0, 0, 0, 3940649673949184]))
                                .contains(tmp)) =>
                            {
                                1
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
                    }
                };
                ((|bytes| match bytes {
                    (x3, x2, x1, x0) => {
                        ((x3 as u32) << 18 | (x2 as u32) << 12 | (x1 as u32) << 6 | (x0 as u32));
                    }
                })(inner))
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

fn Decoder14<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type108> {
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
    (Some(Type108 {
        header,
        file,
        __padding,
    }))
}

fn Decoder15<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type107> {
    (Some((unimplemented!(r#"translate @ Decoder::Slice"#))))
}

fn Decoder16<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(b))
}

fn Decoder17<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type104> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 1,

                    0 => 0,
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
    (Some(Type104 { string, __padding }))
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

fn Decoder21<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type104> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,
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
    (Some(Type104 { string, __padding }))
}

fn Decoder22<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type106> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    0 => 1,

                    tmp if (tmp != 0) => 0,
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
    (Some(Type106 { string, padding }))
}

fn Decoder23<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u32> {
    let inner = {
        let field0 = { (Decoder16(scope, input))? };
        let field1 = { (Decoder16(scope, input))? };
        let field2 = { (Decoder16(scope, input))? };
        let field3 = { (Decoder16(scope, input))? };
        (field0, field1, field2, field3)
    };
    (Some(((|x| u32::from_le_bytes(x))(inner))))
}

fn Decoder24<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type102> {
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
    (Some(Type102 { tag, chunks }))
}

fn Decoder25<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<(u8, u8, u8, u8)> {
    let field0 = { (Decoder20(scope, input))? };
    let field1 = { (Decoder20(scope, input))? };
    let field2 = { (Decoder20(scope, input))? };
    let field3 = { (Decoder20(scope, input))? };
    (Some((field0, field1, field2, field3)))
}

fn Decoder26<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type101> {
    let tag = { (Decoder25(scope, input))? };
    let length = { (Decoder23(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let pad = {
        match (length % 2 == 0) {
            true => {
                let inner = ();
                (Type100::yes(inner))
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
                (Type100::no(inner))
            }
        }
    };
    (Some(Type101 {
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

fn Decoder28<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type83> {
    let length = { (Decoder32(scope, input))? };
    let tag = { (Decoder43(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder32(scope, input))? };
    (Some(Type83 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder29<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type96> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        match b {
            80 => 2,

            112 => 1,

            98 => 0,

            116 => {
                let b = (lookahead.read_byte())?;
                match b {
                    73 => 3,

                    82 => 4,
                }
            }
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder37(scope, input))?;
            (Type96::bKGD(inner))
        }

        1 => {
            let inner = (Decoder38(scope, input))?;
            (Type96::pHYs(inner))
        }

        2 => {
            let inner = (Decoder39(scope, input))?;
            (Type96::PLTE(inner))
        }

        3 => {
            let inner = (Decoder40(scope, input))?;
            (Type96::tIME(inner))
        }

        4 => {
            let inner = (Decoder41(scope, input))?;
            (Type96::tRNS(inner))
        }
    }))
}

fn Decoder30<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type97> {
    let length = { (Decoder32(scope, input))? };
    let tag = { (Decoder35(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder32(scope, input))? };
    (Some(Type97 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder31<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type98> {
    let length = { (Decoder32(scope, input))? };
    let tag = { (Decoder33(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    let crc = { (Decoder32(scope, input))? };
    (Some(Type98 {
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
    (Some(((|x| u32::from_be_bytes(x))(inner))))
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

fn Decoder37<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type88> {
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
    (Some(Type88 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder38<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type90> {
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
    (Some(Type90 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder39<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type91> {
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
    (Some(Type91 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder40<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type93> {
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
    (Some(Type93 {
        length,
        tag,
        data,
        crc,
    }))
}

fn Decoder41<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type95> {
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
    (Some(Type95 {
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
    (Some(((|x| u16::from_be_bytes(x))(inner))))
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

fn Decoder44<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type82> {
    let width = { (Decoder32(scope, input))? };
    let height = { (Decoder32(scope, input))? };
    let bit_depth = { (Decoder16(scope, input))? };
    let color_type = { (Decoder16(scope, input))? };
    let compression_method = { (Decoder16(scope, input))? };
    let filter_method = { (Decoder16(scope, input))? };
    let interlace_method = { (Decoder16(scope, input))? };
    (Some(Type82 {
        width,
        height,
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
    }))
}

fn Decoder45<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
    (Some(Type42 { ff, marker }))
}

fn Decoder46<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type80> {
    let initial_segment = {
        let tree_index = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            if (b == 255) {
                let b = (lookahead.read_byte())?;
                match b {
                    224 => 0,

                    225 => 1,
                }
            } else {
                return None;
            }
        };
        match tree_index {
            0 => {
                let inner = (Decoder48(scope, input))?;
                (Type56::app0(inner))
            }

            1 => {
                let inner = (Decoder49(scope, input))?;
                (Type56::app1(inner))
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
                    }
                } else {
                    return None;
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder50(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let header = { (Decoder51(scope, input))? };
    let scan = { (Decoder52(scope, input))? };
    let dnl = {
        let tree_index = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            if (b == 255) {
                let b = (lookahead.read_byte())?;
                match b {
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

                    220 => 0,
                }
            } else {
                return None;
            }
        };
        match tree_index {
            0 => {
                let inner = (Decoder53(scope, input))?;
                (Type79::some(inner))
            }

            1 => {
                let inner = ();
                (Type79::none(inner))
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
                    }
                } else {
                    return None;
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder54(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    (Some(Type80 {
        initial_segment,
        segments,
        header,
        scan,
        dnl,
        scans,
    }))
}

fn Decoder47<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
    (Some(Type42 { ff, marker }))
}

fn Decoder48<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type46> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type46 {
        marker,
        length,
        data,
    }))
}

fn Decoder49<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type55> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type55 {
        marker,
        length,
        data,
    }))
}

fn Decoder50<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type66> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        if (b == 255) {
            let b = (lookahead.read_byte())?;
            match b {
                227 => 7,

                224 => 4,

                239 => 19,

                228 => 8,

                196 => 1,

                236 => 16,

                235 => 15,

                204 => 2,

                234 => 14,

                225 => 5,

                233 => 13,

                221 => 3,

                231 => 11,

                238 => 18,

                229 => 9,

                219 => 0,

                230 => 10,

                226 => 6,

                254 => 20,

                237 => 17,

                232 => 12,
            }
        } else {
            return None;
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder85(scope, input))?;
            (Type66::dqt(inner))
        }

        1 => {
            let inner = (Decoder86(scope, input))?;
            (Type66::dht(inner))
        }

        2 => {
            let inner = (Decoder87(scope, input))?;
            (Type66::dac(inner))
        }

        3 => {
            let inner = (Decoder88(scope, input))?;
            (Type66::dri(inner))
        }

        4 => {
            let inner = (Decoder48(scope, input))?;
            (Type66::app0(inner))
        }

        5 => {
            let inner = (Decoder49(scope, input))?;
            (Type66::app1(inner))
        }

        6 => {
            let inner = (Decoder89(scope, input))?;
            (Type66::app2(inner))
        }

        7 => {
            let inner = (Decoder90(scope, input))?;
            (Type66::app3(inner))
        }

        8 => {
            let inner = (Decoder91(scope, input))?;
            (Type66::app4(inner))
        }

        9 => {
            let inner = (Decoder92(scope, input))?;
            (Type66::app5(inner))
        }

        10 => {
            let inner = (Decoder93(scope, input))?;
            (Type66::app6(inner))
        }

        11 => {
            let inner = (Decoder94(scope, input))?;
            (Type66::app7(inner))
        }

        12 => {
            let inner = (Decoder95(scope, input))?;
            (Type66::app8(inner))
        }

        13 => {
            let inner = (Decoder96(scope, input))?;
            (Type66::app9(inner))
        }

        14 => {
            let inner = (Decoder97(scope, input))?;
            (Type66::app10(inner))
        }

        15 => {
            let inner = (Decoder98(scope, input))?;
            (Type66::app11(inner))
        }

        16 => {
            let inner = (Decoder99(scope, input))?;
            (Type66::app12(inner))
        }

        17 => {
            let inner = (Decoder100(scope, input))?;
            (Type66::app13(inner))
        }

        18 => {
            let inner = (Decoder101(scope, input))?;
            (Type66::app14(inner))
        }

        19 => {
            let inner = (Decoder102(scope, input))?;
            (Type66::app15(inner))
        }

        20 => {
            let inner = (Decoder103(scope, input))?;
            (Type66::com(inner))
        }
    }))
}

fn Decoder51<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type70> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        if (b == 255) {
            let b = (lookahead.read_byte())?;
            match b {
                198 => 5,

                193 => 1,

                202 => 8,

                205 => 10,

                197 => 4,

                192 => 0,

                201 => 7,

                207 => 12,

                203 => 9,

                195 => 3,

                194 => 2,

                206 => 11,

                199 => 6,
            }
        } else {
            return None;
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder70(scope, input))?;
            (Type70::sof0(inner))
        }

        1 => {
            let inner = (Decoder71(scope, input))?;
            (Type70::sof1(inner))
        }

        2 => {
            let inner = (Decoder72(scope, input))?;
            (Type70::sof2(inner))
        }

        3 => {
            let inner = (Decoder73(scope, input))?;
            (Type70::sof3(inner))
        }

        4 => {
            let inner = (Decoder74(scope, input))?;
            (Type70::sof5(inner))
        }

        5 => {
            let inner = (Decoder75(scope, input))?;
            (Type70::sof6(inner))
        }

        6 => {
            let inner = (Decoder76(scope, input))?;
            (Type70::sof7(inner))
        }

        7 => {
            let inner = (Decoder77(scope, input))?;
            (Type70::sof9(inner))
        }

        8 => {
            let inner = (Decoder78(scope, input))?;
            (Type70::sof10(inner))
        }

        9 => {
            let inner = (Decoder79(scope, input))?;
            (Type70::sof11(inner))
        }

        10 => {
            let inner = (Decoder80(scope, input))?;
            (Type70::sof13(inner))
        }

        11 => {
            let inner = (Decoder81(scope, input))?;
            (Type70::sof14(inner))
        }

        12 => {
            let inner = (Decoder82(scope, input))?;
            (Type70::sof15(inner))
        }
    }))
}

fn Decoder52<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type76> {
    let segments = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                if (b == 255) {
                    let b = (lookahead.read_byte())?;
                    match b {
                        218 => 1,

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
                    }
                } else {
                    return None;
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder50(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let sos = { (Decoder55(scope, input))? };
    let data = { (Decoder69(scope, input))? };
    (Some(Type76 {
        segments,
        sos,
        data,
    }))
}

fn Decoder53<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type78> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type78 {
        marker,
        length,
        data,
    }))
}

fn Decoder54<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type76> {
    let segments = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                if (b == 255) {
                    let b = (lookahead.read_byte())?;
                    match b {
                        218 => 1,

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
                    }
                } else {
                    return None;
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder50(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let sos = { (Decoder55(scope, input))? };
    let data = { (Decoder56(scope, input))? };
    (Some(Type76 {
        segments,
        sos,
        data,
    }))
}

fn Decoder55<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type73> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type73 {
        marker,
        length,
        data,
    }))
}

fn Decoder56<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type75> {
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
                        }
                    }
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let tree_index = {
                        let lookahead = &mut (input.clone());
                        let b = (lookahead.read_byte())?;
                        match b {
                            255 => {
                                let b = (lookahead.read_byte())?;
                                match b {
                                    215 => 8,

                                    214 => 7,

                                    211 => 4,

                                    210 => 3,

                                    0 => 0,

                                    213 => 6,

                                    212 => 5,

                                    209 => 2,

                                    208 => 1,
                                }
                            }

                            tmp if (tmp != 255) => 0,
                        }
                    };
                    match tree_index {
                        0 => {
                            let inner = (Decoder57(scope, input))?;
                            (Type74::mcu(inner))
                        }

                        1 => {
                            let inner = (Decoder58(scope, input))?;
                            (Type74::rst0(inner))
                        }

                        2 => {
                            let inner = (Decoder59(scope, input))?;
                            (Type74::rst1(inner))
                        }

                        3 => {
                            let inner = (Decoder60(scope, input))?;
                            (Type74::rst2(inner))
                        }

                        4 => {
                            let inner = (Decoder61(scope, input))?;
                            (Type74::rst3(inner))
                        }

                        5 => {
                            let inner = (Decoder62(scope, input))?;
                            (Type74::rst4(inner))
                        }

                        6 => {
                            let inner = (Decoder63(scope, input))?;
                            (Type74::rst5(inner))
                        }

                        7 => {
                            let inner = (Decoder64(scope, input))?;
                            (Type74::rst6(inner))
                        }

                        8 => {
                            let inner = (Decoder65(scope, input))?;
                            (Type74::rst7(inner))
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
        (((scan_data.into_iter()).flat_map(
            (|x| match x {
                mcu(v) => {
                    ([v].to_vec());
                }

                rst0(_) => {
                    ([].to_vec());
                }

                rst1(_) => {
                    ([].to_vec());
                }

                rst2(_) => {
                    ([].to_vec());
                }

                rst3(_) => {
                    ([].to_vec());
                }

                rst4(_) => {
                    ([].to_vec());
                }

                rst5(_) => {
                    ([].to_vec());
                }

                rst6(_) => {
                    ([].to_vec());
                }

                rst7(_) => {
                    ([].to_vec());
                }
            }),
        ))
        .collect())
    };
    (Some(Type75 {
        scan_data,
        scan_data_stream,
    }))
}

fn Decoder57<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        match b {
            255 => 1,

            tmp if (tmp != 255) => 0,
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
    }))
}

fn Decoder58<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
    (Some(Type42 { ff, marker }))
}

fn Decoder59<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
    (Some(Type42 { ff, marker }))
}

fn Decoder60<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
    (Some(Type42 { ff, marker }))
}

fn Decoder61<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
    (Some(Type42 { ff, marker }))
}

fn Decoder62<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
    (Some(Type42 { ff, marker }))
}

fn Decoder63<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
    (Some(Type42 { ff, marker }))
}

fn Decoder64<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
    (Some(Type42 { ff, marker }))
}

fn Decoder65<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type42> {
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
    (Some(Type42 { ff, marker }))
}

fn Decoder66<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type72> {
    let num_image_components = { (Decoder16(scope, input))? };
    let image_components = {
        let mut accum = (Vec::new());
        for _ in 0..num_image_components {
            (accum.push((Decoder67(scope, input))?));
        }
        accum
    };
    let start_spectral_selection = { (Decoder16(scope, input))? };
    let end_spectral_selection = { (Decoder16(scope, input))? };
    let approximation_bit_position = { (Decoder16(scope, input))? };
    (Some(Type72 {
        num_image_components,
        image_components,
        start_spectral_selection,
        end_spectral_selection,
        approximation_bit_position,
    }))
}

fn Decoder67<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type71> {
    let component_selector = { (Decoder16(scope, input))? };
    let entropy_coding_table_ids = { (Decoder16(scope, input))? };
    (Some(Type71 {
        component_selector,
        entropy_coding_table_ids,
    }))
}

fn Decoder68<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type77> {
    let num_lines = { (Decoder42(scope, input))? };
    (Some(Type77 { num_lines }))
}

fn Decoder69<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type75> {
    let scan_data = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
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
                        }
                    }

                    tmp if (tmp != 255) => 0,
                }
            };
            if (matching_ix == 0) {
                let next_elem = {
                    let tree_index = {
                        let lookahead = &mut (input.clone());
                        let b = (lookahead.read_byte())?;
                        match b {
                            255 => {
                                let b = (lookahead.read_byte())?;
                                match b {
                                    215 => 8,

                                    213 => 6,

                                    214 => 7,

                                    208 => 1,

                                    210 => 3,

                                    209 => 2,

                                    0 => 0,

                                    212 => 5,

                                    211 => 4,
                                }
                            }

                            tmp if (tmp != 255) => 0,
                        }
                    };
                    match tree_index {
                        0 => {
                            let inner = (Decoder57(scope, input))?;
                            (Type74::mcu(inner))
                        }

                        1 => {
                            let inner = (Decoder58(scope, input))?;
                            (Type74::rst0(inner))
                        }

                        2 => {
                            let inner = (Decoder59(scope, input))?;
                            (Type74::rst1(inner))
                        }

                        3 => {
                            let inner = (Decoder60(scope, input))?;
                            (Type74::rst2(inner))
                        }

                        4 => {
                            let inner = (Decoder61(scope, input))?;
                            (Type74::rst3(inner))
                        }

                        5 => {
                            let inner = (Decoder62(scope, input))?;
                            (Type74::rst4(inner))
                        }

                        6 => {
                            let inner = (Decoder63(scope, input))?;
                            (Type74::rst5(inner))
                        }

                        7 => {
                            let inner = (Decoder64(scope, input))?;
                            (Type74::rst6(inner))
                        }

                        8 => {
                            let inner = (Decoder65(scope, input))?;
                            (Type74::rst7(inner))
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
        (((scan_data.into_iter()).flat_map(
            (|x| match x {
                mcu(v) => {
                    ([v].to_vec());
                }

                rst0(_) => {
                    ([].to_vec());
                }

                rst1(_) => {
                    ([].to_vec());
                }

                rst2(_) => {
                    ([].to_vec());
                }

                rst3(_) => {
                    ([].to_vec());
                }

                rst4(_) => {
                    ([].to_vec());
                }

                rst5(_) => {
                    ([].to_vec());
                }

                rst6(_) => {
                    ([].to_vec());
                }

                rst7(_) => {
                    ([].to_vec());
                }
            }),
        ))
        .collect())
    };
    (Some(Type75 {
        scan_data,
        scan_data_stream,
    }))
}

fn Decoder70<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder71<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder72<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder73<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder74<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder75<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder76<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder77<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder78<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder79<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder80<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder81<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder82<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type69> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type69 {
        marker,
        length,
        data,
    }))
}

fn Decoder83<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type68> {
    let sample_precision = { (Decoder16(scope, input))? };
    let num_lines = { (Decoder42(scope, input))? };
    let num_samples_per_line = { (Decoder42(scope, input))? };
    let num_image_components = { (Decoder16(scope, input))? };
    let image_components = {
        let mut accum = (Vec::new());
        for _ in 0..num_image_components {
            (accum.push((Decoder84(scope, input))?));
        }
        accum
    };
    (Some(Type68 {
        sample_precision,
        num_lines,
        num_samples_per_line,
        num_image_components,
        image_components,
    }))
}

fn Decoder84<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type67> {
    let id = { (Decoder16(scope, input))? };
    let sampling_factor = { (Decoder16(scope, input))? };
    let quantization_table_id = { (Decoder16(scope, input))? };
    (Some(Type67 {
        id,
        sampling_factor,
        quantization_table_id,
    }))
}

fn Decoder85<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type58> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type58 {
        marker,
        length,
        data,
    }))
}

fn Decoder86<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type60> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type60 {
        marker,
        length,
        data,
    }))
}

fn Decoder87<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type62> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type62 {
        marker,
        length,
        data,
    }))
}

fn Decoder88<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type64> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type64 {
        marker,
        length,
        data,
    }))
}

fn Decoder89<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder90<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder91<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder92<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder93<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder94<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder95<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder96<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder97<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder98<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder99<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder100<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder101<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder102<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder103<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type65> {
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
        Type42 { ff, marker }
    };
    let length = { (Decoder42(scope, input))? };
    let data = { (unimplemented!(r#"translate @ Decoder::Slice"#)) };
    (Some(Type65 {
        marker,
        length,
        data,
    }))
}

fn Decoder104<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type63> {
    let restart_interval = { (Decoder42(scope, input))? };
    (Some(Type63 { restart_interval }))
}

fn Decoder105<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type61> {
    let class_table_id = { (Decoder16(scope, input))? };
    let value = { (Decoder16(scope, input))? };
    (Some(Type61 {
        class_table_id,
        value,
    }))
}

fn Decoder106<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type59> {
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
    (Some(Type59 {
        class_table_id,
        num_codes,
        values,
    }))
}

fn Decoder107<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type57> {
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
    (Some(Type57 {
        precision_table_id,
        elements,
    }))
}

fn Decoder108<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type54> {
    let identifier = { (Decoder109(scope, input))? };
    let data = {
        match identifier.string {
            [69, 120, 105, 102] => {
                let inner = (Decoder110(scope, input))?;
                (Type53::exif(inner))
            }

            [104, 116, 116, 112, 58, 47, 47, 110, 115, 46, 97, 100, 111, 98, 101, 46, 99, 111, 109, 47, 120, 97, 112, 47, 49, 46, 48, 47] =>
            {
                let inner = (Decoder111(scope, input))?;
                (Type53::xmp(inner))
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
                (Type53::other(inner))
            }
        }
    };
    (Some(Type54 { identifier, data }))
}

fn Decoder109<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    0 => 1,

                    tmp if (tmp != 0) => 0,
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

fn Decoder110<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type52> {
    let padding = {
        let b = (input.read_byte())?;
        if (b == 0) {
            b
        } else {
            return None;
        }
    };
    let exif = { (Decoder112(scope, input))? };
    (Some(Type52 { padding, exif }))
}

fn Decoder111<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type47> {
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
    (Some(Type47 { xmp }))
}

fn Decoder112<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type51> {
    let byte_order = {
        let tree_index = {
            let lookahead = &mut (input.clone());
            let b = (lookahead.read_byte())?;
            match b {
                77 => 1,

                73 => 0,
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
                (Type48::le(field0, field1))
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
                (Type48::be(field0, field1))
            }
        }
    };
    let magic = {
        match byte_order {
            le(_) => (Decoder113(scope, input))?,

            be(_) => (Decoder42(scope, input))?,
        }
    };
    let offset = {
        match byte_order {
            le(_) => (Decoder23(scope, input))?,

            be(_) => (Decoder32(scope, input))?,
        }
    };
    let ifd = { (unimplemented!(r#"translate @ Decoder::WithRelativeOffset"#)) };
    (Some(Type51 {
        byte_order,
        magic,
        offset,
        ifd,
    }))
}

fn Decoder113<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u16> {
    let inner = {
        let field0 = { (Decoder16(scope, input))? };
        let field1 = { (Decoder16(scope, input))? };
        (field0, field1)
    };
    (Some(((|x| u16::from_le_bytes(x))(inner))))
}

fn Decoder114<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type45> {
    let identifier = { (Decoder115(scope, input))? };
    let data = {
        match identifier.string {
            [74, 70, 73, 70] => {
                let inner = (Decoder116(scope, input))?;
                (Type44::jfif(inner))
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
                (Type44::other(inner))
            }
        }
    };
    (Some(Type45 { identifier, data }))
}

fn Decoder115<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    0 => 1,

                    tmp if (tmp != 0) => 0,
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

fn Decoder116<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type43> {
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
                    (accum.push((Decoder117(scope, input))?));
                }
                accum
            }));
        }
        accum
    };
    (Some(Type43 {
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

fn Decoder117<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type2> {
    let r = { (Decoder16(scope, input))? };
    let g = { (Decoder16(scope, input))? };
    let b = { (Decoder16(scope, input))? };
    (Some(Type2 { r, g, b }))
}

fn Decoder118<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type20> {
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
    let method = { (Decoder16(scope, input))? };
    let file_flags = { (Decoder16(scope, input))? };
    let timestamp = { (Decoder23(scope, input))? };
    let compression_flags = { (Decoder16(scope, input))? };
    let os_id = { (Decoder16(scope, input))? };
    (Some(Type20 {
        magic,
        method,
        file_flags,
        timestamp,
        compression_flags,
        os_id,
    }))
}

fn Decoder119<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    (Some((Decoder127(scope, input))?))
}

fn Decoder120<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type39> {
    let blocks = {
        let mut accum = (Vec::new());
        while true {
            let elem = (Decoder122(scope, input))?;
            if (((|x| x.r#final == 1)())(&elem)) {
                (accum.push(elem));
                break;
            } else {
                (accum.push(elem));
            }
        }
        accum
    };
    let codes = {
        (((blocks.into_iter()).flat_map(
            (|x| match x.data {
                uncompressed(y) => {
                    y.codes_values;
                }

                fixed_huffman(y) => {
                    y.codes_values;
                }

                dynamic_huffman(y) => {
                    y.codes_values;
                }
            }),
        ))
        .collect())
    };
    let inflate = { (unimplemented!(r#"embed_expr is not implemented for Expr::Inflate"#)) };
    (Some(Type39 {
        blocks,
        codes,
        inflate,
    }))
}

fn Decoder121<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type40> {
    let crc = { (Decoder23(scope, input))? };
    let length = { (Decoder23(scope, input))? };
    (Some(Type40 { crc, length }))
}

fn Decoder122<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type38> {
    let r#final = { (Decoder123(scope, input))? };
    let r#type = {
        let inner = {
            let field0 = { (Decoder123(scope, input))? };
            let field1 = { (Decoder123(scope, input))? };
            (field0, field1)
        };
        ((|bits| bits.1 << 1 | bits.0)(inner))
    };
    let data = {
        match r#type {
            0 => {
                let inner = (Decoder124(scope, input))?;
                (Type37::uncompressed(inner))
            }

            1 => {
                let inner = (Decoder125(scope, input))?;
                (Type37::fixed_huffman(inner))
            }

            2 => {
                let inner = (Decoder126(scope, input))?;
                (Type37::dynamic_huffman(inner))
            }
        }
    };
    (Some(Type38 {
        r#final,
        r#type,
        data,
    }))
}

fn Decoder123<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(b))
}

fn Decoder124<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type36> {
    let align = {
        while (input.offset() % 8 != 0) {
            let _ = (input.read_byte())?;
        }
        ()
    };
    let len = {
        let inner = {
            let field0 = { (Decoder123(scope, input))? };
            let field1 = { (Decoder123(scope, input))? };
            let field2 = { (Decoder123(scope, input))? };
            let field3 = { (Decoder123(scope, input))? };
            let field4 = { (Decoder123(scope, input))? };
            let field5 = { (Decoder123(scope, input))? };
            let field6 = { (Decoder123(scope, input))? };
            let field7 = { (Decoder123(scope, input))? };
            let field8 = { (Decoder123(scope, input))? };
            let field9 = { (Decoder123(scope, input))? };
            let field10 = { (Decoder123(scope, input))? };
            let field11 = { (Decoder123(scope, input))? };
            let field12 = { (Decoder123(scope, input))? };
            let field13 = { (Decoder123(scope, input))? };
            let field14 = { (Decoder123(scope, input))? };
            let field15 = { (Decoder123(scope, input))? };
            (
                field0, field1, field2, field3, field4, field5, field6, field7, field8, field9,
                field10, field11, field12, field13, field14, field15,
            )
        };
        ((|bits| {
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
            let field0 = { (Decoder123(scope, input))? };
            let field1 = { (Decoder123(scope, input))? };
            let field2 = { (Decoder123(scope, input))? };
            let field3 = { (Decoder123(scope, input))? };
            let field4 = { (Decoder123(scope, input))? };
            let field5 = { (Decoder123(scope, input))? };
            let field6 = { (Decoder123(scope, input))? };
            let field7 = { (Decoder123(scope, input))? };
            let field8 = { (Decoder123(scope, input))? };
            let field9 = { (Decoder123(scope, input))? };
            let field10 = { (Decoder123(scope, input))? };
            let field11 = { (Decoder123(scope, input))? };
            let field12 = { (Decoder123(scope, input))? };
            let field13 = { (Decoder123(scope, input))? };
            let field14 = { (Decoder123(scope, input))? };
            let field15 = { (Decoder123(scope, input))? };
            (
                field0, field1, field2, field3, field4, field5, field6, field7, field8, field9,
                field10, field11, field12, field13, field14, field15,
            )
        };
        ((|bits| {
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
                    let field0 = { (Decoder123(scope, input))? };
                    let field1 = { (Decoder123(scope, input))? };
                    let field2 = { (Decoder123(scope, input))? };
                    let field3 = { (Decoder123(scope, input))? };
                    let field4 = { (Decoder123(scope, input))? };
                    let field5 = { (Decoder123(scope, input))? };
                    let field6 = { (Decoder123(scope, input))? };
                    let field7 = { (Decoder123(scope, input))? };
                    (
                        field0, field1, field2, field3, field4, field5, field6, field7,
                    )
                };
                ((|bits| {
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
    let codes_values =
        { (((bytes.into_iter()).flat_map((|x| [(literal(x))].to_vec()))).collect()) };
    (Some(Type36 {
        align,
        len,
        nlen,
        bytes,
        codes_values,
    }))
}

fn Decoder125<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type34> {
    let codes = { (unimplemented!(r#"translate @ Decoder::Dynamic"#)) };
    let codes_values = {
        (((codes.into_iter()).flat_map(
            (|x| match x.code {
                256 => {
                    ([].to_vec());
                }

                257 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                258 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                259 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                260 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                261 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                262 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                263 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                264 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                265 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                266 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                267 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                268 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                269 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                270 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                271 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                272 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                273 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                274 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                275 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                276 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                277 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                278 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                279 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                280 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                281 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                282 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                283 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                284 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                285 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                _ => {
                    ([(literal((x.code as u8)))].to_vec());
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

fn Decoder126<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type30> {
    let hlit = {
        let inner = {
            let field0 = { (Decoder123(scope, input))? };
            let field1 = { (Decoder123(scope, input))? };
            let field2 = { (Decoder123(scope, input))? };
            let field3 = { (Decoder123(scope, input))? };
            let field4 = { (Decoder123(scope, input))? };
            (field0, field1, field2, field3, field4)
        };
        ((|bits| bits.4 << 4 | bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0)(inner))
    };
    let hdist = {
        let inner = {
            let field0 = { (Decoder123(scope, input))? };
            let field1 = { (Decoder123(scope, input))? };
            let field2 = { (Decoder123(scope, input))? };
            let field3 = { (Decoder123(scope, input))? };
            let field4 = { (Decoder123(scope, input))? };
            (field0, field1, field2, field3, field4)
        };
        ((|bits| bits.4 << 4 | bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0)(inner))
    };
    let hclen = {
        let inner = {
            let field0 = { (Decoder123(scope, input))? };
            let field1 = { (Decoder123(scope, input))? };
            let field2 = { (Decoder123(scope, input))? };
            let field3 = { (Decoder123(scope, input))? };
            (field0, field1, field2, field3)
        };
        ((|bits| bits.3 << 3 | bits.2 << 2 | bits.1 << 1 | bits.0)(inner))
    };
    let code_length_alphabet_code_lengths = {
        let mut accum = (Vec::new());
        for _ in 0..(hclen + 4) {
            (accum.push({
                let inner = {
                    let field0 = { (Decoder123(scope, input))? };
                    let field1 = { (Decoder123(scope, input))? };
                    let field2 = { (Decoder123(scope, input))? };
                    (field0, field1, field2)
                };
                ((|bits| bits.2 << 2 | bits.1 << 1 | bits.0)(inner))
            }));
        }
        accum
    };
    let literal_length_distance_alphabet_code_lengths =
        { (unimplemented!(r#"translate @ Decoder::Dynamic"#)) };
    let literal_length_distance_alphabet_code_lengths_value = {
        (((literal_length_distance_alphabet_code_lengths.into_iter()).fold(
            (none(())),
            (|x| match (x.1.code as u8) {
                16 => {
                    (
                        x.0,
                        (Vec::from_iter(
                            ((std::iter::repeat(match x.0 {
                                some(y) => {
                                    y;
                                }
                            }))
                            .take((x.1.extra + 3))),
                        )),
                    );
                }

                17 => {
                    (
                        x.0,
                        (Vec::from_iter(((std::iter::repeat(0)).take((x.1.extra + 3))))),
                    );
                }

                18 => {
                    (
                        x.0,
                        (Vec::from_iter(((std::iter::repeat(0)).take((x.1.extra + 11))))),
                    );
                }

                v => {
                    ((some(v)), ([v].to_vec()));
                }
            }),
        ))
        .collect())
    };
    let literal_length_alphabet_code_lengths_value = {
        {
            let ix = 0;
            literal_length_distance_alphabet_code_lengths_value[ix..(ix + (hlit as u16) + 257)]
        }
    };
    let distance_alphabet_code_lengths_value = {
        {
            let ix = ((hlit as u16) + 257);
            literal_length_distance_alphabet_code_lengths_value[ix..(ix + (hdist as u16) + 1)]
        }
    };
    let codes = { (unimplemented!(r#"translate @ Decoder::Dynamic"#)) };
    let codes_values = {
        (((codes.into_iter()).flat_map(
            (|x| match x.code {
                256 => {
                    ([].to_vec());
                }

                257 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                258 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                259 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                260 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                261 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                262 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                263 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                264 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                265 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                266 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                267 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                268 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                269 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                270 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                271 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                272 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                273 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                274 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                275 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                276 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                277 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                278 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                279 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                280 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                281 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                282 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                283 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                284 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                285 => {
                    match x.extra {
                        some(rec) => {
                            ([reference {
                                length: rec.length,
                                distance: rec.distance_record.distance,
                            }]
                            .to_vec());
                        }
                    };
                }

                _ => {
                    ([(literal((x.code as u8)))].to_vec());
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

fn Decoder127<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type21> {
    let string = {
        let mut accum = (Vec::new());
        while true {
            let matching_ix = {
                let lookahead = &mut (input.clone());
                let b = (lookahead.read_byte())?;
                match b {
                    tmp if (tmp != 0) => 0,

                    0 => 1,
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

fn Decoder128<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type0> {
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

fn Decoder129<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type4> {
    let descriptor = { (Decoder145(scope, input))? };
    let global_color_table = {
        match (descriptor.flags & 128 != 0) {
            true => {
                let inner = {
                    let mut accum = (Vec::new());
                    for _ in 0..(2 << (descriptor.flags & 7)) {
                        (accum.push((Decoder143(scope, input))?));
                    }
                    accum
                };
                (Type3::yes(inner))
            }

            false => {
                let inner = ();
                (Type3::no(inner))
            }
        }
    };
    (Some(Type4 {
        descriptor,
        global_color_table,
    }))
}

fn Decoder130<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type17> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        match b {
            33 => {
                let b = (lookahead.read_byte())?;
                match b {
                    255 => 1,

                    254 => 1,

                    249 => 0,

                    1 => 0,
                }
            }

            44 => 0,
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder132(scope, input))?;
            (Type17::graphic_block(inner))
        }

        1 => {
            let inner = (Decoder133(scope, input))?;
            (Type17::special_purpose_block(inner))
        }
    }))
}

fn Decoder131<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type18> {
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

fn Decoder132<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type13> {
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
                    }
                }

                44 => 1,
            }
        };
        match tree_index {
            0 => {
                let inner = (Decoder138(scope, input))?;
                (Type6::some(inner))
            }

            1 => {
                let inner = ();
                (Type6::none(inner))
            }
        }
    };
    let graphic_rendering_block = { (Decoder139(scope, input))? };
    (Some(Type13 {
        graphic_control_extension,
        graphic_rendering_block,
    }))
}

fn Decoder133<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type16> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        if (b == 33) {
            let b = (lookahead.read_byte())?;
            match b {
                254 => 1,

                255 => 0,
            }
        } else {
            return None;
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder134(scope, input))?;
            (Type16::application_extension(inner))
        }

        1 => {
            let inner = (Decoder135(scope, input))?;
            (Type16::comment_extension(inner))
        }
    }))
}

fn Decoder134<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type14> {
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
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder136(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder137(scope, input))? };
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

fn Decoder135<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type15> {
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
                    0 => 1,

                    tmp if (tmp != 0) => 0,
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder136(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder137(scope, input))? };
    (Some(Type15 {
        separator,
        label,
        comment_data,
        terminator,
    }))
}

fn Decoder136<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type8> {
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
    (Some(Type8 { len_bytes, data }))
}

fn Decoder137<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = (input.read_byte())?;
    (Some(if (b == 0) {
        b
    } else {
        return None;
    }))
}

fn Decoder138<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type5> {
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
    let delay_time = { (Decoder113(scope, input))? };
    let transparent_color_index = { (Decoder16(scope, input))? };
    let terminator = { (Decoder137(scope, input))? };
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

fn Decoder139<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type12> {
    let tree_index = {
        let lookahead = &mut (input.clone());
        let b = (lookahead.read_byte())?;
        match b {
            44 => 0,

            33 => 1,
        }
    };
    (Some(match tree_index {
        0 => {
            let inner = (Decoder140(scope, input))?;
            (Type12::table_based_image(inner))
        }

        1 => {
            let inner = (Decoder141(scope, input))?;
            (Type12::plain_text_extension(inner))
        }
    }))
}

fn Decoder140<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type10> {
    let descriptor = { (Decoder142(scope, input))? };
    let local_color_table = {
        match (descriptor.flags & 128 != 0) {
            true => {
                let inner = {
                    let mut accum = (Vec::new());
                    for _ in 0..(2 << (descriptor.flags & 7)) {
                        (accum.push((Decoder143(scope, input))?));
                    }
                    accum
                };
                (Type3::yes(inner))
            }

            false => {
                let inner = ();
                (Type3::no(inner))
            }
        }
    };
    let data = { (Decoder144(scope, input))? };
    (Some(Type10 {
        descriptor,
        local_color_table,
        data,
    }))
}

fn Decoder141<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type11> {
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
    let text_grid_left_position = { (Decoder113(scope, input))? };
    let text_grid_top_position = { (Decoder113(scope, input))? };
    let text_grid_width = { (Decoder113(scope, input))? };
    let text_grid_height = { (Decoder113(scope, input))? };
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
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder136(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder137(scope, input))? };
    (Some(Type11 {
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

fn Decoder142<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type7> {
    let separator = {
        let b = (input.read_byte())?;
        if (b == 44) {
            b
        } else {
            return None;
        }
    };
    let image_left_position = { (Decoder113(scope, input))? };
    let image_top_position = { (Decoder113(scope, input))? };
    let image_width = { (Decoder113(scope, input))? };
    let image_height = { (Decoder113(scope, input))? };
    let flags = { (Decoder16(scope, input))? };
    (Some(Type7 {
        separator,
        image_left_position,
        image_top_position,
        image_width,
        image_height,
        flags,
    }))
}

fn Decoder143<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type2> {
    let r = { (Decoder16(scope, input))? };
    let g = { (Decoder16(scope, input))? };
    let b = { (Decoder16(scope, input))? };
    (Some(Type2 { r, g, b }))
}

fn Decoder144<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type9> {
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
                }
            };
            if (matching_ix == 0) {
                let next_elem = (Decoder136(scope, input))?;
                (accum.push(next_elem));
            } else {
                break;
            }
        }
        accum
    };
    let terminator = { (Decoder137(scope, input))? };
    (Some(Type9 {
        lzw_min_code_size,
        image_data,
        terminator,
    }))
}

fn Decoder145<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type1> {
    let screen_width = { (Decoder113(scope, input))? };
    let screen_height = { (Decoder113(scope, input))? };
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
