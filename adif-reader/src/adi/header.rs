use std::collections::HashMap;

use crate::adi::error::AdiError;

#[derive(Debug, Clone)]
pub struct Header<'a> {
    preamble: &'a str,
    fields: HashMap<&'a str, &'a str>,
}

pub fn parse_header<'a>(text: &'a str) -> Result<Option<Header>, AdiError> {
    // > If the first character in an ADI file is <, it contains no Header.
    // https://adif.org.uk/316/ADIF_316.htm#ADI_File_Format
    let header_start = match text.find("<") {
        Some(0) => return Ok(None),
        Some(n) => n,
        None => return Err(AdiError::NoData),
    };
    let preamble = &text[..header_start];

    todo!();
}
