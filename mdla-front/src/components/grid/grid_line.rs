use mdla_lib::model::GuessResponse;

use crate::components::grid::grid_cell::GridCellComponent;
use yew::prelude::*;

#[derive(Debug)]
pub struct GridLineComponent {
    pub guessed_word: String,
}

#[derive(Debug, Properties, PartialEq)]
pub struct GridLineProperties {
    pub guess: GuessResponse,
    pub width: usize,
}

impl Component for GridLineComponent {
    type Message = ();
    type Properties = GridLineProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            guessed_word: String::default(),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let validation_iter = ctx.props().guess.validation_list.iter();
        html! {
            validation_iter.map(|v| {
                html! {
                    <>
                    <GridCellComponent validation={Some(v.clone())} />
                    </>
                }
            }).collect::<Html>()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
