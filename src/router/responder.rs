use super::ResponseBody;
use super::ResponseBuilder;
pub use hyper::{Body, Error, HeaderMap, Request, Response, StatusCode};

pub type ResponseResult<T = Response<Body>> = Result<T, http::Error>;

impl<T> Responder for ResponseResult<T>
where
    T: ResponseBody,
{
    fn respond_to(self) -> ResponseResult {
        let body = self?.into_body();

        Response::builder().status(StatusCode::OK).body(body)
    }
}

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

impl<T> Responder for CustomResponder<T>
where
    T: Responder,
{
    fn respond_to(self) -> ResponseResult {
        let response = self.responder.respond_to();

        match response {
            Ok(mut res) => {
                *res.status_mut() = self.status.unwrap();
                Ok(res)
            }
            Err(err) => Err(err),
        }
    }
}

impl Responder for String {
    fn respond_to(self) -> ResponseResult {
        let body = self.into_body();

        Response::builder().status(StatusCode::OK).body(body)
    }
}

impl Responder for &'static str {
    fn respond_to(self) -> ResponseResult {
        let body = self.into_body();

        Response::builder().status(StatusCode::OK).body(body)
    }
}

impl Responder for () {
    fn respond_to(self) -> ResponseResult {
        let body = self.into_body();

        Response::builder().status(StatusCode::OK).body(body)
    }
}

impl Responder for Response<Body> {
    fn respond_to(self) -> ResponseResult {
        Ok(self)
    }
}

/* impl Responder for ResponseBuilder {
    fn respond_to(self) -> ResponseResult {
        self
    }
} */

/* impl Responder for Result<String, ()> {
    fn respond_to(self) -> ResponseResult {
        match self {
            Ok(resp_body) => ResponseBuilder::new()
                .status(StatusCode::OK)
                .body(resp_body),
            Err(error) => ResponseBuilder::new()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error),
        }
    }
}

impl Responder for (StatusCode, String) {
    fn respond_to(self) -> ResponseResult {
        ResponseBuilder::new().status(self.0).body(self.1)
    }
}  */
