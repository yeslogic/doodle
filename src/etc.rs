pub type Label = std::borrow::Cow<'static, str>;

pub trait IntoLabel: Into<Label> {}

impl<T> IntoLabel for T where T: Into<Label> {}
