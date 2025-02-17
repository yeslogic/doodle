use std::{cell::RefCell, collections::HashMap};

use super::*;

/// Helper trait for any AST type-rep whose memory footprint and other metrics may require
/// non-empty context to observe (such as a dictionary of type-definitions for LocalType).
pub trait ASTContext {
    type Context<'a>: Sized + 'a;
}

fn niche_product(iter: impl Iterator<Item = usize>) -> usize {
    iter.map(|x| x.saturating_add(1)).fold(1usize, usize::saturating_mul) - 1
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

#[derive(Default)]
struct CacheEntry {
    niches: Option<usize>,
    size: Option<usize>,
    align: Option<usize>,
}

pub(crate) struct SourceContext<'a> {
    def_map: &'a [RustTypeDef],
    cache: Rc<RefCell<HashMap<usize, CacheEntry>>>,
}

macro_rules! cache_get {
    ( $this:expr, $field:ident, $ix:ident, $method:ident ) => {
        {
            let cache = $this.cache.borrow();
            if cache.contains_key(&$ix) {
                match cache[&$ix].$field {
                    Some(ret) => ret,
                    None => {
                        drop(cache);
                        let ret = {
                            let def = &$this.def_map[$ix];
                            def.$method($this)
                        };
                        $this.cache.borrow_mut().get_mut(&$ix).unwrap().$field = Some(ret);
                        ret
                    }
                }
            } else {
                drop(cache);
                let ret = {
                    let def = &$this.def_map[$ix];
                    def.$method($this)
                };
                $this.cache.borrow_mut().entry($ix).or_default().$field = Some(ret);
                ret
            }
        }
    };
}

impl SourceContext<'_> {
    fn get_niches(&self, ix: usize) -> usize {
        cache_get!(self, niches, ix, niches)
    }

    fn get_size(&self, ix: usize) -> usize {
        cache_get!(self, size, ix, size_hint)
    }

    fn get_align(&self, ix: usize) -> usize {
        cache_get!(self, align, ix, align_hint)
    }
}

impl<'a> From<&'a [RustTypeDef]> for SourceContext<'a> {
    fn from(def_map: &'a [RustTypeDef]) -> Self {
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
            RustType::AnonTuple(ts) => {
                // REVIEW - confirm this logic works properly
                niche_product(ts.iter().map(|t| t.niches(context)))
            }
            // conservative estimate based on our assumption we won't see any Verbatim types in gencode structs
            RustType::Verbatim(..) => 0,
        }
    }
}

impl MemSize for RustType {
    fn size_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustType::Atom(at) => at.size_hint(context),
            RustType::AnonTuple(ts) => {
                // FIXME - doesn't respect padding requirements based on alignment
                let mut ret = 0;
                for t in ts.iter() {
                    ret += t.size_hint(context);
                }
                ret
            }
            RustType::Verbatim(..) => unreachable!("unexpected RustType::Verbatim in structural type"),
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
            RustType::Verbatim(..) => unreachable!("unexpected RustType::Verbatim in structural type"),
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
            PrimType::Char => {
                match char::UNICODE_VERSION {
                    (16, 0, 0) => {
                        const { u32::MAX as usize - UTF16_SCALAR_MAX }
                    }
                    _ => unimplemented!("unsupported Unicode version"),
                }
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

/// Local choice of what type to embed as the parameter to make `Vec` a concrete type-instance we can pass into
/// `size_of` and `align_of` methods.
type VecFiller = u8;

impl<T> ASTContext for CompType<Box<T>>
where
    T: ASTContext
{
    type Context<'a> = T::Context<'a>;
}

impl<T> MemSize for CompType<Box<T>>
where
    T: MemSize + CanOptimize + std::fmt::Debug,
    for<'a> T::Context<'a>: Copy
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
            },
            CompType::Result(..) => unimplemented!("unexpected result in structural type"),
            CompType::Borrow(..) => unimplemented!("unexpected borrow in structural type"),
        }
    }

    fn align_hint(&self, context: Self::Context<'_>) -> usize {
        match self {
            CompType::Vec(..) => align_of::<Vec<VecFiller>>(),
            CompType::Option(inner) => inner.align_hint(context),
            CompType::Result(..) => unimplemented!("unexpected result in structural type"),
            CompType::Borrow(..) => unimplemented!("unexpected borrow in structural type"),
        }
    }
}

