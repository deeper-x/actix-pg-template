use deadpool_postgres::Client;
use std::time::SystemTime;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{db::models::Migration, db::models::Ping, settings::errors::MyError};

// retrieve ping records list
pub async fn get_ping_records(client: &Client) -> Result<Vec<Ping>, MyError> {
    let _stmt = include_str!("./sql/ping/get_records.sql");
    let _stmt = _stmt.replace("$table_fields", &Ping::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Ping::from_row_ref(row).unwrap())
        .collect::<Vec<Ping>>();

    Ok(results)
}

pub async fn get_migration_records(client: &Client) -> Result<Vec<Migration>, MyError> {
    let stmt = client
        .prepare("SELECT id, query, ts_created FROM migrations;")
        .await
        .unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Migration::from_row_ref(row).unwrap())
        .collect::<Vec<Migration>>();

    Ok(results)
}

pub async fn get_migration_record(
    client: &Client,
    id_migration: i64,
) -> Result<Migration, MyError> {
    let stmt = client
        .prepare("SELECT id, query, ts_created FROM migrations WHERE id = $1;")
        .await
        .unwrap();

    let row: tokio_postgres::Row = client.query_one(&stmt, &[&id_migration]).await?;

    let id: i64 = row.get("id");
    let query: String = row.get("query");
    let ts_created: SystemTime = row.get("ts_created");

    let m: Migration = Migration::new(id, query, ts_created);

    Ok(m)
}

pub async fn add_migration_record(
    client: &Client,
    migration: Migration,
) -> Result<tokio_postgres::Row, tokio_postgres::Error> {
    let stmt = client
        .prepare("INSERT INTO migrations (query) VALUES ($1) RETURNING id;")
        .await
        .unwrap();

    client.query_one(&stmt, &[&migration.query]).await
}

pub async fn update_migration_record(
    client: &Client,
    migration: Migration,
) -> Result<tokio_postgres::Row, tokio_postgres::Error> {
    let stmt = client
        .prepare("UPDATE migrations set query = $1 where id = $2 RETURNING id;")
        .await
        .unwrap();

    client
        .query_one(&stmt, &[&migration.query, &migration.id])
        .await
}

// add ping record
pub async fn add_ping_record(client: &Client, ping_info: Ping) -> Result<Ping, MyError> {
    let _stmt = include_str!("./sql/ping/add_record.sql");
    let _stmt = _stmt.replace("$table_fields", &Ping::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&ping_info.value])
        .await?
        .iter()
        .map(|row| Ping::from_row_ref(row).unwrap())
        .collect::<Vec<Ping>>()
        .pop()
        .ok_or(MyError::NotFound)
}
