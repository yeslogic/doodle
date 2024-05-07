use super::{
    error::{PResult, ParseError, StateError},
    offset::{BufferOffset, ByteOffset},
};

pub struct ParseMonad<'a> {
    pub(crate) buffer: &'a [u8],
    pub(crate) offset: BufferOffset,
}

impl<'a> ParseMonad<'a> {
    pub fn new(buffer: &'a [u8]) -> ParseMonad<'a> {
        let max_offset = ByteOffset::from_bytes(buffer.len());
        Self {
            buffer,
            offset: BufferOffset::new(max_offset),
        }
    }

    pub fn advance_by(&mut self, offset: u32) -> Result<(), ParseError> {
        let delta = offset as usize;
        self.offset.try_increment(delta)?;
        Ok(())
    }

    /// Attempts to advance ther buffer by one after capturing the value of the byte at the current logical
    /// offset into the buffer. In bits-mode, this will be a sub-indexed 0-or-1-valued `u8` of the bit in question,
    /// reading from LSB to MSB of each byte in turn. Otherwise, it will be an entire byte.
    pub fn read_byte(&mut self) -> Result<u8, ParseError> {
        let (ix, sub_bit) = self.offset.try_increment(1)?.as_bytes();
        let byte = self.buffer[ix];
        let ret = if let Some(n) = sub_bit {
            let i = n as u8;
            (byte & (1 << i)) >> i
        } else {
            byte
        };
        Ok(ret)
    }

    pub fn skip_align(&mut self, n: usize) -> Result<(), ParseError> {
        let current_offset = self.offset.get_current_offset();
        let aligned_offset = match current_offset {
            ByteOffset::Bytes(nbytes) if nbytes % n == 0 => current_offset,
            ByteOffset::Bits { bits_advanced, .. } if bits_advanced % n == 0 => current_offset,

            ByteOffset::Bytes(nbytes) => ByteOffset::from_bytes(((nbytes / n) + 1) * n),
            ByteOffset::Bits {
                starting_byte,
                bits_advanced,
            } => ByteOffset::Bits {
                starting_byte,
                bits_advanced: (((bits_advanced / n) + 1) * n),
            },
        };
        let delta = current_offset.delta(aligned_offset);
        self.offset.try_increment(delta)?;
        Ok(())
    }

    pub fn enter_bits_mode(&mut self) -> Result<(), ParseError> {
        self.offset.enter_bits_mode()
    }

    pub fn escape_bits_mode(&mut self) -> Result<usize, ParseError> {
        self.offset.escape_bits_mode()
    }

    pub fn get_bits_read(&self) -> Option<usize> {
        self.offset.get_current_offset().bits_advanced()
    }

    pub fn start_slice(&mut self, size: usize) -> Result<(), ParseError> {
        let end = self.offset.get_current_offset().increment_by(size);
        let current_limit = self.offset.current_limit();
        if end > current_limit {
            return Err(ParseError::InternalError(StateError::UnstackableSlices));
        }
        unsafe {
            self.offset.open_slice_unchecked(size);
        }
        Ok(())
    }

    pub fn end_slice(&mut self) -> PResult<()> {
        self.offset.close_slice()?;
        Ok(())
    }

    pub fn open_peek_context(&mut self) {
        self.offset.open_peek()
    }

    pub fn close_peek_context(&mut self) -> Result<(), ParseError> {
        self.offset.close_peek()?;
        Ok(())
    }

    pub fn open_peek_not_context(&mut self) {
        self.offset.open_peeknot()
    }

    pub fn close_peek_not_context(&mut self) -> Result<(), ParseError> {
        self.offset.recover()?;
        Ok(())
    }

    pub fn start_alt(&mut self) {
        self.offset.open_parallel()
    }

    pub fn next_alt(&mut self, is_last: bool) -> Result<(), ParseError> {
        self.offset.recover()?;
        if !is_last {
            self.start_alt();
        }
        Ok(())
    }

    pub fn recover(&mut self) -> Result<(), ParseError> {
        self.offset.recover()?;
        Ok(())
    }

    pub fn remaining(&self) -> usize {
        self.offset.rem_local()
    }

    pub fn finish(&self) -> Result<(), ParseError> {
        match self.remaining() {
            0 => Ok(()),
            n => Err(ParseError::IncompleteParse { bytes_remaining: n }),
        }
    }

    pub fn get_current_offset(&self) -> ByteOffset {
        self.offset.get_current_offset()
    }
}

#[cfg(test)]
mod example {
    use std::io::BufWriter;

