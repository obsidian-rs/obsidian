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
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for ObsidianError {
    fn description(&self) -> &str {
        match *self {
            ObsidianError::ParamError(ref msg) => msg,
            ObsidianError::JsonError(ref err) => std::error::Error::description(err),
            ObsidianError::FormError(ref err) => std::error::Error::description(err),
            ObsidianError::GeneralError(ref msg) => msg,
            ObsidianError::NoneError => "Input should not be None",
        }
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
