use hyper::{Request, Body};
use std::collections::HashMap;

pub struct RequestData {
    pub request: Request<Body>,
    pub params: HashMap<String, Vec<String>>,
}

impl RequestData {
    pub fn new(request: Request<Body>, params: HashMap<String, Vec<String>>) -> Self {
        RequestData { request, params }
    }
}
