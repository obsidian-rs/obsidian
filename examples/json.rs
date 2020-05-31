use obsidian::{context::Context, handler::ContextResult, router::Response, App};
use serde::*;

async fn get_user(mut ctx: Context) -> ContextResult {
    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        name: String,
    };

    #[derive(Serialize, Deserialize, Debug)]
    struct UserParam {
        name: String,
        age: i8,
    };

    let user: UserParam = ctx.json().await?;
    println!("user: {:?}", user);

    let user = User {
        name: String::from("Obsidian"),
    };

    Ok(Response::ok().json(user))
}

#[tokio::main]
async fn main() {
    let mut app: App = App::default();

    app.get("/user", get_user);

    app.listen(3000).await;
}
