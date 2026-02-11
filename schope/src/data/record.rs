use common_qso::record::QsoRecord;
use compact_str::{CompactString, ToCompactString};
use mlua::prelude::*;
use time::OffsetDateTime;

use crate::library::datetime::SchopeDateTime;

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub datetime: SchopeDateTime,
    pub band: CompactString,
    pub freq: CompactString,
    pub mode: CompactString,
    pub call: CompactString,
}

impl LuaUserData for Record {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("datetime", |_, this| Ok(this.datetime));
        fields.add_field_method_get("band", |_, this| Ok(this.band.to_string()));
        fields.add_field_method_get("freq", |_, this| {
            this.freq.parse::<f64>().map_err(LuaError::external)
        });
        fields.add_field_method_get("freq_str", |_, this| Ok(this.freq.to_string()));
        fields.add_field_method_get("mode", |_, this| Ok(this.mode.to_string()));
        fields.add_field_method_get("call", |_, this| Ok(this.call.to_string()));
    }
}

impl From<QsoRecord> for Record {
    fn from(value: QsoRecord) -> Self {
        Record {
            datetime: OffsetDateTime::from(value.datetime).into(),
            band: value.band.to_compact_string(),
            freq: value.frequency,
            mode: value.mode,
            call: value.call,
        }
    }
}
