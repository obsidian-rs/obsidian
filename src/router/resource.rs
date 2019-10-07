use hyper::Method;
use std::collections::HashMap;

use super::Route;

/// Resource acts as the intermidiate interface for interaction of routing data structure
#[derive(Clone, Debug)]
pub struct Resource {
    route_map: HashMap<Method, Route>,
}

impl Default for Resource {
    fn default() -> Self {
        Resource {
            route_map: HashMap::new(),
        }
    }
}

impl Resource {
    pub fn add_route(&mut self, method: Method, route: Route) {
        if self.route_map.contains_key(&method) {
            panic!("Route: {} Method: {} error", route.path, route.method);
        }

        self.route_map.insert(method, route);
    }

    pub fn get_route(&self, method: &Method) -> Option<&Route> {
        self.route_map.get(&method)
    }
}
