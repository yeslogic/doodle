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
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::InlineText(s) => write!(f, "{s}"),
            Token::LineBreak => write!(f, "\n"),
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
    pub fn write_to<W: std::io::Write>(self, mut w: W) -> std::io::Result<()> {
        for token in self.inner {
            write!(w, "{token}")?
        }
        Ok(())
    }

    /// Prints the tokens in `self` to stdout.
    pub fn print(self) {
        let oput = std::io::stdout().lock();
        let mut buf = std::io::BufWriter::new(oput);
        self.write_to(&mut buf).unwrap();
    }

    /// Prints the tokens in `self` to stdout, with a single trailing newline.
    pub fn println(self) {
        self.chain(TokenStream::from(Token::LineBreak)).print()
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

    /// Prepends indentation to the first line of the stream, measured in 4-space half-tabs.
    ///
    /// Will prefer using `'\t'` for indentation, and will include a final half-tab iff `stops` is odd.
    pub fn pre_indent(self, stops: u8) -> Self {
        let tabs = stops / 2;
        let hts = stops % 2;

        let mut indent = String::new();
        for _ in 0..tabs {
            indent.push('\t');
        }
        for _ in 0..hts {
            indent.push_str(super::HT);
        }
        tok(indent).then(self)
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
