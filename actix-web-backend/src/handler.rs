use crate::model::User;
use actix_web::{post, web, HttpResponse, Responder};
use crate::error::Result;
use anyhow::Context;

#[post("/login")]
async fn login(pool: web::Data<sqlx::PgPool>) -> Result<impl Responder> {
    let user = User::get_by_username("kev1", &pool)
        .await.context("hi")?;

    Ok(HttpResponse::Ok().json(user))
}
