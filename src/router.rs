mod end_point_type;
mod req_deserializer;
mod request;
mod resource;
mod response;
mod route;
mod trie;

use hyper::Method;

use self::trie::{Trie, RouteValueResult};
use crate::middleware::Middleware;
use crate::context::ObsidianError;

pub use self::end_point_type::EndPointHandler;
pub use self::req_deserializer::{from_cow_map, Error as FormError};
pub use self::request::Params;
pub use self::resource::Resource;
pub use self::response::ResponseBuilder;
pub use self::route::Route;

pub struct Router {
    routes: Trie,
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

    pub fn use_service(&mut self, middleware: impl Middleware) {
        self.routes.insert_default_middleware(middleware);
    }

    pub fn use_service_to(&mut self, path: &str, middleware: impl Middleware) {
        self.routes.insert_middleware(path, middleware);
    }

    pub fn search_route(&self, path: &str) -> Result<RouteValueResult, ObsidianError> {
        self.routes.search_route(path)
    }

    pub fn merge_router(&mut self, path: &str, other: Router) {
        self.routes.insert_sub_trie(path, other.routes);
    }

    fn inject(&mut self, method: Method, path: &str, handler: impl EndPointHandler) {
        let route = Route::new(path.to_string(), method.clone(), handler);

        self.routes.insert_route(path, route);
    }
}
