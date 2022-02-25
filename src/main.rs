use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{scope, Data, Json},
    App, HttpServer, Result,
};
use chrono::{TimeZone, Utc};
use env_logger::Env;
use log::info;
use rand::{
    prelude::{IteratorRandom, StdRng},
    SeedableRng,
};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

mod errors;

#[derive(Debug, Serialize)]
struct GuessResponse {
    validation_list: Vec<Validation>,
}

#[derive(Debug, Serialize)]
struct HintsResponse {
    number_of_letters: usize,
    first_letter: char,
}

#[derive(Debug, Serialize)]
enum Validation {
    #[serde(rename = "□")]
    Correct,
    #[serde(rename = "◯")]
    Present,
    #[serde(rename = "X")]
    NotInWord,
}

#[derive(Debug, Deserialize)]
struct GuessBody {
    guess: String,
}

struct AppState {
    word_list: Vec<String>,
}

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

fn get_words() -> Vec<String> {
    let file = File::open("./word_list").expect("Open file...");

    BufReader::new(file)
        .lines()
        .map(|l| l.unwrap_or_else(|_| panic!("{}", "Read line".to_string())))
        .collect()
}

#[post("/guess")]
async fn guess(data: Data<AppState>, guess_body: Json<GuessBody>) -> Result<Json<GuessResponse>> {
    info!("Body : {guess_body:?}");

    let word: Vec<char> = get_today_word(&data.word_list).chars().collect();

    if word.len() != guess_body.guess.len() {
        return Err(AppError::BadWordLength(word.len()).into());
    }

    let mut validation_list = vec![];

    for (i, c) in guess_body.guess.chars().enumerate() {
        let validation = match (
            &c == word
                .get(i)
                .unwrap_or_else(|| panic!("Index {i} exist for word {word:?}")),
            word.contains(&c),
        ) {
            (true, _) => Validation::Correct,
            (false, true) => Validation::Present,
            (false, false) => Validation::NotInWord,
        };

        validation_list.push(validation);
    }

    let response = GuessResponse { validation_list };
    Ok(Json(response))
}

#[get("/hints")]
async fn hints(data: Data<AppState>) -> Result<Json<HintsResponse>> {
    let word: Vec<char> = get_today_word(&data.word_list).chars().collect();

    let response = HintsResponse {
        first_letter: word[0],
        number_of_letters: word.len(),
    };
    Ok(Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .data(AppState {
                word_list: get_words(),
            })
            .wrap(Logger::default())
            .service(scope("/api").service(guess).service(hints))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
