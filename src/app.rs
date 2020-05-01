use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};

use crate::context::Context;
use crate::error::ObsidianError;
use crate::middleware::Middleware;
use crate::router::{ContextResult, Handler, RouteValueResult, Router};

#[derive(Clone)]
pub struct DefaultAppState {}

#[derive(Default)]
pub struct App<T = DefaultAppState>
where
    T: Clone + Send + Sync + 'static,
{
    router: Router,
    app_state: Option<T>,
}

impl<T> App<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        App {
            router: Router::new(),
            app_state: None,
        }
    }

    pub fn get(&mut self, path: &str, handler: impl Handler) {
        self.router.get(path, handler);
    }

    pub fn post(&mut self, path: &str, handler: impl Handler) {
        self.router.post(path, handler);
    }

    pub fn put(&mut self, path: &str, handler: impl Handler) {
        self.router.put(path, handler);
    }

    pub fn patch(&mut self, path: &str, handler: impl Handler) {
        self.router.patch(path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: impl Handler) {
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

    /// Set app state. The app state must impl Clone.
    /// The app state will be passed into endpoint handler context dynamic data.
    ///
    /// # Example
    /// ```
    /// use obsidian::App;
    ///
    /// #[derive(Clone)]
    /// struct AppState {
    ///     db_connection: String,
    /// }
    ///
    /// let mut app: App<AppState> = App::new();
    /// app.set_app_state(AppState{
    ///     db_connection: "localhost:1433".to_string(),
    /// });
    /// ```
    pub fn set_app_state(&mut self, app_state: T) {
        self.app_state = Some(app_state);
    }

    pub async fn listen(self, addr: &SocketAddr, callback: impl Fn()) {
        let app_server: AppServer = AppServer {
            router: self.router,
        };
        let app_state = self.app_state;

        let service = make_service_fn(move |_| {
            let server_clone = app_server.clone();
            let app_state = app_state.clone();

            async {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    let route_value = server_clone.router.search_route(req.uri().path());

                    AppServer::resolve_endpoint(req, route_value, app_state.clone())
                }))
            }
        });

        let server = Server::bind(&addr).serve(service);

        callback();

        server.await.map_err(|_| println!("Server error")).unwrap();
    }
}

#[derive(Clone)]
struct AppServer {
    router: Router,
}

impl AppServer {
    pub async fn resolve_endpoint<T>(
        req: Request<Body>,
        route_value: Option<RouteValueResult>,
        app_state: Option<T>,
    ) -> Result<Response<Body>, hyper::Error>
    where
        T: Send + Sync + 'static,
    {
        match route_value {
            Some(route_value) => {
                let route = match route_value.get_route(req.method()) {
                    Some(r) => r,
                    None => return Ok::<_, hyper::Error>(page_not_found()),
                };
                let middlewares = route_value.get_middlewares();
                let params = route_value.get_params();
                let mut context = Context::new(req, params);
                let executor = EndpointExecutor::new(&route.handler, middlewares);

                if let Some(state) = app_state {
                    context.add::<T>(state);
                }

                let route_result = executor.next(context).await;

                let route_response = match route_result {
                    Ok(ctx) => {
                        let mut res = Response::builder();
                        if let Some(response) = ctx.take_response() {
                            if let Some(headers) = response.headers() {
                                if let Some(response_headers) = res.headers_mut() {
                                    headers.iter().for_each(move |(key, value)| {
                                        if let Ok(header_value) =
                                            header::HeaderValue::from_str(value)
                                        {
                                            response_headers.insert(key, header_value);
                                        }
                                    });
                                }
                            }

                            if let Some(cookies) = response.cookies() {
                                if let Some(response_headers) = res.headers_mut() {
                                    cookies.iter().for_each(move |cookie| {
                                        if let Ok(header_value) =
                                            header::HeaderValue::from_str(&cookie.to_string())
                                        {
                                            if response_headers.contains_key(header::SET_COOKIE) {
                                                response_headers
                                                    .append(header::SET_COOKIE, header_value);
                                            } else {
                                                response_headers
                                                    .insert(header::SET_COOKIE, header_value);
                                            }
                                        }
                                    });
                                }
                            }

                            res.status(response.status()).body(response.body())
                        } else {
                            // No response found
                            res.status(StatusCode::OK).body(Body::from(""))
                        }
                    }
                    Err(err) => {
                        let body = Body::from(err.to_string());
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(body)
                    }
                };

                Ok::<_, hyper::Error>(route_response.unwrap_or_else(|_| {
                    internal_server_error(ObsidianError::GeneralError(
                        "Error while constructing response body".to_string(),
                    ))
                }))
            }
            _ => Ok::<_, hyper::Error>(page_not_found()),
        }
    }
}

fn page_not_found() -> Response<Body> {
    let mut server_response = Response::new(Body::from("404 Not Found"));
    *server_response.status_mut() = StatusCode::NOT_FOUND;

    server_response
}

fn internal_server_error(err: impl std::error::Error) -> Response<Body> {
    let body = Body::from(err.to_string());
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(body)
        .unwrap()
}

pub struct EndpointExecutor<'a> {
    pub route_endpoint: &'a Arc<dyn Handler>,
    pub middleware: &'a [Arc<dyn Middleware>],
}

impl<'a> EndpointExecutor<'a> {
    pub fn new(
        route_endpoint: &'a Arc<dyn Handler>,
        middleware: &'a [Arc<dyn Middleware>],
    ) -> Self {
        EndpointExecutor {
            route_endpoint,
            middleware,
        }
    }

    pub async fn next(mut self, context: Context) -> ContextResult {
        if let Some((current, all_next)) = self.middleware.split_first() {
            self.middleware = all_next;
            current.handle(context, self).await
        } else {
            self.route_endpoint.call(context).await
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use async_std::task;
    use hyper::{body, body::Buf, StatusCode};

    #[test]
    fn test_app_server_resolve_endpoint() {
        task::block_on(async {
            let mut router = Router::new();

            router.get("/", |mut ctx: Context| async move {
                let body = ctx.take_body();

                let request_body = match body::aggregate(body).await {
                    Ok(buf) => String::from_utf8(buf.bytes().to_vec()),
                    _ => {
                        panic!();
                    }
                };

                assert_eq!(ctx.uri().path(), "/");
                assert_eq!(request_body.unwrap(), "test_app_server");
                ctx.build("test_app_server").ok()
            });

            let app_server = AppServer { router };

            let req_builder = Request::builder();

            let req = req_builder
                .uri("/")
                .body(Body::from("test_app_server"))
                .unwrap();

            let route_value = app_server.router.search_route(req.uri().path());
            let actual_response =
                AppServer::resolve_endpoint::<DefaultAppState>(req, route_value, None)
                    .await
                    .unwrap();

            let mut expected_response = Response::new(Body::from("test_app_server"));
            *expected_response.status_mut() = StatusCode::OK;

            assert_eq!(actual_response.status(), expected_response.status());

            let actual_res_body = match body::aggregate(actual_response).await {
                Ok(buf) => String::from_utf8(buf.bytes().to_vec()),
                _ => panic!(),
            };

            let expected_res_body = match body::aggregate(expected_response).await {
                Ok(buf) => String::from_utf8(buf.bytes().to_vec()),
                _ => panic!(),
            };

            assert_eq!(actual_res_body.unwrap(), expected_res_body.unwrap());
        })
    }
}
