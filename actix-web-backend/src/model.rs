use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl User {
    pub async fn get_by_username(username: &str, pool: &PgPool) -> Result<User, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username=$1")
            .bind(username)
            .fetch_one(pool)
            .await
    }
}
