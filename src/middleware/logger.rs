use futures::Future;

use crate::app::EndpointExecutor;
use crate::context::Context;
use crate::middleware::Middleware;
use crate::{Body, Response};

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
    ) -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        println!(
            "{} {} \n{}",
            context.method(),
            context.uri(),
            context.headers().get("host").unwrap().to_str().unwrap()
        );

        ep_executor.next(context)
    }
}
