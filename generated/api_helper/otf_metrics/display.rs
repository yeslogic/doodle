/// Constructs an inline-text `Token` consisting of the string `str`.
pub fn tok(str: impl Into<doodle::Label>) -> Token {
    Token::InlineText(str.into())
}

/// Converts a `&'static str`, `String`, or `Label` into a `TokenStream`.
///
/// Equivalent to `TokenStream::from(tok(str))`.
pub fn toks(str: impl Into<doodle::Label>) -> TokenStream<'static> {
    TokenStream::from(Token::InlineText(str.into()))
}

/// Simple token consisting of either an inline-string or a newline.
#[derive(Clone)]
pub enum Token {
    InlineText(doodle::Label),
    LineBreak,
    IncreaseIndent,
    DecreaseIndent,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::InlineText(s) => write!(f, "{s}"),
            Token::LineBreak => write!(f, "\n"),
            Token::IncreaseIndent | Token::DecreaseIndent => Ok(()), // No visual representation for indent tokens, since they are handled implicitly by `IndentIter`
        }
    }
}

impl Token {
    /// Creates a new `TokenStream` by prepending `self` to `stream`.
    pub fn then(self, stream: TokenStream<'_>) -> TokenStream<'_> {
        <Token as Into<TokenStream<'static>>>::into(self).chain(stream)
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Token::InlineText(s.into())
    }
}

impl From<Token> for TokenStream<'static> {
    fn from(value: Token) -> Self {
        TokenStream {
            inner: Box::new(std::iter::once(value)),
        }
    }
}

#[must_use]
pub struct TokenStream<'a> {
    inner: Box<dyn Iterator<Item = Token> + 'a>,
}

impl<'a> TokenStream<'a> {
    /// Constructs a new empty `TokenStream`.
    pub fn empty() -> TokenStream<'static> {
        TokenStream {
            inner: Box::new(std::iter::empty()),
        }
    }

    /// Creates a new `TokenStream` from a stream of tokens.
    pub fn from_stream(stream: impl Iterator<Item = Token> + 'a) -> Self {
        TokenStream {
            inner: Box::new(stream),
        }
    }

    /// Writes the tokens in `self` to `w`.
    ///
    /// If successful, returns `Ok(true)` if at least one token was written and `Ok(false)` if `self` was empty.
    pub fn write_to<W: std::io::Write>(self, mut w: W) -> std::io::Result<bool> {
        let mut has_written = false;
        let writer = IndentIter::new(Indent::default(), self.inner);
        for token in writer {
            write!(w, "{token}")?;
            has_written = true;
        }
        Ok(has_written)
    }

    /// Prints the tokens in `self` to stdout.
    pub fn print(self) -> bool {
        if cfg!(test) {
            // In tests, we don't want to print anything.
            let mut buf = std::io::sink();
            return self.write_to(&mut buf).unwrap();
        } else {
            let oput = std::io::stdout().lock();
            let buf = &mut std::io::BufWriter::new(oput);
            self.write_to(buf).unwrap()
        }
    }

    /// Prints the tokens in `self` to stdout, with a single trailing newline.
    ///
    /// If `self` is empty, no newline is printed.
    pub fn println(self) {
        let has_printed = self.print();
        if has_printed {
            println!();
        }
    }

    /// Serializes `self` into a contiguous string.
    pub fn into_string(self) -> String {
        let mut buf = String::new();
        for token in self.inner {
            buf.push_str(&token.to_string());
        }
        buf
    }

    /// Returns a new `TokenStream` where all co-adjacent inline-text tokens are merged into a single string.
    ///
    /// Implicitly assumes that no newline characters occur within `InlineText` tokens.
    pub fn group_lines(self) -> TokenStream<'static> {
        let mut lines = Vec::new();
        let mut line = String::new();
        for token in self.inner {
            match token {
                Token::InlineText(s) => line.push_str(&s),
                Token::DecreaseIndent => lines.push(Token::DecreaseIndent),
                Token::IncreaseIndent => lines.push(Token::IncreaseIndent),
                Token::LineBreak => {
                    lines.push(Token::InlineText(std::borrow::Cow::Owned(line.clone())));
                    line.clear();
                }
            }
        }
        TokenStream {
            inner: Box::new(lines.into_iter()),
        }
    }

    /// Returns a new `TokenStream` equivalent to `self ++ other`.
    pub fn chain(self, other: Self) -> Self {
        TokenStream {
            inner: Box::new(self.inner.chain(other.inner)),
        }
    }

    /// Returns a new `TokenStream` equivalent to `self ++ [glue] ++ other`
    pub fn glue(self, glue: Token, other: Self) -> Self {
        Self {
            inner: Box::new(self.inner.chain(std::iter::once(glue).chain(other.inner))),
        }
    }

    /// Returns a new `TokenStream` equivalent to `[before] ++ self ++ [after]`
    pub fn surround(self, before: Token, after: Token) -> Self {
        Self {
            inner: Box::new(
                std::iter::once(before)
                    .chain(self.inner)
                    .chain(std::iter::once(after)),
            ),
        }
    }

    /// Surrounds the TokenStream with `'('..')'`
    pub fn paren(self) -> Self {
        self.surround(tok("("), tok(")"))
    }

    /// Surrounds the TokenStream with `'['..']'`
    pub fn bracket(self) -> Self {
        self.surround(tok("["), tok("]"))
    }

    /// Appends a forced line-break to `self`.
    pub fn break_line(self) -> TokenStream<'a> {
        TokenStream {
            inner: Box::new(self.inner.chain(std::iter::once(Token::LineBreak))),
        }
    }

    /// Returns a new `TokenStream` where each line in `self` is indented by `stops` half-tabs (i.e. 4 spaces).
    ///
    /// Will prefer using `'\t'` for indentation, and will include a final half-tab iff `stops` is odd.
    ///
    /// Can take a negative value, but the formatting will be incorrect when underflowing the current absolute indentation level.
    pub fn indent_by(self, stops: i8) -> Self {
        // REVIEW - we can change the implementation of IndentIter to support negative stops as equivalent-to-zero and thereby avoid the underflow misalignment-on-reset
        match stops.signum() {
            0 => return self,
            1 => (),
            -1 => return self.decrease_indent_by(-stops as u8),
            _ => unreachable!(),
        }

        let shift = std::iter::repeat(Token::IncreaseIndent).take(stops as usize);
        let reset = std::iter::repeat(Token::DecreaseIndent).take(stops as usize);
        TokenStream {
            inner: Box::new(
                shift.chain(self.inner).chain(reset), // We rely on the fact that `IndentIter` will ignore all indent tokens at the end of the stream, so we can safely append the necessary `DecreaseIndent` tokens here without worrying about trailing newlines in `self`
            ),
        }
    }

    /// Internal helper for `indent_by` to decrease indentation by a positive number of stops.
    ///
    /// # Notes
    ///
    /// If the current indentation level is less than `stops`, the subsequent re-indentation will be incorrect, as the underflow is silently ignored.
    fn decrease_indent_by(self, stops: u8) -> Self {
        let shift = std::iter::repeat(Token::DecreaseIndent).take(stops as usize);
        let reset = std::iter::repeat(Token::IncreaseIndent).take(stops as usize);
        TokenStream {
            inner: Box::new(
                shift.chain(self.inner).chain(reset), // We rely on the fact that `IndentIter` will ignore all indent tokens at the end of the stream, so we can safely append the necessary `DecreaseIndent` tokens here without worrying about trailing newlines in `self`
            ),
        }
    }

    /// Joins a stream of streams with a given separator.
    pub fn join_with(streams: Vec<TokenStream<'static>>, sep: Token) -> TokenStream<'a> {
        TokenStream {
            inner: Box::new(IntersperseIter::new(
                Box::new(streams.into_iter().map(|s| s.inner)),
                sep,
            )),
        }
    }
}

