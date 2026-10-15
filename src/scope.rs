use std::borrow::Cow;

use crate::Label;
use crate::decoder::{Decoder, Value};
use crate::error::UnknownVarError;
use crate::loc_decoder::ParsedValue;
use crate::read::ReadCtxt;

#[derive(Clone, Copy)]
pub struct VoidScope;

impl<'a> EvalScope<'a> for VoidScope {
    type Output = &'a std::convert::Infallible;
    type Error = UnknownVarError;

    fn lookup_var(&'a self, lbl: &str) -> Result<Self::Output, Self::Error> {
        Err(UnknownVarError(std::borrow::Cow::Owned(lbl.to_string())))
    }
}

pub trait EvalScope<'a> {
    type Output;
    type Error;

    fn lookup_var(&'a self, name: &str) -> Result<Self::Output, Self::Error>;
}

impl<'a, V: Clone> EvalScope<'a> for GScope<'a, V> {
    type Output = &'a V;
    type Error = UnknownVarError;

    fn lookup_var(&'a self, name: &str) -> Result<Self::Output, Self::Error> {
        self.get_value_by_name(name)
    }
}

// REVIEW - do we want a specialized type for holding views?
pub type View<'a> = ReadCtxt<'a>;

/// The value bound to a name in a scope, as returned by `get_bindings` for
/// introspection/debugging (e.g. error reporting): either the leaf value itself
/// (`Value` or `ParsedValue`, generically `V`), a bound sub-decoder, or a view's offset.
#[derive(Clone, Debug)]
pub enum ScopeEntry<V: Clone> {
    Value(V),
    Decoder(Decoder),
    View(usize),
}

/// A generic, singly-linked-list-style variable scope, parameterized over the leaf
/// value representation `V` (`Value` for the main evaluator, `ParsedValue` for the
/// location-tracking evaluator). Each variant adds one kind of binding on top of a
/// parent scope; lookups walk up the chain.
///
/// Type aliases `Scope<'a>` (`decoder.rs`) and `LocScope<'a>` (`loc_decoder.rs`)
/// instantiate this for the two evaluators.
pub enum GScope<'a, V: Clone> {
    Empty,
    Multi(&'a GMultiScope<'a, V>),
    Single(GSingleScope<'a, V>),
    Decoder(GDecoderScope<'a, V>),
    View(GViewScope<'a, V>),
}

#[derive(Clone)]
enum ViewOrValue<'a, V: Clone> {
    View(View<'a>),
    Value(Cow<'a, V>),
}

pub struct GMultiScope<'a, V: Clone> {
    parent: &'a GScope<'a, V>,
    entries: Vec<(Label, ViewOrValue<'a, V>)>,
}

pub struct GSingleScope<'a, V: Clone> {
    parent: &'a GScope<'a, V>,
    name: &'a str,
    value: &'a V,
}

pub struct GDecoderScope<'a, V: Clone> {
    parent: &'a GScope<'a, V>,
    name: &'a str,
    decoder: Decoder,
}

pub struct GViewScope<'a, V: Clone> {
    parent: &'a GScope<'a, V>,
    name: &'a str,
    view: View<'a>,
}

impl<'a, V: Clone> GScope<'a, V> {
    pub(crate) fn get_value_by_name(&self, name: &str) -> Result<&V, UnknownVarError> {
        match self {
            GScope::Empty => Err(UnknownVarError(Label::Owned(name.to_string()))),
            GScope::Multi(multi) => multi.get_value_by_name(name),
            GScope::Single(single) => single.get_value_by_name(name),
            GScope::Decoder(decoder) => decoder.parent.get_value_by_name(name),
            GScope::View(view) => view.parent.get_value_by_name(name),
        }
    }

    pub(crate) fn get_decoder_by_name(&self, name: &str) -> &Decoder {
        match self {
            GScope::Empty => panic!("decoder not found: {name}"),
            GScope::Multi(multi) => multi.parent.get_decoder_by_name(name),
            GScope::Single(single) => single.parent.get_decoder_by_name(name),
            GScope::Decoder(decoder) => decoder.get_decoder_by_name(name),
            GScope::View(view) => view.parent.get_decoder_by_name(name),
        }
    }

    pub(crate) fn get_view_by_name(&self, name: &str) -> View<'a> {
        match self {
            GScope::Empty => panic!("view not found: {name}"),
            GScope::Multi(multi) => multi.get_view_by_name(name),
            GScope::Single(single) => single.parent.get_view_by_name(name),
            GScope::Decoder(decoder) => decoder.parent.get_view_by_name(name),
            GScope::View(view) => view.get_view_by_name(name),
        }
    }

    pub fn get_bindings(&self, bindings: &mut Vec<(Label, ScopeEntry<V>)>) {
        match self {
            GScope::Empty => {}
            GScope::Multi(multi) => multi.get_bindings(bindings),
            GScope::Single(single) => single.get_bindings(bindings),
            GScope::Decoder(decoder) => decoder.get_bindings(bindings),
            GScope::View(view) => view.get_bindings(bindings),
        }
    }
}

