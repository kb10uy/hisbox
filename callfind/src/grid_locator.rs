use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    iter::zip,
    str::{FromStr, from_utf8_unchecked},
};

use thiserror::Error as ThisError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GridLocator(GridLocatorInner);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GridLocatorInner {
    ToSquare([u8; 4]),
    ToSub1([u8; 6]),
    ToSub2([u8; 8]),
    ToSub3([u8; 10]),
}

#[derive(Debug, Clone, PartialEq, Eq, ThisError)]
pub enum GridLocatorError {
    #[error("invalid text")]
    InvalidText,

    #[error("invalid length")]
    InvalidLength,

    #[error("out of range")]
    OutOfRange,
}

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

    pub fn indices(&self) -> &[u8] {
        match &self.0 {
            GridLocatorInner::ToSquare(i) => i,
            GridLocatorInner::ToSub1(i) => i,
            GridLocatorInner::ToSub2(i) => i,
            GridLocatorInner::ToSub3(i) => i,
        }
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

impl Display for GridLocator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let indices = self.indices();
        let mut chars = [b'A', b'A', b'0', b'0', b'a', b'a', b'0', b'0', b'a', b'a'];
        for (&i, c) in zip(indices, &mut chars) {
            *c += i;
        }

        // Safety: chars and indices are in ASCII range
        let show_width = f.width().unwrap_or(10).min(indices.len());
        let gl_str = unsafe { from_utf8_unchecked(&chars[..show_width]) };
        f.pad(gl_str)
    }
}

impl FromStr for GridLocator {
    type Err = GridLocatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(GridLocatorError::InvalidText);
        }
        if !matches!(s.len(), 4 | 6 | 8 | 10) {
            return Err(GridLocatorError::InvalidLength);
        }
        let s = s.as_bytes();

        let field = (s[0].to_ascii_uppercase(), s[1].to_ascii_uppercase());
        let square = (s[2], s[3]);
        let sub1 = (s.len() >= 6).then(|| (s[4].to_ascii_lowercase(), s[5].to_ascii_lowercase()));
        let sub2 = (s.len() >= 8).then(|| (s[6].to_ascii_lowercase(), s[7].to_ascii_lowercase()));
        let sub3 = (s.len() >= 10).then(|| (s[8].to_ascii_lowercase(), s[9].to_ascii_lowercase()));
        let (fi_lng, fi_lat) = match field {
            (lng @ b'A'..=b'Z', lat @ b'A'..=b'Z') => (lng - b'A', lat - b'A'),
            _ => return Err(GridLocatorError::OutOfRange),
        };
        let (si_lng, si_lat) = match square {
            (lng @ b'0'..=b'9', lat @ b'0'..=b'9') => (lng - b'0', lat - b'0'),
            _ => return Err(GridLocatorError::OutOfRange),
        };
        let (s1i_lng, s1i_lat) = match sub1 {
            None => {
                return Ok(GridLocator(GridLocatorInner::ToSquare([
                    fi_lng, fi_lat, si_lng, si_lat,
                ])));
            }
            Some((lng @ b'a'..=b'z', lat @ b'a'..=b'z')) => (lng - b'a', lat - b'z'),
            _ => return Err(GridLocatorError::OutOfRange),
        };
        let (s2i_lng, s2i_lat) = match sub2 {
            None => {
                return Ok(GridLocator(GridLocatorInner::ToSub1([
                    fi_lng, fi_lat, si_lng, si_lat, s1i_lng, s1i_lat,
                ])));
            }
            Some((lng @ b'0'..=b'9', lat @ b'0'..=b'9')) => (lng - b'0', lat - b'0'),
            _ => return Err(GridLocatorError::OutOfRange),
        };
        let (s3i_lng, s3i_lat) = match sub3 {
            None => {
                return Ok(GridLocator(GridLocatorInner::ToSub2([
                    fi_lng, fi_lat, si_lng, si_lat, s1i_lng, s1i_lat, s2i_lng, s2i_lat,
                ])));
            }
            Some((lng @ b'a'..=b'z', lat @ b'a'..=b'z')) => (lng - b'a', lat - b'z'),
            _ => return Err(GridLocatorError::OutOfRange),
        };

        Ok(GridLocator(GridLocatorInner::ToSub3([
            fi_lng, fi_lat, si_lng, si_lat, s1i_lng, s1i_lat, s2i_lng, s2i_lat, s3i_lng, s3i_lat,
        ])))
    }
}
