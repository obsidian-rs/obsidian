use super::{Responder, ResponseResult};
use crate::context::Context;

use std::future::Future;
use async_trait::async_trait;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn call(&self, ctx: Context) -> ResponseResult;
}

#[async_trait]
impl<T, F> Handler for T
where
    T: Fn(Context) -> F + Send + Sync + 'static,
    F: Future + Send + 'static,
    F::Output: Responder,
{
    async fn call(&self, ctx: Context) -> ResponseResult {
        (self)(ctx).await.respond_to()
    }
}
