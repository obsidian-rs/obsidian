use serde_derive::*;

use obsidian::{
    context::Context,
    header,
    middleware::{BodyParser, Logger, UrlEncodedParser},
    router::{Responder, ResponseResult},
    App, StatusCode,
};

// Testing example
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

fn responsder_handler(_ctx: Context) -> impl Responder {
    "Testing"
}

fn responsder_result(_ctx: Context) -> Result<String, String> {
    Err(String::from("This is an error"))
}

fn responder_string(_ctx: Context) -> String {
    String::from("Testing for string")
}

fn responder_str(_ctx: Context) -> impl Responder {
    "Testing for str"
}

fn responder_result_string(_ctx: Context) -> Result<String, String> {
    // Ok(String::from("Testing for string"))
    Err(String::from("Testing for string error"))
}

fn responder_option(_ctx: Context) -> Option<String> {
    None
    // Some("Testing for option")
}

// fn responder_json(ctx: Context) -> impl Responder {
//     let user = User {
//         name: String::from("Jun Kai"),
//         age: 26
//     };
//     ctx.json(user)
// }

fn main() {
    let mut app = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/responder-string", responder_string);
    app.get("/responder-str", responder_str);
    app.get("/responder-result-string", responder_result_string);
    app.get("/responder-option", responder_option);

    app.get("/", |_ctx| {
        "<!DOCTYPE html><html><head><link rel=\"shotcut icon\" href=\"favicon.ico\" type=\"image/x-icon\" sizes=\"32x32\" /></head> <h1>Hello Obsidian</h1></html>"
    });

    app.get("/empty-body", |_ctx| ());

    // app.get("/vec", |_ctx| {
    //     vec![1, 2, 3]
    // });
    //
    // app.get("/json", |_ctx, res: ResponseBuilder| {
    //     let point = Point { x: 1, y: 2 };
    //
    //     res.header(header::CONTENT_TYPE, "application/json")
    //         .status(StatusCode::OK)
    //         .json(point)
    // });
    //
    // app.get("/String", |_ctx, res: ResponseBuilder| {
    //     res.status(StatusCode::OK)
    //         .body("<h1>This is a String</h1>".to_string())
    // });
    //
    // app.get("/paramtest", |_ctx, res: ResponseBuilder| {
    //     res.status(StatusCode::OK).send_file("./test.html")
    // });
    //
    // app.post("/paramtest2", |ctx: Context, res: ResponseBuilder| {
    //     let multi_test: Vec<String> = ctx.params("test").into();
    //     let unique_test: String = ctx.params("test2").into();
    //     let json_test = &ctx.json["test_json"];
    //
    //     for value in multi_test {
    //         println!("test / {}", value);
    //     }
    //
    //     println!("test2 / {}", unique_test);
    //     println!("test_json / {}", json_test);
    //
    //     res.status(StatusCode::OK).body("params result")
    // });

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
