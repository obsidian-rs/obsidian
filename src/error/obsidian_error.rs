use std::error::Error;
use std::fmt;
use std::fmt::Display;

use super::IntoErrorResponse;
use serde_json::error::Error as JsonError;

use crate::router::FormError;

/// Errors occurs in Obsidian framework

#[derive(Debug)]
pub enum ObsidianErrorKind {
    CustomError(Box<dyn IntoErrorResponse>),
    ParamError(String),
    JsonError(JsonError),
    FormError(FormError),
    GeneralError(String),
    NoneError,
}

impl std::error::Error for ObsidianErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ObsidianErrorKind::JsonError(ref error) => Some(error),
            ObsidianErrorKind::FormError(ref error) => Some(error),
            _ => None,
        }
    }
}

impl Display for ObsidianErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let error_msg = match *self {
            ObsidianErrorKind::CustomError(ref err) => err.to_string(),
            ObsidianErrorKind::ParamError(ref msg) => msg.to_string(),
            ObsidianErrorKind::JsonError(ref err) => err.to_string(),
            ObsidianErrorKind::FormError(ref err) => err.to_string(),
            ObsidianErrorKind::GeneralError(ref msg) => msg.to_string(),
            ObsidianErrorKind::NoneError => "Input should not be None".to_string(),
        };

        formatter.write_str(&error_msg)
    }
}

// impl From<FormError> for ObsidianError {
//     fn from(error: FormError) -> Self {
//         ObsidianError::FormError(error)
//     }
// }

// impl From<JsonError> for ObsidianError {
//     fn from(error: JsonError) -> Self {
//         ObsidianError::JsonError(error)
//     }
// }
