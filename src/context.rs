use futures::{Future, Stream};
use hyper::{header::HeaderValue, Body, HeaderMap, Method, Request, Uri};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::str::FromStr;
use url::form_urlencoded;

use crate::router::from_cow_form;
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

    pub fn form<T: DeserializeOwned>(&mut self) -> Result<T, ()> {
        let body = self.take_body();

        let chunks = match body.concat2().wait() {
            Ok(chunk) => chunk,
            Err(e) => {
                println!("{}", e);
                hyper::Chunk::default()
            }
        };

        let mut parsed_form_map: HashMap<String, Vec<String>> = HashMap::default();
        let mut cow_form_map = HashMap::default();

        // Parse and merge chunks with same name key
        form_urlencoded::parse(&chunks)
            .into_owned()
            .for_each(|(key, val)| {
                parsed_form_map.entry(key).or_insert(vec![]).push(val);
            });

        // Wrap vec with cow pointer
        parsed_form_map.iter().for_each(|(key, val)| {
            cow_form_map
                .entry(std::borrow::Cow::from(key))
                .or_insert(std::borrow::Cow::from(val));
        });

        Ok(from_cow_form(&cow_form_map).unwrap())
    }

    // Form value with Params
    pub fn form_wparam<T: DeserializeOwned>(&mut self) -> Result<T, ()> {
        unimplemented!()
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

    // Json value with Params
    pub fn json_wparam<T: DeserializeOwned>(&mut self) -> Result<T, serde_json::error::Error> {
        unimplemented!()
    }

    pub fn take_body(&mut self) -> Body {
        std::mem::replace(self.request.body_mut(), Body::empty())
    }
}
