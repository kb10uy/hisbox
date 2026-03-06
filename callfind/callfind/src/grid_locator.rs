mod error;
mod inner;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

use crate::grid_locator::{error::GridLocatorError, inner::GridLocatorInner};

#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct GridLocator(#[serde_as(as = "DisplayFromStr")] GridLocatorInner);

impl GridLocator {
    /// Calculates grid locator from longitude and latitude.
    pub fn from_lnglat(longitude: f64, latitude: f64) -> Result<GridLocator, GridLocatorError> {
        let offset_lng = longitude + 180.0;
        let offset_lat = latitude + 90.0;
        if !(0.0..360.0).contains(&offset_lng) || !(0.0..180.0).contains(&offset_lat) {
            return Err(GridLocatorError::OutOfRange);
        }

        // sub-degree square has 5760 blocks
        let scaled_lng = (offset_lng / 2.0 * 5760.0) as usize;
        let scaled_lat = (offset_lat * 5760.0) as usize;
        let lng_letters = Self::calculate_indices(scaled_lng);
        let lat_letters = Self::calculate_indices(scaled_lat);

        Ok(GridLocator(GridLocatorInner::ToSub3([
            lng_letters[0],
            lat_letters[0],
            lng_letters[1],
            lat_letters[1],
            lng_letters[2],
            lat_letters[2],
            lng_letters[3],
            lat_letters[3],
            lng_letters[4],
            lat_letters[4],
        ])))
    }

    fn calculate_indices(scaled_value: usize) -> [u8; 5] {
        let field = (scaled_value / 57600) as u8;
        let square = (scaled_value / 5760 % 10) as u8;
        let subsq1 = (scaled_value / 240 % 24) as u8;
        let subsq2 = (scaled_value / 24 % 10) as u8;
        let subsq3 = (scaled_value % 24) as u8;
        [field, square, subsq1, subsq2, subsq3]
    }
}

impl FromStr for GridLocator {
    type Err = GridLocatorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

impl Display for GridLocator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}
