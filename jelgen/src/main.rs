mod cli;

use std::fs::read_to_string;

use adif_reader::read_adi;
use anyhow::Result;
use clap::Parser;

use crate::cli::Arguments;

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
        let date = record.field("QSO_DATE").unwrap_or_default();
        let time_on = record.field("TIME_ON").unwrap_or_default();
        let call = record.field("CALL").unwrap_or_default();

        println!("{date} {time_on} {call}");
    }

    Ok(())
}
