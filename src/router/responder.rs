use super::ResponseBody;
pub use hyper::{Body, Error, HeaderMap, Request, Response, StatusCode};

pub type ResponseResult<T = Response<Body>> = Result<T, http::Error>;

pub trait Responder {
    fn respond_to(self) -> ResponseResult;
    fn with_status(self, status: StatusCode) -> CustomResponder<Self>
    where
        Self: ResponseBody + Sized,
    {
        CustomResponder::new(self).with_status(status)
    }
}

/// Allows to override status code and headers for a responder.
pub struct CustomResponder<T> {
    body: T,
    status: Option<StatusCode>,
    // headers: Option<HeaderMap>,
    // error: Option<Error>,
}

impl<T> CustomResponder<T>
where
    T: ResponseBody,
{
    fn new(body: T) -> Self {
        CustomResponder {
            body,
            status: None,
            // headers: None,
            // error: None,
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }
}

impl<T> Responder for CustomResponder<T>
where
    T: ResponseBody,
{
    fn respond_to(self) -> ResponseResult {
        let status = match self.status {
            Some(status) => status,
            None => StatusCode::OK,
        };
        Response::builder()
            .status(status)
            .body(self.body.into_body())
    }
}

impl Responder for String {
    fn respond_to(self) -> ResponseResult {
        Response::builder()
            .status(StatusCode::OK)
            .body(self.into_body())
    }
}

impl Responder for &'static str {
    fn respond_to(self) -> ResponseResult {
        Response::builder()
            .status(StatusCode::OK)
            .body(self.into_body())
    }
}

impl<T> Responder for Option<T>
where
    T: ResponseBody,
{
    fn respond_to(self) -> ResponseResult {
        match self {
            Some(resp) => Response::builder()
                .status(StatusCode::OK)
                .body(resp.into_body()),
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(().into_body()),
        }
    }
}

impl<T, E> Responder for Result<T, E>
where
    T: ResponseBody,
    E: ResponseBody,
{
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
        Response::builder()
            .status(StatusCode::OK)
            .body(().into_body())
    }
}

impl<T> Responder for (StatusCode, T)
where
    T: ResponseBody,
{
    fn respond_to(self) -> ResponseResult {
        Response::builder().status(self.0).body(self.1.into_body())
    }
}
