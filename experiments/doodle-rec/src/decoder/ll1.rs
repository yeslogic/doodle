use std::rc::Rc;

use doodle::prelude::ByteSet;
use doodle::read::ReadCtxt;

use super::Value;
use crate::{
    Format, FormatModule, RecurseCtx,
    determinations::{Choice, Entry, InterpError, PartialFormat, PathTrace, Traversal},
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expr, Label};

    fn seq_of_bytes(value: &Value) -> Vec<u8> {
        match value {
            Value::Seq(vals) => vals
                .iter()
                .map(|v| match v {
                    Value::U8(b) => *b,
                    other => panic!("expected U8, found {other:?}"),
                })
                .collect(),
            other => panic!("expected Seq, found {other:?}"),
        }
    }

    #[test]
    fn repeat_count_and_between() {
        let f = Format::Tuple(vec![
            Format::RepeatCount(
                Box::new(Expr::U8(2)),
                Box::new(Format::Byte(ByteSet::from(b'a'..=b'z'))),
            ),
            Format::RepeatBetween(
                Box::new(Expr::U8(1)),
                Box::new(Expr::U8(3)),
                Box::new(Format::Byte(ByteSet::from([b'x']))),
            ),
        ]);
        let mut module = FormatModule::new();
        let main = module.declare_format(Label::Borrowed("main"), f);
        let interp = LL1Interpreter::new(&module);
        let (value, remaining) = interp
            .parse_level(main.get_level(), ReadCtxt::new(b"abxx"))
            .expect("parses");
        let Value::Tuple(fields) = &value else {
            panic!("expected Tuple, found {value:?}")
        };
        assert_eq!(seq_of_bytes(&fields[0]), vec![b'a', b'b']);
        assert_eq!(seq_of_bytes(&fields[1]), vec![b'x', b'x']);
        assert_eq!(remaining.offset, 4);
    }

    #[test]
    fn peek_and_peek_not() {
        let f = Format::Tuple(vec![
            Format::Peek(Box::new(Format::Byte(ByteSet::from([b'a'])))),
            Format::PeekNot(Box::new(Format::Byte(ByteSet::from([b'b'])))),
            Format::Byte(ByteSet::from([b'a'])),
        ]);
        let mut module = FormatModule::new();
        let main = module.declare_format(Label::Borrowed("main"), f);
        let interp = LL1Interpreter::new(&module);
        let (value, remaining) = interp
            .parse_level(main.get_level(), ReadCtxt::new(b"a"))
            .expect("parses");
        let Value::Tuple(fields) = &value else {
            panic!("expected Tuple, found {value:?}")
        };
        assert!(matches!(fields[0], Value::U8(b'a')));
        assert!(matches!(&fields[1], Value::Tuple(v) if v.is_empty()));
        assert!(matches!(fields[2], Value::U8(b'a')));
        assert_eq!(remaining.offset, 1);
    }

    #[test]
    fn peek_not_matched_fails() {
        let f = Format::PeekNot(Box::new(Format::Byte(ByteSet::from([b'a']))));
        let mut module = FormatModule::new();
        let main = module.declare_format(Label::Borrowed("main"), f);
        let interp = LL1Interpreter::new(&module);
        assert!(
            interp
                .parse_level(main.get_level(), ReadCtxt::new(b"a"))
                .is_err()
        );
    }
}

pub struct LL1Interpreter<'a> {
    module: &'a FormatModule,
}

impl<'a> LL1Interpreter<'a> {
    pub fn new(module: &'a FormatModule) -> Self {
        Self { module }
    }

