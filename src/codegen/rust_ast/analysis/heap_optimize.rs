use super::{aligned_size, MemSize};
use crate::codegen::rust_ast::{
    AtomType, CompType, LocalType, PrimType, RustStruct, RustType, RustTypeDecl, RustTypeDef,
    RustVariant,
};
use core::alloc::Layout;
use std::num::NonZeroUsize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HeapStrategy {
    absolute_cutoff: Option<NonZeroUsize>,
    variant_cutoff: Option<NonZeroUsize>,
}

impl HeapStrategy {
    pub const fn new() -> Self {
        Self {
            absolute_cutoff: None,
            variant_cutoff: None,
        }
    }

    /// Chainable method for mutating the absolute cutoff (i.e. minimum type-size that will be boxed rather than directly embedded).
    ///
    /// A value of `0` is legal, but has the semantics of 'unsetting' the absolute cutoff, meaning that no absolute-cutoff will be enforced.
    #[expect(unused)]
    pub const fn absolute_cutoff(self, cutoff: usize) -> Self {
        Self {
            absolute_cutoff: NonZeroUsize::new(cutoff),
            ..self
        }
    }

    /// Chainable method for mutating the variant cutoff (i.e. minimum difference between variant sizes that causes large variants to be boxed).
    ///
    /// A value of `0` is legal, but has the semantics of 'unsetting' the variant cutoff, meaning that no variant-cutoff will be enforced.
    pub const fn variant_cutoff(self, cutoff: usize) -> Self {
        Self {
            variant_cutoff: NonZeroUsize::new(cutoff),
            ..self
        }
    }

    pub const fn is_never(&self) -> bool {
        matches!(
            self,
            Self {
                absolute_cutoff: None,
                variant_cutoff: None
            }
        )
    }

    pub const fn min_heap_size(&self) -> Option<usize> {
        match self.absolute_cutoff {
            Some(it) => Some(it.get()),
            None => None,
        }
    }

