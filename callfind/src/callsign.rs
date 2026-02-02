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
        (c1, c2, c3) if UNASSIGNED_PREFIXES.iter().any(|p| p.is_match(c1, c2, c3)) => {
            CallsignAssign::Unassigned
        }
        (c1, c2, c3) => {
            if let Some((_, i)) = PREFIXES.iter().find(|p| p.0.is_match(c1, c2, c3)) {
                CallsignAssign::Assigned(AREA_NAMES[*i])
            } else if matches!(c2, b'0' | b'1') {
                CallsignAssign::Unavailable
            } else {
                CallsignAssign::Unassigned
            }
        }
    }
}
