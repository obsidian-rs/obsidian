use serde::*;
use std::{fmt, fmt::Display};

use obsidian::{
    context::Context,
    middleware::Logger,
    router::{header, Responder, Response, Router},
    App, StatusCode,
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

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i8,
}

impl Display for JsonTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{title: {}, content: {}}}", self.title, self.content)
    }
}

// async fn responder_json(mut ctx: Context) -> impl Responder {
//     let person: Person = ctx.json().await?;

//     person.age += 1;

//     Ok(response::json(person))
// }

// async fn responder_obsidian_error(mut ctx: Context) -> impl Responder {
//     let json: JsonTest = ctx.json().await?;
//     println!("{}", json);
//     Ok(response::json(json, StatusCode::OK))
// }

// fn responder_with_header(_ctx: Context) -> impl Responder {
//     let headers = vec![
//         ("X-Custom-Header-4", "custom-value-4"),
//         ("X-Custom-Header-5", "custom-value-5"),
//     ];

//     "here"
//         .header("Content-Type", "application/json")
//         .header("X-Custom-Header", "custom-value")
//         .header("X-Custom-Header-2", "custom-value-2")
//         .header("X-Custom-Header-3", "custom-value-3")
//         .set_headers(headers)
//         .status(StatusCode::CREATED)
// }

#[tokio::main]
async fn main() {
    let mut app = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |_ctx| async {
Response::ok().html("<!DOCTYPE html><html><head><link rel=\"shotcut icon\" href=\"favicon.ico\" type=\"image/x-icon\" sizes=\"32x32\" /></head> <h1>Hello Obsidian</h1></html>")
    });

    app.get("/json", |_ctx| async {
        let point = Point { x: 1, y: 2 };

        Response::created()
            .set_header(header::AUTHORIZATION, "token")
            .set_header_str("X-Custom-Header", "Custom header value")
            .json(point)
    });

    app.get("/json-with-headers", |_ctx| async {
        let point = Point { x: 1, y: 2 };

        let custom_headers = vec![
            ("X-Custom-Header-1", "Custom header 1"),
            ("X-Custom-Header-2", "Custom header 2"),
            ("X-Custom-Header-3", "Custom header 3"),
        ];

        let standard_headers = vec![
            (header::AUTHORIZATION, "token"),
            (header::ACCEPT_CHARSET, "utf-8"),
        ];

        Response::created()
            .with_headers(standard_headers)
            .with_headers_str(custom_headers)
            .json(point)
    });

    app.get("/string-with-headers", |_ctx| async {
        let custom_headers = vec![
            ("X-Custom-Header-1", "Custom header 1"),
            ("X-Custom-Header-2", "Custom header 2"),
            ("X-Custom-Header-3", "Custom header 3"),
        ];

        let standard_headers = vec![
            (header::AUTHORIZATION, "token"),
            (header::ACCEPT_CHARSET, "utf-8"),
        ];

        "Hello World"
            .with_headers(standard_headers)
            .with_headers_str(custom_headers)
    });

    app.get("/empty-body", |_ctx| async { StatusCode::OK });

    app.get("/vec", |_ctx| async {
        vec![1, 2, 3].with_status(StatusCode::CREATED)
    });

    app.get("/String", |_ctx| async {
        "<h1>This is a String</h1>".to_string()
    });

    app.get("/test/radix", |_ctx| async {
        "<h1>Test radix</h1>".to_string()
    });

    app.get("/team/radix", |_ctx| async { "Team radix".to_string() });

    app.get("/test/radix2", |_ctx| async {
        "<h1>Test radix2</h1>".to_string()
    });

    app.get("/jsontest", |_ctx| async {
        Response::ok().file("./testjson.html").await
    });

    app.get("/jsan", |_ctx: Context| async {
        "<h1>jsan</h1>".to_string()
    });

    // app.post("/jsontestapi", |mut ctx: Context| {
    //     async move {
    //         let json: serde_json::Value = ctx.json().await?;

    //         println!("{}", json);

    //         Ok(response::json(json, StatusCode::OK))
    //     }
    // });

    // app.post("/jsonteststructapi", responder_obsidian_error);

    app.get("/test/wildcard/*", |ctx: Context| async move {
        format!(
            "{}<br>{}",
            "<h1>Test wildcard</h1>".to_string(),
            ctx.uri().path()
        )
    });

    app.get("router/test", |ctx: Context| async move {
        format!(
            "{}<br>{}",
            "<h1>router test get</h1>".to_string(),
            ctx.uri().path()
        )
    });
    app.post("router/test", |ctx: Context| async move {
        format!(
            "{}<br>{}",
            "<h1>router test post</h1>".to_string(),
            ctx.uri().path()
        )
    });
    app.put("router/test", |ctx: Context| async move {
        format!(
            "{}<br>{}",
            "<h1>router test put</h1>".to_string(),
            ctx.uri().path()
        )
    });
    app.delete("router/test", |ctx: Context| async move {
        format!(
            "{}<br>{}",
            "<h1>router test delete</h1>".to_string(),
            ctx.uri().path()
        )
    });

    app.get("route/diff_route", |ctx: Context| async move {
        format!(
            "{}<br>{}",
            "<h1>route diff get</h1>".to_string(),
            ctx.uri().path()
        )
    });

    let mut form_router = Router::new();

    form_router.get("/formtest", |_ctx| Response::ok().file("./test.html"));

    // form_router.post("/formtest", |mut ctx: Context| async move{
    //     let param_test: ParamTest = ctx.form().await?;

    //     dbg!(&param_test);

    //     Ok(response::json(param_test, StatusCode::OK))
    // });

    let mut param_router = Router::new();
    let logger = Logger::new();
    app.use_service(logger);

    // param_router.get("/paramtest/:id", |ctx: Context| async move {
    //     let param_test: i32 = ctx.param("id")?;

    //     dbg!(&param_test);

    //     Ok(response::json(param_test, StatusCode::OK))
    // });
    //
    // param_router.get("/paramtest/:id/test", |ctx: Context| async move {
    //     let mut param_test: i32 = ctx.param("id").unwrap();
    //     param_test = param_test * 10;

    //     dbg!(&param_test);

    //     Ok(response::json(param_test, StatusCode::OK))
    // });

    param_router.get("/test-next-wild/*", |_ctx| async {
        "<h1>test next wild</h1>".to_string()
    });

    param_router.get("/*", |_ctx| async {
        "<h1>404 Not Found</h1>"
            .to_string()
            .with_status(StatusCode::NOT_FOUND)
    });

    app.use_router("/params/", param_router);
    app.use_router("/forms/", form_router);
    app.use_static_to("/files/", "/assets/");

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    })
    .await;
}
