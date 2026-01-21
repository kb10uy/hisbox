use thiserror::Error as ThisError;
use time::error::Parse as TimeParseError;

#[derive(Debug, ThisError)]
pub enum QsoError {
    #[error("missing ADIF field: {0}")]
    MissingAdifField(&'static str),

    #[error("datetime parse error: {0}")]
    DateTimeParse(#[from] TimeParseError),
}
