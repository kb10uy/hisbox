use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(version, author, about, long_about)]
pub struct Arguments {
    /// Processor script file.
    pub processor_file: PathBuf,

    /// Input ADIF file.
    pub adif_file: PathBuf,

    /// Specify instruments definition file.
    #[clap(short, long = "instruments")]
    pub instruments_files: Vec<PathBuf>,

    /// Specify operations definition file.
    #[clap(short, long = "operations")]
    pub operations_files: Vec<PathBuf>,
}
