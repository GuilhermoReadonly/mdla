use std::fmt::{Debug, Display};

use actix_web::{error, http::header, http::StatusCode, HttpResponse, HttpResponseBuilder};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) enum AppError {
    BadWordLength(usize),
    WordNotInDictionary(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AppError::BadWordLength(i) => {
                write!(f, "{{\"error\": \"Word should be {} letters\"}}", i)
            }
            AppError::WordNotInDictionary(w) => {
                write!(f, "{{\"error\": \"Word {} is not in our dictionary\"}}", w)
            }
        }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(header::ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::BadWordLength(_) => StatusCode::BAD_REQUEST,
            AppError::WordNotInDictionary(_) => StatusCode::BAD_REQUEST,
        }
    }
}
