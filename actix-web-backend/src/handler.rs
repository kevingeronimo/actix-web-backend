use crate::error::Result;
use crate::{
    dto::{LoginDto, RegisterDto},
    service::AuthService,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};

#[post("/login")]
async fn login(
    pool: Data<sqlx::PgPool>,
    Json(login_dto): Json<LoginDto>,
) -> Result<impl Responder> {
    let user = AuthService::sign_in(login_dto, &pool).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/register")]
async fn register(
    pool: Data<sqlx::PgPool>,
    Json(register_dto): Json<RegisterDto>,
) -> Result<impl Responder> {
    let user = AuthService::sign_up(register_dto, &pool).await?;

    Ok(HttpResponse::Ok().json(user))
}
