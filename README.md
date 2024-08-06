# async_pg template repository

## Project template base on

- `tokio_postgres`
- use of `tokio_pg_mapper` for postgres data mapping
- `deadpool_postgres` for connection pooling
- `dotenv` + `config` for configuration

## Instructions

### NOTE:

1. Create database user
   ```sql
   CREATE USER test_user WITH PASSWORD 'testing';
   ```

2. Create database
   An alternative using SQL:
   ```sql
   CREATE DATABASE testing_db OWNER test_user;
   ```

3. Initialize database

   ```shell
   psql -f sql/schema.sql testing_db
   ```

4. Grant privileges to new user

   ```sql
   GRANT ALL PRIVILEGES ON SCHEMA testing_db TO test_user;
   GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA testing_db TO test_user;
   GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA testing_db TO test_user;
   ``` 

5. Create `.env` file:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   PG__USER=test_user
   PG__PASSWORD=testing
   PG__HOST=127.0.0.1
   PG__PORT=5432
   PG__DBNAME=testing_db
   PG__POOL_MAX_SIZE=16
   ```

6. Run the server:

   ```shell
   cargo run
   ```

7. Using a different terminal send an HTTP POST request to the running server:

   Send a ping:
   ```shell
   echo '{"value": "pong"}' | http -f --json --print h POST http://127.0.0.1:8080/new_ping
   ```

   Retrieve pings:
   ```shell
   http http://127.0.0.1:8080/get_pings
   ```
