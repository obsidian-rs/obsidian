use std::collections::HashMap;

#[derive(Clone)]
pub struct RouteData {
    params: HashMap<String, Vec<String>>,
}

impl RouteData {
    pub fn new() -> Self {
        RouteData {
            params: HashMap::new(),
        }
    }

    pub fn add(mut self, key: String, value: String) {
        (*self.params.entry(key).or_insert(Vec::new())).push(value);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.params.get(key)
    }

    pub fn get_params(self) -> HashMap<String, Vec<String>> {
        self.params
    }
}

impl From<HashMap<String, Vec<String>>> for RouteData {
    fn from(params: HashMap<String, Vec<String>>) -> Self {
        RouteData { params }
    }
}
