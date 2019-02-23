use futures::{Future, Stream};
use serde_derive::*;
use serde_json::Value;
use url::form_urlencoded;

use obsidian::{
    context::Context,
    header,
    middleware::Middleware,
    router::{RequestData, ResponseBuilder},
    App, Body, Request, Response, StatusCode,
};

// Testing example
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub struct BodyParser {}
pub struct UrlEncodedParser {}
pub struct Logger {}

impl BodyParser {
    pub fn new() -> Self {
        BodyParser {}
    }
}

impl Middleware for BodyParser {
    fn handle<'a>(
        &'a self,
        context: Context<'a>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let mut context = context;
        let (parts, body) = context.request.into_parts();

        let b = match body.concat2().wait() {
            Ok(chunk) => chunk,
            Err(e) => {
                println!("{}", e);
                hyper::Chunk::default()
            }
        };

        let json_result: serde_json::Result<Value> = serde_json::from_slice(&b);

        match json_result {
            Ok(json_body) => context.route_data.add_json(json_body),
            Err(e) => println!("{}", e),
        }

        let req = Request::from_parts(parts, Body::from(b));

        context.request = req;
        context.next()
    }
}

impl UrlEncodedParser {
    pub fn new() -> Self {
        UrlEncodedParser {}
    }
}

impl Middleware for UrlEncodedParser {
    fn handle<'a>(
        &'a self,
        context: Context<'a>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let mut context = context;
        let (parts, body) = context.request.into_parts();

        let b = match body.concat2().wait() {
            Ok(chunk) => chunk,
            Err(e) => {
                println!("{}", e);
                hyper::Chunk::default()
            }
        };

        let params_iter = form_urlencoded::parse(b.as_ref()).into_owned();

        for (key, value) in params_iter {
            context.route_data.add_param(key, value);
        }

        let req = Request::from_parts(parts, Body::from(b));

        context.request = req;
        context.next()
    }
}

impl Logger {
    pub fn new() -> Self {
        Logger {}
    }
}

impl Middleware for Logger {
    fn handle<'a>(
        &'a self,
        context: Context<'a>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        println!(
            "{} {} \n{}",
            context.request.method(),
            context.request.uri(),
            context
                .request
                .headers()
                .get("host")
                .unwrap()
                .to_str()
                .unwrap()
        );
        context.next()
    }
}

fn main() {
    let mut app = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |_req, res: ResponseBuilder| {
        res.status(StatusCode::OK).body("<!DOCTYPE html><html><head><link rel=\"shotcut icon\" href=\"favicon.ico\" type=\"image/x-icon\" sizes=\"32x32\" /></head> <h1>Hello Obsidian</h1></html>")
    });

    app.get("/json", |_req, res: ResponseBuilder| {
        let point = Point { x: 1, y: 2 };

        res.header(header::CONTENT_TYPE, "application/json")
            .status(StatusCode::OK)
            .json(point)
    });

    app.get("/empty-body", |_req, res: ResponseBuilder| {
        res.status(StatusCode::OK)
    });

    app.get("/vec", |_req, res: ResponseBuilder| {
        res.status(StatusCode::OK).body(vec![1, 2, 3])
    });

    app.get("/String", |_req, res: ResponseBuilder| {
        res.status(StatusCode::OK)
            .body("<h1>This is a String</h1>".to_string())
    });

    app.get("/paramtest", |_req, res: ResponseBuilder| {
        res.status(StatusCode::OK).send_file("./test.html")
    });

    app.post("/paramtest2", |_req: RequestData, res: ResponseBuilder| {
        let multi_test: Vec<String> = _req.params("test").into();
        let unique_test: String = _req.params("test2").into();
        let json_test = &_req.json["test_json"];

        for value in multi_test {
            println!("test / {}", value);
        }

        println!("test2 / {}", unique_test);
        println!("test_json / {}", json_test);

        res.status(StatusCode::OK).body("params result")
    });

    let body_parser = BodyParser::new();
    let logger = Logger::new();
    let url_parser = UrlEncodedParser::new();

    app.use_service(logger);
    app.use_service(url_parser);
    app.use_service(body_parser);

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    });
}
