use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    iter::zip,
    str::{FromStr, from_utf8_unchecked},
};

use crate::grid_locator::error::GridLocatorError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridLocatorInner {
    ToSquare([u8; 4]),
    ToSub1([u8; 6]),
    ToSub2([u8; 8]),
    ToSub3([u8; 10]),
}

impl GridLocatorInner {
    pub fn indices(&self) -> &[u8] {
        match &self {
            GridLocatorInner::ToSquare(i) => i,
            GridLocatorInner::ToSub1(i) => i,
            GridLocatorInner::ToSub2(i) => i,
            GridLocatorInner::ToSub3(i) => i,
        }
    }
}

impl FromStr for GridLocatorInner {
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
                return Ok(GridLocatorInner::ToSquare([fi_lng, fi_lat, si_lng, si_lat]));
            }
            Some((lng @ b'a'..=b'z', lat @ b'a'..=b'z')) => (lng - b'a', lat - b'a'),
            _ => return Err(GridLocatorError::OutOfRange),
        };
        let (s2i_lng, s2i_lat) = match sub2 {
            None => {
                return Ok(GridLocatorInner::ToSub1([
                    fi_lng, fi_lat, si_lng, si_lat, s1i_lng, s1i_lat,
                ]));
            }
            Some((lng @ b'0'..=b'9', lat @ b'0'..=b'9')) => (lng - b'0', lat - b'0'),
            _ => return Err(GridLocatorError::OutOfRange),
        };
        let (s3i_lng, s3i_lat) = match sub3 {
            None => {
                return Ok(GridLocatorInner::ToSub2([
                    fi_lng, fi_lat, si_lng, si_lat, s1i_lng, s1i_lat, s2i_lng, s2i_lat,
                ]));
            }
            Some((lng @ b'a'..=b'z', lat @ b'a'..=b'z')) => (lng - b'a', lat - b'a'),
            _ => return Err(GridLocatorError::OutOfRange),
        };

        Ok(GridLocatorInner::ToSub3([
            fi_lng, fi_lat, si_lng, si_lat, s1i_lng, s1i_lat, s2i_lng, s2i_lat, s3i_lng, s3i_lat,
        ]))
    }
}

impl Display for GridLocatorInner {
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
