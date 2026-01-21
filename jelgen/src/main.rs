mod cli;
mod qso;

use std::fs::read_to_string;

use adif_reader::read_adi;
use anyhow::Result;
use clap::Parser;
use time::UtcOffset;

use crate::{cli::Arguments, qso::QsoRecord};

fn main() -> Result<()> {
    let args = Arguments::parse();

    let adi_text = read_to_string(args.adif_file)?;
    let adif = read_adi(
        &adi_text,
        args.lenient_length
            .unwrap_or(cli::LenientMode::Bytes)
            .into(),
    )?;

    println!("{} records imported", adif.records().len());
    for record in adif.records() {
        let qso_record = QsoRecord::new(record, UtcOffset::UTC)?;
        println!("{qso_record:?}");
    }

    Ok(())
}
