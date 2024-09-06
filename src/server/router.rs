use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use models::Ping;

use crate::db::{dml, models};
use crate::settings;

// retrives ping records
pub async fn get_ping_records(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let pings: Vec<Ping> = dml::get_ping_records(&client).await?;

    Ok(HttpResponse::Ok().json(pings))
}

// insert ping record
pub async fn add_ping_record(
    ping: web::Json<models::Ping>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let ping_info: models::Ping = ping.into_inner();

    let client: Client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let new_ping: i64 = dml::add_ping_record(&client, ping_info).await?;

    Ok(HttpResponse::Ok().json(new_ping))
}
