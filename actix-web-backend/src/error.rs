use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid credentials")]
    Unauthorized,

    #[error("username already exists")]
    UsernameAlreadyExists,

    #[error("{0:?}")]
    Other(#[from] anyhow::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::UsernameAlreadyExists => StatusCode::CONFLICT,
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let message = match self {
            Error::Unauthorized => self.to_string(),
            Error::UsernameAlreadyExists => self.to_string(),
            Error::Other(ref root_cause) => match root_cause.downcast_ref::<sqlx::Error>() {
                Some(sqlx::Error::RowNotFound) => "not found".to_owned(),
                _ => "internal server error".to_owned(),
            },
        };

        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message,
            error: self.status_code().to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}
