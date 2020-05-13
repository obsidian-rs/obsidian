use chrono::{DateTime, Duration, Local};

use std::collections::HashMap;
use std::ops::Add;

pub trait SessionStorage: Send + Sync {
    fn get_session(&self, id: &str) -> Option<SessionData>;
    fn set_session(&self, id: &str, session: SessionData);
    fn remove_session(&self, id: &str);
}

/// Session data to be consumed by context
#[derive(Clone)]
pub struct SessionData {
    data: HashMap<String, String>,
    max_age: Option<DateTime<Local>>,
    timeout: Option<Duration>,
}

impl SessionData {
    pub fn new() -> Self {
        SessionData {
            data: HashMap::default(),
            max_age: None,
            timeout: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.data.get(name)
    }

    pub fn get_all(&self) -> &HashMap<String, String> {
        &self.data
    }

    pub fn set(&mut self, name: &str, value: impl ToString) -> Option<String> {
        self.data.insert(name.to_string(), value.to_string())
    }

    pub fn refresh_session(&mut self) {
        match self.timeout {
            Some(timeout) => {
                self.max_age = Some(Local::now().add(timeout));
            }
            None => {}
        }
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.max_age = Some(Local::now().add(timeout));
        self
    }

    pub fn is_alive(&self) -> bool {
        match self.max_age {
            Some(max_age) => max_age < Local::now(),
            None => true,
        }
    }

    pub fn remove(&mut self, name: &str) -> Option<String> {
        self.data.remove(name)
    }
}
