extern crate obsidian;

use obsidian::{header, router::ResponseBuilder, App, StatusCode};
use serde_derive::*;

// Testing example

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
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

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    });
}
