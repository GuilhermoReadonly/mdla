use std::fmt::{Debug, Display};

use actix_web::{error, http::StatusCode, HttpResponse, HttpResponseBuilder};
use mdla_lib::model::AppError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ResponseOrError<A> {
    Response(A),
    Error(AppError),
}

impl<A: Debug> Display for ResponseOrError<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ResponseOrError::Error(AppError::BadWordLength {
                size_expected,
                size_received,
                word_sent,
            }) => {
                write!(f, "Word should be {size_expected} letters but received {size_received} : {word_sent}")
            }
            ResponseOrError::Error(AppError::WordNotInDictionary(w)) => {
                write!(f, "Word {w} is not in our dictionary")
            }
            ResponseOrError::Response(guess_response) => {
                write!(f, "Guess response:  {guess_response:?}")
            }
        }
    }
}

impl<A: Debug + Serialize> error::ResponseError for ResponseOrError<A> {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(self)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ResponseOrError::Error(AppError::BadWordLength {
                size_expected: _,
                size_received: _,
                word_sent: _,
            }) => StatusCode::BAD_REQUEST,
            ResponseOrError::Error(AppError::WordNotInDictionary(_)) => StatusCode::BAD_REQUEST,
            ResponseOrError::Response(_) => StatusCode::OK,
        }
    }
}
