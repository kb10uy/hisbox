mod field_name;
mod record;

use std::collections::HashMap;

pub use field_name::FieldName;
pub use record::Record;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdifDocument {
    preamble: String,
    headers: HashMap<String, String>,
    records: Vec<Record>,
}

impl AdifDocument {
    pub fn new<P, H, R, RS>(preamble: P, headers: H, records: RS) -> AdifDocument
    where
        P: Into<String>,
        H: IntoIterator<Item = (String, String)>,
        R: IntoIterator<Item = (String, String)>,
        RS: IntoIterator<Item = R>,
    {
        let preamble = preamble.into();
        let headers = headers
            .into_iter()
            .map(|(mut k, v)| {
                k.make_ascii_uppercase();
                (k, v)
            })
            .collect();
        let records = records.into_iter().map(Record::new).collect();

        AdifDocument {
            preamble,
            headers,
            records,
        }
    }

    pub fn preamble(&self) -> &str {
        &self.preamble
    }

    pub fn header<'a, F: Into<FieldName<'a>>>(&self, name: F) -> Option<&str> {
        let field_name = name.into();
        self.headers.get(field_name.as_str()).map(|s| s.as_str())
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }
}

pub trait IntoAdifDocument {
    fn into_adif_document(self) -> AdifDocument;
}
