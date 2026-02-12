mod cli;
mod data;

use std::{collections::HashMap, fs::read_to_string, io::stdout, process::exit, sync::LazyLock};

use adif_reader::read_adi;
use anyhow::Result;
use callfind::grid_locator::GridLocator;
use clap::Parser;
use common_qso::{
    exchange::QsoExchanges,
    qsl::{QslReceiveStatus, QslSendStatus, QslStatus},
    record::QsoRecord,
};
use compact_str::ToCompactString;
use mlua::prelude::*;
use regex::Regex;
use schope::{
    data::qsl_card::{QslCard, QslCardEntry, QslInfo, QslInstrument, QslOperation},
    engine::{initialize_lua, lua_to_json},
};
use time::UtcOffset;
use tracing::{Level, error, span, warn};
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
    let script_path = args.script_path.canonicalize()?;
    let script_args: HashMap<_, _> = args
        .script_args
        .into_iter()
        .map(|a| (a.0.to_string(), a.1.unwrap_or_default().to_string()))
        .collect();

    let adi_text = read_to_string(args.adif_file)?;
    let adif = read_adi(&adi_text, args.lenient_length.unwrap_or_default().into())?;
    let instruments = read_items_from_tomls::<Instrument>(args.instruments_files);
    let operations = read_items_from_tomls::<Operation>(args.operations_files);

    let mut entries = Vec::with_capacity(adif.records().len());
    for (i, record) in adif.records().iter().enumerate() {
        let span = span!(Level::ERROR, "record_process", index = i);
        let _enter = span.enter();

        let qso_record = QsoRecord::new(record, UtcOffset::UTC)?;
        let qso_exchanges = QsoExchanges::new(record);
        let qsl_status = QslStatus::new(record)?;
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

        entries.push(QslCardEntry {
            qso: qso_record.into(),
            exchange: qso_exchanges.into(),
            info: QslInfo {
                instrument: QslInstrument {
                    antenna: instrument.map(|i| i.antenna.to_compact_string()),
                    rig: instrument.map(|i| i.rig.to_compact_string()),
                    power,
                },
                operation: QslOperation {
                    operator: operation.map(|o| o.operator.to_compact_string()),
                    address: operation.map(|o| o.location.address.to_compact_string()),
                    grid,
                },
                card: QslCard {
                    should_send: matches!(
                        qsl_status.send,
                        Some(QslSendStatus::Queued | QslSendStatus::Requested),
                    ),
                    received: matches!(
                        qsl_status.receive,
                        Some(QslReceiveStatus::Confirmed | QslReceiveStatus::Verified)
                    ),
                    manager,
                },
            },
        });
    }

    let script_text = read_to_string(&script_path)?;
    let Some(script_base) = script_path.parent() else {
        error!("script path is invalid");
        exit(1);
    };

    let lua = initialize_lua(script_base)?;
    let script_table: LuaTable = lua.load(script_text).eval()?;
    let generate: LuaFunction = script_table.get("generate")?;
    let processed_value: LuaValue = generate.call((script_args, entries))?;

    let processed_json = lua_to_json(processed_value)?;
    let stdout = stdout().lock();
    serde_json::to_writer(stdout, &processed_json)?;

    Ok(())
}
