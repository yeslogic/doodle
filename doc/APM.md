# Alternate Processing Model

Relative to [`allsorts`](https://www.github.com/yeslogic/allsorts).

## Interface Changes

- Moving from `doodle::Parser` to [`allsorts::binary::read::ReadCtxt`](https://github.com/yeslogic/allsorts/blob/master/src/binary/read.rs#L56-L60)
- Moving from `doodle::ParseError` to [`allsorts::error::ParseError`](https://github.com/yeslogic/allsorts/blob/master/src/error.rs#L55-L67)

## Type Level Abstractions

- Spawning a [`ReadScope`](https://github.com/yeslogic/allsorts/blob/master/src/binary/read.rs#L29-L33) from the current Ctxt and binding it to an identifier for future use
- Reading a `ReadScope<'a>` into `&'a [u8]`
- Co-indexed lifetimes for borrowed or borrowing types
- Parse into `ReadArray`, along with machine-int marker types for `ReadArray` tagging

## Operational Abstractions

- Parse a 'version'-kinded value and validate it against `ParseError::BadVersion`
