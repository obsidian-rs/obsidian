use obsidian::{context::Context, App};
use obsidian::middleware::cookie_parser::CookieParser;
use cookie::Cookie;

#[tokio::main]
async fn main() {
    let mut app: App = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |ctx: Context| async { 
        if let Some(cookies) = ctx.cookie("cookie_name") {
            let cookies2 = ctx.cookie("cookie_name2").expect("cookie_name2 should be set");
            let response = format!("{}={}; {}={};", cookies.name(), cookies.value(), cookies2.name(), cookies2.value());

            ctx
            .build(response)
            .ok() 
        }
        else {
            ctx
            .build("Set cookies!")
            .with_cookie(Cookie::new("cookie_name", "cookie_value"))
            .with_cookie(Cookie::new("cookie_name2", "cookie_value2"))
            .ok() 
        }
    });

    let cookie_parser = CookieParser::new();
    app.use_service(cookie_parser);

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    })
    .await;
}
