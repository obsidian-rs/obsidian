use obsidian::{context::Context, App, ContextResult};

async fn hello_world(ctx: Context) -> ContextResult {
    ctx.build("Hello World").ok()
}

#[tokio::main]
async fn main() {
    let mut app: App = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", hello_world);

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    })
    .await;
}
