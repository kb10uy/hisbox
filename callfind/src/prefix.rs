use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Prefix {
    ByOneAll(u8),
    ByTwoRange(u8, (u8, u8)),
    ByTwoSpecified(u8, u8),
    ByThreeRange(u8, u8, (u8, u8)),
}

impl Prefix {
    pub fn is_match(&self, c1: u8, c2: u8, c3: u8) -> bool {
        match self {
            Prefix::ByOneAll(p1) => c1 == *p1,
            Prefix::ByTwoRange(p1, (p2s, p2e)) => c1 == *p1 && (*p2s..=*p2e).contains(&c2),
            Prefix::ByTwoSpecified(p1, p2) => c1 == *p1 && c2 == *p2,
            Prefix::ByThreeRange(p1, p2, (p3s, p3e)) => {
                c1 == *p1 && c2 == *p2 && (*p3s..=*p3e).contains(&c3)
            }
        }
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fn ascii_char(c: u8) -> char {
            char::from_u32(c as u32).expect("must be ASCII char")
        }

        match self {
            Prefix::ByOneAll(p1) => write!(f, "{}", ascii_char(*p1)),
            Prefix::ByTwoRange(p1, (p2s, p2e)) => write!(
                f,
                "{0}{1}-{0}{2}",
                ascii_char(*p1),
                ascii_char(*p2s),
                ascii_char(*p2e)
            ),
            Prefix::ByTwoSpecified(p1, p2) => write!(f, "{}{}", ascii_char(*p1), ascii_char(*p2)),
            Prefix::ByThreeRange(p1, p2, (p3s, p3e)) => {
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
        assert!(Prefix::ByOneAll(b'W').is_match(b'W', b'1', b'J'));
        assert!(Prefix::ByTwoRange(b'J', (b'A', b'S')).is_match(b'J', b'L', b'1'));
        assert!(Prefix::ByTwoSpecified(b'3', b'A').is_match(b'3', b'A', b'0'));
        assert!(Prefix::ByThreeRange(b'3', b'D', (b'A', b'M')).is_match(b'3', b'D', b'C'));
    }
}
