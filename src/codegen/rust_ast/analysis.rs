use std::{cell::RefCell, collections::HashMap};

pub(crate) mod heap_optimize;
pub(crate) mod read_width;
use crate::codegen::model::{READ_ARRAY_IS_COPY, VIEW_OBJECT_IS_COPY};

use super::*;
use heap_optimize::{HeapOptimize, HeapOutcome, HeapStrategy};
use read_width::{ReadWidth, ValueWidth};

/// Helper trait for any AST model-type that represents a type-construct in Rust that may rely on non-local
/// context to properly analyze certain static properties of.
pub trait ASTContext {
    type Context<'a>: Sized + 'a;
}

fn niche_product(iter: impl Iterator<Item = usize>) -> usize {
    iter.map(|x| x.saturating_add(1))
        .fold(1usize, usize::saturating_mul)
        - 1
}

fn niche_sum(iter: impl Iterator<Item = usize>) -> usize {
    iter.fold(0usize, usize::saturating_add)
}

/// Helper trait for any AST type-rep that might be subject to specialized optimization
/// when wrapped in `Option` (e.g. `Option<&T>`, `Option<bool>`).
pub trait CanOptimize: ASTContext {
    fn niches(&self, context: Self::Context<'_>) -> usize;

    fn is_optimized(&self, context: Self::Context<'_>) -> bool {
        self.niches(context) > 0
    }
}

pub trait MemSize: CanOptimize {
    fn size_hint(&self, context: Self::Context<'_>) -> usize;

    fn align_hint(&self, context: Self::Context<'_>) -> usize;
}

pub trait CopyEligible: ASTContext {
    fn copy_hint(&self, context: Self::Context<'_>) -> bool;
}

#[derive(Default)]
struct CacheEntry {
    copy: Option<bool>,
    niches: Option<usize>,
    /// Number of machine-bytes a stored type-value takes up, or best approximation
    size: Option<usize>,
    /// Number of buffer-bytes that will be advanced after fully parsing a single value of this type
    width: Option<ValueWidth>,
    align: Option<usize>,
    heap: Option<HeapOutcome>,
}

pub(crate) struct SourceContext<'a> {
    def_map: &'a [RustTypeDecl],
    cache: Rc<RefCell<HashMap<usize, CacheEntry>>>,
}

macro_rules! cache_get {
    ( $this:expr, $field:ident, $ix:ident, $method:ident $( , $pre_arg:expr )? ) => {{
        let cache = $this.cache.borrow();
        if cache.contains_key(&$ix) {
            match &cache[&$ix].$field {
                Some(ret) => ret.clone(),
                None => {
                    drop(cache);
                    let ret = {
                        let def = &$this.def_map[$ix];
                        def.$method($( $pre_arg, )? $this)
                    };
                    $this.cache.borrow_mut().get_mut(&$ix).unwrap().$field = Some(ret.clone());
                    ret
                }
            }
        } else {
            drop(cache);
            let ret = {
                let def = &$this.def_map[$ix];
                def.$method($( $pre_arg, )? $this)
            };
            $this.cache.borrow_mut().entry($ix).or_default().$field = Some(ret.clone());
            ret
        }
    }};
}

impl SourceContext<'_> {
    pub fn get_def(&self, ix: usize) -> &RustTypeDecl {
        &self.def_map[ix]
    }

    pub fn get_niches(&self, ix: usize) -> usize {
        cache_get!(self, niches, ix, niches)
    }

    pub fn get_size(&self, ix: usize) -> usize {
        cache_get!(self, size, ix, size_hint)
    }

    pub fn get_width(&self, ix: usize) -> ValueWidth {
        cache_get!(self, width, ix, read_width)
    }

    pub fn get_align(&self, ix: usize) -> usize {
        cache_get!(self, align, ix, align_hint)
    }

    pub fn get_copy(&self, ix: usize) -> bool {
        cache_get!(self, copy, ix, copy_hint)
    }

    /// NOTE: Due to memoization, only one strategy will be computed for a given type-definition,
    /// meaning that if you then request a different strategy with the same Context object, the old
    /// result will be served up regardless of the actual strategy being passed in the second time
    /// around.
    pub fn get_heap(&self, strategy: HeapStrategy, ix: usize) -> HeapOutcome {
        cache_get!(self, heap, ix, heap_hint, strategy)
    }
}

impl<'a> From<&'a [RustTypeDecl]> for SourceContext<'a> {
    fn from(def_map: &'a [RustTypeDecl]) -> Self {
        SourceContext {
            def_map,
            cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl ASTContext for RustType {
    type Context<'a> = &'a SourceContext<'a>;
}

impl CanOptimize for RustType {
    fn niches(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustType::Atom(at) => at.niches(context),
            RustType::AnonTuple(ts) => niche_product(ts.iter().map(|t| t.niches(context))),
            // conservative estimate based on our assumption we won't see any Verbatim types in gencode structs
            RustType::Verbatim(..) => 0,
            // in actuality, ReadArray has many more niches, but we cannot calculate it reliably because it is an external definition
            RustType::ReadArray(..) => 1,
            // in actuality, ViewObject has many more niches, but we can't predictably calculate them without locking in the backing-type implementation
            RustType::ViewObject(..) => 1,
        }
    }
}

impl MemSize for RustType {
    fn size_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustType::Atom(at) => at.size_hint(context),
            RustType::AnonTuple(ts) => {
                let mut raw_size = 0;
                let mut max_align = 1;
                for t in ts.iter() {
                    raw_size += t.size_hint(context);
                    max_align = max_align.max(t.align_hint(context));
                }
                aligned_size(raw_size, max_align)
            }
            RustType::Verbatim(..) => {
                unreachable!("unexpected RustType::Verbatim in structural type")
            }
            RustType::ReadArray(..) => {
                // FIXME - this is subject to external implementation details
                let sz_scope = {
                    let sz_slice = std::mem::size_of::<&[u8]>();
                    let sz_base = std::mem::size_of::<usize>();
                    sz_slice + sz_base
                };
                let sz_length = std::mem::size_of::<usize>();
                let sz_stride = std::mem::size_of::<usize>();
                sz_scope + sz_length + sz_stride
            }
            RustType::ViewObject(..) => {
                let sz_buffer = std::mem::size_of::<&[u8]>();
                let sz_start_offs = std::mem::size_of::<usize>();
                sz_buffer + sz_start_offs
            }
        }
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustType::Atom(at) => at.align_hint(context),
            RustType::AnonTuple(ts) => {
                // Corner case - if the tuple is empty, (i.e. non-canonical unit), return `align_of::<()>() == 1`.
                let mut ret = 1;
                for t in ts.iter() {
                    ret = ret.max(t.align_hint(context));
                }
                ret
            }
            RustType::Verbatim(..) => {
                unreachable!("unexpected RustType::Verbatim in structural type")
            }
            RustType::ReadArray(..) | RustType::ViewObject(..) => {
                // FIXME - this is subject to external implementation details
                std::mem::align_of::<usize>()
            }
        }
    }
}

