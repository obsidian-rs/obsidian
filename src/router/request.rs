use hyper::{Request, Body};
use std::collections::HashMap;

pub struct RequestData {
    pub request: Request<Body>,
    pub params: HashMap<String, String>,
}

impl RequestData {
    pub fn new(request: Request<Body>, params: HashMap<String, String>) -> Self {
        RequestData { request, params }
    }
}
