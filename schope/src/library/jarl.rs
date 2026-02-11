use mlua::prelude::*;

use crate::library::SchopeModule;

#[derive(Debug)]
pub struct JarlModule {}

impl JarlModule {
    pub fn example() -> i64 {
        1
    }
}

impl SchopeModule for JarlModule {
    fn create_module_table(lua: &Lua, _: LuaMultiValue) -> Result<LuaTable, LuaError> {
        let t = lua.create_table()?;

        t.set(
            "example",
            lua.create_function(|_, _: LuaMultiValue| Ok(Self::example()))?,
        )?;

        Ok(t)
    }
}
