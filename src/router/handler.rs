use super::{Responder, ResponseResult};
use crate::context::Context;

pub trait Handler: Send + Sync + 'static {
    fn call(&self, ctx: Context) -> ResponseResult;
}

impl<T, R> Handler for T
where
    T: Fn(Context) -> R + Send + Sync + 'static,
    R: Responder,
{
    fn call(&self, ctx: Context) -> ResponseResult {
        (self)(ctx).respond_to()
    }
}
