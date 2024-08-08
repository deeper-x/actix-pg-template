use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::db::{dml, models};
use crate::settings;

// retrives ping records
pub async fn get_ping_records(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let pings = dml::get_ping_records(&client).await?;

    Ok(HttpResponse::Ok().json(pings))
}

pub async fn get_migration_records(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let migrations = dml::get_migration_records(&client).await?;

    Ok(HttpResponse::Ok().json(migrations))
}

pub async fn add_migration_record(
    migration: web::Json<models::Migration>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let migration_info = migration.into_inner();

    let client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let new_migration: Result<tokio_postgres::Row, tokio_postgres::Error> =
        dml::add_migration_record(&client, migration_info).await;

    match new_migration {
        Ok(it) => {
            let id: i64 = it.get(0);
            Ok(HttpResponse::Ok().json(id))
        }

        Err(err) => {
            println!("Error on adding migration: {}", err);
            Ok(HttpResponse::InternalServerError().json("todo_err"))
        }
    }
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

    let new_ping = dml::add_ping_record(&client, ping_info).await?;

    Ok(HttpResponse::Ok().json(new_ping))
}
