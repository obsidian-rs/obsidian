use obsidian::{context::Context, handler::ContextResult, App};

async fn hello_world(ctx: Context) -> ContextResult {
    ctx.build("Hello World").ok()
}

#[tokio::main]
async fn main() {
    let mut app: App = App::new();

    app.get("/", hello_world);

    app.listen(3000).await;
}
