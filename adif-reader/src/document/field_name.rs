use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldName<'a>(Cow<'a, str>);

impl<'a> FieldName<'a> {
    pub fn new(name: &'a str) -> FieldName<'a> {
        if name.chars().all(char::is_uppercase) {
            FieldName(Cow::Borrowed(name))
        } else {
            FieldName(Cow::Owned(name.to_uppercase()))
        }
    }

    pub fn new_owned(mut name: String) -> FieldName<'a> {
        name.make_ascii_uppercase();
        FieldName(Cow::Owned(name))
    }

    pub fn as_str(&'a self) -> &'a str {
        &self.0
    }
}

impl<'a> From<&'a str> for FieldName<'a> {
    fn from(value: &'a str) -> Self {
        FieldName::new(value)
    }
}

impl<'a> From<String> for FieldName<'a> {
    fn from(value: String) -> Self {
        FieldName::new_owned(value)
    }
}
