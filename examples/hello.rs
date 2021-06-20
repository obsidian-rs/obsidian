use obsidian::{context::Context, App};

#[tokio::main]
async fn main() {
    let mut app: App = App::new();

    app.get("/", |ctx: Context| async { ctx.build("Hello World").ok() });

    app.listen(3000).await;
}
