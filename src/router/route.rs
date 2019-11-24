use hyper::Method;
use std::sync::Arc;

use super::EndPointHandler;

pub struct Route {
    pub path: String,
    pub method: Method,
    pub handler: Arc<dyn EndPointHandler>,
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
