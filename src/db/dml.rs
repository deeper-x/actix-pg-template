use std::result;

use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Row;

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
