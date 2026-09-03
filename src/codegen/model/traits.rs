use super::*;

pub(crate) trait TraitObject {
    /// Encapsulation of the contextual information needed to implement a trait on an arbitrary RustType
    type TypeInfo<'a>;

    /// Returns the raw name of the trait
    fn get_name() -> &'static str;

    // /// Returns how many lifetime parameters the trait takes
    // fn lt_params() -> usize;

    // /// Returns how many type parameters the trait takes
    // fn ty_params() -> usize;

    // fn satisfies_requirements(
    //     on_type: &RustType,
    //     type_info: Self::TypeInfo<'_>,
    // ) -> bool;

    /// Generates as many trait-impl blocks (possibly for other traits as well) as necessary to implement this trait on the given type
    fn generate_impls(on_type: Box<RustType>, type_info: Self::TypeInfo<'_>) -> Vec<RustTraitImpl>;
}

pub mod object_api {
    use super::*;
    use crate::codegen::{
        DecoderFn,
        catalog::CrossIndex,
        decoder_fname,
        typed_format::{GenType, TypedExpr},
    };

    #[derive(Clone, Copy)]
    pub(crate) struct TypeParseInfo<'a> {
        pub catalog: &'a CrossIndex,
        pub decoders: &'a [DecoderFn<TypedExpr<GenType>>],
    }

    pub struct CommonObject;

    const IMPL_LIFETIME: &str = "'a";
    const GAT_LIFETIME: &str = "'x";
    const METHOD_LIFETIME: &str = "'input";

    fn convert_extra_args(extra_args: &[(Label, GenType)]) -> RustType {
        fn convert_arg_tuple((_, ty): &(Label, GenType)) -> RustType {
            let mut ty0 = ty.to_rust_type();
            ty0.alpha_convert_lifetime(lt(GAT_LIFETIME));
            RustType::selective_borrow(Some(lt(GAT_LIFETIME)), Mut::Immutable, ty0)
        }
        match extra_args {
            [] => RustType::from(PrimType::Unit),
            [arg] => convert_arg_tuple(arg),
            args => {
                let args = args.iter().map(convert_arg_tuple).collect();
                RustType::AnonTuple(args)
            }
        }
    }

    impl TraitObject for CommonObject {
        type TypeInfo<'a> = TypeParseInfo<'a>;

        fn get_name() -> &'static str {
            "CommonObject"
        }

