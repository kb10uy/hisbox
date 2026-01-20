use std::collections::HashMap;

use crate::adi::{
    data::{FieldName, ToFieldName, get_field_value},
    error::AdiError,
    tag::Tag,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    preamble: &'a str,
    fields: HashMap<FieldName<'a>, &'a str>,
}

impl<'a> Header<'a> {
    pub fn parse(text: &'a str) -> Result<(Option<Header<'a>>, usize), AdiError> {
        // > If the first character in an ADI file is <, it contains no Header.
        // https://adif.org.uk/316/ADIF_316.htm#ADI_File_Format
        let header_start = match text.find("<") {
            Some(0) => return Ok((None, 0)),
            Some(n) => n,
            None => return Err(AdiError::NoData),
        };
        let preamble = &text[..header_start];

        let mut fields = HashMap::new();
        let mut consumed = header_start;
        loop {
            match Tag::parse(&text[consumed..]) {
                Ok((
                    Tag::Specifier {
                        name, value_length, ..
                    },
                    c,
                )) => {
                    consumed += c;
                    let value = get_field_value(&text[consumed..], value_length).ok_or(
                        AdiError::ValueTooShort {
                            expected: value_length,
                            maximum: text.len() - consumed,
                        },
                    )?;
                    fields.insert(FieldName::new(name), value);
                    consumed += value_length;
                }
                Ok((Tag::EndOfHeader, c)) => {
                    consumed += c;
                    break;
                }

                Ok(_) => return Err(AdiError::NoEoh),
                Err(e) => return Err(AdiError::Tag(consumed, e)),
            }
        }

        Ok((Some(Header { preamble, fields }), consumed))
    }

    pub fn preamble(&self) -> &'a str {
        self.preamble
    }

    pub fn field<F: ToFieldName<'a>>(&self, f: F) -> Option<&str> {
        let field_name = f.to_field_name();
        self.fields.get(&field_name).copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::adi::data::ToFieldName;

    use super::Header;

    #[test]
    fn parses_header() {
        let adi_text = include_str!("../../fixtures/basic-header.adi");

        let expected = Header {
            preamble: "Fixture ADI File\n",
            fields: vec![
                ("ADIF_VER".to_field_name(), "3.1.6"),
                ("CREATED_TIMESTAMP".to_field_name(), "20260120 000000"),
                ("PROGRAMID".to_field_name(), "jelgen"),
                ("PROGRAMVERSION".to_field_name(), "0.1.0"),
            ]
            .into_iter()
            .collect(),
        };
        assert_eq!(Header::parse(adi_text), Ok((Some(expected), 122)));
    }
}
