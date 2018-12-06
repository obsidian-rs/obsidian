use super::ObsidianResponse;
use hyper::{Body, Request};

pub trait EndPointHandler:
    Fn(Request<Body>, ObsidianResponse) -> ObsidianResponse + Send + Sync + 'static
{
}

impl<T> EndPointHandler for T where
    T: Fn(Request<Body>, ObsidianResponse) -> ObsidianResponse + Send + Sync + 'static
{
}
