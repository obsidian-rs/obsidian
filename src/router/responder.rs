use super::ResponseBuilder;
pub use hyper::{Body, Error, HeaderMap, Request, Response, StatusCode};

pub trait Responder {
    fn respond_to(self) -> ResponseBuilder;
    fn with_status(self, status: StatusCode) -> CustomResponder<Self>
    where
        Self: Sized,
    {
        CustomResponder::new(self).with_status(status)
    }
}

/// Allows to override status code and headers for a responder.
pub struct CustomResponder<T> {
    responder: T,
    status: Option<StatusCode>,
    headers: Option<HeaderMap>,
    error: Option<Error>,
}

impl<T: Responder> CustomResponder<T> {
    fn new(responder: T) -> Self {
        CustomResponder {
            responder,
            status: None,
            headers: None,
            error: None,
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }
}

impl Responder for String {
    fn respond_to(self) -> ResponseBuilder {
        ResponseBuilder::new().status(StatusCode::OK).body(self)
    }
}

impl Responder for ResponseBuilder {
    fn respond_to(self) -> ResponseBuilder {
        ResponseBuilder::new().status(StatusCode::OK).body(self)
    }
}

impl Responder for Result<(), ()> {
    fn respond_to(self) -> ResponseBuilder {
        match self {

        }
    }
}
