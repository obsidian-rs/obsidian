use super::session::{SessionData, SessionStorage};

use std::collections::HashMap;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref STORAGE: Mutex<HashMap<String, SessionData>> = Mutex::new(HashMap::default());
}

pub struct MemorySessionStorage {}

impl MemorySessionStorage {
    pub fn new() -> Self {
        MemorySessionStorage {}
    }
}

impl SessionStorage for MemorySessionStorage {
    fn get_session(&self, id: &str) -> Option<SessionData> {
        STORAGE
            .lock()
            .unwrap()
            .get(id)
            .and_then(|x| Some((*x).clone()))
    }

    fn set_session(&self, id: &str, session: SessionData) {
        STORAGE.lock().unwrap().insert(id.to_string(), session);
    }

    fn remove_session(&self, id: &str) {
        STORAGE.lock().unwrap().remove(id);
    }
}
