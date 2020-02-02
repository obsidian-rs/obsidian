use super::Response;
use super::ResponseBody;
use hyper::{header, StatusCode};

pub trait Responder {
    fn respond_to(self) -> Response;
    fn with_status(self, status: StatusCode) -> Response
    where
        Self: Responder + ResponseBody + Sized,
    {
        Response::new(self).set_status(status)
    }

    fn header(self, key: header::HeaderName, value: &'static str) -> Response
    where
        Self: Responder + ResponseBody + Sized,
    {
        Response::new(self).set_header(key, value)
    }

    fn set_headers(self, headers: Vec<(header::HeaderName, &'static str)>) -> Response
    where
        Self: Responder + ResponseBody + Sized,
    {
        Response::new(self).set_headers(headers)
    }
}

// impl<T> Responder for CustomResponder<T>
// where
//     T: ResponseBody,
// {
//     fn respond_to(self) -> Response {
//         let status = match self.status {
//             Some(status) => status,
//             None => StatusCode::OK,
//         };

//         let mut res = Response::new(status);

//         if let Some(headers) = self.headers {
//             res.set_headers(headers).set_body(self.body);
//         }

//         res
//     }
// }

// enum Either<A, B> {
//     Left(A),
//     Right(B),
// }

// impl<A, B> Responder for Either<A, B>
// where
//     A: Responder,
//     B: Responder,
// {
//     fn respond_to(self) -> ResponseResult {
//         match self {
//             Either::Left(a) => a.respond_to(),
//             Either::Right(b) => b.respond_to(),
//         }
//     }
// }

impl Responder for Response {
    fn respond_to(self) -> Response {
        self
    }
}

impl Responder for String {
    fn respond_to(self) -> Response {
        Response::new(self).set_content_type("text/plain; charset=utf-8")
    }
}

impl Responder for &'static str {
    fn respond_to(self) -> Response {
        self.to_string().respond_to()
    }
}

impl Responder for () {
    fn respond_to(self) -> Response {
        Response::new(())
    }
}

impl<T> Responder for (StatusCode, T)
where
    T: Responder + ResponseBody,
{
    fn respond_to(self) -> Response {
        let (status_code, body) = self;
        Response::new(body).set_status(status_code)
    }
}

impl Responder for Vec<u8> {
    fn respond_to(self) -> Response {
        match serde_json::to_string(&self) {
            Ok(json) => json.with_status(StatusCode::OK).respond_to(),
            Err(e) => {
                eprintln!("serializing failed: {}", e);
                let error = std::error::Error::description(&e).to_string();
                error
                    .with_status(StatusCode::INTERNAL_SERVER_ERROR)
                    .respond_to()
            }
        }
    }
}

impl Responder for StatusCode {
    fn respond_to(self) -> Response {
        ().with_status(self).respond_to()
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
