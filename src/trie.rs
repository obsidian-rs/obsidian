use crate::context::ObsidianError;
use crate::middleware::Middleware;
use crate::router::Resource;
use crate::router::Route;
use std::collections::HashMap;
use std::sync::Arc;

use hyper::Method;

#[derive(Clone, Default)]
pub struct RouteValue {
    middleware: Vec<Arc<dyn Middleware>>,
    route: Resource,
}

impl RouteValue {
    pub fn new(middleware: Vec<Arc<dyn Middleware>>, route: Resource) -> Self {
        RouteValue {
            middleware,
            route,
        }
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
}

#[derive(Clone)]
pub struct Trie {
    head: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            head: Node::new("/".to_string(), None),
        }
    }

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
                                val.route.add_route(route.method.clone(), route);
                            }
                            None => {
                                let mut next_node_val = RouteValue::default();
                                next_node_val.route.add_route(route.method.clone(), route);

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
                        Some(val) => {
                            middleware.append(&mut val.middleware.clone())
                        },
                        None => {},
                    }
                }
                None => {
                    if split_key.peek().is_some() {
                        // Path is not registered
                        return Err(ObsidianError::NoneError);
                    }
                    break;
                }
            }
        }

        match &curr_node.value {
            Some(val) => {
                middleware.append(&mut val.middleware.clone());

                let route_val = RouteValue::new(middleware, val.route.clone());

                return Ok(RouteValueResult::new(route_val, params));
            }
            None => {
                return Err(ObsidianError::NoneError);
            }
        }
    }

    fn insert_default_route(&mut self, route: Route) {
        match &mut self.head.value {
            Some(val) => {
                val.route.add_route(route.method.clone(), route);
            }
            None => {
                let mut val = RouteValue::default();
                val.route.add_route(route.method.clone(), route);

                self.head.value = Some(val);
            }
        }
    }
}

#[derive(Clone)]
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
    /*
    fn search_node(node: &Self, key: &str) -> Result<RouteValueResult, ObsidianError> {
        // Split key and drop additional '/'
        let split_key = key.split('/');
        let mut split_key = split_key.filter(|key| !key.is_empty()).peekable();

        let mut curr_node = node;
        let mut params = HashMap::default();

        while let Some(k) = split_key.next() {
            match *curr_node.get_next_node(k, &mut params) {
                Some(next_node) => {
                    curr_node = next_node;
                }
                None => {
                    if split_key.peek().is_some() {
                        // Path is not registered
                        return Err(ObsidianError::NoneError);
                    }
                    break;
                }
            }
        }

        match &node.value {
            Some(val) => {
                return Ok(RouteValueResult::new(val.clone(), params));
            }
            None => {
                return Err(ObsidianError::NoneError);
            }
        }
    }

    fn insert_route(node: &mut Self, key: &str, route: Route) {
        // Split key and drop additional '/'
        let split_key = key.split('/');
        let mut split_key = split_key.filter(|key| !key.is_empty()).peekable();

        let mut curr_node = node;

        while let Some(k) = split_key.next() {
            match *curr_node.process_insertion(k) {
                Some(next_node) => {
                    if split_key.peek().is_none() {
                        match &mut next_node.value {
                            Some(val) => {
                                val.route.add_route(route.method.clone(), route);
                            }
                            None => {
                                let mut next_node_val = RouteValue::default();
                                next_node_val.route.add_route(route.method.clone(), route);

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

    fn insert_middleware(node: &mut Self, key: &str, middleware: impl Middleware) {
        // Split key and drop additional '/'
        let split_key = key.split('/');
        let mut split_key = split_key.filter(|key| !key.is_empty()).peekable();

        let mut curr_node = node;

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
    } */

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
                    panic!("Ambigous definition between {} and {}", key, node.key);
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
