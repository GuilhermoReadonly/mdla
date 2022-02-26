use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use actix_web::{middleware::Logger, web::scope, App, HttpServer};

use endpoints::{guess, hints};
use env_logger::Env;

use crate::model::AppState;

mod endpoints;
mod errors;
mod model;

fn get_words() -> Vec<String> {
    let file = File::open("./word_list").expect("Open file...");

    BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(i, line)| line.unwrap_or_else(|e| panic!("Read line {i}: {e}")))
        .collect()
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
