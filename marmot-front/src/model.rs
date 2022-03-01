use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct HintsResponse {
    pub number_of_letters: usize,
    pub first_letter: char,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuessResponse {
    pub validation_list: Vec<Validation>,
    pub guess: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Validation {
    #[serde(rename = "□")]
    Correct,
    #[serde(rename = "◯")]
    Present,
    #[serde(rename = "X")]
    NotInWord,
}

#[derive(Debug, Serialize)]
pub struct GuessBody {
    pub guess: String,
}
