use hyper::{Body, Request};
use serde_json::{json, Value};

use crate::router::{Params, FromParam};

pub struct Context {
    pub json: Value,
    pub request: Request<Body>,
    pub params_data: Params,
}

impl Context {
    pub fn new(request: Request<Body>, params_data: Params) -> Self {
        Context {
            request,
            params_data,
            json: json!(null),
        }
    }

    pub fn param<T: FromParam>(&self, key: &str) -> Result<T, T::Err> {
        FromParam::from_params(&self.params_data, key)
    }
}
