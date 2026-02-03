use super::*;
use crate::codegen::rust_ast::analysis::{SourceContext, read_width::ReadWidth as _};

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
    use crate::codegen::{DecoderFn, catalog::CrossIndex, decoder_fname, typed_format::{GenType, TypedExpr}};
    use super::*;


    #[derive(Clone, Copy)]
    pub(crate) struct TypeParseInfo<'a> {
        pub catalog: &'a CrossIndex,
        pub decoders: &'a [DecoderFn<TypedExpr<GenType>>],
    }

    pub struct CommonObject;


    impl TraitObject for CommonObject {
        type TypeInfo<'a> = TypeParseInfo<'a>;

        fn get_name() -> &'static str {
            "CommonObject"
        }

        fn generate_impl(on_type: Box<RustType>, type_info: Self::TypeInfo<'_>) -> RustTraitImpl {
            let trait_name = lbl(Self::get_name());
            let RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(ix, _, params))) = on_type.as_ref() else {
                unreachable!("unexpected non-local type-reference encountered during {trait_name} trait-impl generation: {on_type:?}")
            };
            let param_bindings = match params {
                None => None,
                Some(params) => {
                    match &params.lt_params[..] {
                        [] => None,
                        [lt] => {
                            let lt_var = lt.as_ref().clone();
                            Some(Box::new(DefParams::from_lt(lt_var)))
                        }
                        _ => unreachable!("unexpected number of lifetime parameters encountered during {trait_name} trait-impl generation: {on_type:?}"),
                    }
                }
            };
            let trait_params = None;
            let body = {
                let canonical_decoder = {
                    let decoder_ix_set = type_info.catalog.get(*ix).unwrap();
                    match decoder_ix_set.len() {
                        0 => unreachable!("unexpected empty decoder list encountered during {trait_name} trait-impl generation: {on_type:?}"),
                        1 => {
                            let decoder_ix = decoder_ix_set.as_ref()[0];
                            let decoder_fn = &type_info.decoders[decoder_ix];
                            decoder_fn
                        },
                        2.. => unreachable!("unexpected ambiguous decoder list ({decoder_ix_set:?}) encountered during {trait_name} trait-impl generation: {on_type:?}"),
                    }
                };
                let extra_args = match &canonical_decoder.extra_args {
                    Some(extra_args) if !extra_args.is_empty() => {
                        Some(extra_args)
                    }
                    None => None,
                    Some(_) => {
                        // NOTE - this panic is really for fringe-case discovery, it should eventually be merged with the `None` handling after we are confident there isn't a bug
                        unreachable!("unexpected Some([]) extra_args found in decoder: {canonical_decoder:?}")
                    }
                };
                // NOTE - the body may change as the trait is redesigned in future iterations
                let def_gat_args = {
                    let params = Some(Box::new(DefParams::from_lt(lbl("'a"))));
                    let rhs = {
                        match extra_args {
                            None => RustType::Atom(AtomType::Prim(PrimType::Unit)),
                            Some(extra_args) => {
                                let mut accum = Vec::with_capacity(extra_args.len());
                                for (_ident, ty) in extra_args.iter() {
                                    accum.push(ty.to_rust_type())
                                }
                                RustType::AnonTuple(accum)
                            }
                        }
                    };
                    let decl = TraitItem::AssocType(
                        lbl("Args"),
                        params,
                        Box::new(rhs),
                    );
                    decl
                };
                let def_gat_output = {
                    // REVIEW - this GAT may be subject to redesign, but for now we have `Self::Output = Self`.
                    let lt_ident = match on_type.lt_param() {
                        None => &lbl("'a"),
                        Some(lt) => lt.as_ref(),
                    };
                    let rhs = on_type.clone();
                    TraitItem::AssocType(
                        lbl("Output"),
                        Some(Box::new(DefParams::from_lt(lt_ident.clone()))),
                        rhs,
                    )
                };
                let def_method_parse = {
                    const LT_PARAM: &str = "'input";
                    let params = Some(DefParams {
                        lt_params: vec![lbl(LT_PARAM)],
                        ty_params: vec![],
                    });
                    let sig = {
                        let args = {
                            let arg0 = {
                                let name = lbl("p");
                                let ty = {
                                    let params = UseParams::from_lt(lt(LT_PARAM));
                                    RustType::borrow_of(
                                        None,
                                        Mut::Mutable,
                                        RustType::verbatim("Parser", Some(params)),
                                    )
                                };
                                (name, ty)
                            };
                            let arg1 = {
                                if extra_args.is_some() {
                                    let name = lbl("args");
                                    let ty = RustType::Verbatim(
                                        lbl("Self::Args"),
                                        Some(Box::new(UseParams::from_lt(lt(LT_PARAM)))),
                                    );
                                    (name, ty)
                                } else {
                                    (lbl("_"), RustType::from(PrimType::Unit))
                                }
                            };
                            vec![arg0, arg1]
                        };
                        FnSig::new(
                            args,
                            Some(RustType::result_of(
                                RustType::Verbatim(lbl("Self::Output"), Some(Box::new(UseParams::from_lt(lt(LT_PARAM))))),
                                RustType::imported("ParseError"),
                            )),
                        )
                    };
                    let body = {
                        let fname = decoder_fname(canonical_decoder.ixlabel);
                        let num_extra_args = extra_args.map_or(0, Vec::len);
                        let mut stmts = Vec::with_capacity(num_extra_args + 1);
                        let args = {
                            let mut accum = Vec::with_capacity(num_extra_args + 1);

                            let parser_arg = RustExpr::local("p");
                            accum.push(parser_arg);

                            if let Some(args) = extra_args {
                                for (ix, (ident, _)) in args.iter().enumerate() {
                                    stmts.push(RustStmt::assign(
                                        ident.clone(), RustExpr::local("args").at_pos(ix)
                                    ));
                                    accum.push(RustExpr::local(ident.clone()))
                                }
                            }

                            accum
                        };
                        let call = {
                            RustExpr::FunctionCall(
                                Box::new(RustExpr::local(fname)),
                                args,
                            )
                        };
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
                on_type,
                body,
            }
        }
    }
}

