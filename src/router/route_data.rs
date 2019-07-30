use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Clone)]
pub struct RouteData {
    params: HashMap<String, Vec<String>>,
    json: Value,
}

impl RouteData {
    pub fn new() -> Self {
        RouteData {
            params: HashMap::new(),
            json: json!(null),
        }
    }

    pub fn add_param(&mut self, key: String, value: String) {
        (*self.params.entry(key).or_insert(Vec::new())).push(value);
    }

    pub fn add_json(&mut self, json: Value) {
        self.json = json;
    }

    pub fn get_param(&self, key: &str) -> Option<&Vec<String>> {
        self.params.get(key)
    }

    pub fn get_params(&self) -> &HashMap<String, Vec<String>> {
        &self.params
    }

    pub fn get_json(&self) -> &Value {
        &self.json
    }

    pub fn get_route_data(self) -> (HashMap<String, Vec<String>>, Value) {
        (self.params, self.json)
    }
}

impl From<HashMap<String, Vec<String>>> for RouteData {
    fn from(params: HashMap<String, Vec<String>>) -> Self {
        RouteData {
            params,
            json: json!(null),
        }
    }
}

impl From<Value> for RouteData {
    fn from(json: Value) -> Self {
        RouteData {
            params: HashMap::new(),
            json,
        }
    }
}

impl From<(HashMap<String, Vec<String>>, Value)> for RouteData {
    fn from((params, json): (HashMap<String, Vec<String>>, Value)) -> Self {
        RouteData { params, json }
    }
}
