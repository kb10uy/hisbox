pub mod band;
pub mod contest;
pub mod error;
pub mod record;

use adif_reader::document::Record;

use crate::error::QsoError;

fn get_required_field<'a>(record: &'a Record, field: &'static str) -> Result<&'a str, QsoError> {
    record.field(field).ok_or(QsoError::MissingAdifField(field))
}

fn get_optional_field_oneof<'a>(record: &'a Record, fields: &[&'static str]) -> Option<&'a str> {
    assert!(!fields.is_empty(), "fields must be specified");
    fields.iter().find_map(|f| record.field(*f))
}
