use obsidian::{context::Context, App, ContextResult};
use serde::*;

async fn get_user(ctx: Context) -> ContextResult {
    #[derive(Serialize, Deserialize)]
    struct User {
        name: String,
    }

    let user = User {
        name: String::from("Obsidian"),
    };
    ctx.build_json(user).ok()
}

#[tokio::main]
async fn main() {
    let mut app: App = App::new();

    app.get("/user", get_user);

    app.listen(3000).await;
}
