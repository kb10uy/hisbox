use std::{path::PathBuf, str::FromStr};

use adif_reader::LengthMode;
use clap::{Parser, ValueEnum};
use time::{
    UtcOffset, error::Parse as TimeParseError, format_description::BorrowedFormatItem,
    macros::format_description,
};

const CUSTOM_OFFSET: &[BorrowedFormatItem<'_>] =
    format_description!("[offset_hour]:[offset_minute]");

/// JARL eLog Generator
#[derive(Debug, Clone, Parser)]
#[command(version, author, about, long_about)]
pub struct Arguments {
    /// Processor script file.
    pub processor_file: PathBuf,

    /// Input ADIF file.
    pub adif_file: PathBuf,

    /// Enable lenient length count for ADI file.
    /// Pedantic ADI file must not contain non-ASCII characters.
    #[clap(short, long = "lenient")]
    pub lenient_length: Option<LenientMode>,

    /// Specify datetime offset of imported records.
    /// Pedantic ADI file must have datetime with UTC.
    #[clap(short = 'o', long)]
    pub import_offset: Option<ImportOffset>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum LenientMode {
    /// Count by bytes.
    #[default]
    Bytes,

    /// Count by codepoints.
    Codepoints,

    /// Count by grapheme clusters.
    Graphemes,
}

impl From<LenientMode> for LengthMode {
    fn from(value: LenientMode) -> Self {
        match value {
            LenientMode::Bytes => LengthMode::Bytes,
            LenientMode::Codepoints => LengthMode::Codepoints,
            LenientMode::Graphemes => LengthMode::Graphemes,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImportOffset {
    #[default]
    Utc,
    Jst,
    Custom(UtcOffset),
}

impl FromStr for ImportOffset {
    type Err = TimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UTC" => Ok(ImportOffset::Utc),
            "JST" => Ok(ImportOffset::Jst),
            _ => Ok(ImportOffset::Custom(UtcOffset::parse(s, CUSTOM_OFFSET)?)),
        }
    }
}

impl From<ImportOffset> for UtcOffset {
    fn from(value: ImportOffset) -> Self {
        match value {
            ImportOffset::Utc => UtcOffset::UTC,
            ImportOffset::Jst => UtcOffset::from_hms(9, 0, 0).expect("valid offset"),
            ImportOffset::Custom(utc_offset) => utc_offset,
        }
    }
}
