mod body_parser;
mod logger;
mod url_encoded_parser;

use futures::future::Future;
use hyper::{Body, Response};

pub use self::body_parser::BodyParser;
pub use self::logger::Logger;
pub use self::url_encoded_parser::UrlEncodedParser;
use crate::app::EndpointExecutor;
use crate::context::Context;

pub trait Middleware: Send + Sync + 'static {
    fn handle<'a>(
        &'a self,
        context: Context,
        ep_executor: EndpointExecutor<'a>,
    ) -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;
}