impl CopyEligible for RustType {
    fn copy_hint(&self, context: &SourceContext<'_>) -> bool {
        match self {
            RustType::Atom(at) => at.copy_hint(context),
            RustType::AnonTuple(ts) => ts.iter().all(|t| t.copy_hint(context)),
            RustType::Verbatim(..) => {
                unreachable!("unexpected RustType::Verbatim in structural type")
            }
            RustType::ReadArray(..) => READ_ARRAY_IS_COPY,
            RustType::ViewObject(..) => VIEW_OBJECT_IS_COPY,
        }
    }
}

impl ASTContext for AtomType {
    type Context<'a> = &'a SourceContext<'a>;
}

impl CanOptimize for AtomType {
    fn niches(&self, context: &SourceContext<'_>) -> usize {
        match self {
            AtomType::Prim(pt) => pt.niches(()),
            AtomType::Comp(ct) => ct.niches(context),
            AtomType::TypeRef(lt) => lt.niches(context),
        }
    }
}

impl MemSize for AtomType {
    fn size_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            AtomType::Prim(pt) => pt.size_hint(()),
            AtomType::Comp(ct) => ct.size_hint(context),
            AtomType::TypeRef(lt) => lt.size_hint(context),
        }
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            AtomType::Prim(pt) => pt.align_hint(()),
            AtomType::Comp(ct) => ct.align_hint(context),
            AtomType::TypeRef(lt) => lt.align_hint(context),
        }
    }
}

