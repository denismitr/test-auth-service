use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use uuid::Error as UuidError;

#[derive(Debug, Display)]
pub enum AuthError {
    #[display(fmt = "DuplicateValue: {}", _0)]
    DuplicateValue(String),

    #[display(fmt = "BadId")]
    BadId(),

    #[display(fmt = "GenericError: {}", _0)]
    GenericError(String)
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::BadId => HttpResponse::BadRequest().json("Invalid ID"),
            AuthError::DuplicateValue(ref message) => HttpResponse::BadRequest().json(message),
            AuthError::GenericError(ref message) => HttpResponse::BadRequest().json(message), 
        }
    }
}