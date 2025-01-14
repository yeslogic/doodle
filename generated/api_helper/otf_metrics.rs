use super::util::{U16Set, Wec};
use super::*;
use derive_builder::Builder;
use doodle::Label;
use encoding::{
    all::{MAC_ROMAN, UTF_16BE},
    DecoderTrap, Encoding,
};

// SECTION - Command-line configurable options for what to show

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
#[repr(u8)]
pub enum VerboseLevel {
    #[default]
    Baseline = 0, // Default verbose level: show at least the presence and version of each table, perhaps more for larger or more detailed tables
    Detailed = 1, // Show at least as much as necessary to sanity-check specific values at a debugger level
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Verbosity {
    Minimal,
    VerboseLevel(VerboseLevel),
}

impl std::default::Default for Verbosity {
    fn default() -> Self {
        Verbosity::VerboseLevel(VerboseLevel::default())
    }
}

impl Verbosity {
    fn is_at_least(&self, other: impl Into<Self>) -> bool {
        self >= &other.into()
    }
}

impl VerboseLevel {
    pub const MIN_LEVEL: Self = VerboseLevel::Baseline;
    pub const MAX_LEVEL: Self = VerboseLevel::Detailed;
}

impl From<u8> for VerboseLevel {
    fn from(value: u8) -> Self {
        let clamped = value.clamp(VerboseLevel::MIN_LEVEL as u8, VerboseLevel::MAX_LEVEL as u8);
        unsafe { std::mem::transmute(clamped) }
    }
}

impl From<VerboseLevel> for Verbosity {
    fn from(value: VerboseLevel) -> Self {
        Self::VerboseLevel(value)
    }
}

/// Set of configurable values that control which metrics are shown, and in how much detail
#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(setter(into))]
#[builder(build_fn(error = "std::convert::Infallible"))]
pub struct Config {
    // STUB - Currently only controls bookending, and whether to dump only uncovered tables
    #[builder(default = "8")]
    bookend_size: usize,

    #[builder(default = "3")]
    inline_bookend: usize,

    /// Set to true when we only care about dumping the list of tables that are present in the font but aren't handled yet
    #[builder(default = "false")]
    extra_only: bool,

