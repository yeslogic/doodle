use super::util::{U16Set, Wec};
use super::*;
use doodle::Label;
use encoding::{
    all::{MAC_ROMAN, UTF_16BE},
    DecoderTrap, Encoding,
};

// SECTION - Command-line configurable options for what to show

/// Set of configurable values that control which metrics are shown, and in how much detail
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    // STUB - Currently only controls bookending, and whether to dump only uncovered tables
    bookend_size: usize,
    inline_bookend: usize,
    extra_only: bool,
}

impl Config {
    const DEFAULT_BOOKEND_SIZE: usize = 8;
    const DEFAULT_INLINE_BOOKEND: usize = 3;
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            bookend_size: Self::DEFAULT_BOOKEND_SIZE,
            inline_bookend: Self::DEFAULT_INLINE_BOOKEND,
            extra_only: false,
        }
    }
}

/// Builder-pattern for [`Config`]
pub struct ConfigBuilder {
    bookend_size: Option<usize>,
    inline_bookend: Option<usize>,
    extra_only: Option<bool>,
}

impl ConfigBuilder {
    /// Returns a new `ConfigBuilder` object.
    pub fn new() -> Self {
        Self {
            bookend_size: None,
            inline_bookend: None,
            extra_only: None,
        }
    }

    /// Overwrites the default value of `bookend_size`, which determines how long a prefix and suffix are shown
    /// around the elided middle of a long array.
    ///
    /// Currently controls all such array-elisions across the entire output, without any mechanism for different
    /// bookending sizes per table or section.
    pub fn bookend_size(mut self, size: usize) -> Self {
        self.bookend_size = Some(size);
        self
    }

    /// Overwrites the default value of `inline_bookend`, which determines how long a prefix and suffix are shown
    /// around the elided middle of a long array to be displayed inline.
    ///
    /// Currently controls all such inline-array-elisions across the entire output, without any mechanism for different
    /// inline-bookending sizes per table or section.
    pub fn inline_bookend(mut self, size: usize) -> Self {
        self.inline_bookend = Some(size);
        self
    }

    /// Overwrites the default value of `extra_only`, which is normally `false` but which can be set to `true` to
    /// suppress the output of all recognized metrics, and only prints the list of table-ids that are not given
    /// table links by the current definition of the OpenType format model.
    pub fn extra_only(mut self, extra_only: bool) -> Self {
        self.extra_only = Some(extra_only);
        self
    }

    /// Finalizes a [`ConfigBuilder`] and produces a [`Config`] according to the default- or user-overridden values for
    /// each configurable property.
    pub fn build(self) -> Config {
        Config {
            bookend_size: self.bookend_size.unwrap_or(Config::DEFAULT_BOOKEND_SIZE),
            inline_bookend: self
                .inline_bookend
                .unwrap_or(Config::DEFAULT_INLINE_BOOKEND),
            extra_only: self.extra_only.unwrap_or_default(),
        }
    }
}
// !SECTION

// SECTION - Type aliases for stable referencing of commonly-used generated types
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

// FIXME - add the currently-inlined input types to the set of API-stabilization aliases

pub type OpentypeGdef = opentype_gdef_table;

pub type OpentypeGdefTableData = opentype_gdef_table_data;

pub type OpentypeAttachPoint = opentype_gdef_table_attach_list_link_attach_point_offsets_link;
pub type OpentypeCoverageTable = opentype_coverage_table;
pub type OpentypeCoverageTableData = opentype_coverage_table_data;
pub type OpentypeCoverageRangeRecord = opentype_coverage_table_data_Format2_range_records;

pub type OpentypeGpos = opentype_gpos_table;
// !SECTION

// SECTION - Helper traits for consistent-style conversion from generated types to the types we use to represent them in the API Helper
trait Promote<Original>: Sized {
    /// Convert from `Original` into `Self`.
    ///
    /// # Panics
    ///
    /// Should not panic. If the conversion can fail, use [`TryPromote`] instead
    fn promote(orig: &Original) -> Self;
}

trait TryPromote<Original>: Sized {
    type Error: std::error::Error;

    /// Fallibly convert from the `Original` into `Self`.
    fn try_promote(orig: &Original) -> Result<Self, Self::Error>;
}

trait TryFromRef<Original: _Ref>: Sized {
    type Error: std::error::Error;

    /// Fallibly convert from the GAT `Ref<'a>` defined on Original, into `Self`.
    fn try_from_ref<'a>(orig: <Original as _Ref>::Ref<'a>) -> Result<Self, Self::Error>;
}

trait _Ref {
    type Ref<'a>;
}

impl<T, U> _Ref for (T, U)
where
    T: Copy + 'static,
    U: 'static,
{
    type Ref<'a> = (T, &'a U);
}

fn promote_vec<O, T>(orig_slice: &[O]) -> Vec<T>
where
    T: Promote<O>,
{
    orig_slice.iter().map(T::promote).collect()
}

fn try_promote_vec<O, T, E>(orig_slice: &[O]) -> Result<Vec<T>, E>
where
    T: TryPromote<O, Error = E>,
{
    let mut ret = Vec::with_capacity(orig_slice.len());
    for elem in orig_slice {
        ret.push(T::try_promote(elem)?);
    }
    Ok(ret)
}

fn promote_opt<O, T>(orig_opt: &Option<O>) -> Option<T>
where
    T: Promote<O>,
{
    orig_opt.as_ref().map(T::promote)
}

fn try_promote_opt<O, T>(orig: &Option<O>) -> Result<Option<T>, T::Error>
where
    T: TryPromote<O>,
{
    orig.as_ref().map(T::try_promote).transpose()
}

impl<T, Original> TryPromote<Original> for T
where
    T: Promote<Original>,
{
    type Error = std::convert::Infallible;

    fn try_promote(orig: &Original) -> Result<Self, Self::Error> {
        Ok(<T as Promote<Original>>::promote(orig))
    }
}

impl<T: Clone> Promote<T> for T {
    fn promote(orig: &T) -> T {
        orig.clone()
    }
}

/// Type-agnostic macro for dereferencing machine-generated Offset16 abstactions
/// into Option<T> of the promotable dereference-value
macro_rules! promote_link {
    () => {
        (|offset| promote_opt(&offset.link))
    };
}

// !SECTION

// SECTION - Generic (but niche-use-case) helper definitions
#[derive(Clone)]
#[repr(transparent)]
struct SemVec<Sem, T> {
    inner: Vec<T>,
    __proxy: std::marker::PhantomData<Sem>,
}

impl<Sem, T> SemVec<Sem, T> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            __proxy: std::marker::PhantomData,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: Vec::with_capacity(cap),
            __proxy: std::marker::PhantomData,
        }
    }
}

impl<Sem, T> From<Vec<T>> for SemVec<Sem, T> {
    fn from(v: Vec<T>) -> Self {
        Self {
            inner: v,
            __proxy: std::marker::PhantomData,
        }
    }
}

impl<Sem, T> FromIterator<T> for SemVec<Sem, T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            inner: Vec::from_iter(iter),
            __proxy: std::marker::PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for SemVec<ClassId, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // REVIEW - consider whether we need this distinction when ChainedRule already discriminates on Sem
        f.debug_tuple("ClassIds").field(&self.inner).finish()
    }
}

impl<T> std::fmt::Debug for SemVec<GlyphId, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // REVIEW - consider whether we need this distinction when ChainedRule already discriminates on Sem
        f.debug_tuple("GlyphIds").field(&self.inner).finish()
    }
}

impl<Sem, T> std::ops::Deref for SemVec<Sem, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Marker type for indicating a SemVec (or any types above it) holds GlyphId-semantics u16 values
#[derive(Clone)]
struct GlyphId;
/// Marker type for indicating a SemVec (or any types above it) holds ClassId-semantics u16 values
#[derive(Clone)]
struct ClassId;

// !SECTION
/// Crate-private mirco-module for compile-time same-type assertions that can be chained
pub(crate) mod refl {
    pub(crate) trait Refl<T> {
        type Solution;
    }

    impl<T> Refl<T> for T {
        type Solution = T;
    }

    /// A === B => A, type error otherwise
    ///
    /// If Refl is too heavy-handed we can drop the forced unification and use this to merely document
    /// our expectations about what should be equal without rejecting parameters that are different.
    pub(crate) type ReflType<A, B> = <A as Refl<B>>::Solution;
}
use refl::ReflType;

/// Shorthand for qualifying a TryPromote::Error item
type TPErr<Src, Tgt> = <Tgt as TryPromote<Src>>::Error;

/// Shorthand for qualifying a TryFromRef::Error item in the same style as `TPErr`
type TFRErr<Src, Tgt> = <Tgt as TryFromRef<Src>>::Error;

/// Hint to remind us that an error is being produced locally
type Local<T> = T;

// SECTION - *Metrics and mid- to low-level API-enrichment analogues for raw gencode types
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
    extraMagic: Vec<u32>,
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
    buf: String,
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
            PlatformEncodingLanguageId::Macintosh(macintosh_encoding_id, macintosh_language_id) => {
                match macintosh_encoding_id {
                    MacintoshEncodingId::Roman => macintosh_language_id.is_english(),
                    _ => false,
                }
            }
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
            PlatformEncodingLanguageId::Macintosh(..) | PlatformEncodingLanguageId::Unicode(_) => {
                UTF_16BE
                    .decode(link, DecoderTrap::Ignore)
                    .unwrap()
                    .to_owned()
            }
            PlatformEncodingLanguageId::Windows(..) => UTF_16BE
                .decode(link, DecoderTrap::Ignore)
                .unwrap()
                .to_owned(),
        }
    }
}

impl TryFrom<(u16, u16, u16)> for PlatformEncodingLanguageId {
    type Error = Local<UnknownValueError<u16>>;

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
    type Error = Local<UnknownValueError<u16>>;

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
    type Error = Local<UnknownValueError<u16>>;

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
    type Error = Local<UnknownValueError<u16>>;

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
    type Error = Local<UnknownValueError<u16>>;

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
    lang_tag: String,
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
type PrepMetrics = RawArrayMetrics;
#[derive(Clone, Debug)]
struct GaspMetrics {
    version: u16,
    num_ranges: usize,
    ranges: Vec<GaspRange>,
}

#[derive(Clone, Copy, Debug)]
struct GaspRange {
    range_max_ppem: u16,
    range_gasp_behavior: GaspBehaviorFlags,
}

// NOTE - Version 1 contains all the fields that version 0 contains, so it can be used as the unifying type
type GaspBehaviorFlags = opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version1;

type CoverageRangeRecord = RangeRecord<u16>;

impl Promote<OpentypeCoverageRangeRecord> for CoverageRangeRecord {
    fn promote(orig: &OpentypeCoverageRangeRecord) -> Self {
        RangeRecord {
            start_glyph_id: orig.start_glyph_id,
            end_glyph_id: orig.end_glyph_id,
            value: orig.start_coverage_index,
        }
    }
}

#[derive(Clone, Debug)]
enum CoverageTable {
    Format1 {
        glyph_array: Vec<u16>,
    }, // Individual glyph indices
    Format2 {
        range_records: Vec<CoverageRangeRecord>,
    }, // Range of glyphs
}

impl Promote<OpentypeCoverageTable> for CoverageTable {
    fn promote(orig: &OpentypeCoverageTable) -> Self {
        Self::promote(&orig.data)
    }
}

impl Promote<OpentypeCoverageTableData> for CoverageTable {
    fn promote(orig: &OpentypeCoverageTableData) -> Self {
        match orig {
            OpentypeCoverageTableData::Format1(opentype_coverage_table_data_Format1 {
                glyph_array,
                ..
            }) => Self::Format1 {
                glyph_array: glyph_array.clone(),
            },
            OpentypeCoverageTableData::Format2(opentype_coverage_table_data_Format2 {
                range_records,
                ..
            }) => Self::Format2 {
                range_records: promote_vec(range_records),
            },
        }
    }
}

#[derive(Clone, Debug)]
struct MarkGlyphSet {
    format: u16,
    mark_glyph_set_count: usize,
    coverage: Vec<Option<CoverageTable>>,
}

type OpentypeMarkGlyphSet = opentype_mark_glyph_set;

impl TryPromote<OpentypeMarkGlyphSet> for MarkGlyphSet {
    type Error = Local<UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeMarkGlyphSet) -> Result<Self, Self::Error> {
        match orig.format {
            1 => {
                let coverage = orig
                    .coverage
                    .iter()
                    .map(|off| off.link.as_ref().map(CoverageTable::promote))
                    .collect();
                Ok(MarkGlyphSet {
                    format: 1,
                    mark_glyph_set_count: orig.mark_glyph_set_count as usize,
                    coverage,
                })
            }
            n => Err(UnknownValueError {
                what: String::from("MarkGlyphSets table format"),
                bad_value: n,
            }),
        }
    }
}

// STUB - this represents the fact that we only record but do not interpret the offset for the ItemVariationStore in the current OpenType implementation
type ItemVariationStore = u32;

impl TryPromote<OpentypeGdefTableData> for GdefTableDataMetrics {
    type Error = ReflType<TPErr<OpentypeMarkGlyphSet, MarkGlyphSet>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeGdefTableData) -> Result<Self, Self::Error> {
        Ok(match orig {
            OpentypeGdefTableData::Version1_0 => Self::NoData,
            OpentypeGdefTableData::Version1_2(opentype_gdef_table_data_Version1_2 {
                mark_glyph_sets_def,
            }) => {
                GdefTableDataMetrics::MarkGlyphSetsDef(try_promote_opt(&mark_glyph_sets_def.link)?)
            }
            OpentypeGdefTableData::Version1_3(opentype_gdef_table_data_Version1_3 {
                item_var_store,
            }) => GdefTableDataMetrics::ItemVarStore(*item_var_store),
        })
    }
}

