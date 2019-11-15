use super::{Responder, ResponseResult};
use crate::context::Context;

pub trait EndPointHandler: Send + Sync + 'static {
    fn call_handler(&self, ctx: Context) -> ResponseResult;
}

impl<T, R> EndPointHandler for T
where
    T: Fn(Context) -> R + Send + Sync + 'static,
    R: Responder,
{
    fn call_handler(&self, ctx: Context) -> ResponseResult {
        (self)(ctx).respond_to()
    }
}
