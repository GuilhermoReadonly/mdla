use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    BadWordLength {
        size_expected: usize,
        size_received: usize,
        word_sent: String,
    },
    WordNotInDictionary(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GuessResponse {
    pub validation_list: Vec<Validation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HintsResponse {
    pub number_of_letters: usize,
    pub first_letter: char,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Validation {
    Correct(char),
    Present(char),
    NotInWord(char),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuessBody {
    pub guess: String,
}

pub struct AppState {
    pub all_word_list: Vec<String>,
    pub playable_word_list: Vec<String>,
}
