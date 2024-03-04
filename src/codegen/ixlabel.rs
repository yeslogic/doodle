use crate::Label;

/// Simple type for ad-hoc names using a counter value
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub(crate) struct IxLabel(usize);

impl IxLabel {
    pub(crate) fn to_usize(self) -> usize {
        self.0
    }
}

impl From<usize> for IxLabel {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<IxLabel> for Label {
    fn from(value: IxLabel) -> Self {
        Label::Owned(format!("Type{}", value.0))
    }
}

impl From<&IxLabel> for Label {
    fn from(value: &IxLabel) -> Self {
        Label::Owned(format!("Type{}", value.0))
    }
}

impl AsRef<usize> for IxLabel {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl std::borrow::Borrow<usize> for IxLabel {
    fn borrow(&self) -> &usize {
        &self.0
    }
}