    #[builder(default)]
    verbosity: Verbosity,
}

impl Config {
    const DEFAULT_BOOKEND_SIZE: usize = 8;
    const DEFAULT_INLINE_BOOKEND: usize = 3;
}

impl std::default::Default for Config {
    fn default() -> Self {
        ConfigBuilder::default().build().unwrap()
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

/// Helper trait for promoting unexpectedly-null Offset-Options into non-Option values of the target type.
trait FromNull: Sized {
    /// Constructs the logically 'null' value of type `Self`.
    fn from_null() -> Self;

    /// Returns `value` if `opt` is `Some(value)`, else returns `Self::from_null()`.
    fn renew(opt: Option<Self>) -> Self {
        opt.unwrap_or_else(Self::from_null)
    }
}

impl<T> FromNull for T
where
    T: std::default::Default,
{
    fn from_null() -> Self {
        T::default()
    }

    fn renew(opt: Option<Self>) -> Self {
        opt.unwrap_or_default()
    }
}

impl<T, Original> Promote<Option<Original>> for T
where
    T: FromNull + Promote<Original>,
{
    fn promote(orig: &Option<Original>) -> Self {
        Self::renew(orig.as_ref().map(T::promote))
    }
}

/// Helper trait for converting from a borrowed value of type `Original` into an owned value of type `Self`,
/// as a short-cut to avoid the need to clone fields we would ultimately either discard, simplify, or unpack
/// if we were to implement `From<Original>`  instead.
///
/// Avoids the need for lifetimes in the signature of the trait or its associated impls, relying on the fact
/// that the lifetime of the borrowed source-object has no bearing on the target object's longevity.
trait Promote<Original>: Sized {
    /// Convert from `Original` into `Self`.
    ///
    /// # Panics
    ///
    /// Should not panic. If the conversion can fail, use [`TryPromote`] instead
    fn promote(orig: &Original) -> Self;
}

/// Variant of [`Promote`] for cases where the conversion from `&Original` may have failure-cases.
trait TryPromote<Original>: Sized {
    /// The error-type returned when a given conversion cannot succeed.
    type Error: std::error::Error;

    /// Fallibly convert from the `Original` into `Self`.
    fn try_promote(orig: &Original) -> Result<Self, Self::Error>;
}

/// Custom trait that facilitates conversion from partially-borrowed non-atomic types
/// without needing explicit lifetimes in the trait signature itself.
trait TryFromRef<Original: _Ref>: Sized {
    type Error: std::error::Error;

    /// Fallibly convert from the GAT `Ref<'a>` defined on `Original` (via the `_Ref` trait), into `Self`.
    fn try_from_ref<'a>(orig: <Original as _Ref>::Ref<'a>) -> Result<Self, Self::Error>;
}

/// Helper trait for implementing custom partial-borrow-semantics for non-atomic types
trait _Ref {
    /// A partial borrow of `Self` that lives at least as long as `'a`.
    type Ref<'a>;
}

impl<T, U> _Ref for (T, U)
where
    T: Copy + 'static,
    U: 'static,
{
    type Ref<'a> = (T, &'a U);
}

fn promote_from_null<O, T>(orig_opt: &Option<O>) -> T
where
    T: FromNull + Promote<O>,
{
    T::renew(orig_opt.as_ref().map(T::promote))
}

fn try_promote_from_null<O, T>(orig_opt: &Option<O>) -> Result<T, T::Error>
where
    T: FromNull + TryPromote<O>,
{
    Ok(T::renew(orig_opt.as_ref().map(T::try_promote).transpose()?))
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

fn promote_link<O, T>(orig_link: &Link<O>) -> Link<T>
where
    T: Promote<O>,
{
    orig_link.as_ref().map(T::promote)
}

fn try_promote_opt<O, T>(orig: &Option<O>) -> Result<Option<T>, T::Error>
where
    T: TryPromote<O>,
{
    orig.as_ref().map(T::try_promote).transpose()
}

fn try_promote_link<O, T>(orig: &Link<O>) -> Result<Link<T>, T::Error>
where
    T: TryPromote<O>,
{
    orig.as_ref().map(T::try_promote).transpose()
}

/// Type-agnostic macro for dereferencing machine-generated Offset16 abstractions
/// into Option<T> of the promotable dereference-value
///
/// This may become an identity function we refactor to eliminate intermediate Offset16 records
/// by using LetFormat or other forgetful chaining formats.
macro_rules! follow_link {
    ( opt ) => {
        (|offset| promote_opt(&offset.link))
    };
    ( req ) => {
        (|offset| promote_link(&offset.link))
    };
    ( req, $t:ty $(as $t2:ty)? ) => {
        (|offset| <$t $(as Promote<$t2>)?>::promote(&offset.link))
    }
}

// !SECTION

// SECTION - Generic (but niche-use-case) helper definitions
/// Lexically distinct Option for values that are theoretically non-Nullable and have no FromNull instance.
type Link<T> = Option<T>;

/// Vector of values with representation `T` that have a nominal semantic interpretation specified by `Sem`.
///
/// Though generic, the practical usages of this type are for distinguishing `ClassId := u16` and `GlyphId := u16`
/// semantics in GSUB/GPOS Lookup tables.
#[derive(Clone, Default)]
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

/// Marker type for SemVec (or any types above it) holding GlyphId-semantics u16 values
#[derive(Clone)]
struct GlyphId;
/// Marker type for SemVec (or any types above it) holding ClassId-semantics u16 values
#[derive(Clone)]
struct ClassId;

// !SECTION
/// Crate-private micro-module for compile-time 'same-type' assertions that can be chained
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
    ///
    /// E.g. `ReflType<A, B> = A` (which the compiler might not like, perhaps?)
    pub(crate) type ReflType<A, B> = <A as Refl<B>>::Solution;
}
use refl::ReflType;

/// Shorthand for qualifying a TryPromote::Error item
type TPErr<Src, Tgt> = <Tgt as TryPromote<Src>>::Error;

/// Shorthand for qualifying a TryFromRef::Error item in the same style as `TPErr`
type TFRErr<Src, Tgt> = <Tgt as TryFromRef<Src>>::Error;

/// Hint to remind us that a given error-type has strictly local provenance
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

pub type OpentypeCmapSubtable = opentype_cmap_subtable;

impl Promote<OpentypeCmapSubtable> for CmapSubtable {
    fn promote(orig: &OpentypeCmapSubtable) -> Self {
        CmapSubtable::AnyFormat(orig.format)
    }
}

#[derive(Debug, Clone)]
enum CmapSubtable {
    // STUB[scaffolding] - this is intentionally underimplemented to make encoding-record construction happen sooner for debugging
    AnyFormat(u16),
}

pub type OpentypeEncodingRecord = opentype_encoding_record;

#[derive(Debug, Clone)]
struct EncodingRecord {
    platform: u16,
    encoding: u16,
    subtable: Link<CmapSubtable>,
}

impl Promote<OpentypeEncodingRecord> for EncodingRecord {
    fn promote(orig: &OpentypeEncodingRecord) -> Self {
        EncodingRecord {
            platform: orig.platform,
            encoding: orig.encoding,
            subtable: promote_link(&orig.subtable_offset.link),
        }
    }
}

impl Promote<OpentypeCmap> for Cmap {
    fn promote(orig: &OpentypeCmap) -> Self {
        Cmap {
            version: orig.version,
            encoding_records: promote_vec(&orig.encoding_records),
        }
    }
}

#[derive(Clone, Debug)]
// STUB - enrich with any further details we care about presenting
struct Cmap {
    version: u16,
    encoding_records: Vec<EncodingRecord>,
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
pub enum PlatformEncodingLanguageId {
    Unicode(UnicodeEncodingId),                          // 0
    Macintosh(MacintoshEncodingId, MacintoshLanguageId), // 1
    Windows(WindowsEncodingId, WindowsLanguageId),       // 3
}

impl PlatformEncodingLanguageId {
    // NOTE - implicitly hardcoded to assume 'our locale' is English
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
pub enum UnicodeEncodingId {
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

/// Set of recognized Macintosh-specific encoding id values.
///
/// Cf. [https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[repr(u16)]
pub enum MacintoshEncodingId {
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
pub enum MacintoshLanguageId {
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
pub enum WindowsEncodingId {
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
pub struct WindowsLanguageId(u16);

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
    cmap: Cmap,
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

impl CoverageTable {
    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        match self {
            &CoverageTable::Format1 { ref glyph_array } => Box::new(glyph_array.iter().copied()),
            &CoverageTable::Format2 { ref range_records } => Box::new(
                range_records
                    .iter()
                    .flat_map(|rr| rr.start_glyph_id..=rr.end_glyph_id),
            ),
        }
    }
}

impl FromNull for CoverageTable {
    fn from_null() -> Self {
        // REVIEW - in practice, we could also pick Format2 instead, but Format1 is simpler...
        CoverageTable::Format1 {
            glyph_array: Vec::new(),
        }
    }
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
type ItemVariationStore = ();

// pub type OpentypeItemVariationStore = opentype_common_item_variation_store;
pub type OpentypeItemVariationStoreOffset = opentype_base_table_item_var_store_offset;

// TODO - Scaffolding to be replaced when ItemVariationStore gets proper implementation
impl Promote<()> for () {
    fn promote(_orig: &()) {}
}

impl TryPromote<OpentypeGdefTableData> for GdefTableDataMetrics {
    type Error = ReflType<TPErr<OpentypeMarkGlyphSet, MarkGlyphSet>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeGdefTableData) -> Result<Self, Self::Error> {
        Ok(match orig {
            OpentypeGdefTableData::Version1_0 => Self::default(),
            OpentypeGdefTableData::Version1_2(opentype_gdef_table_data_Version1_2 {
                mark_glyph_sets_def,
            }) => {
                let mark_glyph_sets_def = try_promote_opt(&mark_glyph_sets_def.link)?;
                GdefTableDataMetrics {
                    mark_glyph_sets_def,
                    item_var_store: None,
                }
            }
            OpentypeGdefTableData::Version1_3(opentype_gdef_table_data_Version1_3 {
                mark_glyph_sets_def,
                item_var_store,
            }) => {
                let mark_glyph_sets_def = try_promote_opt(&mark_glyph_sets_def.link)?;
                let item_var_store = promote_opt(&item_var_store.link);
                GdefTableDataMetrics {
                    mark_glyph_sets_def,
                    item_var_store,
                }
            }
        })
    }
}

#[derive(Clone, Debug, Default)]
struct GdefTableDataMetrics {
    mark_glyph_sets_def: Option<MarkGlyphSet>,
    item_var_store: Option<ItemVariationStore>,
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

impl FromNull for ClassDef {
    fn from_null() -> Self {
        ClassDef::Format1 {
            start_glyph_id: 0,
            class_value_array: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct RangeRecord<T> {
    start_glyph_id: u16,
    end_glyph_id: u16,
    value: T,
}

type ClassRangeRecord = RangeRecord<GlyphClass>;

pub type OpentypeClassRangeRecord = opentype_class_def_data_Format2_class_range_records;

impl Promote<OpentypeClassRangeRecord> for ClassRangeRecord {
    fn promote(orig: &OpentypeClassRangeRecord) -> Self {
        RangeRecord {
            start_glyph_id: orig.start_glyph_id,
            end_glyph_id: orig.end_glyph_id,
            value: orig.class,
        }
    }
}

pub type OpentypeClassDef = opentype_class_def;
pub type OpentypeClassDefData = opentype_class_def_data;
pub type OpentypeClassDefFormat1 = opentype_class_def_data_Format1;
pub type OpentypeClassDefFormat2 = opentype_class_def_data_Format2;

impl Promote<OpentypeClassDef> for ClassDef {
    fn promote(orig: &OpentypeClassDef) -> Self {
        Self::promote(&orig.data)
    }
}

impl Promote<OpentypeClassDefData> for ClassDef {
    fn promote(orig: &OpentypeClassDefData) -> Self {
        match orig {
            &OpentypeClassDefData::Format1(OpentypeClassDefFormat1 {
                start_glyph_id,
                ref class_value_array,
                ..
            }) => ClassDef::Format1 {
                start_glyph_id,
                class_value_array: class_value_array.clone(),
            },
            &OpentypeClassDefData::Format2(OpentypeClassDefFormat2 {
                ref class_range_records,
                ..
            }) => ClassDef::Format2 {
                class_range_records: promote_vec(class_range_records),
            },
        }
    }
}

#[derive(Clone, Debug, Default)]
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
                .map(follow_link!(req, AttachPoint))
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
            lig_glyphs.push(try_promote_from_null(&offset.link)?);
        }
        Ok(LigCaretList {
            coverage: CoverageTable::promote(&orig.coverage.link),
            lig_glyphs,
        })
    }
}

#[derive(Clone, Debug, Default)]
struct LigGlyph {
    caret_values: Vec<Link<CaretValue>>,
}

pub type OpentypeLigGlyph = opentype_gdef_table_lig_caret_list_link_lig_glyph_offsets_link;

impl TryPromote<OpentypeLigGlyph> for LigGlyph {
    type Error = ReflType<TPErr<OpentypeCaretValue, CaretValue>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeLigGlyph) -> Result<Self, Self::Error> {
        let mut caret_values = Vec::with_capacity(orig.caret_values.len());
        for offset in orig.caret_values.iter() {
            caret_values.push(try_promote_link(&offset.link)?); // &caret_value.data
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

/// Re-interprets a value with machine type `u8` as an `N`-bit signed integer
/// using the expected two's-complement representation (with machine type `i8`).
///
/// If `N == 8`, operationally equivalent to casting `raw as i8`.
///
/// # Panics
///
/// Due to the limited use of this function, `N` must be in the range `[2, 8]`,
/// and though it not necessarily checked, `raw` should contain no more than `N`
/// significant bits.
///
/// # Examples
///
/// ```no_run
/// // bits::<8>(raw) is the same as `raw as i8` so we omit those cases
///
/// // We only show the significant endpoints of the positive and negative ranges
/// assert_eq!(bits::<4>(0x0), 0x0);
/// assert_eq!(bits::<4>(0x7), 0x7);
/// assert_eq!(bits::<4>(0x8), -0x8);
/// assert_eq!(bits::<4>(0xF), -1);
///
/// // There are only four 2-bit values so we can list them all
/// assert_eq!(bits::<2>(0b00), 0);
/// assert_eq!(bits::<2>(0b01), 1);
/// assert_eq!(bits::<2>(0b10), -2);
/// assert_eq!(bits::<2>(0b11), -1);
/// ```
fn bits<const N: usize>(raw: u8) -> i8 {
    // Shortcut for when we have exactly 8 bits
    if N == 8 {
        return raw as i8;
    }
    assert!(N > 1 && N < 8);
    let range_max: i8 = 1 << N;
    let i_raw = raw as i8;
    if i_raw >= range_max / 2 {
        return i_raw - range_max;
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
    OtherTable { delta_format: u16 },
}

#[derive(Clone, Debug)]
enum CaretValue {
    DesignUnits(u16),  // Format1
    ContourPoint(u16), // Format2
    DesignUnitsWithTable {
        coordinate: u16,
        device: Link<DeviceOrVariationIndexTable>,
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
type OpentypeDVIOtherTable = opentype_common_device_or_variation_index_table_OtherTable;

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
            &OpentypeDeviceOrVariationIndexTable::OtherTable(OpentypeDVIOtherTable {
                delta_format,
                ..
            }) => Ok(DeviceOrVariationIndexTable::OtherTable { delta_format }),
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
                    device: try_promote_link(&table.link)?,
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
            lang_sys: promote_link(&orig.lang_sys.link),
        }
    }
}

#[derive(Clone, Debug)]
struct LangSysRecord {
    lang_sys_tag: u32,
    lang_sys: Link<LangSys>,
}

pub type OpentypeScriptTable = opentype_common_script_table;

impl Promote<OpentypeScriptTable> for ScriptTable {
    fn promote(orig: &OpentypeScriptTable) -> Self {
        ScriptTable {
            default_lang_sys: promote_link(&orig.default_lang_sys.link),
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
            script: promote_link(&orig.script.link),
        }
    }
}

#[derive(Clone, Debug)]
struct ScriptRecord {
    script_tag: u32,
    script: Link<ScriptTable>,
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
            feature: promote_link(&orig.feature.link),
        }
    }
}

#[derive(Clone, Debug)]
struct FeatureRecord {
    feature_tag: u32,
    feature: Link<FeatureTable>,
}

pub type OpentypeGposLookupSubtableExt =
    opentype_gpos_table_lookup_list_link_lookups_link_subtables_link;
pub type OpentypeGsubLookupSubtableExt =
    opentype_gsub_table_lookup_list_link_lookups_link_subtables_link;

pub type OpentypeSubstExtension = opentype_layout_subst_extension;
pub type OpentypePosExtension = opentype_layout_pos_extension;

pub type OpentypeGposLookupSubtable = opentype_layout_ground_pos;
pub type OpentypeGsubLookupSubtable = opentype_layout_ground_subst;

#[derive(Debug)]
pub enum BadExtensionError {
    InconsistentLookup(u16, u16),
}

impl std::fmt::Display for BadExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadExtensionError::InconsistentLookup(expected, actual) => {
                write!(f, "layout extension subtable has inconsistent extension_lookup_type (expecting {expected}, found {actual})")
            }
        }
    }
}

impl std::error::Error for BadExtensionError {}

impl TryPromote<OpentypeGsubLookupSubtableExt> for LookupSubtable {
    type Error = ReflType<
        ReflType<
            TPErr<OpentypeSubstExtension, LookupSubtable>,
            TPErr<OpentypeGsubLookupSubtable, LookupSubtable>,
        >,
        std::convert::Infallible,
    >;

    fn try_promote(orig: &OpentypeGsubLookupSubtableExt) -> Result<Self, Self::Error> {
        match orig {
            OpentypeGsubLookupSubtableExt::GroundSubst(ground) => {
                LookupSubtable::try_promote(ground)
            }
            OpentypeGsubLookupSubtableExt::SubstExtension(ext) => LookupSubtable::try_promote(ext),
        }
    }
}

impl TryPromote<OpentypeSubstExtension> for LookupSubtable {
    type Error =
        ReflType<TPErr<OpentypeGsubLookupSubtable, LookupSubtable>, std::convert::Infallible>;

    fn try_promote(orig: &OpentypeSubstExtension) -> Result<Self, Self::Error> {
        match &orig.extension_offset.link {
            None => unreachable!("SubstExtension Offset should not be Null"),
            Some(ground) => LookupSubtable::try_promote(ground),
        }
    }
}

impl TryPromote<OpentypeGsubLookupSubtable> for LookupSubtable {
    type Error = Local<std::convert::Infallible>; // this is only temporarily the case, as we are almost certainly going to have errors in at least on lookup subtable format

