use std::{error::Error, io::Error as IoError};

use mlua::Error as LuaError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ProcessorError {
    #[error("invalid script path")]
    InvalidPath,

    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[error("Lua error: {0}")]
    Lua(#[from] LuaError),

    #[error("configuration error: {0}")]
    Configuration(Box<dyn Error + Send + Sync>),
}
