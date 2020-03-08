use crate::context::Context;
use crate::error::ObsidianError;

use async_trait::async_trait;
use std::future::Future;

pub type ContextResult<T = ObsidianError> = Result<Context, T>;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn call(&self, ctx: Context) -> ContextResult;
}

#[async_trait]
impl<T, F> Handler for T
where
    T: Fn(Context) -> F + Send + Sync + 'static,
    F: Future<Output = ContextResult> + Send + 'static,
{
    async fn call(&self, ctx: Context) -> ContextResult {
        (self)(ctx).await
    }
}
