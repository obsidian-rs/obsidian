use obsidian::{context::Context, App};
use obsidian::context::memory_session::MemorySessionStorage;
use obsidian::middleware::{cookie_parser::CookieParser, cookie_session::CookieSession};

#[tokio::main]
async fn main() {
    let mut app: App = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |mut ctx: Context| async { 
        let session_content = match ctx.session() {
            Some(session) => {
                session.get("test").unwrap()
            },
            _ => "",
        }.to_owned();
        
        ctx.session_set("test", "session is set!");
        ctx.build(session_content).ok()
    });

    let cookie_parser = CookieParser::new();
    let cookie_session = CookieSession::new(MemorySessionStorage::new());

    app.use_service(cookie_parser);
    app.use_service(cookie_session);

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    })
    .await;
}
