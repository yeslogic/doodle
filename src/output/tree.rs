use std::{fmt, io};

use crate::{Expr, Format, FormatModule, Scope, Value};

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
            Format::Slice(_, format) => self.write_decoded_value(value, format),
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
