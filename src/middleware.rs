mod body_parser;
mod logger;
mod url_encoded_parser;

use futures::future::Future;
use hyper::{Body, Response};

use crate::context::Context;

pub use self::body_parser::BodyParser;
pub use self::logger::Logger;
pub use self::url_encoded_parser::UrlEncodedParser;

pub trait Middleware: Send + Sync + 'static {
    fn handle<'a>(
        &'a self,
        context: Context<'a>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;
}
