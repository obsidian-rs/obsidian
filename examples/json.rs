use obsidian::{
    context::Context, error::ObsidianError, handler::ContextResult, router::Responder, App,
};
use serde::*;

async fn get_user(mut ctx: Context) -> Result<String, ObsidianError> {
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

    Ok("Hello".to_string())
}

#[tokio::main]
async fn main() {
    let mut app: App = App::default();

    app.get("/user", get_user);

    app.listen(3000).await;
}