    pub fn parse_level<'x>(
        &self,
        level: usize,
        input: ReadCtxt<'x>,
    ) -> Result<(Value, ReadCtxt<'x>), InterpError>
    where 'a: 'x
    {
        let ctx = self.module.get_ctx(level);
        let format = self.module.get_format(level);
        let mut trace = PathTrace::new();
        let mut visited = Traversal::new(level);
        self.parse_format(
            format,
            Rc::new(PartialFormat::Empty),
            ctx,
            input,
            &mut trace,
            &mut visited,
        )
    }

    fn parse_format<'x>(
        &self,
        format: &'a Format,
        remnant: Rc<PartialFormat<'a>>,
        ctx: RecurseCtx<'a>,
        input: ReadCtxt<'x>,
        trace: &mut PathTrace,
        visited: &mut Traversal,
    ) -> Result<(Value, ReadCtxt<'x>), InterpError>
    where 'a: 'x
    {
        match format {
            Format::ItemVar(level) => {
                let f = self.module.get_format(*level);
                let ctx = self.module.get_ctx(*level);
                self.parse_format(f, remnant, ctx, input, trace, visited)
            }
            Format::RecVar(rec_ix) => {
                let level = ctx
                    .convert_rec_var(*rec_ix)
                    .unwrap_or_else(|| panic!("recursion variable not found in {ctx:?}: {rec_ix}"));
                if visited.insert(level) == Entry::Novel {
                    let new_ctx = ctx.enter(*rec_ix);
                    let format = new_ctx.get_format().unwrap();
                    let ret = self.parse_format(format, remnant, new_ctx, input, trace, visited)?;
                    let _ = visited.escape();
                    Ok(ret)
                } else {
                    unreachable!("left recursion")
                }
            }
            Format::FailWith(msg) => {
                return Err(InterpError::Fail {
                    message: msg.clone(),
                });
            }
            Format::EndOfInput => {
                let b = input.read_byte();
                if b.is_none() {
                    Ok((Value::Tuple(vec![]), input))
                } else {
                    Err(InterpError::ExpectsEnd)
                }
            }
            Format::Byte(bs) => {
                let (b, input) = input
                    .read_byte()
                    .ok_or(InterpError::BadEpsilon { expects: *bs })?;
                if bs.contains(b) {
                    visited.reset();
                    Ok((Value::U8(b), input))
                } else {
                    Err(InterpError::DeadEnd {
                        start: visited
                            .orig_level
                            .expect("LL1Interpreter's Traversal is always level-anchored via Traversal::new"),
                        trace: trace.clone(),
                        byte: b,
                        expects: *bs,
                    })
                }
            }
            Format::Compute(expr) => {
                let val = expr.eval();
                Ok((val, input))
            }
            Format::Variant(lab, format) => {
                let (val, input) =
                    self.parse_format(format, remnant, ctx, input, trace, visited)?;
                Ok((Value::Variant(lab.clone(), Box::new(val)), input))
            }
            Format::Union(formats) => {
                let mut branches: Vec<ByteSet> = Vec::with_capacity(formats.len());
                let mut accept = None;
                for (ix, branch) in formats.iter().enumerate() {
                    let mut _visited = visited.fork();
                    let dets = branch
                        .solve_determinations(self.module, &mut _visited, ctx)
                        .unwrap();
                    if dets.is_nullable {
                        if let Some(ix0) = accept.replace(ix) {
                            unreachable!("multiple nullable branches: {ix0}, {ix}");
                        };
                    }
                    branches.push(dets.first_set);
                }
                match input.read_byte() {
                    None => match accept {
                        None => {
                            let expects = branches
                                .iter()
                                .fold(ByteSet::empty(), |acc, bs| acc.union(bs));
                            Err(InterpError::BadEpsilon { expects })
                        }
                        Some(ix) => {
                            let (val, input) = self.parse_format(
                                &formats[ix],
                                remnant,
                                ctx,
                                input,
                                trace,
                                visited,
                            )?;
                            Ok((val, input))
                        }
                    },
                    Some((byte, _input)) => {
                        for (ix, bs) in branches.iter().enumerate() {
                            if bs.contains(byte) {
                                let (val, input) = self.parse_format(
                                    &formats[ix],
                                    remnant,
                                    ctx,
                                    input,
                                    trace,
                                    visited,
                                )?;
                                return Ok((Value::Branch(ix, Box::new(val)), input));
                            }
                        }
                        match accept {
                            None => {
                                let expects = branches
                                    .iter()
                                    .fold(ByteSet::empty(), |acc, bs| acc.union(bs));
                                Err(InterpError::BadEpsilon { expects })
                            }
                            Some(ix) => {
                                let (val, input) = self.parse_format(
                                    &formats[ix],
                                    remnant,
                                    ctx,
                                    input,
                                    trace,
                                    visited,
                                )?;
                                Ok((Value::Branch(ix, Box::new(val)), input))
                            }
                        }
                    }
                }
            }
            Format::Repeat(format0) => {
                let mut values = Vec::new();
                let mut input = input;
                let dets = format0
                    .solve_determinations(self.module, visited, ctx)
                    .unwrap();
                if dets.is_nullable {
                    unreachable!("bad repeat of nullable format: {format:?}");
                }
                let dets_next = {
                    let mut visited = visited.fork();
                    remnant
                        .clone()
                        .solve_determinations(self.module, &mut visited, ctx)
                        .unwrap()
                };
                loop {
                    match input.read_byte() {
                        None => {
                            if dets_next.is_nullable {
                                break;
                            } else {
                                return Err(InterpError::BadEpsilon {
                                    expects: dets_next.first_set.union(&dets.first_set),
                                });
                            }
                        }
                        Some((byte, _)) => {
                            if dets.first_set.contains(byte) {
                                trace.push(Choice::RepeatYes);
                                let remnant0 =
                                    Rc::new(PartialFormat::Repeat(format0, remnant.clone()));
                                let (val, new_input) = self
                                    .parse_format(format0, remnant0, ctx, input, trace, visited)?;
                                values.push(val);
                                input = new_input;
                                continue;
                            } else {
                                trace.push(Choice::RepeatNo);
                                break;
                            }
                        }
                    }
                }
                return Ok((Value::Seq(values), input));
            }
            Format::Seq(formats) => {
                let mut values = Vec::with_capacity(formats.len());
                let mut input = input;
                let mut iter = formats.iter();
                while let Some(format) = iter.next() {
                    let remnant0 =
                        Rc::new(PartialFormat::Sequence(iter.as_slice(), remnant.clone()));
                    let (val, new_input) =
                        self.parse_format(format, remnant0, ctx, input, trace, visited)?;
                    values.push(val);
                    input = new_input;
                }
                return Ok((Value::Seq(values), input));
            }
            Format::Tuple(formats) => {
                let mut values = Vec::with_capacity(formats.len());
                let mut input = input;
                let mut iter = formats.iter();
                while let Some(format) = iter.next() {
                    let remnant0 =
                        Rc::new(PartialFormat::Sequence(iter.as_slice(), remnant.clone()));
                    let (val, new_input) =
                        self.parse_format(format, remnant0, ctx, input, trace, visited)?;
                    values.push(val);
                    input = new_input;
                }
                return Ok((Value::Tuple(values), input));
            }
            Format::Maybe(expr, format) => {
                let present = expr.eval().unwrap_bool();
                if present {
                    let (val, input) =
                        self.parse_format(format, remnant, ctx, input, trace, visited)?;
                    Ok((Value::Option(Some(Box::new(val))), input))
                } else {
                    Ok((Value::Option(None), input))
                }
            }
            Format::RepeatCount(count, format0) => {
                let n = count.eval_usize();
                let mut values = Vec::with_capacity(n);
                let mut input = input;
                for _ in 0..n {
                    // Reuses `PartialFormat::Repeat` for every one of the `n` remnants (rather
                    // than a dedicated bounded-count variant) - a conservative approximation
                    // (`solve_determinations` sees "maybe more of format0, then remnant" instead
                    // of the exact remaining count), same simplification as `Decoder`'s use of
                    // `Next::Repeat` for `RepeatCount`'s compiled continuation.
                    let remnant0 = Rc::new(PartialFormat::Repeat(format0, remnant.clone()));
                    let (val, new_input) =
                        self.parse_format(format0, remnant0, ctx, input, trace, visited)?;
                    values.push(val);
                    input = new_input;
                }
                Ok((Value::Seq(values), input))
            }
            Format::RepeatBetween(min, max, format0) => {
                let (min, max) = (min.eval_usize(), max.eval_usize());
                let mut values = Vec::with_capacity(min);
                let mut input = input;
                for _ in 0..min {
                    let remnant0 = Rc::new(PartialFormat::Repeat(format0, remnant.clone()));
                    let (val, new_input) =
                        self.parse_format(format0, remnant0, ctx, input, trace, visited)?;
                    values.push(val);
                    input = new_input;
                }
                if min < max {
                    let dets = format0
                        .solve_determinations(self.module, visited, ctx)
                        .unwrap();
                    if dets.is_nullable {
                        unreachable!("bad repeat of nullable format: {format0:?}");
                    }
                    let dets_next = {
                        let mut visited = visited.fork();
                        remnant
                            .clone()
                            .solve_determinations(self.module, &mut visited, ctx)
                            .unwrap()
                    };
                    for _ in min..max {
                        match input.read_byte() {
                            None => {
                                if dets_next.is_nullable {
                                    break;
                                } else {
                                    return Err(InterpError::BadEpsilon {
                                        expects: dets_next.first_set.union(&dets.first_set),
                                    });
                                }
                            }
                            Some((byte, _)) => {
                                if dets.first_set.contains(byte) {
                                    let remnant0 =
                                        Rc::new(PartialFormat::Repeat(format0, remnant.clone()));
                                    let (val, new_input) = self.parse_format(
                                        format0, remnant0, ctx, input, trace, visited,
                                    )?;
                                    values.push(val);
                                    input = new_input;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                Ok((Value::Seq(values), input))
            }
            // The peek target is parsed against a fresh `PartialFormat::Empty` continuation - it
            // doesn't consume input from the outer position, so the outer `remnant` is irrelevant
            // to whether it matches. `visited` is passed through unforked, matching every other
            // nested `parse_format` call in this function (its own `RecVar` handling already
            // balances insert/escape).
            Format::Peek(format0) => {
                let (val, _discarded) = self.parse_format(
                    format0,
                    Rc::new(PartialFormat::Empty),
                    ctx,
                    input,
                    trace,
                    visited,
                )?;
                Ok((val, input))
            }
            Format::PeekNot(format0) => {
                match self.parse_format(
                    format0,
                    Rc::new(PartialFormat::Empty),
                    ctx,
                    input,
                    trace,
                    visited,
                ) {
                    Ok(_) => Err(InterpError::PeekNotMatched),
                    Err(_) => Ok((Value::Tuple(vec![]), input)),
                }
            }
        }
    }
}
