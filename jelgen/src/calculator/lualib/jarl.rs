use mlua::prelude::*;

use crate::calculator::lualib::CalculatorLibrary;

#[derive(Debug)]
pub struct Jarl {}

impl Jarl {
    pub fn example() -> i64 {
        1
    }
}

impl CalculatorLibrary for Jarl {
    fn create_module_table(lua: &Lua, _: LuaMultiValue) -> Result<LuaTable, LuaError> {
        let t = lua.create_table()?;

        t.set(
            "example",
            lua.create_function(|_, _: LuaMultiValue| Ok(Self::example()))?,
        )?;

        Ok(t)
    }
}
