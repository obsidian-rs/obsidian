pub mod logger;

use async_trait::async_trait;

use crate::app::EndpointExecutor;
use crate::context::Context;
use crate::handler::ContextResult;

/// Middleware trait provides a way to allow user to:
/// 1. alter the request data before proceeding to the next middleware or the handler
/// 2. transform the response on the way out
/// 3. perform side-effects before or after handler processing
///
/// ```
/// use async_trait::async_trait;
/// use obsidian::middleware::Middleware;
///
/// #[async_trait]
/// impl Middleware for ExampleMiddleware {
///     async fn handle<'a>(
///         &'a self,
///         context: Context,
///         ep_executor: EndpointExecutor<'a>,
///     ) -> ContextResult {
///
///     // actions BEFORE handler processing
///     let result = ep_executor.next(context).await?;
///     // actions AFTER handler processing
///
///     Ok(result)
/// }
/// ```
#[async_trait]
pub trait Middleware: Send + Sync + 'static {
    async fn handle<'a>(
        &'a self,
        context: Context,
        ep_executor: EndpointExecutor<'a>,
    ) -> ContextResult;
}
