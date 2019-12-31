use super::ResponseBody;
use crate::error::ObsidianError;
use hyper::{Body, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::error::Error;

// use serde::ser::Serialize;

pub type ResponseResult<T = Response<Body>> = http::Result<T>;

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
}

impl<T> CustomResponder<T>
where
    T: ResponseBody,
{
    fn new(body: T) -> Self {
        CustomResponder { body, status: None }
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
        Response::builder().status(self.0).body(self.1.into_body())
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

// impl<T> Responder for ResponseType<T>
// where
//     T: Serialize,
// {
//     fn respond_to(self) -> ResponseResult {
//         match self {
//             ResponseType::JSON(body) => response::json(body, StatusCode::OK),
//         }
//     }
// }

// impl<T> Responder for (StatusCode, ResponseType<T>)
// where
//     T: Serialize,
// {
//     fn respond_to(self) -> ResponseResult {
//         match self.1 {
//             ResponseType::JSON(body) => response::json(body, self.0),
//         }
//     }
// }

// impl<T> Responder for (u16, ResponseType<T>)
// where
//     T: Serialize,
// {
//     fn respond_to(self) -> ResponseResult {
//         let status_code = match StatusCode::from_u16(self.0) {
//             Ok(status_code) => status_code,
//             Err(_) => {
//                 return Response::builder()
//                     .status(StatusCode::INTERNAL_SERVER_ERROR)
//                     .body("Invalid Status Code".into_body())
//             }
//         };

//         match self.1 {
//             ResponseType::JSON(body) => response::json(body, status_code),
//         }
//     }
// }

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
