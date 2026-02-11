use std::path::PathBuf;

use adif_reader::LengthMode;
use clap::{Parser, ValueEnum};

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

    /// Specify instruments definition file.
    #[clap(short, long = "instruments")]
    pub instruments_files: Vec<PathBuf>,

    /// Specify operations definition file.
    #[clap(short, long = "operations")]
    pub operations_files: Vec<PathBuf>,

    /// Specify default instrument.
    #[clap(short = 'I', long)]
    pub instrument: Option<String>,

    /// Specify default operation.
    #[clap(short = 'O', long)]
    pub operation: Option<String>,

    /// Specify default power.
    /// It overrides default instrument's value.
    #[clap(short = 'P', long)]
    pub power: Option<f64>,
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
