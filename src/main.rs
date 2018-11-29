extern crate obsidian;

use obsidian::{router::Injection, App, Body, Builder};

fn main() {
    let mut app = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |req, res: &mut Builder| {
        res.body(Body::from("Hello Obsidian")).unwrap()
    });

    app.listen(&addr);
}
