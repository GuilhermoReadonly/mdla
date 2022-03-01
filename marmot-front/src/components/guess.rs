use yew::prelude::*;

use crate::model::{GuessResponse, Validation};

#[derive(Debug)]
pub struct GuessComponent;

#[derive(Debug, Properties)]
pub struct GuessProperties {
    pub guess: GuessResponse,
}

impl PartialEq for GuessProperties {
    fn eq(&self, other: &Self) -> bool {
        self.guess.guess == other.guess.guess
    }
}

impl Component for GuessComponent {
    type Message = ();
    type Properties = GuessProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let guess_iter = ctx.props().guess.guess.chars();
        let validation_iter = ctx.props().guess.validation_list.iter();
        html! {
            guess_iter.zip(validation_iter).map(|(c, v)| {
                html! {
                    <td>
                    <GuessCellComponent letter={c} validation={v.clone()} />
                    </td>}
            }).collect::<Html>()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct GuessCellComponent;

#[derive(Debug, Properties, PartialEq)]
pub struct GuessCellProperties {
    pub letter: char,
    pub validation: Validation,
}

impl Component for GuessCellComponent {
    type Message = ();
    type Properties = GuessCellProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let v: char = match ctx.props().validation {
            Validation::Correct => '□',
            Validation::NotInWord => '×',
            Validation::Present => '◯',
        };
        html! {
            <>
                <div class="">{ctx.props().letter} </div>
                <div>{v}</div>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
