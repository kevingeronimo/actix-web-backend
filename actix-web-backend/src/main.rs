use actix_web::{web, App, HttpServer};
use actix_web_backend::handler::{login, register};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:pass@postgres/postgres".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(login)
            .service(register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