    pub const fn min_enum_delta(&self) -> Option<usize> {
        match self.variant_cutoff {
            Some(it) => Some(it.get()),
            None => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) enum HeapAction {
    #[default]
    /// Nothing changes
    Noop,
    /// Changes occur, but too far removed to track explicitly
    NonLocal,
    /// For simple type-embeds (mono-enums and new-types), returns the size of the allocation made
    DirectHeap,
    /// For indicating the outcome for the inner type of an Option-layer
    InOption(Box<HeapAction>),
    /// For indicating that the local reference to a type does not change, but the type's definition itself does
    InDef(Box<HeapAction>),
    InRecord {
        /// Indicates which fields, in order of definition, would be re-typed from `U` to `Box<U>`
        fields: Vec<HeapAction>,
    },
    InTuple {
        /// Indicates which positional arguments, in order of definition, would be re-typed from `U` to `Box<U>`
        pos: Vec<HeapAction>,
    },
    InEnum {
        /// Indicates which variants, in order of definition, would have their contents heap-allocated (and how)
        variants: Vec<HeapAction>,
    },
}

impl HeapAction {
    pub const fn is_noop(&self) -> bool {
        matches!(self, Self::Noop)
    }
}

pub type HeapOutcome = (HeapAction, Layout);

/// Internal type used to encode our decision for what smart-pointer type to use for heap-allocation
type HeapContainer<T> = Box<T>;

/// Constant used for representing the layout of our choice of heap-allocation
pub const HEAP_LAYOUT: Layout = Layout::new::<HeapContainer<str>>();

pub const HEAP_SIZE: usize = HEAP_LAYOUT.size();
pub const HEAP_ALIGN: usize = HEAP_LAYOUT.align();

pub trait HeapOptimize: MemSize {
    /// Speculatively executes the given `strategy` over `self`, returning both the outcome of the
    /// strategy's application, and the new size of the type if the strategy were employed.
    fn heap_hint(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome;
}

pub fn mk_layout<T: MemSize>(t: &T, context: T::Context<'_>) -> Layout
where
    for<'a> T::Context<'a>: Copy,
{
    Layout::from_size_align(t.size_hint(context), t.align_hint(context)).unwrap()
}

impl HeapOptimize for RustStruct {
    fn heap_hint(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        if strategy.is_never() {
            (HeapAction::Noop, mk_layout(self, context))
        } else {
            // NOTE - a rust struct, as a standalone object, cannot be wrapped in a heap allocation; only its fields can be individually heap-allocated
            let fields = match &self {
                RustStruct::Record(fields) => fields,
            };
            let mut raw_size = 0;
            let mut max_align = 1;
            let mut field_actions = Vec::with_capacity(fields.len());
            let mut is_productive = false;
            for (_, f_type) in fields.iter() {
                let (action, layout) = f_type.heap_hint(strategy, context);
                is_productive |= !action.is_noop();
                field_actions.push(action);
                raw_size += layout.size();
                max_align = max_align.max(layout.align());
            }
            let size = aligned_size(raw_size, max_align);
            let ret = if is_productive {
                HeapAction::InRecord {
                    fields: field_actions,
                }
            } else {
                HeapAction::Noop
            };
            (ret, Layout::from_size_align(size, max_align).unwrap())
        }
    }
}

impl HeapOptimize for RustType {
    fn heap_hint(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            RustType::ReadArray(..) => (HeapAction::Noop, mk_layout(self, context)),
            RustType::ViewObject(..) => (HeapAction::Noop, mk_layout(self, context)),
            RustType::Atom(at) => at.heap_hint(strategy, context),
            RustType::AnonTuple(ts) => {
                if strategy.is_never() {
                    return (HeapAction::Noop, mk_layout(self, context));
                }
                let mut raw_size = 0;
                let mut max_align = 1;
                let mut pos_actions = Vec::with_capacity(ts.len());
                let mut is_productive = false;
                for t in ts.iter() {
                    let (action, layout) = t.heap_hint(strategy, context);
                    is_productive |= !action.is_noop();
                    pos_actions.push(action);
                    raw_size += layout.size();
                    max_align = max_align.max(layout.align());
                }
                let size = aligned_size(raw_size, max_align);
                if strategy
                    .min_heap_size()
                    .is_some_and(|min_size| size > min_size)
                {
                    (HeapAction::DirectHeap, HEAP_LAYOUT)
                } else if is_productive {
                    (
                        HeapAction::InTuple { pos: pos_actions },
                        Layout::from_size_align(size, max_align).unwrap(),
                    )
                } else {
                    (
                        HeapAction::Noop,
                        Layout::from_size_align(size, max_align).unwrap(),
                    )
                }
            }
            RustType::Verbatim(..) => {
                unreachable!("unexpected RustType::Verbatim in structural type")
            }
        }
    }
}

impl HeapOptimize for AtomType {
    fn heap_hint(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            AtomType::Prim(pt) => pt.heap_hint(strategy, ()),
            AtomType::Comp(ct) => ct.heap_hint(strategy, context),
            AtomType::TypeRef(lt) => lt.heap_hint(strategy, context),
        }
    }
}

impl HeapOptimize for PrimType {
    fn heap_hint(&self, _: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        (HeapAction::Noop, mk_layout(self, context))
    }
}

impl HeapOptimize for CompType<Box<RustType>> {
    fn heap_hint(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            CompType::Vec(..) => (HeapAction::Noop, mk_layout(self, context)),
            CompType::Option(inner) => {
                let (outcome, _) = inner.heap_hint(strategy, context);
                match &outcome {
                    HeapAction::Noop => (HeapAction::Noop, mk_layout(self, context)),
                    HeapAction::NonLocal
                    | HeapAction::InDef(..)
                    | HeapAction::DirectHeap
                    | HeapAction::InTuple { .. } => {
                        (HeapAction::InOption(Box::new(outcome)), HEAP_LAYOUT)
                    }
                    HeapAction::InEnum { .. } | HeapAction::InRecord { .. } => {
                        unreachable!("unexpected heap outcome in Option inner-type")
                    }
                    HeapAction::InOption(..) => {
                        unreachable!("unexpected double-nested option in structural type")
                    }
                }
            }
            CompType::Result(..) => unreachable!("unexpected result in structural type"),
            // REVIEW - is this an accurate claim, or do we need a bespoke variant for this case?
            CompType::Borrow(..) => (HeapAction::Noop, mk_layout(self, context)),
            CompType::RawSlice(..) => unreachable!("unexpected raw slice in structural type"),
        }
    }
}

impl HeapOptimize for LocalType {
    fn heap_hint(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            LocalType::LocalDef(ix, ..) => {
                let (action, layout) = context.get_heap(strategy, *ix);
                let size = layout.size();
                if strategy.min_heap_size().is_some_and(|sz| size >= sz) {
                    (HeapAction::DirectHeap, HEAP_LAYOUT)
                } else {
                    match action {
                        HeapAction::Noop => (HeapAction::Noop, layout),
                        HeapAction::NonLocal => (HeapAction::NonLocal, layout),
                        HeapAction::InRecord { .. } => {
                            (HeapAction::InDef(Box::new(action)), layout)
                        }
                        HeapAction::InEnum { .. } => (HeapAction::InDef(Box::new(action)), layout),
                        HeapAction::InTuple { .. } => unreachable!(
                            "unexpected in-tuple heap-alloc in local-def type expansion"
                        ),
                        HeapAction::InDef(..) => {
                            unreachable!("unexpected in-def heap-alloc in local-type expansion")
                        }
                        HeapAction::InOption(..) => unreachable!(
                            "unexpected in-option heap-alloc in local-def type expansion"
                        ),
                        HeapAction::DirectHeap => unreachable!(
                            "unexpected single-heap allocation in local-def type expansion"
                        ),
                    }
                }
            }
            LocalType::External(..) => {
                unreachable!("unexpected external type-reference in structural type")
            }
        }
    }
}

