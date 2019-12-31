mod logger;

use futures::future::Future;

pub use self::logger::Logger;

use crate::app::EndpointExecutor;
use crate::context::Context;
use crate::{Body, Response};

pub trait Middleware: Send + Sync + 'static {
    fn handle<'a>(
        &'a self,
        context: Context,
        ep_executor: EndpointExecutor<'a>,
    ) -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;
}
