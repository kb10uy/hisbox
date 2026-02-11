pub mod band;
pub mod contest;
pub mod error;
pub mod record;

use adif_reader::document::Record;
use num::{
    Rational64,
    traits::{FloatErrorKind, ParseFloatError},
};

use crate::error::QsoError;

fn get_required_field<'a>(record: &'a Record, field: &'static str) -> Result<&'a str, QsoError> {
    record.field(field).ok_or(QsoError::MissingAdifField(field))
}

fn get_optional_field_oneof<'a>(record: &'a Record, fields: &[&'static str]) -> Option<&'a str> {
    assert!(!fields.is_empty(), "fields must be specified");
    fields.iter().find_map(|f| record.field(*f))
}

fn parse_decimal(s: &str) -> Result<Rational64, ParseFloatError> {
    let mut parts = s.split('.');
    let int_part = parts.next().expect("first part must exist");
    let fract_part = parts.next();
    let None = parts.next() else {
        return Err(ParseFloatError {
            kind: FloatErrorKind::Invalid,
        });
    };

    let int_value: i64 = match int_part {
        "" => 0,
        v => v.parse().map_err(|_| ParseFloatError {
            kind: FloatErrorKind::Empty,
        })?,
    };
    let (fract_value, fract_digits) = match fract_part {
        Some("") | None => (0, 0),
        Some(v) => (
            v.parse().map_err(|_| ParseFloatError {
                kind: FloatErrorKind::Empty,
            })?,
            v.len(),
        ),
    };

    let denom = 10i64.pow(fract_digits as u32);
    let numer = int_value * denom + fract_value;
    Ok(Rational64::new(numer, denom))
}

#[cfg(test)]
mod tests {
    use num::Rational64;

    use super::parse_decimal;

    #[test]
    fn parse_decimal_parses() {
        assert_eq!(parse_decimal("1").unwrap(), Rational64::new(1, 1));
        assert_eq!(parse_decimal("10").unwrap(), Rational64::new(10, 1));
        assert_eq!(parse_decimal("3.14").unwrap(), Rational64::new(314, 100));
        assert_eq!(parse_decimal("3.").unwrap(), Rational64::new(3, 1));
        assert_eq!(parse_decimal(".5").unwrap(), Rational64::new(5, 10));
    }
}
