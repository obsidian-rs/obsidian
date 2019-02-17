use futures::future::Future;
use hyper::{Body, Response};

use super::context::Context;

pub trait Middleware: Send + Sync + 'static {
    fn handle<'a>(
        &'a self,
        context: Context<'a>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;
}
