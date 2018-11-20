pub trait Injection <'a> {
    fn new() -> Self;

    fn get(&mut self, path: &str, handler: impl Fn() + 'a);
    fn post(&mut self, path: &str, handler: impl Fn() + 'a);
    fn put(&mut self, path: &str, handler: impl Fn() + 'a);
    fn delete(&mut self, path: &str, handler: impl Fn() + 'a);
}