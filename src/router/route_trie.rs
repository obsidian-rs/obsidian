use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use hyper::Method;

use crate::middleware::Middleware;
use crate::router::Resource;
use crate::router::Route;
use crate::ObsidianError;

#[derive(Clone, Default)]
pub struct RouteValue {
    middleware: Vec<Arc<dyn Middleware>>,
    route: Resource,
}

impl fmt::Debug for RouteValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl RouteValue {
    pub fn new(middleware: Vec<Arc<dyn Middleware>>, route: Resource) -> Self {
        RouteValue { middleware, route }
    }
}

pub struct RouteValueResult {
    route_value: RouteValue,
    params: HashMap<String, String>,
}

impl RouteValueResult {
    pub fn new(route_value: RouteValue, params: HashMap<String, String>) -> Self {
        RouteValueResult {
            route_value,
            params,
        }
    }

    pub fn get_route(&self, method: &Method) -> Option<&Route> {
        self.route_value.route.get_route(method)
    }

    pub fn get_middleware(&self) -> &Vec<Arc<dyn Middleware>> {
        &self.route_value.middleware
    }

    pub fn get_params(&self) -> HashMap<String, String> {
        self.params.clone()
    }
}

#[derive(Clone, Debug)]
pub struct RouteTrie {
    head: Node,
}

impl RouteTrie {
    pub fn new() -> Self {
        RouteTrie {
            head: Node::new("/".to_string(), None),
        }
    }

    /// Insert middleware into root node
    pub fn insert_default_middleware(&mut self, middleware: impl Middleware) {
        match &mut self.head.value {
            Some(val) => {
                val.middleware.push(Arc::new(middleware));
            }
            None => {
                let mut val = RouteValue::default();
                val.middleware.push(Arc::new(middleware));

                self.head.value = Some(val);
            }
        }
    }

    /// Insert route values into the trie
    /// Panic if ambigous definition is detected
    pub fn insert_route(&mut self, key: &str, route: Route) {
        // Split key and drop additional '/'
        let split_key = key.split('/');
        let mut split_key = split_key.filter(|key| !key.is_empty()).peekable();

        let mut curr_node = &mut self.head;

        if split_key.peek().is_none() {
            self.insert_default_route(route);
            return;
        }

        while let Some(k) = split_key.next() {
            match *curr_node.process_insertion(k) {
                Some(next_node) => {
                    if split_key.peek().is_none() {
                        match &mut next_node.value {
                            Some(val) => {
                                if let Some(duplicated) =
                                    val.route.add_route(route.method.clone(), route)
                                {
                                    panic!(
                                        "Duplicated route method '{}' at '{}' detected",
                                        duplicated.method, key
                                    );
                                }
                            }
                            None => {
                                let mut next_node_val = RouteValue::default();
                                if let Some(duplicated) =
                                    next_node_val.route.add_route(route.method.clone(), route)
                                {
                                    panic!(
                                        "Duplicated route method '{}' at '{}' detected",
                                        duplicated.method, key
                                    );
                                }

                                next_node.value = Some(next_node_val);
                            }
                        }
                        break;
                    }
                    curr_node = next_node;
                }
                None => {
                    break;
                }
            }
        }
    }

    /// Insert middleware into specific node
    pub fn insert_middleware(&mut self, key: &str, middleware: impl Middleware) {
        // Split key and drop additional '/'
        let split_key = key.split('/');
        let mut split_key = split_key.filter(|key| !key.is_empty()).peekable();

        let mut curr_node = &mut self.head;

        while let Some(k) = split_key.next() {
            match *curr_node.process_insertion(k) {
                Some(next_node) => {
                    if split_key.peek().is_none() {
                        match &mut next_node.value {
                            Some(val) => {
                                val.middleware.push(Arc::new(middleware));
                            }
                            None => {
                                let mut next_node_val = RouteValue::default();
                                next_node_val.middleware.push(Arc::new(middleware));

                                next_node.value = Some(next_node_val);
                            }
                        }
                        break;
                    }
                    curr_node = next_node;
                }
                None => {
                    break;
                }
            }
        }
    }

    /// Search node through the provided key
    /// Middleware will be accumulated throughout the search path
    pub fn search_route(&self, key: &str) -> Result<RouteValueResult, ObsidianError> {
        // Split key and drop additional '/'
        let split_key = key.split('/');
        let mut split_key = split_key.filter(|key| !key.is_empty()).peekable();

        let mut curr_node = &self.head;
        let mut params = HashMap::default();
        let mut middleware = vec![];

        match &curr_node.value {
            Some(val) => {
                middleware.append(&mut val.middleware.clone());
            }
            None => {}
        }

        while let Some(k) = split_key.next() {
            match *curr_node.get_next_node(k, &mut params) {
                Some(next_node) => {
                    curr_node = next_node;

                    match &curr_node.value {
                        Some(val) => middleware.append(&mut val.middleware.clone()),
                        None => {}
                    }
                }
                None => {
                    if curr_node.key == "*" {
                        break;
                    }
                    
                    // Path is not registered
                    return Err(ObsidianError::NoneError);
                }
            }
        }

        match &curr_node.value {
            Some(val) => {
                let route_val = RouteValue::new(middleware, val.route.clone());

                return Ok(RouteValueResult::new(route_val, params));
            }
            None => {
                return Err(ObsidianError::NoneError);
            }
        }
    }

