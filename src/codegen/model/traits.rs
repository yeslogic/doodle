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

    fn generate_impl(on_type: Box<RustType>, type_info: Self::TypeInfo<'_>) -> RustTraitImpl;
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

        fn generate_impl(on_type: Box<RustType>, type_info: Self::TypeInfo<'_>) -> RustTraitImpl {
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
            RustTraitImpl {
                param_bindings,
                trait_name,
                trait_params,
                on_type: Box::new(selftype_with_lt(lt(IMPL_LIFETIME))),
                body,
            }
        }
    }
}

pub mod smallsorts {
    use super::*;
    use crate::fixed;
    use crate::{FormatModule, FormatRef};
    use intmap::IntMap;

    /// Context needed to generate a `ReadUnchecked` impl for a struct that originated from a
    /// `FixedReadKind::FixedFormat`-kinded `ReadArray` element.
    ///
    /// `RustType`/`SourceContext` cannot recover per-field endianness on their own -- a `u32`
    /// field looks the same in `RustType` whether it was parsed as `u32be` or `u32le` -- so
    /// instead of trying to reconstruct the read sequence from the generated type alone, this
    /// resolves `on_type` back to its originating `Format` (via `targets`, populated by
    /// `Elaborator::elaborate_kind`, and `module`) and recovers the exact field layout from there
    /// via `record_fmt::analyze_fixed_shape`.
    #[derive(Clone, Copy)]
    pub(crate) struct FixedFormatInfo<'a> {
        pub module: &'a FormatModule,
        pub targets: &'a IntMap<usize, FormatRef>,
    }

    pub struct ReadUnchecked;

    impl TraitObject for ReadUnchecked {
        // FIXME - at the moemnt, we do not capture enough information to generate the `ReadUnchecked` for SpineShape::Indirect cases
        type TypeInfo<'a> = FixedFormatInfo<'a>;

        fn get_name() -> &'static str {
            "ReadUnchecked"
        }

        fn generate_impl(on_type: Box<RustType>, type_info: Self::TypeInfo<'_>) -> RustTraitImpl {
            let Some(LocalType::LocalDef(ix, name, _params)) = on_type.as_local_type() else {
                unreachable!(
                    "ReadUnchecked can only be generated for a locally-defined type: {on_type:?}"
                )
            };
            let Some(format_ref) = type_info.targets.get(*ix).copied() else {
                unreachable!("no FixedFormat source was recorded for type index {ix}: {on_type:?}")
            };
            let format = type_info.module.get_format(format_ref.get_level());
            let shape = fixed::analyze_fixed_shape(type_info.module, format).unwrap_or_else(|e| {
                unreachable!(
                    "type index {ix} was recorded as a FixedFormat target but is no longer a valid fixed-shape record: {e}"
                )
            });

            let body = match shape {
                fixed::FixedShape::Record { fields, stride } => {
                    let host_type = on_type.clone();
                    let size_const = RustExpr::num_lit(stride);
                    let read_unchecked_method = {
                        let method_lt = Label::Borrowed("'a");
                        let body = {
                            let mut stmts = Vec::with_capacity(fields.len() + 1);
                            let mut field_inits = Vec::with_capacity(fields.len());
                            for (field_name, elem) in &fields {
                                // FIXME - from_spine_elem isn't yet implemented for Indirect
                                let marker = FixedSizeType::from_spine_elem(elem).unwrap();
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
                    vec![
                        TraitItem::AssocType(Label::Borrowed("HostType"), None, host_type),
                        TraitItem::Const(
                            Label::Borrowed("SIZE"),
                            Box::new(RustType::from(PrimType::Usize)),
                            size_const,
                        ),
                        TraitItem::Method(read_unchecked_method),
                    ]
                }
                fixed::FixedShape::Single { format, stride } => todo!(),
            };

            RustTraitImpl {
                param_bindings: None,
                trait_params: None,
                trait_name: Label::from(Self::get_name()),
                on_type,
                body,
            }
        }
    }
}

// SECTION - boilerplate for trait implementation

/// Produces an `impl ReadUnchecked` block as a standalone item for a type that supports this definition.
pub fn impl_standalone_read_unchecked(
    on_type: Box<RustType>,
    context: smallsorts::FixedFormatInfo<'_>,
) -> RustDecl {
    let impl_block = smallsorts::ReadUnchecked::generate_impl(on_type, context);
    RustDecl::TraitImpl(impl_block)
}
// !SECTION
