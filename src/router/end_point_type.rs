use super::ResponseBuilder;
use hyper::{Body, Request};

pub trait EndPointHandler:
    Fn(Request<Body>, ResponseBuilder) -> ResponseBuilder + Send + Sync + 'static
{
}

impl<T> EndPointHandler for T where
    T: Fn(Request<Body>, ResponseBuilder) -> ResponseBuilder + Send + Sync + 'static
{
}
