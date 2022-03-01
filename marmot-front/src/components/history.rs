use yew::prelude::*;

use crate::components::guess::GuessComponent;
use crate::model::GuessResponse;

#[derive(Debug)]
pub struct HistoryComponent;

#[derive(Debug, Properties)]
pub struct HistoryProperties {
    pub past_guesses: Vec<GuessResponse>,
}

impl PartialEq for HistoryProperties {
    fn eq(&self, other: &Self) -> bool {
        if self.past_guesses.len() != other.past_guesses.len() {
            false
        } else {
            for (i, elt) in self.past_guesses.iter().enumerate() {
                let other_guess = other.past_guesses.get(i);
                match other_guess {
                    None => return false,
                    Some(elt_other) => {
                        if elt.guess != elt_other.guess {
                            return false;
                        }
                    }
                }
            }
            true
        }
    }
}

impl Component for HistoryComponent {
    type Message = ();
    type Properties = HistoryProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <table>{
                ctx.props().past_guesses.iter().map(|g| {
                    html! {<tr> <GuessComponent guess={g.clone()} /> </ tr>}
                }).collect::<Html>()
            }</table>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
