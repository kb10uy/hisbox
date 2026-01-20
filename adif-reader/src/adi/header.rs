use std::{collections::HashMap, sync::LazyLock};

use regex::{Regex, RegexBuilder};

use crate::adi::{error::AdiError, field::parse_field};

static RE_EOR: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r#"^[^<]*<EOH>"#)
        .case_insensitive(true)
        .build()
        .expect("regex error")
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    pub preamble: &'a str,
    pub fields: HashMap<&'a str, &'a str>,
}

pub fn parse_header<'a>(text: &'a str) -> Result<(Option<Header<'a>>, usize), AdiError> {
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
    let eor_consumed = loop {
        if let Some(eor) = RE_EOR.captures(&text[consumed..]) {
            break eor.get(0).expect("must capture").end();
        }
        let (field, field_consumed) = match parse_field(&text[consumed..]) {
            Ok(f) => f,
            Err(e) => return Err(AdiError::Tag(consumed, e)),
        };
        fields.insert(field.name, field.value);
        consumed += field_consumed;
    };
    consumed += eor_consumed;

    Ok((Some(Header { preamble, fields }), consumed))
}

#[cfg(test)]
mod tests {
    use super::{Header, parse_header};

    #[test]
    fn parses_header() {
        let adi_text = include_str!("../../fixtures/basic-header.adi");

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
        assert_eq!(parse_header(adi_text), Ok((Some(expected), 122)));
    }
}