        fn generate_impls(
            on_type: Box<RustType>,
            type_info: Self::TypeInfo<'_>,
        ) -> Vec<RustTraitImpl> {
            let trait_name = lbl(Self::get_name());
            let RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(ix, name, params))) =
                on_type.as_ref()
            else {
                unreachable!(
                    "unexpected non-local type-reference encountered during {trait_name} trait-impl generation: {on_type:?}"
                )
            };

            let selftype_with_lt = |lt: RustLt| {
                RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(
                    *ix,
                    name.clone(),
                    params.as_ref().map(|_| Box::new(UseParams::from_lt(lt))),
                )))
            };

            let param_bindings = match params {
                None => None,
                Some(params) => match &params.lt_params[..] {
                    [] => None,
                    [_lt] => Some(Box::new(DefParams::from_lt(lbl(IMPL_LIFETIME)))),
                    _ => unreachable!(
                        "unexpected number of lifetime parameters encountered during {trait_name} trait-impl generation: {on_type:?}"
                    ),
                },
            };
            let trait_params = None;
            let body = {
                let canonical_decoder = {
                    let decoder_ix_set = type_info.catalog.get(*ix).unwrap();
                    match decoder_ix_set.len() {
                        0 => unreachable!(
                            "unexpected empty decoder list encountered during {trait_name} trait-impl generation: {on_type:?}"
                        ),
                        1 => {
                            let decoder_ix = decoder_ix_set.as_ref()[0];
                            let decoder_fn = &type_info.decoders[decoder_ix];
                            decoder_fn
                        }
                        2.. => unreachable!(
                            "unexpected ambiguous decoder list ({decoder_ix_set:?}) encountered during {trait_name} trait-impl generation: {on_type:?}"
                        ),
                    }
                };
                let extra_args = match &canonical_decoder.extra_args {
                    Some(extra_args) => extra_args.as_slice(),
                    None => &[],
                };
                // NOTE - the body may change as the trait is redesigned in future iterations
                let def_gat_args = {
                    let params = Some(Box::new(DefParams::from_lt(lbl(GAT_LIFETIME))));
                    let rhs = convert_extra_args(extra_args);
                    let decl = TraitItem::AssocType(lbl("Args"), params, Box::new(rhs));
                    decl
                };
                let def_gat_output = {
                    // REVIEW - this GAT may be subject to redesign, but for now we have `Self::Output = Self`.
                    let rhs = selftype_with_lt(lt(GAT_LIFETIME));
                    TraitItem::AssocType(
                        lbl("Output"),
                        Some(Box::new(DefParams::from_lt(lbl(GAT_LIFETIME)))),
                        Box::new(rhs),
                    )
                };

                let def_method_parse = {
                    let params = Some(DefParams {
                        lt_params: vec![lbl(METHOD_LIFETIME)],
                        ty_params: vec![],
                    });
                    let sig = {
                        let args = {
                            let arg0 = {
                                let name = lbl("p");
                                let ty = {
                                    let params = UseParams::from_lt(lt(METHOD_LIFETIME));
                                    RustType::borrow_of(
                                        None,
                                        Mut::Mutable,
                                        RustType::verbatim("Parser", Some(params)),
                                    )
                                };
                                (name, ty)
                            };
                            let arg1 = {
                                match extra_args {
                                    [] => (lbl("_"), RustType::from(PrimType::Unit)),
                                    [(ident, _)] => {
                                        let ty = RustType::Verbatim(
                                            lbl("Self::Args"),
                                            Some(Box::new(UseParams::from_lt(lt(METHOD_LIFETIME)))),
                                        );
                                        (ident.clone(), ty)
                                    }
                                    _ => {
                                        let name = lbl("args");
                                        let ty = RustType::Verbatim(
                                            lbl("Self::Args"),
                                            Some(Box::new(UseParams::from_lt(lt(METHOD_LIFETIME)))),
                                        );
                                        (name, ty)
                                    }
                                }
                            };
                            vec![arg0, arg1]
                        };
                        FnSig::new(
                            args,
                            Some(Box::new(RustType::result_of(
                                RustType::Verbatim(
                                    lbl("Self::Output"),
                                    Some(Box::new(UseParams::from_lt(lt(METHOD_LIFETIME)))),
                                ),
                                RustType::imported("ParseError"),
                            ))),
                        )
                    };
                    let body = {
                        let fname = decoder_fname(canonical_decoder.ixlabel);
                        let num_extra_args = extra_args.len();
                        let mut stmts = Vec::with_capacity(num_extra_args + 1);
                        let args = {
                            let mut accum = Vec::with_capacity(num_extra_args + 1);

                            let parser_arg = RustExpr::local("p");
                            accum.push(parser_arg);

                            match extra_args {
                                [] => (),
                                [(ident, _)] => {
                                    accum.push(RustExpr::local(ident.clone()));
                                }
                                args => {
                                    let mut bindings = Vec::with_capacity(args.len());
                                    for (ident, _) in args {
                                        bindings.push(RustPattern::CatchAll(Some(ident.clone())));
                                        accum.push(RustExpr::local(ident.clone()))
                                    }
                                    let lhs_pat = RustPattern::TupleLiteral(bindings);
                                    stmts.push(RustStmt::LetPattern(
                                        lhs_pat,
                                        RustExpr::local("args"),
                                    ))
                                }
                            }
                            accum
                        };
                        let call = RustExpr::FunctionCall(Box::new(RustExpr::local(fname)), args);
                        stmts.push(RustStmt::Return(ReturnKind::Implicit, call));
                        stmts
                    };

                    TraitItem::Method(RustFn::new("parse", params, sig, body))
                };

                vec![def_gat_args, def_gat_output, def_method_parse]
            };
            vec![RustTraitImpl {
                param_bindings,
                trait_name,
                trait_params,
                on_type: Box::new(selftype_with_lt(lt(IMPL_LIFETIME))),
                body,
            }]
        }
    }
}

pub mod smallsorts {
    use super::*;
    use crate::codegen::typed_format::{GenType, TypedFormat};
    use crate::codegen::util::{BTree, StableMap};
    use crate::fixed::{self, SpineElem};
    use crate::{FormatModule, FormatRef};
    use intmap::IntMap;
    use std::collections::BTreeSet;
    use std::rc::Rc;

