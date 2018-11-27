use hyper::{Request, Response};

pub trait EndPointHandler: Fn(Request<()>, Response<()>) -> Response<()> + Send + 'static {}
impl<T> EndPointHandler for T where T: Fn(Request<()>, Response<()>) -> Response<()> + Send + 'static {}
