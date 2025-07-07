use crate::typecheck::UnificationError;
use anyhow::{anyhow, Result as AResult};
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};

use super::Label;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash, PartialOrd, Ord)]
pub enum BaseType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    Char,
}

impl BaseType {
    pub(crate) fn is_numeric(&self) -> bool {
        matches!(self, Self::U8 | Self::U16 | Self::U32 | Self::U64)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum ValueType {
    Any,
    Empty,
    Base(BaseType),
    Tuple(Vec<ValueType>),
    Record(Vec<(Label, ValueType)>),
    Union(BTreeMap<Label, ValueType>),
    Seq(Box<ValueType>),
    Option(Box<ValueType>),
}

impl From<BaseType> for ValueType {
    fn from(b: BaseType) -> Self {
        ValueType::Base(b)
    }
}

impl ValueType {
    pub const BOOL: ValueType = ValueType::Base(BaseType::Bool);

    pub const UNIT: ValueType = ValueType::Tuple(Vec::new());

    pub(crate) fn record_proj(&self, label: &str) -> ValueType {
        match self {
            ValueType::Record(fields) => match fields.iter().find(|(l, _)| label == l) {
                Some((_, t)) => t.clone(),
                None => panic!("{label} not found in record type"),
            },
            _ => panic!("expected record type"),
        }
    }

    pub(crate) fn unwrap_tuple_type(self) -> AResult<Vec<ValueType>> {
        match self {
            ValueType::Tuple(ts) => Ok(ts),
            t => Err(anyhow!("type is not a tuple: {t:?}")),
        }
    }

    pub(crate) fn as_tuple_type(&self) -> &[ValueType] {
        match self {
            ValueType::Tuple(ts) => ts.as_slice(),
            other => panic!("type is not a tuple: {other:?}"),
        }
    }

    pub fn is_equivalent(&self, other: &ValueType) -> Result<(), UnificationError<ValueType>> {
        self.unify(other)?;
        Ok(())
    }

    pub(crate) fn unify(
        &self,
        other: &ValueType,
    ) -> Result<ValueType, UnificationError<ValueType>> {
        match (self, other) {
            (ValueType::Empty, ValueType::Empty) => Ok(ValueType::Empty),

            // NOTE - we have to specify these patterns before the similar cases for Empty because we want (Empty, Any) in either order to yield Empty
            (ValueType::Any, rhs) => Ok(rhs.clone()),
            (lhs, ValueType::Any) => Ok(lhs.clone()),

            (ValueType::Empty, rhs) => Ok(rhs.clone()),
            (lhs, ValueType::Empty) => Ok(lhs.clone()),

            (ValueType::Base(b1), ValueType::Base(b2)) => {
                if b1 == b2 {
                    Ok(ValueType::Base(*b1))
                } else {
                    Err(UnificationError::Unsatisfiable(self.clone(), other.clone()))
                }
            }
            (ValueType::Tuple(ts1), ValueType::Tuple(ts2)) => {
                if ts1.len() != ts2.len() {
                    // tuple arity mismatch
                    return Err(UnificationError::Unsatisfiable(self.clone(), other.clone()));
                }
                let mut ts = Vec::new();
                for (t1, t2) in Iterator::zip(ts1.iter(), ts2.iter()) {
                    ts.push(t1.unify(t2)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            (ValueType::Record(fs1), ValueType::Record(fs2)) => {
                if fs1.len() != fs2.len() {
                    // field count mismatch
                    return Err(UnificationError::Unsatisfiable(self.clone(), other.clone()));
                }
                // NOTE - because fields are parsed in declared order, two records with conflicting field orders are not operationally equivalent
                let mut fs = Vec::new();
                for ((l1, t1), (l2, t2)) in Iterator::zip(fs1.iter(), fs2.iter()) {
                    if l1 != l2 {
                        // field label mismatch
                        return Err(UnificationError::Unsatisfiable(self.clone(), other.clone()));
                    }
                    fs.push((l1.clone(), t1.unify(t2)?));
                }
                Ok(ValueType::Record(fs))
            }
            (ValueType::Union(bs1), ValueType::Union(bs2)) => {
                let mut bs: BTreeMap<Label, ValueType> = BTreeMap::new();

                let keys1 = bs1.keys().collect::<HashSet<_>>();
                let keys2 = bs2.keys().collect::<HashSet<_>>();

                let keys_common = HashSet::union(&keys1, &keys2).cloned();

                for key in keys_common.into_iter() {
                    match (bs1.get(key), bs2.get(key)) {
                        (Some(t1), Some(t2)) => {
                            let t = t1.unify(t2)?;
                            bs.insert(key.clone(), t);
                        }
                        (Some(t), None) | (None, Some(t)) => {
                            bs.insert(key.clone(), t.clone());
                        }
                        (None, None) => unreachable!("key must appear in at least one operand"),
                    }
                }

                Ok(ValueType::Union(bs))
            }
            (ValueType::Seq(t1), ValueType::Seq(t2)) => Ok(ValueType::Seq(Box::new(t1.unify(t2)?))),
            (ValueType::Option(t1), ValueType::Option(t2)) => {
                Ok(ValueType::Option(Box::new(t1.unify(t2)?)))
            }
            (t1, t2) => Err(UnificationError::Unsatisfiable(t1.clone(), t2.clone())),
        }
    }
}

/// Alias to reduce the number of code-sites we need to update if we pick a different Smart-Pointer type
/// as the backer of `TypeHint`
pub(crate) type Container<T> = std::rc::Rc<T>; // Box<T>;

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeHint(Container<ValueType>);

impl TypeHint {
    pub fn into_inner(&self) -> &Container<ValueType> {
        &self.0
    }
}

impl AsRef<ValueType> for TypeHint {
    fn as_ref(&self) -> &ValueType {
        self.0.as_ref()
    }
}

impl Serialize for TypeHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl From<ValueType> for TypeHint {
    fn from(t: ValueType) -> Self {
        Self(Container::new(t))
    }
}
