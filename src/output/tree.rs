use std::{borrow::Cow, fmt, io, ops::Deref, rc::Rc};

use crate::{Expr, Format, FormatModule, Scope, Value};

use super::{Fragment, FragmentBuilder, Symbol};

fn atomic_value_to_string(value: &Value) -> String {
    match value {
        Value::U8(n) => n.to_string(),
        _ => panic!("expected atomic value"),
    }
}

pub fn print_decoded_value(module: &FormatModule, value: &Value, format: &Format) {
    use std::io::Write;
    let scope = Scope::new();
    let frag = MonoidalPrinter::new(module).compile_decoded_value(&scope, value, format);
    let mut lock = io::stdout().lock();
    match write!(&mut lock, "{}", frag) {
        Ok(_) => (),
        Err(e) => eprintln!("error: {e}"),
    }
}

enum Column {
    Branch,
    Space,
}

pub struct Flags {
    collapse_mapped_values: bool,
    omit_implied_values: bool,
    tables_for_record_sequences: bool,
    pretty_ascii_strings: bool,
    pretty_utf8_strings: bool,
    hide_double_underscore_fields: bool,
    show_redundant_formats: bool,
}

#[inline]
fn name_is_ascii_string(name: &str) -> bool {
    name.contains("ascii") && name.contains("string")
}

pub struct MonoidalPrinter<'module> {
    gutter: Vec<Column>,
    preview_len: Option<usize>,
    flags: Flags,
    module: &'module FormatModule,
}

impl<'module> MonoidalPrinter<'module> {
    fn is_implied_value_format(&self, format: &Format) -> bool {
        match format {
            Format::ItemVar(level, _args) => {
                self.is_implied_value_format(self.module.get_format(*level))
            }
            Format::EndOfInput => true,
            Format::Byte(bs) => bs.len() == 1,
            Format::Tuple(fields) => fields.iter().all(|f| self.is_implied_value_format(f)),
            Format::Record(fields) => fields.iter().all(|(_, f)| self.is_implied_value_format(f)),
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::RepeatCount(_, format)
            | Format::RepeatUntilSeq(_, format)
            | Format::RepeatUntilLast(_, format) => self.is_implied_value_format(format),
            Format::Slice(_, format) => self.is_implied_value_format(format),
            _ => false,
        }
    }

    fn is_atomic_format(&self, format: &Format) -> bool {
        match format {
            Format::ItemVar(level, _args) => self.is_atomic_format(self.module.get_format(*level)),
            Format::Byte(_) => true,
            _ => false,
        }
    }

