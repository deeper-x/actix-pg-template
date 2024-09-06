use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{Row, Statement};

use crate::{db::models::Ping, settings::errors::MyError};

// retrieve ping records list
pub async fn get_ping_records(client: &Client) -> Result<Vec<Ping>, MyError> {
    let _stmt: &str = include_str!("./sql/ping/get_records.sql");
    let stmt: Statement = client.prepare(&_stmt).await.unwrap();

    let results: Vec<Ping> = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Ping::from_row_ref(row).unwrap())
        .collect::<Vec<Ping>>();

    Ok(results)
}

// add ping record
pub async fn add_ping_record(client: &Client, ping_info: Ping) -> Result<i64, MyError> {
    let _stmt: &str = include_str!("./sql/ping/add_record.sql");
    let stmt: Statement = client.prepare(&_stmt).await.unwrap();

    let rows: Vec<Row> = client.query(&stmt, &[&ping_info.value]).await.unwrap();

    let idx: i64 = rows[0].get(0);

    Ok(idx)
}
