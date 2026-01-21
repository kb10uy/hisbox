use std::collections::HashMap;

use crate::format::adi::{
    data::{FieldValue, LengthMode, get_field_value},
    error::AdiError,
    tag::Tag,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    pub preamble: &'a str,
    pub fields: HashMap<&'a str, &'a str>,
}

impl<'a> Header<'a> {
    pub fn parse(
        text: &'a str,
        length_mode: LengthMode,
    ) -> Result<(Option<Header<'a>>, usize), AdiError> {
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
                    let value = match get_field_value(&text[consumed..], length_mode, value_length)
                    {
                        FieldValue::Found(v) => v,
                        FieldValue::InvalidBoundary => {
                            return Err(AdiError::CharacterBoundary(consumed));
                        }
                        FieldValue::NotEnough => {
                            return Err(AdiError::ValueTooShort {
                                expected: value_length,
                                maximum: text.len() - consumed,
                            });
                        }
                    };
                    fields.insert(name, value);
                    consumed += value.len();
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
}

#[cfg(test)]
mod tests {
    use crate::format::adi::data::LengthMode;

    use super::Header;

    #[test]
    fn parses_header() {
        let adi_text = include_str!("../../../fixtures/basic-header.adi");

        let expected = Header {
            preamble: "Fixture ADI File\n",
            fields: vec![
                ("ADIF_VER", "3.1.6"),
                ("CREATED_TIMESTAMP", "20260120 000000"),
                ("PROGRAMID", "jelgen"),
                ("PROGRAMVERSION", "0.1.0"),
            ]
            .into_iter()
            .collect(),
        };
        assert_eq!(
            Header::parse(adi_text, LengthMode::Bytes),
            Ok((Some(expected), 122))
        );
    }
}
