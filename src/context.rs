use std::sync::Arc;

use futures::future::Future;
use hyper::{Body, Request, Response};

use super::middleware::Middleware;
use super::router::{EndPointHandler, ResponseBuilder};

pub struct Context<'a> {
    request: Request<Body>,
    response: Response<Body>,
    middleware: &'a mut Vec<Arc<Middleware>>,
    route_endpoint: Arc<dyn EndPointHandler<Output = ResponseBuilder>>,
    current_index: usize,
    // params
}

impl<'a> Context<'a> {
    pub fn new(
        request: Request<Body>,
        route_endpoint: Arc<dyn EndPointHandler<Output = ResponseBuilder>>,
        middleware: &'a mut Vec<Arc<Middleware>>,
    ) -> Self {
        Context {
            request,
            middleware,
            response: Response::new(Body::empty()),
            route_endpoint,
            current_index: 0,
        }
    }

    pub fn next(self) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        if let Some(current) = self.middleware.get(self.current_index) {
            current.run(&self)
        } else {
            let res = ResponseBuilder::new();
            let route_response = (*self.route_endpoint)(self.request, res);
            route_response.into()
        }
    }
}
