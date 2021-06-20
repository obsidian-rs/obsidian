mod middleware;

use middleware::logger_example::*;
use serde::*;
use std::{fmt, fmt::Display};

use obsidian::{
    context::Context,
    router::{header, Responder, Response, Router},
    App, ObsidianError, StatusCode,
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
struct User {
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

// fn responder_with_header(ctx: Context: Context) -> impl Responder {
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
    let mut app: App = App::new();

    app.get("/", |ctx: Context| async {
ctx.build(Response::ok().html("<!DOCTYPE html><html><head><link rel=\"shotcut icon\" href=\"favicon.ico\" type=\"image/x-icon\" sizes=\"32x32\" /></head> <h1>Hello Obsidian</h1></html>")).ok()
    });

    app.get("/json", |ctx: Context| async {
        let point = Point { x: 1, y: 2 };

        ctx.build_json(point)
            .with_status(StatusCode::OK)
            .with_header(header::AUTHORIZATION, "token")
            .with_header_str("X-Custom-Header", "Custom header value")
            .ok()
    });

    app.get("/user", |mut ctx: Context| async {
        #[derive(Serialize, Deserialize, Debug)]
        struct QueryString {
            id: String,
            status: String,
        }

        let params = match ctx.query_params::<QueryString>() {
            Ok(params) => params,
            Err(error) => {
                println!("error: {}", error);
                QueryString {
                    id: String::from(""),
                    status: String::from(""),
                }
            }
        };

        println!("params: {:?}", params);

        ctx.build("").ok()
    });

    app.patch("/patch-here", |ctx: Context| async {
        ctx.build("Here is patch request").ok()
    });

    app.get("/json-with-headers", |ctx: Context| async {
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

        ctx.build(
            Response::created()
                .with_headers(standard_headers)
                .with_headers_str(custom_headers)
                .json(point),
        )
        .ok()
    });

    app.get("/string-with-headers", |ctx: Context| async {
        let custom_headers = vec![
            ("X-Custom-Header-1", "Custom header 1"),
            ("X-Custom-Header-2", "Custom header 2"),
            ("X-Custom-Header-3", "Custom header 3"),
        ];

        let standard_headers = vec![
            (header::AUTHORIZATION, "token"),
            (header::ACCEPT_CHARSET, "utf-8"),
        ];

        ctx.build("Hello World")
            .with_headers(standard_headers)
            .with_headers_str(custom_headers)
            .ok()
    });

    app.get("/empty-body", |ctx: Context| async {
        ctx.build(StatusCode::OK).ok()
    });

    app.get("/vec", |ctx: Context| async {
        ctx.build(vec![1, 2, 3])
            .with_status(StatusCode::CREATED)
            .ok()
    });

    app.get("/String", |ctx: Context| async {
        ctx.build("<h1>This is a String</h1>".to_string()).ok()
    });

    app.get("/test/radix", |ctx: Context| async {
        ctx.build("<h1>Test radix</h1>".to_string()).ok()
    });

    app.get("/team/radix", |ctx: Context| async {
        ctx.build("Team radix".to_string()).ok()
    });

    app.get("/test/radix2", |ctx: Context| async {
        ctx.build("<h1>Test radix2</h1>".to_string()).ok()
    });

    app.get("/jsontest", |ctx: Context| async {
        ctx.build_file("./testjson.html").await.ok()
    });

    app.get("/jsan", |ctx: Context| async {
        ctx.build("<h1>jsan</h1>".to_string()).ok()
    });

    app.get("/test/wildcard/*", |ctx: Context| async move {
        let res = format!(
            "{}<br>{}",
            "<h1>Test wildcard</h1>".to_string(),
            ctx.uri().path()
        );

        ctx.build(res).ok()
    });

    app.get("router/test", |ctx: Context| async move {
        let result = ctx
            .extensions()
            .get::<LoggerExampleData>()
            .ok_or(ObsidianError::NoneError)?;

        dbg!(&result.0);

        let res = Some(format!(
            "{}<br>{}",
            "<h1>router test get</h1>".to_string(),
            ctx.uri().path()
        ));

        ctx.build(res).ok()
    });
    app.post("router/test", |ctx: Context| async move {
        let res = format!(
            "{}<br>{}",
            "<h1>router test post</h1>".to_string(),
            ctx.uri().path()
        );

        ctx.build(res).ok()
    });
    app.put("router/test", |ctx: Context| async move {
        let res = format!(
            "{}<br>{}",
            "<h1>router test put</h1>".to_string(),
            ctx.uri().path()
        );

        ctx.build(res).ok()
    });
    app.delete("router/test", |ctx: Context| async move {
        let res = format!(
            "{}<br>{}",
            "<h1>router test delete</h1>".to_string(),
            ctx.uri().path()
        );

        ctx.build(res).ok()
    });

    app.get("route/diff_route", |ctx: Context| async move {
        let res = format!(
            "{}<br>{}",
            "<h1>route diff get</h1>".to_string(),
            ctx.uri().path()
        );

        ctx.build(res).ok()
    });

    app.scope("admin", |router: &mut Router| {
        router.get("test", |ctx: Context| async move {
            ctx.build("Hello admin test").ok()
        });

        router.get("test2", |ctx: Context| async move {
            ctx.build("Hello admin test 2").ok()
        });
    });

    app.scope("form", |router: &mut Router| {
        router.get("/formtest", |ctx: Context| async move {
            ctx.build_file("/.test.html").await.ok()
        });
    });

    // form_router.post("/formtest", |mut ctx: Context| async move{
    //     let param_test: ParamTest = ctx.form().await?;

    //     dbg!(&param_test);

    //     Ok(response::json(param_test, StatusCode::OK))
    // });

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

    let logger_example = middleware::logger_example::LoggerExample::new();
    app.use_service(logger_example);

    app.scope("params", |router: &mut Router| {
        router.get("/test-next-wild/*", |ctx: Context| async {
            ctx.build("<h1>test next wild</h1>".to_string()).ok()
        });

        router.get("/*", |ctx: Context| async {
            ctx.build(
                "<h1>404 Not Found</h1>"
                    .to_string()
                    .with_status(StatusCode::NOT_FOUND),
            )
            .ok()
        });
    });

    app.use_static_to("/files/", "/assets/");

    app.listen(3000).await;
}
