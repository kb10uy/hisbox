use rhai::{CustomType, EvalAltResult, Module, export_module, plugin::*};
use time::{
    OffsetDateTime, UtcOffset, error::Parse as TimeParseError,
    format_description::well_known::Rfc3339,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SchopDateTime(OffsetDateTime);

#[allow(clippy::wrong_self_convention)]
impl SchopDateTime {
    fn new(dt_str: &str) -> Result<SchopDateTime, Box<EvalAltResult>> {
        Ok(OffsetDateTime::parse(dt_str, &Rfc3339)
            .map_err(map_parse_error)?
            .into())
    }

    fn to_utc(&mut self) -> SchopDateTime {
        self.0.to_offset(UtcOffset::UTC).into()
    }
}

impl CustomType for SchopDateTime {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("DateTime")
            .with_fn("parse_datetime", Self::new)
            .with_fn("to_utc", Self::to_utc);
    }
}

impl From<OffsetDateTime> for SchopDateTime {
    fn from(value: OffsetDateTime) -> Self {
        SchopDateTime(value)
    }
}

impl From<SchopDateTime> for OffsetDateTime {
    fn from(value: SchopDateTime) -> Self {
        value.0
    }
}

fn map_parse_error(e: TimeParseError) -> Box<EvalAltResult> {
    Box::new(EvalAltResult::ErrorSystem(
        "failed to parse datetime str".into(),
        e.into(),
    ))
}

#[export_module]
pub mod module {
    pub type DateTime = SchopDateTime;

    pub fn parse_rfc3339(dt_str: &str) -> Result<SchopDateTime, String> {
        Ok(OffsetDateTime::parse(dt_str, &Rfc3339)
            .map_err(|_| "あああ".to_string())?
            .into())
    }
}
