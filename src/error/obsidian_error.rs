use std::fmt;
use std::fmt::Display;

use serde_json::error::Error as JsonError;

use crate::router::FormError;

/// Errors occurs in Obsidian framework
#[derive(Debug)]
pub enum ObsidianError {
    ParamError(String),
    JsonError(JsonError),
    FormError(FormError),
    GeneralError(String),
    NoneError,
}

impl Display for ObsidianError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let error_msg = match *self {
            ObsidianError::ParamError(ref msg) => msg.to_string(),
            ObsidianError::JsonError(ref err) => err.to_string(),
            ObsidianError::FormError(ref err) => err.to_string(),
            ObsidianError::GeneralError(ref msg) => msg.to_string(),
            ObsidianError::NoneError => "Input should not be None".to_string(),
        };

        formatter.write_str(&error_msg)
    }
}

impl From<FormError> for ObsidianError {
    fn from(error: FormError) -> Self {
        ObsidianError::FormError(error)
    }
}

impl From<JsonError> for ObsidianError {
    fn from(error: JsonError) -> Self {
        ObsidianError::JsonError(error)
    }
}
