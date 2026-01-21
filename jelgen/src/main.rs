mod cli;

use std::fs::read_to_string;

use adif_reader::{LengthMode, read_adi};
use anyhow::Result;
use clap::Parser;

use crate::cli::Arguments;

fn main() -> Result<()> {
    let args = Arguments::parse();

    let adi_text = read_to_string(args.adif_file)?;
    let adif = read_adi(&adi_text, LengthMode::Bytes)?;

    println!("{} records imported", adif.records().len());

    Ok(())
}
