extern crate obsidian;

use obsidian::{header, router::ObsidianResponse, App, StatusCode};
use serde_derive::*;
use serde_json;

// Testing example

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut app = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |_req, res: ObsidianResponse| {
        res.status(StatusCode::OK).body("Hello Obsidian")
    });

    app.get("/json", |_req, res: ObsidianResponse| {
        let point = Point { x: 1, y: 2 };

        let serialized = serde_json::to_string(&point).unwrap();

        res.header(header::CONTENT_TYPE, "application/json")
            .status(StatusCode::OK)
            .body(serialized)
    });

    app.get("/empty-body", |_req, res: ObsidianResponse| {
        res.status(StatusCode::OK)
    });

    app.get("/vec", |_req, res: ObsidianResponse| {
        res.status(StatusCode::OK).body(vec![1, 2, 3])
    });

    app.get("/String", |_req, res: ObsidianResponse| {
        res.status(StatusCode::OK)
            .body("<h1>This is a String</h1>".to_string())
    });

    app.listen(&addr);
}
