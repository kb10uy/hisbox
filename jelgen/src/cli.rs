use std::path::PathBuf;

use clap::Parser;

/// JARL eLog Generator
#[derive(Debug, Clone, Parser)]
#[command(version, author, about, long_about)]
pub struct Arguments {
    /// Input ADIF file.
    pub adif_file: PathBuf,
}
