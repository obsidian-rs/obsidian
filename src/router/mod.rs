mod route;
mod injection;
mod resource;
mod end_point_type;

use std::collections::BTreeMap;
use hyper::Method;

use self::route::Route;
use self::injection::Injection;
use self::resource::Resource;
use self::end_point_type::EndPointHandler;

pub struct Router {
    pub routes: BTreeMap<String, Resource>,
} 

impl Injection for Router {
    fn get(&mut self, path: &str, handler: impl EndPointHandler) {
        self.inject(Method::GET, path, handler);
    }

    fn post(&mut self, path: &str, handler: impl EndPointHandler) {
        self.inject(Method::POST, path, handler);
    }

    fn put(&mut self, path: &str, handler: impl EndPointHandler) {
        self.inject(Method::PUT, path, handler);
    }

    fn delete(&mut self, path: &str, handler: impl EndPointHandler) {
        self.inject(Method::DELETE, path, handler);
    }
}

impl Router {
    fn new() -> Self {
        Router {routes: BTreeMap::new()}
    }

    fn inject(&mut self, method: Method, path: &str, handler: impl EndPointHandler) {
        // Use existing hashmap
        if let Some(route) = self.routes.get_mut(&path.to_string()) {
            route.add_route(method.clone(), Route::new(path.to_string(), method.clone(), Box::new(handler)));
        }
        else {
            let mut resource = Resource::default();
            resource.add_route(method.clone(), Route::new(path.to_string(), method.clone(), Box::new(handler)));

            self.routes.insert(path.to_string(), resource);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_get() {
        let mut router = Router::new();

        router.get("/", |req, res| {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get_route(&Method::GET).unwrap();

        assert_eq!(route.path, "/");
    }

    #[test]
    fn test_router_post() {
        let mut router = Router::new();

        router.post("/", |req, res| {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get_route(&Method::POST).unwrap();

        assert_eq!(route.path, "/");
    }

    #[test]
    fn test_router_put() {
        let mut router = Router::new();

        router.put("/", |req, res| {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get_route(&Method::PUT).unwrap();

        assert_eq!(route.path, "/");
    }

    #[test]
    fn test_router_delete() {
        let mut router = Router::new();

        router.delete("/", |req, res| {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get_route(&Method::DELETE).unwrap();

        assert_eq!(route.path, "/");
    }
}