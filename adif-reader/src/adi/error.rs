use std::num::ParseIntError;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError, PartialEq, Eq)]
pub enum AdiError {
    #[error("no data found")]
    NoData,

    #[error("no <eoh> found")]
    NoEoh,

    #[error("no <eor> found after index {0}")]
    NoEor(usize),

    #[error("tag error at index {0}: {1}")]
    Tag(usize, TagError),
}

#[derive(Debug, ThisError, PartialEq, Eq)]
pub enum TagError {
    #[error("no valid tag found")]
    NoValidTag,

    #[error("invalid length: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("value too short; expected {expected}, max {maximum}")]
    ValueTooShort { expected: usize, maximum: usize },
}
