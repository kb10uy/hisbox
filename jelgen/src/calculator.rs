mod config;
pub mod data;
pub mod error;
mod lualib;

use std::{collections::HashMap, fs::read_to_string, path::Path};

use mlua::prelude::*;
use time::UtcOffset;
use tracing::{Level, debug, span};

use crate::{
    calculator::{
        config::CalculatorConfig,
        data::{ContestSummary, Record, RecordKey, RecordSummary},
        error::ProcessorError,
        lualib::{CalculatorLibrary, jarl::Jarl},
    },
    qso::record::QsoRecord,
};

#[derive(Debug)]
pub struct Calculator {
    lua: Lua,
    config: CalculatorConfig,
    qso_key: LuaFunction,
    process_qso: LuaFunction,
    summarize: LuaFunction,
}

impl Calculator {
    pub fn initialize(
        script_path: impl AsRef<Path>,
        args: HashMap<String, String>,
    ) -> Result<Calculator, ProcessorError> {
        let script_path = script_path.as_ref();
        let script_dir = script_path
            .parent()
            .ok_or(ProcessorError::InvalidPath)?
            .canonicalize()?;

        let lua = Lua::new();
        Self::initialize_lua(&lua, &script_dir)?;

        let script_text = read_to_string(script_path)?;
        let processor_table: LuaTable = lua.load(script_text).eval()?;

        let initialize: LuaFunction = processor_table.get("initialize")?;
        let qso_key: LuaFunction = processor_table.get("qso_key")?;
        let process_qso: LuaFunction = processor_table.get("process_qso")?;
        let summarize: LuaFunction = processor_table.get("summarize")?;

        let config_table: LuaTable =
            span!(Level::ERROR, "initialize").in_scope(|| initialize.call(args))?;
        let config = CalculatorConfig::new(config_table)?;

        Ok(Calculator {
            lua,
            config,
            qso_key,
            process_qso,
            summarize,
        })
    }

    pub fn process_offset(&self) -> UtcOffset {
        self.config.datetime_offset
    }

    pub fn convert_record(
        &self,
        qso_record: &QsoRecord,
        process_offset: UtcOffset,
    ) -> Option<Record> {
        Record::new(&self.lua, qso_record, process_offset)
    }

    pub fn key(&self, record: &Record) -> Result<RecordKey, ProcessorError> {
        let metadata = span!(Level::ERROR, "qso_key").in_scope(|| self.qso_key.call(&record.0))?;
        Ok(self.lua.from_value(metadata)?)
    }

    pub fn process(&self, record: &Record) -> Result<RecordSummary, ProcessorError> {
        let summary =
            span!(Level::ERROR, "process_qso").in_scope(|| self.process_qso.call(&record.0))?;
        Ok(self.lua.from_value(summary)?)
    }

    pub fn summarize<RS, M>(&self, groups: M) -> Result<ContestSummary, ProcessorError>
    where
        RS: IntoIterator<Item = (String, RecordSummary)>,
        M: IntoIterator<Item = (String, RS)>,
    {
        let groups: HashMap<_, Vec<_>> = groups
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        let groups_value = self.lua.to_value(&groups);

        let summary =
            span!(Level::ERROR, "summarize").in_scope(|| self.summarize.call(groups_value))?;
        Ok(self.lua.from_value(summary)?)
    }

    fn initialize_lua(lua: &Lua, script_root: &Path) -> Result<(), LuaError> {
        let root = script_root.to_string_lossy();
        let globals = lua.globals();
        let package: LuaTable = globals.get("package")?;

        // replace print() with debug!()
        let original: LuaFunction = globals.get("print")?;
        let hooked = lua.create_function(|_, args: LuaMultiValue| {
            let arg_texts: Result<Vec<_>, _> = args.iter().map(|v| v.to_string()).collect();
            debug!("{}", arg_texts?.join("\t"));
            Ok(())
        })?;

        globals.set("_print", original)?;
        globals.set("print", hooked)?;

        // set package path
        let package_config: String = package.get("config")?;
        let sep = package_config
            .lines()
            .next()
            .expect("package.config must exist");
        let package_path = format!("{root}{sep}?.lua;{root}{sep}?{sep}init.lua");
        package.set("path", package_path)?;

        // Set provided module loaders
        let package_preload: LuaTable = package.get("preload")?;
        package_preload.set("jarl", lua.create_function(Jarl::create_module_table)?)?;

        Ok(())
    }
}
