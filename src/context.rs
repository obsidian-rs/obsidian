use futures::{Future, Stream};
use hyper::{header::HeaderValue, Body, HeaderMap, Method, Request, Uri};
use serde::de::DeserializeOwned;
use url::form_urlencoded;

use crate::router::{FromParam, Params};

pub struct Context {
    request: Request<Body>,
    params_data: Params,
}

impl Context {
    pub fn new(request: Request<Body>, params_data: Params) -> Self {
        Context {
            request,
            params_data,
        }
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.request.headers()
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        self.request.headers_mut()
    }

    pub fn method(&self) -> &Method {
        self.request.method()
    }

    pub fn uri(&self) -> &Uri {
        self.request.uri()
    }

    pub fn param<T: FromParam>(&mut self, key: &str) -> Result<T, T::Err> {
        if self.params_data.is_empty() {
            self.parse_url_encode()
        }

        FromParam::from_params(&self.params_data, key)
    }

    pub fn json<T: DeserializeOwned>(&mut self) -> Result<T, serde_json::error::Error> {
        let body = self.take_body();

        let chunks = match body.concat2().wait() {
            Ok(chunk) => chunk,
            Err(e) => {
                println!("{}", e);
                hyper::Chunk::default()
            }
        };

        Ok(serde_json::from_slice(&chunks)?)
    }

    pub fn take_body(&mut self) -> Body {
        std::mem::replace(self.request.body_mut(), Body::empty())
    }

    fn parse_url_encode(&mut self) {
        let body = self.take_body();

        let chunks = match body.concat2().wait() {
            Ok(chunk) => chunk,
            Err(e) => {
                println!("{}", e);
                hyper::Chunk::default()
            }
        };

        let params_iter = form_urlencoded::parse(chunks.as_ref()).into_owned();

        for (key, value) in params_iter {
            self.params_data.add_params(key, value);
        }
    }
}
