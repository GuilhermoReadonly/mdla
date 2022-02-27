use log::error;
use std::error::Error;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    model::{GuessBody, GuessResponse, HintsResponse},
    network::request,
};

#[derive(Debug)]
pub enum Msg {
    GetHints,
    GetHintsResponse(Result<HintsResponse, Box<dyn Error>>),
    UpdateGuess(String),
    PostGuess,
    PostGuessResponse(Result<GuessResponse, Box<dyn Error>>),
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
        html! {
            <>
                <h1>{"Marmot"}</h1>
                <p>{format!("Hints: {:?}", &self.hints)}</p>

                <p>{format!("Past guesses: {:?}", &self.past_guesses)}</p>

                <p>{"Guess: "}
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
                    })}>{"Sauvegarder"}</button>
                </p>
            </>
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
                    log::error!("Something terrible happened...: {:?}", e);
                    self.hints = None;

                    error!("Can't fetch hints: {e}");
                }
            },
            Msg::UpdateGuess(guess) => {
                self.current_guess = guess;
            }
            Msg::PostGuess => {
                let current_guess = self.current_guess.clone();
                ctx.link().send_future(async move {
                    match request::<GuessBody, GuessResponse>(
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
                    Ok(guess_response) => {
                        self.past_guesses.push(guess_response);
                    }
                    Err(e) => {
                        log::error!("Something terrible happened...: {:?}", e);
                        error!("Can't post guess: {e}");
                    }
                }
                self.current_guess = String::new();
            }
        };
        true
    }
}
