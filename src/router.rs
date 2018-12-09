mod end_point_type;
mod resource;
mod response;
mod route;

use hyper::Method;
use std::collections::BTreeMap;

pub use self::end_point_type::EndPointHandler;
pub use self::resource::Resource;
pub use self::response::ObsidianResponse;
pub use self::route::Route;

pub struct Router {
    pub routes: BTreeMap<String, Resource>,
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
            routes: BTreeMap::new(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use http::StatusCode;
    use hyper::{Body, Request, Response};

    #[test]
    fn test_router_get() {
        let mut router = Router::new();

        router.get("/", |_req, res: ObsidianResponse| {
            res.status(StatusCode::OK).body("test_get")
        });

        let route = router
            .routes
            .get_mut("/")
            .unwrap()
            .get_route(&Method::GET)
            .unwrap();

        let res = ObsidianResponse::new();
        let req = Request::new(Body::from(""));

        let mut expected_response = Response::new(Body::from("test_get"));
        *expected_response.status_mut() = StatusCode::OK;

        let actual_response = (route.handler)(req, res);
        let actual_response: Response<Body> = actual_response.into();

        assert_eq!(route.path, "/");
        assert_eq!(actual_response.status(), expected_response.status());
    }

    #[test]
    fn test_router_post() {
        let mut router = Router::new();

        router.post("/", |_req, res: ObsidianResponse| {
            res.status(StatusCode::OK).body("test_post")
        });

        let route = router
            .routes
            .get_mut("/")
            .unwrap()
            .get_route(&Method::POST)
            .unwrap();

        let res = ObsidianResponse::new();
        let req = Request::new(Body::from(""));

        let mut expected_response = Response::new(Body::from("test_post"));
        *expected_response.status_mut() = StatusCode::OK;

        let actual_response = (route.handler)(req, res);
        let actual_response: Response<Body> = actual_response.into();

        assert_eq!(route.path, "/");
        assert_eq!(actual_response.status(), expected_response.status());
    }

    #[test]
    fn test_router_put() {
        let mut router = Router::new();

        router.put("/", |_req, res: ObsidianResponse| {
            res.status(StatusCode::OK).body("test_put")
        });

        let route = router
            .routes
            .get_mut("/")
            .unwrap()
            .get_route(&Method::PUT)
            .unwrap();

        let res = ObsidianResponse::new();
        let req = Request::new(Body::from(""));

        let mut expected_response = Response::new(Body::from("test_put"));
        *expected_response.status_mut() = StatusCode::OK;

        let actual_response = (route.handler)(req, res);
        let actual_response: Response<Body> = actual_response.into();

        assert_eq!(route.path, "/");
        assert_eq!(actual_response.status(), expected_response.status());
    }

    #[test]
    fn test_router_delete() {
        let mut router = Router::new();

        router.delete("/", |_req, res: ObsidianResponse| {
            res.status(StatusCode::OK).body("test_delete")
        });

        let route = router
            .routes
            .get_mut("/")
            .unwrap()
            .get_route(&Method::DELETE)
            .unwrap();

        let res = ObsidianResponse::new();
        let req = Request::new(Body::from(""));

        let mut expected_response = Response::new(Body::from("test_delete"));
        *expected_response.status_mut() = StatusCode::OK;

        let actual_response = (route.handler)(req, res);
        let actual_response: Response<Body> = actual_response.into();

        assert_eq!(route.path, "/");
        assert_eq!(actual_response.status(), expected_response.status());
    }
}
