mod route;
mod injection;
mod resource;

use std::collections::BTreeMap;
use std::collections::HashMap;

use self::route::Route;
use self::route::Method;
use self::injection::Injection;
use self::resource::Resource;

pub struct Router {
    pub routes: BTreeMap<String, Resource>,
} 

impl Injection for Router {
    fn get(&mut self, path: &str, handler: impl Fn() + Send + 'static) {
        self.inject(Method::GET, path, handler);
    }

    fn post(&mut self, path: &str, handler: impl Fn() + Send + 'static) {
        self.inject(Method::POST, path, handler);
    }

    fn put(&mut self, path: &str, handler: impl Fn() + Send + 'static) {
        self.inject(Method::PUT, path, handler);
    }

    fn delete(&mut self, path: &str, handler: impl Fn() + Send + 'static) {
        self.inject(Method::DELETE, path, handler);
    }
}

impl Router {
    fn new() -> Self {
        Router {routes: BTreeMap::new()}
    }

    fn inject(&mut self, method: Method, path: &str, handler: impl Fn() + Send + 'static) {
        // Use existing hashmap
        if let Some(route) = self.routes.get_mut(&path.to_string()) {
            route.add_route(method, Route::new(path.to_string(), method, Box::new(handler)));
        }
        else {
            let mut resource = Resource::default();
            resource.add_route(method, Route::new(path.to_string(), method, Box::new(handler)));

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

        router.get("/", || {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get_route(&Method::GET).unwrap();

        assert_eq!(route.path, "/");
    }

    #[test]
    fn test_router_post() {
        let mut router = Router::new();

        router.post("/", || {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get_route(&Method::POST).unwrap();

        assert_eq!(route.path, "/");
    }

    #[test]
    fn test_router_put() {
        let mut router = Router::new();

        router.put("/", || {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get_route(&Method::PUT).unwrap();

        assert_eq!(route.path, "/");
    }

    #[test]
    fn test_router_delete() {
        let mut router = Router::new();

        router.delete("/", || {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get_route(&Method::DELETE).unwrap();

        assert_eq!(route.path, "/");
    }
}