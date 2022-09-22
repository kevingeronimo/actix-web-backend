use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_backend::model::User;
use sqlx::postgres::PgPoolOptions;

#[get("/")]
async fn hello(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let user = User::get_by_username("kev1", &pool).await.unwrap();

    HttpResponse::Ok().json(user)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

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
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
