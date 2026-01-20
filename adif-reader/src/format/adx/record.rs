use std::collections::HashMap;

use roxmltree::{Node, NodeType};

use crate::format::adx::{error::AdxError, field_name::FieldName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<'a> {
    pub fields: HashMap<FieldName<'a>, String>,
}

impl<'a, 'i: 'a> Record<'a> {
    pub fn new(record_element: Node<'a, 'i>) -> Result<Record<'a>, AdxError> {
        let fields: Result<_, AdxError> = record_element
            .children()
            .map(|c| {
                let NodeType::Element = c.node_type() else {
                    return Ok(None);
                };

                let tag_name = c.tag_name().name();
                let value = c.children().flat_map(|n| n.text()).collect();
                match tag_name {
                    "USERDEF" => {
                        let name = c.attribute("FIELDNAME").ok_or(AdxError::RequiredField)?;
                        Ok(Some((FieldName::UserdefRecord(name.to_uppercase()), value)))
                    }
                    "APP" => {
                        let program_id = c.attribute("PROGRAMID").ok_or(AdxError::RequiredField)?;
                        let field_name = c.attribute("FIELDNAME").ok_or(AdxError::RequiredField)?;
                        Ok(Some((
                            FieldName::AppRecord {
                                program_id,
                                field_name: field_name.to_uppercase(),
                            },
                            value,
                        )))
                    }
                    _ => Ok(Some((FieldName::Defined(tag_name), value))),
                }
            })
            .flat_map(|ro| ro.transpose())
            .collect();

        Ok(Record { fields: fields? })
    }
}

#[cfg(test)]
mod tests {
    use roxmltree::{Document, Node};

    use crate::format::adx::field_name::FieldName;

    use super::Record;

    fn example_adx() -> Document<'static> {
        Document::parse(include_str!("../../../fixtures/example.adx")).unwrap()
    }

    fn find_records<'a>(document: &'a Document<'static>) -> Node<'a, 'static> {
        document
            .root_element()
            .children()
            .find(|n| n.tag_name().name().to_uppercase() == "RECORDS")
            .expect("example must have header")
    }

    #[test]
    fn parses_records() {
        let adx = example_adx();
        let records_element = find_records(&adx);
        let record_element = records_element.first_element_child().unwrap();
        let record = Record::new(record_element);
        assert_eq!(
            record,
            Ok(Record {
                fields: vec![
                    (FieldName::Defined("QSO_DATE"), "19900620".to_string()),
                    (FieldName::Defined("TIME_ON"), "1523".to_string()),
                    (FieldName::Defined("CALL"), "VK9NS".to_string()),
                    (FieldName::Defined("BAND"), "20M".to_string()),
                    (FieldName::Defined("MODE"), "RTTY".to_string()),
                    (
                        FieldName::UserdefRecord("SWEATERSIZE".to_string()),
                        "M".to_string(),
                    ),
                    (
                        FieldName::UserdefRecord("SHOESIZE".to_string()),
                        "11".to_string(),
                    ),
                    (
                        FieldName::AppRecord {
                            program_id: "MONOLOG",
                            field_name: "COMPRESSION".to_string(),
                        },
                        "off".to_string(),
                    ),
                ]
                .into_iter()
                .collect()
            })
        )
    }
}
