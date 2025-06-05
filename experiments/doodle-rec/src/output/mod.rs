use std::rc::Rc;

use crate::{Arith, Expr, Format, FormatModule, IntRel, RecurseCtx, Unary, decoder::Value};
pub(crate) use doodle::output::Fragment;
use doodle::{
    output::{FragmentBuilder, Symbol},
    precedence::{Precedence, cond_paren},
};

pub struct Flags {
    omit_implied_values: bool,
    pretty_quoted_strings: bool,
    show_redundant_formats: bool,
}

enum Column {
    Branch,
    Space,
}

pub struct PPrinter<'module> {
    gutter: Vec<Column>,
    preview_len: Option<usize>,
    flags: Flags,
    module: &'module FormatModule,
}

impl<'module> PPrinter<'module> {
    pub fn new(module: &'module FormatModule) -> Self {
        let flags = Flags {
            omit_implied_values: true,
            pretty_quoted_strings: true,
            show_redundant_formats: false,
        };
        Self {
            gutter: Vec::new(),
            preview_len: Some(10),
            flags,
            module,
        }
    }

    pub fn compile_decoded_value(
        &mut self,
        value: &Value,
        fmt: &Format,
        ctx: RecurseCtx<'module>,
    ) -> Fragment {
        match fmt {
            Format::ItemVar(level) => {
                let ctx = self.module.get_ctx(*level);
                self.compile_decoded_value(value, self.module.get_format(*level), ctx)
            }
            Format::RecVar(level) => {
                let ctx = ctx.enter(*level);
                self.compile_decoded_value(value, ctx.get_format().unwrap(), ctx)
            }
            Format::FailWith(..) => unreachable!("uninhabited format (value={value})"),
            Format::EndOfInput => self.compile_value(value),
            Format::Byte(_) => self.compile_value(value),
            Format::Compute(_) => self.compile_value(value),
            Format::Variant(label, format) => match value {
                Value::Variant(label2, value) => {
                    if label == label2 {
                        self.compile_variant(label, value, Some((format, ctx)))
                    } else {
                        panic!("expected variant {label}, found {label2}")
                    }
                }
                _ => panic!("expected variant, found {value}"),
            },
            Format::Union(formats) => match value {
                Value::Branch(ix, value) => {
                    let format = &formats[*ix];
                    self.compile_decoded_value(value, format, ctx)
                }
                _ => panic!("expected branch, found {value:?}"),
            },
            Format::Repeat(format) => match value {
                Value::Seq(values) => self.compile_seq(values, Some((format, ctx))),
                _ => panic!("expected sequence, found {value}"),
            },
            Format::Seq(formats) => match value {
                Value::Seq(values) => {
                    self.compile_seq_formats(values, Some((formats.as_slice(), ctx)))
                }
                _ => panic!("expected sequence, found {value}"),
            },
            Format::Tuple(formats) => match value {
                Value::Tuple(values) => match &values[..] {
                    [Value::U8(b'"'), Value::Seq(str_contents), Value::U8(b'"')]
                        if self.flags.pretty_quoted_strings
                            && self.is_quoted_string(str_contents) =>
                    {
                        self.compile_quoted_string(str_contents)
                    }
                    _ => self.compile_tuple(values, Some((formats, ctx))),
                },
                _ => panic!("expected tuple, found {value}"),
            },
            Format::Maybe(_expr, format) => match value {
                Value::Option(None) => Fragment::string("none"),
                Value::Option(Some(value)) => {
                    self.compile_variant("some", value, Some((format, ctx)))
                }
                _ => panic!("expected option, found {value}"),
            },
        }
    }

