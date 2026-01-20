use std::sync::LazyLock;

use regex::{Regex, RegexBuilder};

use crate::adi::error::TagError;

static RE_FIELD_TAG: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r#"<((EOH)|(EOR)|([^,:<>\{\}]+):(\d+)(:([A-Z]+))?)>"#)
        .case_insensitive(true)
        .build()
        .expect("regex error")
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag<'a> {
    EndOfHeader,
    EndOfRecord,
    Specifier {
        name: &'a str,
        value_length: usize,
        type_indicator: Option<&'a str>,
    },
}

impl<'a> Tag<'a> {
    pub fn parse(text: &'a str) -> Result<(Tag<'a>, usize), TagError> {
        let Some(tag_match) = RE_FIELD_TAG.captures(text) else {
            return Err(TagError::NotValidTag);
        };
        let tag_end = tag_match.get(0).expect("must capture").end();

        if tag_match.get(2).is_some() {
            return Ok((Tag::EndOfHeader, tag_end));
        } else if tag_match.get(3).is_some() {
            return Ok((Tag::EndOfRecord, tag_end));
        }

        let field_name = tag_match.get(4).expect("capture must exist");
        let value_length = tag_match.get(5).expect("capture must exist");
        let type_indicator = tag_match.get(7);

        Ok((
            Tag::Specifier {
                name: field_name.as_str(),
                value_length: value_length.as_str().parse()?,
                type_indicator: type_indicator.map(|m| m.as_str()),
            },
            tag_end,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;

    #[test]
    fn parses_ends() {
        assert_eq!(Tag::parse("<EOH>"), Ok((Tag::EndOfHeader, 5)));
        assert_eq!(Tag::parse("<eoh>"), Ok((Tag::EndOfHeader, 5)));
        assert_eq!(Tag::parse("<EOR>"), Ok((Tag::EndOfRecord, 5)));
        assert_eq!(Tag::parse("<eor>"), Ok((Tag::EndOfRecord, 5)));
    }

    #[test]
    fn parse_specifier() {
        assert_eq!(
            Tag::parse("<CALL:6>"),
            Ok((
                Tag::Specifier {
                    name: "CALL",
                    value_length: 6,
                    type_indicator: None
                },
                8,
            ))
        );
        assert_eq!(
            Tag::parse("<CALL:6:S>"),
            Ok((
                Tag::Specifier {
                    name: "CALL",
                    value_length: 6,
                    type_indicator: Some("S"),
                },
                10,
            ))
        );
    }
}
