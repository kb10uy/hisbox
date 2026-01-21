mod data;
pub mod error;
mod header;
mod record;
mod tag;

use crate::{
    document::{AdifDocument, IntoAdifDocument},
    format::adi::{error::AdiError, header::Header, record::Record, tag::Tag},
};

#[derive(Debug, Clone)]
pub struct AdiDocument<'a> {
    header: Option<Header<'a>>,
    records: Vec<Record<'a>>,
}

impl<'a> AdiDocument<'a> {
    pub fn parse(text: &'a str) -> Result<AdiDocument<'a>, AdiError> {
        let mut consumed = 0;

        let (header, header_consumed) = Header::parse(&text[consumed..])?;
        consumed += header_consumed;

        let mut records = vec![];
        while Tag::has_next(&text[consumed..]) {
            let (record, record_consumed) = Record::parse(&text[consumed..])?;
            consumed += record_consumed;
            records.push(record);
        }

        Ok(AdiDocument { header, records })
    }
}

impl<'a> IntoAdifDocument for AdiDocument<'a> {
    fn into_adif_document(self) -> AdifDocument {
        let (preamble, headers) = match self.header {
            Some(h) => (
                h.preamble,
                Some(
                    h.fields
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v.to_string())),
                ),
            ),
            None => ("", None),
        };
        let records = self.records.into_iter().map(|r| {
            r.fields
                .into_iter()
                .map(|(k, v)| (k.as_str().to_string(), v.to_string()))
        });
        AdifDocument::new(preamble.to_string(), headers.into_iter().flatten(), records)
    }
}
