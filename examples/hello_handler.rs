use obsidian::{context::Context, router::Responder, App};

async fn hello_world(_ctx: Context) -> impl Responder {
    "Hello World"
}

#[tokio::main]
async fn main() {
    let mut app = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", hello_world);

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    })
    .await;
}
