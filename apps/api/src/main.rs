use std::{env, time::Duration};

use actix_web::{guard, web, web::Data, App, HttpServer};

use async_graphql::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

mod graphql;
mod router;

async fn db(db_url: String) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(db_url.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    // .sqlx_logging_level(log::LevelFilter::Info)

    Database::connect(opt).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = db(db_url).await.unwrap();

    // Build the Schema
    let schema = graphql::create_schema(db);

    println!("GraphQL Endpoint: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(
                web::resource("/healthcheck")
                    .guard(guard::Get())
                    .to(router::health_handler),
            )
            .service(web::resource("/").guard(guard::Post()).to(router::index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(router::graphql_playground),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
