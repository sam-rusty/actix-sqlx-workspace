## Actix-SQLX workspace

### Setup Project

1. Install Rust (https://www.rust-lang.org/tools/install)
2. Make sure Postgres server is running.
3. Install sqlx-cli `cargo install sqlx-cli`
4. rename `.env.example` to `.env`
5. Run migrations `sqlx migrate run`
6. install cargo watch `cargo install cargo-watch`
7. prepare query cache for sqlx `cargo sqlx prepare --workspace`
8. Install Nextest for testig https://nexte.st/book/pre-built-binaries.html
9. Check `Makefile` to start app(s)