#[derive(Clone, Debug)]
enum GdefTableDataMetrics {
    NoData,
    MarkGlyphSetsDef(Option<MarkGlyphSet>),
    ItemVarStore(ItemVariationStore),
}

/**
   0 <=> No Glyph Class assigned (implicit default)
   1 <=> Base glyph (single character, spacing glyph)
   2 <=> Ligature glyph (multiple character, spacing glyph)
   3 <=> Mark glyph (non-spacing combining glyph)
   4 <=> Component glyph (part of a single character, spacing glyph)
*/
type GlyphClass = u16; // REVIEW - consider replacing with semantically distinguished const-enum

fn show_glyph_class(gc: &GlyphClass) -> &'static str {
    // REVIEW - to the extent we present this info, decide whether to include semantics, numerals, or both
    match gc {
        0 => "0(none)",
        1 => "1(base)",
        2 => "2(liga)",
        3 => "3(mark)",
        4 => "4(comp)",
        _ => unreachable!("Unexpected glyph class value: {}", gc),
    }
}

#[derive(Clone, Debug)]
enum ClassDef {
    Format1 {
        start_glyph_id: u16,
        class_value_array: Vec<u16>,
    },
    Format2 {
        class_range_records: Vec<ClassRangeRecord>,
    },
}

#[derive(Clone, Copy, Debug)]
struct RangeRecord<T> {
    start_glyph_id: u16,
    end_glyph_id: u16,
    value: T,
}

type ClassRangeRecord = RangeRecord<GlyphClass>;

impl Promote<opentype_class_def_data_Format2_class_range_records> for ClassRangeRecord {
    fn promote(orig: &opentype_class_def_data_Format2_class_range_records) -> Self {
        RangeRecord {
            start_glyph_id: orig.start_glyph_id,
            end_glyph_id: orig.end_glyph_id,
            value: orig.class,
        }
    }
}

impl Promote<opentype_class_def> for ClassDef {
    fn promote(orig: &opentype_class_def) -> Self {
        Self::promote(&orig.data)
    }
}

impl Promote<opentype_class_def_data> for ClassDef {
    fn promote(orig: &opentype_class_def_data) -> Self {
        match orig {
            &opentype_class_def_data::Format1(opentype_class_def_data_Format1 {
                start_glyph_id,
                ref class_value_array,
                ..
            }) => Self::Format1 {
                start_glyph_id,
                class_value_array: class_value_array.clone(),
            },
            &opentype_class_def_data::Format2(opentype_class_def_data_Format2 {
                ref class_range_records,
                ..
            }) => Self::Format2 {
                class_range_records: promote_vec(class_range_records),
            },
        }
    }
}

#[derive(Clone, Debug)]
struct AttachPoint {
    point_indices: Vec<u16>,
}

#[derive(Clone, Debug)]
struct AttachList {
    coverage: CoverageTable,
    attach_points: Vec<AttachPoint>,
}

impl Promote<OpentypeAttachPoint> for AttachPoint {
    fn promote(orig: &OpentypeAttachPoint) -> Self {
        AttachPoint {
            point_indices: orig.point_indices.clone(),
        }
    }
}

type OpentypeAttachList = opentype_gdef_table_attach_list_link;

impl Promote<OpentypeAttachList> for AttachList {
    fn promote(orig: &OpentypeAttachList) -> Self {
        AttachList {
            coverage: CoverageTable::promote(&orig.coverage.link),
            attach_points: orig
                .attach_point_offsets
                .iter()
                .map(|offset| AttachPoint::promote(&offset.link))
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
struct LigCaretList {
    coverage: CoverageTable,
    lig_glyphs: Vec<LigGlyph>,
}

type OpentypeLigCaretList = opentype_gdef_table_lig_caret_list_link;

impl TryPromote<OpentypeLigCaretList> for LigCaretList {
    type Error = ReflType<TPErr<OpentypeLigGlyph, LigGlyph>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeLigCaretList) -> Result<Self, Self::Error> {
        let mut lig_glyphs = Vec::with_capacity(orig.lig_glyph_offsets.len());
        for offset in orig.lig_glyph_offsets.iter() {
            lig_glyphs.push(LigGlyph::try_promote(&offset.link)?);
        }
        Ok(LigCaretList {
            coverage: CoverageTable::promote(&orig.coverage.link),
            lig_glyphs,
        })
    }
}

#[derive(Clone, Debug)]
struct LigGlyph {
    caret_values: Vec<CaretValue>,
}

type OpentypeLigGlyph = opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link;

impl TryPromote<OpentypeLigGlyph> for LigGlyph {
    type Error = ReflType<TPErr<OpentypeCaretValue, CaretValue>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeLigGlyph) -> Result<Self, Self::Error> {
        let mut caret_values = Vec::with_capacity(orig.caret_values.len());
        for offset in orig.caret_values.iter() {
            caret_values.push(CaretValue::try_promote(&offset.link)?); // &caret_value.data
        }
        Ok(LigGlyph { caret_values })
    }
}

// Caret position given as an x- or y-coordinate, as a number of design units
#[derive(Clone, Copy, Debug)]
struct CaretValueDesignUnits {
    coordinate: u16,
}

// Caret position given as the index to a specific contour point on a glyph
#[derive(Clone, Copy, Debug)]
struct ContourPoint {
    index: u16,
}

#[derive(Clone, Debug)]
enum DeltaValues {
    Bits2(Vec<i8>),
    Bits4(Vec<i8>),
    Bits8(Vec<i8>),
}

fn bits<const N: usize>(raw: u8) -> i8 {
    if N == 8 {
        return raw as i8;
    }
    assert!(N > 1 && N < 8);
    let rangemax: i8 = 1 << N;
    let i_raw = raw as i8;
    if i_raw >= rangemax / 2 {
        return i_raw - rangemax;
    } else {
        i_raw
    }
}

impl TryFromRef<(u16, Vec<u16>)> for DeltaValues {
    type Error = Local<UnknownValueError<u16>>;

    fn try_from_ref<'a>(value: (u16, &'a Vec<u16>)) -> Result<Self, Self::Error> {
        match value.0 {
            0x0001 => {
                // 2-bit Deltas
                let mut unpacked_deltas = Vec::with_capacity(8 * value.1.len());
                for packed in value
                    .1
                    .iter()
                    .copied()
                    .flat_map(|word16| word16.to_be_bytes())
                {
                    let hh = bits::<2>(packed >> 6);
                    let hl = bits::<2>(packed >> 4 & 0b11);
                    let lh = bits::<2>(packed >> 2 & 0b11);
                    let ll = bits::<2>(packed & 0b11);
                    unpacked_deltas.extend_from_slice(&[hh, hl, lh, ll]);
                }
                Ok(DeltaValues::Bits2(unpacked_deltas))
            }
            0x0002 => {
                // 4-bit Deltas
                let mut unpacked_deltas = Vec::with_capacity(4 * value.1.len());
                for packed in value
                    .1
                    .iter()
                    .copied()
                    .flat_map(|word16| word16.to_be_bytes())
                {
                    let hi = bits::<4>(packed >> 4);
                    let lo = bits::<4>(packed & 0xf);
                    unpacked_deltas.extend_from_slice(&[hi, lo]);
                }
                Ok(DeltaValues::Bits4(unpacked_deltas))
            }
            0x0003 => {
                // 8-bit Deltas
                let mut unpacked_deltas = Vec::with_capacity(4 * value.1.len());
                for packed in value
                    .1
                    .iter()
                    .copied()
                    .flat_map(|word16| word16.to_be_bytes())
                {
                    unpacked_deltas.push(packed as i8)
                }
                Ok(DeltaValues::Bits8(unpacked_deltas))
            }
            _ => Err(UnknownValueError {
                what: String::from("delta-format"),
                bad_value: value.0,
            }),
        }
    }
}

#[derive(Clone, Debug)]
struct DeviceTable {
    start_size: u16,
    end_size: u16,
    // REVIEW - do we want to unpack the values into i8-valued deltas based on the format?
    delta_values: DeltaValues,
    // Format is implicated by the discriminant of delta_values, so we omit it
}

#[derive(Clone, Debug)]
struct VariationIndexTable {
    delta_set_outer_index: u16,
    delta_set_inner_index: u16,
    // Format := 0x0008 is implicit, so we omit it
}

#[derive(Clone, Debug)]
enum DeviceOrVariationIndexTable {
    DeviceTable(DeviceTable),
    VariationIndexTable(VariationIndexTable),
}

#[derive(Clone, Debug)]
enum CaretValue {
    DesignUnits(u16),  // Format1
    ContourPoint(u16), // Format2
    DesignUnitsWithTable {
        coordinate: u16,
        device: DeviceOrVariationIndexTable,
    }, // Format3
}

type OpentypeCaretValueRaw =
    opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link;
type OpentypeCaretValue =
    opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data;
type OpentypeCaretValueFormat1 =
    opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format1;
type OpentypeCaretValueFormat2 =
    opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format2;
type OpentypeCaretValueFormat3 =
    opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link_caret_values_link_data_Format3;

type OpentypeDeviceOrVariationIndexTable = opentype_common_device_or_variation_index_table;
type OpentypeDeviceTable = opentype_common_device_or_variation_index_table_DeviceTable;
type OpentypeVariationIndexTable =
    opentype_common_device_or_variation_index_table_VariationIndexTable;

impl TryPromote<OpentypeDeviceOrVariationIndexTable> for DeviceOrVariationIndexTable {
    type Error = ReflType<TFRErr<(u16, Vec<u16>), DeltaValues>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeDeviceOrVariationIndexTable) -> Result<Self, Self::Error> {
        match orig {
            &OpentypeDeviceOrVariationIndexTable::DeviceTable(OpentypeDeviceTable {
                start_size,
                end_size,
                delta_format,
                ref delta_values,
            }) => Ok(DeviceOrVariationIndexTable::DeviceTable(DeviceTable {
                start_size,
                end_size,
                delta_values: DeltaValues::try_from_ref((delta_format, delta_values))?,
            })),
            &OpentypeDeviceOrVariationIndexTable::VariationIndexTable(
                OpentypeVariationIndexTable {
                    delta_set_outer_index,
                    delta_set_inner_index,
                    ..
                },
            ) => Ok(DeviceOrVariationIndexTable::VariationIndexTable(
                VariationIndexTable {
                    delta_set_outer_index,
                    delta_set_inner_index,
                },
            )),
        }
    }
}

impl TryPromote<OpentypeCaretValueRaw> for CaretValue {
    type Error = ReflType<TPErr<OpentypeCaretValue, CaretValue>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeCaretValueRaw) -> Result<Self, Self::Error> {
        Self::try_promote(&orig.data)
    }
}

impl TryPromote<OpentypeCaretValue> for CaretValue {
    type Error = ReflType<
        TPErr<OpentypeDeviceOrVariationIndexTable, DeviceOrVariationIndexTable>,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeCaretValue) -> Result<Self, Self::Error> {
        match orig {
            OpentypeCaretValue::Format1(OpentypeCaretValueFormat1 { coordinate }) => {
                Ok(CaretValue::DesignUnits(*coordinate))
            }
            OpentypeCaretValue::Format2(OpentypeCaretValueFormat2 {
                caret_value_point_index,
            }) => Ok(CaretValue::ContourPoint(*caret_value_point_index)),
            OpentypeCaretValue::Format3(OpentypeCaretValueFormat3 { coordinate, table }) => {
                Ok(CaretValue::DesignUnitsWithTable {
                    coordinate: *coordinate,
                    device: DeviceOrVariationIndexTable::try_promote(&table.link)?,
                })
            }
        }
    }
}

#[derive(Clone, Debug)]
struct GdefMetrics {
    major_version: u16,
    minor_version: u16,
    glyph_class_def: Option<ClassDef>,
    attach_list: Option<AttachList>,
    lig_caret_list: Option<LigCaretList>,
    mark_attach_class_def: Option<ClassDef>,
    data: GdefTableDataMetrics,
}

pub type OpentypeLangSys = opentype_common_langsys;

impl Promote<OpentypeLangSys> for LangSys {
    fn promote(orig: &OpentypeLangSys) -> Self {
        LangSys {
            lookup_order_offset: orig.lookup_order_offset,
            required_feature_index: orig.required_feature_index,
            feature_indices: orig.feature_indices.clone(),
        }
    }
}

#[derive(Clone, Debug)]
struct LangSys {
    lookup_order_offset: u16, // should be 0x0000
    required_feature_index: u16,
    feature_indices: Vec<u16>,
}

pub type OpentypeLangSysRecord = opentype_common_script_table_lang_sys_records;

impl Promote<OpentypeLangSysRecord> for LangSysRecord {
    fn promote(orig: &OpentypeLangSysRecord) -> Self {
        LangSysRecord {
            lang_sys_tag: orig.lang_sys_tag,
            lang_sys: LangSys::promote(&orig.lang_sys.link),
        }
    }
}

#[derive(Clone, Debug)]
struct LangSysRecord {
    lang_sys_tag: u32,
    lang_sys: LangSys,
}

pub type OpentypeScriptTable = opentype_common_script_table;

impl Promote<OpentypeScriptTable> for ScriptTable {
    fn promote(orig: &OpentypeScriptTable) -> Self {
        ScriptTable {
            default_lang_sys: promote_opt(&orig.default_lang_sys.link),
            lang_sys_records: promote_vec(&orig.lang_sys_records),
        }
    }
}

#[derive(Clone, Debug)]
struct ScriptTable {
    default_lang_sys: Option<LangSys>,
    lang_sys_records: Vec<LangSysRecord>,
}

pub type OpentypeScriptRecord = opentype_common_script_list_script_records;

impl Promote<OpentypeScriptRecord> for ScriptRecord {
    fn promote(orig: &OpentypeScriptRecord) -> Self {
        ScriptRecord {
            script_tag: orig.script_tag,
            script: ScriptTable::promote(&orig.script.link),
        }
    }
}

#[derive(Clone, Debug)]
struct ScriptRecord {
    script_tag: u32,
    script: ScriptTable,
}

