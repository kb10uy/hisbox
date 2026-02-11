use mlua::prelude::*;

use crate::data::{exchange::Exchange, misc::Misc, record::Record};

#[derive(Debug, Clone, PartialEq)]
pub struct QslCardEntry {
    pub qso: Record,
    pub exchange: Exchange,
    pub misc: Misc,
}

impl IntoLua for QslCardEntry {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("qso", self.qso)?;
        table.set("exchange", self.exchange)?;
        table.set("misc", self.misc)?;

        Ok(LuaValue::Table(table))
    }
}
