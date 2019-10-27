use super::{ResponseResult, Responder};
use crate::context::Context;

pub trait EndPointHandler: Send + Sync + 'static {
  fn call_handler(&self, ctx: Context) -> ResponseResult;
}

impl<T, R> EndPointHandler for T where
    T: Fn(Context) -> R + Send + Sync + 'static,
    R: Responder
{
    fn call_handler(&self, ctx: Context) -> ResponseResult {
        (self)(ctx).respond_to()
    }
}

// impl<T> EndPointHandler for T where
//     T: Fn(Context) -> (dyn Responder) + Send + Sync + 'static
// {
//     fn call_handler(ctx: Context) -> ResponseResult {
//         (self)(ctx).respond_to()
//     }
// }
