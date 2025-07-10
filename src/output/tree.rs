use std::{fmt, io, rc::Rc};

use crate::decoder::SeqKind;
use crate::precedence::{cond_paren, Precedence};
use crate::{
    decoder::Value,
    loc_decoder::{ParseLoc, Parsed, ParsedValue},
};
use crate::{
    Arith, DynFormat, Expr, FieldLabel, Format, FormatModule, IntRel, Pattern, RecordFormat,
    StyleHint, ViewFormat,
};
use crate::{Label, UnaryOp};

use super::{Fragment, FragmentBuilder, Symbol};

fn atomic_value_to_string(value: &Value) -> String {
    match value {
        Value::U8(n) => n.to_string(),
        _ => panic!("expected atomic value"),
    }
}

pub fn print_decoded_value(module: &FormatModule, value: &Value, format: &Format) {
    use std::io::Write;
    let frag = TreePrinter::new(module).compile_decoded_value(value, format);
    let mut lock = io::stdout().lock();
    match write!(&mut lock, "{}", frag) {
        Ok(_) => (),
        Err(e) => eprintln!("error: {e}"),
    }
}

pub fn print_parsed_decoded_value(module: &FormatModule, p_value: &ParsedValue, format: &Format) {
    use std::io::Write;
    let frag = TreePrinter::new(module).compile_parsed_decoded_value(p_value, format);
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
    summarize_boolean_record_set_fields: bool,
}

#[inline]
fn name_is_ascii_string(name: &str) -> bool {
    name.contains("ascii") && name.contains("string")
}

pub struct TreePrinter<'module> {
    gutter: Vec<Column>,
    preview_len: Option<usize>,
    flags: Flags,
    module: &'module FormatModule,
}

type Field<T> = (Label, T);
type FieldPValue = Field<ParsedValue>;
type FieldValue = Field<Value>;

impl<'module> TreePrinter<'module> {
    fn is_implied_value_format_old_style_record(&self, format: &Format) -> bool {
        match format {
            Format::LetFormat(bind, _, inner) => {
                self.is_implied_value_format(bind)
                    && self.is_implied_value_format_old_style_record(inner)
            }
            Format::Compute(..) => true,
            other => unreachable!("unexpected format {other:?}"),
        }
    }

    fn is_implied_value_format_new_style_record(&self, format: &Format) -> bool {
        let record_format = format.to_record_format();
        for (field_label, format) in record_format.iter() {
            match field_label {
                FieldLabel::Permanent { .. } => {
                    if !self.is_implied_value_format(format) {
                        return false;
                    }
                }
                _ => (),
            }
        }
        true
    }

