use std::collections::HashMap;

use hyper::{Body, Request};
use serde_json::{json, Value};

use crate::router::RouteData;

pub struct Context {
    pub request: Request<Body>,
    pub route_data: RouteData,
    pub params_data: HashMap<String, Vec<String>>, // going to optimize into one type later
    pub json: Value,
}

impl Context {
    pub fn new(request: Request<Body>, route_data: RouteData, params_data: HashMap<String, Vec<String>>) -> Self {
        Context {
            request,
            route_data,
            params_data,
            json: json!(null),
        }
    }

    pub fn params(&self, key: &str) -> ParamsBox {
        if let Some(params_collection) = self.params_data.get(key) {
            ParamsBox::new(params_collection.clone())
        } else {
            ParamsBox::new(vec!["".to_string()])
        }
    }
}

pub struct ParamsBox {
    params: Vec<String>,
}

impl ParamsBox {
    pub fn new(params: Vec<String>) -> Self {
        ParamsBox { params }
    }
}

impl Into<String> for ParamsBox {
    fn into(self) -> String {
        self.params.first().unwrap().clone()
    }
}

impl Into<Vec<String>> for ParamsBox {
    fn into(self) -> Vec<String> {
        self.params
    }
}
