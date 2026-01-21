use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldName<'a> {
    Defined(&'a str),
    UserdefHeader(usize),
    UserdefRecord(String),
    AppRecord {
        program_id: &'a str,
        field_name: String,
    },
}

impl<'a> Display for FieldName<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FieldName::Defined(n) => write!(f, "{n}"),
            FieldName::UserdefHeader(i) => write!(f, "USERDEF{i}"),
            FieldName::UserdefRecord(n) => write!(f, "{n}"),
            FieldName::AppRecord { field_name, .. } => write!(f, "{field_name}"),
        }
    }
}