    use crate::codegen::{rust_ast::*, IxLabel, ProdCtxt};
    use crate::codegen::{Generator, ToAst};
    use crate::helper::*;
    use crate::{ByteSet, Expr, Format, FormatModule, FormatRef, Pattern};

    const VALID_ASCII: ByteSet = ByteSet::from_bits([u64::MAX, u64::MAX, 0, 0]);

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

    fn register_text_fmt(module: &mut FormatModule) -> FormatRef {
        let mut bs = ByteSet::from(32..=127);
        bs.insert(b'\t');
        bs.insert(b'\n');
        bs.insert(b'\r');
        let ascii_char_strict = module.define_format("base.ascii-char.strict", Format::Byte(bs));

        let utf8_tail =
            module.define_format("utf8.byte.trailing", drop_n_msb(2, byte_in(0x80..=0xbf)));

        let utf8_1 = map(
            Format::Byte(VALID_ASCII),
            lambda("byte", as_u32(var("byte"))),
        );

        let utf8_2 = map(
            tuple([drop_n_msb(3, byte_in(0xc2..=0xdf)), utf8_tail.call()]),
            lambda(
                "bytes",
                expr_match(
                    var("bytes"),
                    vec![(
                        Pattern::Tuple(vec![bind("x1"), bind("x0")]),
                        shift6_2(var("x1"), var("x0")),
                    )],
                ),
            ),
        );

        let utf8_3 = map(
            union([
                tuple([
                    drop_n_msb(4, is_byte(0xe0)),
                    drop_n_msb(2, byte_in(0xa0..=0xbf)),
                    utf8_tail.call(),
                ]),
                tuple([
                    drop_n_msb(4, byte_in(0xe1..=0xec)),
                    utf8_tail.call(),
                    utf8_tail.call(),
                ]),
                tuple([
                    drop_n_msb(4, is_byte(0xed)),
                    drop_n_msb(2, byte_in(0x80..=0x9f)),
                    utf8_tail.call(),
                ]),
                tuple([
                    drop_n_msb(4, byte_in(0xee..=0xef)),
                    utf8_tail.call(),
                    utf8_tail.call(),
                ]),
            ]),
            lambda(
                "bytes",
                expr_match(
                    var("bytes"),
                    vec![(
                        Pattern::Tuple(vec![bind("x2"), bind("x1"), bind("x0")]),
                        shift6_3(var("x2"), var("x1"), var("x0")),
                    )],
                ),
            ),
        );

        let utf8_4 = map(
            union([
                tuple([
                    drop_n_msb(5, is_byte(0xf0)),
                    drop_n_msb(2, byte_in(0x90..=0xbf)),
                    utf8_tail.call(),
                    utf8_tail.call(),
                ]),
                tuple([
                    drop_n_msb(5, byte_in(0xf1..=0xf3)),
                    utf8_tail.call(),
                    utf8_tail.call(),
                    utf8_tail.call(),
                ]),
                tuple([
                    drop_n_msb(5, is_byte(0xf4)),
                    drop_n_msb(2, byte_in(0x80..=0x8f)),
                    utf8_tail.call(),
                    utf8_tail.call(),
                ]),
            ]),
            lambda(
                "bytes",
                expr_match(
                    var("bytes"),
                    vec![(
                        Pattern::Tuple(vec![bind("x3"), bind("x2"), bind("x1"), bind("x0")]),
                        shift6_4(var("x3"), var("x2"), var("x1"), var("x0")),
                    )],
                ),
            ),
        );

        // https://datatracker.ietf.org/doc/html/rfc3629#section-4
        let utf8_char = module.define_format(
            "text.utf8.char",
            map(
                union([utf8_1, utf8_2, utf8_3, utf8_4]),
                lambda("codepoint", as_char(var("codepoint"))),
            ),
        );

        let ascii_str =
            module.define_format("text.string.ascii", repeat1(ascii_char_strict.call()));
        let utf8_str = module.define_format("text.string.utf8", repeat(utf8_char.call()));

        module.define_format(
            "text.string",
            union_nondet(vec![("ascii", ascii_str.call()), ("utf8", utf8_str.call())]),
        )
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

    fn write_program<W>(
        module: &FormatModule,
        top_format: &Format,
        oput: &mut W,
    ) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        let mut items = Vec::new();

        let Generator {
            sourcemap,
            elaborator,
        } = Generator::compile(module, top_format);
        let tdefs = Vec::from_iter(elaborator.iter_defined_types());
        for (ix, tdef) in tdefs.into_iter().enumerate() {
            let it = RustItem::from_decl(RustDecl::TypeDef(IxLabel::from(ix).into(), tdef.clone()));
            items.push(it);
        }

        for decfn in sourcemap.decoder_skels.iter() {
            items.push(RustItem::from_decl(RustDecl::Function(
                decfn.to_ast(ProdCtxt::default()),
            )));
        }

        let mut content = RustProgram::from_iter(items);
        content.add_import(RustImport {
            path: vec!["doodle".into(), "prelude".into()],
            uses: RustImportItems::Wildcard,
        });
        write!(
            oput,
            r#"
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
{}
"#,
            content.to_fragment()
        )
    }

