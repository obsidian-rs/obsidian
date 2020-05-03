use async_trait::async_trait;

#[cfg(debug_assertions)]
use colored::*;

use obsidian::{context::Context, middleware::Middleware, ContextResult, EndpointExecutor};

#[derive(Default)]
pub struct LoggerExample {}

pub struct LoggerExampleData(pub String);

impl LoggerExample {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LoggerExample {}
    }
}

#[async_trait]
impl Middleware for LoggerExample {
    async fn handle<'a>(
        &'a self,
        mut context: Context,
        ep_executor: EndpointExecutor<'a>,
    ) -> ContextResult {
        #[cfg(debug_assertions)]
        println!("{}", "[debug] Inside middleware".blue());

        println!(
            "{} {}{}",
            context.method(),
            context.headers().get("host").unwrap().to_str().unwrap(),
            context.uri(),
        );

        context.add(LoggerExampleData("This is logger data".to_string()));

        ep_executor.next(context).await
    }
}
