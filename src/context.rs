pub mod cookies;
pub mod memory_session;
pub mod session;

use cookie::Cookie;
use http::Extensions;
use hyper::{body, body::Buf};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use url::form_urlencoded;

use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::From;
use std::str::FromStr;

use self::cookies::CookieParserData;
use self::session::SessionData;
use crate::router::{from_cow_map, ContextResult, Responder, Response};
use crate::ObsidianError;
use crate::{
    header::{HeaderName, HeaderValue},
    Body, HeaderMap, Method, Request, StatusCode, Uri,
};

/// Context contains the data for current http connection context.
/// For example, request information, params, method, and path.
#[derive(Debug)]
pub struct Context {
    request: Request<Body>,
    params_data: HashMap<String, String>,
    response: Option<Response>,
}

impl Context {
    pub fn new(request: Request<Body>, params_data: HashMap<String, String>) -> Self {
        Self {
            request,
            params_data,
            response: None,
        }
    }

    /// Access request headers
    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.request.headers()
    }

    /// Access mutable request header
    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        self.request.headers_mut()
    }

    /// Access request method
    pub fn method(&self) -> &Method {
        self.request.method()
    }

    /// Access request uri
    pub fn uri(&self) -> &Uri {
        self.request.uri()
    }

    /// Access request extensions
    pub fn extensions(&self) -> &Extensions {
        self.request.extensions()
    }

    /// Access mutable request extensions
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        self.request.extensions_mut()
    }

    /// Add dynamic data into request extensions
    pub fn add<T: Send + Sync + 'static>(&mut self, ctx_data: T) {
        self.extensions_mut().insert(ctx_data);
    }

    /// Get dynamic data from request extensions
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.extensions().get::<T>()
    }

    /// Get mutable dynamic data from request extensions
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.extensions_mut().get_mut::<T>()
    }

    /// Method to get the params value according to key.
    /// Panic if key is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use obsidian::StatusCode;
    /// # use obsidian::ContextResult;
    /// # use obsidian::context::Context;
    ///
    /// // Assumming ctx contains params for id and mode
    /// async fn get_handler(ctx: Context) -> ContextResult {
    ///     let id: i32 = ctx.param("id")?;
    ///     let mode: String = ctx.param("mode")?;
    ///
    ///     assert_eq!(id, 1);
    ///     assert_eq!(mode, "edit".to_string());
    ///
    ///     ctx.build("").ok()
    /// }
    ///
    /// ```
    ///
    pub fn param<T: FromStr>(&self, key: &str) -> Result<T, ObsidianError> {
        self.params_data
            .get(key)
            .ok_or(ObsidianError::NoneError)?
            .parse()
            .map_err(|_err| ObsidianError::ParamError(format!("Failed to parse param {}", key)))
    }

    /// Method to get the string query data from the request url.
    /// Untagged is not supported
    ///
    /// # Example
    /// ```
    /// # use serde::*;
    ///
    /// # use obsidian::context::Context;
    /// # use obsidian::{ContextResult, StatusCode};
    ///
    /// #[derive(Deserialize, Serialize, Debug)]
    /// struct QueryString {
    ///     id: i32,
    ///     mode: String,
    /// }
    ///
    /// // Assume ctx contains string query with data {id=1&mode=edit}
    /// async fn get_handler(mut ctx: Context) -> ContextResult {
    ///     let result: QueryString = ctx.query_params()?;
    ///
    ///     assert_eq!(result.id, 1);
    ///     assert_eq!(result.mode, "edit".to_string());
    ///
    ///     ctx.build("").ok()
    /// }
    /// ```
    pub fn query_params<T: DeserializeOwned>(&mut self) -> Result<T, ObsidianError> {
        let query = match self.uri().query() {
            Some(query) => query,
            _ => "",
        }
        .as_bytes();

        Self::parse_queries(&query)
    }

    /// Method to get the forms query data from the request body.
    /// Body is consumed after calling this method.
    /// Untagged is not supported
    ///
    /// # Example
    /// ```
    /// # use serde::*;
    ///
    /// # use obsidian::context::Context;
    /// # use obsidian::{ContextResult, StatusCode};
    ///
    /// #[derive(Deserialize, Serialize, Debug)]
    /// struct FormResult {
    ///     id: i32,
    ///     mode: String,
    /// }
    ///
    /// // Assume ctx contains form query with data {id=1&mode=edit}
    /// async fn get_handler(mut ctx: Context) -> ContextResult {
    ///     let result: FormResult = ctx.form().await?;
    ///
    ///     assert_eq!(result.id, 1);
    ///     assert_eq!(result.mode, "edit".to_string());
    ///
    ///     ctx.build("").ok()
    /// }
    /// ```
    pub async fn form<T: DeserializeOwned>(&mut self) -> Result<T, ObsidianError> {
        let body = self.take_body();

        let buf = match body::aggregate(body).await {
            Ok(buf) => buf,
            _ => {
                return Err(ObsidianError::NoneError);
            }
        };

        Self::parse_queries(buf.bytes())
    }

    /// Form value merge with Params
    pub fn form_with_param<T: DeserializeOwned>(&mut self) -> Result<T, ()> {
        unimplemented!()
    }

    /// Method to get the json data from the request body. Body is consumed after calling this method.
    /// The result can be either handled by using static type or dynamic map.
    /// Panic if parsing fail.
    ///
    /// # Example
    ///
    /// ### Handle by static type
    /// ```
    /// # use serde::*;
    ///
    /// # use obsidian::context::Context;
    /// # use obsidian::{ContextResult, StatusCode};
    ///
    /// #[derive(Deserialize, Serialize, Debug)]
    /// struct JsonResult {
    ///     id: i32,
    ///     mode: String,
    /// }
    ///
    /// // Assume ctx contains json with data {id:1, mode:'edit'}
    /// async fn get_handler(mut ctx: Context) -> ContextResult {
    ///     let result: JsonResult = ctx.json().await?;
    ///
    ///     assert_eq!(result.id, 1);
    ///     assert_eq!(result.mode, "edit".to_string());
    ///
    ///     ctx.build("").ok()
    /// }
    /// ```
    ///
    /// ### Handle by dynamic map
    /// ```
    /// # use serde_json::Value;
    ///
    /// # use obsidian::context::Context;
    /// # use obsidian::{ContextResult, StatusCode};
    ///
    /// // Assume ctx contains json with data {id:1, mode:'edit'}
    /// async fn get_handler(mut ctx: Context) -> ContextResult {
    ///     let result: serde_json::Value = ctx.json().await?;
    ///
    ///     assert_eq!(result["id"], 1);
    ///     assert_eq!(result["mode"], "edit".to_string());
    ///
    ///     ctx.build("").ok()
    /// }
    /// ```
    pub async fn json<T: DeserializeOwned>(&mut self) -> Result<T, ObsidianError> {
        let body = self.take_body();

        let buf = match body::aggregate(body).await {
            Ok(buf) => buf,
            _ => {
                return Err(ObsidianError::NoneError);
            }
        };

        Ok(serde_json::from_slice(buf.bytes())?)
    }

    /// Json value merged with Params
    pub fn json_with_param<T: DeserializeOwned>(&mut self) -> Result<T, ObsidianError> {
        unimplemented!()
    }

    pub fn cookie(&self, name: &str) -> Option<&Cookie> {
        if let Some(cookie_data) = self.get::<CookieParserData>() {
            return cookie_data.cookie_jar().get(name);
        }

        None
    }

    pub fn session(&self) -> Option<&SessionData> {
        self.get::<SessionData>()
    }

    pub fn session_mut(&mut self) -> Option<&mut SessionData> {
        self.get_mut::<SessionData>()
    }

    pub fn session_set(&mut self, name: &str, value: &str) {
        match self.get_mut::<SessionData>() {
            Some(session) => {
                session.set(name, value);
            }
            _ => {
                let mut session = SessionData::new();
                session.set(name, value);
                self.add(session);
            }
        }
    }

    /// Consumes body of the request and replace it with empty body.
    pub fn take_body(&mut self) -> Body {
        std::mem::replace(self.request.body_mut(), Body::empty())
    }

    /// Consumes response
    pub fn take_response(&mut self) -> Option<Response> {
        std::mem::replace(&mut self.response, None)
    }

    pub fn response(&self) -> &Option<Response> {
        &self.response
    }

    pub fn response_mut(&mut self) -> &mut Option<Response> {
        &mut self.response
    }

    /// Build any kind of response which implemented Responder trait
    pub fn build(self, res: impl Responder) -> ResponseBuilder {
        ResponseBuilder::new(self, res.respond_to())
    }

    /// Build data into json format. The data must implement Serialize trait
    pub fn build_json(self, body: impl Serialize) -> ResponseBuilder {
        ResponseBuilder::new(self, Response::ok().json(body))
    }

    /// Build response from static file.
    pub async fn build_file(self, file_path: &str) -> ResponseBuilder {
        ResponseBuilder::new(self, Response::ok().file(file_path).await)
    }

    fn parse_queries<T: DeserializeOwned>(query: &[u8]) -> Result<T, ObsidianError> {
        let mut parsed_form_map: HashMap<String, Vec<String>> = HashMap::default();
        let mut cow_form_map = HashMap::<Cow<str>, Cow<[String]>>::default();

        // Parse and merge chunks with same name key
        form_urlencoded::parse(query)
            .into_owned()
            .for_each(|(key, val)| {
                if !val.is_empty() {
                    parsed_form_map
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .push(val);
                }
            });

        // Wrap vec with cow pointer
        parsed_form_map.iter().for_each(|(key, val)| {
            cow_form_map
                .entry(std::borrow::Cow::from(key))
                .or_insert_with(|| std::borrow::Cow::from(val));
        });

        Ok(from_cow_map(&cow_form_map)?)
    }
}

