use crate::context::Context;
use crate::error::ObsidianError;
use crate::router::{Responder, Response};

use async_trait::async_trait;
use std::future::Future;

/// A HTTP request handler.
///
/// This trait is expected by router to perform function for every request.
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn call(&self, ctx: Context) -> ContextResult;
}

#[async_trait]
impl<T, F, R> Handler for T
where
    T: Fn(Context) -> F + Send + Sync + 'static,
    F: Future<Output = R> + Send + 'static,
    R: Responder + 'static,
{
    async fn call(&self, ctx: Context) -> ContextResult {
        Ok((self)(ctx).await.respond_to())
    }
}

/// `Result` with the error type `ObsidianError`.
pub type ContextResult<E = ObsidianError> = Result<Response, E>;
