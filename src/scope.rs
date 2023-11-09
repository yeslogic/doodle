use crate::{Value, ValueType, Decoder, error::ParseResult, ReadCtxt, Program};

pub struct TypeScope {
    names: Vec<String>,
    types: Vec<ValueType>,
}

pub struct Scope {
    names: Vec<String>,
    values: Vec<Value>,
    decoders: Vec<Option<Decoder>>,
}

pub struct ScopeIter {
    name_iter: std::vec::IntoIter<String>,
    value_iter: std::vec::IntoIter<Value>,
}

impl Iterator for ScopeIter {
    type Item = (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.name_iter.next(), self.value_iter.next()) {
            (Some(name), Some(value)) => Some((name, value)),
            _ => None,
        }
    }
}

impl IntoIterator for &Scope {
    type Item = (String, Value);

    type IntoIter = ScopeIter;

    fn into_iter(self) -> Self::IntoIter {
        ScopeIter {
            name_iter: self.names.clone().into_iter(),
            value_iter: self.values.clone().into_iter(),
        }
    }
}

impl Scope {
    pub fn iter(&self) -> impl Iterator<Item = (String, Value)> {
        (&self).into_iter()
    }
}

impl TypeScope {
    pub fn new() -> Self {
        let names = Vec::new();
        let types = Vec::new();
        TypeScope { names, types }
    }

    pub fn push(&mut self, name: String, t: ValueType) {
        self.names.push(name);
        self.types.push(t);
    }

    pub fn pop(&mut self) -> ValueType {
        self.names.pop();
        self.types.pop().unwrap()
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn truncate(&mut self, len: usize) {
        self.names.truncate(len);
        self.types.truncate(len);
    }

    pub fn get_type_by_name(&self, name: &str) -> &ValueType {
        for (i, n) in self.names.iter().enumerate().rev() {
            if n == name {
                return &self.types[i];
            }
        }
        panic!("variable not found: {name}");
    }
}

impl Scope {
    pub fn new() -> Self {
        let names = Vec::new();
        let values = Vec::new();
        let decoders = Vec::new();
        Scope {
            names,
            values,
            decoders,
        }
    }

    pub fn push(&mut self, name: String, v: Value) {
        self.names.push(name);
        self.values.push(v);
        self.decoders.push(None);
    }

    pub fn pop(&mut self) -> Value {
        self.names.pop();
        self.decoders.pop();
        self.values.pop().unwrap()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn truncate(&mut self, len: usize) {
        self.names.truncate(len);
        self.values.truncate(len);
        self.decoders.truncate(len);
    }

    pub fn extend(&mut self, other: Scope) {
        self.names.extend(other.names);
        self.values.extend(other.values);
        self.decoders.extend(other.decoders);
    }

    pub fn get_index_by_name(&self, name: &str) -> usize {
        for (i, n) in self.names.iter().enumerate().rev() {
            if n == name {
                return i;
            }
        }
        panic!("variable not found: {name}");
    }

    pub fn get_value_by_name(&self, name: &str) -> &Value {
        &self.values[self.get_index_by_name(name)]
    }

    pub(crate) fn call_decoder_by_name<'input>(
        &mut self,
        name: &str,
        program: &Program,
        input: ReadCtxt<'input>,
    ) -> ParseResult<(Value, ReadCtxt<'input>)> {
        let i = self.get_index_by_name(name);
        let mut od = std::mem::replace(&mut self.decoders[i], None);
        if od.is_none() {
            let d = match &self.values[i] {
                Value::Format(f) => Decoder::compile_one(&*f).unwrap(),
                _ => panic!("variable not format: {name}"),
            };
            od = Some(d);
        }
        let res = od.as_ref().unwrap().parse(program, self, input);
        self.decoders[i] = od;
        res
    }
}
