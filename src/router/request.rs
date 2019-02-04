use super::route_data::RouteData;
use hyper::{Body, Request};

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
    pub params_data: RouteData,
}

impl RequestData {
    pub fn new(request: Request<Body>, params_data: RouteData) -> Self {
        RequestData {
            request,
            params_data,
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
