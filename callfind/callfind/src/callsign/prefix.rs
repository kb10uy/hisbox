use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Prefix {
    OneAll(u8),
    TwoRange(u8, (u8, u8)),
    TwoSpecified(u8, u8),
    ThreeRange(u8, u8, (u8, u8)),
}

impl Prefix {
    pub fn is_match(&self, c1: u8, c2: u8, c3: u8) -> bool {
        match self {
            Prefix::OneAll(p1) => c1 == *p1,
            Prefix::TwoRange(p1, (p2s, p2e)) => c1 == *p1 && (*p2s..=*p2e).contains(&c2),
            Prefix::TwoSpecified(p1, p2) => c1 == *p1 && c2 == *p2,
            Prefix::ThreeRange(p1, p2, (p3s, p3e)) => {
                c1 == *p1 && c2 == *p2 && (*p3s..=*p3e).contains(&c3)
            }
        }
    }

    pub fn range_cmp(&self, c1: u8, c2: u8, c3: u8) -> Ordering {
        match self {
            Prefix::OneAll(p1) => p1.cmp(&c1),
            Prefix::TwoRange(p1, p2) => p1.cmp(&c1).then(Self::point_cmp(c2, *p2).reverse()),
            Prefix::TwoSpecified(p1, p2) => p1.cmp(&c1).then(p2.cmp(&c2)),
            Prefix::ThreeRange(p1, p2, p3) => p1
                .cmp(&c1)
                .then(p2.cmp(&c2))
                .then(Self::point_cmp(c3, *p3).reverse()),
        }
    }

    fn point_cmp(x: u8, (l, r): (u8, u8)) -> Ordering {
        assert!(l <= r);
        if (l..=r).contains(&x) {
            Ordering::Equal
        } else if x < l {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fn ascii_char(c: u8) -> char {
            char::from_u32(c as u32).expect("must be ASCII char")
        }

        match self {
            Prefix::OneAll(p1) => write!(f, "{}", ascii_char(*p1)),
            Prefix::TwoRange(p1, (p2s, p2e)) => write!(
                f,
                "{0}{1}-{0}{2}",
                ascii_char(*p1),
                ascii_char(*p2s),
                ascii_char(*p2e)
            ),
            Prefix::TwoSpecified(p1, p2) => write!(f, "{}{}", ascii_char(*p1), ascii_char(*p2)),
            Prefix::ThreeRange(p1, p2, (p3s, p3e)) => {
                write!(
                    f,
                    "{0}{1}{2}-{0}{1}{3}",
                    ascii_char(*p1),
                    ascii_char(*p2),
                    ascii_char(*p3s),
                    ascii_char(*p3e)
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Prefix;

    #[test]
    fn matches_prefix() {
        assert!(Prefix::OneAll(b'W').is_match(b'W', b'1', b'J'));
        assert!(Prefix::TwoRange(b'J', (b'A', b'S')).is_match(b'J', b'L', b'1'));
        assert!(Prefix::TwoSpecified(b'3', b'A').is_match(b'3', b'A', b'0'));
        assert!(Prefix::ThreeRange(b'3', b'D', (b'A', b'M')).is_match(b'3', b'D', b'C'));
    }
}
