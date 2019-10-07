use serde_derive::*;
use std::{fmt, fmt::Display};

use obsidian::{
    context::Context, header, middleware::Logger, router::ResponseBuilder, App, StatusCode,
};

// Testing example
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonTest {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ParamTest {
    test: Vec<String>,
    test2: String,
}

impl Display for JsonTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{title: {}, content: {}}}", self.title, self.content)
    }
}

fn main() {
    let mut app = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |_ctx, res: ResponseBuilder| {
        res.status(StatusCode::OK).body("<!DOCTYPE html><html><head><link rel=\"shotcut icon\" href=\"favicon.ico\" type=\"image/x-icon\" sizes=\"32x32\" /></head> <h1>Hello Obsidian</h1></html>")
    });

    app.get("/json", |_ctx, res: ResponseBuilder| {
        let point = Point { x: 1, y: 2 };

        res.header(header::CONTENT_TYPE, "application/json")
            .status(StatusCode::OK)
            .json(point)
    });

    app.get("/empty-body", |_ctx, res: ResponseBuilder| {
        res.status(StatusCode::OK)
    });

    app.get("/vec", |_ctx, res: ResponseBuilder| {
        res.status(StatusCode::OK).body(vec![1, 2, 3])
    });

    app.get("/String", |_ctx, res: ResponseBuilder| {
        res.status(StatusCode::OK)
            .body("<h1>This is a String</h1>".to_string())
    });

    app.get("/test/radix", |_ctx, res: ResponseBuilder| {
        res.status(StatusCode::OK)
            .body("<h1>Test radix</h1>".to_string())
    });

    app.get("/team/radix", |_ctx, res: ResponseBuilder| {
        res.status(StatusCode::OK)
            .body("<h1>Team radix</h1>".to_string())
    });

    app.get("/test/radix2", |_ctx, res: ResponseBuilder| {
        res.status(StatusCode::OK)
            .body("<h1>Test radix2</h1>".to_string())
    });

    app.get("/formtest", |_ctx, res: ResponseBuilder| {
        res.status(StatusCode::OK).send_file("./test.html")
    });

    app.get("/jsontest", |_ctx: Context, res: ResponseBuilder| {
        res.status(StatusCode::OK).send_file("./testjson.html")
    });

    app.post("/jsontestapi", |mut ctx: Context, res: ResponseBuilder| {
        let json: serde_json::Value = ctx.json().unwrap();

        println!("{}", json);

        res.status(StatusCode::OK).json(json)
    });

    app.post(
        "/jsonteststructapi",
        |mut ctx: Context, res: ResponseBuilder| {
            let json: JsonTest = ctx.json().unwrap();

            println!("{}", json);

            res.status(StatusCode::OK).json(json)
        },
    );

    app.post("/formtest", |mut ctx: Context, res: ResponseBuilder| {
        let param_test: ParamTest = ctx.form().unwrap();

        dbg!(&param_test);

        res.status(StatusCode::OK).json(param_test)
    });

    app.get("/paramtest/:id", |ctx: Context, res: ResponseBuilder| {
        let param_test: i32 = ctx.param("id").unwrap();

        dbg!(&param_test);

        res.status(StatusCode::OK).json(param_test)
    });

    let logger = Logger::new();

    app.use_service(logger);

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    });
}