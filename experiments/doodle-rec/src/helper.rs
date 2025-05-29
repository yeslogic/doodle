use crate::{Expr, Format, Label};
use doodle::prelude::ByteSet;

pub fn tuple(formats: impl IntoIterator<Item = Format>) -> Format {
    Format::Tuple(formats.into_iter().collect())
}

pub fn is_byte(x: u8) -> Format {
    Format::Byte(ByteSet::from([x]))
}

pub fn alts<Name: Into<Label>>(branches: impl IntoIterator<Item = (Name, Format)>) -> Format {
    Format::Union(
        branches
            .into_iter()
            .map(|(name, f)| Format::Variant(name.into(), Box::new(f)))
            .collect(),
    )
}

pub fn byte_seq(bytes: impl IntoIterator<Item = u8>) -> Format {
    Format::Seq(bytes.into_iter().map(is_byte).collect())
}

pub fn repeat(format: Format) -> Format {
    Format::Repeat(Box::new(format))
}

pub fn var(ix: usize) -> Format {
    Format::RecVar(ix)
}

pub fn repeat1(format: Format) -> Format {
    tuple([format.clone(), repeat(format.clone())])
}
