use std::sync::Arc;

use futures::future::Future;
use hyper::{Body, Request, Response};

use crate::middleware::Middleware;
use crate::router::{EndPointHandler, RequestData, ResponseBuilder, RouteData};

pub struct Context<'a> {
    pub request: Request<Body>,
    pub middleware: &'a [Arc<Middleware>],
    pub route_endpoint: &'a Arc<dyn EndPointHandler<Output = ResponseBuilder>>,
    pub route_data: &'a mut RouteData,
}

impl<'a> Context<'a> {
    pub fn new(
        request: Request<Body>,
        route_endpoint: &'a Arc<dyn EndPointHandler<Output = ResponseBuilder>>,
        middleware: &'a [Arc<Middleware>],
        route_data: &'a mut RouteData,
    ) -> Self {
        Context {
            request,
            middleware,
            route_endpoint,
            route_data,
        }
    }

    pub fn next(mut self) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        if let Some((current, all_next)) = self.middleware.split_first() {
            self.middleware = all_next;
            current.handle(self)
        } else {
            let response_builder = ResponseBuilder::new();
            let request_data = RequestData::new(self.request, self.route_data.clone());
            let route_response = (*self.route_endpoint)(request_data, response_builder);
            route_response.into()
        }
    }
}
