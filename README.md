<!-- EDIT /Users/z/rmw/rmw_config/README.md -->

# rmw_config : set / get config from config file and env

<a href="https://docs.rs/rmw_config"><img src="https://img.shields.io/badge/RUST-API%20DOC-blue?style=for-the-badge&logo=docs.rs&labelColor=333" alt="Api Doc"></a>

rmw_config : set / get config from config file and env

[→ examples/main.rs](examples/main.rs)

```rust
use rmw_config::config;

fn main() {
  config!(rmw);
  let mtu = get!(v4_mtu, 1440);
  dbg!(mtu);
}
```


## About

This project is part of **[rmw.link](//rmw.link)** Code Project

![rmw.link logo](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)