pub type OpentypeFeatureTable = opentype_common_feature_table;

impl Promote<OpentypeFeatureTable> for FeatureTable {
    fn promote(orig: &OpentypeFeatureTable) -> Self {
        FeatureTable {
            feature_params: orig.feature_params,
            lookup_list_indices: orig.lookup_list_indices.clone(),
        }
    }
}

#[derive(Clone, Debug)]
struct FeatureTable {
    feature_params: u16,
    lookup_list_indices: Vec<u16>,
}

pub type OpentypeFeatureRecord = opentype_common_feature_list_feature_records;

impl Promote<OpentypeFeatureRecord> for FeatureRecord {
    fn promote(orig: &OpentypeFeatureRecord) -> Self {
        FeatureRecord {
            feature_tag: orig.feature_tag,
            feature: FeatureTable::promote(&orig.feature.link),
        }
    }
}

#[derive(Clone, Debug)]
struct FeatureRecord {
    feature_tag: u32,
    feature: FeatureTable,
}

pub type OpentypeGposLookupSubtable =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link;
pub type OpentypeGsubLookupSubtable =
    opentype_gsub_table_lookup_list_link_lookups_link_subtables_link;

impl TryPromote<OpentypeGsubLookupSubtable> for LookupSubtable {
    type Error = std::convert::Infallible; // this is only temporarily the case, as we are almost certainly going to have errors in at least on lookup subtable format

    fn try_promote(orig: &OpentypeGsubLookupSubtable) -> Result<Self, Self::Error> {
        Ok(match orig {
            OpentypeGsubLookupSubtable::SingleSubst(single_subst) => {
                LookupSubtable::SingleSubst(SingleSubst::promote(single_subst))
            }
            OpentypeGsubLookupSubtable::MultipleSubst => LookupSubtable::MultipleSubst,
            OpentypeGsubLookupSubtable::AlternateSubst => LookupSubtable::AlternateSubst,
            OpentypeGsubLookupSubtable::LigatureSubst => LookupSubtable::LigatureSubst,
            OpentypeGsubLookupSubtable::SequenceContext(seq_ctx) => {
                LookupSubtable::SequenceContext(SequenceContext::promote(seq_ctx))
            }
            OpentypeGsubLookupSubtable::ChainedSequenceContext(chain_ctx) => {
                LookupSubtable::ChainedSequenceContext(ChainedSequenceContext::promote(chain_ctx))
            }
            OpentypeGsubLookupSubtable::SubstExtension => LookupSubtable::SubstExtension,
            OpentypeGsubLookupSubtable::ReverseChainSingleSubst => {
                LookupSubtable::ReverseChainSingleSubst
            }
        })
    }
}

impl TryPromote<OpentypeGposLookupSubtable> for LookupSubtable {
    type Error = ReflType<
        ReflType<TPErr<OpentypeSinglePos, SinglePos>, TPErr<OpentypePairPos, PairPos>>,
        ReflType<TPErr<OpentypeCursivePos, CursivePos>, UnknownValueError<u16>>,
    >;

    fn try_promote(orig: &OpentypeGposLookupSubtable) -> Result<Self, Self::Error> {
        Ok(match orig {
            OpentypeGposLookupSubtable::SinglePos(single_pos) => {
                LookupSubtable::SinglePos(SinglePos::try_promote(single_pos)?)
            }
            OpentypeGposLookupSubtable::PairPos(pair_pos) => {
                LookupSubtable::PairPos(PairPos::try_promote(pair_pos)?)
            }
            OpentypeGposLookupSubtable::CursivePos(cursive_pos) => {
                LookupSubtable::CursivePos(CursivePos::try_promote(cursive_pos)?)
            }
            OpentypeGposLookupSubtable::MarkBasePos => LookupSubtable::MarkBasePos,
            OpentypeGposLookupSubtable::MarkLigPos => LookupSubtable::MarkLigPos,
            OpentypeGposLookupSubtable::MarkMarkPos => LookupSubtable::MarkMarkPos,
            OpentypeGposLookupSubtable::SequenceContext(seq_ctx) => {
                LookupSubtable::SequenceContext(SequenceContext::promote(seq_ctx))
            }
            OpentypeGposLookupSubtable::ChainedSequenceContext(chain_ctx) => {
                LookupSubtable::ChainedSequenceContext(ChainedSequenceContext::promote(chain_ctx))
            }
            OpentypeGposLookupSubtable::PosExtension => LookupSubtable::PosExtension,
        })
    }
}

#[derive(Clone, Debug)]
enum LookupSubtable {
    SinglePos(SinglePos),
    PairPos(PairPos),
    CursivePos(CursivePos),
    MarkBasePos,
    MarkLigPos,
    MarkMarkPos,
    PosExtension,

    SequenceContext(SequenceContext),
    ChainedSequenceContext(ChainedSequenceContext),

    SingleSubst(SingleSubst),
    MultipleSubst,
    AlternateSubst,
    LigatureSubst,
    SubstExtension,
    ReverseChainSingleSubst,
}

pub type OpentypeSingleSubst =
    opentype_gsub_table_lookup_list_link_lookups_link_subtables_link_SingleSubst;
pub type OpentypeSingleSubstInner =
    opentype_gsub_table_lookup_list_link_lookups_link_subtables_link_SingleSubst_subst;
pub type OpentypeSingleSubstFormat1 =
    opentype_gsub_table_lookup_list_link_lookups_link_subtables_link_SingleSubst_subst_Format1;
pub type OpentypeSingleSubstFormat2 =
    opentype_gsub_table_lookup_list_link_lookups_link_subtables_link_SingleSubst_subst_Format2;

impl Promote<OpentypeSingleSubst> for SingleSubst {
    fn promote(orig: &OpentypeSingleSubst) -> Self {
        SingleSubst::promote(&orig.subst)
    }
}

impl Promote<OpentypeSingleSubstInner> for SingleSubst {
    fn promote(orig: &OpentypeSingleSubstInner) -> Self {
        match orig {
            OpentypeSingleSubstInner::Format1(f1) => {
                SingleSubst::Format1(SingleSubstFormat1::promote(f1))
            }
            OpentypeSingleSubstInner::Format2(f2) => {
                SingleSubst::Format2(SingleSubstFormat2::promote(f2))
            }
        }
    }
}

#[derive(Debug, Clone)]
enum SingleSubst {
    Format1(SingleSubstFormat1),
    Format2(SingleSubstFormat2),
}

impl Promote<OpentypeSingleSubstFormat1> for SingleSubstFormat1 {
    fn promote(orig: &OpentypeSingleSubstFormat1) -> Self {
        SingleSubstFormat1 {
            coverage: CoverageTable::promote(&orig.coverage.link),
            delta_glyph_id: orig.delta_glyph_id,
        }
    }
}

#[derive(Debug, Clone)]
struct SingleSubstFormat1 {
    coverage: CoverageTable,
    delta_glyph_id: s16,
}

impl Promote<OpentypeSingleSubstFormat2> for SingleSubstFormat2 {
    fn promote(orig: &OpentypeSingleSubstFormat2) -> Self {
        SingleSubstFormat2 {
            coverage: CoverageTable::promote(&orig.coverage.link),
            substitute_glyph_ids: orig.substitute_glyph_ids.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct SingleSubstFormat2 {
    coverage: CoverageTable,
    substitute_glyph_ids: Vec<u16>,
}

pub type OpentypeChainedSequenceContext = opentype_common_chained_sequence_context;
pub type OpentypeChainedSequenceContextInner = opentype_common_chained_sequence_context_subst;
pub type OpentypeChainedSequenceContextFormat1 =
    opentype_common_chained_sequence_context_subst_Format1;
pub type OpentypeChainedSequenceContextFormat2 =
    opentype_common_chained_sequence_context_subst_Format2;
pub type OpentypeChainedSequenceContextFormat3 =
    opentype_common_chained_sequence_context_subst_Format3;

pub type OpentypeChainedRuleSet =
    opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link;
pub type OpentypeChainedRule = opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link_chained_seq_rules;

impl<Sem> Promote<OpentypeChainedRuleSet> for ChainedRuleSet<Sem>
where
    ChainedRule<Sem>: Promote<OpentypeChainedRule>,
{
    fn promote(orig: &OpentypeChainedRuleSet) -> Self {
        promote_vec(&orig.chained_seq_rules)
    }
}

impl<Sem> Promote<OpentypeChainedRule> for ChainedRule<Sem> {
    fn promote(orig: &OpentypeChainedRule) -> Self {
        ChainedRule {
            backtrack_glyph_count: orig.backtrack_glyph_count,
            backtrack_sequence: orig.backtrack_sequence.clone().into(),
            input_glyph_count: orig.input_glyph_count,
            input_sequence: orig.input_sequence.clone().into(),
            lookahead_glyph_count: orig.lookahead_glyph_count,
            lookahead_sequence: orig.lookahead_sequence.clone().into(),
            seq_lookup_records: promote_vec(&orig.seq_lookup_records),
        }
    }
}

type ChainedRuleSet<Sem> = Vec<ChainedRule<Sem>>;

#[derive(Clone)]
struct ChainedRule<Sem> {
    backtrack_glyph_count: u16, // REVIEW - this field can be re-synthesized from backtrack_sequence.len()
    backtrack_sequence: SemVec<Sem, u16>,
    input_glyph_count: u16, // REVIEW - this field can be re-synthesized from input_sequence.len() + 1
    input_sequence: SemVec<Sem, u16>, // NOTE - unlike the other two sequence-arrays, this one is one shorter than its associated glyph_count field
    lookahead_glyph_count: u16, // REVIEW - this field can be re-synthesized from lookahead_sequence.len()
    lookahead_sequence: SemVec<Sem, u16>,
    seq_lookup_records: Vec<SequenceLookup>,
}

impl std::fmt::Debug for ChainedRule<GlyphId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // REVIEW - at the debug level, present a view of ChainedRule<GlyphId> as if it were its own type `ChainedSequenceRule`
        f.debug_struct("ChainedSequenceRule")
            .field("backtrack_glyph_count", &self.backtrack_glyph_count)
            .field("backtrack_sequence", &self.backtrack_sequence)
            .field("input_glyph_count", &self.input_glyph_count)
            .field("input_sequence", &self.input_sequence)
            .field("lookahead_glyph_count", &self.lookahead_glyph_count)
            .field("lookahead_sequence", &self.lookahead_sequence)
            .field("seq_lookup_records", &self.seq_lookup_records)
            .finish()
    }
}

impl std::fmt::Debug for ChainedRule<ClassId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // REVIEW - at the debug level, present a view of ChainedRule<ClassId> as if it were its own type `ChainedClassSequenceRule`
        f.debug_struct("ChainedClassSequenceRule")
            .field("backtrack_glyph_count", &self.backtrack_glyph_count)
            .field("backtrack_sequence", &self.backtrack_sequence)
            .field("input_glyph_count", &self.input_glyph_count)
            .field("input_sequence", &self.input_sequence)
            .field("lookahead_glyph_count", &self.lookahead_glyph_count)
            .field("lookahead_sequence", &self.lookahead_sequence)
            .field("seq_lookup_records", &self.seq_lookup_records)
            .finish()
    }
}

impl Promote<OpentypeChainedSequenceContext> for ChainedSequenceContext {
    fn promote(orig: &OpentypeChainedSequenceContext) -> Self {
        Self::promote(&orig.subst)
    }
}

impl Promote<OpentypeChainedSequenceContextInner> for ChainedSequenceContext {
    fn promote(orig: &OpentypeChainedSequenceContextInner) -> Self {
        match orig {
            OpentypeChainedSequenceContextInner::Format1(f1) => {
                ChainedSequenceContext::Format1(ChainedSequenceContextFormat1::promote(f1))
            }
            OpentypeChainedSequenceContextInner::Format2(f2) => {
                ChainedSequenceContext::Format2(ChainedSequenceContextFormat2::promote(f2))
            }
            OpentypeChainedSequenceContextInner::Format3(f3) => {
                ChainedSequenceContext::Format3(ChainedSequenceContextFormat3::promote(f3))
            }
        }
    }
}

#[derive(Debug, Clone)]
enum ChainedSequenceContext {
    Format1(ChainedSequenceContextFormat1),
    Format2(ChainedSequenceContextFormat2),
    Format3(ChainedSequenceContextFormat3),
}

