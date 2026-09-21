#![cfg(test)]

use doodle::decoder::{
    Compiler, Program, Value,
    seq_kind::{SeqKind, ValueSeq},
};
use doodle::helper::*;
use doodle::read::ReadCtxt;
use doodle::{Expr, Format, FormatModule, FormatRef};
use doodle_numexpr_macro::numexpr;

#[test]
fn test_native() {
    let mut module = FormatModule::new();
    let fallback =
        module.define_format("test.negative_size.fallback", fmt_variant("small", u32be()));
    let native = module.define_format(
        "test.negative_size.native",
        union_nondet([
            pseudo_record(
                [("x", u32be())],
                fmt_variant(
                    "slice",
                    slice(sub(as_u32(var("x")), Expr::U32(4)), opaque_bytes()),
                ),
            ),
            fallback.call(),
        ]),
    );

    let ok = {
        let mut tmp = Vec::new();
        tmp.extend_from_slice(&u32::to_be_bytes(10));
        tmp.extend_from_slice(b"hello!");
        tmp
    };
    let err = {
        let mut tmp = Vec::new();
        tmp.extend_from_slice(&u32::to_be_bytes(3));
        tmp
    };
    let prog = Compiler::compile_program(&module, &native.call()).expect("compilation failed");
    let ctxt_ok = ReadCtxt::new(&ok);
    let ctxt_err = ReadCtxt::new(&err);
    let (res, _) = prog.run(ctxt_ok).expect("decoding failed on ok-buf"); // should succeed
    check_res_ok(res);
    let (res, _) = prog.run(ctxt_err).expect("decoding failed on err-buf"); // should succeed
    check_res_err(res);
}

#[test]
fn test_numeric() {
    let mut module = FormatModule::new();
    let fallback =
        module.define_format("test.negative_size.fallback", fmt_variant("small", u32be()));
    let numeric = module.define_format(
        "test.negative_size.native",
        union_nondet([
            pseudo_record(
                [("x", u32be())],
                fmt_variant(
                    "slice",
                    slice(numeric(numexpr!("x" -u32 4)), opaque_bytes()),
                ),
            ),
            fallback.call(),
        ]),
    );

    let ok = {
        let mut tmp = Vec::new();
        tmp.extend_from_slice(&u32::to_be_bytes(10));
        tmp.extend_from_slice(b"hello!");
        tmp
    };
    let err = {
        let mut tmp = Vec::new();
        tmp.extend_from_slice(&u32::to_be_bytes(3));
        tmp
    };
    let prog = Compiler::compile_program(&module, &numeric.call()).expect("compilation failed");
    let ctxt_ok = ReadCtxt::new(&ok);
    let ctxt_err = ReadCtxt::new(&err);
    let (res, _) = prog.run(ctxt_ok).expect("decoding failed on ok-buf"); // should succeed
    check_res_ok(res);
    let (res, _) = prog.run(ctxt_err).expect("decoding failed on err-buf"); // will fail
    check_res_err(res);
}

fn check_res_ok(res: Value) {
    let Value::Variant(label, inner) = res.extract_mapped_value().into_inner() else {
        panic!("expected Variant");
    };
    assert_eq!(label, "slice");
    let Value::Seq(SeqKind::Strict(vs)) = *inner else {
        panic!("expected Seq");
    };
    assert_eq!(vs.len(), 6);
    let mut actual = Vec::with_capacity(6);
    for v in vs {
        if let Value::U8(b) = v {
            actual.push(b);
        } else {
            panic!("expected U8");
        }
    }
    assert_eq!(&actual, b"hello!");
}

fn check_res_err(res: Value) {
    let Value::Variant(label, inner) = res.extract_mapped_value().into_inner() else {
        panic!("expected Variant");
    };
    assert_eq!(label, "small");
    let Value::U32(n) = *inner else {
        panic!("expected U32");
    };
    assert_eq!(n, 3);
}
