use std::{fmt, io, rc::Rc};

use crate::{Expr, Format, FormatModule, Scope, Value};

use super::{Fragment, Fragments};

pub fn print_decoded_value(module: &FormatModule, value: &Value, format: &Format) {
    Context::new(io::stdout(), module)
        .write_decoded_value(value, format)
        .unwrap()
}

fn atomic_value_to_string(value: &Value) -> String {
    match value {
        Value::U8(n) => n.to_string(),
        _ => panic!("expected atomic value"),
    }
}

pub fn print_decoded_value_monoidal(module: &FormatModule, value: &Value, format: &Format) {
    use std::io::Write;
    let frag = MonoidalPrinter::new(module).compile_decoded_value(value, format);
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

pub struct Context<'module, W: io::Write> {
    writer: W,
    gutter: Vec<Column>,
    preview_len: Option<usize>,
    flags: Flags,
    module: &'module FormatModule,
    scope: Scope,
}

pub struct Flags {
    collapse_computed_values: bool,
    omit_implied_values: bool,
    tables_for_record_sequences: bool,
    pretty_ascii_strings: bool,
}

impl<'module, W: io::Write> Context<'module, W> {
    pub fn new(writer: W, module: &'module FormatModule) -> Context<'module, W> {
        let flags = Flags {
            collapse_computed_values: true,
            omit_implied_values: true,
            tables_for_record_sequences: true,
            pretty_ascii_strings: true,
        };
        Context {
            writer,
            gutter: Vec::new(),
            preview_len: Some(10),
            flags,
            module,
            scope: Scope::new(),
        }
    }

    pub fn write_decoded_value(&mut self, value: &Value, format: &Format) -> io::Result<()> {
        match format {
            Format::ItemVar(level, _args) => {
                if self.flags.pretty_ascii_strings
                    && self.module.get_name(*level) == "base.asciiz-string"
                {
                    self.write_ascii_string(value)
                } else if self.flags.pretty_ascii_strings
                    && self.module.get_name(*level) == "base.ascii-char"
                {
                    write!(&mut self.writer, "'")?;
                    self.write_ascii_char(value)?;
                    write!(&mut self.writer, "'")
                } else {
                    self.write_decoded_value(value, self.module.get_format(*level))
                }
            }
            Format::Fail => panic!("uninhabited format"),
            Format::EndOfInput => self.write_value(value),
            Format::Align(_) => self.write_value(value),
            Format::Byte(_) => self.write_value(value),
            Format::Union(branches) => match value {
                Value::Variant(label, value) => {
                    let (_, format) = branches.iter().find(|(l, _)| l == label).unwrap();
                    self.write_variant(label, value, Some(format))
                }
                _ => panic!("expected variant"),
            },
            Format::Tuple(formats) => match value {
                Value::Tuple(values) => {
                    if self.flags.pretty_ascii_strings && self.is_ascii_tuple_format(formats) {
                        self.write_ascii_seq(values)
                    } else {
                        self.write_tuple(values, Some(formats))
                    }
                }
                _ => panic!("expected tuple"),
            },
            Format::Record(format_fields) => match value {
                Value::Record(value_fields) => self.write_record(value_fields, Some(format_fields)),
                _ => panic!("expected record"),
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
                        self.write_seq_records(values, format)
                    } else if self.flags.pretty_ascii_strings && self.is_ascii_char_format(format) {
                        self.write_ascii_seq(values)
                    } else {
                        self.write_seq(values, Some(format))
                    }
                }
                _ => panic!("expected sequence"),
            },
            Format::Peek(format) => self.write_decoded_value(value, format),
            Format::Slice(_, format) | Format::FixedSlice(_, format) => {
                self.write_decoded_value(value, format)
            }
            Format::Bits(format) => self.write_decoded_value(value, format),
            Format::WithRelativeOffset(_, format) => self.write_decoded_value(value, format),
            Format::Compute(_expr) => self.write_value(value),
            Format::Match(head, branches) => {
                let head = head.eval(&mut self.scope);
                let initial_len = self.scope.len();
                let (_, format) = branches
                    .iter()
                    .find(|(pattern, _)| head.matches(&mut self.scope, pattern))
                    .expect("exhaustive patterns");
                self.write_decoded_value(value, format)?;
                self.scope.truncate(initial_len);
                Ok(())
            }
            Format::MatchVariant(head, branches) => {
                let head = head.eval(&mut self.scope);
                let initial_len = self.scope.len();
                let (_, _label, format) = branches
                    .iter()
                    .find(|(pattern, _, _)| head.matches(&mut self.scope, pattern))
                    .expect("exhaustive patterns");
                if let Value::Variant(_label, value) = value {
                    self.write_decoded_value(value, format)?;
                } else {
                    panic!("expected variant value");
                }
                self.scope.truncate(initial_len);
                Ok(())
            }
            Format::Dynamic(_) => self.write_value(value),
        }
    }

    pub fn write_value(&mut self, value: &Value) -> io::Result<()> {
        match value {
            Value::Bool(true) => write!(&mut self.writer, "true"),
            Value::Bool(false) => write!(&mut self.writer, "false"),
            Value::U8(i) => write!(&mut self.writer, "{i}"),
            Value::U16(i) => write!(&mut self.writer, "{i}"),
            Value::U32(i) => write!(&mut self.writer, "{i}"),
            Value::Tuple(vals) => self.write_tuple(vals, None),
            Value::Seq(vals) => self.write_seq(vals, None),
            Value::Record(fields) => self.write_record(fields, None),
            Value::Variant(label, value) => self.write_variant(label, value, None),
        }
    }

    fn write_ascii_string(&mut self, value: &Value) -> io::Result<()> {
        let vs = match value {
            Value::Record(fields) => {
                match fields.iter().find(|(label, _)| label == "string").unwrap() {
                    (_, Value::Seq(vs)) => vs,
                    _ => panic!("expected sequence value"),
                }
            }
            _ => panic!("expected record value"),
        };
        self.write_ascii_seq(vs)
    }

    fn write_ascii_seq(&mut self, vals: &[Value]) -> Result<(), io::Error> {
        write!(&mut self.writer, "\"")?;
        for v in vals {
            self.write_ascii_char(v)?;
        }
        write!(&mut self.writer, "\"")
    }

    fn write_ascii_char(&mut self, v: &Value) -> io::Result<()> {
        let b = match v {
            Value::U8(b) => *b,
            _ => panic!("expected U8 value"),
        };
        match b {
            0x00 => write!(&mut self.writer, "\\0"),
            0x09 => write!(&mut self.writer, "\\t"),
            0x0A => write!(&mut self.writer, "\\n"),
            0x0D => write!(&mut self.writer, "\\r"),
            32..=127 => write!(&mut self.writer, "{}", b as char),
            _ => write!(&mut self.writer, "\\x{:02X}", b),
        }
    }

    fn write_tuple(&mut self, vals: &[Value], formats: Option<&[Format]>) -> Result<(), io::Error> {
        if vals.is_empty() {
            write!(&mut self.writer, "()")
        } else {
            let last_index = vals.len() - 1;
            for index in 0..last_index {
                self.write_field_value_continue(index, &vals[index], formats.map(|fs| &fs[index]))?;
            }
            self.write_field_value_last(
                last_index,
                &vals[last_index],
                formats.map(|fs| &fs[last_index]),
            )
        }
    }

    fn write_seq(&mut self, vals: &[Value], format: Option<&Format>) -> Result<(), io::Error> {
        if vals.is_empty() {
            write!(&mut self.writer, "[]")
        } else {
            match self.preview_len {
                Some(preview_len) if vals.len() > preview_len => {
                    let last_index = vals.len() - 1;
                    for (index, val) in vals[0..preview_len].iter().enumerate() {
                        self.write_field_value_continue(index, val, format)?;
                    }
                    if preview_len != last_index {
                        self.write_field_skipped()?;
                    }
                    self.write_field_value_last(last_index, &vals[last_index], format)
                }
                Some(_) | None => {
                    let last_index = vals.len() - 1;
                    for (index, val) in vals[..last_index].iter().enumerate() {
                        self.write_field_value_continue(index, val, format)?;
                    }
                    self.write_field_value_last(last_index, &vals[last_index], format)
                }
            }
        }
    }

    fn write_seq_records(&mut self, vals: &[Value], format: &Format) -> Result<(), io::Error> {
        let fields = self.is_record_with_atomic_fields(format).unwrap();
        let mut cols = Vec::new();
        let mut header = Vec::new();
        for (label, _) in fields {
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
                panic!("expected record value");
            }
            rows.push(row);
        }
        self.write_table(&cols, &header, &rows)
    }

    fn write_table(
        &mut self,
        cols: &[usize],
        header: &[String],
        rows: &[Vec<String>],
    ) -> Result<(), io::Error> {
        self.write_gutter()?;
        write!(&mut self.writer, "└── ")?;
        for (i, th) in header.iter().enumerate() {
            write!(&mut self.writer, " {:>width$}", th, width = cols[i])?;
        }
        writeln!(&mut self.writer)?;
        self.gutter.push(Column::Space);
        for tr in rows {
            self.write_gutter()?;
            for (i, td) in tr.iter().enumerate() {
                write!(&mut self.writer, " {:>width$}", td, width = cols[i])?;
            }
            writeln!(&mut self.writer)?;
        }
        self.gutter.pop();
        Ok(())
    }

    fn is_implied_value_format(&self, format: &Format) -> bool {
        match format {
            Format::ItemVar(level, _args) => {
                self.is_implied_value_format(self.module.get_format(*level))
            }
            Format::EndOfInput => true,
            Format::Byte(bs) => bs.len() == 1,
            Format::Tuple(fields) => fields.iter().all(|f| self.is_implied_value_format(f)),
            Format::Record(fields) => fields.iter().all(|(_, f)| self.is_implied_value_format(f)),
            _ => false,
        }
    }

    fn is_ascii_string_format(&self, format: &Format) -> bool {
        match format {
            Format::ItemVar(level, _args) => {
                self.module.get_name(*level) == "base.asciiz-string"
                    || self.is_ascii_string_format(self.module.get_format(*level))
            }
            Format::Tuple(formats) => self.is_ascii_tuple_format(formats),
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::RepeatCount(_, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => self.is_ascii_char_format(format),
            _ => false,
        }
    }

    fn is_ascii_tuple_format(&self, formats: &[Format]) -> bool {
        !formats.is_empty() && formats.iter().all(|f| self.is_ascii_char_format(f))
    }

    fn is_ascii_char_format(&self, format: &Format) -> bool {
        match format {
            Format::ItemVar(level, _args) => self.module.get_name(*level) == "base.ascii-char",
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
    ) -> Option<&'a [(String, Format)]> {
        match format {
            Format::ItemVar(level, _args) => {
                self.is_record_with_atomic_fields(self.module.get_format(*level))
            }
            Format::Record(fields) => {
                if fields.iter().all(|(_l, f)| self.is_atomic_format(f)) {
                    Some(fields)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn write_record(
        &mut self,
        value_fields: &[(String, Value)],
        format_fields: Option<&[(String, Format)]>,
    ) -> Result<(), io::Error> {
        if value_fields.is_empty() {
            write!(&mut self.writer, "{{}}")
        } else if let Some((_, v)) = value_fields.iter().find(|(label, _)| label == "@value") {
            self.write_value(v)
        } else {
            let initial_len = self.scope.len();
            let last_index = value_fields.len() - 1;
            for (index, (label, value)) in value_fields[..last_index].iter().enumerate() {
                let format = format_fields.map(|fs| &fs[index].1);
                self.write_field_value_continue(label, value, format)?;
                self.scope.push(label.clone(), value.clone());
            }
            let (label, value) = &value_fields[last_index];
            let format = format_fields.map(|fs| &fs[last_index].1);
            self.write_field_value_last(label, value, format)?;
            self.scope.truncate(initial_len);
            Ok(())
        }
    }

    fn write_variant(
        &mut self,
        label: &str,
        value: &Value,
        format: Option<&Format>,
    ) -> io::Result<()> {
        if self.is_atomic_value(value, format) {
            write!(&mut self.writer, "{{ {label} := ")?;
            self.write_value(value)?;
            write!(&mut self.writer, " }}")
            // TODO: write format
        } else {
            self.write_field_value_last(label, value, format)
        }
    }

    fn write_gutter(&mut self) -> io::Result<()> {
        for column in &self.gutter {
            match column {
                Column::Branch => write!(&mut self.writer, "│   ")?,
                Column::Space => write!(&mut self.writer, "    ")?,
            }
        }
        Ok(())
    }

    fn write_field_value_continue(
        &mut self,
        label: impl fmt::Display,
        value: &Value,
        format: Option<&Format>,
    ) -> io::Result<()> {
        self.write_gutter()?;
        write!(&mut self.writer, "├── {label}")?;
        if let Some(format) = format {
            write!(&mut self.writer, " <- ")?;
            self.write_format(format)?;
        }
        self.gutter.push(Column::Branch);
        self.write_field_value(value, format)?;
        self.gutter.pop();
        Ok(())
    }

    fn write_field_value_last(
        &mut self,
        label: impl fmt::Display,
        value: &Value,
        format: Option<&Format>,
    ) -> io::Result<()> {
        self.write_gutter()?;
        write!(&mut self.writer, "└── {label}")?;
        if let Some(format) = format {
            write!(&mut self.writer, " <- ")?;
            self.write_format(format)?;
        }
        self.gutter.push(Column::Space);
        self.write_field_value(value, format)?;
        self.gutter.pop();
        Ok(())
    }

    fn write_field_value(&mut self, value: &Value, format: Option<&Format>) -> io::Result<()> {
        match format {
            Some(format) => {
                if self.flags.omit_implied_values && self.is_implied_value_format(format) {
                    writeln!(&mut self.writer)
                } else if self.is_atomic_value(value, Some(format)) {
                    write!(&mut self.writer, " := ")?;
                    self.write_decoded_value(value, format)?;
                    writeln!(&mut self.writer)
                } else {
                    writeln!(&mut self.writer, " :=")?;
                    self.write_decoded_value(value, format)
                }
            }
            None => {
                if self.is_atomic_value(value, None) {
                    write!(&mut self.writer, " := ")?;
                    self.write_value(value)?;
                    writeln!(&mut self.writer)
                } else {
                    writeln!(&mut self.writer, " :=")?;
                    self.write_value(value)
                }
            }
        }
    }

    fn write_field_skipped(&mut self) -> io::Result<()> {
        self.write_gutter()?;
        writeln!(&mut self.writer, "~")
    }

    fn is_atomic_value(&self, value: &Value, format: Option<&Format>) -> bool {
        if let Some(format) = format {
            if self.flags.pretty_ascii_strings && self.is_ascii_string_format(format) {
                return true;
            }
        }
        match value {
            Value::Bool(_) => true,
            Value::U8(_) => true,
            Value::U16(_) => true,
            Value::U32(_) => true,
            Value::Tuple(values) => values.is_empty(),
            Value::Record(fields) => {
                fields.is_empty()
                    || (self.flags.collapse_computed_values
                        && fields
                            .iter()
                            .find(|(label, _)| label == "@value")
                            .map(|(_, value)| self.is_atomic_value(value, None))
                            .unwrap_or(false))
            }
            Value::Seq(values) => values.is_empty(),
            Value::Variant(_, value) => self.is_atomic_value(value, None),
        }
    }

    fn write_expr(&mut self, expr: &Expr) -> io::Result<()> {
        match expr {
            Expr::Match(head, _) => {
                write!(&mut self.writer, "match ")?;
                self.write_proj_expr(head)?;
                write!(&mut self.writer, " {{ ... }}")
            }
            Expr::Lambda(name, expr) => {
                write!(&mut self.writer, "{name} -> ")?;
                self.write_expr(expr)
            }

            Expr::BitAnd(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " & ")?;
                self.write_proj_expr(expr1)
            }
            Expr::BitOr(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " | ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Eq(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " == ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Ne(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " != ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Lt(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " < ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Gt(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " > ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Lte(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " <= ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Gte(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " >= ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Add(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " + ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Sub(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " - ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Shl(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " << ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Shr(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " >> ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Mul(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " * ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Div(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " / ")?;
                self.write_proj_expr(expr1)
            }
            Expr::Rem(expr0, expr1) => {
                self.write_proj_expr(expr0)?;
                write!(&mut self.writer, " % ")?;
                self.write_proj_expr(expr1)
            }

            Expr::AsU8(expr) => {
                write!(&mut self.writer, "as-u8 ")?;
                self.write_proj_expr(expr)
            }
            Expr::AsU16(expr) => {
                write!(&mut self.writer, "as-u16 ")?;
                self.write_proj_expr(expr)
            }
            Expr::AsU32(expr) => {
                write!(&mut self.writer, "as-u32 ")?;
                self.write_proj_expr(expr)
            }
            Expr::U16Be(bytes) => {
                write!(&mut self.writer, "u16be ")?;
                self.write_proj_expr(bytes)
            }
            Expr::U16Le(bytes) => {
                write!(&mut self.writer, "u16le ")?;
                self.write_proj_expr(bytes)
            }
            Expr::U32Be(bytes) => {
                write!(&mut self.writer, "u32be ")?;
                self.write_proj_expr(bytes)
            }
            Expr::U32Le(bytes) => {
                write!(&mut self.writer, "u32le ")?;
                self.write_proj_expr(bytes)
            }
            Expr::SeqLength(seq) => {
                write!(&mut self.writer, "seq-length ")?;
                self.write_proj_expr(seq)
            }
            Expr::SubSeq(seq, start, length) => {
                write!(&mut self.writer, "sub-seq (")?;
                self.write_expr(start)?;
                write!(&mut self.writer, ", ")?;
                self.write_expr(length)?;
                write!(&mut self.writer, ") ")?;
                self.write_proj_expr(seq)
            }
            Expr::FlatMap(expr, seq) => {
                write!(&mut self.writer, "flat-map (")?;
                self.write_expr(expr)?;
                write!(&mut self.writer, ") ")?;
                self.write_proj_expr(seq)
            }
            Expr::FlatMapAccum(expr, accum, _accum_type, seq) => {
                write!(&mut self.writer, "flat-map-accum (")?;
                self.write_expr(expr)?;
                write!(&mut self.writer, ", ")?;
                self.write_expr(accum)?;
                write!(&mut self.writer, ") ")?;
                self.write_proj_expr(seq)
            }
            Expr::Dup(count, expr) => {
                write!(&mut self.writer, "dup (")?;
                self.write_expr(count)?;
                write!(&mut self.writer, ") ")?;
                self.write_proj_expr(expr)
            }
            Expr::Inflate(expr) => {
                write!(&mut self.writer, "inflate ")?;
                self.write_proj_expr(expr)
            }

            expr => self.write_proj_expr(expr),
        }
    }

    fn write_proj_expr(&mut self, expr: &Expr) -> io::Result<()> {
        match expr {
            Expr::TupleProj(head, index) => {
                self.write_proj_expr(head)?;
                write!(&mut self.writer, ".{index}")
            }
            Expr::RecordProj(head, label) => {
                self.write_proj_expr(head)?;
                write!(&mut self.writer, ".{label}")
            }
            expr => self.write_atomic_expr(expr),
        }
    }

    fn write_atomic_expr(&mut self, expr: &Expr) -> io::Result<()> {
        match expr {
            Expr::Var(name) => {
                write!(&mut self.writer, "{name}")
            }
            Expr::Bool(b) => write!(&mut self.writer, "{b}"),
            Expr::U8(i) => write!(&mut self.writer, "{i}"),
            Expr::U16(i) => write!(&mut self.writer, "{i}"),
            Expr::U32(i) => write!(&mut self.writer, "{i}"),
            Expr::Tuple(..) => write!(&mut self.writer, "(...)"),
            Expr::Record(..) => write!(&mut self.writer, "{{ ... }}"),
            Expr::Variant(label, expr) => {
                write!(&mut self.writer, "{{ {label} := ")?;
                self.write_expr(expr)?;
                write!(&mut self.writer, " }}")
            }
            Expr::Seq(..) => write!(&mut self.writer, "[..]"),
            expr => {
                write!(&mut self.writer, "(")?;
                self.write_expr(expr)?;
                write!(&mut self.writer, ")")
            }
        }
    }

    fn write_format(&mut self, format: &Format) -> io::Result<()> {
        match format {
            Format::Union(_) => write!(&mut self.writer, "_ |...| _"),
            Format::Peek(format) => {
                write!(&mut self.writer, "peek ")?;
                self.write_atomic_format(format)
            }
            Format::Repeat(format) => {
                write!(&mut self.writer, "repeat ")?;
                self.write_atomic_format(format)
            }
            Format::Repeat1(format) => {
                write!(&mut self.writer, "repeat1 ")?;
                self.write_atomic_format(format)
            }
            Format::RepeatCount(len, format) => {
                write!(&mut self.writer, "repeat-count ")?;
                self.write_atomic_expr(len)?;
                write!(&mut self.writer, " ")?;
                self.write_atomic_format(format)
            }
            Format::RepeatUntilLast(len, format) => {
                write!(&mut self.writer, "repeat-until-last ")?;
                self.write_atomic_expr(len)?;
                write!(&mut self.writer, " ")?;
                self.write_atomic_format(format)
            }
            Format::RepeatUntilSeq(len, format) => {
                write!(&mut self.writer, "repeat-until-seq ")?;
                self.write_atomic_expr(len)?;
                write!(&mut self.writer, " ")?;
                self.write_atomic_format(format)
            }
            Format::Slice(len, format) => {
                write!(&mut self.writer, "slice ")?;
                self.write_atomic_expr(len)?;
                write!(&mut self.writer, " ")?;
                self.write_atomic_format(format)
            }
            Format::FixedSlice(size, format) => {
                write!(&mut self.writer, "slice {size} ")?;
                self.write_atomic_format(format)
            }
            Format::Bits(format) => {
                write!(&mut self.writer, "bits ")?;
                self.write_atomic_format(format)
            }
            Format::WithRelativeOffset(offset, format) => {
                write!(&mut self.writer, "with-relative-offset ")?;
                self.write_atomic_expr(offset)?;
                write!(&mut self.writer, " ")?;
                self.write_atomic_format(format)
            }

            Format::Compute(expr) => {
                write!(&mut self.writer, "compute ")?;
                self.write_expr(expr)
            }

            Format::Match(head, _) | Format::MatchVariant(head, _) => {
                write!(&mut self.writer, "match ")?;
                self.write_proj_expr(head)?;
                write!(&mut self.writer, " {{ ... }}")
            }

            Format::Dynamic(_) => {
                write!(&mut self.writer, "dynamic")
            }

            format => self.write_atomic_format(format),
        }
    }

    fn write_atomic_format(&mut self, format: &Format) -> io::Result<()> {
        match format {
            Format::ItemVar(var, args) => {
                write!(&mut self.writer, "{}", self.module.get_name(*var))?;
                if !args.is_empty() {
                    write!(&mut self.writer, "(...)")?;
                }
                Ok(())
            }
            Format::Fail => write!(&mut self.writer, "fail"),
            Format::EndOfInput => write!(&mut self.writer, "end-of-input"),
            Format::Align(n) => write!(&mut self.writer, "align {n}"),

            Format::Byte(bs) => {
                if bs.len() < 128 {
                    write!(&mut self.writer, "[=")?;
                    for b in bs.iter() {
                        write!(&mut self.writer, " {b}")?;
                    }
                    write!(&mut self.writer, "]")
                } else {
                    write!(&mut self.writer, "[!=")?;
                    for b in (!bs).iter() {
                        write!(&mut self.writer, " {b}")?;
                    }
                    write!(&mut self.writer, "]")
                }
            }

            Format::Tuple(formats) if formats.is_empty() => write!(&mut self.writer, "()"),
            Format::Tuple(_) => write!(&mut self.writer, "(...)"),

            Format::Record(fields) if fields.is_empty() => write!(&mut self.writer, "{{}}"),
            Format::Record(_) => write!(&mut self.writer, "{{ ... }}"),

            format => {
                write!(&mut self.writer, "(")?;
                self.write_format(format)?;
                write!(&mut self.writer, ")")
            }
        }
    }
}

pub struct MonoidalPrinter<'module> {
    gutter: Vec<Column>,
    preview_len: Option<usize>,
    flags: Flags,
    module: &'module FormatModule,
    scope: Scope,
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
            _ => false,
        }
    }

    fn is_ascii_string_format(&self, format: &Format) -> bool {
        match format {
            Format::ItemVar(level, _args) => {
                self.module.get_name(*level) == "base.asciiz-string"
                    || self.is_ascii_string_format(self.module.get_format(*level))
            }
            Format::Tuple(formats) => self.is_ascii_tuple_format(formats),
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::RepeatCount(_, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => self.is_ascii_char_format(format),
            _ => false,
        }
    }

    fn is_ascii_tuple_format(&self, formats: &[Format]) -> bool {
        !formats.is_empty() && formats.iter().all(|f| self.is_ascii_char_format(f))
    }

    fn is_ascii_char_format(&self, format: &Format) -> bool {
        match format {
            Format::ItemVar(level, _args) => {
                self.module.get_name(*level).starts_with("base.ascii-char")
            }
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
    ) -> Option<&'a [(String, Format)]> {
        match format {
            Format::ItemVar(level, _args) => {
                self.is_record_with_atomic_fields(self.module.get_format(*level))
            }
            Format::Record(fields) => {
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
            if self.flags.pretty_ascii_strings && self.is_ascii_string_format(format) {
                return true;
            }
        }
        match value {
            Value::Bool(_) => true,
            Value::U8(_) | Value::U16(_) | Value::U32(_) => true,
            Value::Tuple(values) => values.is_empty(),
            Value::Record(fields) => {
                fields.is_empty()
                    || (self.flags.collapse_computed_values
                        && fields
                            .iter()
                            .find_map(|(label, value)| {
                                (label == "@value").then(|| self.is_atomic_value(value, None))
                            })
                            .unwrap_or(false))
            }
            Value::Seq(values) => values.is_empty(),
            Value::Variant(_, value) => self.is_atomic_value(value, None),
        }
    }
}

impl<'module> MonoidalPrinter<'module> {
    pub fn new(module: &'module FormatModule) -> MonoidalPrinter<'module> {
        let flags = Flags {
            collapse_computed_values: true,
            omit_implied_values: true,
            tables_for_record_sequences: true,
            pretty_ascii_strings: true,
        };
        MonoidalPrinter {
            gutter: Vec::new(),
            preview_len: Some(10),
            flags,
            module,
            scope: Scope::new(),
        }
    }

    pub fn compile_decoded_value(&mut self, value: &Value, fmt: &Format) -> Fragment {
        let mut frag = Fragment::Empty;
        match fmt {
            Format::ItemVar(level, _args) => {
                if self.flags.pretty_ascii_strings
                    && self.module.get_name(*level) == "base.asciiz-string"
                {
                    self.compile_ascii_string(value)
                } else if self.flags.pretty_ascii_strings
                    && self.module.get_name(*level) == "base.ascii-char"
                {
                    frag.encat(Fragment::Char('\''));
                    frag.encat(self.compile_ascii_char(value));
                    frag.encat(Fragment::Char('\''));
                    frag
                } else {
                    self.compile_decoded_value(value, self.module.get_format(*level))
                }
            }
            Format::Fail => panic!("uninhabited format (value={value:?}"),
            Format::EndOfInput => self.compile_value(value),
            Format::Align(_) => self.compile_value(value),
            Format::Byte(_) => self.compile_value(value),
            Format::Union(branches) => match value {
                Value::Variant(label, value) => {
                    let (_, format) = branches.iter().find(|(l, _)| l == label).unwrap();
                    self.compile_variant(label, value, Some(format))
                }
                _ => panic!("expected variant, found {value:?}"),
            },
            Format::Tuple(formats) => match value {
                Value::Tuple(values) => {
                    if self.flags.pretty_ascii_strings && self.is_ascii_tuple_format(formats) {
                        self.compile_ascii_seq(values)
                    } else {
                        self.compile_tuple(values, Some(formats))
                    }
                }
                _ => panic!("expected tuple, found {value:?}"),
            },
            Format::Record(format_fields) => match value {
                Value::Record(value_fields) => {
                    self.compile_record(value_fields, Some(format_fields))
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
                    } else if self.flags.pretty_ascii_strings && self.is_ascii_char_format(format) {
                        self.compile_ascii_seq(values)
                    } else {
                        self.compile_seq(values, Some(format))
                    }
                }
                _ => panic!("expected sequence, found {value:?}"),
            },
            Format::Peek(format) => self.compile_decoded_value(value, format),
            Format::Slice(_, format) | Format::FixedSlice(_, format) => {
                self.compile_decoded_value(value, format)
            }
            Format::Bits(format) => self.compile_decoded_value(value, format),
            Format::WithRelativeOffset(_, format) => self.compile_decoded_value(value, format),
            Format::Compute(_expr) => self.compile_value(value),
            Format::Match(head, branches) => {
                let head = head.eval(&mut self.scope);
                let initial_len = self.scope.len();
                let (_, format) = branches
                    .iter()
                    .find(|(pattern, _)| head.matches(&mut self.scope, pattern))
                    .expect("exhaustive patterns");
                frag.encat(self.compile_decoded_value(value, format));
                self.scope.truncate(initial_len);
                frag
            }
            Format::MatchVariant(head, branches) => {
                let head = head.eval(&mut self.scope);
                let initial_len = self.scope.len();
                let (_, _label, format) = branches
                    .iter()
                    .find(|(pattern, _, _)| head.matches(&mut self.scope, pattern))
                    .expect("exhaustive patterns");
                if let Value::Variant(_label, value) = value {
                    frag.encat(self.compile_decoded_value(value, format));
                } else {
                    panic!("expected variant value");
                }
                self.scope.truncate(initial_len);
                frag
            }
            Format::Dynamic(_) => self.compile_value(value),
        }
    }

    pub fn compile_value(&mut self, value: &Value) -> Fragment {
        match value {
            Value::Bool(true) => Fragment::String("true".into()),
            Value::Bool(false) => Fragment::String("false".into()),
            Value::U8(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::U16(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::U32(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::Tuple(vals) => self.compile_tuple(vals, None),
            Value::Seq(vals) => self.compile_seq(vals, None),
            Value::Record(fields) => self.compile_record(fields, None),
            Value::Variant(label, value) => self.compile_variant(label, value, None),
        }
    }

    pub fn compile_ascii_string(&self, value: &Value) -> Fragment {
        let vs = match value {
            Value::Record(fields) => {
                match fields
                    .iter()
                    .find(|(label, _)| label == "string")
                    .unwrap_or_else(|| unreachable!("no string field"))
                {
                    (_, Value::Seq(vs)) => vs,
                    (_, v) => panic!("expected sequence value, found {v:?}"),
                }
            }
            _ => panic!("expected record value, found {value:?}"),
        };
        self.compile_ascii_seq(vs)
    }

    fn compile_ascii_seq(&self, vals: &[Value]) -> Fragment {
        let mut frag = Fragment::new();
        frag.encat(Fragment::Char('"'));
        for v in vals {
            frag.encat(self.compile_ascii_char(v));
        }
        frag.encat(Fragment::Char('"'));
        frag
    }

    fn compile_ascii_char(&self, v: &Value) -> Fragment {
        let b = match v {
            Value::U8(b) => *b,
            _ => panic!("expected U8 value, found {v:?}"),
        };
        match b {
            0x00 => Fragment::String("\\0".into()),
            0x09 => Fragment::String("\\t".into()),
            0x0a => Fragment::String("\\n".into()),
            0x0d => Fragment::String("\\r".into()),
            32..=127 => Fragment::Char(b as char),
            _ => Fragment::String(format!("\\x{b:02X}").into()),
        }
    }

    fn compile_tuple(&mut self, vals: &[Value], formats: Option<&[Format]>) -> Fragment {
        if vals.is_empty() {
            Fragment::String("()".into())
        } else {
            let mut frag = Fragment::new();
            let last_index = vals.len() - 1;
            for index in 0..last_index {
                frag.encat(self.compile_field_value_continue(
                    index,
                    &vals[index],
                    formats.map(|fs| &fs[index]),
                ));
            }
            frag.encat(self.compile_field_value_last(
                last_index,
                &vals[last_index],
                formats.map(|fs| &fs[last_index]),
            ));
            frag
        }
    }

    fn compile_seq(&mut self, vals: &[Value], format: Option<&Format>) -> Fragment {
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
                frag.encat(self.compile_field_value_continue(index, val, format));
            }
            if any_skipped {
                frag.encat(self.compile_field_skipped());
            }
            frag.encat(self.compile_field_value_last(last_index, &vals[last_index], format));
            frag
        }
    }

    fn compile_seq_records(&mut self, vals: &[Value], format: &Format) -> Fragment {
        let fields = self.is_record_with_atomic_fields(format).unwrap();
        let mut cols = Vec::new();
        let mut header = Vec::new();
        for (label, _) in fields {
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
        header: &[String],
        rows: &[Vec<String>],
    ) -> Fragment {
        let mut frags = Fragments::new();
        let frag = frags.active_mut();
        frag.encat(self.compile_gutter());
        frag.encat(Fragment::String("└── ".into()));
        for (i, th) in header.iter().enumerate() {
            frag.encat(Fragment::String(
                format!(" {:>width$}", th, width = cols[i]).into(),
            ));
        }
        frag.engroup().enbreak();
        let mut frag = frags.renew();
        self.gutter.push(Column::Space);
        for tr in rows {
            frag.encat(self.compile_gutter());
            for (i, td) in tr.iter().enumerate() {
                frag.encat(Fragment::String(
                    format!(" {:>width$}", td, width = cols[i]).into(),
                ));
            }
            frag.engroup().enbreak();
            frag = frags.renew();
        }
        self.gutter.pop();
        frags.finalize()
    }

    fn compile_record(
        &mut self,
        value_fields: &[(String, Value)],
        format_fields: Option<&[(String, Format)]>,
    ) -> Fragment {
        if value_fields.is_empty() {
            Fragment::String("{}".into())
        } else if let Some((_, v)) = value_fields.iter().find(|(label, _)| label == "@value") {
            self.compile_value(v)
        } else {
            let mut frag = Fragment::new();
            let initial_len = self.scope.len();
            let last_index = value_fields.len() - 1;
            for (index, (label, value)) in value_fields[..last_index].iter().enumerate() {
                let format = format_fields.map(|fs| &fs[index].1);
                frag.encat(self.compile_field_value_continue(label, value, format));
                self.scope.push(label.clone(), value.clone());
            }
            let (label, value) = &value_fields[last_index];
            let format = format_fields.map(|fs| &fs[last_index].1);
            frag.encat(self.compile_field_value_last(label, value, format));
            self.scope.truncate(initial_len);
            frag
        }
    }

    fn compile_variant(&mut self, label: &str, value: &Value, format: Option<&Format>) -> Fragment {
        if self.is_atomic_value(value, format) {
            let mut frag = Fragment::new();
            frag.encat(Fragment::String(format!("{{ {label} := ").into()));
            frag.encat(self.compile_value(value));
            frag.encat(Fragment::String(" }}".into()));
            frag.engroup();
            frag
            // TODO [inherited, possibly inaccurate] write format
        } else {
            self.compile_field_value_last(label, value, format)
        }
    }

    fn compile_gutter(&self) -> Fragment {
        let mut frags = Fragments::new();
        for column in &self.gutter {
            match column {
                Column::Branch => frags.push(Fragment::String("│   ".into())),
                Column::Space => frags.push(Fragment::String("    ".into())),
            }
        }
        frags.finalize()
    }

    fn compile_field_value_continue(
        &mut self,
        label: impl fmt::Display,
        value: &Value,
        format: Option<&Format>,
    ) -> Fragment {
        let mut frags = Fragments::new();
        frags.push(self.compile_gutter());
        frags.push(Fragment::String(format!("├── {label}").into()));
        if let Some(format) = format {
            frags.push(Fragment::String(" <- ".into()));
            frags.push(self.compile_format(format, 0));
        }
        self.gutter.push(Column::Branch);
        frags.push(self.compile_field_value(value, format));
        self.gutter.pop();
        frags.finalize().group()
    }

    fn compile_field_value_last(
        &mut self,
        label: impl fmt::Display,
        value: &Value,
        format: Option<&Format>,
    ) -> Fragment {
        let mut frags = Fragments::new();
        frags.push(self.compile_gutter());
        frags.push(Fragment::String(format!("└── {label}").into()));
        if let Some(format) = format {
            frags.push(Fragment::String(" <- ".into()));
            frags.push(self.compile_format(format, 0));
        }
        self.gutter.push(Column::Space);
        frags.push(self.compile_field_value(value, format));
        self.gutter.pop();
        frags.finalize().group()
    }

    fn compile_field_value(&mut self, value: &Value, format: Option<&Format>) -> Fragment {
        match format {
            Some(format) => {
                if self.flags.omit_implied_values && self.is_implied_value_format(format) {
                    Fragment::Char('\n')
                } else if self.is_atomic_value(value, Some(format)) {
                    Fragment::seq(
                        [
                            Fragment::String(" := ".into()),
                            self.compile_decoded_value(value, format),
                            Fragment::Char('\n'),
                        ],
                        None,
                    )
                    .group()
                } else {
                    Fragment::seq(
                        [
                            Fragment::String(" :=\n".into()),
                            self.compile_decoded_value(value, format),
                        ],
                        None,
                    )
                    .group()
                }
            }
            None => {
                if self.is_atomic_value(value, None) {
                    Fragment::seq(
                        [
                            Fragment::String(" := ".into()),
                            self.compile_value(value),
                            Fragment::Char('\n'),
                        ],
                        None,
                    )
                    .group()
                } else {
                    Fragment::seq(
                        [Fragment::String(" :=\n".into()), self.compile_value(value)],
                        None,
                    )
                    .group()
                }
            }
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
        Fragment::seq(
            [
                self.compile_expr(lhs, lhs_prec),
                self.compile_expr(rhs, rhs_prec),
            ],
            Some(Fragment::String(op.into())),
        )
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
        let mut frags = Fragments::new();

        frags.push(Fragment::String(op.into()));
        match args {
            None => (),
            Some(args) => {
                let frag = frags.active_mut();
                frag.encat(Fragment::Char('('));
                frag.encat(Fragment::seq(
                    args.into_iter()
                        .map(|arg| self.compile_expr(arg, PREC_ATOM))
                        .collect::<Vec<_>>(),
                    Some(Fragment::String(", ".into())),
                ));
                frag.encat(Fragment::Char(')'));
            }
        }
        frags.push(self.compile_expr(operand, PREC_ATOM));
        frags.finalize_with_sep(Fragment::Char(' '))
    }

    fn compile_expr(&mut self, expr: &Expr, prec: Precedence) -> Fragment {
        match expr {
            Expr::Match(head, _) => cond_paren(
                Fragment::seq(
                    [
                        Fragment::String("match ".into()),
                        self.compile_expr(head, prec + 1),
                        Fragment::String(" { ... }".into()),
                    ],
                    None,
                )
                .group(),
                prec > PREC_MATCH,
            ),
            Expr::Lambda(name, expr) => cond_paren(
                Fragment::seq(
                    [
                        Fragment::String(format!("{name} -> ").into()),
                        self.compile_expr(expr, prec + 1),
                    ],
                    None,
                )
                .group(),
                prec > PREC_ARROW,
            ),
            Expr::BitAnd(lhs, rhs) => cond_paren(
                self.compile_binop(" & ", lhs, rhs, prec, prec + 1),
                prec > PREC_BITAND,
            ),
            Expr::BitOr(lhs, rhs) => cond_paren(
                self.compile_binop(" | ", lhs, rhs, prec, prec + 1),
                prec > PREC_BITOR,
            ),
            Expr::Eq(lhs, rhs) => cond_paren(
                self.compile_binop(" == ", lhs, rhs, prec + 1, prec + 1),
                prec > PREC_COMPARISON,
            ),
            Expr::Ne(lhs, rhs) => cond_paren(
                self.compile_binop(" != ", lhs, rhs, prec + 1, prec + 1),
                prec > PREC_COMPARISON,
            ),
            Expr::Lt(lhs, rhs) => cond_paren(
                self.compile_binop(" < ", lhs, rhs, prec + 1, prec + 1),
                prec > PREC_COMPARISON,
            ),
            Expr::Gt(lhs, rhs) => cond_paren(
                self.compile_binop(" > ", lhs, rhs, prec + 1, prec + 1),
                prec > PREC_COMPARISON,
            ),
            Expr::Lte(lhs, rhs) => cond_paren(
                self.compile_binop(" <= ", lhs, rhs, prec + 1, prec + 1),
                prec > PREC_COMPARISON,
            ),
            Expr::Gte(lhs, rhs) => cond_paren(
                self.compile_binop(" >= ", lhs, rhs, prec + 1, prec + 1),
                prec > PREC_COMPARISON,
            ),
            Expr::Add(lhs, rhs) => cond_paren(
                self.compile_binop(" + ", lhs, rhs, prec, prec + 1),
                prec > PREC_ADDSUB,
            ),
            Expr::Sub(lhs, rhs) => cond_paren(
                self.compile_binop(" - ", lhs, rhs, prec, prec + 1),
                prec > PREC_ADDSUB,
            ),
            Expr::Shl(lhs, rhs) => cond_paren(
                self.compile_binop(" << ", lhs, rhs, prec, prec + 1),
                prec > PREC_BITSHIFT,
            ),
            Expr::Shr(lhs, rhs) => cond_paren(
                self.compile_binop(" >> ", lhs, rhs, prec, prec + 1),
                prec > PREC_BITSHIFT,
            ),
            Expr::Div(lhs, rhs) => cond_paren(
                self.compile_binop(" / ", lhs, rhs, prec, prec + 1),
                prec > PREC_DIVREM,
            ),
            Expr::Rem(lhs, rhs) => cond_paren(
                self.compile_binop(" % ", lhs, rhs, prec, prec + 1),
                prec > PREC_DIVREM,
            ),

            Expr::AsU8(expr) => cond_paren(
                self.compile_prefix("as-u8", None, expr),
                prec > PREC_NON_INFIX,
            ),
            Expr::AsU16(expr) => cond_paren(
                self.compile_prefix("as-u16", None, expr),
                prec > PREC_NON_INFIX,
            ),
            Expr::AsU32(expr) => cond_paren(
                self.compile_prefix("as-u32", None, expr),
                prec > PREC_NON_INFIX,
            ),
            Expr::U16Be(bytes) => cond_paren(
                self.compile_prefix("u16be", None, bytes),
                prec > PREC_NON_INFIX,
            ),
            Expr::U16Le(bytes) => cond_paren(
                self.compile_prefix("u16le", None, bytes),
                prec > PREC_NON_INFIX,
            ),
            Expr::U32Be(bytes) => cond_paren(
                self.compile_prefix("u32be", None, bytes),
                prec > PREC_NON_INFIX,
            ),
            Expr::U32Le(bytes) => cond_paren(
                self.compile_prefix("u32le", None, bytes),
                prec > PREC_NON_INFIX,
            ),
            Expr::SeqLength(seq) => cond_paren(
                self.compile_prefix("seq-length", None, seq),
                prec > PREC_NON_INFIX,
            ),
            Expr::SubSeq(seq, start, length) => cond_paren(
                self.compile_prefix("sub-seq", Some(&[&start, &length]), seq),
                prec > PREC_NON_INFIX,
            ),
            Expr::FlatMap(expr, seq) => cond_paren(
                self.compile_prefix("flat-map", Some(&[&expr]), seq),
                prec > PREC_NON_INFIX,
            ),
            Expr::FlatMapAccum(expr, accum, _accum_type, seq) => cond_paren(
                self.compile_prefix("flat-map-accum", Some(&[&expr, &accum]), seq),
                prec > PREC_NON_INFIX,
            ),
            Expr::Dup(count, expr) => cond_paren(
                self.compile_prefix("dup", Some(&[&count]), expr),
                prec > PREC_NON_INFIX,
            ),
            Expr::Inflate(expr) => cond_paren(
                self.compile_prefix("inflate", None, expr),
                prec > PREC_NON_INFIX,
            ),

            Expr::TupleProj(head, index) => cond_paren(
                Fragment::seq(
                    [
                        self.compile_expr(head, prec + 1),
                        Fragment::Char('.'),
                        Fragment::DisplayAtom(Rc::new(*index)),
                    ],
                    None,
                )
                .group(),
                prec > PREC_PROJ,
            ),
            Expr::RecordProj(head, label) => cond_paren(
                Fragment::seq(
                    [
                        self.compile_expr(head, prec + 1),
                        Fragment::Char('.'),
                        Fragment::String(label.clone().into()),
                    ],
                    None,
                )
                .group(),
                prec > PREC_PROJ,
            ),
            Expr::Var(name) => Fragment::String(name.clone().into()),
            Expr::Bool(b) => Fragment::DisplayAtom(Rc::new(*b)),
            Expr::U8(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::U16(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::U32(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::Tuple(..) => Fragment::String("(...)".into()),
            Expr::Record(..) => Fragment::String("{ ... }".into()),
            Expr::Variant(label, expr) => {
                let mut frag = Fragment::new();
                frag.encat(Fragment::String(format!("{{ {label} := ").into()));
                frag.encat(self.compile_expr(expr, 0));
                frag.encat(Fragment::String(" }".into()));
                frag.engroup();
                frag
            }
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
        let mut frags = Fragments::new();
        frags.push(Fragment::String(label.into()));
        if let Some(args) = args {
            for arg in args.into_iter() {
                frags.push(arg.clone());
            }
        }
        frags.push(self.compile_format(inner, prec + 1));
        frags.finalize_with_sep(Fragment::Char(' '))
    }

    fn compile_format(&mut self, format: &Format, prec: Precedence) -> Fragment {
        match format {
            Format::Union(_) => cond_paren(
                Fragment::String("_ |...| _".into()),
                prec > PREC_FORMAT_COMPOUND,
            ),
            Format::Peek(format) => cond_paren(
                self.compile_nested_format("peek", None, format, prec),
                prec > PREC_FORMAT_COMPOUND,
            ),
            Format::Repeat(format) => cond_paren(
                self.compile_nested_format("repeat", None, format, prec),
                prec > PREC_FORMAT_COMPOUND,
            ),
            Format::Repeat1(format) => cond_paren(
                self.compile_nested_format("repeat1", None, format, prec),
                prec > PREC_FORMAT_COMPOUND,
            ),
            Format::RepeatCount(len, format) => {
                let expr_frag = self.compile_expr(len, PREC_ATOM);
                cond_paren(
                    self.compile_nested_format("repeat-count", Some(&[expr_frag]), format, prec),
                    prec > PREC_FORMAT_COMPOUND,
                )
            }
            Format::RepeatUntilLast(expr, format) => {
                let expr_frag = self.compile_expr(expr, PREC_ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "repeat-until-last",
                        Some(&[expr_frag]),
                        format,
                        prec,
                    ),
                    prec > PREC_FORMAT_COMPOUND,
                )
            }
            Format::RepeatUntilSeq(expr, format) => {
                let expr_frag = self.compile_expr(expr, PREC_ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "repeat-until-seq",
                        Some(&[expr_frag]),
                        format,
                        prec,
                    ),
                    prec > PREC_FORMAT_COMPOUND,
                )
            }
            Format::Slice(len, format) => {
                let expr_frag = self.compile_expr(len, PREC_ATOM);
                cond_paren(
                    self.compile_nested_format("slice", Some(&[expr_frag]), format, prec),
                    prec > PREC_FORMAT_COMPOUND,
                )
            }
            Format::FixedSlice(size, format) => {
                // REVIEW should fixed-size slices display differently from computed slices?
                cond_paren(
                    self.compile_nested_format(
                        "slice",
                        Some(&[Fragment::DisplayAtom(Rc::new(*size))]),
                        format,
                        prec,
                    ),
                    prec > PREC_FORMAT_COMPOUND,
                )
            }
            Format::Bits(format) => cond_paren(
                self.compile_nested_format("bits", None, format, prec),
                prec > PREC_FORMAT_COMPOUND,
            ),
            Format::WithRelativeOffset(offset, format) => {
                let expr_frag = self.compile_expr(offset, PREC_ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "with-relative-offset",
                        Some(&[expr_frag]),
                        format,
                        prec,
                    ),
                    prec > PREC_FORMAT_COMPOUND,
                )
            }
            Format::Compute(expr) => cond_paren(
                Fragment::cat(
                    Fragment::String("compute ".into()),
                    self.compile_expr(expr, 0),
                ),
                prec > PREC_FORMAT_COMPOUND,
            ),
            Format::Match(head, _) | Format::MatchVariant(head, _) => cond_paren(
                Fragment::seq(
                    [
                        Fragment::String("match".into()),
                        self.compile_expr(head, PREC_PROJ),
                        Fragment::String("{ ... }".into()),
                    ],
                    Some(Fragment::Char(' ')),
                ),
                prec > PREC_FORMAT_COMPOUND,
            ),
            Format::Dynamic(_) => Fragment::String("dynamic".into()),

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

            Format::Byte(bs) => {
                if bs.len() < 128 {
                    let mut frags = Fragments::new();
                    frags.push(Fragment::String("[=".into()));
                    for b in bs.iter() {
                        frags.push(Fragment::String(format!(" {b}").into()));
                    }
                    frags.push(Fragment::Char(']'));
                    frags.finalize()
                } else {
                    let mut frags = Fragments::new();
                    frags.push(Fragment::String("[!=".into()));
                    for b in (!bs).iter() {
                        frags.push(Fragment::String(format!(" {b}").into()));
                    }
                    frags.push(Fragment::Char(']'));
                    frags.finalize()
                }
            }
            Format::Tuple(formats) if formats.is_empty() => Fragment::String("()".into()),
            Format::Tuple(_) => Fragment::String("(...)".into()),

            Format::Record(fields) if fields.is_empty() => Fragment::String("{}".into()),
            Format::Record(_) => Fragment::String("{ ... }".into()),
        }
    }
}

/// Operator precedence
type Precedence = u8;

const PREC_ATOM: Precedence = 11;
const PREC_NON_INFIX: Precedence = 10;

const PREC_PROJ: Precedence = 9;

const PREC_BITSHIFT: Precedence = 8;

const PREC_DIVREM: Precedence = 7;
const PREC_BITAND: Precedence = 7;

const PREC_ADDSUB: Precedence = 6;

const PREC_BITOR: Precedence = 5;

const PREC_COMPARISON: Precedence = 4;

const PREC_MATCH: Precedence = 1;
const PREC_ARROW: Precedence = 1;

// Format Precedence

const PREC_FORMAT_COMPOUND: Precedence = 1;
const PREC_FORMAT_ATOM: Precedence = 2;

fn cond_paren(frag: Fragment, should_paren: bool) -> Fragment {
    if should_paren {
        Fragment::seq([Fragment::Char('('), frag, Fragment::Char(')')], None)
    } else {
        frag
    }
}
