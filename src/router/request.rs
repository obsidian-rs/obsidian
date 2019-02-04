use super::route_data::RouteData;
use hyper::{Body, Request};
use serde_json::Value;
use std::collections::HashMap;

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

pub struct RequestData {
    pub request: Request<Body>,
    pub params_data: HashMap<String, Vec<String>>,
    pub json: Value,
}

impl RequestData {
    pub fn new(request: Request<Body>, route_data: RouteData) -> Self {
        let (params_data, json) = route_data.get_route_data();

        RequestData {
            request,
            params_data,
            json,
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
