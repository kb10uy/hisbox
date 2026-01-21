use std::path::PathBuf;

use adif_reader::LengthMode;
use clap::{Parser, ValueEnum};

/// JARL eLog Generator
#[derive(Debug, Clone, Parser)]
#[command(version, author, about, long_about)]
pub struct Arguments {
    /// Input ADIF file.
    pub adif_file: PathBuf,

    /// Enable lenient length count for ADI file.
    #[clap(short, long = "lenient")]
    pub lenient_length: Option<LenientMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LenientMode {
    Bytes,
    Codepoints,
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
