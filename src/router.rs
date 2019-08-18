mod end_point_type;
mod request;
mod resource;
mod response;
mod route;
mod route_data;

use hyper::Method;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::middleware::Middleware;

pub use self::end_point_type::EndPointHandler;
pub use self::request::Params;
pub use self::resource::Resource;
pub use self::response::ResponseBuilder;
pub use self::route::Route;
pub use self::route_data::RouteData;

pub struct Router {
    pub routes: BTreeMap<String, Resource>,
    pub middlewares: Vec<Arc<dyn Middleware>>,
}

impl Clone for Router {
    fn clone(&self) -> Self {
        Router {
            routes: self.routes.clone(),
            middlewares: self.middlewares.clone(),
        }
    }
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: BTreeMap::new(),
            middlewares: Vec::new(),
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
        self.middlewares.push(Arc::new(middleware));
    }

    pub fn inject(&mut self, method: Method, path: &str, handler: impl EndPointHandler) {
        // Use existing hashmap
        (*self
            .routes
            .entry(path.to_string())
            .or_insert(Resource::default()))
        .add_route(
            method.clone(),
            Route::new(path.to_string(), method.clone(), handler),
        );
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use http::StatusCode;
    use hyper::{Body, Request, Response};
    use std::collections::HashMap;

    #[test]
    fn test_router_get() {
        let mut router = Router::new();

        router.get("/", |_req, res: ResponseBuilder| {
            res.status(StatusCode::OK).body("test_get")
        });

        let route = router
            .routes
            .get_mut("/")
            .unwrap()
            .get_route(&Method::GET)
            .unwrap();

        let res = ResponseBuilder::new();
        let req = Request::new(Body::from(""));
        let params = HashMap::new();
        let request_data = RequestData::new(req, RouteData::from(params));

        let mut expected_response = Response::new(Body::from("test_get"));
        *expected_response.status_mut() = StatusCode::OK;

        let actual_response = (route.handler)(request_data, res);
        let actual_response: Response<Body> = actual_response.into();

        assert_eq!(route.path, "/");
        assert_eq!(actual_response.status(), expected_response.status());
    }

    #[test]
    fn test_router_post() {
        let mut router = Router::new();

        router.post("/", |_req, res: ResponseBuilder| {
            res.status(StatusCode::OK).body("test_post")
        });

        let route = router
            .routes
            .get_mut("/")
            .unwrap()
            .get_route(&Method::POST)
            .unwrap();

        let res = ResponseBuilder::new();
        let req = Request::new(Body::from(""));
        let params = HashMap::new();
        let request_data = RequestData::new(req, RouteData::from(params));

        let mut expected_response = Response::new(Body::from("test_post"));
        *expected_response.status_mut() = StatusCode::OK;

        let actual_response = (route.handler)(request_data, res);
        let actual_response: Response<Body> = actual_response.into();

        assert_eq!(route.path, "/");
        assert_eq!(actual_response.status(), expected_response.status());
    }

    #[test]
    fn test_router_put() {
        let mut router = Router::new();

        router.put("/", |_req, res: ResponseBuilder| {
            res.status(StatusCode::OK).body("test_put")
        });

        let route = router
            .routes
            .get_mut("/")
            .unwrap()
            .get_route(&Method::PUT)
            .unwrap();

        let res = ResponseBuilder::new();
        let req = Request::new(Body::from(""));
        let params = HashMap::new();
        let request_data = RequestData::new(req, RouteData::from(params));

        let mut expected_response = Response::new(Body::from("test_put"));
        *expected_response.status_mut() = StatusCode::OK;

        let actual_response = (route.handler)(request_data, res);
        let actual_response: Response<Body> = actual_response.into();

        assert_eq!(route.path, "/");
        assert_eq!(actual_response.status(), expected_response.status());
    }

    #[test]
    fn test_router_delete() {
        let mut router = Router::new();

        router.delete("/", |_req, res: ResponseBuilder| {
            res.status(StatusCode::OK).body("test_delete")
        });

        let route = router
            .routes
            .get_mut("/")
            .unwrap()
            .get_route(&Method::DELETE)
            .unwrap();

        let res = ResponseBuilder::new();
        let req = Request::new(Body::from(""));
        let params = HashMap::new();
        let request_data = RequestData::new(req, RouteData::from(params));

        let mut expected_response = Response::new(Body::from("test_delete"));
        *expected_response.status_mut() = StatusCode::OK;

        let actual_response = (route.handler)(request_data, res);
        let actual_response: Response<Body> = actual_response.into();

        assert_eq!(route.path, "/");
        assert_eq!(actual_response.status(), expected_response.status());
    }
}
 */
