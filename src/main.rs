use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use actix_web::{
    middleware::Logger,
    post,
    web::{scope, Json},
    App, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;
use log::info;
use rand::{prelude::IteratorRandom, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct GuessResponse {
    validation_list: Vec<Validation>,
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

#[post("/guess")]
async fn guess(guess_body: Json<GuessBody>) -> impl Responder {
    info!("Body : {guess_body:?}");

    let mut rng = thread_rng();

    let file = File::open("./word_list").expect("Can't open file...");
    let reader = BufReader::new(file);

    let word = reader
        .lines()
        .choose(&mut rng)
        .expect("Can't choose a word...")
        .expect("Can't read lines...");

    info!("Today word is : {word:?}");
    let word: Vec<char> = word.chars().collect();

    let mut validation_list = vec![];

    for (i, c) in guess_body.guess.chars().enumerate() {
        let validation = match (&c == word.get(i).expect(""), word.contains(&c)) {
            (true, _) => Validation::Correct,
            (false, true) => Validation::Present,
            (false, false) => Validation::NotInWord,
        };

        validation_list.push(validation);
    }

    let body = serde_json::to_string(&GuessResponse { validation_list }).expect("Can't serialize");

    // Create response and set content type
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(scope("/api").service(guess))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