    fn is_record_with_atomic_fields<'a>(
        &'a self,
        format: &'a Format,
    ) -> Option<Cow<'a, [(Cow<'static, str>, Format)]>> {
        match format {
            Format::ItemVar(level, _args) => {
                self.is_record_with_atomic_fields(self.module.get_format(*level))
            }
            Format::Record(fields) => {
                let fields = if self.flags.hide_double_underscore_fields
                    && fields.iter().any(|(l, _)| l.starts_with("__"))
                {
                    Cow::Owned(
                        fields
                            .iter()
                            .filter_map(|(l, x)| {
                                (!l.starts_with("__")).then(|| (l.clone(), x.clone()))
                            })
                            .collect::<Vec<_>>(),
                    )
                } else {
                    Cow::Borrowed(fields.deref())
                };
                if fields.iter().all(|(_l, f)| self.is_atomic_format(f)) {
                    Some(fields)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn is_atomic_value(&self, value: &Value, format: Option<&Format>) -> bool {
        if let Some(format) = format {
            if self.flags.pretty_ascii_strings && format.is_ascii_string_format(self.module) {
                return true;
            }
        }
        match value {
            Value::Char(_) => true,
            Value::Bool(_) => true,
            Value::U8(_) | Value::U16(_) | Value::U32(_) => true,
            Value::Tuple(values) => values.is_empty(),
            Value::Record(fields) => fields.is_empty(),
            Value::Seq(values) => values.is_empty(),
            Value::Variant(label, value) => match format {
                Some(Format::Union(branches)) | Some(Format::NondetUnion(branches)) => {
                    let (_, format) = branches.iter().find(|(l, _)| l == label).unwrap();
                    self.is_atomic_value(value, Some(format))
                }
                _ => self.is_atomic_value(value, None),
            },
            Value::Mapped(orig, value) => {
                if self.flags.collapse_mapped_values {
                    self.is_atomic_value(value, None)
                } else {
                    match format {
                        Some(Format::Map(format, _expr)) => {
                            self.is_atomic_value(orig, Some(format))
                        }
                        _ => self.is_atomic_value(orig, None),
                    }
                }
            }
            Value::Format(_) => false,
        }
    }
}

impl<'module> MonoidalPrinter<'module> {
    pub fn new(module: &'module FormatModule) -> MonoidalPrinter<'module> {
        let flags = Flags {
            collapse_mapped_values: true,
            omit_implied_values: true,
            tables_for_record_sequences: true,
            pretty_ascii_strings: true,
            pretty_utf8_strings: true,
            hide_double_underscore_fields: true,
            show_redundant_formats: false,
        };
        MonoidalPrinter {
            gutter: Vec::new(),
            preview_len: Some(10),
            flags,
            module,
        }
    }

    pub fn compile_decoded_value(
        &mut self,
        scope: &Scope<'_>,
        value: &Value,
        fmt: &Format,
    ) -> Fragment {
        let mut frag = Fragment::Empty;
        match fmt {
            Format::ItemVar(level, _args) => {
                let fmt_name = self.module.get_name(*level);

                // FIXME - this is a bit hackish, we should have a sentinel or marker to avoid magic strings
                if self.flags.pretty_utf8_strings && fmt_name == "utf8.string" {
                    self.compile_string(value)
                } else if self.flags.pretty_ascii_strings && name_is_ascii_string(fmt_name) {
                    self.compile_ascii_string(value)
                } else if self.flags.pretty_ascii_strings && fmt_name.starts_with("base.ascii-char")
                {
                    frag.encat(Fragment::Char('\''));
                    frag.encat(self.compile_ascii_char(value));
                    frag.encat(Fragment::Char('\''));
                    frag
                } else {
                    self.compile_decoded_value(scope, value, self.module.get_format(*level))
                }
            }
            Format::Fail => panic!("uninhabited format (value={value:?}"),
            Format::EndOfInput => self.compile_value(scope, value),
            Format::Align(_) => self.compile_value(scope, value),
            Format::Byte(_) => self.compile_value(scope, value),
            Format::Variant(label, format) => match value {
                Value::Variant(label2, value) => {
                    if label == label2 {
                        self.compile_variant(scope, label, value, Some(format))
                    } else {
                        panic!("expected variant label {label}, found {label2}");
                    }
                }
                _ => panic!("expected variant, found {value:?}"),
            },
            Format::Union(branches) | Format::NondetUnion(branches) => match value {
                Value::Variant(label, value) => {
                    let (_, format) = branches.iter().find(|(l, _)| l == label).unwrap();
                    self.compile_variant(scope, label, value, Some(format))
                }
                _ => panic!("expected variant, found {value:?}"),
            },
            Format::IsoUnion(_branches) => self.compile_value(scope, value),
            Format::Tuple(formats) => match value {
                Value::Tuple(values) => {
                    if self.flags.pretty_ascii_strings && self.is_ascii_tuple_format(formats) {
                        self.compile_ascii_seq(values)
                    } else {
                        self.compile_tuple(scope, values, Some(formats))
                    }
                }
                _ => panic!("expected tuple, found {value:?}"),
            },
            Format::Record(format_fields) => match value {
                Value::Record(value_fields) => {
                    self.compile_record(scope, value_fields, Some(format_fields))
                }
                _ => panic!("expected record, found {value:?}"),
            },
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::RepeatCount(_, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => match value {
                Value::Seq(values) => {
                    if self.flags.tables_for_record_sequences
                        && self.is_record_with_atomic_fields(format).is_some()
                    {
                        self.compile_seq_records(values, format)
                    } else if self.flags.pretty_ascii_strings
                        && format.is_ascii_char_format(self.module)
                    {
                        self.compile_ascii_seq(values)
                    } else {
                        self.compile_seq(scope, values, Some(format))
                    }
                }
                _ => panic!("expected sequence, found {value:?}"),
            },
            Format::Peek(format) => self.compile_decoded_value(scope, value, format),
            Format::PeekNot(_format) => self.compile_value(scope, value),
            Format::Slice(_, format) => self.compile_decoded_value(scope, value, format),
            Format::Bits(format) => self.compile_decoded_value(scope, value, format),
            Format::WithRelativeOffset(_, format) => {
                self.compile_decoded_value(scope, value, format)
            }
            Format::Map(format, _expr) => {
                if self.flags.collapse_mapped_values {
                    self.compile_value(scope, value)
                } else {
                    match value {
                        Value::Mapped(orig, _value) => {
                            self.compile_decoded_value(scope, orig, format)
                        }
                        _ => panic!("expected mapped value, found {value:?}"),
                    }
                }
            }
            Format::Compute(_expr) => self.compile_value(scope, value),
            Format::Match(head, branches) => {
                let head = head.eval(&scope);
                for (pattern, format) in branches {
                    if let Some(pattern_scope) = head.matches(scope, pattern) {
                        frag.encat(self.compile_decoded_value(&pattern_scope, value, format));
                        return frag;
                    }
                }
                panic!("non-exhaustive patterns");
            }
            Format::MatchVariant(head, branches) => {
                let head = head.eval(&scope);
                for (pattern, _label, format) in branches {
                    if let Some(pattern_scope) = head.matches(scope, pattern) {
                        if let Value::Variant(_label, value) = value {
                            frag.encat(self.compile_decoded_value(&pattern_scope, value, format));
                        } else {
                            panic!("expected variant value");
                        }
                        return frag;
                    }
                }
                panic!("non-exhaustive patterns");
            }
            Format::Dynamic(_) => self.compile_value(scope, value),
            Format::Apply(_) => self.compile_value(scope, value),
        }
    }

    fn is_ascii_tuple_format(&self, formats: &[Format]) -> bool {
        !formats.is_empty() && formats.iter().all(|f| f.is_ascii_char_format(self.module))
    }

    pub fn compile_value(&mut self, scope: &Scope<'_>, value: &Value) -> Fragment {
        match value {
            Value::Bool(true) => Fragment::String("true".into()),
            Value::Bool(false) => Fragment::String("false".into()),
            Value::U8(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::U16(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::U32(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::Char(c) => Fragment::DebugAtom(Rc::new(*c)),
            Value::Tuple(vals) => self.compile_tuple(scope, vals, None),
            Value::Seq(vals) => self.compile_seq(scope, vals, None),
            Value::Record(fields) => self.compile_record(scope, fields, None),
            Value::Variant(label, value) => self.compile_variant(scope, label, value, None),
            Value::Mapped(orig, value) => {
                if self.flags.collapse_mapped_values {
                    self.compile_value(scope, value)
                } else {
                    self.compile_value(scope, orig)
                }
            }
            Value::Format(f) => self.compile_format(f, Default::default()),
        }
    }

    fn extract_string_field<'a>(
        &self,
        fields: &'a Vec<(Cow<'static, str>, Value)>,
    ) -> Option<&'a Value> {
        fields
            .iter()
            .find_map(|(label, value)| (label == "string").then_some(value))
    }

    pub fn compile_string(&self, value: &Value) -> Fragment {
        let vs = match value.coerce_mapped_value() {
            Value::Record(fields) => {
                match self
                    .extract_string_field(fields)
                    .unwrap_or_else(|| unreachable!("no string field"))
                {
                    Value::Seq(vs) => vs,
                    v => panic!("expected sequence value, found {v:?}"),
                }
            }
            Value::Seq(vs) => vs,
            v => panic!("expected record or sequence, found {v:?}"),
        };
        self.compile_char_seq(vs)
    }

    pub fn compile_ascii_string(&self, value: &Value) -> Fragment {
        let vs = match value.coerce_mapped_value() {
            Value::Record(fields) => {
                match self
                    .extract_string_field(fields)
                    .unwrap_or_else(|| unreachable!("no string field"))
                {
                    Value::Seq(vs) => vs,
                    v => panic!("expected sequence value, found {v:?}"),
                }
            }
            _ => panic!("expected record value, found {value:?}"),
        };
        self.compile_ascii_seq(vs)
    }

    fn compile_char_seq(&self, vals: &[Value]) -> Fragment {
        let mut frag = Fragment::new();
        frag.encat(Fragment::Char('"'));
        for v in vals {
            frag.encat(self.compile_char(v));
        }
        frag.encat(Fragment::Char('"'));
        frag.group()
    }

    fn compile_ascii_seq(&self, vals: &[Value]) -> Fragment {
        let mut frag = Fragment::new();
        frag.encat(Fragment::Char('"'));
        for v in vals {
            frag.encat(self.compile_ascii_char(v));
        }
        frag.encat(Fragment::Char('"'));
        frag.group()
    }

    fn compile_char(&self, v: &Value) -> Fragment {
        let c = match v.coerce_mapped_value() {
            Value::U8(b) => *b as char,
            Value::Char(c) => *c,
            _ => panic!("expected U8 or Char value, found {v:?}"),
        };
        match c {
            '\x00'..='\x7f' => Fragment::String(c.escape_debug().collect::<String>().into()),
            _ => Fragment::Char(c),
        }
    }

    fn compile_ascii_char(&self, v: &Value) -> Fragment {
        let b = match v {
            Value::U8(b) => *b,
            _ => panic!("expected U8 value, found {v:?}"),
        };
        match b {
            0x00 => Fragment::String("\\0".into()),
            0x09 => Fragment::String("\\t".into()),
            0x0A => Fragment::String("\\n".into()),
            0x0D => Fragment::String("\\r".into()),
            32..=127 => Fragment::Char(b as char),
            _ => Fragment::String(format!("\\x{b:02X}").into()),
        }
    }

    fn compile_tuple(
        &mut self,
        scope: &Scope<'_>,
        vals: &[Value],
        formats: Option<&[Format]>,
    ) -> Fragment {
        if vals.is_empty() {
            Fragment::String("()".into())
        } else {
            let mut frag = Fragment::new();
            let last_index = vals.len() - 1;
            for index in 0..last_index {
                frag.encat(self.compile_field_value_continue(
                    scope,
                    index,
                    &vals[index],
                    formats.map(|fs| &fs[index]),
                    true,
                ));
            }
            frag.encat(self.compile_field_value_last(
                scope,
                last_index,
                &vals[last_index],
                formats.map(|fs| &fs[last_index]),
                true,
            ));
            frag
        }
    }

    fn compile_seq(
        &mut self,
        scope: &Scope<'_>,
        vals: &[Value],
        format: Option<&Format>,
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
            for (index, val) in vals[0..upper_bound].iter().enumerate() {
                frag.encat(self.compile_field_value_continue(scope, index, val, format, false));
            }
            if any_skipped {
                frag.encat(self.compile_field_skipped());
            }
            frag.encat(self.compile_field_value_last(
                scope,
                last_index,
                &vals[last_index],
                format,
                false,
            ));
            frag
        }
    }

    fn compile_seq_records(&mut self, vals: &[Value], format: &Format) -> Fragment {
        let fields = self.is_record_with_atomic_fields(format).unwrap();
        let mut cols = Vec::new();
        let mut header = Vec::new();
        for (label, _) in fields.as_ref() {
            cols.push(label.len());
            header.push(label.clone());
        }
        let mut rows = Vec::new();
        for v in vals {
            let mut row = Vec::new();
            if let Value::Record(fields) = v {
                for (i, (_l, v)) in fields.iter().enumerate() {
                    let cell = atomic_value_to_string(v);
                    cols[i] = std::cmp::max(cols[i], cell.len());
                    row.push(cell);
                }
            } else {
                panic!("expected record value: {v:?}");
            }
            rows.push(row);
        }
        self.compile_table(&cols, &header, &rows)
    }

    fn compile_table(
        &mut self,
        cols: &[usize],
        header: &[Cow<'static, str>],
        rows: &[Vec<String>],
    ) -> Fragment {
        let mut frags = FragmentBuilder::new();
        let frag = frags.active_mut();
        frag.encat(self.compile_gutter());
        frag.encat(Fragment::Symbol(Symbol::Elbow));
        for (i, th) in header.iter().enumerate() {
            frag.encat(Fragment::String(
                format!(" {:>width$}", th, width = cols[i]).into(),
            ));
        }
        frag.engroup().encat_break();
        let mut frag = frags.renew();
        self.gutter.push(Column::Space);
        for tr in rows {
            frag.encat(self.compile_gutter());
            for (i, td) in tr.iter().enumerate() {
                frag.encat(Fragment::String(
                    format!(" {:>width$}", td, width = cols[i]).into(),
                ));
            }
            frag.engroup().encat_break();
            frag = frags.renew();
        }
        self.gutter.pop();
        frags.finalize()
    }

    fn compile_record(
        &mut self,
        scope: &Scope<'_>,
        value_fields: &[(Cow<'static, str>, Value)],
        format_fields: Option<&[(Cow<'static, str>, Format)]>,
    ) -> Fragment {
        let mut value_fields_filt = Vec::new();
        let mut format_fields_filt = format_fields.map(|_| Vec::new());

        let (value_fields, format_fields) = if self.flags.hide_double_underscore_fields
            && value_fields.iter().any(|(lab, _)| lab.starts_with("__"))
        {
            value_fields_filt.extend(
                value_fields
                    .iter()
                    .filter(|(lab, _)| !lab.starts_with("__"))
                    .cloned(),
            );
            // we can unwrap below because format_fields_filt is only Some (and the closure will only be called) if format_fields is Some
            format_fields_filt.as_mut().map(|v: &mut Vec<_>| {
                v.extend(
                    format_fields
                        .unwrap()
                        .iter()
                        .filter(|(lab, _)| !lab.starts_with("__"))
                        .cloned(),
                )
            });
            (value_fields_filt.deref(), format_fields_filt.as_deref())
        } else {
            (value_fields, format_fields)
        };
        if value_fields.is_empty() {
            Fragment::String("{}".into())
        } else {
            let mut frag = Fragment::new();
            let mut record_scope = Scope::child(scope);
            let last_index = value_fields.len() - 1;
            for (index, (label, value)) in value_fields[..last_index].iter().enumerate() {
                let format = format_fields.map(|fs| &fs[index].1);
                frag.encat(self.compile_field_value_continue(
                    &record_scope,
                    label,
                    value,
                    format,
                    true,
                ));
                record_scope.push(label.clone(), value.clone());
            }
            let (label, value) = &value_fields[last_index];
            let format = format_fields.map(|fs| &fs[last_index].1);
            frag.encat(self.compile_field_value_last(&record_scope, label, value, format, true));
            frag
        }
    }

    fn compile_variant(
        &mut self,
        scope: &Scope<'_>,
        label: &str,
        value: &Value,
        format: Option<&Format>,
    ) -> Fragment {
        if self.is_atomic_value(value, format) {
            let mut frag = Fragment::new();
            frag.encat(Fragment::String(format!("{{ {label} := ").into()));
            if let Some(format) = format {
                frag.encat(self.compile_decoded_value(scope, value, format));
            } else {
                frag.encat(self.compile_value(scope, value));
            }
            frag.encat(Fragment::String(" }".into()));
            frag.engroup();
            frag
        } else {
            self.compile_field_value_last(scope, label, value, format, true)
        }
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

    fn is_indirect_format(&self, format: &Format) -> bool {
        match format {
            Format::ItemVar(..) | Format::Dynamic(..) | Format::Apply(..) => true,
            _ => false,
        }
    }

    fn compile_field_value_continue(
        &mut self,
        scope: &Scope<'_>,
        label: impl fmt::Display,
        value: &Value,
        format: Option<&Format>,
        format_needed: bool,
    ) -> Fragment {
        let mut frags = FragmentBuilder::new();
        frags.push(self.compile_gutter());
        frags.push(Fragment::cat(
            Fragment::Symbol(Symbol::Junction),
            Fragment::String(format!("{label}").into()),
        ));

        self.gutter.push(Column::Branch);
        let frag_value = self.compile_field_value(scope, value, format);
        self.gutter.pop();

        if let Some(format) = format {
            if format_needed
                || self.flags.show_redundant_formats
                || (self.is_indirect_format(format) && !frag_value.is_single_line(true))
            {
                frags.push(Fragment::String(" <- ".into()));
                frags.push(self.compile_format(format, Precedence::FORMAT_COMPOUND));
            }
        }
        frags.push(frag_value);
        frags.finalize().group()
    }

    fn compile_field_value_last(
        &mut self,
        scope: &Scope<'_>,
        label: impl fmt::Display,
        value: &Value,
        format: Option<&Format>,
        format_needed: bool,
    ) -> Fragment {
        let mut frags = FragmentBuilder::new();
        frags.push(self.compile_gutter());
        frags.push(Fragment::cat(
            Fragment::Symbol(Symbol::Elbow),
            Fragment::String(format!("{label}").into()),
        ));

        self.gutter.push(Column::Space);
        let frag_value = self.compile_field_value(scope, value, format);
        self.gutter.pop();

        if let Some(format) = format {
            if format_needed
                || self.flags.show_redundant_formats
                || (self.is_indirect_format(format) && !frag_value.is_single_line(true))
            {
                frags.push(Fragment::String(" <- ".into()));
                frags.push(self.compile_format(format, Default::default()));
            }
        }
        frags.push(frag_value);
        frags.finalize().group()
    }

    fn compile_field_value(
        &mut self,
        scope: &Scope<'_>,
        value: &Value,
        format: Option<&Format>,
    ) -> Fragment {
        match format {
            Some(format) => {
                if self.flags.omit_implied_values && self.is_implied_value_format(format) {
                    Fragment::Char('\n')
                } else {
                    Fragment::join_with_wsp(
                        Fragment::String(" :=".into()),
                        self.compile_decoded_value(scope, value, format),
                    )
                    .group()
                }
            }
            None => Fragment::join_with_wsp(
                Fragment::String(" :=".into()),
                self.compile_value(scope, value),
            )
            .group(),
        }
    }

    fn compile_field_skipped(&mut self) -> Fragment {
        self.compile_gutter()
            .cat(Fragment::String("~\n".into()))
            .group()
    }

    #[inline]
    fn compile_binop(
        &mut self,
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

    /// Renders an Expr as a prefix-operator (with optional auxiliary arguments in parentheses)
    /// applied to a nested Expr.
    #[inline]
    fn compile_prefix(
        &mut self,
        op: &'static str,
        args: Option<&[&Expr]>,
        operand: &Expr,
    ) -> Fragment {
        let mut frags = FragmentBuilder::new();

        frags.push(Fragment::String(op.into()));
        match args {
            None => (),
            Some(args) => {
                let frag = frags.active_mut();
                frag.encat(Fragment::Char('('));
                frag.encat(Fragment::seq(
                    args.into_iter()
                        .map(|arg| self.compile_expr(arg, Precedence::default()))
                        .collect::<Vec<_>>(),
                    Some(Fragment::String(", ".into())),
                ));
                frag.encat(Fragment::Char(')'));
            }
        }
        frags.push(self.compile_expr(operand, Precedence::ATOM));
        frags.finalize_with_sep(Fragment::Char(' '))
    }

    fn compile_expr(&mut self, expr: &Expr, prec: Precedence) -> Fragment {
        match expr {
            Expr::Match(head, _) => cond_paren(
                Fragment::String("match ".into())
                    .cat(self.compile_expr(head, Precedence::MATCH))
                    .cat(Fragment::String(" { ... }".into()))
                    .group(),
                prec,
                Precedence::MATCH,
            ),
            Expr::Lambda(name, expr) => cond_paren(
                Fragment::String(name.clone().into())
                    .cat(Fragment::String(" -> ".into()))
                    .cat(self.compile_expr(expr, Precedence::ARROW))
                    .group(),
                prec,
                Precedence::ARROW,
            ),
            Expr::BitAnd(lhs, rhs) => cond_paren(
                self.compile_binop(" & ", lhs, rhs, Precedence::BITAND, Precedence::BITAND),
                prec,
                Precedence::BITAND,
            ),
            Expr::BitOr(lhs, rhs) => cond_paren(
                self.compile_binop(" | ", lhs, rhs, Precedence::BITOR, Precedence::BITOR),
                prec,
                Precedence::BITOR,
            ),
            Expr::Eq(lhs, rhs) => cond_paren(
                self.compile_binop(" == ", lhs, rhs, Precedence::EQUALITY, Precedence::EQUALITY),
                prec,
                Precedence::COMPARE,
            ),
            Expr::Ne(lhs, rhs) => cond_paren(
                self.compile_binop(" != ", lhs, rhs, Precedence::EQUALITY, Precedence::EQUALITY),
                prec,
                Precedence::COMPARE,
            ),
            Expr::Lt(lhs, rhs) => cond_paren(
                self.compile_binop(" < ", lhs, rhs, Precedence::COMPARE, Precedence::COMPARE),
                prec,
                Precedence::COMPARE,
            ),
            Expr::Gt(lhs, rhs) => cond_paren(
                self.compile_binop(" > ", lhs, rhs, Precedence::COMPARE, Precedence::COMPARE),
                prec,
                Precedence::COMPARE,
            ),
            Expr::Lte(lhs, rhs) => cond_paren(
                self.compile_binop(" <= ", lhs, rhs, Precedence::COMPARE, Precedence::COMPARE),
                prec,
                Precedence::COMPARE,
            ),
            Expr::Gte(lhs, rhs) => cond_paren(
                self.compile_binop(" >= ", lhs, rhs, Precedence::COMPARE, Precedence::COMPARE),
                prec,
                Precedence::COMPARE,
            ),
            Expr::Add(lhs, rhs) => cond_paren(
                self.compile_binop(" + ", lhs, rhs, Precedence::ADDSUB, Precedence::ADDSUB),
                prec,
                Precedence::ADDSUB,
            ),
            Expr::Sub(lhs, rhs) => cond_paren(
                self.compile_binop(" - ", lhs, rhs, Precedence::ADDSUB, Precedence::ADDSUB),
                prec,
                Precedence::ADDSUB,
            ),
            Expr::Shl(lhs, rhs) => cond_paren(
                self.compile_binop(" << ", lhs, rhs, Precedence::BITSHIFT, Precedence::BITSHIFT),
                prec,
                Precedence::BITSHIFT,
            ),
            Expr::Shr(lhs, rhs) => cond_paren(
                self.compile_binop(" >> ", lhs, rhs, Precedence::BITSHIFT, Precedence::BITSHIFT),
                prec,
                Precedence::BITSHIFT,
            ),
            Expr::Mul(lhs, rhs) => cond_paren(
                self.compile_binop(" * ", lhs, rhs, Precedence::MUL, Precedence::MUL),
                prec,
                Precedence::MUL,
            ),
            Expr::Div(lhs, rhs) => cond_paren(
                self.compile_binop(" / ", lhs, rhs, Precedence::DIVREM, Precedence::DIVREM),
                prec,
                Precedence::DIVREM,
            ),
            Expr::Rem(lhs, rhs) => cond_paren(
                self.compile_binop(" % ", lhs, rhs, Precedence::DIVREM, Precedence::DIVREM),
                prec,
                Precedence::DIVREM,
            ),
            Expr::AsU8(expr) => cond_paren(
                self.compile_prefix("as-u8", None, expr),
                prec,
                Precedence::CAST,
            ),
            Expr::AsU16(expr) => cond_paren(
                self.compile_prefix("as-u16", None, expr),
                prec,
                Precedence::CAST,
            ),
            Expr::AsU32(expr) => cond_paren(
                self.compile_prefix("as-u32", None, expr),
                prec,
                Precedence::CAST,
            ),
            Expr::AsChar(expr) => cond_paren(
                self.compile_prefix("as-char", None, expr),
                prec,
                Precedence::CAST,
            ),
            Expr::U16Be(bytes) => cond_paren(
                self.compile_prefix("u16be", None, bytes),
                prec,
                Precedence::CAST,
            ),
            Expr::U16Le(bytes) => cond_paren(
                self.compile_prefix("u16le", None, bytes),
                prec,
                Precedence::CAST,
            ),
            Expr::U32Be(bytes) => cond_paren(
                self.compile_prefix("u32be", None, bytes),
                prec,
                Precedence::CAST,
            ),
            Expr::U32Le(bytes) => cond_paren(
                self.compile_prefix("u32le", None, bytes),
                prec,
                Precedence::CAST,
            ),
            Expr::SeqLength(seq) => cond_paren(
                self.compile_prefix("seq-length", None, seq),
                prec,
                Precedence::FUNAPP,
            ),
            Expr::SubSeq(seq, start, length) => cond_paren(
                self.compile_prefix("sub-seq", Some(&[&start, &length]), seq),
                prec,
                Precedence::FUNAPP,
            ),
            Expr::FlatMap(expr, seq) => cond_paren(
                self.compile_prefix("flat-map", Some(&[&expr]), seq),
                prec,
                Precedence::FUNAPP,
            ),
            Expr::FlatMapAccum(expr, accum, _accum_type, seq) => cond_paren(
                self.compile_prefix("flat-map-accum", Some(&[&expr, &accum]), seq),
                prec,
                Precedence::FUNAPP,
            ),
            Expr::Dup(count, expr) => cond_paren(
                self.compile_prefix("dup", Some(&[&count]), expr),
                prec,
                Precedence::FUNAPP,
            ),
            Expr::Inflate(expr) => cond_paren(
                self.compile_prefix("inflate", None, expr),
                prec,
                Precedence::FUNAPP,
            ),

            Expr::TupleProj(head, index) => cond_paren(
                self.compile_expr(head, Precedence::PROJ)
                    .cat(Fragment::Char('.'))
                    .cat(Fragment::DisplayAtom(Rc::new(*index)))
                    .group(),
                prec,
                Precedence::PROJ,
            ),
            Expr::RecordProj(head, label) => cond_paren(
                self.compile_expr(head, Precedence::PROJ)
                    .cat(Fragment::Char('.'))
                    .cat(Fragment::String(label.clone().into()))
                    .group(),
                prec,
                Precedence::PROJ,
            ),
            Expr::Var(name) => Fragment::String(name.clone().into()),
            Expr::Bool(b) => Fragment::DisplayAtom(Rc::new(*b)),
            Expr::U8(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::U16(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::U32(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::Tuple(..) => Fragment::String("(...)".into()),
            Expr::Record(..) => Fragment::String("{ ... }".into()),
            Expr::Variant(label, expr) => Fragment::String("{ ".into())
                .cat(Fragment::String(label.clone().into()))
                .cat(Fragment::String(" := ".into()))
                .cat(self.compile_expr(expr, Default::default()))
                .cat(Fragment::String(" }".into()))
                .group(),
            Expr::Seq(..) => Fragment::String("[..]".into()),
        }
    }

    /// Creates a [Fragment] representing a compound format as a prefix label
    /// followed by a nested inner format.
    fn compile_nested_format(
        &mut self,
        label: &'static str,
        args: Option<&[Fragment]>,
        inner: &Format,
        prec: Precedence,
    ) -> Fragment {
        let mut frags = FragmentBuilder::new();
        frags.push(Fragment::String(label.into()));
        if let Some(args) = args {
            for arg in args.into_iter() {
                frags.push(arg.clone());
            }
        }
        frags.push(self.compile_format(inner, prec.bump_format()));
        frags.finalize_with_sep(Fragment::Char(' '))
    }

    fn compile_format(&mut self, format: &Format, prec: Precedence) -> Fragment {
        match format {
            Format::Variant(label, f) => cond_paren(
                self.compile_nested_format(
                    "variant",
                    Some(&[Fragment::String(label.clone())]),
                    f,
                    prec,
                ),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Union(_) | Format::NondetUnion(_) | Format::IsoUnion(_) => cond_paren(
                Fragment::String("_ |...| _".into()),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Repeat(format) => cond_paren(
                self.compile_nested_format("repeat", None, format, prec),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Repeat1(format) => cond_paren(
                self.compile_nested_format("repeat1", None, format, prec),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::RepeatCount(len, format) => {
                let expr_frag = self.compile_expr(len, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format("repeat-count", Some(&[expr_frag]), format, prec),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::RepeatUntilLast(expr, format) => {
                let expr_frag = self.compile_expr(expr, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "repeat-until-last",
                        Some(&[expr_frag]),
                        format,
                        prec,
                    ),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::RepeatUntilSeq(expr, format) => {
                let expr_frag = self.compile_expr(expr, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "repeat-until-seq",
                        Some(&[expr_frag]),
                        format,
                        prec,
                    ),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::Peek(format) => cond_paren(
                self.compile_nested_format("peek", None, format, prec),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::PeekNot(format) => cond_paren(
                self.compile_nested_format("peek-not", None, format, prec),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Slice(len, format) => {
                let expr_frag = self.compile_expr(len, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format("slice", Some(&[expr_frag]), format, prec),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::Bits(format) => cond_paren(
                self.compile_nested_format("bits", None, format, prec),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::WithRelativeOffset(offset, format) => {
                let expr_frag = self.compile_expr(offset, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "with-relative-offset",
                        Some(&[expr_frag]),
                        format,
                        prec,
                    ),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::Map(format, expr) => {
                let expr_frag = self.compile_expr(expr, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format("map", Some(&[expr_frag]), format, prec),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::Compute(expr) => cond_paren(
                Fragment::cat(
                    Fragment::String("compute ".into()),
                    self.compile_expr(expr, Default::default()),
                ),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Match(head, _) | Format::MatchVariant(head, _) => cond_paren(
                Fragment::String("match ".into())
                    .cat(self.compile_expr(head, Precedence::PROJ))
                    .cat(Fragment::String(" { ... }".into()))
                    .group(),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Dynamic(_) => Fragment::String("dynamic".into()),
            Format::Apply(_) => Fragment::String("apply".into()),

            Format::ItemVar(var, args) => {
                let mut frag = Fragment::new();
                frag.encat(Fragment::String(
                    self.module.get_name(*var).to_string().into(),
                ));
                if !args.is_empty() {
                    frag.encat(Fragment::String("(...)".into()));
                }
                frag
            }
            Format::Fail => Fragment::String("fail".into()),
            Format::EndOfInput => Fragment::String("end-of-input".into()),
            Format::Align(n) => Fragment::String(format!("align {n}").into()),

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
            Format::Tuple(formats) if formats.is_empty() => Fragment::String("()".into()),
            Format::Tuple(_) => Fragment::String("(...)".into()),

            Format::Record(fields) if fields.is_empty() => Fragment::String("{}".into()),
            Format::Record(_) => Fragment::String("{ ... }".into()),
        }
    }
}

/// Operator Precedence classes
///
///
#[derive(Copy, Clone, Debug, Default)]
enum Precedence {
    Atomic, // Highest precedence
    Projection,
    Prefix, // Highest natural precedence
    ArithInfix(ArithLevel),
    BitwiseInfix(BitwiseLevel),
    Comparison(CompareLevel), // Unsound when chained
    Calculus,                 // Arrow and Match
    #[default]
    Top,        // Entry level for neutral context
}

#[derive(Copy, Clone, Debug)]
enum CompareLevel {
    Comparison = 0, // Highest comparative precedence
    Equality,
}

#[derive(Copy, Clone, Debug)]
enum ArithLevel {
    DivRem = 0, // Highest arithmetic precedence
    Mul,
    AddSub,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum BitwiseLevel {
    Shift = 0, // Highest bitwise precedence
    And = 1,
    Or = 2,
}

/// Intransitive partial relation over operator subclasses
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Relation {
    /// `.<`
    Inferior,
    /// `.=`
    Congruent,
    /// ``.>`
    Superior,
    /// ``><``
    Disjoint,
}

trait IntransitiveOrd {
    fn relate(&self, other: &Self) -> Relation;

    fn inferior(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Inferior)
    }

    fn superior(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Superior)
    }

    fn congruent(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Congruent)
    }

    fn disjoint(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Disjoint)
    }
}

impl IntransitiveOrd for CompareLevel {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            (Self::Comparison, Self::Comparison) | (Self::Equality, Self::Equality) => {
                Relation::Congruent
            }
            (Self::Comparison, Self::Equality) => Relation::Disjoint,
            (Self::Equality, Self::Comparison) => Relation::Disjoint,
        }
    }
}

impl IntransitiveOrd for ArithLevel {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            (Self::DivRem, Self::DivRem)
            | (Self::Mul, Self::Mul)
            | (Self::AddSub, Self::AddSub) => Relation::Congruent,
            (Self::DivRem, Self::Mul) | (Self::Mul, Self::DivRem) => Relation::Disjoint,
            (Self::AddSub, _) => Relation::Inferior,
            (_, Self::AddSub) => Relation::Superior,
        }
    }
}

impl IntransitiveOrd for BitwiseLevel {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            (BitwiseLevel::Shift, BitwiseLevel::Shift) => Relation::Congruent,
            (BitwiseLevel::Shift, _) => Relation::Superior,
            (_, BitwiseLevel::Shift) => Relation::Inferior,
            (BitwiseLevel::And, BitwiseLevel::And) => Relation::Congruent,
            (BitwiseLevel::And, BitwiseLevel::Or) => Relation::Superior,
            (BitwiseLevel::Or, BitwiseLevel::And) => Relation::Inferior,
            (BitwiseLevel::Or, BitwiseLevel::Or) => Relation::Congruent,
        }
    }
}

/// Rules:
///   x .= x
///   Atomic .> Proj .> Prefix .> *Infix .> Comparison .> Calculus .> Top
///   rel(x, y) = rel(ArithInfix(x), ArithInfix(y))
///   rel(x, y) = rel(BitwiseInfix(x), BitwiseInfix(y))
///   Bitwise(_) >< Arith(_)
impl IntransitiveOrd for Precedence {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            // Trivial Congruences
            (Self::Atomic, Self::Atomic) => Relation::Congruent,
            (Self::Projection, Self::Projection) => Relation::Congruent,
            (Self::Prefix, Self::Prefix) => Relation::Congruent,
            (Self::Calculus, Self::Calculus) => Relation::Congruent,
            (Self::Top, Self::Top) => Relation::Congruent,

            // Descending relations
            (Self::Atomic, _) => Relation::Superior,
            (_, Self::Atomic) => Relation::Superior,
            (Self::Projection, _) => Relation::Superior,
            (_, Self::Projection) => Relation::Inferior,
            (Self::Prefix, _) => Relation::Superior,
            (_, Self::Prefix) => Relation::Inferior,

            // Ascending relations
            (Self::Top, _) => Relation::Inferior,
            (_, Self::Top) => Relation::Superior,
            (Self::Calculus, _) => Relation::Inferior,
            (_, Self::Calculus) => Relation::Superior,

            // Implications
            (Self::ArithInfix(x), Self::ArithInfix(y)) => x.relate(y),
            (Self::BitwiseInfix(x), Self::BitwiseInfix(y)) => x.relate(y),
            (Self::Comparison(x), Self::Comparison(y)) => x.relate(y),

            // Ascending relations (continued)
            (Self::Comparison(_), _) => Relation::Inferior,
            (_, Self::Comparison(_)) => Relation::Superior,

            // Disjunctions
            (Self::ArithInfix(_), Self::BitwiseInfix(_)) => Relation::Disjoint,
            (Self::BitwiseInfix(_), Self::ArithInfix(_)) => Relation::Disjoint,
        }
    }
}

impl Precedence {
    #![allow(dead_code)]
    const TOP: Self = Precedence::Top;
    const ARROW: Self = Precedence::Calculus;
    const MATCH: Self = Precedence::Calculus;
    const COMPARE: Self = Precedence::Comparison(CompareLevel::Comparison);
    const EQUALITY: Self = Precedence::Comparison(CompareLevel::Equality);
    const BITOR: Self = Precedence::BitwiseInfix(BitwiseLevel::Or);
    const ADDSUB: Self = Precedence::ArithInfix(ArithLevel::AddSub);
    const BITAND: Self = Precedence::BitwiseInfix(BitwiseLevel::And);
    const DIVREM: Self = Precedence::ArithInfix(ArithLevel::DivRem);
    const MUL: Self = Precedence::ArithInfix(ArithLevel::Mul);
    const BITSHIFT: Self = Precedence::BitwiseInfix(BitwiseLevel::Shift);
    const FUNAPP: Self = Precedence::Prefix;
    const CAST: Self = Precedence::Prefix;
    const PROJ: Self = Precedence::Projection;
    const ATOM: Self = Precedence::Atomic;

    const FORMAT_COMPOUND: Self = Self::Top;
    const FORMAT_ATOM: Self = Self::Atomic;

    fn bump_format(&self) -> Self {
        match self {
            Precedence::Top => Precedence::Atomic,
            Precedence::Atomic => Precedence::Atomic,
            _ => unreachable!("Unexpected non-format precedence level {self:?}"),
        }
    }
}

fn cond_paren(frag: Fragment, current: Precedence, cutoff: Precedence) -> Fragment {
    match current.relate(&cutoff) {
        Relation::Disjoint | Relation::Superior => {
            Fragment::Char('(').cat(frag).cat(Fragment::Char(')'))
        }
        Relation::Congruent | Relation::Inferior => frag,
    }
}
