use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "ping")]
pub struct Ping {
    pub value: String,
    pub ts_created: SystemTime,
}

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "migrations")]
pub struct Migration {
    pub id: Option<i64>,
    pub query: String,
    pub ts_created: Option<SystemTime>,
}

impl Migration {
    pub fn new(id: i64, query: String, ts_created: SystemTime) -> Migration {
        Migration {
            id: Some(id),
            query,
            ts_created: Some(ts_created),
        }
    }
}
