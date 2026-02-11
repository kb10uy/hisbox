use common_qso::contest::QsoExchanges;
use compact_str::CompactString;
use mlua::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Exchange {
    pub tx_report: CompactString,
    pub tx_number: CompactString,
    pub rx_report: CompactString,
    pub rx_number: CompactString,
}

impl LuaUserData for Exchange {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("tx_report", |_, this| Ok(this.tx_report.to_string()));
        fields.add_field_method_get("tx_number", |_, this| Ok(this.tx_number.to_string()));
        fields.add_field_method_get("rx_report", |_, this| Ok(this.rx_report.to_string()));
        fields.add_field_method_get("rx_number", |_, this| Ok(this.rx_number.to_string()));
    }
}

impl From<QsoExchanges> for Exchange {
    fn from(value: QsoExchanges) -> Self {
        Exchange {
            tx_report: value.sent.report.unwrap_or_default(),
            tx_number: value.sent.number.unwrap_or_default(),
            rx_report: value.received.report.unwrap_or_default(),
            rx_number: value.received.number.unwrap_or_default(),
        }
    }
}
