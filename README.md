<!-- EDIT /Users/z/rmw/rmw_logger/README.md -->

# rmw_log : use fern config display log

<a href="https://docs.rs/rmw_log"><img src="https://img.shields.io/badge/RUST-API%20DOC-blue?style=for-the-badge&logo=docs.rs&labelColor=333" alt="Api Doc"></a>

[→ examples/main.rs](examples/main.rs)

```rust
use log::info;

fn main() {
  rmw_log::init()
    .level_for("surf", log::LevelFilter::Warn)
    .apply()
    .unwrap();
  info!("test");
}
```


## About

This project is part of **[rmw.link](//rmw.link)** Code Project

![rmw.link logo](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)
