mod error;
mod field_name;
mod header;
mod record;

use roxmltree::{Document, NodeType};

use crate::format::adx::{error::AdxError, header::Header, record::Record};

#[derive(Debug, Clone)]
pub struct AdxDocument<'a> {
    pub header: Header<'a>,
    pub records: Vec<Record<'a>>,
}

impl<'a, 'i: 'a> AdxDocument<'a> {
    pub fn parse(document: &'a Document<'i>) -> Result<AdxDocument<'a>, AdxError> {
        let root = document.root_element();
        if root.tag_name().name() != "ADX" {
            return Err(AdxError::NoAdx);
        }

        let mut header_element = None;
        let mut records_element = None;
        for child in root.children() {
            let NodeType::Element = child.node_type() else {
                continue;
            };

            match child.tag_name().name() {
                "HEADER" => {
                    header_element = Some(child);
                }
                "RECORDS" => {
                    records_element = Some(child);
                }
                _ => (),
            }
        }

        let Some(header_element) = header_element else {
            return Err(AdxError::NoHeader);
        };
        let Some(records_element) = records_element else {
            return Err(AdxError::NoRecords);
        };

        let header = Header::new(header_element)?;
        let records: Result<Vec<_>, _> = records_element
            .children()
            .filter(|c| c.node_type() == NodeType::Element && c.tag_name().name() == "RECORD")
            .map(Record::new)
            .collect();

        Ok(AdxDocument {
            header,
            records: records?,
        })
    }
}
