# rust-rocket-sqlx-sample

A clean architecture style Sample using Rust's Rocket, sqlx and PostgreSQL.

## How to Use

```shell
git clone https://github.com/net3i/rust-rocket-sqlx-sample
cd rust-rocket-sqlx-sample
docker-compose up -d
cargo test
cargo run
```

## Notes

- This project does not include migration files. Please create the users and products tables yourself, using the following Rust structs as a reference:

```rust
pub struct Product {
    pub id: i32,
    pub name: String,
}

pub struct User{
    pub id: i32,
    pub name: String,
}
```