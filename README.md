# rust-rocket-sqlx-sample

![GitHub](https://img.shields.io/github/license/net3i/rust-rocket-sqlx-sample)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/net3i/rust-rocket-sqlx-sample)
![GitHub last commit (by committer)](https://img.shields.io/github/last-commit/net3i/rust-rocket-sqlx-sample)
![X (formerly Twitter) Follow](https://img.shields.io/twitter/follow/edo1z)


A clean architecture style Sample using Rust's [Rocket](https://rocket.rs/)(v0.5), [sqlx](https://github.com/launchbadge/sqlx)(v0.6) and PostgreSQL.

## How to Use

```shell
git clone https://github.com/edo1z/rust-rocket-sqlx-sample
cd rust-rocket-sqlx-sample
docker-compose up -d
cp .env.example .env
sh migrate.sh
cargo test
cargo run
```

## Features
- Tests for controller, use_case, and repository are created using [mockall](https://github.com/asomers/mockall).
- In the repository integration tests, transactions are created and rolled back in each function, enabling parallel execution of tests.
- In the repository, `DbRepoError` is returned and converted to `AppError` in use_case. `AppError` corresponds to Rocket's [responder](https://api.rocket.rs/v0.5/rocket/response/trait.Responder.html), so it can be used as a response as is.
- When an error occurs in the repository, an error log is output. Error log output uses [tracing](https://github.com/tokio-rs/tracing/tree/v0.1.x), and outputs the circumstances leading up to the log, file path, and line number. Also, the values of each function's arguments can be logged.
- When logging, the conversion from `sqlx::Error` etc. to `DbRepoError` is also performed, so the dedicated macro `log_into!` is used.
- AppError also has error creation macros like anyhow, `app_err!`, `app_err_bail!`, `app_err_ensure!`.

## Error Log Output

By using the features of [tracing](https://github.com/tokio-rs/tracing/tree/v0.1.x) and adding the [instrument attribute](https://docs.rs/tracing/latest/tracing/attr.instrument.html) to the function you want to output, the function will be output in the course of error occurrence. The arguments of the function can also be output. To prevent important information from leaking into the log, we generally use [skip_all](https://docs.rs/tracing/latest/tracing/attr.instrument.html#skipping-fields) to hide all arguments and explicitly display the arguments you want to show. Also, by using the `log_into!` macro, the file name and line number of the error occurrence are also output.

```rust
#[instrument(name = "user_repo/update", skip_all, fields(id = %id))]
```

### Example of Error Log Output

```log
2023-12-05T07:43:24.644738Z ERROR user_controller/update{id=12}:user_use_case/update{id=12}:user_repo/update{id=12}: rust_rocket_sqlx_sample::repositories::user_repo: [DbRepoError::SqlxError] no rows returned by a query that expected to return at least one row (src/repositories/user_repo.rs:85)
```

