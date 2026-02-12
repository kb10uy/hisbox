use std::{convert::Infallible, path::PathBuf, str::FromStr};

use adif_reader::LengthMode;
use clap::{Parser, ValueEnum};
use compact_str::{CompactString, ToCompactString};

/// Generates JSON data for QSL cards.
#[derive(Debug, Clone, Parser)]
#[command(version, author, about, long_about)]
pub struct Arguments {
    /// Processor script file.
    pub script_path: PathBuf,

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

    /// Specify arguments passed to script.
    #[clap(short = 'A', long = "args")]
    pub script_args: Vec<ScriptArg>,

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScriptArg(pub CompactString, pub Option<CompactString>);

impl FromStr for ScriptArg {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('=') {
            Some((k, v)) => Ok(ScriptArg(
                k.to_compact_string(),
                Some(v.to_compact_string()),
            )),
            None => Ok(ScriptArg(s.to_compact_string(), None)),
        }
    }
}
