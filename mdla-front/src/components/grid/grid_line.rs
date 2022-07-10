use mdla_lib::model::GuessResponse;

use crate::components::grid::grid_cell::GridCellComponent;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateGuess(char, _position) => {
                self.guessed_word.push_str(&char);
                if let Some(cb) = &ctx.props().on_guessed_word_change {
                    cb.emit(self.guessed_word.clone());
                }
            }
        }
        false
    }
}