    /// Insert src trie into the des as a child trie
    /// src will be under the node of des with the key path
    ///
    /// For example, /src/ -> /des/ with 'example' key path
    /// src will be located at /des/example/src/
    pub fn insert_sub_route(des: &mut Self, key: &str, src: Self) {
        // Split key and drop additional '/'
        let split_key = key.split('/');
        let mut split_key = split_key.filter(|key| !key.is_empty()).peekable();

        let mut curr_node = &mut des.head;

        if split_key.peek().is_none() {
            des.head = src.head;
            return;
        }

        while let Some(k) = split_key.next() {
            match *curr_node.process_insertion(k) {
                Some(next_node) => {
                    if split_key.peek().is_none() {
                        if next_node.value.is_some() || next_node.child_nodes.len() > 0 {
                            panic!("There is conflict between main router and sub router at '{}'. Make sure main router does not consist any routing data in '{}'.", key, key);
                        }

                        next_node.value = src.head.value;
                        next_node.child_nodes = src.head.child_nodes;
                        break;
                    }
                    curr_node = next_node;
                }
                None => {
                    break;
                }
            }
        }
    }

    fn insert_default_route(&mut self, route: Route) {
        match &mut self.head.value {
            Some(val) => {
                if let Some(duplicated) = val.route.add_route(route.method.clone(), route) {
                    panic!(
                        "Duplicated route method '{}' at '/' detected",
                        duplicated.method
                    );
                }
            }
            None => {
                let mut val = RouteValue::default();
                if let Some(duplicated) = val.route.add_route(route.method.clone(), route) {
                    panic!(
                        "Duplicated route method '{}' at '/' detected",
                        duplicated.method
                    );
                }

                self.head.value = Some(val);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    key: String,
    value: Option<RouteValue>,
    child_nodes: Vec<Node>,
}

impl Node {
    fn new(key: String, value: Option<RouteValue>) -> Self {
        Node {
            key,
            value,
            child_nodes: Vec::default(),
        }
    }

    fn process_insertion(&mut self, key: &str) -> Box<Option<&mut Self>> {
        let action = self.get_insertion_action(key);

        match action.name {
            ActionName::CreateNewNode => {
                let new_node = Self::new(key.to_string(), None);

                self.child_nodes.push(new_node);
                match self.child_nodes.last_mut() {
                    Some(node) => return Box::new(Some(node)),
                    None => {}
                };
            }
            ActionName::NextNode => {
                match self.child_nodes.get_mut(action.payload.node_index) {
                    Some(node) => {
                        return Box::new(Some(node));
                    }
                    None => {}
                };
            }
            ActionName::SplitKey => {
                match self.child_nodes.get_mut(action.payload.node_index) {
                    Some(node) => {
                        return node.process_insertion(&key[action.payload.match_count..]);
                    }
                    None => {}
                };
            }
            ActionName::SplitNode => {
                match self.child_nodes.get_mut(action.payload.node_index) {
                    Some(node) => {
                        let count = action.payload.match_count;
                        let child_key = node.key[count..].to_string();
                        let new_key = key[count..].to_string();
                        node.key = key[..count].to_string();

                        let mut inter_node = Self::new(child_key, None);

                        // Move out the previous child and transfer to intermediate node
                        inter_node.child_nodes = std::mem::replace(&mut node.child_nodes, vec![]);

                        node.child_nodes.push(inter_node);
                        let new_node = Self::new(new_key, None);

                        node.child_nodes.push(new_node);
                        match node.child_nodes.last_mut() {
                            Some(result_node) => return Box::new(Some(result_node)),
                            None => {}
                        }
                    }
                    None => {}
                }
            }
            ActionName::Error => match self.child_nodes.get(action.payload.node_index) {
                Some(node) => {
                    panic!(
                        "ERROR: Ambigous definition between {} and {}",
                        key, node.key
                    );
                }
                None => {}
            },
        }

        unreachable!();
    }

    fn get_insertion_action(&self, key: &str) -> Action {
        for (index, node) in self.child_nodes.iter().enumerate() {
            let is_param = node.key.chars().nth(0).unwrap_or(' ') == ':' || &key[0..1] == ":";
            if is_param {
                // Only allow one param leaf in on children series
                if key == node.key {
                    return Action::new(ActionName::NextNode, ActionPayload::new(0, index));
                } else {
                    return Action::new(ActionName::Error, ActionPayload::new(0, index));
                }
            }

            if node.key == "*" {
                return Action::new(ActionName::Error, ActionPayload::new(0, index));
            }

            // Not param
            let mut temp_key_ch = key.chars();
            let mut count = 0;

            for k in node.key.chars() {
                let t_k = match temp_key_ch.next() {
                    Some(key) => key,
                    None => break,
                };

                if t_k == k {
                    count = count + 1;
                } else {
                    break;
                }
            }

            if count == key.len() {
                return Action::new(ActionName::NextNode, ActionPayload::new(count, index));
            } else if count == node.key.len() {
                return Action::new(ActionName::SplitKey, ActionPayload::new(count, index));
            } else if count != 0 {
                return Action::new(ActionName::SplitNode, ActionPayload::new(count, index));
            }
        }

        Action::new(ActionName::CreateNewNode, ActionPayload::new(0, 0))
    }

    // Helper function to consume the whole key and get the next available node
    fn get_next_node(&self, key: &str, params: &mut HashMap<String, String>) -> Box<Option<&Self>> {
        for node in self.child_nodes.iter() {
            let is_param = node.key.chars().nth(0).unwrap_or(' ') == ':' || &key[0..1] == ":";
            if is_param {
                params.insert(node.key[1..].to_string(), key.to_string());
                return Box::new(Some(node));
            }

            let is_wildcard = node.key == "*";
            if is_wildcard {
                return Box::new(Some(node));
            }

            let mut temp_key_ch = key.chars();
            let mut count = 0;

            for k in node.key.chars() {
                let t_k = match temp_key_ch.next() {
                    Some(key) => key,
                    None => break,
                };

                if t_k == k {
                    count = count + 1;
                } else {
                    break;
                }
            }

            if count == key.len() {
                // fully match
                return Box::new(Some(node));
            } else if count == node.key.len() {
                // break key
                return node.get_next_node(&key[count..], params);
            }

            continue;
        }

        // Not found
        Box::new(None)
    }
}

enum ActionName {
    NextNode,
    CreateNewNode,
    SplitNode,
    SplitKey,
    Error,
}

struct ActionPayload {
    match_count: usize,
    node_index: usize,
}

struct Action {
    name: ActionName,
    payload: ActionPayload,
}

impl Action {
    pub fn new(name: ActionName, payload: ActionPayload) -> Self {
        Action { name, payload }
    }
}

impl ActionPayload {
    pub fn new(match_count: usize, node_index: usize) -> Self {
        ActionPayload {
            match_count,
            node_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::Logger;
    use crate::router::ResponseBuilder;

    #[test]
    fn radix_trie_head_test() {
        let mut route_trie = RouteTrie::new();
        let logger = Logger::new();
        let handler = |_x, _y| ResponseBuilder::new().body("test");

        route_trie.insert_default_middleware(logger);
        route_trie.insert_route("/", Route::new(Method::GET, handler));

        let result = route_trie.search_route("/");

        assert!(result.is_ok());

        match result {
            Ok(route) => {
                let middleware = route.get_middleware();
                let route_value = route.get_route(&Method::GET).unwrap();

                assert_eq!(middleware.len(), 1);
                assert_eq!(route_value.method, Method::GET);
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn radix_trie_normal_test() {
        let mut route_trie = RouteTrie::new();
        let logger = Logger::new();
        let handler = |_x, _y| ResponseBuilder::new().body("test");

        route_trie.insert_default_middleware(logger);
        route_trie.insert_route("/normal/test/", Route::new(Method::GET, handler));

        let result = route_trie.search_route("/normal/test/");

        assert!(result.is_ok());

        match result {
            Ok(route) => {
                let middleware = route.get_middleware();
                let route_value = route.get_route(&Method::GET).unwrap();

                assert_eq!(middleware.len(), 1);
                assert_eq!(route_value.method, Method::GET);
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn radix_trie_not_found_test() {
        let mut route_trie = RouteTrie::new();
        let logger = Logger::new();
        let handler = |_x, _y| ResponseBuilder::new().body("test");

        route_trie.insert_default_middleware(logger);
        route_trie.insert_route("/normal/test/", Route::new(Method::GET, handler));

        let result = route_trie.search_route("/fail/test/");

        assert!(result.is_err());
    }

    #[test]
    fn radix_trie_split_node_and_key_test() {
        let mut route_trie = RouteTrie::new();
        let logger = Logger::new();
        let handler = |_x, _y| ResponseBuilder::new().body("test");

        route_trie.insert_default_middleware(logger);
        route_trie.insert_route("/normal/test/", Route::new(Method::GET, handler));

        route_trie.insert_route("/noral/test/", Route::new(Method::GET, handler));

        let normal_result = route_trie.search_route("/normal/test/");
        let noral_result = route_trie.search_route("/noral/test/");

        assert!(normal_result.is_ok());
        assert!(noral_result.is_ok());

        match normal_result {
            Ok(route) => {
                let middleware = route.get_middleware();
                let route_value = route.get_route(&Method::GET).unwrap();

                assert_eq!(middleware.len(), 1);
                assert_eq!(route_value.method, Method::GET);
            }
            _ => {
                assert!(false);
            }
        }

        match noral_result {
            Ok(route) => {
                let middleware = route.get_middleware();
                let route_value = route.get_route(&Method::GET).unwrap();

                assert_eq!(middleware.len(), 1);
                assert_eq!(route_value.method, Method::GET);
            }
            _ => {
                assert!(false);
            }
        }
    }
}
