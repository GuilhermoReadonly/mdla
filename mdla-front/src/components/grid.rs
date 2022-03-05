use mdla_lib::model::{GuessResponse, Validation};
use yew::prelude::*;

/// Grid
///
///
#[derive(Debug)]
pub struct GridComponent;

#[derive(Debug, Properties, PartialEq)]
pub struct GridProperties {
    pub past_guesses: Vec<GuessResponse>,
    pub length: usize,
    pub width: usize,
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
        let lines = 0..ctx.props().length;
        let width = ctx.props().width;
        html! {
            <table class="grid">{
                lines.into_iter().map(|i| {
                    let guess = ctx.props().past_guesses.get(i).cloned();

                    html! {<tr> <GridLineComponent guess={guess} width={width} /> </ tr>}
                }).collect::<Html>()
            }</table>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}

/// Grid Line
///
///
#[derive(Debug)]
pub struct GridLineComponent;

#[derive(Debug, Properties, PartialEq)]
pub struct GridLineProperties {
    pub guess: Option<GuessResponse>,
    pub width: usize,
}

impl Component for GridLineComponent {
    type Message = ();
    type Properties = GridLineProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &ctx.props().guess {
            None => {
                let cells = 0..ctx.props().width;
                cells
                    .into_iter()
                    .map(|_| {
                        html! {
                            <>
                            <GridCellComponent validation={None} />
                            </>
                        }
                    })
                    .collect::<Html>()
            }
            Some(g) => {
                let validation_iter = g.validation_list.iter();
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}

/// Grid cell
///
///
#[derive(Debug)]
pub struct GridCellComponent;

#[derive(Debug, Properties, PartialEq)]
pub struct GridCellProperties {
    pub validation: Option<Validation>,
}

impl Component for GridCellComponent {
    type Message = ();
    type Properties = GridCellProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (c, class) = match ctx.props().validation {
            Some(Validation::Correct(c)) => (c, "correct"),
            Some(Validation::NotInWord(c)) => (c, "not-in-word"),
            Some(Validation::Present(c)) => (c, "present"),
            None => (' ', ""),
        };
        html! {
            <>
                <td class={class}>{c} </td>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
