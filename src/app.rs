use futures::{future, Future, Stream};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::net::SocketAddr;
use url::form_urlencoded;

use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};

use super::context::Context;
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
pub struct AppServer {
    sub_router: BTreeMap<String, Router>,
    main_router: Router,
}

impl AppServer {
    pub fn resolve_endpoint(
        &self,
        req: Request<Body>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let (parts, body) = req.into_parts();
        if let Some(path) = self.main_router.routes.clone().get(parts.uri.path()) {
            let route = path.get_route(&parts.method).unwrap().clone();
            let middlewares = self.main_router.middlewares.clone();

            // Get forms params from body
            Box::new(body.concat2().and_then(move |b| {
                let params = form_urlencoded::parse(b.as_ref())
                    .into_owned()
                    .collect::<HashMap<String, String>>();

                for (key, value) in &params {
                    println!("{} / {}", key, value);
                }

                let req = Request::from_parts(parts, Body::from(b));

                let context = Context::new(req, &route.handler, &middlewares, &params);

                context.next()
            }))
        } else {
            let server_response = Response::new(Body::from("404 Not Found"));

            Box::new(future::ok(server_response))
        }
    }
}
