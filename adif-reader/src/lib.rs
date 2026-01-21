pub mod document;
pub mod error;
mod format;

use roxmltree::Document;

use crate::{
    document::{AdifDocument, IntoAdifDocument},
    error::AdifError,
    format::{adi::AdiDocument, adx::AdxDocument},
};

pub fn read_adi(adi_text: &str) -> Result<AdifDocument, AdifError> {
    let adi = AdiDocument::parse(adi_text)?;
    Ok(adi.into_adif_document())
}

pub fn read_adx(adx_text: &str) -> Result<AdifDocument, AdifError> {
    let xml = Document::parse(adx_text)?;
    let adx = AdxDocument::parse(&xml)?;
    Ok(adx.into_adif_document())
}
