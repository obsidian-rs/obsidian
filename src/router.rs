mod end_point_type;
mod req_deserializer;
mod request;
mod resource;
mod response;
mod route;

use hyper::Method;

use crate::middleware::Middleware;

pub use self::end_point_type::EndPointHandler;
pub use self::req_deserializer::{from_cow_map, Error as FormError};
pub use self::request::Params;
pub use self::resource::Resource;
pub use self::response::ResponseBuilder;
pub use self::route::Route;
use crate::trie::Trie;

pub struct Router {
    pub routes: Trie,
}

impl Clone for Router {
    fn clone(&self) -> Self {
        Router {
            routes: self.routes.clone(),
        }
    }
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Trie::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: impl EndPointHandler) {
        self.inject(Method::GET, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: impl EndPointHandler) {
        self.inject(Method::POST, path, handler);
    }

    pub fn put(&mut self, path: &str, handler: impl EndPointHandler) {
        self.inject(Method::PUT, path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: impl EndPointHandler) {
        self.inject(Method::DELETE, path, handler);
    }

    pub fn add_service(&mut self, middleware: impl Middleware) {
        self.routes.insert_default_middleware(middleware);
    }

    pub fn add_service_to(&mut self, path: &str, middleware: impl Middleware) {
        self.routes.insert_middleware(path, middleware);
    }

    pub fn inject(&mut self, method: Method, path: &str, handler: impl EndPointHandler) {
        let route = Route::new(path.to_string(), method.clone(), handler);

        self.routes.insert_route(path, route);
    }
}
