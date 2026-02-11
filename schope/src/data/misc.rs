use callfind::grid_locator::GridLocator;
use compact_str::CompactString;
use mlua::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Misc {
    pub antenna: Option<CompactString>,
    pub rig: Option<CompactString>,
    pub power: Option<f64>,
    pub operator: Option<CompactString>,
    pub address: Option<CompactString>,
    pub grid: Option<GridLocator>,
    pub manager: Option<CompactString>,
}

impl IntoLua for Misc {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("antenna", self.antenna.map(|s| s.to_string()))?;
        table.set("rig", self.rig.map(|s| s.to_string()))?;
        table.set("power", self.power)?;
        table.set("operator", self.operator.map(|s| s.to_string()))?;
        table.set("address", self.address.map(|s| s.to_string()))?;
        table.set("grid", self.grid.map(|s| s.to_string()))?;
        table.set("manager", self.manager.map(|s| s.to_string()))?;

        Ok(LuaValue::Table(table))
    }
}
