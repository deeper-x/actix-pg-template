use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use confik::{Configuration as _, EnvSource};

use dotenvy::dotenv;
use tokio_postgres::NoTls;

use crate::settings::config::ServerConfig;
use deadpool_postgres::Pool;

use env_logger;

mod settings {
    pub mod config;
    pub mod errors;
}
mod db {
    pub mod dml;
    pub mod models;
}

mod server {
    pub mod router;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. enable logging info level
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // 2. load dotenv
    dotenv().ok();

    // 3. build server configuration via config::Configuration implementations
    let config: ServerConfig = ServerConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    // 4. database setup, configuring pool
    let pool: Pool = config.pg.create_pool(None, NoTls).unwrap();

    // 5. http server instance setup, linking each scope to its routes
    let server: actix_web::dev::Server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                // ping default skeleton
                web::scope("/ping")
                    .route("/get", web::get().to(server::router::get_ping_records))
                    .route("/post", web::post().to(server::router::add_ping_record)),
            )
    })
    .bind(config.server_addr.clone())?
    .run();

    // 6. Output start string
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
