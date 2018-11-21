mod route;
mod injection;

use std::collections::BTreeMap;
use std::collections::HashMap;

use self::route::Route;
use self::route::Method;
use self::injection::Injection;

pub struct Router<'a> {
    pub routes: BTreeMap<String, HashMap<Method, Route<'a>>>,
} 

impl <'a> Injection<'a> for Router<'a> {
    fn new() -> Self {
        Router {routes: BTreeMap::new()}
    }

    fn get(&mut self, path: &str, handler: impl Fn() + 'a) {
        self.inject(Method::GET, path, handler);
    }

    fn post(&mut self, path: &str, handler: impl Fn() + 'a) {
        self.inject(Method::POST, path, handler);
    }

    fn put(&mut self, path: &str, handler: impl Fn() + 'a) {
        self.inject(Method::PUT, path, handler);
    }

    fn delete(&mut self, path: &str, handler: impl Fn() + 'a) {
        self.inject(Method::DELETE, path, handler);
    }
}

impl <'a> Router <'a> {
    fn inject(&mut self, method: Method, path: &str, handler: impl Fn() + 'a) {
        if !self.routes.contains_key(&path.to_string()) {
            // Construct new hashmap
            let mut new_routes_map: HashMap<Method, Route<'a>> = HashMap::new();
            new_routes_map.insert(method, Route::new(path.to_string(), Box::new(handler)));

            self.routes.insert(path.to_string(), new_routes_map);
        }
        else {
            // Use existing hashmap
            if let Some(route) = self.routes.get_mut(&path.to_string()) {
                route.insert(method, Route::new(path.to_string(), Box::new(handler)));
            }
            else {
                panic!("Hash Map condition error");
            }
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
            .get(&Method::GET).unwrap();

        assert_eq!(route.path, "/");
    }

    #[test]
    fn test_router_post() {
        let mut router = Router::new();

        router.post("/", || {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get(&Method::POST).unwrap();

        assert_eq!(route.path, "/");
    }

    #[test]
    fn test_router_put() {
        let mut router = Router::new();

        router.put("/", || {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get(&Method::PUT).unwrap();

        assert_eq!(route.path, "/");
    }

    #[test]
    fn test_router_delete() {
        let mut router = Router::new();

        router.delete("/", || {});

        let route = router.routes
            .get_mut("/").unwrap()
            .get(&Method::DELETE).unwrap();

        assert_eq!(route.path, "/");
    }
}