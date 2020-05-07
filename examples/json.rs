use obsidian::{context::Context, handler::ContextResult, App};
use serde::*;

async fn get_user(ctx: Context) -> ContextResult {
    #[derive(Serialize, Deserialize)]
    struct User {
        name: String,
    };

    let user = User {
        name: String::from("Obsidian"),
    };
    ctx.build_json(user).ok()
}

#[tokio::main]
async fn main() {
    let mut app: App = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/user", get_user);

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    })
    .await;
}
