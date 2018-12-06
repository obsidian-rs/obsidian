use futures::future;
use std::collections::BTreeMap;
use std::net::SocketAddr;

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server, StatusCode};

use super::router::{EndPointHandler, ObsidianResponse, Router};

pub struct App {
    sub_services: BTreeMap<String, Router>,
    main_router: Router,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            sub_services: BTreeMap::new(),
            main_router: Router::new(),
        };

        app.get("/favicon.ico", |_req, res: ObsidianResponse| {
            res.status(StatusCode::OK)
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

    pub fn listen(self, addr: &SocketAddr) {
        let server = AppServer {
            sub_services: self.sub_services,
            main_router: self.main_router,
        };

        let service = move || {
            let server_clone = server.clone();

            service_fn(
                move |req| -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
                    // Find the route
                    let res = ObsidianResponse::new();

                    if let Some(path) = server_clone.main_router.routes.get(req.uri().path()) {
                        // Get response
                        let route_response =
                            (path.get_route(&req.method()).unwrap().handler)(req, res);

                        // Convert into response
                        let server_response = route_response.into();

                        Box::new(future::ok(server_response))
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

        hyper::rt::run(server);
    }
}

#[derive(Clone)]
pub struct AppServer {
    sub_services: BTreeMap<String, Router>,
    main_router: Router,
}
