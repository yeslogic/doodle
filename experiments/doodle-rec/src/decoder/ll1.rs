use std::rc::Rc;

use doodle::prelude::ByteSet;
use doodle::read::ReadCtxt;

use super::Value;
use crate::{
    Format, FormatModule, RecurseCtx,
    determinations::{Choice, InterpError, PartialFormat, PathTrace, Traversal},
};

pub struct LL1Interpreter<'a> {
    module: &'a FormatModule,
}

impl<'a> LL1Interpreter<'a> {
    pub fn new(module: &'a FormatModule) -> Self {
        Self { module }
    }

    pub fn parse_level(
        &self,
        level: usize,
        input: ReadCtxt<'a>,
    ) -> Result<(Value, ReadCtxt), InterpError> {
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

    fn parse_format(
        &self,
        format: &'a Format,
        remnant: Rc<PartialFormat<'a>>,
        ctx: RecurseCtx<'a>,
        input: ReadCtxt<'a>,
        trace: &mut PathTrace,
        visited: &mut Traversal,
    ) -> Result<(Value, ReadCtxt<'a>), InterpError> {
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
                if visited.insert(level) {
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
                        start: visited.orig_level,
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
                    let mut _visited = Traversal::new(visited.orig_level);
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
                    let mut visited = Traversal::new(visited.orig_level);
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
        }
    }
}
