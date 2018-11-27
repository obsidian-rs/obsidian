use hyper::{Method, Response};

use super::EndPointHandler;

pub struct Route {
    pub path: String,
    pub method: Method,
    pub handler: Box<dyn EndPointHandler<Output=Response<()>>>,
}

impl Route {
    pub fn new(path: String, method: Method, handler: impl EndPointHandler) -> Self {
        Route {path, method, handler: Box::new(handler)}
    }
}
