extern crate obsidian;

use obsidian::{router::ObsidianResponse, App, StatusCode};

// Testing example

fn main() {
    let mut app = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |_req, res: ObsidianResponse| {
        res.status(StatusCode::OK).body("Hello Obsidian")
    });

    app.get("/empty-body", |_req, res: ObsidianResponse| {
        res.status(StatusCode::OK)
    });

    app.get("/String", |_req, res: ObsidianResponse| {
        res.status(StatusCode::OK)
            .body("<h1>This is a String</h1>".to_string())
    });

    app.listen(&addr);
}
