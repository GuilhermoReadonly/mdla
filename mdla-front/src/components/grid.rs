use mdla_lib::model::{GuessResponse, Validation};
use stylist::{css, StyleSource, YieldStyle};
use web_sys::HtmlInputElement;
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
    pub on_guessed_word_change: Callback<String>,
}

impl YieldStyle for GridComponent {
    fn style_from(&self) -> StyleSource<'static> {
        css!(
            "
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

            td.editable {
                padding: 0;
            }

            td.editabe > input {
                border: none;
                height: 100%;
                outline: none;
                background-color: var(--color-back-grid);
                font-size: 30px;
                width: calc(100% - 2 * var(--width-padding-cell));
                text-align: center;
            }
        "
        )
    }
}

pub enum Msg2 {
    UpdateGuess(String),
}

impl Component for GridComponent {
    type Message = Msg2;
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
            <table class={self.style()}>{
                lines.into_iter().map(|i| {
                    let guess = ctx.props().past_guesses.get(i).cloned();

                    let editable = i == 0 && guess.is_none() || i > 0 && guess.is_none() && ctx.props().past_guesses.get(i - 1).cloned().is_some();

                    let on_guessed_word_change: Option<Callback<String>> = if editable {
                        Some(ctx.link().callback(|guessed_word: String| {
                            Msg2::UpdateGuess(guessed_word)
                        }))
                    } else {
                        None
                    };

                    html! {<tr> <GridLineComponent editable={editable} guess={guess} width={width} {on_guessed_word_change}  /> </ tr>}
                }).collect::<Html>()
            }</table>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg2::UpdateGuess(char) => {
                _ctx.props().on_guessed_word_change.emit(char);
            }
        }
        false
    }
}

/// Grid Line
///
///
#[derive(Debug)]
pub struct GridLineComponent {
    pub guessed_word: String,
}

#[derive(Debug, Properties, PartialEq)]
pub struct GridLineProperties {
    pub guess: Option<GuessResponse>,
    pub width: usize,
    pub editable: bool,
    pub on_guessed_word_change: Option<Callback<String>>,
}

pub enum Msg {
    UpdateGuess(String, usize),
}

impl Component for GridLineComponent {
    type Message = Msg;
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
        match &ctx.props().guess {
            None => {
                if ctx.props().editable {
                    let cells = 0..ctx.props().width;
                    cells
                        .into_iter()
                        .map(|cell_number| {
                            html! {
                                <>
                                    <td class="editabe">
                                        <input
                                            type="text"
                                            maxlength="1"
                                            onchange={ctx.link().callback(move |e: Event| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                let value = input.value();
                                                Msg::UpdateGuess(value, cell_number)
                                            })}
                                        />
                                    </td>
                                </>
                            }
                        })
                        .collect::<Html>()
                } else {
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
        match _msg {
            Msg::UpdateGuess(char, _position) => {
                self.guessed_word.push_str(&char);
                if let Some(cb) = &_ctx.props().on_guessed_word_change {
                    cb.emit(self.guessed_word.clone());
                }
            }
        }
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
