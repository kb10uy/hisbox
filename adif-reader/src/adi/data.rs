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
}

pub trait ToFieldName<'a> {
    fn to_field_name(self) -> FieldName<'a>;
}

impl<'a> ToFieldName<'a> for FieldName<'a> {
    fn to_field_name(self) -> FieldName<'a> {
        self
    }
}

impl<'a> ToFieldName<'a> for &'a str {
    fn to_field_name(self) -> FieldName<'a> {
        FieldName::new(self)
    }
}

impl<'a> ToFieldName<'a> for String {
    fn to_field_name(self) -> FieldName<'a> {
        FieldName::new_owned(self)
    }
}

/// If `text` is long enough to fetch string that has `length`, return `Some`.
pub fn get_field_value(text: &str, length: usize) -> Option<&str> {
    (text.len() >= length).then(|| &text[..length])
}
