extern crate obsidian;

use futures::future::Future;
use hyper::{Body, Response};
use obsidian::{header, router::ResponseBuilder, App, Context, Middleware, StatusCode};
use serde_derive::*;

// Testing example

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub struct BodyParser {
    body: String,
}

impl BodyParser {
    pub fn new(body: String) -> Self {
        BodyParser { body }
    }
}

impl Middleware for BodyParser {
    fn run<'a>(
        &'a self,
        context: Context<'a>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        println!("{}", self.body);
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

    let body_parser = BodyParser::new("body parser".to_string());
    let body_parser_test2 = BodyParser::new("body_parser_test2".to_string());

    app.use_service(body_parser);
    app.use_service(body_parser_test2);

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    });
}
