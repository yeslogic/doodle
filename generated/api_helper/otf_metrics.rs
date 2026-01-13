use super::util::{EnumLen, U16Set, Wec, trisect_unchecked};
use super::*;
use derive_builder::Builder;
use doodle::Label;
use encoding::{
    DecoderTrap, Encoding,
    all::{MAC_ROMAN, UTF_16BE},
};
use fixed::types::{I2F14, I16F16};

/// Half-Tab for partial indentation
const HT: &str = "    ";

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

pub type OpentypeTag = u32;

impl From<u32> for Tag {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Tag> for u32 {
    fn from(value: Tag) -> Self {
        value.0
    }
}

impl Promote<OpentypeTag> for Tag {
    fn promote(orig: &OpentypeTag) -> Self {
        Self(*orig)
    }
}

// REVIEW - no module-level definition so the names are the semi-arbitrary 'first' one the code-generator sees
pub type OpentypeFixed = opentype_head_table_font_revision;
pub type Fixed = I16F16;

impl Promote<OpentypeFixed> for Fixed {
    fn promote(orig: &OpentypeFixed) -> Self {
        match orig {
            OpentypeFixed::Fixed32(raw) => I16F16::from_bits(*raw as i32),
        }
    }
}

pub type OpentypeF2Dot14 = opentype_gvar_tuple_record_coordinates;
pub type F2Dot14 = I2F14;

impl Promote<OpentypeF2Dot14> for F2Dot14 {
    fn promote(orig: &OpentypeF2Dot14) -> Self {
        match orig {
            OpentypeF2Dot14::F2Dot14(raw) => I2F14::from_bits(*raw as i16),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
struct Tag(pub u32);

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.0.to_be_bytes();
        let mut write = move |b: u8| {
            if !b.is_ascii_control() {
                use std::fmt::Write;
                f.write_char(b as char)
            } else {
                write!(f, "\\x{b:02x}")
            }
        };
        for byte in bytes {
            write(byte)?;
        }
        Ok(())
    }
}

// SECTION - Type aliases for stable referencing of commonly-used generated types
pub type OpentypeFontDirectory<'input> = opentype_table_directory<'input>;
pub type OpentypeGlyf = opentype_glyf_table;
pub type GlyphDescription = opentype_glyf_description;
pub type SimpleGlyph = opentype_glyf_simple;

pub type OpentypeCmap<'a> = opentype_cmap_table<'a>;
pub type OpentypeHead = opentype_head_table;
pub type OpentypeHhea = opentype_hhea_table;

pub type OpentypeHmtx = opentype_hmtx_table;
pub type OpentypeHmtxLongMetric = opentype_hmtx_table_long_metrics;

pub type OpentypeMaxp = opentype_maxp_table;
pub type OpentypeName<'a> = opentype_name_table<'a>;
pub type OpentypeOs2 = opentype_os2_table;
pub type OpentypePost = opentype_post_table;

pub type OpentypeBase<'a> = opentype_base_table<'a>;
pub type OpentypeGdef<'a> = opentype_gdef_table<'a>;

pub type OpentypeGdefTableData<'a> = opentype_gdef_table_data<'a>;

pub type OpentypeAttachPoint = opentype_gdef_attach_point;
pub type OpentypeCoverageTable = opentype_coverage_table;
pub type OpentypeCoverageTableData = opentype_coverage_table_data;
pub type OpentypeCoverageRangeRecord = opentype_coverage_table_data_Format2_range_records;

pub type OpentypeGpos<'input> = opentype_gpos_table<'input>;
pub type OpentypeGsub<'input> = opentype_gsub_table<'input>;

pub type OpentypeKern<'a> = opentype_kern_table<'a>;
pub type OpentypeStat<'a> = opentype_stat_table<'a>;
pub type OpentypeFvar<'input> = opentype_fvar_table<'input>;
pub type OpentypeGvar<'input> = opentype_gvar_table<'input>;

// STUB[epic=horizontal-for-vertical] - change to distinguished type names once we have them
pub type OpentypeVhea = opentype_hhea_table;
pub type OpentypeVmtx = opentype_hmtx_table;
pub type OpentypeVmtxLongMetric = opentype_hmtx_table_long_metrics;
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

#[macro_use]
/// Submodule for boilerplate around objects holding views and offsets, and the nominal objects those offsets point to.
pub mod container {
    use super::{PResult, Parser, View};

    /// Boilerplate macro for implementing `ViewFrame` for a type holding a `View`.
    ///
    /// By default, assumes the `View` is kept in a field named `table_scope`, in which case only the type-name is needed.
    /// Otherwise, `frame!( <TYPENAME> . <field_ident> )` should be passed in.
    macro_rules! frame {
        ($ty:ident) => {
            impl<'input> $crate::api_helper::otf_metrics::container::ViewFrame<'input>
                for $ty<'input>
            {
                fn scope(&self) -> View<'input> {
                    self.table_scope
                }
            }
        };
        ($ty:ident . $field:ident) => {
            impl<'input> $crate::api_helper::otf_metrics::container::ViewFrame<'input>
                for $ty<'input>
            {
                fn scope(&self) -> View<'input> {
                    self.$field
                }
            }
        };
    }
    /// Trait marking a type as holding a `View` that can be used for offset-parsing its direct fields (or their subtrees).
    pub trait ViewFrame<'input> {
        /// Returns the `View`-object directly held by `Self`.
        fn scope(&self) -> View<'input>;
    }

    /// Trait implemented over marker-type proxies that implement the most natural parse for their
    pub trait CommonObject {
        type Args<'a>: Sized;
        type Output<'a>: Sized;

        fn parse<'input>(
            p: &mut Parser<'input>,
            args: Self::Args<'input>,
        ) -> PResult<Self::Output<'input>>;

        fn parse_offset<'input>(
            view: View<'input>,
            offset: usize,
            args: Self::Args<'input>,
        ) -> PResult<Self::Output<'input>> {
            let mut p = Parser::from(view.offset(offset)?);
            Self::parse(&mut p, args)
        }

        fn parse_nullable_offset<'input>(
            view: View<'input>,
            offset: usize,
            args: Self::Args<'input>,
        ) -> PResult<Option<Self::Output<'input>>> {
            if offset == 0 {
                Ok(None)
            } else {
                Ok(Some(Self::parse_offset(view, offset, args)?))
            }
        }
    }

    pub trait SingleContainer<Obj>
    where
        Obj: CommonObject,
    {
        fn get_offset(&self) -> usize;

        fn get_args(&self) -> Obj::Args<'_>;
    }

    impl<Con, Obj> MultiContainer<Obj, 1> for Con
    where
        Con: SingleContainer<Obj>,
        Obj: CommonObject,
    {
        fn get_offset_array(&self) -> [usize; 1] {
            [self.get_offset()]
        }

        fn get_args_array(&self) -> [Obj::Args<'_>; 1] {
            [self.get_args()]
        }
    }

    pub trait OptContainer<Obj>
    where
        // REVIEW - consider relaxing this constraint in future
        Obj: for<'a> CommonObject<Args<'a> = ()>,
    {
        fn contains_object(&self) -> bool;

        fn get_offset(&self) -> Option<usize>;
    }

    pub trait MultiContainer<Obj, const N: usize>
    where
        Obj: CommonObject,
    {
        fn get_offset_array(&self) -> [usize; N];

        fn get_args_array(&self) -> [Obj::Args<'_>; N];
    }

    pub trait DynContainer<Obj>
    where
        Obj: CommonObject,
    {
        fn count(&self) -> usize;

        fn iter_offsets(&self) -> impl Iterator<Item = usize>;

        fn iter_args(&self) -> impl Iterator<Item = Obj::Args<'_>>;
    }

    pub trait MultiDynContainer<Obj, const N: usize>
    where
        Obj: CommonObject,
    {
        fn counts(&self) -> [usize; N];

        fn iter_offsets_at_index(&self, ix: usize) -> impl Iterator<Item = usize>;

        fn iter_args_at_index(&self, ix: usize) -> impl Iterator<Item = Obj::Args<'_>>;
    }

    pub trait MultiOptContainer<Obj, const N: usize>
    where
        // REVIEW - consider relaxing this constraint in future
        // TODO: if this constraint is removed, add a method to query arguments
        Obj: for<'a> CommonObject<Args<'a> = ()>,
    {
        fn has_index(&self, ix: usize) -> bool {
            self.get_offset_at_index(ix).is_some()
        }

        fn get_offset_at_index(&self, ix: usize) -> Option<usize>;
    }
}

pub fn reify<'input, Frame, Obj>(frame: &'input Frame, _proxy: Obj) -> Obj::Output<'input>
where
    Frame: container::ViewFrame<'input> + container::SingleContainer<Obj>,
    Obj: container::CommonObject,
{
    let args = frame.get_args();
    let offset = frame.get_offset();
    Obj::parse_offset(frame.scope(), offset, args).expect(&format!(
        "failed to parse (reify::<{}, {}>)",
        std::any::type_name::<Frame>(),
        std::any::type_name::<Obj>()
    ))
}

pub fn reify_index<'input, Frame, Obj, const N: usize>(
    frame: &'input Frame,
    _proxy: Obj,
    ix: usize,
) -> Obj::Output<'input>
where
    Frame: container::ViewFrame<'input> + container::MultiContainer<Obj, N>,
    Obj: container::CommonObject<Args<'input>: Clone>,
{
    let tmp = frame.get_args_array();
    let offset = frame.get_offset_array()[ix];
    Obj::parse_offset(frame.scope(), offset, tmp[ix].clone()).expect(&format!(
        "failed to parse (reify_index::<{}, {}>(.., {ix})",
        std::any::type_name::<Frame>(),
        std::any::type_name::<Obj>()
    ))
}

pub fn reify_all_index<'input, Frame, Obj, const N: usize>(
    frame: &'input Frame,
    _proxy: Obj,
    ix: usize,
) -> impl Iterator<Item = Obj::Output<'input>> + 'input
where
    Frame: container::ViewFrame<'input> + container::MultiDynContainer<Obj, N>,
    Obj: container::CommonObject + 'static,
{
    assert!(ix < N, "index out of bounds");
    let offset_iter = frame.iter_offsets_at_index(ix);
    let args_iter = frame.iter_args_at_index(ix);
    Iterator::zip(offset_iter, args_iter).map(move |(offset, args)| {
        // REVIEW - should individual failure-to-parse cause the entire iteration to panic?
        Obj::parse_offset(frame.scope(), offset, args).expect(&format!(
            "failed to parse (reify_all_index::<{}, {}>(.., {ix})",
            std::any::type_name::<Frame>(),
            std::any::type_name::<Obj>()
        ))
    })
}

pub fn reify_all<'input, Frame, Obj>(
    frame: &'input Frame,
    _proxy: Obj,
) -> impl Iterator<Item = Obj::Output<'input>> + 'input
where
    Frame: container::ViewFrame<'input> + container::DynContainer<Obj>,
    Obj: container::CommonObject<Args<'input>: Clone> + 'static,
{
    // REVIEW - should individual failure-to-parse cause the entire iteration to panic?
    Iterator::zip(frame.iter_offsets(), frame.iter_args()).map(move |(offset, args)| {
        Obj::parse_offset(frame.scope(), offset, args).expect(&format!(
            "failed to parse (reify_all::<{}, {}>)",
            std::any::type_name::<Frame>(),
            std::any::type_name::<Obj>()
        ))
    })
}

pub fn reify_opt<'input, Frame, Obj>(
    frame: &'input Frame,
    _proxy: Obj,
) -> Option<Obj::Output<'input>>
where
    Frame: container::ViewFrame<'input> + container::OptContainer<Obj>,
    Obj: 'static + for<'a> container::CommonObject<Args<'a> = ()>,
{
    frame.get_offset().map(|offset| {
        Obj::parse_offset(frame.scope(), offset, ()).expect(&format!(
            "failed to parse (reify_opt::<{}, {}>)",
            std::any::type_name::<Frame>(),
            std::any::type_name::<Obj>()
        ))
    })
}

pub fn reify_opt_index<'input, Frame, Obj, const N: usize>(
    frame: &'input Frame,
    _proxy: Obj,
    ix: usize,
) -> Option<Obj::Output<'input>>
where
    Frame: container::ViewFrame<'input> + container::MultiOptContainer<Obj, N>,
    // TODO - if contraint on Args is lifted on trait itself, remove this
    Obj: 'static + for<'a> container::CommonObject<Args<'a> = ()>,
{
    frame.get_offset_at_index(ix).map(|offset| {
        Obj::parse_offset(frame.scope(), offset, ()).expect(&format!(
            "failed to parse (reify_opt_index::<{}, {}>(.., {ix}))",
            std::any::type_name::<Frame>(),
            std::any::type_name::<Obj>()
        ))
    })
}

pub fn reify_dep<'input, Con, Obj>(
    view: View<'input>,
    container: &'input Con,
    _proxy: Obj,
) -> PResult<Obj::Output<'input>>
where
    Con: container::SingleContainer<Obj>,
    Obj: container::CommonObject,
{
    let args = container.get_args();
    let offset = container.get_offset();
    Obj::parse_offset(view, offset, args)
}

pub fn reify_index_dep<'input, Con, Obj, const N: usize>(
    view: View<'input>,
    container: &'input Con,
    _proxy: Obj,
    ix: usize,
) -> PResult<Obj::Output<'input>>
where
    Con: container::MultiContainer<Obj, N>,
    Obj: container::CommonObject<Args<'input>: Clone>,
{
    let tmp = container.get_args_array();
    let offset = container.get_offset_array()[ix];
    Obj::parse_offset(view, offset, tmp[ix].clone())
}

pub fn reify_all_dep<'input, Con, Obj>(
    view: View<'input>,
    container: &'input Con,
    _proxy: Obj,
) -> impl Iterator<Item = PResult<Obj::Output<'input>>> + 'input
where
    Con: container::DynContainer<Obj>,
    Obj: container::CommonObject<Args<'input>: Clone> + 'static,
{
    Iterator::zip(container.iter_offsets(), container.iter_args())
        .map(move |(offset, args)| Obj::parse_offset(view, offset, args))
}

pub fn reify_all_index_dep<'input, Con, Obj, const N: usize>(
    view: View<'input>,
    container: &'input Con,
    _proxy: Obj,
    ix: usize,
) -> impl Iterator<Item = PResult<Obj::Output<'input>>> + 'input
where
    Con: container::MultiDynContainer<Obj, N>,
    Obj: container::CommonObject + 'static,
{
    assert!(ix < N, "index out of bounds");
    let offset_iter = container.iter_offsets_at_index(ix);
    let args_iter = container.iter_args_at_index(ix);
    Iterator::zip(offset_iter, args_iter)
        .map(move |(offset, args)| Obj::parse_offset(view, offset, args))
}

pub fn reify_opt_index_dep<'input, Con, Obj, const N: usize>(
    view: View<'input>,
    container: &'input Con,
    _proxy: Obj,
    ix: usize,
) -> Option<PResult<Obj::Output<'input>>>
where
    Con: container::MultiOptContainer<Obj, N>,
    // TODO - if contraint on Args is lifted on trait itself, remove this
    Obj: 'static + for<'a> container::CommonObject<Args<'a> = ()>,
{
    container
        .get_offset_at_index(ix)
        .map(move |offset| Obj::parse_offset(view, offset, ()))
}
pub mod obj {
    use super::container::CommonObject;
    use super::{PResult, Parser, View};

    macro_rules! proxy {
        ($real:ident => $proxy:ident) => {
            use super::$real;
            pub struct $proxy;
        };
        ($real:ident =>  $proxy:ident @ $doc:meta ) => {
            use super::$real;
            #[$doc]
            pub struct $proxy;
        };
    }

    proxy!(OpentypeLigCaretList => LigCarList);

    impl CommonObject for LigCarList {
        type Args<'a> = ();
        type Output<'a> = OpentypeLigCaretList<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gdef_lig_caret_list(p)
        }
    }

    proxy!(OpentypeAttachList => AttList);

    impl CommonObject for AttList {
        type Args<'a> = ();
        type Output<'a> = OpentypeAttachList<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gdef_attach_list(p)
        }
    }

    proxy!(OpentypeKerningArray => KernArr);

    impl CommonObject for KernArr {
        /// Args: `(left_glyph_count, right_glyph_count)`
        type Args<'a> = (u16, u16);
        type Output<'a> = OpentypeKerningArray;

        fn parse<'input>(
            p: &mut Parser<'input>,
            (left_glyph_count, right_glyph_count): (u16, u16),
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_kern_kerning_array(p, left_glyph_count, right_glyph_count)
        }
    }

    proxy!(OpentypeKernClassTable => KernCls);

    impl CommonObject for KernCls {
        type Args<'a> = ();
        type Output<'a> = OpentypeKernClassTable;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_kern_class_table(p)
        }
    }

    proxy!(OpentypeRuleSet => SeqRuleSet);

    impl CommonObject for SeqRuleSet {
        type Args<'a> = ();
        type Output<'a> = OpentypeRuleSet<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_sequence_context_rule_set(p)
        }
    }

    proxy!(OpentypeRule => SeqRule);

    impl CommonObject for SeqRule {
        type Args<'a> = ();
        type Output<'a> = OpentypeRule;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_sequence_context_rule(p)
        }
    }

    proxy!(OpentypeChainedRuleSet => ChainRuleSet);

    impl CommonObject for ChainRuleSet {
        type Args<'a> = ();
        type Output<'a> = OpentypeChainedRuleSet<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            _: Self::Args<'input>,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_chained_sequence_rule_set(p)
        }
    }

    proxy!(OpentypeChainedRule => ChainRule);

    impl CommonObject for ChainRule {
        type Args<'a> = ();
        type Output<'a> = OpentypeChainedRule;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_chained_sequence_rule(p)
        }
    }

    proxy!(OpentypeAlternateSet => AltSet);

    impl CommonObject for AltSet {
        type Args<'a> = ();
        type Output<'a> = OpentypeAlternateSet;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gsub_alternate_subst_alternate_set(p)
        }
    }

    proxy!(OpentypeLigature => LigTable);

    impl CommonObject for LigTable {
        type Args<'a> = ();
        type Output<'a> = OpentypeLigature;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gsub_ligature_subst_ligature_table(p)
        }
    }

    proxy!(OpentypeLigatureSet => LigSet);

    impl CommonObject for LigSet {
        type Args<'a> = ();
        type Output<'a> = OpentypeLigatureSet<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gsub_ligature_subst_ligature_set(p)
        }
    }

    proxy!(OpentypeLigatureAttach => LigAtt);

    impl CommonObject for LigAtt {
        ///  Args: `mark_class_count`
        type Args<'a> = u16;
        type Output<'a> = OpentypeLigatureAttach<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            mark_class_count: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_ligature_attach(p, mark_class_count)
        }
    }

    proxy!(OpentypeMark2Array => Mark2Arr);

    impl CommonObject for Mark2Arr {
        /// Args: `mark_class_count`
        type Args<'a> = u16;
        type Output<'a> = OpentypeMark2Array<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            mark_class_count: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_mark2_array(p, mark_class_count)
        }
    }

    proxy!(OpentypeCaretValue => CaretVal);

    impl CommonObject for CaretVal {
        type Args<'a> = ();
        type Output<'a> = OpentypeCaretValue<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gdef_caret_value(p)
        }
    }

    proxy!(OpentypeLigGlyph => LigGlyph);

    impl CommonObject for LigGlyph {
        type Args<'a> = ();
        type Output<'a> = OpentypeLigGlyph<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gdef_lig_glyph(p)
        }
    }

    proxy!(OpentypeAttachPoint => AttPoint);

    impl CommonObject for AttPoint {
        type Args<'a> = ();
        type Output<'a> = OpentypeAttachPoint;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gdef_attach_point(p)
        }
    }

    proxy!(OpentypeItemVariationStore => ItemVarStore);

    impl CommonObject for ItemVarStore {
        type Args<'a> = ();
        type Output<'a> = OpentypeItemVariationStore<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _args: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_common_item_variation_store(p)
        }
    }

    proxy!(OpentypeMarkGlyphSet => MarkGlSet);

    impl CommonObject for MarkGlSet {
        type Args<'a> = ();
        type Output<'a> = OpentypeMarkGlyphSet<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gdef_mark_glyph_set(p)
        }
    }

    proxy!(OpentypeAxisValue => AxisValTbl);

    impl CommonObject for AxisValTbl {
        type Args<'a> = ();
        type Output<'a> = OpentypeAxisValue;

        fn parse<'input>(p: &mut Parser<'input>, _args: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_stat_axis_value_table(p)
        }
    }

    proxy!(OpentypeAxisValueArray => AxisValueArr);

    impl CommonObject for AxisValueArr {
        /// Args: `axis_value_count`
        type Args<'a> = u16;
        type Output<'a> = OpentypeAxisValueArray<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            axis_value_count: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_stat_axis_value_array(p, axis_value_count)
        }
    }

    proxy!(OpentypeDesignAxesArray => DAxisArray);

    impl CommonObject for DAxisArray {
        /// Args: `design_axis_count`
        type Args<'a> = u16;
        type Output<'a> = OpentypeDesignAxesArray;

        fn parse<'input>(
            p: &mut Parser<'input>,
            design_axis_count: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_stat_design_axes_array(p, design_axis_count)
        }
    }

    proxy!(OpentypeCmapSubtable => CmapSub);

    impl CommonObject for CmapSub {
        /// Args: `platform_id`
        type Args<'a> = u16;
        type Output<'a> = OpentypeCmapSubtable<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            platform_id: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_cmap_subtable(p, platform_id)
        }
    }

    proxy!(OpentypeItemVariationData => ItemVarData);

    impl CommonObject for ItemVarData {
        type Args<'a> = ();
        type Output<'a> = OpentypeItemVariationData;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_common_item_variation_data(p)
        }
    }

    proxy!(OpentypeVariationRegionList => VarRegList);

    impl CommonObject for VarRegList {
        type Args<'a> = ();
        type Output<'a> = OpentypeVariationRegionList;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_common_variation_region_list(p)
        }
    }

    proxy!(OpentypeAnchorTable => AncTable);

    impl CommonObject for AncTable {
        type Args<'a> = ();
        type Output<'a> = OpentypeAnchorTable<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_anchor_table(p)
        }
    }

    proxy!(OpentypeCoverageTable => CovTable);

    impl CommonObject for CovTable {
        type Args<'a> = ();
        type Output<'a> = OpentypeCoverageTable;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_coverage_table(p)
        }
    }

    proxy!(OpentypeMarkArray => MarkArr);

    impl CommonObject for MarkArr {
        type Args<'a> = ();
        type Output<'a> = OpentypeMarkArray<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _args: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_mark_array(p)
        }
    }

    proxy!(OpentypeBaseArray => BaseArr);

    impl CommonObject for BaseArr {
        /// Args : `mark_class_count`
        type Args<'a> = u16;
        type Output<'a> = OpentypeBaseArray<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            mark_class_count: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_base_array(p, mark_class_count)
        }
    }

    proxy!(OpentypeLigatureArray => LigArr);

    impl CommonObject for LigArr {
        /// Args : `mark_class_count`
        type Args<'a> = u16;
        type Output<'a> = OpentypeLigatureArray<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            mark_class_count: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_ligature_array(p, mark_class_count)
        }
    }

    proxy!(OpentypeClassDef => ClsDef @ doc = "ClassDef");

    impl CommonObject for ClsDef {
        type Args<'a> = ();
        type Output<'a> = OpentypeClassDef;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_class_def(p)
        }
    }

    proxy!(OpentypeDeviceOrVariationIndexTable => DevTable);

    impl CommonObject for DevTable {
        type Args<'a> = ();
        type Output<'a> = OpentypeDeviceOrVariationIndexTable;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_common_device_or_variation_index_table(p)
        }
    }

    proxy!(OpentypeVariationAxisRecord => AxisRec);

    impl CommonObject for AxisRec {
        type Args<'a> = ();
        type Output<'a> = OpentypeVariationAxisRecord;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_fvar_variation_axis_record(p)
        }
    }

    proxy!(OpentypeInstanceRecord => InstanceRec);

    impl CommonObject for InstanceRec {
        /// Args: `(axis_count, instance_size)`
        type Args<'a> = (u16, u16);
        type Output<'a> = OpentypeInstanceRecord;

        fn parse<'input>(
            p: &mut Parser<'input>,

            (axis_count, instance_size): (u16, u16),
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_fvar_instance_record(p, axis_count, instance_size)
        }
    }

    proxy!(OpentypeGlyphVariationData => GVarData);

    impl CommonObject for GVarData {
        /// Args: `(len, axis_count)`
        type Args<'a> = (usize, u16);
        type Output<'a> = OpentypeGlyphVariationData<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            (len, axis_count): (usize, u16),
        ) -> PResult<Self::Output<'input>> {
            p.start_slice(len)?;
            let ret = crate::Decoder_opentype_gvar_glyph_variation_data(p, axis_count)?;
            p.end_slice()?;
            Ok(ret)
        }
    }

    proxy!(OpentypeGvarTupleRecord => SharedTupleArr);

    impl CommonObject for SharedTupleArr {
        /// Args: `(shared_tuple_count, axis_count)`
        type Args<'a> = (usize, u16);
        type Output<'a> = Vec<OpentypeGvarTupleRecord>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            (shared_tuple_count, axis_count): (usize, u16),
        ) -> PResult<Self::Output<'input>> {
            let mut accum = Vec::with_capacity(shared_tuple_count);
            for _ in 0..shared_tuple_count {
                accum.push(crate::Decoder_opentype_gvar_tuple_record(p, axis_count)?);
            }
            Ok(accum)
        }
    }

    proxy!(OpentypeGposLookupSubtable => PosLookup);

    impl CommonObject for PosLookup {
        /// Args : lookup_type`
        type Args<'a> = u16;
        type Output<'a> = OpentypeGposLookupSubtable<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            lookup_type: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_ground_pos(p, lookup_type)
        }
    }

    proxy!(OpentypeGsubLookupSubtable => SubstLookup);

    impl CommonObject for SubstLookup {
        /// Args : `lookup_type`
        type Args<'a> = u16;
        type Output<'a> = OpentypeGsubLookupSubtable<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            lookup_type: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_ground_subst(p, lookup_type)
        }
    }

    proxy!(OpentypeGvarSerializedData => GvarSerData);

    impl CommonObject for GvarSerData {
        /// Args : `(shared_point_numbers, tuple_variation_headers)`
        type Args<'a> = (bool, &'a [super::OpentypeGvarTupleVariationHeader]);
        type Output<'a> = OpentypeGvarSerializedData;

        fn parse<'input>(
            p: &mut Parser<'input>,
            (shared_point_numbers, tuple_variation_headers): Self::Args<'input>,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gvar_serialized_data(
                p,
                shared_point_numbers,
                tuple_variation_headers,
            )
        }
    }

    proxy!(OpentypeSequenceTable => SeqTable);

    impl CommonObject for SeqTable {
        type Args<'a> = ();
        type Output<'a> = OpentypeSequenceTable;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_multiple_subst_sequence_table(p)
        }
    }

    proxy!(OpentypeGposLookupSubtableExt => PosSubtable);

    impl CommonObject for PosSubtable {
        /// Args : `lookup_type`
        type Args<'a> = u16;
        type Output<'a> = OpentypeGposLookupSubtableExt<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            lookup_type: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gpos_lookup_subtable(p, lookup_type)
        }
    }

    proxy!(OpentypeGsubLookupSubtableExt => SubstSubtable);

    impl CommonObject for SubstSubtable {
        /// Args : `lookup_type`
        type Args<'a> = u16;
        type Output<'a> = OpentypeGsubLookupSubtableExt<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            lookup_type: u16,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gsub_lookup_subtable(p, lookup_type)
        }
    }

    proxy!(OpentypeGposLookupTable => PosLookupTable);

    impl CommonObject for PosLookupTable {
        type Args<'a> = ();
        type Output<'a> = OpentypeGposLookupTable<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gpos_lookup_table(p)
        }
    }

    proxy!(OpentypeGsubLookupTable => SubstLookupTable);

    impl CommonObject for SubstLookupTable {
        type Args<'a> = ();
        type Output<'a> = OpentypeGsubLookupTable<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gsub_lookup_table(p)
        }
    }

    proxy!(OpentypeScriptList => ScrList);

    impl CommonObject for ScrList {
        type Args<'a> = ();
        type Output<'a> = OpentypeScriptList<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_script_list(p)
        }
    }

    proxy!(OpentypeScriptTable => ScrTable);

    impl CommonObject for ScrTable {
        type Args<'a> = ();
        type Output<'a> = OpentypeScriptTable<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_script_table(p)
        }
    }

    proxy!(OpentypeFeatureList => FeatList);

    impl CommonObject for FeatList {
        type Args<'a> = ();
        type Output<'a> = OpentypeFeatureList<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_feature_list(p)
        }
    }

    proxy!(OpentypeGposLookupList => PosLookups);

    impl CommonObject for PosLookups {
        type Args<'a> = ();
        type Output<'a> = OpentypeGposLookupList<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gpos_lookup_list(p)
        }
    }

    proxy!(OpentypeGsubLookupList => SubstLookups);

    impl CommonObject for SubstLookups {
        type Args<'a> = ();
        type Output<'a> = OpentypeGsubLookupList<'a>;

        fn parse<'input>(
            p: &mut Parser<'input>,
            _: Self::Args<'input>,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_gsub_lookup_list(p)
        }
    }

    proxy!(OpentypeFeatureVariations => FeatVar);

    impl CommonObject for FeatVar {
        type Args<'a> = ();
        type Output<'a> = OpentypeFeatureVariations<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_feature_variations(p)
        }
    }

    proxy!(OpentypeLangSys => LangSys);

    impl CommonObject for LangSys {
        type Args<'a> = ();
        type Output<'a> = OpentypeLangSys;

        fn parse<'input>(
            p: &mut Parser<'input>,
            _: Self::Args<'input>,
        ) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_langsys(p)
        }
    }

    proxy!(OpentypeFeatureTable => FeatTable);

    impl CommonObject for FeatTable {
        type Args<'a> = ();
        type Output<'a> = OpentypeFeatureTable<'a>;

        fn parse<'input>(p: &mut Parser<'input>, _: ()) -> PResult<Self::Output<'input>> {
            crate::Decoder_opentype_layout_feature_table(p)
        }
    }
}

/// Union over errors that arise during parsing, and generic-type errors arising in manual post-conversion
///
/// For soundness, `E` should not be [`doodle::parser::error::ParseError`]
#[derive(Debug)]
pub enum ValueParseError<E: std::error::Error> {
    Value(E),
    Parse(doodle::parser::error::ParseError),
}

impl<E: std::error::Error> ValueParseError<E> {
    /// Infallibly lifts a conversion error to a [`ValueParseError`]
    ///
    /// # Notes
    ///
    /// Should not be used to construct [`ValueParseError<ParseError>`]
    pub fn value(e: E) -> Self {
        ValueParseError::Value(e)
    }

    /// Fallibly coerces `self` into an error of the generic error-type `E`.
    ///
    /// # Panics
    ///
    /// Will panic if `self` represents a parser error.
    pub fn coerce_value(self) -> E {
        match self {
            ValueParseError::Value(e) => e,
            ValueParseError::Parse(e) => panic!("parse error: {e}"),
        }
    }
}

impl<E: std::error::Error> std::fmt::Display for ValueParseError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueParseError::Value(e) => write!(f, "value error: {e}"),
            ValueParseError::Parse(e) => write!(f, "parse error: {e}"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for ValueParseError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ValueParseError::Value(e) => Some(e),
            ValueParseError::Parse(e) => Some(e),
        }
    }
}

// NOTE - without specialization, we cannot have both this and From<ParseError>, so we choose the latter
/*
impl<E: std::error::Error> From<E> for ValueParseError<E> {
    fn from(e: E) -> Self {
        ValueParseError::Value(e)
    }
}
*/

impl<E: std::error::Error> From<doodle::parser::error::ParseError> for ValueParseError<E> {
    fn from(e: doodle::parser::error::ParseError) -> Self {
        ValueParseError::Parse(e)
    }
}

/// Variant of `Promote` for objects holding offsets but which do not encapsulate the `View` they are relative to.
trait PromoteView<Original>: Sized {
    /// Convert a source-object to `Self` using the provided `View``.
    fn promote_view(orig: &Original, view: View<'_>) -> PResult<Self>;
}

/// Variant of `TryPromote` for objects holding offsets but which do not encapsulate the `View` they are relative to.
trait TryPromoteView<Original>: Sized {
    /// The error-type returned when a given conversion cannot succeed.
    type Error<'input>: std::error::Error
    where
        Original: 'input;

    /// Fallibly post-process from a source-object to `Self` using the provided `View``.
    fn try_promote_view<'input>(
        orig: &'input Original,
        view: View<'input>,
    ) -> Result<Self, ValueParseError<Self::Error<'input>>>
    where
        Original: 'input;
}

