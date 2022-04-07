use log::{error, warn};
use mdla_lib::model::{GuessBody, GuessResponse, GuessResponseOrError, HintsResponse};
use std::error::Error;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    components::{grid::GridComponent, message_box::MessageBox},
    network::request,
};

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
                    <GridComponent length={6} width={hints.number_of_letters} past_guesses={self.past_guesses.clone()} />
                    <p>
                        <input
                        type="text"
                        value=""
                        required=true
                        onchange={ctx.link().callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let value = input.value();
                            Msg::UpdateGuess(value)
                        })}/>
                        <button onclick={ctx.link().callback(move|_| {
                            Msg::PostGuess
                        })}>{"Envoyer"}</button>
                    </p>
                    <MessageBox />
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
                        self.past_guesses.push(guess_response);
                    }
                    Ok(GuessResponseOrError::Error(app_error)) => {
                        warn!("Bad request...: {:?}", app_error);
                        //TODO: display error message on screen
                    }
                    Err(e) => {
                        error!("Something terrible happened...: {:?}", e);
                    }
                }
                self.current_guess = String::new();
            }
        };
        true
    }
}
