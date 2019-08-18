use futures::{Future, Stream};
use hyper::{header::HeaderValue, Body, HeaderMap, Method, Request, Uri};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::str::FromStr;
use url::form_urlencoded;

use crate::router::Params;

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

    pub fn param<T: FromStr>(&self, key: &str) -> Result<T, T::Err> {
        self.params_data.get_params(key).unwrap().parse()
    }

    pub fn form<T: DeserializeOwned>(&mut self) -> Result<T, serde_json::error::Error> {
        let body = self.take_body();

        let chunks = match body.concat2().wait() {
            Ok(chunk) => chunk,
            Err(e) => {
                println!("{}", e);
                hyper::Chunk::default()
            }
        };

        // Temporary use serde_json for deserialization as serde_urlencoded does not support same name field yet.
        // For type other than collection string and string, deserialize_with need to be implement
        let mut temp_map: HashMap<String, Vec<serde_json::Value>> = HashMap::default();
        let mut json_map: HashMap<String, serde_json::Value> = HashMap::default();

        form_urlencoded::parse(&chunks)
            .into_owned()
            .for_each(|(key, val)| {
                temp_map
                    .entry(key)
                    .or_insert(vec![])
                    .push(serde_json::Value::String(val));
            });

        temp_map.iter().for_each(|(key, val)| {
            match val.len() > 1 {
                true => json_map.insert(key.to_string(), serde_json::Value::Array(val.to_owned())),
                false => json_map.insert(
                    key.to_string(),
                    serde_json::Value::String(val.first().unwrap().to_string()),
                ),
            };
        });

        let json_string = serde_json::to_string(&json_map)?;

        Ok(serde_json::from_str(&json_string)?)
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
}
