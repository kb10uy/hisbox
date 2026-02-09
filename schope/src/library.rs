pub mod datetime;
pub mod jarl;

use mlua::prelude::*;

pub trait SchopeModule {
    fn create_module_table(lua: &Lua, args: LuaMultiValue) -> Result<LuaTable, LuaError>;
}
