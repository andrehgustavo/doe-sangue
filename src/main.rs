//! src/main.rs
// use doe_sangue_backend::configuration::get_configuration;
// use doe_sangue_backend::startup::run;
// use sqlx::PgPool;
// use std::net::TcpListener;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let configuration = get_configuration().expect("Failed to read configuration.");
//     // Renamed!
//     let connection_pool = PgPool::connect(&configuration.database.connection_string())
//         .await
//         .expect("Failed to connect to Postgres.");
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address)?;
//     run(listener, connection_pool)?.await
// }
use doe_sangue_backend::configuration::get_configuration;
use doe_sangue_backend::startup::Application;
use doe_sangue_backend::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("doe_sangue_backend".into(), "info".into());
    init_subscriber(subscriber);
    
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}