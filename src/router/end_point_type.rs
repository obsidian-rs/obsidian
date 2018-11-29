use http::response::Builder;
use hyper::{Body, Request, Response};

pub trait EndPointHandler:
    Fn(Request<Body>, &mut Builder) -> Response<Body> + Send + Sync + 'static
{
}

impl<T> EndPointHandler for T where
    T: Fn(Request<Body>, &mut Builder) -> Response<Body> + Send + Sync + 'static
{
}
