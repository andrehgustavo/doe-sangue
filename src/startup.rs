//! src/startup.rs
// use crate::routes::{ping, add_user, list_all, get_user, edit_user, delete_user};
// use actix_web::dev::Server;
// use actix_web::{web, App, HttpServer};
// use sqlx::PgPool;
// use std::net::TcpListener;

// pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
//     // Wrap the pool using web::Data, which boils down to an Arc smart pointer
//     let db_pool = web::Data::new(db_pool);
//     let server = HttpServer::new(move || {
//         App::new()
//             .route("/ping", web::get().to(ping))
//             .route("/users", web::get().to(list_all))
//             .route("/users", web::post().to(add_user))
//             .route("/users/{id}", web::get().to(get_user))
//             .route("/users", web::put().to(edit_user))
//             .route("/users/{id}", web::delete().to(delete_user))
//             //.route("/users/add", web::post().to(users::add))
//             // Our pool is already wrapped in a Data: 
//             // using .data would add another Arc pointer on top 
//             // of the existing one - an unnecessary indirection.
//             // .app_data instead does not perform an additional layer of wrapping.
//             .app_data(db_pool.clone())
//     })
//     .listen(listener)?
//     .run();
//     Ok(server)
// }
use tracing_actix_web::TracingLogger;
use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database)
            .await
            .expect("Failed to connect to Postgres.");

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.with_db())
        .await
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .route("/ping", web::get().to(ping))
            .route("/users", web::get().to(list_all))
            .route("/users", web::post().to(add_user))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::put().to(edit_user))
            .route("/users/{id}", web::delete().to(delete_user))
            //.route("/users/add", web::post().to(users::add))
            // Our pool is already wrapped in a Data: 
            // using .data would add another Arc pointer on top 
            // of the existing one - an unnecessary indirection.
            // .app_data instead does not perform an additional layer of wrapping.
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}