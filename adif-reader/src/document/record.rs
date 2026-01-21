use std::collections::HashMap;

use crate::document::FieldName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    fields: HashMap<String, String>,
}

impl Record {
    pub(super) fn new<R: IntoIterator<Item = (String, String)>>(fields: R) -> Record {
        let fields = fields
            .into_iter()
            .map(|(mut k, v)| {
                k.make_ascii_uppercase();
                (k, v)
            })
            .collect();

        Record { fields }
    }

    pub fn field<'a, F: Into<FieldName<'a>>>(&self, name: F) -> Option<&str> {
        let field_name = name.into();
        self.fields.get(field_name.as_str()).map(|s| s.as_str())
    }
}