    fn try_promote(orig: &OpentypeGsubLookupSubtable) -> Result<Self, Self::Error> {
        Ok(match orig {
            OpentypeGsubLookupSubtable::SingleSubst(single_subst) => {
                LookupSubtable::SingleSubst(SingleSubst::promote(single_subst))
            }
            OpentypeGsubLookupSubtable::MultipleSubst(multi_subst) => {
                LookupSubtable::MultipleSubst(MultipleSubst::promote(multi_subst))
            }
            OpentypeGsubLookupSubtable::AlternateSubst(alt_subst) => {
                LookupSubtable::AlternateSubst(AlternateSubst::promote(alt_subst))
            }
            OpentypeGsubLookupSubtable::LigatureSubst(ligature_subst) => {
                LookupSubtable::LigatureSubst(LigatureSubst::promote(ligature_subst))
            }
            OpentypeGsubLookupSubtable::SequenceContext(seq_ctx) => {
                LookupSubtable::SequenceContext(SequenceContext::promote(seq_ctx))
            }
            OpentypeGsubLookupSubtable::ChainedSequenceContext(chain_ctx) => {
                LookupSubtable::ChainedSequenceContext(ChainedSequenceContext::promote(chain_ctx))
            }
            OpentypeGsubLookupSubtable::ReverseChainSingleSubst(rev_chain_single_subst) => {
                LookupSubtable::ReverseChainSingleSubst(ReverseChainSingleSubst::promote(
                    rev_chain_single_subst,
                ))
            }
        })
    }
}

impl TryPromote<OpentypeGposLookupSubtableExt> for LookupSubtable {
    type Error = ReflType<
        ReflType<
            TPErr<OpentypeGposLookupSubtable, LookupSubtable>,
            TPErr<OpentypePosExtension, LookupSubtable>,
        >,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeGposLookupSubtableExt) -> Result<Self, Self::Error> {
        match orig {
            OpentypeGposLookupSubtableExt::PosExtension(ext) => LookupSubtable::try_promote(ext),
            OpentypeGposLookupSubtableExt::GroundPos(ground) => LookupSubtable::try_promote(ground),
        }
    }
}

impl TryPromote<OpentypePosExtension> for LookupSubtable {
    type Error =
        ReflType<TPErr<OpentypeGposLookupSubtable, LookupSubtable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePosExtension) -> Result<Self, Self::Error> {
        match &orig.extension_offset.link {
            None => unreachable!("PosExtension Offset should not be Null"),
            Some(ground) => LookupSubtable::try_promote(ground),
        }
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
            OpentypeGposLookupSubtable::MarkBasePos(mb_pos) => {
                LookupSubtable::MarkBasePos(MarkBasePos::try_promote(mb_pos)?)
            }
            OpentypeGposLookupSubtable::MarkLigPos(ml_pos) => {
                LookupSubtable::MarkLigPos(MarkLigPos::try_promote(ml_pos)?)
            }
            OpentypeGposLookupSubtable::MarkMarkPos(mm_pos) => {
                LookupSubtable::MarkMarkPos(MarkMarkPos::try_promote(mm_pos)?)
            }
            OpentypeGposLookupSubtable::SequenceContext(seq_ctx) => {
                LookupSubtable::SequenceContext(SequenceContext::promote(seq_ctx))
            }
            OpentypeGposLookupSubtable::ChainedSequenceContext(chain_ctx) => {
                LookupSubtable::ChainedSequenceContext(ChainedSequenceContext::promote(chain_ctx))
            }
        })
    }
}

#[derive(Clone, Debug)]
enum LookupSubtable {
    SinglePos(SinglePos),
    PairPos(PairPos),
    CursivePos(CursivePos),
    MarkBasePos(MarkBasePos),
    MarkLigPos(MarkLigPos),
    MarkMarkPos(MarkMarkPos),

    SequenceContext(SequenceContext),
    ChainedSequenceContext(ChainedSequenceContext),

    SingleSubst(SingleSubst),
    MultipleSubst(MultipleSubst),
    AlternateSubst(AlternateSubst),
    LigatureSubst(LigatureSubst),
    ReverseChainSingleSubst(ReverseChainSingleSubst),
}

pub type OpentypeMarkMarkPos = opentype_layout_mark_mark_pos;

impl TryPromote<OpentypeMarkMarkPos> for MarkMarkPos {
    type Error = ReflType<
        ReflType<TPErr<OpentypeMarkArray, MarkArray>, TPErr<OpentypeMark2Array, Mark2Array>>,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeMarkMarkPos) -> Result<Self, Self::Error> {
        Ok(MarkMarkPos {
            mark1_coverage: CoverageTable::promote(&orig.mark1_coverage_offset.link),
            mark2_coverage: CoverageTable::promote(&orig.mark2_coverage_offset.link),
            mark1_array: try_promote_from_null(&orig.mark1_array_offset.link)?,
            mark2_array: try_promote_from_null(&orig.mark2_array_offset.link)?,
        })
    }
}

#[derive(Debug, Clone)]
struct MarkMarkPos {
    mark1_coverage: CoverageTable,
    mark2_coverage: CoverageTable,
    mark1_array: MarkArray,
    mark2_array: Mark2Array,
}

pub type OpentypeMark2Array = opentype_layout_mark_mark_pos_mark2_array_offset_link;

