use std::net::SocketAddr;
use std::sync::Arc;

use futures::{future, Future, Stream};
use hyper::{service::service_fn, Body, Request, Response, Server, StatusCode};

use crate::context::Context;
use crate::middleware::Middleware;
use crate::router::{EndPointHandler, ResponseBuilder, Router};

pub struct App {
    router: Router,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        App {
            router: Router::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: impl EndPointHandler) {
        self.router.get(path, handler);
    }

    pub fn post(&mut self, path: &str, handler: impl EndPointHandler) {
        self.router.post(path, handler);
    }

    pub fn put(&mut self, path: &str, handler: impl EndPointHandler) {
        self.router.put(path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: impl EndPointHandler) {
        self.router.delete(path, handler);
    }

    /// Apply middleware in the provided route
    pub fn use_service_to(&mut self, path: &str, middleware: impl Middleware) {
        self.router.use_service_to(path, middleware);
    }

    /// Apply middleware in current relative route
    pub fn use_service(&mut self, middleware: impl Middleware) {
        self.router.use_service(middleware);
    }

    /// Apply route handler in current relative route
    pub fn use_router(&mut self, path: &str, router: Router) {
        self.router.use_router(path, router);
    }

    /// Serve static files by the virtual path as the route and directory path as the server file path
    pub fn use_static_to(&mut self, virtual_path: &str, dir_path: &str) {
        self.router.use_static_to(virtual_path, dir_path);
    }

    /// Serve static files by the directory path as the route and server file path
    pub fn use_static(&mut self, dir_path: &str) {
        self.router.use_static(dir_path);
    }

    pub fn listen(self, addr: &SocketAddr, callback: impl Fn()) {
        let app_server = AppServer {
            router: self.router,
        };

        let service = move || {
            let server_clone = app_server.clone();

            service_fn(
                move |req| -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
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
    router: Router,
}

impl AppServer {
    pub fn resolve_endpoint(
        &self,
        req: Request<Body>,
    ) -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let (parts, body) = req.into_parts();

        // Currently support only one router until radix tree complete.
        if let Ok(path) = self.router.search_route(parts.uri.path()) {
            // Temporary used as the hyper stream thread block. async will be used soon
            Box::new(body.concat2().and_then(move |b| {
                let route = match path.get_route(&parts.method) {
                    Some(r) => r,
                    None => return page_not_found(),
                };
                let middlewares = path.get_middleware();
                let params = path.get_params();
                let req = Request::from_parts(parts, Body::from(b));
                let context = Context::new(req, params);

                let executor = EndpointExecutor::new(&route.handler, middlewares);

                executor.next(context)
            }))
        } else {
            page_not_found()
        }
    }
}

fn page_not_found() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    let mut server_response = Response::new(Body::from("404 Not Found"));
    *server_response.status_mut() = StatusCode::NOT_FOUND;

    Box::new(future::ok(server_response))
}

pub struct EndpointExecutor<'a> {
    pub route_endpoint: &'a Arc<dyn EndPointHandler<Output = ResponseBuilder>>,
    pub middleware: &'a [Arc<dyn Middleware>],
}

impl<'a> EndpointExecutor<'a> {
    pub fn new(
        route_endpoint: &'a Arc<dyn EndPointHandler<Output = ResponseBuilder>>,
        middleware: &'a [Arc<dyn Middleware>],
    ) -> Self {
        EndpointExecutor {
            route_endpoint,
            middleware,
        }
    }

    pub fn next(
        mut self,
        context: Context,
    ) -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
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
    use futures::Stream;
    use hyper::StatusCode;

    #[test]
    fn test_app_server_resolve_endpoint() {
        let mut router = Router::new();

        router.get("/", |mut context: Context, res: ResponseBuilder| {
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

        let app_server = AppServer { router };

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
