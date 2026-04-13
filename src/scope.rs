use crate::decoder::{Scope, UnknownVarError, Value};
use crate::loc_decoder::{LocScope, ParsedValue};

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

impl<'a> EvalScope<'a> for Scope<'a> {
    type Output = &'a Value;
    type Error = UnknownVarError;

    fn lookup_var(&'a self, name: &str) -> Result<Self::Output, Self::Error> {
        self.get_value_by_name(name)
    }
}

impl<'a> EvalScope<'a> for LocScope<'a> {
    type Output = &'a ParsedValue;
    type Error = UnknownVarError;

    fn lookup_var(&'a self, name: &str) -> Result<Self::Output, Self::Error> {
        self.get_value_by_name(name)
    }
}
