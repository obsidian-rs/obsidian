# Obsidian

## What's Obsidian?

`Obsidian` is a web-application framework in Rust with a vision to make Rust web development fun.

## Code Status

|                                    **Version**                                     |                                                                     **Master**                                                                      |
| :--------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------------------------------------------------------------: |
| [![](http://meritbadge.herokuapp.com/obsidian)](https://crates.io/crates/obsidian) | [![Actions Status](https://github.com/obsidian-rs/obsidian/workflows/Obsidian%20Action/badge.svg)](https://github.com/obsidian-rs/obsidian/actions) |

## Hello World
```rust
use obsidian::App;

fn main() {
  let mut app = App::new();
  let addr = ([127, 0, 0, 1], 3000).into();

  app.get("/", |_ctx| {
    "Hello World"
  });

  app.listen(&addr, || {
    println!("server is listening to {}", &addr);
  });
}
```

## Hello World (with handler function)
```rust
use obsidian::{App, Responder, context::Context};

fn hello_world(_ctx: Context) -> impl Responder {
  "Hello World"
}

fn main() {
  let mut app = App::new();
  let addr = ([127, 0, 0, 1], 3000).into();

  app.get("/", hello_world);

  app.listen(&addr, || {
    println!("server is listening to {}", &addr);
  });
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
