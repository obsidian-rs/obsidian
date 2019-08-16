use futures::Future;
use hyper::{Body, Response};

use super::Middleware;

use crate::app::EndpointExecutor;
use crate::context::Context;
pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Logger {}
    }
}

impl Middleware for Logger {
    fn handle<'a>(
        &'a self,
        context: Context,
        ep_executor: EndpointExecutor<'a>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        println!(
            "{} {} \n{}",
            context.method(),
            context.uri(),
            context
                .headers()
                .get("host")
                .unwrap()
                .to_str()
                .unwrap()
        );

        ep_executor.next(context)
    }
}
