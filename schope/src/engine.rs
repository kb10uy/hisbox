use std::path::Path;

use mlua::prelude::*;
use tracing::debug;

use crate::library::{SchopeModule, datetime::DateTimeModule, jarl::JarlModule};

pub fn initialize_lua(script_base: &Path) -> Result<Lua, LuaError> {
    let lua = Lua::new();

    replace_print(&lua)?;
    set_package_path(&lua, script_base)?;
    register_provided_features(&lua)?;

    Ok(lua)
}

/// Replaces Lua's `print()` with `debug!()`.
fn replace_print(lua: &Lua) -> Result<(), LuaError> {
    let globals = lua.globals();

    let original: LuaFunction = globals.get("print")?;
    let hooked = lua.create_function(|_, args: LuaMultiValue| {
        let arg_texts: Result<Vec<_>, _> = args.iter().map(|v| v.to_string()).collect();
        debug!("{}", arg_texts?.join("\t"));
        Ok(())
    })?;

    globals.set("_print", original)?;
    globals.set("print", hooked)?;

    Ok(())
}

/// Sets Lua package path including `base_path`.
fn set_package_path(lua: &Lua, base_path: &Path) -> Result<(), LuaError> {
    let root = base_path.to_string_lossy();
    let globals = lua.globals();
    let package: LuaTable = globals.get("package")?;

    let package_config: String = package.get("config")?;
    let sep = package_config
        .lines()
        .next()
        .expect("package.config must exist");
    let package_path = format!("{root}{sep}?.lua;{root}{sep}?{sep}init.lua");
    package.set("path", package_path)?;

    Ok(())
}

fn register_provided_features(lua: &Lua) -> Result<(), LuaError> {
    let globals = lua.globals();
    let package: LuaTable = globals.get("package")?;

    let package_preload: LuaTable = package.get("preload")?;
    package_preload.set(
        "datetime",
        lua.create_function(DateTimeModule::create_module_table)?,
    )?;
    package_preload.set(
        "jarl",
        lua.create_function(JarlModule::create_module_table)?,
    )?;

    Ok(())
}
