use roxmltree::Error as XmlError;
use thiserror::Error as ThisError;

pub use crate::format::{adi::error::AdiError, adx::error::AdxError};

#[derive(Debug, ThisError)]
pub enum AdifError {
    #[error("ADI error {0}")]
    Adi(#[from] AdiError),

    #[error("ADX error {0}")]
    Adx(#[from] AdxError),

    #[error("XML error {0}")]
    Xml(#[from] XmlError),
}
