//! Specialized types and combinators used to define the display-form
//! of processed OTF structures, without forcing the caller to decide
//! whether to build string-buffers in-memory or to emit lines to stdout
//! one at a time.

use std::fmt::Write as _;

/// Constructs an inline-text `Token` consisting of the string `str`.
///
/// # Note
///
/// It is the caller's responsibility to ensure that `str` does not contain any newline characters.
pub fn tok(str: impl Into<doodle::Label>) -> Token {
    Token::InlineText(str.into())
}

/// Converts a `&'static str`, `String`, or `Label` into a `TokenStream`.
///
/// Equivalent to `TokenStream::from(tok(str))`.
pub fn toks(str: impl Into<doodle::Label>) -> TokenStream {
    TokenStream::from(Token::InlineText(str.into()))
}

/// Simple token consisting of either an inline-string or a newline.
///
/// Also includes control-tokens to increase or decrease the relative indentation
/// by one stage (i.e. one half-tab).
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
            Token::InlineText(s) => f.write_str(s.as_ref()),
            Token::LineBreak => f.write_char('\n'),
            Token::IncreaseIndent | Token::DecreaseIndent => Ok(()), // No visual representation for indent tokens, since they are handled implicitly by `IndentIter`
        }
    }
}

impl Token {
    /// Creates a new `TokenStream` by prepending `self` to `stream`.
    pub fn then(self, stream: TokenStream) -> TokenStream {
        TokenStream {
            inner: Box::new(std::iter::once(self).chain(stream.inner)),
        }
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Token::InlineText(s.into())
    }
}

impl From<Token> for TokenStream {
    fn from(value: Token) -> Self {
        TokenStream {
            inner: Box::new(std::iter::once(value)),
        }
    }
}

#[must_use]
pub struct TokenStream {
    inner: Box<dyn Iterator<Item = Token> + 'static>,
}

impl TokenStream {
    /// Constructs a new empty `TokenStream`.
    pub fn empty() -> TokenStream {
        TokenStream {
            inner: Box::new(std::iter::empty()),
        }
    }

    /// Creates a new `TokenStream` from a stream of tokens.
    pub fn from_stream(stream: impl Iterator<Item = Token> + 'static) -> Self {
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

    /// Prints the tokens in `self` to stdout, followed by a newline character
    /// if self was not empty.
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
    /// # Note
    ///
    /// Implicitly assumes that no newline characters occur within `InlineText` tokens.
    pub fn group_lines(self) -> TokenStream {
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
    ///
    /// Will include a line-break even if `self` was originally empty.
    pub fn break_line(self) -> TokenStream {
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

    /// Joins a stream-of-streams with a given separator.
    pub fn join_with(streams: Vec<TokenStream>, sep: Token) -> TokenStream {
        TokenStream {
            inner: Box::new(IntersperseIter::new(
                Box::new(streams.into_iter().map(|s| s.inner)),
                sep,
            )),
        }
    }
}

pub struct IndentIter {
    level: Indent,
    stream: Box<dyn Iterator<Item = Token> + 'static>,
    at_line_start: bool,
    // Temporary cache for the next token to be emitted by `stream`, to avoid indenting after the final linebreak in `stream`
    next_token: Option<Token>,
}

impl IndentIter {
    fn new(level: Indent, stream: Box<dyn Iterator<Item = Token> + 'static>) -> Self {
        Self {
            level,
            stream,
            at_line_start: true,
            next_token: None,
        }
    }
}

impl Iterator for IndentIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token.take().or_else(|| {
            let next = self.stream.next();
            match next {
                Some(Token::LineBreak) => {
                    self.at_line_start = true;
                    next
                }
                Some(Token::IncreaseIndent) => {
                    self.level.0 += 1;
                    self.next()
                }
                Some(Token::DecreaseIndent) => {
                    self.level.0 = self.level.0.saturating_sub(1);
                    self.next()
                }
                Some(token) => {
                    if self.at_line_start {
                        self.next_token = Some(token);
                        self.at_line_start = false;
                        Some(tok(self.level.render()))
                    } else {
                        Some(token)
                    }
                }
                None => None,
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Newtype representing indentation level, measured in 4-space half-tabs.
pub struct Indent(u8);

impl Indent {
    /// Half-Tab for partial indentation
    pub const HT: &str = "    ";

    /// Render the appropriate indentation level as a string.
    pub fn render(self) -> String {
        let n = self.0;

        let indent_str = "\t".repeat((n / 2) as usize);
        if n % 2 > 0 {
            indent_str + Self::HT
        } else {
            indent_str
        }
    }
}

/// Helper iterator for joining a stream-of-streams with a separator ([`TokenStream::join_with`]).
pub struct IntersperseIter<T: Clone> {
    items: Box<dyn Iterator<Item = Box<dyn Iterator<Item = T> + 'static>> + 'static>,
    rest: Box<dyn Iterator<Item = T> + 'static>,
    sep: T,
    /// Flag used to determine whether the most recent `rest` iterator yielded from `items` was non-empty.
    non_empty: bool,
}

impl<T: 'static + Clone> IntersperseIter<T> {
    pub fn new(
        items: Box<dyn Iterator<Item = Box<dyn Iterator<Item = T> + 'static>> + 'static>,
        sep: T,
    ) -> Self {
        // Pre-filter: skip any segment that yields no items, reconstituting non-empty ones
        // by prepending their consumed first element. This guarantees every segment seen
        // by `next()` is non-empty, so separators are never emitted speculatively.
        let filtered = items.filter_map(|mut iter| {
            let first = iter.next()?;
            let reconstituted: Box<dyn Iterator<Item = T> + 'static> =
                Box::new(std::iter::once(first).chain(iter));
            Some(reconstituted)
        });
        Self {
            items: Box::new(filtered),
            rest: Box::new(std::iter::empty()),
            sep,
            non_empty: false,
        }
    }
}

impl<T: Clone> std::iter::Iterator for IntersperseIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rest.next() {
            None => match self.items.next() {
                None => None,
                Some(iter) => {
                    self.rest = iter;
                    if self.non_empty {
                        self.non_empty = false;
                        Some(self.sep.clone())
                    } else {
                        self.next()
                    }
                }
            },
            Some(item) => {
                self.non_empty = true;
                Some(item)
            }
        }
    }
}
