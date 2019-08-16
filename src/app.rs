use crate::context::Context;
use crate::middleware::Middleware;
use crate::router::{EndPointHandler, Params, ResponseBuilder, Router};
use futures::{future, Future, Stream};
use hyper::{service::service_fn, Body, Request, Response, Server};
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;


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
            let route = match path.get_route(&parts.method) {
                Some(r) => r.to_owned(),
                None => return page_not_found(),
            };
            let middlewares = self.main_router.middlewares.to_owned();

            // Temporary used as the hyper stream thread block. async will be used soon
            Box::new(body.concat2().and_then(move |b| {
                let req = Request::from_parts(parts, Body::from(b));
                let context = Context::new(req, Params::default());
                let executor = EndpointExecutor::new(&route.handler, &middlewares);

                executor.next(context)
            }))
        } else {
            page_not_found()
        }
    }
}

pub fn page_not_found() -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    let server_response = Response::new(Body::from("404 Not Found"));

    Box::new(future::ok(server_response))
}

pub struct EndpointExecutor<'a> {
    pub route_endpoint: &'a Arc<dyn EndPointHandler<Output = ResponseBuilder>>,
    pub middleware: &'a [Arc<Middleware>],
}

impl<'a> EndpointExecutor<'a> {
    pub fn new(
        route_endpoint: &'a Arc<dyn EndPointHandler<Output = ResponseBuilder>>,
        middleware: &'a [Arc<Middleware>],
    ) -> Self {
        EndpointExecutor {
            route_endpoint,
            middleware,
        }
    }

    pub fn next(
        mut self,
        context: Context,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        if let Some((current, all_next)) = self.middleware.split_first() {
            self.middleware = all_next;
            current.handle(context, self)
        } else {
            let response_builder = ResponseBuilder::new();
            let route_response = (*self.route_endpoint)(context, response_builder);
            route_response.into()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hyper::StatusCode;

    #[test]
    fn test_app_server_resolve_endpoint() {
        let mut main_router = Router::new();

        main_router.get("/", |mut context: Context, res: ResponseBuilder| {
            let body = context.take_body();

            let request_body = body
                .map_err(|_| ())
                .fold(vec![], |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok(acc)
                })
                .and_then(|v| String::from_utf8(v).map_err(|_| ()));

            assert_eq!(context.uri().path(), "/");
            assert_eq!(request_body.wait().unwrap(), "test_app_server");
            res.status(StatusCode::OK).body("test_app_server")
        });

        let app_server = AppServer {
            sub_router: BTreeMap::new(),
            main_router,
        };

        let mut req_builder = Request::builder();

        let req = req_builder
            .uri("/")
            .body(Body::from("test_app_server"))
            .unwrap();

        let actual_response = app_server.resolve_endpoint(req).wait().unwrap();

        let mut expected_response = Response::new(Body::from("test_app_server"));
        *expected_response.status_mut() = StatusCode::OK;

        assert_eq!(actual_response.status(), expected_response.status());

        let actual_res_body = actual_response
            .into_body()
            .map_err(|_| ())
            .fold(vec![], |mut acc, chunk| {
                acc.extend_from_slice(&chunk);
                Ok(acc)
            })
            .and_then(|v| String::from_utf8(v).map_err(|_| ()));

        let expected_res_body = expected_response
            .into_body()
            .map_err(|_| ())
            .fold(vec![], |mut acc, chunk| {
                acc.extend_from_slice(&chunk);
                Ok(acc)
            })
            .and_then(|v| String::from_utf8(v).map_err(|_| ()));

        assert_eq!(
            actual_res_body.wait().unwrap(),
            expected_res_body.wait().unwrap()
        );
    }
}

