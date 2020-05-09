use obsidian::{context::Context, App, ObsidianError};

#[derive(Clone)]
pub struct AppState {
    pub db_connection_string: String,
}

#[tokio::main]
async fn main() {
    let mut app: App<AppState> = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.set_app_state(AppState {
        db_connection_string: "localhost:1433".to_string(),
    });

    app.get("/", |ctx: Context| async {
        let app_state = ctx.get::<AppState>().ok_or(ObsidianError::NoneError)?;
        let res = Some(format!(
            "connection string: {}",
            &app_state.db_connection_string
        ));

        ctx.build(res).ok()
    });

    app.listen(3000).await;
}
