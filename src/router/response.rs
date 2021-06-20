use super::ResponseBody;

use async_std::fs;
use http::StatusCode;
use hyper::{header, Body};
use serde::ser::Serialize;

#[derive(Debug)]
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

    pub fn with_status(self, status: StatusCode) -> Self {
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

    // Alias set_header method
    pub fn with_header(self, key: header::HeaderName, value: &'static str) -> Self {
        self.set_header(key, value)
    }

    pub fn set_header_str(self, key: &'static str, value: &'static str) -> Self {
        self.set_header(
            header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
            value,
        )
    }

    // Alias set_header_str method
    pub fn with_header_str(self, key: &'static str, value: &'static str) -> Self {
        self.set_header_str(key, value)
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

    // Alias set_headers method
    pub fn with_headers(self, headers: Vec<(header::HeaderName, &'static str)>) -> Self {
        self.set_headers(headers)
    }

    pub fn set_headers_str(mut self, headers: Vec<(&'static str, &'static str)>) -> Self {
        let values: Vec<(header::HeaderName, &'static str)> = headers
            .iter()
            .map(|(k, v)| (header::HeaderName::from_bytes(k.as_bytes()).unwrap(), *v))
            .collect();

        match self.headers {
            Some(ref mut x) => x.extend_from_slice(&values),
            None => self.headers = Some(values),
        };
        self
    }

    // Alias set_headers_str method
    pub fn with_headers_str(self, headers: Vec<(&'static str, &'static str)>) -> Self {
        self.set_headers_str(headers)
    }

    pub fn html(self, body: impl ResponseBody) -> Self {
        self.set_content_type("text/html").set_body(body)
    }

    pub fn json(self, body: impl Serialize) -> Self {
        match serde_json::to_string(&body) {
            Ok(val) => self.set_content_type("application/json").set_body(val),
            Err(err) => self
                .set_content_type("application/json")
                .set_body(err.to_string())
                .set_status(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn file(self, file_path: &str) -> Self {
        match fs::read_to_string(file_path.to_string()).await {
            Ok(content) => self.set_body(content),
            Err(err) => {
                dbg!(&err);
                self.set_body(err.to_string())
                    .set_status(StatusCode::NOT_FOUND)
            }
        }
    }

    // Utilities
    pub fn ok() -> Self {
        Response::new(()).with_status(StatusCode::OK)
    }

    pub fn created() -> Self {
        Response::new(()).with_status(StatusCode::CREATED)
    }

    pub fn internal_server_error() -> Self {
        Response::new(()).with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hyper::StatusCode;
    use serde::*;

    #[test]
    fn test_response() {
        let response = Response::new("Hello World");
        assert_eq!(response.status(), StatusCode::OK);
        // TODO: add testing for body once the Responder is refactored
    }

    #[test]
    fn test_response_utilities_status() {
        assert_eq!(Response::ok().status(), StatusCode::OK);
        assert_eq!(Response::created().status(), StatusCode::CREATED);
        assert_eq!(
            Response::internal_server_error().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_complete_response() {
        #[derive(Serialize, Deserialize, Debug)]
        struct Person {
            name: String,
            age: i8,
        }

        let person = Person {
            name: String::from("Jun Kai"),
            age: 27,
        };
        let response = Response::created()
            .set_header(header::AUTHORIZATION, "token")
            .json(person);

        assert_eq!(response.status(), StatusCode::CREATED);
        assert!(response
            .headers()
            .as_ref()
            .unwrap()
            .contains(&(header::CONTENT_TYPE, "application/json")));
        assert!(response
            .headers()
            .as_ref()
            .unwrap()
            .contains(&(header::AUTHORIZATION, "token")));
    }
}
