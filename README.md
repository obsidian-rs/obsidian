<p align="center">
  <a href="https://obsidian-rs.github.io">
    <img alt="Obsidian Logo" src=".github/media/logo.png" width="450">
  </a>
  <h1 align="center">
    Obsidian
  </h1>
</p>

<p align="center"><strong>Obsidian</strong> is an ergonomic Rust async http framework for reliable and efficient web.</p>

<div align="center">
    <a href="https://crates.io/crates/obsidian">
      <img alt="Obsidian crate" src="https://img.shields.io/crates/v/obsidian.svg">
    </a>
    <a href="https://github.com/obsidian-rs/obsidian/actions">
      <img alt="GitHub Actions status" src="https://github.com/obsidian-rs/obsidian/workflows/Obsidian%20Action/badge.svg">
    </a>
</div>

## Hello World

```rust
use obsidian::{context::Context, App};

#[tokio::main]
async fn main() {
    let mut app: App = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |ctx: Context| async { ctx.build("Hello World").ok() });

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    }).await;
}
```

## Hello World (with handler function)

```rust
use obsidian::{context::Context, App, ContextResult};

async fn hello_world(ctx: Context) -> ContextResult {
    ctx.build("Hello World").ok()
}


#[tokio::main]
async fn main() {
    let mut app: App = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", hello_world);

    app.listen(&addr, || {
        println!("server is listening to {}", &addr);
    })
    .await;
}
```

## JSON Response

```rust
use obsidian::{context::Context, App, ContextResult};
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

```

## Example Files

Example are located in `example/main.rs`.

## Run Example

```
cargo run --example example
```

## Current State

NOT READY FOR PRODUCTION YET!
