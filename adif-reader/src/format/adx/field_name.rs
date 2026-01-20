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
