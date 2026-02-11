mod cli;
mod data;

use std::{collections::HashMap, fs::read_to_string, path::Path, process::exit};

use adif_reader::read_adi;
use anyhow::Result;
use clap::Parser;
use serde::de::DeserializeOwned;
use tracing::error;
use tracing_subscriber::EnvFilter;

use crate::{
    cli::Arguments,
    data::{Instrument, Operation},
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
    let instruments = read_items_from_tomls::<Instrument>(args.instruments_files);
    let operations = read_items_from_tomls::<Operation>(args.operations_files);

    for record in adif.records() {}

    Ok(())
}

fn read_items_from_tomls<T: DeserializeOwned>(
    files: impl IntoIterator<Item = impl AsRef<Path>>,
) -> HashMap<String, T> {
    let mut items = HashMap::new();

    for file in files {
        let toml = match read_to_string(file.as_ref()) {
            Ok(t) => t,
            Err(e) => {
                error!(
                    "failed to read {}: {e}",
                    file.as_ref().to_str().unwrap_or_default()
                );
                exit(1);
            }
        };
        let file_items: HashMap<_, T> = match toml::from_str(&toml) {
            Ok(i) => i,
            Err(e) => {
                error!(
                    "failed to parse {}: {e}",
                    file.as_ref().to_str().unwrap_or_default()
                );
                exit(1);
            }
        };
        items.extend(file_items);
    }
    items
}
