# Game API Service

## Requirements
- [SQLx CLI](https://crates.io/crates/sqlx-cli)

## SQLite Database Setup with SQLx
The SQLx CLI requires libssl, if you don't already have it you can install it with the following command:
```sh
sudo apt-get install libssl-dev
```

Next, install the SQLx CLI with Cargo if you don't already have it:
```sh
cargo install sqlx-cli
```

Run migrations to create and properly configure necessary tables:
```sh
sqlx migrate run
```
