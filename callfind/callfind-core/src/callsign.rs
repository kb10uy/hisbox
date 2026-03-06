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

    pub fn range_order(&self, other: &Prefix) -> Ordering {
        match (self, other) {
            (Prefix::OneAll(l), Prefix::OneAll(r)) => l.cmp(r),
            (Prefix::OneAll(l), Prefix::TwoRange(r, _)) => l.cmp(r),
            (Prefix::OneAll(l), Prefix::TwoSpecified(r, _)) => l.cmp(r),
            (Prefix::OneAll(l), Prefix::ThreeRange(r, _, _)) => l.cmp(r),
            (Prefix::TwoRange(l1, l2), Prefix::TwoRange(r1, r2)) => {
                l1.cmp(r1).then(Self::range_order_overwrap_cmp(*l2, *r2))
            }
            (Prefix::TwoRange(l1, l2), Prefix::TwoSpecified(r1, r2)) => {
                l1.cmp(r1).then(Self::range_order_cmp(*r2, *l2).reverse())
            }
            (Prefix::TwoRange(l1, l2), Prefix::ThreeRange(r1, r2, _)) => {
                l1.cmp(r1).then(Self::range_order_cmp(*r2, *l2).reverse())
            }
            (Prefix::TwoSpecified(l1, l2), Prefix::TwoSpecified(r1, r2)) => {
                l1.cmp(r1).then(l2.cmp(r2))
            }
            (Prefix::TwoSpecified(l1, l2), Prefix::ThreeRange(r1, r2, _)) => {
                l1.cmp(r1).then(l2.cmp(r2))
            }
            (Prefix::ThreeRange(l1, l2, l3), Prefix::ThreeRange(r1, r2, r3)) => l1
                .cmp(r1)
                .then(l2.cmp(r2))
                .then(Self::range_order_overwrap_cmp(*l3, *r3)),

            // (Prefix::TwoRange(l, _), Prefix::OneAll(r)) => todo!(),
            // (Prefix::TwoSpecified(_, _), Prefix::OneAll(_)) => todo!(),
            // (Prefix::TwoSpecified(_, _), Prefix::TwoRange(_, _)) => todo!(),
            // (Prefix::ThreeRange(_, _, _), Prefix::OneAll(_)) => todo!(),
            // (Prefix::ThreeRange(_, _, _), Prefix::TwoRange(_, _)) => todo!(),
            // (Prefix::ThreeRange(_, _, _), Prefix::TwoSpecified(_, _)) => todo!(),
            _ => other.range_order(self).reverse(),
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

    fn range_order_cmp(x: u8, (l, r): (u8, u8)) -> Ordering {
        assert!(l <= r);
        if (l..=r).contains(&x) {
            Ordering::Equal
        } else if x < l {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn range_order_overwrap_cmp(lhs: (u8, u8), rhs: (u8, u8)) -> Ordering {
        assert!(lhs.0 <= lhs.1);
        assert!(rhs.0 <= rhs.1);
        if lhs.0 <= rhs.1 && rhs.0 <= lhs.1 {
            Ordering::Equal
        } else if lhs.1 < rhs.0 {
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
