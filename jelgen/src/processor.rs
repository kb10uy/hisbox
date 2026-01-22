mod config;
pub mod data;
mod error;

use std::{collections::HashMap, fs::read_to_string, path::Path};

use mlua::prelude::*;
use time::UtcOffset;

use crate::processor::{
    config::ProcessorConfig,
    data::{QsoSummary, Record},
    error::ProcessorError,
};

#[derive(Debug)]
pub struct Processor {
    lua: Lua,
    config: ProcessorConfig,
    process_qso: LuaFunction,
}

impl Processor {
    pub fn initialize(
        module_path: impl AsRef<Path>,
        args: HashMap<String, String>,
    ) -> Result<Processor, ProcessorError> {
        let lua = Lua::new();

        let module_path = module_path.as_ref();
        let module_script = read_to_string(module_path)?;
        let module: LuaTable = lua.load(module_script).eval()?;

        let initialize: LuaFunction = module.get("initialize")?;
        let process_qso: LuaFunction = module.get("process_qso")?;

        let config_table: LuaTable = initialize.call(args)?;
        let config = ProcessorConfig::new(config_table)?;

        Ok(Processor {
            lua,
            config,
            process_qso,
        })
    }

    pub fn process_offset(&self) -> UtcOffset {
        self.config.datetime_offset
    }

    pub fn process(&self, record: Record) -> Result<QsoSummary, ProcessorError> {
        let summary = self.process_qso.call(self.lua.to_value(&record))?;
        Ok(self.lua.from_value(summary)?)
    }
}
