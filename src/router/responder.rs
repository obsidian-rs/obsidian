use super::ResponseBody;
pub use hyper::{Body, Error, HeaderMap, Request, Response, StatusCode};

pub type ResponseResult<T = Response<Body>> = Result<T, http::Error>;

pub trait Responder {
    fn respond_to(self) -> ResponseResult;
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
    fn respond_to(self) -> ResponseResult {
        Response::builder().status(StatusCode::OK).body(self.into_body())
    }
}

impl Responder for &'static str {
    fn respond_to(self) -> ResponseResult {
        Response::builder().status(StatusCode::OK).body(self.into_body())
    }
}

impl Responder for Result<String, ()> {
    fn respond_to(self) -> ResponseResult {
        match self {
            Ok(resp_body) => Response::builder()
                .status(StatusCode::OK)
                .body(resp_body.into_body()),
            Err(error) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.into_body()),
        }
    }
}

impl Responder for Result<&'static str, ()> {
    fn respond_to(self) -> ResponseResult {
        match self {
            Ok(resp_body) => Response::builder()
                .status(StatusCode::OK)
                .body(resp_body.into_body()),
            Err(error) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.into_body()),
        }
    }
}

impl Responder for () {
    fn respond_to(self) -> ResponseResult {
        Response::builder().status(StatusCode::OK).body(().into_body())
    }
}

impl Responder for (StatusCode, String) {
    fn respond_to(self) -> ResponseResult {
        Response::builder().status(self.0).body(self.1.into_body())
    }
}
