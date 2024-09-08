use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum RespError {
    Unauthorized,
    Custom(String),
}

impl Display for RespError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl ResponseError for RespError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Custom(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::Unauthorized => HttpResponse::Unauthorized().finish(),
            Self::Custom(msg) => HttpResponse::InternalServerError().body(msg.to_string()),
        }
    }
}

pub type CommonResponse = actix_web::Result<HttpResponse>;