impl Promote<OpentypeChainedSequenceContextFormat1> for ChainedSequenceContextFormat1 {
    fn promote(orig: &OpentypeChainedSequenceContextFormat1) -> Self {
        ChainedSequenceContextFormat1 {
            coverage: CoverageTable::promote(&orig.coverage.link),
            chained_seq_rule_sets: orig
                .chained_seq_rule_sets
                .iter()
                .map(promote_link!())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct ChainedSequenceContextFormat1 {
    coverage: CoverageTable,
    chained_seq_rule_sets: Vec<Option<ChainedRuleSet<GlyphId>>>,
}

impl Promote<OpentypeChainedSequenceContextFormat2> for ChainedSequenceContextFormat2 {
    fn promote(orig: &OpentypeChainedSequenceContextFormat2) -> Self {
        Self {
            coverage: CoverageTable::promote(&orig.coverage.link),
            backtrack_class_def: ClassDef::promote(&orig.backtrack_class_def.link),
            input_class_def: ClassDef::promote(&orig.input_class_def.link),
            lookahead_class_def: ClassDef::promote(&orig.lookahead_class_def.link),
            chained_class_seq_rule_sets: orig
                .chained_class_seq_rule_sets
                .iter()
                .map(promote_link!())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct ChainedSequenceContextFormat2 {
    coverage: CoverageTable,
    backtrack_class_def: ClassDef,
    input_class_def: ClassDef,
    lookahead_class_def: ClassDef,
    chained_class_seq_rule_sets: Vec<Option<ChainedRuleSet<ClassId>>>,
}

impl Promote<OpentypeChainedSequenceContextFormat3> for ChainedSequenceContextFormat3 {
    fn promote(orig: &OpentypeChainedSequenceContextFormat3) -> Self {
        type OpentypeCoverageTableLink =
            opentype_common_chained_sequence_context_subst_Format1_coverage;
        let follow = |covs: &Vec<OpentypeCoverageTableLink>| -> Vec<CoverageTable> {
            covs.iter()
                .map(|offset| CoverageTable::promote(&offset.link))
                .collect()
        };
        Self {
            backtrack_glyph_count: orig.backtrack_glyph_count,
            backtrack_coverages: follow(&orig.backtrack_coverages),
            input_glyph_count: orig.input_glyph_count,
            input_coverages: follow(&orig.input_coverages),
            lookahead_glyph_count: orig.lookahead_glyph_count,
            lookahead_coverages: follow(&orig.lookahead_coverages),
            seq_lookup_records: promote_vec(&orig.seq_lookup_records),
        }
    }
}

#[derive(Debug, Clone)]
struct ChainedSequenceContextFormat3 {
    backtrack_glyph_count: u16, // REVIEW - this field can be re-synthesized from `backtrack_coverages.len()`
    backtrack_coverages: Vec<CoverageTable>,
    input_glyph_count: u16, // REVIEW - this field can be re-synthesized from `input_coverages.len()`
    input_coverages: Vec<CoverageTable>,
    lookahead_glyph_count: u16, // REVIEW - this field can be re-synthesized from `lookahead_coverages.len()`
    lookahead_coverages: Vec<CoverageTable>,
    seq_lookup_records: Vec<SequenceLookup>,
}

pub type OpentypeSequenceContext = opentype_common_sequence_context;
pub type OpentypeSequenceContextInner = opentype_common_sequence_context_subst;
pub type OpentypeSequenceContextFormat1 = opentype_common_sequence_context_subst_Format1;
pub type OpentypeSequenceContextFormat2 = opentype_common_sequence_context_subst_Format2;
pub type OpentypeSequenceContextFormat3 = opentype_common_sequence_context_subst_Format3;

impl Promote<OpentypeSequenceContext> for SequenceContext {
    fn promote(orig: &OpentypeSequenceContext) -> Self {
        // FIXME - if we rename the field `subst`, fix this
        SequenceContext::promote(&orig.subst)
    }
}

impl Promote<OpentypeSequenceContextInner> for SequenceContext {
    fn promote(orig: &OpentypeSequenceContextInner) -> Self {
        match orig {
            OpentypeSequenceContextInner::Format1(f1) => {
                SequenceContext::Format1(SequenceContextFormat1::promote(f1))
            }
            OpentypeSequenceContextInner::Format2(f2) => {
                SequenceContext::Format2(SequenceContextFormat2::promote(f2))
            }
            OpentypeSequenceContextInner::Format3(f3) => {
                SequenceContext::Format3(SequenceContextFormat3::promote(f3))
            }
        }
    }
}

#[derive(Debug, Clone)]
enum SequenceContext {
    Format1(SequenceContextFormat1),
    Format2(SequenceContextFormat2),
    Format3(SequenceContextFormat3),
}

pub type OpentypeSequenceLookup = opentype_common_sequence_lookup;
type SequenceLookup = OpentypeSequenceLookup;

impl Promote<OpentypeSequenceContextFormat1> for SequenceContextFormat1 {
    fn promote(orig: &OpentypeSequenceContextFormat1) -> Self {
        Self {
            coverage: CoverageTable::promote(&orig.coverage.link),
            seq_rule_sets: orig.seq_rule_sets.iter().map(promote_link!()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct SequenceContextFormat1 {
    coverage: CoverageTable,
    seq_rule_sets: Vec<Option<RuleSet>>,
}

impl Promote<OpentypeSequenceContextFormat2> for SequenceContextFormat2 {
    fn promote(orig: &OpentypeSequenceContextFormat2) -> Self {
        Self {
            coverage: CoverageTable::promote(&orig.coverage.link),
            class_def: ClassDef::promote(&orig.class_def.link),
            class_seq_rule_sets: orig
                .class_seq_rule_sets
                .iter()
                .map(promote_link!())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct SequenceContextFormat2 {
    coverage: CoverageTable,
    class_def: ClassDef,
    class_seq_rule_sets: Vec<Option<RuleSet>>,
}

impl Promote<OpentypeSequenceContextFormat3> for SequenceContextFormat3 {
    fn promote(orig: &OpentypeSequenceContextFormat3) -> Self {
        Self {
            glyph_count: orig.glyph_count,
            coverage_tables: orig
                .coverage_tables
                .iter()
                .map(|offset| CoverageTable::promote(&offset.link))
                .collect(),
            // NOTE - can only clone here (instead of calling promote) because SequenceLookup := OpentypeSequenceLookup
            // REVIEW - given that Clone => Promote<Self>, do we want to abstract this to avoid the need to refactor if SequenceLookup is redefined (with a manual Promote impl)?
            seq_lookup_records: orig.seq_lookup_records.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct SequenceContextFormat3 {
    glyph_count: u16,
    coverage_tables: Vec<CoverageTable>,
    seq_lookup_records: Vec<SequenceLookup>,
}

pub type OpentypeRuleSet = opentype_common_sequence_context_subst_Format1_seq_rule_sets_link;

// REVIEW - if RuleSet becomes an alias instead of a newtype, remove this definition and rename the following impl on Vec<Option<Rule>>
impl Promote<OpentypeRuleSet> for RuleSet {
    fn promote(orig: &OpentypeRuleSet) -> Self {
        Self(<Vec<Rule>>::promote(orig))
    }
}

impl Promote<OpentypeRuleSet> for Vec<Rule> {
    fn promote(orig: &OpentypeRuleSet) -> Self {
        orig.rules
            .iter()
            .map(|offset| Rule::promote(&offset.link))
            .collect()
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
// REVIEW - should this be a simple type alias instead?
struct RuleSet(Vec<Rule>);

pub type OpentypeRule =
    opentype_common_sequence_context_subst_Format1_seq_rule_sets_link_rules_link;

impl Promote<OpentypeRule> for Rule {
    fn promote(orig: &OpentypeRule) -> Self {
        Rule {
            glyph_count: orig.glyph_count,
            input_sequence: orig.input_sequence.clone(),
            // NOTE - we can only specify seq_lookup_records this way because we use SequenceLookup as its own analogue
            seq_lookup_records: orig.seq_lookup_records.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    glyph_count: u16, // REVIEW - this field can be re-synthesized via `input_sequence.len() + 1`
    input_sequence: Vec<u16>,
    seq_lookup_records: Vec<SequenceLookup>,
}

// NOTE - forgoing a type-analogue as it would be a carbon copy of the machine-generated type

pub type OpentypeCursivePos =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_CursivePos;
pub type OpentypeCursivePosSubtable =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_CursivePos_subtable;
pub type OpentypeCursivePosFormat1 =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_CursivePos_subtable_Format1;

impl TryPromote<OpentypeCursivePos> for CursivePos {
    type Error = ReflType<TPErr<OpentypeCursivePosSubtable, CursivePos>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeCursivePos) -> Result<Self, Self::Error> {
        CursivePos::try_promote(&orig.subtable)
    }
}

impl TryPromote<OpentypeCursivePosSubtable> for CursivePos {
    type Error =
        ReflType<TPErr<OpentypeCursivePosFormat1, CursivePosFormat1>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeCursivePosSubtable) -> Result<Self, Self::Error> {
        match orig {
            OpentypeCursivePosSubtable::Format1(f1) => {
                Ok(CursivePos::Format1(CursivePosFormat1::try_promote(f1)?))
            }
        }
    }
}

#[derive(Debug, Clone)]
enum CursivePos {
    Format1(CursivePosFormat1),
}

impl TryPromote<OpentypeCursivePosFormat1> for CursivePosFormat1 {
    type Error = ReflType<TPErr<OpentypeEntryExitRecord, EntryExitRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeCursivePosFormat1) -> Result<Self, Self::Error> {
        Ok(CursivePosFormat1 {
            coverage: CoverageTable::promote(&orig.coverage.link),
            entry_exit_records: try_promote_vec(&orig.entry_exit_records)?,
        })
    }
}

#[derive(Debug, Clone)]
struct CursivePosFormat1 {
    coverage: CoverageTable,
    entry_exit_records: Vec<EntryExitRecord>,
}

pub type OpentypeEntryExitRecord = opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_CursivePos_subtable_Format1_entry_exit_records;

impl TryPromote<OpentypeEntryExitRecord> for EntryExitRecord {
    type Error = ReflType<TPErr<OpentypeAnchorTable, AnchorTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeEntryExitRecord) -> Result<Self, Self::Error> {
        Ok(EntryExitRecord {
            entry_anchor: try_promote_opt(&orig.entry_anchor.link)?,
            exit_anchor: try_promote_opt(&orig.exit_anchor.link)?,
        })
    }
}

#[derive(Debug, Clone)]
struct EntryExitRecord {
    entry_anchor: Option<AnchorTable>,
    exit_anchor: Option<AnchorTable>,
}

pub type OpentypeAnchorTable = opentype_common_anchor_table;
pub type OpentypeAnchorTableTable = opentype_common_anchor_table_table;

pub type OpentypeAnchorTableFormat1 = opentype_common_anchor_table_table_Format1;
pub type OpentypeAnchorTableFormat2 = opentype_common_anchor_table_table_Format2;
pub type OpentypeAnchorTableFormat3 = opentype_common_anchor_table_table_Format3;

impl TryPromote<OpentypeAnchorTable> for AnchorTable {
    type Error = ReflType<TPErr<OpentypeAnchorTableTable, AnchorTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeAnchorTable) -> Result<Self, Self::Error> {
        AnchorTable::try_promote(&orig.table)
    }
}

impl TryPromote<OpentypeAnchorTableTable> for AnchorTable {
    type Error =
        ReflType<TPErr<OpentypeAnchorTableFormat3, AnchorTableFormat3>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeAnchorTableTable) -> Result<Self, Self::Error> {
        Ok(match orig {
            OpentypeAnchorTableTable::Format1(f1) => {
                AnchorTable::Format1(AnchorTableFormat1::promote(f1))
            }
            OpentypeAnchorTableTable::Format2(f2) => {
                AnchorTable::Format2(AnchorTableFormat2::promote(f2))
            }
            OpentypeAnchorTableTable::Format3(f3) => {
                AnchorTable::Format3(AnchorTableFormat3::try_promote(f3)?)
            }
        })
    }
}

#[derive(Debug, Clone)]
enum AnchorTable {
    Format1(AnchorTableFormat1),
    Format2(AnchorTableFormat2),
    Format3(AnchorTableFormat3),
}

// TODO - s16be Format , so change to i16 when appropriate
#[allow(non_camel_case_types)]
/// Scaffolding type to allow for convenient switch-over from u16 to i16 on field parsed as s16[be]
type s16 = u16;

impl Promote<OpentypeAnchorTableFormat1> for AnchorTableFormat1 {
    fn promote(orig: &OpentypeAnchorTableFormat1) -> Self {
        AnchorTableFormat1 {
            x_coordinate: orig.x_coordinate,
            y_coordinate: orig.y_coordinate,
        }
    }
}

impl Promote<OpentypeAnchorTableFormat2> for AnchorTableFormat2 {
    fn promote(orig: &OpentypeAnchorTableFormat2) -> Self {
        AnchorTableFormat2 {
            x_coordinate: orig.x_coordinate,
            y_coordinate: orig.y_coordinate,
            anchor_point: orig.anchor_point,
        }
    }
}

impl TryPromote<OpentypeAnchorTableFormat3> for AnchorTableFormat3 {
    type Error = ReflType<
        TPErr<OpentypeDeviceOrVariationIndexTable, DeviceOrVariationIndexTable>,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeAnchorTableFormat3) -> Result<Self, Self::Error> {
        Ok(AnchorTableFormat3 {
            x_coordinate: orig.x_coordinate,
            y_coordinate: orig.y_coordinate,
            x_device: try_promote_opt(&orig.x_device_offset.link)?,
            y_device: try_promote_opt(&orig.y_device_offset.link)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AnchorTableFormat1 {
    x_coordinate: s16,
    y_coordinate: s16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AnchorTableFormat2 {
    x_coordinate: s16,
    y_coordinate: s16,
    anchor_point: u16,
}

#[derive(Debug, Clone)]
struct AnchorTableFormat3 {
    x_coordinate: s16,
    y_coordinate: s16,
    x_device: Option<DeviceOrVariationIndexTable>,
    y_device: Option<DeviceOrVariationIndexTable>,
}

pub type OpentypePairPos = opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_PairPos;

pub type OpentypePairPosSubtable =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_PairPos_subtable;
pub type OpentypePairPosFormat1 =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_PairPos_subtable_Format1;
pub type OpentypePairPosFormat2 =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_PairPos_subtable_Format2;

impl TryPromote<OpentypePairPos> for PairPos {
    type Error = ReflType<TPErr<OpentypePairPosSubtable, PairPos>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePairPos) -> Result<Self, Self::Error> {
        Self::try_promote(&orig.subtable)
    }
}

impl TryPromote<OpentypePairPosSubtable> for PairPos {
    type Error = ReflType<
        ReflType<
            TPErr<OpentypePairPosFormat1, PairPosFormat1>,
            TPErr<OpentypePairPosFormat2, PairPosFormat2>,
        >,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypePairPosSubtable) -> Result<Self, Self::Error> {
        Ok(match orig {
            OpentypePairPosSubtable::Format1(f1) => {
                PairPos::Format1(PairPosFormat1::try_promote(f1)?)
            }
            OpentypePairPosSubtable::Format2(f2) => {
                PairPos::Format2(PairPosFormat2::try_promote(f2)?)
            }
        })
    }
}

#[derive(Debug, Clone)]
enum PairPos {
    Format1(PairPosFormat1),
    Format2(PairPosFormat2),
}

impl TryPromote<OpentypePairPosFormat1> for PairPosFormat1 {
    type Error = ReflType<TPErr<OpentypePairSet, PairSet>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePairPosFormat1) -> Result<Self, Self::Error> {
        let mut pair_sets = Vec::with_capacity(orig.pair_sets.len());
        for offset in orig.pair_sets.iter() {
            let pair_set = PairSet::try_promote(&offset.link)?;
            pair_sets.push(pair_set)
        }

        Ok(PairPosFormat1 {
            coverage: CoverageTable::promote(&orig.coverage.link),
            pair_sets,
        })
    }
}

impl TryPromote<OpentypePairPosFormat2> for PairPosFormat2 {
    type Error = ReflType<TPErr<OpentypeClass2Record, Class2Record>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePairPosFormat2) -> Result<Self, Self::Error> {
        let mut store = Vec::with_capacity(orig.class1_count as usize * orig.class2_count as usize);

        for class1_record in orig.class1_records.iter() {
            for class2_record in class1_record.class2_records.iter() {
                store.push(Class2Record::try_promote(class2_record)?);
            }
        }
        let class1_records = Wec::from_vec(store, orig.class2_count as usize);

        Ok(PairPosFormat2 {
            coverage: CoverageTable::promote(&orig.coverage.link),
            class_def1: ClassDef::promote(&orig.class_def1.link),
            class_def2: ClassDef::promote(&orig.class_def2.link),
            class1_records,
        })
    }
}

#[derive(Debug, Clone)]
struct PairPosFormat1 {
    coverage: CoverageTable,
    pair_sets: Vec<PairSet>,
}

#[derive(Debug, Clone)]
struct PairPosFormat2 {
    coverage: CoverageTable,
    class_def1: ClassDef,
    class_def2: ClassDef,
    class1_records: Class1RecordList,
}

type Class1RecordList = Wec<Class2Record>;

pub type OpentypeClass2Record = opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_PairPos_subtable_Format2_class1_records_class2_records;

impl TryPromote<OpentypeClass2Record> for Class2Record {
    type Error = ReflType<TPErr<OpentypeValueRecord, ValueRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeClass2Record) -> Result<Self, Self::Error> {
        Ok(Class2Record {
            value_record1: try_promote_opt(&orig.value_record1)?,
            value_record2: try_promote_opt(&orig.value_record2)?,
        })
    }
}

#[derive(Debug, Clone)]
struct Class2Record {
    value_record1: Option<ValueRecord>,
    value_record2: Option<ValueRecord>,
}

pub type OpentypePairSet = opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_PairPos_subtable_Format1_pair_sets_link;
pub type OpentypePairValueRecord = opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_PairPos_subtable_Format1_pair_sets_link_pair_value_records;

type PairSet = Vec<PairValueRecord>;

impl TryPromote<OpentypePairSet> for PairSet {
    type Error = ReflType<TPErr<OpentypePairValueRecord, PairValueRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePairSet) -> Result<Self, Self::Error> {
        let mut accum = Vec::with_capacity(orig.pair_value_records.len());
        for record in orig.pair_value_records.iter() {
            accum.push(PairValueRecord::try_promote(record)?);
        }
        Ok(accum)
    }
}

impl TryPromote<OpentypePairValueRecord> for PairValueRecord {
    type Error = ReflType<TPErr<OpentypeValueRecord, ValueRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePairValueRecord) -> Result<Self, Self::Error> {
        Ok(PairValueRecord {
            second_glyph: orig.second_glyph,
            value_record1: try_promote_opt(&orig.value_record1)?,
            value_record2: try_promote_opt(&orig.value_record2)?,
        })
    }
}

#[derive(Debug, Clone)]
struct PairValueRecord {
    second_glyph: u16,
    value_record1: Option<ValueRecord>,
    value_record2: Option<ValueRecord>,
}

pub type OpentypeSinglePos =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SinglePos;
pub type OpentypeSinglePosSubtable =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SinglePos_subtable;

pub type OpentypeSinglePosFormat1 =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SinglePos_subtable_Format1;
pub type OpentypeSinglePosFormat2 =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link_SinglePos_subtable_Format2;

impl TryPromote<OpentypeSinglePos> for SinglePos {
    type Error = ReflType<TPErr<OpentypeSinglePosSubtable, SinglePos>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeSinglePos) -> Result<Self, Self::Error> {
        Self::try_promote(&orig.subtable)
    }
}

impl TryPromote<OpentypeSinglePosSubtable> for SinglePos {
    type Error = ReflType<
        ReflType<
            TPErr<OpentypeSinglePosFormat1, SinglePosFormat1>,
            TPErr<OpentypeSinglePosFormat2, SinglePosFormat2>,
        >,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeSinglePosSubtable) -> Result<Self, Self::Error> {
        Ok(match orig {
            OpentypeSinglePosSubtable::Format1(f1) => {
                SinglePos::Format1(SinglePosFormat1::try_promote(f1)?)
            }
            OpentypeSinglePosSubtable::Format2(f2) => {
                SinglePos::Format2(SinglePosFormat2::try_promote(f2)?)
            }
        })
    }
}

#[derive(Debug, Clone)]
enum SinglePos {
    Format1(SinglePosFormat1),
    Format2(SinglePosFormat2),
}

impl TryPromote<OpentypeSinglePosFormat1> for SinglePosFormat1 {
    type Error = ReflType<TPErr<OpentypeValueRecord, ValueRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeSinglePosFormat1) -> Result<Self, Self::Error> {
        Ok(SinglePosFormat1 {
            coverage: CoverageTable::promote(&orig.coverage_offset.link),
            value_record: ValueRecord::try_promote(&orig.value_record)?,
        })
    }
}

impl TryPromote<OpentypeSinglePosFormat2> for SinglePosFormat2 {
    type Error = ReflType<TPErr<OpentypeValueRecord, ValueRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeSinglePosFormat2) -> Result<Self, Self::Error> {
        let mut value_records = Vec::with_capacity(orig.value_records.len());
        for value_record in orig.value_records.iter() {
            value_records.push(ValueRecord::try_promote(value_record)?);
        }
        Ok(SinglePosFormat2 {
            coverage: CoverageTable::promote(&orig.coverage_offset.link),
            value_records,
        })
    }
}

#[derive(Debug, Clone)]
struct SinglePosFormat1 {
    coverage: CoverageTable,
    value_record: ValueRecord,
}

#[derive(Debug, Clone)]
struct SinglePosFormat2 {
    coverage: CoverageTable,
    value_records: Vec<ValueRecord>,
}

pub type OpentypeValueRecord = opentype_common_value_record;

impl TryPromote<OpentypeValueRecord> for ValueRecord {
    type Error = ReflType<
        TPErr<OpentypeDeviceOrVariationIndexTable, DeviceOrVariationIndexTable>,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeValueRecord) -> Result<Self, Self::Error> {
        Ok(ValueRecord {
            x_placement: orig.x_placement.map(as_s16),
            y_placement: orig.y_placement.map(as_s16),
            x_advance: orig.x_advance.map(as_s16),
            y_advance: orig.y_advance.map(as_s16),
            x_placement_device: orig
                .x_placement_device
                .as_ref()
                .map(|offset| DeviceOrVariationIndexTable::try_promote(&offset.link))
                .transpose()?,
            y_placement_device: orig
                .y_placement_device
                .as_ref()
                .map(|offset| DeviceOrVariationIndexTable::try_promote(&offset.link))
                .transpose()?,
            x_advance_device: orig
                .x_advance_device
                .as_ref()
                .map(|offset| DeviceOrVariationIndexTable::try_promote(&offset.link))
                .transpose()?,
            y_advance_device: orig
                .y_advance_device
                .as_ref()
                .map(|offset| DeviceOrVariationIndexTable::try_promote(&offset.link))
                .transpose()?,
        })
    }
}

#[derive(Debug, Clone)]
struct ValueRecord {
    x_placement: Option<i16>,
    y_placement: Option<i16>,
    x_advance: Option<i16>,
    y_advance: Option<i16>,
    x_placement_device: Option<DeviceOrVariationIndexTable>,
    y_placement_device: Option<DeviceOrVariationIndexTable>,
    x_advance_device: Option<DeviceOrVariationIndexTable>,
    y_advance_device: Option<DeviceOrVariationIndexTable>,
}

type LookupFlag = opentype_gsub_table_lookup_list_link_lookups_link_lookup_flag;

pub type OpentypeGposLookupTable = opentype_gpos_table_lookup_list_link_lookups_link;
pub type OpentypeGsubLookupTable = opentype_gsub_table_lookup_list_link_lookups_link;

impl TryPromote<OpentypeGposLookupTable> for LookupTable {
    type Error =
        ReflType<TPErr<OpentypeGposLookupSubtable, LookupSubtable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeGposLookupTable) -> Result<Self, Self::Error> {
        let mut subtables = Vec::with_capacity(orig.subtables.len());
        for offset in orig.subtables.iter() {
            subtables.push(LookupSubtable::try_promote(&offset.link)?);
        }

        Ok(LookupTable {
            lookup_type: orig.lookup_type,
            lookup_flag: orig.lookup_flag,
            subtables,
            mark_filtering_set: orig.mark_filtering_set,
        })
    }
}

impl TryPromote<OpentypeGsubLookupTable> for LookupTable {
    type Error = ReflType<
        TPErr<OpentypeGsubLookupSubtable, LookupSubtable>,
        std::convert::Infallible,
        // may easily become `UnknownValueError<u16>` but infallible for now
    >;

    fn try_promote(orig: &OpentypeGsubLookupTable) -> Result<Self, Self::Error> {
        let mut subtables = Vec::with_capacity(orig.subtables.len());
        for offset in orig.subtables.iter() {
            subtables.push(LookupSubtable::try_promote(&offset.link)?);
        }

        Ok(LookupTable {
            lookup_type: orig.lookup_type,
            lookup_flag: orig.lookup_flag,
            subtables,
            mark_filtering_set: orig.mark_filtering_set,
        })
    }
}

#[derive(Clone, Debug)]
struct LookupTable {
    lookup_type: u16,
    lookup_flag: LookupFlag,
    subtables: Vec<LookupSubtable>,
    mark_filtering_set: Option<u16>,
}

type ScriptList = Vec<ScriptRecord>;
type FeatureList = Vec<FeatureRecord>;
type LookupList = Vec<LookupTable>;

pub type OpentypeScriptList = opentype_common_script_list;
pub type OpentypeFeatureList = opentype_common_feature_list;

impl Promote<OpentypeScriptList> for ScriptList {
    fn promote(orig: &OpentypeScriptList) -> Self {
        promote_vec(&orig.script_records)
    }
}

impl Promote<OpentypeFeatureList> for FeatureList {
    fn promote(orig: &OpentypeFeatureList) -> Self {
        promote_vec(&orig.feature_records)
    }
}

pub type OpentypeGposLookupList = opentype_gpos_table_lookup_list_link;
pub type OpentypeGsubLookupList = opentype_gsub_table_lookup_list_link;

impl TryPromote<OpentypeGposLookupList> for LookupList {
    type Error = ReflType<TPErr<OpentypeGposLookupTable, LookupTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeGposLookupList) -> Result<Self, Self::Error> {
        let mut accum = Vec::with_capacity(orig.lookups.len());
        for offset in orig.lookups.iter() {
            accum.push(LookupTable::try_promote(&offset.link)?);
        }
        Ok(accum)
    }
}

impl TryPromote<OpentypeGsubLookupList> for LookupList {
    type Error = TPErr<OpentypeGsubLookupTable, LookupTable>;

    fn try_promote(orig: &OpentypeGsubLookupList) -> Result<Self, Self::Error> {
        let mut accum = Vec::with_capacity(orig.lookups.len());
        for offset in orig.lookups.iter() {
            accum.push(LookupTable::try_promote(&offset.link)?);
        }
        Ok(accum)
    }
}

#[derive(Clone, Debug)]
/// Common API type for summarizing GPOS and GSUB
struct LayoutMetrics {
    major_version: u16,
    minor_version: u16,
    script_list: ScriptList,
    feature_list: FeatureList,
    lookup_list: LookupList,
}

#[derive(Clone, Debug)]
pub struct OptionalTableMetrics {
    cvt: Option<CvtMetrics>,
    fpgm: Option<FpgmMetrics>,
    loca: Option<LocaMetrics>,
    glyf: Option<GlyfMetrics>,
    prep: Option<PrepMetrics>,
    gasp: Option<GaspMetrics>,
    // STUB - more tables may end up in between these fields as we add support for them in the order in which microsoft lists them
    gdef: Option<GdefMetrics>,
    gpos: Option<LayoutMetrics>,
    gsub: Option<LayoutMetrics>,
    // STUB - add more tables as we expand opentype definition
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TableDiscriminator {
    Gpos,
    Gsub,
}

#[derive(Debug, Copy, Clone, Default, Hash)]
struct Ctxt {
    whence: Option<TableDiscriminator>,
}

impl From<TableDiscriminator> for Ctxt {
    fn from(value: TableDiscriminator) -> Self {
        Self {
            whence: Some(value),
        }
    }
}

impl Ctxt {
    fn new() -> Self {
        Ctxt { whence: None }
    }

    fn get_disc(self) -> Option<TableDiscriminator> {
        self.whence
    }
}
// !SECTION

/// Common error type for marking a parsed value as unexpected/unknown relative to a set of predefined values we recognize
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
                opentype_maxp_table_data::MaxpUnknown(_) => MaxpMetrics::UnknownVersion { version },
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
                    let buf = plat_encoding_lang.convert(&record.offset.link);
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
                            let lang_tag = utf16be_convert(&record.offset.link);
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
                                coordinates: *simple.end_points_of_contour.last().unwrap() as usize
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
        let prep = {
            let prep = &dir.table_links.prep;
            prep.as_ref().map(|prep| RawArrayMetrics(prep.len()))
        };
        let gasp = {
            let gasp = &dir.table_links.gasp;
            gasp.as_ref().map(|gasp| GaspMetrics {
                version: gasp.version,
                num_ranges: gasp.num_ranges as usize,
                ranges: gasp
                    .gasp_ranges
                    .iter()
                    .map(|r| GaspRange {
                        range_max_ppem: r.range_max_ppem,
                        range_gasp_behavior: match &r.range_gasp_behavior {
                            &opentype_gasp_table_gasp_ranges_range_gasp_behavior::Version0(
                                opentype_gasp_table_gasp_ranges_range_gasp_behavior_Version0 {
                                    dogray,
                                    gridfit,
                                },
                            ) => GaspBehaviorFlags {
                                symmetric_smoothing: false,
                                symmetric_gridfit: false,
                                dogray,
                                gridfit,
                            },
                            opentype_gasp_table_gasp_ranges_range_gasp_behavior::Version1(x) => *x,
                        },
                    })
                    .collect(),
            })
        };
        // STUB - anything before GDEF goes here
        let gdef = {
            let gdef = &dir.table_links.gdef;
            gdef.as_ref()
                .map(|gdef| {
                    TestResult::Ok(GdefMetrics {
                        major_version: gdef.major_version,
                        minor_version: gdef.minor_version,
                        glyph_class_def: try_promote_opt(&gdef.glyph_class_def.link)?,
                        attach_list: try_promote_opt(&gdef.attach_list.link)?,
                        lig_caret_list: try_promote_opt(&gdef.lig_caret_list.link)?,
                        mark_attach_class_def: try_promote_opt(&gdef.mark_attach_class_def.link)?,
                        data: GdefTableDataMetrics::try_promote(&gdef.data)?,
                    })
                })
                .transpose()?
        };
        let gpos = {
            let gpos = &dir.table_links.gpos;
            gpos.as_ref()
                .map(|gpos| {
                    TestResult::Ok(LayoutMetrics {
                        major_version: gpos.major_version,
                        minor_version: gpos.minor_version,
                        script_list: ScriptList::try_promote(&gpos.script_list.link)?,
                        feature_list: FeatureList::try_promote(&gpos.feature_list.link)?,
                        lookup_list: LookupList::try_promote(&gpos.lookup_list.link)?,
                    })
                })
                .transpose()?
        };
        let gsub = {
            let gsub = &dir.table_links.gsub;
            gsub.as_ref()
                .map(|gsub| {
                    TestResult::Ok(LayoutMetrics {
                        major_version: gsub.major_version,
                        minor_version: gsub.minor_version,
                        script_list: ScriptList::try_promote(&gsub.script_list.link)?,
                        feature_list: FeatureList::try_promote(&gsub.feature_list.link)?,
                        lookup_list: LookupList::try_promote(&gsub.lookup_list.link)?,
                    })
                })
                .transpose()?
        };
        OptionalTableMetrics {
            cvt,
            fpgm,
            loca,
            glyf,
            prep,
            gasp,
            // TODO - add more optional tables as they are added to the spec
            gdef,
            gpos,
            gsub,
        }
    };
    let extraMagic = dir
        .table_records
        .iter()
        .map(|r| r.table_id)
        .filter(is_extra)
        .collect();
    Ok(SingleFontMetrics {
        sfnt_version: dir.sfnt_version,
        num_tables: dir.num_tables as usize,
        required,
        optional,
        extraMagic,
    })
}

/// Returns `true` if `table_id` is not a first-class OpenType table in our current implementation
fn is_extra(table_id: &u32) -> bool {
    let bytes = table_id.to_be_bytes();
    match &bytes {
        b"cmap" | b"head" | b"hhea" | b"hmtx" | b"maxp" | b"name" | b"OS/2" | b"post" => false,
        b"cvt " | b"fpgm" | b"loca" | b"glyf" | b"prep" | b"gasp" => false,
        b"GDEF" | b"GPOS" | b"GSUB" => false,
        // FIXME - update with more cases as we handle more table records
        _ => true,
    }
}

fn bounding_box(gl: &opentype_glyf_table_Glyph) -> BoundingBox {
    BoundingBox {
        x_min: as_s16(gl.x_min),
        y_min: as_s16(gl.y_min),
        x_max: as_s16(gl.x_max),
        y_max: as_s16(gl.y_max),
    }
}

pub fn show_opentype_stats(metrics: &OpentypeMetrics, conf: &Config) {
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
                        show_font_metrics(font, conf);
                    }
                    None => {
                        println!("=== Skipping Index {i} ===");
                    }
                }
            }
        }
        OpentypeMetrics::SingleFont(single) => show_font_metrics(single, conf),
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

