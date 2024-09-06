use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "ping")]
pub struct Ping {
    pub value: String,
    pub ts_created: Option<SystemTime>,
}
