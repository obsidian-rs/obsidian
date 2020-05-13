pub mod cookie_parser;
pub mod cookie_session;
pub mod logger;

use async_trait::async_trait;

use crate::app::EndpointExecutor;
use crate::context::Context;
use crate::router::ContextResult;

#[async_trait]
pub trait Middleware: Send + Sync + 'static {
    async fn handle<'a>(
        &'a self,
        context: Context,
        ep_executor: EndpointExecutor<'a>,
    ) -> ContextResult;
}
