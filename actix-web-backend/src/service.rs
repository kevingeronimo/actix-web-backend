use crate::{
    dto::{LoginDto, RegisterDto},
    error::{Error, Result},
    model::User,
    util,
};
use anyhow::{anyhow, Context};
use sqlx::PgPool;

pub struct AuthService;

impl AuthService {
    pub async fn sign_in(dto: LoginDto, pool: &PgPool) -> Result<User> {
        let user = User::get_by_username(&dto.username, pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => Error::Unauthorized,
                _ => anyhow!(e).into(),
            })?;

        if util::verify_password(dto.password, user.password.to_owned()).await? {
            Ok(user)
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub async fn sign_up(dto: RegisterDto, pool: &PgPool) -> Result<User> {
        if User::get_by_username(&dto.username, pool).await.is_ok() {
            return Err(Error::UsernameAlreadyExists);
        }

        // password is dropped after hashing.
        let password = util::hash_password(dto.password).await?;
        let dto = RegisterDto {
            username: dto.username,
            password,
        };

        User::create(dto, pool)
            .await
            .context("failed to create user")
            .map_err(|e| e.into())
    }
}