    /// Context needed to generate a `ReadUnchecked` impl for a struct that originated from a
    /// `FixedReadKind::FixedFormat`-kinded `ReadArray` element.
    ///
    /// `RustType`/`SourceContext` cannot recover per-field endianness on their own -- a `u32`
    /// field looks the same in `RustType` whether it was parsed as `u32be` or `u32le` -- so
    /// instead of trying to reconstruct the read sequence from the generated type alone, this
    /// resolves `on_type` back to its originating `Format` (via `targets`, populated by
    /// `Elaborator::elaborate_kind`, and `module`) and recovers the exact field layout from there
    /// via `record_fmt::analyze_fixed_shape`.
    ///
    /// `t_formats` and `defined_types` are the same level-keyed elaborated-format cache and
    /// index-keyed type-decl table `Elaborator` itself uses (`Elaborator::t_formats`,
    /// `CodeGen::defined_types`), threaded through so that a `SpineElem::Indirect(fref, ..)` --
    /// whether the whole `FixedShape::Single` target or a nested `Record` field -- can be
    /// resolved back to the ad-hoc Rust type its own target format was elaborated into, without
    /// requiring that target to have been independently registered in `targets` itself.
    #[derive(Clone, Copy)]
    pub(crate) struct FixedFormatInfo<'a> {
        pub module: &'a FormatModule,
        pub targets: &'a IntMap<usize, FormatRef>,
        pub t_formats: &'a StableMap<usize, Rc<TypedFormat<GenType>>, BTree>,
        pub defined_types: &'a [RustTypeDecl],
    }

    /// Possible types that can occur as a fixed-size spine-element of a `FixedReadKind::FixedFormat`
    ///
    /// Specifically, encapsulates `MarkerType` and `LocalType` (which in turn must be a fixed-size struct).
    #[derive(Debug, Clone)]
    enum SpineType {
        Adhoc((usize, Label, Option<Box<UseParams>>)),
        Marker(MarkerType),
    }

    impl<'a> FixedFormatInfo<'a> {
        /// Resolves the `FormatRef` inside a `SpineElem::Indirect` to the `(defined_types index,
        /// type name, lifetime params)` of the ad-hoc Rust type its target format was elaborated
        /// into. Always re-resolves from `fref`'s own level (rather than assuming it coincides
        /// with some enclosing `on_type`'s own index), so this is correct even for a `fref` that
        /// is reached through a chain of `ItemVar` aliases whose final target differs from
        /// whatever format registered the enclosing type in `targets`.
        fn try_resolve_adhoc(
            &self,
            fref: FormatRef,
        ) -> Option<(usize, Label, Option<Box<UseParams>>)> {
            let t_inner = self.t_formats.get(&fref.get_level())?;
            let gt = t_inner.get_type()?;
            let (ix, name, params) = gt.try_as_adhoc()?;
            Some((ix, name.clone(), params))
        }

        /// Given a `SpineElem`, infallibly induces the corresponding `FixedSizeType`,
        /// panicking if any of the requisite operations fail for any reason.
        fn spine_to_fixed(&self, elem: &SpineElem) -> FixedSizeType {
            match elem {
                SpineElem::Raw(base_kind) => {
                    FixedSizeType::Marker(MarkerType::from_base_kind_endian(*base_kind))
                }
                &SpineElem::Indirect(fref, _kind) => match self.to_marker_or_adhoc(fref).unwrap() {
                    SpineType::Adhoc((ix, name, params)) => {
                        FixedSizeType::Adhoc(LocalType::LocalDef(ix, name, params))
                    }
                    SpineType::Marker(mt) => FixedSizeType::Marker(mt),
                },
            }
        }

        /// Given a `FormatRef` pointing to a proposed fixed-size type, extract the type it associates to,
        /// whether an adhoc LocalType or a primitive MarkerType
        fn to_marker_or_adhoc(&self, fref: FormatRef) -> Option<SpineType> {
            let t_inner = self.t_formats.get(&fref.get_level())?;
            if let Some(hint) = t_inner.get_hint() {
                match hint {
                    &crate::StyleHint::Common(crate::CommonOp::EndianParse(k)) => {
                        return Some(SpineType::Marker(MarkerType::from_base_kind_endian(k)));
                    }
                    _ => (),
                }
            }
            let gt = t_inner.get_type()?;
            let (ix, name, params) = gt.try_as_adhoc()?;
            Some(SpineType::Adhoc((ix, name.clone(), params)))
        }
    }

    pub struct ReadUnchecked;

    impl ReadUnchecked {
        /// Does the actual work of [`TraitObject::generate_impls`], parameterized over an
        /// explicit `format_ref` rather than looking it up via `type_info.targets` -- so that it
        /// can be called recursively for a `SpineElem::Indirect` target that was never itself
        /// registered as its own `ReadArray`'s `FixedFormat` element (e.g. a `Variant`-wrapped
        /// primitive reached only as a nested field of some other `Record`-shaped array element).
        ///
        /// Returns every `RustTraitImpl` needed to make `on_type`'s own impl compile -- its own,
        /// plus (via recursion) one for each distinct `SpineElem::Indirect` target reachable from
        /// it. A `SpineElem::Indirect`'s target is guaranteed (by construction, see
        /// `fixed::SpineElem`) to itself require at most one more `CommonOp` read with no further
        /// indirection, so this recursion is bounded to depth 1 and cannot cycle. Duplicate
        /// impls for the same dependency reached from multiple fields of the same `Record` are
        /// suppressed locally (via `recursed`); duplicates arising from separate top-level calls
        /// (e.g. two different arrays sharing the same indirect element-type) are left for the
        /// caller to dedupe across the whole batch of generated decls.
        fn generate_impls_for(
            on_type: Box<RustType>,
            format_ref: FormatRef,
            type_info: &FixedFormatInfo<'_>,
        ) -> Vec<RustTraitImpl> {
            let Some(LocalType::LocalDef(ix, name, _params)) = on_type.as_local_type() else {
                unreachable!(
                    "ReadUnchecked can only be generated for a locally-defined type: {on_type:?}"
                )
            };
            let shape = fixed::analyze_fixed_shape(type_info.module, format_ref).unwrap_or_else(|e| {
                unreachable!(
                    "type index {ix} was recorded as a FixedFormat target but is no longer a valid fixed-shape record: {e}"
                )
            });

            match shape {
                fixed::FixedShape::Record { fields, stride } => {
                    let host_type = on_type.clone();
                    let size_const = RustExpr::num_lit(stride);
                    let mut dep_impls = Vec::new();
                    let mut recursed = BTreeSet::new();
                    let read_unchecked_method = {
                        let method_lt = Label::Borrowed("'a");
                        let body = {
                            let mut stmts = Vec::with_capacity(fields.len() + 1);
                            let mut field_inits = Vec::with_capacity(fields.len());
                            for (field_name, elem) in &fields {
                                let marker = type_info.spine_to_fixed(elem);
                                if let (
                                    SpineElem::Indirect(fref, _),
                                    FixedSizeType::Adhoc(LocalType::LocalDef(
                                        dep_ix,
                                        dep_name,
                                        dep_params,
                                    )),
                                ) = (elem, &marker)
                                {
                                    if recursed.insert(*dep_ix) {
                                        let dep_on_type = Box::new(RustType::Atom(
                                            AtomType::TypeRef(LocalType::LocalDef(
                                                *dep_ix,
                                                dep_name.clone(),
                                                dep_params.clone(),
                                            )),
                                        ));
                                        dep_impls.extend(Self::generate_impls_for(
                                            dep_on_type,
                                            *fref,
                                            type_info,
                                        ));
                                    }
                                }
                                let read_expr = RustExpr::FunctionCall(
                                    Box::new(RustExpr::Entity(RustEntity::Scoped(
                                        vec![marker.type_name()],
                                        lbl("read_unchecked"),
                                    ))),
                                    vec![RustExpr::local("ctxt")],
                                );
                                match field_name {
                                    Some(field_name) => {
                                        stmts.push(RustStmt::assign(field_name.clone(), read_expr));
                                        field_inits.push((
                                            field_name.clone(),
                                            Some(RustExpr::local(field_name.clone())),
                                        ));
                                    }
                                    None => stmts.push(RustStmt::Expr(read_expr)),
                                }
                            }
                            let construct = RustExpr::Struct(
                                Constructor::Simple(name.clone()),
                                StructExpr::Record(field_inits),
                            );
                            stmts.push(RustStmt::Return(ReturnKind::Implicit, construct));
                            stmts
                        };
                        let sig = {
                            let arg_type = RustType::borrow_of(
                                None,
                                Mut::Mutable,
                                RustType::Verbatim(
                                    Label::Borrowed("ReadCtxt"),
                                    Some(Box::new(UseParams::from_lt(lt(method_lt.clone())))),
                                ),
                            );
                            let ret = on_type.clone();
                            FnSig::new(vec![(lbl("ctxt"), arg_type)], Some(ret))
                        };
                        RustFn::new_unsafe(
                            "read_unchecked",
                            Some(DefParams::from_lt(method_lt)),
                            sig,
                            body,
                        )
                    };
                    let own_body = vec![
                        TraitItem::AssocType(Label::Borrowed("HostType"), None, host_type),
                        TraitItem::Const(
                            Label::Borrowed("SIZE"),
                            Box::new(RustType::from(PrimType::Usize)),
                            size_const,
                        ),
                        TraitItem::Method(read_unchecked_method),
                    ];
                    dep_impls.push(RustTraitImpl {
                        param_bindings: None,
                        trait_params: None,
                        trait_name: Label::from(Self::get_name()),
                        on_type,
                        body: own_body,
                    });
                    dep_impls
                }
                fixed::FixedShape::Single {
                    format: elem,
                    stride,
                } => match elem {
                    SpineElem::Raw(_) => {
                        // NOTE - Raw BaseKinds will already have a trait impl, so we generate nothing
                        Vec::new()
                    }
                    SpineElem::Indirect(fref, kind) => {
                        let Some((dep_ix, ..)) = type_info.try_resolve_adhoc(fref) else {
                            unreachable!(
                                "indirect spine-elem target {fref:?} has no resolvable ad-hoc Rust type"
                            )
                        };
                        let RustTypeDef::Enum(vars) = &type_info.defined_types[dep_ix].def else {
                            unreachable!(
                                "expected an ad-hoc enum def for Variant-wrapped spine-elem target {fref:?}: {:?}",
                                type_info.defined_types[dep_ix].def,
                            )
                        };
                        let [variant] = vars.as_slice() else {
                            unreachable!(
                                "expected exactly one variant in ad-hoc enum for Variant-wrapped spine-elem target {fref:?}: {vars:?}"
                            )
                        };
                        let variant_label = variant.get_label().clone();
                        let marker = FixedSizeType::Marker(MarkerType::from_base_kind_endian(kind));
                        let inner_read = RustExpr::FunctionCall(
                            Box::new(RustExpr::Entity(RustEntity::Scoped(
                                vec![marker.type_name()],
                                lbl("read_unchecked"),
                            ))),
                            vec![RustExpr::local("ctxt")],
                        );
                        let construct = RustExpr::Struct(
                            Constructor::Compound(name.clone(), variant_label),
                            StructExpr::Tuple(vec![inner_read]),
                        );
                        let method_lt = Label::Borrowed("'a");
                        let body = vec![RustStmt::Return(ReturnKind::Implicit, construct)];
                        let sig = {
                            let arg_type = RustType::borrow_of(
                                None,
                                Mut::Mutable,
                                RustType::Verbatim(
                                    Label::Borrowed("ReadCtxt"),
                                    Some(Box::new(UseParams::from_lt(lt(method_lt.clone())))),
                                ),
                            );
                            let ret = on_type.clone();
                            FnSig::new(vec![(lbl("ctxt"), arg_type)], Some(ret))
                        };
                        let read_unchecked_method = RustFn::new_unsafe(
                            "read_unchecked",
                            Some(DefParams::from_lt(method_lt)),
                            sig,
                            body,
                        );
                        let own_body = vec![
                            TraitItem::AssocType(
                                Label::Borrowed("HostType"),
                                None,
                                on_type.clone(),
                            ),
                            TraitItem::Const(
                                Label::Borrowed("SIZE"),
                                Box::new(RustType::from(PrimType::Usize)),
                                RustExpr::num_lit(stride),
                            ),
                            TraitItem::Method(read_unchecked_method),
                        ];
                        vec![RustTraitImpl {
                            param_bindings: None,
                            trait_params: None,
                            trait_name: Label::from(Self::get_name()),
                            on_type,
                            body: own_body,
                        }]
                    }
                },
            }
        }
    }

    impl TraitObject for ReadUnchecked {
        type TypeInfo<'a> = FixedFormatInfo<'a>;

        fn get_name() -> &'static str {
            "ReadUnchecked"
        }

        fn generate_impls(
            on_type: Box<RustType>,
            type_info: Self::TypeInfo<'_>,
        ) -> Vec<RustTraitImpl> {
            let Some(LocalType::LocalDef(ix, ..)) = on_type.as_local_type() else {
                unreachable!(
                    "ReadUnchecked can only be generated for a locally-defined type: {on_type:?}"
                )
            };
            let Some(format_ref) = type_info.targets.get(*ix).copied() else {
                unreachable!("no FixedFormat source was recorded for type index {ix}: {on_type:?}")
            };
            Self::generate_impls_for(on_type, format_ref, &type_info)
        }
    }
}

// SECTION - boilerplate for trait implementation

/// Produces an `impl ReadUnchecked` block as a standalone item for a type that supports this definition.
pub fn impl_standalone_read_unchecked(
    on_type: Box<RustType>,
    context: smallsorts::FixedFormatInfo<'_>,
) -> Vec<RustDecl> {
    let impl_blocks = smallsorts::ReadUnchecked::generate_impls(on_type, context);
    Vec::from_iter(impl_blocks.into_iter().map(RustDecl::TraitImpl))
}
// !SECTION
