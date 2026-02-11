use callfind::grid_locator::GridLocator;
use compact_str::CompactString;
use mlua::prelude::*;

use crate::library::datetime::SchopeDateTime;

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub datetime: SchopeDateTime,
    pub band: CompactString,
    pub frequency: f64,
    pub mode: CompactString,
    pub call: CompactString,
}

impl LuaUserData for Record {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("datetime", |_, this| Ok(this.datetime));
        fields.add_field_method_get("band", |_, this| Ok(this.band.to_string()));
        fields.add_field_method_get("frequency", |_, this| Ok(this.frequency));
        fields.add_field_method_get("mode", |_, this| Ok(this.mode.to_string()));
        fields.add_field_method_get("call", |_, this| Ok(this.call.to_string()));
    }
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct Misc {
    pub antenna: Option<CompactString>,
    pub power: Option<f64>,
    pub operator: Option<CompactString>,
    pub address: Option<CompactString>,
    pub grid: Option<GridLocator>,
}

impl LuaUserData for Misc {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("antenna", |_, this| {
            Ok(this.antenna.as_ref().map(ToString::to_string))
        });
        fields.add_field_method_get("power", |_, this| Ok(this.power));
        fields.add_field_method_get("operator", |_, this| {
            Ok(this.operator.as_ref().map(ToString::to_string))
        });
        fields.add_field_method_get("address", |_, this| {
            Ok(this.address.as_ref().map(ToString::to_string))
        });
        fields.add_field_method_get("grid", |_, this| Ok(this.grid.map(|s| s.to_string())));
    }
}
