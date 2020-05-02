use async_trait::async_trait;
use std::time::Instant;

use crate::app::EndpointExecutor;
use crate::context::Context;
use crate::middleware::Middleware;
use crate::router::ContextResult;

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
    ) -> ContextResult {
        let start = Instant::now();
        println!("[info] {} {}", context.method(), context.uri(),);
        println!("{:#?}", context);

        match ep_executor.next(context).await {
            Ok(context_after) => {
                let duration = start.elapsed();
                println!(
                    "[info] Sent {} in {:?}",
                    context_after.response().as_ref().unwrap().status(),
                    duration
                );
                Ok(context_after)
            }
            Err(error) => {
                println!("[error] {}", error);
                Err(error)
            }
        }
    }
}