impl<'a, V: Clone> GMultiScope<'a, V> {
    pub(crate) fn new(parent: &'a GScope<'a, V>) -> GMultiScope<'a, V> {
        let entries = Vec::new();
        GMultiScope { parent, entries }
    }

    pub fn with_capacity(parent: &'a GScope<'a, V>, capacity: usize) -> GMultiScope<'a, V> {
        let entries = Vec::with_capacity(capacity);
        GMultiScope { parent, entries }
    }

    /// Pushes a new binding to the scope using a borrow that lives at least as long as the scope itself
    pub fn push(&mut self, name: impl Into<Label>, v: &'a V) {
        self.entries
            .push((name.into(), ViewOrValue::Value(Cow::Borrowed(v))))
    }

    /// Pushes a new binding to the scope using an owned value
    pub fn push_owned(&mut self, name: impl Into<Label>, v: V) {
        self.entries
            .push((name.into(), ViewOrValue::Value(Cow::Owned(v))))
    }

    pub fn push_view(&mut self, name: impl Into<Label>, view: View<'a>) {
        self.entries.push((name.into(), ViewOrValue::View(view)));
    }

    fn get_view_by_name(&self, name: &str) -> View<'a> {
        for (n, v) in self.entries.iter().rev() {
            if n == name {
                if let ViewOrValue::View(v) = v {
                    return *v;
                } else {
                    log::warn!(
                        "MultiScope::get_view_by_name: query for `{name}` encountered a value-binding before any view-bindings, skipping..."
                    );
                    continue;
                }
            }
        }
        self.parent.get_view_by_name(name)
    }

    fn get_value_by_name(&self, name: &str) -> Result<&V, UnknownVarError> {
        for (n, v) in self.entries.iter().rev() {
            if n == name {
                if let ViewOrValue::Value(v) = v {
                    return Ok(v);
                } else {
                    log::warn!(
                        "MultiScope::get_value_by_name: query for `{name}` encountered a view-binding before any value-bindings, skipping..."
                    );
                    continue;
                }
            }
        }
        self.parent.get_value_by_name(name)
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, ScopeEntry<V>)>) {
        for (name, vv) in self.entries.iter().rev() {
            match vv {
                ViewOrValue::View(view) => {
                    bindings.push((name.clone(), ScopeEntry::View(view.offset)))
                }
                ViewOrValue::Value(value) => {
                    bindings.push((name.clone(), ScopeEntry::Value(value.clone().into_owned())));
                }
            }
        }
        self.parent.get_bindings(bindings);
    }
}

impl<'a, V: Clone> GSingleScope<'a, V> {
    pub fn new(parent: &'a GScope<'a, V>, name: &'a str, value: &'a V) -> GSingleScope<'a, V> {
        GSingleScope {
            parent,
            name,
            value,
        }
    }

    fn get_value_by_name(&self, name: &str) -> Result<&V, UnknownVarError> {
        if self.name == name {
            Ok(self.value)
        } else {
            self.parent.get_value_by_name(name)
        }
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, ScopeEntry<V>)>) {
        bindings.push((
            Label::Owned(self.name.to_string()),
            ScopeEntry::Value(self.value.clone()),
        ));
        self.parent.get_bindings(bindings);
    }
}

impl<'a, V: Clone> GDecoderScope<'a, V> {
    pub(crate) fn new(
        parent: &'a GScope<'a, V>,
        name: &'a str,
        decoder: Decoder,
    ) -> GDecoderScope<'a, V> {
        GDecoderScope {
            parent,
            name,
            decoder,
        }
    }

    fn get_decoder_by_name(&self, name: &str) -> &Decoder {
        if self.name == name {
            &self.decoder
        } else {
            self.parent.get_decoder_by_name(name)
        }
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, ScopeEntry<V>)>) {
        bindings.push((
            self.name.to_string().into(),
            ScopeEntry::Decoder(self.decoder.clone()),
        ));
        self.parent.get_bindings(bindings);
    }
}

impl<'a, V: Clone> GViewScope<'a, V> {
    pub(crate) fn new(
        parent: &'a GScope<'a, V>,
        name: &'a str,
        view: View<'a>,
    ) -> GViewScope<'a, V> {
        GViewScope { parent, name, view }
    }

    fn get_view_by_name(&self, name: &str) -> View<'a> {
        if self.name == name {
            self.view
        } else {
            self.parent.get_view_by_name(name)
        }
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, ScopeEntry<V>)>) {
        bindings.push((
            self.name.to_string().into(),
            ScopeEntry::View(self.view.offset),
        ));
        self.parent.get_bindings(bindings);
    }
}

pub type Scope<'a> = GScope<'a, Value>;
pub type MultiScope<'a> = GMultiScope<'a, Value>;
pub type SingleScope<'a> = GSingleScope<'a, Value>;
pub type DecoderScope<'a> = GDecoderScope<'a, Value>;
pub type ViewScope<'a> = GViewScope<'a, Value>;

pub type LocScope<'a> = GScope<'a, ParsedValue>;
pub type LocMultiScope<'a> = GMultiScope<'a, ParsedValue>;
pub type LocSingleScope<'a> = GSingleScope<'a, ParsedValue>;
pub type LocDecoderScope<'a> = GDecoderScope<'a, ParsedValue>;
pub type LocViewScope<'a> = GViewScope<'a, ParsedValue>;
pub type LocScopeEntry = ScopeEntry<ParsedValue>;