pub struct IndentIter<'a> {
    level: Indent,
    stream: Box<dyn Iterator<Item = Token> + 'a>,
    at_line_start: bool,
    // Temporary cache for the next token to be emitted by `stream`, to avoid indenting after the final linebreak in `stream`
    next_token: Option<Token>,
}

impl<'a> IndentIter<'a> {
    fn new(level: Indent, stream: Box<dyn Iterator<Item = Token> + 'a>) -> Self {
        Self {
            level,
            stream,
            at_line_start: true,
            next_token: None,
        }
    }
}

impl<'a> Iterator for IndentIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tok) = self.next_token.take() {
            return Some(tok);
        } else {
            let next = self.stream.next();
            match next {
                Some(Token::LineBreak) => {
                    self.at_line_start = true;
                    return next;
                }
                Some(Token::IncreaseIndent) => {
                    self.level.0 += 1;
                    return self.next();
                }
                Some(Token::DecreaseIndent) => {
                    self.level.0 = self.level.0.saturating_sub(1);
                    return self.next();
                }
                Some(tok) => {
                    if self.at_line_start {
                        self.next_token = Some(tok);
                        self.at_line_start = false;
                        let full_tabs = self.level.0 / 2;
                        let partial_tabs = self.level.0 % 2;

                        let indent_str = "\t".repeat(full_tabs as usize)
                            + if partial_tabs > 0 { Indent::HT } else { "" };
                        return Some(Token::InlineText(indent_str.into()));
                    } else {
                        return Some(tok);
                    }
                }
                None => return None,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Newtype representing indentation level, measured in 4-space half-tabs.
pub struct Indent(u8);

impl Indent {
    /// Half-Tab for partial indentation
    pub const HT: &str = "    ";
}

pub struct IntersperseIter<'a, T: Clone> {
    items: Box<dyn Iterator<Item = Box<dyn Iterator<Item = T> + 'a>>>,
    rest: Box<dyn Iterator<Item = T> + 'a>,
    sep: T,
    non_empty: bool,
}

impl<'a, T: 'static + Clone> IntersperseIter<'a, T> {
    pub fn new(items: Box<dyn Iterator<Item = Box<dyn Iterator<Item = T> + 'a>>>, sep: T) -> Self {
        Self {
            items,
            rest: Box::new(std::iter::empty()),
            sep,
            non_empty: false,
        }
    }
}

impl<'a, T: Clone> std::iter::Iterator for IntersperseIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rest.next() {
            None => match self.items.next() {
                // Return None iff there are no more tokens in `rest` and no more TokenStreams in `items`
                None => None,
                Some(iter) => {
                    // Once we have run out of tokens in `rest`, we reset `rest` to the next TokenStream in `items`
                    self.rest = iter;
                    if self.non_empty {
                        // If the previous item (i.e. the full `rest` before depletion) was non-empty, emit the separator and reset to `non_empty = false`
                        self.non_empty = false;
                        Some(self.sep.clone())
                    } else {
                        // Since the previous `rest` was empty, we skip the separator and recurse the next call to yield the first proper item in our updated `rest`
                        self.next()
                    }
                }
            },
            Some(item) => {
                // If at least one item is yielded by `rest`, our current iterator is non-empty and we set `non_empty` to true.
                self.non_empty = true;
                Some(item)
            }
        }
    }
}