pub struct ResponseBuilder {
    ctx: Context,
    response: Response,
}

impl ResponseBuilder {
    pub fn new(ctx: Context, response: Response) -> Self {
        ResponseBuilder { ctx, response }
    }

    /// set http status code for response
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.response = self.response.set_status(status);
        self
    }

    /// set http header for response
    pub fn with_header(mut self, key: HeaderName, value: &'static str) -> Self {
        self.response = self.response.set_header(key, value);
        self
    }

    /// set custom http header for response with `&str` key
    pub fn with_header_str(mut self, key: &'static str, value: &'static str) -> Self {
        self.response = self.response.set_header_str(key, value);
        self
    }

    pub fn with_headers(mut self, headers: &[(HeaderName, &'static str)]) -> Self {
        self.response = self.response.set_headers(headers);
        self
    }

    pub fn with_headers_str(mut self, headers: &[(&'static str, &'static str)]) -> Self {
        self.response = self.response.set_headers_str(headers);
        self
    }

    pub fn with_cookie(mut self, cookie: Cookie<'static>) -> Self {
        self.response = self.response.set_cookie(cookie);
        self
    }

    pub fn with_cookies(mut self, cookies: &[Cookie<'static>]) -> Self {
        self.response = self.response.set_cookies(cookies);
        self
    }

    pub fn ok(mut self) -> ContextResult {
        *self.ctx.response_mut() = Some(self.response);
        Ok(self.ctx)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use async_std::task;
    use hyper::{Body, Request};
    use serde::*;
    use serde_json::json;

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct FormResult {
        id: i32,
        mode: String,
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct FormExtraResult {
        id: i32,
        mode: String,
        #[serde(default)]
        extra: i32,
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct JsonResult {
        id: i32,
        mode: String,
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct JsonExtraResult {
        id: i32,
        mode: String,
        #[serde(default)]
        extra: i32,
    }

    #[test]
    fn test_params() -> Result<(), ObsidianError> {
        let mut params_map = HashMap::default();

        params_map.insert("id".to_string(), "1".to_string());
        params_map.insert("mode".to_string(), "edit".to_string());

        let request = Request::new(Body::from(""));

        let ctx = Context::new(request, params_map);

        let id: i32 = ctx.param("id")?;
        let mode: String = ctx.param("mode")?;

        assert_eq!(id, 1);
        assert_eq!(mode, "edit".to_string());

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_params_without_value() {
        let mut params_map = HashMap::default();

        params_map.insert("mode".to_string(), "edit".to_string());

        let request = Request::new(Body::from(""));

        let ctx = Context::new(request, params_map);

        let _mode: String = ctx.param("mode").unwrap();
        let _id: i32 = ctx.param("id").unwrap();
    }

    #[test]
    fn test_string_query() -> Result<(), ObsidianError> {
        let params_map = HashMap::default();

        let mut request = Request::new(Body::from(""));
        *request.uri_mut() = Uri::from_str("/test/test?id=1&mode=edit").unwrap();

        let mut ctx = Context::new(request, params_map);

        let actual_result: FormResult = ctx.query_params()?;
        let expected_result = FormResult {
            id: 1,
            mode: "edit".to_string(),
        };

        assert_eq!(actual_result, expected_result);
        Ok(())
    }

    #[test]
    fn test_form() -> Result<(), ObsidianError> {
        task::block_on(async {
            let params = HashMap::default();
            let request = Request::new(Body::from("id=1&mode=edit"));

            let mut ctx = Context::new(request, params);

            let actual_result: FormResult = ctx.form().await?;
            let expected_result = FormResult {
                id: 1,
                mode: "edit".to_string(),
            };

            assert_eq!(actual_result, expected_result);
            Ok(())
        })
    }

    #[test]
    fn test_form_with_extra_body() -> Result<(), ObsidianError> {
        task::block_on(async {
            let params = HashMap::default();
            let request = Request::new(Body::from("id=1&mode=edit&extra=true"));

            let mut ctx = Context::new(request, params);

            let actual_result: FormResult = ctx.form().await?;
            let expected_result = FormResult {
                id: 1,
                mode: "edit".to_string(),
            };

            assert_eq!(actual_result, expected_result);
            Ok(())
        })
    }

    #[test]
    fn test_form_with_extra_field() -> Result<(), ObsidianError> {
        task::block_on(async {
            let params = HashMap::default();
            let request = Request::new(Body::from("id=1&mode=edit"));

            let mut ctx = Context::new(request, params);

            let actual_result: FormExtraResult = ctx.form().await?;
            let expected_result = FormExtraResult {
                id: 1,
                mode: "edit".to_string(),
                extra: i32::default(),
            };

            assert_eq!(actual_result, expected_result);
            Ok(())
        })
    }

    #[test]
    fn test_json_struct() -> Result<(), ObsidianError> {
        task::block_on(async {
            let params = HashMap::default();
            let request = Request::new(Body::from("{\"id\":1,\"mode\":\"edit\"}"));

            let mut ctx = Context::new(request, params);

            let actual_result: JsonResult = ctx.json().await?;
            let expected_result = JsonResult {
                id: 1,
                mode: "edit".to_string(),
            };

            assert_eq!(actual_result, expected_result);
            Ok(())
        })
    }

    #[test]
    fn test_json_value() -> Result<(), ObsidianError> {
        task::block_on(async {
            let params = HashMap::default();
            let request = Request::new(Body::from("{\"id\":1,\"mode\":\"edit\"}"));

            let mut ctx = Context::new(request, params);

            let actual_result: serde_json::Value = ctx.json().await?;

            assert_eq!(actual_result["id"], json!(1));
            assert_eq!(actual_result["mode"], json!("edit"));
            Ok(())
        })
    }
}
