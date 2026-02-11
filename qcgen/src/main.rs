mod cli;
mod data;

use std::{fs::read_to_string, sync::LazyLock};

use adif_reader::read_adi;
use anyhow::Result;
use callfind::grid_locator::GridLocator;
use clap::Parser;
use common_qso::{contest::QsoExchanges, record::QsoRecord};
use compact_str::ToCompactString;
use regex::Regex;
use schope::data::{exchange::Exchange, misc::Misc, record::Record};
use time::UtcOffset;
use tracing::{Level, span, warn};
use tracing_subscriber::EnvFilter;

use crate::{
    cli::Arguments,
    data::{Instrument, Operation, read_items_from_tomls},
};

static RE_EXTRA_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"!(\w+):([^\s]+)"#).expect("valid regex"));

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Arguments::parse();

    let adi_text = read_to_string(args.adif_file)?;
    let adif = read_adi(&adi_text, args.lenient_length.unwrap_or_default().into())?;
    let instruments = read_items_from_tomls::<Instrument>(args.instruments_files);
    let operations = read_items_from_tomls::<Operation>(args.operations_files);

    for (i, record) in adif.records().iter().enumerate() {
        let span = span!(Level::ERROR, "record_process", index = i);
        let _enter = span.enter();

        let qso_record = QsoRecord::new(record, UtcOffset::UTC)?;
        let qso_exchanges = QsoExchanges::new(record);
        let qso_power: Option<f64> = record.field("TX_PWR").and_then(|p| p.parse().ok());

        let mut instrument_key = args.instrument.as_deref();
        let mut operation_key = args.operation.as_deref();
        let mut manager = None;
        let comment = record.field("COMMENT").unwrap_or_default();
        for extra_tag in RE_EXTRA_TAG.captures_iter(comment) {
            let key = extra_tag.get(1).expect("group must exist");
            let value = extra_tag.get(2).expect("group must exist");
            match key.as_str() {
                "inst" => instrument_key = Some(value.as_str()),
                "op" => operation_key = Some(value.as_str()),
                "manager" => manager = Some(value.as_str()),
                otherwise => {
                    warn!("unknown extra tag: {otherwise}");
                    continue;
                }
            }
        }

        let instrument = instrument_key.and_then(|k| instruments.get(k));
        let operation = operation_key.and_then(|k| operations.get(k));
        let manager = manager.map(|s| s.to_compact_string());
        let power = qso_power
            .or(args.power)
            .or(instrument.and_then(|i| i.default_power));
        let grid = operation.and_then(|o| o.location.grid).or(operation
            .and_then(|o| o.location.lnglat)
            .and_then(|(lng, lat)| GridLocator::from_lnglat(lng, lat).ok()));

        let schope_record: Record = qso_record.into();
        let schope_exchange: Exchange = qso_exchanges.into();
        let schope_misc = Misc {
            antenna: instrument.map(|i| i.antenna.to_compact_string()),
            power,
            operator: operation.map(|o| o.operator.to_compact_string()),
            address: operation.map(|o| o.location.address.to_compact_string()),
            grid,
            manager,
        };
    }

    Ok(())
}
