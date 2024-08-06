use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{db::models::Ping, settings::errors::MyError};

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
