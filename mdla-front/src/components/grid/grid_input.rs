use log::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug)]
pub struct GridInputComponent {
    pub guessed_word: String,
}

#[derive(Debug, Properties, PartialEq)]
pub struct GridInputProperties {
    pub width: usize,
    pub on_guessed_word_change: Callback<String>,
}

#[derive(Debug)]
pub enum Msg {
    UpdateGuess(String, usize),
}

impl Component for GridInputComponent {
    type Message = Msg;
    type Properties = GridInputProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            guessed_word: String::default(),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cells = 0..ctx.props().width;
        cells
            .into_iter()
            .map(|cell_number| {
                html! {
                    <>
                        <td>
                            <input
                                class="input-cell"
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
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("Message = {msg:?}");
        match msg {
            Msg::UpdateGuess(char, position) => {
                if position < self.guessed_word.len() {
                    self.guessed_word.insert_str(position, &char);
                } else {
                    self.guessed_word.push_str(&char);
                }
                ctx.props()
                    .on_guessed_word_change
                    .emit(self.guessed_word.clone());
            }
        }
        false
    }
}
