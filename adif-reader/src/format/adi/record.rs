use std::collections::HashMap;

use crate::{
    format::adi::{data::get_field_value, error::AdiError, tag::Tag},
    document::FieldName,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<'a> {
    pub fields: HashMap<FieldName<'a>, &'a str>,
}

impl<'a> Record<'a> {
    pub fn parse(text: &'a str) -> Result<(Record<'a>, usize), AdiError> {
        let mut fields = HashMap::new();
        let mut consumed = 0;
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
                Ok((Tag::EndOfRecord, c)) => {
                    consumed += c;
                    break;
                }

                Ok(_) => return Err(AdiError::NoEor(consumed)),
                Err(e) => return Err(AdiError::Tag(consumed, e)),
            }
        }

        Ok((Record { fields }, consumed))
    }
}

#[cfg(test)]
mod tests {
    use crate::document::ToFieldName;

    use super::Record;

    #[test]
    fn parses_header() {
        let expected = Record {
            fields: vec![("CALL".to_field_name(), "JL1HIS")]
                .into_iter()
                .collect(),
        };
        assert_eq!(Record::parse("<CALL:6>JL1HIS<eor>"), Ok((expected, 19)));
    }
}
