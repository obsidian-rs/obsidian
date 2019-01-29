use std::collections::HashMap;
use std::sync::Arc;

use futures::future::Future;
use hyper::{Body, Request, Response};

use super::middleware::Middleware;
use super::router::{EndPointHandler, RequestData, ResponseBuilder};

pub struct Context<'a> {
    pub request: Request<Body>,
    pub middleware: &'a [Arc<Middleware>],
    pub route_endpoint: &'a Arc<dyn EndPointHandler<Output = ResponseBuilder>>,
    pub params: &'a HashMap<String, String>,
}

impl<'a> Context<'a> {
    pub fn new(
        request: Request<Body>,
        route_endpoint: &'a Arc<dyn EndPointHandler<Output = ResponseBuilder>>,
        middleware: &'a [Arc<Middleware>],
        params: &'a HashMap<String, String>,
    ) -> Self {
        Context {
            request,
            middleware,
            route_endpoint,
            params,
        }
    }

    pub fn next(mut self) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        if let Some((current, all_next)) = self.middleware.split_first() {
            self.middleware = all_next;
            current.run(self)
        } else {
            let res = ResponseBuilder::new();
            let request_data = RequestData::new(self.request, self.params.clone());
            let route_response = (*self.route_endpoint)(request_data, res);
            route_response.into()
        }
    }
}
