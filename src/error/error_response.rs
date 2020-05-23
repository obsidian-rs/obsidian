use crate::router::FormError;
use http;
use hyper::{Body, Response, StatusCode};
use serde_json::error::Error as JsonError;
use std::fmt;

#[derive(Debug)]
pub struct ObsidianError {
    inner: Box<dyn IntoErrorResponse>,
}

#[derive(Debug)]
pub enum InternalError {
    GeneralError(String),
    NoneError(String),
    ParamError(String),
}

impl fmt::Display for InternalError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let error_msg = match *self {
            InternalError::GeneralError(ref msg) => msg.to_string(),
            InternalError::NoneError(ref msg) => msg.to_string(),
            InternalError::ParamError(ref msg) => msg.to_string(),
        };

        formatter.write_str(&error_msg)
    }
}

impl ObsidianError {
    pub fn into_error_response(&self) -> Result<Response<Body>, http::Error> {
        self.inner.into_error_response()
    }
}

impl fmt::Display for ObsidianError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}

pub trait IntoErrorResponse: fmt::Debug + fmt::Display {
    /// convert Error into error response
    fn into_error_response(&self) -> Result<Response<Body>, http::Error> {
        let body = Body::from(self.to_string());
        Response::builder().status(self.status_code()).body(body)
    }

    /// status code for this error response
    /// this will return Internal Server Error (500) by default
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl<T: IntoErrorResponse + 'static> From<T> for ObsidianError {
    fn from(err: T) -> Self {
        ObsidianError {
            inner: Box::new(err),
        }
    }
}

impl IntoErrorResponse for JsonError {}
impl IntoErrorResponse for FormError {}
impl IntoErrorResponse for hyper::error::Error {}
impl IntoErrorResponse for String {}
impl IntoErrorResponse for InternalError {
    /// convert Error into error response
    fn into_error_response(&self) -> Result<Response<Body>, http::Error> {
        let body = Body::from(self.to_string());
        Response::builder().status(self.status_code()).body(body)
    }

    /// status code for this error response
    /// this will return Internal Server Error (500) by default
    fn status_code(&self) -> StatusCode {
        match self {
            InternalError::GeneralError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InternalError::NoneError(_) => StatusCode::NOT_FOUND,
            InternalError::ParamError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<std::convert::Infallible> for ObsidianError {
    fn from(_: std::convert::Infallible) -> Self {
        // `std::convert::Infallible` indicates an error
        // that will never happen
        unreachable!()
    }
}

// impl From<InternalError> for ObsidianError {
//     fn from(err: InternalError) -> Self {
//         ObsidianError {
//             inner: Box::new(err),
//         }
//     }
// }
