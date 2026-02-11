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
