use super::{
    size::{aligned_size, MemSize},
    AtomType, CompType, LocalType, PrimType, RustStruct, RustType, RustTypeDef, RustVariant,
};
use core::alloc::Layout;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum HeapStrategy {
    /// Default strategy: never box anything
    #[default]
    Never,
    /// Simple strategy: heap-indirect types in all struct contexts when larger than `N` bytes
    ///
    /// When applied to a direct positional argument of a tuple that will still be heap-allocated,
    /// the inner heap-allocation is lifted to avoid excessive indirection
    ContextFreeLifted(usize),
    /// Clippy strategy: heap-indirect all variants more than `N` bytes larger than the smallest variant
    EnumDiscrepancy(usize),
}

impl HeapStrategy {
    pub const fn is_never(&self) -> bool {
        matches!(self, Self::Never)
    }

    pub const fn min_heap_size(&self) -> Option<usize> {
        match *self {
            HeapStrategy::Never => None,
            HeapStrategy::EnumDiscrepancy(..) => None,
            HeapStrategy::ContextFreeLifted(min_size) => Some(min_size),
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
    fn dry_run(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome;
}

pub fn mk_layout<T: MemSize>(t: &T, context: T::Context<'_>) -> Layout
where
    for<'a> T::Context<'a>: Copy,
{
    Layout::from_size_align(t.size_hint(context), t.align_hint(context)).unwrap()
}

impl HeapOptimize for RustStruct {
    fn dry_run(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match strategy {
            HeapStrategy::Never => (HeapAction::Noop, mk_layout(self, context)),
            _ => {
                // NOTE - a rust struct, as a standalone object, cannot be wrapped in a heap allocation; only its fields can be individually heap-allocated
                let fields = match &self {
                    RustStruct::Record(fields) => fields,
                };
                let mut raw_size = 0;
                let mut max_align = 1;
                let mut field_actions = Vec::with_capacity(fields.len());
                let mut is_productive = false;
                for (_, f_type) in fields.iter() {
                    let (action, layout) = f_type.dry_run(strategy, context);
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
}

impl HeapOptimize for RustType {
    fn dry_run(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            RustType::Atom(at) => at.dry_run(strategy, context),
            RustType::AnonTuple(ts) => {
                if strategy.is_never() {
                    return (HeapAction::Noop, mk_layout(self, context));
                }
                let mut raw_size = 0;
                let mut max_align = 1;
                let mut pos_actions = Vec::with_capacity(ts.len());
                let mut is_productive = false;
                for t in ts.iter() {
                    let (action, layout) = t.dry_run(strategy, context);
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
    fn dry_run(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            AtomType::Prim(pt) => pt.dry_run(strategy, ()),
            AtomType::Comp(ct) => ct.dry_run(strategy, context),
            AtomType::TypeRef(lt) => lt.dry_run(strategy, context),
        }
    }
}

impl HeapOptimize for PrimType {
    fn dry_run(&self, _: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        (HeapAction::Noop, mk_layout(self, context))
    }
}

impl HeapOptimize for CompType<Box<RustType>> {
    fn dry_run(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            CompType::Vec(..) => (HeapAction::Noop, mk_layout(self, context)),
            CompType::Option(inner) => {
                let (outcome, _) = inner.dry_run(strategy, context);
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
            CompType::Borrow(..) => unreachable!("unexpected borrow in structural type"),
        }
    }
}

impl HeapOptimize for LocalType {
    fn dry_run(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            LocalType::LocalDef(ix, _) => {
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

impl HeapOptimize for RustTypeDef {
    fn dry_run(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            RustTypeDef::Enum(vars) => match strategy {
                HeapStrategy::Never => (HeapAction::Noop, mk_layout(self, context)),
                HeapStrategy::EnumDiscrepancy(threshold_delta) => {
                    let mut var_sizes = Vec::with_capacity(vars.len());
                    let mut var_actions = Vec::with_capacity(vars.len());
                    let mut is_productive = false;
                    let mut max_var_size = 0;
                    let mut max_align = 1;
                    for var in vars.iter() {
                        let (action, layout) = var.dry_run(strategy, context);
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
                }
                HeapStrategy::ContextFreeLifted(..) => {
                    let mut var_actions = Vec::with_capacity(vars.len());
                    let mut max_var_size = 0;
                    let mut max_align = 1;
                    let mut is_productive = false;
                    for var in vars.iter() {
                        let (action, layout) = var.dry_run(strategy, context);
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
                }
            },
            RustTypeDef::Struct(str) => str.dry_run(strategy, context),
        }
    }
}

impl HeapOptimize for RustVariant {
    fn dry_run(&self, strategy: HeapStrategy, context: Self::Context<'_>) -> HeapOutcome {
        match self {
            RustVariant::Unit(..) => (HeapAction::Noop, Layout::from_size_align(0, 1).unwrap()),
            RustVariant::Tuple(_, args) => {
                let &[ref typ] = args.as_slice() else {
                    panic!("expected mono-variant, found {self:?}")
                };
                let (action, layout) = typ.dry_run(strategy, context);
                let sz = layout.size();
                if strategy
                    .min_heap_size()
                    .is_some_and(|min_size| sz > min_size)
                {
                    (HeapAction::DirectHeap, HEAP_LAYOUT)
                } else if action.is_noop() {
                    (HeapAction::Noop, layout)
                } else {
                    (HeapAction::NonLocal, layout)
                }
            }
        }
    }
}
