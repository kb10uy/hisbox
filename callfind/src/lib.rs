pub mod prefix;

include!(concat!(env!("OUT_DIR"), "/prefixes.rs"));

const UNASSIGNED_PREFIXES: &[prefix::Prefix] = &[
    prefix::Prefix::ByTwoRange(b'E', (b'8', b'9')),
    prefix::Prefix::ByTwoRange(b'Z', (b'4', b'7')),
    prefix::Prefix::ByTwoSpecified(b'4', b'N'),
    prefix::Prefix::ByTwoSpecified(b'H', b'5'),
    prefix::Prefix::ByTwoSpecified(b'J', b'9'),
    prefix::Prefix::ByTwoSpecified(b'S', b'4'),
    prefix::Prefix::ByTwoSpecified(b'T', b'9'),
    prefix::Prefix::ByTwoSpecified(b'V', b'9'),
    prefix::Prefix::ByTwoSpecified(b'Y', b'Z'),
    prefix::Prefix::ByTwoSpecified(b'Z', b'9'),
    prefix::Prefix::ByTwoRange(b'U', (b'2', b'9')),
    prefix::Prefix::ByTwoRange(b'X', (b'2', b'9')),
];

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
