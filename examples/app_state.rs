use obsidian::{context::Context, App};

#[derive(Clone)]
pub struct AppState {
    pub db_connection_string: String,
}

#[tokio::main]
async fn main() {
    let mut app: App<AppState> = App::new();

    app.set_app_state(AppState {
        db_connection_string: "localhost:1433".to_string(),
    });

    app.get("/", |ctx: Context| async move {
        let app_state = ctx.get::<AppState>().unwrap();
        let res = Some(format!(
            "connection string: {}",
            &app_state.db_connection_string
        ));

        res
    });

    app.listen(3000).await;
}
