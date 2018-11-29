use hyper::{Body, Method, Response};

use super::EndPointHandler;
use std::sync::Arc;

pub struct Route {
    pub path: String,
    pub method: Method,
    pub handler: Arc<dyn EndPointHandler<Output = Response<Body>>>,
}

impl Clone for Route {
    fn clone(&self) -> Route {
        Route {
            path: self.path.clone(),
            method: self.method.clone(),
            handler: self.handler.clone(),
        }
    }
}

impl Route {
    pub fn new(path: String, method: Method, handler: impl EndPointHandler) -> Self {
        Route {
            path,
            method,
            handler: Arc::new(handler),
        }
    }
}
