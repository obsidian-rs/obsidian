use futures::future;
use std::collections::BTreeMap;
use std::net::SocketAddr;

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server};

use super::router::{EndPointHandler, Injection, Router};

fn send_response(
    req: Request<Body>,
) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    Box::new(future::ok(
        Response::builder().body(Body::from("hello")).unwrap(),
    ))
}

pub struct App {
    sub_services: BTreeMap<String, Router>,
    main_router: Router,
}

impl Injection for App {
    fn get(&mut self, path: &str, handler: impl EndPointHandler) {
        self.main_router.get(path, handler);
    }

    fn post(&mut self, path: &str, handler: impl EndPointHandler) {
        self.main_router.post(path, handler);
    }

    fn put(&mut self, path: &str, handler: impl EndPointHandler) {
        self.main_router.put(path, handler);
    }

    fn delete(&mut self, path: &str, handler: impl EndPointHandler) {
        self.main_router.delete(path, handler);
    }
}

impl App {
    pub fn new() -> Self {
        App {
            sub_services: BTreeMap::new(),
            main_router: Router::new(),
        }
    }

    pub fn listen(self, addr: &SocketAddr) {
        let server = AppServer {
            sub_services: self.sub_services,
            main_router: self.main_router,
        };

        let service = move || {
            let ser = server.clone();

            service_fn(
                move |req| -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
                    // Find the route
                    // send_response(req)
                    let mut res = Response::builder();
                    let path = ser.main_router.routes.get("/").unwrap();

                    Box::new(future::ok((path.get_route(&Method::GET).unwrap().handler)(
                        req, &mut res,
                    )))
                },
            )
        };

        let server = Server::bind(&addr)
            .serve(service)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }

    fn use_service(&mut self) {
        // middleware
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct AppServer {
    sub_services: BTreeMap<String, Router>,
    main_router: Router,
}
