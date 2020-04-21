use obsidian::{context::Context, App};

#[tokio::main]
async fn main() {
    let mut app: App = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |ctx: Context| async { ctx.build("Hello World").ok() });

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    })
    .await;
}
