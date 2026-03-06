use thiserror::Error as ThisError;

#[derive(Debug, Clone, PartialEq, Eq, ThisError)]
pub enum GridLocatorError {
    #[error("invalid text")]
    InvalidText,

    #[error("invalid length")]
    InvalidLength,

    #[error("out of range")]
    OutOfRange,
}
