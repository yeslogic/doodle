pub mod error;
pub mod offset;

use error::{PResult, ParseError, StateError};
use offset::{BufferOffset, ByteOffset};

/// Stateful parser with an associated buffer and offset-tracker.
pub struct Parser<'a> {
    pub(crate) buffer: &'a [u8],
    pub(crate) offset: BufferOffset,
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser` from an immutably-borrowed buffer.
    pub fn new(buffer: &'a [u8]) -> Parser<'a> {
        let max_offset = ByteOffset::from_bytes(buffer.len());
        Self {
            buffer,
            offset: BufferOffset::new(max_offset),
        }
    }

    /// Advances the offset by `offset` positions, as if calling [`Self::read_byte`] that many
    /// times and ignoring the output beyond `Ok`/`Err` distinctions.
    ///
    /// In particular, this will advance the specified number of bits in bits-mode, and bytes in
    /// bytes-mode, and otherwise has no inherent unit attached to it.
    ///
    /// # Note
    ///
    /// For convenience of usage in generated code, the type of the `offset` parameter
    /// is `u32`, as that is much more commonly used in `Format` and `Expr` internally
    /// than `usize`, which would be the natural type for this parameter.
    pub fn advance_by(&mut self, offset: u32) -> Result<(), ParseError> {
        let delta = offset as usize;
        self.offset.try_increment(delta)?;
        Ok(())
    }

    /// Attempts to advance ther buffer by one after capturing the value of the byte at the current logical
    /// offset into the buffer.
    ///
    /// In bits-mode, this will be a sub-indexed 0-or-1-valued `u8` of the bit in question,
    /// reading from LSB to MSB of each byte in turn.
    ///
    /// Otherwise, it will be an entire byte.
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

    /// Advances the current buffer-offset by the minimum number of positions,
    /// in the range `0..n`, such that it is aligned to the nearest greater-or-equal
    /// multiple of `n`.
    ///
    /// In bytes-mode, alignment is measured in terms of how many bytes have been read
    /// since the start of the entire buffer.
    ///
    /// In bits-mode, alignment is measured in terms of how many **bits** have been read
    /// since **entering** bits-mode, and the absolute position within the full buffer
    /// is not regarded.
    ///
    /// # Examples
    ///
    /// ```
    /// use doodle::parser::{Parser, offset::ByteOffset};
    /// let buf = [0u8; 64];
    /// let mut p = Parser::new(&buf);
    ///
    /// let _ = p.read_byte().unwrap(); // now at offset 1B
    /// p.enter_bits_mode().unwrap(); // now at offset 1B:+0b
    /// let _ = p.read_byte().unwrap(); // now at offset 1B:+1b
    /// p.skip_align(16).unwrap(); // now at offset 1B:+16b (and not 1B:+8b ~ 2B (% 16b = 0), or 1B:+120b ~ 16B (% 16B = 0))
    /// assert_eq!(p.get_current_offset(), ByteOffset::Bits { starting_byte: 1, bits_advanced: 16 });
    /// p.escape_bits_mode().unwrap(); // now at offset 3B
    /// p.skip_align(4).unwrap(); // now at offset 4B
    /// assert_eq!(p.get_current_offset(), ByteOffset::Bytes(4));
    /// ```
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

    /// While in bytes-mode (the implicit default), switches to bits-mode until we return to an
    /// earlier offset due to a `restore` or `recover` operation (corner-case),
    /// or until `escape_bits_mode` is called (more common).
    ///
    /// # Note
    ///
    /// Will return an `Err(ParseError::Internal)` value if the current binary mode is already
    /// bits-mode.
    pub fn enter_bits_mode(&mut self) -> Result<(), ParseError> {
        self.offset.enter_bits_mode()
    }

    /// Explicitly escapes from bits-mode and returns to bytes-mode, skipping to the next full byte
    /// if at least one bit of the immediate byte has already been consumed.
    ///
    /// # Note
    ///
    /// Will return an `Err(ParseError::Internal)` value if the current binary mode is not
    /// bits-mode.
    pub fn escape_bits_mode(&mut self) -> Result<usize, ParseError> {
        self.offset.escape_bits_mode()
    }

    /// Returns the number of bits that have been read thusfar in the current
    /// bits-mode context.
    ///
    /// Will return `None` instead if we are not in bits-mode.
    pub fn get_bits_read(&self) -> Option<usize> {
        self.offset.get_current_offset().bits_advanced()
    }

    /// Attempts to open a new slice starting at the current offset and spanning
    /// `size` bytes of buffer.
    ///
    /// Will fail with an appropriate `Err` value if the size of the slice is too large
    /// to fit into an existing slice-context, or would run past the end of the buffer itself.
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

    /// Attempts to close the most recently-opened slice, skipping to the end of the slice
    /// if succesful.
    ///
    /// Will only fail if there are no slices to close, or if the current offset-value somehow
    /// exceeded the slice end-point unexpectedly. In either case, the error cannot be recovered
    /// from.
    pub fn end_slice(&mut self) -> PResult<()> {
        self.offset.close_slice()?;
        Ok(())
    }

    /// Opens a new Peek context, marking the current offset and its modality to be restored
    /// when the matching [`Parser::close_peek_context`] call is reached.
    pub fn open_peek_context(&mut self) {
        self.offset.open_peek()
    }

    /// Closes the most recently-opened Peek context, restoring the offset and its modality
    /// at the time at which it was opened.
    ///
    /// Will return an error if there is no peek context to close.
    pub fn close_peek_context(&mut self) -> PResult<()> {
        self.offset.close_peek()?;
        Ok(())
    }

    /// Opens a new PeekNot context, marking the current offset and its modality to be recover3d
    /// when the matching [`Parser::close_peek_not_context`] call is reached.
    pub fn open_peek_not_context(&mut self) {
        self.offset.open_peeknot()
    }

    /// Closes the most recently-opened PeekNot context, recovering the offset and its modality
    /// at the time at which it was opened.
    ///
    /// Will return an error if there is no PeekNot context to close.
    pub fn close_peek_not_context(&mut self) -> PResult<()> {
        self.offset.recover()?;
        Ok(())
    }

    /// Opens an 'Alts' (non-deterministic alternation) context (as with `Format::UnionNondet`/`Decoder::Alts`),
    /// saving the immediate value of the offset to be recovered upon any branch of the alternation
    /// encountering a failed parse that is not caught by an intervening fail-safe recovery-point, as
    /// with another `Alts` or a `PeekNot` within any branch.
    pub fn start_alt(&mut self) {
        self.offset.open_parallel()
    }

    /// Rewinds to the branch-point of the latest 'Alts' context (e.g. after an individual branch could not be parsed succesfully).
    ///
    /// According to the `is_last` parameter, may also re-create the branch-point to allow chaining with future branches.
    ///
    ///   - If `is_last == true`, it is assumed that the new branch we are proceeding to is the final one, and we should no longer
    ///     be able to recover to the branch-point if it encounters failure
    ///   - If `is_last == false`, it is assumed there is at least one more branch after the one we are proceeding to attempt to parse,
    ///     and we will be able to recover to the branch-point if the immediately successive branch-parse also fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// // Format: Format::UnionNondet(vec![("A", A), ("B", B), ("C", C)])
    /// // Derived Logic (without nesting, the following is desirable)
    /// let mut p = Parser::new(&buf);
    /// p.start_alt();
    /// match parse_a(&mut p) {
    ///     Ok(result_a) => return ..., // short circuit to escape future alternation branches
    ///     Err(_) => p.next_alt(false)?, // the next branch, B, is not the last
    /// }
    /// match parse_b(&mut p) {
    ///     Ok(result_b) => return ..., // short circuit to escape future alternation branches
    ///     Err(_) => p.next_alt(true)?, // the next branch, C, is the last
    /// }
    /// match parse_c(&mut p) {
    ///     Ok(result_c) => return ...,
    ///     Err(e) => return Err(e),
    /// }
    /// ```
    pub fn next_alt(&mut self, is_last: bool) -> Result<(), ParseError> {
        self.offset.recover()?;
        if !is_last {
            self.start_alt();
        }
        Ok(())
    }

    /// Returns the number of bytes that remain in the current, possibly slice-limited context.
    ///
    /// The return value will be `0` if and only if `self.read_byte` will return `Err(ParseError::Overrun(...))`.
    pub fn remaining(&self) -> usize {
        self.offset.rem_local()
    }

    /// Attempts to finish parsing as implied by an `EndOfInput` token, returning an error
    /// if we have not actually finished processing either a slice or the entire buffer.
    ///
    /// The error value will indicate internally how many unconsumed bytes remained at the time of failure.
    pub fn finish(&self) -> Result<(), ParseError> {
        match self.remaining() {
            0 => Ok(()),
            n => Err(ParseError::IncompleteParse { bytes_remaining: n }),
        }
    }

    /// Returns the current value of the `ByteOffset` stored internally.
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
        fn Decoder0<'input>(input: &mut Parser<'input>) -> PResult<Type0> {
            Decoder1(input)
        }

        #[allow(non_snake_case)]
        fn Decoder1<'input>(input: &mut Parser<'input>) -> PResult<Type0> {
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
        fn Decoder2<'input>(input: &mut Parser<'input>) -> PResult<Vec<u8>> {
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
        fn Decoder3<'input>(input: &mut Parser<'input>) -> PResult<Vec<char>> {
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
        fn Decoder4<'input>(input: &mut Parser<'input>) -> PResult<char> {
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
        fn Decoder5<'input>(input: &mut Parser<'input>) -> PResult<u8> {
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
        fn Decoder6<'input>(input: &mut Parser<'input>) -> PResult<u8> {
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
            let mut monad = Parser::new(buf);
            Ok(String::from(Decoder0(&mut monad)?))
        }

        assert_eq!(&run_utf8_parse("hello world".as_bytes())?, "hello world");
        assert_eq!(&run_utf8_parse("λx.x²".as_bytes())?, "λx.x²");
        Ok(())
    }
}