    #[test]
    fn write_code_to_file() -> std::io::Result<()> {
        let mut module = FormatModule::new();
        let text = register_text_fmt(&mut module);
        let handle = std::fs::File::create(std::path::Path::new("utf8reader.rs"))?;
        write_program(&module, &text.call(), &mut BufWriter::new(handle))?;
        Ok(())
    }

    #[test]
    fn test_engine() -> Result<(), crate::parser::error::ParseError> {
        use crate::parser::error::ParseError;
        use crate::prelude::*;

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        enum Type0 {
            ascii(Vec<u8>),
            utf8(Vec<char>),
        }

        impl From<Type0> for String {
            fn from(value: Type0) -> Self {
                match value {
                    Type0::ascii(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                    Type0::utf8(glyphs) => String::from_iter(glyphs.into_iter()),
                }
            }
        }

        #[allow(non_snake_case)]
        fn Decoder0<'input>(input: &mut ParseMonad<'input>) -> PResult<Type0> {
            Decoder1(input)
        }

        #[allow(non_snake_case)]
        fn Decoder1<'input>(input: &mut ParseMonad<'input>) -> PResult<Type0> {
            // Branch #0
            input.start_alt();
            match Decoder2(input) {
                Ok(inner) => return Ok(Type0::ascii(inner)),
                Err(_) => input.next_alt(true)?,
            }
            match Decoder3(input) {
                Ok(inner) => return Ok(Type0::utf8(inner)),
                Err(e) => Err(e),
            }
        }

