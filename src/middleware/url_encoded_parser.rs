use futures::{Future, Stream};
use hyper::{Body, Request, Response};
use url::form_urlencoded;

use super::Middleware;
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
        context: Context<'a>,
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
            context.route_data.add_param(key, value);
        }

        let req = Request::from_parts(parts, Body::from(b));

        context.request = req;
        context.next()
    }
}
