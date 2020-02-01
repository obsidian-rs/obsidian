use super::{Responder, ResponseBody};

use async_std::fs;
use http::StatusCode;
use hyper::{header, Body};
use serde::ser::Serialize;
use serde_json;

// Response::new(200).json()
pub struct Response {
    body: Body,
    status: StatusCode,
    headers: Option<Vec<(header::HeaderName, &'static str)>>,
}

impl Response {
    pub fn new(body: impl ResponseBody) -> Self {
        Response {
            body: body.into_body(),
            status: StatusCode::OK,
            headers: None,
        }
    }

    // pub fn json(body: Body) -> Self {
    //     self.set_content_type("application/json");
    //     self.set_body(body)
    // }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn status_mut(&mut self) -> &mut StatusCode {
        &mut self.status
    }

    pub fn body(self) -> Body {
        self.body
    }

    pub fn headers(&self) -> &Option<Vec<(header::HeaderName, &'static str)>> {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut Option<Vec<(header::HeaderName, &'static str)>> {
        &mut self.headers
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.set_status(status)
    }

    pub fn set_status(mut self, status: http::StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn set_body(mut self, body: impl ResponseBody) -> Self {
        self.body = body.into_body();
        self
    }

    pub fn set_header(mut self, key: header::HeaderName, value: &'static str) -> Self {
        match self.headers {
            Some(ref mut x) => x.push((key, value)),
            None => self.headers = Some(vec![(key, value)]),
        };
        self
    }

    pub fn set_content_type(self, content_type: &'static str) -> Self {
        self.set_header(header::CONTENT_TYPE, content_type)
    }

    pub fn set_headers(mut self, headers: Vec<(header::HeaderName, &'static str)>) -> Self {
        match self.headers {
            Some(ref mut x) => x.extend_from_slice(&headers),
            None => self.headers = Some(headers),
        };
        self
    }

    // pub fn set_headers(mut self, headers: Vec<(header::HeaderName, &'static str)>) -> Self {
    //     let response_headers = self.res.headers_mut();

    //     headers.iter().for_each(move |(key, value)| {
    //         response_headers.insert(key, header::HeaderValue::from_static(value));
    //     });

    //     self
    // }

    pub fn json(body: impl Serialize) -> Self {
        match serde_json::to_string(&body) {
            Ok(val) => Response::new(val).set_content_type("application/json"),
            Err(err) => Response::new(std::error::Error::description(&err).to_string())
                .set_status(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn file(file_path: &str) -> Self {
        match fs::read_to_string(file_path.to_string()).await {
            Ok(content) => Response::new(content),
            Err(err) => {
                dbg!(&err);
                Response::new(std::error::Error::description(&err).to_string())
                    .set_status(StatusCode::NOT_FOUND)
            }
        }
    }
}

// pub fn json(body: impl Serialize) -> CustomResponder<String> {
//     match serde_json::to_string(&body) {
//         Ok(val) => val
//             .status(StatusCode::OK)
//             .header(header::CONTENT_TYPE, "application/json"),
//         Err(err) => std::error::Error::description(&err)
//             .to_string()
//             .status(StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }

// pub async fn file(file_path: &str) -> impl Responder {
//     match fs::read_to_string(file_path.to_string()).await {
//         Ok(content) => content.status(StatusCode::OK),
//         Err(err) => {
//             dbg!(&err);
//             std::error::Error::description(&err)
//                 .to_string()
//                 .status(StatusCode::NOT_FOUND)
//         }
//     }
// }
