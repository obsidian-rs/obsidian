use futures::Future;
use hyper::{Body, Response};

use super::Middleware;
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
        context: Context<'a>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        println!(
            "{} {} \n{}",
            context.request.method(),
            context.request.uri(),
            context
                .request
                .headers()
                .get("host")
                .unwrap()
                .to_str()
                .unwrap()
        );
        context.next()
    }
}
