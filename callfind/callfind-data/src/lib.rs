use callfind_core::callsign::Prefix;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

// sorted for binary search
pub const UNASSIGNED_PREFIXES: &[Prefix] = &[
    Prefix::TwoSpecified(b'4', b'N'),
    Prefix::TwoRange(b'E', (b'8', b'9')),
    Prefix::TwoSpecified(b'H', b'5'),
    Prefix::TwoSpecified(b'J', b'9'),
    Prefix::TwoSpecified(b'S', b'4'),
    Prefix::TwoSpecified(b'T', b'9'),
    Prefix::TwoRange(b'U', (b'2', b'9')),
    Prefix::TwoSpecified(b'V', b'9'),
    Prefix::TwoRange(b'X', (b'2', b'9')),
    Prefix::TwoSpecified(b'Y', b'Z'),
    Prefix::TwoRange(b'Z', (b'4', b'7')),
    Prefix::TwoSpecified(b'Z', b'9'),
];
