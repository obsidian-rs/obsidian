use hyper::Method;
use std::collections::HashMap;

use super::Route;

/// Resource acts as the intermidiate interface for interaction of routing data structure
/// Resource is binding with the path and handling all of the request method for that path
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
    pub fn add_route(&mut self, method: Method, route: Route) -> Option<Route> {
        self.route_map.insert(method, route)
    }

    pub fn get_route(&self, method: &Method) -> Option<&Route> {
        self.route_map.get(&method)
    }
}