fn show_font_metrics(font: &SingleFontMetrics, conf: &Config) {
    if !conf.extra_only {
        show_magic(font.sfnt_version);
        show_required_metrics(&font.required, conf);
        show_optional_metrics(&font.optional, conf);
    }
    show_extra_magic(&font.extraMagic);
}

fn show_extra_magic(table_ids: &[u32]) {
    for id in table_ids.iter() {
        println!("[MISSING IMPL]: `{}`", format_magic(*id));
    }
}

fn show_required_metrics(required: &RequiredTableMetrics, conf: &Config) {
    show_cmap_metrics(&required.cmap, conf);
    show_head_metrics(&required.head, conf);
    show_hhea_metrics(&required.hhea, conf);
    show_htmx_metrics(&required.hmtx, conf);
    show_maxp_metrics(&required.maxp, conf);
    show_name_metrics(&required.name, conf);
    show_os2_metrics(&required.os2, conf);
    show_post_metrics(&required.post, conf);
}

fn show_optional_metrics(optional: &OptionalTableMetrics, conf: &Config) {
    show_cvt_metrics(&optional.cvt, conf);
    show_fpgm_metrics(&optional.fpgm, conf);
    show_loca_metrics(&optional.loca, conf);
    show_glyf_metrics(&optional.glyf, conf);
    show_prep_metrics(&optional.prep, conf);
    show_gasp_metrics(&optional.gasp, conf);
    // STUB - anything between gasp and gdef go here
    show_gdef_metrics(&optional.gdef, conf);

    show_layout_metrics(&optional.gpos, Ctxt::from(TableDiscriminator::Gpos), conf);
    show_layout_metrics(&optional.gsub, Ctxt::from(TableDiscriminator::Gsub), conf);
}

