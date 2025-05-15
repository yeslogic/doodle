use doodle::{helper::*, prelude::ByteSet, Expr, Format, FormatModule, FormatRef};
use doodle_formats::format::base::BaseModule;

/// Greedily matches the given format, skipping any bytes that don't match.
fn skip_to(f: Format) -> Format {
    // FIXME - we need a combinator for this to avoid infinite recursion while constructing the Format
    Format::UnionNondet(vec![
        f.clone(),
        monad_seq(Format::ANY_BYTE, skip_to(f)),
    ])
}

fn only(f: Format) -> Format {
    chain(f, "val", monad_seq(Format::EndOfInput, compute(var("val"))))
}

/// Runs a parser and discards its value, returning `Format::EMPTY`
fn void(f: Format) -> Format {
    monad_seq(f, Format::EMPTY)
}

/// Matches a string (as bytes) exactly, and returns it as a Seq(u8)
fn match_str(bytes: &[u8]) -> Format {
    todo!()
}

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    let simple_ws: ByteSet = ByteSet::from([0x00, 0x09, 0x0C, 0x20]);
    let ascii_lf: u8 = 0x0A;
    let ascii_cr: u8 = 0x0D;

    let inline_char = Format::Byte(ByteSet::from([ascii_lf, ascii_cr]).complement());


    let simple_eol = module.define_format("pdf.util.simple_eol",
        Format::UnionNondet(vec![
            void(is_bytes(&[ascii_cr, ascii_lf])),
            void(is_byte(ascii_lf)),
        ])
    );

    let eol = module.define_format(
        "pdf.util.eol",
        Format::UnionNondet(vec![
            simple_eol.call(),
            void(is_byte(ascii_cr)),
        ])
    );

    let comment = module.define_format(
        "pdf.util.comment",
        monad_seq(
            is_byte(b'%'),
            monad_seq(
                repeat(inline_char),
                eol.call()
            )
        )
    );

    let any_ws = module.define_format("pdf.util.any_ws",
        Format::UnionNondet(vec![
            void(byte_in(simple_ws)), // $simpleWS
            comment.call(), // Comment
            eol.call(), // EOL
        ])
    );

    let token = |f: Format| {
        chain(f, "tok", monad_seq(repeat(any_ws.call()), compute(var("tok"))))
    };

    let unsigned_lead_digits = todo!();

    // TODO - add support to allow de-stringification of numbers
    let unsigned_number = module.define_format("pdf.util.unsigned_number",
        Format::UnionNondet(vec![
            unsigned_lead_digits.call(),
            monad_seq(is_byte(b'.'), todo!()),
        ])
    );
    // TODO: add support to allow de-stringification of numbers
    let number = module.define_format("pdf.util.number", union([
        monad_seq(is_byte(b'-'), map(unsigned_number.call(), lambda("digits", concat(Expr::Seq(vec![Expr::Seq(vec![as_char(Expr::U8(b'-'))]), var("digits")]))))),
        monad_seq(optional(is_byte(b'+')), unsigned_number.call()),
    ]));

    let pdf_chunk = todo!();

    module.define_format("pdf.main",
        only(
            record_auto([
                ("version", skip_to(monad_seq(is_bytes(b"%PDF-"), number.call()))),
                ("chunks", repeat(pdf_chunk.call())),
                ("__rem", repeat(any_ws.call())),
            ])
        )
    )
}
