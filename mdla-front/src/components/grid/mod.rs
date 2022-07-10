use crate::components::grid::grid_input::GridInputComponent;
use crate::components::grid::grid_line::GridLineComponent;
use mdla_lib::model::GuessResponse;
use stylist::{css, StyleSource, YieldStyle};
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
        let lines = 0..ctx.props().past_guesses.len();
        let width = ctx.props().width;

        html! {
            <>
            <table class={self.style()}>
            {
                lines
                .into_iter()
                .map(|i| {
                    let guess = ctx.props().past_guesses.get(i).cloned();
                    html! {<tr> <GridLineComponent guess={guess.expect("A guess should exist.")} width={width} /> </ tr>}
                })
                .collect::<Html>()
            }
            {
                html! {<tr> <GridInputComponent width={width} on_guessed_word_change={ctx.props().on_guessed_word_change.clone()} /> </ tr>}
            }

            </table>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}

impl YieldStyle for GridComponent {
    fn style_from(&self) -> StyleSource<'static> {
        css!(
            "
            margin-left: auto;
            margin-right: auto;
            border-spacing: 0;
            background-color: var(--color-back-grid);
            color: var(--color-police-grid);

            td {
                width: calc(var(--cell-size) - 2 * var(--width-padding-cell));
                height: calc(var(--cell-size) - 2 * var(--width-padding-cell));
                text-align: center;
                position: relative;
                padding: var(--width-padding-cell);
                
                border: 1px solid var(--color-border-grid);
                z-index: 0;
            }

            .present {
                background-color: var(--color-present);
                border-radius: 50%;
            }
            
            .correct {
                background-color: var(--color-correct);
            }
            
            .not-in-word {
                background-color: var(--color-not-in-word);
            }



            .input-cell {
                border: none;
                height: 100%;
                outline: none;
                font-size: 30px;
                width: calc(100% - 2 * var(--width-padding-cell));
                text-align: center;
                background-color: var(--color-back-grid);
                color: var(--color-police-grid);
            }
        "
        )
    }
}