        #[allow(non_snake_case)]
        fn Decoder2<'input>(input: &mut ParseMonad<'input>) -> PResult<Vec<u8>> {
            let mut accum = Vec::new();
            while input.remaining() > 0 {
                let matching_ix = {
                    input.open_peek_context();
                    let b = input.read_byte()?;
                    if ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0])
                        .contains(b)
                    {
                        1
                    } else {
                        0
                    }
                };
                input.close_peek_context()?;
                if matching_ix == 0 {
                    break;
                } else {
                    let next_elem = Decoder6(input)?;
                    accum.push(next_elem);
                }
            }
            input.finish()?;
            Ok(accum)
        }

        #[allow(non_snake_case)]
        fn Decoder3<'input>(input: &mut ParseMonad<'input>) -> PResult<Vec<char>> {
            let mut accum = Vec::new();
            while input.remaining() > 0 {
                let matching_ix = {
                    input.open_peek_context();
                    let b = input.read_byte()?;
                    match b {
                        tmp if ByteSet::from_bits([
                            18446744073709551615,
                            18446744073709551615,
                            0,
                            0,
                        ])
                        .contains(tmp) =>
                        {
                            0
                        }

                        tmp if ByteSet::from_bits([0, 0, 0, 4294967292]).contains(tmp) => 0,

                        224 => 0,

                        tmp if ByteSet::from_bits([0, 0, 0, 35175782154240]).contains(tmp) => 0,

                        237 => 0,

                        tmp if ByteSet::from_bits([0, 0, 0, 211106232532992]).contains(tmp) => 0,

                        240 => 0,

                        tmp if ByteSet::from_bits([0, 0, 0, 3940649673949184]).contains(tmp) => 0,

                        244 => 0,

                        _other => {
                            return Err(crate::parser::error::ParseError::ExcludedBranch);
                        }
                    }
                };
                input.close_peek_context()?;
                if matching_ix == 0 {
                    let next_elem = Decoder4(input)?;
                    accum.push(next_elem);
                } else {
                    break;
                }
            }
            input.finish()?;
            Ok(accum)
        }

        #[allow(non_snake_case, unreachable_patterns)]
        fn Decoder4<'input>(input: &mut ParseMonad<'input>) -> PResult<char> {
            let inner = {
                let tree_index = {
                    input.open_peek_context();
                    let b = input.read_byte()?;
                    match b {
                        tmp if ByteSet::from_bits([
                            18446744073709551615,
                            18446744073709551615,
                            0,
                            0,
                        ])
                        .contains(tmp) =>
                        {
                            0
                        }

                        tmp if ByteSet::from_bits([0, 0, 0, 4294967292]).contains(tmp) => 1,

                        224 => 2,

                        tmp if ByteSet::from_bits([0, 0, 0, 35175782154240]).contains(tmp) => 2,

                        237 => 2,

                        tmp if ByteSet::from_bits([0, 0, 0, 211106232532992]).contains(tmp) => 2,

                        240 => 3,

                        tmp if ByteSet::from_bits([0, 0, 0, 3940649673949184]).contains(tmp) => 3,

                        244 => 3,

                        _other => {
                            return Err(crate::parser::error::ParseError::ExcludedBranch);
                        }
                    }
                };
                input.close_peek_context()?;
                match tree_index {
                    0 => {
                        let inner = {
                            let b = input.read_byte()?;
                            if ByteSet::from_bits([
                                18446744073709551615,
                                18446744073709551615,
                                0,
                                0,
                            ])
                            .contains(b)
                            {
                                b
                            } else {
                                return Err(ParseError::FailToken);
                            }
                        };
                        (|byte: u8| byte as u32)(inner)
                    }

                    1 => {
                        let inner = {
                            let field0 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if ByteSet::from_bits([0, 0, 0, 4294967292]).contains(b) {
                                        b
                                    } else {
                                        return Err(ParseError::FailToken);
                                    }
                                };
                                (|raw: u8| raw & 31)(inner)
                            };
                            let field1 = { Decoder5(input)? };
                            (field0, field1)
                        };
                        (|bytes: (u8, u8)| match bytes {
                            (x1, x0) => ((x1 as u32) << 6) | (x0 as u32),

                            _other => {
                                unreachable!(r#"unexpected: {:?}"#, _other);
                            }
                        })(inner)
                    }

                    2 => {
                        let inner = {
                            let tree_index = {
                                input.open_peek_context();
                                let b = input.read_byte()?;
                                match b {
                                    224 => 0,

                                    tmp if ByteSet::from_bits([0, 0, 0, 35175782154240])
                                        .contains(tmp) =>
                                    {
                                        1
                                    }

                                    237 => 2,

                                    tmp if ByteSet::from_bits([0, 0, 0, 211106232532992])
                                        .contains(tmp) =>
                                    {
                                        3
                                    }

                                    _other => {
                                        return Err(ParseError::ExcludedBranch);
                                    }
                                }
                            };
                            input.close_peek_context()?;
                            match tree_index {
                                0 => {
                                    let field0 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if b == 224 {
                                                b
                                            } else {
                                                return Err(ParseError::FailToken);
                                            }
                                        };
                                        (|raw: u8| raw & 15)(inner)
                                    };
                                    let field1 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if ByteSet::from_bits([0, 0, 18446744069414584320, 0])
                                                .contains(b)
                                            {
                                                b
                                            } else {
                                                return Err(ParseError::FailToken);
                                            }
                                        };
                                        (|raw: u8| raw & 63)(inner)
                                    };
                                    let field2 = { Decoder5(input)? };
                                    (field0, field1, field2)
                                }

                                1 => {
                                    let field0 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if ByteSet::from_bits([0, 0, 0, 35175782154240])
                                                .contains(b)
                                            {
                                                b
                                            } else {
                                                return Err(ParseError::FailToken);
                                            }
                                        };
                                        (|raw: u8| raw & 15)(inner)
                                    };
                                    let field1 = { Decoder5(input)? };
                                    let field2 = { Decoder5(input)? };
                                    (field0, field1, field2)
                                }

                                2 => {
                                    let field0 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if b == 237 {
                                                b
                                            } else {
                                                return Err(ParseError::FailToken);
                                            }
                                        };
                                        (|raw: u8| raw & 15)(inner)
                                    };
                                    let field1 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if ByteSet::from_bits([0, 0, 4294967295, 0]).contains(b)
                                            {
                                                b
                                            } else {
                                                return Err(ParseError::FailToken);
                                            }
                                        };
                                        (|raw: u8| raw & 63)(inner)
                                    };
                                    let field2 = { Decoder5(input)? };
                                    (field0, field1, field2)
                                }

                                3 => {
                                    let field0 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if ByteSet::from_bits([0, 0, 0, 211106232532992])
                                                .contains(b)
                                            {
                                                b
                                            } else {
                                                return Err(ParseError::FailToken);
                                            }
                                        };
                                        (|raw: u8| raw & 15)(inner)
                                    };
                                    let field1 = { Decoder5(input)? };
                                    let field2 = { Decoder5(input)? };
                                    (field0, field1, field2)
                                }

                                _other => {
                                    return Err(ParseError::ExcludedBranch);
                                }
                            }
                        };
                        (|bytes: (u8, u8, u8)| match bytes {
                            (x2, x1, x0) => ((x2 as u32) << 12) | ((x1 as u32) << 6) | (x0 as u32),

                            _other => {
                                unreachable!(r#"unexpected: {:?}"#, _other);
                            }
                        })(inner)
                    }

                    3 => {
                        let inner = {
                            let tree_index = {
                                input.open_peek_context();
                                let b = input.read_byte()?;
                                match b {
                                    240 => 0,

                                    tmp if ByteSet::from_bits([0, 0, 0, 3940649673949184])
                                        .contains(tmp) =>
                                    {
                                        1
                                    }

                                    244 => 2,

                                    _other => {
                                        return Err(ParseError::ExcludedBranch);
                                    }
                                }
                            };
                            input.close_peek_context()?;
                            match tree_index {
                                0 => {
                                    let field0 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if b == 240 {
                                                b
                                            } else {
                                                return Err(ParseError::ExcludedBranch);
                                            }
                                        };
                                        (|raw: u8| raw & 7)(inner)
                                    };
                                    let field1 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if ByteSet::from_bits([0, 0, 18446744073709486080, 0])
                                                .contains(b)
                                            {
                                                b
                                            } else {
                                                return Err(ParseError::ExcludedBranch);
                                            }
                                        };
                                        (|raw: u8| raw & 63)(inner)
                                    };
                                    let field2 = { Decoder5(input)? };
                                    let field3 = { Decoder5(input)? };
                                    (field0, field1, field2, field3)
                                }

                                1 => {
                                    let field0 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if ByteSet::from_bits([0, 0, 0, 3940649673949184])
                                                .contains(b)
                                            {
                                                b
                                            } else {
                                                return Err(ParseError::ExcludedBranch);
                                            }
                                        };
                                        (|raw: u8| raw & 7)(inner)
                                    };
                                    let field1 = { Decoder5(input)? };
                                    let field2 = { Decoder5(input)? };
                                    let field3 = { Decoder5(input)? };
                                    (field0, field1, field2, field3)
                                }

                                2 => {
                                    let field0 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if b == 244 {
                                                b
                                            } else {
                                                return Err(ParseError::ExcludedBranch);
                                            }
                                        };
                                        (|raw: u8| raw & 7)(inner)
                                    };
                                    let field1 = {
                                        let inner = {
                                            let b = input.read_byte()?;
                                            if ByteSet::from_bits([0, 0, 65535, 0]).contains(b) {
                                                b
                                            } else {
                                                return Err(ParseError::ExcludedBranch);
                                            }
                                        };
                                        (|raw: u8| raw & 63)(inner)
                                    };
                                    let field2 = { Decoder5(input)? };
                                    let field3 = { Decoder5(input)? };
                                    (field0, field1, field2, field3)
                                }

                                _other => {
                                    return Err(ParseError::ExcludedBranch);
                                }
                            }
                        };
                        (|bytes: (u8, u8, u8, u8)| match bytes {
                            (x3, x2, x1, x0) => {
                                ((x3 as u32) << 18)
                                    | ((x2 as u32) << 12)
                                    | ((x1 as u32) << 6)
                                    | (x0 as u32)
                            }

                            _other => {
                                unreachable!(r#"unexpected: {:?}"#, _other);
                            }
                        })(inner)
                    }

                    _other => {
                        return Err(ParseError::ExcludedBranch);
                    }
                }
            };
            Ok((|codepoint: u32| char::from_u32(codepoint).unwrap())(inner))
        }

        #[allow(non_snake_case, unused_variables)]
        fn Decoder5<'input>(input: &mut ParseMonad<'input>) -> PResult<u8> {
            let inner = {
                let b = input.read_byte()?;
                if ByteSet::from_bits([0, 0, 18446744073709551615, 0]).contains(b) {
                    b
                } else {
                    return Err(ParseError::ExcludedBranch);
                }
            };
            Ok((|raw: u8| raw & 63)(inner))
        }

        #[allow(non_snake_case, unused_variables)]
        fn Decoder6<'input>(input: &mut ParseMonad<'input>) -> PResult<u8> {
            let b = input.read_byte()?;
            Ok(
                if ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0])
                    .contains(b)
                {
                    b
                } else {
                    return Err(ParseError::ExcludedBranch);
                },
            )
        }

        fn run_utf8_parse(buf: &[u8]) -> Result<String, ParseError> {
            let mut monad = ParseMonad::new(buf);
            Ok(String::from(Decoder0(&mut monad)?))
        }

        assert_eq!(&run_utf8_parse("hello world".as_bytes())?, "hello world");
        assert_eq!(&run_utf8_parse("λx.x²".as_bytes())?, "λx.x²");
        Ok(())
    }
}
