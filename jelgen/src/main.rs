mod cli;
mod processor;
mod qso;

use std::fs::read_to_string;

use adif_reader::read_adi;
use anyhow::Result;
use clap::Parser;

use crate::{
    cli::Arguments,
    processor::{Processor, data::Record},
    qso::record::QsoRecord,
};

fn main() -> Result<()> {
    let args = Arguments::parse();

    let adi_text = read_to_string(args.adif_file)?;
    let adif = read_adi(&adi_text, args.lenient_length.unwrap_or_default().into())?;
    println!("{} records imported", adif.records().len());

    let processor = Processor::initialize(args.processor_file, Default::default())?;
    let import_offset = args.import_offset.unwrap_or_default().into();
    let process_offset = processor.process_offset();
    for record in adif.records() {
        let qso_record = QsoRecord::new(record, import_offset)?;
        let Some(process_record) = Record::new(qso_record, process_offset) else {
            continue;
        };

        processor.process(process_record)?;
    }

    Ok(())
}
