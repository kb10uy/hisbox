use rhai::{CustomType, EvalAltResult};
use time::{
    OffsetDateTime, UtcOffset, error::Parse as TimeParseError,
    format_description::well_known::Rfc3339,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ScopDateTime(OffsetDateTime);

// #[allow(clippy::wrong_self_convention)]
impl ScopDateTime {
    fn new(dt_str: &str) -> Result<ScopDateTime, Box<EvalAltResult>> {
        Ok(OffsetDateTime::parse(dt_str, &Rfc3339)
            .map_err(map_parse_error)?
            .into())
    }

    fn to_utc(&mut self) -> ScopDateTime {
        self.0.to_offset(UtcOffset::UTC).into()
    }
}

impl CustomType for ScopDateTime {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("DateTime")
            .with_fn("parse_datetime", Self::new)
            .with_fn("to_utc", Self::to_utc);
    }
}

impl From<OffsetDateTime> for ScopDateTime {
    fn from(value: OffsetDateTime) -> Self {
        ScopDateTime(value)
    }
}

impl From<ScopDateTime> for OffsetDateTime {
    fn from(value: ScopDateTime) -> Self {
        value.0
    }
}

fn map_parse_error(e: TimeParseError) -> Box<EvalAltResult> {
    Box::new(EvalAltResult::ErrorSystem(
        "failed to parse datetime str".into(),
        e.into(),
    ))
}