impl CopyEligible for AtomType {
    fn copy_hint(&self, context: &SourceContext<'_>) -> bool {
        match self {
            AtomType::Prim(pt) => pt.copy_hint(()),
            AtomType::Comp(ct) => ct.copy_hint(context),
            AtomType::TypeRef(lt) => lt.copy_hint(context),
        }
    }
}

macro_rules! one_to_one {
    ( size $self:expr , $( $variant:ident => $ty:ty ),+ $(,)? ) => {
        match $self {
            $( Self::$variant => size_of::<$ty>(), )+
        }
    };
    ( align $self:expr , $( $variant:ident => $ty:ty ),+ $(,)? ) => {
        match $self {
            $( Self::$variant => align_of::<$ty>(), )+
        }
    }
}

impl ASTContext for PrimType {
    type Context<'a> = ();
}

// Maximum valid char, beyond which is the first value-niche
const UTF16_SCALAR_MAX: usize = 0x10FFF;

impl CanOptimize for PrimType {
    fn niches(&self, _: ()) -> usize {
        match self {
            PrimType::Unit => 0,
            PrimType::Bool => const { (u8::MAX as usize + 1) - 2 },
            // Because Char is Unicode, there are invalid ranges that form niches
            PrimType::Char => match char::UNICODE_VERSION {
                (16, 0, 0) => const { u32::MAX as usize - UTF16_SCALAR_MAX },
                _ => unimplemented!("unsupported Unicode version"),
            },
            PrimType::U8 | PrimType::U16 | PrimType::U32 | PrimType::U64 | PrimType::Usize => 0,
        }
    }
}

impl MemSize for PrimType {
    fn size_hint(&self, _: ()) -> usize {
        one_to_one! { size self,
            Unit => (),
            U8 => u8,
            U16 => u16,
            U32 => u32,
            U64 => u64,
            Bool => bool,
            Char => char,
            Usize => usize
        }
    }

    fn align_hint(&self, _: ()) -> usize {
        one_to_one! { align self,
            Unit => (),
            U8 => u8,
            U16 => u16,
            U32 => u32,
            U64 => u64,
            Bool => bool,
            Char => char,
            Usize => usize,
        }
    }
}

impl CopyEligible for PrimType {
    fn copy_hint(&self, _: ()) -> bool {
        // NOTE - as implemented, all PrimTypes are copy, but we don't want to hardcode this and forget if we add non-Copy primtypes later on
        matches!(
            self,
            PrimType::U8
                | PrimType::U16
                | PrimType::U32
                | PrimType::U64
                | PrimType::Usize
                | PrimType::Unit
                | PrimType::Bool
                | PrimType::Char
        )
    }
}

/// Local choice of what type to embed as the parameter to make `Vec` a concrete type-instance we can pass into
/// `size_of` and `align_of` methods.
type VecFiller = u8;

impl<T> ASTContext for CompType<Box<T>>
where
    T: ASTContext,
{
    type Context<'a> = T::Context<'a>;
}

