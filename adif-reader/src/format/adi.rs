mod data;
mod error;
mod header;
mod record;
mod tag;

use crate::format::adi::{header::Header, record::Record, tag::Tag};

pub use error::{AdiError, TagError};

#[derive(Debug, Clone)]
pub struct AdiDocument<'a> {
    pub header: Option<Header<'a>>,
    pub records: Vec<Record<'a>>,
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
