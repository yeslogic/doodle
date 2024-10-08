use super::*;

pub type TestResult<T = ()> = Result<T, Box<dyn Send + Sync + std::error::Error>>;

// Stabilization aliases to avoid hard-coding shifting numbers as formats are enriched with more possibilities
pub type Top = main_data;
pub type OpentypeData = opentype_main;
pub type TarBlock = tar_header_with_data;
pub type PngData = png_main;
pub type JpegData = jpeg_main;
pub type JpegApp01 = jpeg_frame_initial_segment;
pub type JfifData = jpeg_app0_jfif;
pub type TiffData = tiff_main;
pub type App0Data = jpeg_app0_data_data;
pub type App1Data = jpeg_app1_data_data;
pub type ExifData = jpeg_app1_exif;
pub type XmpData = jpeg_app1_xmp;
pub type GifData = gif_main;
pub type GifLogicalScreenDesc = gif_logical_screen_descriptor;
pub type RiffData = riff_main;
pub type ExifByteOrder = tiff_main_byte_order;
pub type GzipChunk = gzip_main;

pub fn try_decode_gzip(test_file: &str) -> TestResult<Vec<GzipChunk>> {
    let buffer = std::fs::read(std::path::Path::new(test_file))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::gzip(dat) => Ok(dat),
        other => unreachable!("expected gzip, found {other:?}"),
    }
}

pub mod png_metrics {
    use super::*;
    use std::fmt::Write;

    fn abbrev(buf: &mut String, data: &[u8]) -> std::fmt::Result {
        const CUTOFF: usize = 16;
        const MARGIN: usize = 4;
        write!(buf, "[")?;
        if data.len() > CUTOFF {
            let lead = &data[..MARGIN];
            let trail = &data[data.len() - MARGIN..];
            let skip = data.len() - 2 * MARGIN;
            for byte in lead {
                write!(buf, "{:02x}", byte)?;
            }
            write!(buf, "...({} bytes skipped)...", skip)?;
            for byte in trail {
                write!(buf, "{:02x}", byte)?;
            }
        } else {
            for byte in data {
                write!(buf, "{:02x}", byte)?;
            }
        }
        write!(buf, "]")
    }

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
    pub struct GenericMetrics {
        count: usize,
    }

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct OptZlibMetrics {
        is_compressed: bool,
    }

    #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
    pub struct SingleZlibMetrics {
        is_present: bool,
        // opt_invalid_bytes: Option<Vec<u8>>,
    }

    pub type SbitMetrics = GenericMetrics;
    pub type SpltMetrics = GenericMetrics;
    pub type HistMetrics = GenericMetrics;
    pub type SrgbMetrics = GenericMetrics;
    pub type BkgdMetrics = GenericMetrics;
    pub type ChrmMetrics = GenericMetrics;
    pub type GamaMetrics = GenericMetrics;
    pub type IccpMetrics = SingleZlibMetrics;
    pub type PhysMetrics = GenericMetrics;

    pub type ItxtMetrics = Vec<OptZlibMetrics>;
    pub type ZtxtMetrics = GenericMetrics;

    pub type TextMetrics = GenericMetrics;
    pub type TimeMetrics = GenericMetrics;
    pub type TrnsMetrics = GenericMetrics;

    #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
    pub struct PngMetrics {
        tRNS: TrnsMetrics,
        cHRM: ChrmMetrics,
        gAMA: GamaMetrics,
        iCCP: IccpMetrics,
        sBIT: SbitMetrics,
        sRGB: SrgbMetrics,
        iTXt: ItxtMetrics,
        tEXt: TextMetrics,
        zTXt: ZtxtMetrics,
        bKGD: BkgdMetrics,
        hIST: HistMetrics,
        pHYs: PhysMetrics,
        sPLT: SpltMetrics,
        tIME: TimeMetrics,
    }

    pub fn analyze_png(test_file: &str) -> TestResult<PngMetrics> {
        let buffer = std::fs::read(std::path::Path::new(test_file))?;
        let mut input = Parser::new(&buffer);
        let dat = Decoder_png_main(&mut input)?;
        let mut metrics = PngMetrics::default();
        for chunk in dat.chunks.iter().chain(dat.more_chunks.iter()) {
            match &chunk.data {
                png_chunk_data::PLTE(_) => (), // ignoring critical chunk PLTE
                png_chunk_data::sRGB(_) => metrics.sRGB.count += 1,
                png_chunk_data::bKGD(_) => metrics.bKGD.count += 1,
                png_chunk_data::cHRM(_) => metrics.cHRM.count += 1,
                png_chunk_data::gAMA(_) => metrics.gAMA.count += 1,
                png_chunk_data::iCCP(_) => {
                    metrics.iCCP.is_present = true;
                }
                png_chunk_data::iTXt(x) => match x.compression_flag {
                    0 => metrics.iTXt.push(OptZlibMetrics {
                        is_compressed: false,
                    }),
                    1 => metrics.iTXt.push(OptZlibMetrics {
                        is_compressed: true,
                    }),
                    other => unreachable!("compression flag {other} is not recognized"),
                },
                png_chunk_data::pHYs(_) => metrics.pHYs.count += 1,
                png_chunk_data::tEXt(_) => metrics.tEXt.count += 1,
                png_chunk_data::tIME(_) => metrics.tIME.count += 1,
                png_chunk_data::tRNS(_) => metrics.tRNS.count += 1,
                png_chunk_data::zTXt(_) => metrics.zTXt.count += 1,
                png_chunk_data::hIST(_) => metrics.hIST.count += 1,
                png_chunk_data::sBIT(_) => metrics.sBIT.count += 1,
                png_chunk_data::sPLT(_) => metrics.sPLT.count += 1,
                png_chunk_data::unknown(_) => eprintln!(
                    "unknown png chunk type: {}",
                    String::from_utf8_lossy(&chunk.tag)
                ),
            }
        }
        Ok(metrics)
    }