    fn is_implied_value_format(&self, format: &Format) -> bool {
        match format {
            Format::ItemVar(level, _args) => {
                self.is_implied_value_format(self.module.get_format(*level))
            }
            Format::EndOfInput => true,
            Format::Byte(bs) => bs.len() == 1,
            Format::Tuple(fields) => fields.iter().all(|f| self.is_implied_value_format(f)),
            Format::Hint(StyleHint::Record { old_style }, inner) => {
                if *old_style {
                    self.is_implied_value_format_old_style_record(inner)
                } else {
                    self.is_implied_value_format_new_style_record(inner)
                }
            }
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

    fn try_as_record_with_atomic_fields<'a>(
        &'a self,
        format: &'a Format,
    ) -> Option<Vec<(&'a Label, &'a Format)>> {
        let mut fields = Vec::new();
        match format {
            Format::ItemVar(level, _args) => {
                self.try_as_record_with_atomic_fields(self.module.get_format(*level))
            }
            Format::Hint(StyleHint::Record { .. }, ..) => {
                let record_fmt = format.to_record_format();
                for (field_label, field_format) in record_fmt.iter() {
                    match field_label {
                        FieldLabel::Permanent { in_value, .. }
                            if !(in_value.starts_with("__")
                                && self.flags.hide_double_underscore_fields) =>
                        {
                            if self.is_atomic_format(field_format) {
                                fields.push((*in_value, *field_format));
                            } else {
                                return None;
                            }
                        }
                        _ => continue,
                    }
                }
                Some(fields)
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
            Value::U8(_) | Value::U16(_) | Value::U32(_) | Value::U64(_) => true,
            Value::Usize(_) => true,
            Value::Tuple(values) => values.is_empty(),
            Value::Record(fields) => fields.is_empty(),
            Value::Seq(values) => values.is_empty(),
            Value::EnumFromTo(range) => range.is_empty(), // since this nominally represents a Seq, apply Seq-style logic
            Value::Variant(label, value) => match format {
                Some(Format::Variant(label2, format)) => {
                    assert_eq!(label, label2);
                    self.is_atomic_value(value.as_ref(), Some(format))
                }
                _ => self.is_atomic_value(value.as_ref(), None),
            },
            Value::Mapped(orig, value) => {
                if self.flags.collapse_mapped_values {
                    self.is_atomic_value(value.as_ref(), None)
                } else {
                    match format {
                        Some(Format::Map(format, _expr)) => {
                            self.is_atomic_value(orig.as_ref(), Some(format))
                        }
                        _ => self.is_atomic_value(orig.as_ref(), None),
                    }
                }
            }
            Value::Branch(n, value) => match format.map(|f| self.unwrap_itemvars(f)) {
                Some(Format::Union(branches)) | Some(Format::UnionNondet(branches)) => {
                    let format = &branches[*n];
                    self.is_atomic_value(value.as_ref(), Some(format))
                }
                Some(Format::Match(_head, branches)) => {
                    let (_pattern, format) = &branches[*n];
                    self.is_atomic_value(value.as_ref(), Some(format))
                }
                // expose wrapped format contents
                Some(Format::Let(.., inner))
                | Some(Format::Hint(.., inner))
                | Some(Format::WithRelativeOffset(.., inner))
                | Some(Format::MonadSeq(.., inner))
                | Some(Format::LetFormat(.., inner)) => {
                    self.is_atomic_value(value.as_ref(), Some(inner))
                }
                None => self.is_atomic_value(value.as_ref(), None),
                f => panic!("expected format suitable for branch: {f:?}"),
            },
            Value::Option(None) => true,
            // REVIEW - do we have a better alternative to passing in `None` on the following line?
            Value::Option(Some(value)) => self.is_atomic_value(value, None),
        }
    }

    fn is_atomic_parsed_value(&self, value: &ParsedValue, format: Option<&Format>) -> bool {
        if let Some(format) = format {
            if self.flags.pretty_ascii_strings && format.is_ascii_string_format(self.module) {
                return true;
            }
        }
        self.is_atomic_value(value.into_cow_value().as_ref(), format)
    }

    fn unwrap_itemvars<'a>(&'a self, format: &'a Format) -> &'a Format {
        match format {
            &Format::ItemVar(level, ..) => self.unwrap_itemvars(self.module.get_format(level)),
            _ => format,
        }
    }

    fn compile_location(&self, loc: ParseLoc) -> Fragment {
        match loc {
            ParseLoc::InBuffer { offset, length } => {
                Fragment::string(format!("BUF({offset}:+{length})"))
            }
            ParseLoc::Synthesized => Fragment::string("<SYNTH>"),
        }
    }

    fn compile_with_location(&self, frag: Fragment, loc: ParseLoc) -> Fragment {
        Fragment::intervene(
            frag,
            Fragment::string(" \t"),
            self.compile_location(loc)
                .delimit(Fragment::Char('['), Fragment::Char(']')),
        )
    }

    fn compile_parsed_value(&mut self, value: &ParsedValue) -> Fragment {
        match value {
            ParsedValue::Flat(Parsed { loc, inner }) => {
                let symbol = match inner {
                    Value::Bool(true) => Fragment::String("true".into()),
                    Value::Bool(false) => Fragment::String("false".into()),
                    Value::U8(i) => Fragment::DisplayAtom(Rc::new(*i)),
                    Value::U16(i) => Fragment::DisplayAtom(Rc::new(*i)),
                    Value::U32(i) => Fragment::DisplayAtom(Rc::new(*i)),
                    Value::U64(i) => Fragment::DisplayAtom(Rc::new(*i)),
                    Value::Char(c) => Fragment::DebugAtom(Rc::new(*c)),
                    _ => unreachable!("found non-flat Value in ParsedValue::Flat: {inner:?}"),
                };
                self.compile_with_location(symbol, *loc)
            }
            ParsedValue::Tuple(vals) => self.compile_parsed_tuple(vals, None),
            ParsedValue::Seq(vals) => self.compile_parsed_seq(vals, None),
            ParsedValue::Record(fields) => self.compile_parsed_record(fields, None),
            ParsedValue::Variant(label, value) => self.compile_parsed_variant(label, value, None),
            ParsedValue::Mapped(orig, value) => {
                if self.flags.collapse_mapped_values {
                    self.compile_parsed_value(value)
                } else {
                    self.compile_parsed_value(orig)
                }
            }
            ParsedValue::Branch(_n, value) => self.compile_parsed_value(value),
            ParsedValue::Option(None) => Fragment::string("none"),
            ParsedValue::Option(Some(value)) => self.compile_parsed_variant("some", value, None),
        }
    }
}

impl<'module> TreePrinter<'module> {
    pub fn new(module: &'module FormatModule) -> TreePrinter<'module> {
        let flags = Flags {
            collapse_mapped_values: true,
            omit_implied_values: true,
            tables_for_record_sequences: true,
            pretty_ascii_strings: true,
            pretty_utf8_strings: true,
            hide_double_underscore_fields: true,
            show_redundant_formats: false,
            summarize_boolean_record_set_fields: true,
        };
        TreePrinter {
            gutter: Vec::new(),
            preview_len: Some(10),
            flags,
            module,
        }
    }

    pub fn compile_parsed_decoded_value(&mut self, value: &ParsedValue, fmt: &Format) -> Fragment {
        let mut frag = Fragment::Empty;
        match fmt {
            Format::ItemVar(level, _args) => {
                let fmt_name = self.module.get_name(*level);

                // FIXME - this is a bit hackish, we should have a sentinel or marker to avoid magic strings
                if self.flags.pretty_utf8_strings && fmt_name == "text.string.utf8" {
                    self.compile_parsed_string(value)
                } else if self.flags.pretty_ascii_strings && name_is_ascii_string(fmt_name) {
                    self.compile_parsed_ascii_string(value)
                } else if self.flags.pretty_ascii_strings && fmt_name.starts_with("base.ascii-char")
                {
                    frag.append(Fragment::Char('\''));
                    frag.append(self.compile_parsed_ascii_char(value));
                    frag.append(Fragment::Char('\''));
                    frag
                } else {
                    self.compile_parsed_decoded_value(value, self.module.get_format(*level))
                }
            }
            Format::Fail => panic!("uninhabited format (value={value:?}"),
            Format::EndOfInput | Format::SkipRemainder => self.compile_parsed_value(value),
            Format::Align(_) => self.compile_parsed_value(value),
            Format::Byte(_) => self.compile_parsed_value(value),
            // NOTE : Pos self-documents its position so we don't really need to annotate that...
            Format::Pos => self.compile_value(value.into_cow_value().as_ref()),
            Format::DecodeBytes(_bytes, format) => self.compile_parsed_decoded_value(value, format),
            Format::Variant(label, format) => match value {
                ParsedValue::Variant(label2, value) => {
                    if label == label2 {
                        self.compile_parsed_variant(label, value, Some(format))
                    } else {
                        panic!("expected variant label {label}, found {label2}");
                    }
                }
                _ => panic!("expected variant, found {value:?}"),
            },
            Format::Union(branches) | Format::UnionNondet(branches) => match value {
                ParsedValue::Branch(n, value) => {
                    let format = &branches[*n];
                    self.compile_parsed_decoded_value(value, format)
                }
                _ => panic!("expected branch, found {value:?}"),
            },
            Format::Tuple(formats) => match value {
                ParsedValue::Tuple(parsed_tuple) => {
                    if self.flags.pretty_ascii_strings && self.are_all_ascii_formats(formats) {
                        self.compile_parsed_ascii_seq(parsed_tuple)
                    } else {
                        self.compile_parsed_tuple(parsed_tuple, Some(formats))
                    }
                }
                _ => panic!("expected tuple, found {value:?}"),
            },
            Format::Sequence(formats) => match value {
                ParsedValue::Seq(parsed_seq) => {
                    if self.flags.pretty_ascii_strings && self.are_all_ascii_formats(formats) {
                        self.compile_parsed_ascii_seq(parsed_seq)
                    } else {
                        self.compile_parsed_seq_formats(parsed_seq, Some(formats.as_slice()))
                    }
                }
                _ => panic!("expected sequence, found {value:?}"),
            },
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::ForEach(_, _, format)
            | Format::RepeatCount(_, format)
            | Format::RepeatBetween(_, _, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => match value {
                ParsedValue::Seq(values) => {
                    if self.flags.tables_for_record_sequences
                        && self.try_as_record_with_atomic_fields(format).is_some()
                    {
                        self.compile_parsed_seq_records(values, format)
                    } else if self.flags.pretty_ascii_strings
                        && format.is_ascii_char_format(self.module)
                    {
                        self.compile_parsed_ascii_seq(values)
                    } else {
                        self.compile_parsed_seq(values, Some(format))
                    }
                }
                _ => panic!("expected sequence, found {value:?}"),
            },
            Format::AccumUntil(.., format) => match value {
                ParsedValue::Tuple(values) => match values.get_inner().as_slice() {
                    [accum, vs] => {
                        let accum = self.compile_parsed_value(accum);
                        let vs = match vs {
                            ParsedValue::Seq(values) => {
                                if self.flags.tables_for_record_sequences
                                    && self.try_as_record_with_atomic_fields(format).is_some()
                                {
                                    self.compile_parsed_seq_records(values, format)
                                } else if self.flags.pretty_ascii_strings
                                    && format.is_ascii_char_format(self.module)
                                {
                                    self.compile_parsed_ascii_seq(values)
                                } else {
                                    self.compile_parsed_seq(values, Some(format))
                                }
                            }
                            _ => panic!("expected sequence, found {vs:?}"),
                        };
                        // FIXME - this will probably break often, so adjust as necessary
                        frag.append(accum);
                        frag.append(Fragment::string(", "));
                        frag.append(vs);
                        frag.delimit(Fragment::Char('('), Fragment::Char(')'))
                    }
                    _ => panic!("expected 2-tuple, found {values:#?}"),
                },
                _ => panic!("expected sequence, found {value:?}"),
            },
            Format::Maybe(_, format) => match value {
                ParsedValue::Option(opt_val) => match opt_val {
                    Some(val) => self.compile_parsed_variant("some", val, Some(format)),
                    None => self.compile_parsed_variant(
                        "none",
                        &ParsedValue::from_evaluated(Value::UNIT),
                        Some(&Format::EMPTY),
                    ),
                },
                _ => panic!("expected Option, found {value:?}"),
            },
            Format::LiftedOption(fmt) => match value {
                ParsedValue::Option(opt_val) => match (fmt, opt_val) {
                    (None, None) => Fragment::string("none"),
                    (Some(fmt), Some(val)) => self.compile_parsed_variant("some", val, Some(fmt)),
                    (Some(_), None) => panic!("expected Some, found None"),
                    (None, Some(_)) => panic!("expected None, found {opt_val:?}"),
                },
                _ => panic!("expected Option, found {value:?}"),
            },
            Format::Peek(format) => self.compile_parsed_decoded_value(value, format),
            Format::PeekNot(_format) => self.compile_parsed_value(value),
            Format::Slice(_, format) => self.compile_parsed_decoded_value(value, format),
            Format::Bits(format) => self.compile_parsed_decoded_value(value, format),
            Format::WithRelativeOffset(_, _, format) => {
                self.compile_parsed_decoded_value(value, format)
            }
            Format::Map(format, _expr) => {
                if self.flags.collapse_mapped_values {
                    self.compile_parsed_value(value)
                } else {
                    match value {
                        ParsedValue::Mapped(orig, _value) => {
                            self.compile_parsed_decoded_value(orig, format)
                        }
                        _ => panic!("expected mapped value, found {value:?}"),
                    }
                }
            }
            Format::Where(format, _expr) => self.compile_parsed_decoded_value(value, format),
            Format::Compute(_expr) => self.compile_parsed_value(value),
            Format::Let(_name, _expr, format) => self.compile_parsed_decoded_value(value, format),
            Format::LetView(_name, format) => self.compile_parsed_decoded_value(value, format),
            Format::Match(_head, branches) => match value {
                ParsedValue::Branch(index, value) => {
                    let (_pattern, format) = &branches[*index];
                    frag.append(self.compile_parsed_decoded_value(value, format));
                    frag
                }
                _ => panic!("expected branch, found {value:?}"),
            },
            Format::Dynamic(_name, _dynformat, format) => {
                self.compile_parsed_decoded_value(value, format)
            }
            Format::Apply(_) => self.compile_parsed_value(value),
            Format::LetFormat(_f0, _name, f) => self.compile_parsed_decoded_value(value, f),
            Format::MonadSeq(_f0, f) => self.compile_parsed_decoded_value(value, f),
            Format::Hint(_hint, f) => self.compile_parsed_decoded_value(value, f),
            // REVIEW[epic=view-format] - is this correct?
            Format::WithView(_ident, _vf) => self.compile_parsed_value(value),
        }
    }

    pub fn compile_decoded_value(&mut self, value: &Value, fmt: &Format) -> Fragment {
        let mut frag = Fragment::Empty;
        match fmt {
            Format::ItemVar(level, _args) => {
                let fmt_name = self.module.get_name(*level);

                // FIXME - this is a bit hackish, we should have a sentinel or marker to avoid magic strings
                if self.flags.pretty_utf8_strings && fmt_name == "text.string.utf8" {
                    self.compile_string(value)
                } else if self.flags.pretty_ascii_strings && name_is_ascii_string(fmt_name) {
                    self.compile_ascii_string(value)
                } else if self.flags.pretty_ascii_strings && fmt_name.starts_with("base.ascii-char")
                {
                    frag.append(Fragment::Char('\''));
                    frag.append(self.compile_ascii_char(value));
                    frag.append(Fragment::Char('\''));
                    frag
                } else {
                    self.compile_decoded_value(value, self.module.get_format(*level))
                }
            }
            Format::DecodeBytes(_bytes, f) => self.compile_decoded_value(value, f),
            Format::Fail => panic!("uninhabited format (value={value}"),
            Format::SkipRemainder | Format::EndOfInput => self.compile_value(value),
            Format::Align(_) => self.compile_value(value),
            Format::Byte(_) => self.compile_value(value),
            Format::Pos => self.compile_value(value),
            Format::Variant(label, format) => match value {
                Value::Variant(label2, value) => {
                    if label == label2 {
                        self.compile_variant(label, value, Some(format))
                    } else {
                        panic!("expected variant label {label}, found {label2}");
                    }
                }
                _ => panic!("expected variant, found {value}"),
            },
            Format::Union(branches) | Format::UnionNondet(branches) => match value {
                Value::Branch(n, value) => {
                    let format = &branches[*n];
                    self.compile_decoded_value(value, format)
                }
                _ => panic!("expected branch, found {value}"),
            },
            Format::Tuple(formats) => match value {
                Value::Tuple(values) => {
                    if self.flags.pretty_ascii_strings && self.are_all_ascii_formats(formats) {
                        self.compile_ascii_seq(values)
                    } else {
                        self.compile_tuple(values, Some(formats))
                    }
                }
                _ => panic!("expected tuple, found {value}"),
            },
            Format::Sequence(formats) => match value {
                Value::Seq(values) => {
                    if self.flags.pretty_ascii_strings && self.are_all_ascii_formats(formats) {
                        self.compile_ascii_seq(values)
                    } else {
                        self.compile_seq_formats(values, Some(formats.as_slice()))
                    }
                }
                _ => panic!("expected sequence, found {value}"),
            },
            Format::LiftedOption(fmt) => match value {
                Value::Option(opt_val) => match (fmt, opt_val) {
                    (None, None) => Fragment::string("none"),
                    (Some(fmt), Some(val)) => self.compile_variant("some", val, Some(fmt)),
                    (Some(_), None) => panic!("expected Some, found None"),
                    (None, Some(_)) => panic!("expected None, found {opt_val:?}"),
                },
                _ => panic!("expected Option, found {value:?}"),
            },
            Format::Hint(StyleHint::Record { .. }, ..) => match value {
                Value::Record(value_fields) => {
                    let record_format = fmt.to_record_format();
                    self.compile_record(value_fields, Some(&record_format))
                }
                _ => panic!("expected record, found {value}"),
            },
            Format::Hint(StyleHint::AsciiStr, str_format) => {
                if self.flags.pretty_ascii_strings {
                    self.compile_ascii_string(value)
                } else {
                    self.compile_decoded_value(value, str_format)
                }
            }
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::ForEach(_, _, format)
            | Format::RepeatCount(_, format)
            | Format::RepeatBetween(_, _, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => match value {
                Value::Seq(values) => {
                    if self.flags.tables_for_record_sequences
                        && self.try_as_record_with_atomic_fields(format).is_some()
                    {
                        self.compile_seq_records(values, format)
                    } else if self.flags.pretty_ascii_strings
                        && format.is_ascii_char_format(self.module)
                    {
                        self.compile_ascii_seq(values)
                    } else {
                        self.compile_seq(values, Some(format))
                    }
                }
                _ => panic!("expected sequence, found {value}"),
            },
            Format::AccumUntil(.., format) => match value {
                Value::Tuple(values) => match &values[..] {
                    [accum, seq] => {
                        let accum = self.compile_value(accum);
                        let seq = match seq {
                            Value::Seq(values) => {
                                if self.flags.tables_for_record_sequences
                                    && self.try_as_record_with_atomic_fields(format).is_some()
                                {
                                    self.compile_seq_records(values, format)
                                } else if self.flags.pretty_ascii_strings
                                    && format.is_ascii_char_format(self.module)
                                {
                                    self.compile_ascii_seq(values)
                                } else {
                                    self.compile_seq(values, Some(format))
                                }
                            }
                            _ => panic!("expected sequence, found {seq}"),
                        };
                        // FIXME - this may be easily-broken formatting and need some tweaking
                        frag.append(accum);
                        frag.append(Fragment::string(", "));
                        frag.append(seq);
                        frag.delimit(Fragment::Char('('), Fragment::Char(')'))
                    }
                    _ => panic!("expected 2-tuple, found {values:#?}"),
                },
                _ => panic!("expected tuple, found {value}"),
            },
            Format::Maybe(_, inner) => match value {
                Value::Option(opt_val) => match opt_val {
                    Some(val) => self.compile_variant("some", val, Some(inner)),
                    None => self.compile_variant("none", &Value::UNIT, Some(&Format::EMPTY)),
                },
                _ => panic!("expected Option, found {value}"),
            },
            Format::Peek(format) => self.compile_decoded_value(value, format),
            Format::PeekNot(_format) => self.compile_value(value),
            Format::Slice(_, format) => self.compile_decoded_value(value, format),
            Format::Bits(format) => self.compile_decoded_value(value, format),
            Format::WithRelativeOffset(_base_addr, _offset, format) => {
                self.compile_decoded_value(value, format)
            }
            Format::Map(format, _expr) => {
                if self.flags.collapse_mapped_values {
                    self.compile_value(value)
                } else {
                    match value {
                        Value::Mapped(orig, _value) => self.compile_decoded_value(orig, format),
                        _ => panic!("expected mapped value, found {value}"),
                    }
                }
            }
            Format::Where(format, _expr) => self.compile_decoded_value(value, format),
            Format::Compute(_expr) => self.compile_value(value),
            Format::Let(_name, _expr, format) => self.compile_decoded_value(value, format),
            Format::LetView(_name, format) => self.compile_decoded_value(value, format),
            Format::Match(_head, branches) => match value {
                Value::Branch(index, value) => {
                    let (_pattern, format) = &branches[*index];
                    frag.append(self.compile_decoded_value(value, format));
                    frag
                }
                _ => panic!("expected branch, found {value}"),
            },
            Format::Dynamic(_name, _dynformat, format) => self.compile_decoded_value(value, format),
            Format::Apply(_) => self.compile_value(value),
            Format::LetFormat(.., f) | Format::MonadSeq(_, f) => {
                self.compile_decoded_value(value, f)
            }
            // REVIEW[epic=view-format] - is this correct?
            Format::WithView(_ident, _vf) => self.compile_value(value),
        }
    }

    fn are_all_ascii_formats(&self, formats: &[Format]) -> bool {
        !formats.is_empty() && formats.iter().all(|f| f.is_ascii_char_format(self.module))
    }
    pub fn compile_value(&mut self, value: &Value) -> Fragment {
        match value {
            Value::Bool(true) => Fragment::string("true"),
            Value::Bool(false) => Fragment::string("false"),
            Value::U8(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::U16(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::U32(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::U64(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::Usize(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Value::Char(c) => Fragment::DebugAtom(Rc::new(*c)),
            Value::Tuple(vals) => self.compile_tuple(vals, None),
            Value::Seq(vals) => self.compile_seq(vals, None),
            Value::EnumFromTo(range) => Fragment::intervene(
                Fragment::DisplayAtom(Rc::new(range.start)),
                Fragment::string(".."),
                Fragment::DisplayAtom(Rc::new(range.end)),
            )
            .delimit(Fragment::Char('['), Fragment::Char(']')),
            Value::Record(fields) => self.compile_record(fields, None),
            Value::Variant(label, value) => self.compile_variant(label, value, None),
            Value::Mapped(orig, value) => {
                if self.flags.collapse_mapped_values {
                    self.compile_value(value)
                } else {
                    self.compile_value(orig)
                }
            }
            Value::Branch(_n, value) => self.compile_value(value),
            Value::Option(None) => Fragment::string("none"),
            Value::Option(Some(value)) => self.compile_variant("some", value, None),
        }
    }

    fn extract_string_field<'a, T>(&self, fields: &'a [Field<T>]) -> Option<&'a T> {
        fields
            .iter()
            .find_map(|(label, value)| (label == "string").then_some(value))
    }

    pub fn compile_parsed_string(&self, value: &ParsedValue) -> Fragment {
        let vs = match value.coerce_mapped_value() {
            ParsedValue::Record(fields) => {
                match self
                    .extract_string_field(fields.inner.as_slice())
                    .unwrap_or_else(|| unreachable!("no string field"))
                {
                    ParsedValue::Seq(vs) => vs,
                    v => panic!("expected sequence (parsed-)value, found {v:?}"),
                }
            }
            ParsedValue::Seq(vs) => vs,
            v => panic!("expected record or sequence, found {v:?}"),
        };
        self.compile_parsed_char_seq(vs)
    }

    pub fn compile_string(&self, value: &Value) -> Fragment {
        let vs = match value.coerce_mapped_value() {
            Value::Record(fields) => {
                match self
                    .extract_string_field(fields)
                    .unwrap_or_else(|| unreachable!("no string field"))
                {
                    Value::Seq(vs) => vs,
                    v => panic!("expected sequence value, found {v}"),
                }
            }
            Value::Seq(vs) => vs,
            v => panic!("expected record or sequence, found {v}"),
        };
        self.compile_char_seq(vs)
    }

    pub fn compile_parsed_ascii_string(&self, value: &ParsedValue) -> Fragment {
        let vs = match value.coerce_mapped_value() {
            ParsedValue::Record(fields) => {
                match self
                    .extract_string_field(fields.get_inner())
                    .unwrap_or_else(|| unreachable!("no string field"))
                {
                    ParsedValue::Seq(vs) => vs,
                    v => panic!("expected sequence value, found {v:?}"),
                }
            }
            ParsedValue::Seq(vs) => vs,
            _ => panic!("expected record value, found {value:?}"),
        };
        self.compile_parsed_ascii_seq(vs)
    }

    pub fn compile_ascii_string(&self, value: &Value) -> Fragment {
        let vs = match value.coerce_mapped_value() {
            Value::Record(fields) => {
                match self
                    .extract_string_field(fields)
                    .unwrap_or_else(|| unreachable!("no string field"))
                {
                    Value::Seq(vs) => vs,
                    v => panic!("expected sequence value, found {v}"),
                }
            }
            Value::Seq(vs) => vs,
            _ => panic!("expected record value, found {value}"),
        };
        self.compile_ascii_seq(vs)
    }

    fn compile_parsed_char_seq(&self, vals: &Parsed<SeqKind<ParsedValue>>) -> Fragment {
        let mut frag = Fragment::new();
        frag.append(Fragment::Char('"'));
        for v in vals.inner.iter() {
            frag.append(self.compile_parsed_char(v));
        }
        frag.append(Fragment::Char('"'));
        self.compile_with_location(frag.group(), vals.loc)
    }

    fn compile_char_seq<'a, S>(&self, vals: &'a S) -> Fragment
    where
        S: Clone,
        &'a S: IntoIterator<Item = &'a Value>,
    {
        let mut frag = Fragment::new();
        frag.append(Fragment::Char('"'));
        for v in vals {
            frag.append(self.compile_char(v));
        }
        frag.append(Fragment::Char('"'));
        frag.group()
    }

    fn compile_parsed_ascii_seq<'a, S>(&self, vals: &'a Parsed<S>) -> Fragment
    where
        &'a S: IntoIterator<Item = &'a ParsedValue>,
        S: Clone,
    {
        let mut frag = Fragment::new();
        frag.append(Fragment::Char('"'));
        for v in &vals.inner {
            frag.append(self.compile_parsed_ascii_char(v));
        }
        frag.append(Fragment::Char('"'));
        self.compile_with_location(frag.group(), vals.loc)
    }

    fn compile_ascii_seq<'a, S>(&self, vals: &'a S) -> Fragment
    where
        S: Clone,
        &'a S: IntoIterator<Item = &'a Value>,
    {
        let mut frag = Fragment::new();
        frag.append(Fragment::Char('"'));
        for v in vals {
            frag.append(self.compile_ascii_char(v));
        }
        frag.append(Fragment::Char('"'));
        frag.group()
    }

    fn compile_parsed_char(&self, v: &ParsedValue) -> Fragment {
        let c = match v.coerce_mapped_value() {
            ParsedValue::Flat(Parsed { inner, .. }) => match inner {
                Value::U8(b) => *b as char,
                Value::Char(c) => *c,
                _v => panic!("expected U8 or Char value, found {_v:?}"),
            },
            _other => panic!("expected Flat (parsed-)value, found {_other:?}"),
        };
        match c {
            '\x00'..='\x7f' => Fragment::String(c.escape_debug().collect::<String>().into()),
            _ => Fragment::Char(c),
        }
    }

    fn compile_char(&self, v: &Value) -> Fragment {
        let c = match v.coerce_mapped_value() {
            Value::U8(b) => *b as char,
            Value::Char(c) => *c,
            _ => panic!("expected U8 or Char value, found {v}"),
        };
        match c {
            '\x00'..='\x7f' => Fragment::String(c.escape_debug().collect::<String>().into()),
            _ => Fragment::Char(c),
        }
    }

    fn compile_parsed_ascii_char(&self, v: &ParsedValue) -> Fragment {
        let (_loc, b) = match v {
            ParsedValue::Flat(Parsed {
                loc,
                inner: Value::U8(b),
            }) => (*loc, *b),
            _ => panic!("expected U8 value, found {v:?}"),
        };

        // NOTE - ignoring location because ascii strings are printed inline and we can't clutter them
        match b {
            0x00 => Fragment::String("\\0".into()),
            0x09 => Fragment::String("\\t".into()),
            0x0A => Fragment::String("\\n".into()),
            0x0D => Fragment::String("\\r".into()),
            32..=127 => Fragment::Char(b as char),
            _ => Fragment::String(format!("\\x{b:02X}").into()),
        }
    }

    fn compile_ascii_char(&self, v: &Value) -> Fragment {
        let b = match v {
            Value::U8(b) => *b,
            _ => panic!("expected U8 value, found {v}"),
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

    fn compile_tuple(&mut self, vals: &[Value], formats: Option<&[Format]>) -> Fragment {
        if vals.is_empty() {
            Fragment::String("()".into())
        } else {
            let mut frag = Fragment::new();
            let last_index = vals.len() - 1;
            for index in 0..last_index {
                frag.append(self.compile_field_value_continue(
                    index,
                    &vals[index],
                    formats.map(|fs| &fs[index]),
                    true,
                ));
            }
            frag.append(self.compile_field_value_last(
                last_index,
                &vals[last_index],
                formats.map(|fs| &fs[last_index]),
                true,
            ));
            frag
        }
    }

    fn compile_seq_formats(
        &mut self,
        vals: &SeqKind<Value>,
        formats: Option<&[Format]>,
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
                let format = formats.map(|fs| &fs[index]);
                frag.append(self.compile_field_value_continue(index, val, format, false));
            }
            if any_skipped {
                frag.append(self.compile_field_skipped());
            }
            let format = formats.map(|fs| &fs[last_index]);
            frag.append(self.compile_field_value_last(
                last_index,
                &vals[last_index],
                format,
                false,
            ));
            frag
        }
    }

    fn compile_parsed_tuple(
        &mut self,
        vals: &Parsed<Vec<ParsedValue>>,
        formats: Option<&[Format]>,
    ) -> Fragment {
        let Parsed { inner, .. } = vals;
        let frag_value = if inner.is_empty() {
            Fragment::String("()".into())
        } else {
            let mut frag = Fragment::new();
            let last_index = inner.len() - 1;
            for index in 0..last_index {
                frag.append(self.compile_parsed_field_value_continue(
                    index,
                    &inner[index],
                    formats.map(|fs| &fs[index]),
                    true,
                ));
            }
            frag.append(self.compile_parsed_field_value_last(
                last_index,
                &inner[last_index],
                formats.map(|fs| &fs[last_index]),
                true,
            ));
            frag
        };
        // FIXME - does location information for the overall tuple give us anything notable?
        // self.compile_with_location(symbol, *loc)
        frag_value
    }

    fn compile_parsed_seq(
        &mut self,
        vals: &Parsed<SeqKind<ParsedValue>>,
        format: Option<&Format>,
    ) -> Fragment {
        let Parsed { inner, .. } = vals;
        if inner.is_empty() {
            Fragment::String("[]".into())
        } else {
            let mut frag = Fragment::new();
            let last_index = inner.len() - 1;
            let (upper_bound, any_skipped) = match self.preview_len {
                Some(preview_len) if inner.len() > preview_len => {
                    (preview_len, preview_len != last_index)
                }
                Some(_) | None => (last_index, false),
            };
            for ix in 0..upper_bound {
                let val = &inner[ix];
                frag.append(self.compile_parsed_field_value_continue(ix, val, format, false));
            }
            if any_skipped {
                frag.append(self.compile_field_skipped());
            }
            frag.append(self.compile_parsed_field_value_last(
                last_index,
                &inner[last_index],
                format,
                false,
            ));
            frag
        }
    }

    fn compile_parsed_seq_formats(
        &mut self,
        vals: &Parsed<SeqKind<ParsedValue>>,
        formats: Option<&[Format]>,
    ) -> Fragment {
        let Parsed { inner, .. } = vals;
        if inner.is_empty() {
            Fragment::String("[]".into())
        } else {
            let mut frag = Fragment::new();
            let last_index = inner.len() - 1;
            let (upper_bound, any_skipped) = match self.preview_len {
                Some(preview_len) if inner.len() > preview_len => {
                    (preview_len, preview_len != last_index)
                }
                Some(_) | None => (last_index, false),
            };
            for ix in 0..upper_bound {
                let val = &inner[ix];
                let format = formats.map(|fs| &fs[ix]);
                frag.append(self.compile_parsed_field_value_continue(ix, val, format, false));
            }
            if any_skipped {
                frag.append(self.compile_field_skipped());
            }
            let format = formats.map(|fs| &fs[last_index]);
            frag.append(self.compile_parsed_field_value_last(
                last_index,
                &inner[last_index],
                format,
                false,
            ));
            frag
        }
    }

    fn compile_seq(&mut self, vals: &SeqKind<Value>, format: Option<&Format>) -> Fragment {
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

    fn compile_parsed_seq_records(
        &mut self,
        vals: &Parsed<SeqKind<ParsedValue>>,
        format: &Format,
    ) -> Fragment {
        let fields = self.try_as_record_with_atomic_fields(format).unwrap();
        let mut cols = Vec::new();
        let mut header = Vec::new();
        for (label, _) in fields.into_iter() {
            cols.push(label.len());
            header.push(label.clone());
        }
        let mut rows = Vec::new();
        let mut locs = Vec::new();
        for v in vals.inner.iter() {
            let mut row = Vec::new();
            if let ParsedValue::Record(Parsed { loc, inner: fields }) = v {
                for (i, (_l, v)) in fields.iter().enumerate() {
                    let cell = atomic_value_to_string(&v.clone_into_value());
                    cols[i] = std::cmp::max(cols[i], cell.len());
                    row.push(cell);
                    locs.push(*loc);
                }
            } else {
                panic!("expected record value: {v:?}");
            }
            rows.push(row);
        }
        self.compile_parsed_table(&cols, &header, &rows, &locs)
    }

    fn compile_seq_records<'a, S>(&mut self, vals: &'a S, format: &Format) -> Fragment
    where
        S: Clone,
        &'a S: IntoIterator<Item = &'a Value>,
    {
        let fields = self.try_as_record_with_atomic_fields(format).unwrap();
        let mut cols = Vec::new();
        let mut header = Vec::new();
        for (label, _) in fields.into_iter() {
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
                panic!("expected record value: {v}");
            }
            rows.push(row);
        }
        self.compile_table(&cols, &header, &rows)
    }

    fn compile_parsed_table(
        &mut self,
        cols: &[usize],
        header: &[Label],
        rows: &[Vec<String>],
        locs: &[ParseLoc],
    ) -> Fragment {
        let mut frags = FragmentBuilder::new();
        let frag = frags.active_mut();
        frag.append(self.compile_gutter());
        frag.append(Fragment::Symbol(Symbol::Elbow));
        for (i, th) in header.iter().enumerate() {
            frag.append(Fragment::String(
                format!(" {:>width$}", th, width = cols[i]).into(),
            ));
        }
        frag.enclose().append_break();
        let mut frag = frags.renew();
        self.gutter.push(Column::Space);
        for (tr, loc) in Iterator::zip(rows.iter(), locs.iter()) {
            frag.append(self.compile_gutter());
            for (i, td) in tr.iter().enumerate() {
                frag.append(Fragment::String(
                    format!(" {:>width$}", td, width = cols[i]).into(),
                ));
            }
            frag.enclose()
                .append(Fragment::string(" \t"))
                .append(
                    self.compile_location(*loc)
                        .delimit(Fragment::Char('['), Fragment::Char(']')),
                )
                .append_break();
            frag = frags.renew();
        }
        self.gutter.pop();
        frags.finalize()
    }

    fn compile_table(
        &mut self,
        cols: &[usize],
        header: &[Label],
        rows: &[Vec<String>],
    ) -> Fragment {
        let mut frags = FragmentBuilder::new();
        let frag = frags.active_mut();
        frag.append(self.compile_gutter());
        frag.append(Fragment::Symbol(Symbol::Elbow));
        for (i, th) in header.iter().enumerate() {
            frag.append(Fragment::String(
                format!(" {:>width$}", th, width = cols[i]).into(),
            ));
        }
        frag.enclose().append_break();
        let mut frag = frags.renew();
        self.gutter.push(Column::Space);
        for tr in rows {
            frag.append(self.compile_gutter());
            for (i, td) in tr.iter().enumerate() {
                frag.append(Fragment::String(
                    format!(" {:>width$}", td, width = cols[i]).into(),
                ));
            }
            frag.enclose().append_break();
            frag = frags.renew();
        }
        self.gutter.pop();
        frags.finalize()
    }

    fn compile_parsed_record(
        &mut self,
        p_value_fields: &Parsed<Vec<FieldPValue>>,
        format_fields: Option<&RecordFormat<'_>>,
    ) -> Fragment {
        let Parsed {
            inner: value_fields,
            ..
        } = p_value_fields;
        let mut value_fields_keep = Vec::new();

        let v_fields = if self.flags.hide_double_underscore_fields
            && value_fields.iter().any(|(lab, _)| lab.starts_with("__"))
        {
            value_fields_keep.extend(
                value_fields
                    .iter()
                    .filter(|(lab, _)| !lab.starts_with("__"))
                    .cloned(),
            );
            value_fields_keep.as_slice()
        } else {
            value_fields
        };

        if v_fields.is_empty() {
            Fragment::String("{}".into())
        } else if v_fields.iter().all(|(_, v)| v.is_boolean())
            && self.flags.summarize_boolean_record_set_fields
        {
            self.compile_parsed_bool_flags(v_fields)
        } else {
            let mut frag = Fragment::new();
            let last_index = v_fields.len() - 1;
            for (label, value) in v_fields[..last_index].iter() {
                let format = format_fields
                    .and_then(|fs| fs.lookup_value_field(label))
                    .map(|(f, _)| f);
                frag.append(self.compile_parsed_field_value_continue(label, value, format, true));
            }
            let (label, value) = &v_fields[last_index];
            let format = format_fields
                .and_then(|fs| fs.lookup_value_field(label))
                .map(|(f, _)| f);
            frag.append(self.compile_parsed_field_value_last(label, value, format, true));
            frag
        }
    }

    fn compile_parsed_bool_flags(&mut self, value_fields: &[FieldPValue]) -> Fragment {
        let mut set_fields = Vec::with_capacity(value_fields.len());

        for (label, value) in value_fields {
            if value.coerce_mapped_value().into_cow_value().unwrap_bool() {
                set_fields.push(Fragment::String(label.clone()));
            }
        }
        Fragment::string("bool-flags").cat(
            Fragment::seq(set_fields, Some(Fragment::Char('|')))
                .delimit(Fragment::Char('['), Fragment::Char(']')),
        )
    }

    fn compile_record(
        &mut self,
        value_fields: &[FieldValue],
        format_spine: Option<&RecordFormat<'_>>,
    ) -> Fragment {
        let mut value_fields_keep = Vec::new();

        let v_fields = if self.flags.hide_double_underscore_fields
            && value_fields.iter().any(|(lab, _)| lab.starts_with("__"))
        {
            value_fields_keep.extend(
                value_fields
                    .iter()
                    .filter(|(lab, _)| !lab.starts_with("__"))
                    .cloned(),
            );
            value_fields_keep.as_slice()
        } else {
            value_fields
        };

        if v_fields.is_empty() {
            Fragment::String("{}".into())
        } else if v_fields.iter().all(|(_, v)| v.is_boolean())
            && self.flags.summarize_boolean_record_set_fields
        {
            self.compile_bool_flags(v_fields)
        } else {
            let mut frag = Fragment::new();
            let last_index = v_fields.len() - 1;
            for (label, value) in v_fields[..last_index].iter() {
                let format = format_spine
                    .and_then(|fs| fs.lookup_value_field(label))
                    .map(|(f, _)| f);
                frag.append(self.compile_field_value_continue(label, value, format, true));
            }
            let (label, value) = &v_fields[last_index];
            let format = format_spine
                .and_then(|fs| fs.lookup_value_field(label))
                .map(|(f, _)| f);
            frag.append(self.compile_field_value_last(label, value, format, true));
            frag
        }
    }

    fn compile_bool_flags(&mut self, value_fields: &[FieldValue]) -> Fragment {
        let mut set_fields = Vec::with_capacity(value_fields.len());

        for (label, value) in value_fields {
            if value.coerce_mapped_value().unwrap_bool() {
                set_fields.push(Fragment::String(label.clone()));
            }
        }
        Fragment::string("bool-flags").cat(
            Fragment::seq(set_fields, Some(Fragment::Char('|')))
                .delimit(Fragment::Char('['), Fragment::Char(']')),
        )
    }

    fn compile_parsed_variant(
        &mut self,
        label: &str,
        value: &ParsedValue,
        format: Option<&Format>,
    ) -> Fragment {
        if self.flags.omit_implied_values
            && format.is_some_and(|format| self.is_implied_value_format(format))
        {
            Fragment::string(label.to_string())
        } else if self.is_atomic_parsed_value(value, format) {
            let mut frag = Fragment::new();
            frag.append(Fragment::String(format!("{{ {label} := ").into()));
            if let Some(format) = format {
                frag.append(self.compile_parsed_decoded_value(value, format));
            } else {
                frag.append(self.compile_parsed_value(value));
            }
            frag.append(Fragment::String(" }".into()));
            frag.enclose();
            frag
        } else {
            self.compile_parsed_field_value_last(label, value, format, true)
        }
    }

    fn compile_variant(&mut self, label: &str, value: &Value, format: Option<&Format>) -> Fragment {
        if self.flags.omit_implied_values
            && format.is_some_and(|format| self.is_implied_value_format(format))
        {
            Fragment::string(label.to_string())
        } else if self.is_atomic_value(value, format) {
            let mut frag = Fragment::new();
            frag.append(Fragment::String(format!("{{ {label} := ").into()));
            if let Some(format) = format {
                frag.append(self.compile_decoded_value(value, format));
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
        matches!(
            format,
            Format::ItemVar(..) | Format::Dynamic(..) | Format::Apply(..)
        )
    }

    fn compile_parsed_field_value_continue(
        &mut self,
        label: impl fmt::Display,
        value: &ParsedValue,
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
        let frag_value = self.compile_parsed_field_value(value, format);
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
        // let tagged = self.compile_with_location(frag_value, value.get_loc());
        frags.push(frag_value);
        frags.finalize().group()
    }

    fn compile_field_value_continue(
        &mut self,
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
        let frag_value = self.compile_field_value(value, format);
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

    fn compile_parsed_field_value_last(
        &mut self,
        label: impl fmt::Display,
        value: &ParsedValue,
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
        let frag_value = self.compile_parsed_field_value(value, format);
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
        // let tagged = self.compile_with_location(frag_value, value.get_loc());
        frags.push(frag_value);
        frags.finalize().group()
    }

    fn compile_field_value_last(
        &mut self,
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
        let frag_value = self.compile_field_value(value, format);
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

    fn compile_parsed_field_value(
        &mut self,
        value: &ParsedValue,
        format: Option<&Format>,
    ) -> Fragment {
        match format {
            Some(format) => {
                if self.flags.omit_implied_values && self.is_implied_value_format(format) {
                    Fragment::cat(
                        Fragment::string(" \t"),
                        self.compile_location(value.get_loc())
                            .delimit(Fragment::Char('['), Fragment::Char(']')),
                    )
                    .cat_break()
                } else {
                    Fragment::join_with_wsp_eol(
                        Fragment::String(" :=".into()),
                        self.compile_parsed_decoded_value(value, format),
                        Fragment::cat(
                            Fragment::string(" \t"),
                            self.compile_location(value.get_loc())
                                .delimit(Fragment::Char('['), Fragment::Char(']')),
                        ),
                    )
                    .group()
                }
            }
            None => Fragment::join_with_wsp_eol(
                Fragment::String(" :=".into()),
                self.compile_parsed_value(value),
                Fragment::cat(
                    Fragment::string(" \t"),
                    self.compile_location(value.get_loc())
                        .delimit(Fragment::Char('['), Fragment::Char(']')),
                ),
            )
            .group(),
        }
    }

    fn compile_field_value(&mut self, value: &Value, format: Option<&Format>) -> Fragment {
        match format {
            Some(format) => {
                if self.flags.omit_implied_values && self.is_implied_value_format(format) {
                    Fragment::Char('\n')
                } else {
                    Fragment::join_with_wsp(
                        Fragment::String(" :=".into()),
                        self.compile_decoded_value(value, format),
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

    fn compile_field_skipped(&mut self) -> Fragment {
        self.compile_gutter()
            .cat(Fragment::String("~\n".into()))
            .group()
    }

    fn binary_op(
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
    fn prefix_op(&mut self, op: &'static str, args: Option<&[&Expr]>, operand: &Expr) -> Fragment {
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

    // NOTE - currently used only for `Expr::Destructure, otherwise patterns are not shown`.
    fn compile_pattern(&mut self, pat: &Pattern) -> Fragment {
        match pat {
            Pattern::Binding(name) => Fragment::String(name.clone()),
            Pattern::Tuple(elts) => Fragment::seq(
                elts.iter().map(|e| self.compile_pattern(e)),
                Some(Fragment::String(", ".into())),
            )
            .delimit(Fragment::Char('('), Fragment::Char(')'))
            .group(),
            Pattern::Option(Some(pat)) => Fragment::string("Some")
                .cat(Fragment::Char('('))
                .cat(self.compile_pattern(pat))
                .cat(Fragment::Char(')'))
                .group(),
            Pattern::Option(None) => Fragment::string("None"),
            Pattern::Wildcard => Fragment::string("_"),
            Pattern::Seq(pats) => Fragment::seq(
                pats.iter().map(|e| self.compile_pattern(e)),
                Some(Fragment::String(", ".into())),
            )
            .delimit(Fragment::Char('['), Fragment::Char(']'))
            .group(),
            Pattern::Variant(name, pat) => Fragment::String(name.clone())
                .cat(Fragment::Char('('))
                .cat(self.compile_pattern(pat))
                .cat(Fragment::Char(')'))
                .group(),
            Pattern::Int(..)
            | Pattern::U8(..)
            | Pattern::U16(..)
            | Pattern::U32(..)
            | Pattern::U64(..)
            | Pattern::Bool(..)
            | Pattern::Char(..) => unreachable!("compile_pattern: unexpected pattern: {pat:?}"),
        }
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
            Expr::Destructure(head, pat, expr) => cond_paren(
                Fragment::String("pat-bind ".into())
                    .cat(Fragment::Char('['))
                    .cat(self.compile_pattern(pat))
                    .cat(Fragment::String(" = ".into()))
                    .cat(self.compile_expr(head, Precedence::TOP))
                    .cat(Fragment::string("] "))
                    .cat(self.compile_expr(expr, prec)),
                prec,
                // REVIEW - does this need its own precedence level?
                Precedence::MATCH,
            ),
            Expr::Lambda(name, expr) => cond_paren(
                Fragment::String(name.clone())
                    .cat(Fragment::String(" -> ".into()))
                    .cat(self.compile_expr(expr, Precedence::ARROW))
                    .group(),
                prec,
                Precedence::ARROW,
            ),
            Expr::IntRel(IntRel::Eq, lhs, rhs) => cond_paren(
                self.binary_op(" == ", lhs, rhs, Precedence::EQUALITY, Precedence::EQUALITY),
                prec,
                Precedence::COMPARE,
            ),
            Expr::IntRel(IntRel::Ne, lhs, rhs) => cond_paren(
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
            Expr::Arith(Arith::BoolAnd, lhs, rhs) => cond_paren(
                self.binary_op(" && ", lhs, rhs, Precedence::BITAND, Precedence::BITAND),
                prec,
                Precedence::LOGICAL_AND,
            ),
            Expr::Arith(Arith::BoolOr, lhs, rhs) => cond_paren(
                self.binary_op(" || ", lhs, rhs, Precedence::BITOR, Precedence::BITOR),
                prec,
                Precedence::LOGICAL_OR,
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
            Expr::Append(lhs, rhs) => cond_paren(
                self.binary_op(" ++ ", lhs, rhs, Precedence::APPEND, Precedence::APPEND),
                prec,
                Precedence::APPEND,
            ),
            Expr::Unary(UnaryOp::BoolNot, expr) => cond_paren(
                self.prefix_op("!", None, expr),
                prec,
                Precedence::LOGICAL_NEGATE,
            ),
            Expr::Unary(UnaryOp::IntPred, expr) => cond_paren(
                self.prefix_op("pred", None, expr),
                prec,
                Precedence::NUMERIC_PREFIX,
            ),
            Expr::Unary(UnaryOp::IntSucc, expr) => cond_paren(
                self.prefix_op("succ", None, expr),
                prec,
                Precedence::NUMERIC_PREFIX,
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
            Expr::U16Be(bytes) => cond_paren(
                self.prefix_op("u16be", None, bytes),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::U16Le(bytes) => cond_paren(
                self.prefix_op("u16le", None, bytes),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::U32Be(bytes) => cond_paren(
                self.prefix_op("u32be", None, bytes),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::U32Le(bytes) => cond_paren(
                self.prefix_op("u32le", None, bytes),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::U64Be(bytes) => cond_paren(
                self.prefix_op("u64be", None, bytes),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::U64Le(bytes) => cond_paren(
                self.prefix_op("u64le", None, bytes),
                prec,
                Precedence::CAST_PREFIX,
            ),
            Expr::SeqLength(seq) => cond_paren(
                self.prefix_op("seq-length", None, seq),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::SeqIx(seq, index) => cond_paren(
                self.prefix_op("seq-ix", Some(&[index]), seq),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::SubSeq(seq, start, length) => cond_paren(
                self.prefix_op("sub-seq", Some(&[start, length]), seq),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::SubSeqInflate(seq, start, length) => cond_paren(
                self.prefix_op("sub-seq-inflate", Some(&[start, length]), seq),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::FlatMap(expr, seq) => cond_paren(
                self.prefix_op("flat-map", Some(&[expr]), seq),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::FlatMapAccum(expr, accum, _accum_type, seq) => cond_paren(
                self.prefix_op("flat-map-accum", Some(&[expr, accum]), seq),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::LeftFold(expr, accum, _accum_type, seq) => cond_paren(
                self.prefix_op("left-fold", Some(&[expr, accum]), seq),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::FindByKey(is_sorted, keying_fn, query, seq) => cond_paren(
                self.prefix_op(
                    if *is_sorted {
                        "binary-search"
                    } else {
                        "linear-search"
                    },
                    Some(&[keying_fn, query]),
                    seq,
                ),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::FlatMapList(expr, _ret_type, seq) => cond_paren(
                self.prefix_op("flat-map-list", Some(&[expr]), seq),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::Dup(count, expr) => cond_paren(
                self.prefix_op("dup", Some(&[count]), expr),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::EnumFromTo(start, stop) => cond_paren(
                self.binary_op(
                    " .. ",
                    start,
                    stop,
                    Precedence::FUN_APPLICATION, // REVIEW - determine whether this precedence is proper
                    Precedence::FUN_APPLICATION,
                ),
                prec,
                // REVIEW - determine whether this precedence is proper
                Precedence::FUN_APPLICATION,
            ),
            Expr::LiftOption(Some(expr)) => cond_paren(
                self.prefix_op("some", None, expr),
                prec,
                Precedence::FUN_APPLICATION,
            ),
            Expr::LiftOption(None) => Fragment::string("none"),
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
                    .cat(Fragment::String(label.clone()))
                    .group(),
                prec,
                Precedence::PROJ,
            ),
            Expr::Var(name) => Fragment::String(name.clone()),
            Expr::Bool(b) => Fragment::DisplayAtom(Rc::new(*b)),
            Expr::U8(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::U16(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::U32(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::U64(i) => Fragment::DisplayAtom(Rc::new(*i)),
            Expr::Tuple(..) => Fragment::String("(...)".into()),
            Expr::Record(..) => Fragment::String("{ ... }".into()),
            Expr::Variant(label, expr) => Fragment::String("{ ".into())
                .cat(Fragment::String(label.clone()))
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
            for arg in args.iter() {
                frags.push(arg.clone());
            }
        }
        frags.push(self.compile_format(inner, prec.bump_format()));
        frags.finalize_with_sep(Fragment::Char(' '))
    }

    // FIXME - without a first-class record, Formats will be printed in less sensible ways
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
            Format::UnionNondet(_) | Format::Union(_) => cond_paren(
                Fragment::String("_ |...| _".into()),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Maybe(expr, f) => {
                let frag_expr = self.compile_expr(expr, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format("maybe", Some(&[frag_expr]), f, prec),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
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
            Format::LiftedOption(None) => {
                // REVIEW - do we wish to specify this more formally, or is this good enough?
                Fragment::string("lifted-none")
            }
            Format::LiftedOption(Some(format)) => cond_paren(
                self.compile_nested_format("lifted-some", None, format, prec),
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
            Format::RepeatBetween(min, max, format) => {
                let expr_frag = self.compile_expr(
                    &Expr::Tuple(vec![*min.clone(), *max.clone()]),
                    Precedence::ATOM,
                );
                cond_paren(
                    self.compile_nested_format("repeat-between", Some(&[expr_frag]), format, prec),
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
            Format::AccumUntil(f_done, f_update, init, _vt, format) => {
                let done_frag = self.compile_expr(f_done, Precedence::ATOM);
                let update_frag = self.compile_expr(f_update, Precedence::ATOM);
                let init_frag = self.compile_expr(init, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "accum-until",
                        Some(&[done_frag, update_frag, init_frag]),
                        format,
                        prec,
                    ),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::DecodeBytes(expr, format) => {
                let expr_frag = self.compile_expr(expr, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format("decode-bytes", Some(&[expr_frag]), format, prec),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::ForEach(expr, lbl, format) => {
                let expr_frag = self.compile_expr(expr, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "for-each",
                        Some(&[expr_frag, Fragment::String(lbl.clone())]),
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
            Format::WithRelativeOffset(base_addr, offset, format) => {
                let base_frag = self.compile_expr(base_addr, Precedence::ATOM);
                let offs_frag = self.compile_expr(offset, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "with-relative-offset",
                        Some(&[base_frag, offs_frag]),
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
            Format::Where(format, expr) => {
                let expr_frag = self.compile_expr(expr, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format("assert", Some(&[expr_frag]), format, prec),
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
            Format::Let(name, expr, format) => {
                let expr_frag = self.compile_expr(expr, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "let",
                        Some(&[Fragment::String(name.clone()), expr_frag]),
                        format,
                        prec,
                    ),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::LetView(name, format) => cond_paren(
                self.compile_nested_format(
                    "let-view",
                    Some(&[Fragment::String(name.clone())]),
                    format,
                    prec,
                ),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::MonadSeq(f0, f) => {
                let fmt_frag = self.compile_format(f0, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format("monad-seq", Some(&[fmt_frag]), f, prec),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::LetFormat(f0, name, f) => {
                // FIXME - do we want to print more than a stub if the format is simple enough to represent?
                let fmt_frag = self.compile_format(f0, Precedence::ATOM);
                cond_paren(
                    self.compile_nested_format(
                        "let-format",
                        Some(&[Fragment::String(name.clone()), fmt_frag]),
                        f,
                        prec,
                    ),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::Match(head, _) => cond_paren(
                Fragment::String("match ".into())
                    .cat(self.compile_expr(head, Precedence::PROJ))
                    .cat(Fragment::String(" { ... }".into()))
                    .group(),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::Dynamic(name, dynformat, format) => {
                let dyn_frag = match dynformat {
                    DynFormat::Huffman(_, _) => Fragment::String("huffman".into()),
                };
                cond_paren(
                    self.compile_nested_format(
                        "dynamic",
                        Some(&[Fragment::String(name.clone()), dyn_frag]),
                        format,
                        prec,
                    ),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
            Format::Apply(_) => Fragment::String("apply".into()),

            Format::ItemVar(var, args) => {
                let mut frag = Fragment::new();
                frag.append(Fragment::String(
                    self.module.get_name(*var).to_string().into(),
                ));
                if !args.is_empty() {
                    frag.append(Fragment::String("(...)".into()));
                }
                frag
            }
            Format::Fail => Fragment::string("fail"),
            Format::SkipRemainder => Fragment::string("skip-remainder"),
            Format::EndOfInput => Fragment::string("end-of-input"),
            Format::Pos => Fragment::string("pos"),
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

            Format::Sequence(formats) if formats.is_empty() => Fragment::String("[]".into()),
            Format::Sequence(_) => Fragment::String("[ ... ]".into()),

            Format::Hint(StyleHint::Record { old_style: true }, inner) => match inner.as_ref() {
                Format::Compute(expr) if matches!(&**expr, Expr::Record(vs) if vs.is_empty()) => {
                    Fragment::String("{}".into())
                }
                Format::Compute(..) => Fragment::String("{ ... }".into()),
                Format::LetFormat(..) => Fragment::String("{ ... }".into()),
                Format::MonadSeq(..) => Fragment::String("{ ... }".into()),
                _ => unreachable!("unexpected old-style record-hint inner format: {inner:?}"),
            },
            Format::Hint(StyleHint::Record { old_style: false }, inner) => {
                // FIXME - print enhanced output for new-style records
                match inner.as_ref() {
                    Format::Compute(expr) if matches!(&**expr, Expr::Record(vs) if vs.is_empty()) => {
                        Fragment::String("{}".into())
                    }
                    Format::Compute(..) => Fragment::String("{ ... }".into()),
                    Format::LetFormat(..) => Fragment::String("{ ... }".into()),
                    Format::MonadSeq(..) => Fragment::String("{ ... }".into()),
                    _ => unreachable!("unexpected old-style record-hint inner format: {inner:?}"),
                }
            }
            Format::Hint(StyleHint::AsciiStr, str_format) => cond_paren(
                self.compile_nested_format("ascii-str", None, str_format, prec),
                prec,
                Precedence::FORMAT_COMPOUND,
            ),
            Format::WithView(ident, view_format) => {
                let view_frag = match view_format {
                    ViewFormat::ReadOffsetLen(offset, len) => {
                        let offset_frag = self.compile_expr(offset, Precedence::Top);
                        let len_frag = self.compile_expr(len, Precedence::Top);
                        let mut builder = FragmentBuilder::new();
                        builder.push(Fragment::string("read-offset-len"));
                        builder.push(Fragment::Char('('));
                        builder.push(offset_frag);
                        builder.push(Fragment::string(", "));
                        builder.push(len_frag);
                        builder.push(Fragment::Char(')'));
                        builder.finalize()
                    }
                };
                cond_paren(
                    Fragment::string("with-view")
                        .intervene(Fragment::Char(' '), Fragment::String(ident.clone()))
                        .intervene(Fragment::Char(' '), view_frag),
                    prec,
                    Precedence::FORMAT_COMPOUND,
                )
            }
        }
    }
}
