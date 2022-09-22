use actix_session::{
    config::PersistentSession, storage::RedisActorSessionStore, SessionMiddleware,
};
use actix_web::{
    cookie::{time, Key},
    middleware::Logger,
    web, App, HttpServer,
};
use actix_web_backend::handler::{login, protected, register};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:pass@postgres/postgres".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(login)
            .service(register)
            .service(protected)
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("redis:6379"),
                    secret_key.clone(),
                )
                .session_lifecycle(
                    PersistentSession::default().session_ttl(time::Duration::days(1)),
                )
                .cookie_secure(false)
                .build(),
            )
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
    .map_err(Into::into)
}
