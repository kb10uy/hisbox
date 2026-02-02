use crate::callsign::prefix;

include!(concat!(env!("OUT_DIR"), "/prefixes.rs"));

pub const UNASSIGNED_PREFIXES: &[prefix::Prefix] = &[
    prefix::Prefix::TwoRange(b'E', (b'8', b'9')),
    prefix::Prefix::TwoRange(b'Z', (b'4', b'7')),
    prefix::Prefix::TwoSpecified(b'4', b'N'),
    prefix::Prefix::TwoSpecified(b'H', b'5'),
    prefix::Prefix::TwoSpecified(b'J', b'9'),
    prefix::Prefix::TwoSpecified(b'S', b'4'),
    prefix::Prefix::TwoSpecified(b'T', b'9'),
    prefix::Prefix::TwoSpecified(b'V', b'9'),
    prefix::Prefix::TwoSpecified(b'Y', b'Z'),
    prefix::Prefix::TwoSpecified(b'Z', b'9'),
    prefix::Prefix::TwoRange(b'U', (b'2', b'9')),
    prefix::Prefix::TwoRange(b'X', (b'2', b'9')),
];
