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
    pub fn insert_route(&mut self, path: &str, route: Route) {
        // Split path string and drop additional '/'
        let mut splitted_path = path.split('/').filter(|key| !key.is_empty()).peekable();

        let mut curr_node = &mut self.head;

        // if the path is "/"
        if splitted_path.peek().is_none() {
            self.insert_default_route(route);
            return;
        }

        while let Some(k) = splitted_path.next() {
            match curr_node.process_insertion(k) {
                Some(next_node) => {
                    if splitted_path.peek().is_none() {
                        match &mut next_node.value {
                            Some(val) => {
                                if let Some(duplicated) =
                                    val.route.add_route(route.method.clone(), route)
                                {
                                    panic!(
                                        "Duplicated route method '{}' at '{}' detected",
                                        duplicated.method, path
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
                                        duplicated.method, path
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
            match curr_node.process_insertion(k) {
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
        let mut split_key = split_key
            .filter(|key| !key.is_empty())
            .collect::<Vec<&str>>();

        let mut curr_node = &self.head;
        let mut params = HashMap::default();
        let mut middleware = vec![];

        match &curr_node.value {
            Some(val) => {
                middleware.append(&mut val.middleware.clone());
            }
            None => {}
        }

        if !split_key.is_empty() {
            match curr_node.get_next_node(&mut split_key, &mut params, &mut middleware, false) {
                Some(handler_node) => {
                    curr_node = handler_node;
                }
                None => {
                    // Path is not registered
                    return Err(ObsidianError::NoneError);
                }
            }
        }

        match &curr_node.value {
            Some(val) => {
                let route_val = RouteValue::new(middleware, val.route.clone());

                Ok(RouteValueResult::new(route_val, params))
            }
            None => Err(ObsidianError::NoneError),
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
            match curr_node.process_insertion(k) {
                Some(next_node) => {
                    if split_key.peek().is_none() {
                        if next_node.value.is_some() || !next_node.child_nodes.is_empty() {
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

    fn is_param(&self) -> bool {
        self.key.chars().next().unwrap_or(' ') == ':'
    }

    /// Process the side effects of node insertion
    fn process_insertion(&mut self, key: &str) -> Option<&mut Self> {
        let action = self.get_insertion_action(key);

        match action.name {
            ActionName::CreateNewNode => {
                let new_node = Self::new(key.to_string(), None);

                self.child_nodes.push(new_node);
                if let Some(node) = self.child_nodes.last_mut() {
                    return Some(node);
                };
            }
            ActionName::NextNode => {
                if let Some(node) = self.child_nodes.get_mut(action.payload.node_index) {
                    return Some(node);
                };
            }
            ActionName::SplitKey => {
                if let Some(node) = self.child_nodes.get_mut(action.payload.node_index) {
                    return node.process_insertion(&key[action.payload.match_count..]);
                };
            }
            ActionName::SplitNode => {
                if let Some(node) = self.child_nodes.get_mut(action.payload.node_index) {
                    let count = action.payload.match_count;
                    let child_key = node.key[count..].to_string();
                    let new_key = key[count..].to_string();
                    node.key = key[..count].to_string();

                    let mut inter_node = Self::new(child_key, None);

                    // Move out the previous child and transfer to intermediate node
                    inter_node.child_nodes = std::mem::replace(&mut node.child_nodes, vec![]);
                    inter_node.value = std::mem::replace(&mut node.value, None);

                    node.child_nodes.push(inter_node);

                    // In the case of insert key length less than matched node key length
                    if new_key.is_empty() {
                        return Some(node);
                    }

                    let new_node = Self::new(new_key, None);

                    node.child_nodes.push(new_node);
                    if let Some(result_node) = node.child_nodes.last_mut() {
                        return Some(result_node);
                    }
                };
            }
            ActionName::Error => {
                if let Some(node) = self.child_nodes.get(action.payload.node_index) {
                    panic!(
                        "ERROR: Ambigous definition between {} and {}",
                        key, node.key
                    );
                }
            }
        }

        unreachable!();
    }

    /// Determine the action required to be performed for the new route path
    fn get_insertion_action(&self, key: &str) -> Action {
        for (index, node) in self.child_nodes.iter().enumerate() {
            let is_param = node.is_param() || key.chars().next().unwrap_or(' ') == ':';
            if is_param {
                // Only allow one param leaf in on children series
                if key == node.key {
                    return Action::new(ActionName::NextNode, ActionPayload::new(0, index));
                } else {
                    return Action::new(ActionName::Error, ActionPayload::new(0, index));
                }
            }

            // Wildcard can only be the last leaf
            if node.key == "*" && key != "*" {
                return Action::new(ActionName::Error, ActionPayload::new(0, index));
            }

            let mut temp_key_ch = key.chars();
            let mut count = 0;

            // match characters
            for k in node.key.chars() {
                let t_k = match temp_key_ch.next() {
                    Some(key) => key,
                    None => break,
                };

                if t_k == k {
                    count += t_k.len_utf8();
                } else {
                    break;
                }
            }

            if count == key.len() && count == node.key.len() {
                return Action::new(ActionName::NextNode, ActionPayload::new(count, index));
            }

            if count == node.key.len() {
                return Action::new(ActionName::SplitKey, ActionPayload::new(count, index));
            }

            if count != 0 {
                return Action::new(ActionName::SplitNode, ActionPayload::new(count, index));
            }
        }

        // No child node matched the key, creates new node
        Action::new(ActionName::CreateNewNode, ActionPayload::new(0, 0))
    }

    // Helper function to consume the whole key and get the next available node
    fn get_next_node(
        &self,
        key: &mut Vec<&str>,
        params: &mut HashMap<String, String>,
        middleware: &mut Vec<std::sync::Arc<(dyn Middleware + 'static)>>,
        is_break_parent: bool,
    ) -> Option<&Self> {
        let curr_key = key.remove(0);

        for node in self.child_nodes.iter() {
            let mut break_key = false;

            if !is_break_parent {
                // Check param
                if node.is_param() {
                    if key.is_empty() {
                        match &node.value {
                            Some(curr_val) => {
                                params.insert(node.key[1..].to_string(), curr_key.to_string());
                                middleware.append(&mut curr_val.middleware.clone());
                                return Some(node);
                            }
                            None => {
                                continue;
                            }
                        }
                    } else {
                        match node.get_next_node(key, params, middleware, break_key) {
                            Some(final_val) => {
                                params.insert(node.key[1..].to_string(), curr_key.to_string());

                                match &node.value {
                                    Some(curr_val) => {
                                        middleware.append(&mut curr_val.middleware.clone());
                                    }
                                    None => {}
                                }

                                return Some(final_val);
                            }
                            None => {
                                continue;
                            }
                        }
                    }
                }

                // Check wildcard
                if node.key == "*" {
                    match &node.value {
                        Some(curr_val) => {
                            middleware.append(&mut curr_val.middleware.clone());
                        }
                        None => {}
                    }

                    return Some(node);
                }
            }

            let mut temp_key_ch = curr_key.chars();
            let mut count = 0;

            // match characters
            for k in node.key.chars() {
                let t_k = match temp_key_ch.next() {
                    Some(key) => key,
                    None => break,
                };

                if t_k == k {
                    count += t_k.len_utf8();
                } else {
                    break;
                }
            }

            if count == node.key.len() && count != curr_key.len() {
                // break key
                break_key = true;
                key.insert(0, &curr_key[count..]);
            }

            if count != 0 && count == node.key.len() {
                if key.is_empty() {
                    match &node.value {
                        Some(curr_val) => {
                            middleware.append(&mut curr_val.middleware.clone());
                            return Some(node);
                        }
                        None => {
                            for child in node.child_nodes.iter() {
                                if child.key == "*" {
                                    if let Some(child_val) = &child.value {
                                        middleware.append(&mut child_val.middleware.clone());
                                        return Some(child);
                                    }
                                }
                            }

                            continue;
                        }
                    }
                } else if let Some(final_val) =
                    node.get_next_node(key, params, middleware, break_key)
                {
                    if let Some(curr_val) = &node.value {
                        middleware.append(&mut curr_val.middleware.clone());
                    }

                    return Some(final_val);
                }
            }

            continue;
        }

        // Not found
        None
    }
}

/// Action to be performed by the node
enum ActionName {
    NextNode,
    CreateNewNode,
    SplitNode,
    SplitKey,
    Error,
}

/// Action Payload:
/// characters matched for the node key and insert key
/// node index in the node vector
struct ActionPayload {
    match_count: usize,
    node_index: usize,
}

/// Container for actions will be performed in the trie
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
                let route_value = route.get_route(&Method::GET).is_some();

                assert_eq!(middleware.len(), 1);
                assert!(route_value);
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
        let logger2 = Logger::new();
        let handler = |_x, _y| ResponseBuilder::new().body("test");

        route_trie.insert_default_middleware(logger);
        route_trie.insert_route("/normal/test/", Route::new(Method::GET, handler));
        route_trie.insert_route("/ノーマル/テスト/", Route::new(Method::GET, handler));
        route_trie.insert_middleware("/ノーマル/テスト/", logger2);

        let result = route_trie.search_route("/normal/test/");

        assert!(result.is_ok());

        match result {
            Ok(route) => {
                let middleware = route.get_middleware();
                let route_value = route.get_route(&Method::GET).is_some();

                assert_eq!(middleware.len(), 1);
                assert!(route_value);
            }
            _ => {
                assert!(false);
            }
        }

        let result = route_trie.search_route("/ノーマル/テスト/");

        assert!(result.is_ok());

        match result {
            Ok(route) => {
                let middleware = route.get_middleware();
                let route_value = route.get_route(&Method::GET).is_some();

                assert_eq!(middleware.len(), 2);
                assert!(route_value);
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
        let logger2 = Logger::new();
        let logger3 = Logger::new();
        let handler = |_x, _y| ResponseBuilder::new().body("test");

        route_trie.insert_default_middleware(logger);
        route_trie.insert_route("/normal/test/", Route::new(Method::GET, handler));
        route_trie.insert_route("/noral/test/", Route::new(Method::GET, handler));
        route_trie.insert_route("/ノーマル/テスト/", Route::new(Method::GET, handler));
        route_trie.insert_route("/ノーマル/テーブル/", Route::new(Method::GET, handler));
        route_trie.insert_middleware("/noral/test/", logger2);
        route_trie.insert_middleware("/ノーマル/テーブル/", logger3);

        let test_cases = vec![
            ("/normal/test/", 1),
            ("/noral/test/", 2),
            ("/ノーマル/テスト/", 1),
            ("/ノーマル/テーブル/", 2),
        ];

        for case in test_cases.iter() {
            let normal_result = route_trie.search_route(case.0);

            assert!(normal_result.is_ok());

            match normal_result {
                Ok(route) => {
                    let middleware = route.get_middleware();
                    let route_value = route.get_route(&Method::GET).is_some();

                    assert_eq!(middleware.len(), case.1);
                    assert!(route_value);
                }
                _ => {
                    assert!(false);
                }
            }
        }
    }

    #[test]
    fn radix_trie_wildcard_test() {
        let mut route_trie = RouteTrie::new();
        let handler = |_x, _y| ResponseBuilder::new().body("test");
        let logger = Logger::new();
        let logger2 = Logger::new();
        let logger3 = Logger::new();

        route_trie.insert_route("/normal/test/*", Route::new(Method::GET, handler));
        route_trie.insert_middleware("/normal/test/*", logger);
        route_trie.insert_middleware("/normal/test/*", logger2);
        route_trie.insert_middleware("/normal/test/*", logger3);

        let test_cases = vec![
            "/normal/test/test",
            "/normal/test/123",
            "/normal/test/こんにちは",
            "/normal/test/啊",
        ];

        for case in test_cases.iter() {
            let normal_result = route_trie.search_route(case);

            assert!(normal_result.is_ok());

            match normal_result {
                Ok(route) => {
                    let middleware = route.get_middleware();
                    let route_value = route.get_route(&Method::GET).is_some();

                    assert_eq!(middleware.len(), 3);
                    assert!(route_value);
                }
                _ => {
                    assert!(false);
                }
            }
        }
    }

    #[should_panic]
    #[test]
    fn radix_trie_wildcard_param_conflict_test() {
        let mut route_trie = RouteTrie::new();
        let handler = |_x, _y| ResponseBuilder::new().body("test");

        route_trie.insert_route("/normal/test/*", Route::new(Method::GET, handler));
        route_trie.insert_route("/normal/test/:param", Route::new(Method::GET, handler));
    }

    #[should_panic]
    #[test]
    fn radix_trie_param_wildcard_conflict_test() {
        let mut route_trie = RouteTrie::new();
        let handler = |_x, _y| ResponseBuilder::new().body("test");

        route_trie.insert_route("/normal/test/:param", Route::new(Method::GET, handler));
        route_trie.insert_route("/normal/test/*", Route::new(Method::GET, handler));
    }
}