impl<T> MemSize for CompType<Box<T>>
where
    T: MemSize + CanOptimize + std::fmt::Debug,
    for<'a> T::Context<'a>: Copy,
{
    fn size_hint(&self, context: Self::Context<'_>) -> usize {
        match self {
            CompType::Vec(..) => size_of::<Vec<VecFiller>>(),
            CompType::Option(inner) => {
                if inner.is_optimized(context) {
                    inner.size_hint(context)
                } else {
                    inner.size_hint(context) + inner.align_hint(context)
                }
            }
            CompType::Result(..) => unimplemented!("unexpected result in structural type"),
            CompType::Borrow(..) => size_of::<usize>(),
            CompType::RawSlice(..) => unimplemented!("unexpected raw slice in structural type"),
        }
    }

    fn align_hint(&self, context: Self::Context<'_>) -> usize {
        match self {
            CompType::Vec(..) => align_of::<Vec<VecFiller>>(),
            CompType::Option(inner) => inner.align_hint(context),
            CompType::Result(..) => unimplemented!("unexpected result in structural type"),
            CompType::Borrow(..) => align_of::<usize>(),
            CompType::RawSlice(..) => unimplemented!("unexpected raw slice in structural type"),
        }
    }
}

impl<T> CanOptimize for CompType<Box<T>>
where
    T: CanOptimize + MemSize + std::fmt::Debug,
    for<'a> T::Context<'a>: Copy,
{
    fn niches(&self, context: Self::Context<'_>) -> usize {
        match self {
            // Vec<T> has enough niches that all values of `n: u8` are optimizable
            CompType::Vec(..) => usize::MAX,
            CompType::Option(inner) => match inner.niches(context) {
                0 => match inner.align_hint(context) {
                    n @ 1..=7 => (1 << (8 * n)) - 1,
                    8 => usize::MAX,
                    n => unreachable!("align of {n} is not an expected case: {inner:?}"),
                },
                n => n - 1,
            },
            // Option<&T> cannot be optimized, but &T itself and Option<Option<&T>> (and above) can be
            CompType::Borrow(..) => 1,
            CompType::Result(..) => unreachable!("unexpected result in structural type"),
            CompType::RawSlice(..) => unimplemented!("unexpected raw slice in structural type"),
        }
    }
}

impl<T> CopyEligible for CompType<Box<T>>
where
    T: CopyEligible + std::fmt::Debug,
    for<'a> T::Context<'a>: Copy,
{
    fn copy_hint(&self, context: Self::Context<'_>) -> bool {
        match self {
            CompType::Borrow(_lt, Mut::Immutable, _) => true,
            CompType::Borrow(_lt, Mut::Mutable, _) => {
                unreachable!("unexpected mutable borrow in generative type: {self:?}")
            }
            CompType::Vec(..) => false,
            CompType::RawSlice(..) => unreachable!("unexpected raw slice in structural type"),
            CompType::Option(inner) => inner.copy_hint(context),
            CompType::Result(ok_t, err_t) => ok_t.copy_hint(context) && err_t.copy_hint(context),
        }
    }
}

impl ASTContext for LocalType {
    type Context<'a> = &'a SourceContext<'a>;
}

impl CanOptimize for LocalType {
    fn niches(&self, context: &SourceContext<'_>) -> usize {
        match self {
            // Note - this can be circular if we are not careful, but we don't expect circularity in practice
            LocalType::LocalDef(ix, ..) => context.get_niches(*ix),
            LocalType::External(_) => {
                unreachable!("unexpected external type-reference in structural type")
            }
        }
    }
}

impl MemSize for LocalType {
    fn size_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            LocalType::LocalDef(ix, ..) => context.get_size(*ix),
            LocalType::External(_) => {
                unreachable!("unexpected external type-reference in structural type")
            }
        }
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            LocalType::LocalDef(ix, ..) => context.get_align(*ix),
            LocalType::External(_) => {
                unreachable!("unexpected external type-reference in structural type")
            }
        }
    }
}

