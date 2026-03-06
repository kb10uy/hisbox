mod data;
mod prefix;

use crate::callsign::data::{AREA_NAMES, PREFIXES, UNASSIGNED_PREFIXES};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallsignAssign {
    Assigned(&'static str),
    Unassigned,
    Unavailable,
}

pub fn lookup_prefix_area(ascii_callsign: &[u8]) -> CallsignAssign {
    if ascii_callsign.len() < 3 {
        return CallsignAssign::Unavailable;
    }
    match (ascii_callsign[0], ascii_callsign[1], ascii_callsign[2]) {
        (b'0'..=b'9', b'0'..=b'9', _) => CallsignAssign::Unavailable,
        (b'Q' | b'0' | b'1', _, _) => CallsignAssign::Unavailable,
        (c1, c2, c3) => {
            if let Ok(i) = UNASSIGNED_PREFIXES.binary_search_by(|probe| probe.range_cmp(c1, c2, c3))
                && UNASSIGNED_PREFIXES[i].is_match(c1, c2, c3)
            {
                return CallsignAssign::Unassigned;
            }
            match PREFIXES.binary_search_by(|probe| probe.0.range_cmp(c1, c2, c3)) {
                Ok(i) if PREFIXES[i].0.is_match(c1, c2, c3) => {
                    CallsignAssign::Assigned(AREA_NAMES[PREFIXES[i].1])
                }
                Ok(_) => CallsignAssign::Unavailable,
                Err(_) if matches!(c2, b'0' | b'1') => CallsignAssign::Unavailable,
                _ => CallsignAssign::Unassigned,
            }
        }
    }
}
