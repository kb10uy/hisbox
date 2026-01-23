mod config;
pub mod data;
mod error;

use std::{collections::HashMap, fs::read_to_string, path::Path};

use mlua::prelude::*;
use time::UtcOffset;
use tracing::{Level, debug, span};

use crate::{
    processor::{
        config::ProcessorConfig,
        data::{QsoMetadata, QsoSummary, RecordInner},
        error::ProcessorError,
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
        module_path: impl AsRef<Path>,
        args: HashMap<String, String>,
    ) -> Result<Processor, ProcessorError> {
        let lua = Lua::new();
        Self::replace_print(&lua)?;

        let module_path = module_path.as_ref();
        let module_script = read_to_string(module_path)?;
        let module: LuaTable = lua.load(module_script).eval()?;

        let initialize: LuaFunction = module.get("initialize")?;
        let qso_metadata: LuaFunction = module.get("qso_metadata")?;
        let process_qso: LuaFunction = module.get("process_qso")?;
        let calculate_total: LuaFunction = module.get("calculate_total")?;

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
        let metadata = span!(Level::ERROR, "qso_metadata").in_scope(|| self.qso_metadata.call(&record.0))?;
        Ok(self.lua.from_value(metadata)?)
    }

    pub fn process(&self, record: &Record) -> Result<QsoSummary, ProcessorError> {
        let summary = span!(Level::ERROR, "process_qso").in_scope(|| self.process_qso.call(&record.0))?;
        Ok(self.lua.from_value(summary)?)
    }

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
