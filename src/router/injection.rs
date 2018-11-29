use super::EndPointHandler;

pub trait Injection {
    fn get(&mut self, path: &str, handler: impl EndPointHandler);
    fn post(&mut self, path: &str, handler: impl EndPointHandler);
    fn put(&mut self, path: &str, handler: impl EndPointHandler);
    fn delete(&mut self, path: &str, handler: impl EndPointHandler);
}
