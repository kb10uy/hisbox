mod calculator;
mod cli;
mod qso;
mod scop;

use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    fs::read_to_string,
};

use adif_reader::{document::Record, read_adi};
use anyhow::Result;
use clap::Parser;
use mlua::prelude::*;
use time::UtcOffset;
use tracing::{Level, info, instrument, span, warn};
use tracing_subscriber::EnvFilter;

use crate::{
    calculator::{Calculator, data::RecordSummary, error::ProcessorError},
    cli::Arguments,
    qso::{error::QsoError, record::QsoRecord},
};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Arguments::parse();

    let adi_text = read_to_string(args.adif_file)?;
    let adif = read_adi(&adi_text, args.lenient_length.unwrap_or_default().into())?;
    info!("{} records imported", adif.records().len());

    let processor = Calculator::initialize(args.processor_file, Default::default())?;
    let process_offset = processor.process_offset();
    let import_offset = args.import_offset.unwrap_or_default().into();

    let mut processed_ids = HashSet::new();
    let mut groups: HashMap<String, Vec<(String, RecordSummary)>> = HashMap::new();

    for (i, record) in adif.records().iter().enumerate() {
        let span = span!(Level::ERROR, "record_process", index = i);
        let _enter = span.enter();

        let qso_record = match QsoRecord::new(record, import_offset) {
            Ok(r) => r,
            Err(e) => {
                warn!("cannot construct ({e})");
                continue;
            }
        };

        let Some(processor_record) = processor.convert_record(&qso_record, process_offset) else {
            warn!("report/number not found");
            continue;
        };

        let metadata = match processor.key(&processor_record) {
            Ok(s) => s,
            Err(e) => {
                warn!("metadata failed ({e})");
                continue;
            }
        };
        if processed_ids.contains(&metadata.id) {
            warn!("id {} duplicate", metadata.id);
            continue;
        }

        let summary = match processor.process(&processor_record) {
            Ok(s) => s,
            Err(e) => {
                warn!("process failed ({e})");
                continue;
            }
        };

        let group_data = (metadata.id.clone(), summary);
        match groups.entry(metadata.group) {
            Entry::Occupied(mut o) => o.get_mut().push(group_data),
            Entry::Vacant(v) => {
                v.insert(vec![group_data]);
            }
        }
        processed_ids.insert(metadata.id);
    }

    println!("Groups:");
    for (group, summaries) in &groups {
        println!("{group}: {}", summaries.len());
    }

    let summary = processor.summarize(groups)?;
    println!("Summary: {summary:?}");

    Ok(())
}

#[derive(Debug)]
struct MainProcess {
    processor: Calculator,
    import_offset: UtcOffset,
    process_offset: UtcOffset,

    processed_ids: HashSet<String>,
    groups: HashMap<String, Vec<(String, RecordSummary)>>,
}

enum ProcessStatus {
    Processed(RecordSummary),
    Duplicate,
    NoNumbers,
    QsoError(QsoError),
    ProcessorError(ProcessorError),
}

/*
impl MainProcess {
    #[instrument(skip(self, record))]
    fn process_record(&mut self, (i, record): (usize, &Record)) -> ProcessStatus {
        let qso_record = match QsoRecord::new(record, self.import_offset) {
            Ok(r) => r,
            Err(e) => return ProcessStatus::QsoError(e),
        };

        let Some(processor_record) = self
            .processor
            .convert_record(&qso_record, self.process_offset)
        else {
            return ProcessStatus::NoNumbers;
        };

        let metadata = match self.processor.metadata(&processor_record) {
            Ok(s) => s,
            Err(e) => return ProcessStatus::LuaError(e),
        };
        if self.processed_ids.contains(&metadata.id) {
            warn!("id {} duplicate", metadata.id);
            continue;
        }

        let summary = match processor.process(&processor_record) {
            Ok(s) => s,
            Err(e) => {
                warn!("process failed ({e})");
                continue;
            }
        };
        Ok(())
    }
}
*/