impl TryPromote<OpentypeMark2Array> for Mark2Array {
    type Error = ReflType<TPErr<OpentypeMark2Record, Mark2Record>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeMark2Array) -> Result<Self, Self::Error> {
        Ok(Mark2Array {
            mark2_records: try_promote_vec(&orig.mark2_records)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct Mark2Array {
    mark2_records: Vec<Mark2Record>,
}

pub type OpentypeMark2Record = opentype_layout_mark_mark_pos_mark2_array_offset_link_mark2_records;

impl TryPromote<OpentypeMark2Record> for Mark2Record {
    type Error = ReflType<TPErr<OpentypeAnchorTable, AnchorTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeMark2Record) -> Result<Self, Self::Error> {
        let mut mark2_anchors = Vec::with_capacity(orig.mark2_anchor_offsets.len());
        for offset in orig.mark2_anchor_offsets.iter() {
            mark2_anchors.push(try_promote_opt(&offset.link)?);
        }
        Ok(Mark2Record { mark2_anchors })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct Mark2Record {
    mark2_anchors: Vec<Option<AnchorTable>>,
}

pub type OpentypeMarkLigPos = opentype_layout_mark_lig_pos;

impl TryPromote<OpentypeMarkLigPos> for MarkLigPos {
    type Error = ReflType<
        ReflType<TPErr<OpentypeLigatureArray, LigatureArray>, TPErr<OpentypeMarkArray, MarkArray>>,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeMarkLigPos) -> Result<Self, Self::Error> {
        Ok(MarkLigPos {
            mark_coverage: CoverageTable::promote(&orig.mark_coverage_offset.link),
            ligature_coverage: CoverageTable::promote(&orig.ligature_coverage_offset.link),
            mark_array: try_promote_from_null(&orig.mark_array_offset.link)?,
            ligature_array: try_promote_from_null(&orig.ligature_array_offset.link)?,
        })
    }
}

#[derive(Debug, Clone)]
struct MarkLigPos {
    mark_coverage: CoverageTable,
    ligature_coverage: CoverageTable,
    mark_array: MarkArray,
    ligature_array: LigatureArray,
}

pub type OpentypeLigatureArray = opentype_layout_mark_lig_pos_ligature_array_offset_link;

impl TryPromote<OpentypeLigatureArray> for LigatureArray {
    type Error = ReflType<TPErr<OpentypeLigatureAttach, LigatureAttach>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeLigatureArray) -> Result<Self, Self::Error> {
        let mut ligature_attach = Vec::with_capacity(orig.ligature_attach_offsets.len());
        for offset in orig.ligature_attach_offsets.iter() {
            ligature_attach.push(try_promote_from_null(&offset.link)?);
        }
        Ok(LigatureArray { ligature_attach })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct LigatureArray {
    ligature_attach: Vec<LigatureAttach>,
}

pub type OpentypeLigatureAttach =
    opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets_link;

impl TryPromote<OpentypeLigatureAttach> for LigatureAttach {
    type Error = ReflType<TPErr<OpentypeComponentRecord, ComponentRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeLigatureAttach) -> Result<Self, Self::Error> {
        Ok(LigatureAttach {
            component_records: try_promote_vec(&orig.component_records)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct LigatureAttach {
    component_records: Vec<ComponentRecord>,
}

pub type OpentypeComponentRecord = opentype_layout_mark_lig_pos_ligature_array_offset_link_ligature_attach_offsets_link_component_records;

impl TryPromote<OpentypeComponentRecord> for ComponentRecord {
    type Error = ReflType<TPErr<OpentypeAnchorTable, AnchorTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeComponentRecord) -> Result<Self, Self::Error> {
        let mut ligature_anchors = Vec::with_capacity(orig.ligature_anchor_offsets.len());
        for offset in orig.ligature_anchor_offsets.iter() {
            ligature_anchors.push(try_promote_opt(&offset.link)?);
        }

        Ok(ComponentRecord { ligature_anchors })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct ComponentRecord {
    ligature_anchors: Vec<Option<AnchorTable>>,
}

pub type OpentypeMarkBasePos = opentype_layout_mark_base_pos;

impl TryPromote<OpentypeMarkBasePos> for MarkBasePos {
    type Error = ReflType<
        ReflType<TPErr<OpentypeBaseArray, BaseArray>, TPErr<OpentypeMarkArray, MarkArray>>,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeMarkBasePos) -> Result<Self, Self::Error> {
        Ok(MarkBasePos {
            mark_coverage: CoverageTable::promote(&orig.mark_coverage_offset.link),
            base_coverage: CoverageTable::promote(&orig.base_coverage_offset.link),
            mark_array: try_promote_from_null(&orig.mark_array_offset.link)?,
            base_array: try_promote_from_null(&orig.base_array_offset.link)?,
        })
    }
}

pub type OpentypeMarkArray = opentype_layout_mark_array;

impl TryPromote<OpentypeMarkArray> for MarkArray {
    type Error = ReflType<TPErr<OpentypeMarkRecord, MarkRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeMarkArray) -> Result<Self, Self::Error> {
        Ok(MarkArray {
            mark_records: try_promote_vec(&orig.mark_records)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct MarkArray {
    mark_records: Vec<MarkRecord>,
}

pub type OpentypeMarkRecord = opentype_layout_mark_array_mark_records;

impl TryPromote<OpentypeMarkRecord> for MarkRecord {
    type Error = ReflType<TPErr<OpentypeAnchorTable, AnchorTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeMarkRecord) -> Result<Self, Self::Error> {
        Ok(MarkRecord {
            mark_class: orig.mark_class,
            mark_anchor: try_promote_link(&orig.mark_anchor_offset.link)?,
        })
    }
}

#[derive(Debug, Clone)]
struct MarkRecord {
    mark_class: u16,
    mark_anchor: Link<AnchorTable>,
}

pub type OpentypeBaseArray = opentype_layout_mark_base_pos_base_array_offset_link;

impl TryPromote<OpentypeBaseArray> for BaseArray {
    type Error = ReflType<TPErr<OpentypeBaseRecord, BaseRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeBaseArray) -> Result<Self, Self::Error> {
        Ok(BaseArray {
            base_records: try_promote_vec(&orig.base_records)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct BaseArray {
    base_records: Vec<BaseRecord>,
}

pub type OpentypeBaseRecord = opentype_layout_mark_base_pos_base_array_offset_link_base_records;

impl TryPromote<OpentypeBaseRecord> for BaseRecord {
    type Error = ReflType<TPErr<OpentypeAnchorTable, AnchorTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeBaseRecord) -> Result<Self, Self::Error> {
        let mut base_anchors = Vec::with_capacity(orig.base_anchor_offsets.len());
        for offset in orig.base_anchor_offsets.iter() {
            base_anchors.push(try_promote_opt(&offset.link)?);
        }

        Ok(BaseRecord { base_anchors })
    }
}

#[derive(Debug, Clone, Default)]
struct BaseRecord {
    base_anchors: Vec<Option<AnchorTable>>,
}
#[derive(Debug, Clone)]
struct MarkBasePos {
    mark_coverage: CoverageTable,
    base_coverage: CoverageTable,
    mark_array: MarkArray,
    base_array: BaseArray,
}

pub type OpentypeReverseChainSingleSubst = opentype_layout_reverse_chain_single_subst;

impl Promote<OpentypeReverseChainSingleSubst> for ReverseChainSingleSubst {
    fn promote(orig: &OpentypeReverseChainSingleSubst) -> Self {
        fn promote_coverages(
            offsets: &'_ Vec<opentype_layout_reverse_chain_single_subst_coverage>,
        ) -> Vec<CoverageTable> {
            offsets
                .iter()
                .map(|offset| CoverageTable::promote(&offset.link))
                .collect()
        }
        ReverseChainSingleSubst {
            coverage: CoverageTable::promote(&orig.coverage.link),
            backtrack_coverages: promote_coverages(&orig.backtrack_coverage_tables),
            lookahead_coverages: promote_coverages(&orig.lookahead_coverage_tables),
            glyph_count: orig.glyph_count,
            substitute_glyph_ids: orig.substitute_glyph_ids.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct ReverseChainSingleSubst {
    coverage: CoverageTable,
    backtrack_coverages: Vec<CoverageTable>,
    lookahead_coverages: Vec<CoverageTable>,
    glyph_count: u16, // NOTE - this field is technically extraneous due to being equal to `substitute_glyph_ids.len() as u16`
    substitute_glyph_ids: Vec<u16>,
}

pub type OpentypeLigatureSubst = opentype_layout_ligature_subst;

impl Promote<OpentypeLigatureSubst> for LigatureSubst {
    fn promote(orig: &OpentypeLigatureSubst) -> Self {
        LigatureSubst {
            coverage: CoverageTable::promote(&orig.coverage.link),
            ligature_sets: orig
                .ligature_sets
                .iter()
                .map(|offset| LigatureSet::promote(&offset.link))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct LigatureSubst {
    coverage: CoverageTable,
    ligature_sets: Vec<LigatureSet>,
}

pub type OpentypeLigatureSet = opentype_layout_ligature_subst_ligature_sets_link;

#[derive(Debug, Clone, Default)]
struct LigatureSet {
    ligatures: Vec<Link<Ligature>>,
}

pub type OpentypeLigature = opentype_layout_ligature_subst_ligature_sets_link_ligatures_link;

#[derive(Debug, Clone)]
struct Ligature {
    ligature_glyph: u16,
    component_count: u16,
    component_glyph_ids: Vec<u16>,
}

impl Promote<OpentypeLigature> for Ligature {
    fn promote(orig: &OpentypeLigature) -> Self {
        Ligature {
            ligature_glyph: orig.ligature_glyph,
            component_count: orig.component_count,
            component_glyph_ids: orig.component_glyph_ids.clone(),
        }
    }
}

impl Promote<OpentypeLigatureSet> for LigatureSet {
    fn promote(orig: &OpentypeLigatureSet) -> Self {
        LigatureSet {
            ligatures: orig
                .ligatures
                .iter()
                .map(|offset| promote_link(&offset.link))
                .collect(),
        }
    }
}

pub type OpentypeAlternateSubst = opentype_layout_alternate_subst;

impl Promote<OpentypeAlternateSubst> for AlternateSubst {
    fn promote(orig: &OpentypeAlternateSubst) -> Self {
        AlternateSubst {
            coverage: CoverageTable::promote(&orig.coverage.link),
            alternate_sets: orig
                .alternate_sets
                .iter()
                .map(|offset| AlternateSet::promote(&offset.link))
                .collect(),
        }
    }
}

pub type OpentypeAlternateSet = opentype_layout_alternate_subst_alternate_sets_link;

impl Promote<OpentypeAlternateSet> for AlternateSet {
    fn promote(orig: &OpentypeAlternateSet) -> Self {
        AlternateSet {
            alternate_glyph_ids: orig.alternate_glyph_ids.clone(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct AlternateSet {
    // REVIEW - we can implement FromNull for AlternateSet perhaps
    alternate_glyph_ids: Vec<u16>,
}

#[derive(Clone, Debug)]
struct AlternateSubst {
    coverage: CoverageTable,
    alternate_sets: Vec<AlternateSet>,
}

pub type OpentypeMultipleSubst = opentype_layout_multiple_subst;
pub type OpentypeMultipleSubstInner = opentype_layout_multiple_subst_subst;
pub type OpentypeMultipleSubstFormat1 = opentype_layout_multiple_subst_subst_Format1;

impl Promote<OpentypeMultipleSubst> for MultipleSubst {
    fn promote(orig: &OpentypeMultipleSubst) -> Self {
        MultipleSubst {
            coverage: CoverageTable::promote(&orig.coverage.link),
            subst: MultipleSubstInner::promote(&orig.subst),
        }
    }
}

impl Promote<OpentypeMultipleSubstInner> for MultipleSubstInner {
    fn promote(orig: &OpentypeMultipleSubstInner) -> Self {
        match orig {
            OpentypeMultipleSubstInner::Format1(f1) => MultipleSubstFormat1::promote(f1),
        }
    }
}

#[derive(Debug, Clone)]
struct MultipleSubst {
    coverage: CoverageTable,
    subst: MultipleSubstInner,
}

type MultipleSubstInner = MultipleSubstFormat1; // FIXME - nominally an enum but only one variant so we inline

impl Promote<OpentypeMultipleSubstFormat1> for MultipleSubstFormat1 {
    fn promote(orig: &OpentypeMultipleSubstFormat1) -> Self {
        MultipleSubstFormat1 {
            sequences: orig
                .sequences
                .iter()
                .map(follow_link!(req, SequenceTable))
                .collect(),
        }
    }
}

pub type OpentypeSequenceTable = opentype_layout_multiple_subst_subst_Format1_sequences_link;

impl Promote<OpentypeSequenceTable> for SequenceTable {
    fn promote(orig: &OpentypeSequenceTable) -> Self {
        Self {
            substitute_glyph_ids: orig.substitute_glyph_ids.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct SequenceTable {
    substitute_glyph_ids: Vec<u16>,
}

#[derive(Debug, Clone)]
struct MultipleSubstFormat1 {
    sequences: Vec<SequenceTable>,
}

pub type OpentypeSingleSubst = opentype_layout_single_subst;
pub type OpentypeSingleSubstInner = opentype_layout_single_subst_subst;
pub type OpentypeSingleSubstFormat1 = opentype_layout_single_subst_subst_Format1;
pub type OpentypeSingleSubstFormat2 = opentype_layout_single_subst_subst_Format2;

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
pub type OpentypeChainedRule = opentype_common_chained_sequence_context_subst_Format1_chained_seq_rule_sets_link_chained_seq_rules_link;

impl<Sem> Promote<OpentypeChainedRuleSet> for ChainedRuleSet<Sem>
where
    ChainedRule<Sem>: Promote<OpentypeChainedRule>,
{
    fn promote(orig: &OpentypeChainedRuleSet) -> Self {
        orig.chained_seq_rules
            .iter()
            .map(|offset| promote_link(&offset.link))
            .collect()
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
            seq_lookup_records: orig.seq_lookup_records.clone(),
        }
    }
}

type ChainedRuleSet<Sem> = Vec<Link<ChainedRule<Sem>>>;

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
                .map(follow_link!(req, ChainedRuleSet<GlyphId>))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct ChainedSequenceContextFormat1 {
    coverage: CoverageTable,
    chained_seq_rule_sets: Vec<ChainedRuleSet<GlyphId>>,
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
                .map(follow_link!(req, ChainedRuleSet<ClassId>))
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
    chained_class_seq_rule_sets: Vec<ChainedRuleSet<ClassId>>,
}

impl Promote<OpentypeChainedSequenceContextFormat3> for ChainedSequenceContextFormat3 {
    fn promote(orig: &OpentypeChainedSequenceContextFormat3) -> Self {
        type OpentypeCoverageTableLink = opentype_layout_reverse_chain_single_subst_coverage;
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
            seq_lookup_records: orig.seq_lookup_records.clone(),
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
            seq_rule_sets: orig.seq_rule_sets.iter().map(follow_link!(req)).collect(),
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
                .map(follow_link!(opt))
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

// REVIEW - if RuleSet becomes an alias instead of a new-type, remove this definition and rename the following impl on Vec<Rule>
impl Promote<OpentypeRuleSet> for RuleSet {
    fn promote(orig: &OpentypeRuleSet) -> Self {
        Self(<Vec<Link<Rule>>>::promote(orig))
    }
}

impl Promote<OpentypeRuleSet> for Vec<Link<Rule>> {
    fn promote(orig: &OpentypeRuleSet) -> Self {
        orig.rules
            .iter()
            .map(|offset| promote_link(&offset.link))
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
// REVIEW - should this be a simple type alias instead?
struct RuleSet(Vec<Link<Rule>>);

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

pub type OpentypeCursivePos = opentype_layout_cursive_pos;

impl TryPromote<OpentypeCursivePos> for CursivePos {
    type Error = ReflType<TPErr<OpentypeEntryExitRecord, EntryExitRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeCursivePos) -> Result<Self, Self::Error> {
        Ok(CursivePos {
            coverage: CoverageTable::promote(&orig.coverage.link),
            entry_exit_records: try_promote_vec(&orig.entry_exit_records)?,
        })
    }
}

#[derive(Debug, Clone)]
struct CursivePos {
    coverage: CoverageTable,
    entry_exit_records: Vec<EntryExitRecord>,
}

pub type OpentypeEntryExitRecord = opentype_layout_cursive_pos_entry_exit_records;

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

pub type OpentypePairPos = opentype_layout_pair_pos;

pub type OpentypePairPosSubtable = opentype_layout_pair_pos_subtable;
pub type OpentypePairPosFormat1 = opentype_layout_pair_pos_subtable_Format1;
pub type OpentypePairPosFormat2 = opentype_layout_pair_pos_subtable_Format2;

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
            let pair_set = try_promote_from_null(&offset.link)?;
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

pub type OpentypeClass2Record =
    opentype_layout_pair_pos_subtable_Format2_class1_records_class2_records;

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

pub type OpentypePairSet = opentype_layout_pair_pos_subtable_Format1_pair_sets_link;
pub type OpentypePairValueRecord =
    opentype_layout_pair_pos_subtable_Format1_pair_sets_link_pair_value_records;

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

pub type OpentypeSinglePos = opentype_layout_single_pos;
pub type OpentypeSinglePosSubtable = opentype_layout_single_pos_subtable;
pub type OpentypeSinglePosFormat1 = opentype_layout_single_pos_subtable_Format1;
pub type OpentypeSinglePosFormat2 = opentype_layout_single_pos_subtable_Format2;

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
        let follow = |device: &Option<opentype_common_value_record_x_advance_device>| match device {
            Some(dev) => try_promote_opt(&dev.link),
            None => Ok(None),
        };
        Ok(ValueRecord {
            x_placement: orig.x_placement.map(as_s16),
            y_placement: orig.y_placement.map(as_s16),
            x_advance: orig.x_advance.map(as_s16),
            y_advance: orig.y_advance.map(as_s16),
            x_placement_device: follow(&orig.x_placement_device)?,
            y_placement_device: follow(&orig.y_placement_device)?,
            x_advance_device: follow(&orig.x_advance_device)?,
            y_advance_device: follow(&orig.y_advance_device)?,
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
        const POS_EXTENSION_LOOKUP_TYPE: u16 = 9;

        let lookup_type = match orig.lookup_type {
            POS_EXTENSION_LOOKUP_TYPE => {
                let mut extension_lookup_type: Option<u16> = None;
                for (_ix, offset) in orig.subtables.iter().enumerate() {
                    match &offset.link {
                        None => eprintln!("empty subtable at lookup {_ix}"),
                        Some(subtable @ OpentypeGposLookupSubtableExt::PosExtension(ext)) => {
                            if let Some(tmp) = extension_lookup_type.replace(ext.extension_lookup_type)
                            {
                                if tmp != ext.extension_lookup_type {
                                    // FIXME - we don't have an error type that makes this easy to fold into the returned error, so we panic for now
                                    let _err = BadExtensionError::InconsistentLookup(
                                        tmp,
                                        ext.extension_lookup_type,
                                    );
                                    panic!("{_err}");
                                }
                            }
                            subtables.push(LookupSubtable::try_promote(subtable)?);
                        }
                        Some(_other) => unreachable!(
                            "lookup type is PosExtension, found non-PosExtension subtable: {_other:?}"
                        ),
                    }
                }
                extension_lookup_type.unwrap_or(POS_EXTENSION_LOOKUP_TYPE)
            }
            ground_type => {
                for (_ix, offset) in orig.subtables.iter().enumerate() {
                    if let Some(subtable) = try_promote_link(&offset.link)? {
                        subtables.push(subtable);
                    } else {
                        // REVIEW - this is not necessary but helps us track whether this case happens
                        eprintln!("empty subtable at lookup {_ix}");
                    }
                }
                ground_type
            }
        };

        // NOTE - the fact that a LookupTable had an Extension lookup type originally is erased
        Ok(LookupTable {
            lookup_type,
            lookup_flag: orig.lookup_flag,
            subtables,
            mark_filtering_set: orig.mark_filtering_set,
        })
    }
}

impl TryPromote<OpentypeGsubLookupTable> for LookupTable {
    type Error = ReflType<
        TPErr<OpentypeGsubLookupSubtable, LookupSubtable>,
        std::convert::Infallible, // for compatibility with GPOS promotion, can't use BadExtensionError as the error types woudl collide
    >;

    fn try_promote(orig: &OpentypeGsubLookupTable) -> Result<Self, Self::Error> {
        let mut subtables = Vec::with_capacity(orig.subtables.len());
        const SUBST_EXTENSION_LOOKUP_TYPE: u16 = 7;

        let lookup_type = match orig.lookup_type {
            SUBST_EXTENSION_LOOKUP_TYPE => {
                let mut extension_lookup_type: Option<u16> = None;
                for (_ix, offset) in orig.subtables.iter().enumerate() {
                    match &offset.link {
                        None => eprintln!("empty subtable at lookup {_ix}"),
                        Some(subtable @ OpentypeGsubLookupSubtableExt::SubstExtension(ext)) => {
                            if let Some(tmp) = extension_lookup_type.replace(ext.extension_lookup_type)
                            {
                                if tmp != ext.extension_lookup_type {
                                    // FIXME - we don't have an error type that makes this easy to fold into the returned error, so we panic for now
                                    let _err = BadExtensionError::InconsistentLookup(
                                        tmp,
                                        ext.extension_lookup_type,
                                    );
                                    panic!("{_err}");
                                }
                            }
                            subtables.push(LookupSubtable::try_promote(subtable)?);
                        }
                        Some(_other) => unreachable!(
                            "lookup type is SubstExtension, found non-SubstExtension subtable: {_other:?}"
                        ),
                    }
                }
                extension_lookup_type.unwrap_or(SUBST_EXTENSION_LOOKUP_TYPE)
            }
            ground_type => {
                for (_ix, offset) in orig.subtables.iter().enumerate() {
                    if let Some(subtable) = try_promote_link(&offset.link)? {
                        subtables.push(subtable);
                    } else {
                        // REVIEW - this is not necessary but helps us track whether this case happens
                        eprintln!("empty subtable at lookup {_ix}");
                    }
                }
                ground_type
            }
        };

        // NOTE - the fact that a LookupTable had an Extension lookup type originally is erased
        Ok(LookupTable {
            lookup_type,
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
type LookupList = Vec<Link<LookupTable>>;

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
            accum.push(try_promote_link(&offset.link)?);
        }
        Ok(accum)
    }
}

impl TryPromote<OpentypeGsubLookupList> for LookupList {
    type Error = TPErr<OpentypeGsubLookupTable, LookupTable>;

    fn try_promote(orig: &OpentypeGsubLookupList) -> Result<Self, Self::Error> {
        let mut accum = Vec::with_capacity(orig.lookups.len());
        for offset in orig.lookups.iter() {
            accum.push(try_promote_link(&offset.link)?);
        }
        Ok(accum)
    }
}

pub type OpentypeFeatureVariations = opentype_layout_feature_variations;

impl Promote<OpentypeFeatureVariations> for FeatureVariations {
    fn promote(_orig: &OpentypeFeatureVariations) -> FeatureVariations {
        // STUB - implement proper promotion rules once feature variation type is refined
        ()
    }
}

// STUB - implement proper model-type for FeatureVariations values
type FeatureVariations = ();

#[derive(Clone, Debug)]
/// Common API type for summarizing GPOS and GSUB
struct LayoutMetrics {
    major_version: u16,
    minor_version: u16,
    script_list: ScriptList,
    feature_list: FeatureList,
    lookup_list: LookupList,
    feature_variations: Option<FeatureVariations>,
}

#[derive(Clone, Debug)]
struct BaseMetrics {
    major_version: u16,
    minor_version: u16,
    // STUB - add more fields as desired
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
    base: Option<BaseMetrics>,
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

pub fn analyze_font_fast(test_file: &str) -> TestResult<()> {
    let buffer = std::fs::read(std::path::Path::new(test_file))?;
    let mut input = Parser::new(&buffer);
    let _ = Decoder_opentype_main(&mut input)?;
    Ok(())
}

pub fn analyze_font(test_file: &str) -> TestResult<OpentypeMetrics> {
    let buffer = std::fs::read(std::path::Path::new(test_file))?;
    let mut input = Parser::new(&buffer);
    let font = Decoder_opentype_main(&mut input)?;
    // TODO: do we want to collect (and return) any metrics here?
    match font.directory {
        opentype_main_directory::TTCHeader(multi) => {
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
        opentype_main_directory::TableDirectory(single) => Ok(OpentypeMetrics::SingleFont(
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
            Cmap::promote(cmap)
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
                    let buf = if let Some(buf) = &record.offset.link {
                        plat_encoding_lang.convert(buf)
                    } else {
                        String::new()
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
                        for (_ix, record) in v1data.lang_tag_records.iter().enumerate() {
                            let lang_tag = if let Some(tag) = &record.offset.link {
                                utf16be_convert(tag)
                            } else {
                                if cfg!(debug_assertions) {
                                    eprintln!("lang_tag record offset is 0 at index {_ix}");
                                }
                                String::new()
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
        let base = {
            let base = &dir.table_links.base;
            base.as_ref()
                .map(|base| {
                    TestResult::Ok(BaseMetrics {
                        major_version: base.major_version,
                        minor_version: base.minor_version,
                    })
                })
                .transpose()?
        };
        let gdef = {
            let gdef = &dir.table_links.gdef;
            gdef.as_ref()
                .map(|gdef| {
                    TestResult::Ok(GdefMetrics {
                        major_version: gdef.major_version,
                        minor_version: gdef.minor_version,
                        glyph_class_def: promote_opt(&gdef.glyph_class_def.link),
                        attach_list: promote_opt(&gdef.attach_list.link),
                        lig_caret_list: try_promote_opt(&gdef.lig_caret_list.link)?,
                        mark_attach_class_def: promote_opt(&gdef.mark_attach_class_def.link),
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
                        script_list: ScriptList::promote(&gpos.script_list.link),
                        feature_list: FeatureList::promote(&gpos.feature_list.link),
                        lookup_list: try_promote_from_null(&gpos.lookup_list.link)?,
                        feature_variations: gpos
                            .feature_variations_offset
                            .as_ref()
                            .map(|offset| promote_link(&offset.link))
                            .flatten(),
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
                        script_list: ScriptList::promote(&gsub.script_list.link),
                        feature_list: FeatureList::promote(&gsub.feature_list.link),
                        lookup_list: try_promote_from_null(&gsub.lookup_list.link)?,
                        feature_variations: gsub
                            .feature_variations_offset
                            .as_ref()
                            .map(|offset| promote_link(&offset.link))
                            .flatten(),
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
            base,
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
        b"GDEF" | b"GPOS" | b"GSUB" | b"BASE" => false,
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
    println!("(sfntVersion: {})", format_magic(magic));
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
        println!("{}: [MISSING IMPL]", format_magic(*id));
    }
}

fn show_required_metrics(required: &RequiredTableMetrics, conf: &Config) {
    show_cmap_metrics(&required.cmap, conf);
    show_head_metrics(&required.head, conf);
    show_hhea_metrics(&required.hhea, conf);
    show_hmtx_metrics(&required.hmtx, conf);
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
    show_base_metrics(&optional.base, conf);
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
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
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
            match &data.mark_glyph_sets_def {
                None => println!("\tMarkGlyphSet: <none>"),
                Some(mgs) => show_mark_glyph_set(mgs, conf),
            }
            match &data.item_var_store {
                None => println!("\tItemVariationStore: <none>"),
                Some(ivs) => show_item_variation_store(ivs),
            }
        }
    }
}

fn show_base_metrics(base: &Option<BaseMetrics>, _conf: &Config) {
    if let Some(BaseMetrics {
        major_version,
        minor_version,
    }) = base
    {
        println!(
            "BASE: version {}",
            format_version_major_minor(*major_version, *minor_version)
        )
        // STUB - add print methods (possibly gated by verbosity levels) as BaseMetrics gets more fields
    }
}

fn format_table_disc(disc: TableDiscriminator) -> &'static str {
    match disc {
        TableDiscriminator::Gpos => "GPOS",
        TableDiscriminator::Gsub => "GSUB",
    }
}

fn show_layout_metrics(layout: &Option<LayoutMetrics>, ctxt: Ctxt, conf: &Config) {
    if let Some(LayoutMetrics {
        major_version,
        minor_version,
        script_list,
        feature_list,
        lookup_list,
        feature_variations: _feature_variations,
    }) = layout
    {
        println!(
            "{}: version {}",
            format_table_disc(ctxt.get_disc().expect("Ctxt missing TableDiscriminator")),
            format_version_major_minor(*major_version, *minor_version)
        );
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            show_script_list(&script_list, conf);
            show_feature_list(&feature_list, conf);
            show_lookup_list(&lookup_list, ctxt, conf);
        }
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
                let Some(ScriptTable {
                    default_lang_sys,
                    lang_sys_records,
                }) = &item.script
                else {
                    unreachable!("missing ScriptTable at index {ix} in ScriptList");
                };
                println!("\t\t[{ix}]: {}", format_magic(item.script_tag));
                match default_lang_sys {
                    None => (),
                    langsys @ Some(..) => {
                        print!("\t\t    [Default LangSys]: ");
                        show_langsys(langsys, conf);
                    }
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

fn show_langsys(lang_sys: &Link<LangSys>, conf: &Config) {
    let Some(LangSys {
        lookup_order_offset,
        required_feature_index,
        feature_indices,
    }) = lang_sys
    else {
        unreachable!("missing langsys");
    };
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
                let feature = feature.as_ref().unwrap_or_else(|| {
                    unreachable!("bad feature link (tag: {})", format_magic(*feature_tag));
                });
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
            match table {
                Some(table) => show_lookup_table(table, ctxt, conf),
                None => unreachable!("None lookup-table at index {ix}"),
            }
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
                    CursivePos { coverage, .. } => {
                        format!("entryExit({})", format_coverage_table(coverage))
                    }
                }
            };
            ("CursivePos", contents)
        }
        LookupSubtable::MarkBasePos(mb_pos) => {
            let contents = {
                match mb_pos {
                    MarkBasePos {
                        mark_coverage,
                        base_coverage,
                        mark_array,
                        base_array,
                    } => {
                        let mut mark_iter = mark_coverage.iter();
                        let mut base_iter = base_coverage.iter();
                        format!(
                            "Mark({})+Base({})=>MarkArray[{}]+BaseArray[{}]",
                            format_coverage_table(mark_coverage),
                            format_coverage_table(base_coverage),
                            format_mark_array(mark_array, &mut mark_iter),
                            format_base_array(base_array, &mut base_iter),
                        )
                    }
                }
            };
            ("MarkBasePos", contents)
        }
        LookupSubtable::MarkLigPos(ml_pos) => {
            let contents = {
                match ml_pos {
                    MarkLigPos {
                        mark_coverage,
                        ligature_coverage,
                        mark_array,
                        ligature_array,
                    } => {
                        let mut mark_iter = mark_coverage.iter();
                        let mut ligature_iter = ligature_coverage.iter();
                        format!(
                            "Mark({})+Ligature({})=>MarkArray[{}]+LigatureArray[{}]",
                            format_coverage_table(mark_coverage),
                            format_coverage_table(ligature_coverage),
                            format_mark_array(mark_array, &mut mark_iter),
                            format_ligature_array(ligature_array, &mut ligature_iter),
                        )
                    }
                }
            };
            ("MarkLigPos", contents)
        }
        LookupSubtable::MarkMarkPos(mm_pos) => {
            let contents = {
                match mm_pos {
                    MarkMarkPos {
                        mark1_coverage,
                        mark2_coverage,
                        mark1_array,
                        mark2_array,
                    } => {
                        let mut mark1_iter = mark1_coverage.iter();
                        let mut mark2_iter = mark2_coverage.iter();
                        format!(
                            "Mark({})+Mark({})=>MarkArray[{}]+Mark2Array[{}]",
                            format_coverage_table(mark1_coverage),
                            format_coverage_table(mark2_coverage),
                            format_mark_array(mark1_array, &mut mark1_iter),
                            format_mark2_array(mark2_array, &mut mark2_iter),
                        )
                    }
                }
            };
            ("MarkMarkPos", contents)
        }

        LookupSubtable::SingleSubst(single_subst) => {
            let contents = match single_subst {
                // STUB[placeholder]
                _ => format!("(..)"),
            };
            ("SingleSubst", contents)
        }
        LookupSubtable::MultipleSubst(multi_subst) => {
            let contents = {
                let MultipleSubst {
                    coverage,
                    subst: MultipleSubstFormat1 { sequences },
                } = &multi_subst;
                // REVIEW - is this the right balance of specificity and brevity?
                format!(
                    "{}=>SequenceTable[{}]",
                    format_coverage_table(coverage),
                    sequences.len()
                )
            };
            ("MultipleSubst", contents)
        }
        LookupSubtable::AlternateSubst(alt_subst) => {
            let contents = match alt_subst {
                AlternateSubst {
                    coverage,
                    alternate_sets,
                } => {
                    format!(
                        "{}=>{}",
                        format_coverage_table(coverage),
                        format_alternate_sets(alternate_sets)
                    )
                }
            };
            ("AlternateSubst", contents)
        }
        LookupSubtable::LigatureSubst(lig_subst) => {
            let contents = match lig_subst {
                LigatureSubst {
                    coverage,
                    ligature_sets,
                } => {
                    let mut iter = coverage.iter();
                    format_ligature_sets(ligature_sets, &mut iter)
                }
            };
            ("LigatureSubst", contents)
        }
        LookupSubtable::ReverseChainSingleSubst(rev_subst) => {
            let contents = match rev_subst {
                ReverseChainSingleSubst {
                    coverage,
                    backtrack_coverages,
                    lookahead_coverages,
                    substitute_glyph_ids,
                    ..
                } => {
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
                    let input_pattern = format_coverage_table(coverage);
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
                    let substitute_ids = format_glyphid_array_hex(substitute_glyph_ids, true);
                    format!(
                        "{backtrack_pattern}{input_pattern}{lookahead_pattern}=>{substitute_ids}"
                    )
                }
            };
            ("RevChainSingleSubst", contents)
        }
        LookupSubtable::SequenceContext(seq_ctx) => {
            let contents = match seq_ctx {
                SequenceContext::Format1(SequenceContextFormat1 { coverage, .. }) => {
                    format!("Glyphs({})", format_coverage_table(coverage))
                }
                SequenceContext::Format2(SequenceContextFormat2 { coverage, .. }) => {
                    format!("Classes({})", format_coverage_table(coverage))
                }
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
                    coverage, ..
                }) => {
                    // TODO - even if it means overly verbose output, this might be too little info to be useful compared to discriminant-only display
                    format!("ChainedGlyphs({})", format_coverage_table(coverage))
                }
                ChainedSequenceContext::Format2(ChainedSequenceContextFormat2 {
                    coverage, ..
                }) => {
                    // TODO - even if it means overly verbose output, this might be too little info to be useful compared to discriminant-only display
                    // REVIEW - consider what other details (e.g. class-def summary metrics) to show in implicitly- or explicitly-verbose display format
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

fn format_mark2_array(arr: &Mark2Array, coverage: &mut impl Iterator<Item = u16>) -> String {
    fn format_mark2_record(mark2_record: &Mark2Record, cov: u16) -> String {
        const CLASS_ANCHORS: usize = 2;
        format!(
            "{cov:04x}: {}",
            format_indexed_nullable(
                &mark2_record.mark2_anchors,
                |ix, anchor| format!("[{ix}]=>{}", format_anchor_table(anchor)),
                CLASS_ANCHORS,
                |n, (start, end)| format!("...(skipping {n} indices spanning {start}..={end})...",),
            )
        )
    }

    const MARK2_ARRAY_BOOKEND: usize = 2;
    format_items_inline(
        &arr.mark2_records,
        |mark2_record| {
            format_mark2_record(mark2_record, coverage.next().expect("missing coverage"))
        },
        MARK2_ARRAY_BOOKEND,
        |n| format!("...(skipping {n} Mark2Records)..."),
    )
}

fn format_indexed_nullable<T>(
    opt_items: &[Option<T>],
    mut show_fn: impl FnMut(usize, &T) -> String,
    bookend: usize,
    ellipsis: impl Fn(usize, (usize, usize)) -> String,
) -> String {
    let items: Vec<(usize, &T)> = opt_items
        .iter()
        .enumerate()
        .filter_map(|(ix, opt)| opt.as_ref().map(|v| (ix, v)))
        .collect();
    let mut buffer = Vec::<String>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

    let count = items.len();

    if count > bookend * 2 {
        for _ix in 0..bookend {
            let (ix, it) = items[_ix];
            buffer.push(show_fn(ix, it));
        }
        buffer.push(ellipsis(
            count - bookend * 2,
            (items[bookend].0, items[count - bookend - 1].0),
        ));
        for _ix in (count - bookend)..count {
            let (ix, it) = items[_ix];
            buffer.push(show_fn(ix, it));
        }
    } else {
        for (ix, it) in items.into_iter() {
            buffer.push(show_fn(ix, it));
        }
    }
    format!("[{}]", buffer.join(", "))
}

fn format_ligature_array(
    ligature_array: &LigatureArray,
    coverage: &mut impl Iterator<Item = u16>,
) -> String {
    fn format_ligature_attach(ligature_attach: &LigatureAttach, cov: u16) -> String {
        fn format_component_record(component_record: &ComponentRecord) -> String {
            const CLASS_ANCHOR_BOOKEND: usize = 2;
            format_indexed_nullable(
                &component_record.ligature_anchors,
                |ix, anchor| format!("[{ix}]=>{}", format_anchor_table(anchor)),
                CLASS_ANCHOR_BOOKEND,
                |n_skipped, (first, last)| {
                    format!("...(skipping {n_skipped} indices from {first} to {last})...")
                },
            )
        }

        const COMPONENTS_BOOKEND: usize = 1;
        format!(
            "{cov:04x}={}",
            format_items_inline(
                &ligature_attach.component_records,
                format_component_record,
                COMPONENTS_BOOKEND,
                |_| format!("..."),
            )
        )
    }

    const ATTACHES_INLINE: usize = 2;
    format_items_inline(
        &ligature_array.ligature_attach,
        |attach| format_ligature_attach(attach, coverage.next().expect("missing coverage")),
        ATTACHES_INLINE,
        |n_skipped| format!("...(skipping {n_skipped})..."),
    )
}

fn format_base_array(base_array: &BaseArray, coverage: &mut impl Iterator<Item = u16>) -> String {
    fn format_base_record(base_record: &BaseRecord, cov: u16) -> String {
        const CLASS_ANCHOR_BOOKEND: usize = 2;
        format!(
            "{cov:04x}: {}",
            format_indexed_nullable(
                &base_record.base_anchors,
                |ix, anchor| format!("[{ix}]=>{}", format_anchor_table(anchor)),
                CLASS_ANCHOR_BOOKEND,
                |n_skipped, (first, last)| format!(
                    "...(skipping {n_skipped} indices from {first} to {last})..."
                )
            )
        )
    }

    const BASE_ARRAY_BOOKEND: usize = 2;
    format_items_inline(
        &base_array.base_records,
        |base_record| format_base_record(base_record, coverage.next().expect("missing coverage")),
        BASE_ARRAY_BOOKEND,
        |n_skipped| format!("...({n_skipped} skipped)..."),
    )
}

fn format_mark_array(mark_array: &MarkArray, coverage: &mut impl Iterator<Item = u16>) -> String {
    fn format_mark_record(mark_record: &MarkRecord, cov: u16) -> String {
        format!(
            "{cov:04x}=({}, {})",
            mark_record.mark_class,
            format_anchor_table(mark_record.mark_anchor.as_ref().expect("broken link"))
        )
    }

    // FIXME[magic] - arbitrary local bookending const
    const MARK_ARRAY_BOOKEND: usize = 2;
    format_items_inline(
        &mark_array.mark_records,
        |mark_record| format_mark_record(mark_record, coverage.next().expect("missing coverage")),
        MARK_ARRAY_BOOKEND,
        |n_skipped| format!("...({n_skipped} skipped)..."),
    )
}

fn format_anchor_table(anchor: &AnchorTable) -> String {
    match anchor {
        AnchorTable::Format1(AnchorTableFormat1 {
            x_coordinate,
            y_coordinate,
        }) => {
            format!("({}, {})", as_s16(*x_coordinate), as_s16(*y_coordinate))
        }
        AnchorTable::Format2(f2) => {
            format!(
                "({}, {})@[{}]",
                as_s16(f2.x_coordinate),
                as_s16(f2.y_coordinate),
                f2.anchor_point
            )
        }
        AnchorTable::Format3(AnchorTableFormat3 {
            x_coordinate,
            y_coordinate,
            x_device,
            y_device,
        }) => {
            let extra = match (x_device, y_device) {
                (None, None) => unreachable!("unexpected both-Null DeviceOrVariationIndexTable-offsets in AnchorTable::Format3"),
                (Some(ref x), Some(ref y)) => {
                    format!("×({}, {})", format_device_or_variation_index_table(x), format_device_or_variation_index_table(y))
                }
                (Some(ref x), None) => {
                    format!("×({}, ⅈ)", format_device_or_variation_index_table(x))
                }
                (None, Some(ref y)) => {
                    format!("×(ⅈ, {})", format_device_or_variation_index_table(y))
                }
            };
            format!(
                "({}, {}){}",
                as_s16(*x_coordinate),
                as_s16(*y_coordinate),
                extra
            )
        }
    }
}

fn format_ligature_sets(
    lig_sets: &[LigatureSet],
    coverage: &mut impl Iterator<Item = u16>,
) -> String {
    fn format_ligature_set(lig_set: &LigatureSet, cov: u16) -> String {
        fn format_ligature(lig: &Ligature, cov: u16) -> String {
            // NOTE - bec
            format!(
                "(#{cov:04x}.{} => {})",
                format_glyphid_array_hex(&lig.component_glyph_ids, false),
                lig.ligature_glyph,
            )
        }
        // FIXME[magic] - arbitrary local bookending const
        const LIG_BOOKEND: usize = 2;
        format_items_inline(
            &lig_set.ligatures,
            |lig| match lig {
                Some(lig) => format_ligature(lig, cov),
                None => unreachable!("missing (None) ligature"),
            },
            LIG_BOOKEND,
            |n_skipped| format!("...({n_skipped} skipped)..."),
        )
    }
    match lig_sets {
        [ref set] => format_ligature_set(set, coverage.next().expect("missing coverage")),
        more => {
            const LIG_SET_BOOKEND: usize = 1;
            format_items_inline(
                more,
                |lig_set| format_ligature_set(lig_set, coverage.next().expect("missing coverage")),
                LIG_SET_BOOKEND,
                |_| String::from(".."),
            )
        }
    }
}

fn format_alternate_sets(alt_sets: &[AlternateSet]) -> String {
    fn format_alternate_set(alt_set: &AlternateSet) -> String {
        const ALT_GLYPH_BOOKEND: usize = 2;
        format_items_inline(
            &alt_set.alternate_glyph_ids,
            |glyph_id| format!("{}", glyph_id),
            ALT_GLYPH_BOOKEND,
            |_| "..".to_string(),
        )
    }
    match alt_sets {
        [ref set] => format_alternate_set(set),
        more => {
            const ALT_SET_BOOKEND: usize = 1;
            format_items_inline(more, format_alternate_set, ALT_SET_BOOKEND, |count| {
                format!("...({count} skipped)...")
            })
        }
    }
}

fn format_sequence_lookup(sl: &SequenceLookup) -> String {
    let s_ix = sl.sequence_index;
    let ll_ix = sl.lookup_list_index;
    // NOTE - the number in `\[_\]` is meant to mimic the index display of the show_items_elided formatting of LookupList, so it is the lookup index. The number after `@` is the positional index to apply the lookup to
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
        set_flags.push("IGNORE_LIGATURES");
    }
    if flags.ignore_marks {
        set_flags.push("IGNORE_MARKS");
    }
    if flags.use_mark_filtering_set {
        set_flags.push("USE_MARK_FILTERING_SET");
    }

    let str_flags = if set_flags.is_empty() {
        String::from("∅")
    } else {
        set_flags.join(" | ")
    };

    let str_macf = match flags.mark_attachment_class_filter {
        // NOTE - If we are not filtering by mark attachment class, we don't need to print anything for that field
        0 => String::new(),
        // REVIEW - if horizontal space is at a premium, we may want to shorten or partially elide the label-string
        n => format!("; mark_attachment_class_filter = {n}"),
    };

    format!("LookupFlag ({str_flags}{str_macf})")
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
    match ivs {
        _ => println!("\tItemVariationStore: <unimplemented>"),
    }
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

fn format_caret_value(cv: &Link<CaretValue>) -> String {
    match cv {
        None => unreachable!("caret value null link"),
        Some(cv) => match cv {
            // REVIEW - this isn't really a canonical abbreviation, so we might adjust what we show for Design Units (Format 1)
            CaretValue::DesignUnits(du) => format!("{du}du"),
            CaretValue::ContourPoint(ix) => format!("#{ix}"),
            CaretValue::DesignUnitsWithTable { coordinate, device } => match device {
                None => unreachable!("dev-table in caret value format 3 with null offset"),
                Some(table) => {
                    format!(
                        "{}du+{}",
                        coordinate,
                        format_device_or_variation_index_table(table)
                    )
                }
            },
        },
    }
}

fn format_device_or_variation_index_table(table: &DeviceOrVariationIndexTable) -> String {
    match table {
        DeviceOrVariationIndexTable::DeviceTable(dev_table) => format_device_table(dev_table),
        DeviceOrVariationIndexTable::VariationIndexTable(var_ix_table) => {
            format_variation_index_table(var_ix_table)
        }
        DeviceOrVariationIndexTable::OtherTable { delta_format } => {
            format!("[<DeltaFormat {delta_format}>]")
        }
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

/// Compact inline display of an array representing a sequence (rather than a set) of glyphIds
// REVIEW - we have no cap on how long a glyphId sequence we are willing to show unabridged and we might want one in theory
fn format_glyphid_array_hex(glyphs: &impl AsRef<[u16]>, is_standalone: bool) -> String {
    let glyph_array = glyphs.as_ref();

    const BYTES_PER_GLYPH: usize = 2;

    // Display prefix and associated overhead in bytes
    const PREFIX: &str = "#";
    const BYTE_OVERHEAD_PREFIX: usize = PREFIX.len();

    // how many extra String-bytes we use per glyph, not counting the glyph itself
    const GLUE: &str = ".";
    const BYTE_OVERHEAD_PER_GLYPH: usize = GLUE.len();

    // If the number of GLUE-strings we need is less than N (viz. the number of glyphs), this is the difference between N and the actual number we use
    const PER_GLYPH_OVERCOUNT: usize = 1;

    if glyph_array.is_empty() {
        // Short-circuit for empty-glyph array
        return String::from("ε");
    }
    let nglyphs = glyph_array.len();

    // We would use saturating-sub instead of raw `-` on nglyphs and PER_GLYPH_OVERCOUNT but nglyphs is not zero if we are here.
    let nbytes = (if is_standalone {
        BYTE_OVERHEAD_PREFIX
    } else {
        0
    }) + (nglyphs * BYTES_PER_GLYPH)
        + (BYTE_OVERHEAD_PER_GLYPH * (nglyphs - PER_GLYPH_OVERCOUNT));

    // Initialize a buffer with enough capacity it ought not need to reallocate or grow
    let mut buffer = String::with_capacity(nbytes);

    // Fill the buffer
    if is_standalone {
        buffer.push_str(PREFIX);
    }

    for (ix, glyph) in glyph_array.iter().enumerate() {
        if ix > 0 {
            buffer.push_str(GLUE);
        }
        // REVIEW - do we want to eliminate zero-padding for compactness, or keep it for consistency/legibility?
        buffer.push_str(&format!("{:04x}", glyph));
    }
    buffer
}

// FIXME - we might want a more flexible model where the `show_XYZZY`/`format_XYZZY` dichotomy is erased, such as a generic Writer or Fragment-producer...
fn format_coverage_table(cov: &CoverageTable) -> String {
    match cov {
        CoverageTable::Format1 { ref glyph_array } => {
            let num_glyphs = glyph_array.len();
            match glyph_array.as_slice() {
                &[] => format!("∅"),
                &[id] => format!("[{id}]"),
                &[first, .., last] => format!("[{num_glyphs} ∈ [{first},{last}]]"),
            }
        }
        CoverageTable::Format2 { ref range_records } => match range_records.as_slice() {
            &[] => format!("∅"),
            &[rr] => format!("[∀: {}..={}]", rr.start_glyph_id, rr.end_glyph_id),
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
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
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
}

// REVIEW - this construction suggests we may really want a Write-generic or Fragment-like output model to avoid duplication between I/O show and String formatting functions
fn show_items_inline<T>(
    items: &[T],
    show_fn: impl FnMut(&T) -> String,
    bookend: usize,
    ellipsis: impl Fn(usize) -> String,
) {
    let oput = format_items_inline(items, show_fn, bookend, ellipsis);
    println!("{oput}");
}

fn format_items_inline<T>(
    items: &[T],
    mut show_fn: impl FnMut(&T) -> String,
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

fn show_cmap_metrics(cmap: &Cmap, conf: &Config) {
    print!("cmap: version {}", cmap.version);
    if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
        println!();
        let show_record = |ix: usize, record: &EncodingRecord| {
            // TODO[enrichment]: if we implement subtables and more verbosity levels, show subtable details
            let EncodingRecord {
                platform,
                encoding,
                subtable: _subtable,
            } = record;
            println!("\t[{ix}]: platform={}, encoding={}", platform, encoding);
        };
        show_items_elided(
            &cmap.encoding_records,
            show_record,
            conf.bookend_size,
            |start, stop| format!("\t(skipping encoding records {start}..{stop})"),
        )
    } else {
        println!(", {} encoding tables", cmap.encoding_records.len());
    }
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

fn show_hmtx_metrics(hmtx: &HmtxMetrics, conf: &Config) {
    let show_unified = |ix: usize, hmet: &UnifiedHmtxMetric| match &hmet.advance_width {
        Some(width) => println!(
            "\tGlyph ID [{ix}]: advanceWidth={width}, lsb={}",
            hmet.left_side_bearing
        ),
        None => println!("\tGlyph ID [{ix}]: lsb={}", hmet.left_side_bearing),
    };
    if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
        show_items_elided(&hmtx.0, show_unified, conf.bookend_size, |start, stop| {
            format!("    (skipping hmetrics {start}..{stop})")
        });
    } else {
        println!("hmtx: {} hmetrics", hmtx.0.len())
    }
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

fn show_name_metrics(name: &NameMetrics, conf: &Config) {
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
    if conf.verbosity.is_at_least(VerboseLevel::Baseline) {
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
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            show_items_elided(
                glyf.glyphs.as_slice(),
                show_glyph_metric,
                conf.bookend_size,
                |start, stop| format!("    (skipping glyphs {start}..{stop})"),
            )
        }
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

pub mod lookup_subtable {
    use super::{
        OpentypeGposLookupSubtable, OpentypeGposLookupSubtableExt, OpentypeGsubLookupSubtable,
        OpentypeGsubLookupSubtableExt, Parser, TestResult, UnknownValueError,
    };
    use crate::{opentype_main_directory, opentype_ttc_header_header, Decoder_opentype_main};

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    struct Both {
        gsub: bool,
        gpos: bool,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct LookupSet {
        single_pos: bool,
        pair_pos: bool,
        cursive_pos: bool,
        mark_base_pos: bool,
        mark_lig_pos: bool,
        mark_mark_pos: bool,
        pos_extension: bool,

        sequence_context: Both,
        chained_sequence_context: Both,

        single_subst: bool,
        multiple_subst: bool,
        alternate_subst: bool,
        ligature_subst: bool,
        subst_extension: bool,
        reverse_chain_single_subst: bool,
    }

    pub enum SingleOrMulti<T> {
        Single(T),
        Multi(Vec<Option<T>>),
    }

    pub fn analyze_font_lookups(test_file: &str) -> TestResult<SingleOrMulti<LookupSet>> {
        let buffer = std::fs::read(std::path::Path::new(test_file))?;
        let mut input = Parser::new(&buffer);
        let font = Decoder_opentype_main(&mut input)?;
        match font.directory {
            opentype_main_directory::TTCHeader(multi) => {
                let ret = match multi.header {
                    opentype_ttc_header_header::UnknownVersion(n) => {
                        return Err(Box::new(UnknownValueError {
                            what: format!("ttc header version"),
                            bad_value: n,
                        }));
                    }
                    opentype_ttc_header_header::Version1(v1header) => {
                        let mut lookup_metrics =
                            Vec::with_capacity(v1header.table_directories.len());
                        for font in v1header.table_directories.iter() {
                            let tmp = match &font.link {
                                Some(dir) => Some(analyze_table_directory_lookups(dir)),
                                None => None,
                            };
                            lookup_metrics.push(tmp);
                        }
                        lookup_metrics
                    }
                    opentype_ttc_header_header::Version2(v2header) => {
                        let mut lookup_metrics =
                            Vec::with_capacity(v2header.table_directories.len());
                        for font in v2header.table_directories.iter() {
                            let tmp = match &font.link {
                                Some(dir) => Some(analyze_table_directory_lookups(dir)),
                                None => None,
                            };
                            lookup_metrics.push(tmp);
                        }
                        lookup_metrics
                    }
                };
                Ok(SingleOrMulti::Multi(ret))
            }
            opentype_main_directory::TableDirectory(single) => Ok(SingleOrMulti::Single(
                analyze_table_directory_lookups(&single),
            )),
        }
    }

    fn analyze_table_directory_lookups(dir: &super::opentype_table_directory) -> LookupSet {
        let mut ret = LookupSet::default();
        if let Some(lookup_list) = dir
            .table_links
            .gpos
            .as_ref()
            .and_then(|gpos| gpos.lookup_list.link.as_ref())
        {
            for entry in lookup_list.lookups.iter() {
                if let Some(subtable) = entry.link.as_ref().and_then(|lookup| {
                    lookup
                        .subtables
                        .first()
                        .and_then(|subtable| subtable.link.as_ref())
                }) {
                    let ground = match subtable {
                        OpentypeGposLookupSubtableExt::PosExtension(ext) => {
                            ret.pos_extension = true;
                            match &ext.extension_offset.link {
                                None => unreachable!("missing link"),
                                Some(ground) => ground,
                            }
                        }
                        OpentypeGposLookupSubtableExt::GroundPos(ground) => ground,
                    };
                    match ground {
                        OpentypeGposLookupSubtable::SinglePos(..) => ret.single_pos = true,
                        OpentypeGposLookupSubtable::PairPos(..) => ret.pair_pos = true,
                        OpentypeGposLookupSubtable::CursivePos(..) => ret.cursive_pos = true,
                        OpentypeGposLookupSubtable::MarkBasePos(..) => ret.mark_base_pos = true,
                        OpentypeGposLookupSubtable::MarkLigPos(..) => ret.mark_lig_pos = true,
                        OpentypeGposLookupSubtable::MarkMarkPos(..) => ret.mark_mark_pos = true,
                        OpentypeGposLookupSubtable::SequenceContext(..) => {
                            ret.sequence_context.gpos = true
                        }
                        OpentypeGposLookupSubtable::ChainedSequenceContext(..) => {
                            ret.chained_sequence_context.gpos = true
                        }
                    }
                }
            }
        }
        if let Some(lookup_list) = dir
            .table_links
            .gsub
            .as_ref()
            .and_then(|gsub| gsub.lookup_list.link.as_ref())
        {
            for entry in lookup_list.lookups.iter() {
                if let Some(subtable) = entry.link.as_ref().and_then(|lookup| {
                    lookup
                        .subtables
                        .first()
                        .and_then(|subtable| subtable.link.as_ref())
                }) {
                    let ground = match subtable {
                        OpentypeGsubLookupSubtableExt::SubstExtension(ext) => {
                            ret.subst_extension = true;
                            match &ext.extension_offset.link {
                                None => unreachable!("missing link"),
                                Some(ground) => ground,
                            }
                        }
                        OpentypeGsubLookupSubtableExt::GroundSubst(ground) => ground,
                    };
                    match ground {
                        OpentypeGsubLookupSubtable::SingleSubst(..) => ret.single_subst = true,
                        OpentypeGsubLookupSubtable::MultipleSubst(..) => ret.multiple_subst = true,
                        OpentypeGsubLookupSubtable::AlternateSubst(..) => {
                            ret.alternate_subst = true
                        }
                        OpentypeGsubLookupSubtable::LigatureSubst(..) => ret.ligature_subst = true,
                        OpentypeGsubLookupSubtable::ReverseChainSingleSubst(..) => {
                            ret.reverse_chain_single_subst = true
                        }
                        OpentypeGsubLookupSubtable::SequenceContext(..) => {
                            ret.sequence_context.gsub = true
                        }
                        OpentypeGsubLookupSubtable::ChainedSequenceContext(..) => {
                            ret.chained_sequence_context.gsub = true
                        }
                    }
                }
            }
        }
        ret
    }

    pub fn collate_lookups_table<S: std::fmt::Display>(samples: &[(S, SingleOrMulti<LookupSet>)]) {
        let header = [
            "Pos1", "Pos2", "Pos3", "Pos4", "Pos5", "Pos6", "Pos7", "Pos8", "Pos9", "Sub1", "Sub2",
            "Sub3", "Sub4", "Sub5", "Sub6", "Sub7", "Sub8", "Location",
        ];
        let header_line = header.join("\t");

        fn write_lookups(buf: &mut String, lookups: LookupSet) {
            let show_bool = |buf: &mut String, value: bool| {
                if value {
                    buf.push_str("✅\t")
                } else {
                    buf.push_str("❌\t")
                }
            };

            show_bool(buf, lookups.single_pos);
            show_bool(buf, lookups.pair_pos);
            show_bool(buf, lookups.cursive_pos);
            show_bool(buf, lookups.mark_base_pos);
            show_bool(buf, lookups.mark_lig_pos);
            show_bool(buf, lookups.mark_mark_pos);
            show_bool(buf, lookups.pos_extension);
            show_bool(buf, lookups.sequence_context.gpos);
            show_bool(buf, lookups.chained_sequence_context.gpos);
            show_bool(buf, lookups.single_subst);
            show_bool(buf, lookups.multiple_subst);
            show_bool(buf, lookups.alternate_subst);
            show_bool(buf, lookups.ligature_subst);
            show_bool(buf, lookups.subst_extension);
            show_bool(buf, lookups.reverse_chain_single_subst);
            show_bool(buf, lookups.sequence_context.gsub);
            show_bool(buf, lookups.chained_sequence_context.gsub);
        }

        println!("{header_line}");
        for (sample, lookups) in samples.iter() {
            match lookups {
                SingleOrMulti::Single(lookups) => {
                    let mut line = String::new();
                    write_lookups(&mut line, *lookups);
                    println!("{line}{sample}")
                }
                SingleOrMulti::Multi(font_lookups) => {
                    for (ix, opt_lookups) in font_lookups.iter().enumerate() {
                        if let Some(lookups) = opt_lookups {
                            let mut line = String::new();
                            write_lookups(&mut line, *lookups);
                            println!("{line}{sample}[{ix}]")
                        }
                    }
                }
            }
        }
    }
}
