use std::collections::HashMap;

use crate::{
    document::FieldName,
    format::adi::{
        data::{FieldValue, LengthMode, get_field_value},
        error::AdiError,
        tag::Tag,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<'a> {
    pub fields: HashMap<FieldName<'a>, &'a str>,
}

impl<'a> Record<'a> {
    pub fn parse(text: &'a str, length_mode: LengthMode) -> Result<(Record<'a>, usize), AdiError> {
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
                    fields.insert(FieldName::new(name), value);
                    consumed += value.len();
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
    use crate::format::adi::data::LengthMode;

    use super::Record;

    #[test]
    fn parses_header() {
        let expected = Record {
            fields: vec![("CALL".into(), "JL1HIS")].into_iter().collect(),
        };
        assert_eq!(
            Record::parse("<CALL:6>JL1HIS<eor>", LengthMode::Bytes),
            Ok((expected, 19))
        );
    }
}
