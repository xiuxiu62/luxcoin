use std::error;

use thiserror::Error;

pub type LuxResult<T> = Result<T, LuxError>;

#[derive(Debug, Error)]
pub enum LuxError {
    #[error("0")]
    InvalidTransaction(String),
    #[error("Unknown error: {0}")]
    Unknown(Box<dyn error::Error>),
}
