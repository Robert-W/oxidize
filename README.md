<div align="center">
    <h2>oxidize</h2>
</div>

<p align="center">Small(ish) Rust Web Server for API development</p>

## Pre-requisites

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Docker](https://docs.docker.com/get-docker)

Next install the `sqlx-cli` using cargo.

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

## Getting Started

1. Run `docker compose up`
1. Run `cargo run`

## TODO

- [x] Implement Sample Service
- [x] Add tests directory
- [x] Setup tracing to replace logging
- [ ] Setup Dockerfile
