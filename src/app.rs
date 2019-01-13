use futures::future;
use std::collections::BTreeMap;
use std::net::SocketAddr;

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server};

use super::middleware::Middleware;
use super::router::{EndPointHandler, ResponseBuilder, Router};

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
                    // Find the route
                    let res = ResponseBuilder::new();

                    // Run middleware
                    //server_clone.run_middleware(req);

                    if let Some(path) = server_clone.main_router.routes.get(req.uri().path()) {
                        // Get response
                        let route_response =
                            (path.get_route(&req.method()).unwrap().handler)(req, res);

                        // Convert into response
                        let future_response = route_response.into();

                        future_response
                    } else {
                        let server_response = Response::new(Body::from("404 Not Found"));

                        Box::new(future::ok(server_response))
                    }
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
pub struct AppServer {
    sub_router: BTreeMap<String, Router>,
    main_router: Router,
}

impl AppServer {
    /*pub fn resolve_endpoint(
        &self,
        req: Request<Body>,
        route: Route,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let context = Context.new(req, route.handler, main_router);
    }*/
}
