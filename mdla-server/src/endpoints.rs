use actix_web::{
    get, post,
    web::{Data, Json},
    Result,
};
use chrono::{TimeZone, Utc};

use log::{info, warn};
use rand::{
    prelude::{IteratorRandom, StdRng},
    SeedableRng,
};

use crate::{
    errors::AppError,
    model::{AppState, GuessBody, GuessResponse, HintsResponse, Validation},
};

fn get_today_word(words: &[String]) -> String {
    // The goal here is to get a number that change everyday in order to initialise the seed of the random number generator.
    let days_since_y0 = (Utc::now() - Utc.ymd(1, 1, 1).and_hms(0, 0, 0)).num_days();
    info!("Seed init to: {days_since_y0}");

    let mut rng: StdRng = SeedableRng::seed_from_u64(days_since_y0.unsigned_abs());

    let word = words
        .iter()
        .choose(&mut rng)
        .expect("Choose a word...")
        .clone();

    info!("Today word is : {word:?}");
    word
}

#[post("/guess")]
pub async fn guess(
    data: Data<AppState>,
    guess_body: Json<GuessBody>,
) -> Result<Json<GuessResponse>> {
    info!("Body : {guess_body:?}");

    let word: Vec<char> = get_today_word(&data.word_list)
        .to_uppercase()
        .chars()
        .collect();

    if word.len() != guess_body.guess.len() {
        let error = AppError::BadWordLength {
            size_expected: word.len(),
            size_received: guess_body.guess.len(),
            word_sent: guess_body.guess.clone(),
        };
        warn!("{error}");
        return Err(error.into());
    }
    if !data.word_list.contains(&guess_body.guess.to_uppercase()) {
        let error = AppError::WordNotInDictionary(guess_body.guess.clone());
        warn!("{error}");
        return Err(error.into());
    }

    let mut validation_list = vec![];

    for (i, c) in guess_body.guess.to_uppercase().chars().enumerate() {
        let validation = match (
            &c == word
                .get(i)
                .unwrap_or_else(|| panic!("Index {i} exist for word {word:?}")),
            word.contains(&c),
        ) {
            (true, _) => Validation::Correct(c),
            (false, true) => Validation::Present(c),
            (false, false) => Validation::NotInWord(c),
        };

        validation_list.push(validation);
    }

    let response = GuessResponse { validation_list };
    Ok(Json(response))
}

#[get("/hints")]
pub async fn hints(data: Data<AppState>) -> Result<Json<HintsResponse>> {
    let word: Vec<char> = get_today_word(&data.word_list).chars().collect();

    let response = HintsResponse {
        first_letter: word[0],
        number_of_letters: word.len(),
    };
    Ok(Json(response))
}