impl CopyEligible for LocalType {
    fn copy_hint(&self, context: &SourceContext<'_>) -> bool {
        match self {
            LocalType::LocalDef(ix, ..) => context.get_copy(*ix),
            LocalType::External(ext_type) => {
                unreachable!(
                    "unexpected external type-reference encountered during copy-analysis: {ext_type}"
                )
            }
        }
    }
}

impl ASTContext for RustTypeDecl {
    type Context<'a> = &'a SourceContext<'a>;
}

impl CanOptimize for RustTypeDecl {
    fn niches(&self, context: &SourceContext<'_>) -> usize {
        self.def.niches(context)
    }
}

impl MemSize for RustTypeDecl {
    fn size_hint(&self, context: &SourceContext<'_>) -> usize {
        self.def.size_hint(context)
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        self.def.align_hint(context)
    }
}

impl CopyEligible for RustTypeDecl {
    fn copy_hint(&self, context: &SourceContext<'_>) -> bool {
        self.def.copy_hint(context)
    }
}

impl ASTContext for RustTypeDef {
    type Context<'a> = &'a SourceContext<'a>;
}

impl CanOptimize for RustTypeDef {
    fn niches(&self, context: &SourceContext<'_>) -> usize {
        match self {
            // Note - this can be circular if we are not careful, but we don't expect circularity in practice
            RustTypeDef::Struct(def) => def.niches(context),
            RustTypeDef::Enum(vars) => niche_sum(vars.iter().map(|v| v.niches(context))),
        }
    }
}

impl MemSize for RustTypeDef {
    fn size_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustTypeDef::Struct(def) => def.size_hint(context),
            RustTypeDef::Enum(vars) => {
                // TODO: detect and report if there is a huge disparity between variant sizes (as clippy warns us about)
                // NOTE: there will be discrepancies in cases where niche-optimization can fit a tag into a niche of one variant to eliminate an external tag-field
                let max_size = vars.iter().map(|v| v.size_hint(context)).max().unwrap_or(0);
                let max_align = vars
                    .iter()
                    .map(|v| v.align_hint(context))
                    .max()
                    .unwrap_or(1);
                // We can't simply add as the largest variant may not be the largest-alignment variant (e.g. [u8; 71] vs Vec<u8>)
                aligned_size(max_size + 1, max_align)
            }
        }
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustTypeDef::Struct(def) => def.align_hint(context),
            RustTypeDef::Enum(vars) => vars
                .iter()
                .map(|v| v.align_hint(context))
                .max()
                .unwrap_or(1),
        }
    }
}

impl CopyEligible for RustTypeDef {
    fn copy_hint(&self, context: &SourceContext<'_>) -> bool {
        match self {
            RustTypeDef::Struct(def) => def.copy_hint(context),
            RustTypeDef::Enum(vars) => vars.iter().all(|v| v.copy_hint(context)),
        }
    }
}

impl ASTContext for RustVariant {
    type Context<'a> = &'a SourceContext<'a>;
}

impl CanOptimize for RustVariant {
    fn niches(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustVariant::Unit(..) => 0,
            RustVariant::Tuple(.., elts) => niche_product(elts.iter().map(|e| e.niches(context))),
        }
    }
}

impl MemSize for RustVariant {
    fn size_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustVariant::Unit(..) => 0,
            RustVariant::Tuple(.., elts) => {
                let mut raw_size = 0;
                let mut max_align = 1;
                for elt in elts.iter() {
                    raw_size += elt.size_hint(context);
                    max_align = max_align.max(elt.align_hint(context));
                }
                aligned_size(raw_size, max_align)
            }
        }
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustVariant::Unit(..) => 1,
            RustVariant::Tuple(.., elts) => elts
                .iter()
                .map(|e| e.align_hint(context))
                .max()
                .unwrap_or(1),
        }
    }
}

impl CopyEligible for RustVariant {
    fn copy_hint(&self, context: &SourceContext<'_>) -> bool {
        match self {
            RustVariant::Unit(..) => true,
            RustVariant::Tuple(.., elts) => elts.iter().all(|e| e.copy_hint(context)),
        }
    }
}

