use async_trait::async_trait;

use obsidian::{context::Context, middleware::Middleware, Body, EndpointExecutor, Response};

#[derive(Default)]
pub struct LoggerExample {}

pub struct LoggerExampleData(pub String);

impl LoggerExample {
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
    ) -> Response<Body> {
        println!(
            "{} {} \n{}",
            context.method(),
            context.uri(),
            context.headers().get("host").unwrap().to_str().unwrap()
        );

        context.add(LoggerExampleData("This is logger data".to_string()));

        ep_executor.next(context).await
    }
}
