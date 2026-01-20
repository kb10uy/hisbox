use std::collections::HashMap;

use roxmltree::{Node, NodeType};

use crate::format::adx::{error::AdxError, field_name::FieldName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    pub fields: HashMap<FieldName<'a>, String>,
}

impl<'a, 'i: 'a> Header<'a> {
    pub fn new(header_element: Node<'a, 'i>) -> Result<Header<'a>, AdxError> {
        let fields: Result<_, AdxError> = header_element
            .children()
            .map(|c| {
                let NodeType::Element = c.node_type() else {
                    return Ok(None);
                };

                let tag_name = c.tag_name().name();
                let value = c.children().flat_map(|n| n.text()).collect();
                if tag_name == "USERDEF" {
                    let id = c
                        .attribute("FIELDID")
                        .ok_or(AdxError::RequiredField)?
                        .parse()?;
                    Ok(Some((FieldName::UserdefHeader(id), value)))
                } else {
                    Ok(Some((FieldName::Defined(tag_name), value)))
                }
            })
            .flat_map(|ro| ro.transpose())
            .collect();

        Ok(Header { fields: fields? })
    }
}

#[cfg(test)]
mod tests {
    use roxmltree::{Document, Node};

    use crate::format::adx::field_name::FieldName;

    use super::Header;

    fn example_adx() -> Document<'static> {
        Document::parse(include_str!("../../../fixtures/example.adx")).unwrap()
    }

    fn find_header<'a>(document: &'a Document<'static>) -> Node<'a, 'static> {
        document
            .root_element()
            .children()
            .find(|n| n.tag_name().name().to_uppercase() == "HEADER")
            .expect("example must have header")
    }

    #[test]
    fn parses_header() {
        let adx = example_adx();
        let header_element = find_header(&adx);
        let header = Header::new(header_element);
        assert_eq!(
            header,
            Ok(Header {
                fields: vec![
                    (FieldName::Defined("ADIF_VER"), "3.0.5".to_string()),
                    (FieldName::Defined("PROGRAMID"), "monolog".to_string()),
                    (FieldName::UserdefHeader(1), "EPC".to_string()),
                    (FieldName::UserdefHeader(2), "SWEATERSIZE".to_string()),
                    (FieldName::UserdefHeader(3), "SHOESIZE".to_string()),
                ]
                .into_iter()
                .collect()
            })
        )
    }
}
