use mdla_lib::model::{GuessResponse, Validation};
use stylist::{css, YieldStyle, StyleSource};
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

impl YieldStyle for GridComponent {
    fn style_from(&self) -> StyleSource<'static> {
        css!("
            margin-left: auto;
            margin-right: auto;
            background-color: var(--color-back-grid);
            min-height: calc(6 * var(--cell-size) + 12 * var(--width-cell-border));
            border-spacing: 0;
            background-color: var(--color-back-grid);

            td {
                width: calc(var(--cell-size) - 2 * var(--width-padding-cell));
                height: calc(var(--cell-size) - 2 * var(--width-padding-cell));
                text-align: center;
                position: relative;
                padding: var(--width-padding-cell);
                color: var(--color-police-grid);
                border: 1px solid var(--color-border-grid);
                z-index: 0;
            }

            td.present {
                background-color: var(--color-present);
                border-radius: 50%;
            }
            
            td.correct {
                background-color: var(--color-correct);
            }
            
            td.not-in-word {
                background-color: var(--color-not-in-word);
            }
        ")
    }
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

        // let toto = include_str!("grid.css");
        // let style = Style::new(toto).unwrap();

        html! {
            <table class={self.style()}>{
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
