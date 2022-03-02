use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct HintsResponse {
    pub number_of_letters: usize,
    pub first_letter: char,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GuessResponse {
    pub validation_list: Vec<Validation>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Validation {
    Correct(char),
    Present(char),
    NotInWord(char),
}

#[derive(Debug, Serialize)]
pub struct GuessBody {
    pub guess: String,
}
