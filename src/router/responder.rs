use super::ResponseBody;
use hyper::{header, Body, Response, StatusCode};

pub type ResponseResult<T = Body> = http::Result<Response<T>>;

pub trait Responder {
    fn respond_to(self) -> ResponseResult;
    fn status(self, status: StatusCode) -> CustomResponder<Self>
    where
        Self: Responder + ResponseBody + Sized,
    {
        CustomResponder::new(self).status(status)
    }

    fn header(self, key: header::HeaderName, value: &'static str) -> CustomResponder<Self>
    where
        Self: Responder + ResponseBody + Sized,
    {
        CustomResponder::new(self).header(key, value)
    }

    fn set_headers(self, headers: Vec<(header::HeaderName, &'static str)>) -> CustomResponder<Self>
    where
        Self: Responder + ResponseBody + Sized,
    {
        CustomResponder::new(self).set_headers(headers)
    }
}

/// Allows to override status code and headers for a responder.
pub struct CustomResponder<T> {
    body: T,
    status: Option<StatusCode>,
    headers: Option<Vec<(header::HeaderName, &'static str)>>,
}

impl<T> CustomResponder<T>
where
    T: Responder + ResponseBody,
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

    pub fn header(mut self, key: header::HeaderName, value: &'static str) -> Self {
        match self.headers {
            Some(ref mut x) => x.push((key, value)),
            None => self.headers = Some(vec![(key, value)]),
        };
        self
    }

    pub fn set_headers(mut self, headers: Vec<(header::HeaderName, &'static str)>) -> Self {
        match self.headers {
            Some(ref mut x) => x.extend_from_slice(&headers),
            None => self.headers = Some(headers),
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
                    headers.iter().for_each(move |(key, value)| {
                        response_headers.insert(key, header::HeaderValue::from_static(value));
                    });
                }
                None => {}
            }
        }
        res.status(status).body(self.body.into_body())
    }
}

enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Responder for Either<A, B>
where
    A: Responder,
    B: Responder,
{
    fn respond_to(self) -> ResponseResult {
        match self {
            Either::Left(a) => a.respond_to(),
            Either::Right(b) => b.respond_to(),
        }
    }
}

impl Responder for String {
    fn respond_to(self) -> ResponseResult {
        self.header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .status(StatusCode::OK)
            .respond_to()
    }
}

impl Responder for &'static str {
    fn respond_to(self) -> ResponseResult {
        self.to_string().respond_to()
    }
}

impl Responder for () {
    fn respond_to(self) -> ResponseResult {
        ().status(StatusCode::OK).respond_to()
    }
}

impl<T> Responder for (StatusCode, T)
where
    T: Responder + ResponseBody,
{
    fn respond_to(self) -> ResponseResult {
        let (status_code, body) = self;
        body.status(status_code).respond_to()
    }
}

impl Responder for Vec<u8> {
    fn respond_to(self) -> ResponseResult {
        match serde_json::to_string(&self) {
            Ok(json) => json.status(StatusCode::OK).respond_to(),
            Err(e) => {
                eprintln!("serializing failed: {}", e);
                let error = std::error::Error::description(&e).to_string();
                error.status(StatusCode::INTERNAL_SERVER_ERROR).respond_to()
            }
        }
    }
}

impl Responder for ResponseResult {
    fn respond_to(self) -> ResponseResult {
        self
    }
}

impl Responder for StatusCode {
    fn respond_to(self) -> ResponseResult {
        ().status(self).respond_to()
    }
}

// impl Responder for Option<String> {
//     fn respond_to(self) -> ResponseResult {
//         match self {
//             Some(resp) => resp.respond_to(),
//             None => "Not Found"
//                 .to_string()
//                 .status(StatusCode::NOT_FOUND)
//                 .respond_to(),
//         }
//     }
// }

// impl Responder for Option<&'static str> {
//     fn respond_to(self) -> ResponseResult {
//         match self {
//             Some(resp) => resp.respond_to(),
//             None => "Not Found"
//                 .to_string()
//                 .status(StatusCode::NOT_FOUND)
//                 .respond_to(),
//         }
//     }
// }

// impl Responder for Option<Vec<u8>> {
//     fn respond_to(self) -> ResponseResult {
//         match self {
//             Some(resp) => resp.respond_to(),
//             None => "Not Found"
//                 .to_string()
//                 .status(StatusCode::NOT_FOUND)
//                 .respond_to(),
//         }
//     }
// }

// impl<T, E> Responder for Result<T, E>
// where
//     T: ResponseBody,
//     E: ResponseBody,
// {
//     fn respond_to(self) -> ResponseResult {
//         match self {
//             Ok(resp_body) => Response::builder()
//                 .status(StatusCode::OK)
//                 .body(resp_body.into_body()),
//             Err(error) => Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body(error.into_body()),
//         }
//     }
// }

// impl<T> Responder for Result<T, ObsidianError>
// where
//     T: ResponseBody,
// {
//     fn respond_to(self) -> ResponseResult {
//         match self {
//             Ok(_) => Response::builder()
//                 .status(StatusCode::OK)
//                 .body(().into_body()),
//             Err(e) => Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body(e.description().to_string().into_body()),
//         }
//     }
// }

// impl Responder for Result<ResponseResult, ObsidianError> {
//     fn respond_to(self) -> ResponseResult {
//         match self {
//             Ok(x) => x,
//             Err(e) => Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body(e.description().to_string().into_body()),
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use hyper::StatusCode;

    #[test]
    fn test_str_responder() {
        let result = "Hello World".respond_to();
        if let Ok(response) = result {
            assert_eq!(response.status(), StatusCode::OK);
            // TODO: add testing for body once the Responder is refactored
        }
    }

    #[test]
    fn test_string_responder() {
        let result = "Hello World".to_string().respond_to();
        if let Ok(response) = result {
            assert_eq!(response.status(), StatusCode::OK);
            // TODO: add testing for body once the Responder is refactored
        }
    }

    // #[test]
    // fn test_option_responder() {
    //     let some_result = Some("Hello World").respond_to();
    //     if let Ok(response) = some_result {
    //         assert_eq!(response.status(), StatusCode::OK);
    //         // TODO: add testing for body once the Responder is refactored
    //     }

    //     let none_result = None::<String>.respond_to();
    //     if let Ok(response) = none_result {
    //         assert_eq!(response.status(), StatusCode::NOT_FOUND);
    //         // TODO: add testing for body once the Responder is refactored
    //     }
    // }

    // #[test]
    // fn test_result_responder() {
    //     let ok_result = Ok::<&str, &str>("Hello World").respond_to();
    //     if let Ok(response) = ok_result {
    //         assert_eq!(response.status(), StatusCode::OK);
    //         // TODO: add testing for body once the Responder is refactored
    //     }

    //     let err_result = Err::<&str, &str>("Some error").respond_to();
    //     if let Ok(response) = err_result {
    //         assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    //         // TODO: add testing for body once the Responder is refactored
    //     }
    // }

    #[test]
    fn test_responder_with_custom_status() {
        let result = "Test".status(StatusCode::CREATED).respond_to();
        if let Ok(response) = result {
            assert_eq!(response.status(), StatusCode::CREATED);
        }
    }

    #[test]
    fn test_responder_with_custom_header() {
        let result = "Test"
            .header(header::CONTENT_TYPE, "application/json")
            .respond_to();
        if let Ok(response) = result {
            assert_eq!(response.status(), StatusCode::OK);
            assert!(response.headers().contains_key("Content-Type"));
        }
    }
}
