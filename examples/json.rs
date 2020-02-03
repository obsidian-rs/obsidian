use obsidian::{
    context::Context,
    router::{Responder, Response},
    App,
};
use serde::*;

async fn get_user(_ctx: Context) -> impl Responder {
    #[derive(Serialize, Deserialize)]
    struct User {
        name: String,
    };

    let user = User {
        name: String::from("Obsidian"),
    };
    Response::ok().json(user)
}

#[tokio::main]
async fn main() {
    let mut app = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/user", get_user);

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    })
    .await;
}