fn show_cvt_metrics(cvt: &Option<CvtMetrics>, _conf: &Config) {
    if let Some(RawArrayMetrics(count)) = cvt {
        println!("cvt: FWORD[{count}]")
    }
}

fn show_fpgm_metrics(fpgm: &Option<FpgmMetrics>, _conf: &Config) {
    if let Some(RawArrayMetrics(count)) = fpgm {
        println!("fpgm: uint8[{count}]")
    }
}

fn show_prep_metrics(prep: &Option<PrepMetrics>, _conf: &Config) {
    if let Some(RawArrayMetrics(count)) = prep {
        println!("prep: uint8[{count}]")
    }
}

fn show_loca_metrics(loca: &Option<LocaMetrics>, _conf: &Config) {
    if let Some(()) = loca {
        println!("loca: (details omitted)")
    }
}

fn show_gdef_metrics(gdef: &Option<GdefMetrics>, conf: &Config) {
    if let Some(GdefMetrics {
        major_version,
        minor_version,
        glyph_class_def,
        attach_list,
        lig_caret_list,
        mark_attach_class_def,
        data,
    }) = gdef
    {
        println!(
            "GDEF: version {}",
            format_version_major_minor(*major_version, *minor_version)
        );
        if let Some(glyph_class_def) = glyph_class_def {
            show_glyph_class_def(glyph_class_def, conf);
        }
        if let Some(attach_list) = attach_list {
            show_attach_list(attach_list, conf);
        }
        if let Some(lig_caret_list) = lig_caret_list {
            show_lig_caret_list(lig_caret_list, conf);
        }
        if let Some(mark_attach_class_def) = mark_attach_class_def {
            show_mark_attach_class_def(mark_attach_class_def, conf);
        }
        match data {
            GdefTableDataMetrics::NoData => {}
            GdefTableDataMetrics::MarkGlyphSetsDef(mark_glyph_set) => match mark_glyph_set {
                None => println!("\tMarkGlyphSet: <none>"),
                Some(mgs) => show_mark_glyph_set(mgs, conf),
            },
            GdefTableDataMetrics::ItemVarStore(ivs) => show_item_variation_store(ivs),
        }
    }
}

fn format_table_disc(disc: TableDiscriminator) -> &'static str {
    match disc {
        TableDiscriminator::Gpos => "GPOS",
        TableDiscriminator::Gsub => "GSUB",
    }
}

fn show_layout_metrics(gpos: &Option<LayoutMetrics>, ctxt: Ctxt, conf: &Config) {
    if let Some(LayoutMetrics {
        major_version,
        minor_version,
        script_list,
        feature_list,
        lookup_list,
    }) = gpos
    {
        println!(
            "{}: version {}",
            format_table_disc(ctxt.get_disc().expect("Ctxt missing TableDiscriminator")),
            format_version_major_minor(*major_version, *minor_version)
        );
        show_script_list(&script_list, conf);
        show_feature_list(&feature_list, conf);
        show_lookup_list(&lookup_list, ctxt, conf);
    }
}

fn show_script_list(script_list: &ScriptList, conf: &Config) {
    if script_list.is_empty() {
        println!("\tScriptList [empty]");
    } else {
        println!("\tScriptList");
        show_items_elided(
            script_list,
            |ix, item| {
                let ScriptTable {
                    default_lang_sys,
                    lang_sys_records,
                } = &item.script;
                println!("\t\t[{ix}]: {}", format_magic(item.script_tag));
                if let Some(langsys) = default_lang_sys {
                    print!("\t\t    [Default LangSys]: ");
                    show_langsys(langsys, conf);
                }
                show_lang_sys_records(lang_sys_records, conf)
            },
            conf.bookend_size,
            |start, stop| format!("skipping ScriptRecords {}..{}", start, stop),
        );
    }
}

fn show_lang_sys_records(lang_sys_records: &[LangSysRecord], conf: &Config) {
    if lang_sys_records.is_empty() {
        println!("\t\t    LangSysRecords: <empty list>");
    } else {
        println!("\t\t    LangSysRecords:");
        show_items_elided(
            lang_sys_records,
            |ix, item| {
                print!("\t\t\t[{ix}]: {}; ", format_magic(item.lang_sys_tag));
                show_langsys(&item.lang_sys, conf);
            },
            conf.bookend_size,
            |start, stop| format!("\t\t    (skipping LangSysRecords {}..{})", start, stop),
        )
    }
}

fn show_langsys(lang_sys: &LangSys, conf: &Config) {
    let LangSys {
        lookup_order_offset,
        required_feature_index,
        feature_indices,
    } = lang_sys;
    debug_assert_eq!(*lookup_order_offset, 0);
    match required_feature_index {
        0xFFFF => print!("feature-indices: "),
        other => print!("feature-indices (required: {}): ", other),
    }
    show_items_inline(
        feature_indices,
        |ix: &u16| format!("{}", ix),
        conf.inline_bookend,
        |num_skipped: usize| format!("...({} skipped)...", num_skipped),
    );
}

fn show_feature_list(feature_list: &FeatureList, conf: &Config) {
    if feature_list.is_empty() {
        println!("\tFeatureList [empty]");
    } else {
        println!("\tFeatureList");
        show_items_elided(
            feature_list,
            |ix, item| {
                let FeatureRecord {
                    feature_tag,
                    feature,
                } = item;
                print!("\t\t[{ix}]: {}", format_magic(*feature_tag));
                show_feature_table(feature, conf);
            },
            conf.bookend_size,
            |start, stop| format!("\t    (skipping FeatureIndices {}..{})", start, stop),
        );
    }
}

fn show_feature_table(table: &FeatureTable, conf: &Config) {
    let FeatureTable {
        feature_params,
        lookup_list_indices,
    } = table;
    match feature_params {
        0 => (),
        offset => print!("[parameters located at SoF+{}B]", offset),
    }
    show_items_inline(
        lookup_list_indices,
        |index| format!("{}", index),
        conf.inline_bookend,
        |num_skipped| format!("...({} skipped)...", num_skipped),
    );
}

fn show_lookup_list(lookup_list: &LookupList, ctxt: Ctxt, conf: &Config) {
    println!("\tLookupList:");
    show_items_elided(
        lookup_list,
        move |ix, table| {
            print!("\t\t[{ix}]: ");
            show_lookup_table(table, ctxt, conf);
        },
        conf.bookend_size,
        |start, stop| format!("\t    (skipping LookupTables {}..{})", start, stop),
    );
}

fn show_lookup_table(table: &LookupTable, ctxt: Ctxt, conf: &Config) {
    // NOTE - because we print the kind of the lookup here, we don't need to list it for every element
    // LINK[format-lookup-subtable] -  (see format_lookup_subtable below)
    print!(
        "LookupTable: kind={}",
        format_lookup_type(ctxt, table.lookup_type),
    );
    show_lookup_flag(&table.lookup_flag);
    if let Some(filtering_set) = table.mark_filtering_set {
        print!(", markFilteringSet=GDEF->MarkGlyphSet[{}]", filtering_set)
    }
    print!(": ");
    show_items_inline(
        &table.subtables,
        |subtable| format_lookup_subtable(subtable, false, conf),
        conf.inline_bookend,
        |n_skipped| format!("...({n_skipped} skipped)..."),
    );
}

