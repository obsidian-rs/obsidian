use futures::{Future, Stream};
use hyper::{Body, Request, Response};
use url::form_urlencoded;

use super::Middleware;

use crate::app::EndpointExecutor;
use crate::context::Context;
pub struct UrlEncodedParser {}

impl UrlEncodedParser {
    pub fn new() -> Self {
        UrlEncodedParser {}
    }
}

impl Middleware for UrlEncodedParser {
    fn handle<'a>(
        &'a self,
        context: Context,
        ep_executor: EndpointExecutor<'a>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let mut context = context;
        let (parts, body) = context.request.into_parts();

        let b = match body.concat2().wait() {
            Ok(chunk) => chunk,
            Err(e) => {
                println!("{}", e);
                hyper::Chunk::default()
            }
        };

        let params_iter = form_urlencoded::parse(b.as_ref()).into_owned();

        for (key, value) in params_iter {
            context.params_data.add_params(key, value);
        }

        let req = Request::from_parts(parts, Body::from(b));
        context.request = req;

        ep_executor.next(context)
    }
}
