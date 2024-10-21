use std::fmt;
use std::fmt::Display;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug, PartialEq)]
pub enum ApiError {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    ValidationError(Vec<String>),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::BadRequest(err)
            | ApiError::InternalServerError(err)
            | ApiError::NotFound(err)
            | ApiError::InvalidData(err)
            | ApiError::Unknown(err)
            | ApiError::Conflict(err) => writeln!(f, "{},", err),
            ApiError::ValidationError(mex_vec) => mex_vec.iter().fold(Ok(()), |result, err| {
                result.and_then(|_| writeln!(f, "{}, ", err))
            }),
        }
    }
}

/// Automatically convert ApiErrors to ResponseError
impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::BadRequest(_) | ApiError::ValidationError(_) | ApiError::InvalidData(_) => {
                StatusCode::BAD_REQUEST
            }
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::InternalServerError(_) | ApiError::Unknown(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            ApiError::BadRequest(error) => HttpResponse::BadRequest().json(error.to_string()),
            ApiError::NotFound(message) => HttpResponse::NotFound().json(message.to_string()),
            ApiError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json(&errors.to_vec())
            }
            ApiError::InternalServerError(error) => {
                HttpResponse::InternalServerError().json(error.to_string())
            }
            ApiError::Conflict(error) => HttpResponse::Conflict().json(error.to_string()),
            ApiError::InvalidData(error) => HttpResponse::BadRequest().json(error.to_string()),
            ApiError::Unknown(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
