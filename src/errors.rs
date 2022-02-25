use std::fmt::{Debug, Display};

use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) enum AppError {
    BadWordLength(usize),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AppError::BadWordLength(i) => {
                write!(f, "{{\"error\": \"Word should be {} letters\"}}", i)
            }
        }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::BadWordLength(_) => StatusCode::BAD_REQUEST,
        }
    }
}