// ANCHOR[format-lookup-subtable]
fn format_lookup_subtable(
    subtable: &LookupSubtable,
    show_lookup_type: bool,
    _conf: &Config,
) -> String {
    // STUB - because the subtables are both partial (more variants exist) and abridged (existing variants are missing details), reimplement as necessary
    let (label, contents) = match subtable {
        LookupSubtable::SinglePos(single_pos) => {
            let contents = {
                match single_pos {
                    SinglePos::Format1(SinglePosFormat1 { value_record, .. }) => {
                        format!("single({})", format_value_record(value_record))
                    }
                    SinglePos::Format2(SinglePosFormat2 { coverage, .. }) => {
                        format!("array({})", format_coverage_table(coverage))
                    }
                }
            };
            ("SinglePos", contents)
        }
        LookupSubtable::PairPos(pair_pos) => {
            let contents = {
                match pair_pos {
                    PairPos::Format1(PairPosFormat1 { coverage, .. }) => {
                        format!("byGlyph({})", format_coverage_table(coverage))
                    }
                    PairPos::Format2(PairPosFormat2 {
                        coverage,
                        class_def1,
                        class_def2,
                        class1_records,
                    }) => {
                        let rows = class1_records.rows();
                        let cols = class1_records.width();

                        validate_class_count(class_def1, rows);
                        validate_class_count(class_def2, cols);

                        // REVIEW - if not too verbose, we might want a compact overview of the Class1Record array, specifically which index-pairs constitute actual adjustments
                        let _populated_class_pairs: Vec<(usize, usize)> = {
                            Iterator::zip(0..rows, 0..cols)
                                .filter(|ixpair| {
                                    let it = &class1_records[*ixpair];
                                    it.value_record1.is_some() || it.value_record2.is_some()
                                })
                                .collect()
                        };
                        // maximum number of index-pairs we are willing to display inline (chosen arbitrarily)
                        // TODO - should this be a more general parameter in the Config type?
                        const MAX_POPULATION: usize = 3;
                        if _populated_class_pairs.len() <= MAX_POPULATION {
                            format!(
                                "byClass{:?}({})",
                                _populated_class_pairs,
                                format_coverage_table(coverage)
                            )
                        } else {
                            format!(
                                "byClass[{} ∈ {rows} x {cols}]({})",
                                _populated_class_pairs.len(),
                                format_coverage_table(coverage)
                            )
                        }
                    }
                }
            };
            ("PairPos", contents)
        }
        LookupSubtable::CursivePos(cursive_pos) => {
            let contents = {
                match cursive_pos {
                    CursivePos::Format1(CursivePosFormat1 { coverage, .. }) => {
                        format!("entryExit({})", format_coverage_table(coverage))
                    }
                }
            };
            ("CursivePos", contents)
        }
        LookupSubtable::MarkBasePos => ("MarkBasePos", format!("(..)")),
        LookupSubtable::MarkLigPos => ("MarkLigPos", format!("(..)")),
        LookupSubtable::MarkMarkPos => ("MarkMarkPos", format!("(..)")),
        LookupSubtable::PosExtension => ("PosExt", format!("(..)")),

        LookupSubtable::SingleSubst(single_subst) => {
            let contents = match single_subst {
                // STUB[placeholder]
                _ => format!("(..)"),
            };
            ("SingleSubst", contents)
        }
        LookupSubtable::MultipleSubst => ("MultipleSubst", format!("(..)")),
        LookupSubtable::AlternateSubst => ("AlternateSubst", format!("(..)")),
        LookupSubtable::LigatureSubst => ("LigatureSubst", format!("(..)")),
        LookupSubtable::SubstExtension => ("SubstExt", format!("(..)")),
        LookupSubtable::ReverseChainSingleSubst => ("RevChainSingleSubst", format!("(..)")),

        LookupSubtable::SequenceContext(seq_ctx) => {
            let contents = match seq_ctx {
                SequenceContext::Format1(SequenceContextFormat1 {
                    coverage,
                    seq_rule_sets,
                }) => format!("Glyphs({})", format_coverage_table(coverage)),
                SequenceContext::Format2(SequenceContextFormat2 {
                    coverage,
                    class_seq_rule_sets,
                    ..
                }) => format!("Classes({})", format_coverage_table(coverage)),
                SequenceContext::Format3(SequenceContextFormat3 {
                    coverage_tables,
                    seq_lookup_records,
                    ..
                }) => {
                    // REVIEW - since we are already within an inline elision context, try to avoid taking up too much space per item, but this might not want to be a hardcoded value
                    const INLINE_INLINE_BOOKEND: usize = 1;
                    // FIXME - show_lookup_table calls this function through show_items_inline already, so we might want to reduce how many values we are willing to show proportionally
                    let input_pattern = format_items_inline(
                        coverage_tables,
                        |cov| format_coverage_table(cov),
                        INLINE_INLINE_BOOKEND,
                        |n| format!("(..{n}..)"),
                    );
                    let seq_lookups = format_items_inline(
                        seq_lookup_records,
                        |seq_lookup| format_sequence_lookup(seq_lookup),
                        INLINE_INLINE_BOOKEND,
                        |n| format!("(..{n}..)"),
                    );
                    format!("{input_pattern}=>{seq_lookups}")
                }
            };
            ("SeqCtx", contents)
        }
        LookupSubtable::ChainedSequenceContext(chain_ctx) => {
            let contents = match chain_ctx {
                ChainedSequenceContext::Format1(ChainedSequenceContextFormat1 {
                    coverage,
                    chained_seq_rule_sets,
                }) => {
                    // TODO - even if it means overly verbose output, this might be too little info to be useful compared to discriminant-only display
                    format!("ChainedGlyphs({})", format_coverage_table(coverage))
                }
                ChainedSequenceContext::Format2(ChainedSequenceContextFormat2 {
                    coverage,
                    chained_class_seq_rule_sets,
                    ..
                }) => {
                    // TODO - even if it means overly verbose output, this might be too little info to be useful compared to discriminant-only display
                    // REVIEW - consider what other details (e.g. class-def summary metrics) to show in implicitly- or explictly-verbose display format
                    format!("ChainedClasses({})", format_coverage_table(coverage))
                }
                ChainedSequenceContext::Format3(ChainedSequenceContextFormat3 {
                    backtrack_coverages,
                    input_coverages,
                    lookahead_coverages,
                    seq_lookup_records,
                    ..
                }) => {
                    // REVIEW - since we are already within an inline elision context, try to avoid taking up too much space per item, but this might not want to be a hardcoded value
                    const INLINE_INLINE_BOOKEND: usize = 1;
                    // FIXME - show_lookup_table calls this function through show_items_inline already, so we might want to reduce how many values we are willing to show proportionally
                    let backtrack_pattern = if backtrack_coverages.is_empty() {
                        String::new()
                    } else {
                        let tmp = format_items_inline(
                            backtrack_coverages,
                            format_coverage_table,
                            INLINE_INLINE_BOOKEND,
                            |n| format!("(..{n}..)"),
                        );
                        format!("(?<={tmp})")
                    };
                    let input_pattern = format_items_inline(
                        input_coverages,
                        format_coverage_table,
                        INLINE_INLINE_BOOKEND,
                        |n| format!("(..{n}..)"),
                    );
                    let lookahead_pattern = if lookahead_coverages.is_empty() {
                        String::new()
                    } else {
                        let tmp = format_items_inline(
                            lookahead_coverages,
                            format_coverage_table,
                            INLINE_INLINE_BOOKEND,
                            |n| format!("(..{n}..)"),
                        );
                        format!("(?={tmp})")
                    };
                    let seq_lookups = format_items_inline(
                        seq_lookup_records,
                        |seq_lookup| format_sequence_lookup(seq_lookup),
                        INLINE_INLINE_BOOKEND,
                        |n| format!("(..{n}..)"),
                    );
                    format!("{backtrack_pattern}{input_pattern}{lookahead_pattern}=>{seq_lookups}")
                }
            };
            ("ChainSeqCtx", contents)
        }
    };
    if show_lookup_type {
        format!("{label}{contents}")
    } else {
        contents
    }
}

fn format_sequence_lookup(sl: &SequenceLookup) -> String {
    let s_ix = sl.sequence_index;
    let ll_ix = sl.lookup_list_index;
    format!("[{}]@{}", ll_ix, s_ix)
}

/// Checks that the given ClassDef (assumed to be Some) contains the expected number of classes.
///
/// Panics if opt_classdef is None.
fn validate_class_count(class_def: &ClassDef, expected_classes: usize) {
    match class_def {
        ClassDef::Format1 {
            class_value_array,
            start_glyph_id: _start_id,
        } => {
            // REVIEW - this is a bit low-level, but there might be slightly better structures than BTreeSet (e.g. Vec<bool>)
            let max = expected_classes as u16;
            let mut actual_set = U16Set::new();
            actual_set.insert(0);
            for (_ix, value) in class_value_array.iter().enumerate() {
                if *value >= max {
                    panic!("expecting {expected_classes} starting at 0, found ClassValue {value} (>= {max}) at index {_ix} (glyph id: {})", *_start_id + _ix as u16);
                }
                let _ = actual_set.insert(*value);
            }
            assert_eq!(
                actual_set.len(),
                expected_classes,
                "expected to find {expected_classes} ClassDefs, found {}-element set {:?}",
                actual_set.len(),
                actual_set
            );
        }
        ClassDef::Format2 {
            class_range_records,
        } => {
            let max = expected_classes as u16;
            let mut actual_set = U16Set::new();
            actual_set.insert(0);
            for (_ix, rr) in class_range_records.iter().enumerate() {
                let value = rr.value;
                if value >= max {
                    panic!("expecting {expected_classes} starting at 0, found ClassValue {value} (>= {max}) at index {_ix} (glyph range: {} -> {})", rr.start_glyph_id, rr.end_glyph_id);
                }
                let _ = actual_set.insert(value);
            }
            assert_eq!(
                actual_set.len(),
                expected_classes,
                "expected to find {expected_classes} ClassDefs, found {}-element set {:?}",
                actual_set.len(),
                actual_set
            );
        }
    }
}

fn format_value_record(record: &ValueRecord) -> String {
    let ValueRecord {
        x_placement,
        y_placement,
        x_advance,
        y_advance,
        x_placement_device,
        y_placement_device,
        x_advance_device,
        y_advance_device,
    } = record;
    const NUM_FRAGMENTS: usize = 4;
    let mut buf = Vec::<String>::with_capacity(NUM_FRAGMENTS);

    // helper to indicate whether a field exists
    let elide = |opt_val: &Option<_>| -> Option<&'static str> { opt_val.as_ref().map(|_| "(..)") };

    buf.extend(format_opt_xy("placement", *x_placement, *y_placement));
    buf.extend(format_opt_xy("advance", *x_advance, *y_advance));
    buf.extend(format_opt_xy(
        "placement(device)",
        elide(x_placement_device),
        elide(y_placement_device),
    ));
    buf.extend(format_opt_xy(
        "advance(device)",
        elide(x_advance_device),
        elide(y_advance_device),
    ));

    if buf.is_empty() {
        // REVIEW - this is highly unlikely, right..?
        String::from("<Empty ValueRecord>")
    } else {
        buf.join("; ")
    }
}

fn format_opt_xy<T>(what: &str, x: Option<T>, y: Option<T>) -> Option<String>
where
    T: std::fmt::Display,
{
    match (x, y) {
        (None, None) => None,
        (Some(x), Some(y)) => Some(format!("{what}: ({x},{y})")),
        (Some(x), None) => Some(format!("{what}[x]: {x}")),
        (None, Some(y)) => Some(format!("{what}[y]: {y}")),
    }
}

/// Prints a summary of a given `LookupFlag` value, including logic to avoid printing anything for the default flag value.
///
/// Because of this elision, will also print a prefix that separates the displayed content from the previous field
fn show_lookup_flag(flags: &LookupFlag) {
    if flags.mark_attachment_class_filter != 0
        || flags.right_to_left
        || flags.ignore_ligatures
        || flags.ignore_base_glyphs
        || flags.ignore_marks
        || flags.use_mark_filtering_set
    {
        print!(", flags={}", format_lookup_flag(flags))
    }
}

fn format_lookup_flag(flags: &LookupFlag) -> String {
    let mut set_flags = Vec::new();
    if flags.right_to_left {
        set_flags.push("RIGHT_TO_LEFT");
    }
    if flags.ignore_base_glyphs {
        set_flags.push("IGNORE_BASE_GLYPHS");
    }
    if flags.ignore_ligatures {
        set_flags.push("IGNORE_LIGATURES)");
    }
    if flags.ignore_marks {
        set_flags.push("IGNORE_MARKS");
    }
    if flags.use_mark_filtering_set {
        set_flags.push("USE_MARK_FILTERING_SET");
    }

    let str_bitflags = if set_flags.is_empty() {
        String::from("∅")
    } else {
        set_flags.join(" | ")
    };

    let str_macf = match flags.mark_attachment_class_filter {
        0 => String::from("∅"),
        n => format!("Class=={n}"),
    };

    format!("LookupFlag ({str_bitflags} ; mark_attachment_class_filter = {str_macf})")
}

fn format_lookup_type(ctxt: Ctxt, ltype: u16) -> &'static str {
    match ctxt.get_disc() {
        None => unreachable!("format_lookup_kind called with neutral (whence := None) Ctxt"),
        Some(TableDiscriminator::Gpos) => match ltype {
            1 => "SinglePos",
            2 => "PairPos",
            3 => "CursivePos",
            4 => "MarkBasePos",
            5 => "MarkLigPos",
            6 => "MarkMarkPos",
            7 => "SequenceContext",
            8 => "ChainedSequenceContext",
            9 => "PosExt",
            _ => unreachable!("unexpected GPOS lookup-type {ltype} (expected 1..=9)"),
        },
        Some(TableDiscriminator::Gsub) => match ltype {
            1 => "SingleSubst",
            2 => "MultipleSubst",
            3 => "AlternateSubst",
            4 => "LigatureSubst",
            5 => "SequenceContext",
            6 => "ChainedSequenceContext",
            7 => "SubstExt",
            8 => "ReverseChainedSingleSubst",
            _ => unreachable!("unexpected GSUB lookup-type {ltype} (expected 1..=8)"),
        },
    }
}