    pub fn collate_png_table<S: std::fmt::Display>(samples: &[(S, PngMetrics)]) {
        let header = [
            "bKGD", "cHRM", "gAMA", "hIST", "iCCP", "iTXt", "pHYs", "sBIT", "sPLT", "sRGB", "tEXt",
            "tIME", "tRNS", "zTXt", "Filename",
        ];
        let header_line = header.join("\t");

        fn write_metrics(buf: &mut String, metrics: &PngMetrics) {
            let show_count = |buf: &mut String, metrics: &GenericMetrics| match metrics.count {
                0 => buf.push_str("❌\t"),
                1 => buf.push_str("✅\t"),
                2.. => buf.push_str("➕\t"),
            };

            let show_single_zlib =
                |buf: &mut String, metrics: &SingleZlibMetrics| match metrics.is_present {
                    true => buf.push_str("✅\t"),
                    false => buf.push_str("❌\t"),
                };

            let show_count_optzlib = |buf: &mut String, metrics: &Vec<OptZlibMetrics>| {
                let mut all = true;
                let mut any = false;
                for m in metrics {
                    all = all && m.is_compressed;
                    any = any || m.is_compressed;
                }
                match (all, any) {
                    (true, false) => buf.push_str("❌\t"), // only possible when empty
                    (true, true) => buf.push_str("✅\t"),
                    (false, true) => buf.push_str("⭕\t"),
                    (false, false) => buf.push_str("❓\t"),
                }
            };

            show_count(buf, &metrics.bKGD);
            show_count(buf, &metrics.cHRM);
            show_count(buf, &metrics.gAMA);
            show_count(buf, &metrics.hIST);
            show_single_zlib(buf, &metrics.iCCP);
            show_count_optzlib(buf, &metrics.iTXt);
            show_count(buf, &metrics.pHYs);
            show_count(buf, &metrics.sBIT);
            show_count(buf, &metrics.sPLT);
            show_count(buf, &metrics.sRGB);
            show_count(buf, &metrics.tEXt);
            show_count(buf, &metrics.tIME);
            show_count(buf, &metrics.tRNS);
            show_count(buf, &metrics.zTXt);
        }

        println!("{header_line}");
        for (sample, metrics) in samples.iter() {
            let mut line = String::new();
            write_metrics(&mut line, metrics);
            println!("{line}{sample}");
        }
    }
}

pub mod otf_metrics {
    use super::*;
    use encoding::{
        all::{MAC_ROMAN, UTF_16BE},
        DecoderTrap, Encoding,
    };

    pub type OpentypeFontDirectory = opentype_table_directory;
    pub type OpentypeGlyf = opentype_glyf_table;
    pub type GlyphDescription = opentype_glyf_description;
    pub type SimpleGlyph = opentype_glyf_simple;

    pub type OpentypeCmap = opentype_cmap_table;
    pub type OpentypeHead = opentype_head_table;
    pub type OpentypeHhea = opentype_hhea_table;

    pub type OpentypeHmtx = opentype_hmtx_table;
    pub type OpentypeHmtxHmetric = opentype_hmtx_table_h_metrics;

    pub type OpentypeMaxp = opentype_maxp_table;
    pub type OpentypeName = opentype_name_table;
    pub type OpentypeOs2 = opentype_os2_table;
    pub type OpentypePost = opentype_post_table;

    #[derive(Clone, Debug)]
    pub enum OpentypeMetrics {
        MultiFont(MultiFontMetrics),
        SingleFont(SingleFontMetrics),
    }

    #[derive(Clone, Debug)]
    pub struct MultiFontMetrics {
        version: (u16, u16),
        num_fonts: usize,
        font_metrics: Vec<Option<SingleFontMetrics>>,
    }

    #[derive(Clone, Debug)]
    pub struct SingleFontMetrics {
        sfnt_version: u32, // magic(0x0001_0000 | b"OTTO")
        num_tables: usize,
        required: RequiredTableMetrics,
        optional: OptionalTableMetrics,
    }

    #[derive(Clone, Copy, Debug)]
    // STUB - enrich with any further details we care about presenting
    struct CmapMetrics {
        version: u16,
        num_tables: usize,
    }

    #[derive(Clone, Copy, Debug)]
    // STUB - enrich with any further details we care about presenting
    struct HeadMetrics {
        major_version: u16,
        minor_version: u16,
        dir_hint: DirectionHint,
    }

    #[derive(Clone, Copy, Debug)]
    // STUB - enrich with any further details we care about presenting
    struct HheaMetrics {
        major_version: u16,
        minor_version: u16,
        num_lhm: usize,
    }

