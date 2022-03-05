use std::fmt::{Debug, Display};

use actix_web::{error, http::StatusCode, HttpResponse, HttpResponseBuilder};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum AppError {
    BadWordLength {
        size_expected: usize,
        size_received: usize,
        word_sent: String,
    },
    WordNotInDictionary(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AppError::BadWordLength {
                size_expected,
                size_received,
                word_sent,
            } => {
                write!(f, "Word should be {size_expected} letters but received {size_received} : {word_sent}")
            }
            AppError::WordNotInDictionary(w) => {
                write!(f, "Word {w} is not in our dictionary")
            }
        }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(self)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadWordLength {
                size_expected: _,
                size_received: _,
                word_sent: _,
            } => StatusCode::BAD_REQUEST,
            AppError::WordNotInDictionary(_) => StatusCode::BAD_REQUEST,
        }
    }
}
