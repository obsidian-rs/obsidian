use futures::{Future, Stream};
use hyper::{Body, Request, Response};
use serde_json::Value;

use super::Middleware;

use crate::app::EndpointExecutor;
use crate::context::Context;
pub struct BodyParser {}

impl BodyParser {
    pub fn new() -> Self {
        BodyParser {}
    }
}

impl Middleware for BodyParser {
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

        let json_result: serde_json::Result<Value> = serde_json::from_slice(&b);

        /* match json_result {
            Ok(json_body) => context.json.add_json(json_body),
            Err(e) => println!("{}", e),
        } */

        let req = Request::from_parts(parts, Body::from(b));
        context.request = req;

        ep_executor.next(context)
    }
}