pub mod smallsorts {
    use super::*;

    #[expect(dead_code)]
    pub struct ReadBinaryDep;

    pub struct ReadFixedSizeDep;

    impl TraitObject for ReadFixedSizeDep {
        type TypeInfo<'a> = &'a SourceContext<'a>;

        fn get_name() -> &'static str {
            "ReadFixedSizeDep"
        }

        // fn lt_params() -> usize {
        //     0
        // }

        // fn ty_params() -> usize {
        //     0
        // }

        // fn satisfies_requirements(
        //     on_type: &RustType,
        //     type_info: Self::TypeInfo<'_>,
        // ) -> bool {
        //     on_type.read_width(type_info).as_fixed().is_some()
        // }

        fn generate_impl(
            on_type: Box<RustType>,
            type_info: Self::TypeInfo<'_>,
        ) -> RustTraitImpl {
            let body = {
                let size_method = {
                    let body = {
                        let size = on_type.read_width(type_info).as_fixed().unwrap();
                        let val_size = RustExpr::num_lit(size);
                        vec![RustStmt::Return(ReturnKind::Implicit, val_size)]
                    };
                    let sig = {
                        // FIXME - add qualification scoping to RustType Verbatim to avoid this hardcoding
                        let arg_type = RustType::Verbatim(
                            lbl("Self::Args"),
                            Some(Box::new(UseParams::from_lt(RustLt::WILD))),
                        );
                        let ret = RustType::from(PrimType::Usize);
                        FnSig::new(vec![(lbl("args"), arg_type)], Some(ret))
                    };
                    RustFn::new("size", None, sig, body)
                };
                vec![TraitItem::Method(size_method)]
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

/// Produces an `impl ReadFixedSizeDep` block as a standalone item, assuming that `ReadBinaryDep`
/// will be implemented separately.
#[expect(dead_code)]
pub fn impl_standalone_read_fixed_size_dep(
    on_type: Box<RustType>,
    context: &SourceContext<'_>,
) -> RustDecl {
    let impl_block = smallsorts::ReadFixedSizeDep::generate_impl(on_type, context);
    RustDecl::TraitImpl(impl_block)
}

#[expect(dead_code)]
pub fn impl_standalone_read_binary_dep(
    _on_type: Box<RustType>,
    _context: &SourceContext<'_>,
) -> RustDecl {
    // let impl_block = smallsorts::ReadBinaryDep::generate_impl(on_type, context);
    // RustDecl::TraitImpl(impl_block)
    todo!()
}
// !SECTION
