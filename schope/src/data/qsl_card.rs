use crate::data::{exchange::Exchange, misc::Misc, record::Record};

#[derive(Debug, Clone, PartialEq)]
pub struct QslCardEntry {
    pub qso: Record,
    pub exchange: Exchange,
    pub misc: Misc,
}
