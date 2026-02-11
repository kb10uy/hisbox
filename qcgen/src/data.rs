use std::{collections::HashMap, fs::read_to_string, path::Path, process::exit};

use callfind::grid_locator::GridLocator;
use serde::{Deserialize, de::DeserializeOwned};
use tracing::error;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Instrument {
    pub rig: String,
    pub antenna: String,
    pub default_power: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Operation {
    pub operator: String,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Location {
    pub address: String,
    pub grid: Option<GridLocator>,
    pub lnglat: Option<(f64, f64)>,
}

pub fn read_items_from_tomls<T: DeserializeOwned>(
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
