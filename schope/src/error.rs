use std::path::PathBuf;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum SchopeError {
    #[error("invalid script path")]
    InvalidPath(PathBuf),
}
