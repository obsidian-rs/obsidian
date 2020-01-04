use async_trait::async_trait;

use crate::app::EndpointExecutor;
use crate::context::Context;
use crate::middleware::Middleware;
use crate::{Body, Response};

#[derive(Default)]
pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Logger {}
    }
}

#[async_trait]
impl Middleware for Logger {
    async fn handle<'a>(
        &'a self,
        context: Context,
        ep_executor: EndpointExecutor<'a>,
    ) -> Response<Body> {
        println!(
            "{} {} \n{}",
            context.method(),
            context.uri(),
            context.headers().get("host").unwrap().to_str().unwrap()
        );

        ep_executor.next(context).await
    }
}