/// Custom trait that facilitates conversion from partially-borrowed non-atomic types
/// without needing explicit lifetimes in the trait signature itself.
trait TryFromRef<Original: _Ref>: Sized {
    type Error: std::error::Error;

    /// Fallibly convert from the GAT `Ref<'a>` defined on `Original` (via the `_Ref` trait), into `Self`.
    fn try_from_ref(orig: <Original as _Ref>::Ref<'_>) -> Result<Self, Self::Error>;
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

/// Takes an option over `O` and directly promotes it to `T` if `Some(_)`, otherwise returning `T::from_null()`.
fn promote_from_null<O, T>(orig_opt: &Option<O>) -> T
where
    T: FromNull + Promote<O>,
{
    T::renew(orig_opt.as_ref().map(T::promote))
}

/// Takes an option over `O` and calls `try_promote` if it is `Some(_)`, otherwise returning
/// `Ok(T::from_null())`.
fn try_promote_from_null<O, T>(orig_opt: &Option<O>) -> Result<T, T::Error>
where
    T: FromNull + TryPromote<O>,
{
    Ok(T::renew(orig_opt.as_ref().map(T::try_promote).transpose()?))
}

/// Takes an iterable over `O` and directly promotes each item to `T`, returning a `Vec<T>`.
fn promote_all<O, I, T>(orig: I) -> Vec<T>
where
    T: Promote<O>,
    I: IntoIterator<Item = O>,
{
    orig.into_iter().map(|raw| T::promote(&raw)).collect()
}

/// Takes an iterable over `O` and directly promotes each item to `T`, returning a `Vec<T>`.
fn promote_all_ok<O, I, T, E>(orig: I, count: usize) -> Result<Vec<T>, E>
where
    T: Promote<O>,
    I: IntoIterator<Item = Result<O, E>>,
{
    let mut ret = Vec::with_capacity(count);
    for raw in orig {
        ret.push(T::promote(&raw?));
    }
    Ok(ret)
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

fn promote_vec_view<O, T>(orig_slice: &[O], view: View<'_>) -> PResult<Vec<T>>
where
    T: PromoteView<O>,
{
    let mut ret = Vec::with_capacity(orig_slice.len());
    for elem in orig_slice {
        ret.push(T::promote_view(elem, view)?);
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

fn promote_link<O, T>(orig_link: &Link<O>) -> Link<T>
where
    T: Promote<O>,
{
    orig_link.as_ref().map(T::promote)
}

fn try_promote_link<O, T>(orig: &Link<O>) -> Result<Link<T>, T::Error>
where
    T: TryPromote<O>,
{
    orig.as_ref().map(T::try_promote).transpose()
}

fn try_promote_opt_view<'input, O, T>(
    orig: &'input Option<O>,
    view: View<'input>,
) -> Result<Option<T>, ValueParseError<T::Error<'input>>>
where
    T: TryPromoteView<O>,
{
    orig.as_ref()
        .map(|orig| T::try_promote_view(orig, view))
        .transpose()
}

// !SECTION

// SECTION - Generic (but niche-use-case) helper definitions
/// Lexically distinct Option for values that are theoretically non-Nullable and have no FromNull instance.
type Link<T> = Option<T>;

/// Wrapper type for CommonObject artifacts for offsets that are intended to be nullable (which yields Option<T::Output>)
pub(crate) struct Nullable<T>(pub T);
pub(crate) struct Mandatory<T>(pub T);

impl<O: container::CommonObject> container::CommonObject for Nullable<O> {
    type Args<'a> = O::Args<'a>;
    type Output<'a> = Option<O::Output<'a>>;

    fn parse_offset<'input>(
        view: View<'input>,
        offset: usize,
        args: Self::Args<'input>,
    ) -> PResult<Self::Output<'input>> {
        O::parse_nullable_offset(view, offset, args)
    }

    fn parse<'input>(
        _p: &mut Parser<'input>,
        _args: Self::Args<'input>,
    ) -> PResult<Self::Output<'input>> {
        unimplemented!("Nullable::parse is explicitly left unimplemented")
    }
}

impl<O: container::CommonObject> container::CommonObject for Mandatory<O> {
    type Args<'a> = O::Args<'a>;
    type Output<'a> = O::Output<'a>;

    fn parse_offset<'input>(
        view: View<'input>,
        offset: usize,
        args: Self::Args<'input>,
    ) -> PResult<Self::Output<'input>> {
        assert_ne!(
            offset,
            0,
            "Mandatory<{}> found offset=0 (null)",
            std::any::type_name::<O>()
        );
        O::parse_offset(view, offset, args)
    }

    fn parse<'input>(
        _p: &mut Parser<'input>,
        _args: Self::Args<'input>,
    ) -> PResult<Self::Output<'input>> {
        unimplemented!("Mandatory::parse is explicitly left unimplemented")
    }
}

/// Vector of values with representation `T` that have a nominal semantic interpretation specified by `Sem`.
///
/// Though generic, the practical usages of this type are for distinguishing `ClassId := u16` and `GlyphId := u16`
/// semantics in GSUB/GPOS Lookup tables.
#[repr(transparent)]
struct SemVec<Sem, T> {
    inner: Vec<T>,
    __proxy: std::marker::PhantomData<Sem>,
}

impl<Sem, T> Default for SemVec<Sem, T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            __proxy: Default::default(),
        }
    }
}

impl<Sem, T: Clone> Clone for SemVec<Sem, T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            __proxy: self.__proxy,
        }
    }
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

    pub(crate) type ReflType3<A, B, C> = ReflType<A, ReflType<B, C>>;

    pub(crate) type ReflType4<A, B, C, D> = ReflType<ReflType<A, B>, ReflType<C, D>>;

    pub(crate) type ReflType5<A, B, C, D, E> = ReflType<ReflType<A, B>, ReflType3<C, D, E>>;

    pub(crate) type ReflType6<A, B, C, D, E, F> = ReflType<ReflType3<A, B, C>, ReflType3<D, E, F>>;

    pub(crate) type ReflType7<A, B, C, D, E, F, G> =
        ReflType<ReflType3<A, B, C>, ReflType4<D, E, F, G>>;
}
use refl::ReflType;

/// Shorthand for qualifying a TryPromote::Error item
type TPErr<Src, Tgt> = <Tgt as TryPromote<Src>>::Error;
type TPVErr<'a, Src, Tgt> = <Tgt as TryPromoteView<Src>>::Error<'a>;

/// Shorthand for qualifying a TryFromRef::Error item in the same style as `TPErr`
type TFRErr<Src, Tgt> = <Tgt as TryFromRef<Src>>::Error;

/// Hint to remind us that a given error-type has strictly local provenance
type Local<T> = T;

// SECTION - crate-local trait impls on top-level table types

frame!(OpentypeGdef);

impl<'input> container::MultiContainer<Nullable<obj::ClsDef>, 2> for OpentypeGdef<'input> {
    fn get_offset_array(&self) -> [usize; 2] {
        [
            self.glyph_class_def.offset as usize,
            self.mark_attach_class_def.offset as usize,
        ]
    }

    fn get_args_array(&self) -> [<Nullable<obj::AttList> as container::CommonObject>::Args<'_>; 2] {
        [(); 2]
    }
}