impl<T> CanOptimize for CompType<Box<T>>
where
    T: CanOptimize + MemSize + std::fmt::Debug,
    for<'a> T::Context<'a>: Copy
{
    fn niches(&self, context: Self::Context<'_>) -> usize {
        match self {
            // Vec<T> has enough niches that all values of `n: u8` are optimizable
            CompType::Vec(..) => usize::MAX,
            CompType::Option(inner) => {
                match inner.niches(context) {
                    0 => match inner.align_hint(context) {
                        n @ 1..=7 => (1 << (8 * n)) - 1,
                        8 => usize::MAX,
                        n => unreachable!("align of {n} is not an expected case: {inner:?}"),
                    }
                    n => n - 1,
                }
            }
            // Option<&T> cannot be optimized, but &T itself and Option<Option<&T>> (and above) can be
            CompType::Borrow(..) => unreachable!("unexpected borrow in structural type"),
            CompType::Result(..) => unreachable!("unexpected result in structural type")
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
            LocalType::LocalDef(ix, _) => context.get_niches(*ix),
            LocalType::External(_) => unreachable!("unexpected external type-reference in structural type"),
        }
    }
}

impl MemSize for LocalType {
    fn size_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            LocalType::LocalDef(ix, _) => context.get_size(*ix),
            LocalType::External(_) => unreachable!("unexpected external type-reference in structural type"),
        }
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            LocalType::LocalDef(ix, _) => context.get_align(*ix),
            LocalType::External(_) => unreachable!("unexpected external type-reference in structural type"),
        }
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
                vars.iter().map(|v| v.size_hint(context)).max().unwrap_or(0) +
                vars.iter().map(|v| v.align_hint(context)).max().unwrap_or(1)
            }
        }
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustTypeDef::Struct(def) => def.align_hint(context),
            RustTypeDef::Enum(vars) => vars.iter().map(|v| v.align_hint(context)).max().unwrap_or(1),
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
            RustVariant::Tuple(.., elts) => elts.iter().map(|e| e.size_hint(context)).sum::<usize>(),
        }
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustVariant::Unit(..) => 1,
            RustVariant::Tuple(.., elts) => elts.iter().map(|e| e.align_hint(context)).max().unwrap_or(1),
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
            RustStruct::Record(fields) => niche_product(fields.iter().map(|f| f.1.niches(context)))
        }
    }
}

impl MemSize for RustStruct {
    fn size_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            // FIXME - doesn't respect padding requirements based on alignment
            RustStruct::Record(items) => items.iter().map(|f| f.1.size_hint(context)).sum::<usize>(),
        }
    }

    fn align_hint(&self, context: &SourceContext<'_>) -> usize {
        match self {
            RustStruct::Record(items) => items.iter().map(|f| f.1.align_hint(context)).max().unwrap_or(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eq_size_hint<T>(ast_t: RustType, context: &[RustTypeDef]) {
        let expected = size_of::<T>();
        let actual = ast_t.size_hint(context);
        assert_eq!(actual, expected);
    }

    fn eq_size_hint_option<T>(ast_t: RustType, context: &[RustTypeDef]) {
        let expected = size_of::<Option<T>>();
        let actual = (RustType::from(CompType::Option(Box::new(ast_t)))).size_hint(context);
        assert_eq!(actual, expected);
    }

    #[test]
    fn size_hint_prim() {
        eq_size_hint::<()>(RustType::from(PrimType::Unit), &[]);
        eq_size_hint::<bool>(RustType::from(PrimType::Bool), &[]);
        eq_size_hint::<u8>(RustType::from(PrimType::U8), &[]);
        eq_size_hint::<u16>(RustType::from(PrimType::U16), &[]);
        eq_size_hint::<u32>(RustType::from(PrimType::U32), &[]);
        eq_size_hint::<u64>(RustType::from(PrimType::U64), &[]);
        eq_size_hint::<char>(RustType::from(PrimType::Char), &[]);
    }

    #[test]
    fn size_hint_option_prim() {
        eq_size_hint_option::<()>(RustType::from(PrimType::Unit), &[]);
        eq_size_hint_option::<bool>(RustType::from(PrimType::Bool), &[]);
        eq_size_hint_option::<u8>(RustType::from(PrimType::U8), &[]);
        eq_size_hint_option::<u16>(RustType::from(PrimType::U16), &[]);
        eq_size_hint_option::<u32>(RustType::from(PrimType::U32), &[]);
        eq_size_hint_option::<u64>(RustType::from(PrimType::U64), &[]);
        eq_size_hint_option::<char>(RustType::from(PrimType::Char), &[]);
    }
}
