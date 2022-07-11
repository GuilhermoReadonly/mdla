use crate::components::grid::grid_input::GridInputComponent;
use crate::components::grid::grid_line::GridLineComponent;
use mdla_lib::model::{GuessResponse, Validation};
use yew::prelude::*;

mod grid_cell;
mod grid_input;
mod grid_line;

#[derive(Debug)]
pub struct GridComponent;

#[derive(Debug, Properties, PartialEq)]
pub struct GridProperties {
    pub past_guesses: Vec<GuessResponse>,
    pub width: usize,
    pub on_guessed_word_change: Callback<String>,
    pub on_validate: Callback<()>,
}

impl Component for GridComponent {
    type Message = ();
    type Properties = GridProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let past_guesses = &ctx.props().past_guesses;
        let width = ctx.props().width;

        html! {
            <>
            <table class="grid">
            // Past guesses grid
            {
                past_guesses
                .iter()
                .map(|g| {
                    let guess = g.clone();
                    html! {<tr> <GridLineComponent guess={guess} width={width} /> </ tr>}
                })
                .collect::<Html>()
            }
            // Input grid: displayed if game still going
            {
                if past_guesses.last().map_or(false, |last_guess| last_guess.validation_list.iter().all(|v| matches!(v, Validation::Correct(_))))
                {
                    html! {}
                } else {
                    html!{
                        <tr>
                            <GridInputComponent
                                width={width}
                                on_guessed_word_change={ctx.props().on_guessed_word_change.clone()}
                                on_validate={ctx.props().on_validate.clone()}
                            />
                        </ tr>
                    }
                }
            }

            </table>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
