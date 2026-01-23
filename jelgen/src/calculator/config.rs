use mlua::prelude::*;
use time::{UtcOffset, format_description::BorrowedFormatItem, macros::format_description};

use crate::calculator::error::ProcessorError;

const CUSTOM_OFFSET: &[BorrowedFormatItem<'_>] =
    format_description!("[offset_hour]:[offset_minute]");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculatorConfig {
    pub datetime_offset: UtcOffset,
}

impl CalculatorConfig {
    pub fn new(table: LuaTable) -> Result<CalculatorConfig, ProcessorError> {
        let datetime_offset =
            UtcOffset::parse(&table.get::<String>("datetime_offset")?, CUSTOM_OFFSET)
                .map_err(|e| ProcessorError::Configuration(Box::new(e)))?;

        Ok(CalculatorConfig { datetime_offset })
    }
}
