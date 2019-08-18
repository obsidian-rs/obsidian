use std::collections::HashMap;

#[derive(Default)]
pub struct Params {
    params_map: HashMap<String, String>,
}

impl Params {
    pub fn new(params_map: HashMap<String, String>) -> Self {
        Params { params_map }
    }
    pub fn get_params(&self, key: &str) -> Option<&String> {
        self.params_map.get(key)
    }

    pub fn add_params(&mut self, key: String, val: String) {
        self.params_map.insert(key, val);
    }
}
