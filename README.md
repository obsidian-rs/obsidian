<p align="center">
  <a href="https://obsidian-rs.github.io">
    <img alt="Obsidian Logo" src=".github/media/logo.png" width="256">
  </a>
  <h1 align="center">
    Obsidian
    <a href="https://github.com/obsidian-rs/obsidian/actions">
      <img alt="GitHub Actions status" src="https://github.com/obsidian-rs/obsidian/workflows/Obsidian%20Action/badge.svg">
    </a>
    <a href="https://crates.io/crates/obsidian">
      <img alt="Obsidian crate" src="http://meritbadge.herokuapp.com/obsidian">
    </a>
  </h1>
</p>

## What's Obsidian?

`Obsidian` is a web-application framework in Rust with a vision to make Rust web development fun.

## Code Status

|                                    **Version**                                     |                                                                     **Master**                                                                      |
| :--------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------------------------------------------------------------: |
| [![](http://meritbadge.herokuapp.com/obsidian)](https://crates.io/crates/obsidian) | [![Actions Status](https://github.com/obsidian-rs/obsidian/workflows/Obsidian%20Action/badge.svg)](https://github.com/obsidian-rs/obsidian/actions) |

## Hello World
```rust
use obsidian::App;

#[tokio::main]
async fn main() {
    let mut app = App::new();
    let addr = ([127, 0, 0, 1], 3000).into();

    app.get("/", |_ctx| async { "Hello World" });

    app.listen(&addr, || {
        {
            println!("server is listening to {}", &addr);
        }
    }).await;
}
```

## Hello World (with handler function)
```rust
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
```

## Example Files

Example are located in `example/main.rs`.

## Run Example

```
cargo run --example example
```

## Current State

NOT READY FOR PRODUCTION!
