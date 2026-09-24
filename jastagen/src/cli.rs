use std::{path::PathBuf, str::FromStr};

use adif_reader::LengthMode;
use clap::{Parser, ValueEnum};
use time::{
    UtcOffset, error::Parse as TimeParseError, format_description::BorrowedFormatItem,
    macros::format_description,
};

const CUSTOM_OFFSET: &[BorrowedFormatItem<'_>] =
    format_description!("[offset_hour]:[offset_minute]");

/// JASTA SSTV Activity Contest sheet generator
#[derive(Debug, Clone, Parser)]
#[command(version, author, about, long_about)]
pub struct Arguments {
    /// Input ADIF file exported from the logger.
    pub adif_file: PathBuf,

    /// Entrant settings file.
    #[clap(short, long, default_value = "jasta.toml")]
    pub settings: PathBuf,

    /// AD1C cty.dat country file used to resolve DXCC entities.
    #[clap(short, long, default_value = "cty.dat")]
    pub cty: PathBuf,

    /// Contest year. Defaults to the current year, or the previous one before August.
    #[clap(short, long)]
    pub year: Option<i32>,

    /// Keep only records carrying this ADIF CONTEST_ID, such as `JASTA-SSTV`.
    /// Without it, membership is inferred from mode, band, date, and serial.
    #[clap(short = 'C', long)]
    pub contest_id: Option<String>,

    /// Directory the sheets are written to.
    #[clap(short = 'O', long, default_value = ".")]
    pub out_dir: PathBuf,

    /// Encoding of the written sheets.
    #[clap(short, long, default_value = "utf8")]
    pub encoding: SheetEncoding,

    /// Enable lenient length count for ADI file.
    /// Pedantic ADI file must not contain non-ASCII characters.
    #[clap(short, long = "lenient")]
    pub lenient_length: Option<LenientMode>,

    /// Specify datetime offset of imported records.
    /// Wavelog exports UTC, which is the default.
    #[clap(short = 'o', long)]
    pub import_offset: Option<ImportOffset>,
}

/// The character encoding the sheets are written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum SheetEncoding {
    /// UTF-8 without a byte order mark.
    #[default]
    #[clap(name = "utf8")]
    Utf8,

    /// Shift_JIS, which is what MMJASTA wrote.
    #[clap(name = "sjis")]
    ShiftJis,
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
