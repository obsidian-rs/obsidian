use futures::{future, Future, Stream};
use hyper::{service::service_fn, Body, Request, Response, Server};
use std::collections::BTreeMap;
use std::net::SocketAddr;

use crate::context::Context;
use crate::middleware::Middleware;
use crate::router::{EndPointHandler, ResponseBuilder, RouteData, Router};

/// There are two level of router
/// - App level -> main_router, middleware for this level will be run for all endpoint
/// - Router level -> sub_router, smaller group of endpoint
pub struct App {
    sub_router: BTreeMap<String, Router>,
    main_router: Router,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            sub_router: BTreeMap::new(),
            main_router: Router::new(),
        };

        app.get("/favicon.ico", |_req, res: ResponseBuilder| {
            res.send_file("./favicon.ico")
        });

        app
    }

    pub fn get(&mut self, path: &str, handler: impl EndPointHandler) {
        self.main_router.get(path, handler);
    }

    pub fn post(&mut self, path: &str, handler: impl EndPointHandler) {
        self.main_router.post(path, handler);
    }

    pub fn put(&mut self, path: &str, handler: impl EndPointHandler) {
        self.main_router.put(path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: impl EndPointHandler) {
        self.main_router.delete(path, handler);
    }

    pub fn use_service(&mut self, middleware: impl Middleware) {
        self.main_router.add_service(middleware);
    }

    pub fn listen(self, addr: &SocketAddr, callback: impl Fn()) {
        let app_server = AppServer {
            sub_router: self.sub_router,
            main_router: self.main_router,
        };

        let service = move || {
            let server_clone = app_server.clone();

            service_fn(
                move |req| -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
                    // Resolve the route endpoint
                    server_clone.resolve_endpoint(req)
                },
            )
        };

        let server = Server::bind(&addr)
            .serve(service)
            .map_err(|e| eprintln!("server error: {}", e));

        callback();

        hyper::rt::run(server);
    }
}

#[derive(Clone)]
struct AppServer {
    sub_router: BTreeMap<String, Router>,
    main_router: Router,
}

impl AppServer {
    pub fn resolve_endpoint(
        &self,
        req: Request<Body>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let (parts, body) = req.into_parts();

        // Currently support only one router until radix tree complete.
        if let Some(ref path) = self.main_router.routes.get(parts.uri.path()) {
            // Temporary used to owned to move the variables for lifetime in and_then
            let route = path.get_route(&parts.method).unwrap().to_owned();
            let middlewares = self.main_router.middlewares.to_owned();

            // Temporary used as the hyper stream thread block. async will be used soon
            Box::new(body.concat2().and_then(move |b| {
                let req = Request::from_parts(parts, Body::from(b));
                let route_data = &mut RouteData::new();

                let context = Context::new(req, &route.handler, &middlewares, route_data);

                context.next()
            }))
        } else {
            let server_response = Response::new(Body::from("404 Not Found"));

            Box::new(future::ok(server_response))
        }
    }
}
