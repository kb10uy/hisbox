mod config;
pub mod data;
mod error;
mod lualib;

use std::{collections::HashMap, fs::read_to_string, path::Path};

use mlua::prelude::*;
use time::UtcOffset;
use tracing::{Level, debug, span};

use crate::{
    processor::{
        config::ProcessorConfig,
        data::{QsoMetadata, QsoSummary, RecordInner},
        error::ProcessorError,
        lualib::{ProcessorLuaLibrary, jarl::Jarl},
    },
    qso::record::QsoRecord,
};

#[derive(Debug)]
pub struct Processor {
    lua: Lua,
    config: ProcessorConfig,
    qso_metadata: LuaFunction,
    process_qso: LuaFunction,
    calculate_total: LuaFunction,
}

impl Processor {
    pub fn initialize(
        script_path: impl AsRef<Path>,
        args: HashMap<String, String>,
    ) -> Result<Processor, ProcessorError> {
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
        let qso_metadata: LuaFunction = processor_table.get("qso_metadata")?;
        let process_qso: LuaFunction = processor_table.get("process_qso")?;
        let calculate_total: LuaFunction = processor_table.get("calculate_total")?;

        let config_table: LuaTable = initialize.call(args)?;
        let config = ProcessorConfig::new(config_table)?;

        Ok(Processor {
            lua,
            config,
            qso_metadata,
            process_qso,
            calculate_total,
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

    pub fn metadata(&self, record: &Record) -> Result<QsoMetadata, ProcessorError> {
        let metadata =
            span!(Level::ERROR, "qso_metadata").in_scope(|| self.qso_metadata.call(&record.0))?;
        Ok(self.lua.from_value(metadata)?)
    }

    pub fn process(&self, record: &Record) -> Result<QsoSummary, ProcessorError> {
        let summary =
            span!(Level::ERROR, "process_qso").in_scope(|| self.process_qso.call(&record.0))?;
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

#[derive(Debug)]
#[repr(transparent)]
pub struct Record(LuaValue);

impl Record {
    pub fn new(lua: &Lua, qso_record: &QsoRecord, process_offset: UtcOffset) -> Option<Record> {
        let inner = RecordInner::new(qso_record, process_offset)?;
        Some(Record(lua.to_value(&inner).ok()?))
    }
}
