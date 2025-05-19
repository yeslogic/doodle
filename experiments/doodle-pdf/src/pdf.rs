use std::ops::Range;

use doodle::{
    BaseType, Expr, Format, FormatModule, FormatRef, IntoLabel, Label, Pattern, ValueType,
    helper::*, prelude::ByteSet,
};
use doodle_formats::format::base::BaseModule;

fn hex_val() -> Format {
    const HEX09: Range<u8> = b'0'..b'9';
    const HEX_MAJ: Range<u8> = b'A'..b'F';
    const HEX_MIN: Range<u8> = b'a'..b'f';

    union([
        map(byte_in(HEX09), lambda("b", sub(var("b"), Expr::U8(b'0')))),
        map(
            byte_in(HEX_MIN),
            lambda("b", sub(var("b"), Expr::U8(b'a' - 10))),
        ),
        map(
            byte_in(HEX_MAJ),
            lambda("b", sub(var("b"), Expr::U8(b'A' - 10))),
        ),
    ])
}

fn record_daedalus<Name: IntoLabel + AsRef<str>>(
    fields: impl IntoIterator<Item = (Name, Format), IntoIter: DoubleEndedIterator>,
) -> Format {
    let mut uniq_ix: Option<usize> = None;
    let mut fields_ext = Vec::new();
    for (_ix, (label, format)) in fields.into_iter().enumerate() {
        let field = match label.as_ref() {
            _nil if _nil.is_empty() || _nil.starts_with("__") => (None, format),
            _tmp if _tmp.starts_with("_") || _tmp.starts_with("@") => {
                (Some((Label::from(_tmp.to_owned()), true)), format)
            }
            uniq if uniq.starts_with("$$") => match uniq_ix.replace(_ix) {
                None => {
                    // patch all previous fields to be temporary
                    for (fld_info, _) in fields_ext.iter_mut() {
                        if let Some((_, is_persist)) = fld_info {
                            *is_persist = false;
                        }
                    }
                    (Some((uniq.replace("$", "x").into(), true)), format)
                }
                Some(old) => {
                    unreachable!("cannot have more than one '$$' field: {} != {}", old, _ix)
                }
            },
            other => (Some((other.to_owned().into(), uniq_ix.is_none())), format),
        };
        fields_ext.push(field)
    }
    record_ext(fields_ext, true)
}

fn hex_num_u32(len: usize) -> Format {
    let raw = repeat_count(Expr::U32(len as u32), hex_val());
    map(
        raw,
        lambda(
            "raw",
            left_fold(
                lambda_tuple(["acc", "b"], add(var("b"), mul(var("acc"), Expr::U32(16)))),
                Expr::U32(0),
                ValueType::Base(BaseType::U32),
                var("raw"),
            ),
        ),
    )
}

fn interpret(assocs: impl IntoIterator<Item = (Format, Expr)>) -> Format {
    union(assocs.into_iter().map(|(f, e)| monad_seq(f, compute(e))))
}

/// Greedily matches the given format, skipping any bytes that don't match.
fn skip_to(_f: Format) -> Format {
    // FIXME - we need a combinator for this to avoid infinite recursion while constructing the Format
    // Format::UnionNondet(vec![
    //     f.clone(),
    //     monad_seq(Format::ANY_BYTE, skip_to(f)),
    // ])
    todo!()
}

/// Parse `f` and ensure that there is no trailing content
fn only(f: Format) -> Format {
    chain(f, "val", monad_seq(Format::EndOfInput, compute(var("val"))))
}

/// Runs a parser and discards its value, returning `Format::EMPTY`
fn void(f: Format) -> Format {
    monad_seq(f, Format::EMPTY)
}

/// Matches a string (as bytes) exactly, and returns it as a Seq(u8)
fn match_str(bytes: &[u8]) -> Format {
    byte_seq(bytes)
}

const ASCII_LF: u8 = b'\n';
const ASCII_CR: u8 = b'\r';

const ASCII_NUL: u8 = b'\0';
const ASCII_TAB: u8 = b'\t';
const ASCII_FF: u8 = b'\x0c';
const ASCII_SPACE: u8 = b' ';

