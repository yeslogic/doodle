use std::{fmt, io};

use crate::{Expr, Format, FormatModule, Value};

pub fn print_decoded_value(module: &FormatModule, value: &Value, format: &Format) {
    Context::new(io::stdout(), module)
        .write_decoded_value(value, format)
        .unwrap()
}

fn is_atomic_value(value: &Value) -> bool {
    match value {
        Value::Bool(_) => true,
        Value::U8(_) => true,
        Value::U16(_) => true,
        Value::U32(_) => true,
        Value::Tuple(values) => values.is_empty(),
        Value::Record(fields) => fields.is_empty(),
        Value::Seq(values) => values.is_empty(),
        Value::Variant(_, value) => is_atomic_value(value),
    }
}

/// Attempt to recover a format that can be used to render a value that was
/// decoded with a map format. We currently handle cases like:
///
/// - `map (fun x => x) format`
/// - `map (fun x => x.foo) format`
/// - `map (fun x => x.3) format`
/// - `map (fun x => x.foo.bar.3) format`
fn mapped_format<'format>(expr: &Expr, format: &'format Format) -> Option<&'format Format> {
    match expr {
        Expr::Var(0) => Some(format),
        Expr::RecordProj(head, label) => match mapped_format(head, format)? {
            Format::Record(fields) => fields.iter().find_map(|(l, f)| (l == label).then_some(f)),
            _ => None,
        },
        Expr::TupleProj(head, index) => match mapped_format(head, format)? {
            Format::Tuple(formats) => formats.get(*index),
            _ => None,
        },
        _ => None,
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
    module: &'module FormatModule,
    names: Vec<String>,
    values: Vec<Value>,
}

impl<'module, W: io::Write> Context<'module, W> {
    pub fn new(writer: W, module: &'module FormatModule) -> Context<'module, W> {
        Context {
            writer,
            gutter: Vec::new(),
            preview_len: Some(10),
            module,
            names: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn write_decoded_value(&mut self, value: &Value, format: &Format) -> io::Result<()> {
        match format {
            Format::ItemVar(level) => {
                self.write_decoded_value(value, self.module.get_format(*level))
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
                Value::Tuple(values) => self.write_tuple(values, Some(formats)),
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
                Value::Seq(values) => self.write_seq(values, Some(format)),
                _ => panic!("expected sequence"),
            },
            Format::Peek(format) => self.write_decoded_value(value, format),
            Format::Slice(_, format) => self.write_decoded_value(value, format),
            Format::WithRelativeOffset(_, format) => self.write_decoded_value(value, format),
            Format::Map(expr, format) => match mapped_format(expr, format) {
                Some(format) => self.write_decoded_value(value, format),
                None => self.write_value(value),
            },
            Format::Match(head, branches) => {
                let head = head.eval(&mut self.values);
                let initial_len = self.values.len();
                let (_, format) = branches
                    .iter()
                    .find(|(pattern, _)| head.matches(&mut self.values, pattern))
                    .expect("exhaustive patterns");
                for i in 0..(self.values.len() - initial_len) {
                    self.names.push(format!("x{i}")); // TODO: use better names
                }
                self.write_decoded_value(value, format)?;
                self.names.truncate(initial_len);
                self.values.truncate(initial_len);
                Ok(())
            }
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

    fn write_record(
        &mut self,
        value_fields: &[(String, Value)],
        format_fields: Option<&[(String, Format)]>,
    ) -> Result<(), io::Error> {
        if value_fields.is_empty() {
            write!(&mut self.writer, "{{}}")
        } else {
            let initial_len = self.names.len();
            let last_index = value_fields.len() - 1;
            for (index, (label, value)) in value_fields[..last_index].iter().enumerate() {
                let format = format_fields.map(|fs| &fs[index].1);
                self.write_field_value_continue(label, value, format)?;
                self.names.push(label.clone());
                self.values.push(value.clone());
            }
            let (label, value) = &value_fields[last_index];
            let format = format_fields.map(|fs| &fs[last_index].1);
            self.write_field_value_last(label, value, format)?;
            self.names.truncate(initial_len);
            self.values.truncate(initial_len);
            Ok(())
        }
    }

    fn write_variant(
        &mut self,
        label: &str,
        value: &Value,
        format: Option<&Format>,
    ) -> io::Result<()> {
        if is_atomic_value(value) {
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
        write!(&mut self.writer, " :=")?;
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
        write!(&mut self.writer, " :=")?;
        self.gutter.push(Column::Space);
        self.write_field_value(value, format)?;
        self.gutter.pop();
        Ok(())
    }

    fn write_field_value(&mut self, value: &Value, format: Option<&Format>) -> io::Result<()> {
        if is_atomic_value(value) {
            write!(&mut self.writer, " ")?;
            match format {
                Some(format) => self.write_decoded_value(value, format)?,
                None => self.write_value(value)?,
            }
            writeln!(&mut self.writer)
        } else {
            writeln!(&mut self.writer)?;
            match format {
                Some(format) => self.write_decoded_value(value, format),
                None => self.write_value(value),
            }
        }
    }

    fn write_field_skipped(&mut self) -> io::Result<()> {
        self.write_gutter()?;
        writeln!(&mut self.writer, "~")
    }

    fn write_expr(&mut self, expr: &Expr) -> io::Result<()> {
        match expr {
            Expr::Match(head, _) => {
                write!(&mut self.writer, "match ")?;
                self.write_proj_expr(head)?;
                write!(&mut self.writer, " {{ ... }}")
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
            Expr::FlatMapAccum(expr, accum, seq) => {
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
            Expr::Var(index) => {
                let name = &self.names[self.names.len() - index - 1];
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
            Expr::UnwrapVariant(expr) => {
                write!(&mut self.writer, "unwrap(")?;
                self.write_expr(expr)?;
                write!(&mut self.writer, ")")
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
            Format::WithRelativeOffset(offset, format) => {
                write!(&mut self.writer, "with-relative-offset ")?;
                self.write_atomic_expr(offset)?;
                write!(&mut self.writer, " ")?;
                self.write_atomic_format(format)
            }

            Format::Map(expr, format) => {
                let name = "x";
                write!(&mut self.writer, "map (fun {name} => ")?;
                self.names.push(name.to_string());
                // FIXME push value
                self.write_expr(expr)?;
                self.names.pop();
                write!(&mut self.writer, ") ")?;
                self.write_atomic_format(format)
            }

            Format::Match(head, _) => {
                write!(&mut self.writer, "match ")?;
                self.write_proj_expr(head)?;
                write!(&mut self.writer, " {{ ... }}")
            }

            format => self.write_atomic_format(format),
        }
    }

    fn write_atomic_format(&mut self, format: &Format) -> io::Result<()> {
        match format {
            Format::ItemVar(var) => {
                write!(&mut self.writer, "{}", self.module.get_name(*var))
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
