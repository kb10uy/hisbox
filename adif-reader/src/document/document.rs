use std::collections::HashMap;

use crate::adi::AdiDocument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdifDocument {
    preamble: String,
    headers: HashMap<String, String>,
    records: Vec<HashMap<String, String>>,
}
