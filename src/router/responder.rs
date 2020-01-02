use super::ResponseBody;
use crate::error::ObsidianError;
use hyper::{header::HeaderValue, Body, Response, StatusCode};
use std::error::Error;

pub type ResponseResult<T = Response<Body>> = http::Result<T>;

pub trait Responder {
    fn respond_to(self) -> ResponseResult;
    fn status(self, status: StatusCode) -> CustomResponder<Self>
    where
        Self: ResponseBody + Sized,
    {
        CustomResponder::new(self).status(status)
    }

    fn header(self, key: &'static str, value: &'static str) -> CustomResponder<Self>
    where
        Self: ResponseBody + Sized,
    {
        CustomResponder::new(self).header(key, value)
    }
}

/// Allows to override status code and headers for a responder.
pub struct CustomResponder<T> {
    body: T,
    status: Option<StatusCode>,
    headers: Option<Vec<(&'static str, &'static str)>>,
}

impl<T> CustomResponder<T>
where
    T: ResponseBody,
{
    fn new(body: T) -> Self {
        CustomResponder {
            body,
            status: None,
            headers: None,
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }

    pub fn header(mut self, key: &'static str, value: &'static str) -> Self {
        match self.headers {
            Some(ref mut x) => x.push((key, value)),
            None => {
                self.headers = Some(vec![(key, value)]);
            }
        };
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

        let mut res = Response::builder();
        if let Some(headers) = self.headers {
            match res.headers_mut() {
                Some(response_headers) => {
                    headers.iter().for_each(|(key, value)| {
                        response_headers.insert(*key, HeaderValue::from_static(value));
                    });
                }
                None => {}
            }
        }
        res.status(status).body(self.body.into_body())
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

impl<T> Responder for Result<T, ObsidianError>
where
    T: ResponseBody,
{
    fn respond_to(self) -> ResponseResult {
        match self {
            Ok(_) => Response::builder()
                .status(StatusCode::OK)
                .body(().into_body()),
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(e.description().to_string().into_body()),
        }
    }
}

impl Responder for Result<ResponseResult, ObsidianError> {
    fn respond_to(self) -> ResponseResult {
        match self {
            Ok(x) => x,
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(e.description().to_string().into_body()),
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
        let (status_code, body) = self;
        Response::builder()
            .status(status_code)
            .body(body.into_body())
    }
}

impl Responder for Vec<u8> {
    fn respond_to(self) -> ResponseResult {
        Response::builder()
            .status(StatusCode::OK)
            .body(self.into_body())
    }
}

impl Responder for StatusCode {
    fn respond_to(self) -> ResponseResult {
        Response::builder().status(self).body(().into_body())
    }
}

impl Responder for ResponseResult {
    fn respond_to(self) -> ResponseResult {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hyper::StatusCode;

    #[test]
    fn test_custom_responder() {
        let result = "Test".with_status(StatusCode::CREATED).respond_to();
        if let Ok(response) = result {
            assert_eq!(response.status(), StatusCode::CREATED);
        }
    }
}
