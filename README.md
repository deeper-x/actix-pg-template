# Rust async_pg template repository
[![Rust](https://github.com/deeper-x/actix-pg-template/actions/workflows/rust.yml/badge.svg)](https://github.com/deeper-x/actix-pg-template/actions/workflows/rust.yml)

## Project template base on

- `tokio_postgres`
- use of `tokio_pg_mapper` for postgres data mapping
- `deadpool_postgres` for connection pooling
- `dotenv` + `config` for configuration


   ```shell
   cargo run
   ```

Using a different terminal send an HTTP POST request to the running server:

   Send a ping:
   ```shell
   echo '{"value": "pong"}' | http -f --json --print h POST http://127.0.0.1:8080/ping/post
   ```

   Retrieve pings:
   ```shell
   http http://127.0.0.1:8080/ping/get
   ```

Migrations list:
```sh
http http://127.0.0.1:8080/migration/all
```

Migration info:
```sh
http http://127.0.0.1:8080/migration/details/14
```

Update migration:

```sh
echo '{"id": 14, "query": "demo query edited from cli"}' | http -f --json --print hb http://127.0.0.1:8080/migration/update 
```