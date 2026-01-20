mod field_name;

use std::collections::HashMap;

pub use field_name::{FieldName, ToFieldName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdifDocument {
    preamble: String,
    headers: HashMap<String, String>,
    records: Vec<HashMap<String, String>>,
}
