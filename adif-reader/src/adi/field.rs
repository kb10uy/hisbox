use std::sync::LazyLock;

use regex::{Regex, RegexBuilder};

use crate::adi::error::TagError;

static RE_FIELD_TAG: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r#"<([^,:<>\{\}]+):(\d+)(:([A-Z]+))?>"#)
        .case_insensitive(true)
        .build()
        .expect("regex error")
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field<'a> {
    name: &'a str,
    type_indicator: Option<&'a str>,
    value: &'a str,
}

pub fn parse_field<'a>(text: &'a str) -> Result<(Field<'a>, usize), TagError> {
    let Some(tag_match) = RE_FIELD_TAG.captures(text) else {
        return Err(TagError::NoValidTag);
    };
    let tag_end = tag_match.get(0).expect("must capture").end();
    let field_name = tag_match.get(1).expect("capture must exist");
    let value_length = tag_match.get(2).expect("capture must exist");
    let type_indicator = tag_match.get(4);

    let value_length = value_length.as_str().parse::<usize>()?;
    let after_tag = &text[tag_end..];
    if after_tag.len() >= value_length {
        Ok((
            Field {
                name: field_name.as_str(),
                type_indicator: type_indicator.map(|t| t.as_str()),
                value: &after_tag[..value_length],
            },
            tag_end + value_length,
        ))
    } else {
        Err(TagError::ValueTooShort {
            expected: value_length,
            maximum: after_tag.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Field, TagError, parse_field};

    #[test]
    fn parses_tags() {
        assert_eq!(
            parse_field("<CALL:6>JL1HIS"),
            Ok((
                Field {
                    name: "CALL",
                    type_indicator: None,
                    value: "JL1HIS",
                },
                14
            ))
        );

        assert_eq!(
            parse_field("<CALL:6:S>JL1HIS"),
            Ok((
                Field {
                    name: "CALL",
                    type_indicator: Some("S"),
                    value: "JL1HIS",
                },
                16
            ))
        );
    }

    #[test]
    fn fails_on_tag_errors() {
        assert_eq!(parse_field("CALL"), Err(TagError::NoValidTag));
        assert_eq!(parse_field("<CALL:"), Err(TagError::NoValidTag));
        assert_eq!(parse_field("<CALL>"), Err(TagError::NoValidTag));
        assert_eq!(parse_field("<CALL:6:S:X>JL1HIS"), Err(TagError::NoValidTag));
        assert_eq!(
            parse_field("<CALL:6>JL1HI"),
            Err(TagError::ValueTooShort {
                expected: 6,
                maximum: 5
            })
        );
    }
}