    fn compile_variant(
        &mut self,
        label: &str,
        value: &Value,
        format: Option<(&Format, RecurseCtx<'module>)>,
    ) -> Fragment {
        if self.flags.omit_implied_values
            && format.is_some_and(|(format, ctx)| self.is_implied_value_format(format, ctx))
        {
            Fragment::string(label.to_string())
        } else if self.is_atomic_value(value, format) {
            let mut frag = Fragment::new();
            frag.append(Fragment::String(format!("{{ {label} := ").into()));
            if let Some((format, ctx)) = format {
                frag.append(self.compile_decoded_value(value, format, ctx));
            } else {
                frag.append(self.compile_value(value));
            }
            frag.append(Fragment::String(" }".into()));
            frag.enclose();
            frag
        } else {
            self.compile_field_value_last(label, value, format, true)
        }
    }

    fn compile_quoted_string(&mut self, str_contents: &[Value]) -> Fragment {
        Fragment::seq(
            str_contents.iter().map(|v| match v {
                Value::U8(b) => Fragment::Char(*b as char),
                Value::Char(c) => Fragment::Char(*c),
                _ => panic!("expected U8 or Char value, found {v}"),
            }),
            None,
        )
        .delimit(Fragment::Char('\"'), Fragment::Char('\"'))
    }

    fn compile_tuple(
        &mut self,
        vals: &[Value],
        formats: Option<(&[Format], RecurseCtx<'module>)>,
    ) -> Fragment {
        if vals.is_empty() {
            Fragment::String("()".into())
        } else {
            let mut frag = Fragment::new();
            let last_index = vals.len() - 1;
            for index in 0..last_index {
                frag.append(self.compile_field_value_continue(
                    index,
                    &vals[index],
                    formats.map(|(fs, ctx)| (&fs[index], ctx)),
                    true,
                ));
            }
            frag.append(self.compile_field_value_last(
                last_index,
                &vals[last_index],
                formats.map(|(fs, ctx)| (&fs[last_index], ctx)),
                true,
            ));
            frag
        }
    }

    fn compile_seq_formats(
        &mut self,
        vals: &[Value],
        formats: Option<(&[Format], RecurseCtx<'module>)>,
    ) -> Fragment {
        if vals.is_empty() {
            Fragment::string("[]")
        } else {
            let mut frag = Fragment::new();
            let last_index = vals.len() - 1;
            let (upper_bound, any_skipped) = match self.preview_len {
                Some(preview_len) if vals.len() > preview_len => {
                    (preview_len, preview_len != last_index)
                }
                Some(_) | None => (last_index, false),
            };
            for index in 0..upper_bound {
                let val = &vals[index];
                let format = formats.map(|(fs, ctx)| (&fs[index], ctx));
                frag.append(self.compile_field_value_continue(index, val, format, false));
            }
            if any_skipped {
                frag.append(self.compile_field_skipped());
            }
            let format = formats.map(|(fs, ctx)| (&fs[last_index], ctx));
            frag.append(self.compile_field_value_last(
                last_index,
                &vals[last_index],
                format,
                false,
            ));
            frag
        }
    }

    fn compile_seq(
        &mut self,
        vals: &[Value],
        format: Option<(&Format, RecurseCtx<'module>)>,
    ) -> Fragment {
        if vals.is_empty() {
            Fragment::String("[]".into())
        } else {
            let mut frag = Fragment::new();
            let last_index = vals.len() - 1;
            let (upper_bound, any_skipped) = match self.preview_len {
                Some(preview_len) if vals.len() > preview_len => {
                    (preview_len, preview_len != last_index)
                }
                Some(_) | None => (last_index, false),
            };
            for index in 0..upper_bound {
                let val = &vals[index];
                frag.append(self.compile_field_value_continue(index, val, format, false));
            }
            if any_skipped {
                frag.append(self.compile_field_skipped());
            }
            frag.append(self.compile_field_value_last(
                last_index,
                &vals[last_index],
                format,
                false,
            ));
            frag
        }
    }

    fn compile_value(&mut self, value: &Value) -> Fragment {
        match value {
            Value::Bool(true) => Fragment::string("true"),
            Value::Bool(false) => Fragment::string("false"),
            Value::U8(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::U16(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::U32(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::U64(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::Char(c) => Fragment::DebugAtom(Rc::new(*c)),
            Value::Tuple(vals) => self.compile_tuple(vals, None),
            Value::Seq(vals) => self.compile_seq(vals, None),
            Value::Variant(label, value) => self.compile_variant(label, value, None),
            Value::Branch(_n, value) => self.compile_value(value),
            Value::Option(None) => Fragment::string("none"),
            Value::Option(Some(value)) => self.compile_variant("some", value, None),
        }
    }

    fn is_quoted_string(&self, str_contents: &[Value]) -> bool {
        str_contents
            .iter()
            .all(|v| matches!(v, Value::U8(0..=127) | Value::Char(_)))
    }

    fn is_implied_value_format(&self, format: &Format, ctx: RecurseCtx<'module>) -> bool {
        match format {
            Format::ItemVar(level) => {
                let format = self.module.get_format(*level);
                let ctx = self.module.get_ctx(*level);
                self.is_implied_value_format(format, ctx)
            }
            Format::RecVar(level) => {
                let ctx = ctx.enter(*level);
                self.is_implied_value_format(ctx.get_format().unwrap(), ctx)
            }
            Format::EndOfInput => true,
            Format::Byte(bs) => bs.len() == 1,
            Format::Repeat(format) => self.is_implied_value_format(format, ctx),
            Format::Seq(formats) | Format::Tuple(formats) => {
                formats.iter().all(|f| self.is_implied_value_format(f, ctx))
            }
            _ => false,
        }
    }

    fn is_atomic_value(
        &self,
        value: &Value,
        format: Option<(&Format, RecurseCtx<'module>)>,
    ) -> bool {
        match value {
            Value::Char(_) => true,
            Value::Bool(_) => true,
            Value::U8(_) | Value::U16(_) | Value::U32(_) | Value::U64(_) => true,
            Value::Tuple(values) => values.is_empty(),
            Value::Seq(values) => values.is_empty(),
            Value::Variant(label, value) => match format {
                Some((Format::Variant(label2, format), ctx)) => {
                    assert_eq!(label, label2);
                    self.is_atomic_value(value.as_ref(), Some((format, ctx)))
                }
                _ => self.is_atomic_value(value.as_ref(), None),
            },
            Value::Branch(n, value) => {
                match format.map(|(f, ctx)| (self.unwrap_itemvars(f, ctx), ctx)) {
                    Some((Format::Union(branches), ctx)) => {
                        let format = &branches[*n];
                        self.is_atomic_value(value.as_ref(), Some((format, ctx)))
                    }
                    None => self.is_atomic_value(value.as_ref(), None),
                    f => panic!("expected format suitable for branch: {f:?}"),
                }
            }
            Value::Option(None) => true,
            Value::Option(Some(value)) => self.is_atomic_value(value, None),
        }
    }

    fn unwrap_itemvars<'a>(&'a self, format: &'a Format, ctx: RecurseCtx<'a>) -> &'a Format {
        match format {
            &Format::ItemVar(level, ..) => self.unwrap_itemvars(self.module.get_format(level), ctx),
            &Format::RecVar(level, ..) => {
                let ctx = ctx.enter(level);
                self.unwrap_itemvars(ctx.get_format().unwrap(), ctx)
            }
            _ => format,
        }
    }

    fn compile_field_value_continue(
        &mut self,
        label: impl std::fmt::Display,
        value: &Value,
        format: Option<(&Format, RecurseCtx<'module>)>,
        format_needed: bool,
    ) -> Fragment {
        let mut frags = FragmentBuilder::new();
        frags.push(self.compile_gutter());
        frags.push(Fragment::cat(
            Fragment::Symbol(Symbol::Junction),
            Fragment::String(format!("{label}").into()),
        ));

        self.gutter.push(Column::Branch);
        let frag_value = self.compile_field_value(value, format);
        self.gutter.pop();

        if let Some((format, ctx)) = format {
            if format_needed
                || self.flags.show_redundant_formats
                || (self.is_indirect_format(format) && !frag_value.is_single_line(true))
            {
                frags.push(Fragment::String(" <- ".into()));
                frags.push(self.compile_format(format, ctx, Precedence::FORMAT_COMPOUND));
            }
        }
        frags.push(frag_value);
        frags.finalize().group()
    }

    fn compile_field_value_last(
        &mut self,
        label: impl std::fmt::Display,
        value: &Value,
        format: Option<(&Format, RecurseCtx<'module>)>,
        format_needed: bool,
    ) -> Fragment {
        let mut frags = FragmentBuilder::new();
        frags.push(self.compile_gutter());
        frags.push(Fragment::cat(
            Fragment::Symbol(Symbol::Elbow),
            Fragment::String(format!("{label}").into()),
        ));

        self.gutter.push(Column::Space);
        let frag_value = self.compile_field_value(value, format);
        self.gutter.pop();

        if let Some((format, ctx)) = format {
            if format_needed
                || self.flags.show_redundant_formats
                || (self.is_indirect_format(format) && !frag_value.is_single_line(true))
            {
                frags.push(Fragment::String(" <- ".into()));
                frags.push(self.compile_format(format, ctx, Default::default()));
            }
        }
        frags.push(frag_value);
        frags.finalize().group()
    }

    fn compile_field_skipped(&self) -> Fragment {
        self.compile_gutter()
            .cat(Fragment::String("~\n".into()))
            .group()
    }

    fn compile_gutter(&self) -> Fragment {
        let mut frags = FragmentBuilder::new();
        for column in &self.gutter {
            match column {
                Column::Branch => frags.push(Fragment::Symbol(Symbol::Pipe)),
                Column::Space => frags.push(Fragment::Symbol(Symbol::Vacuum)),
            }
        }
        frags.finalize()
    }

    fn compile_field_value(
        &mut self,
        value: &Value,
        format: Option<(&Format, RecurseCtx<'module>)>,
    ) -> Fragment {
        match format {
            Some((format, ctx)) => {
                if self.flags.omit_implied_values && self.is_implied_value_format(format, ctx) {
                    Fragment::Char('\n')
                } else {
                    Fragment::join_with_wsp(
                        Fragment::String(" :=".into()),
                        self.compile_decoded_value(value, format, ctx),
                    )
                    .group()
                }
            }
            None => {
                Fragment::join_with_wsp(Fragment::String(" :=".into()), self.compile_value(value))
                    .group()
            }
        }
    }

    fn is_indirect_format(&self, format: &Format) -> bool {
        matches!(format, Format::ItemVar(..) | Format::RecVar(..))
    }

    fn compile_format(
        &mut self,
        format: &Format,
        ctx: RecurseCtx<'module>,
        prec: Precedence,
    ) -> Fragment {
        match format {
            Format::Variant(label, f) => cond_paren(
                self.compile_nested_format(
                    "variant",
                    Some(&[Fragment::String(label.clone())]),
                    f,
                    ctx,
                    prec,
                ),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Union(_) => cond_paren(
                Fragment::String("_ |...| _".into()),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Maybe(expr, f) => {
                let frag_expr = self.compile_expr(expr, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format("maybe", Some(&[frag_expr]), f, ctx, prec),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::Repeat(format) => cond_paren(
                self.compile_nested_format("repeat", None, format, ctx, prec),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Compute(expr) => cond_paren(
                Fragment::cat(
                    Fragment::String("compute ".into()),
                    self.compile_expr(expr, Default::default()),
                ),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::ItemVar(var) => Fragment::String(self.module.get_name(*var).to_string().into()),
            Format::RecVar(var) => Fragment::String(
                self.module
                    .get_name(ctx.convert_rec_var(*var).unwrap())
                    .to_string()
                    .into(),
            ),
            Format::Byte(bs) => match bs.len() {
                0 => unreachable!("matches against the empty byteset are unsatisfiable"),
                1..=127 => {
                    let mut frags = FragmentBuilder::new();
                    frags.push(Fragment::String("[=".into()));
                    for b in bs.iter() {
                        frags.push(Fragment::String(format!(" {b}").into()));
                    }
                    frags.push(Fragment::Char(']'));
                    frags.finalize()
                }
                128..=255 => {
                    let mut frags = FragmentBuilder::new();
                    frags.push(Fragment::String("[!=".into()));
                    for b in (!bs).iter() {
                        frags.push(Fragment::String(format!(" {b}").into()));
                    }
                    frags.push(Fragment::Char(']'));
                    frags.finalize()
                }
                256 => Fragment::String("U8".into()),
                _n => unreachable!("impossible ByteSet size {_n}"),
            },
            Format::FailWith(lab) => Fragment::string(format!("fail({})", lab)),
            Format::EndOfInput => Fragment::string("end-of-input"),
            Format::Tuple(formats) if formats.is_empty() => Fragment::String("()".into()),
            Format::Tuple(_) => Fragment::String("(...)".into()),

            Format::Seq(formats) if formats.is_empty() => Fragment::String("[]".into()),
            Format::Seq(_) => Fragment::String("[ ... ]".into()),
        }
    }

    fn compile_nested_format(
        &mut self,
        label: &'static str,
        args: Option<&[Fragment]>,
        inner: &Format,
        ctx: RecurseCtx<'module>,
        prec: Precedence,
    ) -> Fragment {
        let mut frags = FragmentBuilder::new();
        frags.push(Fragment::String(label.into()));
        if let Some(args) = args {
            for arg in args.iter() {
                frags.push(arg.clone());
            }
        }
        frags.push(self.compile_format(inner, ctx, prec.bump_format()));
        frags.finalize_with_sep(Fragment::Char(' '))
    }

    fn compile_expr(&self, expr: &Expr, prec: Precedence) -> Fragment {
        match expr {
            Expr::IntRel(IntRel::Eq, lhs, rhs) => cond_paren(
                self.binary_op(" == ", lhs, rhs, Precedence::EQUALITY, Precedence::EQUALITY),
                prec,
                Precedence::COMPARE,
            ),
            Expr::IntRel(IntRel::Neq, lhs, rhs) => cond_paren(
                self.binary_op(" != ", lhs, rhs, Precedence::EQUALITY, Precedence::EQUALITY),
                prec,
                Precedence::COMPARE,
            ),
            Expr::IntRel(IntRel::Lt, lhs, rhs) => cond_paren(
                self.binary_op(" < ", lhs, rhs, Precedence::COMPARE, Precedence::COMPARE),
                prec,
                Precedence::COMPARE,
            ),
            Expr::IntRel(IntRel::Gt, lhs, rhs) => cond_paren(
                self.binary_op(" > ", lhs, rhs, Precedence::COMPARE, Precedence::COMPARE),
                prec,
                Precedence::COMPARE,
            ),
            Expr::IntRel(IntRel::Lte, lhs, rhs) => cond_paren(
                self.binary_op(" <= ", lhs, rhs, Precedence::COMPARE, Precedence::COMPARE),
                prec,
                Precedence::COMPARE,
            ),
            Expr::IntRel(IntRel::Gte, lhs, rhs) => cond_paren(
                self.binary_op(" >= ", lhs, rhs, Precedence::COMPARE, Precedence::COMPARE),
                prec,
                Precedence::COMPARE,
            ),
            Expr::Arith(Arith::Add, lhs, rhs) => cond_paren(
                self.binary_op(" + ", lhs, rhs, Precedence::ADD_SUB, Precedence::ADD_SUB),
                prec,
                Precedence::ADD_SUB,
            ),
            Expr::Arith(Arith::Sub, lhs, rhs) => cond_paren(
                self.binary_op(" - ", lhs, rhs, Precedence::ADD_SUB, Precedence::ADD_SUB),
                prec,
                Precedence::ADD_SUB,
            ),
            Expr::Arith(Arith::Mul, lhs, rhs) => cond_paren(
                self.binary_op(" * ", lhs, rhs, Precedence::MUL, Precedence::MUL),
                prec,
                Precedence::MUL,
            ),
            Expr::Arith(Arith::Div, lhs, rhs) => cond_paren(
                self.binary_op(" / ", lhs, rhs, Precedence::DIV_REM, Precedence::DIV_REM),
                prec,
                Precedence::DIV_REM,
            ),
            Expr::Arith(Arith::Rem, lhs, rhs) => cond_paren(
                self.binary_op(" % ", lhs, rhs, Precedence::DIV_REM, Precedence::DIV_REM),
                prec,
                Precedence::DIV_REM,
            ),
            Expr::Arith(Arith::BitAnd, lhs, rhs) => cond_paren(
                self.binary_op(" & ", lhs, rhs, Precedence::BITAND, Precedence::BITAND),
                prec,
                Precedence::BITAND,
            ),
            Expr::Arith(Arith::BitOr, lhs, rhs) => cond_paren(
                self.binary_op(" | ", lhs, rhs, Precedence::BITOR, Precedence::BITOR),
                prec,
                Precedence::BITOR,
            ),
            Expr::Arith(Arith::Shl, lhs, rhs) => cond_paren(
                self.binary_op(
                    " << ",
                    lhs,
                    rhs,
                    Precedence::BIT_SHIFT,
                    Precedence::BIT_SHIFT,
                ),
                prec,
                Precedence::BIT_SHIFT,
            ),
            Expr::Arith(Arith::Shr, lhs, rhs) => cond_paren(
                self.binary_op(
                    " >> ",
                    lhs,
                    rhs,
                    Precedence::BIT_SHIFT,
                    Precedence::BIT_SHIFT,
                ),
                prec,
                Precedence::BIT_SHIFT,
            ),
            Expr::Unary(Unary::BoolNot, expr) => cond_paren(
                self.prefix_op("!", None, expr),
                prec,
                Precedence::LOGICAL_NEGATE,
            ),
            Expr::AsU8(expr) => cond_paren(
                self.prefix_op("as-u8", None, expr),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::AsU16(expr) => cond_paren(
                self.prefix_op("as-u16", None, expr),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::AsU32(expr) => cond_paren(
                self.prefix_op("as-u32", None, expr),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::AsU64(expr) => cond_paren(
                self.prefix_op("as-u64", None, expr),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::AsChar(expr) => cond_paren(
                self.prefix_op("as-char", None, expr),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::LiftOption(Some(expr)) => cond_paren(
                self.prefix_op("some", None, expr),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::LiftOption(None) => Fragment::string("none"),
            Expr::Bool(b) => Fragment::DisplayAtom(Rc::new(*b)),
            Expr::U8(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::U16(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::U32(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::U64(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::Tuple(..) => Fragment::String("(...)".into()),
            Expr::Variant(label, expr) => Fragment::String("{ ".into())
                .cat(Fragment::String(label.clone()))
                .cat(Fragment::String(" := ".into()))
                .cat(self.compile_expr(expr, Default::default()))
                .cat(Fragment::String(" }".into()))
                .group(),
            Expr::Seq(..) => Fragment::String("[..]".into()),
        }
    }

    fn prefix_op(&self, op: &'static str, args: Option<&[&Expr]>, operand: &Expr) -> Fragment {
        let mut frags = FragmentBuilder::new();

        frags.push(Fragment::String(op.into()));
        match args {
            None => (),
            Some(args) => {
                let frag = frags.active_mut();
                frag.append(Fragment::Char('('));
                frag.append(Fragment::seq(
                    args.iter()
                        .map(|arg| self.compile_expr(arg, Precedence::default()))
                        .collect::<Vec<_>>(),
                    Some(Fragment::String(", ".into())),
                ));
                frag.append(Fragment::Char(')'));
            }
        }
        frags.push(self.compile_expr(operand, Precedence::ATOM));
        frags.finalize_with_sep(Fragment::Char(' '))
    }

    fn binary_op(
        &self,
        op: &'static str,
        lhs: &Expr,
        rhs: &Expr,
        lhs_prec: Precedence,
        rhs_prec: Precedence,
    ) -> Fragment {
        self.compile_expr(lhs, lhs_prec)
            .cat(Fragment::String(op.into()))
            .cat(self.compile_expr(rhs, rhs_prec))
            .group()
    }
}
