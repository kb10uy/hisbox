use callfind::grid_locator::GridLocator;
use serde::Deserialize;

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
