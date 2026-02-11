use compact_str::CompactString;
use mlua::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ContestNumber {
    pub tx_report: CompactString,
    pub tx_number: CompactString,
    pub rx_report: CompactString,
    pub rx_number: CompactString,
}

impl LuaUserData for ContestNumber {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("tx_report", |_, this| Ok(this.tx_report.to_string()));
        fields.add_field_method_get("tx_number", |_, this| Ok(this.tx_number.to_string()));
        fields.add_field_method_get("rx_report", |_, this| Ok(this.rx_report.to_string()));
        fields.add_field_method_get("rx_number", |_, this| Ok(this.rx_number.to_string()));
    }
}