    #[derive(Clone, Copy, Debug)]
    enum MaxpMetrics {
        Postscript { version: u32 }, // version 0.5
        // STUB - enrich with any further details we care about presenting
        Version1 { version: u32 },       // version 1.0
        UnknownVersion { version: u32 }, // anything else
    }

    #[derive(Clone, Debug)]
    struct HmtxMetrics(Vec<UnifiedHmtxMetric>);

    #[derive(Copy, Clone, Debug)]
    struct UnifiedHmtxMetric {
        advance_width: Option<u16>,
        left_side_bearing: i16,
    }

    #[derive(Clone, Debug)]
    // STUB - enrich with any further details we care about presenting
    struct NameMetrics {
        version: u16,
        name_count: usize,
        name_records: Vec<NameRecord>,
        lang_tag_records: Option<Vec<LangTagRecord>>,
    }

    #[derive(Clone, Debug)]
    // STUB - this is probably less than we eventually want (assuming we care about presenting this info)
    struct NameRecord {
        plat_encoding_lang: PlatformEncodingLanguageId,
        name_id: NameId,
        buf: Option<String>,
    }

    // STUB - turn into enum?
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialOrd, PartialEq, Debug, Ord, Eq, Hash)]
    struct NameId(u16);

    impl NameId {
        pub const COPYRIGHT_NOTICE: Self = NameId(0);
        pub const FAMILY_NAME: Self = NameId(1);
        pub const SUBFAMILY_NAME: Self = NameId(2);
        pub const UNIQUE_FONT_IDENTIFICATION: Self = NameId(3);
        pub const FULL_FONT_NAME: Self = NameId(4);
        pub const VERSION_STRING: Self = NameId(5);
        pub const POSTSCRIPT_NAME: Self = NameId(6);
        pub const TRADEMARK: Self = NameId(7);
        pub const MANUFACTURER_NAME: Self = NameId(8);
        pub const DESIGNER_NAME: Self = NameId(9);
        pub const DESCRIPTION: Self = NameId(10);
        pub const VENDOR_URL: Self = NameId(11);
        pub const DESIGNER_URL: Self = NameId(12);
        pub const LICENSE_DESCRIPTION: Self = NameId(13);
        pub const LICENSE_INFO_URL: Self = NameId(14);
        pub const TYPOGRAPHIC_FAMILY_NAME: Self = NameId(16);
        pub const TYPOGRAPHIC_SUBFAMILY_NAME: Self = NameId(17);
        pub const COMPAT_FULL_NAME: Self = NameId(18);
        pub const SAMPLE_TEXT: Self = NameId(19);
        pub const POSTSCRIPT_FONT_NAME: Self = NameId(20);
        pub const WWS_FAMILY_NAME: Self = NameId(21);
        pub const WWS_SUBFAMILY_NAME: Self = NameId(22);
        pub const LIGHT_BACKGROUND_PALETTE: Self = NameId(23);
        pub const DARK_BACKGROUND_PALETTE: Self = NameId(24);
        pub const VARIATIONS_POSTSCRIPT_NAME_PREFIX: Self = NameId(25);
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    enum PlatformEncodingLanguageId {
        Unicode(UnicodeEncodingId),                          // 0
        Macintosh(MacintoshEncodingId, MacintoshLanguageId), // 1
        Windows(WindowsEncodingId, WindowsLanguageId),       // 3
    }

    impl PlatformEncodingLanguageId {
        fn matches_locale(&self, buf: &str) -> bool {
            match self {
                PlatformEncodingLanguageId::Unicode(_) => buf.is_ascii(),
                PlatformEncodingLanguageId::Macintosh(
                    macintosh_encoding_id,
                    macintosh_language_id,
                ) => match macintosh_encoding_id {
                    MacintoshEncodingId::Roman => macintosh_language_id.is_english(),
                    _ => false,
                },
                PlatformEncodingLanguageId::Windows(_, windows_language_id) => {
                    windows_language_id.is_english()
                }
            }
        }

        fn convert(&self, link: &[u8]) -> String {
            match self {
                PlatformEncodingLanguageId::Macintosh(MacintoshEncodingId::Roman, ..) => MAC_ROMAN
                    .decode(link, DecoderTrap::Ignore)
                    .unwrap()
                    .to_owned(),
                PlatformEncodingLanguageId::Macintosh(..)
                | PlatformEncodingLanguageId::Unicode(_) => UTF_16BE
                    .decode(link, DecoderTrap::Ignore)
                    .unwrap()
                    .to_owned(),
                PlatformEncodingLanguageId::Windows(..) => UTF_16BE
                    .decode(link, DecoderTrap::Ignore)
                    .unwrap()
                    .to_owned(),
            }
        }
    }

    impl TryFrom<(u16, u16, u16)> for PlatformEncodingLanguageId {
        type Error = UnknownValueError<u16>;

        fn try_from(value: (u16, u16, u16)) -> Result<Self, Self::Error> {
            let (platform_id, encoding_id, language_id) = value;

            match platform_id {
                0 => Ok(Self::Unicode(UnicodeEncodingId::try_from(encoding_id)?)),
                1 => {
                    let macintosh_encoding_id = MacintoshEncodingId::try_from(encoding_id)?;
                    let macintosh_language_id = MacintoshLanguageId::try_from(language_id)?;
                    Ok(Self::Macintosh(
                        macintosh_encoding_id,
                        macintosh_language_id,
                    ))
                }
                3 => {
                    let windows_encoding_id = WindowsEncodingId::try_from(encoding_id)?;
                    // NOTE - this conversion is currently infallible, but if we refine it, replace with try_from() with `?`
                    let windows_language_id = WindowsLanguageId::from(language_id);
                    Ok(Self::Windows(windows_encoding_id, windows_language_id))
                }
                bad_value => Err(UnknownValueError {
                    what: String::from("platform ID"),
                    bad_value,
                }),
            }
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    enum UnicodeEncodingId {
        Semantics_Unicode1Dot0 = 0, // deprecated
        Semantics_Unicode1Dot1 = 1, // deprecated
        Semantics_UCS = 2,          // deprecated
        Semantics_Unicode2_BMP = 3,
        Semantics_Unicode2_Full = 4,
    }

    impl TryFrom<u16> for UnicodeEncodingId {
        type Error = UnknownValueError<u16>;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Semantics_Unicode1Dot0),
                1 => Ok(Self::Semantics_Unicode1Dot1),
                2 => Ok(Self::Semantics_UCS),
                3 => Ok(Self::Semantics_Unicode2_BMP),
                4 => Ok(Self::Semantics_Unicode2_Full),
                _ => Err(UnknownValueError {
                    what: "Unicode Encoding ID".into(),
                    bad_value: value,
                }),
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    #[repr(u16)]
    enum MacintoshEncodingId {
        Roman = 0,
        Japanese = 1,
        TradChinese = 2,
        Korean = 3,
        Arabic = 4,
        Hebrew = 5,
        Greek = 6,
        Russian = 7,
        RSymbol = 8,
        Devanagari = 9,
        Gurmukhi = 10,
        Gujarati = 11,
        Oriya = 12,
        Bengali = 13,
        Tamil = 14,
        Telugu = 15,
        Kannada = 16,
        Malayalam = 17,
        Sinhalese = 18,
        Burmese = 19,
        Khmer = 20,
        Thai = 21,
        Laotian = 22,
        Georgian = 23,
        Armenian = 24,
        SimplChinese = 25,
        Tibetan = 26,
        Mongolian = 27,
        Geez = 28,
        Slavic = 29,
        Vietnamese = 30,
        Sindhi = 31,
        Uninterpreted = 32,
    }

    impl TryFrom<u16> for MacintoshEncodingId {
        type Error = UnknownValueError<u16>;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0..=32 => unsafe { Ok(std::mem::transmute(value)) },
                bad_value => Err(UnknownValueError {
                    what: "Macintosh Encoding ID".into(),
                    bad_value,
                }),
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    #[repr(u16)]
    enum MacintoshLanguageId {
        English, // 0
        // STUB - for this API, we don't necessarily need to have a full list of all languages as first-class variants, but it might be nice for later if we decide to present certain languages preferentially on a per-font basis
        Other(u16), // 1..=150
    }

    impl TryFrom<u16> for MacintoshLanguageId {
        type Error = UnknownValueError<u16>;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                // NOTE - only values 0..=150 are populated according to [this spec](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html)
                0 => Ok(MacintoshLanguageId::English),
                1..=150 => Ok(MacintoshLanguageId::Other(value)),
                bad_value => Err(UnknownValueError {
                    what: String::from("Macintosh language ID"),
                    bad_value,
                }),
            }
        }
    }

    impl MacintoshLanguageId {
        const fn is_english(self) -> bool {
            matches!(self, Self::English)
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[repr(u16)]
    enum WindowsEncodingId {
        Symbol = 0,
        UnicodeBMP = 1, // preferred
        ShiftJIS = 2,
        PRC = 3,
        Big5 = 4,
        Wansung = 5,
        Johab = 6,
        UnicodeFull = 10,
    }

    impl TryFrom<u16> for WindowsEncodingId {
        type Error = UnknownValueError<u16>;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            Ok(match value {
                0 => Self::Symbol,
                1 => Self::UnicodeBMP,
                2 => Self::ShiftJIS,
                3 => Self::PRC,
                4 => Self::Big5,
                5 => Self::Wansung,
                6 => Self::Johab,
                // 7..=9 are reserved
                10 => Self::UnicodeFull,
                bad_value => {
                    return Err(UnknownValueError {
                        what: String::from("Windows Encoding ID"),
                        bad_value,
                    })
                }
            })
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[repr(transparent)]
    struct WindowsLanguageId(u16);

    impl From<u16> for WindowsLanguageId {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }

    impl WindowsLanguageId {
        // STUB - there are more English locales, and many more non-English language tags, but we don't need a full list for now
        pub const EN: Self = WindowsLanguageId(0x0009);
        pub const EN_US: Self = WindowsLanguageId(0x0409);
        pub const EN_GB: Self = WindowsLanguageId(0x0809);
        pub const EN_AU: Self = WindowsLanguageId(0x0C09);
        pub const EN_CA: Self = WindowsLanguageId(0x1009);
        pub const EN_NZ: Self = WindowsLanguageId(0x1409);

        const fn is_english(self) -> bool {
            // FIXME - there are other English locales but we don't expect to find them that often, at least in abstract
            matches!(
                self,
                Self::EN | Self::EN_US | Self::EN_GB | Self::EN_AU | Self::EN_CA | Self::EN_NZ
            )
        }
    }

    impl std::fmt::Display for NameId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // REVIEW - might we want to write out the meaning (as a string) instead, where possible?
            write!(f, "{}", self.0)
        }
    }

    #[derive(Clone, Debug)]
    struct LangTagRecord {
        lang_tag: Option<String>,
    }

    #[derive(Clone, Debug)]
    struct Os2Metrics {
        version: u16,
        // STUB - is anything else relevant?
    }

    #[derive(Clone, Debug)]
    struct PostMetrics {
        version: u32,
        is_fixed_pitch: bool,
        // STUB - is anything else relevant?
    }

    #[derive(Clone, Debug)]
    pub struct RequiredTableMetrics {
        cmap: CmapMetrics,
        head: HeadMetrics,
        hhea: HheaMetrics,
        maxp: MaxpMetrics,
        hmtx: HmtxMetrics,
        name: NameMetrics,
        os2: Os2Metrics,
        post: PostMetrics,
    }

    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    // number of elements in the contained array
    pub struct RawArrayMetrics(usize);

    type CvtMetrics = RawArrayMetrics;
    type FpgmMetrics = RawArrayMetrics;

    #[derive(Clone, Debug)]
    pub struct GlyfMetrics {
        num_glyphs: usize,
        glyphs: Vec<GlyphMetric>,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum GlyphMetric {
        Empty,
        Simple(SimpleGlyphMetric),
        Composite(CompositeGlyphMetric),
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct SimpleGlyphMetric {
        contours: usize,
        coordinates: usize,
        instructions: usize,
        bounding_box: BoundingBox,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct CompositeGlyphMetric {
        components: usize,
        instructions: usize,
        bounding_box: BoundingBox,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct BoundingBox {
        x_min: i16,
        y_min: i16,
        x_max: i16,
        y_max: i16,
    }

    impl std::fmt::Display for BoundingBox {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "[({}, {}) <-> ({}, {})]",
                self.x_min, self.y_min, self.x_max, self.y_max
            )
        }
    }

    type LocaMetrics = ();

    #[derive(Clone, Debug)]
    pub struct OptionalTableMetrics {
        cvt: Option<CvtMetrics>,
        fpgm: Option<FpgmMetrics>,
        loca: Option<LocaMetrics>,
        glyf: Option<GlyfMetrics>,
        // STUB - add more tables as we expand opentype definition
    }

    #[derive(Debug)]
    pub struct UnknownValueError<T> {
        what: String,
        bad_value: T,
    }

    impl<T> std::fmt::Display for UnknownValueError<T>
    where
        T: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "bad {}: {}", self.what, self.bad_value)
        }
    }

    impl<T> std::error::Error for UnknownValueError<T> where T: std::fmt::Display + std::fmt::Debug {}

    pub fn analyze_font(test_file: &str) -> TestResult<OpentypeMetrics> {
        let buffer = std::fs::read(std::path::Path::new(test_file))?;
        let mut input = Parser::new(&buffer);
        let dat = Decoder_opentype_main(&mut input)?;
        // TODO: do we want to collect (and return) any metrics here?
        match dat.font.directory {
            opentype_font_directory::TTCHeader(multi) => {
                let version = (multi.major_version, multi.minor_version);
                let (num_fonts, font_metrics) = match multi.header {
                    opentype_ttc_header_header::UnknownVersion(n) => {
                        return Err(Box::new(UnknownValueError {
                            what: format!("ttc header version"),
                            bad_value: n,
                        }));
                    }
                    opentype_ttc_header_header::Version1(v1header) => {
                        let mut font_metrics = Vec::with_capacity(v1header.table_directories.len());
                        for font in v1header.table_directories.iter() {
                            let tmp = match &font.link {
                                Some(dir) => Some(analyze_table_directory(dir)?),
                                None => None,
                            };
                            font_metrics.push(tmp);
                        }
                        (v1header.num_fonts as usize, font_metrics)
                    }
                    opentype_ttc_header_header::Version2(v2header) => {
                        let mut font_metrics = Vec::with_capacity(v2header.table_directories.len());
                        for font in v2header.table_directories.iter() {
                            let tmp = match &font.link {
                                Some(dir) => Some(analyze_table_directory(dir)?),
                                None => None,
                            };
                            font_metrics.push(tmp);
                        }
                        (v2header.num_fonts as usize, font_metrics)
                    }
                };
                let ret = MultiFontMetrics {
                    version,
                    num_fonts,
                    font_metrics,
                };
                Ok(OpentypeMetrics::MultiFont(ret))
            }
            opentype_font_directory::TableDirectory(single) => Ok(OpentypeMetrics::SingleFont(
                analyze_table_directory(&single)?,
            )),
        }
    }

    fn utf16be_convert(buf: &[u8]) -> String {
        UTF_16BE
            .decode(buf, DecoderTrap::Ignore)
            .unwrap()
            .to_owned()
    }

    pub fn analyze_table_directory(dir: &OpentypeFontDirectory) -> TestResult<SingleFontMetrics> {
        let required = {
            let cmap = {
                let cmap = &dir.table_links.cmap;
                CmapMetrics {
                    version: cmap.version,
                    num_tables: cmap.num_tables as usize,
                }
            };
            let head = {
                let head = &dir.table_links.head;
                HeadMetrics {
                    major_version: head.major_version,
                    minor_version: head.minor_version,
                    dir_hint: head.font_direction_hint.try_into().unwrap(),
                }
            };
            let hhea = {
                let hhea = &dir.table_links.hhea;
                HheaMetrics {
                    major_version: hhea.major_version,
                    minor_version: hhea.minor_version,
                    num_lhm: hhea.number_of_long_horizontal_metrics as usize,
                }
            };
            let maxp = {
                let maxp = &dir.table_links.maxp;
                let version = maxp.version;
                match &maxp.data {
                    opentype_maxp_table_data::MaxpPostScript => MaxpMetrics::Postscript { version },
                    opentype_maxp_table_data::MaxpV1(_table) => MaxpMetrics::Version1 { version },
                    opentype_maxp_table_data::MaxpUnknown(_) => {
                        MaxpMetrics::UnknownVersion { version }
                    }
                }
            };
            let hmtx = {
                let hmtx = &dir.table_links.hmtx;
                let mut accum =
                    Vec::with_capacity(hmtx.h_metrics.len() + hmtx.left_side_bearings.len());
                for hmet in hmtx.h_metrics.iter() {
                    accum.push(UnifiedHmtxMetric {
                        advance_width: Some(hmet.advance_width),
                        left_side_bearing: as_s16(hmet.left_side_bearing),
                    });
                }
                for lsb in hmtx.left_side_bearings.iter() {
                    accum.push(UnifiedHmtxMetric {
                        advance_width: None,
                        left_side_bearing: as_s16(*lsb),
                    });
                }
                HmtxMetrics(accum)
            };
            let name = {
                let name = &dir.table_links.name;
                let name_records = {
                    let mut tmp = Vec::with_capacity(name.name_records.len());
                    for record in name.name_records.iter() {
                        let plat_encoding_lang = PlatformEncodingLanguageId::try_from((
                            record.platform,
                            record.encoding,
                            record.language,
                        ))?;
                        let buf = match &record.offset.link {
                            Some(link) => Some(plat_encoding_lang.convert(&link)),
                            None => None,
                        };
                        tmp.push(NameRecord {
                            plat_encoding_lang,
                            name_id: NameId(record.name_id),
                            buf,
                        });
                    }
                    tmp
                };
                let lang_tag_records = {
                    match &name.data {
                        opentype_name_table_data::NameVersion0 => None,
                        opentype_name_table_data::NameVersion1(v1data) => {
                            let mut tmp = Vec::with_capacity(v1data.lang_tag_records.len());
                            for record in v1data.lang_tag_records.iter() {
                                let lang_tag = match &record.offset.link {
                                    Some(link) => Some(utf16be_convert(&link)),
                                    None => None,
                                };
                                tmp.push(LangTagRecord { lang_tag })
                            }
                            Some(tmp)
                        }
                        opentype_name_table_data::NameVersionUnknown(ver) => {
                            return Err(Box::new(UnknownValueError {
                                what: format!("name table version"),
                                bad_value: *ver,
                            }))
                        }
                    }
                };
                NameMetrics {
                    version: name.version,
                    name_count: name.name_count as usize,
                    name_records,
                    lang_tag_records,
                }
            };
            let os2 = {
                let os2 = &dir.table_links.os2;
                Os2Metrics {
                    version: os2.version,
                }
            };
            let post = {
                let post = &dir.table_links.post;
                PostMetrics {
                    version: post.version,
                    is_fixed_pitch: post.is_fixed_pitch != 0,
                }
            };
            RequiredTableMetrics {
                cmap,
                head,
                hhea,
                maxp,
                hmtx,
                name,
                os2,
                post,
            }
        };
        let optional = {
            let cvt = dir
                .table_links
                .cvt
                .as_ref()
                .map(|cvt| RawArrayMetrics(cvt.len()));
            let fpgm = dir
                .table_links
                .fpgm
                .as_ref()
                .map(|fpgm| RawArrayMetrics(fpgm.len()));
            let loca = dir.table_links.loca.as_ref().map(|_| ());
            let glyf = dir.table_links.glyf.as_ref().map(|glyf| {
                let num_glyphs = glyf.len();
                let glyphs = glyf
                    .iter()
                    .map(|g| match g {
                        opentype_glyf_table::EmptyGlyph => GlyphMetric::Empty,
                        opentype_glyf_table::Glyph(gl) => match &gl.description {
                            GlyphDescription::HeaderOnly => GlyphMetric::Empty,
                            GlyphDescription::Simple(simple) => {
                                GlyphMetric::Simple(SimpleGlyphMetric {
                                    contours: gl.number_of_contours as usize,
                                    coordinates: *simple.end_points_of_contour.last().unwrap()
                                        as usize
                                        + 1,
                                    instructions: simple.instruction_length as usize,
                                    bounding_box: bounding_box(gl),
                                })
                            }
                            GlyphDescription::Composite(comp) => {
                                GlyphMetric::Composite(CompositeGlyphMetric {
                                    components: comp.glyphs.len(),
                                    instructions: comp.instructions.len(),
                                    bounding_box: bounding_box(gl),
                                })
                            }
                        },
                    })
                    .collect();
                GlyfMetrics { num_glyphs, glyphs }
            });
            OptionalTableMetrics {
                cvt,
                fpgm,
                loca,
                glyf,
            }
        };
        Ok(SingleFontMetrics {
            sfnt_version: dir.sfnt_version,
            num_tables: dir.num_tables as usize,
            required,
            optional,
        })
    }

    fn bounding_box(gl: &opentype_glyf_table_Glyph) -> BoundingBox {
        BoundingBox {
            x_min: as_s16(gl.x_min),
            y_min: as_s16(gl.y_min),
            x_max: as_s16(gl.x_max),
            y_max: as_s16(gl.y_max),
        }
    }

    pub fn show_opentype_stats(metrics: &OpentypeMetrics) {
        match metrics {
            OpentypeMetrics::MultiFont(multi) => {
                println!(
                    "TTC: version {} ({} fonts)",
                    format_version_major_minor(multi.version.0, multi.version.1),
                    multi.num_fonts
                );
                for (i, o_font) in multi.font_metrics.iter().enumerate() {
                    match o_font.as_ref() {
                        Some(font) => {
                            println!("=== Font @ Index {i} ===");
                            show_font_metrics(font);
                        }
                        None => {
                            println!("=== Skipping Index {i} ===");
                        }
                    }
                }
            }
            OpentypeMetrics::SingleFont(single) => show_font_metrics(single),
        }
    }

    fn show_magic(magic: u32) {
        println!("Magic: {}", format_magic(magic));
    }

    fn format_magic(magic: u32) -> String {
        let bytes = magic.to_be_bytes();
        let show = |b: u8| {
            if b.is_ascii_alphanumeric() {
                String::from(b as char)
            } else {
                format!("{:02x}", b)
            }
        };
        format!(
            "{}{}{}{}",
            show(bytes[0]),
            show(bytes[1]),
            show(bytes[2]),
            show(bytes[3])
        )
    }

    fn show_font_metrics(font: &SingleFontMetrics) {
        show_magic(font.sfnt_version);
        show_required_metrics(&font.required);
        show_optional_metrics(&font.optional);
    }

    fn show_required_metrics(required: &RequiredTableMetrics) {
        show_cmap_metrics(&required.cmap);
        show_head_metrics(&required.head);
        show_hhea_metrics(&required.hhea);
        show_htmx_metrics(&required.hmtx);
        show_maxp_metrics(&required.maxp);
        show_name_metrics(&required.name);
        show_os2_metrics(&required.os2);
        show_post_metrics(&required.post);
    }

    fn show_optional_metrics(optional: &OptionalTableMetrics) {
        show_cvt_metrics(&optional.cvt);
        show_fpgm_metrics(&optional.fpgm);
        show_loca_metrics(&optional.loca);
        show_glyf_metrics(&optional.glyf);
    }

    fn show_cvt_metrics(cvt: &Option<CvtMetrics>) {
        match cvt {
            Some(RawArrayMetrics(count)) => println!("cvt: FWORD[{count}]"),
            None => (),
        }
    }

    fn show_fpgm_metrics(fpgm: &Option<FpgmMetrics>) {
        match fpgm {
            Some(RawArrayMetrics(count)) => println!("fpgm: uint8[{count}]"),
            None => (),
        }
    }

    fn show_loca_metrics(loca: &Option<LocaMetrics>) {
        match loca {
            Some(()) => println!("loca"),
            None => (),
        }
    }

    /// Enumerates the contents of a slice, showing only the first and last `bookend` items if the slice is long enough.
    ///
    /// Each item is shown with `show_fn`, and the `elision_message` is printed (along with the range of indices skipped)
    /// if the slice length exceeds than 2 * `bookend`, in between the initial and terminal span of `bookend` items.
    fn show_items_elided<T>(
        items: &[T],
        show_fn: impl Fn(usize, &T),
        bookend: usize,
        fn_message: impl Fn(usize, usize) -> String,
    ) {
        let count = items.len();
        if count > bookend * 2 {
            for ix in 0..bookend {
                show_fn(ix, &items[ix]);
            }
            println!("{}", fn_message(bookend, count - bookend));
            for ix in (count - bookend)..count {
                show_fn(ix, &items[ix]);
            }
        } else {
            for (ix, item) in items.iter().enumerate() {
                show_fn(ix, item);
            }
        }
    }

    fn format_version16dot16(v: u32) -> String {
        let major = (v >> 16) as u16;
        let minor = ((v & 0xf000) >> 12) as u16;
        format_version_major_minor(major, minor)
    }

    fn format_version_major_minor(major: u16, minor: u16) -> String {
        format!("{}.{}", major, minor)
    }

    fn show_cmap_metrics(cmap: &CmapMetrics) {
        println!(
            "cmap: version {}, {} encoding tables",
            cmap.version, cmap.num_tables
        );
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    #[repr(u16)]
    enum DirectionHint {
        FullyMixed = 0,
        StrongLR = 1,
        NeutralLR = 2,
        StrongRL = 0xffff,  // -1
        NeutralRL = 0xfffe, // -2
    }

    impl TryFrom<u16> for DirectionHint {
        // TODO - replace with actual error-type
        type Error = ();

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(DirectionHint::FullyMixed),
                1 => Ok(DirectionHint::StrongLR),
                2 => Ok(DirectionHint::NeutralLR),
                0xffff => Ok(DirectionHint::StrongRL),
                0xfffe => Ok(DirectionHint::NeutralRL),
                _ => Err(()),
            }
        }
    }

    impl std::fmt::Display for DirectionHint {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                DirectionHint::FullyMixed => write!(f, "Fully Mixed-Direction"),
                DirectionHint::StrongLR => write!(f, "Strong LTR only"),
                DirectionHint::NeutralLR => write!(f, "Strong LTR or Neutral"),
                DirectionHint::StrongRL => write!(f, "Strong RTL only"),
                DirectionHint::NeutralRL => write!(f, "Strong RTL or Neutral"),
            }
        }
    }

    fn show_head_metrics(head: &HeadMetrics) {
        println!(
            "head: version {}, {}",
            format_version_major_minor(head.major_version, head.minor_version),
            head.dir_hint,
        );
    }

    fn show_hhea_metrics(hhea: &HheaMetrics) {
        println!(
            "hhea: table version {}, {} horizontal long metrics",
            format_version_major_minor(hhea.major_version, hhea.minor_version),
            hhea.num_lhm,
        );
    }

    fn show_htmx_metrics(hmtx: &HmtxMetrics) {
        let show_unified = |ix: usize, hmet: &UnifiedHmtxMetric| match &hmet.advance_width {
            Some(width) => println!(
                "\tGlyph ID [{ix}]: advanceWidth={width}, lsb={}",
                hmet.left_side_bearing
            ),
            None => println!("\tGlyph ID [{ix}]: lsb={}", hmet.left_side_bearing),
        };

        show_items_elided(&hmtx.0, show_unified, 8, |start, stop| {
            format!("    (skipping hmetrics {start}..{stop})")
        });
    }

    fn show_maxp_metrics(maxp: &MaxpMetrics) {
        match maxp {
            MaxpMetrics::Postscript { version } => println!(
                "maxp: version {} (PostScript)",
                format_version16dot16(*version)
            ),
            MaxpMetrics::UnknownVersion { version } => println!(
                "maxp: version {} (not recognized)",
                format_version16dot16(*version)
            ),
            // STUB - currently limited by definition of Version1 variant, but the information available in the type may be enriched later
            MaxpMetrics::Version1 { version } => println!(
                "maxp: version {} (contents omitted)",
                format_version16dot16(*version)
            ),
        }
    }

    fn show_name_metrics(name: &NameMetrics) {
        // STUB - add more details if appropriate
        match &name.lang_tag_records {
            Some(records) => {
                println!(
                    "name: version {}, {} name_records, {} language tag records",
                    name.version,
                    name.name_count,
                    records.len()
                );
            }
            None => println!(
                "name: version {}, {} name_records",
                name.version, name.name_count
            ),
        }
        let mut no_name_yet = true;
        for record in name.name_records.iter() {
            match record {
                // STUB - if there are any more name records we care about, add them here
                &NameRecord {
                    name_id: NameId::FULL_FONT_NAME,
                    plat_encoding_lang,
                    buf: Some(ref buf),
                } => {
                    if no_name_yet && plat_encoding_lang.matches_locale(buf) {
                        println!("\tFull Font Name: {}", buf);
                        no_name_yet = false;
                    }
                }
                _ => continue,
            }
        }
    }

    fn show_os2_metrics(os2: &Os2Metrics) {
        // STUB - Metrics is just an alias for the raw type, enrich and refactor if appropriate
        println!("os/2: version {}", os2.version);
    }

    fn show_post_metrics(post: &PostMetrics) {
        // STUB - Metrics is just an alias for the raw type, enrich and refactor if appropriate
        println!(
            "post: version {} ({})",
            format_version16dot16(post.version),
            if post.is_fixed_pitch {
                "monospaced"
            } else {
                "proportionally spaced"
            }
        );
    }

    // NOTE - scaffolding to mark the values we currently parse into u16 but which are logically i16, to flag changes to the gencode API as they crop up
    const fn as_s16(v: u16) -> i16 {
        v as i16
    }

    fn show_glyf_metrics(glyf: &Option<GlyfMetrics>) {
        if let Some(glyf) = glyf.as_ref() {
            println!("glyf: {} glyphs", glyf.num_glyphs);
            show_items_elided(
                glyf.glyphs.as_slice(),
                show_glyph_metric,
                8,
                |start, stop| format!("    (skipping glyphs {start}..{stop})"),
            )
        } else {
            println!("glyf: <not present>")
        }
    }

    fn show_glyph_metric(ix: usize, glyf: &GlyphMetric) {
        print!("\t[{ix}]: ");
        match glyf {
            GlyphMetric::Empty => println!("<empty>"),
            GlyphMetric::Simple(simple) => {
                println!(
                    "Simple Glyph [{} contours, {} coordinates, {} instructions, xy: {}]",
                    simple.contours, simple.coordinates, simple.instructions, simple.bounding_box
                );
            }
            GlyphMetric::Composite(composite) => {
                println!(
                    "Composite Glyph [{} components, {} instructions, xy: {}]",
                    composite.components, composite.instructions, composite.bounding_box,
                );
            }
        }
    }
}

pub use otf_metrics::*;
pub use png_metrics::*;