impl<'input> container::SingleContainer<Nullable<obj::AttList>> for OpentypeGdef<'input> {
    fn get_offset(&self) -> usize {
        self.attach_list.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> container::SingleContainer<Nullable<obj::LigCarList>> for OpentypeGdef<'input> {
    fn get_offset(&self) -> usize {
        self.lig_caret_list.offset as usize
    }

    fn get_args(&self) -> () {}
}

frame!(OpentypeGpos);

impl<'input> container::SingleContainer<Mandatory<obj::ScrList>> for OpentypeGpos<'input> {
    fn get_offset(&self) -> usize {
        self.script_list.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> container::SingleContainer<Mandatory<obj::FeatList>> for OpentypeGpos<'input> {
    fn get_offset(&self) -> usize {
        self.feature_list.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> container::SingleContainer<Mandatory<obj::PosLookups>> for OpentypeGpos<'input> {
    fn get_offset(&self) -> usize {
        self.lookup_list.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> container::OptContainer<Nullable<obj::FeatVar>> for OpentypeGpos<'input> {
    fn contains_object(&self) -> bool {
        self.feature_variations_offset.is_some()
    }

    fn get_offset(&self) -> Option<usize> {
        self.feature_variations_offset
            .map(|offs| offs.offset as usize)
    }
}

frame!(OpentypeGsub);

impl<'input> container::SingleContainer<Mandatory<obj::ScrList>> for OpentypeGsub<'input> {
    fn get_offset(&self) -> usize {
        self.script_list.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> container::SingleContainer<Mandatory<obj::FeatList>> for OpentypeGsub<'input> {
    fn get_offset(&self) -> usize {
        self.feature_list.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> container::SingleContainer<Mandatory<obj::SubstLookups>> for OpentypeGsub<'input> {
    fn get_offset(&self) -> usize {
        self.lookup_list.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> container::OptContainer<Nullable<obj::FeatVar>> for OpentypeGsub<'input> {
    fn contains_object(&self) -> bool {
        self.feature_variations_offset.is_some()
    }

    fn get_offset(&self) -> Option<usize> {
        self.feature_variations_offset
            .map(|offs| offs.offset as usize)
    }
}

impl<'input> container::SingleContainer<obj::DAxisArray> for OpentypeStat<'input> {
    fn get_offset(&self) -> usize {
        self.design_axes.offset as usize
    }

    fn get_args(&self) -> <obj::DAxisArray as container::CommonObject>::Args<'_> {
        self.design_axis_count
    }
}

frame!(OpentypeStat);

impl<'input> container::SingleContainer<obj::AxisValueArr> for OpentypeStat<'input> {
    fn get_offset(&self) -> usize {
        self.axis_value_offsets.offset as usize
    }

    fn get_args(&self) -> <obj::AxisValueArr as container::CommonObject>::Args<'_> {
        self.axis_value_count
    }
}
// !SECTION

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
    required: Heap<RequiredTableMetrics>,
    optional: Heap<OptionalTableMetrics>,
    extraMagic: Vec<u32>,
}

pub type OpentypeCmapSubtable<'a> = opentype_cmap_subtable<'a>;

impl<'a> Promote<OpentypeCmapSubtable<'a>> for CmapSubtable {
    fn promote(orig: &OpentypeCmapSubtable) -> Self {
        CmapSubtable::AnyFormat(orig.format)
    }
}

#[derive(Debug, Clone)]
enum CmapSubtable {
    // STUB[scaffolding] - this is intentionally underimplemented to make encoding-record construction happen sooner for debugging
    AnyFormat(u16),
}

pub type OpentypeEncodingRecord<'a> = opentype_encoding_record<'a>;

#[derive(Debug, Clone)]
struct EncodingRecord {
    platform: u16,
    encoding: u16,
    subtable: Link<CmapSubtable>,
}

impl<'input> container::SingleContainer<Nullable<obj::CmapSub>> for OpentypeEncodingRecord<'input> {
    fn get_offset(&self) -> usize {
        self.subtable.offset as usize
    }

    fn get_args(&self) -> <obj::CmapSub as container::CommonObject>::Args<'_> {
        self.platform
    }
}

impl<'a> PromoteView<OpentypeEncodingRecord<'a>> for EncodingRecord {
    fn promote_view(orig: &OpentypeEncodingRecord, view: View<'_>) -> PResult<Self> {
        Ok(EncodingRecord {
            platform: orig.platform,
            encoding: orig.encoding,
            subtable: promote_link(&reify_dep(view, orig, Nullable(obj::CmapSub))?),
        })
    }
}

impl<'a> Promote<OpentypeCmap<'a>> for Cmap {
    fn promote(orig: &OpentypeCmap) -> Self {
        Cmap {
            version: orig.version,
            encoding_records: promote_vec_view(&orig.encoding_records, orig.table_scope)
                .expect("bad parse"),
        }
    }
}

// REVIEW - this style of naming and promotion-impl may be useful for other top-level table types
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
enum MaxpMetrics {
    Postscript { version: u32 }, // version 0.5
    // STUB - enrich with any further details we care about presenting
    Version1 { version: u32 },       // version 1.0
    UnknownVersion { version: u32 }, // anything else
}

#[derive(Clone, Copy, Debug)]
// STUB - enrich with any further details we care about presenting
struct HheaMetrics {
    major_version: u16,
    minor_version: u16,
    num_lhm: usize,
}

#[derive(Clone, Copy, Debug)]
// STUB - enrich with any further details we care about presenting
struct VheaMetrics {
    major_version: u16,
    minor_version: u16,
    num_lvm: usize,
}

fn promote_count_array_link<O, T>(count: u16, (offset, link): (u32, Option<&Vec<O>>)) -> Vec<T>
where
    T: Promote<O>,
{
    let mut result = Vec::with_capacity(count as usize);
    if count == 0 {
        // FIXME - in at least one font test-case, the commented-out assertion on the following line causes a panic
        // debug_assert_eq!(offset, 0, "count=0 requires offset=0, but found offset={offset} instead");
        assert!(link.is_none() || link.is_some_and(Vec::is_empty));
    } else {
        assert_ne!(
            offset, 0,
            "count>0 requires offset>0, but found offset=0 instead"
        );
        let Some(origs) = link else {
            unreachable!("offset>0 but link is None");
        };
        assert_eq!(origs.len(), count as usize);
        for orig in origs.iter() {
            let promoted = T::promote(orig);
            result.push(promoted);
        }
    }
    result
}

frame!(OpentypeGvar);

struct GlyphVariationDataArray<'a, 'input> {
    array_scope: View<'input>,
    axis_count: u16,
    offsets_array: &'a opentype_loca_table_offsets,
}

impl<'input> container::DynContainer<Nullable<obj::GVarData>> for OpentypeGvar<'input> {
    fn count(&self) -> usize {
        match &self.glyph_variation_data_offsets {
            opentype_loca_table_offsets::Offsets16(half16s) => half16s.len() - 1,
            opentype_loca_table_offsets::Offsets32(off32s) => off32s.len() - 1,
        }
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        let ret: Box<dyn Iterator<Item = usize>> = match &self.glyph_variation_data_offsets {
            opentype_loca_table_offsets::Offsets16(half16s) => Box::new(
                half16s
                    .iter()
                    .map(|half16| *half16 as usize * 2)
                    .take(half16s.len() - 1),
            ),
            opentype_loca_table_offsets::Offsets32(off32s) => Box::new(
                off32s
                    .iter()
                    .map(|off32| *off32 as usize)
                    .take(off32s.len() - 1),
            ),
        };
        // REVIEW - does this merit an intermediate GVarDataArray structure instead?
        ret.map(|array_rel_offs| array_rel_offs + self.glyph_variation_data_array_offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = (usize, u16)> {
        let lengths: Box<dyn Iterator<Item = usize>> = match &self.glyph_variation_data_offsets {
            opentype_loca_table_offsets::Offsets16(half16s) => Box::new(
                Iterator::zip(half16s.iter(), half16s[1..].iter())
                    .map(|(this, next)| (*next - *this) as usize * 2),
            ),
            opentype_loca_table_offsets::Offsets32(off32s) => Box::new(
                Iterator::zip(off32s.iter(), off32s[1..].iter())
                    .map(|(this, next)| (*next - *this) as usize),
            ),
        };
        Iterator::zip(lengths, std::iter::repeat(self.axis_count))
    }
}

impl<'input> container::SingleContainer<Nullable<obj::SharedTupleArr>> for OpentypeGvar<'input> {
    fn get_offset(&self) -> usize {
        self.shared_tuples.offset as usize
    }

    fn get_args(&self) -> (usize, u16) {
        (self.shared_tuple_count as usize, self.axis_count)
    }
}

impl<'input> Promote<OpentypeGvar<'input>> for GvarMetrics {
    fn promote(orig: &OpentypeGvar<'input>) -> Self {
        let glyph_variation_data_array = reify_all(orig, Nullable(obj::GVarData))
            .map(|data| data.as_ref().map(GlyphVariationData::promote))
            .collect();
        let shared_tuples = promote_from_null(&reify(orig, Nullable(obj::SharedTupleArr)));
        Self {
            major_version: orig.major_version,
            minor_version: orig.minor_version,
            shared_tuples,
            glyph_count: orig.glyph_count,
            flags: Promote::promote(&orig.flags),
            glyph_variation_data_array,
        }
    }
}

pub type OpentypeGvarTupleRecord = opentype_gvar_tuple_record;

impl Promote<Vec<OpentypeGvarTupleRecord>> for Vec<GvarTupleRecord> {
    fn promote(orig: &Vec<OpentypeGvarTupleRecord>) -> Self {
        promote_vec(orig)
    }
}

impl Promote<OpentypeGvarTupleRecord> for GvarTupleRecord {
    fn promote(orig: &OpentypeGvarTupleRecord) -> Self {
        GvarTupleRecord {
            coordinates: promote_vec(&orig.coordinates),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GvarTupleRecord {
    coordinates: Vec<F2Dot14>,
}

pub type OpentypeGvarFlags = opentype_gvar_table_flags;

impl Promote<OpentypeGvarFlags> for GvarFlags {
    fn promote(orig: &OpentypeGvarFlags) -> Self {
        GvarFlags {
            is_long_offset: orig.is_long_offset,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GvarFlags {
    is_long_offset: bool,
}

pub type OpentypeGvarTupleVariationHeader = opentype_gvar_tuple_variation_header;

impl Promote<OpentypeGvarTupleVariationHeader> for GvarTupleVariationHeader {
    fn promote(orig: &OpentypeGvarTupleVariationHeader) -> Self {
        GvarTupleVariationHeader {
            variation_data_size: orig.variation_data_size,
            tuple_index: GvarTupleVariationHeaderTupleIndex::promote(&orig.tuple_index),
            peak_tuple: promote_opt(&orig.peak_tuple),
            intermediate_tuples: promote_opt(&orig.intermediate_tuples),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GvarTupleVariationHeader {
    variation_data_size: u16,
    tuple_index: GvarTupleVariationHeaderTupleIndex,
    peak_tuple: Option<GvarTupleRecord>,
    intermediate_tuples: Option<GvarIntermediateTuples>,
}

pub type OpentypeGvarTupleVariationHeaderTupleIndex =
    opentype_gvar_tuple_variation_header_tuple_index;

impl Promote<OpentypeGvarTupleVariationHeaderTupleIndex> for GvarTupleVariationHeaderTupleIndex {
    fn promote(orig: &OpentypeGvarTupleVariationHeaderTupleIndex) -> Self {
        GvarTupleVariationHeaderTupleIndex {
            embedded_peak_tuple: orig.embedded_peak_tuple,
            intermediate_region: orig.intermediate_region,
            private_point_numbers: orig.private_point_numbers,
            tuple_index: orig.tuple_index,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GvarTupleVariationHeaderTupleIndex {
    embedded_peak_tuple: bool,
    intermediate_region: bool,
    private_point_numbers: bool,
    tuple_index: u16,
}

pub type OpentypeGvarIntermediateTuples = opentype_gvar_tuple_variation_header_intermediate_tuples;

impl Promote<OpentypeGvarIntermediateTuples> for GvarIntermediateTuples {
    fn promote(orig: &OpentypeGvarIntermediateTuples) -> Self {
        GvarIntermediateTuples {
            start_tuple: GvarTupleRecord::promote(&orig.start_tuple),
            end_tuple: GvarTupleRecord::promote(&orig.end_tuple),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GvarIntermediateTuples {
    start_tuple: GvarTupleRecord,
    end_tuple: GvarTupleRecord,
}

pub type OpentypeGlyphVariationData<'input> = opentype_gvar_glyph_variation_data<'input>;

frame!(OpentypeGlyphVariationData.data_scope);

impl<'input> container::SingleContainer<obj::GvarSerData> for OpentypeGlyphVariationData<'input> {
    fn get_offset(&self) -> usize {
        self.data_offset as usize
    }

    fn get_args(&self) -> <obj::GvarSerData as container::CommonObject>::Args<'_> {
        (
            self.tuple_variation_count.shared_point_numbers,
            &self.tuple_variation_headers,
        )
    }
}

impl<'input> Promote<OpentypeGlyphVariationData<'input>> for GlyphVariationData {
    fn promote(orig: &OpentypeGlyphVariationData) -> Self {
        GlyphVariationData {
            shared_point_numbers: orig.tuple_variation_count.shared_point_numbers,
            tuple_count: orig.tuple_variation_count.tuple_count,
            tuple_variation_headers: promote_vec(&orig.tuple_variation_headers),
            data: GvarSerializedData::promote(&reify_index(orig, obj::GvarSerData, 0)),
        }
    }
}

pub type OpentypePackedPoints = opentype_var_packed_point_numbers_run_points;
pub type OpentypePackedPointRun = opentype_var_packed_point_numbers_runs;
pub type OpentypePackedPointRuns = (u16, Vec<OpentypePackedPointRun>);

impl Promote<OpentypePackedPointRuns> for PackedPointNumbers {
    fn promote(orig: &OpentypePackedPointRuns) -> Self {
        PackedPointNumbers {
            point_numbers: promote_vec(&orig.1),
        }
    }
}

impl Promote<OpentypePackedPointRun> for PackedPointRun {
    fn promote(orig: &OpentypePackedPointRun) -> Self {
        PackedPointRun::promote(&orig.points)
    }
}

impl Promote<OpentypePackedPoints> for PackedPointRun {
    fn promote(orig: &OpentypePackedPoints) -> Self {
        match orig {
            OpentypePackedPoints::Points8(points) => PackedPointRun::Short(points.clone()),
            OpentypePackedPoints::Points16(points) => PackedPointRun::Long(points.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PackedPointRun {
    Short(Vec<u8>),
    Long(Vec<u16>),
}

#[derive(Debug, Clone)]
pub struct PackedPointNumbers {
    point_numbers: Vec<PackedPointRun>,
}

pub type OpentypeXYCoordinateDeltas = (u16, Vec<OpentypeCoordinateDeltaRun>);
pub type OpentypeCoordinateDeltaRun =
    opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas;
pub type OpentypeXYCoordinateDeltaDeltas =
    opentype_gvar_serialized_data_per_tuple_variation_data_x_and_y_coordinate_deltas_deltas;

impl Promote<OpentypeCoordinateDeltaRun> for CoordinateDeltas {
    fn promote(orig: &OpentypeCoordinateDeltaRun) -> Self {
        CoordinateDeltas::promote(&orig.deltas)
    }
}

impl Promote<OpentypeXYCoordinateDeltaDeltas> for CoordinateDeltas {
    fn promote(orig: &OpentypeXYCoordinateDeltaDeltas) -> Self {
        match orig {
            &OpentypeXYCoordinateDeltaDeltas::Delta0(run_length) => {
                CoordinateDeltas::Zero { run_length }
            }
            OpentypeXYCoordinateDeltaDeltas::Delta8(raw) => CoordinateDeltas::Short {
                deltas: raw.iter().map(|x| *x as i8).collect(),
            },
            OpentypeXYCoordinateDeltaDeltas::Delta16(raw) => CoordinateDeltas::Long {
                deltas: raw.iter().map(|x| *x as i16).collect(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum CoordinateDeltas {
    Zero { run_length: u8 },
    Short { deltas: Vec<i8> },
    Long { deltas: Vec<i16> },
}

impl Promote<OpentypeXYCoordinateDeltas> for XYCoordinateDeltas {
    fn promote(orig: &OpentypeXYCoordinateDeltas) -> Self {
        XYCoordinateDeltas {
            xy_deltas: promote_vec(&orig.1),
        }
    }
}
#[derive(Debug, Clone)]
pub struct XYCoordinateDeltas {
    xy_deltas: Vec<CoordinateDeltas>,
}

pub type OpentypeGvarPerTupleVariationData = opentype_gvar_serialized_data_per_tuple_variation_data;

impl Promote<OpentypeGvarPerTupleVariationData> for GvarPerTupleVariationData {
    fn promote(orig: &OpentypeGvarPerTupleVariationData) -> Self {
        GvarPerTupleVariationData {
            private_point_numbers: promote_opt(&orig.private_point_numbers),
            x_and_y_coordinate_deltas: XYCoordinateDeltas::promote(&orig.x_and_y_coordinate_deltas),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GvarPerTupleVariationData {
    private_point_numbers: Option<PackedPointNumbers>,
    x_and_y_coordinate_deltas: XYCoordinateDeltas,
}

pub type OpentypeGvarSerializedData = opentype_gvar_serialized_data;

impl Promote<OpentypeGvarSerializedData> for GvarSerializedData {
    fn promote(orig: &OpentypeGvarSerializedData) -> Self {
        GvarSerializedData {
            shared_point_numbers: promote_opt(&orig.shared_point_numbers),
            per_tuple_variation_data: promote_vec(&orig.per_tuple_variation_data),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GvarSerializedData {
    shared_point_numbers: Option<PackedPointNumbers>,
    per_tuple_variation_data: Vec<GvarPerTupleVariationData>,
}

#[derive(Debug, Clone)]
pub struct GlyphVariationData {
    shared_point_numbers: bool,
    tuple_count: u16,
    tuple_variation_headers: Vec<GvarTupleVariationHeader>,
    data: GvarSerializedData,
}

#[derive(Debug, Clone)]
struct GvarMetrics {
    major_version: u16,
    minor_version: u16,
    shared_tuples: Vec<GvarTupleRecord>,
    glyph_count: u16,
    flags: GvarFlags,
    glyph_variation_data_array: Vec<Option<GlyphVariationData>>,
}

#[derive(Debug, Clone)]
struct FvarMetrics {
    major_version: u16,
    minor_version: u16,
    axes: Vec<VariationAxisRecord>,
    instances: Vec<InstanceRecord>,
}

frame!(OpentypeFvar);

impl<'input> container::DynContainer<obj::AxisRec> for OpentypeFvar<'input> {
    fn count(&self) -> usize {
        self.axis_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        (0..self.axis_count).map(|ix: u16| (self.offset_axes + ix * self.axis_size) as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'input> container::DynContainer<obj::InstanceRec> for OpentypeFvar<'input> {
    fn count(&self) -> usize {
        self.axis_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        (0..self.instance_count).map(|ix: u16| {
            (self.offset_axes + (self.axis_count * self.axis_size) + ix * self.instance_size)
                as usize
        })
    }

    fn iter_args(&self) -> impl Iterator<Item = (u16, u16)> {
        std::iter::repeat((self.axis_count, self.instance_size))
    }
}

impl<'input> Promote<OpentypeFvar<'input>> for FvarMetrics {
    fn promote(orig: &OpentypeFvar) -> Self {
        let axes = reify_all(orig, obj::AxisRec)
            .map(|raw| VariationAxisRecord::promote(&raw))
            .collect();
        let instances = reify_all(orig, obj::InstanceRec)
            .map(|raw| InstanceRecord::promote(&raw))
            .collect();
        FvarMetrics {
            major_version: orig.major_version,
            minor_version: orig.minor_version,
            axes,
            instances,
        }
    }
}

pub type OpentypeUserTuple = opentype_fvar_user_tuple;

impl Promote<OpentypeUserTuple> for UserTuple {
    fn promote(orig: &OpentypeUserTuple) -> Self {
        promote_vec(&orig.coordinates)
    }
}

type UserTuple = Vec<Fixed>;

pub type OpentypeInstanceRecord = opentype_fvar_instance_record;

// REVIEW - currently not implemented in the OpentypeLayoutScriptList => ScrList spec;
type InstanceFlags = ();

impl Promote<OpentypeInstanceRecord> for InstanceRecord {
    fn promote(orig: &OpentypeInstanceRecord) -> InstanceRecord {
        InstanceRecord {
            subfamily_nameid: NameId::from(orig.subfamily_nameid),
            flags: InstanceFlags::promote(&orig.flags),
            coordinates: UserTuple::promote(&orig.coordinates),
            postscript_nameid: promote_opt(&orig.postscript_nameid),
        }
    }
}

#[derive(Debug, Clone)]
struct InstanceRecord {
    subfamily_nameid: NameId,
    flags: InstanceFlags,
    coordinates: UserTuple,
    postscript_nameid: Option<NameId>,
}

pub type OpentypeVariationAxisRecord = opentype_fvar_variation_axis_record;

pub type OpentypeVariationAxisRecordFlags = opentype_fvar_variation_axis_record_flags;

impl Promote<OpentypeVariationAxisRecord> for VariationAxisRecord {
    fn promote(orig: &OpentypeVariationAxisRecord) -> Self {
        VariationAxisRecord {
            axis_tag: Tag::promote(&orig.axis_tag),
            min_value: Fixed::promote(&orig.min_value),
            default_value: Fixed::promote(&orig.default_value),
            max_value: Fixed::promote(&orig.max_value),
            flags: VariationAxisRecordFlags::promote(&orig.flags),
            axis_name_id: NameId::from(orig.axis_name_id),
        }
    }
}

impl Promote<OpentypeVariationAxisRecordFlags> for VariationAxisRecordFlags {
    fn promote(orig: &OpentypeVariationAxisRecordFlags) -> VariationAxisRecordFlags {
        VariationAxisRecordFlags {
            hidden_axis: orig.hidden_axis,
        }
    }
}
#[derive(Clone, Copy, Debug)]
struct VariationAxisRecordFlags {
    hidden_axis: bool,
}

#[derive(Debug, Clone, Copy)]
struct VariationAxisRecord {
    axis_tag: Tag,
    min_value: Fixed,
    default_value: Fixed,
    max_value: Fixed,
    flags: VariationAxisRecordFlags,
    axis_name_id: NameId,
}

#[derive(Debug, Clone)]
struct StatMetrics {
    major_version: u16,
    minor_version: u16,
    design_axes: Vec<DesignAxis>,
    axis_values: Vec<AxisValue>,
    elided_fallback_name_id: NameId,
}

pub type OpentypeAxisValueArray<'a> = opentype_stat_axis_value_array<'a>;

frame!(OpentypeAxisValueArray.array_scope);

impl<'a> container::DynContainer<obj::AxisValTbl> for OpentypeAxisValueArray<'a> {
    fn count(&self) -> usize {
        self.axis_values.len()
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.axis_values.iter().map(|x| x.offset as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <obj::AxisValTbl as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

pub type OpentypeAxisValue = opentype_stat_axis_value_table;
pub type OpentypeAxisValueData = opentype_stat_axis_value_table_data;

pub type OpentypeAxisValueFormat1 = opentype_stat_axis_value_table_data_Format1;
pub type OpentypeAxisValueFormat2 = opentype_stat_axis_value_table_data_Format2;
pub type OpentypeAxisValueFormat3 = opentype_stat_axis_value_table_data_Format3;
pub type OpentypeAxisValueFormat4 = opentype_stat_axis_value_table_data_Format4;

pub type OpentypeAxisValueFlags = opentype_stat_axis_value_table_data_Format1_flags;

impl Promote<OpentypeAxisValueFlags> for AxisValueFlags {
    fn promote(orig: &OpentypeAxisValueFlags) -> Self {
        Self {
            elidable_axis_value_name: orig.elidable_axis_value_name,
            older_sibling_font_attribute: orig.older_sibling_font_attribute,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct AxisValueFlags {
    elidable_axis_value_name: bool,
    older_sibling_font_attribute: bool,
}

impl Promote<OpentypeAxisValueFormat1> for AxisValueFormat1 {
    fn promote(orig: &OpentypeAxisValueFormat1) -> Self {
        AxisValueFormat1 {
            axis_index: orig.axis_index,
            flags: AxisValueFlags::promote(&orig.flags),
            value_name_id: NameId::from(orig.value_name_id),
            value: Fixed::promote(&orig.value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct AxisValueFormat1 {
    axis_index: u16,
    flags: AxisValueFlags,
    value_name_id: NameId,
    value: Fixed,
}

impl Promote<OpentypeAxisValueFormat2> for AxisValueFormat2 {
    fn promote(orig: &OpentypeAxisValueFormat2) -> Self {
        AxisValueFormat2 {
            axis_index: orig.axis_index,
            flags: AxisValueFlags::promote(&orig.flags),
            value_name_id: NameId::from(orig.value_name_id),
            nominal_value: Fixed::promote(&orig.nominal_value),
            range_min_value: Fixed::promote(&orig.range_min_value),
            range_max_value: Fixed::promote(&orig.range_max_value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct AxisValueFormat2 {
    axis_index: u16,
    flags: AxisValueFlags,
    value_name_id: NameId,
    nominal_value: Fixed,
    range_min_value: Fixed,
    range_max_value: Fixed,
}

impl Promote<OpentypeAxisValueFormat3> for AxisValueFormat3 {
    fn promote(orig: &OpentypeAxisValueFormat3) -> Self {
        AxisValueFormat3 {
            axis_index: orig.axis_index,
            flags: AxisValueFlags::promote(&orig.flags),
            value_name_id: NameId::from(orig.value_name_id),
            value: Fixed::promote(&orig.value),
            linked_value: Fixed::promote(&orig.linked_value),
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct AxisValueFormat3 {
    axis_index: u16,
    flags: AxisValueFlags,
    value_name_id: NameId,
    value: Fixed,
    linked_value: Fixed,
}

pub type OpentypeAxisValueRecord = opentype_stat_axis_value_table_data_Format4_axis_values;

impl Promote<OpentypeAxisValueRecord> for AxisValueRecord {
    fn promote(orig: &OpentypeAxisValueRecord) -> Self {
        AxisValueRecord {
            axis_index: orig.axis_index,
            value: Fixed::promote(&orig.value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct AxisValueRecord {
    axis_index: u16,
    value: Fixed,
}

impl Promote<OpentypeAxisValueFormat4> for AxisValueFormat4 {
    fn promote(orig: &OpentypeAxisValueFormat4) -> Self {
        AxisValueFormat4 {
            flags: AxisValueFlags::promote(&orig.flags),
            value_name_id: NameId::from(orig.value_name_id),
            axis_values: promote_vec(&orig.axis_values),
        }
    }
}

#[derive(Debug, Clone)]
struct AxisValueFormat4 {
    flags: AxisValueFlags,
    value_name_id: NameId,
    axis_values: Vec<AxisValueRecord>,
}

impl Promote<OpentypeAxisValue> for AxisValue {
    fn promote(orig: &OpentypeAxisValue) -> Self {
        Self::promote(&orig.data)
    }
}

impl Promote<OpentypeAxisValueData> for AxisValue {
    fn promote(orig: &OpentypeAxisValueData) -> Self {
        match orig {
            OpentypeAxisValueData::Format1(f1) => AxisValue::Format1(AxisValueFormat1::promote(f1)),
            OpentypeAxisValueData::Format2(f2) => AxisValue::Format2(AxisValueFormat2::promote(f2)),
            OpentypeAxisValueData::Format3(f3) => AxisValue::Format3(AxisValueFormat3::promote(f3)),
            OpentypeAxisValueData::Format4(f4) => AxisValue::Format4(AxisValueFormat4::promote(f4)),
        }
    }
}

#[derive(Debug, Clone)]
enum AxisValue {
    Format1(AxisValueFormat1),
    Format2(AxisValueFormat2),
    Format3(AxisValueFormat3),
    Format4(AxisValueFormat4),
}

pub type OpentypeDesignAxesArray = opentype_stat_design_axes_array;

impl Promote<OpentypeDesignAxesArray> for Vec<DesignAxis> {
    fn promote(orig: &OpentypeDesignAxesArray) -> Self {
        promote_vec(&orig.design_axes)
    }
}

pub type OpentypeDesignAxis = opentype_stat_design_axes_array_design_axes;

impl Promote<OpentypeDesignAxis> for DesignAxis {
    fn promote(orig: &OpentypeDesignAxis) -> Self {
        DesignAxis {
            axis_tag: Tag(orig.axis_tag),
            axis_name_id: NameId::from(orig.axis_name_id),
            axis_ordering: orig.axis_ordering,
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct DesignAxis {
    axis_tag: Tag,
    axis_name_id: NameId,
    axis_ordering: u16,
}

impl<'a> Promote<OpentypeStat<'a>> for StatMetrics {
    fn promote(orig: &OpentypeStat) -> Self {
        let design_axes = <Vec<DesignAxis>>::promote(&reify(orig, obj::DAxisArray));
        let axis_values = {
            let axis_value_array = reify(orig, obj::AxisValueArr);
            reify_all(&axis_value_array, obj::AxisValTbl)
                .map(|raw| AxisValue::promote(&raw))
                .collect()
        };
        StatMetrics {
            major_version: orig.major_version,
            minor_version: orig.minor_version,
            design_axes,
            axis_values,
            elided_fallback_name_id: NameId::from(orig.elided_fallback_name_id),
        }
    }
}

#[derive(Clone, Debug)]
struct HmtxMetrics(Vec<UnifiedBearing>);

#[derive(Clone, Debug)]
struct VmtxMetrics(Vec<UnifiedBearing>);

/// Unified Left-or-TOP side bearing
#[derive(Copy, Clone, Debug)]
struct UnifiedBearing {
    advance_width: Option<u16>, // FIXME - name is misleading as this also represents 'advance_height' for vmtx
    left_side_bearing: i16, // FIXME - name is misleading as this also represents 'top_side_bearing' for vmtx
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

impl From<u16> for NameId {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Promote<u16> for NameId {
    fn promote(value: &u16) -> Self {
        Self(*value)
    }
}

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

    fn convert(&self, data: &[u8]) -> String {
        match self {
            PlatformEncodingLanguageId::Macintosh(MacintoshEncodingId::Roman, ..) => MAC_ROMAN
                .decode(data, DecoderTrap::Ignore)
                .unwrap()
                .to_owned(),
            PlatformEncodingLanguageId::Macintosh(..) | PlatformEncodingLanguageId::Unicode(_) => {
                UTF_16BE
                    .decode(data, DecoderTrap::Ignore)
                    .unwrap()
                    .to_owned()
            }
            PlatformEncodingLanguageId::Windows(..) => UTF_16BE
                .decode(data, DecoderTrap::Ignore)
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
            0..=32 => unsafe { Ok(std::mem::transmute::<u16, MacintoshEncodingId>(value)) },
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
                });
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
    cmap: Heap<Cmap>,
    head: Heap<HeadMetrics>,
    hhea: Heap<HheaMetrics>,
    maxp: Heap<MaxpMetrics>,
    hmtx: Heap<HmtxMetrics>,
    name: Heap<NameMetrics>,
    os2: Heap<Os2Metrics>,
    post: Heap<PostMetrics>,
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

pub(crate) type CoverageRangeRecord = RangeRecord<u16>;

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
pub(crate) enum CoverageTable {
    Format1 {
        glyph_array: Vec<u16>,
    }, // Individual glyph indices
    Format2 {
        range_records: Vec<CoverageRangeRecord>,
    }, // Range of glyphs
}

impl CoverageTable {
    pub(crate) fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        match self {
            CoverageTable::Format1 { glyph_array } => Box::new(glyph_array.iter().copied()),
            CoverageTable::Format2 { range_records } => Box::new(
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

type OpentypeMarkGlyphSet<'a> = opentype_gdef_mark_glyph_set<'a>;

impl<'input> container::DynContainer<Nullable<obj::CovTable>> for OpentypeMarkGlyphSet<'input> {
    fn count(&self) -> usize {
        self.mark_glyph_set_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.coverage.iter().map(|off| off.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

frame!(OpentypeMarkGlyphSet);

impl<'a> TryPromote<OpentypeMarkGlyphSet<'a>> for MarkGlyphSet {
    type Error = Local<UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeMarkGlyphSet) -> Result<Self, Self::Error> {
        match orig.format {
            1 => {
                let coverage = reify_all(orig, Nullable(obj::CovTable))
                    .map(|raw| raw.as_ref().map(CoverageTable::promote))
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

pub type OpentypeRegionAxisCoordinates =
    opentype_common_variation_region_list_variation_regions_region_axes;
#[derive(Debug, Clone, Copy)]
struct RegionAxisCoordinates {
    start_coord: F2Dot14,
    peak_coord: F2Dot14,
    end_coord: F2Dot14,
}

impl Promote<OpentypeRegionAxisCoordinates> for RegionAxisCoordinates {
    fn promote(orig: &OpentypeRegionAxisCoordinates) -> Self {
        RegionAxisCoordinates {
            start_coord: F2Dot14::promote(&orig.start_coord),
            peak_coord: F2Dot14::promote(&orig.peak_coord),
            end_coord: F2Dot14::promote(&orig.end_coord),
        }
    }
}

pub type OpentypeVariationRegionList = opentype_common_variation_region_list;

#[derive(Debug, Clone, Default)]
struct VariationRegionList(Vec<Vec<RegionAxisCoordinates>>);

impl Promote<OpentypeVariationRegionList> for VariationRegionList {
    fn promote(orig: &OpentypeVariationRegionList) -> Self {
        let mut major_accum = Vec::with_capacity(orig.region_count as usize);
        for per_region in orig.variation_regions.iter() {
            let mut minor_accum = Vec::with_capacity(orig.axis_count as usize);
            for coords in per_region.region_axes.iter() {
                minor_accum.push(RegionAxisCoordinates::promote(coords));
            }
            major_accum.push(minor_accum);
        }
        Self(major_accum)
    }
}

pub type OpentypeDeltaSets = opentype_common_item_variation_data_delta_sets;

impl Promote<OpentypeDeltaSets> for DeltaSets {
    fn promote(orig: &OpentypeDeltaSets) -> Self {
        match orig {
            OpentypeDeltaSets::Delta32Sets(delta32s) => DeltaSets::Delta32Sets(
                delta32s
                    .iter()
                    .map(|delta_set| {
                        (
                            delta_set
                                .delta_data_full_word
                                .iter()
                                .map(|u: &u32| *u as i32)
                                .collect(),
                            delta_set
                                .delta_data_half_word
                                .iter()
                                .map(|u: &u16| *u as i16)
                                .collect(),
                        )
                    })
                    .collect(),
            ),
            OpentypeDeltaSets::Delta16Sets(delta16s) => DeltaSets::Delta16Sets(
                delta16s
                    .iter()
                    .map(|delta_set| {
                        (
                            delta_set
                                .delta_data_full_word
                                .iter()
                                .map(|u: &u16| *u as i16)
                                .collect(),
                            delta_set
                                .delta_data_half_word
                                .iter()
                                .map(|u: &u8| *u as i8)
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        }
    }
}

pub type Deltas<Full, Half> = (Vec<Full>, Vec<Half>);

#[derive(Debug, Clone)]
enum DeltaSets {
    Delta16Sets(Vec<Deltas<i16, i8>>),
    Delta32Sets(Vec<Deltas<i32, i16>>),
}

pub type OpentypeItemVariationData = opentype_common_item_variation_data;

impl Promote<OpentypeItemVariationData> for ItemVariationData {
    fn promote(orig: &OpentypeItemVariationData) -> Self {
        ItemVariationData {
            item_count: orig.item_count,
            long_words: orig.word_delta_count.long_words,
            word_count: orig.word_delta_count.word_count,
            region_index_count: orig.region_index_count,
            region_indices: orig.region_indices.clone(),
            delta_sets: DeltaSets::promote(&orig.delta_sets),
        }
    }
}

#[derive(Debug, Clone)]
struct ItemVariationData {
    item_count: u16,
    long_words: bool,
    word_count: u16,
    region_index_count: u16,
    region_indices: Vec<u16>,
    delta_sets: DeltaSets,
}

pub type OpentypeItemVariationStore<'a> = opentype_common_item_variation_store<'a>;

frame!(OpentypeItemVariationStore);

impl<'input> container::SingleContainer<Nullable<obj::VarRegList>>
    for OpentypeItemVariationStore<'input>
{
    fn get_offset(&self) -> usize {
        self.variation_region_list.offset as usize
    }

    fn get_args(&self) -> <obj::VarRegList as container::CommonObject>::Args<'_> {}
}

impl<'input> container::DynContainer<Nullable<obj::ItemVarData>>
    for OpentypeItemVariationStore<'input>
{
    fn count(&self) -> usize {
        self.item_variation_data_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.item_variation_data_list
            .iter()
            .map(|x| x.offset as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <obj::ItemVarData as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

impl<'a> Promote<OpentypeItemVariationStore<'a>> for ItemVariationStore {
    fn promote(orig: &OpentypeItemVariationStore) -> Self {
        let variation_region_list = promote_from_null(&reify(orig, Nullable(obj::VarRegList)));
        let item_variation_data_list = reify_all(orig, Nullable(obj::ItemVarData))
            .map(|raw| promote_link(&raw))
            .collect();
        ItemVariationStore {
            variation_region_list,
            item_variation_data_list,
        }
    }
}

// STUB - this represents the fact that we only record but do not interpret the offset for the ItemVariationStore in the current OpenType implementation
#[derive(Clone, Debug)]
struct ItemVariationStore {
    variation_region_list: VariationRegionList,
    item_variation_data_list: Vec<Option<ItemVariationData>>,
}

// Helper for dummy promotions for unit model-types from arbitrary fixed preimage-types
// NOTE - we can't implement generically (e.g. over all `T`) since those might clash with derived instances
macro_rules! promote_to_unit {
    ( $( $x:ty ),+ $(,)? ) => {
        $(
            impl Promote<$x> for () { fn promote(_orig: &$x) {} }
        )*
    };
}

promote_to_unit! {
    (),
    u16,
}

pub type OpentypeGdefData1Dot2<'a> = opentype_gdef_table_data_Version1_2<'a>;

impl<'a> container::SingleContainer<Nullable<obj::MarkGlSet>> for OpentypeGdefData1Dot2<'a> {
    fn get_offset(&self) -> usize {
        self.mark_glyph_sets_def.offset as usize
    }

    fn get_args(&self) -> () {}
}
pub type OpentypeGdefData1Dot3<'a> = opentype_gdef_table_data_Version1_3<'a>;

impl<'a> container::SingleContainer<Nullable<obj::MarkGlSet>> for OpentypeGdefData1Dot3<'a> {
    fn get_offset(&self) -> usize {
        self.mark_glyph_sets_def.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> container::SingleContainer<Nullable<obj::ItemVarStore>> for OpentypeGdefData1Dot3<'a> {
    fn get_offset(&self) -> usize {
        self.item_var_store.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> TryPromoteView<OpentypeGdefTableData<'a>> for GdefTableDataMetrics {
    type Error<'input>
        = ReflType<TPErr<OpentypeMarkGlyphSet<'input>, MarkGlyphSet>, UnknownValueError<u16>>
    where
        'a: 'input;

    fn try_promote_view<'input>(
        orig: &'input OpentypeGdefTableData<'a>,
        view: View<'input>,
    ) -> Result<Self, ValueParseError<Self::Error<'input>>>
    where
        'a: 'input,
    {
        Ok(match orig {
            OpentypeGdefTableData::Version1_0 => Self::default(),
            OpentypeGdefTableData::Version1_2(inner) => {
                let mark_glyph_sets_def =
                    try_promote_opt(&reify_dep(view, inner, Nullable(obj::MarkGlSet))?)
                        .map_err(ValueParseError::value)?;
                GdefTableDataMetrics {
                    mark_glyph_sets_def,
                    item_var_store: None,
                }
            }
            OpentypeGdefTableData::Version1_3(inner) => {
                let mark_glyph_sets_def =
                    try_promote_opt(&reify_dep(view, inner, Nullable(obj::MarkGlSet))?)
                        .map_err(ValueParseError::value)?;
                let item_var_store =
                    promote_opt(&reify_dep(view, inner, Nullable(obj::ItemVarStore))?);

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
pub(crate) struct RangeRecord<T> {
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
        match *orig {
            OpentypeClassDefData::Format1(OpentypeClassDefFormat1 {
                start_glyph_id,
                ref class_value_array,
                ..
            }) => ClassDef::Format1 {
                start_glyph_id,
                class_value_array: class_value_array.clone(),
            },
            OpentypeClassDefData::Format2(OpentypeClassDefFormat2 {
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

type OpentypeAttachList<'a> = opentype_gdef_attach_list<'a>;

frame!(OpentypeAttachList.list_scope);

impl<'a> container::SingleContainer<obj::CovTable> for OpentypeAttachList<'a> {
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> container::DynContainer<obj::AttPoint> for OpentypeAttachList<'a> {
    fn count(&self) -> usize {
        self.attach_point_offsets.len()
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.attach_point_offsets
            .iter()
            .map(|offset| offset.offset as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <obj::AttPoint as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

impl<'a> Promote<OpentypeAttachList<'a>> for AttachList {
    fn promote(orig: &OpentypeAttachList) -> Self {
        let coverage = CoverageTable::promote(&reify(orig, obj::CovTable));
        let attach_points = reify_all(orig, obj::AttPoint)
            .map(|raw| AttachPoint::promote(&raw))
            .collect();
        AttachList {
            coverage,
            attach_points,
        }
    }
}

#[derive(Clone, Debug)]
struct LigCaretList {
    coverage: CoverageTable,
    lig_glyphs: Vec<LigGlyph>,
}

type OpentypeLigCaretList<'a> = opentype_gdef_lig_caret_list<'a>;

frame!(OpentypeLigCaretList.list_scope);

impl container::SingleContainer<obj::CovTable> for OpentypeLigCaretList<'_> {
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> container::DynContainer<Nullable<obj::LigGlyph>> for OpentypeLigCaretList<'a> {
    fn count(&self) -> usize {
        self.lig_glyph_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.lig_glyph_offsets
            .iter()
            .map(|offs| offs.offset as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <obj::LigGlyph as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

impl<'a> TryPromote<OpentypeLigCaretList<'a>> for LigCaretList {
    type Error = ReflType<TPErr<OpentypeLigGlyph<'a>, LigGlyph>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeLigCaretList) -> Result<Self, Self::Error> {
        let coverage = CoverageTable::promote(&reify(orig, obj::CovTable));
        let mut lig_glyphs = Vec::with_capacity(container::DynContainer::count(orig));
        for opt_raw in reify_all(orig, Nullable(obj::LigGlyph)) {
            lig_glyphs.push(try_promote_from_null(&opt_raw)?);
        }
        Ok(LigCaretList {
            coverage,
            lig_glyphs,
        })
    }
}

#[derive(Clone, Debug, Default)]
struct LigGlyph {
    caret_values: Vec<Link<CaretValue>>,
}

pub type OpentypeLigGlyph<'a> = opentype_gdef_lig_glyph<'a>;

frame!(OpentypeLigGlyph);

impl<'a> container::DynContainer<Nullable<obj::CaretVal>> for OpentypeLigGlyph<'a> {
    fn count(&self) -> usize {
        self.caret_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.caret_values.iter().map(|offs| offs.offset as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <obj::CaretVal as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

impl<'a> TryPromote<OpentypeLigGlyph<'a>> for LigGlyph {
    type Error = ReflType<TPErr<OpentypeCaretValueData<'a>, CaretValue>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeLigGlyph) -> Result<Self, Self::Error> {
        let mut caret_values = Vec::with_capacity(container::DynContainer::count(orig));
        for opt_raw in reify_all(orig, Nullable(obj::CaretVal)) {
            caret_values.push(try_promote_opt(&opt_raw)?); // &caret_value.data
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
/// ```ignore
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
        i_raw - range_max
    } else {
        i_raw
    }
}

impl TryFromRef<(u16, Vec<u16>)> for DeltaValues {
    type Error = Local<UnknownValueError<u16>>;

    fn try_from_ref(value: (u16, &Vec<u16>)) -> Result<Self, Self::Error> {
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
    Device(DeviceTable),
    VariationIndex(VariationIndexTable),
    NonStandard { delta_format: u16 },
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

type OpentypeCaretValue<'a> = opentype_gdef_caret_value<'a>;
type OpentypeCaretValueData<'a> = opentype_gdef_caret_value_data<'a>;
type OpentypeCaretValueFormat1 = opentype_gdef_caret_value_data_Format1;
type OpentypeCaretValueFormat2 = opentype_gdef_caret_value_data_Format2;
type OpentypeCaretValueFormat3<'a> = opentype_gdef_caret_value_data_format3<'a>;

type OpentypeDeviceOrVariationIndexTable = opentype_common_device_or_variation_index_table;
type OpentypeDeviceTable = opentype_common_device_or_variation_index_table_DeviceTable;
type OpentypeVariationIndexTable =
    opentype_common_device_or_variation_index_table_VariationIndexTable;
type OpentypeDVIOtherTable = opentype_common_device_or_variation_index_table_OtherTable;

impl TryPromote<OpentypeDeviceOrVariationIndexTable> for DeviceOrVariationIndexTable {
    type Error = ReflType<TFRErr<(u16, Vec<u16>), DeltaValues>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeDeviceOrVariationIndexTable) -> Result<Self, Self::Error> {
        match *orig {
            OpentypeDeviceOrVariationIndexTable::DeviceTable(OpentypeDeviceTable {
                start_size,
                end_size,
                delta_format,
                ref delta_values,
            }) => Ok(DeviceOrVariationIndexTable::Device(DeviceTable {
                start_size,
                end_size,
                delta_values: DeltaValues::try_from_ref((delta_format, delta_values))?,
            })),
            OpentypeDeviceOrVariationIndexTable::VariationIndexTable(
                OpentypeVariationIndexTable {
                    delta_set_outer_index,
                    delta_set_inner_index,
                    ..
                },
            ) => Ok(DeviceOrVariationIndexTable::VariationIndex(
                VariationIndexTable {
                    delta_set_outer_index,
                    delta_set_inner_index,
                },
            )),
            OpentypeDeviceOrVariationIndexTable::OtherTable(OpentypeDVIOtherTable {
                delta_format,
                ..
            }) => Ok(DeviceOrVariationIndexTable::NonStandard { delta_format }),
        }
    }
}

impl<'a> TryPromote<OpentypeCaretValue<'a>> for CaretValue {
    type Error = ReflType<TPErr<OpentypeCaretValueData<'a>, CaretValue>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeCaretValue) -> Result<Self, Self::Error> {
        Self::try_promote(&orig.data)
    }
}

frame!(OpentypeCaretValueFormat3);

impl<'a> container::SingleContainer<Nullable<obj::DevTable>> for OpentypeCaretValueFormat3<'a> {
    fn get_offset(&self) -> usize {
        self.table.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> TryPromote<OpentypeCaretValueData<'a>> for CaretValue {
    type Error = ReflType<
        TPErr<OpentypeDeviceOrVariationIndexTable, DeviceOrVariationIndexTable>,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeCaretValueData) -> Result<Self, Self::Error> {
        match orig {
            OpentypeCaretValueData::Format1(OpentypeCaretValueFormat1 { coordinate }) => {
                Ok(CaretValue::DesignUnits(*coordinate))
            }
            OpentypeCaretValueData::Format2(OpentypeCaretValueFormat2 {
                caret_value_point_index,
            }) => Ok(CaretValue::ContourPoint(*caret_value_point_index)),
            OpentypeCaretValueData::Format3(format3) => {
                let device = try_promote_opt(&reify(format3, Nullable(obj::DevTable)))?;
                Ok(CaretValue::DesignUnitsWithTable {
                    coordinate: format3.coordinate,
                    device,
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
pub type OpentypeLangSys = opentype_layout_langsys;

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

pub type OpentypeLangSysRecord = opentype_layout_lang_sys_record;

impl container::SingleContainer<Nullable<obj::LangSys>> for OpentypeLangSysRecord {
    fn get_offset(&self) -> usize {
        self.lang_sys.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl PromoteView<OpentypeLangSysRecord> for LangSysRecord {
    fn promote_view(orig: &OpentypeLangSysRecord, view: View<'_>) -> PResult<Self> {
        Ok(LangSysRecord {
            lang_sys_tag: Tag(orig.lang_sys_tag),
            lang_sys: promote_opt(&reify_dep(view, orig, Nullable(obj::LangSys))?),
        })
    }
}

#[derive(Clone, Debug)]
struct LangSysRecord {
    lang_sys_tag: Tag,
    lang_sys: Link<LangSys>,
}

pub type OpentypeScriptTable<'input> = opentype_layout_script_table<'input>;

frame!(OpentypeScriptTable.script_scope);

impl<'input> container::SingleContainer<Nullable<obj::LangSys>> for OpentypeScriptTable<'input> {
    fn get_offset(&self) -> usize {
        self.default_lang_sys.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> Promote<OpentypeScriptTable<'input>> for ScriptTable {
    fn promote(orig: &OpentypeScriptTable) -> Self {
        ScriptTable {
            default_lang_sys: promote_opt(&reify(orig, Nullable(obj::LangSys))),
            lang_sys_records: promote_vec_view(&orig.lang_sys_records, orig.script_scope)
                .expect("failed to parse"),
        }
    }
}

#[derive(Clone, Debug)]
struct ScriptTable {
    default_lang_sys: Option<LangSys>,
    lang_sys_records: Vec<LangSysRecord>,
}

pub type OpentypeScriptRecord<'input> = opentype_layout_script_record<'input>;

impl<'input> container::SingleContainer<Nullable<obj::ScrTable>> for OpentypeScriptRecord<'input> {
    fn get_offset(&self) -> usize {
        self.script.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> PromoteView<OpentypeScriptRecord<'input>> for ScriptRecord {
    fn promote_view(orig: &OpentypeScriptRecord, view: View<'_>) -> PResult<Self> {
        Ok(ScriptRecord {
            script_tag: Tag(orig.script_tag),
            script: promote_opt(&reify_dep(view, orig, Nullable(obj::ScrTable))?),
        })
    }
}

#[derive(Clone, Debug)]
struct ScriptRecord {
    script_tag: Tag,
    script: Link<ScriptTable>,
}

pub type OpentypeFeatureTable<'input> = opentype_layout_feature_table<'input>;

impl<'input> Promote<OpentypeFeatureTable<'input>> for FeatureTable {
    fn promote(orig: &OpentypeFeatureTable<'input>) -> Self {
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

pub type OpentypeFeatureRecord<'input> = opentype_layout_feature_record<'input>;

impl<'input> container::SingleContainer<obj::FeatTable> for OpentypeFeatureRecord<'input> {
    fn get_offset(&self) -> usize {
        self.feature.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> PromoteView<OpentypeFeatureRecord<'input>> for FeatureRecord {
    fn promote_view(orig: &OpentypeFeatureRecord, view: View<'_>) -> PResult<Self> {
        Ok(FeatureRecord {
            feature_tag: Tag(orig.feature_tag),
            feature: FeatureTable::promote(&reify_dep(view, orig, obj::FeatTable)?),
        })
    }
}

#[derive(Clone, Debug)]
struct FeatureRecord {
    feature_tag: Tag,
    feature: FeatureTable,
}

pub type OpentypeGposLookupSubtableExt<'input> = opentype_gpos_lookup_subtable<'input>;
pub type OpentypeGsubLookupSubtableExt<'input> = opentype_gsub_lookup_subtable<'input>;

pub type OpentypeSubstExtension<'input> = opentype_layout_subst_extension<'input>;
pub type OpentypePosExtension<'input> = opentype_layout_pos_extension<'input>;

pub type OpentypeGposLookupSubtable<'input> = opentype_layout_ground_pos<'input>;
pub type OpentypeGsubLookupSubtable<'input> = opentype_layout_ground_subst<'input>;

#[derive(Debug)]
pub enum BadExtensionError {
    InconsistentLookup(u16, u16),
}

impl std::fmt::Display for BadExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadExtensionError::InconsistentLookup(expected, actual) => {
                write!(
                    f,
                    "layout extension subtable has inconsistent extension_lookup_type (expecting {expected}, found {actual})"
                )
            }
        }
    }
}

impl std::error::Error for BadExtensionError {}

impl<'input> TryPromote<OpentypeGsubLookupSubtableExt<'input>> for LookupSubtable {
    type Error = ReflType<
        ReflType<
            TPErr<OpentypeSubstExtension<'input>, LookupSubtable>,
            TPErr<OpentypeGsubLookupSubtable<'input>, LookupSubtable>,
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

frame!(OpentypeSubstExtension);

impl<'input> container::SingleContainer<obj::SubstLookup> for OpentypeSubstExtension<'input> {
    fn get_offset(&self) -> usize {
        self.extension_offset.offset as usize
    }

    fn get_args(&self) -> u16 {
        self.extension_lookup_type
    }
}

impl<'input> TryPromote<OpentypeSubstExtension<'input>> for LookupSubtable {
    type Error = ReflType<
        TPErr<OpentypeGsubLookupSubtable<'input>, LookupSubtable>,
        std::convert::Infallible,
    >;

    fn try_promote(orig: &OpentypeSubstExtension) -> Result<Self, Self::Error> {
        let raw = reify_index(orig, obj::SubstLookup, 0);
        LookupSubtable::try_promote(&raw)
    }
}

impl<'input> TryPromote<OpentypeGsubLookupSubtable<'input>> for LookupSubtable {
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

impl<'input> TryPromote<OpentypeGposLookupSubtableExt<'input>> for LookupSubtable {
    type Error = ReflType<
        ReflType<
            TPErr<OpentypeGposLookupSubtable<'input>, LookupSubtable>,
            TPErr<OpentypePosExtension<'input>, LookupSubtable>,
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

frame!(OpentypePosExtension);

impl<'input> container::SingleContainer<obj::PosLookup> for OpentypePosExtension<'input> {
    fn get_offset(&self) -> usize {
        self.extension_offset.offset as usize
    }

    fn get_args(&self) -> u16 {
        self.extension_lookup_type
    }
}

impl<'input> TryPromote<OpentypePosExtension<'input>> for LookupSubtable {
    type Error =
        ReflType<TPErr<OpentypeGposLookupSubtable<'input>, LookupSubtable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePosExtension) -> Result<Self, Self::Error> {
        let ground = reify_index(orig, obj::PosLookup, 0);
        LookupSubtable::try_promote(&ground)
    }
}

impl<'input> TryPromote<OpentypeGposLookupSubtable<'input>> for LookupSubtable {
    type Error = refl::ReflType7<
        TPErr<OpentypeSinglePos<'input>, SinglePos>,
        TPErr<OpentypePairPos<'input>, PairPos>,
        TPErr<OpentypeCursivePos<'input>, CursivePos>,
        TPErr<OpentypeMarkBasePos<'input>, MarkBasePos>,
        TPErr<OpentypeMarkLigPos<'input>, MarkLigPos>,
        TPErr<OpentypeMarkMarkPos<'input>, MarkMarkPos>,
        UnknownValueError<u16>,
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

pub type OpentypeMarkMarkPos<'input> = opentype_layout_mark_mark_pos<'input>;

frame!(OpentypeMarkMarkPos);

impl<'input> container::MultiContainer<obj::CovTable, 2> for OpentypeMarkMarkPos<'input> {
    fn get_offset_array(&self) -> [usize; 2] {
        [
            self.mark1_coverage.offset as usize,
            self.mark2_coverage.offset as usize,
        ]
    }

    fn get_args_array(&self) -> [(); 2] {
        [(); 2]
    }
}

impl<'input> container::SingleContainer<Nullable<obj::MarkArr>> for OpentypeMarkMarkPos<'input> {
    fn get_offset(&self) -> usize {
        self.mark1_array.offset as usize
    }

    fn get_args(&self) -> <obj::MarkArr as container::CommonObject>::Args<'_> {}
}

impl<'input> container::SingleContainer<Nullable<obj::Mark2Arr>> for OpentypeMarkMarkPos<'input> {
    fn get_offset(&self) -> usize {
        self.mark2_array.offset as usize
    }

    fn get_args(&self) -> <obj::Mark2Arr as container::CommonObject>::Args<'_> {
        self.mark_class_count
    }
}

impl<'input> OpentypeMarkMarkPos<'input> {
    pub(crate) fn mark1_array(&self) -> Result<MarkArray, UnknownValueError<u16>> {
        let raw = reify(self, Nullable(obj::MarkArr));
        try_promote_from_null(&raw)
    }

    pub(crate) fn mark2_array(&self) -> Result<Mark2Array, UnknownValueError<u16>> {
        let raw = reify(self, Nullable(obj::Mark2Arr));
        try_promote_from_null(&raw)
    }
}

impl<'input> TryPromote<OpentypeMarkMarkPos<'input>> for MarkMarkPos {
    type Error = ReflType<
        ReflType<
            TPErr<OpentypeMarkArray<'input>, MarkArray>,
            TPErr<OpentypeMark2Array<'input>, Mark2Array>,
        >,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeMarkMarkPos) -> Result<Self, Self::Error> {
        Ok(MarkMarkPos {
            mark1_coverage: CoverageTable::promote(&reify_index(orig, obj::CovTable, 0)),
            mark2_coverage: CoverageTable::promote(&reify_index(orig, obj::CovTable, 1)),
            mark1_array: orig.mark1_array()?,
            mark2_array: orig.mark2_array()?,
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

pub type OpentypeMark2Array<'input> = opentype_layout_mark2_array<'input>;

impl<'input> OpentypeMark2Array<'input> {
    pub(crate) fn mark2_records(&self) -> Result<Vec<Mark2Record>, UnknownValueError<u16>> {
        let mut records = Vec::with_capacity(self.mark2_records.len());
        for mark2_record in self.mark2_records.iter() {
            let record = Mark2Record::try_promote_view(mark2_record, self.array_scope)
                .map_err(ValueParseError::coerce_value)?;
            records.push(record);
        }
        Ok(records)
    }
}

impl<'input> TryPromote<OpentypeMark2Array<'input>> for Mark2Array {
    type Error =
        ReflType<TPVErr<'input, OpentypeMark2Record<'input>, Mark2Record>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeMark2Array) -> Result<Self, Self::Error> {
        Ok(Mark2Array {
            mark2_records: orig.mark2_records()?,
        })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub(crate) struct Mark2Array {
    mark2_records: Vec<Mark2Record>,
}

pub type OpentypeMark2Record<'input> = opentype_layout_mark2_array_mark2_record<'input>;

impl<'a> TryPromoteView<OpentypeMark2Record<'a>> for Mark2Record {
    type Error<'input>
        = ReflType<TPErr<OpentypeAnchorTable<'input>, AnchorTable>, UnknownValueError<u16>>
    where
        OpentypeMark2Record<'a>: 'input;

    fn try_promote_view<'input>(
        orig: &'input OpentypeMark2Record<'a>,
        view: View<'input>,
    ) -> Result<Self, ValueParseError<Self::Error<'input>>>
    where
        'a: 'input,
    {
        let mut mark2_anchors = Vec::with_capacity(orig.mark2_anchor_offsets.len());
        for &offset in orig.mark2_anchor_offsets.iter() {
            if offset == 0 {
                mark2_anchors.push(None);
            } else {
                let view = view.offset(offset as usize)?;
                let mut p = Parser::from(view);
                let ret = Decoder_opentype_layout_anchor_table(&mut p)?;
                mark2_anchors.push(Some(
                    AnchorTable::try_promote(&ret).map_err(ValueParseError::value)?,
                ))
            }
        }
        Ok(Mark2Record { mark2_anchors })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub(crate) struct Mark2Record {
    mark2_anchors: Vec<Option<AnchorTable>>,
}

pub type OpentypeMarkLigPos<'input> = opentype_layout_mark_lig_pos<'input>;

impl<'input> container::ViewFrame<'input> for OpentypeMarkLigPos<'input> {
    fn scope(&self) -> View<'input> {
        self.table_scope
    }
}

impl<'input> container::MultiContainer<obj::CovTable, 2> for OpentypeMarkLigPos<'input> {
    fn get_offset_array(&self) -> [usize; 2] {
        [
            self.mark_coverage.offset as usize,
            self.ligature_coverage.offset as usize,
        ]
    }

    fn get_args_array(&self) -> [(); 2] {
        [(), ()]
    }
}

impl<'input> container::SingleContainer<Nullable<obj::MarkArr>> for OpentypeMarkLigPos<'input> {
    fn get_offset(&self) -> usize {
        self.mark_array.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> container::SingleContainer<Nullable<obj::LigArr>> for OpentypeMarkLigPos<'input> {
    fn get_offset(&self) -> usize {
        self.ligature_array.offset as usize
    }

    fn get_args(&self) -> u16 {
        self.mark_class_count
    }
}

impl<'input> TryPromote<OpentypeMarkLigPos<'input>> for MarkLigPos {
    type Error = ReflType<TPErr<OpentypeMarkArray<'input>, MarkArray>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeMarkLigPos) -> Result<Self, Self::Error> {
        let mark_coverage = CoverageTable::promote(&reify_index(orig, obj::CovTable, 0));
        let ligature_coverage = CoverageTable::promote(&reify_index(orig, obj::CovTable, 1));
        let mark_array = try_promote_from_null(&reify(orig, Nullable(obj::MarkArr)))?;
        let ligature_array = try_promote_from_null(&reify(orig, Nullable(obj::LigArr)))?;
        Ok(MarkLigPos {
            mark_coverage,
            ligature_coverage,
            mark_array,
            ligature_array,
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

pub type OpentypeLigatureArray<'input> = opentype_layout_ligature_array<'input>;

frame!(OpentypeLigatureArray.array_scope);

impl<'input> container::DynContainer<obj::LigAtt> for OpentypeLigatureArray<'input> {
    fn count(&self) -> usize {
        self.ligature_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.ligature_attach_offsets.iter().map(|o| *o as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <obj::LigAtt as container::CommonObject>::Args<'_>> {
        std::iter::repeat(self.mark_class_count)
    }
}

impl<'input> TryPromote<OpentypeLigatureArray<'input>> for LigatureArray {
    type Error =
        ReflType<TPErr<OpentypeLigatureAttach<'input>, LigatureAttach>, UnknownValueError<u16>>;
    fn try_promote(orig: &OpentypeLigatureArray) -> Result<Self, Self::Error> {
        let mut ligature_attach = Vec::with_capacity(container::DynContainer::count(orig));
        for raw in reify_all(orig, obj::LigAtt) {
            ligature_attach.push(LigatureAttach::try_promote(&raw)?);
        }
        Ok(LigatureArray { ligature_attach })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct LigatureArray {
    ligature_attach: Vec<LigatureAttach>,
}

pub type OpentypeLigatureAttach<'input> = opentype_layout_ligature_attach<'input>;

impl<'input> TryPromote<OpentypeLigatureAttach<'input>> for LigatureAttach {
    type Error =
        ReflType<TPErr<OpentypeComponentRecord<'input>, ComponentRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeLigatureAttach) -> Result<Self, Self::Error> {
        let component_records = try_promote_vec(&orig.component_records)?;
        Ok(LigatureAttach { component_records })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct LigatureAttach {
    component_records: Vec<ComponentRecord>,
}

pub type OpentypeComponentRecord<'input> = opentype_layout_ligature_attach_component_record<'input>;

frame!(OpentypeComponentRecord.record_scope);

impl<'input> container::DynContainer<Nullable<obj::AncTable>> for OpentypeComponentRecord<'input> {
    fn count(&self) -> usize {
        self.ligature_anchor_offsets.len()
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.ligature_anchor_offsets
            .iter()
            .map(|offs| *offs as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <obj::AncTable as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

impl<'input> TryPromote<OpentypeComponentRecord<'input>> for ComponentRecord {
    type Error = ReflType<TPErr<OpentypeAnchorTable<'input>, AnchorTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeComponentRecord) -> Result<Self, Self::Error> {
        let mut ligature_anchors = Vec::with_capacity(orig.ligature_anchor_offsets.len());
        for raw in reify_all(orig, Nullable(obj::AncTable)) {
            ligature_anchors.push(raw.as_ref().map(AnchorTable::try_promote).transpose()?);
        }

        Ok(ComponentRecord { ligature_anchors })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct ComponentRecord {
    ligature_anchors: Vec<Option<AnchorTable>>,
}

pub type OpentypeMarkBasePos<'input> = opentype_layout_mark_base_pos<'input>;

frame!(OpentypeMarkBasePos);

impl<'input> container::MultiContainer<obj::CovTable, 2> for OpentypeMarkBasePos<'input> {
    fn get_offset_array(&self) -> [usize; 2] {
        [
            self.mark_coverage.offset as usize,
            self.base_coverage.offset as usize,
        ]
    }

    fn get_args_array(&self) -> [(); 2] {
        [(); 2]
    }
}

impl<'input> container::SingleContainer<Mandatory<obj::MarkArr>> for OpentypeMarkBasePos<'input> {
    fn get_offset(&self) -> usize {
        self.mark_array.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> container::SingleContainer<Mandatory<obj::BaseArr>> for OpentypeMarkBasePos<'input> {
    fn get_offset(&self) -> usize {
        self.base_array.offset as usize
    }

    fn get_args(&self) -> u16 {
        self.mark_class_count
    }
}

impl<'input> TryPromote<OpentypeMarkBasePos<'input>> for MarkBasePos {
    type Error = ReflType<
        ReflType<
            TPErr<OpentypeBaseArray<'input>, BaseArray>,
            TPErr<OpentypeMarkArray<'input>, MarkArray>,
        >,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeMarkBasePos) -> Result<Self, Self::Error> {
        Ok(MarkBasePos {
            mark_coverage: CoverageTable::promote(&reify_index(orig, obj::CovTable, 0)),
            base_coverage: CoverageTable::promote(&reify_index(orig, obj::CovTable, 1)),
            mark_array: MarkArray::try_promote(&reify(orig, Mandatory(obj::MarkArr)))?,
            base_array: BaseArray::try_promote(&reify(orig, Mandatory(obj::BaseArr)))?,
        })
    }
}

pub type OpentypeMarkArray<'input> = opentype_layout_mark_array<'input>;

frame!(OpentypeMarkArray.array_scope);

impl<'input> container::DynContainer<Nullable<obj::AncTable>> for OpentypeMarkArray<'input> {
    fn count(&self) -> usize {
        self.mark_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.mark_records
            .iter()
            .map(|mark_rec| mark_rec.mark_anchor.offset as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <Nullable<obj::AncTable> as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

impl<'input> TryPromote<OpentypeMarkArray<'input>> for MarkArray {
    type Error = ReflType<TPErr<OpentypeAnchorTable<'input>, AnchorTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeMarkArray) -> Result<Self, Self::Error> {
        let mut mark_records = Vec::with_capacity(orig.mark_records.len());
        let links = reify_all(orig, Nullable(obj::AncTable));
        for (record, raw_anchor) in Iterator::zip(orig.mark_records.iter(), links) {
            let mark_class = record.mark_class;
            let mark_anchor = raw_anchor
                .as_ref()
                .map(AnchorTable::try_promote)
                .transpose()?;
            mark_records.push(MarkRecord {
                mark_class,
                mark_anchor,
            });
        }
        Ok(MarkArray { mark_records })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub(crate) struct MarkArray {
    mark_records: Vec<MarkRecord>,
}

pub type OpentypeMarkRecord<'input> = opentype_layout_mark_record<'input>;

// impl TryPromote<OpentypeMarkRecord> for MarkRecord {
//     type Error = ReflType<TPErr<OpentypeAnchorTable, AnchorTable>, UnknownValueError<u16>>;

//     fn try_promote(orig: &OpentypeMarkRecord) -> Result<Self, Self::Error> {
//         Ok(MarkRecord {
//             mark_class: orig.mark_class,
//             mark_anchor: try_promote_link(&orig.mark_anchor_offset.link)?,
//         })
//     }
// }

#[derive(Debug, Clone)]
struct MarkRecord {
    mark_class: u16,
    mark_anchor: Link<AnchorTable>,
}

pub type OpentypeBaseArray<'input> = opentype_layout_base_array<'input>;

impl<'input> TryPromote<OpentypeBaseArray<'input>> for BaseArray {
    type Error =
        ReflType<TPVErr<'input, OpentypeBaseRecord<'input>, BaseRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeBaseArray) -> Result<Self, Self::Error> {
        let mut base_records = Vec::with_capacity(orig.base_records.len());

        for base_record in orig.base_records.iter() {
            base_records.push(
                BaseRecord::try_promote_view(base_record, orig.array_scope)
                    .map_err(ValueParseError::coerce_value)?,
            );
        }
        Ok(BaseArray { base_records })
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct BaseArray {
    base_records: Vec<BaseRecord>,
}

pub type OpentypeBaseRecord<'input> = opentype_layout_base_array_base_record<'input>;

impl<'input> container::DynContainer<Nullable<obj::AncTable>> for OpentypeBaseRecord<'input> {
    fn count(&self) -> usize {
        self.base_anchor_offsets.len()
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.base_anchor_offsets
            .iter()
            .map(|offs: &u16| *offs as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat_n((), self.count())
    }
}

impl<'a> TryPromoteView<OpentypeBaseRecord<'a>> for BaseRecord {
    type Error<'input>
        = ReflType<TPErr<OpentypeAnchorTable<'a>, AnchorTable>, UnknownValueError<u16>>
    where
        'a: 'input;

    fn try_promote_view<'input>(
        orig: &'input OpentypeBaseRecord<'a>,
        view: View<'input>,
    ) -> Result<Self, ValueParseError<Self::Error<'input>>>
    where
        'a: 'input,
    {
        use container::DynContainer;
        let mut base_anchors =
            Vec::with_capacity(DynContainer::<Nullable<obj::AncTable>>::count(orig));
        for res_raw in reify_all_dep(view, orig, Nullable(obj::AncTable)) {
            let raw = res_raw?;
            base_anchors.push(try_promote_opt(&raw).map_err(ValueParseError::value)?);
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

pub type OpentypeReverseChainSingleSubst<'a> = opentype_layout_reverse_chain_single_subst<'a>;

frame!(OpentypeReverseChainSingleSubst);

impl<'a> container::MultiDynContainer<Mandatory<obj::CovTable>, 3>
    for OpentypeReverseChainSingleSubst<'a>
{
    fn counts(&self) -> [usize; 3] {
        [
            1,
            self.backtrack_glyph_count as usize,
            self.lookahead_glyph_count as usize,
        ]
    }

    fn iter_offsets_at_index(&self, ix: usize) -> impl Iterator<Item = usize> {
        match ix {
            0 => Box::new(std::iter::once(self.coverage.offset as usize))
                as Box<dyn Iterator<Item = usize>>,
            1 => Box::new(
                self.backtrack_coverage_tables
                    .iter()
                    .map(|offs| offs.offset as usize),
            ),
            2 => Box::new(
                self.lookahead_coverage_tables
                    .iter()
                    .map(|offs| offs.offset as usize),
            ),
            _ => unreachable!("bad index {ix}"),
        }
    }

    fn iter_args_at_index(
        &self,
        _: usize,
    ) -> impl Iterator<Item = <Mandatory<obj::CovTable> as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

impl<'a> Promote<OpentypeReverseChainSingleSubst<'a>> for ReverseChainSingleSubst {
    fn promote(orig: &OpentypeReverseChainSingleSubst) -> Self {
        let coverage = CoverageTable::promote(
            &reify_all_index(orig, Mandatory(obj::CovTable), 0)
                .next()
                .unwrap(),
        );
        let backtrack_coverages = reify_all_index(orig, Mandatory(obj::CovTable), 1)
            .map(|raw| CoverageTable::promote(&raw))
            .collect();
        let lookahead_coverages = reify_all_index(orig, Mandatory(obj::CovTable), 2)
            .map(|raw| CoverageTable::promote(&raw))
            .collect();
        ReverseChainSingleSubst {
            coverage,
            backtrack_coverages,
            lookahead_coverages,
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

pub type OpentypeLigatureSubst<'a> = opentype_layout_ligature_subst<'a>;

frame!(OpentypeLigatureSubst);

impl<'a> container::SingleContainer<Mandatory<obj::CovTable>> for OpentypeLigatureSubst<'a> {
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> container::DynContainer<Mandatory<obj::LigSet>> for OpentypeLigatureSubst<'a> {
    fn count(&self) -> usize {
        self.ligature_set_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.ligature_sets.iter().map(|offs| offs.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'a> Promote<OpentypeLigatureSubst<'a>> for LigatureSubst {
    fn promote(orig: &OpentypeLigatureSubst) -> Self {
        let coverage = CoverageTable::promote(&reify(orig, Mandatory(obj::CovTable)));
        let ligature_sets = reify_all(orig, Mandatory(obj::LigSet))
            .map(|raw| LigatureSet::promote(&raw))
            .collect();
        LigatureSubst {
            coverage,
            ligature_sets,
        }
    }
}

#[derive(Debug, Clone)]
struct LigatureSubst {
    coverage: CoverageTable,
    ligature_sets: Vec<LigatureSet>,
}

pub type OpentypeLigatureSet<'a> = opentype_gsub_ligature_subst_ligature_set<'a>;

frame!(OpentypeLigatureSet.set_scope);

impl<'a> container::DynContainer<Mandatory<obj::LigTable>> for OpentypeLigatureSet<'a> {
    fn count(&self) -> usize {
        self.ligature_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.ligatures.iter().map(|offs| offs.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

#[derive(Debug, Clone, Default)]
struct LigatureSet {
    ligatures: Vec<Ligature>,
}

pub type OpentypeLigature = opentype_gsub_ligature_subst_ligature_table;

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

impl<'a> Promote<OpentypeLigatureSet<'a>> for LigatureSet {
    fn promote(orig: &OpentypeLigatureSet) -> Self {
        let ligatures = reify_all(orig, Mandatory(obj::LigTable))
            .map(|raw| Ligature::promote(&raw))
            .collect();
        LigatureSet { ligatures }
    }
}

pub type OpentypeAlternateSubst<'a> = opentype_gsub_alternate_subst<'a>;

frame!(OpentypeAlternateSubst);

impl<'a> container::SingleContainer<Mandatory<obj::CovTable>> for OpentypeAlternateSubst<'a> {
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> <Mandatory<obj::CovTable> as container::CommonObject>::Args<'_> {
        ()
    }
}

impl<'a> container::DynContainer<Mandatory<obj::AltSet>> for OpentypeAlternateSubst<'a> {
    fn count(&self) -> usize {
        self.alternate_set_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.alternate_sets.iter().map(|offs| offs.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'a> Promote<OpentypeAlternateSubst<'a>> for AlternateSubst {
    fn promote(orig: &OpentypeAlternateSubst) -> Self {
        AlternateSubst {
            coverage: CoverageTable::promote(&reify(orig, Mandatory(obj::CovTable))),
            alternate_sets: reify_all(orig, Mandatory(obj::AltSet))
                .map(|raw| AlternateSet::promote(&raw))
                .collect(),
        }
    }
}

pub type OpentypeAlternateSet = opentype_gsub_alternate_subst_alternate_set;

#[derive(Clone, Debug, Default)]
struct AlternateSet {
    // REVIEW - we can implement FromNull for AlternateSet perhaps
    alternate_glyph_ids: Vec<u16>,
}

impl Promote<OpentypeAlternateSet> for AlternateSet {
    fn promote(orig: &OpentypeAlternateSet) -> Self {
        AlternateSet {
            alternate_glyph_ids: orig.alternate_glyph_ids.clone(),
        }
    }
}

#[derive(Clone, Debug)]
struct AlternateSubst {
    coverage: CoverageTable,
    alternate_sets: Vec<AlternateSet>,
}

pub type OpentypeMultipleSubst<'input> = opentype_layout_multiple_subst<'input>;
pub type OpentypeMultipleSubstInner = opentype_layout_multiple_subst_subst;
pub type OpentypeMultipleSubstFormat1 = opentype_layout_multiple_subst_subst_Format1;

frame!(OpentypeMultipleSubst);

impl<'input> container::SingleContainer<Mandatory<obj::CovTable>>
    for OpentypeMultipleSubst<'input>
{
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl container::DynContainer<obj::SeqTable> for OpentypeMultipleSubstFormat1 {
    fn count(&self) -> usize {
        self.sequence_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.sequence_offsets.iter().map(|offset| *offset as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <obj::SeqTable as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

impl<'input> Promote<OpentypeMultipleSubst<'input>> for MultipleSubst {
    fn promote(orig: &OpentypeMultipleSubst) -> Self {
        MultipleSubst {
            coverage: CoverageTable::promote(&reify(orig, Mandatory(obj::CovTable))),
            subst: MultipleSubstInner::promote_view(&orig.subst, orig.table_scope).unwrap(),
        }
    }
}

impl PromoteView<OpentypeMultipleSubstInner> for MultipleSubstInner {
    fn promote_view(orig: &OpentypeMultipleSubstInner, view: View<'_>) -> PResult<Self> {
        match orig {
            OpentypeMultipleSubstInner::Format1(f1) => MultipleSubstFormat1::promote_view(f1, view),
        }
    }
}

impl PromoteView<OpentypeMultipleSubstFormat1> for MultipleSubstFormat1 {
    fn promote_view(orig: &OpentypeMultipleSubstFormat1, view: View<'_>) -> PResult<Self> {
        let mut sequences =
            Vec::with_capacity(container::DynContainer::<obj::SeqTable>::count(orig));
        for res_raw in reify_all_dep(view, orig, obj::SeqTable) {
            sequences.push(SequenceTable::promote(&res_raw?));
        }
        Ok(MultipleSubstFormat1 { sequences })
    }
}

#[derive(Debug, Clone)]
struct MultipleSubst {
    coverage: CoverageTable,
    subst: MultipleSubstInner,
}

type MultipleSubstInner = MultipleSubstFormat1; // FIXME - nominally an enum but only one variant so we inline

pub type OpentypeSequenceTable = opentype_layout_multiple_subst_sequence_table;

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

pub type OpentypeSingleSubst<'input> = opentype_layout_single_subst<'input>;
pub type OpentypeSingleSubstInner<'input> = opentype_layout_single_subst_subst<'input>;
pub type OpentypeSingleSubstFormat1<'input> = opentype_layout_single_subst_format1<'input>;
pub type OpentypeSingleSubstFormat2<'input> = opentype_layout_single_subst_format2<'input>;

impl<'input> Promote<OpentypeSingleSubst<'input>> for SingleSubst {
    fn promote(orig: &OpentypeSingleSubst) -> Self {
        SingleSubst::promote(&orig.subst)
    }
}

impl<'input> Promote<OpentypeSingleSubstInner<'input>> for SingleSubst {
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

frame!(OpentypeSingleSubstFormat1);

impl<'input> container::MultiContainer<obj::CovTable, 1> for OpentypeSingleSubstFormat1<'input> {
    fn get_offset_array(&self) -> [usize; 1] {
        [self.coverage.offset as usize]
    }

    fn get_args_array(&self) -> [<obj::CovTable as container::CommonObject>::Args<'_>; 1] {
        [()]
    }
}

impl<'input> Promote<OpentypeSingleSubstFormat1<'input>> for SingleSubstFormat1 {
    fn promote(orig: &OpentypeSingleSubstFormat1) -> Self {
        SingleSubstFormat1 {
            coverage: CoverageTable::promote(&reify_index(orig, obj::CovTable, 0)),
            delta_glyph_id: orig.delta_glyph_id,
        }
    }
}

#[derive(Debug, Clone)]
struct SingleSubstFormat1 {
    coverage: CoverageTable,
    delta_glyph_id: s16,
}

frame!(OpentypeSingleSubstFormat2);

impl<'input> container::MultiContainer<obj::CovTable, 1> for OpentypeSingleSubstFormat2<'input> {
    fn get_offset_array(&self) -> [usize; 1] {
        [self.coverage.offset as usize]
    }

    fn get_args_array(&self) -> [<obj::CovTable as container::CommonObject>::Args<'_>; 1] {
        [()]
    }
}

impl<'input> Promote<OpentypeSingleSubstFormat2<'input>> for SingleSubstFormat2 {
    fn promote(orig: &OpentypeSingleSubstFormat2) -> Self {
        SingleSubstFormat2 {
            coverage: CoverageTable::promote(&reify_index(orig, obj::CovTable, 0)),
            substitute_glyph_ids: orig.substitute_glyph_ids.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct SingleSubstFormat2 {
    coverage: CoverageTable,
    substitute_glyph_ids: Vec<u16>,
}

// SECTION - chained sequence context lookup

// REVIEW - should this be a compartmentalized submodule?
pub type OpentypeChainedSequenceContext<'a> = opentype_layout_chained_sequence_context<'a>;
pub type OpentypeChainedSequenceContextInner<'a> =
    opentype_layout_chained_sequence_context_subst<'a>;
pub type OpentypeChainedSequenceContextFormat1<'a> =
    opentype_layout_chained_sequence_context_format1<'a>;
pub type OpentypeChainedSequenceContextFormat2<'a> =
    opentype_layout_chained_sequence_context_format2<'a>;
pub type OpentypeChainedSequenceContextFormat3 = opentype_layout_chained_sequence_context_format3;

pub type OpentypeChainedRuleSet<'a> = opentype_layout_chained_sequence_rule_set<'a>;
pub type OpentypeChainedRule = opentype_layout_chained_sequence_rule;

frame!(OpentypeChainedSequenceContext);

frame!(OpentypeChainedRuleSet);
impl<'a> container::DynContainer<Mandatory<obj::ChainRule>> for OpentypeChainedRuleSet<'a> {
    fn count(&self) -> usize {
        self.chained_seq_rule_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.chained_seq_rules
            .iter()
            .map(|offset| offset.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'a, Sem> Promote<OpentypeChainedRuleSet<'a>> for ChainedRuleSet<Sem>
where
    ChainedRule<Sem>: Promote<OpentypeChainedRule>,
{
    fn promote(orig: &OpentypeChainedRuleSet) -> Self {
        reify_all(orig, Mandatory(obj::ChainRule))
            .map(|raw| ChainedRule::<Sem>::promote(&raw))
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

impl<'a> Promote<OpentypeChainedSequenceContext<'a>> for ChainedSequenceContext {
    fn promote(orig: &OpentypeChainedSequenceContext) -> Self {
        Self::promote_view(&orig.subst, container::ViewFrame::scope(orig)).expect("bad parse")
    }
}

impl<'a> PromoteView<OpentypeChainedSequenceContextInner<'a>> for ChainedSequenceContext {
    fn promote_view(orig: &OpentypeChainedSequenceContextInner, view: View<'_>) -> PResult<Self> {
        match orig {
            OpentypeChainedSequenceContextInner::Format1(f1) => {
                Ok(ChainedSequenceContext::Format1(
                    ChainedSequenceContextFormat1::promote_view(f1, view)?,
                ))
            }
            OpentypeChainedSequenceContextInner::Format2(f2) => {
                Ok(ChainedSequenceContext::Format2(
                    ChainedSequenceContextFormat2::promote_view(f2, view)?,
                ))
            }
            OpentypeChainedSequenceContextInner::Format3(f3) => {
                Ok(ChainedSequenceContext::Format3(
                    ChainedSequenceContextFormat3::promote_view(f3, view)?,
                ))
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

// SECTION - CHained Sequence Context Format 1
#[derive(Debug, Clone)]
struct ChainedSequenceContextFormat1 {
    coverage: CoverageTable,
    chained_seq_rule_sets: Vec<ChainedRuleSet<GlyphId>>,
}

impl<'a> container::SingleContainer<Mandatory<obj::CovTable>>
    for OpentypeChainedSequenceContextFormat1<'a>
{
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> container::DynContainer<Nullable<obj::ChainRuleSet>>
    for OpentypeChainedSequenceContextFormat1<'a>
{
    fn count(&self) -> usize {
        self.chained_seq_rule_set_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.chained_seq_rule_sets
            .iter()
            .map(|offs| offs.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'a> PromoteView<OpentypeChainedSequenceContextFormat1<'a>> for ChainedSequenceContextFormat1 {
    fn promote_view(orig: &OpentypeChainedSequenceContextFormat1, view: View<'_>) -> PResult<Self> {
        let coverage = CoverageTable::promote(&reify_dep(view, orig, Mandatory(obj::CovTable))?);
        let mut chained_seq_rule_sets = Vec::with_capacity(container::DynContainer::count(orig));
        for opt_raw in reify_all_dep(view, orig, Nullable(obj::ChainRuleSet)) {
            chained_seq_rule_sets.push(promote_from_null(&opt_raw?));
        }
        Ok(ChainedSequenceContextFormat1 {
            coverage,
            chained_seq_rule_sets,
        })
    }
}
// !SECTION

#[derive(Debug, Clone)]
struct ChainedSequenceContextFormat2 {
    coverage: CoverageTable,
    backtrack_class_def: ClassDef,
    input_class_def: ClassDef,
    lookahead_class_def: ClassDef,
    chained_class_seq_rule_sets: Vec<ChainedRuleSet<ClassId>>,
}

impl<'a> container::SingleContainer<Mandatory<obj::CovTable>>
    for OpentypeChainedSequenceContextFormat2<'a>
{
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> container::MultiContainer<Mandatory<obj::ClsDef>, 3>
    for OpentypeChainedSequenceContextFormat2<'a>
{
    fn get_offset_array(&self) -> [usize; 3] {
        [
            self.backtrack_class_def.offset as usize,
            self.input_class_def.offset as usize,
            self.lookahead_class_def.offset as usize,
        ]
    }

    fn get_args_array(&self) -> [(); 3] {
        [(); 3]
    }
}

impl<'a> container::DynContainer<Nullable<obj::ChainRuleSet>>
    for OpentypeChainedSequenceContextFormat2<'a>
{
    fn count(&self) -> usize {
        self.chained_class_seq_rule_set_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.chained_class_seq_rule_sets
            .iter()
            .map(|offs| offs.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'a> PromoteView<OpentypeChainedSequenceContextFormat2<'a>> for ChainedSequenceContextFormat2 {
    fn promote_view(orig: &OpentypeChainedSequenceContextFormat2, view: View<'_>) -> PResult<Self> {
        let coverage = CoverageTable::promote(&reify_dep(view, orig, Mandatory(obj::CovTable))?);
        let backtrack_class_def =
            ClassDef::promote(&reify_index_dep(view, orig, Mandatory(obj::ClsDef), 0)?);
        let input_class_def =
            ClassDef::promote(&reify_index_dep(view, orig, Mandatory(obj::ClsDef), 1)?);
        let lookahead_class_def =
            ClassDef::promote(&reify_index_dep(view, orig, Mandatory(obj::ClsDef), 2)?);
        let mut chained_class_seq_rule_sets =
            Vec::with_capacity(container::DynContainer::count(orig));
        for opt_raw in reify_all_dep(view, orig, Nullable(obj::ChainRuleSet)) {
            chained_class_seq_rule_sets.push(promote_from_null(&opt_raw?));
        }
        Ok(Self {
            coverage,
            backtrack_class_def,
            input_class_def,
            lookahead_class_def,
            chained_class_seq_rule_sets,
        })
    }
}

impl container::MultiDynContainer<Mandatory<obj::CovTable>, 3>
    for OpentypeChainedSequenceContextFormat3
{
    fn counts(&self) -> [usize; 3] {
        [
            self.backtrack_glyph_count as usize,
            self.input_glyph_count as usize,
            self.lookahead_glyph_count as usize,
        ]
    }

    fn iter_offsets_at_index(&self, ix: usize) -> impl Iterator<Item = usize> {
        let slice: &[_] = match ix {
            0 => &self.backtrack_coverages,
            1 => &self.input_coverages,
            2 => &self.lookahead_coverages,
            _ => unreachable!("bad index {ix}"),
        };
        slice.iter().map(|offs| offs.offset as usize)
    }

    fn iter_args_at_index(
        &self,
        _ix: usize,
    ) -> impl Iterator<Item = <Mandatory<obj::CovTable> as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

impl PromoteView<OpentypeChainedSequenceContextFormat3> for ChainedSequenceContextFormat3 {
    fn promote_view(orig: &OpentypeChainedSequenceContextFormat3, view: View<'_>) -> PResult<Self> {
        let follow = |ix: usize| {
            promote_all_ok(
                reify_all_index_dep(view, orig, Mandatory(obj::CovTable), ix),
                container::MultiDynContainer::counts(orig)[ix],
            )
        };
        let backtrack_coverages = follow(0)?;
        let input_coverages = follow(1)?;
        let lookahead_coverages = follow(2)?;
        Ok(Self {
            backtrack_coverages,
            input_coverages,
            lookahead_coverages,
            seq_lookup_records: orig.seq_lookup_records.clone(),
        })
    }
}

#[derive(Debug, Clone)]
struct ChainedSequenceContextFormat3 {
    backtrack_coverages: Vec<CoverageTable>,
    input_coverages: Vec<CoverageTable>,
    lookahead_coverages: Vec<CoverageTable>,
    seq_lookup_records: Vec<SequenceLookup>,
}
// !SECTION

// SECTION - sequence context lookup
pub type OpentypeSequenceContext<'a> = opentype_layout_sequence_context<'a>;
pub type OpentypeSequenceContextInner<'a> = opentype_layout_sequence_context_subst<'a>;

frame!(OpentypeSequenceContext);

impl<'a> Promote<OpentypeSequenceContext<'a>> for SequenceContext {
    fn promote(orig: &OpentypeSequenceContext) -> Self {
        // FIXME - if we rename the field `subst`, fix this
        SequenceContext::promote_view(&orig.subst, container::ViewFrame::scope(orig))
            .expect("bad parse")
    }
}

impl<'a> PromoteView<OpentypeSequenceContextInner<'a>> for SequenceContext {
    fn promote_view(orig: &OpentypeSequenceContextInner, view: View<'_>) -> PResult<Self> {
        Ok(match orig {
            OpentypeSequenceContextInner::Format1(f1) => {
                SequenceContext::Format1(SequenceContextFormat1::promote_view(f1, view)?)
            }
            OpentypeSequenceContextInner::Format2(f2) => {
                SequenceContext::Format2(SequenceContextFormat2::promote_view(f2, view)?)
            }
            OpentypeSequenceContextInner::Format3(f3) => {
                SequenceContext::Format3(SequenceContextFormat3::promote_view(f3, view)?)
            }
        })
    }
}

#[derive(Debug, Clone)]
enum SequenceContext {
    Format1(SequenceContextFormat1),
    Format2(SequenceContextFormat2),
    Format3(SequenceContextFormat3),
}

#[derive(Debug, Clone)]
struct SequenceContextFormat1 {
    coverage: CoverageTable,
    seq_rule_sets: Vec<RuleSet<GlyphId>>,
}

pub type OpentypeSequenceContextFormat1<'a> = opentype_layout_sequence_context_format1<'a>;

impl<'a> container::SingleContainer<Mandatory<obj::CovTable>>
    for OpentypeSequenceContextFormat1<'a>
{
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> container::DynContainer<Nullable<obj::SeqRuleSet>> for OpentypeSequenceContextFormat1<'a> {
    fn count(&self) -> usize {
        self.seq_rule_set_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.seq_rule_sets.iter().map(|offs| offs.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'a> PromoteView<OpentypeSequenceContextFormat1<'a>> for SequenceContextFormat1 {
    fn promote_view(orig: &OpentypeSequenceContextFormat1, view: View<'_>) -> PResult<Self> {
        let coverage = CoverageTable::promote(&reify_dep(view, orig, Mandatory(obj::CovTable))?);
        let mut seq_rule_sets = Vec::with_capacity(container::DynContainer::count(orig));
        for opt_raw in reify_all_dep(view, orig, Nullable(obj::SeqRuleSet)) {
            seq_rule_sets.push(promote_from_null(&opt_raw?))
        }
        Ok(Self {
            coverage,
            seq_rule_sets,
        })
    }
}

#[derive(Debug, Clone)]
struct SequenceContextFormat2 {
    coverage: CoverageTable,
    class_def: ClassDef,
    class_seq_rule_sets: Vec<RuleSet<ClassId>>,
}

pub type OpentypeSequenceContextFormat2<'a> = opentype_layout_sequence_context_format2<'a>;

impl<'a> container::SingleContainer<Mandatory<obj::CovTable>>
    for OpentypeSequenceContextFormat2<'a>
{
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> container::SingleContainer<Mandatory<obj::ClsDef>> for OpentypeSequenceContextFormat2<'a> {
    fn get_offset(&self) -> usize {
        self.class_def.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'a> container::DynContainer<Nullable<obj::SeqRuleSet>> for OpentypeSequenceContextFormat2<'a> {
    fn count(&self) -> usize {
        self.class_seq_rule_set_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.class_seq_rule_sets
            .iter()
            .map(|offs| offs.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'a> PromoteView<OpentypeSequenceContextFormat2<'a>> for SequenceContextFormat2 {
    fn promote_view(orig: &OpentypeSequenceContextFormat2, view: View<'_>) -> PResult<Self> {
        let coverage = CoverageTable::promote(&reify_dep(view, orig, Mandatory(obj::CovTable))?);
        let class_def = ClassDef::promote(&reify_dep(view, orig, Mandatory(obj::ClsDef))?);
        let mut class_seq_rule_sets = Vec::with_capacity(container::DynContainer::count(orig));
        for opt_raw in reify_all_dep(view, orig, Nullable(obj::SeqRuleSet)) {
            class_seq_rule_sets.push(promote_from_null(&opt_raw?))
        }
        Ok(Self {
            coverage,
            class_def,
            class_seq_rule_sets,
        })
    }
}

pub type OpentypeSequenceLookup = opentype_layout_sequence_lookup;
type SequenceLookup = OpentypeSequenceLookup;

#[derive(Debug, Clone)]
struct SequenceContextFormat3 {
    glyph_count: u16,
    coverage_tables: Vec<CoverageTable>,
    seq_lookup_records: Vec<SequenceLookup>,
}

pub type OpentypeSequenceContextFormat3 = opentype_layout_sequence_context_format3;

impl container::DynContainer<Mandatory<obj::CovTable>> for OpentypeSequenceContextFormat3 {
    fn count(&self) -> usize {
        self.glyph_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.coverage_tables.iter().map(|offs| offs.offset as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <Mandatory<obj::CovTable> as container::CommonObject>::Args<'_>> {
        std::iter::repeat(())
    }
}

impl PromoteView<OpentypeSequenceContextFormat3> for SequenceContextFormat3 {
    fn promote_view(orig: &OpentypeSequenceContextFormat3, view: View<'_>) -> PResult<Self> {
        let coverage_tables = promote_all_ok(
            reify_all_dep(view, orig, Mandatory(obj::CovTable)),
            container::DynContainer::count(orig),
        )?;
        Ok(Self {
            glyph_count: orig.glyph_count,
            coverage_tables,
            // NOTE - can only clone here (instead of calling promote) because SequenceLookup := OpentypeSequenceLookup
            seq_lookup_records: orig.seq_lookup_records.clone(),
        })
    }
}

pub type OpentypeRuleSet<'a> = opentype_layout_sequence_context_rule_set<'a>;

frame!(OpentypeRuleSet);

impl<'a> container::DynContainer<Mandatory<obj::SeqRule>> for OpentypeRuleSet<'a> {
    fn count(&self) -> usize {
        self.rule_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.rules.iter().map(|offs| offs.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'a, Sem> Promote<OpentypeRuleSet<'a>> for Vec<Rule<Sem>> {
    fn promote(orig: &OpentypeRuleSet<'a>) -> Self {
        let mut accum = Vec::with_capacity(container::DynContainer::count(orig));
        for raw in reify_all(orig, Mandatory(obj::SeqRule)) {
            accum.push(Rule::promote(&raw))
        }
        accum
    }
}

type RuleSet<Sem> = Vec<Rule<Sem>>;

pub type OpentypeRule = opentype_layout_sequence_context_rule;

impl<Sem> Promote<OpentypeRule> for Rule<Sem> {
    fn promote(orig: &OpentypeRule) -> Self {
        Rule {
            glyph_count: orig.glyph_count,
            input_sequence: SemVec::from(orig.input_sequence.clone()),
            // NOTE - we can only specify seq_lookup_records this way because we use SequenceLookup as its own analogue
            seq_lookup_records: orig.seq_lookup_records.clone(),
        }
    }
}

struct Rule<Sem> {
    glyph_count: u16, // REVIEW - this field can be re-synthesized via `input_sequence.len() + 1`
    input_sequence: SemVec<Sem, u16>,
    seq_lookup_records: Vec<SequenceLookup>,
}

impl<Sem> Clone for Rule<Sem> {
    fn clone(&self) -> Self {
        Self {
            glyph_count: self.glyph_count,
            input_sequence: self.input_sequence.clone(),
            seq_lookup_records: self.seq_lookup_records.clone(),
        }
    }
}

impl<Sem> std::fmt::Debug for Rule<Sem>
where
    SemVec<Sem, u16>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("glyph_count", &self.glyph_count)
            .field("input_sequence", &self.input_sequence)
            .field("seq_lookup_records", &self.seq_lookup_records)
            .finish()
    }
}
// !SECTION

pub type OpentypeCursivePos<'input> = opentype_layout_cursive_pos<'input>;

impl<'input> container::ViewFrame<'input> for OpentypeCursivePos<'input> {
    fn scope(&self) -> View<'input> {
        self.table_scope
    }
}

impl<'input> container::SingleContainer<Mandatory<obj::CovTable>> for OpentypeCursivePos<'input> {
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> TryPromote<OpentypeCursivePos<'input>> for CursivePos {
    type Error = ReflType<
        TPVErr<'input, OpentypeEntryExitRecord<'input>, EntryExitRecord>,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeCursivePos) -> Result<Self, Self::Error> {
        let mut entry_exit_records = Vec::with_capacity(orig.entry_exit_records.len());
        for entry_exit in orig.entry_exit_records.iter() {
            entry_exit_records.push(
                EntryExitRecord::try_promote_view(entry_exit, orig.table_scope)
                    .map_err(ValueParseError::coerce_value)?,
            );
        }
        Ok(CursivePos {
            coverage: CoverageTable::promote(&reify(orig, Mandatory(obj::CovTable))),
            entry_exit_records,
        })
    }
}

#[derive(Debug, Clone)]
struct CursivePos {
    coverage: CoverageTable,
    entry_exit_records: Vec<EntryExitRecord>,
}

pub type OpentypeEntryExitRecord<'input> = opentype_layout_entry_exit_record<'input>;

impl<'input> container::MultiContainer<Nullable<obj::AncTable>, 2>
    for OpentypeEntryExitRecord<'input>
{
    fn get_args_array(&self) -> [(); 2] {
        [(); 2]
    }

    fn get_offset_array(&self) -> [usize; 2] {
        [
            self.entry_anchor.offset as usize,
            self.exit_anchor.offset as usize,
        ]
    }
}

impl<'a> TryPromoteView<OpentypeEntryExitRecord<'a>> for EntryExitRecord {
    type Error<'input>
        = ReflType<TPErr<OpentypeAnchorTable<'a>, AnchorTable>, UnknownValueError<u16>>
    where
        'a: 'input;

    fn try_promote_view<'input>(
        orig: &'input OpentypeEntryExitRecord<'a>,
        view: View<'input>,
    ) -> Result<Self, ValueParseError<Self::Error<'input>>>
    where
        'a: 'input,
    {
        Ok(EntryExitRecord {
            entry_anchor: try_promote_opt(&reify_index_dep(
                view,
                orig,
                Nullable(obj::AncTable),
                0,
            )?)
            .map_err(ValueParseError::value)?,
            exit_anchor: try_promote_opt(&reify_index_dep(view, orig, Nullable(obj::AncTable), 1)?)
                .map_err(ValueParseError::value)?,
        })
    }
}

#[derive(Debug, Clone)]
struct EntryExitRecord {
    entry_anchor: Option<AnchorTable>,
    exit_anchor: Option<AnchorTable>,
}

pub type OpentypeAnchorTable<'input> = opentype_layout_anchor_table<'input>;
pub type OpentypeAnchorTableTable<'input> = opentype_layout_anchor_table_table<'input>;

pub type OpentypeAnchorTableFormat1 = opentype_layout_anchor_table_table_Format1;
pub type OpentypeAnchorTableFormat2 = opentype_layout_anchor_table_table_Format2;
pub type OpentypeAnchorTableFormat3<'input> = opentype_layout_anchor_table_format3<'input>;

impl<'input> TryPromote<OpentypeAnchorTable<'input>> for AnchorTable {
    type Error =
        ReflType<TPErr<OpentypeAnchorTableTable<'input>, AnchorTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeAnchorTable) -> Result<Self, Self::Error> {
        AnchorTable::try_promote(&orig.table)
    }
}

impl<'input> TryPromote<OpentypeAnchorTableTable<'input>> for AnchorTable {
    type Error = ReflType<
        TPErr<OpentypeAnchorTableFormat3<'input>, AnchorTableFormat3>,
        UnknownValueError<u16>,
    >;

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

impl<'input> TryPromote<OpentypeAnchorTableFormat3<'input>> for AnchorTableFormat3 {
    type Error = ReflType<
        TPErr<OpentypeDeviceOrVariationIndexTable, DeviceOrVariationIndexTable>,
        UnknownValueError<u16>,
    >;

    fn try_promote(orig: &OpentypeAnchorTableFormat3) -> Result<Self, Self::Error> {
        fn expand(
            scope: View<'_>,
            offset: u16,
        ) -> Result<Option<DeviceOrVariationIndexTable>, UnknownValueError<u16>> {
            if offset == 0 {
                Ok(None)
            } else {
                let view = scope
                    .offset(offset as usize)
                    .expect("bad offset in anchor table");
                let mut p = Parser::from(view);
                let ret = Decoder_opentype_common_device_or_variation_index_table(&mut p)
                    .expect("bad device table parse");
                Ok(Some(DeviceOrVariationIndexTable::try_promote(&ret)?))
            }
        }
        let x_device = expand(orig.table_scope, orig.x_device.offset)?;
        let y_device = expand(orig.table_scope, orig.y_device.offset)?;
        Ok(AnchorTableFormat3 {
            x_coordinate: orig.x_coordinate,
            y_coordinate: orig.y_coordinate,
            x_device,
            y_device,
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

pub type OpentypePairPos<'input> = opentype_layout_pair_pos<'input>;

pub type OpentypePairPosSubtable<'input> = opentype_layout_pair_pos_subtable<'input>;
pub type OpentypePairPosFormat1<'input> = opentype_layout_pair_pos_format1<'input>;
pub type OpentypePairPosFormat2<'input> = opentype_layout_pair_pos_format2<'input>;

impl<'input> TryPromote<OpentypePairPos<'input>> for PairPos {
    type Error = ReflType<TPErr<OpentypePairPosSubtable<'input>, PairPos>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePairPos) -> Result<Self, Self::Error> {
        Self::try_promote(&orig.subtable)
    }
}

impl<'input> TryPromote<OpentypePairPosSubtable<'input>> for PairPos {
    type Error = ReflType<
        ReflType<
            TPErr<OpentypePairPosFormat1<'input>, PairPosFormat1>,
            TPErr<OpentypePairPosFormat2<'input>, PairPosFormat2>,
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

frame!(OpentypePairPosFormat1);

impl<'a> container::SingleContainer<Mandatory<obj::CovTable>> for OpentypePairPosFormat1<'a> {
    fn get_offset(&self) -> usize {
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> TryPromote<OpentypePairPosFormat1<'input>> for PairPosFormat1 {
    type Error = ReflType<TPErr<OpentypePairSet<'input>, PairSet>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePairPosFormat1) -> Result<Self, Self::Error> {
        let coverage = CoverageTable::promote(&reify(orig, Mandatory(obj::CovTable)));
        // FIXME - implement container traits instead of open-coding parse logic
        let mut pair_sets = Vec::with_capacity(orig.pair_sets.len());
        for pair_set in orig.pair_sets.iter() {
            if pair_set.offset == 0 {
                pair_sets.push(PairSet::from_null());
            } else {
                let view = orig
                    .table_scope
                    .offset(pair_set.offset as usize)
                    .expect("bad offset to PairSet");
                let mut p = Parser::from(view);
                let ret = Decoder_opentype_layout_pair_pos_pair_set(
                    &mut p,
                    orig.value_format1,
                    orig.value_format2,
                )
                .expect("bad PairSet parse");
                pair_sets.push(PairSet::try_promote(&ret)?)
            }
        }

        Ok(PairPosFormat1 {
            coverage,
            pair_sets,
        })
    }
}

impl<'input> container::ViewFrame<'input> for OpentypePairPosFormat2<'input> {
    fn scope(&self) -> View<'input> {
        self.table_scope
    }
}

impl<'input> container::MultiContainer<obj::CovTable, 1> for OpentypePairPosFormat2<'input> {
    fn get_offset_array(&self) -> [usize; 1] {
        // REVIEW[epic=hardcoded-assumption] - based on current implementation, `self.coverage` is an ad-hoc record type with a single u16 `offset` field
        [self.coverage.offset as usize]
    }

    fn get_args_array(&self) -> [(); 1] {
        [()]
    }
}

impl<'input> container::MultiContainer<obj::ClsDef, 2> for OpentypePairPosFormat2<'input> {
    fn get_offset_array(&self) -> [usize; 2] {
        // REVIEW[epic=hardcoded-assumption] - based on current implementation, `self.class_def{1,2}` is an ad-hoc record type with a single u16 `offset` field
        [
            self.class_def1.offset as usize,
            self.class_def2.offset as usize,
        ]
    }

    fn get_args_array(&self) -> [(); 2] {
        [(); 2]
    }
}

impl<'input> TryPromote<OpentypePairPosFormat2<'input>> for PairPosFormat2 {
    type Error =
        ReflType<TPVErr<'input, OpentypeClass2Record, Class2Record>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePairPosFormat2) -> Result<Self, Self::Error> {
        let mut store = Vec::with_capacity(orig.class1_count as usize * orig.class2_count as usize);

        for class1_record in orig.class1_records.iter() {
            for class2_record in class1_record.class2_records.iter() {
                store.push(
                    Class2Record::try_promote_view(class2_record, orig.table_scope)
                        .map_err(ValueParseError::coerce_value)?,
                );
            }
        }
        let class1_records = Wec::from_vec(store, orig.class2_count as usize);

        Ok(PairPosFormat2 {
            coverage: CoverageTable::promote(&reify_index(orig, obj::CovTable, 0)),
            class_def1: ClassDef::promote(&reify_index(orig, obj::ClsDef, 0)),
            class_def2: ClassDef::promote(&reify_index(orig, obj::ClsDef, 1)),
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

pub type OpentypeClass2Record = opentype_layout_pair_pos_class2_record;

impl TryPromoteView<OpentypeClass2Record> for Class2Record {
    type Error<'input> =
        ReflType<TPVErr<'input, OpentypeValueRecord, ValueRecord>, UnknownValueError<u16>>;

    fn try_promote_view<'input>(
        orig: &'input OpentypeClass2Record,
        view: View<'input>,
    ) -> Result<Self, ValueParseError<Self::Error<'input>>>
    where
        OpentypeClass2Record: 'input,
    {
        Ok(Class2Record {
            value_record1: try_promote_opt_view(&orig.value_record1, view)?,
            value_record2: try_promote_opt_view(&orig.value_record2, view)?,
        })
    }
}

#[derive(Debug, Clone)]
struct Class2Record {
    value_record1: Option<ValueRecord>,
    value_record2: Option<ValueRecord>,
}

pub type OpentypePairSet<'input> = opentype_layout_pair_pos_pair_set<'input>;
pub type OpentypePairValueRecord = opentype_layout_pair_pos_pair_value_record;

type PairSet = Vec<PairValueRecord>;

impl<'input> container::ViewFrame<'input> for OpentypePairSet<'input> {
    fn scope(&self) -> View<'input> {
        self.set_scope
    }
}

impl<'input> TryPromote<OpentypePairSet<'input>> for PairSet {
    type Error =
        ReflType<TPVErr<'input, OpentypePairValueRecord, PairValueRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypePairSet) -> Result<Self, Self::Error> {
        let mut accum = Vec::with_capacity(orig.pair_value_records.len());
        for record in orig.pair_value_records.iter() {
            accum.push(
                PairValueRecord::try_promote_view(record, orig.set_scope)
                    .map_err(ValueParseError::coerce_value)?,
            );
        }
        Ok(accum)
    }
}

impl TryPromoteView<OpentypePairValueRecord> for PairValueRecord {
    type Error<'input> =
        ReflType<TPVErr<'input, OpentypeValueRecord, ValueRecord>, UnknownValueError<u16>>;

    fn try_promote_view<'input>(
        orig: &'input OpentypePairValueRecord,
        view: View<'input>,
    ) -> Result<Self, ValueParseError<Self::Error<'input>>>
    where
        OpentypePairValueRecord: 'input,
    {
        Ok(PairValueRecord {
            second_glyph: orig.second_glyph,
            value_record1: try_promote_opt_view(&orig.value_record1, view)?,
            value_record2: try_promote_opt_view(&orig.value_record2, view)?,
        })
    }
}

#[derive(Debug, Clone)]
struct PairValueRecord {
    second_glyph: u16,
    value_record1: Option<ValueRecord>,
    value_record2: Option<ValueRecord>,
}

pub type OpentypeSinglePos<'input> = opentype_layout_single_pos<'input>;
pub type OpentypeSinglePosSubtable<'input> = opentype_layout_single_pos_subtable<'input>;
pub type OpentypeSinglePosFormat1<'input> = opentype_layout_single_pos_format1<'input>;
pub type OpentypeSinglePosFormat2<'input> = opentype_layout_single_pos_format2<'input>;

impl<'input> TryPromote<OpentypeSinglePos<'input>> for SinglePos {
    type Error =
        ReflType<TPErr<OpentypeSinglePosSubtable<'input>, SinglePos>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeSinglePos) -> Result<Self, Self::Error> {
        Self::try_promote(&orig.subtable)
    }
}

impl<'input> TryPromote<OpentypeSinglePosSubtable<'input>> for SinglePos {
    type Error = ReflType<
        ReflType<
            TPErr<OpentypeSinglePosFormat1<'input>, SinglePosFormat1>,
            TPErr<OpentypeSinglePosFormat2<'input>, SinglePosFormat2>,
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

impl<'input> container::ViewFrame<'input> for OpentypeSinglePosFormat1<'input> {
    fn scope(&self) -> View<'input> {
        self.table_scope
    }
}

impl<'input> container::ViewFrame<'input> for OpentypeSinglePosFormat2<'input> {
    fn scope(&self) -> View<'input> {
        self.table_scope
    }
}

impl<'input> container::MultiContainer<obj::CovTable, 1> for OpentypeSinglePosFormat1<'input> {
    fn get_offset_array(&self) -> [usize; 1] {
        // REVIEW[epic=hardcoded-assumption] - `read_phantom_view_offset16` call in format def yields a `{ offset: u16 }` record
        [self.coverage.offset as usize]
    }

    fn get_args_array(&self) -> [(); 1] {
        [()]
    }
}

impl<'input> TryPromote<OpentypeSinglePosFormat1<'input>> for SinglePosFormat1 {
    type Error = ReflType<TPVErr<'input, OpentypeValueRecord, ValueRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeSinglePosFormat1) -> Result<Self, Self::Error> {
        Ok(SinglePosFormat1 {
            coverage: CoverageTable::promote(&reify_index(orig, obj::CovTable, 0)),
            value_record: ValueRecord::try_promote_view(&orig.value_record, orig.table_scope)
                .map_err(ValueParseError::coerce_value)?,
        })
    }
}

impl<'input> container::SingleContainer<obj::CovTable> for OpentypeSinglePosFormat2<'input> {
    fn get_offset(&self) -> usize {
        // REVIEW[epic=hardcoded-assumption] - `read_phantom_view_offset16` call in format def yields a `{ offset: u16 }` record
        self.coverage.offset as usize
    }

    fn get_args(&self) -> () {}
}

impl<'input> TryPromote<OpentypeSinglePosFormat2<'input>> for SinglePosFormat2 {
    type Error = ReflType<TPVErr<'input, OpentypeValueRecord, ValueRecord>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeSinglePosFormat2) -> Result<Self, Self::Error> {
        let mut value_records = Vec::with_capacity(orig.value_records.len());
        for value_record in orig.value_records.iter() {
            value_records.push(
                ValueRecord::try_promote_view(value_record, orig.table_scope)
                    .map_err(ValueParseError::coerce_value)?,
            );
        }
        Ok(SinglePosFormat2 {
            coverage: CoverageTable::promote(&reify_index(orig, obj::CovTable, 0)),
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

pub type OpentypeValueRecord = opentype_layout_value_record;

impl container::MultiOptContainer<Nullable<obj::DevTable>, 4> for OpentypeValueRecord {
    fn get_offset_at_index(&self, ix: usize) -> Option<usize> {
        let offs = match ix {
            Self::IX_X_PLACEMENT_DEVICE => &self.x_placement_device,
            Self::IX_Y_PLACEMENT_DEVICE => &self.y_placement_device,
            Self::IX_X_ADVANCE_DEVICE => &self.x_advance_device,
            Self::IX_Y_ADVANCE_DEVICE => &self.y_advance_device,
            // REVIEW - should this be a panic, or `return None`?
            _ => unreachable!("MultiOptContainer::get_offset_at_index: out-of-bounds ({ix} >= 4)"),
        };
        offs.as_ref().map(|o| o.offset as usize)
    }
}

impl OpentypeValueRecord {
    pub(crate) const IX_X_PLACEMENT_DEVICE: usize = 0;
    pub(crate) const IX_Y_PLACEMENT_DEVICE: usize = 1;
    pub(crate) const IX_X_ADVANCE_DEVICE: usize = 2;
    pub(crate) const IX_Y_ADVANCE_DEVICE: usize = 3;
}

impl TryPromoteView<OpentypeValueRecord> for ValueRecord {
    type Error<'input> = ReflType<
        TPErr<OpentypeDeviceOrVariationIndexTable, DeviceOrVariationIndexTable>,
        UnknownValueError<u16>,
    >;

    fn try_promote_view<'input>(
        orig: &'input OpentypeValueRecord,
        view: View<'input>,
    ) -> Result<Self, ValueParseError<Self::Error<'input>>>
    where
        OpentypeValueRecord: 'input,
    {
        // NOTE - we do not distinguish between omitted device-fields and included-but-zeroed device-fields
        let follow = |ix: usize| -> Result<
            Option<DeviceOrVariationIndexTable>,
            ValueParseError<Self::Error<'input>>,
        > {
            Ok(try_promote_opt(
                &reify_opt_index_dep(view, orig, Nullable(obj::DevTable), ix)
                    .transpose()?
                    .flatten(),
            )
            .map_err(ValueParseError::value)?)
        };
        Ok(ValueRecord {
            x_placement: orig.x_placement.map(as_s16),
            y_placement: orig.y_placement.map(as_s16),
            x_advance: orig.x_advance.map(as_s16),
            y_advance: orig.y_advance.map(as_s16),
            x_placement_device: follow(OpentypeValueRecord::IX_X_PLACEMENT_DEVICE)?,
            y_placement_device: follow(OpentypeValueRecord::IX_Y_PLACEMENT_DEVICE)?,
            x_advance_device: follow(OpentypeValueRecord::IX_X_ADVANCE_DEVICE)?,
            y_advance_device: follow(OpentypeValueRecord::IX_Y_ADVANCE_DEVICE)?,
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

type LookupFlag = opentype_gpos_lookup_table_lookup_flag;

pub type OpentypeGposLookupTable<'input> = opentype_gpos_lookup_table<'input>;
pub type OpentypeGsubLookupTable<'input> = opentype_gsub_lookup_table<'input>;

frame!(OpentypeGposLookupTable);

impl<'input> container::DynContainer<Mandatory<obj::PosSubtable>>
    for OpentypeGposLookupTable<'input>
{
    fn count(&self) -> usize {
        self.sub_table_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.subtables.iter().map(|offset| offset.offset as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <obj::PosSubtable as container::CommonObject>::Args<'_>> {
        std::iter::repeat(self.lookup_type)
    }
}

impl<'input> TryPromote<OpentypeGposLookupTable<'input>> for LookupTable {
    type Error =
        ReflType<TPErr<OpentypeGposLookupSubtable<'input>, LookupSubtable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeGposLookupTable) -> Result<Self, Self::Error> {
        let mut subtables = Vec::with_capacity(orig.subtables.len());
        const POS_EXTENSION_LOOKUP_TYPE: u16 = 9;
        let subtable_iter = reify_all(orig, Mandatory(obj::PosSubtable));

        let lookup_type = match orig.lookup_type {
            POS_EXTENSION_LOOKUP_TYPE => {
                let mut extension_lookup_type: Option<u16> = None;

                for (_ix, raw) in subtable_iter.enumerate() {
                    match &raw {
                        OpentypeGposLookupSubtableExt::PosExtension(ext) => {
                            if let Some(tmp) =
                                extension_lookup_type.replace(ext.extension_lookup_type)
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
                            subtables.push(LookupSubtable::try_promote(&raw)?);
                        }
                        _other => unreachable!(
                            "lookup type is PosExtension, found non-PosExtension subtable: {_other:?}"
                        ),
                    }
                }
                extension_lookup_type.unwrap_or(POS_EXTENSION_LOOKUP_TYPE)
            }
            ground_type => {
                for (_ix, raw) in subtable_iter.enumerate() {
                    let subtable = LookupSubtable::try_promote(&raw)?;
                    subtables.push(subtable);
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

frame!(OpentypeGsubLookupTable);

impl<'input> container::DynContainer<obj::SubstSubtable> for OpentypeGsubLookupTable<'input> {
    fn count(&self) -> usize {
        self.sub_table_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.subtables.iter().map(|offset| offset.offset as usize)
    }

    fn iter_args(
        &self,
    ) -> impl Iterator<Item = <obj::SubstSubtable as container::CommonObject>::Args<'_>> {
        std::iter::repeat(self.lookup_type)
    }
}

impl<'input> TryPromote<OpentypeGsubLookupTable<'input>> for LookupTable {
    type Error = ReflType<
        TPErr<OpentypeGsubLookupSubtable<'input>, LookupSubtable>,
        std::convert::Infallible, // for compatibility with GPOS promotion, can't use BadExtensionError as the error types would collide
    >;

    fn try_promote(orig: &OpentypeGsubLookupTable) -> Result<Self, Self::Error> {
        let mut subtables = Vec::with_capacity(orig.subtables.len());
        const SUBST_EXTENSION_LOOKUP_TYPE: u16 = 7;
        let subtable_iter = reify_all(orig, obj::SubstSubtable);

        let lookup_type = match orig.lookup_type {
            SUBST_EXTENSION_LOOKUP_TYPE => {
                let mut extension_lookup_type: Option<u16> = None;
                for (_ix, raw) in subtable_iter.enumerate() {
                    match &raw {
                        OpentypeGsubLookupSubtableExt::SubstExtension(ext) => {
                            if let Some(tmp) =
                                extension_lookup_type.replace(ext.extension_lookup_type)
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
                            subtables.push(LookupSubtable::try_promote(&raw)?);
                        }
                        _other => unreachable!(
                            "lookup type is SubstExtension, found non-SubstExtension subtable: {_other:?}"
                        ),
                    }
                }
                extension_lookup_type.unwrap_or(SUBST_EXTENSION_LOOKUP_TYPE)
            }
            ground_type => {
                for (_ix, raw) in subtable_iter.enumerate() {
                    let subtable = LookupSubtable::try_promote(&raw)?;
                    subtables.push(subtable);
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
type LookupList = Vec<LookupTable>;

pub type OpentypeScriptList<'input> = opentype_layout_script_list<'input>;
pub type OpentypeFeatureList<'input> = opentype_layout_feature_list<'input>;

frame!(OpentypeScriptList);

impl<'input> Promote<OpentypeScriptList<'input>> for ScriptList {
    fn promote(orig: &OpentypeScriptList) -> Self {
        promote_vec_view(&orig.script_records, orig.table_scope).expect("failed to parse")
    }
}

frame!(OpentypeFeatureList.list_scope);

impl<'input> Promote<OpentypeFeatureList<'input>> for FeatureList {
    fn promote(orig: &OpentypeFeatureList) -> Self {
        promote_vec_view(&orig.feature_records, orig.list_scope).expect("failed to parse")
    }
}

pub type OpentypeGposLookupList<'input> = opentype_gpos_lookup_list<'input>;
pub type OpentypeGsubLookupList<'input> = opentype_gsub_lookup_list<'input>;

frame!(OpentypeGposLookupList.list_scope);

impl<'input> container::DynContainer<Mandatory<obj::PosLookupTable>>
    for OpentypeGposLookupList<'input>
{
    fn count(&self) -> usize {
        self.lookup_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.lookups.iter().map(|lookup| lookup.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'input> TryPromote<OpentypeGposLookupList<'input>> for LookupList {
    type Error =
        ReflType<TPErr<OpentypeGposLookupTable<'input>, LookupTable>, UnknownValueError<u16>>;

    fn try_promote(orig: &OpentypeGposLookupList) -> Result<Self, Self::Error> {
        let mut accum = Vec::with_capacity(container::DynContainer::count(orig));
        for raw in reify_all(orig, Mandatory(obj::PosLookupTable)) {
            accum.push(LookupTable::try_promote(&raw)?);
        }
        Ok(accum)
    }
}

frame!(OpentypeGsubLookupList.list_scope);

impl<'input> container::DynContainer<Mandatory<obj::SubstLookupTable>>
    for OpentypeGsubLookupList<'input>
{
    fn count(&self) -> usize {
        self.lookup_count as usize
    }

    fn iter_offsets(&self) -> impl Iterator<Item = usize> {
        self.lookups.iter().map(|lookup| lookup.offset as usize)
    }

    fn iter_args(&self) -> impl Iterator<Item = ()> {
        std::iter::repeat(())
    }
}

impl<'input> TryPromote<OpentypeGsubLookupList<'input>> for LookupList {
    type Error = TPErr<OpentypeGsubLookupTable<'input>, LookupTable>;

    fn try_promote(orig: &OpentypeGsubLookupList) -> Result<Self, Self::Error> {
        let mut accum = Vec::with_capacity(container::DynContainer::count(orig));
        for (_ix, raw) in reify_all(orig, Mandatory(obj::SubstLookupTable)).enumerate() {
            accum.push(LookupTable::try_promote(&raw)?);
        }
        Ok(accum)
    }
}

pub type OpentypeFeatureVariations<'input> = opentype_layout_feature_variations<'input>;

impl<'input> Promote<OpentypeFeatureVariations<'input>> for FeatureVariations {
    fn promote(_orig: &OpentypeFeatureVariations) -> FeatureVariations {
        // STUB - implement proper promotion rules once feature variation type is refined
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
impl LayoutMetrics {
    fn promote_gpos(gpos: &OpentypeGpos<'_>) -> TestResult<Heap<LayoutMetrics>> {
        let script_list = ScriptList::promote(&reify(gpos, Mandatory(obj::ScrList)));
        let feature_list = FeatureList::promote(&reify(gpos, Mandatory(obj::FeatList)));
        let lookup_list = LookupList::try_promote(&reify(gpos, Mandatory(obj::PosLookups)))?;
        let feature_variations = promote_opt(&reify_opt(gpos, Nullable(obj::FeatVar)).flatten());
        Ok(Heap::new(Self {
            major_version: gpos.major_version,
            minor_version: gpos.minor_version,
            script_list,
            feature_list,
            lookup_list,
            feature_variations,
        }))
    }

    fn promote_gsub(gsub: &OpentypeGsub<'_>) -> TestResult<Heap<LayoutMetrics>> {
        let script_list = ScriptList::promote(&reify(gsub, Mandatory(obj::ScrList)));
        let feature_list = FeatureList::promote(&reify(gsub, Mandatory(obj::FeatList)));
        let lookup_list = LookupList::try_promote(&reify(gsub, Mandatory(obj::SubstLookups)))?;
        let feature_variations = promote_opt(&reify_opt(gsub, Nullable(obj::FeatVar)).flatten());
        Ok(Heap::new(Self {
            major_version: gsub.major_version,
            minor_version: gsub.minor_version,
            script_list,
            feature_list,
            lookup_list,
            feature_variations,
        }))
    }
}

#[derive(Clone, Debug)]
struct BaseMetrics {
    major_version: u16,
    minor_version: u16,
    // STUB - add more fields as desired
}

pub type OpentypeKernCoverage = opentype_kern_kern_subtable_coverage;

impl Promote<OpentypeKernCoverage> for KernFlags {
    fn promote(orig: &OpentypeKernCoverage) -> Self {
        KernFlags {
            r#override: orig.r#override,
            cross_stream: orig.cross_stream,
            minimum: orig.minimum,
            horizontal: orig.horizontal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct KernFlags {
    r#override: bool,
    cross_stream: bool,
    minimum: bool,
    horizontal: bool,
}

pub type OpentypeKernSubtable<'a> = opentype_kern_kern_subtable<'a>;

impl<'a> Promote<OpentypeKernSubtable<'a>> for KernSubtable {
    fn promote(orig: &OpentypeKernSubtable) -> Self {
        let flags = KernFlags::promote(&orig.coverage);
        let data = KernSubtableData::promote(&orig.data);
        KernSubtable { flags, data }
    }
}

#[derive(Debug, Clone)]
struct KernSubtable {
    flags: KernFlags,
    data: KernSubtableData,
}

pub type OpentypeKernPair = opentype_kern_subtable_format0_kern_pairs;

impl Promote<OpentypeKernPair> for KernPair {
    fn promote(orig: &OpentypeKernPair) -> Self {
        KernPair {
            left: orig.left,
            right: orig.right,
            value: as_s16(orig.value),
        }
    }
}

#[derive(Clone, Debug, Copy)]
struct KernPair {
    left: u16,
    right: u16,
    value: i16,
}

impl PartialEq for KernPair {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl Eq for KernPair {}

impl PartialOrd for KernPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KernPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this_key = ((self.left as u32) << 16) & (self.right as u32);
        let other_key = ((other.left as u32) << 16) & (other.right as u32);
        this_key.cmp(&other_key)
    }
}

pub type OpentypeKernSubtableFormat0 = opentype_kern_subtable_format0;

impl Promote<OpentypeKernSubtableFormat0> for KernSubtableFormat0 {
    fn promote(orig: &OpentypeKernSubtableFormat0) -> Self {
        KernSubtableFormat0 {
            kern_pairs: promote_vec(&orig.kern_pairs),
        }
    }
}
#[derive(Clone, Debug)]
#[repr(transparent)]
struct KernSubtableFormat0 {
    // REVIEW - is Vec the most apt container-type given that we know the array is sorted by left-right key?
    kern_pairs: Vec<KernPair>,
}

pub type OpentypeKernSubtableFormat2<'a> = opentype_kern_subtable_format2<'a>;

frame!(OpentypeKernSubtableFormat2);

impl<'a> container::MultiContainer<Mandatory<obj::KernCls>, 2> for OpentypeKernSubtableFormat2<'a> {
    fn get_offset_array(&self) -> [usize; 2] {
        [
            self.left_class_offset.offset as usize,
            self.right_class_offset.offset as usize,
        ]
    }

    fn get_args_array(&self) -> [(); 2] {
        [(); 2]
    }
}

impl<'a> OpentypeKernSubtableFormat2<'a> {
    fn n_glyphs_in_class(view: View<'_>, offset: usize) -> u16 {
        assert!(offset != 0);
        let field_shift = size_of::<u16>();

        let start_of_field = view
            .offset(offset as usize + field_shift)
            .expect("bad offset");
        let value = start_of_field.read_u16be().expect("bad value");
        value
    }

    pub fn left_glyph_count(&self) -> u16 {
        Self::n_glyphs_in_class(self.table_scope, self.left_class_offset.offset as usize)
    }

    pub fn right_glyph_count(&self) -> u16 {
        Self::n_glyphs_in_class(self.table_scope, self.right_class_offset.offset as usize)
    }
}

impl<'a> container::SingleContainer<Mandatory<obj::KernArr>> for OpentypeKernSubtableFormat2<'a> {
    fn get_offset(&self) -> usize {
        self.kerning_array_offset.offset as usize
    }

    fn get_args(&self) -> (u16, u16) {
        (self.left_glyph_count(), self.right_glyph_count())
    }
}

impl<'a> Promote<OpentypeKernSubtableFormat2<'a>> for KernSubtableFormat2 {
    fn promote(orig: &OpentypeKernSubtableFormat2) -> Self {
        let left_class = KernClassTable::promote(&reify_index(orig, Mandatory(obj::KernCls), 0));
        let right_class = KernClassTable::promote(&reify_index(orig, Mandatory(obj::KernCls), 1));
        let kerning_array = KerningArray::promote(&reify(orig, Mandatory(obj::KernArr)));
        KernSubtableFormat2 {
            left_class,
            right_class,
            kerning_array,
        }
    }
}

pub type OpentypeKerningArray = opentype_kern_kerning_array;

#[derive(Clone, Debug)]
#[repr(transparent)]
struct KerningArray(Wec<i16>);

impl Promote<OpentypeKerningArray> for KerningArray {
    fn promote(orig: &OpentypeKerningArray) -> Self {
        let height = orig.left_glyph_count as usize;
        let width = orig.right_glyph_count as usize;
        let size = height * width;
        let mut accum = Wec::with_capacity(width, size);
        for row in orig.kerning_values.iter() {
            accum.extend(row.iter().map(|u| as_s16(*u)));
        }
        KerningArray(accum)
    }
}

#[derive(Debug, Clone)]
struct KernSubtableFormat2 {
    left_class: KernClassTable,
    right_class: KernClassTable,
    kerning_array: KerningArray,
}

pub type OpentypeKernClassTable = opentype_kern_class_table;

impl Promote<OpentypeKernClassTable> for KernClassTable {
    fn promote(orig: &OpentypeKernClassTable) -> Self {
        KernClassTable {
            first_glyph: orig.first_glyph,
            n_glyphs: orig.n_glyphs,
            class_values: orig.class_values.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct KernClassTable {
    first_glyph: u16,
    n_glyphs: u16,
    class_values: Vec<u16>,
}

pub type OpentypeKernSubtableData<'a> = opentype_kern_kern_subtable_data<'a>;

impl<'a> Promote<OpentypeKernSubtableData<'a>> for KernSubtableData {
    fn promote(orig: &OpentypeKernSubtableData) -> Self {
        match orig {
            OpentypeKernSubtableData::Format0(f0) => {
                KernSubtableData::Format0(KernSubtableFormat0::promote(f0))
            }
            OpentypeKernSubtableData::Format2(f2) => {
                KernSubtableData::Format2(KernSubtableFormat2::promote(f2))
            }
        }
    }
}

#[derive(Clone, Debug)]
enum KernSubtableData {
    Format0(KernSubtableFormat0),
    Format2(KernSubtableFormat2),
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
struct KernMetrics {
    subtables: Vec<KernSubtable>,
}

type Heap<T> = Box<T>;
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
    gdef: Option<Heap<GdefMetrics>>,
    gpos: Option<Heap<LayoutMetrics>>,
    gsub: Option<Heap<LayoutMetrics>>,
    // STUB - add more tables as we expand opentype definition
    fvar: Option<Heap<FvarMetrics>>,
    gvar: Option<Heap<GvarMetrics>>,
    // STUB - add more tables as we expand opentype definition
    kern: Option<KernMetrics>,
    stat: Option<Heap<StatMetrics>>,
    vhea: Option<VheaMetrics>,
    vmtx: Option<VmtxMetrics>,
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
                        what: "ttc header version".to_string(),
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
            Heap::new(Cmap::promote(cmap))
        };
        let head = {
            let head = &dir.table_links.head;
            Heap::new(HeadMetrics {
                major_version: head.major_version,
                minor_version: head.minor_version,
                dir_hint: head.font_direction_hint.try_into().unwrap(),
            })
        };
        let hhea = {
            let hhea = &dir.table_links.hhea;
            Heap::new(HheaMetrics {
                major_version: hhea.major_version,
                minor_version: hhea.minor_version,
                num_lhm: hhea.number_of_long_metrics as usize,
            })
        };
        let maxp = {
            let maxp = &dir.table_links.maxp;
            let version = maxp.version;
            Heap::new(match &maxp.data {
                opentype_maxp_table_data::MaxpPostScript => MaxpMetrics::Postscript { version },
                opentype_maxp_table_data::MaxpV1(_table) => MaxpMetrics::Version1 { version },
                opentype_maxp_table_data::MaxpUnknown(_) => MaxpMetrics::UnknownVersion { version },
            })
        };
        let hmtx = {
            let hmtx = &dir.table_links.hmtx;
            let mut accum =
                Vec::with_capacity(hmtx.long_metrics.len() + hmtx.left_side_bearings.len());
            for hmet in hmtx.long_metrics.iter() {
                accum.push(UnifiedBearing {
                    advance_width: Some(hmet.advance_width),
                    left_side_bearing: as_s16(hmet.left_side_bearing),
                });
            }
            for lsb in hmtx.left_side_bearings.iter() {
                accum.push(UnifiedBearing {
                    advance_width: None,
                    left_side_bearing: as_s16(*lsb),
                });
            }
            Heap::new(HmtxMetrics(accum))
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
                    let buf = plat_encoding_lang.convert(record.string.data);
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
                            let lang_tag = utf16be_convert(&record.lang_tag.data);
                            tmp.push(LangTagRecord { lang_tag })
                        }
                        Some(tmp)
                    }
                    opentype_name_table_data::NameVersionUnknown(ver) => {
                        return Err(Box::new(UnknownValueError {
                            what: "name table version".to_string(),
                            bad_value: *ver,
                        }));
                    }
                }
            };
            Heap::new(NameMetrics {
                version: name.version,
                name_count: name.name_count as usize,
                name_records,
                lang_tag_records,
            })
        };
        let os2 = {
            let os2 = &dir.table_links.os2;
            Heap::new(Os2Metrics {
                version: os2.version,
            })
        };
        let post = {
            let post = &dir.table_links.post;
            Heap::new(PostMetrics {
                version: post.version,
                is_fixed_pitch: post.is_fixed_pitch != 0,
            })
        };
        Heap::new(RequiredTableMetrics {
            cmap,
            head,
            hhea,
            maxp,
            hmtx,
            name,
            os2,
            post,
        })
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
                .map(|g| match &g {
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
                    TestResult::Ok(Heap::new(GdefMetrics {
                        major_version: gdef.major_version,
                        minor_version: gdef.minor_version,
                        glyph_class_def: promote_opt(&reify_index(gdef, Nullable(obj::ClsDef), 0)),
                        attach_list: promote_opt(&reify(gdef, Nullable(obj::AttList))),
                        lig_caret_list: try_promote_opt(&reify(gdef, Nullable(obj::LigCarList)))?,
                        mark_attach_class_def: promote_opt(&reify_index(
                            gdef,
                            Nullable(obj::ClsDef),
                            1,
                        )),
                        data: GdefTableDataMetrics::try_promote_view(&gdef.data, gdef.table_scope)?,
                    }))
                })
                .transpose()?
        };
        let gpos = {
            let gpos = &dir.table_links.gpos;

            gpos.as_ref().map(LayoutMetrics::promote_gpos).transpose()?
        };
        let gsub = {
            let gsub = &dir.table_links.gsub;
            gsub.as_ref().map(LayoutMetrics::promote_gsub).transpose()?
        };
        let fvar = promote_opt(&dir.table_links.fvar).map(Heap::new);
        let gvar = promote_opt(&dir.table_links.gvar).map(Heap::new);
        let kern = {
            let kern = &dir.table_links.kern;
            kern.as_ref().map(|kern| KernMetrics {
                subtables: promote_vec(&kern.subtables),
            })
        };
        let stat = promote_opt(&dir.table_links.stat).map(Heap::new);
        let vhea = {
            let vhea = &dir.table_links.vhea;
            vhea.as_ref().map(|vhea| VheaMetrics {
                major_version: vhea.major_version,
                minor_version: vhea.minor_version >> 12, // we only care about 0 vs 0x1000, so we
                num_lvm: vhea.number_of_long_metrics as usize,
            })
        };
        let vmtx = {
            let vmtx = &dir.table_links.vmtx;
            vmtx.as_ref().map(|vmtx| {
                // FIXME - if name gets changed to top_side_bearings, correct accordingly
                let mut accum =
                    Vec::with_capacity(vmtx.long_metrics.len() + vmtx.left_side_bearings.len());
                for vmet in vmtx.long_metrics.iter() {
                    accum.push(UnifiedBearing {
                        advance_width: Some(vmet.advance_width),
                        // FIXME - if name gets changed to top_side_bearing, correct accordingly
                        left_side_bearing: as_s16(vmet.left_side_bearing),
                    });
                }
                // FIXME - if name gets changed to top_side_bearings, correct accordingly
                for tsb in vmtx.left_side_bearings.iter() {
                    accum.push(UnifiedBearing {
                        advance_width: None,
                        // FIXME - if name gets changed to top_side_bearing, correct accordingly
                        left_side_bearing: as_s16(*tsb),
                    });
                }
                VmtxMetrics(accum)
            })
        };
        Heap::new(OptionalTableMetrics {
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
            // TODO - add more variation tables as they are added to the spec
            fvar,
            gvar,
            // TODO - add more optional tables as they are added to the spec
            kern,
            stat,
            vhea,
            vmtx,
        })
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
        b"fvar" | b"gvar" => false,
        b"kern" | b"STAT" | b"vhea" | b"vmtx" => false,
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

// REVIEW - less commonly used due to implementation of `Tag` type, which diverges in Display slightly
fn format_magic(magic: u32) -> String {
    let bytes = magic.to_be_bytes();
    let show = |b: u8| {
        if b.is_ascii_alphanumeric() {
            String::from(b as char)
        } else {
            format!("{b:02x}")
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
    show_gdef_metrics(optional.gdef.as_deref(), conf);
    show_layout_metrics(
        optional.gpos.as_deref(),
        Ctxt::from(TableDiscriminator::Gpos),
        conf,
    );
    show_layout_metrics(
        optional.gsub.as_deref(),
        Ctxt::from(TableDiscriminator::Gsub),
        conf,
    );

    show_fvar_metrics(optional.fvar.as_deref(), conf);
    show_gvar_metrics(optional.gvar.as_deref(), conf);

    show_kern_metrics(&optional.kern, conf);
    show_stat_metrics(optional.stat.as_deref(), conf);
    show_vhea_metrics(&optional.vhea, conf);
    show_vmtx_metrics(&optional.vmtx, conf);
}

fn show_gvar_metrics(gvar: Option<&GvarMetrics>, conf: &Config) {
    let Some(gvar) = gvar else { return };
    if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
        fn display_shared_tuples(
            shared_tuples: &[GvarTupleRecord],
        ) -> display::TokenStream<'static> {
            fn display_shared_tuple_record(
                tuple: &GvarTupleRecord,
            ) -> display::TokenStream<'static> {
                use display::{tok, toks};
                const COORD_BOOKEND: usize = 4;
                arrayfmt::display_items_inline(
                    &tuple.coordinates,
                    |coord| toks(format!("{}", coord)),
                    COORD_BOOKEND,
                    |n_skipped| toks(format!("...({n_skipped} skipped)...")),
                )
            }

            use display::toks;
            const RECORDS_BOOKEND: usize = 4;
            arrayfmt::display_items_elided(
                shared_tuples,
                |shared_tuple_ix, record| {
                    toks(format!("[{shared_tuple_ix}]: "))
                        .pre_indent(4)
                        .chain(display_shared_tuple_record(record))
                },
                RECORDS_BOOKEND,
                |start, stop| toks(format!("\t{HT}(skipping shared tuples {start}..{stop})")),
            )
        }
        fn display_glyph_variation_data_array(
            array: &[Option<GlyphVariationData>],
        ) -> display::TokenStream<'_> {
            fn display_glyph_variation_data(
                table: &GlyphVariationData,
            ) -> display::TokenStream<'static> {
                fn display_tuple_variation_header(
                    header: &GvarTupleVariationHeader,
                ) -> display::TokenStream<'static> {
                    display::Token::from(format!(
                        "Header (size: {} bytes)",
                        header.variation_data_size
                    ))
                    .into()
                    // WIP
                }

                match &table.tuple_variation_headers[..] {
                    [] => unreachable!("empty tuple variation headers"), // FIXME - final version should not panic, but we want to figure out whether this case happens
                    [header] => {
                        display_tuple_variation_header(header)
                        // WIP - we may want to show more than just the header in future
                    }
                    headers => {
                        let headers_str = arrayfmt::display_items_elided(
                            headers,
                            |ix, header| {
                                toks(format!("[{ix}]: "))
                                    .pre_indent(6)
                                    .chain(display_tuple_variation_header(header))
                            },
                            4,
                            |start, stop| {
                                toks(format!(
                                    "(skipping tuple variation headers {start}..{stop})"
                                ))
                                .pre_indent(5)
                            },
                        );
                        // WIP - we may want to show more than just the headers in future
                        LineBreak.then(headers_str)
                    }
                }
            }

            use display::{Token::LineBreak, toks};
            const TABLE_BOOKEND: usize = 4;
            arrayfmt::display_nullable(
                array,
                |ix, table| {
                    toks(format!("[{ix}]: "))
                        .pre_indent(4)
                        .chain(display_glyph_variation_data(table))
                },
                TABLE_BOOKEND,
                |n, (start, stop)| {
                    toks(format!(
                        "(skipping {n} glyph variation data tables between indices {start}..{stop})"
                    ))
                    .pre_indent(3)
                },
            )
        }

        // WIP
        println!(
            "gvar: version {}",
            format_version_major_minor(gvar.major_version, gvar.minor_version)
        );
        println!("\tShared Tuples ({} total):", gvar.shared_tuples.len());
        display_shared_tuples(&gvar.shared_tuples).println();
        println!(
            "\tGlyph Variation Data ({} glyphs):",
            gvar.glyph_variation_data_array.len()
        );
        display_glyph_variation_data_array(&gvar.glyph_variation_data_array).println();
        // WIP
    } else {
        print!(
            "gvar: version {}",
            format_version_major_minor(gvar.major_version, gvar.minor_version)
        );
        println!(
            "; {} shared tuples, {} glyph variation data tables",
            gvar.shared_tuples.len(),
            gvar.glyph_variation_data_array.len(),
        );
    }
}

fn show_fvar_metrics(fvar: Option<&FvarMetrics>, conf: &Config) {
    let Some(fvar) = fvar else { return };
    if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
        println!(
            "fvar: version {}",
            format_version_major_minor(fvar.major_version, fvar.minor_version)
        );
        println!("\tAxes:");
        fn display_variation_axis_record(
            axis: &VariationAxisRecord,
        ) -> display::TokenStream<'static> {
            display::Token::from(format!(
                "'{}' axis: [{}, {}] (default: {}){}{:?}",
                axis.axis_tag,
                axis.min_value,
                axis.max_value,
                axis.default_value,
                if axis.flags.hidden_axis {
                    " (hidden) "
                } else {
                    " "
                },
                axis.axis_name_id,
            ))
            .into()
        }
        fn display_instance_record(
            instance: &InstanceRecord,
            conf: &Config,
        ) -> display::TokenStream<'static> {
            display::Token::from(format!(
                "Subfamily={:?};{} ",
                instance.subfamily_nameid,
                match instance.postscript_nameid {
                    None => String::new(),
                    Some(name_id) => format!(" Postscript={name_id:?};"),
                },
            ))
            .then(arrayfmt::display_items_inline(
                &instance.coordinates,
                |coord| display::Token::from(format!("{coord:+}")).into(),
                conf.inline_bookend,
                |n_skipped| {
                    display::Token::from(format!("...(skipping {n_skipped} coordinates)...")).into()
                },
            ))
        }

        // FIXME: rewerite into pure TokenStream
        arrayfmt::display_items_elided(
            &fvar.axes,
            |ix, axis| {
                display::Token::from(format!("\t\t[{ix}]: "))
                    .then(display_variation_axis_record(axis))
            },
            conf.bookend_size,
            |start, stop| {
                display::Token::from(format!("\t(skipping axis records {start}..{stop})")).into()
            },
        )
        .println();
        println!("\tInstances:");
        arrayfmt::display_items_elided(
            &fvar.instances,
            |ix, instance| {
                display::Token::from(format!("\t\t[{ix}]: "))
                    .then(display_instance_record(instance, conf))
            },
            conf.bookend_size,
            |start, stop| {
                display::Token::from(format!("\t(skipping instance records {start}..{stop})"))
                    .into()
            },
        )
        .println();
    } else {
        // FIXME - rewrite into pure TokenStream
        print!(
            "fvar: version {}",
            format_version_major_minor(fvar.major_version, fvar.minor_version)
        );
        println!(
            "; {} axes, {} instances",
            fvar.axes.len(),
            fvar.instances.len()
        );
    }
}

fn show_cvt_metrics(cvt: &Option<CvtMetrics>, _conf: &Config) {
    let Some(RawArrayMetrics(count)) = cvt else {
        return;
    };

    println!("cvt: FWORD[{count}]")
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

fn show_gdef_metrics(gdef: Option<&GdefMetrics>, conf: &Config) {
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
        // WIP
        println!(
            "GDEF: version {}",
            format_version_major_minor(*major_version, *minor_version)
        );
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            if let Some(glyph_class_def) = glyph_class_def {
                show_glyph_class_def(glyph_class_def, conf);
            }
            if let Some(attach_list) = attach_list {
                display_attach_list(attach_list, conf).println(); // WIP
            }
            if let Some(lig_caret_list) = lig_caret_list {
                // WIP
                display_lig_caret_list(lig_caret_list, conf).println();
            }
            if let Some(mark_attach_class_def) = mark_attach_class_def {
                show_mark_attach_class_def(mark_attach_class_def, conf);
            }
            match &data.mark_glyph_sets_def {
                // WIP
                None => println!("\tMarkGlyphSet: <none>"),
                Some(mgs) => display_mark_glyph_set(mgs, conf).println(),
            }
            match &data.item_var_store {
                None => println!("\tItemVariationStore: <none>"),
                Some(ivs) => display_item_variation_store(ivs, conf).println(),
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

fn show_layout_metrics(layout: Option<&LayoutMetrics>, ctxt: Ctxt, conf: &Config) {
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
            show_script_list(script_list, conf);
            show_feature_list(feature_list, conf);
            display_lookup_list(lookup_list, ctxt, conf).println();
        }
    }
}

fn show_script_list(script_list: &ScriptList, conf: &Config) {
    use display::{Token::LineBreak, toks};
    if script_list.is_empty() {
        println!("\tScriptList [empty]");
    } else {
        println!("\tScriptList");
        arrayfmt::display_items_elided(
            script_list,
            |ix, item| {
                let Some(ScriptTable {
                    default_lang_sys,
                    lang_sys_records,
                }) = &item.script
                else {
                    unreachable!("missing ScriptTable at index {ix} in ScriptList");
                };

                toks(format!("[{ix}]: {}", item.script_tag))
                    .pre_indent(4)
                    .glue(LineBreak, {
                        match default_lang_sys {
                            None => display_lang_sys_records(lang_sys_records, conf),
                            langsys @ Some(..) => toks("[Default LangSys]: ")
                                .pre_indent(5)
                                .chain(display_langsys(langsys, conf))
                                .glue(LineBreak, display_lang_sys_records(lang_sys_records, conf)),
                        }
                    })
            },
            conf.bookend_size,
            |start, stop| toks(format!("skipping ScriptRecords {start}..{stop}")),
        )
        .println()
    }
}

fn display_lang_sys_records(
    lang_sys_records: &[LangSysRecord],
    conf: &Config,
) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, toks};
    if lang_sys_records.is_empty() {
        toks("LangSysRecords: <empty list>").pre_indent(5)
    } else {
        toks("LangSysRecords:").pre_indent(5).glue(
            LineBreak,
            arrayfmt::display_items_elided(
                lang_sys_records,
                |ix, item| {
                    toks(format!("[{ix}]: {}; ", item.lang_sys_tag))
                        .pre_indent(6)
                        .chain(display_langsys(&item.lang_sys, conf))
                },
                conf.bookend_size,
                |start, stop| {
                    toks(format!("(skipping LangSysRecords {start}..{stop})")).pre_indent(5)
                },
            ),
        )
    }
}

fn display_langsys(lang_sys: &Link<LangSys>, conf: &Config) -> display::TokenStream<'static> {
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
        0xFFFF => display::Token::from(format!("feature-indices: ")),
        other => display::Token::from(format!("feature-indices (required: {other}): ")),
    }
    .then(arrayfmt::display_items_inline(
        feature_indices,
        |ix: &u16| display::Token::from(format!("{ix}")).into(),
        conf.inline_bookend,
        |num_skipped: usize| display::Token::from(format!("...({num_skipped} skipped)...")).into(),
    ))
}

fn show_feature_list(feature_list: &FeatureList, conf: &Config) {
    if feature_list.is_empty() {
        // FIXME - turn into pure tokenstream
        println!("\tFeatureList [empty]");
    } else {
        println!("\tFeatureList");
        arrayfmt::display_items_elided(
            feature_list,
            |ix, item| {
                let FeatureRecord {
                    feature_tag,
                    feature,
                } = item;
                display::Token::from(format!("\t\t[{ix}]: {feature_tag}"))
                    .then(show_feature_table(feature, conf))
            },
            conf.bookend_size,
            |start, stop| {
                display::Token::from(format!("\t    (skipping FeatureIndices {start}..{stop})"))
                    .into()
            },
        )
        .println()
    }
}

fn show_feature_table(table: &FeatureTable, conf: &Config) -> display::TokenStream<'static> {
    let FeatureTable {
        feature_params,
        lookup_list_indices,
    } = table;

    let stream = arrayfmt::display_items_inline(
        lookup_list_indices,
        |index| display::Token::from(format!("{index}")).into(),
        conf.inline_bookend,
        |num_skipped| display::Token::from(format!("...({num_skipped} skipped)...")).into(),
    );
    match feature_params {
        0 => stream,
        offset => {
            display::Token::from(format!("[parameters located at SoF+{offset}B]")).then(stream)
        }
    }
}

fn display_lookup_list(
    lookup_list: &LookupList,
    ctxt: Ctxt,
    conf: &Config,
) -> display::TokenStream<'static> {
    display::tok("\tLookupList:").then(display::Token::LineBreak.then(
        arrayfmt::display_items_elided(
            lookup_list,
            move |ix, table| {
                display::tok(format!("\t\t[{ix}]: ")).then(display_lookup_table(table, ctxt, conf))
            },
            conf.bookend_size,
            |start, stop| display::toks(format!("\t    (skipping LookupTables {start}..{stop})")),
        ),
    ))
}

fn display_lookup_table(
    table: &LookupTable,
    ctxt: Ctxt,
    conf: &Config,
) -> display::TokenStream<'static> {
    // NOTE - because we print the kind of the lookup here, we don't need to list it for every element
    // LINK[format-lookup-subtable] -  (see format_lookup_subtable below)
    let mut stream = display::Token::from(format!(
        "LookupTable: kind={}",
        format_lookup_type(ctxt, table.lookup_type),
    ))
    .then(display_lookup_flags(&table.lookup_flag));
    if let Some(filtering_set) = table.mark_filtering_set {
        stream = stream.chain(
            display::Token::from(format!(
                ", markFilteringSet=GDEF->MarkGlyphSet[{filtering_set}]"
            ))
            .into(),
        );
    }
    stream.chain(
        display::Token::InlineText(": ".into()).then(arrayfmt::display_items_inline(
            &table.subtables,
            |subtable| display_lookup_subtable(subtable, false, conf),
            conf.inline_bookend,
            |n_skipped| display::Token::from(format!("...({n_skipped} skipped)...")).into(),
        )),
    )
}

// ANCHOR[format-lookup-subtable]
fn display_lookup_subtable(
    subtable: &LookupSubtable,
    show_lookup_type: bool,
    conf: &Config,
) -> display::TokenStream<'static> {
    use display::{tok, toks};
    // STUB - because the subtables are both partial (more variants exist) and abridged (existing variants are missing details), reimplement as necessary
    // FIXME - refactor so contents is a pure tokenstream
    // WIP
    let (label, contents) = match subtable {
        LookupSubtable::SinglePos(single_pos) => {
            let contents = {
                match single_pos {
                    SinglePos::Format1(SinglePosFormat1 { value_record, .. }) => {
                        tok("single").then(display_value_record(value_record).paren())
                    }
                    SinglePos::Format2(SinglePosFormat2 { coverage, .. }) => {
                        tok("array").then(display_coverage_table(coverage).paren())
                    }
                }
            };
            ("SinglePos", contents)
        }
        LookupSubtable::PairPos(pair_pos) => {
            let contents = {
                match pair_pos {
                    PairPos::Format1(PairPosFormat1 { coverage, .. }) => {
                        tok("byGlyph").then(display_coverage_table(coverage).paren())
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
                            tok(format!("byClass{:?}", _populated_class_pairs,))
                                .then(display_coverage_table(coverage).surround(tok("("), tok(")")))
                        } else {
                            tok(format!(
                                "byClass[{} ∈ {rows} x {cols}]",
                                _populated_class_pairs.len(),
                            ))
                            .then(display_coverage_table(coverage).paren())
                        }
                    }
                }
            };
            ("PairPos", contents)
        }
        LookupSubtable::CursivePos(CursivePos { coverage, .. }) => {
            let contents = tok("entryExit").then(display_coverage_table(coverage).paren());
            ("CursivePos", contents)
        }
        LookupSubtable::MarkBasePos(MarkBasePos {
            mark_coverage,
            base_coverage,
            mark_array,
            base_array,
        }) => {
            let contents = {
                let mut mark_iter = mark_coverage.iter();
                let mut base_iter = base_coverage.iter();
                let mark_lhs = tok("Mark").then(display_coverage_table(mark_coverage).paren());
                let base_lhs = tok("Base").then(display_coverage_table(base_coverage).paren());
                let mark_rhs =
                    tok("MarkArray").then(display_mark_array(mark_array, &mut mark_iter).bracket());
                let base_rhs =
                    tok("BaseArray").then(display_base_array(base_array, &mut base_iter).bracket());
                let lhs = mark_lhs.glue(tok("+"), base_lhs);
                let rhs = mark_rhs.glue(tok("+"), base_rhs);
                lhs.glue(tok("=>"), rhs)
            };
            ("MarkBasePos", contents)
        }
        LookupSubtable::MarkLigPos(MarkLigPos {
            mark_coverage,
            ligature_coverage,
            mark_array,
            ligature_array,
        }) => {
            let contents = {
                let mut mark_iter = mark_coverage.iter();
                let mut ligature_iter = ligature_coverage.iter();
                let mark_lhs = tok("Mark").then(display_coverage_table(mark_coverage).paren());
                let lig_lhs =
                    tok("Ligature").then(display_coverage_table(ligature_coverage).paren());
                let mark_rhs =
                    tok("MarkArray").then(display_mark_array(mark_array, &mut mark_iter).bracket());
                let lig_rhs = tok("LigatureArray")
                    .then(display_ligature_array(ligature_array, &mut ligature_iter).bracket());
                let lhs = mark_lhs.glue(tok("+"), lig_lhs);
                let rhs = mark_rhs.glue(tok("+"), lig_rhs);
                lhs.glue(tok("=>"), rhs)
            };
            ("MarkLigPos", contents)
        }
        LookupSubtable::MarkMarkPos(MarkMarkPos {
            mark1_coverage,
            mark2_coverage,
            mark1_array,
            mark2_array,
        }) => {
            let contents = {
                let mut mark1_iter = mark1_coverage.iter();
                let mut mark2_iter = mark2_coverage.iter();
                let mark_lhs = tok("Mark").then(display_coverage_table(mark1_coverage).paren());
                let mark2_lhs = tok("Mark").then(display_coverage_table(mark2_coverage).paren());
                let mark_rhs = tok("MarkArray")
                    .then(display_mark_array(mark1_array, &mut mark1_iter).bracket());
                let mark2_rhs = tok("Mark2Array")
                    .then(display_mark2_array(mark2_array, &mut mark2_iter).bracket());
                let lhs = mark_lhs.glue(tok("+"), mark2_lhs);
                let rhs = mark_rhs.glue(tok("+"), mark2_rhs);
                lhs.glue(tok("=>"), rhs)
            };
            ("MarkMarkPos", contents)
        }
        LookupSubtable::SingleSubst(single_subst) => {
            let contents = match single_subst {
                SingleSubst::Format1(SingleSubstFormat1 {
                    coverage,
                    delta_glyph_id,
                }) => {
                    display_coverage_table(coverage).chain(toks(format!("=>({delta_glyph_id:+})")))
                }
                SingleSubst::Format2(SingleSubstFormat2 {
                    coverage,
                    substitute_glyph_ids,
                }) => {
                    let iter = coverage.iter();
                    display_coverage_table(coverage).glue(
                        tok("=>"),
                        arrayfmt::display_coverage_linked_array(
                            substitute_glyph_ids,
                            iter,
                            |orig_glyph, subst_glyph| {
                                toks(format!(
                                    "{}->{}",
                                    format_glyphid_hex(orig_glyph, true),
                                    format_glyphid_hex(*subst_glyph, true),
                                ))
                            },
                            conf.inline_bookend,
                            |n_skipped| toks(format!("...(skipping {n_skipped} substs)...")),
                        ),
                    )
                }
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
                display_coverage_table(coverage)
                    .chain(toks(format!("=>SequenceTable[{}]", sequences.len())))
            };
            ("MultipleSubst", contents)
        }
        LookupSubtable::AlternateSubst(AlternateSubst {
            coverage,
            alternate_sets,
        }) => {
            let contents = {
                display_coverage_table(coverage)
                    .glue(tok("=>"), display_alternate_sets(alternate_sets))
            };
            ("AlternateSubst", contents)
        }
        LookupSubtable::LigatureSubst(LigatureSubst {
            coverage,
            ligature_sets,
        }) => {
            // WIP
            let contents = display_ligature_sets(ligature_sets, coverage.iter());
            ("LigatureSubst", contents)
        }
        LookupSubtable::ReverseChainSingleSubst(ReverseChainSingleSubst {
            coverage,
            backtrack_coverages,
            lookahead_coverages,
            substitute_glyph_ids,
            ..
        }) => {
            let contents = {
                // REVIEW - since we are already within an inline elision context, try to avoid taking up too much space per item, but this might not want to be a hardcoded value
                const INLINE_INLINE_BOOKEND: usize = 1;
                // FIXME - show_lookup_table calls this function through display_items_inline already, so we might want to reduce how many values we are willing to show proportionally
                let backtrack_pattern = if backtrack_coverages.is_empty() {
                    display::TokenStream::empty()
                } else {
                    arrayfmt::display_items_inline(
                        backtrack_coverages,
                        display_coverage_table,
                        INLINE_INLINE_BOOKEND,
                        |n| display::toks(format!("(..{n}..)")),
                    )
                    .surround(tok("(?<="), tok(")"))
                };
                let input_pattern = display_coverage_table(coverage);
                let lookahead_pattern = if lookahead_coverages.is_empty() {
                    display::TokenStream::empty()
                } else {
                    arrayfmt::display_items_inline(
                        lookahead_coverages,
                        display_coverage_table,
                        INLINE_INLINE_BOOKEND,
                        |n| display::toks(format!("(..{n}..)")),
                    )
                    .surround(tok("(?="), tok(")"))
                };
                let substitute_ids = format_glyphid_array_hex(substitute_glyph_ids, true);

                backtrack_pattern
                    .chain(input_pattern)
                    .chain(lookahead_pattern)
                    .chain(toks(format!("=>{substitute_ids}")))
            };
            ("RevChainSingleSubst", contents)
        }
        LookupSubtable::SequenceContext(seq_ctx) => {
            let contents = match seq_ctx {
                SequenceContext::Format1(SequenceContextFormat1 { coverage, .. }) => {
                    tok("Glyphs").then(display_coverage_table(coverage).paren())
                }
                SequenceContext::Format2(SequenceContextFormat2 { coverage, .. }) => {
                    tok("Classes").then(display_coverage_table(coverage).paren())
                }
                SequenceContext::Format3(SequenceContextFormat3 {
                    coverage_tables,
                    seq_lookup_records,
                    ..
                }) => {
                    // REVIEW - since we are already within an inline elision context, try to avoid taking up too much space per item, but this might not want to be a hardcoded value
                    const INLINE_INLINE_BOOKEND: usize = 1;
                    // FIXME - show_lookup_table calls this function through display_items_inline already, so we might want to reduce how many values we are willing to show proportionally
                    let input_pattern = arrayfmt::display_items_inline(
                        coverage_tables,
                        display_coverage_table,
                        INLINE_INLINE_BOOKEND,
                        |n| toks(format!("(..{n}..)")),
                    );
                    let seq_lookups = arrayfmt::display_items_inline(
                        seq_lookup_records,
                        display_sequence_lookup,
                        INLINE_INLINE_BOOKEND,
                        |n| toks(format!("(..{n}..)")),
                    );
                    input_pattern.glue(tok("=>"), seq_lookups)
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
                    tok("ChainedGlyphs").then(display_coverage_table(coverage).paren())
                }
                ChainedSequenceContext::Format2(ChainedSequenceContextFormat2 {
                    coverage, ..
                }) => {
                    // TODO - even if it means overly verbose output, this might be too little info to be useful compared to discriminant-only display
                    // REVIEW - consider what other details (e.g. class-def summary metrics) to show in implicitly- or explicitly-verbose display format
                    tok("ChainedClasses").then(display_coverage_table(coverage).paren())
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
                    // FIXME - show_lookup_table calls this function through display_items_inline already, so we might want to reduce how many values we are willing to show proportionally
                    let backtrack_pattern = if backtrack_coverages.is_empty() {
                        display::TokenStream::empty()
                    } else {
                        arrayfmt::display_items_inline(
                            backtrack_coverages,
                            display_coverage_table,
                            INLINE_INLINE_BOOKEND,
                            |n| toks(format!("(..{n}..)")),
                        )
                        .surround(tok("(?<="), tok(")"))
                    };
                    let input_pattern = arrayfmt::display_items_inline(
                        input_coverages,
                        display_coverage_table,
                        INLINE_INLINE_BOOKEND,
                        |n| toks(format!("(..{n}..)")),
                    );
                    let lookahead_pattern = if lookahead_coverages.is_empty() {
                        display::TokenStream::empty()
                    } else {
                        arrayfmt::display_items_inline(
                            lookahead_coverages,
                            display_coverage_table,
                            INLINE_INLINE_BOOKEND,
                            |n| toks(format!("(..{n}..)")),
                        )
                        .surround(tok("(?="), tok(")"))
                    };
                    let seq_lookups = arrayfmt::display_items_inline(
                        seq_lookup_records,
                        display_sequence_lookup,
                        INLINE_INLINE_BOOKEND,
                        |n| toks(format!("(..{n}..)")),
                    );
                    backtrack_pattern
                        .chain(input_pattern)
                        .chain(lookahead_pattern)
                        .glue(tok("=>"), seq_lookups)
                }
            };
            ("ChainSeqCtx", contents)
        }
    };
    let label = tok(label);
    // WIP
    if show_lookup_type {
        label.then(contents)
    } else {
        contents
    }
}

fn show_stat_metrics(stat: Option<&StatMetrics>, conf: &Config) {
    fn display_design_axis(axis: &DesignAxis, _conf: &Config) -> display::TokenStream<'static> {
        display::toks(format!(
            "Tag={} ; Axis NameID={} ; Ordering={}",
            axis.axis_tag, axis.axis_name_id.0, axis.axis_ordering
        ))
    }
    fn display_axis_value(value: &AxisValue, conf: &Config) -> display::TokenStream<'static> {
        fn format_axis_value_flags(flags: &AxisValueFlags) -> String {
            let mut set_flags = Vec::new();
            if flags.elidable_axis_value_name {
                // REVIEW - is there a pithier, but not obfuscating, string we can use instead?
                set_flags.push("ELIDABLE_AXIS_VALUE_NAME");
            }
            if flags.older_sibling_font_attribute {
                // REVIEW - is there a pithier, but not obfuscating, string we can use instead?
                set_flags.push("OLDER_SIBLING_FONT_ATTRIBUTE");
            }
            if set_flags.is_empty() {
                String::new()
            } else {
                format!(" (Flags: {})", set_flags.join(" | "))
            }
        }

        use display::{tok, toks};
        match value {
            AxisValue::Format1(AxisValueFormat1 {
                axis_index,
                flags,
                value_name_id,
                value,
            }) => toks(format!(
                "Axis[{}]{}: {:?} = {}",
                axis_index,
                format_axis_value_flags(flags),
                value_name_id,
                value
            )),
            AxisValue::Format2(AxisValueFormat2 {
                axis_index,
                flags,
                value_name_id,
                nominal_value,
                range_min_value,
                range_max_value,
            }) => toks(format!(
                "Axis[{}]{}: {:?} = {} ∈ [{}, {}]",
                axis_index,
                format_axis_value_flags(flags),
                value_name_id,
                nominal_value,
                range_min_value,
                range_max_value
            )),
            AxisValue::Format3(AxisValueFormat3 {
                axis_index,
                flags,
                value_name_id,
                value,
                linked_value,
            }) => toks(format!(
                "Axis[{}]{}: {:?} = {} (-> {})",
                axis_index,
                format_axis_value_flags(flags),
                value_name_id,
                value,
                linked_value
            )),
            AxisValue::Format4(AxisValueFormat4 {
                flags,
                value_name_id,
                axis_values,
            }) => tok(format!(
                "{:?}{}: ",
                value_name_id,
                format_axis_value_flags(flags),
            ))
            .then(arrayfmt::display_items_inline(
                axis_values,
                |axis_value| {
                    display::toks(format!(
                        "Axis[{}] = {}",
                        axis_value.axis_index, axis_value.value
                    ))
                },
                conf.inline_bookend,
                |n_skipped| {
                    display::toks(format!("...(skipping {n_skipped} AxisValue records)..."))
                },
            )),
        }
    }
    if let Some(stat) = stat {
        println!(
            "STAT: version {} (elidedFallbackName: name[id={}])",
            format_version_major_minor(stat.major_version, stat.minor_version),
            stat.elided_fallback_name_id.0
        );
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            match stat.design_axes.len() {
                0 => (),
                n => {
                    // FIXME - promote to first-class tokenstream
                    println!("\tDesignAxes: {n} total");
                    arrayfmt::display_items_elided(
                        &stat.design_axes,
                        |ix, d_axis| {
                            display::tok(format!("\t\t[{ix}]: "))
                                .then(display_design_axis(d_axis, conf))
                        },
                        conf.bookend_size,
                        |start, stop| {
                            display::toks(format!(
                                "\t(skipping design-axes from index {start}..{stop})"
                            ))
                        },
                    )
                    .println()
                }
            }
            match stat.axis_values.len() {
                0 => (),
                n => {
                    println!("\tAxisValues: {n} total");
                    arrayfmt::display_items_elided(
                        &stat.axis_values,
                        |ix, a_value| {
                            display::tok(format!("\t\t[{ix}]: "))
                                .then(display_axis_value(a_value, conf))
                        },
                        conf.bookend_size,
                        |start, stop| {
                            display::toks(format!(
                                "\t(skipping design-axes from index {start}..{stop})"
                            ))
                        },
                    )
                    .println()
                }
            }
        }
    }
}

fn show_kern_metrics(kern: &Option<KernMetrics>, conf: &Config) {
    use display::{tok, toks};
    fn display_kern_subtable(
        subtable: &KernSubtable,
        conf: &Config,
    ) -> display::TokenStream<'static> {
        use display::{tok, toks};
        fn format_kern_flags(flags: KernFlags) -> String {
            let mut params = Vec::new();
            if flags.r#override {
                params.push("override");
            }
            if flags.cross_stream {
                params.push("x-stream")
            }
            if flags.minimum {
                params.push("min")
            } else {
                params.push("kern")
            }
            if flags.horizontal {
                params.push("h")
            } else {
                params.push("v")
            }

            params.join(" | ")
        }

        fn display_kern_subtable_data(
            subtable_data: &KernSubtableData,
            conf: &Config,
        ) -> display::TokenStream<'static> {
            use display::{tok, toks};
            match subtable_data {
                KernSubtableData::Format0(KernSubtableFormat0 { kern_pairs }) => {
                    arrayfmt::display_items_inline(
                        kern_pairs,
                        |kern_pair| {
                            toks(format!(
                                "({},{}) {:+}",
                                format_glyphid_hex(kern_pair.left, true),
                                format_glyphid_hex(kern_pair.right, true),
                                kern_pair.value
                            ))
                        },
                        conf.inline_bookend,
                        |n| toks(format!("(..{n}..)")),
                    )
                }
                KernSubtableData::Format2(KernSubtableFormat2 {
                    left_class,
                    right_class,
                    kerning_array,
                }) => {
                    fn display_kern_class_table(
                        table: &KernClassTable,
                        conf: &Config,
                    ) -> display::TokenStream<'static> {
                        use display::{tok, toks};
                        tok(format!(
                            "Classes[first={}, nGlyphs={}]: ",
                            format_glyphid_hex(table.first_glyph, true),
                            table.n_glyphs,
                        ))
                        .then(arrayfmt::display_items_inline(
                            &table.class_values,
                            |id| toks(u16::to_string(id)),
                            conf.inline_bookend,
                            |n| toks(format!("(..{n}..)")),
                        ))
                    }
                    fn display_kerning_array(
                        array: &KerningArray,
                        conf: &Config,
                    ) -> display::TokenStream<'static> {
                        arrayfmt::display_wec_rows_elided(
                            &array.0,
                            |ix, row| {
                                display::tok(format!("\t\t[{ix}]: ")).then(
                                    arrayfmt::display_items_inline(
                                        row,
                                        |kern_val| display::toks(format!("{kern_val:+}")),
                                        conf.inline_bookend,
                                        |n| display::toks(format!("(..{n}..)")),
                                    ),
                                )
                            },
                            conf.bookend_size / 2, // FIXME - magic constant adjustment
                            |start, stop| {
                                display::toks(format!(
                                    "\t\t(skipping kerning array rows {start}..{stop})"
                                ))
                            },
                        )
                    }
                    let left = tok("LeftClass=").then(display_kern_class_table(left_class, conf));

                    let right =
                        tok("RightClass=").then(display_kern_class_table(right_class, conf));
                    let kern =
                        tok("KerningArray:").then(display_kerning_array(kerning_array, conf));
                    left.glue(tok("\t"), right).glue(tok("\t"), kern)
                }
            }
        }

        tok(format!(
            "KernSubtable ({}): ",
            format_kern_flags(subtable.flags)
        ))
        .then(display_kern_subtable_data(&subtable.data, conf))
    }

    if let Some(kern) = kern {
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            println!("kern");
            arrayfmt::display_items_elided(
                &kern.subtables,
                |ix, subtable| {
                    toks(format!("[{ix}]: "))
                        .pre_indent(2)
                        .chain(display_kern_subtable(subtable, conf))
                },
                conf.bookend_size,
                |start, stop| {
                    toks(format!("(skipping kern subtables {start}..{stop})")).pre_indent(1)
                },
            )
            .println();
        } else {
            println!("kern: {} kerning subtables", kern.subtables.len());
        }
    }
}

fn display_mark2_array(
    arr: &Mark2Array,
    coverage: &mut impl Iterator<Item = u16>,
) -> display::TokenStream<'static> {
    use display::toks;

    fn display_mark2_record(mark2_record: &Mark2Record, cov: u16) -> display::TokenStream<'static> {
        const CLASS_ANCHORS: usize = 2;
        use display::{tok, toks};
        tok(format!("{}: ", format_glyphid_hex(cov, true),)).then(
            arrayfmt::display_inline_nullable(
                &mark2_record.mark2_anchors,
                |ix, anchor| tok(format!("[{ix}]=>")).then(display_anchor_table(anchor)),
                CLASS_ANCHORS,
                |n, (start, end)| {
                    toks(format!(
                        "...(skipping {n} indices spanning {start}..={end})..."
                    ))
                },
            ),
        )
    }

    const MARK2_ARRAY_BOOKEND: usize = 2;
    arrayfmt::display_items_inline(
        &arr.mark2_records,
        |mark2_record| {
            display_mark2_record(mark2_record, coverage.next().expect("missing coverage"))
        },
        MARK2_ARRAY_BOOKEND,
        |n| toks(format!("...(skipping {n} Mark2Records)...")),
    )
}

fn display_ligature_array(
    ligature_array: &LigatureArray,
    coverage: &mut impl Iterator<Item = u16>,
) -> display::TokenStream<'static> {
    fn display_ligature_attach(
        ligature_attach: &LigatureAttach,
        cov: u16,
    ) -> display::TokenStream<'static> {
        fn display_component_record(
            component_record: &ComponentRecord,
        ) -> display::TokenStream<'static> {
            use display::{tok, toks};
            const CLASS_ANCHOR_BOOKEND: usize = 2;
            arrayfmt::display_inline_nullable(
                &component_record.ligature_anchors,
                |ix, anchor| tok(format!("[{ix}]=>")).then(display_anchor_table(anchor)),
                CLASS_ANCHOR_BOOKEND,
                |n_skipped, (first, last)| {
                    toks(format!(
                        "...(skipping {n_skipped} indices from {first} to {last})..."
                    ))
                },
            )
        }

        use display::{tok, toks};
        const COMPONENTS_BOOKEND: usize = 1;
        tok(format!("{cov:04x}=")).then(arrayfmt::display_items_inline(
            &ligature_attach.component_records,
            display_component_record,
            COMPONENTS_BOOKEND,
            |_| toks(".."),
        ))
    }

    use display::toks;
    const ATTACHES_INLINE: usize = 2;
    arrayfmt::display_items_inline(
        &ligature_array.ligature_attach,
        |attach| display_ligature_attach(attach, coverage.next().expect("missing coverage")),
        ATTACHES_INLINE,
        |n_skipped| toks(format!("...(skipping {n_skipped})...")),
    )
}

fn display_base_array(
    base_array: &BaseArray,
    coverage: &mut impl Iterator<Item = u16>,
) -> display::TokenStream<'static> {
    fn display_base_record(base_record: &BaseRecord, cov: u16) -> display::TokenStream<'static> {
        use display::{tok, toks};
        const CLASS_ANCHOR_BOOKEND: usize = 2;
        tok(format!("{cov:04x}: ")).then(arrayfmt::display_inline_nullable(
            &base_record.base_anchors,
            |ix, anchor| tok(format!("[{ix}]=>")).then(display_anchor_table(anchor)),
            CLASS_ANCHOR_BOOKEND,
            |n_skipped, (first, last)| {
                toks(format!(
                    "...(skipping {n_skipped} indices from {first} to {last})..."
                ))
            },
        ))
    }

    use display::toks;
    const BASE_ARRAY_BOOKEND: usize = 2;
    arrayfmt::display_items_inline(
        &base_array.base_records,
        |base_record| display_base_record(base_record, coverage.next().expect("missing coverage")),
        BASE_ARRAY_BOOKEND,
        |n_skipped| toks(format!("...({n_skipped} skipped)...")),
    )
}

fn display_mark_array(
    mark_array: &MarkArray,
    coverage: &mut impl Iterator<Item = u16>,
) -> display::TokenStream<'static> {
    fn display_mark_record(mark_record: &MarkRecord, cov: u16) -> display::TokenStream<'static> {
        use display::{tok, toks};
        tok(format!("{cov:04x}=({}, ", mark_record.mark_class,)).then(
            display_anchor_table(mark_record.mark_anchor.as_ref().expect("broken link"))
                .chain(toks(")")),
        )
    }

    // FIXME[magic] - arbitrary local bookending const
    const MARK_ARRAY_BOOKEND: usize = 2;
    arrayfmt::display_items_inline(
        &mark_array.mark_records,
        |mark_record| display_mark_record(mark_record, coverage.next().expect("missing coverage")),
        MARK_ARRAY_BOOKEND,
        |n_skipped| display::toks(format!("...({n_skipped} skipped)...")),
    )
}

fn display_anchor_table(anchor: &AnchorTable) -> display::TokenStream<'static> {
    use display::{tok, toks};
    match anchor {
        AnchorTable::Format1(AnchorTableFormat1 {
            x_coordinate,
            y_coordinate,
        }) => toks(format!(
            "({}, {})",
            as_s16(*x_coordinate),
            as_s16(*y_coordinate)
        )),
        AnchorTable::Format2(f2) => toks(format!(
            "({}, {})@[{}]",
            as_s16(f2.x_coordinate),
            as_s16(f2.y_coordinate),
            f2.anchor_point
        )),
        AnchorTable::Format3(AnchorTableFormat3 {
            x_coordinate,
            y_coordinate,
            x_device,
            y_device,
        }) => {
            let extra = match (x_device, y_device) {
                (None, None) => unreachable!(
                    "unexpected both-Null DeviceOrVariationIndexTable-offsets in AnchorTable::Format3"
                ),
                (Some(x), Some(y)) => {
                    // FIXME - refactor format_device_or_variation_index_table to be TokenStream
                    toks(format!(
                        "×({}, {})",
                        format_device_or_variation_index_table(x),
                        format_device_or_variation_index_table(y)
                    ))
                }
                (Some(x), None) => {
                    // FIXME - refactor format_device_or_variation_index_table to be TokenStream
                    toks(format!(
                        "×({}, ⅈ)",
                        format_device_or_variation_index_table(x)
                    ))
                }
                (None, Some(y)) => {
                    // FIXME - refactor format_device_or_variation_index_table to be TokenStream
                    toks(format!(
                        "×(ⅈ, {})",
                        format_device_or_variation_index_table(y)
                    ))
                }
            };
            tok(format!(
                "({}, {})",
                as_s16(*x_coordinate),
                as_s16(*y_coordinate),
            ))
            .then(extra)
        }
    }
}

fn display_ligature_sets(
    lig_sets: &[LigatureSet],
    mut coverage: impl Iterator<Item = u16>,
) -> display::TokenStream<'static> {
    fn display_ligature_set(lig_set: &LigatureSet, cov: u16) -> display::TokenStream<'static> {
        fn display_ligature(lig: &Ligature, cov: u16) -> display::TokenStream<'static> {
            // FIXME - refactor format_glyphid_array_hex to be TokenStream
            display::toks(format!(
                "(#{cov:04x}.{} => {})",
                format_glyphid_array_hex(&lig.component_glyph_ids, false),
                lig.ligature_glyph,
            ))
        }
        // FIXME[magic] - arbitrary local bookending const
        const LIG_BOOKEND: usize = 2;
        arrayfmt::display_items_inline(
            &lig_set.ligatures,
            |lig| display_ligature(lig, cov),
            LIG_BOOKEND,
            |n_skipped| display::toks(format!("...({n_skipped} skipped)...")),
        )
    }
    match lig_sets {
        [set] => display_ligature_set(set, coverage.next().expect("missing coverage")),
        more => {
            const LIG_SET_BOOKEND: usize = 1;
            arrayfmt::display_coverage_linked_array(
                more,
                coverage,
                |cov, lig_set| display_ligature_set(lig_set, cov),
                LIG_SET_BOOKEND,
                |_| display::toks(".."),
            )
        }
    }
}

fn display_alternate_sets(alt_sets: &[AlternateSet]) -> display::TokenStream<'static> {
    fn display_alternate_set(alt_set: &AlternateSet) -> display::TokenStream<'static> {
        use display::toks;
        const ALT_GLYPH_BOOKEND: usize = 2;
        arrayfmt::display_items_inline(
            &alt_set.alternate_glyph_ids,
            |glyph_id| toks(format_glyphid_hex(*glyph_id, true)),
            ALT_GLYPH_BOOKEND,
            |_| toks("..".to_string()),
        )
    }
    match alt_sets {
        [set] => display_alternate_set(set),
        more => {
            const ALT_SET_BOOKEND: usize = 1;
            arrayfmt::display_items_inline(more, display_alternate_set, ALT_SET_BOOKEND, |count| {
                display::toks(format!("...({count} skipped)..."))
            })
        }
    }
}

fn display_sequence_lookup(sl: &SequenceLookup) -> display::TokenStream<'static> {
    let s_ix = sl.sequence_index;
    let ll_ix = sl.lookup_list_index;
    // NOTE - the number in `\[_\]` is meant to mimic the index display of the display_items_elided formatting of LookupList, so it is the lookup index. The number after `@` is the positional index to apply the lookup to
    display::toks(format!("[{ll_ix}]@{s_ix}"))
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
                    panic!(
                        "expecting {expected_classes} starting at 0, found ClassValue {value} (>= {max}) at index {_ix} (glyph id: {})",
                        *_start_id + _ix as u16
                    );
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
                    panic!(
                        "expecting {expected_classes} starting at 0, found ClassValue {value} (>= {max}) at index {_ix} (glyph range: {} -> {})",
                        rr.start_glyph_id, rr.end_glyph_id
                    );
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

fn display_value_record(record: &ValueRecord) -> display::TokenStream<'static> {
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
    use display::{tok, toks};
    const NUM_FRAGMENTS: usize = 4;
    let mut buf = Vec::with_capacity(NUM_FRAGMENTS);

    // helper to indicate whether a field exists
    let elide = |opt_val: &Option<_>| -> Option<&'static str> { opt_val.as_ref().map(|_| "(..)") };

    buf.extend(display_opt_xy("placement", *x_placement, *y_placement));
    buf.extend(display_opt_xy("advance", *x_advance, *y_advance));
    buf.extend(display_opt_xy(
        "placement(device)",
        elide(x_placement_device),
        elide(y_placement_device),
    ));
    buf.extend(display_opt_xy(
        "advance(device)",
        elide(x_advance_device),
        elide(y_advance_device),
    ));

    if buf.is_empty() {
        // REVIEW - this is highly unlikely, right..?
        toks("<Empty ValueRecord>")
    } else {
        display::TokenStream::join_with(buf, tok("; "))
    }
}

fn display_opt_xy<T>(
    what: &str,
    x: Option<T>,
    y: Option<T>,
) -> Option<display::TokenStream<'static>>
where
    T: std::fmt::Display,
{
    use display::toks;
    match (x, y) {
        (None, None) => None,
        (Some(x), Some(y)) => Some(toks(format!("{what}: ({x},{y})"))),
        (Some(x), None) => Some(toks(format!("{what}[x]: {x}"))),
        (None, Some(y)) => Some(toks(format!("{what}[y]: {y}"))),
    }
}

/// Prints a summary of a given `LookupFlag` value, including logic to avoid printing anything for the default flag value.
///
/// Because of this elision, will also print a prefix that separates the displayed content from the previous field
fn display_lookup_flags(flags: &LookupFlag) -> display::TokenStream<'static> {
    fn display_lookup_flag(flags: &LookupFlag) -> display::TokenStream<'static> {
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

        display::Token::from(format!("LookupFlag ({str_flags}{str_macf})")).into()
    }

    if flags.mark_attachment_class_filter != 0
        || flags.right_to_left
        || flags.ignore_ligatures
        || flags.ignore_base_glyphs
        || flags.ignore_marks
        || flags.use_mark_filtering_set
    {
        display::tok(", flags=").then(display_lookup_flag(flags))
    } else {
        // FIXME - add special case for empty-string TokenStream
        display::toks("")
    }
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

fn display_mark_glyph_set(mgs: &MarkGlyphSet, conf: &Config) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, tok, toks};
    tok("\tMarkGlyphSet:").then(LineBreak.then(arrayfmt::display_items_elided(
        &mgs.coverage,
        |ix, item| match item {
            None => toks(format!("\t\t[{ix}]: <none>")).into(),
            Some(covt) => {
                tok(format!("\t\t[{ix}]: ")).then(display_coverage_table_summary(covt, conf))
            }
        },
        conf.bookend_size,
        |start, stop| toks(format!("\t    (skipping coverage tables {start}..{stop})")),
    )))
}

fn display_item_variation_store(
    ivs: &ItemVariationStore,
    conf: &Config,
) -> display::TokenStream<'static> {
    fn display_variation_regions(
        vrl: &VariationRegionList,
        conf: &Config,
    ) -> display::TokenStream<'static> {
        fn display_variation_axes(
            per_region: &[RegionAxisCoordinates],
            conf: &Config,
        ) -> display::TokenStream<'static> {
            use display::toks;
            display::TokenStream::join_with(
                vec![
                    arrayfmt::display_table_column_horiz(
                        "\t\t start |",
                        per_region,
                        |coords| toks(format!("{:.03}", coords.start_coord)),
                        conf.inline_bookend,
                        |n_skipped| toks(format!("..{n_skipped:02}..")),
                    ),
                    arrayfmt::display_table_column_horiz(
                        "\t\t  peak |",
                        per_region,
                        |coords| toks(format!("{:.03}", coords.peak_coord)),
                        conf.inline_bookend,
                        |n_skipped| toks(format!("..{n_skipped:02}..")),
                    ),
                    arrayfmt::display_table_column_horiz(
                        "\t\t   end |",
                        per_region,
                        |coords| toks(format!("{:.03}", coords.end_coord)),
                        conf.inline_bookend,
                        |n_skipped| toks(format!("..{n_skipped:02}..")),
                    ),
                ],
                display::Token::LineBreak,
            )
        }
        use display::{Token::LineBreak, tok, toks};

        tok(format!(
            "\t    VariationRegions: {} regions ({} axes)",
            vrl.0.len(),
            vrl.0[0].len()
        ))
        .then(LineBreak.then(arrayfmt::display_items_elided(
            &vrl.0,
            |ix, per_region| {
                tok(format!("\t\t[{ix}]:"))
                    .then(LineBreak.then(display_variation_axes(per_region, conf)))
            },
            conf.bookend_size,
            |start_ix, end_ix| toks(format!("\t    (skipping regions {start_ix}..{end_ix})")),
        )))
    }
    fn display_variation_data_array(
        ivda: &[Option<ItemVariationData>],
        conf: &Config,
    ) -> display::TokenStream<'static> {
        fn display_variation_data(
            ivd: &ItemVariationData,
            conf: &Config,
        ) -> display::TokenStream<'static> {
            fn display_delta_sets(
                _sets: &DeltaSets,
                _conf: &Config,
            ) -> display::TokenStream<'static> {
                // STUB - figure out what we actually want to show
                toks(format!("\t\t\t<show_delta_sets: incomplete>"))
            }

            use display::{Token::LineBreak, tok, toks};

            let full_bits = if ivd.long_words { 32 } else { 16 };

            tok("ItemVariationData:")
                .then(LineBreak.then(
                    tok(format!("\t\t\t{} region indices: ", ivd.region_index_count)).then(
                        arrayfmt::display_items_inline(
                            &ivd.region_indices,
                            |ix| toks(format!("{ix}")),
                            conf.inline_bookend,
                            |n_skipped| toks(format!("...({n_skipped})...")),
                        ),
                    ),
                ))
                .chain(
                    tok(format!(
                        "\t\t\t{} delta-sets ({} full [{}-bit], {} half [{}-bit]): ",
                        ivd.item_count,
                        ivd.word_count,
                        full_bits,
                        ivd.region_index_count - ivd.word_count,
                        full_bits >> 1
                    ))
                    .then(LineBreak.then(display_delta_sets(&ivd.delta_sets, conf))),
                )
        }
        use display::{Token::LineBreak, tok, toks};
        // WIP - refactor using pre-indent+glue
        tok(format!("\t    ItemVariationData[{}]", ivda.len())).then(LineBreak.then(
            arrayfmt::display_items_elided(
                ivda,
                |ix, o_ivd| match o_ivd {
                    Some(ivd) => {
                        tok(format!("\t\t[{ix}]: ")).then(display_variation_data(ivd, conf))
                    }
                    None => toks(format!("\t\t[{ix}]: <NONE>")),
                },
                conf.bookend_size,
                |start_ix, stop_ix| {
                    toks(format!(
                        "\t    ...(skipping entries {start_ix}..{stop_ix})..."
                    ))
                },
            ),
        ))
    }
    use display::{Token::LineBreak, tok, toks};

    tok("\tItemVariationStore:")
        .then(LineBreak.then(display_variation_regions(&ivs.variation_region_list, conf)))
        .chain(LineBreak.then(display_variation_data_array(
            &ivs.item_variation_data_list,
            conf,
        )))
}

fn display_lig_caret_list(
    lig_caret_list: &LigCaretList,
    conf: &Config,
) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, tok, toks};
    toks("LigCaretList:")
        .glue(
            LineBreak,
            display_coverage_table_summary(&lig_caret_list.coverage, conf).pre_indent(4),
        )
        .glue(
            LineBreak,
            arrayfmt::display_items_elided(
                &lig_caret_list.lig_glyphs,
                |ix, lig_glyph| {
                    toks(format!("[{ix}]: "))
                        .pre_indent(4)
                        .chain(arrayfmt::display_items_inline(
                            &lig_glyph.caret_values,
                            display_caret_value,
                            conf.inline_bookend,
                            |num_skipped| toks(format!("...({num_skipped})...")),
                        ))
                },
                conf.bookend_size,
                |start, stop| toks(format!("(skipping LigGlyphs {start}..{stop})")).pre_indent(3),
            ),
        )

    // NOTE - since coverage tables are used in MarkGlyphSet, we don't want to force-indent within the `show_coverage_table` function, so we do it before instead.
}

fn display_caret_value(cv: &Link<CaretValue>) -> display::TokenStream<'static> {
    // FIXME - refactor each branch to produce tokenstream
    display::toks(match cv {
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
                        format_device_or_variation_index_table(table) // WIP
                    )
                }
            },
        },
    })
}

// TODO - refactor into display model
fn format_device_or_variation_index_table(table: &DeviceOrVariationIndexTable) -> String {
    match table {
        DeviceOrVariationIndexTable::Device(dev_table) => format_device_table(dev_table),
        DeviceOrVariationIndexTable::VariationIndex(var_ix_table) => {
            format_variation_index_table(var_ix_table)
        }
        DeviceOrVariationIndexTable::NonStandard { delta_format } => {
            format!("[<DeltaFormat {delta_format}>]")
        }
    }
}

// TODO - refactor into display model
fn format_device_table(dev_table: &DeviceTable) -> String {
    // REVIEW - we are so far down the stack there is very little we can display inline for the delta-values, but we have them on hand if we wish to show them in some abbreviated form...
    format!("{}..{}", dev_table.start_size, dev_table.end_size)
}

// TODO - refactor into display model
fn format_variation_index_table(var_ix_table: &VariationIndexTable) -> String {
    format!(
        "{}->{}",
        var_ix_table.delta_set_outer_index, var_ix_table.delta_set_inner_index
    )
}

fn display_attach_list(attach_list: &AttachList, conf: &Config) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, tok, toks};
    fn display_attach_point(point_indices: &[u16], conf: &Config) -> display::TokenStream<'static> {
        arrayfmt::display_items_inline(
            point_indices,
            |point_ix| toks(u16::to_string(point_ix)),
            conf.inline_bookend,
            |num_skipped| toks(format!("...({num_skipped})...")),
        )
    }

    // WIP
    toks("AttachList:")
        .pre_indent(2)
        .glue(
            LineBreak,
            display_coverage_table_summary(&attach_list.coverage, conf).pre_indent(4),
        )
        .glue(
            LineBreak,
            arrayfmt::display_items_elided(
                &attach_list.attach_points,
                |ix, AttachPoint { point_indices }| {
                    toks(format!("[{ix}]: "))
                        .pre_indent(4)
                        .chain(display_attach_point(point_indices, conf))
                },
                conf.bookend_size,
                |start, stop| {
                    toks(format!(
                        "(skipping attach points for glyphs {start}..{stop})"
                    ))
                    .pre_indent(3)
                },
            ),
        )
}

fn format_glyphid_hex(glyph: u16, is_standalone: bool) -> String {
    if is_standalone {
        format!("#{glyph:04x}")
    } else {
        format!("{glyph:04x}")
    }
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
        buffer.push_str(&format_glyphid_hex(*glyph, false));
    }
    buffer
}

fn display_coverage_table(cov: &CoverageTable) -> display::TokenStream<'static> {
    use display::{tok, toks};
    match cov {
        CoverageTable::Format1 { glyph_array } => {
            let num_glyphs = glyph_array.len();
            match glyph_array.as_slice() {
                [] => toks("∅"),
                [id] => toks(format!("[{id}]")),
                [first, .., last] => toks(format!("[{num_glyphs} ∈ [{first},{last}]]")),
            }
        }
        CoverageTable::Format2 { range_records } => match range_records.as_slice() {
            [] => toks("∅"),
            [rr] => toks(format!("[∀: {}..={}]", rr.start_glyph_id, rr.end_glyph_id)),
            [first, .., last] => {
                let num_glyphs: u16 = range_records
                    .iter()
                    .map(|rr| rr.end_glyph_id - rr.start_glyph_id + 1)
                    .sum();
                let num_ranges = range_records.len();
                let min_glyph = first.start_glyph_id;
                let max_glyph = last.end_glyph_id;
                toks(format!(
                    "[{num_ranges} ranges; {num_glyphs} ∈ [{min_glyph},{max_glyph}]]"
                ))
            }
        },
    }
}

fn display_coverage_table_summary(
    cov: &CoverageTable,
    conf: &Config,
) -> display::TokenStream<'static> {
    use display::{tok, toks};
    match cov {
        CoverageTable::Format1 { glyph_array } => {
            tok("Glyphs Covered: ").then(arrayfmt::display_items_inline(
                glyph_array,
                |item| toks(format!("{item}")),
                conf.inline_bookend,
                |num_skipped| toks(format!("...({num_skipped})...")),
            ))
        }
        CoverageTable::Format2 { range_records } => {
            tok("Glyph Ranges Covered: ").then(arrayfmt::display_items_inline(
                range_records,
                display_coverage_range_record,
                conf.inline_bookend,
                |num_skipped| toks(format!("...({num_skipped})...")),
            ))
        }
    }
}

fn show_mark_attach_class_def(mark_attach_class_def: &ClassDef, conf: &Config) {
    println!("\tMarkAttachClassDef:");
    show_class_def(mark_attach_class_def, format_mark_attach_class, conf);
}

fn format_mark_attach_class(mark_attach_class: &u16) -> String {
    // STUB - if we come up with a semantic association for specific numbers, add branches here
    format!("{mark_attach_class}")
}

fn show_glyph_class_def(class_def: &ClassDef, conf: &Config) {
    println!("\tGlyphClassDef:");
    show_class_def(class_def, show_glyph_class, conf)
}

fn show_class_def<L: Into<Label>>(
    class_def: &ClassDef,
    show_fn: impl Fn(&u16) -> L,
    conf: &Config,
) {
    use display::{tok, toks};
    // WIP
    match *class_def {
        ClassDef::Format1 {
            start_glyph_id,
            ref class_value_array,
        } => {
            match start_glyph_id {
                0 => (),
                // REVIEW - indent level model might be useful instead of ad-hoc tabs and spaces
                1 => println!("\t    (skipping uncovered glyph 0)"),
                n => println!("\t    (skipping uncovered glyphs 0..{n})"),
            }
            arrayfmt::display_items_elided(
                class_value_array,
                |ix, item| {
                    let gix = start_glyph_id as usize + ix;
                    tok(format!("\t\tGlyph [{gix}]: ")).then(toks(show_fn(item)))
                },
                conf.bookend_size,
                |start, stop| {
                    toks(format!(
                        "\t    (skipping glyphs {}..{})",
                        format_glyphid_hex(start_glyph_id + start as u16, false),
                        format_glyphid_hex(start_glyph_id + stop as u16, false),
                    ))
                },
            )
            .println() // WIP
        }
        ClassDef::Format2 {
            ref class_range_records,
        } => arrayfmt::display_items_elided(
            class_range_records,
            |_ix, class_range| {
                tok(format!(
                    "\t\t({} -> {}): ",
                    class_range.start_glyph_id, class_range.end_glyph_id,
                ))
                .then(toks(show_fn(&class_range.value)))
            },
            conf.bookend_size,
            |start, stop| {
                let low_end = class_range_records[start].start_glyph_id;
                let high_end = class_range_records[stop - 1].end_glyph_id;
                toks(format!(
                    "\t    (skipping ranges covering glyphs {low_end}..={high_end})",
                ))
            },
        )
        .println(), // WIP
    }
}

fn display_coverage_range_record(
    coverage_range: &CoverageRangeRecord,
) -> display::TokenStream<'static> {
    let span = coverage_range.end_glyph_id - coverage_range.start_glyph_id;
    let end_coverage_index = coverage_range.value + span;
    display::toks(format!(
        "({} -> {}): {}(->{})",
        coverage_range.start_glyph_id,
        coverage_range.end_glyph_id,
        coverage_range.value,
        end_coverage_index
    ))
}

fn show_gasp_metrics(gasp: &Option<GaspMetrics>, conf: &Config) {
    if let Some(GaspMetrics {
        version,
        num_ranges,
        ranges,
    }) = gasp
    {
        fn display_gasp_range(_ix: usize, range: &GaspRange) -> display::TokenStream<'static> {
            use display::{tok, toks};

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
                    toks("(no flags)")
                } else {
                    toks(format!("({buffer})"))
                }
            };
            if _ix == 0 && range.range_max_ppem == 0xFFFF {
                tok("\t[∀ PPEM] ").then(disp)
            } else {
                tok(format!("\t[PPEM <= {}]  ", range.range_max_ppem)).then(disp)
            }
        }
        println!("gasp: version {version}, {num_ranges} ranges");
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            arrayfmt::display_items_elided(
                ranges,
                display_gasp_range,
                conf.bookend_size,
                |start, stop| {
                    display::toks(format!(
                        "    skipping gasp ranges for max_ppem values {}..={}",
                        ranges[start].range_max_ppem,
                        ranges[stop - 1].range_max_ppem
                    ))
                },
            )
            .println(); // WIP
        }
    }
}

fn format_version16dot16(v: u32) -> String {
    let major = (v >> 16) as u16;
    let minor = ((v & 0xf000) >> 12) as u16;
    format_version_major_minor(major, minor)
}

fn format_version_major_minor(major: u16, minor: u16) -> String {
    format!("{major}.{minor}")
}

fn show_cmap_metrics(cmap: &Cmap, conf: &Config) {
    use display::{Token::LineBreak, tok, toks};

    tok(format!("cmap: version {}", cmap.version))
        .then(if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            let show_record = |ix: usize, record: &EncodingRecord| {
                // TODO[enrichment]: if we implement subtables and more verbosity levels, show subtable details
                let EncodingRecord {
                    platform,
                    encoding,
                    subtable: _subtable,
                } = record;
                toks(format!(
                    "\t[{ix}]: platform={platform}, encoding={encoding}"
                ))
            };
            LineBreak.then(arrayfmt::display_items_elided(
                &cmap.encoding_records,
                show_record,
                conf.bookend_size,
                |start, stop| toks(format!("\t(skipping encoding records {start}..{stop})")),
            ))
        } else {
            toks(format!(", {} encoding tables", cmap.encoding_records.len()))
        })
        .println() // WIP
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

fn show_vhea_metrics(vhea: &Option<VheaMetrics>, _conf: &Config) {
    if let Some(vhea) = vhea {
        println!(
            "vhea: table version {}, {} vertical long metrics",
            format_version_major_minor(vhea.major_version, vhea.minor_version),
            vhea.num_lvm,
        )
    }
}

fn show_hmtx_metrics(hmtx: &HmtxMetrics, conf: &Config) {
    fn display_unified(ix: usize, hmet: &UnifiedBearing) -> display::TokenStream<'static> {
        use display::toks;
        match &hmet.advance_width {
            Some(width) => toks(format!(
                "\tGlyph ID [{ix}]: advanceWidth={width}, lsb={}",
                hmet.left_side_bearing
            )),
            None => toks(format!("\tGlyph ID [{ix}]: lsb={}", hmet.left_side_bearing)),
        }
    }
    use display::{Token::LineBreak, tok, toks};
    if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
        tok("hmtx")
            .then(LineBreak.then(arrayfmt::display_items_elided(
                &hmtx.0,
                display_unified,
                conf.bookend_size,
                |start, stop| toks(format!("{HT}(skipping hmetrics {start}..{stop})")),
            )))
            .println()
    } else {
        println!("hmtx: {} hmetrics", hmtx.0.len())
    }
}

fn show_vmtx_metrics(vmtx: &Option<VmtxMetrics>, conf: &Config) {
    fn display_unified(ix: usize, vmet: &UnifiedBearing) -> display::TokenStream<'static> {
        use display::{tok, toks};
        const INDENT_LEVEL: u8 = 2;

        match &vmet.advance_width {
            // FIXME - `_width` is a misnomer, should be `_height`
            Some(height) => toks(format!(
                "Glyph ID [{ix}]: advanceHeight={height}, tsb={}",
                vmet.left_side_bearing // FIXME - `left` is a misnomer, should be `top`
            ))
            .pre_indent(INDENT_LEVEL),
            None => toks(format!("Glyph ID [{ix}]: tsb={}", vmet.left_side_bearing))
                .pre_indent(INDENT_LEVEL), // FIXME - `left` is a misnomer, should be `top`
        }
    }
    use display::{Token::LineBreak, tok, toks};

    // FIXME - add mechanism for auto-computation of current indent level
    const INDENT_LEVEL: u8 = 1;
    if let Some(vmtx) = vmtx {
        let disp = if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            tok("vmtx").then(LineBreak.then(arrayfmt::display_items_elided(
                &vmtx.0,
                display_unified,
                conf.bookend_size,
                |start, stop| {
                    toks(format!("(skipping vmetrics {start}..{stop})")).pre_indent(INDENT_LEVEL)
                },
            )))
        } else {
            toks(format!("vmtx: {} vmetrics", vmtx.0.len()))
        };
        disp.println()
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
                        println!("\tFull Font Name: {buf}");
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
#[inline(always)]
const fn as_s16(v: u16) -> i16 {
    v as i16
}

fn show_glyf_metrics(glyf: &Option<GlyfMetrics>, conf: &Config) {
    fn display_glyph_metric(ix: usize, glyf: &GlyphMetric) -> display::TokenStream<'static> {
        use display::{tok, toks};
        const INDENT_LEVEL: u8 = 2;

        tok(format!("[{ix}]: "))
            .then(match glyf {
                GlyphMetric::Empty => toks("<empty>"),
                GlyphMetric::Simple(simple) => toks(format!(
                    "Simple Glyph [{} contours, {} coordinates, {} instructions, xy: {}]",
                    simple.contours, simple.coordinates, simple.instructions, simple.bounding_box
                )),
                GlyphMetric::Composite(composite) => toks(format!(
                    "Composite Glyph [{} components, {} instructions, xy: {}]",
                    composite.components, composite.instructions, composite.bounding_box,
                )),
            })
            .pre_indent(INDENT_LEVEL)
    }

    use display::{Token::LineBreak, tok, toks};
    const INDENT_LEVEL: u8 = 1;
    let disp = if let Some(glyf) = glyf.as_ref() {
        let hdr = tok(format!("glyf: {} glyphs", glyf.num_glyphs));
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            hdr.then(LineBreak.then(arrayfmt::display_items_elided(
                glyf.glyphs.as_slice(),
                display_glyph_metric,
                conf.bookend_size,
                |start, stop| {
                    toks(format!("(skipping glyphs {start}..{stop})")).pre_indent(INDENT_LEVEL)
                },
            )))
        } else {
            hdr.into()
        }
    } else {
        toks("glyf: <not present>")
    };
    disp.println()
}

pub mod lookup_subtable {
    use super::{
        OpentypeGposLookupSubtable, OpentypeGposLookupSubtableExt, OpentypeGsubLookupSubtable,
        OpentypeGsubLookupSubtableExt, Parser, TestResult, UnknownValueError,
    };
    use crate::{
        Decoder_opentype_main,
        api_helper::otf_metrics::{Mandatory, obj},
        opentype_main_directory, opentype_ttc_header_header,
    };

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
                            what: "ttc header version".to_string(),
                            bad_value: n,
                        }));
                    }
                    opentype_ttc_header_header::Version1(v1header) => {
                        let mut lookup_metrics =
                            Vec::with_capacity(v1header.table_directories.len());
                        for font in v1header.table_directories.iter() {
                            let per_font = font.link.as_ref().map(analyze_table_directory_lookups);
                            lookup_metrics.push(per_font);
                        }
                        lookup_metrics
                    }
                    opentype_ttc_header_header::Version2(v2header) => {
                        let mut lookup_metrics =
                            Vec::with_capacity(v2header.table_directories.len());
                        for font in v2header.table_directories.iter() {
                            let per_font = font.link.as_ref().map(analyze_table_directory_lookups);
                            lookup_metrics.push(per_font);
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
            .map(|gpos| super::reify(gpos, Mandatory(obj::PosLookups)))
        {
            for entry in super::reify_all(&lookup_list, Mandatory(obj::PosLookupTable)) {
                if let Some(subtable) = super::reify_all(&entry, Mandatory(obj::PosSubtable)).next()
                {
                    let ground = match &subtable {
                        OpentypeGposLookupSubtableExt::PosExtension(ext) => {
                            ret.pos_extension = true;
                            &super::reify(ext, super::obj::PosLookup)
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
            .map(|gsub| super::reify(gsub, Mandatory(obj::SubstLookups)))
        {
            for entry in super::reify_all(&lookup_list, Mandatory(obj::SubstLookupTable)) {
                if let Some(subtable) = super::reify_all(&entry, super::obj::SubstSubtable).next() {
                    let ground = match &subtable {
                        OpentypeGsubLookupSubtableExt::SubstExtension(ext) => {
                            ret.subst_extension = true;
                            &super::reify(ext, super::obj::SubstLookup)
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

mod display {

    pub fn tok(str: impl Into<doodle::Label>) -> Token {
        Token::InlineText(str.into())
    }

    pub fn toks(str: impl Into<doodle::Label>) -> TokenStream<'static> {
        TokenStream::from(Token::InlineText(str.into()))
    }

    #[derive(Clone)]
    pub enum Token {
        InlineText(doodle::Label),
        LineBreak,
    }

    impl std::fmt::Display for Token {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Token::InlineText(s) => write!(f, "{s}"),
                Token::LineBreak => write!(f, "\n"),
            }
        }
    }

    impl Token {
        pub fn then(self, stream: TokenStream<'_>) -> TokenStream<'_> {
            <Token as Into<TokenStream<'static>>>::into(self).chain(stream)
        }
    }

    impl From<String> for Token {
        fn from(s: String) -> Self {
            Token::InlineText(s.into())
        }
    }

    impl From<Token> for TokenStream<'static> {
        fn from(value: Token) -> Self {
            TokenStream {
                inner: Box::new(std::iter::once(value)),
            }
        }
    }

    #[must_use]
    pub struct TokenStream<'a> {
        inner: Box<dyn Iterator<Item = Token> + 'a>,
    }

    impl<'a> TokenStream<'a> {
        // FIXME - join_with will double-separate if empty is encountered
        pub fn empty() -> TokenStream<'static> {
            TokenStream {
                inner: Box::new(std::iter::empty()),
            }
        }

        pub fn from_stream(stream: impl Iterator<Item = Token> + 'a) -> Self {
            TokenStream {
                inner: Box::new(stream),
            }
        }

        pub fn write_to<W: std::io::Write>(self, mut w: W) -> std::io::Result<()> {
            for token in self.inner {
                write!(w, "{token}")?
            }
            Ok(())
        }

        pub fn print(self) {
            let oput = std::io::stdout().lock();
            let mut buf = std::io::BufWriter::new(oput);
            self.write_to(&mut buf).unwrap();
        }

        pub fn println(self) {
            self.chain(TokenStream::from(Token::LineBreak)).print()
        }

        pub fn into_string(self) -> String {
            let mut buf = String::new();
            for token in self.inner {
                buf.push_str(&token.to_string());
            }
            buf
        }

        pub fn group_lines(self) -> TokenStream<'static> {
            let mut lines = Vec::new();
            let mut line = String::new();
            for token in self.inner {
                match token {
                    Token::InlineText(s) => line.push_str(&s),
                    Token::LineBreak => {
                        lines.push(Token::InlineText(std::borrow::Cow::Owned(line.clone())));
                        line.clear();
                    }
                }
            }
            TokenStream {
                inner: Box::new(lines.into_iter()),
            }
        }

        pub fn chain(self, other: Self) -> Self {
            TokenStream {
                inner: Box::new(self.inner.chain(other.inner)),
            }
        }

        pub fn glue(self, glue: Token, other: Self) -> Self {
            Self {
                inner: Box::new(self.inner.chain(std::iter::once(glue).chain(other.inner))),
            }
        }

        pub fn surround(self, before: Token, after: Token) -> Self {
            Self {
                inner: Box::new(
                    std::iter::once(before)
                        .chain(self.inner)
                        .chain(std::iter::once(after)),
                ),
            }
        }

        /// Surrounds the TokenStream with `'('..')'`
        pub fn paren(self) -> Self {
            self.surround(tok("("), tok(")"))
        }

        /// Surrounds the TokenStream with `'['..']'`
        pub fn bracket(self) -> Self {
            self.surround(tok("["), tok("]"))
        }

        pub fn break_line(self) -> TokenStream<'a> {
            TokenStream {
                inner: Box::new(self.inner.chain(std::iter::once(Token::LineBreak))),
            }
        }

        /// Prepends indentation to the first line of the stream, measured 4-space half-tabs.
        ///
        /// Will prefer using `'\t'` for indentation, and will include a final half-tab iff `stops` is odd.
        pub fn pre_indent(self, stops: u8) -> Self {
            let tabs = stops / 2;
            let hts = stops % 2;

            let mut indent = String::new();
            for _ in 0..tabs {
                indent.push('\t');
            }
            for _ in 0..hts {
                indent.push_str(super::HT);
            }
            tok(indent).then(self)
        }

        pub fn join_with(streams: Vec<TokenStream<'static>>, sep: Token) -> TokenStream<'a> {
            TokenStream {
                inner: Box::new(IntersperseIter::new(
                    Box::new(streams.into_iter().map(|s| s.inner)),
                    sep,
                )),
            }
        }
    }

    pub struct IntersperseIter<'a, T: Clone> {
        items: Box<dyn Iterator<Item = Box<dyn Iterator<Item = T> + 'a>>>,
        rest: Box<dyn Iterator<Item = T> + 'a>,
        sep: T,
        non_empty: bool,
    }

    impl<'a, T: 'static + Clone> IntersperseIter<'a, T> {
        pub fn new(
            items: Box<dyn Iterator<Item = Box<dyn Iterator<Item = T> + 'a>>>,
            sep: T,
        ) -> Self {
            Self {
                items,
                rest: Box::new(std::iter::empty()),
                sep,
                non_empty: false,
            }
        }
    }

    impl<'a, T: Clone> std::iter::Iterator for IntersperseIter<'a, T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.rest.next() {
                None => match self.items.next() {
                    None => None,
                    Some(iter) => {
                        self.rest = iter;
                        if self.non_empty {
                            self.non_empty = false;
                            Some(self.sep.clone())
                        } else {
                            self.next()
                        }
                    }
                },
                Some(item) => {
                    self.non_empty = true;
                    Some(item)
                }
            }
        }
    }
}

// TODO - rewrite the functions to either be Write-generic or used Fragment-like output model to avoid duplication between I/O show and String formatting functions
mod arrayfmt {
    use super::display::{
        Token::{self, LineBreak},
        TokenStream, tok, toks,
    };
    use crate::api_helper::util::{EnumLen, Wec, trisect_unchecked};

    /// Generic helper for displaying an array of possibly-None elements, skipping over
    /// all None-values and only showing up to the first, and last `N` elements, where
    /// `N` is determined by `bookend` (shows all elements if the lenght is less than or equal to `2 * bookend`).
    ///
    /// The `show_fn` function is used to display individual elements, and takes both the index and the value of the element in question.
    ///
    /// The `ellipsis` function is used to signal information about the skipped middle-elements (if any), and takes two arguments:
    /// the number of elements that were skipped over (after filtering out None-values), and a tuple `(start, stop)` where `start` is the first
    /// element-index skipped and `stop` is the element-index where display resumes.
    pub(crate) fn display_nullable<T>(
        opt_items: &[Option<T>],
        mut show_fn: impl FnMut(usize, &T) -> TokenStream<'static>,
        bookend: usize,
        ellipsis: impl Fn(usize, (usize, usize)) -> TokenStream<'static>,
    ) -> TokenStream<'static> {
        let items: Vec<(usize, &T)> = opt_items
            .iter()
            .enumerate()
            .filter_map(|(ix, opt)| opt.as_ref().map(|v| (ix, v)))
            .collect();
        let mut buffer =
            Vec::<TokenStream<'static>>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

        let count = items.len();

        if count > bookend * 2 {
            let (left_bookend, middle, right_bookend) =
                unsafe { trisect_unchecked(&items, bookend, bookend) };

            for (ix, it) in left_bookend {
                buffer.push(show_fn(*ix, it));
            }

            let n_skipped = count - bookend * 2;
            assert_eq!(middle.len(), n_skipped);
            buffer.push(ellipsis(n_skipped, (middle[0].0, middle[n_skipped - 1].0)));

            for (ix, it) in right_bookend {
                buffer.push(show_fn(*ix, it));
            }
        } else {
            for (ix, it) in items.into_iter() {
                buffer.push(show_fn(ix, it));
            }
        }
        TokenStream::join_with(buffer, LineBreak)
    }

    /// Generic helper for displaying an array of possibly-None elements, skipping over
    /// all None-values and only showing up to the first, and last `N` elements, where
    /// `N` is determined by `bookend` (shows all elements if the lenght is less than or equal to `2 * bookend`).
    ///
    /// The `show_fn` function is used to display individual elements, and takes both the index and the value of the element in question.
    ///
    /// The `ellipsis` function is used to signal information about the skipped middle-elements (if any), and takes two arguments:
    /// the number of elements that were skipped over (after filtering out None-values), and a tuple `(start, stop)` where `start` is the first
    /// element-index skipped and `stop` is the element-index where display resumes.
    pub(crate) fn display_inline_nullable<T>(
        opt_items: &[Option<T>],
        mut show_fn: impl FnMut(usize, &T) -> TokenStream<'static>,
        bookend: usize,
        ellipsis: impl Fn(usize, (usize, usize)) -> TokenStream<'static>,
    ) -> TokenStream<'static> {
        let items: Vec<(usize, &T)> = opt_items
            .iter()
            .enumerate()
            .filter_map(|(ix, opt)| opt.as_ref().map(|v| (ix, v)))
            .collect();
        let mut buffer =
            Vec::<TokenStream<'static>>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

        let count = items.len();

        if count > bookend * 2 {
            let (left_bookend, middle, right_bookend) =
                unsafe { trisect_unchecked(&items, bookend, bookend) };

            for (ix, it) in left_bookend {
                buffer.push(show_fn(*ix, it));
            }

            let n_skipped = count - bookend * 2;
            assert_eq!(middle.len(), n_skipped);
            buffer.push(ellipsis(n_skipped, (middle[0].0, middle[n_skipped - 1].0)));

            for (ix, it) in right_bookend {
                buffer.push(show_fn(*ix, it));
            }
        } else {
            for (ix, it) in items.into_iter() {
                buffer.push(show_fn(ix, it));
            }
        }
        TokenStream::join_with(buffer, tok(", ")).bracket()
    }

    /// Generic helper for displaying an array of elements, showing at most `bookend` elements at the start and end of the array and a single ellipsis if the array is longer than `2 * bookend`.
    ///
    /// Each element that is to be displayed is formatted using the provided closure `fmt_fn`.
    ///
    /// All elements will be written on the same line, and so the `fmt_fn` closure should not include any line-breaks.
    ///
    /// The `ellipsis` function is used to signal information about the skipped middle-elements (if any), and takes the number of elements that were skipped over.
    pub(crate) fn display_items_inline<T>(
        items: &[T],
        mut fmt_fn: impl FnMut(&T) -> TokenStream<'static>,
        bookend: usize,
        ellipsis: impl Fn(usize) -> TokenStream<'static>,
    ) -> TokenStream<'static> {
        // Allocate a buffer big enough to hold one string per item in the array, or enough items to show both bookends and one ellipsis-string
        let mut buffer =
            Vec::<TokenStream<'static>>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

        let count = items.len();
        if count > bookend * 2 {
            for item in &items[..bookend] {
                buffer.push(fmt_fn(item));
            }

            buffer.push(ellipsis(count - bookend * 2));

            for item in &items[count - bookend..] {
                buffer.push(fmt_fn(item));
            }
        } else {
            buffer.extend(items.iter().map(fmt_fn));
        }
        TokenStream::join_with(buffer, tok(", ")).bracket()
    }

    /// Enumerates the contents of a slice, showing only the first and last `bookend` items if the slice is long enough.
    ///
    /// Each item is shown with `show_fn`, and `fn_message` is used to signal the range of indices skipped.
    /// If the slice length is less than or equal to `2 * bookend`, all elements are displayed and `ellipsis` is not called.
    pub(crate) fn display_items_elided<T>(
        items: &[T],
        show_fn: impl Fn(usize, &T) -> TokenStream<'static>,
        bookend: usize,
        ellipsis: impl Fn(usize, usize) -> TokenStream<'static>,
    ) -> TokenStream<'static> {
        let mut buffer =
            Vec::<TokenStream<'static>>::with_capacity(Ord::min(items.len(), bookend * 2 + 1));

        let count = items.len();
        if count > bookend * 2 {
            for (ix, item) in items.iter().enumerate().take(bookend) {
                buffer.push(show_fn(ix, item));
            }

            buffer.push(ellipsis(bookend, count - bookend));

            for (ix, item) in items.iter().enumerate().skip(count - bookend) {
                buffer.push(show_fn(ix, item));
            }
        } else {
            buffer.extend(items.iter().enumerate().map(|(ix, item)| show_fn(ix, item)));
        }
        TokenStream::join_with(buffer, LineBreak)
    }

    // Enumerates the contents of a Wec<T>, showing only the first and last `bookend` rows if the Wec is tall enough.
    ///
    /// Each row is shown with `show_fn`, and the `elision_message` is printed (along with the range of indices skipped)
    /// if the slice length exceeds than 2 * `bookend`, in between the initial and terminal span of `bookend` items.
    // TODO - move into arrayfmt module
    pub(crate) fn display_wec_rows_elided<T>(
        matrix: &Wec<T>,
        show_fn: impl Fn(usize, &[T]) -> TokenStream<'static>,
        bookend: usize,
        fn_message: impl Fn(usize, usize) -> TokenStream<'static>,
    ) -> TokenStream<'static> {
        let count = matrix.rows();
        let mut lines = Vec::with_capacity(Ord::min(count, bookend * 2 + 1));

        if count > bookend * 2 {
            for ix in 0..bookend {
                lines.push(show_fn(ix, &matrix[ix]));
            }
            lines.push(fn_message(bookend, count - bookend));
            for ix in (count - bookend)..count {
                lines.push(show_fn(ix, &matrix[ix]));
            }
        } else {
            let mut lines = Vec::with_capacity(count);
            lines.extend(
                matrix
                    .iter_rows()
                    .enumerate()
                    .map(|(ix, row)| show_fn(ix, row)),
            );
        }
        TokenStream::join_with(lines, LineBreak)
    }

    pub(crate) fn display_coverage_linked_array<T>(
        items: &[T],
        coverage: impl Iterator<Item = u16>,
        mut fmt_fn: impl FnMut(u16, &T) -> TokenStream<'static>,
        bookend: usize,
        ellipsis: impl FnOnce(usize) -> TokenStream<'static>,
    ) -> TokenStream<'static> {
        let count = items.len();
        let mut buffer = Vec::with_capacity(Ord::min(count, bookend * 2 + 1));

        let mut ix_iter = EnumLen::new(coverage, count);

        if count > bookend * 2 {
            for (ix, glyph_id) in ix_iter.iter_with().take(bookend) {
                buffer.push(fmt_fn(glyph_id, &items[ix]));
            }

            let n_skipped = count - bookend * 2;

            buffer.push(ellipsis(n_skipped));

            for (ix, glyph_id) in ix_iter.iter_with().skip(n_skipped).take(bookend) {
                buffer.push(fmt_fn(glyph_id, &items[ix]));
            }
        } else {
            for (ix, glyph_id) in ix_iter.iter_with() {
                buffer.push(fmt_fn(glyph_id, &items[ix]));
            }
        }

        // NOTE - boolean control-flag for strictness; when false, will not return an error if there are leftover items in the coverage iterator
        const FORBID_LEFTOVER_COVERAGE: bool = false;

        match ix_iter.finish(FORBID_LEFTOVER_COVERAGE) {
            Ok(_) => {}
            Err(e) => panic!("format_coverage_linked_array found error: {e}"),
        }
        TokenStream::join_with(buffer, tok(", ")).bracket()
    }

    pub(crate) fn display_table_column_horiz<A>(
        heading: &'static str,
        items: &[A],
        mut show_fn: impl FnMut(&A) -> TokenStream<'static>,
        bookend: usize,
        ellipsis: impl FnOnce(usize) -> TokenStream<'static>,
    ) -> TokenStream<'static> {
        let count = items.len();
        let mut buf = Vec::with_capacity(Ord::min(count, 2 * bookend + 1));
        if count > 2 * bookend {
            let (left_bookend, _middle, right_bookend) =
                unsafe { trisect_unchecked(items, bookend, bookend) };

            for it in left_bookend {
                buf.push(show_fn(it));
            }

            let n_skipped = count - bookend * 2;
            buf.push(ellipsis(n_skipped));

            for it in right_bookend {
                buf.push(show_fn(it));
            }
        } else {
            buf.extend(items.iter().map(show_fn));
        }

        tok(heading).then(TokenStream::join_with(buf, tok(" ")))
    }
}