fn show_mark_glyph_set(mgs: &MarkGlyphSet, conf: &Config) {
    println!("\tMarkGlyphSet:");
    show_items_elided(
        &mgs.coverage,
        |ix, item| match item {
            None => println!("\t\t[{ix}]: <none>"),
            Some(covt) => {
                print!("\t\t[{ix}]: ");
                show_coverage_table(covt, conf);
            }
        },
        conf.bookend_size,
        |start, stop| format!("\t    (skipping coverage tables {}..{})", start, stop),
    )
}

fn show_item_variation_store(ivs: &ItemVariationStore) {
    println!("(UNINTERPRETED: ItemVariationStore @GDEF+{:#0x})", *ivs);
}

fn show_lig_caret_list(lig_caret_list: &LigCaretList, conf: &Config) {
    println!("\tLigCaretList:");
    // NOTE - since coverage tables are used in MarkGlyphSet, we don't want to force-indent within the `show_coverage_table` function, so we do it before instead.
    print!("\t\t");
    show_coverage_table(&lig_caret_list.coverage, conf);
    show_items_elided(
        &lig_caret_list.lig_glyphs,
        |ix, lig_glyph| {
            print!("\t\t[{ix}]: ");
            show_items_inline(
                &lig_glyph.caret_values,
                format_caret_value,
                conf.inline_bookend,
                |num_skipped| format!("...({num_skipped})..."),
            )
        },
        conf.bookend_size,
        |start, stop| format!("\t    (skipping LigGlyphs {}..{})", start, stop),
    )
}

fn format_caret_value(cv: &CaretValue) -> String {
    match cv {
        // REVIEW - this isn't really a canonical abbreviation, so we might adjust what we show for Design Units (Format 1)
        CaretValue::DesignUnits(du) => format!("{du}du"),
        CaretValue::ContourPoint(ix) => format!("#{ix}"),
        CaretValue::DesignUnitsWithTable { coordinate, device } => match device {
            DeviceOrVariationIndexTable::DeviceTable(dev_table) => {
                format!("{}du+[{}]", coordinate, format_device_table(dev_table))
            }
            DeviceOrVariationIndexTable::VariationIndexTable(var_ix_table) => {
                format!(
                    "{}du+[{}]",
                    coordinate,
                    format_variation_index_table(var_ix_table)
                )
            }
        },
    }
}

fn format_device_table(dev_table: &DeviceTable) -> String {
    // REVIEW - we are so far down the stack there is very little we can display inline for the delta-values, but we have them on hand if we wish to show them in some abbreviated form...
    format!("{}..{}", dev_table.start_size, dev_table.end_size)
}

fn format_variation_index_table(var_ix_table: &VariationIndexTable) -> String {
    format!(
        "{}->{}",
        var_ix_table.delta_set_outer_index, var_ix_table.delta_set_inner_index
    )
}

fn show_attach_list(attach_list: &AttachList, conf: &Config) {
    println!("\tAttachList:");
    // NOTE - since coverage tables are used in MarkGlyphSet, we don't want to force-indent within the `show_coverage_table` function, so we do it before instead.
    print!("\t\t");
    show_coverage_table(&attach_list.coverage, conf);
    show_items_elided(
        &attach_list.attach_points,
        |ix, AttachPoint { point_indices }| {
            print!("\t\t[{ix}]:");
            show_items_inline(
                point_indices,
                |point_ix| format!("{}", point_ix),
                conf.inline_bookend,
                |num_skipped| format!("...({num_skipped})..."),
            );
        },
        conf.bookend_size,
        |start, stop| {
            format!(
                "\t    (skipping attach points for glyphs {}..{})",
                start, stop
            )
        },
    )
}

// FIXME - we might want a more flexible model where the `show_XYZZY`/`format_XYZZY` dichotomy is erased, such as a generic Writer or Fragment-producer...
fn format_coverage_table(cov: &CoverageTable) -> String {
    match cov {
        CoverageTable::Format1 { ref glyph_array } => {
            let num_glyphs = glyph_array.len();
            match glyph_array.as_slice() {
                &[] => format!("∅"),
                &[id] => format!("[glyphId={id}]"),
                &[first, .., last] => format!("[{num_glyphs} ∈ [{first},{last}]]"),
            }
        }
        CoverageTable::Format2 { ref range_records } => match range_records.as_slice() {
            &[] => format!("∅"),
            &[rr] => format!("[∀ glyphId ∈ [{},{}]]", rr.start_glyph_id, rr.end_glyph_id),
            &[first, .., last] => {
                let num_glyphs: u16 = range_records
                    .iter()
                    .map(|rr| rr.end_glyph_id - rr.start_glyph_id + 1)
                    .sum();
                let num_ranges = range_records.len();
                let min_glyph = first.start_glyph_id;
                let max_glyph = last.end_glyph_id;
                format!("[{num_ranges} ranges; {num_glyphs} ∈ [{min_glyph},{max_glyph}]]")
            }
        },
    }
}

fn show_coverage_table(cov: &CoverageTable, conf: &Config) {
    match cov {
        CoverageTable::Format1 { ref glyph_array } => {
            print!("Glyphs Covered: ");
            show_items_inline(
                glyph_array,
                |item| format!("{}", item),
                conf.inline_bookend,
                |num_skipped| format!("...({num_skipped})..."),
            );
        }
        CoverageTable::Format2 { ref range_records } => {
            print!("Glyph Ranges Covered: ");
            show_items_inline(
                range_records,
                format_coverage_range_record,
                conf.inline_bookend,
                |num_skipped| format!("...({num_skipped})..."),
            );
        }
    }
}

fn show_mark_attach_class_def(mark_attach_class_def: &ClassDef, conf: &Config) {
    println!("\tMarkAttachClassDef:");
    show_class_def(mark_attach_class_def, format_mark_attach_class, conf);
}

fn format_mark_attach_class(mark_attach_class: &u16) -> String {
    match mark_attach_class {
        // STUB - if we come up with a semantic association for specific numbers, add those in before this catchall
        _ => format!("{}", mark_attach_class),
    }
}

fn show_glyph_class_def(class_def: &ClassDef, conf: &Config) {
    println!("\tGlyphClassDef:");
    show_class_def(class_def, show_glyph_class, conf)
}

fn show_class_def<S: std::fmt::Display>(
    class_def: &ClassDef,
    show_fn: impl Fn(&u16) -> S,
    conf: &Config,
) {
    match class_def {
        &ClassDef::Format1 {
            start_glyph_id,
            ref class_value_array,
        } => {
            match start_glyph_id {
                0 => (),
                1 => println!("\t    (skipping uncovered glyph 0)"),
                n => println!("\t    (skipping uncovered glyphs 0..{n})"),
            }
            show_items_elided(
                class_value_array,
                |ix, item| {
                    let gix = start_glyph_id as usize + ix;
                    println!("\t\tGlyph [{gix}]: {}", show_fn(item));
                },
                conf.bookend_size,
                |start, stop| {
                    format!(
                        "\t    (skipping glyphs {}..{})",
                        start_glyph_id as usize + start,
                        start_glyph_id as usize + stop
                    )
                },
            )
        }
        &ClassDef::Format2 {
            ref class_range_records,
        } => show_items_elided(
            class_range_records,
            |_ix, class_range| {
                println!(
                    "\t\t({} -> {}): {}",
                    class_range.start_glyph_id,
                    class_range.end_glyph_id,
                    show_fn(&class_range.value)
                );
            },
            conf.bookend_size,
            |start, stop| {
                let low_end = class_range_records[start].start_glyph_id;
                let high_end = class_range_records[stop - 1].end_glyph_id;
                format!(
                    "\t    (skipping ranges covering glyphs {}..={})",
                    low_end, high_end
                )
            },
        ),
    }
}

fn format_coverage_range_record(coverage_range: &CoverageRangeRecord) -> String {
    let span = coverage_range.end_glyph_id - coverage_range.start_glyph_id;
    let end_coverage_index = coverage_range.value + span;
    format!(
        "({} -> {}): {}(->{})",
        coverage_range.start_glyph_id,
        coverage_range.end_glyph_id,
        coverage_range.value,
        end_coverage_index
    )
}

fn show_gasp_metrics(gasp: &Option<GaspMetrics>, conf: &Config) {
    if let Some(GaspMetrics {
        version,
        num_ranges,
        ranges,
    }) = gasp
    {
        let show_gasp_range = |_ix: usize, range: &GaspRange| {
            let GaspBehaviorFlags {
                symmetric_smoothing: syms,
                symmetric_gridfit: symgrift,
                dogray: dg,
                gridfit: grift,
            } = range.range_gasp_behavior;
            // NOTE - Meanings attributed [here](https://learn.microsoft.com/en-us/typography/opentype/spec/gasp)
            let disp = {
                let mut sep = ""; // Dynamic separator that starts out empty but becomes " | " if any flag-string is pushed
                let mut buffer = String::new();
                for flag in [
                    if syms { "SYMMETRIC_SMOOTHING" } else { "" },
                    if symgrift { "SYMMETRIC_GRIDFIT" } else { "" },
                    if dg { "DOGRAY" } else { "" },
                    if grift { "GRIDFIT" } else { "" },
                ]
                .iter()
                {
                    if flag.is_empty() {
                        continue;
                    } else {
                        buffer.push_str(sep);
                        buffer.push_str(flag);
                        sep = " | ";
                    }
                }
                if buffer.is_empty() {
                    Label::Borrowed("(no flags)")
                } else {
                    Label::Owned(format!("({buffer})"))
                }
            };
            if _ix == 0 && range.range_max_ppem == 0xFFFF {
                println!("\t[∀ PPEM] {}", disp);
            } else {
                println!("\t[PPEM <= {}]  {}", range.range_max_ppem, disp)
            }
        };
        println!("gasp: version {version}, {num_ranges} ranges");
        show_items_elided(
            &ranges,
            show_gasp_range,
            conf.bookend_size,
            |start, stop| {
                format!(
                    "    skipping gasp ranges for max_ppem values {}..={}",
                    &ranges[start].range_max_ppem,
                    &ranges[stop - 1].range_max_ppem
                )
            },
        );
    }
}

// REVIEW - this construction suggests we may really want a Write-generic or Fragment-like output model to avoid duplication between I/O show and String formatting functions
fn show_items_inline<T>(
    items: &[T],
    show_fn: impl Fn(&T) -> String,
    bookend: usize,
    ellipsis: impl Fn(usize) -> String,
) {
    let oput = format_items_inline(items, show_fn, bookend, ellipsis);
    println!("{oput}");
}

fn format_items_inline<T>(
    items: &[T],
    show_fn: impl Fn(&T) -> String,
    bookend: usize,
    ellipsis: impl Fn(usize) -> String,
) -> String {
    // Allocate a buffer big enough to hold one string per item in the array, or enough items to show both bookends and one ellipsis-string
    let mut buffer = Vec::<String>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

    let count = items.len();
    if count > bookend * 2 {
        for ix in 0..bookend {
            buffer.push(show_fn(&items[ix]));
        }
        buffer.push(ellipsis(count - bookend * 2));
        for ix in (count - bookend)..count {
            buffer.push(show_fn(&items[ix]));
        }
    } else {
        for ix in 0..count {
            buffer.push(show_fn(&items[ix]));
        }
    }
    format!("[{}]", buffer.join(", "))
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

fn show_cmap_metrics(cmap: &CmapMetrics, _conf: &Config) {
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
    type Error = Local<UnknownValueError<u16>>;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DirectionHint::FullyMixed),
            1 => Ok(DirectionHint::StrongLR),
            2 => Ok(DirectionHint::NeutralLR),
            0xffff => Ok(DirectionHint::StrongRL),
            0xfffe => Ok(DirectionHint::NeutralRL),
            _ => Err(UnknownValueError {
                what: String::from("direction-hint"),
                bad_value: value,
            }),
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

fn show_head_metrics(head: &HeadMetrics, _conf: &Config) {
    println!(
        "head: version {}, {}",
        format_version_major_minor(head.major_version, head.minor_version),
        head.dir_hint,
    );
}

fn show_hhea_metrics(hhea: &HheaMetrics, _conf: &Config) {
    println!(
        "hhea: table version {}, {} horizontal long metrics",
        format_version_major_minor(hhea.major_version, hhea.minor_version),
        hhea.num_lhm,
    );
}

fn show_htmx_metrics(hmtx: &HmtxMetrics, conf: &Config) {
    let show_unified = |ix: usize, hmet: &UnifiedHmtxMetric| match &hmet.advance_width {
        Some(width) => println!(
            "\tGlyph ID [{ix}]: advanceWidth={width}, lsb={}",
            hmet.left_side_bearing
        ),
        None => println!("\tGlyph ID [{ix}]: lsb={}", hmet.left_side_bearing),
    };

    show_items_elided(&hmtx.0, show_unified, conf.bookend_size, |start, stop| {
        format!("    (skipping hmetrics {start}..{stop})")
    });
}

fn show_maxp_metrics(maxp: &MaxpMetrics, _conf: &Config) {
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

fn show_name_metrics(name: &NameMetrics, _conf: &Config) {
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
                ref buf,
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

fn show_os2_metrics(os2: &Os2Metrics, _conf: &Config) {
    // TODO - Metrics type is a stub, enrich if anything is 'interesting'
    println!("os/2: version {}", os2.version);
}

fn show_post_metrics(post: &PostMetrics, _conf: &Config) {
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

fn show_glyf_metrics(glyf: &Option<GlyfMetrics>, conf: &Config) {
    if let Some(glyf) = glyf.as_ref() {
        println!("glyf: {} glyphs", glyf.num_glyphs);
        show_items_elided(
            glyf.glyphs.as_slice(),
            show_glyph_metric,
            conf.bookend_size,
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