const __WS_MASK: u64 = 1u64 | 1 << ASCII_TAB | 1 << ASCII_FF | 1 << ASCII_SPACE;
const SIMPLE_WS: ByteSet = ByteSet::from_bits([__WS_MASK, 0, 0, 0]);

const DIGIT: ByteSet = ByteSet::from_bits([0b11_1111_1111 << b'0', 0, 0, 0]);

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    {
        let _simple_ws = ByteSet::from([b'\0', b'\t', 0x0C, b' ']);
        debug_assert_eq!(_simple_ws, SIMPLE_WS);

        let _digit = ByteSet::from(b'0'..=b'9');
        debug_assert_eq!(_digit, DIGIT);
    }

    let inline_char = Format::Byte(ByteSet::from([ASCII_LF, ASCII_CR]).complement());

    let simple_eol = module.define_format(
        "pdf.util.simple_eol",
        Format::UnionNondet(vec![
            void(is_bytes(&[ASCII_CR, ASCII_LF])),
            void(is_byte(ASCII_LF)),
        ]),
    );

    let eol = module.define_format(
        "pdf.util.eol",
        Format::UnionNondet(vec![simple_eol.call(), void(is_byte(ASCII_CR))]),
    );

    let then_eol =
        |format: Format| chain(format, "ret", monad_seq(eol.call(), compute(var("ret"))));

    let comment = module.define_format(
        "pdf.util.comment",
        monad_seq(is_byte(b'%'), monad_seq(repeat(inline_char), eol.call())),
    );

    let any_ws = module.define_format(
        "pdf.util.any_ws",
        Format::UnionNondet(vec![
            void(byte_in(SIMPLE_WS)), // $simpleWS
            comment.call(),           // Comment
            eol.call(),               // EOL
        ]),
    );

    let token = |f: Format| {
        chain(
            f,
            "tok",
            monad_seq(repeat(any_ws.call()), compute(var("tok"))),
        )
    };

    let kw = |b: &[u8]| token(match_str(b));

    let between = |open: &[u8], close: &[u8], f: Format| {
        monad_seq(
            kw(open),
            chain(f, "val", monad_seq(kw(close), compute(var("val")))),
        )
    };

    let natural = module.define_format("pdf.util.natural", repeat1(Format::Byte(DIGIT)));

    let unsigned_lead_digits = chain(
        natural.call(),
        "n",
        Format::UnionNondet(vec![
            chain(
                cons(is_byte(b'.'), repeat(Format::Byte(DIGIT))),
                "frac",
                compute(append(var("n"), var("frac"))),
            ),
            compute(var("n")),
        ]),
    );

    // TODO - add support to allow de-stringification of numbers
    let unsigned_number = module.define_format(
        "pdf.util.unsigned_number",
        Format::UnionNondet(vec![
            unsigned_lead_digits,
            cons(is_byte(b'.'), repeat1(Format::Byte(DIGIT))),
        ]),
    );

    // TODO: add support to allow de-stringification of numbers
    let number = module.define_format(
        "pdf.util.number",
        union([
            monad_seq(
                is_byte(b'-'),
                map(
                    unsigned_number.call(),
                    lambda(
                        "digits",
                        concat(Expr::Seq(vec![
                            Expr::Seq(vec![as_char(Expr::U8(b'-'))]),
                            var("digits"),
                        ])),
                    ),
                ),
            ),
            monad_seq(optional(is_byte(b'+')), unsigned_number.call()),
        ]),
    );

    let v_null = void(kw(b"null"));
    let v_bool = interpret([
        (kw(b"true"), Expr::Bool(true)),
        (kw(b"false"), Expr::Bool(false)),
    ]);
    let v_ref = record_auto([
        ("obj", token(natural.call())),
        ("gen", token(natural.call())),
        ("__kw", kw(b"R")),
    ]);
    let name_char = {
        let ok_raw = ByteSet::from(*b"\0\t\n\x0C\r ()<>[]{}/%#").complement();
        let name_esc = monad_seq(
            is_byte(b'#'),
            where_nonzero::<U8>(map(hex_num_u32(2), lambda("x32", as_u8(var("x32"))))),
        );
        union([Format::Byte(ok_raw), name_esc])
    };
    let v_name = module.define_format(
        "pdf.value.name",
        token(cons(is_byte(b'/'), repeat(name_char))),
    );
    let v_string = {
        let string_in_parens = module.define_format_args(
            "pdf.value.string_in_parens",
            vec![(Label::Borrowed("lim"), ValueType::Base(BaseType::U64))],
            if_then_else(
                expr_eq(var("lim"), Expr::U64(0)),
                Format::Fail, // "string nesting limit exceeded"
                snoc(cons(match_str(b"("), todo!()), match_str(b")")), // TODO: support mutual recursion
            ),
        );

        let string_esc = todo!();

        let string_chunk = module.define_format_args(
            "pdf.value.string_chunk",
            vec![(Label::Borrowed("lim"), ValueType::Base(BaseType::U64))],
            Format::UnionNondet(vec![
                string_in_parens.call_args(vec![var("lim")]),
                string_esc,
                repeat1(byte_in(ByteSet::from(*b"\\()").complement())),
            ]),
        );

        let string_chars = module.define_format_args(
            "pdf.value.string_chars",
            vec![(Label::Borrowed("lim"), ValueType::Base(BaseType::U64))],
            map(
                repeat(string_chunk.call_args(vec![var("lim")])),
                lambda("xs", flat_map(lambda("x", var("x")), var("xs"))),
            ),
        );

        record_daedalus([
            ("__open", is_byte(b'(')),
            ("$$", string_chars.call_args(vec![Expr::U64(16)])),
            ("__close", is_byte(b')')),
            ("__skipWS", repeat(any_ws.call())),
        ])
    };
    let v_hex_string = { todo!() };
    let pdf_value = module.define_format(
        "pdf.value",
        union_nondet([
            ("null", v_null),
            ("bool", v_bool),
            ("ref", v_ref),
            ("name", v_name.call()),
            ("string", v_string),
            ("string", v_hex_string),
        ]),
    );
    let pdf_value_type = module.get_format_type(pdf_value.get_level()).clone();
    let stream: FormatRef = todo!("implement Stream (val : Value)");
    let top_decl_def = module.define_format_args(
        "pdf.top_decl",
        vec![(Label::Borrowed("val"), pdf_value_type)],
        record([
            ("stream", stream.call_args(vec![var("val")])),
            ("value", compute(var("val"))),
        ]),
    );
    let obj_decl = module.define_format(
        "pdf.object",
        record_auto([
            ("id", token(natural.call())),
            ("gen", token(natural.call())),
            ("__kw_obj", kw(b"obj")),
            ("_val", pdf_value.call()),
            ("obj", top_decl_def.call_args(vec![var("_val")])),
            ("__kw_end", kw(b"endobj")),
        ]),
    );
    let cross_ref_section: FormatRef = todo!();
    let pdf_dict = {
        let dict = module.define_format_args(
            "pdf.util.dict",
            vec![(Label::Borrowed("lim"), ValueType::Base(BaseType::U64))],
            between(
                b"<<",
                b">>",
                repeat(record([
                    ("key", v_name.call()),
                    ("value", pdf_value.call_args(vec![var("lim")])),
                ])),
            ),
        );

        module.define_format("pdf.util.pdf-dict", dict.call_args(vec![Expr::U64(32)]))
    };

    let trailer = module.define_format("pdf.trailer", monad_seq(kw(b"trailer"), pdf_dict.call()));

    let pdf_chunk = module.define_format(
        "pdf.chunk",
        record_auto([
            ("objects", repeat(obj_decl.call())),
            ("xref", cross_ref_section.call()),
            ("trailer", trailer.call()),
            ("__start_xref", then_eol(match_str(b"startxref"))),
            ("declared_start_xref", then_eol(natural.call())),
            ("__kw_eof", kw(b"%%EOF")),
        ]),
    );

    module.define_format(
        "pdf.main",
        only(record_auto([
            (
                "version",
                skip_to(monad_seq(is_bytes(b"%PDF-"), number.call())),
            ),
            ("chunks", repeat(pdf_chunk.call())),
            ("__rem", repeat(any_ws.call())),
        ])),
    )
}
