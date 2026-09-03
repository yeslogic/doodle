use doodle::byte_set::ByteSet;
use doodle::helper::*;
use doodle::{Expr, Format, FormatModule, FormatRef};

// mask table for bitwise and in order to drop N bits, for N = 0 ..= 5
// We technically don't need a mask to drop 0, but it keeps the other indices intuitively correct
const DROPMASKS: [u8; 6] = [
    0b1111_1111, // Drop 0
    0b0111_1111, // Drop 1
    0b0011_1111, // Drop 2
    0b0001_1111, // Drop 3
    0b0000_1111, // Drop 4
    0b0000_0111, // Drop 5
];

fn drop_n_msb(n: usize, format: Format) -> Format {
    map(
        format,
        lambda("raw", bit_and(var("raw"), Expr::U8(DROPMASKS[n]))),
    )
}

pub fn main(module: &mut FormatModule) -> (FormatRef, FormatRef) {
    let utf8_tail = module.define_format("utf8.byte.trailing", drop_n_msb(2, byte_in(0x80..=0xBF)));

    let ascii_nz: ByteSet = ByteSet::intersection(&VALID_ASCII, &!(ByteSet::singleton(0)));

    let utf8_1_nz = map(Format::Byte(ascii_nz), lambda("byte", as_u32(var("byte"))));

    let utf8_2 = map(
        tuple([drop_n_msb(3, byte_in(0xC2..=0xDF)), utf8_tail.call()]),
        lambda_tuple(["x1", "x0"], shift6_2(var("x1"), var("x0"))),
    );

    let utf8_3 = map(
        union([
            tuple([
                drop_n_msb(4, is_byte(0xE0)),
                drop_n_msb(2, byte_in(0xA0..=0xBF)),
                utf8_tail.call(),
            ]),
            tuple([
                drop_n_msb(4, byte_in(0xE1..=0xEC)),
                utf8_tail.call(),
                utf8_tail.call(),
            ]),
            tuple([
                drop_n_msb(4, is_byte(0xED)),
                drop_n_msb(2, byte_in(0x80..=0x9F)),
                utf8_tail.call(),
            ]),
            tuple([
                drop_n_msb(4, byte_in(0xEE..=0xEF)),
                utf8_tail.call(),
                utf8_tail.call(),
            ]),
        ]),
        lambda_tuple(
            ["x2", "x1", "x0"],
            shift6_3(var("x2"), var("x1"), var("x0")),
        ),
    );

    let utf8_4 = map(
        union([
            tuple([
                drop_n_msb(5, is_byte(0xF0)),
                drop_n_msb(2, byte_in(0x90..=0xBF)),
                utf8_tail.call(),
                utf8_tail.call(),
            ]),
            tuple([
                drop_n_msb(5, byte_in(0xF1..=0xF3)),
                utf8_tail.call(),
                utf8_tail.call(),
                utf8_tail.call(),
            ]),
            tuple([
                drop_n_msb(5, is_byte(0xF4)),
                drop_n_msb(2, byte_in(0x80..=0x8F)),
                utf8_tail.call(),
                utf8_tail.call(),
            ]),
        ]),
        lambda_tuple(
            ["x3", "x2", "x1", "x0"],
            shift6_4(var("x3"), var("x2"), var("x1"), var("x0")),
        ),
    );

    let utf8_char_nz = module.define_format(
        "text.utf8.char.non-null",
        map(
            union([utf8_1_nz, utf8_2, utf8_3, utf8_4]),
            lambda("codepoint", as_char(var("codepoint"))),
        ),
    );

    // https://datatracker.ietf.org/doc/html/rfc3629#section-4
    let utf8_char = module.define_format(
        "text.utf8.char",
        union([
            map(
                is_byte(0),
                lambda("_", Expr::AsChar(Box::new(Expr::U32(0)))),
            ),
            utf8_char_nz.call(),
        ]),
    );

    let utf8_zstr = module.define_format("text.string.utf8.non-null", repeat(utf8_char_nz.call()));
    let utf8_str = module.define_format("text.string.utf8", repeat(utf8_char.call()));

    let text = module.define_format("text.string", utf8_str.call());
    (text, utf8_zstr)
}

fn shift6_2(hi: Expr, lo: Expr) -> Expr {
    bit_or(shl(as_u32(hi), Expr::U32(6)), as_u32(lo))
}

fn shift6_3(hi: Expr, mid: Expr, lo: Expr) -> Expr {
    bit_or(shl(as_u32(hi), Expr::U32(12)), shift6_2(mid, lo))
}

fn shift6_4(hh: Expr, hl: Expr, lh: Expr, ll: Expr) -> Expr {
    bit_or(shl(as_u32(hh), Expr::U32(18)), shift6_3(hl, lh, ll))
}
