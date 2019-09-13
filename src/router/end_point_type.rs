use super::Responder;
use super::ResponseResult;

use super::ResponseBuilder;
use crate::context::Context;

use hyper::{Body, Response};

pub trait EndPointHandler: Send + Sync + 'static {
    fn call_handler(&self, ctx: Context) -> ResponseResult;
}

impl<T, K> EndPointHandler for T
where
    T: Fn(Context, ResponseBuilder) -> ResponseResult<K> + Send + Sync + 'static,
    K: Responder,
{
    fn call_handler(&self, ctx: Context) -> ResponseResult {
        match (self)(ctx, ResponseBuilder::new()) {
            Ok(res) => res.respond_to(),
            Err(err) => Err(err),
        }
    }
}

// pub trait EndPointHandler:
//     Fn(Context, ResponseBuilder) -> ResponseBuilder + Send + Sync + 'static
// {
// }

// impl<T> EndPointHandler for T where
//     T: Fn(Context, ResponseBuilder) -> ResponseBuilder + Send + Sync + 'static
// {
// }
