use crate::error::Result;
use crate::{
    dto::{LoginDto, RegisterDto},
    service::AuthService,
};
use actix_session::Session;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};

#[post("/login")]
async fn login(
    session: Session,
    pool: Data<sqlx::PgPool>,
    Json(login_dto): Json<LoginDto>,
) -> Result<impl Responder> {
    let user = AuthService::sign_in(login_dto, &pool).await?;
    session
        .insert("username", &user.username)
        .map_err(anyhow::Error::new)?;

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

#[get("/protected")]
async fn protected(session: Session) -> Result<impl Responder> {
    if let Some(username) = session
        .get::<String>("username")
        .map_err(anyhow::Error::new)?
    {
        Ok(HttpResponse::Ok().body(format!("Welcome! {username}")))
    } else {
        Err(crate::error::Error::Unauthorized)
    }
}
