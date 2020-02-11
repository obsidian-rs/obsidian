use super::Response;
use super::ResponseBody;
use hyper::{header, StatusCode};
use cookie::Cookie;

pub trait Responder {
    fn respond_to(self) -> Response;
    fn with_status(self, status: StatusCode) -> Response
    where
        Self: Responder + ResponseBody + Sized,
    {
        Response::new(self).set_status(status)
    }

    fn with_header(self, key: header::HeaderName, value: &'static str) -> Response
    where
        Self: Responder + ResponseBody + Sized,
    {
        Response::new(self).set_header(key, value)
    }

    fn with_headers(self, headers: Vec<(header::HeaderName, &'static str)>) -> Response
    where
        Self: Responder + ResponseBody + Sized,
    {
        Response::new(self).set_headers(headers)
    }

    fn with_headers_str(self, headers: Vec<(&'static str, &'static str)>) -> Response
    where
        Self: Responder + ResponseBody + Sized,
    {
        Response::new(self).set_headers_str(headers)
    }

    fn with_cookie(self, cookie: Cookie<'static>) -> Response
    where
        Self: Responder + ResponseBody + Sized,
    {
        Response::new(self).set_cookie(cookie)
    }

    fn with_cookies(self, cookies: Vec<Cookie<'static>>) -> Response
    where
        Self: Responder + ResponseBody + Sized,
    {
        Response::new(self).set_cookies(cookies)
    }

    fn with_cookie_raw(self, cookie: &str) -> Response
    where
        Self: Responder + ResponseBody + Sized,
    {
        Response::new(self).set_header(header::SET_COOKIE, cookie)
    }
}

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
                let error = e.to_string();
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

impl Responder for Option<String> {
    fn respond_to(self) -> Response {
        match self {
            Some(resp) => resp.respond_to(),
            None => "Not Found"
                .to_string()
                .with_status(StatusCode::NOT_FOUND)
                .respond_to(),
        }
    }
}

impl Responder for Option<&'static str> {
    fn respond_to(self) -> Response {
        match self {
            Some(resp) => resp.respond_to(),
            None => "Not Found"
                .to_string()
                .with_status(StatusCode::NOT_FOUND)
                .respond_to(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hyper::StatusCode;

    #[test]
    fn test_str_responder() {
        let response = "Hello World".respond_to();
        assert_eq!(response.status(), StatusCode::OK);
        // TODO: add testing for body once the Responder is refactored
    }

    #[test]
    fn test_string_responder() {
        let response = "Hello World".to_string().respond_to();
        assert_eq!(response.status(), StatusCode::OK);
        // TODO: add testing for body once the Responder is refactored
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
        let response = "Test".with_status(StatusCode::CREATED).respond_to();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[test]
    fn test_responder_with_custom_header() {
        let response = "Test"
            .with_header(header::CONTENT_TYPE, "application/json")
            .respond_to();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response
            .headers()
            .as_ref()
            .unwrap()
            .contains(&(header::CONTENT_TYPE, "application/json".to_string())));
    }
}
