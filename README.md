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
cargo test -- --test-threads=1
cargo run
```

## Overview of how it works

Use Rocket's [rocket_db_pools](https://api.rocket.rs/v0.5-rc/rocket_db_pools/index.html) to retrieve DB Pool connections from Controller arguments. This is passed as an argument to use_case, which passes the connection as an argument to each function in the repository. Only one connection is used per request, and it is returned to the Pool as soon as the request is processed.

### Mocking
Since use_case and repository are Trait, mock can be easily created by [mockall](https://github.com/asomers/mockall) when testing.

### DB Transaction
DB transactions are basically assumed to be used only within each function of repository, but transactions can be generated from DB Pool connections on the use_case side if necessary. However, since the functions in repository do not accept Transaction type, it is necessary to create a separate function dedicated to transactions. (sqlx0.6.0 uses [PoolConnection](https://docs.rs/sqlx/0.6.3/sqlx/pool/struct.PoolConnection.html) and [Transaction](https://docs.rs/sqlx/0.6.3/sqlx/struct.Transaction.html#) implements the [Executor](https://docs.rs/sqlx/0.6.3/sqlx/trait.Executor.html) trace. I could use this to create a function that accepts both, I stopped because it didn't work with mockall, but I'm thinking it might be possible by making it something like Box<dyn Executor>)

### DB Test
The DB test assumes that you `truncate` the table to be used before starting the test. Since `cargo test` is executed in parallel by default, if we do not use transactions, the timing of `truncate` etc. is unpredictable and the test results will change each time. Transactions are difficult to use because they cannot be passed as arguments to repository functions, as described above. Since I think the use of transaction is similar to a real serial process, I run the test as `cargo test -- --test-threads=1` so that the test result will be as expected.

## What I would like to do in the future
- I would like to include SeaORM.
- I want to be able to use transactions in my tests easily.
- Right now Rocket's sqlx is 0.6.0, but SeaORM is 0.7.0. Maybe we will stop using Rocket's db related libraries.