use callfind::grid_locator::GridLocator;
use compact_str::CompactString;
use mlua::prelude::*;

use crate::data::{exchange::Exchange, record::Record};

#[derive(Debug, Clone, PartialEq)]
pub struct QslCardEntry {
    pub qso: Record,
    pub exchange: Exchange,
    pub info: QslInfo,
}

impl IntoLua for QslCardEntry {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("qso", self.qso)?;
        table.set("exchange", self.exchange)?;
        table.set("info", self.info)?;

        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QslInfo {
    pub instrument: QslInstrument,
    pub operation: QslOperation,
    pub card: QslCard,
}

impl IntoLua for QslInfo {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("instrument", self.instrument)?;
        table.set("operation", self.operation)?;
        table.set("manager", self.card)?;

        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QslInstrument {
    pub antenna: Option<CompactString>,
    pub rig: Option<CompactString>,
    pub power: Option<f64>,
}

impl IntoLua for QslInstrument {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("antenna", self.antenna.map(|s| s.to_string()))?;
        table.set("rig", self.rig.map(|s| s.to_string()))?;
        table.set("power", self.power)?;

        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QslOperation {
    pub operator: Option<CompactString>,
    pub address: Option<CompactString>,
    pub grid: Option<GridLocator>,
}

impl IntoLua for QslOperation {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("operator", self.operator.map(|s| s.to_string()))?;
        table.set("address", self.address.map(|s| s.to_string()))?;
        table.set("grid", self.grid.map(|s| s.to_string()))?;

        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QslCard {
    pub should_send: bool,
    pub received: bool,
    pub manager: Option<CompactString>,
}

impl IntoLua for QslCard {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("should_send", self.should_send)?;
        table.set("received", self.received)?;
        table.set("manager", self.manager.map(|s| s.to_string()))?;

        Ok(LuaValue::Table(table))
    }
}
