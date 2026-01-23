pub mod jarl;

use mlua::prelude::*;

pub trait CalculatorLibrary {
    fn create_module_table(lua: &Lua, args: LuaMultiValue) -> Result<LuaTable, LuaError>;
}
