use log::{error, warn};
use mdla_lib::model::{
    AppError, GuessBody, GuessResponse, GuessResponseOrError, HintsResponse, Validation,
};
use std::error::Error;

use yew::prelude::*;

use crate::{
    components::{grid::GridComponent, message_box::MessageBox},
    network::request,
};

const PICTO_RED: char = '🟥';
const PICTO_YELLOW: char = '🟡';
const PICTO_BLUE: char = '🟦';

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub text: Vec<String>,
    pub severity: Severity,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Severity {
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
pub enum Msg {
    GetHints,
    GetHintsResponse(Result<HintsResponse, Box<dyn Error>>),
    UpdateGuess(String),
    PostGuess,
    PostGuessResponse(Result<GuessResponseOrError, Box<dyn Error>>),
}

#[derive(Debug)]
pub struct GamePageComponent {
    hints: Option<HintsResponse>,
    past_guesses: Vec<GuessResponse>,
    current_guess: String,
    message: Option<Message>,
}

impl GamePageComponent {
    fn get_picto_result(&self) -> Vec<String> {
        let mut result = vec![];

        for guess in self.past_guesses.iter() {
            let mut line = String::default();
            for validation in guess.validation_list.iter() {
                match &validation {
                    Validation::Correct(_) => line.push(PICTO_RED),
                    Validation::Present(_) => line.push(PICTO_YELLOW),
                    Validation::NotInWord(_) => line.push(PICTO_BLUE),
                }
            }
            result.push(line)
        }
        let mut last_line = String::default();
        for _ in 0..self.current_guess.len() {
            last_line.push(PICTO_RED)
        }
        result.push(last_line);
        result
    }
}

impl Component for GamePageComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetHints);
        Self {
            hints: None,
            past_guesses: vec![],
            current_guess: String::new(),
            message: None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(hints) = &self.hints {
            html! {
                <>
                    <h1>{"MdlA"}</h1>
                    <p>{format!("Mot de {} lettres commençant par {}", hints.number_of_letters, hints.first_letter)}</p>
                    <GridComponent
                        width={hints.number_of_letters}
                        past_guesses={self.past_guesses.clone()}
                        on_guessed_word_change={ctx.link().callback(Msg::UpdateGuess)}
                        on_validate={ctx.link().callback(|_| Msg::PostGuess)}
                    />
                    <MessageBox message={self.message.clone()} />
                </>
            }
        } else {
            html! {}
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Message received: {:?}", msg);

        match msg {
            Msg::GetHints => {
                ctx.link().send_future(async move {
                    match request::<(), HintsResponse>("GET", "/api/hints", None).await {
                        Ok(data) => Msg::GetHintsResponse(Ok(data)),
                        Err(err) => Msg::GetHintsResponse(Err(Box::new(err))),
                    }
                });
            }
            Msg::GetHintsResponse(response) => match response {
                Ok(hints) => {
                    self.hints = Some(hints);
                }
                Err(e) => {
                    error!("Something terrible happened...: {:?}", e);
                    self.hints = None;
                }
            },
            Msg::UpdateGuess(guess) => {
                self.current_guess = guess;
            }
            Msg::PostGuess => {
                self.message = None;

                let current_guess = self.current_guess.clone();

                ctx.link().send_future(async move {
                    match request::<GuessBody, GuessResponseOrError>(
                        "POST",
                        "/api/guess",
                        Some(GuessBody {
                            guess: current_guess,
                        }),
                    )
                    .await
                    {
                        Ok(data) => Msg::PostGuessResponse(Ok(data)),
                        Err(err) => Msg::PostGuessResponse(Err(Box::new(err))),
                    }
                });
            }
            Msg::PostGuessResponse(response) => {
                match response {
                    Ok(GuessResponseOrError::Response(guess_response)) => {
                        let correct_guess = guess_response
                            .clone()
                            .validation_list
                            .into_iter()
                            .all(|v| matches!(v, Validation::Correct(_)));
                        if correct_guess {
                            self.message = Some(Message {
                                severity: Severity::Info,
                                text: [
                                    vec![
                                        "Bravo ! \\o/".to_string(),
                                        "".to_string(),
                                        "Partage ton score:".to_string(),
                                        "".to_string(),
                                    ],
                                    self.get_picto_result(),
                                ]
                                .concat(),
                            })
                        }

                        self.past_guesses.push(guess_response);
                    }
                    Ok(GuessResponseOrError::Error(app_error)) => {
                        warn!("Bad request...: {:?}", app_error);
                        match app_error{
                            AppError::WordNotInDictionary(w) => {
                                self.message = Some(Message{severity:Severity::Warn, text:vec![format!("Le mot {w} n'est pas dans notre dictionnaire.")]})
                            }
                            AppError::BadWordLength { size_expected: se, size_received: sr, word_sent: w } => {
                                self.message = Some(Message{severity:Severity::Warn, text:vec![format!("Le mot {w} a {sr} lettres mais le mot a deviner doit en avoir {se}.")]})
                            }
                        }
                    }
                    Err(e) => {
                        error!("Something terrible happened...: {:?}", e);
                        self.message = Some(Message {
                            severity: Severity::Error,
                            text: vec![format!("Quelque chose cloche... Reviens dans quelques minutes le temps que le serveur revienne de vacances ! :)")],
                        })
                    }
                }
                self.current_guess = String::new();
            }
        };
        true
    }
}
