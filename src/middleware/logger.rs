use async_trait::async_trait;
use std::time::Instant;

use crate::app::EndpointExecutor;
use crate::context::Context;
use crate::handler::ContextResult;
use crate::middleware::Middleware;

use colored::*;

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

        #[cfg(debug_assertions)]
        println!("{} {:#?}", "[debug]".cyan(), context);

        match ep_executor.next(context).await {
            Ok(response) => {
                let duration = start.elapsed();
                let status = response.status();
                println!("[info] Sent {} in {:?}", status, duration);
                Ok(response)
            }
            Err(error) => {
                println!("{} {}", "[error]".red(), error);
                Err(error)
            }
        }
    }
}
