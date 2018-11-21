pub trait Injection{
    fn get(&mut self, path: &str, handler: impl Fn() + Send + 'static);
    fn post(&mut self, path: &str, handler: impl Fn() + Send + 'static);
    fn put(&mut self, path: &str, handler: impl Fn() + Send + 'static);
    fn delete(&mut self, path: &str, handler: impl Fn() + Send + 'static);
}