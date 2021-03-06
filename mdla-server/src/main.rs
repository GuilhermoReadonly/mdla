use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use actix_web::{
    middleware::Logger,
    web::{self, scope, Data},
    App, HttpRequest, HttpServer,
};

use actix_files::{Files, NamedFile};

use endpoints::{guess, hints};
use env_logger::Env;
use log::info;
use mdla_lib::model::AppState;
use structopt::StructOpt;

mod endpoints;
mod errors;

fn get_words(file: &str) -> Vec<String> {
    let file_words = File::open(file).expect("Open words file...");

    let words: Vec<String> = BufReader::new(file_words)
        .lines()
        .enumerate()
        .map(|(i, line)| line.unwrap_or_else(|e| panic!("Read line {i}: {e}")))
        .collect();

    let nb_words = words.len();
    info!("{nb_words} words loaded from file {file}");

    words
}

async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./resources/web-app/index.html")?)
}

#[derive(StructOpt, Debug)]
#[structopt(about = "the server side of mdla !")]
pub struct Cli {
    #[structopt(short = "p", long = "port", default_value = "8000")]
    port: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let opt = Cli::from_args();
    let port = opt.port;

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(AppState {
                all_word_list: get_words("./word_list_all.db"),
                playable_word_list: get_words("./word_list_playable.db"),
            }))
            .wrap(Logger::default())
            .service(scope("/api").service(guess).service(hints))
            .route("/", web::get().to(index))
            .service(Files::new("/", "./resources/web-app/"))
    })
    .bind(format!("0.0.0.0:{port}"))?
    .run()
    .await
}
