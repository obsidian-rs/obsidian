use std::sync::Arc;

use super::EndPointHandler;
use crate::Method;

pub struct Route {
    pub method: Method,
    pub handler: Arc<dyn EndPointHandler>,
}

impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Route {{ method: {} }}", self.method)
    }
}

impl Clone for Route {
    fn clone(&self) -> Route {
        Route {
            method: self.method.clone(),
            handler: self.handler.clone(),
        }
    }
}

impl Route {
    pub fn new(method: Method, handler: impl EndPointHandler) -> Self {
        Route {
            method,
            handler: Arc::new(handler),
        }
    }
}
