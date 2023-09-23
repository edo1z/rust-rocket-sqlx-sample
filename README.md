# rust-rocket-sqlx-sample

A clean architecture style Sample using Rust's Rocket, sqlx and PostgreSQL.

## How to Use

```shell
git clone https://github.com/net3i/rust-rocket-sqlx-sample
cd rust-rocket-sqlx-sample
docker-compose up -d
cp .env.example .env
sh migrate.sh
cargo test -- --test-threads=1
cargo run
```