impl HeapOptimize for RustTypeDecl {
    fn heap_hint(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        self.def.heap_hint(strategy, context)
    }
}

impl HeapOptimize for RustTypeDef {
    fn heap_hint(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            RustTypeDef::Struct(str) => str.heap_hint(strategy, context),
            RustTypeDef::Enum(vars) => {
                if let Some(threshold_delta) = strategy.min_enum_delta() {
                    let mut var_sizes = Vec::with_capacity(vars.len());
                    let mut var_actions = Vec::with_capacity(vars.len());
                    let mut is_productive = false;
                    let mut max_var_size = 0;
                    let mut max_align = 1;
                    for var in vars.iter() {
                        let (action, layout) = var.heap_hint(strategy, context);
                        var_sizes.push(layout.size());
                        is_productive |= !action.is_noop();
                        var_actions.push(action);
                        max_var_size = max_var_size.max(layout.size());
                        max_align = max_align.max(layout.align());
                    }
                    let min_var_size = var_sizes.iter().min().expect("unexpected empty enum");
                    if var_sizes
                        .iter()
                        .any(|sz| *sz >= min_var_size + threshold_delta)
                    {
                        let mut max_var_size = *min_var_size;
                        let mut var_alloc = Vec::with_capacity(vars.len());
                        for (sz, act) in Iterator::zip(var_sizes.iter(), var_actions.into_iter()) {
                            let new_size = if *sz >= min_var_size + threshold_delta {
                                var_alloc.push(HeapAction::DirectHeap);
                                HEAP_SIZE
                            } else {
                                var_alloc.push(act);
                                *sz
                            };
                            max_var_size = max_var_size.max(new_size);
                        }
                        (
                            HeapAction::InEnum {
                                variants: var_alloc,
                            },
                            Layout::from_size_align(max_var_size, HEAP_ALIGN).unwrap(),
                        )
                    } else {
                        if is_productive {
                            (
                                HeapAction::NonLocal,
                                Layout::from_size_align(
                                    aligned_size(max_var_size + 1, max_align),
                                    max_align,
                                )
                                .unwrap(),
                            )
                        } else {
                            (HeapAction::Noop, mk_layout(self, context))
                        }
                    }
                } else if let Some(_abs_cutoff) = strategy.min_heap_size() {
                    let mut var_actions = Vec::with_capacity(vars.len());
                    let mut max_var_size = 0;
                    let mut max_align = 1;
                    let mut is_productive = false;
                    for var in vars.iter() {
                        let (action, layout) = var.heap_hint(strategy, context);
                        is_productive |= !action.is_noop();
                        var_actions.push(action);
                        max_var_size = max_var_size.max(layout.size());
                        max_align = max_align.max(layout.align());
                    }
                    if is_productive {
                        (
                            HeapAction::InEnum {
                                variants: var_actions,
                            },
                            Layout::from_size_align(
                                aligned_size(max_var_size + 1, max_align),
                                max_align,
                            )
                            .unwrap(),
                        )
                    } else {
                        (HeapAction::Noop, mk_layout(self, context))
                    }
                } else {
                    (HeapAction::Noop, mk_layout(self, context))
                }
            }
        }
    }
}

impl HeapOptimize for RustVariant {
    fn heap_hint(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            RustVariant::Unit(..) => (HeapAction::Noop, Layout::from_size_align(0, 1).unwrap()),
            RustVariant::Tuple(_, ts) => {
                if strategy.is_never() {
                    return (HeapAction::Noop, mk_layout(self, context));
                }
                let mut raw_size = 0;
                let mut max_align = 1;
                let mut pos_actions = Vec::with_capacity(ts.len());
                let mut is_productive = false;
                for t in ts.iter() {
                    let (action, layout) = t.heap_hint(strategy, context);
                    is_productive |= !action.is_noop();
                    pos_actions.push(action);
                    raw_size += layout.size();
                    max_align = max_align.max(layout.align());
                }
                let size = aligned_size(raw_size, max_align);
                if strategy
                    .min_heap_size()
                    .is_some_and(|min_size| size > min_size)
                {
                    (HeapAction::DirectHeap, HEAP_LAYOUT)
                } else if is_productive {
                    (
                        HeapAction::InTuple { pos: pos_actions },
                        Layout::from_size_align(size, max_align).unwrap(),
                    )
                } else {
                    (
                        HeapAction::Noop,
                        Layout::from_size_align(size, max_align).unwrap(),
                    )
                }
            }
        }
    }
}