impl ASTContext for RustStruct {
    type Context<'a> = &'a SourceContext<'a>;
}

impl CanOptimize for RustStruct {
    fn niches(&self, context: &SourceContext<'_>) -> usize {
        match self {
            // Because `Vec` in particular has `usize::MAX + 1` niches, we need to be careful to avoid overflow
            RustStruct::Record(fields) => niche_product(fields.iter().map(|f| f.1.niches(context))),
        }
    }
}

impl MemSize for RustStruct {
    fn size_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustStruct::Record(items) => {
                let mut raw_size = 0;
                let mut max_align = 1;
                for item in items.iter() {
                    raw_size += item.1.size_hint(context);
                    max_align = max_align.max(item.1.align_hint(context));
                }
                aligned_size(raw_size, max_align)
            }
        }
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustStruct::Record(items) => items
                .iter()
                .map(|f| f.1.align_hint(context))
                .max()
                .unwrap_or(1),
        }
    }
}

impl CopyEligible for RustStruct {
    fn copy_hint(&self, context: &SourceContext<'_>) -> bool {
        match self {
            RustStruct::Record(fields) => fields.iter().all(|f| f.1.copy_hint(context)),
        }
    }
}

/// Optimized function that computes the smallest multiple of `align` greater than or equal to `size`.
pub(crate) fn aligned_size(size: usize, align: usize) -> usize {
    (size + align - 1) & !(align - 1)
}

#[cfg(test)]
fn roundup(size: usize, align: usize) -> usize {
    if size % align == 0 {
        size
    } else {
        (size / align + 1) * align
    }
}

#[cfg(test)]
mod algo_test {
    use super::*;
    use proptest::prelude::*;

    fn small_two_powers() -> impl Strategy<Value = usize> {
        (0usize..=3).prop_map(|x| 1usize << x)
    }

    proptest! {
        #[test]
        fn test_roundup_equality(x: usize, y in small_two_powers()) {
            prop_assert_eq!(roundup(x, y), aligned_size(x, y));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eq_size_hint<T, Rep>(ast_t: Rep, context: Rep::Context<'_>)
    where
        Rep: MemSize + std::fmt::Debug,
        for<'a> Rep::Context<'a>: Copy,
    {
        let expected = size_of::<T>();
        let actual = ast_t.size_hint(context);
        assert_eq!(actual, expected);
    }

    fn eq_size_hint_option<T, Rep>(ast_t: Rep, context: Rep::Context<'_>)
    where
        Rep: MemSize + std::fmt::Debug,
        for<'a> Rep::Context<'a>: Copy,
    {
        let expected = size_of::<Option<T>>();
        let actual = (CompType::Option(Box::new(ast_t))).size_hint(context);
        assert_eq!(actual, expected);
    }

    #[test]
    fn size_hint_prim() {
        eq_size_hint::<(), _>(PrimType::Unit, ());
        eq_size_hint::<bool, _>(PrimType::Bool, ());
        eq_size_hint::<u8, _>(PrimType::U8, ());
        eq_size_hint::<u16, _>(PrimType::U16, ());
        eq_size_hint::<u32, _>(PrimType::U32, ());
        eq_size_hint::<u64, _>(PrimType::U64, ());
        eq_size_hint::<char, _>(PrimType::Char, ());
    }

    #[test]
    fn size_hint_option_prim() {
        eq_size_hint_option::<(), _>(PrimType::Unit, ());
        eq_size_hint_option::<bool, _>(PrimType::Bool, ());
        eq_size_hint_option::<u8, _>(PrimType::U8, ());
        eq_size_hint_option::<u16, _>(PrimType::U16, ());
        eq_size_hint_option::<u32, _>(PrimType::U32, ());
        eq_size_hint_option::<u64, _>(PrimType::U64, ());
        eq_size_hint_option::<char, _>(PrimType::Char, ());
    }
}
